// Manually written file.
//
// Rust port of `OMCompiler/Compiler/Util/Settings.mo`, whose every function
// is `external "C"` and resolves to the runtime in
// `OMCompiler/Compiler/runtime/settingsimpl.c` + `Settings_omc.cpp`.
//
// `Settings` holds a handful of process-global configuration values that omc
// (and the helper processes it spawns) consult: the compiler version string,
// the temp-directory path, the OpenModelica installation directory, the
// Modelica library search path, the user's home directory and an `echo`
// flag. In the C runtime these live in `static` variables that are shared by
// every thread without locking, because they are set during startup and then
// only read. We mirror that "set once, read often" model with a single
// process-global `Mutex` (these are cold-path config accesses, never a hot
// loop), which also makes the Rust port thread-safe where the C original
// merely relied on convention.
//
// Where the C runtime mutates these paths it also pushes them into the
// environment (`putenv`) so child processes — gcc, the simulation runtime,
// shell scripts — inherit `OPENMODELICAHOME` / `OPENMODELICALIBRARY`. We
// reproduce that via [`crate::System::setEnv`].

#![allow(non_snake_case)]

use std::sync::Mutex;

use anyhow::{Result, bail};
use arcstr::ArcStr;

use crate::Autoconf;

/// Compiler version string, the analogue of the C build's `CONFIG_VERSION`
/// (`Settings_getVersionNr` returns it verbatim). The C build injects the git
/// revision through `revision.h`; we let a build inject the same value via the
/// `OPENMODELICA_REVISION` environment variable at compile time and otherwise
/// fall back to the current development version. Defining it here keeps the
/// version next to the function that exposes it, matching `Settings_omc.cpp`.
const VERSION: &str = match option_env!("OPENMODELICA_REVISION") {
    Some(v) => v,
    None => "v1.27.0-dev",
};

/// Process-global settings, mirroring the `static` variables in
/// `settingsimpl.c`. `None` means "not yet computed / cleared"; the getters
/// lazily fill the cache exactly like the C functions do.
struct SettingsState {
    /// `tempDirectoryPath` in C.
    temp_directory_path: Option<ArcStr>,
    /// `echo` in C — initialised to 1 (true).
    echo: i32,
    /// `omc_installationPath` in C.
    installation_path: Option<ArcStr>,
    /// `omc_modelicaPath` in C.
    modelica_path: Option<ArcStr>,
    /// `omc_userHome` in C.
    user_home: Option<ArcStr>,
}

static STATE: Mutex<SettingsState> = Mutex::new(SettingsState {
    temp_directory_path: None,
    echo: 1,
    installation_path: None,
    modelica_path: None,
    user_home: None,
});

/// `covertToForwardSlashesInPlace` in C: on Windows the runtime rewrites
/// backslashes to forward slashes so the rest of the compiler only ever sees
/// `/`. On Unix it is a no-op. `Autoconf::isWindows` is a compile-time
/// constant, so the conversion is folded away entirely on Unix targets.
fn convert_to_forward_slashes(path: &str) -> ArcStr {
    if Autoconf::isWindows {
        ArcStr::from(path.replace('\\', "/"))
    } else {
        ArcStr::from(path)
    }
}

/// Push a path into the environment so spawned child processes inherit it,
/// mirroring `commonSetEnvVar` (which uses `putenv`, i.e. an overwriting set).
fn set_env_var(var: &str, value: &ArcStr) {
    crate::System::setEnv(ArcStr::from(var), value.clone(), true);
}

pub fn getVersionNr() -> ArcStr {
    ArcStr::from(VERSION)
}

pub fn setTempDirectoryPath(inString: ArcStr) {
    // `SettingsImpl__setTempDirectoryPath`: unconditionally replace the cached
    // path (the C version frees the previous string; we just drop it).
    STATE.lock().unwrap().temp_directory_path = Some(inString);
}

pub fn getTempDirectoryPath() -> ArcStr {
    // `SettingsImpl__getTempDirectoryPath`: lazily initialise from `$TMPDIR`,
    // defaulting to `/tmp`. (The Windows branch uses `GetTempPath`; we only
    // build for Unix here, and Autoconf::isWindows would gate a port of it.)
    let mut state = STATE.lock().unwrap();
    if let Some(p) = &state.temp_directory_path {
        return p.clone();
    }
    let path = match std::env::var("TMPDIR") {
        Ok(s) if !s.is_empty() => ArcStr::from(s),
        _ => arcstr::literal!("/tmp"),
    };
    state.temp_directory_path = Some(path.clone());
    path
}

pub fn setInstallationDirectoryPath(inString: ArcStr) {
    // `SettingsImpl__setInstallationDirectoryPath`: an empty string clears the
    // cache; otherwise store the (slash-normalised) path and export it.
    let mut state = STATE.lock().unwrap();
    if inString.is_empty() {
        state.installation_path = None;
        return;
    }
    let path = convert_to_forward_slashes(&inString);
    set_env_var("OPENMODELICAHOME", &path);
    state.installation_path = Some(path);
}

