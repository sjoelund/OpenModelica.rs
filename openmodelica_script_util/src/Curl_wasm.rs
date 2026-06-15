//! wasm stub for [`crate::Curl`]. The native module downloads files over native
//! libcurl (the `curl` crate, which pulls `curl-sys`/`socket2` — no wasm target).
//! A browser-wasm build would fetch via the host instead; until that is wired,
//! downloads report failure (callers treat a `false`/`Err` as "download failed").

use std::sync::Arc;

use anyhow::{Result, bail};
use arcstr::ArcStr;

use metamodelica::List;

pub fn multiDownload(
    _urlFileList: Arc<List<(Arc<List<ArcStr>>, ArcStr)>>,
    _maxParallel: i32,
) -> Result<bool> {
    bail!("Curl.multiDownload: network downloads (libcurl) are unavailable on wasm")
}
