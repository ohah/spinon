import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { gunzipSync } from "node:zlib";

const evidenceDir = dirname(fileURLToPath(import.meta.url));
const repositoryRoot = resolve(evidenceDir, "../../../..");
const fixtureRelativePath =
  "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-five-fit-auto-minimum-precomparison.html";
const fixtureBytes = readFileSync(resolve(repositoryRoot, fixtureRelativePath));
const compressedObservationBytes = readFileSync(resolve(evidenceDir, "chrome157-observation.ndjson.gz"));
const observationBytes = gunzipSync(compressedObservationBytes);
const tolerance = 0.01;
const expectedTexts = {
  A: "AAAA BBBB CCCC DDDD EEEE FFFF",
  B: "AAAAAA BBBBB CCCC DDD",
  C: "AAAAA BBBB CCC DDDD",
  D: "AAAA BBB CCCCC DDDD",
  E: "AAA BBBB CCCCCC",
};
const expectedLineCounts = { A: 6, B: 4, C: 4, D: 4, E: 3 };
const expectedTailOrders = [["min-content"], ["max-content"], ["auto"]];
const expectedCaps = ["0px", "10px", "20%", "30px", "40px"];
const expectedTopologies = [
  "span2Six-forward",
  "span2Six-reverse",
  "span2Span3Six-forward",
  "span2Span3Six-reverse",
];
const expectedTopologyItems = {
  span2Six: [["A", 2, 2], ["B", 3, 2], ["C", 4, 2], ["D", 5, 2], ["E", 6, 2]],
  span2Span3Six: [["A", 2, 2], ["B", 3, 2], ["C", 4, 3], ["D", 5, 3]],
};

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

function near(left, right, epsilon = tolerance) {
  return Number.isFinite(left) && Number.isFinite(right) && Math.abs(left - right) <= epsilon;
}

function vectorDelta(left, right) {
  return Math.max(...left.map((value, index) => Math.abs(value - right[index])));
}

function permutations(values) {
  const output = [];
  const visit = (remaining, prefix) => {
    if (remaining.length === 0) {
      output.push(prefix);
      return;
    }
    for (let index = 0; index < remaining.length; index += 1) {
      visit([...remaining.slice(0, index), ...remaining.slice(index + 1)], [...prefix, remaining[index]]);
    }
  };
  visit(values, []);
  return output;
}

function combinations(values, count) {
  const output = [];
  const visit = (start, prefix) => {
    if (prefix.length === count) {
      output.push(prefix);
      return;
    }
    for (let index = start; index <= values.length - (count - prefix.length); index += 1) {
      visit(index + 1, [...prefix, values[index]]);
    }
  };
  visit(0, []);
  return output;
}

function group(records, keyFn) {
  const output = new Map();
  for (const record of records) {
    const key = keyFn(record);
    const bucket = output.get(key) ?? [];
    bucket.push(record);
    output.set(key, bucket);
  }
  return output;
}

