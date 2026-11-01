# Aurora upstream bug triage (Block Viewer)

Block Viewer tracks coding-ready fixes from [Aurora-Viewer/Aurora-Viewer](https://github.com/Aurora-Viewer/Aurora-Viewer) issues (GitHub type **Bug**). Open count upstream is large (~225); this build lands Category A Windows/logic fixes, not GPU/Linux/platform storms.

## Landed in this Block build

| Upstream | Title | Notes |
|----------|-------|-------|
| #1776 | Panic in `suggest::after` | Cursor indices clamped |
| #1775 | Rezzing ignores active group | `group_id` = `world.groups.active` |
| #1849 | Remove from outfit no-op | Allow wearable folders without `FT_OUTFIT` |
| #1780 | `/me` raw in toast | Emote formatting in notification excerpt |
| #1815 | Movement toolbar opens camera | `sync_tab` prefers sole movement flag |
| #1812 | Experience temp attach blocks Wear | Skip `missing_items` in outfit plan + filter |
| #1784 | Stretch Textures unchecked | TE repeats scale with prim size when off |
| #1824 / #1761 | AO window stretches at UI scale | AO floater `steady_size()` |

## Next pass (still Category A)

| Upstream | Title | Likely files |
|----------|-------|--------------|
| #1739 / #1735 | Detach then reattach (RLVa) | `appearance.rs` plan / sync |
| #1789 | Prefs overlap when narrow | `ui/options/**` |
| #1737 | Notes missing on own profile | `ui/profile.rs` |
| #1773 | Dupe anim-for-everyone window | anim play UI |
| #1741 | Numpad Home/PgUp/PgDn | keybind logical keys vs numpad |
| #1637 | Quick Jump ignored | movement prefs |

## Deferred (B/C — need repro, GPU, or platform)

GPU device lost / stall storms, Linux Wayland freezes, media CEF blank pages on Linux, BOM PBR NVIDIA, teleports to wrong spot, land ACL add (caps), Mac dictation, multi-window destabilization, feature requests (clear all notifications, local texture auto-refresh, controllers).
