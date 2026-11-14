import { createHash } from "node:crypto";
import { createReadStream, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createGunzip } from "node:zlib";
import { createInterface } from "node:readline";

const evidenceDir = dirname(fileURLToPath(import.meta.url));
const repositoryRoot = resolve(evidenceDir, "../../../..");
const fixturePath = resolve(
  repositoryRoot,
  "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-two-fit-mixed-auto-minimum-precomparison.html",
);
const observationPath = resolve(evidenceDir, "chrome-observation.ndjson.gz");
const capturePath = resolve(evidenceDir, "capture.json");
const collectorPath = resolve(evidenceDir, "capture-chrome.mjs");
const lifecyclePath = resolve(evidenceDir, "agent-browser-lifecycle.json");
const mixedScreenshotPath = resolve(evidenceDir, "chrome157-two-fit-mixed-sample.png");
const overflowScreenshotPath = resolve(evidenceDir, "chrome157-two-fit-mixed-overflow-sample.png");
const verificationPath = resolve(evidenceDir, "verification.json");
const expectedSceneCount = 92_160;
const epsilon = 0.001;
const texts = {
  A: "AAAA BBBB CCCC DDDD EEEE FFFF",
  B: "AAAAAA BBBBB CCCC DDD",
  C: "AAAAA BBBB CCC DDDD",
  D: "AAAA BBB CCCCC DDDD",
  E: "AAA BBBB CCCCCC",
};
const topologyItems = {
  span2Six: [
    ["A", 2, 2],
    ["B", 3, 2],
    ["C", 4, 2],
    ["D", 5, 2],
    ["E", 6, 2],
  ],
  span2Span3Six: [
    ["A", 2, 2],
    ["B", 3, 2],
    ["C", 4, 3],
    ["D", 5, 3],
  ],
};
const tails = [
  ["min-content", "min-content", "max-content", "auto"],
  ["min-content", "max-content", "min-content", "auto"],
  ["max-content", "auto", "min-content", "min-content"],
  ["auto", "min-content", "max-content", "min-content"],
];
const capPairs = [
  ["0px", "0%"],
  ["10px", "20px"],
  ["10px", "20%"],
  ["20%", "120%"],
];

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

function near(left, right, tolerance = epsilon) {
  return Number.isFinite(left) && Number.isFinite(right) && Math.abs(left - right) <= tolerance;
}

function vectorError(left, right) {
  if (!left || !right || left.length !== right.length) return Number.POSITIVE_INFINITY;
  return Math.max(0, ...left.map((value, index) => Math.abs(value - right[index])));
}

function sum(values) {
  return values.reduce((total, value) => total + value, 0);
}

function baseTopology(topology) {
  return topology.replace(/-(forward|reverse)$/, "");
}

function sourceOrder(topology) {
  return topology.endsWith("-reverse") ? "reverse" : "forward";
}

function expectedTracks(scene) {
  const [firstPosition, secondPosition] = scene.fitPositions;
  const [firstCap, secondCap] = scene.fitCaps;
  const expected = Array(6);
  expected[firstPosition - 1] = `fit-content(${firstCap})`;
  expected[secondPosition - 1] = `fit-content(${secondCap})`;
  let tailIndex = 0;
  for (let index = 0; index < expected.length; index += 1) {
    if (expected[index] === undefined) {
      expected[index] = scene.tail[tailIndex];
      tailIndex += 1;
    }
  }
  return expected;
}

