//! Discord Rich Presence (Legacy Viewer FSDiscordConnect): local Discord IPC
//! shows that the resident is in-world, with optional name and region, and
//! keeps the activity synced while they move. Port of
//! `indra/newview/fsdiscordconnect.cpp` (originally LGPL 2.1).
//!
//! Application ID is public (Discord Developer Portal). The public key is
//! kept for Interactions later; Rich Presence IPC does not use it.

use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Discord application for Block Viewer Rich Presence.
pub const APP_ID: &str = "1566521197796135042";
/// Discord application public key (Interactions / OAuth verify; not used by IPC).
#[allow(dead_code)]
pub const PUBLIC_KEY: &str = "8fe32740737b1fd188e6adbfb2dd8f3513e9dc071584d94ed5373f77da24ce17";

/// Preferences › Privacy › Discord (FSEnableDiscordIntegration, FSShareNameToDiscord,
/// FSMaxSharedMaturity, FSBlacklistedRegionNames).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Settings {
    /// Automatically connect when in-world (FSEnableDiscordIntegration).
    pub enabled: bool,
    /// Put the avatar name in the activity details (FSShareNameToDiscord).
    pub share_name: bool,
    /// Highest SIM_ACCESS_* whose region name may be shared (0 = never).
    pub max_maturity: u8,
    /// Region names that stay hidden even when maturity allows them.
    pub blacklist: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: true,
            share_name: false,
            max_maturity: block_net::search::SIM_ACCESS_ADULT,
            blacklist: Vec::new(),
        }
    }
}

impl Settings {
    pub fn region_allowed(&self, name: &str, sim_access: u8) -> bool {
        if self.max_maturity == 0 || sim_access > self.max_maturity {
            return false;
        }
        let lower = name.to_ascii_lowercase();
        !self
            .blacklist
            .iter()
            .any(|b| b.eq_ignore_ascii_case(&lower) || b.to_ascii_lowercase() == lower)
    }
}

/// What the worker paints into Discord.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snapshot {
    pub details: String,
    pub state: String,
    pub start_unix: i64,
    pub party_id: String,
    pub party_size: i32,
    pub party_max: i32,
    pub grid_label: String,
}

enum Cmd {
    Set(Snapshot),
    Clear,
    Shutdown,
}

/// Background Discord IPC client; the frame loop only sends snapshots.
pub struct Presence {
    tx: Option<Sender<Cmd>>,
    worker: Option<JoinHandle<()>>,
    pub(crate) started_unix: i64,
    last_sent: Option<Snapshot>,
    last_attempt: Instant,
    connected_wanted: bool,
}

impl Default for Presence {
    fn default() -> Self {
        Self {
            tx: None,
            worker: None,
            started_unix: 0,
            last_sent: None,
            last_attempt: Instant::now() - Duration::from_secs(60),
            connected_wanted: false,
        }
    }
}

impl Drop for Presence {
    fn drop(&mut self) {
        self.shutdown();
    }
}

impl Presence {
    pub fn session_start(&mut self) {
        self.started_unix = unix_now();
    }

    /// Connect when enabled and in-world; clear / stop otherwise.
    pub fn sync(&mut self, prefs: &Settings, in_world: bool, snapshot: Option<Snapshot>) {
        let want = prefs.enabled && in_world;
        if !want {
            if self.connected_wanted {
                self.clear();
                self.connected_wanted = false;
            }
            return;
        }
        self.connected_wanted = true;
        if self.started_unix == 0 {
            self.session_start();
        }
        self.ensure_worker();
        let Some(snap) = snapshot else {
            return;
        };
        // Discord asks for at most ~1 update / 15 s unless the text changed.
        let changed = self.last_sent.as_ref() != Some(&snap);
        let due = self.last_attempt.elapsed() >= Duration::from_secs(15);
        if !changed && !due {
            return;
        }
        if let Some(tx) = &self.tx {
            let _ = tx.send(Cmd::Set(snap.clone()));
            self.last_sent = Some(snap);
            self.last_attempt = Instant::now();
        }
    }

    pub fn clear(&mut self) {
        self.last_sent = None;
        if let Some(tx) = &self.tx {
            let _ = tx.send(Cmd::Clear);
        }
    }

    fn ensure_worker(&mut self) {
        if self.tx.is_some() {
            return;
        }
        let (tx, rx) = mpsc::channel::<Cmd>();
        let started = self.started_unix;
        let worker = thread::Builder::new()
            .name("discord-presence".into())
            .spawn(move || worker_loop(rx, started))
            .ok();
        self.tx = Some(tx);
        self.worker = worker;
    }

