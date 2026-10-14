(() => {
  const validIds = [
    "visible", "clip-x", "clip-y", "clip-both", "clip-margin-padding",
    "clip-margin-content", "clip-margin-border", "clip-margin-hidden",
    "mixed-visible-hidden", "mixed-clip-auto", "positioned-contained",
    "positioned-escape", "nested-clip",
  ];
  const width = 1280;
  const height = 1100;
  window.scrollTo(0, 0);
  if (innerWidth !== width || innerHeight !== height || devicePixelRatio !== 1) {
    throw new Error(`viewport mismatch ${innerWidth}x${innerHeight} @${devicePixelRatio}`);
  }
  if (scrollX !== 0 || scrollY !== 0) throw new Error("page is not at scroll origin");
  const oracle = window.collectC131ClipOracle();
  const oracleById = new Map(oracle.cases.map((row) => [row.id, row]));
  const sections = new Map(
    Array.from(document.querySelectorAll("[data-case]"), (section) => [section.dataset.case, section]),
  );
  if (validIds.some((id) => !oracleById.has(id) || !sections.has(id))) {
    throw new Error("one or more valid clip rows are absent from the fixed fixture");
  }
  const identityCounts = Object.create(null);
  const rows = [];
  const identities = new Set();
  let pointCount = 0;
  const label = (element) => {
    if (!element) return "null";
    const section = element.closest?.("[data-case]");
    const id = section?.dataset.case;
    if (!validIds.includes(id)) return "root";
    const target = section.querySelector(".target, .abs-target");
    const box = section.querySelector(".box");
    const nested = section.querySelector(".nested-inner");
    if (element === target || target.contains(element)) return `${id}:target`;
    if (nested && (element === nested || nested.contains(element))) return `${id}:inner`;
    if (element === box || box.contains(element)) return `${id}:box`;
    return `${id}:case`;
  };
  for (let y = 0; y < height; y++) {
    const runs = [];
    let active;
    let runStart = 0;
    for (let x = 0; x < width; x++) {
      const identity = label(document.elementFromPoint(x, y));
      pointCount++;
      identities.add(identity);
      identityCounts[identity] = (identityCounts[identity] || 0) + 1;
      if (active === undefined) {
        active = identity;
        runStart = x;
      } else if (identity !== active) {
        runs.push({ from: runStart, to: x - 1, identity: active });
        active = identity;
        runStart = x;
      }
    }
    runs.push({ from: runStart, to: width - 1, identity: active });
    rows.push({ y, runs });
  }
  const expectedPointCount = width * height;
  if (pointCount !== expectedPointCount) {
    throw new Error(`point count ${pointCount} != ${expectedPointCount}`);
  }
  const cases = validIds.map((id) => {
    const section = sections.get(id);
    const box = section.querySelector(".box");
    const target = section.querySelector(".target, .abs-target");
    const inner = section.querySelector(".nested-inner");
    const frame = (element) => {
      const rect = element.getBoundingClientRect();
      return { x: rect.x, y: rect.y, width: rect.width, height: rect.height, right: rect.right, bottom: rect.bottom };
    };
    return {
      id,
      targetId: target.id,
      section: frame(section),
      box: frame(box),
      target: frame(target),
      inner: inner ? frame(inner) : null,
      computedOverflow: {
        x: oracleById.get(id).overflowX,
        y: oracleById.get(id).overflowY,
        clipMargin: oracleById.get(id).overflowClipMargin,
      },
    };
  });
  return {
    browser: {
      product: "Chrome for Testing",
      version: "155.0.8059.40",
      revision: "cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}",
      userAgent: oracle.userAgent,
      viewport: { width: innerWidth, height: innerHeight, dpr: devicePixelRatio, scrollX, scrollY },
    },
    fixture: {
      path: "tests/fixtures/css/c13/overflow-clip-c13-1.html",
      sha256: "b7acd7e63f79355b28cd480511b9be82e0f2bef478f1aadf685eb3b0bf4ef16a",
      sourceObservationSha256: "fd6d9e66825bc453609245950635f4fc56054ae9011b14d2f8901d490ecde4b5",
      targetFrameHitIdentity2dSha256: "248c9d15a56ef332428b0c1e341bac1b1ebe5ce6ea3ac7d01c95a95bd787fe5b",
    },
    scan: {
      mode: "full-viewport-integer-grid-2d-combined-c131-page",
      caseCount: cases.length,
      pointCount,
      identityProjection: "each valid case section, box, target and nested inner keeps its case id; invalid/float rows and other page elements map to runtime viewport root; null is preserved",
      uniqueIdentities: [...identities].sort(),
      identityCounts,
    },
    cases,
    rows,
  };
})()