function assertScene(record) {
  const { scene } = record;
  assert(scene.fitPositions.length === 2, `scene ${record.index}: fit positions`);
  assert(
    scene.fitPositions[0] >= 1 && scene.fitPositions[0] < scene.fitPositions[1] && scene.fitPositions[1] <= 6,
    `scene ${record.index}: fit position range/order`,
  );
  assert(
    capPairs.some((pair) => JSON.stringify(pair) === JSON.stringify(scene.fitCaps)) ||
      capPairs.some((pair) => JSON.stringify([...pair].reverse()) === JSON.stringify(scene.fitCaps)),
    `scene ${record.index}: unknown ordered cap pair`,
  );
  assert(tails.some((tail) => JSON.stringify(tail) === JSON.stringify(scene.tail)), `scene ${record.index}: tail`);
  const tracks = expectedTracks(scene);
  assert(JSON.stringify(tracks) === JSON.stringify(scene.trackMembers), `scene ${record.index}: authored tracks`);
  assert(scene.trackList === tracks.join(" "), `scene ${record.index}: track serialization`);
  assert(["start", "stretch"].includes(scene.alignContent), `scene ${record.index}: alignment`);

  const base = baseTopology(scene.topology);
  const items = topologyItems[base];
  assert(items, `scene ${record.index}: unknown topology`);
  const expectedItems = sourceOrder(scene.topology) === "reverse" ? [...items].reverse() : items;
  assert(JSON.stringify(expectedItems) === JSON.stringify(scene.items), `scene ${record.index}: span topology`);
  const maskItemIds = items.map(([id]) => id).sort();
  assert(JSON.stringify(maskItemIds) === JSON.stringify(scene.maskItemIds), `scene ${record.index}: semantic mask IDs`);
  assert(scene.minimumMask >= 0 && scene.minimumMask < 2 ** maskItemIds.length, `scene ${record.index}: mask range`);
  const expectedAutoIds = maskItemIds.filter((_, bit) => (scene.minimumMask & (1 << bit)) !== 0);
  assert(JSON.stringify(expectedAutoIds) === JSON.stringify(scene.autoItemIds), `scene ${record.index}: mask decoding`);

  assert(record.rowValues.length === 7, `scene ${record.index}: row count`);
  assert(record.rowValues.every((value) => Number.isFinite(value) && value >= 0), `scene ${record.index}: row values`);
  const cssRows = record.usedRows.trim().split(/\s+/).map((token) => Number.parseFloat(token));
  assert(cssRows.length === record.rowValues.length, `scene ${record.index}: CSS row count`);
  assert(cssRows.every((value, index) => near(value, record.rowValues[index])), `scene ${record.index}: CSS row values`);
  assert(vectorError(record.gridFrame, [0, 0, 40, 220]) <= epsilon, `scene ${record.index}: grid frame`);
  assert(record.computedAutoRows === scene.trackList, `scene ${record.index}: computed track list`);
  assert(
    record.computedOverflowX === "visible" && record.computedOverflowY === "visible",
    `scene ${record.index}: overflow visibility changed`,
  );
  assert(record.items.length === items.length + 1, `scene ${record.index}: item count`);

  const itemById = new Map(record.items.map((item) => [item.id, item]));
  const empty = itemById.get("empty");
  assert(empty && empty.minHeight === "0px", `scene ${record.index}: explicit-row control`);
  assert(near(empty.frame[1], 0) && near(empty.frame[3], 10), `scene ${record.index}: explicit-row geometry`);
  for (const id of maskItemIds) {
    const observed = itemById.get(id);
    assert(observed, `scene ${record.index}: missing ${id}`);
    assert(observed.text === texts[id], `scene ${record.index}: text changed for ${id}`);
    assert(observed.minHeight === (expectedAutoIds.includes(id) ? "auto" : "0px"), `scene ${record.index}: minimum for ${id}`);
    assert(observed.frame.length === 4 && observed.frame.every(Number.isFinite), `scene ${record.index}: frame for ${id}`);
    assert(observed.payloadFrame?.length === 4, `scene ${record.index}: payload frame for ${id}`);
    assert(observed.lineRects.length > 0, `scene ${record.index}: line rectangles for ${id}`);
    assert(observed.lineRects.flat().every(Number.isFinite), `scene ${record.index}: invalid line rect for ${id}`);

    const [, start, span] = items.find(([itemId]) => itemId === id);
    const firstTrack = start - 1;
    const expectedY = sum(record.rowValues.slice(0, firstTrack));
    const expectedHeight = sum(record.rowValues.slice(firstTrack, firstTrack + span));
    assert(near(observed.frame[1], expectedY, 0.01), `scene ${record.index}: ${id} span start`);
    assert(near(observed.frame[3], expectedHeight, 0.01), `scene ${record.index}: ${id} span height`);
  }
}

function spanRowSumError(record) {
  let maximum = 0;
  for (const [id, start, span] of record.scene.items) {
    const observed = record.items.find((item) => item.id === id);
    const expectedHeight = sum(record.rowValues.slice(start - 1, start - 1 + span));
    maximum = Math.max(maximum, Math.abs(observed.frame[3] - expectedHeight));
  }
  return maximum;
}

