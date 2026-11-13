#!/usr/bin/env python3
"""Add a Second Life username to the Block Tester nameplate roster.

Updates peers/block-staff.json (testers) and peers/block-peers.json so the
viewer can show the mark without a rebuild. Usernames are normalized the
same way as crates/block-viewer/src/world/staff.rs.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
STAFF = ROOT / "peers" / "block-staff.json"
PEERS = ROOT / "peers" / "block-peers.json"

# SL usernames: letters, digits, underscore; optional dotted last name.
USERNAME_RE = re.compile(r"^[A-Za-z0-9_][A-Za-z0-9_\.]{1,63}$")


def normalize_username(raw: str) -> str:
    s = raw.strip().lower().replace(" ", ".")
    while s.endswith("."):
        s = s[:-1]
    if s.endswith(".resident"):
        s = s[: -len(".resident")]
    return s


def validate(raw: str) -> str:
    n = normalize_username(raw)
    if not n or not USERNAME_RE.match(n):
        raise SystemExit(f"invalid username: {raw!r}")
    if n in {"heard", "fr3sh3r"}:
        raise SystemExit(f"{n} is already staff/manager — not added as tester")
    return n


def load(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def dump(path: Path, data: dict) -> None:
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def add_unique(names: list[str], name: str) -> bool:
    existing = {normalize_username(x) for x in names}
    if name in existing:
        return False
    names.append(name)
    return True


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("username", help="Second Life username (legacy or modern)")
    ap.add_argument(
        "--check",
        action="store_true",
        help="Exit 0 if already listed, 1 if would add, 2 if invalid",
    )
    args = ap.parse_args()
    try:
        name = validate(args.username)
    except SystemExit as e:
        if args.check:
            print(e)
            raise SystemExit(2) from e
        raise

    staff = load(STAFF)
    peers = load(PEERS)
    testers = staff.setdefault("testers", {}).setdefault("usernames", [])
    peer_names = peers.setdefault("usernames", [])

    added_staff = add_unique(testers, name)
    added_peers = add_unique(peer_names, name)

    if args.check:
        if not added_staff and not added_peers:
            print(f"already listed: {name}")
            raise SystemExit(2)
        print(f"would add: {name}")
        raise SystemExit(0)

    if not added_staff and not added_peers:
        print(f"already listed: {name}")
        raise SystemExit(2)

    dump(STAFF, staff)
    dump(PEERS, peers)
    print(f"added tester: {name}")


if __name__ == "__main__":
    main()