function validateRecord(record) {
  const { scene } = record;
  assert(scene.fitPositions.length === 5 && scene.fitCaps.length === 5,
    `scene ${record.index}: expected five fit positions and caps`);
  assert(scene.fitPositions.every((position, index) =>
    position >= 1 && position <= 6 && (index === 0 || scene.fitPositions[index - 1] < position)),
  `scene ${record.index}: fit positions must be unique, ordered and within six tracks`);
  assert(scene.trackMembers.filter((track) => track.startsWith("fit-content(")).length === 5,
    `scene ${record.index}: fit track multiplicity`);
  assert(scene.tail.length === 1 &&
    ["min-content", "max-content", "auto"].includes(scene.tail[0]) &&
    scene.trackMembers.filter((track) => track === scene.tail[0]).length === 1 &&
    scene.trackMembers.filter((track) => ["min-content", "max-content", "auto"].includes(track)).length === 1,
  `scene ${record.index}: five-fit tail value and multiplicity`);
  assert(scene.fitCaps.every((cap) => expectedCaps.includes(cap)) &&
    new Set(scene.fitCaps).size === 5 &&
    expectedCaps.every((cap) => scene.fitCaps.includes(cap)),
  `scene ${record.index}: five unique typed caps`);
  for (let index = 0; index < scene.fitPositions.length; index += 1) {
    assert(scene.trackMembers[scene.fitPositions[index] - 1] === `fit-content(${scene.fitCaps[index]})`,
      `scene ${record.index}: cap not bound to authored position`);
  }
  const fitPositionSet = new Set(scene.fitPositions);
  const observedTail = scene.trackMembers.filter((_, index) => !fitPositionSet.has(index + 1));
  assert(JSON.stringify(observedTail) === JSON.stringify(scene.tail),
    `scene ${record.index}: tail order does not match authored order`);
  assert(expectedTailOrders.some((tail) => JSON.stringify(tail) === JSON.stringify(scene.tail)),
    `scene ${record.index}: unplanned tail order`);
  assert(record.usedRows.trim().split(/\s+/).length === 7 && record.rowValues.length === 7,
    `scene ${record.index}: expected one explicit and six implicit used rows`);
  const serializedRows = record.usedRows.trim().split(/\s+/).map(Number.parseFloat);
  assert(record.rowValues.every((value, index) => Number.isFinite(value) && value >= 0 &&
    near(value, serializedRows[index], 0.001)),
  `scene ${record.index}: finite row values must match the computed row serialization`);
  assert(record.computedAutoRows === scene.trackList &&
    record.computedAutoRows.trim().split(/\s+/).length === 6,
  `scene ${record.index}: six authored auto-row members must remain ordered`);
  assert(record.computedAlignContent === scene.alignContent,
    `scene ${record.index}: computed alignment differs from the input`);
  assert(vectorDelta(record.gridFrame, [0, 0, 40, 220]) <= 0.001,
    `scene ${record.index}: definite Grid geometry changed`);
  const baseTopology = scene.topology.replace(/-(forward|reverse)$/, "");
  const expectedItems = expectedTopologyItems[baseTopology];
  assert(expectedItems && expectedTopologies.includes(scene.topology),
    `scene ${record.index}: unknown topology`);
  const orderedExpectedItems = scene.topology.endsWith("-reverse")
    ? [...expectedItems].reverse()
    : expectedItems;
  assert(JSON.stringify(scene.items) === JSON.stringify(orderedExpectedItems),
    `scene ${record.index}: explicit span placements changed`);
  assert(record.items.length === scene.items.length + 1 && record.items[0].id === "empty",
    `scene ${record.index}: explicit-row empty control/item inventory`);

  let eligibleOccurrences = 0;
  let contentFloorViolations = 0;
  let maxSpanFrameError = 0;
  for (const item of record.items) {
    if (item.id === "empty") {
      assert(vectorDelta(item.frame, [0, 0, 40, 10]) <= tolerance && item.minHeight === "0px",
        `scene ${record.index}: empty control must remain in the explicit row at min-height:0`);
      continue;
    }
    assert(item.minHeight === (scene.minimum === "auto" ? "auto" : "0px"),
      `scene ${record.index}: ${item.id} computed min-height`);
    assert(item.text === expectedTexts[item.id] && item.lineRects.length === expectedLineCounts[item.id],
      `scene ${record.index}: ${item.id} text payload or line layout changed`);
    assert(item.payloadFrame && item.payloadFrame.length === 4,
      `scene ${record.index}: ${item.id} payload frame is missing`);
    const placement = scene.items.find(([id]) => id === item.id);
    assert(placement, `scene ${record.index}: ${item.id} placement is missing`);
    const [, startLine, span] = placement;
    const firstImplicitRow = startLine - 2;
    const coveredRows = record.rowValues.slice(firstImplicitRow + 1, firstImplicitRow + span + 1);
    assert(coveredRows.length === span, `scene ${record.index}: ${item.id} row span length`);
    const expectedY = record.rowValues.slice(0, startLine - 1).reduce((sum, size) => sum + size, 0);
    const expectedHeight = coveredRows.reduce((sum, size) => sum + size, 0);
    const error = Math.max(Math.abs(item.frame[1] - expectedY), Math.abs(item.frame[3] - expectedHeight));
    maxSpanFrameError = Math.max(maxSpanFrameError, error);
    assert(error <= tolerance && near(item.frame[0], 0) && near(item.frame[2], 40),
      `scene ${record.index}: ${item.id} frame must equal its occupied track span`);
    const coveredTracks = scene.trackMembers.slice(firstImplicitRow, firstImplicitRow + span);
    const eligible = coveredTracks.some((track) => track === "auto" || track.startsWith("fit-content("));
    if (eligible) {
      eligibleOccurrences += 1;
      if (item.frame[3] + tolerance < item.payloadFrame[3]) {
        contentFloorViolations += 1;
      }
    }
  }
  return { eligibleOccurrences, contentFloorViolations, maxSpanFrameError };
}

const records = observationBytes.toString("utf8").trimEnd().split("\n").map((line) => JSON.parse(line));
assert(records.length === 34_560, `expected 34,560 observations, received ${records.length}`);
assert(records.every((record, index) => record.index === index), "scene indices must be continuous and unique");
assert(records[0].userAgent.includes("Chrome/157.0.0.0") && records.every((record) => record.userAgent === records[0].userAgent),
  "all observations must come from the pinned Chrome 157 agent-browser session");
assert(records.every((record) => record.devicePixelRatio === 1), "DPR must remain one for every observation");

