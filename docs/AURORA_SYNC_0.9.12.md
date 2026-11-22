# Aurora Viewer v0.9.12 → Block Viewer sync

Block is ~549 Aurora commits behind `v0.9.12` (on top of the prior `v0.9.4` sync). A direct `git merge v0.9.12` is not viable: Git reports **1,322** conflicts and stages duplicate `crates/aurora-*` next to `crates/block-*`.

## Strategy (same as [PR #41](https://github.com/ape-squared/Block-Viewer/pull/41))

1. Export Aurora tag `v0.9.12` with `git archive`.
2. Rename `crates/aurora-*` → `crates/block-*` and rewrite crate identifiers in sources.
3. Copy the tree into Block (excluding `.git` / `target`).
4. Restore Block branding from the pre-sync branch (see script `$KeepFromOurs`).
5. Re-apply Block-only commits/features that are not in Aurora.
6. Replace user-visible **Aurora** product strings with **Block** (keep Aurora attribution in `NOTICE.md` / `LICENSE.md` only).
7. `scripts/check.ps1` or at least `cargo check -p block-viewer`, then PR.

Run:

```powershell
git fetch upstream tag v0.9.12
git checkout -B sync/aurora-0.9.12 origin/main
.\scripts\sync-aurora-tag.ps1 v0.9.12
```

## Aurora 0.9.12 feature highlights (release notes)

Not an exhaustive list — see [Aurora v0.9.12](https://github.com/Aurora-Viewer/Aurora-Viewer/releases/tag/v0.9.12).

| Area | Feature |
|------|---------|
| Avatar | Animation editor BVH/couple mode; Game Controls; prim-water swim; local avatar cache |
| Inventory | Gallery + combination views; notecard find/replace |
| Chat | Cross-device DM history (grid) |
| Audio/voice | Close mic while muted; camera vs avatar hear point; bad-cert radio warning |
| UI | Custom `.ftl` language file; in-viewer SL account creation; second camera window; Phototools window; Accessibility glass toggle; env list A–Z; many new locales |
| Weather | Redesigned weather window, main/card switches, breeze wind (#4277, #4313, #4321) |
| Render | Live world filters + photo LUTs (JuicyBomb pack) |
| Updater | Linux in-place updates |
| Fixes | Large batch of RLVa, FSR jitter, GPU recovery, estate/EEP weather, portals, mesh upload, etc. |

## Block gaps before sync (grep vs Aurora `v0.9.12`)

These paths exist on Aurora but not on Block `main` today:

- Game Controls (`game_control`)
- Second camera window (`second_camera`)
- Full Phototools UI (`PhototoolsUi`)
- Redesigned weather environment window (`weather_window` — partial weather GPU port only on 0.2.29)
- In-viewer account creation (`create_account`)
- Inventory combination view (`combination_view`)

Sync should bring all of the above in code; branding pass renames “Aurora Weather” → “Weather” or “Block Weather” in menus.

## Branding keep list

- `assets/branding/block/**`
- `LICENSE.md`, `NOTICE.md`, `TRADEMARKS.md`, `README.md` (Block copy)
- `news/*`, `peers/*`, `.github/release-notes-header.md`
- `update-policy.json` (merge with care)

## After sync

- Re-validate Block-only UX: pie menus, boxed nameplates, conversation log, tester peers, auto-reply wording.
- Run demo + grid smoke tests called out in PR #41.
- Version: propose `0.3.0` once stable (0.2.x retained Block line; 0.9.12 parity is a major capability jump).