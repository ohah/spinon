JSON.stringify({
  browser: {
    userAgent: navigator.userAgent,
    viewport: [innerWidth, innerHeight],
    devicePixelRatio,
  },
  results: window.result.map((scene) => ({
    id: scene.id,
    columnsPx: Array.from(
      scene.columns.matchAll(/(-?[0-9.]+)px/g),
      (match) => Number(match[1]),
    ),
    rowsPx: Array.from(
      scene.rows.matchAll(/(-?[0-9.]+)px/g),
      (match) => Number(match[1]),
    ),
    areas: scene.areas,
    rootSize: scene.root.slice(2),
    autoColumns: scene.autoColumns,
    autoRows: scene.autoRows,
    heroFramePx:
      scene.children.find((child) => child.className.includes("hero"))
        ?.relativeRect ?? null,
    childFramesPx: Object.fromEntries(
      scene.children.map((child) => [
        child.className.split(" ").at(-1),
        child.relativeRect,
      ]),
    ),
    overflowPx: scene.children.reduce(
      (out, child) => {
        const rect = child.relativeRect;
        return [
          Math.max(out[0], rect[0] + rect[2] - scene.root[2], 0),
          Math.max(out[1], rect[1] + rect[3] - scene.root[3], 0),
        ];
      },
      [0, 0],
    ),
  })),
})
