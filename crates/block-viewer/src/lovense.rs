//! Lovense Connect / Remote local LAN API for Block Viewer.
//!
//! Talks to the user's Lovense app on this machine (HTTP `:20010` or HTTPS
//! `:30010`). Opt-in from Preferences › Fun › Lovense. Never phones home
//! to Block servers — commands stay on localhost / LAN.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Default PC Connect / Game Mode HTTP command URL.
pub const DEFAULT_HTTP: &str = "http://127.0.0.1:20010/command";
/// Default PC Connect HTTPS URL (`127-0-0-1.lovense.club`).
pub const DEFAULT_HTTPS: &str = "https://127-0-0-1.lovense.club:30010/command";

const PLATFORM: &str = "Block Viewer";
const TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Master switch — off by default.
    pub enabled: bool,
    /// POST target. Empty → try HTTP then HTTPS defaults.
    pub endpoint: String,
    /// Allow `/blocktoy …` nearby / IM self-commands.
    pub chat_commands: bool,
    /// Cap strength 0..=20 (Lovense vibrate scale).
    pub max_strength: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: String::new(),
            chat_commands: true,
            max_strength: 20,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // polled by upcoming UI / chat commands
pub struct Toy {
    pub id: String,
    pub name: String,
    pub nick: String,
    pub battery: Option<u8>,
    pub connected: bool,
}

#[derive(Debug, Default)]
struct State {
    toys: Vec<Toy>,
    last_error: Option<String>,
    last_ok: Option<Instant>,
    busy: bool,
}

/// Shared Lovense bridge (polled from the UI / chat commands).
#[derive(Clone, Default)]
pub struct Bridge {
    inner: Arc<Mutex<State>>,
}

impl Bridge {
    #[allow(dead_code)] // polled by upcoming UI / chat commands
    pub fn toys(&self) -> Vec<Toy> {
        self.inner.lock().map(|s| s.toys.clone()).unwrap_or_default()
    }

    #[allow(dead_code)]
    pub fn last_error(&self) -> Option<String> {
        self.inner.lock().ok().and_then(|s| s.last_error.clone())
    }

    #[allow(dead_code)]
    pub fn last_ok_ago(&self) -> Option<Duration> {
        self.inner.lock().ok().and_then(|s| s.last_ok.map(|t| t.elapsed()))
    }

    pub fn busy(&self) -> bool {
        self.inner.lock().map(|s| s.busy).unwrap_or(false)
    }

    fn set_busy(&self, busy: bool) {
        if let Ok(mut s) = self.inner.lock() {
            s.busy = busy;
        }
    }

    fn set_result(&self, toys: Option<Vec<Toy>>, err: Option<String>) {
        if let Ok(mut s) = self.inner.lock() {
            s.busy = false;
            if let Some(t) = toys {
                s.toys = t;
                s.last_ok = Some(Instant::now());
                s.last_error = None;
            }
            if let Some(e) = err {
                s.last_error = Some(e);
            }
        }
    }

    /// Refresh the toy list on a background thread.
    pub fn refresh(&self, settings: &Settings) {
        if !settings.enabled || self.busy() {
            return;
        }
        self.set_busy(true);
        let bridge = self.clone();
        let settings = settings.clone();
        std::thread::Builder::new()
            .name("block-lovense".into())
            .spawn(move || match command(&settings, json!({ "command": "GetToys" })) {
                Ok(body) => match parse_toys(&body) {
                    Ok(toys) => bridge.set_result(Some(toys), None),
                    Err(e) => bridge.set_result(None, Some(e)),
                },
                Err(e) => bridge.set_result(None, Some(e)),
            })
            .ok();
    }

    /// Vibrate (or Stop). `strength` is clamped to preferences.
    pub fn vibrate(&self, settings: &Settings, toy_id: Option<&str>, strength: u8, time_sec: f64) {
        if !settings.enabled {
            return;
        }
        let strength = strength.min(settings.max_strength).min(20);
        let action = if strength == 0 {
            "Stop".to_string()
        } else {
            format!("Vibrate:{strength}")
        };
        let mut body = json!({
            "command": "Function",
            "action": action,
            "timeSec": time_sec,
            "apiVer": 1,
            "stopPrevious": 1,
        });
        if let Some(id) = toy_id.filter(|s| !s.is_empty()) {
            body["toy"] = json!(id);
        }
        let bridge = self.clone();
        let settings = settings.clone();
        std::thread::Builder::new()
            .name("block-lovense-fn".into())
            .spawn(move || match command(&settings, body) {
                Ok(_) => {
                    if let Ok(mut s) = bridge.inner.lock() {
                        s.last_ok = Some(Instant::now());
                        s.last_error = None;
                    }
                }
                Err(e) => {
                    if let Ok(mut s) = bridge.inner.lock() {
                        s.last_error = Some(e);
                    }
                }
            })
            .ok();
    }

