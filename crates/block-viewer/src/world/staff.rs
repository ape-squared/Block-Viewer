//! The staff Aurora marks: the Aurora team, with an aurora on their name
//! plate (`ui/nameplate/aurora.rs`), and Linden Lab's employees, with
//! twinkling stars and a running shine (`ui/nameplate/linden.rs`), on the
//! Second Life grids only (Agni and Aditi); each
//! with its tag next to their name in the lists, the chat and their
//! profile (`ui/staff.rs`). Only Aurora draws them: the team's list ships
//! with the viewer and nothing is sent to the grid. Aurora's own; the
//! Legacy Viewer only colours Linden names (`ui::colors::is_linden`).

use super::{World, block_staff};
use std::sync::atomic::{AtomicBool, Ordering};
use uuid::Uuid;

/// The agent is on a Second Life grid (Agni or Aditi): the only grids
/// whose "Linden" last name is Linden Lab's. Set by the app with the grid
/// it logged in to (`set_second_life`); off until then.
static SECOND_LIFE: AtomicBool = AtomicBool::new(false);

/// The grid of the session is Second Life's (Agni or Aditi), or not.
pub fn set_second_life(on: bool) {
    SECOND_LIFE.store(on, Ordering::Relaxed);
}

/// The team's avatar keys.
const TEAM: &[Uuid] = &[
    uuid::uuid!("348d7782-8438-4480-b482-608ca731b3a8"),
    uuid::uuid!("eb902302-0a1a-4f51-9ca6-7148822d9e95"),
    uuid::uuid!("322be05d-8851-4eaf-b4f2-a3347ebde023"),
    uuid::uuid!("2401c1a6-0dde-4daf-b60f-420473687744"),
    uuid::uuid!("d94d9882-b0bb-400b-9299-e51e021e921e"),
    uuid::uuid!("64043ec6-40a2-44f2-8b50-d88c82b00548"),
    uuid::uuid!("79e97f63-2410-43d0-aab2-92486a31b309"),
    uuid::uuid!("b62dc313-c65b-44dd-9553-5dc222143024"),
];

/// Block managers recognised by agent key (nameplate "Block Manager").
const MANAGER_IDS: &[Uuid] = &[
    // Fr3sh3r
    uuid::uuid!("4250c4e6-4819-4b0e-a20e-44a3509bc4c5"),
];

/// Block testers recognised by agent key (nameplate "Block Tester").
const TESTER_IDS: &[Uuid] = &[];

/// Block developers recognised by username (legacy or modern form).
const DEV_USERNAMES: &[&str] = &["heard"];

/// Block managers recognised by username.
const MANAGER_USERNAMES: &[&str] = &["fr3sh3r"];

/// Block testers recognised by username (seed; also in `peers/block-staff.json`).
const TESTER_USERNAMES: &[&str] = &["r1sk1kutta", "jr.alpha"];

/// Whose staff a resident is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mark {
    /// Block Viewer developer / team.
    Block,
    /// Block Viewer manager (community / ops).
    Manager,
    /// Block Viewer tester.
    Tester,
    /// The Aurora team.
    Aurora,
    /// A Linden Lab employee: the last name "Linden" (LLMuteList::isLinden),
    /// on the Second Life grids only (OpenSim names are anyone's).
    Linden,
}

/// Whether `id` is a known Block manager agent key.
pub fn is_manager(id: &Uuid) -> bool {
    !id.is_nil() && (MANAGER_IDS.contains(id) || block_staff::listed(block_staff::Role::Manager, Some(id), ""))
}

