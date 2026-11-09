import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const evidenceDir = dirname(fileURLToPath(import.meta.url));
const repositoryRoot = resolve(evidenceDir, "../../../..");
const fixturePath = resolve(evidenceDir, "oracle-fixture.html");
const observationPath = resolve(evidenceDir, "chrome-matrix-observation.ndjson");
const screenshotPath = resolve(evidenceDir, "chrome157-one-fit-auto-min-sample.png");
const lifecyclePath = resolve(evidenceDir, "agent-browser-lifecycle.json");
const browserCli = process.env.AGENT_BROWSER_PATH ?? "/opt/homebrew/bin/agent-browser";
const session = "spinon-c114-one-fit-auto-min-precomparison-20261110";
const expectedSceneCount = 1_440;

const fixtureSha256 = createHash("sha256").update(readFileSync(fixturePath)).digest("hex");
const collectorSha256 = createHash("sha256")
  .update(readFileSync(fileURLToPath(import.meta.url)))
  .digest("hex");
let opened = false;
let closed = false;
let closeError = null;

function runBrowser(args, timeout = 120_000) {
  return execFileSync(browserCli, ["--session", session, ...args], {
    cwd: repositoryRoot,
    encoding: "utf8",
    maxBuffer: 64 * 1024 * 1024,
    timeout,
  }).trim();
}

function parseEvalOutput(output) {
  let decoded = JSON.parse(output);
  if (typeof decoded === "string") decoded = JSON.parse(decoded);
  return decoded;
}

function require(condition, message) {
  if (!condition) throw new Error(message);
}

try {
  runBrowser(["open", pathToFileURL(fixturePath).href]);
  opened = true;
  runBrowser(["set", "viewport", "1280", "900"]);
  const matrix = parseEvalOutput(
    runBrowser(["eval", "JSON.stringify(window.captureMatrix())"], 240_000),
  );
  require(matrix.caseCount === expectedSceneCount, `expected ${expectedSceneCount} scenes, got ${matrix.caseCount}`);
  require(matrix.scenes.length === expectedSceneCount, `captured ${matrix.scenes.length} scenes`);
  require(matrix.uniqueTailOrders === 30, `expected 30 tail orders, got ${matrix.uniqueTailOrders}`);
  require(matrix.fitContentPositions.join(",") === "1,2,3,4,5,6", "fit position coverage changed");
  require(matrix.dpr === 1, `expected DPR 1, got ${matrix.dpr}`);
  require(matrix.platform === "MacIntel", `unexpected platform ${matrix.platform}`);
  require(matrix.userAgent.includes("HeadlessChrome/157.0.0.0"), `unexpected browser ${matrix.userAgent}`);

  const rows = matrix.scenes.map((scene, index) => {
    require(scene.items.length === 9, `scene ${index}: expected 9 items`);
    require(scene.items[0].minHeight === "0px", `scene ${index}: control min-height changed`);
    require(scene.items.slice(1).every((item) => item.minHeight === "auto"),
      `scene ${index}: text item lost computed min-height:auto`);
    return { record: "scene", index, ...scene };
  });

  const metadata = {
    record: "metadata",
    schema: "spinon.c11-4.repeated-text-autoplacement-one-fit-auto-min-precomparison/v1",
    capturedAtUtc: new Date().toISOString(),
    browserCli,
    userAgent: matrix.userAgent,
    platform: matrix.platform,
    devicePixelRatio: matrix.dpr,
    viewportCssPx: [1280, 900],
    fixturePath: "spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-one-fit-auto-min-precomparison-2026-11-10/oracle-fixture.html",
    fixtureSha256,
    collectorPath: "spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-one-fit-auto-min-precomparison-2026-11-10/capture-chrome.mjs",
    collectorSha256,
    sceneCount: expectedSceneCount,
    uniqueTrackListCount: new Set(rows.map((scene) => scene.authoredAutoRows.join(" "))).size,
    fitPositions: 6,
    tailOrders: 30,
    caps: ["20px", "20%", "0%", "120%"],
    alignments: ["start", "stretch"],
    payloads: matrix.payloads,
    payloadLineCounts: matrix.payloadLineCounts,
  };
  writeFileSync(observationPath, `${JSON.stringify(metadata)}\n${rows.map((scene) => JSON.stringify(scene)).join("\n")}\n`);

  const sampleIndex = rows.findIndex((scene) =>
    scene.fitPosition === 4 && scene.cap === "0%" && scene.align === "start");
  require(sampleIndex >= 0, "sample scene missing");
  const sample = rows[sampleIndex];
  const tail = [...sample.tail];
  tail.splice(sample.fitPosition - 1, 0, `fit-content(${sample.cap})`);
  runBrowser(["eval", `(() => { const grid = document.querySelector('#sample-grid'); grid.style.gridAutoRows = ${JSON.stringify(tail.join(" "))}; grid.style.alignContent = 'start'; document.querySelector('#sample-summary').textContent = JSON.stringify({tracks: getComputedStyle(grid).gridAutoRows, rows: getComputedStyle(grid).gridTemplateRows, itemFrames: [...grid.children].map(node => { const r = node.getBoundingClientRect(); return [r.x, r.y, r.width, r.height] })}, null, 2); return true })()`]);
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
process.stdout.write(`Chrome one-fit auto-min matrix: ${expectedSceneCount} scenes captured\n`);
