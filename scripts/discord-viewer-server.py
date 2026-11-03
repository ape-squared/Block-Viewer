#!/usr/bin/env python3
"""Reshape the Block. Discord for closed viewer testing.

Keeps the Viewer category (and its channels). Deletes every other channel /
category. Strips Create Instant Invite from @everyone / non-staff roles so
invites stay staff-only. Posts fresh info embeds in Viewer text channels.

Env:
  DISCORD_BOT_TOKEN   required — bot with Manage Channels, Manage Roles,
                      Manage Permissions, Send Messages
  DISCORD_GUILD_ID    optional — defaults to Block. (1566518132397445301)

Usage:
  python3 scripts/discord-viewer-server.py           # dry-run
  python3 scripts/discord-viewer-server.py --apply   # mutate the guild
"""

from __future__ import annotations

import argparse
import json
import os
import sys
import time
import urllib.error
import urllib.request

API = "https://discord.com/api/v10"
DEFAULT_GUILD = "1566518132397445301"
# Discord permission bit: CREATE_INSTANT_INVITE
PERM_CREATE_INVITE = 1 << 0
# Roles that may keep invite rights (name match, case-insensitive).
STAFF_ROLE_NAMES = {
    "staff",
    "admin",
    "admins",
    "mod",
    "mods",
    "moderator",
    "moderators",
    "owner",
    "team",
    "tester lead",
    "leads",
}


def die(msg: str, code: int = 1) -> None:
    print(msg, file=sys.stderr)
    raise SystemExit(code)


def api(token: str, method: str, path: str, body: dict | None = None) -> object:
    data = None if body is None else json.dumps(body).encode()
    req = urllib.request.Request(
        API + path,
        data=data,
        method=method,
        headers={
            "Authorization": f"Bot {token}",
            "Content-Type": "application/json",
            "User-Agent": "BlockViewer-DiscordReshape/1.0",
        },
    )
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            raw = resp.read()
            if not raw:
                return None
            return json.loads(raw.decode())
    except urllib.error.HTTPError as e:
        detail = e.read().decode("utf-8", "replace")
        die(f"{method} {path} → HTTP {e.code}: {detail}")


