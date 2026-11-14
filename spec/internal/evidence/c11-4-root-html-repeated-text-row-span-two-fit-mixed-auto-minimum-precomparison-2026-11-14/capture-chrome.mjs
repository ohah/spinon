import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import {
  appendFileSync,
  createReadStream,
  createWriteStream,
  readFileSync,
  unlinkSync,
  writeFileSync,
} from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { createGzip } from "node:zlib";
import { pipeline } from "node:stream/promises";

const evidenceDir = dirname(fileURLToPath(import.meta.url));
const repositoryRoot = resolve(evidenceDir, "../../../..");
const fixturePath = resolve(
  repositoryRoot,
  "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-two-fit-mixed-auto-minimum-precomparison.html",
);
const rawObservationPath = resolve(evidenceDir, "chrome-observation.ndjson");
const observationPath = resolve(evidenceDir, "chrome-observation.ndjson.gz");
const metadataPath = resolve(evidenceDir, "capture.json");
const screenshotPath = resolve(evidenceDir, "chrome157-two-fit-mixed-sample.png");
const overflowScreenshotPath = resolve(evidenceDir, "chrome157-two-fit-mixed-overflow-sample.png");
const lifecyclePath = resolve(evidenceDir, "agent-browser-lifecycle.json");
const browserCli = process.env.AGENT_BROWSER_PATH ?? "/opt/homebrew/bin/agent-browser";
const session = "spinon-c114-two-fit-mixed-20261114";
const expectedSceneCount = 92_160;
const batchSize = 240;
const textById = {
  A: "AAAA BBBB CCCC DDDD EEEE FFFF",
  B: "AAAAAA BBBBB CCCC DDD",
  C: "AAAAA BBBB CCC DDDD",
  D: "AAAA BBB CCCCC DDDD",
  E: "AAA BBBB CCCCCC",
};

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

function ownSessionIsActive() {
  const output = execFileSync(browserCli, ["session", "list", "--json"], {
    cwd: repositoryRoot,
    encoding: "utf8",
    maxBuffer: 8 * 1024 * 1024,
    timeout: 30_000,
  });
  const result = JSON.parse(output);
  const sessions = result.data?.sessions ?? result.sessions ?? [];
  return sessions.some((entry) => entry.session === session || entry.name === session);
}

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

function validateRecord(record) {
  const { scene } = record;
  assert(scene.fitPositions.length === 2, `scene ${record.index}: two fit positions`);
  assert(scene.fitCaps.length === 2, `scene ${record.index}: two fit caps`);
  assert(scene.trackMembers.length === 6, `scene ${record.index}: six authored tracks`);
  assert(
    scene.trackMembers.filter((track) => track.startsWith("fit-content(")).length === 2 &&
      scene.trackMembers.filter((track) => track === "min-content").length === 2 &&
      scene.trackMembers.filter((track) => track === "max-content").length === 1 &&
      scene.trackMembers.filter((track) => track === "auto").length === 1,
    `scene ${record.index}: track multiset changed`,
  );
  assert(record.rowValues.length === 7, `scene ${record.index}: expected one explicit and six implicit rows`);
  assert(
    record.rowValues.every((value) => Number.isFinite(value) && value >= 0),
    `scene ${record.index}: rows must be finite and non-negative`,
  );
  assert(
    record.gridFrame.length === 4 &&
      record.gridFrame.every((value, index) => Math.abs(value - [0, 0, 40, 220][index]) <= 0.001),
    `scene ${record.index}: definite grid frame changed`,
  );
  assert(record.computedAutoRows === scene.trackList, `scene ${record.index}: track order changed`);
  assert(record.items.length === scene.items.length + 1, `scene ${record.index}: item count changed`);
  for (const item of record.items) {
    const expectedMinimum = scene.autoItemIds.includes(item.id) ? "auto" : "0px";
    assert(item.minHeight === expectedMinimum, `scene ${record.index}: ${item.id} minimum changed`);
    if (item.id !== "empty") {
      assert(item.text === textById[item.id], `scene ${record.index}: ${item.id} text changed`);
      assert(item.lineRects.length > 0, `scene ${record.index}: ${item.id} has no text line rects`);
      assert(
        item.lineRects.flat().every(Number.isFinite),
        `scene ${record.index}: ${item.id} line rect contains a non-finite value`,
      );
    }
  }
}

const fixtureBytes = readFileSync(fixturePath);
const collectorBytes = readFileSync(fileURLToPath(import.meta.url));
const fixtureSha256 = createHash("sha256").update(fixtureBytes).digest("hex");
const collectorSha256 = createHash("sha256").update(collectorBytes).digest("hex");
let opened = false;
let closed = false;
let closeError = null;
let ownSessionActiveAfterClose = null;
let firstRecord = null;
let captured = 0;
let completed = false;
let sampleIndex = -1;
let overflowSampleIndex = -1;

