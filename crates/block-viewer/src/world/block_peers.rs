//! Who else runs Block Viewer — for the nameplate badge.
//!
//! The grid does not broadcast viewer names to peers. Detection is:
//! a curated JSON list on GitHub (`peers/block-peers.json`), known staff /
//! mods, the user's own "advertise" preference for their plate, and — when
//! Advertise is on and the worn bridge is Ready — live region-local pings
//! through the query script (`<auroraBlock>` / `Query::PeerAnnounce`).

use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::staff;

const MAX_DOWNLOAD: usize = 256 * 1024;
const BUILTIN: &str = include_str!("../../../../peers/block-peers.json");
/// How long a live bridge ping keeps another avatar's badge lit.
const LIVE_TTL: Duration = Duration::from_secs(45);
/// How often we ask our bridge to RegionSay a presence ping.
#[allow(dead_code)]
pub const ANNOUNCE_INTERVAL: Duration = Duration::from_secs(12);

#[derive(Debug, Clone, Deserialize, Default)]
struct PeerFile {
    #[serde(default)]
    usernames: Vec<String>,
    #[serde(default)]
    ids: Vec<String>,
}

#[derive(Debug, Default)]
struct PeerSet {
    ids: HashSet<Uuid>,
    names: HashSet<String>,
    /// Avatars whose worn bridge recently announced Block presence.
    live: HashMap<Uuid, Instant>,
    started: bool,
}

fn store() -> &'static Mutex<PeerSet> {
    static STORE: OnceLock<Mutex<PeerSet>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(load_initial()))
}

fn cache_path() -> PathBuf {
    crate::settings::cache_dir().join("block-peers.json")
}

fn load_initial() -> PeerSet {
    let raw = std::fs::read_to_string(cache_path()).ok().filter(|t| t.len() <= MAX_DOWNLOAD);
    parse(raw.as_deref().unwrap_or(BUILTIN)).unwrap_or_default()
}

fn parse(raw: &str) -> Option<PeerSet> {
    let file: PeerFile = serde_json::from_str(raw).ok()?;
    let mut set = PeerSet::default();
    for id in file.ids {
        if let Ok(u) = Uuid::parse_str(id.trim()) {
            set.ids.insert(u);
        }
    }
    for name in file.usernames {
        let n = staff::normalize_username(&name);
        if !n.is_empty() {
            set.names.insert(n);
        }
    }
    Some(set)
}

fn save_cache(raw: &str) {
    let _ = std::fs::write(cache_path(), raw.as_bytes());
}

/// Refresh the peers list once per process from GitHub (or `AURORA_PEERS_URL`).
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
    let url = if let Ok(override_url) = std::env::var("AURORA_PEERS_URL") {
        let trimmed = override_url.trim().to_owned();
        if trimmed.is_empty() {
            return;
        }
        trimmed
    } else {
        let base = crate::links::BLOCK_PEERS;
        if base.is_empty() {
            return;
        }
        base.to_owned()
    };
    std::thread::Builder::new()
        .name("block-peers".into())
        .spawn(move || {
            let client = match reqwest::blocking::Client::builder()
                .user_agent(block_version::user_agent())
                .https_only(!url.starts_with("http://127.0.0.1") && !url.starts_with("http://localhost"))
                .timeout(Duration::from_secs(8))
                .build()
            {
                Ok(c) => c,
                Err(err) => {
                    log::debug!("block peers client: {err}");
                    return;
                }
            };
            match client.get(&url).send().and_then(|r| r.error_for_status()).and_then(|r| r.text()) {
                Ok(text) if text.len() <= MAX_DOWNLOAD => {
                    if let Some(set) = parse(&text) {
                        save_cache(&text);
                        if let Ok(mut guard) = store().lock() {
                            guard.ids = set.ids;
                            guard.names = set.names;
                        }
                        log::info!("Block peers list refreshed from {url}");
                    }
                }
                Ok(_) => log::warn!("block peers ignored: response too large"),
                Err(err) => log::debug!("block peers fetch: {err}"),
            }
        })
        .ok();
}

fn listed(id: &Uuid, legacy: &str) -> bool {
    ensure_fresh();
    let Ok(guard) = store().lock() else {
        return false;
    };
    if !id.is_nil() && guard.ids.contains(id) {
        return true;
    }
    let n = staff::normalize_username(legacy);
    !n.is_empty() && guard.names.contains(&n)
}

fn live(id: &Uuid, now: Instant) -> bool {
    let Ok(mut guard) = store().lock() else {
        return false;
    };
    guard.live.retain(|_, at| now.duration_since(*at) < LIVE_TTL);
    guard.live.get(id).is_some_and(|at| now.duration_since(*at) < LIVE_TTL)
}

/// A nearby Block bridge announced this avatar (OwnerSay `<auroraBlock>`).
#[allow(dead_code)]
pub fn note_live(id: Uuid, now: Instant) {
    if id.is_nil() {
        return;
    }
    if let Ok(mut guard) = store().lock() {
        guard.live.insert(id, now);
        if guard.live.len() > 256 {
            // Drop the oldest entries if the map grows without bound.
            let mut ages: Vec<_> = guard.live.iter().map(|(k, v)| (*k, *v)).collect();
            ages.sort_by_key(|(_, at)| *at);
            for (k, _) in ages.into_iter().take(64) {
                guard.live.remove(&k);
            }
        }
    }
}

/// Offline / unit-test helper: clear live detections.
#[cfg(test)]
pub fn clear_live() {
    if let Ok(mut guard) = store().lock() {
        guard.live.clear();
    }
}

/// Whether this avatar should get the Block Viewer nameplate badge.
pub fn is_block_viewer(world: &super::World, id: &Uuid, legacy: &str, advertise_self: bool) -> bool {
    if id.is_nil() {
        return false;
    }
    if *id == world.agent_id {
        return advertise_self;
    }
    if staff::is_block_team(world, id) || staff::is_block_dev_name(legacy) || staff::is_block_mod_name(legacy) {
        return true;
    }
    if live(id, Instant::now()) {
        return true;
    }
    listed(id, legacy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_peers_parse() {
        let set = parse(BUILTIN).expect("builtin peers");
        assert!(set.names.contains("heard"));
        assert!(set.ids.contains(&uuid::uuid!("4250c4e6-4819-4b0e-a20e-44a3509bc4c5")));
    }

    #[test]
    fn live_ping_marks_a_peer_until_it_expires() {
        clear_live();
        let id = uuid::uuid!("aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee");
        let now = Instant::now();
        note_live(id, now);
        assert!(live(&id, now));
        assert!(!live(&id, now + LIVE_TTL + Duration::from_secs(1)));
        clear_live();
    }
}
