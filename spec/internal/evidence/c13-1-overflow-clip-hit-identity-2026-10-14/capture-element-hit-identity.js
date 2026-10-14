(() => {
  const oracle = window.collectC131ClipOracle();
  const cases = [];
  for (const row of oracle.cases) {
    const section = Array.from(document.querySelectorAll("[data-case]"))
      .find((item) => item.dataset.case === row.id);
    const target = section.querySelector(".target, .abs-target");
    if (!target) continue;
    const label = (element) => {
      if (!element) return "null";
      if (element === target || target.contains(element)) return "target";
      if (element.classList?.contains("nested-inner")) return "inner";
      if (element.classList?.contains("box")) return "box";
      if (element.classList?.contains("case")) return "root";
      return element.tagName.toLowerCase();
    };
    const rect = target.getBoundingClientRect();
    const targetFrame = {
      x: rect.x,
      y: rect.y,
      width: rect.width,
      height: rect.height,
      right: rect.right,
      bottom: rect.bottom,
    };
    const axes = {};
    for (const axis of ["x", "y"]) {
      const horizontal = axis === "x";
      const start = Math.floor(horizontal ? rect.left : rect.top);
      const end = Math.ceil(horizontal ? rect.right : rect.bottom) - 1;
      const fixed = row.hitRanges[axis].fixedAt;
      const runs = [];
      let active = null;
      let from = start;
      for (let point = start; point <= end; point++) {
        const x = horizontal ? point : fixed;
        const y = horizontal ? fixed : point;
        const name = label(document.elementFromPoint(x, y));
        if (name !== active) {
          if (active !== null) runs.push({ from, to: point - 1, hit: active });
          active = name;
          from = point;
        }
      }
      if (active !== null) runs.push({ from, to: end, hit: active });
      axes[axis] = { fixedAt: fixed, runs };
    }
    cases.push({ id: row.id, targetId: target.id, target: targetFrame, axes });
  }
  return JSON.stringify({
    browser: {
      version: "155.0.8059.40",
      userAgent: oracle.userAgent,
      viewport: oracle.viewport,
    },
    cases,
  });
})()
