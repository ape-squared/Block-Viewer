//! Remote Block staff / manager / tester roster for nameplates.
//!
//! Same idea as `block_peers`: a curated JSON on GitHub
//! (`peers/block-staff.json`) refreshes at runtime so Discord bots or
//! GitHub edits can grant marks without rebuilding the viewer.
//! Built-in seed + hardcoded fallbacks in `staff.rs` stay available offline.

use serde::Deserialize;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use uuid::Uuid;

use super::staff;

const MAX_DOWNLOAD: usize = 256 * 1024;
const BUILTIN: &str = include_str!("../../../../peers/block-staff.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Staff,
    Manager,
    Tester,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct RoleFile {
    #[serde(default)]
    usernames: Vec<String>,
    #[serde(default)]
    ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct StaffFile {
    #[serde(default)]
    staff: RoleFile,
    #[serde(default)]
    managers: RoleFile,
    #[serde(default)]
    testers: RoleFile,
}

#[derive(Debug, Default, Clone)]
struct RoleSet {
    ids: HashSet<Uuid>,
    names: HashSet<String>,
}

#[derive(Debug, Default)]
struct Roster {
    staff: RoleSet,
    managers: RoleSet,
    testers: RoleSet,
    started: bool,
}

fn store() -> &'static Mutex<Roster> {
    static STORE: OnceLock<Mutex<Roster>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(load_initial()))
}

fn cache_path() -> PathBuf {
    crate::settings::cache_dir().join("block-staff.json")
}

fn load_initial() -> Roster {
    let raw = std::fs::read_to_string(cache_path()).ok().filter(|t| t.len() <= MAX_DOWNLOAD);
    parse(raw.as_deref().unwrap_or(BUILTIN)).unwrap_or_else(|| parse(BUILTIN).unwrap_or_default())
}

fn fill_role(out: &mut RoleSet, file: RoleFile) {
    for id in file.ids {
        if let Ok(u) = Uuid::parse_str(id.trim()) {
            out.ids.insert(u);
        }
    }
    for name in file.usernames {
        let n = staff::normalize_username(&name);
        if !n.is_empty() {
            out.names.insert(n);
        }
    }
}

fn parse(raw: &str) -> Option<Roster> {
    let file: StaffFile = serde_json::from_str(raw).ok()?;
    let mut roster = Roster::default();
    fill_role(&mut roster.staff, file.staff);
    fill_role(&mut roster.managers, file.managers);
    fill_role(&mut roster.testers, file.testers);
    Some(roster)
}

fn save_cache(raw: &str) {
    let _ = std::fs::write(cache_path(), raw.as_bytes());
}

/// Refresh the staff roster once per process from GitHub (or `BLOCK_STAFF_URL`).
pub fn ensure_fresh() {
    if crate::block_env::demo_mode() || block_updater::test_environment().is_some() {
        return;
    }
    let Ok(mut guard) = store().lock() else {
        return;
    };
    if guard.started {
        return;
    }
    guard.started = true;
    drop(guard);
    let url = if let Ok(override_url) = std::env::var("BLOCK_STAFF_URL").or_else(|_| std::env::var("AURORA_STAFF_URL")) {
        let trimmed = override_url.trim().to_owned();
        if trimmed.is_empty() {
            return;
        }
        trimmed
    } else {
        let base = crate::links::BLOCK_STAFF;
        if base.is_empty() {
            return;
        }
        base.to_owned()
    };
    std::thread::Builder::new()
        .name("block-staff".into())
        .spawn(move || {
            let client = match reqwest::blocking::Client::builder()
                .user_agent(block_version::user_agent())
                .https_only(!url.starts_with("http://127.0.0.1") && !url.starts_with("http://localhost"))
                .timeout(Duration::from_secs(8))
                .build()
            {
                Ok(c) => c,
                Err(err) => {
                    log::debug!("block staff client: {err}");
                    return;
                }
            };
            match client.get(&url).send().and_then(|r| r.error_for_status()).and_then(|r| r.text()) {
                Ok(text) if text.len() <= MAX_DOWNLOAD => {
                    if let Some(set) = parse(&text) {
                        save_cache(&text);
                        if let Ok(mut guard) = store().lock() {
                            guard.staff = set.staff;
                            guard.managers = set.managers;
                            guard.testers = set.testers;
                        }
                        log::info!("Block staff roster refreshed from {url}");
                    }
                }
                Ok(_) => log::warn!("block staff ignored: response too large"),
                Err(err) => log::debug!("block staff fetch: {err}"),
            }
        })
        .ok();
}

fn role_set(roster: &Roster, role: Role) -> &RoleSet {
    match role {
        Role::Staff => &roster.staff,
        Role::Manager => &roster.managers,
        Role::Tester => &roster.testers,
    }
}

/// Whether the remote / cached roster lists this avatar under `role`.
pub fn listed(role: Role, id: Option<&Uuid>, legacy: &str) -> bool {
    ensure_fresh();
    let Ok(guard) = store().lock() else {
        return false;
    };
    let set = role_set(&guard, role);
    if let Some(id) = id
        && !id.is_nil()
        && set.ids.contains(id)
    {
        return true;
    }
    let n = staff::normalize_username(legacy);
    !n.is_empty() && set.names.contains(&n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_staff_parse() {
        let roster = parse(BUILTIN).expect("builtin staff");
        assert!(roster.staff.names.contains("heard"));
        assert!(roster.managers.names.contains("fr3sh3r"));
        assert!(roster.managers.ids.contains(&uuid::uuid!("4250c4e6-4819-4b0e-a20e-44a3509bc4c5")));
        assert!(roster.testers.names.contains("r1sk1kutta"));
        assert!(roster.testers.names.contains("jr.alpha"));
    }

    #[test]
    fn jr_alpha_normalizes_as_tester_username() {
        assert_eq!(staff::normalize_username("Jr.Alpha"), "jr.alpha");
        assert_eq!(staff::normalize_username("jr.alpha.resident"), "jr.alpha");
        assert_eq!(staff::normalize_username("Jr Alpha"), "jr.alpha");
    }
}