    fn shutdown(&mut self) {
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(Cmd::Shutdown);
        }
        if let Some(h) = self.worker.take() {
            let _ = h.join();
        }
    }
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn worker_loop(rx: mpsc::Receiver<Cmd>, mut start: i64) {
    let mut client = DiscordIpcClient::new(APP_ID);
    let mut connected = false;
    let mut backoff = Instant::now() - Duration::from_secs(30);

    loop {
        let cmd = match rx.recv_timeout(Duration::from_secs(2)) {
            Ok(c) => c,
            Err(mpsc::RecvTimeoutError::Timeout) => continue,
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        };
        match cmd {
            Cmd::Shutdown => {
                if connected {
                    let _ = client.clear_activity();
                    let _ = client.close();
                }
                break;
            }
            Cmd::Clear => {
                if connected {
                    let _ = client.clear_activity();
                }
            }
            Cmd::Set(snap) => {
                if !connected {
                    if backoff.elapsed() < Duration::from_secs(5) {
                        continue;
                    }
                    backoff = Instant::now();
                    match client.connect() {
                        Ok(()) => {
                            connected = true;
                            start = snap.start_unix.max(start);
                            log::info!("discord: rich presence connected");
                        }
                        Err(e) => {
                            log::debug!("discord: connect failed ({e}); will retry");
                            continue;
                        }
                    }
                }
                if let Err(e) = set_activity(&mut client, &snap, start) {
                    log::debug!("discord: update failed ({e}); reconnecting");
                    let _ = client.close();
                    connected = false;
                }
            }
        }
    }
}

fn set_activity(client: &mut DiscordIpcClient, snap: &Snapshot, start: i64) -> Result<(), Box<dyn std::error::Error>> {
    let mut act = activity::Activity::new()
        .name("Block Viewer")
        .state(&snap.state)
        .timestamps(activity::Timestamps::new().start(start))
        .assets(
            activity::Assets::new()
                .large_image("block")
                .large_text("Block Viewer")
                .small_image("block")
                .small_text(&snap.grid_label),
        );
    if !snap.details.is_empty() {
        act = act.details(&snap.details);
    }
    if !snap.party_id.is_empty() && snap.party_max > 0 {
        act = act.party(
            activity::Party::new()
                .id(&snap.party_id)
                .size([snap.party_size.max(1), snap.party_max.max(snap.party_size.max(1))]),
        );
    }
    client.set_activity(act)?;
    Ok(())
}

/// Build the snapshot from the live world (Legacy Viewer updateRichPresence).
pub fn snapshot_from_world(
    prefs: &Settings,
    world: &crate::world::World,
    start_unix: i64,
    share_loc_ok: bool,
    share_name_ok: bool,
    party_size: i32,
    party_max: i32,
    grid_label: &str,
) -> Snapshot {
    let region = world.main_region.and_then(|h| world.regions.get(&h));
    let (name, sim_access, region_id) = region
        .map(|r| {
            let access = r.info.as_ref().map(|i| i.sim_access).unwrap_or(0);
            (r.name.clone(), access, format!("{:016x}", r.handle))
        })
        .unwrap_or_else(|| (String::new(), 0, String::new()));

    let state = if share_loc_ok && !name.is_empty() && prefs.region_allowed(&name, sim_access) {
        let pos = world.agent.position;
        format!("{name} ({:.0}, {:.0}, {:.0})", pos.x, pos.y, pos.z)
    } else if share_loc_ok {
        "Hidden Region".into()
    } else {
        "In world".into()
    };

    let details = if share_name_ok && prefs.share_name {
        let id = world.agent_id;
        world
            .social
            .avatar_names
            .get(&id)
            .map(|n| {
                let display = n.display_name.trim();
                let user = n.username.trim();
                if !display.is_empty() && !user.is_empty() && !display.eq_ignore_ascii_case(user) {
                    format!("{display} ({user})")
                } else if !display.is_empty() {
                    display.to_owned()
                } else if !user.is_empty() {
                    user.to_owned()
                } else {
                    world.social.name_of(&id)
                }
            })
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| world.social.name_of(&id))
    } else {
        String::new()
    };

    Snapshot {
        details,
        state,
        start_unix,
        party_id: region_id,
        party_size: party_size.max(1),
        party_max: party_max.max(party_size).max(1),
        grid_label: if grid_label.is_empty() {
            "Second Life".into()
        } else {
            grid_label.to_owned()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blacklist_and_maturity_gate_region_names() {
        let mut s = Settings::default();
        assert!(s.region_allowed("Plaza", block_net::search::SIM_ACCESS_PG));
        s.max_maturity = block_net::search::SIM_ACCESS_PG;
        assert!(!s.region_allowed("Club", block_net::search::SIM_ACCESS_ADULT));
        s.blacklist.push("secret".into());
        assert!(!s.region_allowed("Secret", block_net::search::SIM_ACCESS_PG));
        s.max_maturity = 0;
        assert!(!s.region_allowed("Plaza", block_net::search::SIM_ACCESS_PG));
    }
}
