//! Cached favourite landmarks for the login destination list
//! (feedback: Favorite Landmarks at Login). Written while in-world when the
//! favourites bar resolves region/SLURL; read on the login screen without
//! inventory.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const FILE: &str = "login_favorites.json";
const MAX: usize = 24;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub name: String,
    /// `Region/x/y/z` or maps.secondlife.com SLURL — anything [`crate::slurl::parse`] takes.
    pub place: String,
}

pub fn path(agent: &uuid::Uuid) -> PathBuf {
    crate::settings::account_dir(agent).join(FILE)
}

pub fn load(agent: Option<&uuid::Uuid>) -> Vec<Entry> {
    let Some(agent) = agent else {
        return Vec::new();
    };
    let Ok(bytes) = std::fs::read(path(agent)) else {
        return Vec::new();
    };
    serde_json::from_slice::<Vec<Entry>>(&bytes)
        .unwrap_or_default()
        .into_iter()
        .filter(|e| !e.name.trim().is_empty() && !e.place.trim().is_empty())
        .take(MAX)
        .collect()
}

/// Replace the cache when the favourites bar has resolved places.
pub fn save(agent: &uuid::Uuid, entries: &[Entry]) {
    let trimmed: Vec<Entry> = entries
        .iter()
        .filter(|e| !e.name.trim().is_empty() && !e.place.trim().is_empty())
        .take(MAX)
        .cloned()
        .collect();
    let Ok(bytes) = serde_json::to_vec_pretty(&trimmed) else {
        return;
    };
    let target = path(agent);
    if let Some(parent) = target.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = crate::atomic_file::write(&target, &bytes);
}

/// Build cache rows from the favourites currently shown in the bars.
pub fn from_view(list: &[crate::ui::bars::favorites::Favorite]) -> Vec<Entry> {
    list.iter()
        .filter_map(|f| {
            let place = f.slurl.clone().or_else(|| {
                (!f.region.is_empty()).then(|| {
                    if let Some(g) = f.global {
                        let x = g.x.rem_euclid(256.0).round() as i32;
                        let y = g.y.rem_euclid(256.0).round() as i32;
                        let z = g.z.round() as i32;
                        format!("{}/{x}/{y}/{z}", f.region)
                    } else {
                        f.region.clone()
                    }
                })
            })?;
            Some(Entry {
                name: f.name.clone(),
                place,
            })
        })
        .take(MAX)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_view_keeps_named_places() {
        let list = [crate::ui::bars::favorites::Favorite {
            item: uuid::Uuid::nil(),
            asset: uuid::Uuid::nil(),
            name: "Plaza".into(),
            region: "Welcome".into(),
            rating: "",
            global: Some(glam::DVec3::new(128.0, 128.0, 25.0)),
            slurl: None,
            thumbnail: None,
        }];
        let entries = from_view(&list);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "Plaza");
        assert!(entries[0].place.contains("Welcome"));
    }
}
