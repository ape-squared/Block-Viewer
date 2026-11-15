# 02 — Kick streaming platform setup (research)

**Official starting point:** [Kick Help Center](https://help.kick.com/)

**Safety rule for all handbook copy:** Never show a real stream key. Always use `••••••••` / `REDACTED`. Tell students that a leaked key = someone else can stream to their channel until they reset it.

**[NEEDS CONFIRMATION]** Help Center article paths and Creator Dashboard URLs change. Re-verify the week of class and paste current deep links into the final handbook.

---

## 1. What Kick is (plain English)

Kick is a live streaming platform. Viewers watch on kick.com; creators send video from OBS (or similar) using a **stream URL + stream key**. Chat, categories, titles, and moderation live in Kick’s web/app UI — not inside Block Viewer.

---

## 2. Creating a Kick account

### Steps (generic — match current Help articles)

1. Go to https://kick.com/ (or the signup URL linked from Help).
2. Sign up with email or supported OAuth (Google, etc. — **as offered at signup time**).
3. Verify email if prompted.
4. Complete basic profile (username — choose carefully; renaming may be limited).

**Practical:** Username becomes part of the channel URL. Avoid joke names you’ll hate on a resume thumbnail.

**Security:** Unique password + enable 2FA / authenticator if Kick offers it in account settings. Don’t reuse Discord/Steam passwords.

**Common mistake:** Streaming before verifying email / accepting creator terms — dashboard features may be locked.

---

## 3. Creator profile setup

Students should fill:

| Field | Guidance |
| --- | --- |
| Display name | Readable brand name |
| Bio | What they stream (e.g. Second Life / Block Viewer builds, hangouts) |
| Profile & offline banner | Clear face or branded art; readable at small size |
| Links | Discord, socials (optional) |

**Why:** First impression when someone clicks from raid/search before you are live.

**SHOT:** Profile edit page with personal info blurred if needed.

---

## 4. Creator Dashboard (concepts)

After logging in as a creator, the **Creator Dashboard** (wording may be “Creator Dashboard”, “Studio”, or similar) typically exposes:

- Go Live / stream controls
- Stream title, category, language, tags
- Stream key / encoder settings
- Chat moderation tools
- VOD / clips (if enabled for the account)
- Analytics (later)

**Practical class walk:** Open dashboard → find Stream → copy **server** and **key** only into OBS → never into Discord.

**SHOT:** Dashboard overview; crop out earnings / private IDs.

---

## 5. Stream settings & stream keys

### Finding the key (pattern)

1. Dashboard → Stream / Settings / Encoder (exact labels vary).
2. Reveal or copy **Stream Key**.
3. Copy **Ingest / Server URL** (often RTMPS).

### Connecting OBS (verified pattern for custom services)

1. OBS → Settings → Stream.
2. Service: **Custom…** (unless Kick appears as a listed service in the student’s OBS version — prefer whatever Kick’s current Help article says).
3. Server: paste Kick ingest URL.
4. Stream Key: paste key.
5. Apply → OK.

**Docs cross-check:** OBS streaming settings — https://obsproject.com/kb/streaming-settings  
**Kick:** current Help article for OBS / stream key.

### Key hygiene

- Treat like a password.
- If leaked: **reset/regenerate** key in Kick dashboard immediately; update OBS.
- Don’t screenshot keys into class Discord; instructors should use redacted examples only.
- Don’t store keys in public GitHub gists or handbook drafts.

---

## 6. Titles, categories, tags

Before / when going live:

| Field | Beginner tips |
| --- | --- |
| Title | Specific + searchable (“Building in SL with Block Viewer — chill creative”) |
| Category | Closest accurate game/IRL category Kick offers for Second Life / Just Chatting / Creative — **[NEEDS CONFIRMATION]** current category taxonomy |
| Tags | Few relevant tags; avoid tag stuffing |
| Language | Match spoken language |

**Why:** Discovery and accurate expectations. Misleading category = frustrated viewers + possible enforcement.

---

## 7. Going live safely (checklist)

1. Scenes ready (Starting Soon → Live).
2. Mic/desktop levels checked with a **local recording** first (optional but recommended).
3. Kick title/category set.
4. OBS connected; **Start Streaming** while on Starting Soon.
5. Confirm Kick dashboard / channel page shows **LIVE**.
6. Switch to Live scene; greet chat.
7. Pin rules / Discord if desired.

**Practical:** First stream = friends-only or unlisted expectation; don’t stress viewer count.

**Never:** Paste stream key in chat “to help a friend debug.”

---

## 8. Ending streams correctly

1. Switch to Ending scene; thank viewers.
2. OBS → **Stop Streaming**.
3. Confirm channel is offline.
4. Optional: Stop Recording; remux MKV if needed.
5. Review VOD/clips if Kick saved them.

**Why Ending scene first:** Abrupt cut mid-sentence feels broken; gives chat a moment to follow.

---

## 9. Chat, moderation, community basics

### Concepts to teach

- **Mods** — trusted helpers who can timeout/ban.
- **Timeout vs ban** — temporary vs permanent (exact tools per Kick Help).
- **Slow mode / followers-only** — reduce spam during raids.
- **Blocked terms** — catch slurs/links if available.

### Practical community management

1. Write 3–5 channel rules (respect, no hate, no spoilers if relevant, no self-promo spam).
2. Appoint mods only after trust.
3. Don’t argue with bait in public — timeout → move on.
4. Use Discord for deeper hangouts (Block Viewer soft-open Discord on site).

**Cite:** Kick Help articles on moderation — re-link at freeze time.

---

## 10. Account security

| Practice | Why |
| --- | --- |
| Unique password + 2FA | Account takeover = lost channel |
| Stream key = secret | Hijacked broadcasts |
| Phishing awareness | Fake “verify Kick” sites |
| Don’t share backup codes | Same as password |
| Separate browser profile for streaming (practical) | Fewer accidental tab leaks on Display Capture |

---

## 11. Troubleshooting Kick connection

| Symptom | Likely causes | Fixes |
| --- | --- | --- |
| Won’t go live / immediate disconnect | Wrong server URL, revoked key, OBS service mismatch | Re-copy server+key; regenerate key; check Kick status |
| Live on OBS but not visible | Wrong account, cache, region delay | Open channel in private window; confirm correct login |
| Choppy for viewers | Bitrate/upload, Wi‑Fi | Lower bitrate; Ethernet; see OBS network dropped frames |
| Audio missing on Kick only | Mic muted in OBS, wrong track | Check mixer; Advanced Audio tracks if using multi-track |
| “Failed to connect” | Firewall/antivirus, corporate network blocking RTMPS | Allow OBS; try phone-hotspot test to isolate ISP |

**If Kick status page exists**, link it when verified.

---

## 12. Multistream note

Streaming Kick **and** another platform: see `03-aitum-multistream.md`. Dual OBS instances are discouraged for beginners (CPU + two keys to mess up).

---

## 13. Screenshot needs

`00-meta-sources-gaps.md` items 9–12, 25.

---

## 14. Gaps for editors

- Exact ingest hostname strings (copy from dashboard, don’t hardcode stale hosts in the book).
- Category name for Second Life / metaverse content on Kick.
- Whether Kick lists a first-party OBS service entry in latest OBS.
- Clip/VOD retention policy — check Help; don’t invent durations.