try {
  writeFileSync(rawObservationPath, "");
  runBrowser(["open", pathToFileURL(fixturePath).href]);
  opened = true;
  runBrowser(["set", "viewport", "1280", "900"]);
  const pageInfo = parseEvalOutput(
    runBrowser([
      "eval",
      "JSON.stringify({count:window.__c114DualFitMixedScenes.length,ua:navigator.userAgent,dpr:devicePixelRatio,viewport:[innerWidth,innerHeight]})",
    ]),
  );
  assert(pageInfo.count === expectedSceneCount, `expected ${expectedSceneCount} scenes; got ${pageInfo.count}`);
  assert(pageInfo.ua.includes("HeadlessChrome/157."), `expected Chrome 157; got ${pageInfo.ua}`);
  assert(pageInfo.dpr === 1, `expected DPR 1; got ${pageInfo.dpr}`);
  assert(JSON.stringify(pageInfo.viewport) === JSON.stringify([1280, 900]), "viewport changed");

  for (let start = 0; start < expectedSceneCount; start += batchSize) {
    const count = Math.min(batchSize, expectedSceneCount - start);
    const batch = parseEvalOutput(
      runBrowser([
        "eval",
        `JSON.stringify(window.runC114DualFitMixedBatch(${start}, ${count}))`,
      ]),
    );
    assert(batch.start === start && batch.end === start + count, `batch ${start}: bounds changed`);
    assert(batch.total === expectedSceneCount && batch.results.length === count, `batch ${start}: count changed`);
    const outputLines = [];
    for (const record of batch.results) {
      assert(record.index === captured, `scene index discontinuity at ${captured}`);
      validateRecord(record);
      if (firstRecord === null) firstRecord = record;
      outputLines.push(JSON.stringify(record));
      captured += 1;
    }
    appendFileSync(rawObservationPath, `${outputLines.join("\n")}\n`);
    if (captured % (batchSize * 10) === 0 || captured === expectedSceneCount) {
      process.stdout.write(`CAPTURED ${captured}/${expectedSceneCount}\n`);
    }
  }

  assert(captured === expectedSceneCount, `captured ${captured} scenes`);
  sampleIndex = Number(
    runBrowser([
      "eval",
      "window.__c114DualFitMixedScenes.findIndex((scene) => scene.fitPositions.join(',') === '2,5' && scene.fitCaps.join(',') === '10px,20px' && scene.alignContent === 'stretch' && scene.topology === 'span2Span3Six-reverse' && scene.minimumMask === 5)",
    ]),
  );
  assert(sampleIndex >= 0, "representative mixed scene not found");
  runBrowser(["eval", `JSON.stringify(window.showC114DualFitMixedScene(${sampleIndex}))`]);
  runBrowser(["screenshot", screenshotPath]);
  overflowSampleIndex = Number(
    runBrowser([
      "eval",
      "window.__c114DualFitMixedScenes.findIndex((scene) => scene.fitPositions[0]===4 && scene.fitPositions[1]===5 && /^0px$/.test(scene.fitCaps[0]) && /^0%$/.test(scene.fitCaps[1]) && /^stretch$/.test(scene.alignContent) && /^span2Six-forward$/.test(scene.topology) && scene.minimumMask===0)",
    ]),
  );
  assert(overflowSampleIndex >= 0, "visible-overflow control scene not found");
  runBrowser(["eval", `JSON.stringify(window.showC114DualFitMixedScene(${overflowSampleIndex}))`]);
  runBrowser(["screenshot", overflowScreenshotPath]);
  completed = true;
} finally {
  try {
    runBrowser(["close"]);
    closed = true;
  } catch (error) {
    closeError = String(error);
  }
  try {
    ownSessionActiveAfterClose = ownSessionIsActive();
  } catch (error) {
    closeError = [closeError, `session-list check failed: ${String(error)}`].filter(Boolean).join("; ");
  }
  writeFileSync(
    lifecyclePath,
    `${JSON.stringify(
      {
        session,
        opened,
        closed,
        closeError,
        ownSessionActiveAfterClose,
        captured,
        completed,
      },
      null,
      2,
    )}\n`,
  );
}

assert(closed, `Agent Browser session was not closed: ${closeError ?? "unknown error"}`);
assert(ownSessionActiveAfterClose === false, "Agent Browser session remains active after close");
assert(completed && captured === expectedSceneCount, "capture did not complete");
await pipeline(
  createReadStream(rawObservationPath),
  createGzip({ level: 9, mtime: 0 }),
  createWriteStream(observationPath),
);
unlinkSync(rawObservationPath);

const observationSha256 = createHash("sha256").update(readFileSync(observationPath)).digest("hex");
const mixedScreenshotSha256 = createHash("sha256").update(readFileSync(screenshotPath)).digest("hex");
const overflowScreenshotSha256 = createHash("sha256").update(readFileSync(overflowScreenshotPath)).digest("hex");
writeFileSync(
  metadataPath,
  `${JSON.stringify(
    {
      schema: "spinon.c11-4.root-html-row-span-two-fit-mixed.capture/v1",
      capturedAtUtc: new Date().toISOString(),
      browserCli,
      browserUserAgent: firstRecord?.userAgent ?? null,
      devicePixelRatio: firstRecord?.devicePixelRatio ?? null,
      viewport: [1280, 900],
      sceneCount: captured,
      batchSize,
      fixture: {
        path: "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-two-fit-mixed-auto-minimum-precomparison.html",
        sha256: fixtureSha256,
      },
      collector: {
        path: "spec/internal/evidence/c11-4-root-html-repeated-text-row-span-two-fit-mixed-auto-minimum-precomparison-2026-11-14/capture-chrome.mjs",
        sha256: collectorSha256,
      },
      observation: {
        path: "chrome-observation.ndjson.gz",
        sha256: observationSha256,
      },
      screenshots: {
        mixed: {
          path: "chrome157-two-fit-mixed-sample.png",
          sha256: mixedScreenshotSha256,
        },
        visibleOverflow: {
          path: "chrome157-two-fit-mixed-overflow-sample.png",
          sha256: overflowScreenshotSha256,
        },
      },
      representativeScene: sampleIndex,
      visibleOverflowControlScene: overflowSampleIndex,
      visibleOverflowScreenshot: "chrome157-two-fit-mixed-overflow-sample.png",
      lifecycle: "agent-browser-lifecycle.json",
    },
    null,
    2,
  )}\n`,
);
process.stdout.write(`CAPTURED ${captured}/${expectedSceneCount}; CLOSED ${closed}; SAMPLE ${sampleIndex}\n`);
