//! Medal.tv auto-clipping: when a moment happens, POST a small event to the
//! Medal desktop app on the local machine so it saves a clip from its replay
//! buffer. See https://medal.tv/portal/docs/auto-clipping/api
//!
//! Medal must be running and recording; the viewer never captures video
//! itself. The public key comes from the Medal developer portal for this
//! game (Preferences › Privacy › Medal.tv, or `AURORA_MEDAL_PUBLIC_KEY`).

use serde::{Deserialize, Serialize};
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Local Medal Auto Clipping listener (loopback only).
pub const ENDPOINT: &str = "http://127.0.0.1:12665/api/v1/event/invoke";

/// Preferences › Privacy › Medal.tv.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Settings {
    /// Send clip events to Medal when asked (hotkey / photo).
    pub enabled: bool,
    /// Game public key from the Medal Auto Clipping portal (header `publicKey`).
    pub public_key: String,
    /// Portal event id as a string (`"1"`).
    pub event_id: String,
    /// Clip title shown in the Medal library.
    pub event_name: String,
    /// Total clip length in seconds (typically 15–60).
    pub duration_secs: u32,
    /// Milliseconds to wait after the event before Medal saves (post-roll).
    pub capture_delay_ms: u32,
    /// Also fire a clip when Snapshot to Disk succeeds.
    pub clip_on_photo: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: false,
            public_key: String::new(),
            event_id: "1".into(),
            event_name: "Block Viewer clip".into(),
            duration_secs: 30,
            capture_delay_ms: 0,
            clip_on_photo: false,
        }
    }
}

impl Settings {
    /// Effective public key: preference, else `AURORA_MEDAL_PUBLIC_KEY`.
    pub fn public_key(&self) -> String {
        let trimmed = self.public_key.trim();
        if !trimmed.is_empty() {
            return trimmed.to_owned();
        }
        std::env::var("AURORA_MEDAL_PUBLIC_KEY")
            .ok()
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .unwrap_or_default()
    }

    pub fn ready(&self) -> bool {
        self.enabled && !self.public_key().is_empty() && !self.event_id.trim().is_empty()
    }
}

#[derive(Debug, Clone)]
struct Invoke {
    public_key: String,
    event_id: String,
    event_name: String,
    duration_secs: u32,
    capture_delay_ms: u32,
    screenshot: bool,
}

enum Cmd {
    Invoke(Invoke),
    Shutdown,
}

/// Background Medal client; the frame loop only queues clip requests.
#[derive(Default)]
pub struct Client {
    tx: Option<Sender<Cmd>>,
    worker: Option<JoinHandle<()>>,
}

impl Drop for Client {
    fn drop(&mut self) {
        self.shutdown();
    }
}

impl Client {
    fn ensure_worker(&mut self) {
        if self.tx.is_some() {
            return;
        }
        let (tx, rx) = mpsc::channel::<Cmd>();
        let worker = thread::Builder::new()
            .name("medal-tv".into())
            .spawn(move || {
                let http = match reqwest::blocking::Client::builder().timeout(Duration::from_secs(3)).build() {
                    Ok(c) => c,
                    Err(e) => {
                        log::warn!("medal: HTTP client failed ({e})");
                        return;
                    }
                };
                while let Ok(cmd) = rx.recv() {
                    match cmd {
                        Cmd::Shutdown => break,
                        Cmd::Invoke(req) => post_event(&http, &req),
                    }
                }
            })
            .ok();
        if worker.is_some() {
            self.tx = Some(tx);
            self.worker = worker;
        }
    }

    pub fn shutdown(&mut self) {
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(Cmd::Shutdown);
        }
        if let Some(h) = self.worker.take() {
            let _ = h.join();
        }
    }

    /// Queue a SaveClip (or screenshot) for the current preferences.
    pub fn save_clip(&mut self, prefs: &Settings, screenshot: bool) -> Result<(), &'static str> {
        if !prefs.enabled {
            return Err("medal-disabled");
        }
        let key = prefs.public_key();
        if key.is_empty() {
            return Err("medal-no-key");
        }
        let event_id = prefs.event_id.trim();
        if event_id.is_empty() {
            return Err("medal-no-event");
        }
        self.ensure_worker();
        let Some(tx) = &self.tx else {
            return Err("medal-worker");
        };
        let duration = prefs.duration_secs.clamp(5, 120);
        let req = Invoke {
            public_key: key,
            event_id: event_id.to_owned(),
            event_name: {
                let n = prefs.event_name.trim();
                if n.is_empty() {
                    "Block Viewer clip".into()
                } else {
                    n.chars().take(40).collect()
                }
            },
            duration_secs: duration,
            capture_delay_ms: prefs.capture_delay_ms.min(60_000),
            screenshot,
        };
        tx.send(Cmd::Invoke(req)).map_err(|_| "medal-worker")
    }
}

fn post_event(http: &reqwest::blocking::Client, req: &Invoke) {
    let action = if req.screenshot { "SaveScreenshot" } else { "SaveClip" };
    let body = request_body(
        &req.event_id,
        &req.event_name,
        req.duration_secs,
        req.capture_delay_ms,
        req.screenshot,
    );
    match http
        .post(ENDPOINT)
        .header("Content-Type", "application/json; charset=utf-8")
        .header("publicKey", &req.public_key)
        .json(&body)
        .send()
    {
        Ok(resp) => {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(code) = v.get("errorCode").and_then(|c| c.as_str()) {
                    log::warn!("medal: {code} ({status})");
                    return;
                }
                if v.get("success").and_then(|s| s.as_bool()) == Some(true) {
                    let actions = v.get("actionsCompleted").and_then(|a| a.as_array()).map(|a| a.len()).unwrap_or(0);
                    log::info!("medal: accepted ({action}, actionsCompleted={actions})");
                    return;
                }
            }
            log::warn!("medal: unexpected response {status}: {text}");
        }
        Err(e) if e.is_connect() => {
            log::debug!("medal: not running ({e})");
        }
        Err(e) => log::warn!("medal: request failed ({e})"),
    }
}

/// Build the JSON body the same way the worker does (unit-tested).
pub fn request_body(event_id: &str, event_name: &str, duration: u32, delay_ms: u32, screenshot: bool) -> serde_json::Value {
    let action = if screenshot { "SaveScreenshot" } else { "SaveClip" };
    serde_json::json!({
        "eventId": event_id,
        "eventName": event_name,
        "triggerActions": [action],
        "clipOptions": {
            "duration": duration,
            "captureDelayMs": delay_ms,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_request_matches_medal_api_shape() {
        let body = request_body("1", "Block Viewer clip", 30, 0, false);
        assert_eq!(body["eventId"], "1");
        assert_eq!(body["eventName"], "Block Viewer clip");
        assert_eq!(body["triggerActions"][0], "SaveClip");
        assert_eq!(body["clipOptions"]["duration"], 30);
        assert_eq!(body["clipOptions"]["captureDelayMs"], 0);
    }

    #[test]
    fn screenshot_action_and_ready_need_key() {
        let body = request_body("2", "Shot", 15, 2000, true);
        assert_eq!(body["triggerActions"][0], "SaveScreenshot");
        let mut s = Settings::default();
        assert!(!s.ready());
        s.enabled = true;
        s.public_key = "test-key".into();
        assert!(s.ready());
        assert_eq!(s.public_key(), "test-key");
    }
}
