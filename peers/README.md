# Peer and staff lists

These JSON files are fetched at runtime from `main` (see `links.rs`).
Edit them on GitHub — **no viewer rebuild** for new peers or nameplates.

| File | Effect |
| --- | --- |
| `block-peers.json` | Block Viewer peer badge on nameplates |
| `block-staff.json` | Staff / Manager / Tester nameplate marks |

## `block-staff.json`

```json
{
  "staff": { "usernames": ["heard"], "ids": [] },
  "managers": { "usernames": ["fr3sh3r"], "ids": ["…"] },
  "testers": { "usernames": ["r1sk1kutta", "jr.alpha"], "ids": [] }
}
```

- Usernames: legacy or modern (`jr.alpha`, `Jr Alpha`, `jr.alpha.resident` all match).
- IDs: agent UUIDs (optional; useful when the display name changes).
- Role priority when someone is listed twice: **Manager > Tester > Staff**.

## Updating without a rebuild

1. **GitHub:** edit the file on `main` (or open a PR). Live clients pick it up on next launch (or next process start).
2. **Discord bot:** commit an updated `peers/block-staff.json` through the GitHub Contents API to `main`. Same schema — the viewer only reads the raw JSON URL.

Local / CI override: set `BLOCK_STAFF_URL` (or `AURORA_STAFF_URL`) to a different JSON URL.