const expectedPositionSets = combinations([1, 2, 3, 4, 5, 6], 5).map((positions) => positions.join(","));
const expectedCapOrders = permutations(expectedCaps);
const uniqueTrackLists = new Set();
const uniquePositions = new Set();
const uniqueCapOrders = new Set();
const uniqueTailOrders = new Set();
const validations = records.map((record) => {
  const scene = record.scene;
  uniqueTrackLists.add(scene.trackList);
  uniquePositions.add(scene.fitPositions.join(","));
  uniqueCapOrders.add(scene.fitCaps.join(","));
  uniqueTailOrders.add(scene.tail.join(","));
  return validateRecord(record);
});
assert(uniqueTrackLists.size === 2_160, `expected 2,160 authored track lists, got ${uniqueTrackLists.size}`);
assert(uniquePositions.size === 6 && expectedPositionSets.every((value) => uniquePositions.has(value)),
  "all six fit-position quintuples must be observed");
assert(uniqueCapOrders.size === 120 && expectedCapOrders.every((value) => uniqueCapOrders.has(value.join(","))),
  "all 120 authored cap assignments must be observed");
assert(uniqueTailOrders.size === 3 && expectedTailOrders.every((value) => uniqueTailOrders.has(value.join(","))),
  "all three tail values must be observed");
const trackListGroups = group(records, (record) => record.scene.trackList);
assert([...trackListGroups.values()].every((bucket) => bucket.length === 16),
  "each track list must have two alignments, four topologies and two minimum modes");

const sourceOrderGroups = group(records, (record) => {
  const scene = record.scene;
  return `${scene.trackList}|${scene.alignContent}|${scene.minimum}|${scene.topology.replace(/-(forward|reverse)$/, "")}`;
});
let maxSourceOrderDelta = 0;
for (const [key, pair] of sourceOrderGroups) {
  const forward = pair.find((record) => record.scene.topology.endsWith("-forward"));
  const reverse = pair.find((record) => record.scene.topology.endsWith("-reverse"));
  assert(forward && reverse, `source-order pair missing: ${key}`);
  for (let index = 0; index < forward.rowValues.length; index += 1) {
    maxSourceOrderDelta = Math.max(maxSourceOrderDelta, Math.abs(forward.rowValues[index] - reverse.rowValues[index]));
  }
  const reverseItems = new Map(reverse.items.map((item) => [item.id, item]));
  for (const item of forward.items) {
    const counterpart = reverseItems.get(item.id);
    assert(counterpart, `source-order semantic item missing: ${key}/${item.id}`);
    maxSourceOrderDelta = Math.max(maxSourceOrderDelta, vectorDelta(item.frame, counterpart.frame));
  }
}
assert(sourceOrderGroups.size === 17_280 && maxSourceOrderDelta <= tolerance,
  "source-order reversal must preserve semantic row and item geometry");

const alignmentGroups = group(records, (record) =>
  `${record.scene.trackList}|${record.scene.topology}|${record.scene.minimum}`);
let maxNonAutoStretchDelta = 0;
let maxAutoStretchDelta = 0;
for (const [key, pair] of alignmentGroups) {
  const start = pair.find((record) => record.scene.alignContent === "start");
  const stretch = pair.find((record) => record.scene.alignContent === "stretch");
  assert(start && stretch, `alignment pair missing: ${key}`);
  const autoRow = start.scene.trackMembers.indexOf("auto") + 1;
  const startTotal = start.rowValues.reduce((sum, size) => sum + size, 0);
  const stretchTotal = stretch.rowValues.reduce((sum, size) => sum + size, 0);
  if (autoRow > 0) {
    assert(near(stretchTotal, Math.max(startTotal, 220), 0.02), `auto-tail stretch total differs from the definite Grid size: ${key}`);
  } else {
    assert(near(stretchTotal, startTotal, 0.02), `min/max-content tail must leave free space unused: ${key}`);
  }
  for (let index = 0; index < start.rowValues.length; index += 1) {
    const change = stretch.rowValues[index] - start.rowValues[index];
    if (autoRow > 0 && index === autoRow) {
      assert(change >= -tolerance, `stretch shrank the auto track: ${key}`);
      maxAutoStretchDelta = Math.max(maxAutoStretchDelta, change);
    } else {
      maxNonAutoStretchDelta = Math.max(maxNonAutoStretchDelta, Math.abs(change));
      assert(Math.abs(change) <= tolerance, `stretch changed a non-auto row: ${key}/${index}`);
    }
  }
}
assert(alignmentGroups.size === 17_280 && maxNonAutoStretchDelta <= tolerance,
  "stretch may add free space only to the auto row");

const minimumGroups = group(records, (record) =>
  `${record.scene.trackList}|${record.scene.alignContent}|${record.scene.topology}`);
