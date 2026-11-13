/**
 * Block Viewer — tester nameplate signup.
 *
 * POST JSON { "username": "jr.alpha" } from the site.
 * Commits the name into peers/block-staff.json + peers/block-peers.json on main
 * (same roster the viewer fetches for Tester nameplates).
 *
 * Secrets: GITHUB_TOKEN — fine-grained PAT, Contents Read/Write on
 * ape-squared/Block-Viewer only.
 */

const REPO = "ape-squared/Block-Viewer";
const BRANCH = "main";
const STAFF_PATH = "peers/block-staff.json";
const PEERS_PATH = "peers/block-peers.json";
const ALLOW_ORIGIN = [
  "https://ape-squared.github.io",
  "http://127.0.0.1:8765",
  "http://localhost:8765",
];

const USERNAME_RE = /^[A-Za-z0-9_][A-Za-z0-9_.]{1,63}$/;

function corsHeaders(origin) {
  const allow = ALLOW_ORIGIN.includes(origin) ? origin : ALLOW_ORIGIN[0];
  return {
    "Access-Control-Allow-Origin": allow,
    "Access-Control-Allow-Methods": "POST, OPTIONS",
    "Access-Control-Allow-Headers": "Content-Type",
    "Access-Control-Max-Age": "86400",
    Vary: "Origin",
  };
}

function json(body, status, origin) {
  return new Response(JSON.stringify(body), {
    status,
    headers: {
      "Content-Type": "application/json; charset=utf-8",
      ...corsHeaders(origin),
    },
  });
}

function normalizeUsername(raw) {
  let s = String(raw || "")
    .trim()
    .toLowerCase()
    .replace(/ /g, ".");
  while (s.endsWith(".")) s = s.slice(0, -1);
  if (s.endsWith(".resident")) s = s.slice(0, -".resident".length);
  return s;
}

function validateUsername(raw) {
  const n = normalizeUsername(raw);
  if (!n || !USERNAME_RE.test(n)) return { error: "invalid username" };
  if (n === "heard" || n === "fr3sh3r") {
    return { error: "that name is staff/manager, not a tester signup" };
  }
  return { name: n };
}

function b64ToUtf8(b64) {
  const bin = atob(b64.replace(/\n/g, ""));
  const bytes = Uint8Array.from(bin, (c) => c.charCodeAt(0));
  return new TextDecoder().decode(bytes);
}

function utf8ToB64(text) {
  const bytes = new TextEncoder().encode(text);
  let bin = "";
  for (const b of bytes) bin += String.fromCharCode(b);
  return btoa(bin);
}

async function gh(token, method, path, body) {
  const res = await fetch(`https://api.github.com${path}`, {
    method,
    headers: {
      Accept: "application/vnd.github+json",
      Authorization: `Bearer ${token}`,
      "X-GitHub-Api-Version": "2022-11-28",
      "User-Agent": "block-tester-signup",
      ...(body ? { "Content-Type": "application/json" } : {}),
    },
    body: body ? JSON.stringify(body) : undefined,
  });
  const text = await res.text();
  let data = null;
  try {
    data = text ? JSON.parse(text) : null;
  } catch {
    data = { raw: text };
  }
  if (!res.ok) {
    const msg = data?.message || text || res.statusText;
    const err = new Error(`GitHub ${method} ${path}: ${res.status} ${msg}`);
    err.status = res.status;
    err.data = data;
    throw err;
  }
  return data;
}

async function getFile(token, path) {
  const data = await gh(
    token,
    "GET",
    `/repos/${REPO}/contents/${path}?ref=${encodeURIComponent(BRANCH)}`,
  );
  return {
    sha: data.sha,
    json: JSON.parse(b64ToUtf8(data.content)),
  };
}

async function putFile(token, path, contentObj, sha, message) {
  const content = utf8ToB64(JSON.stringify(contentObj, null, 2) + "\n");
  return gh(token, "PUT", `/repos/${REPO}/contents/${path}`, {
    message,
    content,
    sha,
    branch: BRANCH,
  });
}

function addUnique(list, name) {
  const existing = new Set((list || []).map((x) => normalizeUsername(x)));
  if (existing.has(name)) return false;
  list.push(name);
  return true;
}

export default {
  async fetch(request, env) {
    const origin = request.headers.get("Origin") || "";

    if (request.method === "OPTIONS") {
      return new Response(null, { status: 204, headers: corsHeaders(origin) });
    }

    if (request.method === "GET") {
      return json(
        {
          ok: true,
          service: "block-tester-signup",
          usage: 'POST {"username":"your.sl.name"}',
        },
        200,
        origin,
      );
    }

    if (request.method !== "POST") {
      return json({ ok: false, error: "method not allowed" }, 405, origin);
    }

    if (!env.GITHUB_TOKEN) {
      return json(
        {
          ok: false,
          error: "signup backend is not configured yet",
          fallback: "issue",
        },
        503,
        origin,
      );
    }

    let payload;
    try {
      payload = await request.json();
    } catch {
      return json({ ok: false, error: "expected JSON body" }, 400, origin);
    }

    const checked = validateUsername(payload?.username);
    if (checked.error) {
      return json({ ok: false, error: checked.error }, 400, origin);
    }
    const name = checked.name;

    try {
      const staffFile = await getFile(env.GITHUB_TOKEN, STAFF_PATH);
      const peersFile = await getFile(env.GITHUB_TOKEN, PEERS_PATH);

      staffFile.json.testers = staffFile.json.testers || { usernames: [], ids: [] };
      staffFile.json.testers.usernames = staffFile.json.testers.usernames || [];
      peersFile.json.usernames = peersFile.json.usernames || [];

      const addedStaff = addUnique(staffFile.json.testers.usernames, name);
      const addedPeers = addUnique(peersFile.json.usernames, name);

      if (!addedStaff && !addedPeers) {
        return json(
          {
            ok: true,
            username: name,
            already: true,
            message: "already on the Block Tester roster",
          },
          200,
          origin,
        );
      }

      const message = `Add ${name} to Block Tester roster.`;
      if (addedStaff) {
        await putFile(env.GITHUB_TOKEN, STAFF_PATH, staffFile.json, staffFile.sha, message);
        // re-fetch peers sha in case of race after staff write
        const peersFresh = await getFile(env.GITHUB_TOKEN, PEERS_PATH);
        peersFresh.json.usernames = peersFresh.json.usernames || [];
        addUnique(peersFresh.json.usernames, name);
        await putFile(env.GITHUB_TOKEN, PEERS_PATH, peersFresh.json, peersFresh.sha, message);
      } else {
        await putFile(env.GITHUB_TOKEN, PEERS_PATH, peersFile.json, peersFile.sha, message);
      }

      return json(
        {
          ok: true,
          username: name,
          already: false,
          message: "added to the Block Tester nameplate roster",
        },
        200,
        origin,
      );
    } catch (err) {
      console.error(String(err), err?.data);
      return json(
        {
          ok: false,
          error: "could not update roster",
          detail: String(err?.message || err),
          fallback: "issue",
        },
        502,
        origin,
      );
    }
  },
};
