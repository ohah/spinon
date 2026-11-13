import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { createWriteStream, existsSync, readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { once } from "node:events";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { createGzip } from "node:zlib";

const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, "../../../..");
const fixtureRelative = "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-one-fit-mixed-auto-minimum-precomparison.html";
const fixture = join(root, fixtureRelative);
const captureScriptRelative = "spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-mixed-auto-minimum-precomparison-2026-11-14/capture-chrome.mjs";
const chromeBinaryPath = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
const output = join(here, "chrome157-one-fit-mixed-observation.ndjson.gz");
const screenshot = join(here, "chrome157-one-fit-mixed-sample.png");
const manifestPath = join(here, "capture-manifest.json");
const lifecycle = join(here, "agent-browser-lifecycle.json");
const browserCli = process.env.AGENT_BROWSER_PATH ?? "/opt/homebrew/bin/agent-browser";
const namespace = "spinononefitmix14";
const session = "onefitmixpre";
const expectedSceneCount = 120_960;
const expectedTrackLists = 900;
const batchSize = 240;
const run = (args, timeout = 120_000) => execFileSync(browserCli, [
  "--namespace", namespace,
  "--session", session,
  ...args,
], { cwd: root, encoding: "utf8", maxBuffer: 64 * 1024 * 1024, timeout }).trim();
const parse = (value) => {
  const decoded = JSON.parse(value);
  return typeof decoded === "string" ? JSON.parse(decoded) : decoded;
};
const gzip = createGzip({ level: 9 });
const outputStream = createWriteStream(output);
gzip.pipe(outputStream);
const writeGzip = async (chunk) => {
  if (!gzip.write(chunk)) await once(gzip, "drain");
};

let openAttempted = false;
let closed = false;
let finished = false;
let capturedCount = 0;
let fallbackSampleIndex = -1;
let overflowSampleIndex = -1;
const trackLists = new Set();
const topologyCounts = new Map();
const masksByTopology = new Map();
const sceneCountsByFitPosition = new Map();

