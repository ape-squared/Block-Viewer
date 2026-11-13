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

1. **Website:** testers submit on [Tester signup](https://ape-squared.github.io/Block-Viewer/testers/) — a Cloudflare Worker commits `block-staff.json` + `block-peers.json` on `main` (or a GitHub issue with `<!-- block-tester-signup v1 -->` is closed by `.github/workflows/tester-signup.yml`).
2. **GitHub:** edit the file on `main` (or open a PR). Live clients pick it up on next launch (or next process start).
3. **Discord bot / Actions:** `repository_dispatch` type `tester-signup` with `{ "username": "…" }`, or `workflow_dispatch` on **Tester signup**. Same schema — the viewer only reads the raw JSON URL.

Local / CI override: set `BLOCK_STAFF_URL` (or `AURORA_STAFF_URL`) to a different JSON URL.

Worker source: `workers/tester-signup/`. Secret: `GITHUB_TOKEN` (fine-grained PAT, Contents read/write on this repo).