let candidateEligibleOccurrences = 0;
let candidateFloorViolations = 0;
let baselineFloorViolations = 0;
let changedTextFrames = 0;
let maxMinimumTrackDelta = 0;
let maxMinimumItemDelta = 0;
for (const [key, pair] of minimumGroups) {
  const zero = pair.find((record) => record.scene.minimum === "zero");
  const automatic = pair.find((record) => record.scene.minimum === "auto");
  assert(zero && automatic, `minimum-mode pair missing: ${key}`);
  for (let index = 0; index < zero.rowValues.length; index += 1) {
    maxMinimumTrackDelta = Math.max(maxMinimumTrackDelta, Math.abs(zero.rowValues[index] - automatic.rowValues[index]));
  }
  const zeroItems = new Map(zero.items.map((item) => [item.id, item]));
  for (const item of automatic.items) {
    const baseline = zeroItems.get(item.id);
    assert(baseline, `minimum-mode item missing: ${key}/${item.id}`);
    const difference = vectorDelta(item.frame, baseline.frame);
    maxMinimumItemDelta = Math.max(maxMinimumItemDelta, difference);
    if (item.id !== "empty" && difference > tolerance) changedTextFrames += 1;
  }
  const zeroValidation = validateRecord(zero);
  const autoValidation = validateRecord(automatic);
  baselineFloorViolations += zeroValidation.contentFloorViolations;
  candidateEligibleOccurrences += autoValidation.eligibleOccurrences;
  candidateFloorViolations += autoValidation.contentFloorViolations;
}
assert(minimumGroups.size === 17_280 && candidateFloorViolations === 0,
  "every candidate eligible row-span item must meet its payload floor");

const capGroups = group(records, (record) => {
  const scene = record.scene;
  const direction = scene.topology.endsWith("-reverse") ? "reverse" : "forward";
  const baseTopology = scene.topology.replace(/-(forward|reverse)$/, "");
  return JSON.stringify([scene.fitPositions, scene.tail, scene.alignContent, scene.minimum, baseTopology, direction]);
});
assert(capGroups.size === 288, `expected 288 groups for cap permutations, got ${capGroups.size}`);
for (const [key, variants] of capGroups) {
  const observed = new Set(variants.map((record) => record.scene.fitCaps.join(",")));
  assert(variants.length === 120 && expectedCapOrders.every((order) => observed.has(order.join(","))),
    `cap assignments incomplete in group: ${key}`);
}

const result = {
  schema: "spinon.c11-4.repeated-text-row-span-five-fit-auto-minimum-independent-verification.v2",
  verifiedAtUtc: new Date().toISOString(),
  browserUserAgent: records[0].userAgent,
  fixture: {
    path: fixtureRelativePath,
    sha256: createHash("sha256").update(fixtureBytes).digest("hex"),
  },
  observations: {
    path: "chrome157-observation.ndjson.gz",
    compression: "gzip",
    compressedBytes: compressedObservationBytes.length,
    compressedSha256: createHash("sha256").update(compressedObservationBytes).digest("hex"),
    uncompressedBytes: observationBytes.length,
    uncompressedSha256: createHash("sha256").update(observationBytes).digest("hex"),
    sceneCount: records.length,
    pairedKeys: records.length / 2,
    uniqueTrackLists: uniqueTrackLists.size,
    fitPositionQuintuples: uniquePositions.size,
    capAssignments: uniqueCapOrders.size,
    tailOrders: uniqueTailOrders.size,
  },
  automaticMinimum: {
    eligibleCandidateItemOccurrences: candidateEligibleOccurrences,
    baselinePayloadFloorViolations: baselineFloorViolations,
    candidatePayloadFloorViolations: candidateFloorViolations,
  },
  relationships: {
    sourceOrderPairs: sourceOrderGroups.size,
    maximumSourceOrderDeltaCssPx: maxSourceOrderDelta,
    alignmentPairs: alignmentGroups.size,
    maximumNonAutoStretchDeltaCssPx: maxNonAutoStretchDelta,
    maximumAutoStretchDeltaCssPx: maxAutoStretchDelta,
    minimumModePairs: minimumGroups.size,
    changedTextItemFrames: changedTextFrames,
    maximumMinimumTrackDeltaCssPx: maxMinimumTrackDelta,
    maximumMinimumItemDeltaCssPx: maxMinimumItemDelta,
    capPermutationGroups: capGroups.size,
    maximumSpanFrameErrorCssPx: Math.max(...validations.map((item) => item.maxSpanFrameError)),
  },
};
writeFileSync(resolve(evidenceDir, "verification.json"), `${JSON.stringify(result, null, 2)}\n`);
process.stdout.write(`${JSON.stringify(result)}\n`);
