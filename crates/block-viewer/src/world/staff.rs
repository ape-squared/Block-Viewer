//! Block team marks on name plates (`ui/nameplate`), lists, chat and
//! profiles (`ui/staff.rs`). Matching is local to the viewer — nothing is
//! sent to the grid. Linden Lab employees keep the star mark via the
//! "Linden" last name rule.

use super::World;
use uuid::Uuid;

/// Extra agent UUIDs for Block developers (optional; usernames below are enough).
const TEAM_IDS: &[Uuid] = &[];

/// Whose staff a resident is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mark {
    /// Block Viewer developer / team.
    Block,
    /// A Linden Lab employee: the last name "Linden".
    Linden,
}

/// Whether `id` is a known Block team agent key.
pub fn is_staff(id: &Uuid) -> bool {
    !id.is_nil() && (TEAM_IDS.contains(id) || crate::demo::staff::is_staff(id))
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

/// The only Block developer for now: Heard (`heard` / `heard.resident`).
pub fn is_block_dev_name(raw: &str) -> bool {
    normalize_username(raw) == "heard"
}

/// Whether this avatar is Block team (by UUID and/or username).
pub fn is_block_team(world: &World, id: &Uuid) -> bool {
    if is_staff(id) {
        return true;
    }
    world
        .social
        .avatar_names
        .get(id)
        .is_some_and(|n| is_block_dev_name(&n.user_name()) || is_block_dev_name(&n.legacy_first))
}

/// The mark of `id`, if any.
pub fn mark(world: &World, id: &Uuid) -> Option<Mark> {
    if is_block_team(world, id) {
        return Some(Mark::Block);
    }
    let names = &world.social.avatar_names;
    let linden = names
        .get(id)
        .is_some_and(|n| super::mutes::is_linden(&n.user_name_for_display(&names.options)));
    (linden || crate::demo::staff::is_linden(id)).then_some(Mark::Linden)
}

/// The mark of a resident whose legacy name is already known (name plates).
pub fn mark_named(id: &Uuid, legacy: &str) -> Option<Mark> {
    if is_staff(id) || is_block_dev_name(legacy) {
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
    fn heard_is_block_dev() {
        for name in ["heard", "Heard", "heard.resident", "Heard Resident", "HEARD.RESIDENT"] {
            assert!(is_block_dev_name(name), "{name}");
            assert_eq!(mark_named(&Uuid::from_u128(1), name), Some(Mark::Block));
        }
        assert!(!is_block_dev_name("hearing"));
        assert!(!is_block_dev_name("heardish"));
        assert_eq!(mark_named(&Uuid::from_u128(1), "Nova Example"), None);
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
