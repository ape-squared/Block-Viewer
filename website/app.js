(() => {
  const reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  const reveals = document.querySelectorAll(".reveal");
  if (reduce) {
    reveals.forEach((el) => el.classList.add("is-in"));
  } else if ("IntersectionObserver" in window) {
    const io = new IntersectionObserver(
      (entries) => {
        for (const e of entries) {
          if (!e.isIntersecting) continue;
          e.target.classList.add("is-in");
          io.unobserve(e.target);
        }
      },
      { rootMargin: "0px 0px -8% 0px", threshold: 0.12 }
    );
    reveals.forEach((el) => io.observe(el));
  } else {
    reveals.forEach((el) => el.classList.add("is-in"));
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

  // latest.json → every [data-latest-version] / [data-latest-download]
  const verEls = document.querySelectorAll("[data-latest-version]");
  const dlEls = document.querySelectorAll("[data-latest-download]");
  if (verEls.length || dlEls.length) {
    const css = document.querySelector('link[rel="stylesheet"]');
    const latestUrl = css
      ? new URL("latest.json", css.href).href
      : new URL("latest.json", location.href).href;
    fetch(latestUrl, { cache: "no-store" })
      .then((r) => (r.ok ? r.json() : null))
      .then((data) => {
        if (!data) return;
        const tag = data.tag || ("v" + data.version);
        const version = data.version || String(tag).replace(/^v/, "");
        const zipName = data.asset || ("Block-Viewer-v" + version + "-windows-x64.zip");
        const assetUrl =
          "https://github.com/ape-squared/Block-Viewer/releases/download/" +
          encodeURIComponent(tag) +
          "/" +
          encodeURIComponent(zipName);
        verEls.forEach((el) => {
          el.textContent = el.closest(".btn-download") ? tag : "latest " + tag;
        });
        dlEls.forEach((el) => {
          el.href = assetUrl;
          if (!el.classList.contains("btn-download")) {
            el.textContent = "Download " + zipName;
          }
        });
      })
      .catch(() => {
        verEls.forEach((el) => {
          el.textContent = "see GitHub Releases";
        });
      });
  }
})();