/// `stripbinpath` in C: given the path of the omc binary/library, repeatedly
/// drop the trailing path component until a `bin` or `lib` component has been
/// removed, yielding the installation prefix (e.g. `/opt/openmodelica/bin/omc`
/// → `/opt/openmodelica`). Errors out if the path contains neither `bin` nor
/// `lib`, matching the C runtime's hard failure.
fn strip_bin_path(path: &str) -> Result<ArcStr> {
    if !path.contains("bin") && !path.contains("lib") {
        bail!(
            "could not deduce the OpenModelica installation directory from \
             executable path: [{path}], please set OPENMODELICAHOME"
        );
    }
    let mut s = path.to_string();
    loop {
        match s.rfind('/') {
            Some(idx) => {
                let removed = s.split_off(idx); // removed starts with '/'
                if &removed[1..] == "bin" || &removed[1..] == "lib" {
                    break;
                }
            }
            // C asserts the slash exists; reaching the start without finding a
            // bin/lib component is the same unrecoverable situation.
            None => bail!(
                "could not deduce the OpenModelica installation directory from \
                 executable path: [{path}], please set OPENMODELICAHOME"
            ),
        }
    }
    Ok(ArcStr::from(s))
}

pub fn getInstallationDirectoryPath() -> Result<ArcStr> {
    // `SettingsImpl__getInstallationDirectoryPath` + the `Settings_omc.cpp`
    // wrapper, which throws (here: returns `Err`) when no path can be found.
    //
    // Resolution order:
    //   1. the cached value (set previously or by `setInstallationDirectoryPath`);
    //   2. `$OPENMODELICAHOME`, covering installed/configured environments —
    //      this is also what the C bootstrapping build reads;
    //   3. the directory derived from the running executable's own path via
    //      `strip_bin_path`, the portable analogue of the C runtime's
    //      `dladdr`-based lookup.
    {
        let state = STATE.lock().unwrap();
        if let Some(p) = &state.installation_path {
            return Ok(p.clone());
        }
    }

    if let Ok(env) = std::env::var("OPENMODELICAHOME") {
        if !env.is_empty() {
            let path = convert_to_forward_slashes(&env);
            let mut state = STATE.lock().unwrap();
            state.installation_path = Some(path.clone());
            return Ok(path);
        }
    }

    let exe = std::env::current_exe()
        .map_err(|e| anyhow::anyhow!("failed to determine executable path: {e}"))?;
    let exe = convert_to_forward_slashes(&exe.to_string_lossy());
    let path = strip_bin_path(&exe)?;

    let mut state = STATE.lock().unwrap();
    set_env_var("OPENMODELICAHOME", &path);
    state.installation_path = Some(path.clone());
    Ok(path)
}

pub fn setModelicaPath(inString: ArcStr) {
    // `SettingsImpl__setModelicaPath`: empty clears, otherwise store and export.
    let mut state = STATE.lock().unwrap();
    if inString.is_empty() {
        state.modelica_path = None;
        return;
    }
    let path = convert_to_forward_slashes(&inString);
    set_env_var("OPENMODELICALIBRARY", &path);
    state.modelica_path = Some(path);
}

pub fn getModelicaPath(runningTestsuite: bool) -> Result<ArcStr> {
    // `SettingsImpl__getModelicaPath`:
    //   - cached value wins;
    //   - else `$OPENMODELICALIBRARY` if set;
    //   - else, when running the testsuite, that env var is mandatory (the C
    //     runtime exits with an error message — we return `Err`);
    //   - else default to `<home>/.openmodelica/libraries/`.
    {
        let state = STATE.lock().unwrap();
        if let Some(p) = &state.modelica_path {
            return Ok(p.clone());
        }
    }

    let computed = match std::env::var("OPENMODELICALIBRARY") {
        Ok(env) if !env.is_empty() => ArcStr::from(env),
        _ => {
            if runningTestsuite {
                bail!("When using --running-testsuite, OPENMODELICALIBRARY must be set");
            }
            // `getHomeDir` locks `STATE` itself, so resolve it before re-locking.
            let home = getHomeDir(false);
            ArcStr::from(format!("{home}/.openmodelica/libraries/"))
        }
    };
    let path = convert_to_forward_slashes(&computed);

    let mut state = STATE.lock().unwrap();
    // Another thread may have populated the cache while we computed; honour it.
    if let Some(p) = &state.modelica_path {
        return Ok(p.clone());
    }
    if !runningTestsuite {
        set_env_var("OPENMODELICALIBRARY", &path);
    }
    state.modelica_path = Some(path.clone());
    Ok(path)
}

pub fn getHomeDir(runningTestsuite: bool) -> ArcStr {
    // `Settings_getHomeDir`: the testsuite must be insensitive to the real home
    // directory, so it always sees "". Otherwise read `$HOME`, caching the
    // result. (The C runtime additionally falls back to `getpwuid()` when
    // `$HOME` is unset; that path needs libc and is not ported — `$HOME` is
    // set in every environment we target. An unset `$HOME` yields "", matching
    // the C runtime's final fallback.)
    if runningTestsuite {
        return arcstr::literal!("");
    }
    let mut state = STATE.lock().unwrap();
    if let Some(p) = &state.user_home {
        return p.clone();
    }
    let home = match std::env::var("HOME") {
        Ok(s) if !s.is_empty() => convert_to_forward_slashes(&s),
        _ => arcstr::literal!(""),
    };
    state.user_home = Some(home.clone());
    home
}

pub fn getEcho() -> i32 {
    STATE.lock().unwrap().echo
}

pub fn setEcho(echo: i32) {
    STATE.lock().unwrap().echo = echo;
}

/* TODO: Implement an external C function for bootstrapped omc or remove me. DO NOT SIMPLY REMOVE THIS COMMENT
public function dumpSettings
  external "C" Settings_dumpSettings() annotation(Library = "omcruntime");
end dumpSettings;*/
