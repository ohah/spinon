import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { createWriteStream, existsSync, readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { once } from "node:events";
import { fileURLToPath, pathToFileURL } from "node:url";
import { dirname, join, resolve } from "node:path";
import { createGzip } from "node:zlib";

const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, "../../../..");
const fixtureRelative = "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-five-fit-mixed-auto-minimum-precomparison.html";
const fixture = join(root, fixtureRelative);
const output = join(here, "chrome157-five-fit-mixed-observation.ndjson.gz");
const screenshot = join(here, "chrome157-five-fit-mixed-sample.png");
const lifecycle = join(here, "agent-browser-lifecycle.json");
const browserCli = process.env.AGENT_BROWSER_PATH ?? "/opt/homebrew/bin/agent-browser";
const namespace = "spinon5mix12";
const session = "5mixpre";
const expectedSceneCount = 414_720;
const expectedTrackLists = 2_160;
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

let opened = false;
let closed = false;
let finished = false;
let capturedCount = 0;
let sampleIndex = -1;
const trackLists = new Set();
const topologyCounts = new Map();
const masksByTopology = new Map();

try {
  run(["open", pathToFileURL(fixture).href]);
  opened = true;
  run(["set", "viewport", "1280", "900"]);
  const sceneCount = Number(run(["eval", "window.__c114FiveFitMixedSceneCount"]));
  if (sceneCount !== expectedSceneCount) throw new Error(`expected ${expectedSceneCount} scenes, got ${sceneCount}`);
  const trackListCount = Number(run(["eval", "window.__c114FiveFitMixedTrackListCount"]));
  if (trackListCount !== expectedTrackLists) throw new Error(`expected ${expectedTrackLists} track lists, got ${trackListCount}`);
  const sentinels = parse(run(["eval", `JSON.stringify([0,31,32,63,64,79,80,95,96,191,414719].map((index) => window.c114FiveFitMixedSceneAt(index)))` ]));
  const expectedSentinels = [
    ["start", "span2Six-forward", 0, []],
    ["start", "span2Six-forward", 31, ["A", "B", "C", "D", "E"]],
    ["start", "span2Six-reverse", 0, []],
    ["start", "span2Six-reverse", 31, ["A", "B", "C", "D", "E"]],
    ["start", "span2Span3Six-forward", 0, []],
    ["start", "span2Span3Six-forward", 15, ["A", "B", "C", "D"]],
    ["start", "span2Span3Six-reverse", 0, []],
    ["start", "span2Span3Six-reverse", 15, ["A", "B", "C", "D"]],
    ["stretch", "span2Six-forward", 0, []],
    ["stretch", "span2Span3Six-reverse", 15, ["A", "B", "C", "D"]],
    ["stretch", "span2Span3Six-reverse", 15, ["A", "B", "C", "D"]],
  ];
  for (let index = 0; index < sentinels.length; index += 1) {
    const scene = sentinels[index];
    const expected = expectedSentinels[index];
    if (scene.alignContent !== expected[0] || scene.topology !== expected[1]
      || scene.minimumMask !== expected[2] || JSON.stringify(scene.autoItems) !== JSON.stringify(expected[3])) {
      throw new Error(`scene-index sentinel ${index} mismatch: ${JSON.stringify(scene)}`);
    }
  }
  const browser = parse(run(["eval", "JSON.stringify({ userAgent: navigator.userAgent, platform: navigator.platform, devicePixelRatio: window.devicePixelRatio })"]));
  const fixtureBytes = readFileSync(fixture);
  const fixtureSha256 = createHash("sha256").update(fixtureBytes).digest("hex");
  const metadata = {
    record: "metadata",
    schema: "spinon.c11-4.repeated-text-row-span-five-fit-mixed-auto-min-precomparison/v1",
    capturedAtUtc: new Date().toISOString(),
    browserCli: execFileSync(browserCli, ["--version"], { cwd: root, encoding: "utf8" }).trim(),
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
    fitPositionSets: 6,
    orderedCapAssignments: 120,
    singletonTailValues: ["min-content", "max-content", "auto"],
    capValues: ["0px", "10px", "20%", "30px", "40px"],
    alignments: ["start", "stretch"],
    topologies: ["span2Six-forward", "span2Six-reverse", "span2Span3Six-forward", "span2Span3Six-reverse"],
    minimumAssignments: { span2Six: 32, span2Span3Six: 16 },
    minimumMaskBitOrder: { span2Six: ["A", "B", "C", "D", "E"], span2Span3Six: ["A", "B", "C", "D"] },
    observationPath: "chrome157-five-fit-mixed-observation.ndjson.gz",
  };
  await writeGzip(`${JSON.stringify(metadata)}\n`);

  for (let start = 0; start < sceneCount; start += batchSize) {
    const size = Math.min(batchSize, sceneCount - start);
    const batch = parse(run(["eval", `JSON.stringify(window.runC114FiveFitMixedBatch(${start}, ${size}))`]));
    if (batch.start !== start || batch.end !== start + size || batch.total !== sceneCount) {
      throw new Error(`batch ${start} bounds mismatch`);
    }
    for (const record of batch.results) {
      if (record.index !== capturedCount) throw new Error(`non-contiguous scene order at ${capturedCount}`);
      trackLists.add(record.scene.trackList);
      const topology = record.scene.topology;
      topologyCounts.set(topology, (topologyCounts.get(topology) ?? 0) + 1);
      const masks = masksByTopology.get(topology) ?? new Set();
      masks.add(record.scene.minimumMask);
      masksByTopology.set(topology, masks);
      if (sampleIndex < 0 && record.scene.minimumMask === 5
        && record.scene.fitPositions.join(",") === "1,2,3,4,5"
        && record.scene.alignContent === "start"
        && record.scene.topology === "span2Six-forward"
        && record.scene.autoItems.join(",") === "A,C") {
        sampleIndex = record.index;
      }
      capturedCount += 1;
    }
    await writeGzip(`${batch.results.map((record) => JSON.stringify({ record: "scene", ...record })).join("\n")}\n`);
    if (capturedCount % 9_600 === 0 || capturedCount === sceneCount) {
      process.stdout.write(`CAPTURED ${capturedCount}/${sceneCount}\n`);
    }
  }

  if (capturedCount !== sceneCount) throw new Error(`captured ${capturedCount}`);
  if (trackLists.size !== expectedTrackLists) throw new Error(`expected ${expectedTrackLists} track lists, got ${trackLists.size}`);
  if (sampleIndex < 0) throw new Error("representative mixed-mask sample missing");
  for (const topology of metadata.topologies) {
    const family = topology.startsWith("span2Six") ? "span2Six" : "span2Span3Six";
    const expectedMasks = metadata.minimumAssignments[family];
    if (masksByTopology.get(topology)?.size !== expectedMasks) {
      throw new Error(`${topology}: expected ${expectedMasks} masks, got ${masksByTopology.get(topology)?.size}`);
    }
  }

  gzip.end();
  await once(outputStream, "finish");
  finished = true;
  run(["eval", `JSON.stringify(window.showC114FiveFitMixedScene(${sampleIndex}))`]);
  run(["eval", "(() => { const grid=document.querySelector('#grid'); grid.style.transform='scale(3.5)'; grid.style.transformOrigin='top left'; return true; })()"]);
  run(["set", "viewport", "480", "900"]);
  run(["screenshot", screenshot]);

  const compressed = readFileSync(output);
  metadata.observationBytes = compressed.length;
  metadata.observationSha256 = createHash("sha256").update(compressed).digest("hex");
  metadata.topologySceneCounts = Object.fromEntries(topologyCounts);
  metadata.distinctMasksObserved = Object.fromEntries([...masksByTopology].map(([key, value]) => [key, value.size]));
  writeFileSync(join(here, "capture-manifest.json"), `${JSON.stringify(metadata, null, 2)}\n`);
  process.stdout.write(`OBSERVATION ${output}\nSCREENSHOT ${screenshot}\nFIXTURE_SHA256 ${fixtureSha256}\n`);
} finally {
  if (opened) {
    try { run(["close"]); closed = true; } catch (error) { process.stderr.write(`CLOSE_ERROR ${error.message}\n`); }
  }
  writeFileSync(lifecycle, `${JSON.stringify({ namespace, session, opened, closed, observedAtUtc: new Date().toISOString() }, null, 2)}\n`);
  if (!finished) {
    gzip.destroy();
    outputStream.destroy();
    if (existsSync(output)) unlinkSync(output);
  }
  process.stdout.write(`BROWSER_CLOSED ${closed}\n`);
}
