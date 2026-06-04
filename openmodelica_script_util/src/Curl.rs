// Manually written file.
//
// Rust port of `OMCompiler/Compiler/Util/Curl.mo`, whose single function
// is an `external "C"` shim into `OMCompiler/Compiler/runtime/om_curl.c`
// (libcurl's multi interface). Instead of linking libcurl we spawn the
// `curl` command-line tool — it is universally available wherever omc
// runs, handles TLS/redirects, and keeps this crate dependency-free.
//
// Semantics mirrored from `om_curl_multi_download`:
//
//   * Each work item is `(mirror URLs, target filename)`. The first URL is
//     fetched into `<filename>.tmp<N>` (N = global transfer counter) and
//     renamed over the target on success.
//   * On failure the temp file is removed; if more mirror URLs remain the
//     item is re-queued (at the front) with the tail of the URL list,
//     otherwise the download counts as failed and the result is `false`.
//   * At most `maxParallel` transfers run concurrently.
//   * Diagnostics use the same message templates and token order as the C
//     implementation (token text comes from curl's stderr instead of
//     `curl_easy_strerror`).

#![allow(non_snake_case)]

use std::collections::VecDeque;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use arcstr::ArcStr;
use metamodelica::List;
use openmodelica_util::Error;
use openmodelica_util::ErrorTypes;
use openmodelica_util::Gettext;

/// One in-flight `curl` process and the state needed to retry/cleanup.
struct Transfer {
    child: Child,
    url: ArcStr,
    /// Remaining mirror URLs to try if this one fails.
    next_try: Arc<List<ArcStr>>,
    filename: ArcStr,
    tmp_filename: String,
}