    pub fn stop_all(&self, settings: &Settings) {
        self.vibrate(settings, None, 0, 1.0);
    }
}

fn endpoints(settings: &Settings) -> Vec<String> {
    let custom = settings.endpoint.trim();
    if !custom.is_empty() {
        return vec![custom.to_string()];
    }
    vec![DEFAULT_HTTP.to_string(), DEFAULT_HTTPS.to_string()]
}

fn command(settings: &Settings, body: Value) -> Result<Value, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(TIMEOUT)
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| e.to_string())?;
    let mut last = "Lovense Connect did not answer.".to_string();
    for url in endpoints(settings) {
        match client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("X-platform", PLATFORM)
            .json(&body)
            .send()
        {
            Ok(resp) => {
                let status = resp.status();
                let text = resp.text().unwrap_or_default();
                if !status.is_success() {
                    last = format!("HTTP {status}: {text}");
                    continue;
                }
                return serde_json::from_str(&text).or_else(|_| Ok(json!({ "raw": text })));
            }
            Err(e) => last = e.to_string(),
        }
    }
    Err(last)
}

fn parse_toys(body: &Value) -> Result<Vec<Toy>, String> {
    let code = body.get("code").and_then(|c| c.as_i64()).unwrap_or(0);
    if code != 0 && code != 200 {
        return Err(format!("Lovense error code {code}"));
    }
    let toys_val = body
        .pointer("/data/toys")
        .cloned()
        .or_else(|| body.get("toys").cloned())
        .unwrap_or(Value::Null);
    let map: Value = match toys_val {
        Value::String(s) => serde_json::from_str(&s).unwrap_or(Value::Null),
        other => other,
    };
    let mut out = Vec::new();
    if let Some(obj) = map.as_object() {
        for (id, t) in obj {
            out.push(Toy {
                id: t.get("id").and_then(|v| v.as_str()).unwrap_or(id).to_string(),
                name: t.get("name").and_then(|v| v.as_str()).unwrap_or("toy").to_string(),
                nick: t.get("nickName").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                battery: t
                    .get("battery")
                    .and_then(|v| v.as_u64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
                    .map(|n| n.min(100) as u8),
                connected: t
                    .get("status")
                    .map(|v| match v {
                        Value::String(s) => s == "1" || s.eq_ignore_ascii_case("true"),
                        Value::Number(n) => n.as_u64() == Some(1),
                        Value::Bool(b) => *b,
                        _ => false,
                    })
                    .unwrap_or(true),
            });
        }
    }
    Ok(out)
}

/// Nearby / IM self-command: `/blocktoy vibrate [0-20] [seconds]` or `/blocktoy stop`.
pub fn parse_chat_command(line: &str) -> Option<ChatCmd> {
    let line = line.trim();
    let rest = line.strip_prefix("/blocktoy").or_else(|| line.strip_prefix("/lovense"))?.trim();
    if rest.is_empty() || rest.eq_ignore_ascii_case("help") {
        return Some(ChatCmd::Help);
    }
    let mut parts = rest.split_whitespace();
    let verb = parts.next()?.to_ascii_lowercase();
    match verb.as_str() {
        "stop" | "off" => Some(ChatCmd::Stop),
        "list" | "toys" | "status" => Some(ChatCmd::Refresh),
        "vibrate" | "vib" | "v" => {
            let strength = parts.next().and_then(|s| s.parse().ok()).unwrap_or(10u8);
            let secs = parts.next().and_then(|s| s.parse().ok()).unwrap_or(5.0f64);
            Some(ChatCmd::Vibrate { strength, secs })
        }
        _ => Some(ChatCmd::Help),
    }
}

#[derive(Debug, Clone)]
pub enum ChatCmd {
    Help,
    Stop,
    Refresh,
    Vibrate { strength: u8, secs: f64 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_verbs() {
        assert!(matches!(parse_chat_command("/blocktoy stop"), Some(ChatCmd::Stop)));
        assert!(matches!(
            parse_chat_command("/lovense vibrate 12 3"),
            Some(ChatCmd::Vibrate { strength: 12, secs: 3.0 })
        ));
    }

    #[test]
    fn toys_json_string() {
        let body = json!({
            "code": 200,
            "data": {
                "toys": "{\"abc\":{\"id\":\"abc\",\"name\":\"lush\",\"nickName\":\"\",\"status\":\"1\",\"battery\":80}}"
            }
        });
        let toys = parse_toys(&body).unwrap();
        assert_eq!(toys.len(), 1);
        assert_eq!(toys[0].name, "lush");
        assert_eq!(toys[0].battery, Some(80));
    }
}
