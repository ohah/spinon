import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const evidenceDir = dirname(fileURLToPath(import.meta.url));
const repositoryRoot = resolve(evidenceDir, "../../../..");
const fixturePath = resolve(
  repositoryRoot,
  "tests/fixtures/css/c11/root-html-grid-repeated-text-autoplacement-two-fit-content-precomparison.html",
);
const observationPath = resolve(evidenceDir, "chrome-matrix-observation.ndjson");
const screenshotPath = resolve(evidenceDir, "chrome157-two-fit-sample.png");
const lifecyclePath = resolve(evidenceDir, "agent-browser-lifecycle.json");
const browserCli = process.env.AGENT_BROWSER_PATH ?? "/opt/homebrew/bin/agent-browser";
const session = "spinon-c11-4-two-fit-autoplacement-precomparison-20261110";
const expectedSceneCount = 2_880;
const batchSize = 120;

mkdirSync(evidenceDir, { recursive: true });
const fixture = readFileSync(fixturePath);
const fixtureSha256 = createHash("sha256").update(fixture).digest("hex");
const collectorSha256 = createHash("sha256")
  .update(readFileSync(fileURLToPath(import.meta.url)))
  .digest("hex");
let opened = false;
let closed = false;
let closeError = null;
let captured = [];

function runBrowser(args, timeout = 120_000) {
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

function near(actual, expected, tolerance = 0.01) {
  return Number.isFinite(actual) && Math.abs(actual - expected) <= tolerance;
}

function require(condition, message) {
  if (!condition) throw new Error(message);
}

function validateScene(record) {
  const { scene } = record;
  require(scene.fitPositions.length === 2, `scene ${record.index}: expected two fit positions`);
  require(scene.fitPositions[0] < scene.fitPositions[1], `scene ${record.index}: fit positions must be unique and ordered`);
  require(scene.fitPositions[0] >= 1 && scene.fitPositions[1] <= 6, `scene ${record.index}: fit position out of range`);
  require(scene.fitCaps.length === 2, `scene ${record.index}: expected two ordered fit caps`);
  require(scene.trackMembers.filter((item) => item.startsWith("fit-content(")).length === 2,
    `scene ${record.index}: authored list must contain exactly two fit tracks`);
  require(scene.trackMembers.filter((item) => item === "min-content").length === 2,
    `scene ${record.index}: authored list must contain two min-content tracks`);
  require(scene.trackMembers.filter((item) => item === "max-content").length === 1,
    `scene ${record.index}: authored list must contain one max-content track`);
  require(scene.trackMembers.filter((item) => item === "auto").length === 1,
    `scene ${record.index}: authored list must contain one auto track`);
  for (let index = 0; index < 2; index += 1) {
    require(scene.trackMembers[scene.fitPositions[index] - 1] === `fit-content(${scene.fitCaps[index]})`,
      `scene ${record.index}: cap assignment does not match authored position`);
  }
  const fitPositions = new Set(scene.fitPositions);
  const tail = scene.trackMembers.filter((_, index) => !fitPositions.has(index + 1));
  require(JSON.stringify(tail) === JSON.stringify(scene.tail), `scene ${record.index}: tail order changed`);
  require(record.rows.length === 9 && record.items.length === 9,
    `scene ${record.index}: expected one explicit and eight implicit rows/items`);
  require(record.rows.every((value) => Number.isFinite(value) && value >= 0),
    `scene ${record.index}: row size is not finite and non-negative`);
  require(record.computedAutoRows === scene.trackList,
    `scene ${record.index}: CSSOM track order or cap values changed`);
  require(record.computedAutoRows.trim().split(/\s+/).length === 6,
    `scene ${record.index}: computed row list must have six members`);
  require((record.computedAutoRows.match(/fit-content\(/g) ?? []).length === 2,
    `scene ${record.index}: computed list must preserve two fit tracks`);
  require(record.computedAlignContent === scene.alignContent,
    `scene ${record.index}: computed alignment changed`);
  require(near(record.gridFrame[0], 40) && near(record.gridFrame[1], 220),
    `scene ${record.index}: definite Grid frame changed`);
  require(near(record.rows[0], 10), `scene ${record.index}: explicit row changed`);
  require(near(record.items[0].frame[0], 0) && near(record.items[0].frame[1], 0)
    && near(record.items[0].frame[2], 40) && near(record.items[0].frame[3], 10),
  `scene ${record.index}: explicit-row control item changed`);

  let top = 10;
  for (let index = 0; index < 8; index += 1) {
    const item = record.items[index + 1];
    const rowIndex = index + 1;
    const rowSize = record.rows[rowIndex];
    const lineCount = [2, 2, 2, 3, 2, 2, 3, 1][index];
    require(item.id === String(index + 1), `scene ${record.index}: auto-placement DOM order changed`);
    require(near(item.frame[0], 0) && near(item.frame[1], top)
      && near(item.frame[2], 40) && near(item.frame[3], rowSize),
    `scene ${record.index}: item frame does not match its implicit row`);
    require(item.payload !== null && near(item.payload[0], 0) && near(item.payload[1], top)
      && near(item.payload[2], 40) && near(item.payload[3], lineCount * 10),
    `scene ${record.index}: fixed text payload geometry changed`);
    require(item.lineCount === lineCount, `scene ${record.index}: text line count changed`);
    require(item.lineTops.length === lineCount
      && item.lineTops.every((value, line) => line === 0 || value > item.lineTops[line - 1]),
    `scene ${record.index}: text line positions changed`);
    top += rowSize;
  }
}

try {
  runBrowser(["open", pathToFileURL(fixturePath).href]);
  opened = true;
  runBrowser(["set", "viewport", "1280", "900"]);
  const count = Number(runBrowser(["eval", "window.__c114TwoFitScenes.length"]));
  require(count === expectedSceneCount, `expected ${expectedSceneCount} scenes, got ${count}`);

  for (let start = 0; start < count; start += batchSize) {
    const size = Math.min(batchSize, count - start);
    const batch = parseEvalOutput(runBrowser([
      "eval",
      `JSON.stringify(window.runC114TwoFitBatch(${start}, ${size}))`,
    ], 180_000));
    require(batch.start === start && batch.end === start + size && batch.total === count,
      `batch ${start}: returned bounds do not match request`);
    captured.push(...batch.results);
    process.stdout.write(`CAPTURED ${captured.length}/${count}\n`);
  }

  require(captured.length === expectedSceneCount, `captured ${captured.length} scenes`);
  require(captured.every((record, index) => record.index === index), "scene indices are not continuous");
  const uniqueLists = new Set(captured.map((record) => record.scene.trackList));
  require(uniqueLists.size === 1_440, `expected 1,440 unique authored lists, got ${uniqueLists.size}`);
  for (const record of captured) validateScene(record);

  writeFileSync(
    observationPath,
    `${captured.map((record) => JSON.stringify(record)).join("\n")}\n`,
  );
  const sampleIndex = captured.findIndex(({ scene }) =>
    scene.fitPositions.join(",") === "2,5"
      && scene.fitCaps.join(",") === "10px,20%"
      && scene.alignContent === "stretch");
  require(sampleIndex >= 0, "representative sample scene is missing");
  runBrowser(["eval", `JSON.stringify(window.showC114TwoFitScene(${sampleIndex}))`]);
  runBrowser(["screenshot", screenshotPath]);

  const metadata = {
    record: "metadata",
    schema: "spinon.c11-4.repeated-text-autoplacement-two-fit-content-precomparison/v1",
    capturedAtUtc: new Date().toISOString(),
    browserCli,
    userAgent: captured[0].userAgent,
    platform: captured[0].platform,
    devicePixelRatio: captured[0].devicePixelRatio,
    viewportCssPx: [1280, 900],
    fixturePath: "tests/fixtures/css/c11/root-html-grid-repeated-text-autoplacement-two-fit-content-precomparison.html",
    fixtureSha256,
    collectorPath: "spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-two-fit-content-precomparison-2026-11-10/capture-chrome.mjs",
    collectorSha256,
    sceneCount: count,
    uniqueTrackListCount: uniqueLists.size,
    fitPositionPairs: 15,
    orderedCapAssignments: 8,
    tailOrders: 12,
    alignments: ["start", "stretch"],
  };
  const sceneLines = captured.map((record) => JSON.stringify({ record: "scene", ...record }));
  writeFileSync(observationPath, `${JSON.stringify(metadata)}\n${sceneLines.join("\n")}\n`);
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
process.stdout.write(`Chrome matrix: ${expectedSceneCount} scenes · ${captured.length === expectedSceneCount ? "captured" : "partial"}\n`);
