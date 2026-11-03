#!/usr/bin/env python3
"""Tidy the Block. Discord for closed viewer testing.

- Merge Development channels into Staff, then remove the Development category
- Emoji-label categories
- Strip Create Instant Invite from non-staff roles
- Refresh a short info embed in Viewer text channels

Does **not** mass-delete Community / Staff / Information / Voice / Viewer.

Env:
  DISCORD_BOT_TOKEN   required
  DISCORD_GUILD_ID    optional — defaults to Block. (1566518132397445301)

Usage:
  python3 scripts/discord-viewer-server.py           # dry-run
  python3 scripts/discord-viewer-server.py --apply
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
PERM_CREATE_INVITE = 1 << 0
PERM_ADMINISTRATOR = 1 << 3

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
    "block.",  # branded staff/owner role seen on the guild
}

# Category rename map: substring match (casefold) → new name with emoji.
CATEGORY_TITLES = {
    "viewer": "🧪 Viewer",
    "community": "💬 Community",
    "staff": "🛡️ Staff",
    "information": "ℹ️ Information",
    "info": "ℹ️ Information",
    "voice": "🔊 Voice",
    "development": "🛡️ Staff",  # merged away; rename only if somehow kept
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
            "User-Agent": "BlockViewer-DiscordReshape/1.1",
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


def cat_key(name: str) -> str | None:
    n = name.casefold()
    # strip leading emoji / symbols for matching
    stripped = "".join(ch for ch in n if ch.isalnum() or ch.isspace() or ch in "-_").strip()
    for key in ("viewer", "community", "staff", "information", "development", "voice", "info"):
        if key in stripped:
            return "information" if key == "info" else key
    return None


def is_staff_role(role: dict, guild_id: str) -> bool:
    name = (role.get("name") or "").strip()
    perms = int(role.get("permissions") or 0)
    if role.get("id") == guild_id:
        return False
    if role.get("managed"):
        return True
    if perms & PERM_ADMINISTRATOR:
        return True
    low = name.casefold()
    if low in STAFF_ROLE_NAMES:
        return True
    return any(k in low for k in ("staff", "admin", "mod"))


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--apply", action="store_true")
    args = ap.parse_args()
    dry = not args.apply

    token = (os.environ.get("DISCORD_BOT_TOKEN") or "").strip()
    if not token:
        die("Set DISCORD_BOT_TOKEN")
    guild = (os.environ.get("DISCORD_GUILD_ID") or DEFAULT_GUILD).strip()

    me = api(token, "GET", "/users/@me")
    assert isinstance(me, dict)
    print(f"bot={me.get('username')} id={me.get('id')} guild={guild} dry_run={dry}")

    channels = api(token, "GET", f"/guilds/{guild}/channels")
    assert isinstance(channels, list)
    roles = api(token, "GET", f"/guilds/{guild}/roles")
    assert isinstance(roles, list)

    categories = {c["id"]: c for c in channels if c.get("type") == 4}
    by_key: dict[str, dict] = {}
    for c in categories.values():
        key = cat_key(c.get("name") or "")
        if key and key not in by_key:
            by_key[key] = c

    staff_cat = by_key.get("staff")
    dev_cat = by_key.get("development")
    if not staff_cat:
        die("No Staff category found")

    print("\ncategories:")
    for c in sorted(categories.values(), key=lambda x: x.get("position", 0)):
        print(f"  #{c['name']} ({c['id']}) → key={cat_key(c.get('name') or '')}")

    # 1) Move Development children under Staff
    move: list[dict] = []
    if dev_cat:
        for c in channels:
            if c.get("parent_id") == dev_cat["id"]:
                move.append(c)
        print(f"\nmerge into Staff ({staff_cat['name']}):")
        if not move:
            print("  (no development channels)")
        for c in move:
            print(f"  move #{c['name']} → Staff")
    else:
        print("\nno Development category — nothing to merge")

    # 2) Category emoji renames (skip development — it gets deleted)
    renames: list[tuple[dict, str]] = []
    for c in categories.values():
        key = cat_key(c.get("name") or "")
        if not key or key == "development":
            continue
        want = CATEGORY_TITLES.get(key)
        if want and (c.get("name") or "") != want:
            renames.append((c, want))
    print("\nrename categories:")
    if not renames:
        print("  (already titled)")
    for c, want in renames:
        print(f"  #{c['name']} → {want}")

    delete_cats = [dev_cat] if dev_cat else []
    print("\ndelete empty categories after merge:")
    for c in delete_cats:
        print(f"  delete category #{c['name']}")

    # 3) Invite permissions
    print("\ninvite permissions:")
    invite_patches: list[tuple[dict, int]] = []
    for role in roles:
        name = (role.get("name") or "").strip()
        perms = int(role.get("permissions") or 0)
        has_invite = (perms & PERM_CREATE_INVITE) != 0
        if is_staff_role(role, guild):
            print(f"  keep invite @{name}")
            continue
        if not has_invite:
            print(f"  already no invite @{name}")
            continue
        new_perms = perms & ~PERM_CREATE_INVITE
        print(f"  strip invite @{name}: {perms} → {new_perms}")
        invite_patches.append((role, new_perms))

    if dry:
        print("\nDry-run only. Re-run with --apply to mutate.")
        return

    # Apply moves
    for c in move:
        print(f"moving #{c['name']}…")
        api(token, "PATCH", f"/channels/{c['id']}", {"parent_id": staff_cat["id"]})
        time.sleep(0.5)

    # Rename categories
    for c, want in renames:
        print(f"renaming → {want}")
        api(token, "PATCH", f"/channels/{c['id']}", {"name": want})
        time.sleep(0.5)

    # Delete development category (should be empty)
    for c in delete_cats:
        print(f"deleting category #{c['name']}…")
        api(token, "DELETE", f"/channels/{c['id']}")
        time.sleep(0.5)

    for role, new_perms in invite_patches:
        print(f"stripping invite @{role['name']}…")
        api(
            token,
            "PATCH",
            f"/guilds/{guild}/roles/{role['id']}",
            {"permissions": str(new_perms)},
        )
        time.sleep(0.4)

    # Refresh embeds once in viewer text channels (not every channel spam)
    remaining = api(token, "GET", f"/guilds/{guild}/channels")
    assert isinstance(remaining, list)
    viewer = next((c for c in remaining if c.get("type") == 4 and cat_key(c.get("name") or "") == "viewer"), None)
    thumb = (
        "https://raw.githubusercontent.com/ape-squared/Block-Viewer/main/"
        "assets/branding/block/01-logos/mark/block-mark-256.png"
    )
    embed = {
        "title": "Block Viewer — closed tester Discord",
        "description": (
            "Invite-only server focused on **viewer testing**.\n\n"
            "• Staff send invites — don’t share invite links publicly.\n"
            "• Builds land in `#viewer-releases`.\n"
            "• Bugs / ideas: `#viewer-bugs` · `#viewer-ideas` · `#viewer-help`.\n"
            "• Source: https://github.com/ape-squared/Block-Viewer"
        ),
        "color": 0xD4FF3F,
        "thumbnail": {"url": thumb},
        "footer": {"text": "Block Viewer · staff invite only"},
    }
    if viewer:
        targets = [
            c
            for c in remaining
            if c.get("type") == 0
            and c.get("parent_id") == viewer["id"]
            and "release" in (c.get("name") or "").casefold()
        ]
        if not targets:
            targets = [
                c
                for c in remaining
                if c.get("type") == 0 and c.get("parent_id") == viewer["id"]
            ][:1]
        for c in targets:
            print(f"posting info embed → #{c['name']}")
            api(
                token,
                "POST",
                f"/channels/{c['id']}/messages",
                {
                    "content": "Server layout refreshed — viewer testing focus; Development now lives under Staff.",
                    "embeds": [embed],
                    "allowed_mentions": {"parse": []},
                },
            )
            api(
                token,
                "PATCH",
                f"/channels/{c['id']}",
                {
                    "topic": (
                        "Closed viewer testing · staff invites only · "
                        "https://github.com/ape-squared/Block-Viewer"
                    )
                },
            )
            time.sleep(0.4)

    print("\nDone.")


if __name__ == "__main__":
    main()
