//! Block Viewer environment flags. Prefer `BLOCK_*`; silently accept legacy
//! `AURORA_*` so older scripts keep working without advertising them.

use std::ffi::OsString;

/// True when offline demo mode is requested (`BLOCK_DEMO` or legacy `AURORA_DEMO`).
pub fn demo_mode() -> bool {
    flag_set("BLOCK_DEMO") || flag_set("AURORA_DEMO")
}

/// Read `BLOCK_{suffix}` first, then `AURORA_{suffix}`.
pub fn var(suffix: &str) -> Option<OsString> {
    let block = format!("BLOCK_{suffix}");
    std::env::var_os(&block).filter(|v| !v.is_empty()).or_else(|| {
        let legacy = format!("AURORA_{suffix}");
        std::env::var_os(legacy).filter(|v| !v.is_empty())
    })
}

/// String form of [`var`].
pub fn var_string(suffix: &str) -> Option<String> {
    var(suffix).map(|v| v.to_string_lossy().into_owned())
}

fn flag_set(name: &str) -> bool {
    std::env::var_os(name).is_some_and(|v| {
        let t = v.to_string_lossy();
        let t = t.trim();
        !t.is_empty() && t != "0" && !t.eq_ignore_ascii_case("false")
    })
}
