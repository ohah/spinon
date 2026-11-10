import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const evidenceDir = dirname(fileURLToPath(import.meta.url));
const fixtureRelativePath =
  "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-three-fit-auto-minimum-precomparison.html";
const repositoryRoot = resolve(evidenceDir, "../../../..");
const fixturePath = resolve(repositoryRoot, fixtureRelativePath);
const observationPath = resolve(evidenceDir, "chrome157-observation.ndjson");
const verificationPath = resolve(evidenceDir, "verification.json");
const tolerance = 0.01;
const expectedModes = ["zero", "auto"];
const expectedTopologies = [
  "span2Six-forward",
  "span2Six-reverse",
  "span2Span3Six-forward",
  "span2Span3Six-reverse",
];
const expectedCaps = [
  ["0px", "10px", "20%"],
  ["0px", "20%", "10px"],
  ["10px", "0px", "20%"],
  ["10px", "20%", "0px"],
  ["20%", "0px", "10px"],
  ["20%", "10px", "0px"],
];

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

function near(left, right, epsilon = tolerance) {
  return Number.isFinite(left) && Number.isFinite(right) && Math.abs(left - right) <= epsilon;
}

function delta(left, right) {
  return Math.max(...left.map((value, index) => Math.abs(value - right[index])));
}

function group(records, keyFn) {
  const result = new Map();
  for (const record of records) {
    const key = keyFn(record);
    const bucket = result.get(key) ?? [];
    bucket.push(record);
    result.set(key, bucket);
  }
  return result;
}

const observationsBytes = readFileSync(observationPath);
const records = observationsBytes
  .toString("utf8")
  .trimEnd()
  .split("\n")
  .map((line) => JSON.parse(line));
const fixtureBytes = readFileSync(fixturePath);
const observationsSha256 = createHash("sha256").update(observationsBytes).digest("hex");
const fixtureSha256 = createHash("sha256").update(fixtureBytes).digest("hex");
assert(records.length === 11_520, `expected 11,520 observations, got ${records.length}`);
assert(records.every((record, index) => record.index === index), "observation indices are incomplete or duplicated");

const uniqueTrackLists = new Set();
const fitTriples = new Set();
const tracklistBuckets = group(records, (record) => {
  const { scene } = record;
  uniqueTrackLists.add(scene.trackList);
  fitTriples.add(scene.fitPositions.join(","));
  assert(scene.fitPositions.length === 3 && scene.fitCaps.length === 3, `scene ${record.index}: fit count`);
  assert(scene.fitPositions[0] < scene.fitPositions[1] && scene.fitPositions[1] < scene.fitPositions[2],
    `scene ${record.index}: fit positions not strictly increasing`);
  assert(scene.trackMembers.filter((value) => value.startsWith("fit-content(")).length === 3,
    `scene ${record.index}: fit member multiplicity`);
  assert(scene.trackMembers.filter((value) => value === "min-content").length === 1 &&
    scene.trackMembers.filter((value) => value === "max-content").length === 1 &&
    scene.trackMembers.filter((value) => value === "auto").length === 1,
  `scene ${record.index}: tail multiset`);
  assert(record.computedAutoRows === scene.trackList, `scene ${record.index}: computed authored order`);
  assert(record.rowValues.length === 7 && record.rowValues.every((value) => Number.isFinite(value) && value >= 0),
    `scene ${record.index}: used row vector`);
  assert(record.gridFrame.every((value, index) => near(value, [0, 0, 40, 220][index], 0.001)),
    `scene ${record.index}: grid frame`);
  assert(expectedModes.includes(scene.minimum), `scene ${record.index}: minimum mode`);
  assert(expectedTopologies.includes(scene.topology), `scene ${record.index}: topology`);
  return scene.trackList;
});
assert(uniqueTrackLists.size === 720, `expected 720 track lists, got ${uniqueTrackLists.size}`);
assert(fitTriples.size === 20, `expected 20 fit-position triples, got ${fitTriples.size}`);
assert(tracklistBuckets.size === 720 && [...tracklistBuckets.values()].every((bucket) => bucket.length === 16),
  "each track list must have 16 alignment/topology/minimum observations");