function frameMap(record) {
  return new Map(record.items.map((item) => [item.id, item]));
}

function compareSourceOrders(forward, reverse) {
  let maximum = vectorError(forward.rowValues, reverse.rowValues);
  const left = frameMap(forward);
  const right = frameMap(reverse);
  for (const id of left.keys()) {
    const a = left.get(id);
    const b = right.get(id);
    maximum = Math.max(maximum, vectorError(a.frame, b.frame));
    if (a.payloadFrame && b.payloadFrame) maximum = Math.max(maximum, vectorError(a.payloadFrame, b.payloadFrame));
    if (a.lineRects.length === b.lineRects.length) {
      for (let index = 0; index < a.lineRects.length; index += 1) {
        maximum = Math.max(maximum, vectorError(a.lineRects[index], b.lineRects[index]));
      }
    } else {
      maximum = Number.POSITIVE_INFINITY;
    }
  }
  return maximum;
}

const capture = JSON.parse(readFileSync(capturePath, "utf8"));
const lifecycle = JSON.parse(readFileSync(lifecyclePath, "utf8"));
const fixtureSha256 = createHash("sha256").update(readFileSync(fixturePath)).digest("hex");
const collectorSha256 = createHash("sha256").update(readFileSync(collectorPath)).digest("hex");
const observationSha256 = createHash("sha256").update(readFileSync(observationPath)).digest("hex");
const mixedScreenshotSha256 = createHash("sha256").update(readFileSync(mixedScreenshotPath)).digest("hex");
const overflowScreenshotSha256 = createHash("sha256").update(readFileSync(overflowScreenshotPath)).digest("hex");
assert(capture.fixture.sha256 === fixtureSha256, "fixture hash changed after capture");
assert(capture.collector.sha256 === collectorSha256, "collector hash changed after capture");
assert(capture.observation.sha256 === observationSha256, "observation hash changed after capture");
assert(capture.screenshots.mixed.sha256 === mixedScreenshotSha256, "mixed screenshot hash changed after capture");
assert(capture.screenshots.visibleOverflow.sha256 === overflowScreenshotSha256, "overflow screenshot hash changed after capture");
assert(lifecycle.closed && !lifecycle.ownSessionActiveAfterClose, "Agent Browser session cleanup was not verified");
assert(lifecycle.completed && lifecycle.captured === expectedSceneCount, "Agent Browser capture lifecycle incomplete");
assert(capture.sceneCount === expectedSceneCount, "capture metadata scene count");

const sceneKeys = new Set();
const trackLists = new Set();
const topologyCounts = new Map();
const masksByGroup = new Map();
const sourcePairs = new Map();
const alignmentPairs = new Map();
const endpointPairs = new Map();
let sceneCount = 0;
let automaticItemOccurrences = 0;
let visibleOverflowScenes = 0;
let gridItemOverflowScenes = 0;
let payloadOverflowScenes = 0;
let lineRectOverflowScenes = 0;
let maximumGridItemOverflowPx = 0;
let maximumPayloadOverflowPx = 0;
let maximumLineRectOverflowPx = 0;
const overflowByMask = new Map();
const overflowByFitCaps = new Map();
const overflowByTopology = new Map();
let minimumRowPx = Number.POSITIVE_INFINITY;
let maximumRowPx = 0;
let maximumSpanErrorPx = 0;

