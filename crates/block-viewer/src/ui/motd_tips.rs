//! Loading-screen tips (MOTD-style), refreshed from GitHub without a rebuild.
//! Built-in JSON ships with the viewer; `main`'s `motd/tips.json` replaces it
//! when the download succeeds (same pattern as login news).

use serde::Deserialize;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

const MAX_TIP: usize = 280;
const MAX_TIPS: usize = 40;
const MAX_DOWNLOAD: usize = 64 * 1024;
const BUILTIN: &str = include_str!("../../../../motd/tips.json");

#[derive(Debug, Clone, Deserialize, Default)]
struct TipFile {
    #[serde(default)]
    tips: Vec<TipEntry>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct TipEntry {
    #[serde(default)]
    en: String,
    #[serde(default)]
    fr: String,
    #[serde(default)]
    es: String,
    #[serde(default)]
    de: String,
    #[serde(default)]
    it: String,
    #[serde(default)]
    #[serde(rename = "pt-BR")]
    pt_br: String,
    #[serde(default)]
    ja: String,
}

impl TipEntry {
    fn pick(&self, language: &str) -> Option<&str> {
        let base = language.split('-').next().unwrap_or(language);
        let candidates: &[&str] = match language {
            "pt-BR" | "pt_BR" => &[&self.pt_br, &self.en],
            _ if base.eq_ignore_ascii_case("fr") => &[&self.fr, &self.en],
            _ if base.eq_ignore_ascii_case("es") => &[&self.es, &self.en],
            _ if base.eq_ignore_ascii_case("de") => &[&self.de, &self.en],
            _ if base.eq_ignore_ascii_case("it") => &[&self.it, &self.en],
            _ if base.eq_ignore_ascii_case("ja") => &[&self.ja, &self.en],
            _ if base.eq_ignore_ascii_case("pt") => &[&self.pt_br, &self.en],
            _ => &[&self.en],
        };
        candidates.iter().copied().find(|s| !s.trim().is_empty())
    }
}

fn clip(text: &str) -> String {
    let clean: String = text.chars().filter(|c| !c.is_control() || *c == '\n').collect();
    let clean = clean.trim();
    if clean.chars().count() <= MAX_TIP {
        return clean.to_owned();
    }
    let mut cut: String = clean.chars().take(MAX_TIP.saturating_sub(1)).collect();
    cut.truncate(cut.trim_end().len());
    cut.push('…');
    cut
}

fn bounded(file: TipFile) -> Vec<String> {
    let language = block_i18n::language();
    file.tips
        .into_iter()
        .filter_map(|tip| tip.pick(language).map(clip))
        .filter(|s| !s.is_empty())
        .take(MAX_TIPS)
        .collect()
}

fn parse(raw: &str) -> Option<Vec<String>> {
    let file: TipFile = serde_json::from_str(raw).ok()?;
    let tips = bounded(file);
    (!tips.is_empty()).then_some(tips)
}

fn builtin() -> Vec<String> {
    parse(BUILTIN).unwrap_or_else(|| vec![block_i18n::tr!("common-loading-world").to_owned()])
}

struct Store {
    tips: Vec<String>,
    started: bool,
}

fn store() -> &'static Mutex<Store> {
    static STORE: OnceLock<Mutex<Store>> = OnceLock::new();
    STORE.get_or_init(|| {
        Mutex::new(Store {
            tips: cache().unwrap_or_else(builtin),
            started: false,
        })
    })
}

fn cache_path() -> PathBuf {
    crate::settings::cache_dir().join("motd-tips.json")
}

fn cache() -> Option<Vec<String>> {
    let path = cache_path();
    if std::fs::metadata(&path).ok()?.len() > MAX_DOWNLOAD as u64 {
        return None;
    }
    parse(&std::fs::read_to_string(path).ok()?)
}

fn save_cache(raw: &str) {
    let _ = std::fs::write(cache_path(), raw.as_bytes());
}

/// Kick a background refresh once per process (login / first load).
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
    let url = if let Ok(override_url) = std::env::var("AURORA_MOTD_URL") {
        let trimmed = override_url.trim().to_owned();
        if trimmed.is_empty() {
            return;
        }
        trimmed
    } else {
        let base = crate::links::MOTD_TIPS;
        if base.is_empty() {
            return;
        }
        base.to_owned()
    };
    std::thread::Builder::new()
        .name("motd-tips".into())
        .spawn(move || {
            let client = match reqwest::blocking::Client::builder()
                .user_agent(block_version::user_agent())
                .https_only(!url.starts_with("http://127.0.0.1") && !url.starts_with("http://localhost"))
                .timeout(Duration::from_secs(8))
                .build()
            {
                Ok(c) => c,
                Err(err) => {
                    log::debug!("motd tips client: {err}");
                    return;
                }
            };
            match client.get(&url).send().and_then(|r| r.error_for_status()).and_then(|r| r.text()) {
                Ok(text) if text.len() <= MAX_DOWNLOAD => {
                    if let Some(tips) = parse(&text) {
                        save_cache(&text);
                        if let Ok(mut guard) = store().lock() {
                            guard.tips = tips;
                        }
                        log::info!("Loading tips refreshed from {url}");
                    }
                }
                Ok(_) => log::warn!("motd tips ignored: response too large"),
                Err(err) => log::debug!("motd tips fetch: {err}"),
            }
        })
        .ok();
}

/// Tip for this frame of the loading screen (rotates slowly).
pub fn current(elapsed: f32) -> Arc<str> {
    current_indexed(elapsed).0
}

/// Tip text plus 1-based index and total count (Claude teleport cards).
pub fn current_indexed(elapsed: f32) -> (Arc<str>, usize, usize) {
    ensure_fresh();
    let tips = store().lock().map(|g| g.tips.clone()).unwrap_or_else(|_| builtin());
    if tips.is_empty() {
        return (Arc::from(""), 0, 0);
    }
    // ~8 s per tip so the eye can read it
    let i = ((elapsed / 8.0).floor() as usize) % tips.len();
    (Arc::from(tips[i].as_str()), i + 1, tips.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_tips_parse() {
        let tips = parse(BUILTIN).expect("builtin motd");
        assert!(!tips.is_empty());
        assert!(tips.iter().all(|t| t.chars().count() <= MAX_TIP));
    }
}