def is_viewer_category(ch: dict) -> bool:
    name = (ch.get("name") or "").strip().lower()
    return ch.get("type") == 4 and "viewer" in name


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--apply", action="store_true", help="perform mutations")
    ap.add_argument(
        "--keep-channel",
        action="append",
        default=[],
        help="extra channel name to keep even outside Viewer (repeatable)",
    )
    args = ap.parse_args()
    dry = not args.apply

    token = (os.environ.get("DISCORD_BOT_TOKEN") or "").strip()
    if not token:
        die("Set DISCORD_BOT_TOKEN")
    guild = (os.environ.get("DISCORD_GUILD_ID") or DEFAULT_GUILD).strip()

    me = api(token, "GET", "/users/@me")
    assert isinstance(me, dict)
    print(f"bot={me.get('username')}#{me.get('discriminator')} id={me.get('id')}")
    print(f"guild={guild} dry_run={dry}")

    channels = api(token, "GET", f"/guilds/{guild}/channels")
    assert isinstance(channels, list)
    roles = api(token, "GET", f"/guilds/{guild}/roles")
    assert isinstance(roles, list)

    viewer_cats = [c for c in channels if is_viewer_category(c)]
    if not viewer_cats:
        die("No category with 'viewer' in the name — aborting to avoid wiping the server")
    viewer_ids = {c["id"] for c in viewer_cats}
    print("viewer categories:")
    for c in viewer_cats:
        print(f"  #{c['name']} ({c['id']})")

    keep_names = {n.strip().lower() for n in args.keep_channel if n.strip()}
    keep: set[str] = set(viewer_ids)
    for c in channels:
        parent = c.get("parent_id")
        if parent in viewer_ids:
            keep.add(c["id"])
        if (c.get("name") or "").strip().lower() in keep_names:
            keep.add(c["id"])

    to_delete = [c for c in channels if c["id"] not in keep]
    # Delete children before categories (Discord wants empty-ish categories).
    to_delete.sort(key=lambda c: (c.get("type") == 4, c.get("position", 0)))

    print("\nkeep:")
    for c in sorted((x for x in channels if x["id"] in keep), key=lambda x: (x.get("type") != 4, x.get("position", 0))):
        kind = "category" if c.get("type") == 4 else "channel"
        print(f"  keep {kind} #{c['name']} ({c['id']})")

    print("\ndelete:")
    if not to_delete:
        print("  (nothing)")
    for c in to_delete:
        kind = "category" if c.get("type") == 4 else "channel"
        print(f"  delete {kind} #{c['name']} ({c['id']})")

    # Invite permission: strip from @everyone and non-staff roles.
    print("\ninvite permissions (strip Create Instant Invite from non-staff):")
    for role in roles:
        name = (role.get("name") or "").strip()
        perms = int(role.get("permissions") or 0)
        is_everyone = role.get("id") == guild
        is_staff = (not is_everyone) and (
            name.lower() in STAFF_ROLE_NAMES
            or "staff" in name.lower()
            or "admin" in name.lower()
            or role.get("managed")
            or (perms & (1 << 3)) != 0  # ADMINISTRATOR
        )
        has_invite = (perms & PERM_CREATE_INVITE) != 0
        if is_staff:
            print(f"  keep invite on role @{name} ({role['id']})")
            continue
        if not has_invite:
            print(f"  already no invite @{name}")
            continue
        new_perms = perms & ~PERM_CREATE_INVITE
        print(f"  strip invite from @{name} ({role['id']}) {perms} → {new_perms}")
        if not dry:
            api(
                token,
                "PATCH",
                f"/guilds/{guild}/roles/{role['id']}",
                {"permissions": str(new_perms)},
            )
            time.sleep(0.4)

    if dry:
        print("\nDry-run only. Re-run with --apply to mutate.")
        return

    for c in to_delete:
        print(f"deleting #{c['name']}…")
        api(token, "DELETE", f"/channels/{c['id']}")
        time.sleep(0.6)

    # Fresh embeds in remaining text channels under Viewer.
    thumb = (
        "https://raw.githubusercontent.com/ape-squared/Block-Viewer/main/"
        "assets/branding/block/01-logos/mark/block-mark-256.png"
    )
    remaining = api(token, "GET", f"/guilds/{guild}/channels")
    assert isinstance(remaining, list)
    text_under_viewer = [
        c
        for c in remaining
        if c.get("type") == 0 and c.get("parent_id") in viewer_ids
    ]
    embed = {
        "title": "Block Viewer — closed tester Discord",
        "description": (
            "This server is **invite-only** and focused on **viewer testing**.\n\n"
            "• Staff send invites — please do not share invite links publicly.\n"
            "• Builds land in `#viewer-releases`.\n"
            "• Use the testing channels for bugs, logs, and feedback on builds.\n"
            "• Source & site: https://github.com/ape-squared/Block-Viewer"
        ),
        "color": 0xD4FF3F,
        "thumbnail": {"url": thumb},
        "footer": {"text": "Block Viewer · staff invite only"},
    }
    for c in text_under_viewer:
        name = (c.get("name") or "").lower()
        # Prefer info-ish channels; still post once to each text channel.
        print(f"posting info embed → #{c['name']}")
        api(
            token,
            "POST",
            f"/channels/{c['id']}/messages",
            {
                "content": "Viewer testing only — outdated community channels were removed.",
                "embeds": [embed],
                "allowed_mentions": {"parse": []},
            },
        )
        # Topic nudge on releases / testing channels
        if any(k in name for k in ("release", "test", "info", "announce", "general")):
            topic = (
                "Closed viewer testing · staff invites only · "
                "https://github.com/ape-squared/Block-Viewer"
            )
            api(token, "PATCH", f"/channels/{c['id']}", {"topic": topic})
        time.sleep(0.5)

    print("\nDone.")


if __name__ == "__main__":
    main()