let maximumSpanFrameErrorCssPx = 0;
let eligibleCandidateItemOccurrences = 0;
let candidateContentFloorViolations = 0;
for (const record of records) {
  const { scene } = record;
  for (const item of record.items) {
    if (item.id === "empty") {
      assert(near(item.frame[1], 0) && near(item.frame[3], 10) && item.minHeight === "0px",
        `scene ${record.index}: explicit-row control`);
      continue;
    }
    assert(item.minHeight === (scene.minimum === "auto" ? "auto" : "0px"),
      `scene ${record.index}: item minimum mode`);
    const placement = scene.items.find(([id]) => id === item.id);
    assert(placement, `scene ${record.index}: missing placement ${item.id}`);
    const [, startLine, span] = placement;
    const firstImplicitRow = startLine - 2;
    const spannedRowSizes = record.rowValues.slice(firstImplicitRow + 1, firstImplicitRow + span + 1);
    assert(spannedRowSizes.length === span, `scene ${record.index}: span row count ${item.id}`);
    const expectedY = record.rowValues.slice(0, startLine - 1).reduce((sum, size) => sum + size, 0);
    const expectedHeight = spannedRowSizes.reduce((sum, size) => sum + size, 0);
    const error = Math.max(Math.abs(item.frame[1] - expectedY), Math.abs(item.frame[3] - expectedHeight));
    maximumSpanFrameErrorCssPx = Math.max(maximumSpanFrameErrorCssPx, error);
    assert(error <= tolerance, `scene ${record.index}: span frame row-sum mismatch ${item.id}`);

    const coveredTracks = scene.trackMembers.slice(firstImplicitRow, firstImplicitRow + span);
    const eligible = coveredTracks.some((track) => track === "auto" || track.startsWith("fit-content("));
    if (eligible && scene.minimum === "auto") {
      eligibleCandidateItemOccurrences += 1;
      if (item.frame[3] + tolerance < item.payloadFrame[3]) candidateContentFloorViolations += 1;
    }
  }
}
assert(candidateContentFloorViolations === 0, "candidate item frame fell below payload content floor");

let maximumSourceOrderDeltaCssPx = 0;
let maximumNonAutoStretchDeltaCssPx = 0;
let maximumAutoTrackStretchDeltaCssPx = 0;
const sourceOrderPairs = group(records, (record) => {
  const { scene } = record;
  const topology = scene.topology.replace(/-(forward|reverse)$/, "");
  return `${scene.trackList}|${scene.alignContent}|${scene.minimum}|${topology}`;
});
for (const [key, pair] of sourceOrderPairs) {
  const forward = pair.find((record) => record.scene.topology.endsWith("-forward"));
  const reverse = pair.find((record) => record.scene.topology.endsWith("-reverse"));
  assert(forward && reverse, `source-order pair missing ${key}`);
  for (let index = 0; index < forward.rowValues.length; index += 1) {
    maximumSourceOrderDeltaCssPx = Math.max(maximumSourceOrderDeltaCssPx,
      Math.abs(forward.rowValues[index] - reverse.rowValues[index]));
  }
  const reverseItems = new Map(reverse.items.map((item) => [item.id, item]));
  for (const item of forward.items) {
    const other = reverseItems.get(item.id);
    assert(other, `source-order item missing ${key}/${item.id}`);
    maximumSourceOrderDeltaCssPx = Math.max(maximumSourceOrderDeltaCssPx, delta(item.frame, other.frame));
  }
}
assert(sourceOrderPairs.size === 5_760, `source-order pairs ${sourceOrderPairs.size}`);
assert(maximumSourceOrderDeltaCssPx <= tolerance, "source-order reversal changed semantic geometry");

const alignmentPairs = group(records, (record) =>
  `${record.scene.trackList}|${record.scene.topology}|${record.scene.minimum}`);
for (const [key, pair] of alignmentPairs) {
  const start = pair.find((record) => record.scene.alignContent === "start");
  const stretch = pair.find((record) => record.scene.alignContent === "stretch");
  assert(start && stretch, `alignment pair missing ${key}`);
  const autoTrackRow = start.scene.trackMembers.indexOf("auto") + 1;
  const startTotal = start.rowValues.reduce((sum, value) => sum + value, 0);
  const stretchTotal = stretch.rowValues.reduce((sum, value) => sum + value, 0);
  assert(near(stretchTotal, Math.max(startTotal, 220), 0.02), `stretch total mismatch ${key}`);
  for (let row = 0; row < 7; row += 1) {
    const change = stretch.rowValues[row] - start.rowValues[row];
    if (row === autoTrackRow) {
      assert(change >= -tolerance, `stretch shrank auto track ${key}`);
      maximumAutoTrackStretchDeltaCssPx = Math.max(maximumAutoTrackStretchDeltaCssPx, change);
    } else {
      maximumNonAutoStretchDeltaCssPx = Math.max(maximumNonAutoStretchDeltaCssPx, Math.abs(change));
      assert(Math.abs(change) <= tolerance, `stretch changed non-auto track ${key}/${row}`);
    }
  }
}
assert(alignmentPairs.size === 5_760, `alignment pairs ${alignmentPairs.size}`);

let changedTextItemFramesAfterAutoMinimum = 0;
let maximumAutoMinimumItemDeltaCssPx = 0;
let maximumAutoMinimumTrackDeltaCssPx = 0;
const minimumPairs = group(records, (record) =>
  `${record.scene.trackList}|${record.scene.alignContent}|${record.scene.topology}`);
