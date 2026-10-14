(() => {
  const validIds = [
    "visible", "clip-x", "clip-y", "clip-both", "clip-margin-padding",
    "clip-margin-content", "clip-margin-border", "clip-margin-hidden",
    "mixed-visible-hidden", "mixed-clip-auto", "positioned-contained",
    "positioned-escape", "nested-clip",
  ];
  const oracle = window.collectC131ClipOracle();
  const byId = new Map(oracle.cases.map((row) => [row.id, row]));
  const cases = [];
  let pointCount = 0;
  const label = (element, target) => {
    if (!element) return "null";
    if (element === target || target.contains(element)) return "target";
    if (element.classList?.contains("nested-inner")) return "inner";
    if (element.classList?.contains("box")) return "box";
    if (element.classList?.contains("case")) return "root";
    const tag = element.tagName.toLowerCase();
    if (["main", "body", "html"].includes(tag)) return tag;
    throw new Error("unmodeled Chrome topmost element: " + tag + "#" + element.id);
  };
  for (const id of validIds) {
    const row = byId.get(id);
    if (!row) throw new Error("missing oracle row " + id);
    const section = Array.from(document.querySelectorAll("[data-case]"))
      .find((item) => item.dataset.case === id);
    if (!section) throw new Error("missing fixture row " + id);
    const target = section.querySelector(".target, .abs-target");
    if (!target) throw new Error("missing target in valid row " + id);
    const rect = target.getBoundingClientRect();
    const frame = {
      x: rect.x, y: rect.y, width: rect.width, height: rect.height,
      right: rect.right, bottom: rect.bottom,
    };
    const xStart = Math.floor(rect.left);
    const xEnd = Math.ceil(rect.right) - 1;
    const yStart = Math.floor(rect.top);
    const yEnd = Math.ceil(rect.bottom) - 1;
    const rows = [];
    for (let y = yStart; y <= yEnd; y++) {
      const runs = [];
      let active = null;
      let from = xStart;
      for (let x = xStart; x <= xEnd; x++) {
        const hit = label(document.elementFromPoint(x, y), target);
        pointCount++;
        if (hit !== active) {
          if (active !== null) runs.push({ from, to: x - 1, hit: active });
          active = hit;
          from = x;
        }
      }
      if (active !== null) runs.push({ from, to: xEnd, hit: active });
      rows.push({ y, runs });
    }
    cases.push({ id, targetId: target.id, target: frame, xStart, xEnd, yStart, yEnd, rows });
  }
  return JSON.stringify({
    browser: {
      product: "Chrome for Testing",
      version: "155.0.8059.40",
      revision: "cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}",
      userAgent: oracle.userAgent,
      viewport: { width: innerWidth, height: innerHeight, dpr: devicePixelRatio },
    },
    fixture: {
      path: "tests/fixtures/css/c13/overflow-clip-c13-1.html",
      sha256: "b7acd7e63f79355b28cd480511b9be82e0f2bef478f1aadf685eb3b0bf4ef16a",
      sourceObservationSha256: "fd6d9e66825bc453609245950635f4fc56054ae9011b14d2f8901d490ecde4b5",
      parentHitIdentitySha256: "55682451470f701400ad29f0d86494a0811207fc589ec71b1e62d1a921186db5",
    },
    scan: { mode: "target-frame-integer-grid-2d", caseCount: cases.length, pointCount },
    cases,
  });
})()