try {
  openAttempted = true;
  run(["open", pathToFileURL(fixture).href]);
  run(["set", "viewport", "1280", "900"]);
  const sceneCount = Number(run(["eval", "window.__c114OneFitMixedSceneCount"]));
  if (sceneCount !== expectedSceneCount) {
    throw new Error("expected " + expectedSceneCount + " scenes, got " + sceneCount);
  }
  const trackListCount = Number(run(["eval", "window.__c114OneFitMixedTrackListCount"]));
  if (trackListCount !== expectedTrackLists) {
    throw new Error("expected " + expectedTrackLists + " track lists, got " + trackListCount);
  }

  const sentinelIndices = [0, 1, 3, 4, 7, 8, 15, 16, 23, 24, 47, 48, 95, 96, 191, 5759, 5760, 17279, 17280, 120959];
  const sentinels = parse(run([
    "eval",
    "JSON.stringify(" + JSON.stringify(sentinelIndices) + ".map((index) => window.c114OneFitMixedSceneAt(index)))",
  ]));
  const sentinelByIndex = Object.fromEntries(sentinelIndices.map((index, offset) => [index, sentinels[offset]]));
  if (sentinelByIndex[0].fitPosition !== 1 || sentinelByIndex[0].minimumMask !== 0
    || sentinelByIndex[1].minimumMask !== 1 || sentinelByIndex[3].minimumMask !== 3
    || sentinelByIndex[4].topology !== "span2-reversed"
    || sentinelByIndex[8].topology !== "span2-before-span3"
    || sentinelByIndex[16].topology !== "span3-before-span2"
    || sentinelByIndex[23].alignContent !== "start"
    || sentinelByIndex[24].alignContent !== "stretch"
    || sentinelByIndex[47].alignContent !== "stretch"
    || sentinelByIndex[95].fitPosition !== 1
    || sentinelByIndex[96].fitPosition !== 1
    || sentinelByIndex[5759].fitPosition !== 1
    || sentinelByIndex[5760].fitPosition !== 2
    || sentinelByIndex[17279].fitPosition !== 3
    || sentinelByIndex[17280].fitPosition !== 4
    || sentinelByIndex[120959].fitPosition !== 6) {
    throw new Error("scene-index sentinels do not match the documented order");
  }

  const browser = parse(run([
    "eval",
    "JSON.stringify({ userAgent: navigator.userAgent, platform: navigator.platform, devicePixelRatio: window.devicePixelRatio })",
  ]));
  if (!browser.userAgent.includes("HeadlessChrome/157.")) {
    throw new Error("expected pinned HeadlessChrome 157, got " + browser.userAgent);
  }
  const fixtureBytes = readFileSync(fixture);
  const fixtureSha256 = createHash("sha256").update(fixtureBytes).digest("hex");
  const captureScriptSha256 = createHash("sha256").update(readFileSync(fileURLToPath(import.meta.url))).digest("hex");
  const chromeBinarySha256 = createHash("sha256").update(readFileSync(chromeBinaryPath)).digest("hex");
  const metadata = {
    record: "metadata",
    schema: "spinon.c11-4.repeated-text-row-span-one-fit-mixed-auto-min-precomparison/v1",
    capturedAtUtc: new Date().toISOString(),
    browserCli: execFileSync(browserCli, ["--version"], { cwd: root, encoding: "utf8" }).trim(),
    collector: { path: captureScriptRelative, sha256: captureScriptSha256 },
    chromeBinary: { path: chromeBinaryPath, sha256: chromeBinarySha256 },
    environment: {
      macOS: execFileSync("sw_vers", ["-productVersion"], { encoding: "utf8" }).trim(),
      architecture: execFileSync("uname", ["-m"], { encoding: "utf8" }).trim(),
      node: process.version,
    },
    namespace,
    session,
    userAgent: browser.userAgent,
    platform: browser.platform,
    devicePixelRatio: browser.devicePixelRatio,
    viewportCssPx: [1280, 900],
    fixturePath: fixtureRelative,
    fixtureBytes: fixtureBytes.length,
    fixtureSha256,
    sceneCount,
    uniqueTrackListCount: expectedTrackLists,
    fitPositions: [1, 2, 3, 4, 5, 6],
    fitCapsByPosition: {
      "1-3": ["20px", "20%", "0%", "120%"],
      "4-6": ["0px", "0%", "10px", "20px", "20%", "120%"],
    },
    uniqueTailOrders: 30,
    alignments: ["start", "stretch"],
    topologies: [
      "span2-forward",
      "span2-reversed",
      "span2-before-span3",
      "span3-before-span2",
      "span2-six-forward",
      "span2-six-reversed",
      "span2-span3-six-forward",
      "span2-span3-six-reversed",
    ],
    minimumMaskCounts: {
      "span2-forward": 4,
      "span2-reversed": 4,
      "span2-before-span3": 8,
      "span3-before-span2": 8,
      "span2-six-forward": 32,
      "span2-six-reversed": 32,
      "span2-span3-six-forward": 16,
      "span2-span3-six-reversed": 16,
    },
    observationPath: "chrome157-one-fit-mixed-observation.ndjson.gz",
    sampleScreenshot: "chrome157-one-fit-mixed-sample.png",
  };
  await writeGzip(JSON.stringify(metadata) + "\n");

  for (let start = 0; start < sceneCount; start += batchSize) {
    const size = Math.min(batchSize, sceneCount - start);
    const batch = parse(run([
      "eval",
      "JSON.stringify(window.runC114OneFitMixedBatch(" + start + ", " + size + "))",
    ]));
    if (batch.start !== start || batch.end !== start + size || batch.total !== sceneCount) {
      throw new Error("batch " + start + " bounds mismatch");
    }
    for (const record of batch.results) {
      if (record.index !== capturedCount) {
        throw new Error("non-contiguous scene order at " + capturedCount);
      }
      const scene = record.scene;
      trackLists.add(scene.trackList);
      topologyCounts.set(scene.topology, (topologyCounts.get(scene.topology) ?? 0) + 1);
      const masks = masksByTopology.get(scene.topology) ?? new Set();
      masks.add(scene.minimumMask);
      masksByTopology.set(scene.topology, masks);
      sceneCountsByFitPosition.set(
        scene.fitPosition,
        (sceneCountsByFitPosition.get(scene.fitPosition) ?? 0) + 1,
      );

      const maximumMask = (1 << scene.maskItemIds.length) - 1;
      const mixed = scene.minimumMask !== 0 && scene.minimumMask !== maximumMask;
      if (fallbackSampleIndex < 0 && mixed && scene.autoItems.length === 1
        && scene.alignContent === "stretch") {
        fallbackSampleIndex = record.index;
      }
      const gridContentBottom = record.gridFrame[3];
      if (overflowSampleIndex < 0 && mixed && scene.fitPosition >= 4
        && scene.alignContent === "stretch"
        && scene.topology === "span2-six-forward"
        && record.items.some((item) => item.payloadFrame
          && item.payloadFrame[1] + item.payloadFrame[3] > gridContentBottom + 0.01)) {
        overflowSampleIndex = record.index;
      }
      capturedCount += 1;
    }
    await writeGzip(
      batch.results.map((record) => JSON.stringify({ record: "scene", ...record })).join("\n") + "\n",
    );
    if (capturedCount % 9_600 === 0 || capturedCount === sceneCount) {
      process.stdout.write("CAPTURED " + capturedCount + "/" + sceneCount + "\n");
    }
  }

  if (capturedCount !== sceneCount) throw new Error("captured " + capturedCount);
  if (trackLists.size !== expectedTrackLists) {
    throw new Error("expected " + expectedTrackLists + " track lists, got " + trackLists.size);
  }
  const expectedTopologyCounts = {
    "span2-forward": 2_880,
    "span2-reversed": 2_880,
    "span2-before-span3": 5_760,
    "span3-before-span2": 5_760,
    "span2-six-forward": 34_560,
    "span2-six-reversed": 34_560,
    "span2-span3-six-forward": 17_280,
    "span2-span3-six-reversed": 17_280,
  };
  for (const [topology, expectedCount] of Object.entries(expectedTopologyCounts)) {
    if (topologyCounts.get(topology) !== expectedCount) {
      throw new Error(topology + ": expected " + expectedCount + " scenes, got " + topologyCounts.get(topology));
    }
    if (masksByTopology.get(topology)?.size !== metadata.minimumMaskCounts[topology]) {
      throw new Error(topology + ": incomplete minimum masks");
    }
  }
  const expectedCountsByPosition = {
    1: 5_760,
    2: 5_760,
    3: 5_760,
    4: 34_560,
    5: 34_560,
    6: 34_560,
  };
  for (const [position, expectedCount] of Object.entries(expectedCountsByPosition)) {
    if (sceneCountsByFitPosition.get(Number(position)) !== expectedCount) {
      throw new Error("fit position " + position + " has an incomplete scene matrix");
    }
  }

  gzip.end();
  await once(outputStream, "finish");
  finished = true;
  const sampleIndex = overflowSampleIndex >= 0 ? overflowSampleIndex : fallbackSampleIndex;
  if (sampleIndex < 0) throw new Error("representative mixed-mask scene missing");
  run(["eval", "JSON.stringify(window.showC114OneFitMixedScene(" + sampleIndex + "))"]);
  run(["set", "viewport", "480", "900"]);
  run(["screenshot", screenshot]);

  const compressed = readFileSync(output);
  const screenshotBytes = readFileSync(screenshot);
  metadata.observationBytes = compressed.length;
  metadata.observationSha256 = createHash("sha256").update(compressed).digest("hex");
  metadata.sampleScreenshotBytes = screenshotBytes.length;
  metadata.sampleScreenshotSha256 = createHash("sha256").update(screenshotBytes).digest("hex");
  metadata.sampleIndex = sampleIndex;
  metadata.sampleShowsGridEndOverflow = overflowSampleIndex >= 0;
  metadata.topologySceneCounts = Object.fromEntries(topologyCounts);
  metadata.distinctMasksObserved = Object.fromEntries([...masksByTopology].map(([key, value]) => [key, value.size]));
  metadata.sceneCountsByFitPosition = Object.fromEntries(sceneCountsByFitPosition);
  writeFileSync(manifestPath, JSON.stringify(metadata, null, 2) + "\n");
  process.stdout.write("OBSERVATION " + output + "\n");
  process.stdout.write("SCREENSHOT " + screenshot + "\n");
  process.stdout.write("FIXTURE_SHA256 " + fixtureSha256 + "\n");
} finally {
  if (openAttempted) {
    try {
      run(["close"]);
      closed = true;
    } catch (error) {
      process.stderr.write("CLOSE_ERROR " + error.message + "\n");
    }
  }
  writeFileSync(lifecycle, JSON.stringify({
    namespace,
    session,
    openAttempted,
    closed,
    observedAtUtc: new Date().toISOString(),
  }, null, 2) + "\n");
  if (!finished) {
    gzip.destroy();
    outputStream.destroy();
    if (existsSync(output)) unlinkSync(output);
  }
  process.stdout.write("BROWSER_CLOSED " + closed + "\n");
  if (openAttempted && !closed) process.exitCode = 1;
}
