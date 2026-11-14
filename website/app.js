(() => {
  const reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  // Tall blocks (full docs articles) can never meet a fractional threshold while
  // only a slice of them fits the viewport — they would stay opacity:0 forever.
  const reveals = document.querySelectorAll(".reveal");
  const show = (el) => el.classList.add("is-in");
  if (reduce) {
    reveals.forEach(show);
  } else if ("IntersectionObserver" in window) {
    const vh = () => window.innerHeight || document.documentElement.clientHeight || 800;
    const io = new IntersectionObserver(
      (entries) => {
        for (const e of entries) {
          if (!e.isIntersecting) continue;
          show(e.target);
          io.unobserve(e.target);
        }
      },
      { rootMargin: "0px 0px -8% 0px", threshold: 0 }
    );
    reveals.forEach((el) => {
      if (el.getBoundingClientRect().height > vh() * 0.85) show(el);
      else io.observe(el);
    });
  } else {
    reveals.forEach(show);
  }

  // Steps get the edge mark when they enter view
  document.querySelectorAll(".steps li").forEach((li) => {
    if (li.classList.contains("reveal")) return;
    li.classList.add("reveal");
    if (reduce) li.classList.add("is-in");
    else if ("IntersectionObserver" in window) {
      const io = new IntersectionObserver(
        (entries) => {
          for (const e of entries) {
            if (!e.isIntersecting) continue;
            e.target.classList.add("is-in");
            io.unobserve(e.target);
          }
        },
        { threshold: 0.2 }
      );
      io.observe(li);
    }
  });

  const nav = document.querySelector("[data-nav]");
  if (nav) {
    const onScroll = () => nav.classList.toggle("is-scrolled", window.scrollY > 8);
    onScroll();
    window.addEventListener("scroll", onScroll, { passive: true });
  }

  // Soft in-view scale for the world band photo
  document.querySelectorAll(".world-band").forEach((band) => {
    if (reduce) {
      band.classList.add("is-in");
      return;
    }
    if (!("IntersectionObserver" in window)) {
      band.classList.add("is-in");
      return;
    }
    const io = new IntersectionObserver(
      (entries) => {
        for (const e of entries) {
          if (!e.isIntersecting) continue;
          e.target.classList.add("is-in");
          io.unobserve(e.target);
        }
      },
      { threshold: 0.2 }
    );
    io.observe(band);
  });

  const toggle = document.querySelector("[data-nav-toggle]");
  if (toggle && nav) {
    toggle.addEventListener("click", () => {
      const open = nav.classList.toggle("is-open");
      toggle.setAttribute("aria-expanded", open ? "true" : "false");
    });
    nav.querySelectorAll(".nav-links a").forEach((a) => {
      a.addEventListener("click", () => {
        nav.classList.remove("is-open");
        toggle.setAttribute("aria-expanded", "false");
      });
    });
  }

  // Latest release → every [data-latest-version] / [data-latest-download]
  // Prefer GitHub API so the button always points at a real asset zip/dmg
  // (instant download). Fall back to website/latest.json if the API is down.
  const verEls = document.querySelectorAll("[data-latest-version]");
  const dlEls = document.querySelectorAll("[data-latest-download]");
  if (verEls.length || dlEls.length) {
    const REPO = "ape-squared/Block-Viewer";
    const css = document.querySelector('link[rel="stylesheet"]');
    const latestJson = css
      ? new URL("latest.json", css.href).href
      : new URL("latest.json", location.href).href;

    const preferMac = () => {
      const ua = navigator.userAgent || "";
      const plat = navigator.platform || "";
      return /Mac|iPhone|iPad|iPod/.test(plat) || /Mac OS X/.test(ua);
    };

    const pickAsset = (assets, platform) => {
      const list = assets || [];
      if (platform === "mac") {
        return list.find((a) => /macos-arm64\.dmg$/i.test(a.name) && !/\.sha256$/i.test(a.name));
      }
      return list.find((a) => /windows-x64\.zip$/i.test(a.name) && !/symbols/i.test(a.name) && !/\.sha256$/i.test(a.name));
    };

    const fromApi = (release) => {
      const tag = release.tag_name || release.tag;
      const assets = (release.assets || []).map((a) => ({
        name: a.name,
        url: a.browser_download_url || a.url,
      }));
      return { tag, assets };
    };

    const fromJson = (data) => {
      const tag = data.tag || ("v" + data.version);
      const version = data.version || String(tag).replace(/^v/, "");
      const win = data.asset || ("Block-Viewer-v" + version + "-windows-x64.zip");
      const mac = data.mac_asset || ("Block-Viewer-v" + version + "-macos-arm64.dmg");
      const base = "https://github.com/" + REPO + "/releases/download/" + encodeURIComponent(tag) + "/";
      return {
        tag,
        assets: [
          { name: win, url: base + encodeURIComponent(win) },
          { name: mac, url: base + encodeURIComponent(mac) },
        ],
      };
    };

    const apply = (info) => {
      if (!info || !info.tag) return;
      const tag = info.tag;
      const version = String(tag).replace(/^v/, "");
      verEls.forEach((el) => {
        el.textContent = el.closest(".btn-download") ? tag : "latest " + tag;
      });
      dlEls.forEach((el) => {
        let platform = (el.getAttribute("data-latest-download") || "").toLowerCase();
        if (platform !== "windows" && platform !== "mac") {
          platform = preferMac() ? "mac" : "windows";
        }
        const asset = pickAsset(info.assets, platform) || pickAsset(info.assets, "windows");
        if (!asset) return;
        el.href = asset.url;
        el.setAttribute("download", asset.name);
        el.setAttribute("data-platform", platform);
        const main = el.querySelector(".btn-main");
        if (main && !el.hasAttribute("data-keep-label")) {
          main.textContent =
            platform === "mac" ? "Download for macOS" : "Download for Windows";
        }
        const sub = el.querySelector(".btn-sub:not([data-latest-version])");
        if (sub && !el.hasAttribute("data-keep-label")) {
          sub.textContent = platform === "mac" ? "Apple silicon DMG · " + tag : "portable zip · " + tag;
        }
        if (!el.classList.contains("btn-download") && !el.querySelector(".btn-main")) {
          el.textContent = "Download " + asset.name;
        }
      });
    };

    fetch("https://api.github.com/repos/" + REPO + "/releases/latest", {
      headers: { Accept: "application/vnd.github+json" },
      cache: "no-store",
    })
      .then((r) => (r.ok ? r.json() : Promise.reject(new Error("api"))))
      .then((release) => apply(fromApi(release)))
      .catch(() =>
        fetch(latestJson, { cache: "no-store" })
          .then((r) => (r.ok ? r.json() : null))
          .then((data) => {
            if (data) apply(fromJson(data));
            else verEls.forEach((el) => { el.textContent = "see GitHub Releases"; });
          })
          .catch(() => {
            verEls.forEach((el) => { el.textContent = "see GitHub Releases"; });
          })
      );
  }
})();
