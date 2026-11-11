import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const evidenceDir = dirname(fileURLToPath(import.meta.url));
const repositoryRoot = resolve(evidenceDir, "../../../..");
const fixtureRelativePath =
  "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-four-fit-auto-minimum-precomparison.html";
const fixturePath = resolve(repositoryRoot, fixtureRelativePath);
const observationPath = resolve(evidenceDir, "chrome157-observation.ndjson");
const screenshotPath = resolve(evidenceDir, "chrome157-four-fit-auto-minimum-sample.png");
const lifecyclePath = resolve(evidenceDir, "agent-browser-lifecycle.json");
const browserCli = process.env.AGENT_BROWSER_PATH ?? "/opt/homebrew/bin/agent-browser";
const session = "spinon-c114-four-fit-row-span-auto-minimum-20261111";
const expectedSceneCount = 11_520;
const batchSize = 240;

mkdirSync(evidenceDir, { recursive: true });
const fixtureSha256 = createHash("sha256").update(readFileSync(fixturePath)).digest("hex");
const captureScriptSha256 = createHash("sha256")
  .update(readFileSync(fileURLToPath(import.meta.url)))
  .digest("hex");
let opened = false;
let closed = false;
let closeError = null;
const captured = [];

function runBrowser(args, timeout = 120_000) {
  return execFileSync(browserCli, ["--session", session, ...args], {
    cwd: repositoryRoot,
    encoding: "utf8",
    maxBuffer: 32 * 1024 * 1024,
    timeout,
  }).trim();
}

function parseEvalOutput(output) {
  const decoded = JSON.parse(output);
  return typeof decoded === "string" ? JSON.parse(decoded) : decoded;
}

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

try {
  runBrowser(["open", pathToFileURL(fixturePath).href]);
  opened = true;
  runBrowser(["set", "viewport", "1280", "900"]);
  const sceneCount = Number(runBrowser(["eval", "window.__c114FourFitScenes.length"]));
  assert(sceneCount === expectedSceneCount, `expected ${expectedSceneCount} scenes, got ${sceneCount}`);

  for (let start = 0; start < sceneCount; start += batchSize) {
    const end = Math.min(start + batchSize, sceneCount);
    const output = runBrowser([
      "eval",
      `JSON.stringify(window.runC114FourFitBatch(${start}, ${end - start}))`,
    ]);
    const batch = parseEvalOutput(output);
    assert(batch.start === start && batch.end === end && batch.total === sceneCount,
      `batch bounds mismatch at ${start}`);
    captured.push(...batch.results);
    if (captured.length % 960 === 0 || captured.length === sceneCount) {
      process.stdout.write(`CAPTURED ${captured.length}/${sceneCount}\n`);
    }
  }

  assert(captured.length === expectedSceneCount, "captured scene count mismatch");
  assert(captured.every((record, index) => record.index === index), "scene indices are incomplete");
  writeFileSync(observationPath, `${captured.map((record) => JSON.stringify(record)).join("\n")}\n`);

  const sampleIndex = captured.findIndex(({ scene }) =>
    scene.fitPositions.join(",") === "1,3,4,6" &&
    scene.fitCaps.join(",") === "0px,10px,20%,30px" &&
    scene.tail.join(",") === "min-content,auto" &&
    scene.alignContent === "start" &&
    scene.topology === "span2Span3Six-forward" &&
    scene.minimum === "auto");
  assert(sampleIndex >= 0, "sample scene not found");
  runBrowser(["eval", `JSON.stringify(window.showC114FourFitScene(${sampleIndex}))`]);
  runBrowser(["eval", `(() => { const grid = document.querySelector("#grid"); grid.style.transform = "scale(4)"; grid.style.transformOrigin = "top left"; return true; })()`]);
  runBrowser(["set", "viewport", "480", "900"]);
  runBrowser(["screenshot", screenshotPath]);

  const manifest = {
    schema: "spinon.c11-4.repeated-text-row-span-four-fit-auto-minimum-precomparison.capture.v1",
    capturedAtUtc: new Date().toISOString(),
    browserCli,
    session,
    browserUserAgent: captured[0].userAgent,
    devicePixelRatio: captured[0].devicePixelRatio,
    fixture: { path: fixtureRelativePath, sha256: fixtureSha256 },
    collector: {
      path: "spec/internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-auto-minimum-precomparison-2026-11-11/capture-chrome.mjs",
      sha256: captureScriptSha256,
    },
    sceneCount: captured.length,
    observations: "chrome157-observation.ndjson",
    sampleScreenshot: "chrome157-four-fit-auto-minimum-sample.png",
  };
  writeFileSync(resolve(evidenceDir, "capture-manifest.json"), `${JSON.stringify(manifest, null, 2)}\n`);
} finally {
  if (opened) {
    try {
      runBrowser(["close"]);
      closed = true;
    } catch (error) {
      closeError = error.message;
      process.stderr.write(`Agent Browser close failed: ${error.message}\n`);
    }
  }
  writeFileSync(
    lifecyclePath,
    `${JSON.stringify({ browserCli, session, opened, closed, observedAtUtc: new Date().toISOString() }, null, 2)}\n`,
  );
  if (closeError) throw new Error(`Agent Browser cleanup failed: ${closeError}`);
}