const lines = createInterface({
  input: createReadStream(observationPath).pipe(createGunzip()),
  crlfDelay: Infinity,
});
for await (const line of lines) {
  if (!line) continue;
  const record = JSON.parse(line);
  assert(record.index === sceneCount, `non-contiguous scene index ${record.index} at ${sceneCount}`);
  assertScene(record);
  maximumSpanErrorPx = Math.max(maximumSpanErrorPx, spanRowSumError(record));
  sceneCount += 1;
  const { scene } = record;
  trackLists.add(scene.trackList);
  automaticItemOccurrences += scene.autoItemIds.length;
  let sceneGridItemOverflow = 0;
  let scenePayloadOverflow = 0;
  let sceneLineOverflow = 0;
  for (const item of record.items) {
    sceneGridItemOverflow = Math.max(sceneGridItemOverflow, item.frame[1] + item.frame[3] - 220);
    if (item.payloadFrame) {
      scenePayloadOverflow = Math.max(scenePayloadOverflow, item.payloadFrame[1] + item.payloadFrame[3] - 220);
    }
    for (const rect of item.lineRects) {
      sceneLineOverflow = Math.max(sceneLineOverflow, rect[1] + rect[3] - 220);
    }
  }
  if (sceneGridItemOverflow > epsilon) {
    gridItemOverflowScenes += 1;
    maximumGridItemOverflowPx = Math.max(maximumGridItemOverflowPx, sceneGridItemOverflow);
  }
  if (scenePayloadOverflow > epsilon) {
    payloadOverflowScenes += 1;
    maximumPayloadOverflowPx = Math.max(maximumPayloadOverflowPx, scenePayloadOverflow);
  }
  if (sceneLineOverflow > epsilon) {
    lineRectOverflowScenes += 1;
    maximumLineRectOverflowPx = Math.max(maximumLineRectOverflowPx, sceneLineOverflow);
  }
  if (Math.max(sceneGridItemOverflow, scenePayloadOverflow, sceneLineOverflow) > epsilon) {
    visibleOverflowScenes += 1;
    const mask = String(scene.minimumMask);
    overflowByMask.set(mask, (overflowByMask.get(mask) ?? 0) + 1);
    const caps = scene.fitCaps.join("|");
    overflowByFitCaps.set(caps, (overflowByFitCaps.get(caps) ?? 0) + 1);
    overflowByTopology.set(scene.topology, (overflowByTopology.get(scene.topology) ?? 0) + 1);
    assert(scene.alignContent === "stretch", `scene ${record.index}: overflow alignment`);
    assert(baseTopology(scene.topology) === "span2Six", `scene ${record.index}: overflow topology`);
    assert(
      (scene.fitCaps[0] === "0px" && scene.fitCaps[1] === "0%") ||
        (scene.fitCaps[0] === "0%" && scene.fitCaps[1] === "0px"),
      `scene ${record.index}: overflow cap pair`,
    );
    assert(sceneGridItemOverflow <= epsilon, `scene ${record.index}: Grid item frame exceeds border edge`);
    assert(sum(record.rowValues) <= 220 + epsilon, `scene ${record.index}: used tracks exceed Grid block-size`);
  }
  minimumRowPx = Math.min(minimumRowPx, ...record.rowValues);
  maximumRowPx = Math.max(maximumRowPx, ...record.rowValues);

  const topology = baseTopology(scene.topology);
  const order = sourceOrder(scene.topology);
  const topologyKey = scene.topology;
  topologyCounts.set(topologyKey, (topologyCounts.get(topologyKey) ?? 0) + 1);
  const identity = [scene.trackList, scene.alignContent, scene.topology, scene.minimumMask].join("\u001f");
  assert(!sceneKeys.has(identity), `duplicate scene identity ${identity}`);
  sceneKeys.add(identity);

  const maskGroup = [scene.trackList, scene.alignContent, topology].join("\u001f");
  const maskSet = masksByGroup.get(maskGroup) ?? new Set();
  maskSet.add(scene.minimumMask);
  masksByGroup.set(maskGroup, maskSet);

  const sourceKey = [scene.trackList, scene.alignContent, topology, scene.minimumMask].join("\u001f");
  const sourcePair = sourcePairs.get(sourceKey) ?? {};
  assert(!sourcePair[order], `duplicate ${order} source order for ${sourceKey}`);
  sourcePair[order] = record;
  sourcePairs.set(sourceKey, sourcePair);

  const alignmentKey = [scene.trackList, scene.topology, scene.minimumMask].join("\u001f");
  const alignmentPair = alignmentPairs.get(alignmentKey) ?? {};
  assert(!alignmentPair[scene.alignContent], `duplicate ${scene.alignContent} alignment for ${alignmentKey}`);
  alignmentPair[scene.alignContent] = record;
  alignmentPairs.set(alignmentKey, alignmentPair);

  const maximumMask = 2 ** scene.maskItemIds.length - 1;
  if (scene.minimumMask === 0 || scene.minimumMask === maximumMask) {
    const endpointKey = [scene.trackList, scene.topology, scene.alignContent].join("\u001f");
    const endpointPair = endpointPairs.get(endpointKey) ?? {};
    endpointPair[scene.minimumMask === 0 ? "zero" : "auto"] = record;
    endpointPairs.set(endpointKey, endpointPair);
  }

}