/// `c_add_message(NULL, -1, ErrorType_runtime, ErrorLevel_error, ...)`
/// equivalent: an ad-hoc runtime error with no source location.
fn add_error(template: &str, tokens: Vec<ArcStr>) -> Result<()> {
    let mut toks: Arc<List<ArcStr>> = Arc::new(List::Nil);
    for t in tokens.into_iter().rev() {
        toks = metamodelica::cons(t, toks);
    }
    Error::addMessage(
        ErrorTypes::Message {
            id: -1,
            ty: ErrorTypes::MessageType::SIMULATION,
            severity: ErrorTypes::Severity::ERROR,
            message: Gettext::TranslatableContent::notrans { r#str: ArcStr::from(template) },
        },
        toks,
    )
}

/// Start the next transfer from the queue, mirroring `addTransfer` in
/// `om_curl.c`. Returns `None` (and possibly flips `result` to false)
/// when the item could not be started.
fn start_transfer(
    queue: &mut VecDeque<(Arc<List<ArcStr>>, ArcStr)>,
    transfer_number: &mut i32,
    result: &mut bool,
) -> Result<Option<Transfer>> {
    let Some((urls, filename)) = queue.pop_front() else {
        return Ok(None);
    };
    let List::Cons { head: url, tail: next_try } = &*urls else {
        // A work item with no URLs at all; the C version would read past
        // the list end. Report it as a failed download instead.
        add_error("Curl error for URL %s: %s", vec![
            ArcStr::from(format!("(no URL given for {filename})")),
            ArcStr::from("empty mirror list"),
        ])?;
        *result = false;
        return Ok(None);
    };
    let n = *transfer_number;
    *transfer_number += 1;
    let tmp_filename = format!("{filename}.tmp{n}");

    // `om_curl.c` opens the output with fopen before handing it to curl and
    // reports failure as "Failed to open file for writing"; creating the
    // file up front gives us the same early diagnostic (e.g. for a missing
    // directory) with the same message.
    if let Err(e) = std::fs::File::create(&tmp_filename) {
        let _ = e; // the C message only includes the file name
        add_error("Failed to open file for writing: %s", vec![ArcStr::from(tmp_filename)])?;
        *result = false;
        return Ok(None);
    }

    let child = Command::new("curl")
        .arg("--location")            // CURLOPT_FOLLOWLOCATION
        .arg("--connect-timeout").arg("8") // CURLOPT_CONNECTTIMEOUT
        .arg("--fail")                // CURLOPT_FAILONERROR
        .arg("--silent").arg("--show-error") // no progress bar, errors on stderr
        .arg("--user-agent").arg("OpenModelica/1.0") // CURLOPT_USERAGENT
        .arg("--output").arg(&tmp_filename)
        .arg("--").arg(url.as_str())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn();

    match child {
        Ok(child) => Ok(Some(Transfer {
            child,
            url: url.clone(),
            next_try: next_try.clone(),
            filename,
            tmp_filename,
        })),
        Err(e) => {
            // curl itself is missing/not executable: every retry would fail
            // the same way, so report and drop the item.
            let _ = std::fs::remove_file(&tmp_filename);
            add_error("Curl error for URL %s: %s", vec![
                url.clone(),
                ArcStr::from(format!("failed to run curl: {e}")),
            ])?;
            *result = false;
            Ok(None)
        }
    }
}

/// Handle one finished transfer, mirroring the `CURLMSG_DONE` branch of
/// `om_curl_multi_download`.
fn finish_transfer(
    t: Transfer,
    queue: &mut VecDeque<(Arc<List<ArcStr>>, ArcStr)>,
    result: &mut bool,
) -> Result<()> {
    let mut child = t.child;
    let output = {
        // try_wait() already returned Some, so this does not block.
        let status = child.wait()?;
        let mut stderr = String::new();
        if let Some(mut pipe) = child.stderr.take() {
            use std::io::Read;
            let _ = pipe.read_to_string(&mut stderr);
        }
        (status, stderr)
    };
    let (status, stderr) = output;

    if status.success() {
        if let Err(e) = std::fs::rename(&t.tmp_filename, t.filename.as_str()) {
            add_error("Failed to rename file after downloading with curl %s %s: %s", vec![
                ArcStr::from(t.tmp_filename.clone()),
                t.filename.clone(),
                ArcStr::from(e.to_string()),
            ])?;
        }
        return Ok(());
    }

    // Failed download: remove the partial file and either retry the next
    // mirror or give up on this item.
    let _ = std::fs::remove_file(&t.tmp_filename);
    let err_text = ArcStr::from(stderr.trim().to_string());
    if matches!(&*t.next_try, List::Nil) {
        add_error("Curl error for URL %s: %s", vec![t.url.clone(), err_text])?;
        *result = false;
    } else {
        add_error("Will try another mirror due to curl error for URL %s: %s", vec![t.url.clone(), err_text])?;
        queue.push_front((t.next_try.clone(), t.filename.clone()));
    }
    Ok(())
}

pub fn multiDownload(
    urlFileList: Arc<List<(Arc<List<ArcStr>>, ArcStr)>>,
    maxParallel: i32,
) -> Result<bool> {
    let max_parallel = maxParallel.max(1) as usize;
    let mut queue: VecDeque<(Arc<List<ArcStr>>, ArcStr)> = VecDeque::new();
    {
        let mut cur = urlFileList;
        while let List::Cons { head, tail } = &*cur {
            queue.push_back(head.clone());
            let tail = tail.clone();
            cur = tail;
        }
    }
    let mut running: Vec<Transfer> = Vec::new();
    let mut transfer_number = 1;
    let mut result = true;

    while !queue.is_empty() || !running.is_empty() {
        // Top up the pool of concurrent transfers.
        while running.len() < max_parallel && !queue.is_empty() {
            if let Some(t) = start_transfer(&mut queue, &mut transfer_number, &mut result)? {
                running.push(t);
            }
        }

        // Reap finished transfers without blocking on any single one.
        let mut progressed = false;
        let mut i = 0;
        while i < running.len() {
            match running[i].child.try_wait() {
                Ok(Some(_)) => {
                    let t = running.swap_remove(i);
                    finish_transfer(t, &mut queue, &mut result)?;
                    progressed = true;
                }
                Ok(None) => i += 1,
                Err(e) => {
                    let t = running.swap_remove(i);
                    let _ = std::fs::remove_file(&t.tmp_filename);
                    add_error("Curl error for URL %s: %s", vec![
                        t.url.clone(),
                        ArcStr::from(format!("failed to wait for curl: {e}")),
                    ])?;
                    result = false;
                    progressed = true;
                }
            }
        }

        if !progressed && !running.is_empty() {
            // `om_curl.c` blocks in curl_multi_wait(..., 1000ms); a short
            // poll interval keeps the latency low without spinning.
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use metamodelica::{cons, nil};

    fn item(urls: &[&str], file: &str) -> (Arc<List<ArcStr>>, ArcStr) {
        let mut l = nil::<ArcStr>();
        for u in urls.iter().rev() {
            l = cons(ArcStr::from(*u), l);
        }
        (l, ArcStr::from(file))
    }

    /// Download via file:// URLs so the test runs without network access.
    #[test]
    fn downloads_and_retries_mirrors() {
        let dir = std::env::temp_dir().join(format!("curl_rs_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let src = dir.join("src.txt");
        std::fs::write(&src, "payload").unwrap();
        let src_url = format!("file://{}", src.display());
        let missing_url = format!("file://{}/does-not-exist", dir.display());

        let ok_target = dir.join("ok.txt");
        let retry_target = dir.join("retry.txt");
        let fail_target = dir.join("fail.txt");

        let list = cons(
            item(&[&src_url], &ok_target.display().to_string()),
            cons(
                // First mirror fails, second succeeds.
                item(&[&missing_url, &src_url], &retry_target.display().to_string()),
                cons(
                    // All mirrors fail.
                    item(&[&missing_url], &fail_target.display().to_string()),
                    nil(),
                ),
            ),
        );

        let success = multiDownload(list, 2).unwrap();
        assert!(!success, "one item has no working mirror");
        assert_eq!(std::fs::read_to_string(&ok_target).unwrap(), "payload");
        assert_eq!(std::fs::read_to_string(&retry_target).unwrap(), "payload");
        assert!(!fail_target.exists());
        // Temp files must not be left behind.
        for e in std::fs::read_dir(&dir).unwrap() {
            let name = e.unwrap().file_name().into_string().unwrap();
            assert!(!name.contains(".tmp"), "leftover temp file {name}");
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
}