for (const [key, pair] of minimumPairs) {
  const zero = pair.find((record) => record.scene.minimum === "zero");
  const auto = pair.find((record) => record.scene.minimum === "auto");
  assert(zero && auto, `minimum pair missing ${key}`);
  for (let row = 0; row < 7; row += 1) {
    maximumAutoMinimumTrackDeltaCssPx = Math.max(maximumAutoMinimumTrackDeltaCssPx,
      Math.abs(auto.rowValues[row] - zero.rowValues[row]));
  }
  const zeroItems = new Map(zero.items.map((item) => [item.id, item]));
  for (const item of auto.items) {
    const base = zeroItems.get(item.id);
    assert(base, `minimum pair item missing ${key}/${item.id}`);
    const difference = delta(item.frame, base.frame);
    maximumAutoMinimumItemDeltaCssPx = Math.max(maximumAutoMinimumItemDeltaCssPx, difference);
    if (item.id !== "empty" && difference > tolerance) changedTextItemFramesAfterAutoMinimum += 1;
  }
}
assert(minimumPairs.size === 5_760, `automatic-minimum pairs ${minimumPairs.size}`);

const capGroups = group(records, (record) => {
  const { scene } = record;
  const baseTopology = scene.topology.replace(/-(forward|reverse)$/, "");
  const direction = scene.topology.endsWith("-reverse") ? "reverse" : "forward";
  return JSON.stringify([
    scene.fitPositions,
    scene.tail,
    scene.alignContent,
    scene.minimum,
    baseTopology,
    direction,
  ]);
});
let capSensitiveGroups = 0;
let maximumTrackRangeAcrossCapAssignmentsCssPx = 0;
for (const [key, variants] of capGroups) {
  assert(variants.length === 6, `cap variants missing for ${key}`);
  const actual = new Set(variants.map((record) => record.scene.fitCaps.join("|")));
  assert(expectedCaps.every((assignment) => actual.has(assignment.join("|"))), `cap permutation missing ${key}`);
  let groupRange = 0;
  for (let row = 0; row < 7; row += 1) {
    const sizes = variants.map((record) => record.rowValues[row]);
    groupRange = Math.max(groupRange, Math.max(...sizes) - Math.min(...sizes));
  }
  maximumTrackRangeAcrossCapAssignmentsCssPx = Math.max(maximumTrackRangeAcrossCapAssignmentsCssPx, groupRange);
  if (groupRange > tolerance) capSensitiveGroups += 1;
}
assert(capGroups.size === 1_920, `cap groups ${capGroups.size}`);

const result = {
  schema: "spinon.c11-4.repeated-text-row-span-three-fit-auto-minimum-independent-verification.v1",
  verifiedAtUtc: new Date().toISOString(),
  browserUserAgent: records[0].userAgent,
  fixture: { path: fixtureRelativePath, sha256: fixtureSha256 },
  observations: {
    path: "chrome157-observation.ndjson",
    bytes: observationsBytes.length,
    sha256: observationsSha256,
    scenes: records.length,
    uniqueTrackLists: uniqueTrackLists.size,
    uniqueFitPositionTriples: fitTriples.size,
    pairedMinimumKeys: minimumPairs.size,
    expectedCapPermutationsPerGroup: 6,
    verifiedCapAssignmentGroups: capGroups.size,
  },
  checks: {
    authoredTrackOrderPreserved: true,
    sevenFiniteUsedRows: true,
    itemFrameEqualsSpanTrackSum: true,
    sourceOrderInvariant: true,
    stretchOnlyChangesAutoTrack: true,
    candidateAutomaticMinimumRespectsPayloadFloor: true,
    maximumSpanFrameErrorCssPx: maximumSpanFrameErrorCssPx,
    candidateEligibleItemOccurrences: eligibleCandidateItemOccurrences,
    candidateContentFloorViolations,
    sourceOrderPairs: sourceOrderPairs.size,
    maximumSourceOrderDeltaCssPx,
    alignmentPairs: alignmentPairs.size,
    maximumNonAutoStretchDeltaCssPx,
    maximumAutoTrackStretchDeltaCssPx,
    automaticMinimumPairs: minimumPairs.size,
    changedTextItemFramesAfterAutoMinimum,
    maximumAutoMinimumItemDeltaCssPx,
    maximumAutoMinimumTrackDeltaCssPx,
    capSensitiveGroups,
    maximumTrackRangeAcrossCapAssignmentsCssPx,
  },
};
writeFileSync(verificationPath, `${JSON.stringify(result, null, 2)}\n`);
process.stdout.write(`${JSON.stringify(result)}\n`);
