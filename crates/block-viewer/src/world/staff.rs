//! Block team marks on name plates (`ui/nameplate`), lists, chat and
//! profiles (`ui/staff.rs`). Matching is local to the viewer — nothing is
//! sent to the grid. Linden Lab employees keep the star mark via the
//! "Linden" last name rule.

use super::World;
use std::sync::atomic::{AtomicBool, Ordering};
use uuid::Uuid;

/// The agent is on a Second Life grid: the only grids whose "Linden" last
/// name is Linden Lab's. Set by the app (`set_second_life`); off until then.
static SECOND_LIFE: AtomicBool = AtomicBool::new(false);

/// The grid of the session is Second Life's, or not.
pub fn set_second_life(on: bool) {
    SECOND_LIFE.store(on, Ordering::Relaxed);
}

/// Extra agent UUIDs for Block developers (optional; usernames below are enough).
const TEAM_IDS: &[Uuid] = &[];

/// Block moderators recognised by agent key (nameplate "Block Mod").
const MOD_IDS: &[Uuid] = &[
    // Fr3sh3r
    uuid::uuid!("4250c4e6-4819-4b0e-a20e-44a3509bc4c5"),
];

/// Whose staff a resident is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mark {
    /// Block Viewer developer / team.
    Block,
    /// Block Viewer moderator (community / support).
    Mod,
    /// A Linden Lab employee: the last name "Linden".
    Linden,
}

/// Whether `id` is a known Block team agent key.
pub fn is_staff(id: &Uuid) -> bool {
    !id.is_nil() && (TEAM_IDS.contains(id) || is_mod(id) || crate::demo::staff::is_staff(id))
}

/// Whether `id` is a known Block moderator agent key.
pub fn is_mod(id: &Uuid) -> bool {
    !id.is_nil() && MOD_IDS.contains(id)
}

/// Normalize an SL username / legacy name for team checks:
/// `Heard Resident`, `heard.resident`, `heard` → `heard`.
pub fn normalize_username(raw: &str) -> String {
    let mut s = raw.trim().to_ascii_lowercase().replace(' ', ".");
    while s.ends_with('.') {
        s.pop();
    }
    if let Some(stripped) = s.strip_suffix(".resident") {
        s = stripped.to_string();
    }
    s
}

/// Block developers recognised by username (legacy or modern form).
const DEV_USERNAMES: &[&str] = &["heard", "4krazy"];

/// Block moderators recognised by username.
const MOD_USERNAMES: &[&str] = &["fr3sh3r"];

/// Whether `raw` is a known Block developer username.
pub fn is_block_dev_name(raw: &str) -> bool {
    let n = normalize_username(raw);
    DEV_USERNAMES.iter().any(|d| n == *d)
}

/// Whether `raw` is a known Block moderator username.
pub fn is_block_mod_name(raw: &str) -> bool {
    let n = normalize_username(raw);
    MOD_USERNAMES.iter().any(|d| n == *d)
}

/// Whether this avatar is Block team (by UUID and/or username).
pub fn is_block_team(world: &World, id: &Uuid) -> bool {
    if is_staff(id) {
        return true;
    }
    world.social.avatar_names.get(id).is_some_and(|n| {
        is_block_dev_name(&n.user_name())
            || is_block_dev_name(&n.legacy_first)
            || is_block_mod_name(&n.user_name())
            || is_block_mod_name(&n.legacy_first)
    })
}

/// The mark of `id`, if any.
pub fn mark(world: &World, id: &Uuid) -> Option<Mark> {
    if is_mod(id)
        || world
            .social
            .avatar_names
            .get(id)
            .is_some_and(|n| is_block_mod_name(&n.user_name()) || is_block_mod_name(&n.legacy_first))
    {
        return Some(Mark::Mod);
    }
    if is_block_team(world, id) {
        return Some(Mark::Block);
    }
    let names = &world.social.avatar_names;
    let linden = SECOND_LIFE.load(Ordering::Relaxed)
        && names
            .get(id)
            .is_some_and(|n| super::mutes::is_linden(&n.user_name_for_display(&names.options)));
    (linden || crate::demo::staff::is_linden(id)).then_some(Mark::Linden)
}

/// The mark of a resident whose legacy name is already known (name plates).
pub fn mark_named(id: &Uuid, legacy: &str) -> Option<Mark> {
    if is_mod(id) || is_block_mod_name(legacy) {
        Some(Mark::Mod)
    } else if (is_staff(id) && !is_mod(id)) || is_block_dev_name(legacy) {
        Some(Mark::Block)
    } else if super::mutes::is_linden(legacy) || crate::demo::staff::is_linden(id) {
        Some(Mark::Linden)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heard_and_4krazy_are_block_devs() {
        for name in ["heard", "Heard", "heard.resident", "Heard Resident", "HEARD.RESIDENT"] {
            assert!(is_block_dev_name(name), "{name}");
            assert_eq!(mark_named(&Uuid::from_u128(1), name), Some(Mark::Block));
        }
        for name in ["4krazy", "4Krazy", "4krazy.resident", "4Krazy Resident"] {
            assert!(is_block_dev_name(name), "{name}");
            assert_eq!(mark_named(&Uuid::from_u128(2), name), Some(Mark::Block));
        }
        assert!(!is_block_dev_name("hearing"));
        assert!(!is_block_dev_name("heardish"));
        assert!(!is_block_dev_name("4krazyy"));
        assert_eq!(mark_named(&Uuid::from_u128(1), "Nova Example"), None);
    }

    #[test]
    fn fr3sh3r_is_block_mod() {
        let id = uuid::uuid!("4250c4e6-4819-4b0e-a20e-44a3509bc4c5");
        assert!(is_mod(&id));
        assert_eq!(mark_named(&id, "Someone Else"), Some(Mark::Mod));
        for name in ["fr3sh3r", "Fr3sh3r", "fr3sh3r.resident", "Fr3sh3r Resident"] {
            assert!(is_block_mod_name(name), "{name}");
            assert_eq!(mark_named(&Uuid::from_u128(9), name), Some(Mark::Mod));
        }
    }

    #[test]
    fn lindens_are_marked_by_their_last_name() {
        let id = Uuid::from_u128(7);
        assert_eq!(mark_named(&id, "Philip Linden"), Some(Mark::Linden));
        assert_eq!(mark_named(&id, "philip.linden"), Some(Mark::Linden));
        assert_eq!(mark_named(&id, "Lindenberg Resident"), None);
    }

    #[test]
    fn normalize_strips_resident() {
        assert_eq!(normalize_username("Heard Resident"), "heard");
        assert_eq!(normalize_username("heard.resident"), "heard");
    }
}