/// Whether `id` is a known Block tester agent key.
pub fn is_tester(id: &Uuid) -> bool {
    !id.is_nil() && (TESTER_IDS.contains(id) || block_staff::listed(block_staff::Role::Tester, Some(id), ""))
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

/// Whether `raw` is a known Block developer username.
pub fn is_block_dev_name(raw: &str) -> bool {
    let n = normalize_username(raw);
    DEV_USERNAMES.iter().any(|d| n == *d) || block_staff::listed(block_staff::Role::Staff, None, &n)
}

/// Whether `raw` is a known Block manager username.
pub fn is_block_manager_name(raw: &str) -> bool {
    let n = normalize_username(raw);
    MANAGER_USERNAMES.iter().any(|d| n == *d) || block_staff::listed(block_staff::Role::Manager, None, &n)
}

/// Whether `raw` is a known Block tester username.
pub fn is_block_tester_name(raw: &str) -> bool {
    let n = normalize_username(raw);
    TESTER_USERNAMES.iter().any(|d| n == *d) || block_staff::listed(block_staff::Role::Tester, None, &n)
}

/// Whether this avatar is Block team (by UUID and/or username).
pub fn is_block_team(world: &World, id: &Uuid) -> bool {
    if is_manager(id) || is_tester(id) || block_staff::listed(block_staff::Role::Staff, Some(id), "") {
        return true;
    }
    world.social.avatar_names.get(id).is_some_and(|n| {
        is_block_dev_name(&n.user_name())
            || is_block_dev_name(&n.legacy_first)
            || is_block_manager_name(&n.user_name())
            || is_block_manager_name(&n.legacy_first)
            || is_block_tester_name(&n.user_name())
            || is_block_tester_name(&n.legacy_first)
    })
}

/// Whether `id` is a member of the Aurora team.
pub fn is_staff(id: &Uuid) -> bool {
    !id.is_nil() && (TEAM.contains(id) || crate::demo::staff::is_staff(id))
}

/// The mark of `id`, if any: the Aurora team by key; Linden Lab by the
/// legacy name of the cached avatar name (a display name can say anything).
pub fn mark(world: &World, id: &Uuid) -> Option<Mark> {
    if is_manager(id)
        || world.social.avatar_names.get(id).is_some_and(|n| {
            is_block_manager_name(&n.user_name()) || is_block_manager_name(&n.legacy_first)
        })
    {
        return Some(Mark::Manager);
    }
    if is_tester(id)
        || world.social.avatar_names.get(id).is_some_and(|n| {
            is_block_tester_name(&n.user_name()) || is_block_tester_name(&n.legacy_first)
        })
    {
        return Some(Mark::Tester);
    }
    if is_block_team(world, id) {
        return Some(Mark::Block);
    }
    if is_staff(id) {
        return Some(Mark::Aurora);
    }
    let names = &world.social.avatar_names;
    let linden = SECOND_LIFE.load(Ordering::Relaxed)
        && names
            .get(id)
            .is_some_and(|n| super::mutes::is_linden(&n.user_name_for_display(&names.options)));
    (linden || crate::demo::staff::is_linden(id)).then_some(Mark::Linden)
}

/// The mark of a resident whose legacy name is `legacy` (the name plates,
/// which already have it).
pub fn mark_named(id: &Uuid, legacy: &str) -> Option<Mark> {
    mark_on(SECOND_LIFE.load(Ordering::Relaxed), id, legacy)
}

/// `mark_named` on a grid that is Second Life's (`second_life`) or not.
fn mark_on(second_life: bool, id: &Uuid, legacy: &str) -> Option<Mark> {
    if is_manager(id) || is_block_manager_name(legacy) {
        Some(Mark::Manager)
    } else if is_tester(id) || is_block_tester_name(legacy) {
        Some(Mark::Tester)
    } else if is_block_dev_name(legacy) || block_staff::listed(block_staff::Role::Staff, Some(id), "") {
        Some(Mark::Block)
    } else if is_staff(id) {
        Some(Mark::Aurora)
    } else if (second_life && super::mutes::is_linden(legacy)) || crate::demo::staff::is_linden(id) {
        Some(Mark::Linden)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nobody_is_staff_by_default() {
        assert!(!is_staff(&Uuid::nil()));
        assert!(!is_staff(&Uuid::from_u128(1)));
        assert!(TEAM.iter().all(|id| !id.is_nil()));
    }

    #[test]
    fn the_team_is_marked_by_key() {
        assert!(TEAM.iter().all(is_staff));
        for (i, id) in TEAM.iter().enumerate() {
            assert!(!TEAM[i + 1..].contains(id), "{id} twice");
            // whatever their name says, on any grid
            assert_eq!(mark_on(true, id, "Someone Linden"), Some(Mark::Aurora));
            assert_eq!(mark_on(false, id, "Someone Linden"), Some(Mark::Aurora));
        }
    }

    #[test]
    fn lindens_are_marked_by_their_last_name_on_second_life_only() {
        let id = Uuid::from_u128(7);
        assert_eq!(mark_on(true, &id, "Philip Linden"), Some(Mark::Linden));
        assert_eq!(mark_on(true, &id, "philip.linden"), Some(Mark::Linden));
        assert_eq!(mark_on(true, &id, "Lindenberg Resident"), None);
        assert_eq!(mark_on(true, &id, "Nova Example"), None);
        // an OpenSim or other grid: anyone may be named "Linden"
        assert_eq!(mark_on(false, &id, "Philip Linden"), None);
        assert_eq!(mark_on(false, &id, "philip.linden"), None);
    }
}
