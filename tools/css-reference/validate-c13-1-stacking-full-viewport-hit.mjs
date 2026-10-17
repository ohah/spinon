import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { resolve } from "node:path";

export const EXPECTED_IDS = [
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

export const EXPECTED_EDGE_CONTROLS = [
  { x: 538, y: 64, hitId: "root" },
  { x: 539, y: 64, hitId: "root" },
  { x: 0, y: 127, hitId: "root" },
  { x: 539, y: 127, hitId: "root" },
  { x: 539.5, y: 0.5, hitId: null },
  { x: 0.5, y: 127.5, hitId: null },
  { x: 539.5, y: 127.5, hitId: null },
  { x: -0.5, y: 64, hitId: null },
  { x: 540, y: 64, hitId: null },
  { x: 269, y: -0.5, hitId: null },
  { x: 269, y: 128, hitId: null },
];

const EXPECTED_FIXTURE_SHA256 = "6b691ddf5afd44543b438b47200f9a6f85981fa3d3c30e1619185a65d320791e";
const EXPECTED_GRID_SHA256 = "866073e0c6f092d4b2459fa3b5cfd2ca38dfb8935f582ee3c9614dba3bfc7483";
const EXPECTED_SCREENSHOT_SHA256 = "a9727d516837df6c8a5d77e624aee3463b08ebfb991d8eb622d52d026c64585c";

function requireCondition(condition, message) {
  if (!condition) throw new Error(message);
}

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

export function validateCapture({ oracle, fixtureBytes, screenshotBytes }) {
  requireCondition(oracle?.schemaVersion === 1, "schemaVersion must be 1");
  requireCondition(
    oracle.source === "Agent Browser Chrome elementFromPoint at integer CSS coordinates",
    "capture source is not the pinned Agent Browser query",
  );
  requireCondition(
    oracle.queryCoordinateConvention === "integer CSS points (x,y), not pixel centers",
    "coordinate convention changed",
  );
  requireCondition(
    oracle.fixtureSha256 === EXPECTED_FIXTURE_SHA256 && sha256(fixtureBytes) === EXPECTED_FIXTURE_SHA256,
    "fixture hash mismatch",
  );
  requireCondition(oracle.browser?.platform === "macOS", "browser platform changed");
  requireCondition(oracle.browser?.platformVersion === "26.5.1", "macOS version changed");
  requireCondition(oracle.browser?.architecture === "arm", "browser architecture changed");
  requireCondition(
    oracle.browser?.fullVersionList?.some(
      ({ brand, version }) => brand === "Google Chrome" && version === "155.0.8059.85",
    ),
    "Chrome full version changed",
  );
  requireCondition(
    oracle.browser?.userAgent?.includes("HeadlessChrome/155.0.0.0"),
    "Chrome user agent changed",
  );

  const viewport = oracle.viewport;
  requireCondition(
    viewport?.width === 540 && viewport?.height === 128 && viewport?.devicePixelRatio === 1,
    "viewport or DPR changed",
  );
  requireCondition(viewport?.scrollX === 0 && viewport?.scrollY === 0, "page scroll changed");
  requireCondition(
    JSON.stringify(oracle.nodeIds) === JSON.stringify(EXPECTED_IDS),
    "node ID order changed",
  );
  requireCondition(oracle.pointCount === 69_120, "point count changed");
  requireCondition(oracle.rowCount === 128 && oracle.rows?.length === 128, "row count changed");
  requireCondition(Array.isArray(oracle.rows), "rows are missing");

  const expanded = Buffer.alloc(oracle.pointCount);
  const nodeHitCounts = Array(EXPECTED_IDS.length).fill(0);
  let runCount = 0;
  for (let y = 0; y < oracle.rows.length; y += 1) {
    const runs = oracle.rows[y];
    requireCondition(Array.isArray(runs) && runs.length > 0, `row ${y} is empty`);
    let x = 0;
    let previousIndex = -1;
    for (const run of runs) {
      requireCondition(Array.isArray(run) && run.length === 2, `row ${y} has malformed run`);
      const [nodeIndex, width] = run;
      requireCondition(
        Number.isInteger(nodeIndex) && nodeIndex >= 0 && nodeIndex < EXPECTED_IDS.length,
        `row ${y} has unregistered node index`,
      );
      requireCondition(Number.isInteger(width) && width > 0, `row ${y} has invalid run width`);
      requireCondition(nodeIndex !== previousIndex, `row ${y} has a noncanonical adjacent run`);
      requireCondition(x + width <= viewport.width, `row ${y} exceeds viewport width`);
      expanded.fill(nodeIndex, y * viewport.width + x, y * viewport.width + x + width);
      nodeHitCounts[nodeIndex] += width;
      x += width;
      previousIndex = nodeIndex;
      runCount += 1;
    }
    requireCondition(x === viewport.width, `row ${y} has a gap or overlap`);
  }
  const expandedHash = sha256(expanded);
  requireCondition(expandedHash === EXPECTED_GRID_SHA256, "expanded grid digest changed");
  requireCondition(oracle.expandedNodeIndexSha256 === expandedHash, "expanded grid digest mismatch");
  requireCondition(
    nodeHitCounts.reduce((sum, count) => sum + count, 0) === oracle.pointCount,
    "expanded point count mismatch",
  );

  requireCondition(oracle.screenshot?.width === 540 && oracle.screenshot?.height === 128,
    "screenshot dimensions changed");
  requireCondition(
    oracle.screenshot?.sha256 === EXPECTED_SCREENSHOT_SHA256 &&
      sha256(screenshotBytes) === EXPECTED_SCREENSHOT_SHA256,
    "screenshot hash mismatch",
  );
  requireCondition(
    Array.isArray(oracle.edgeControls) && oracle.edgeControls.length === EXPECTED_EDGE_CONTROLS.length,
    "viewport edge controls changed",
  );
  for (let index = 0; index < EXPECTED_EDGE_CONTROLS.length; index += 1) {
    const actual = oracle.edgeControls[index];
    const expected = EXPECTED_EDGE_CONTROLS[index];
    requireCondition(
      actual?.x === expected.x && actual?.y === expected.y && actual?.hitId === expected.hitId,
      `viewport edge controls changed at index ${index}`,
    );
  }

  return {
    pointCount: oracle.pointCount,
    rowCount: oracle.rowCount,
    runCount,
    expandedNodeIndexSha256: expandedHash,
    nodeHitCounts: Object.fromEntries(
      EXPECTED_IDS.map((id, index) => [id, nodeHitCounts[index]]),
    ),
  };
}

async function main() {
  const root = process.cwd();
  const evidence = resolve(root, "spec/internal/evidence/c13-1-stacking-app-full-viewport-hit-2026-10-18");
  const [oracleBytes, fixtureBytes, screenshotBytes] = await Promise.all([
    readFile(resolve(evidence, "chrome-full-viewport-hit.json")),
    readFile(resolve(root, "tests/fixtures/css/c13/overflow-clip-stacking-descendants-c13-1.html")),
    readFile(resolve(evidence, "chrome155-stacking-full-viewport.png")),
  ]);
  const oracle = JSON.parse(oracleBytes.toString("utf8"));
  const summary = validateCapture({ oracle, fixtureBytes, screenshotBytes });
  process.stdout.write(`${JSON.stringify(summary, null, 2)}\n`);
}

if (process.argv[1] && fileURLToPath(import.meta.url) === resolve(process.argv[1])) {
  await main();
}
