import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const evidenceDir = dirname(fileURLToPath(import.meta.url));
const repositoryRoot = resolve(evidenceDir, "../../../..");
const fixturePath = resolve(evidenceDir, "oracle-fixture.html");
const observationPath = resolve(evidenceDir, "chrome157-observation.ndjson");
const screenshotPath = resolve(evidenceDir, "chrome157-three-fit-auto-min-sample.png");
const lifecyclePath = resolve(evidenceDir, "agent-browser-lifecycle.json");
const browserCli = process.env.AGENT_BROWSER_PATH ?? "/opt/homebrew/bin/agent-browser";
const session = "spinon-c114-three-fit-autoplacement-precomparison-20261111";
const expectedSceneCount = 5_760;
const batchSize = 240;

mkdirSync(evidenceDir, { recursive: true });
const fixtureSha256 = createHash("sha256").update(readFileSync(fixturePath)).digest("hex");
const collectorSha256 = createHash("sha256").update(readFileSync(fileURLToPath(import.meta.url))).digest("hex");
let opened = false;
let closed = false;
let closeError = null;
const captured = [];

function runBrowser(args, timeout = 180_000) {
  return execFileSync(browserCli, ["--session", session, ...args], {
    cwd: repositoryRoot,
    encoding: "utf8",
    maxBuffer: 64 * 1024 * 1024,
    timeout,
  }).trim();
}

function parseEvalOutput(output) {
  const decoded = JSON.parse(output);
  return typeof decoded === "string" ? JSON.parse(decoded) : decoded;
}

function require(condition, message) {
  if (!condition) throw new Error(message);
}

