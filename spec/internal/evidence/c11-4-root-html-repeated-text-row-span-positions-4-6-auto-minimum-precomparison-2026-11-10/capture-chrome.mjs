import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import {
  appendFileSync,
  mkdirSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const evidenceDir = dirname(fileURLToPath(import.meta.url));
const repositoryRoot = resolve(evidenceDir, "../../../..");
const fixturePath = resolve(
  repositoryRoot,
  "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-positions-4-6-auto-minimum-precomparison.html",
);
const observationPath = resolve(evidenceDir, "chrome157-observation.ndjson");
const summaryPath = resolve(evidenceDir, "capture-summary.json");
const screenshotPath = resolve(evidenceDir, "chrome157-row-span-positions-4-6-auto-sample.png");
const lifecyclePath = resolve(evidenceDir, "agent-browser-lifecycle.json");
const browserCli = process.env.AGENT_BROWSER_PATH ?? "/opt/homebrew/bin/agent-browser";
const session = "spinon-c114-row-span-pos456-auto-min-20261110";
const expectedSceneCount = 8_640;
const batchSize = 96;
const tolerance = 0.01;

mkdirSync(evidenceDir, { recursive: true });
const fixtureBytes = readFileSync(fixturePath);
const fixtureSha256 = createHash("sha256").update(fixtureBytes).digest("hex");
const captureScriptSha256 = createHash("sha256")
  .update(readFileSync(fileURLToPath(import.meta.url)))
  .digest("hex");
writeFileSync(observationPath, "");
const captured = [];
let opened = false;
let closed = false;
let beforeClose = null;
let afterClose = null;
let closeError = null;

function runBrowser(args, timeout = 180_000) {
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

function maxFrameError(actual, expected) {
  return Math.max(...actual.map((value, index) => Math.abs(value - expected[index])));
}

function validateScene(record) {
  const { scene } = record;
  assert(
    scene.fitPosition >= 4 && scene.fitPosition <= 6,
    `scene ${record.index}: fit-content position outside the six-row fit-position profile`,
  );
  assert(
    scene.trackMembers.length === 6 &&
      scene.trackMembers.filter((member) => member.startsWith("fit-content(")).length === 1 &&
      scene.trackMembers.filter((member) => member === "min-content").length === 2 &&
      scene.trackMembers.filter((member) => member === "max-content").length === 2 &&
      scene.trackMembers.filter((member) => member === "auto").length === 1,
    `scene ${record.index}: track-list multiplicity changed`,
  );
  assert(
    scene.trackMembers[scene.fitPosition - 1] === `fit-content(${scene.cap})`,
    `scene ${record.index}: cap moved from its authored position`,
  );
  assert(
    scene.minHeight === "0px" || scene.minHeight === "auto",
    `scene ${record.index}: unexpected min-height mode`,
  );
  assert(
    record.userAgent.includes("HeadlessChrome/157."),
    `scene ${record.index}: expected Agent Browser HeadlessChrome 157, got ${record.userAgent}`,
  );
  assert(
    maxFrameError(record.gridFrame, [0, 0, 40, 220]) <= tolerance,
    `scene ${record.index}: definite Grid frame changed`,
  );
  assert(
    record.rowValues.length === 7 &&
      record.rowValues.every((value) => Number.isFinite(value) && value >= 0),
    `scene ${record.index}: expected one explicit and six used implicit rows`,
  );
  assert(
    record.computedAutoRows === scene.trackList,
    `scene ${record.index}: computed grid-auto-rows changed authored order or value type`,
  );
  assert(
    record.computedAlignContent === scene.alignContent,
    `scene ${record.index}: align-content changed`,
  );
  assert(
    record.items.length === scene.items.length + 1,
    `scene ${record.index}: unexpected direct item count`,
  );

  for (const item of record.items) {
    if (item.id === "empty") {
      assert(item.minHeight === "0px", `scene ${record.index}: explicit control minimum changed`);
      assert(maxFrameError(item.frame, [0, 0, 40, 10]) <= tolerance, `scene ${record.index}: explicit row`);
      continue;
    }
    const placement = scene.items.find(([id]) => id === item.id);
    assert(placement, `scene ${record.index}: unknown text item ${item.id}`);
    const [, rowStart, rowSpan] = placement;
    assert(item.minHeight === scene.minHeight, `scene ${record.index}: item minimum mode changed`);
    assert(item.frame[0] === 0 && item.frame[2] === 40, `scene ${record.index}: item column changed`);
    assert(item.payloadFrame, `scene ${record.index}: missing payload frame`);
    assert(item.lineRects.length > 0, `scene ${record.index}: missing text line rectangles`);
    const firstTrack = rowStart - 1;
    const expectedY = record.rowValues.slice(0, firstTrack).reduce((sum, size) => sum + size, 0);
    const expectedHeight = record.rowValues
      .slice(firstTrack, firstTrack + rowSpan)
      .reduce((sum, size) => sum + size, 0);
    assert(
      Math.abs(item.frame[1] - expectedY) <= tolerance &&
        Math.abs(item.frame[3] - expectedHeight) <= tolerance,
      `scene ${record.index}: ${item.id} frame does not equal its used row-span sum`,
    );
  }
}

function usesAutomaticMinimumTrack(scene, item) {
  const [, rowStartText, rowSpanText] = item;
  const rowStart = Number(rowStartText);
  const rowSpan = Number(rowSpanText);
  for (let track = rowStart - 1; track < rowStart - 1 + rowSpan; track += 1) {
    if (track === 0) continue;
    const implicitIndex = track - 1;
    const member = scene.trackMembers[implicitIndex % scene.trackMembers.length];
    if (member === "auto" || member.startsWith("fit-content(")) return true;
  }
  return false;
}

function verifyPairs(records) {
  const pairs = new Map();
  const sourceOrderGroups = new Map();
  const alignmentGroups = new Map();
  for (const record of records) {
    const prior = pairs.get(record.scene.key) ?? {};
    prior[record.scene.minHeight] = record;
    pairs.set(record.scene.key, prior);

    const topology = record.scene.topology;
    const topologyPair =
      topology === "span2-six-forward" || topology === "span2-six-reversed"
        ? "span2"
        : "span2-span3";
    const order =
      topology === "span2-six-forward" || topology === "span2-span3-six-forward"
        ? "forward"
        : "reverse";
    const orderKey = [
      record.scene.fitPosition,
      record.scene.tail.join(" "),
      record.scene.cap,
      record.scene.alignContent,
      record.scene.minHeight,
      topologyPair,
    ].join("|");
    const orderPair = sourceOrderGroups.get(orderKey) ?? {};
    orderPair[order] = record;
    sourceOrderGroups.set(orderKey, orderPair);

    const alignmentKey = [
      record.scene.fitPosition,
      record.scene.tail.join(" "),
      record.scene.cap,
      record.scene.topology,
      record.scene.minHeight,
    ].join("|");
    const alignmentPair = alignmentGroups.get(alignmentKey) ?? {};
    alignmentPair[record.scene.alignContent] = record;
    alignmentGroups.set(alignmentKey, alignmentPair);
  }
  assert(pairs.size === expectedSceneCount / 2, `expected paired keys, got ${pairs.size}`);

  let eligibleItemCount = 0;
  let baselineBelowContentFloor = 0;
  let candidateBelowContentFloor = 0;
  let candidateVisibleContentOverflow = 0;
  let automaticMinimumChangedFrameCount = 0;
  let maxTrackDeltaPx = 0;
  let maxItemFrameDeltaPx = 0;
  let maxTextLineDelta = 0;
  let maxSourceOrderFrameDeltaPx = 0;
  let maxNonAutoStretchDeltaPx = 0;
  let maxAutoTrackStretchDeltaPx = 0;
  let autoTrackGrowthCount = 0;

  for (const [key, pair] of pairs) {
    assert(pair["0px"] && pair.auto, `missing 0/auto pair for ${key}`);
    const baseline = pair["0px"];
    const candidate = pair.auto;
    assert(
      baseline.scene.trackList === candidate.scene.trackList &&
        baseline.scene.topology === candidate.scene.topology &&
        baseline.scene.alignContent === candidate.scene.alignContent &&
        baseline.scene.fitPosition === candidate.scene.fitPosition,
      `paired scene inputs differ for ${key}`,
    );
    const baselineById = new Map(baseline.items.map((item) => [item.id, item]));
    const candidateById = new Map(candidate.items.map((item) => [item.id, item]));
    for (let index = 0; index < baseline.rowValues.length; index += 1) {
      maxTrackDeltaPx = Math.max(
        maxTrackDeltaPx,
        Math.abs(baseline.rowValues[index] - candidate.rowValues[index]),
      );
    }
    for (const [id, rowStart, rowSpan] of baseline.scene.items) {
      const before = baselineById.get(id);
      const after = candidateById.get(id);
      assert(before && after, `paired semantic item missing for ${key}: ${id}`);
      assert(before.lineRects.length === after.lineRects.length, `text wrapping changed for ${key}: ${id}`);
      maxTextLineDelta = Math.max(maxTextLineDelta, Math.abs(before.lineRects.length - after.lineRects.length));
      maxItemFrameDeltaPx = Math.max(maxItemFrameDeltaPx, maxFrameError(before.frame, after.frame));
      if (maxFrameError(before.frame, after.frame) > tolerance) automaticMinimumChangedFrameCount += 1;

      if (usesAutomaticMinimumTrack(baseline.scene, [id, rowStart, rowSpan])) {
        eligibleItemCount += 1;
        if (before.frame[3] + tolerance < before.payloadFrame[3]) baselineBelowContentFloor += 1;
        if (after.frame[3] + tolerance < after.payloadFrame[3]) candidateBelowContentFloor += 1;
        if (after.contentOverflowBottom > tolerance) candidateVisibleContentOverflow += 1;
      }
    }
  }

  assert(sourceOrderGroups.size === expectedSceneCount / 2, "source-order pair count changed");
  for (const [key, pair] of sourceOrderGroups) {
    assert(pair.forward && pair.reverse, `missing source-order pair for ${key}`);
    const forward = pair.forward;
    const reverse = pair.reverse;
    for (let index = 0; index < forward.rowValues.length; index += 1) {
      const delta = Math.abs(forward.rowValues[index] - reverse.rowValues[index]);
      maxSourceOrderFrameDeltaPx = Math.max(maxSourceOrderFrameDeltaPx, delta);
      assert(delta <= tolerance, `source order changed used row ${index} for ${key}`);
    }
    const reverseById = new Map(reverse.items.map((item) => [item.id, item]));
    for (const item of forward.items) {
      const counterpart = reverseById.get(item.id);
      assert(counterpart, `source-order item missing for ${key}: ${item.id}`);
      const delta = maxFrameError(item.frame, counterpart.frame);
      maxSourceOrderFrameDeltaPx = Math.max(maxSourceOrderFrameDeltaPx, delta);
      assert(delta <= tolerance, `source order changed ${item.id} frame for ${key}`);
    }
  }

  assert(alignmentGroups.size === expectedSceneCount / 2, "alignment pair count changed");
  for (const [key, pair] of alignmentGroups) {
    assert(pair.start && pair.stretch, `missing start/stretch pair for ${key}`);
    for (let index = 0; index < pair.start.rowValues.length; index += 1) {
      const delta = pair.stretch.rowValues[index] - pair.start.rowValues[index];
      const rowIndex = index - 1;
      const member = rowIndex < 0 ? null : pair.start.scene.trackMembers[rowIndex % 6];
      if (member === "auto") {
        assert(delta >= -tolerance, `stretch shrank auto track ${index} for ${key}`);
        if (delta > tolerance) autoTrackGrowthCount += 1;
        maxAutoTrackStretchDeltaPx = Math.max(maxAutoTrackStretchDeltaPx, delta);
      } else {
        maxNonAutoStretchDeltaPx = Math.max(maxNonAutoStretchDeltaPx, Math.abs(delta));
        assert(Math.abs(delta) <= tolerance, `stretch changed non-auto track ${index} for ${key}`);
      }
    }
  }

  assert(candidateBelowContentFloor === 0, `candidate fell below content floor ${candidateBelowContentFloor} times`);
  assert(candidateVisibleContentOverflow === 0, `candidate has visible content overflow ${candidateVisibleContentOverflow} times`);
  assert(maxTextLineDelta === 0, "paired min-height modes changed text wrapping");

  return {
    schema: "spinon.c11-4.repeated-text-row-span-positions-4-6-auto-minimum-precomparison.v1",
    sceneCount: records.length,
    pairedSceneCount: pairs.size,
    chrome: records[0]?.userAgent,
    fixtureSha256,
    captureScriptSha256,
    baselineSceneCount: records.filter((record) => record.scene.minHeight === "0px").length,
    candidateSceneCount: records.filter((record) => record.scene.minHeight === "auto").length,
    eligibleItemCount,
    baselineBelowContentFloor,
    candidateBelowContentFloor,
    candidateVisibleContentOverflow,
    automaticMinimumChangedFrameCount,
    sourceOrderPairCount: sourceOrderGroups.size,
    maxSourceOrderFrameDeltaPx,
    alignmentPairCount: alignmentGroups.size,
    maxNonAutoStretchDeltaPx,
    maxAutoTrackStretchDeltaPx,
    autoTrackGrowthCount,
    maxTrackDeltaPx,
    maxItemFrameDeltaPx,
    maxTextLineDelta,
    toleranceCssPx: tolerance,
    sceneDefinition: {
      fitPositions: [4, 5, 6],
      uniqueTailOrders: 30,
      caps: ["0px", "0%", "10px", "20px", "20%", "120%"],
      alignments: ["start", "stretch"],
      topologies: [
        "span2-six-forward",
        "span2-six-reversed",
        "span2-span3-six-forward",
        "span2-span3-six-reversed",
      ],
      minimumModes: ["0px", "auto"],
    },
  };
}

try {
  runBrowser(["open", pathToFileURL(fixturePath).href]);
  opened = true;
  runBrowser(["set", "viewport", "1280", "900"]);
  const sceneCount = Number(runBrowser(["eval", "window.__c114SixRowSpanAutoScenes.length"]));
  assert(sceneCount === expectedSceneCount, `expected ${expectedSceneCount} scenes, got ${sceneCount}`);

  for (let start = 0; start < sceneCount; start += batchSize) {
    const count = Math.min(batchSize, sceneCount - start);
    const output = runBrowser([
      "eval",
      `JSON.stringify(window.runC114SixRowSpanAutoBatch(${start}, ${count}))`,
    ]);
    const batch = parseEvalOutput(output);
    assert(batch.start === start && batch.end === start + count, `batch boundaries changed at ${start}`);
    assert(batch.total === expectedSceneCount, `browser scene count changed at ${start}`);
    assert(batch.results.length === count, `batch result count changed at ${start}`);
    for (const record of batch.results) validateScene(record);
    captured.push(...batch.results);
    appendFileSync(observationPath, `${batch.results.map((record) => JSON.stringify(record)).join("\n")}\n`);
    writeFileSync(
      resolve(evidenceDir, "capture-progress.json"),
      `${JSON.stringify({ captured: captured.length, expected: expectedSceneCount }, null, 2)}\n`,
    );
  }

  assert(captured.length === expectedSceneCount, `captured ${captured.length} scenes`);
  const summary = verifyPairs(captured);
  const sampleIndex = captured.findIndex(
    (record) =>
      record.scene.minHeight === "auto" &&
      record.scene.fitPosition === 4 &&
      record.scene.cap === "0%" &&
      record.scene.alignContent === "start" &&
      record.scene.topology === "span2-six-forward",
  );
  assert(sampleIndex >= 0, "sample scene not found");
  runBrowser(["eval", `JSON.stringify(window.showC114SixRowSpanAutoPair(${sampleIndex}))`]);
  runBrowser(["screenshot", screenshotPath]);
  writeFileSync(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);
} finally {
  if (opened) {
    try {
      beforeClose = parseEvalOutput(runBrowser(["session", "info", "--json"]));
    } catch {
      beforeClose = null;
    }
    try {
      runBrowser(["close"]);
      closed = true;
    } catch (error) {
      closeError = error.message;
    }
    try {
      afterClose = parseEvalOutput(runBrowser(["session", "info", "--json"]));
    } catch {
      afterClose = null;
    }
  }
  writeFileSync(
    lifecyclePath,
    `${JSON.stringify({ session, opened, closed, closeError, beforeClose, afterClose }, null, 2)}\n`,
  );
}

if (!closed) throw new Error(`Agent Browser session was not closed: ${closeError ?? "unknown error"}`);
rmSync(resolve(evidenceDir, "capture-progress.json"), { force: true });
process.stdout.write(`${JSON.stringify(verifyPairs(captured), null, 2)}\n`);