assert(sceneCount === expectedSceneCount, `expected ${expectedSceneCount} scenes; got ${sceneCount}`);
assert(trackLists.size === 480, `expected 480 track lists; got ${trackLists.size}`);
assert(sceneKeys.size === expectedSceneCount, "scene identity coverage incomplete");
assert(sourcePairs.size === 46_080, `source pair count ${sourcePairs.size}`);
assert(alignmentPairs.size === 46_080, `alignment pair count ${alignmentPairs.size}`);
assert(endpointPairs.size === 3_840, `endpoint pair count ${endpointPairs.size}`);
assert(automaticItemOccurrences === 215_040, `automatic-minimum item count ${automaticItemOccurrences}`);
assert(maximumSpanErrorPx <= 0.01, `span/row sum error ${maximumSpanErrorPx}px`);
assert(visibleOverflowScenes === 320, `visible overflow scene count ${visibleOverflowScenes}`);
assert(payloadOverflowScenes === 320, `payload overflow scene count ${payloadOverflowScenes}`);
assert(lineRectOverflowScenes === 320, `text line overflow scene count ${lineRectOverflowScenes}`);
assert(gridItemOverflowScenes === 0, `Grid item overflow scene count ${gridItemOverflowScenes}`);
assert(maximumPayloadOverflowPx <= 30.001, `payload overflow exceeded 30px: ${maximumPayloadOverflowPx}`);
assert(maximumLineRectOverflowPx <= 30.001, `line overflow exceeded 30px: ${maximumLineRectOverflowPx}`);
assert(overflowByFitCaps.get("0px|0%") === 160, "first zero cap assignment overflow count");
assert(overflowByFitCaps.get("0%|0px") === 160, "reversed zero cap assignment overflow count");
assert(overflowByTopology.get("span2Six-forward") === 160, "forward span2Six overflow count");
assert(overflowByTopology.get("span2Six-reverse") === 160, "reverse span2Six overflow count");

let maximumSourceOrderErrorPx = 0;
for (const [key, pair] of sourcePairs) {
  assert(pair.forward && pair.reverse, `missing source-order mate for ${key}`);
  const error = compareSourceOrders(pair.forward, pair.reverse);
  maximumSourceOrderErrorPx = Math.max(maximumSourceOrderErrorPx, error);
}
assert(maximumSourceOrderErrorPx <= 0.01, `source-order geometry diverged by ${maximumSourceOrderErrorPx}px`);

let maximumNonAutoStretchErrorPx = 0;
let minimumAutoStretchDeltaPx = Number.POSITIVE_INFINITY;
let maximumAutoStretchDeltaPx = 0;
for (const [key, pair] of alignmentPairs) {
  assert(pair.start && pair.stretch, `missing alignment mate for ${key}`);
  const autoIndex = pair.start.scene.trackMembers.indexOf("auto");
  assert(autoIndex >= 0 && pair.start.scene.trackMembers.lastIndexOf("auto") === autoIndex, `auto track count for ${key}`);
  const usedRowIndex = autoIndex + 1;
  for (let row = 0; row < pair.start.rowValues.length; row += 1) {
    const delta = pair.stretch.rowValues[row] - pair.start.rowValues[row];
    if (row === usedRowIndex) {
      assert(delta >= -epsilon, `stretch shrank auto row for ${key}`);
      minimumAutoStretchDeltaPx = Math.min(minimumAutoStretchDeltaPx, delta);
      maximumAutoStretchDeltaPx = Math.max(maximumAutoStretchDeltaPx, delta);
    } else {
      maximumNonAutoStretchErrorPx = Math.max(maximumNonAutoStretchErrorPx, Math.abs(delta));
    }
  }
}
assert(maximumNonAutoStretchErrorPx <= 0.01, `non-auto track stretch error ${maximumNonAutoStretchErrorPx}px`);
let maximumZeroToAllAutoFrameDeltaPx = 0;
for (const [key, pair] of endpointPairs) {
  assert(pair.zero && pair.auto, `missing zero/all-auto endpoint for ${key}`);
  const zeroFrames = frameMap(pair.zero);
  const autoFrames = frameMap(pair.auto);
  for (const [id, zero] of zeroFrames) {
    const auto = autoFrames.get(id);
    assert(auto, `missing endpoint item ${id} for ${key}`);
    maximumZeroToAllAutoFrameDeltaPx = Math.max(
      maximumZeroToAllAutoFrameDeltaPx,
      vectorError(zero.frame, auto.frame),
    );
  }
}