function validateScene(record) {
  const { scene } = record;
  require(scene.fitPositions.length === 3, `scene ${record.index}: expected three fit positions`);
  require(scene.fitPositions[0] < scene.fitPositions[1] && scene.fitPositions[1] < scene.fitPositions[2],
    `scene ${record.index}: fit positions must be ordered and unique`);
  require(scene.fitPositions[0] >= 1 && scene.fitPositions[2] <= 6,
    `scene ${record.index}: fit position is out of range`);
  require(scene.fitCaps.length === 3 && new Set(scene.fitCaps).size === 3,
    `scene ${record.index}: expected three distinct ordered fit caps`);
  require(JSON.stringify([...scene.fitCaps].sort()) === JSON.stringify(["0px", "10px", "20%"].sort()),
    `scene ${record.index}: unexpected cap assignment`);
  require(scene.trackMembers.length === 6, `scene ${record.index}: expected six authored tracks`);
  require(scene.trackMembers.filter((item) => item.startsWith("fit-content(")).length === 3,
    `scene ${record.index}: authored list must contain exactly three fit tracks`);
  require(scene.trackMembers.filter((item) => item === "min-content").length === 1
    && scene.trackMembers.filter((item) => item === "max-content").length === 1
    && scene.trackMembers.filter((item) => item === "auto").length === 1,
  `scene ${record.index}: tail multiplicity changed`);
  for (let index = 0; index < 3; index += 1) {
    require(scene.trackMembers[scene.fitPositions[index] - 1] === `fit-content(${scene.fitCaps[index]})`,
      `scene ${record.index}: cap assignment does not match its authored position`);
  }
  const fitPositions = new Set(scene.fitPositions);
  const tail = scene.trackMembers.filter((_, index) => !fitPositions.has(index + 1));
  require(JSON.stringify(tail) === JSON.stringify(scene.tail), `scene ${record.index}: tail order changed`);
  require(record.rows.length === 9 && record.items.length === 9,
    `scene ${record.index}: expected one explicit and eight implicit rows/items`);
  require(record.items[0].minHeight === "0px", `scene ${record.index}: explicit-row control changed`);
  require(record.items.slice(1).every((item) => item.minHeight === scene.minHeight),
    `scene ${record.index}: auto-placement item minimum differs from the scene`);
  require(record.rows.every((value) => Number.isFinite(value) && value >= 0),
    `scene ${record.index}: row size is not finite and non-negative`);
  require(record.computedAutoRows === scene.trackList,
    `scene ${record.index}: CSSOM track order or cap values changed`);
  require(record.computedAutoRows.trim().split(/\s+/).length === 6,
    `scene ${record.index}: computed row list must have six members`);
  require((record.computedAutoRows.match(/fit-content\(/g) ?? []).length === 3,
    `scene ${record.index}: computed list must preserve three fit tracks`);
  require(record.computedAlignContent === scene.alignContent,
    `scene ${record.index}: computed alignment changed`);
  require(scene.domOrder === "forward" || scene.domOrder === "reverse",
    `scene ${record.index}: unexpected DOM order`);
  require(scene.minHeight === "0px" || scene.minHeight === "auto",
    `scene ${record.index}: unexpected minimum mode`);
  require(Math.abs(record.gridFrame[0] - 40) <= 0.01 && Math.abs(record.gridFrame[1] - 220) <= 0.01,
    `scene ${record.index}: definite Grid frame changed`);
  require(Math.abs(record.rows[0] - 10) <= 0.01, `scene ${record.index}: explicit row changed`);
  require(record.items[0].id === "empty-0" && JSON.stringify(record.items[0].frame) === JSON.stringify([0, 0, 40, 10]),
    `scene ${record.index}: explicit-row control item changed`);

  const expectedIds = scene.domOrder === "forward" ? [1, 2, 3, 4, 5, 6, 7, 8] : [8, 7, 6, 5, 4, 3, 2, 1];
  let top = 10;
  for (let index = 0; index < expectedIds.length; index += 1) {
    const item = record.items[index + 1];
    const id = expectedIds[index];
    const lineCount = [2, 2, 2, 3, 2, 2, 3, 1][id - 1];
    const rowSize = record.rows[index + 1];
    require(item.id === String(id), `scene ${record.index}: source-order item changed at row ${index + 1}`);
    require(Math.abs(item.frame[0]) <= 0.01 && Math.abs(item.frame[1] - top) <= 0.01
      && Math.abs(item.frame[2] - 40) <= 0.01 && Math.abs(item.frame[3] - rowSize) <= 0.01,
    `scene ${record.index}: item frame does not match its implicit row`);
    require(item.payload !== null && Math.abs(item.payload[0]) <= 0.01 && Math.abs(item.payload[1] - top) <= 0.01
      && Math.abs(item.payload[2] - 40) <= 0.01 && Math.abs(item.payload[3] - lineCount * 10) <= 0.01,
    `scene ${record.index}: fixed text payload geometry changed`);
    require(item.lineCount === lineCount && item.lineTops.length === lineCount,
      `scene ${record.index}: payload line count changed for item ${id}`);
    require(item.lineTops.every((value, line) => line === 0 || value > item.lineTops[line - 1]),
      `scene ${record.index}: line positions are not strictly increasing`);
    top += rowSize;
  }
}

try {
  runBrowser(["open", pathToFileURL(fixturePath).href]);
  opened = true;
  runBrowser(["set", "viewport", "1280", "900"]);
  const count = Number(runBrowser(["eval", "window.__c114ThreeFitScenes.length"]));
  require(count === expectedSceneCount, `expected ${expectedSceneCount} scenes, got ${count}`);

  for (let start = 0; start < count; start += batchSize) {
    const size = Math.min(batchSize, count - start);
    const batch = parseEvalOutput(runBrowser([
      "eval",
      `JSON.stringify(window.runC114ThreeFitBatch(${start}, ${size}))`,
    ]));
    require(batch.start === start && batch.end === start + size && batch.total === count,
      `batch ${start}: returned bounds do not match the request`);
    captured.push(...batch.results);
    process.stdout.write(`CAPTURED ${captured.length}/${count}\n`);
  }

  require(captured.length === expectedSceneCount, `captured ${captured.length} scenes`);
  require(captured.every((record, index) => record.index === index), "scene indices are not continuous");
  for (const record of captured) validateScene(record);

  const trackLists = new Set(captured.map(({ scene }) => scene.trackList));
  require(trackLists.size === 720, `expected 720 authored lists, got ${trackLists.size}`);
  const metadata = {
    record: "metadata",
    schema: "spinon.c11-4.repeated-text-autoplacement-three-fit-auto-min-precomparison/v1",
    capturedAtUtc: new Date().toISOString(),
    browserCli,
    userAgent: captured[0].userAgent,
    platform: captured[0].platform,
    devicePixelRatio: captured[0].devicePixelRatio,
    viewportCssPx: [1280, 900],
    fixturePath: "spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-three-fit-auto-min-precomparison-2026-11-11/oracle-fixture.html",
    fixtureSha256,
    collectorPath: "spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-three-fit-auto-min-precomparison-2026-11-11/capture-chrome.mjs",
    collectorSha256,
    sceneCount: count,
    uniqueTrackListCount: trackLists.size,
    fitPositionTriples: 20,
    tailOrders: 6,
    orderedCapAssignments: 6,
    alignments: ["start", "stretch"],
    domOrders: ["forward", "reverse"],
    minimumModes: ["0px", "auto"],
  };
  const sceneLines = captured.map((record) => JSON.stringify({ record: "scene", ...record }));
  writeFileSync(observationPath, `${JSON.stringify(metadata)}\n${sceneLines.join("\n")}\n`);

  const sampleIndex = captured.findIndex(({ scene }) =>
    scene.fitPositions.join(",") === "1,3,6"
      && scene.fitCaps.join(",") === "0px,10px,20%"
      && scene.alignContent === "start"
      && scene.domOrder === "forward"
      && scene.minHeight === "auto");
  require(sampleIndex >= 0, "representative sample scene is missing");
  runBrowser(["eval", `JSON.stringify(window.showC114ThreeFitScene(${sampleIndex}))`]);
  runBrowser(["screenshot", screenshotPath]);
} finally {
  if (opened) {
    try {
      runBrowser(["close"]);
      closed = true;
    } catch (error) {
      closeError = String(error);
    }
  }
  writeFileSync(lifecyclePath, `${JSON.stringify({
    session,
    opened,
    closed,
    closeError,
    terminalBrowserUsed: false,
    simulatorStarted: false,
  }, null, 2)}\n`);
}

if (!closed) throw new Error(`Agent Browser session failed to close: ${closeError ?? "not opened"}`);
process.stdout.write(`Chrome three-fit auto-placement matrix: ${expectedSceneCount} scenes captured and session closed\n`);
