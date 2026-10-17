(async () => {
const expectedIds = [
  "root",
  "visible-parent",
  "visible-layer",
  "visible-target",
  "visible-overlay",
  "clip-parent",
  "clip-layer",
  "clip-target",
  "clip-overlay",
  "hidden-parent",
  "hidden-layer",
  "hidden-target",
  "hidden-overlay",
];

const observation = window.collectC131StackingClipObservation();
const viewport = {
  width: innerWidth,
  height: innerHeight,
  devicePixelRatio,
  scrollX,
  scrollY,
};
if (
  viewport.width !== 540 ||
  viewport.height !== 128 ||
  viewport.devicePixelRatio !== 1 ||
  viewport.scrollX !== 0 ||
  viewport.scrollY !== 0 ||
  observation.viewport.width !== viewport.width ||
  observation.viewport.height !== viewport.height ||
  observation.viewport.devicePixelRatio !== viewport.devicePixelRatio ||
  observation.viewport.scrollX !== 0 ||
  observation.viewport.scrollY !== 0
) {
  throw new Error(`viewport mismatch: ${JSON.stringify({ viewport, observation: observation.viewport })}`);
}

const actualIds = [...document.querySelectorAll("[data-id]")].map((node) => node.dataset.id);
if (JSON.stringify(actualIds) !== JSON.stringify(expectedIds)) {
  throw new Error(`node inventory mismatch: ${JSON.stringify(actualIds)}`);
}
if (observation.cases.map((entry) => entry.id).join(",") !== "visible,clip,hidden") {
  throw new Error("case order mismatch");
}

const nodeIndex = new Map(expectedIds.map((id, index) => [id, index]));
const expanded = new Uint8Array(viewport.width * viewport.height);
const rows = [];
for (let y = 0; y < viewport.height; y += 1) {
  const runs = [];
  let previousIndex = -1;
  let runLength = 0;
  for (let x = 0; x < viewport.width; x += 1) {
    const element = document.elementFromPoint(x, y);
    const id = element?.closest("[data-id]")?.getAttribute("data-id");
    const index = nodeIndex.get(id);
    if (index === undefined) {
      throw new Error(`unregistered hit at integer point (${x},${y}): ${id ?? "null"}`);
    }
    expanded[y * viewport.width + x] = index;
    if (index === previousIndex) {
      runLength += 1;
    } else {
      if (runLength > 0) runs.push([previousIndex, runLength]);
      previousIndex = index;
      runLength = 1;
    }
  }
  if (runLength > 0) runs.push([previousIndex, runLength]);
  if (runs.reduce((sum, run) => sum + run[1], 0) !== viewport.width) {
    throw new Error(`row coverage mismatch at y=${y}`);
  }
  rows.push(runs);
}

const digest = await crypto.subtle.digest("SHA-256", expanded);
const expandedNodeIndexSha256 = [...new Uint8Array(digest)]
  .map((byte) => byte.toString(16).padStart(2, "0"))
  .join("");
const edgeProbePoints = [
  [538, 64], [539, 64], [0, 127], [539, 127],
  [539.5, 0.5], [0.5, 127.5], [539.5, 127.5],
  [-0.5, 64], [540, 64], [269, -0.5], [269, 128],
];
const edgeControls = edgeProbePoints.map(([x, y]) => ({
  x,
  y,
  hitId: document.elementFromPoint(x, y)
    ?.closest("[data-id]")
    ?.getAttribute("data-id") ?? null,
}));
const browser = await navigator.userAgentData.getHighEntropyValues([
  "fullVersionList",
  "platformVersion",
  "architecture",
]);
return {
  schemaVersion: 1,
  source: "Agent Browser Chrome elementFromPoint at integer CSS coordinates",
  fixtureSha256: "6b691ddf5afd44543b438b47200f9a6f85981fa3d3c30e1619185a65d320791e",
  browser: {
    userAgent: navigator.userAgent,
    ...browser,
  },
  viewport,
  queryCoordinateConvention: "integer CSS points (x,y), not pixel centers",
  edgeControls,
  nodeIds: expectedIds,
  pointCount: expanded.length,
  rowCount: rows.length,
  expandedNodeIndexSha256,
  rows,
};
})()