const expectedMasksPerProfile = {
  span2Six: 32,
  span2Span3Six: 16,
};
for (const [key, masks] of masksByGroup) {
  const profile = key.includes("span2Span3Six") ? "span2Span3Six" : "span2Six";
  assert(masks.size === expectedMasksPerProfile[profile], `mask coverage incomplete for ${key}`);
}
for (const [topology, count] of topologyCounts) {
  const base = baseTopology(topology);
  const expected = base === "span2Six" ? 30_720 : 15_360;
  assert(count === expected, `${topology} scene count ${count}; expected ${expected}`);
}

const verification = {
  schema: "spinon.c11-4.root-html-row-span-two-fit-mixed.verification/v1",
  capture: "capture.json",
  fixtureSha256,
  collectorSha256,
  observationSha256,
  screenshots: {
    mixedSha256: mixedScreenshotSha256,
    visibleOverflowSha256: overflowScreenshotSha256,
  },
  browser: {
    userAgent: capture.browserUserAgent,
    devicePixelRatio: capture.devicePixelRatio,
    viewport: capture.viewport,
  },
  matrix: {
    sceneCount,
    formula: "480 track lists × 2 alignments × 2 source orders × (32 span2Six masks + 16 span2Span3Six masks) = 92,160",
    uniqueTrackListCount: trackLists.size,
    uniqueSceneIdentityCount: sceneKeys.size,
    topologyCounts: Object.fromEntries([...topologyCounts].sort()),
    endpointCounts: { allZero: 3_840, allAuto: 3_840 },
    automaticMinimumItemOccurrences: automaticItemOccurrences,
    sourceOrderPairs: sourcePairs.size,
    alignmentPairs: alignmentPairs.size,
    maskCountsPerProfile: expectedMasksPerProfile,
  },
  relations: {
    maximumSpanRowSumErrorCssPx: maximumSpanErrorPx,
    maximumSourceOrderGeometryErrorCssPx: maximumSourceOrderErrorPx,
    maximumNonAutoTrackStretchErrorCssPx: maximumNonAutoStretchErrorPx,
    autoTrackStretchDeltaRangeCssPx: [minimumAutoStretchDeltaPx, maximumAutoStretchDeltaPx],
    minimumUsedRowCssPx: minimumRowPx,
    maximumUsedRowCssPx: maximumRowPx,
    visibleOverflowSceneCount: visibleOverflowScenes,
    visibleOverflowSceneCountsByMinimumMask: Object.fromEntries([...overflowByMask].sort()),
    visibleOverflowSceneCountsByOrderedCapPair: Object.fromEntries([...overflowByFitCaps].sort()),
    visibleOverflowSceneCountsByTopology: Object.fromEntries([...overflowByTopology].sort()),
    gridItemOverflowSceneCount: gridItemOverflowScenes,
    payloadOverflowSceneCount: payloadOverflowScenes,
    textLineRectOverflowSceneCount: lineRectOverflowScenes,
    maximumGridItemOverflowCssPx: maximumGridItemOverflowPx,
    maximumPayloadOverflowCssPx: maximumPayloadOverflowPx,
    maximumTextLineOverflowCssPx: maximumLineRectOverflowPx,
    endpointPairCount: endpointPairs.size,
    zeroToAllAutoEndpointFrameDeltaMaxCssPx: maximumZeroToAllAutoFrameDeltaPx,
  },
  scope: "Chrome 157 CSS Grid oracle only; this file does not verify Spinon RuntimeSession or mobile rendering.",
};
writeFileSync(verificationPath, `${JSON.stringify(verification, null, 2)}\n`);
process.stdout.write(`VERIFIED ${sceneCount}/${expectedSceneCount}; ${verification.matrix.uniqueTrackListCount} track lists\n`);
