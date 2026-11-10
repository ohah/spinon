import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const evidenceDir = dirname(fileURLToPath(import.meta.url));
const repositoryRoot = resolve(evidenceDir, "../../../..");
const fixtureRelativePath =
  "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-three-fit-auto-minimum-precomparison.html";
const fixturePath = resolve(repositoryRoot, fixtureRelativePath);
const observationPath = resolve(evidenceDir, "chrome157-observation.ndjson");
const verificationPath = resolve(evidenceDir, "verification.json");
const screenshotPath = resolve(evidenceDir, "chrome157-three-fit-auto-minimum-sample.png");
const lifecyclePath = resolve(evidenceDir, "agent-browser-lifecycle.json");
const browserCli = process.env.AGENT_BROWSER_PATH ?? "/opt/homebrew/bin/agent-browser";
const session = "spinon-c114-three-fit-row-span-auto-minimum-20261110";
const expectedSceneCount = 11_520;
const batchSize = 240;
const expectedText = {
  A: "AAAA BBBB CCCC DDDD EEEE FFFF",
  B: "AAAAAA BBBBB CCCC DDD",
  C: "AAAAA BBBB CCC DDDD",
  D: "AAAA BBB CCCCC DDDD",
  E: "AAA BBBB CCCCCC",
};
const expectedLineCount = { A: 6, B: 4, C: 4, D: 4, E: 3 };
const expectedTopologyItems = {
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

mkdirSync(evidenceDir, { recursive: true });
const fixtureSha256 = createHash("sha256").update(readFileSync(fixturePath)).digest("hex");
const captureScriptSha256 = createHash("sha256")
  .update(readFileSync(fileURLToPath(import.meta.url)))
  .digest("hex");
let opened = false;
let closed = false;
let closeError = null;
const captured = [];

function runBrowser(args, options = {}) {
  return execFileSync(browserCli, ["--session", session, ...args], {
    cwd: repositoryRoot,
    encoding: "utf8",
    maxBuffer: 32 * 1024 * 1024,
    timeout: options.timeout ?? 120_000,
  }).trim();
}

function parseEvalOutput(output) {
  const decoded = JSON.parse(output);
  return typeof decoded === "string" ? JSON.parse(decoded) : decoded;
}

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

function near(actual, expected, tolerance = 0.01) {
  return Number.isFinite(actual) && Math.abs(actual - expected) <= tolerance;
}

function frameError(actual, expected) {
  return Math.max(...actual.map((value, index) => Math.abs(value - expected[index])));
}

function expectedTailOrders() {
  return [
    ["min-content", "max-content", "auto"],
    ["min-content", "auto", "max-content"],
    ["max-content", "min-content", "auto"],
    ["max-content", "auto", "min-content"],
    ["auto", "min-content", "max-content"],
    ["auto", "max-content", "min-content"],
  ];
}

function capAssignments() {
  return [
    ["0px", "10px", "20%"],
    ["0px", "20%", "10px"],
    ["10px", "0px", "20%"],
    ["10px", "20%", "0px"],
    ["20%", "0px", "10px"],
    ["20%", "10px", "0px"],
  ];
}

function validateScene(record) {
  const { scene } = record;
  assert(
    scene.fitPositions.length === 3 &&
      scene.fitCaps.length === 3 &&
      scene.fitPositions[0] < scene.fitPositions[1] &&
      scene.fitPositions[1] < scene.fitPositions[2] &&
      scene.fitPositions[0] >= 1 &&
      scene.fitPositions[2] <= 6,
    `scene ${record.index}: invalid three-fit positions`,
  );
  assert(
    record.scene.trackMembers.filter((track) => track.startsWith("fit-content(")).length === 3 &&
      record.scene.trackMembers.filter((track) => track === "min-content").length === 1 &&
      record.scene.trackMembers.filter((track) => track === "max-content").length === 1 &&
      record.scene.trackMembers.filter((track) => track === "auto").length === 1,
    `scene ${record.index}: wrong three-fit track multiset`,
  );
  for (let index = 0; index < scene.fitPositions.length; index += 1) {
    assert(
      scene.trackMembers[scene.fitPositions[index] - 1] ===
        `fit-content(${scene.fitCaps[index]})`,
      `scene ${record.index}: fit cap does not match its authored position`,
    );
  }
  const fitPositionSet = new Set(scene.fitPositions);
  const observedTail = scene.trackMembers.filter((_, index) => !fitPositionSet.has(index + 1));
  assert(
    JSON.stringify(observedTail) === JSON.stringify(scene.tail),
    `scene ${record.index}: remaining track order changed`,
  );
  assert(
    expectedTailOrders().some((tail) => JSON.stringify(tail) === JSON.stringify(scene.tail)),
    `scene ${record.index}: unplanned tail order`,
  );
  assert(record.rowValues.length === 7, `scene ${record.index}: expected seven used rows`);
  assert(
    record.rowValues.every((value) => Number.isFinite(value) && value >= 0),
    `scene ${record.index}: used rows must be finite and non-negative`,
  );
  const serializedRows = record.usedRows.trim().split(/\s+/).map(Number.parseFloat);
  assert(serializedRows.length === 7, `scene ${record.index}: computed row count changed`);
  assert(
    serializedRows.every((value, index) => near(value, record.rowValues[index], 0.001)),
    `scene ${record.index}: row-value observation mismatch`,
  );
  assert(record.computedAutoRows === scene.trackList, `scene ${record.index}: authored track order changed`);
  assert(record.computedAutoRows.trim().split(/\s+/).length === 6, `scene ${record.index}: auto-row list length`);
  assert(record.computedAlignContent === scene.alignContent, `scene ${record.index}: alignment changed`);
  assert(record.gridFrame.length === 4 && frameError(record.gridFrame, [0, 0, 40, 220]) <= 0.001,
    `scene ${record.index}: definite Grid frame changed`);
  const baseTopology = scene.topology.replace(/-(forward|reverse)$/, "");
  const orderedExpectedItems = expectedTopologyItems[baseTopology];
  assert(orderedExpectedItems, `scene ${record.index}: unknown topology`);
  const expectedItems = scene.topology.endsWith("-reverse")
    ? [...orderedExpectedItems].reverse()
    : orderedExpectedItems;
  assert(JSON.stringify(scene.items) === JSON.stringify(expectedItems), `scene ${record.index}: item topology changed`);

  let maxSpanFrameErrorPx = 0;
  let eligibleCount = 0;
  let candidateFloorViolations = 0;
  for (const item of record.items) {
    if (item.id === "empty") {
      assert(frameError(item.frame, [0, 0, 40, 10]) <= 0.01, `scene ${record.index}: explicit-row control`);
      assert(item.minHeight === "0px", `scene ${record.index}: explicit-row minimum changed`);
      continue;
    }
    assert(item.minHeight === (scene.minimum === "auto" ? "auto" : "0px"),
      `scene ${record.index}: ${item.id} minimum mode changed`);
    const placement = scene.items.find(([id]) => id === item.id);
    assert(placement, `scene ${record.index}: placement missing for ${item.id}`);
    assert(near(item.frame[0], 0) && near(item.frame[2], 40), `scene ${record.index}: column changed for ${item.id}`);
    const [, startLine, span] = placement;
    const firstRowIndex = startLine - 2;
    const endRowIndex = firstRowIndex + span;
    const expectedY = record.rowValues.slice(0, startLine - 1).reduce((sum, size) => sum + size, 0);
    const expectedHeight = record.rowValues.slice(firstRowIndex + 1, endRowIndex + 1)
      .reduce((sum, size) => sum + size, 0);
    const error = Math.max(
      Math.abs(item.frame[1] - expectedY),
      Math.abs(item.frame[3] - expectedHeight),
    );
    maxSpanFrameErrorPx = Math.max(maxSpanFrameErrorPx, error);
    assert(error <= 0.01, `scene ${record.index}: ${item.id} frame differs from its spanned rows`);
    assert(item.text === expectedText[item.id], `scene ${record.index}: text payload changed for ${item.id}`);
    assert(item.lineRects.length === expectedLineCount[item.id], `scene ${record.index}: line count changed for ${item.id}`);
    assert(item.payloadFrame, `scene ${record.index}: payload frame missing for ${item.id}`);

    const coveredTracks = scene.trackMembers.slice(firstRowIndex, endRowIndex);
    const eligible = coveredTracks.some((track) => track === "auto" || track.startsWith("fit-content("));
    if (eligible) {
      eligibleCount += 1;
      if (scene.minimum === "auto" && item.frame[3] + 0.01 < item.payloadFrame[3]) {
        candidateFloorViolations += 1;
      }
    }
  }
  return { maxSpanFrameErrorPx, eligibleCount, candidateFloorViolations };
}

function validateRelationships(records) {
  const sourceGroups = new Map();
  const alignmentGroups = new Map();
  const minimumGroups = new Map();
  const capGroups = new Map();
  let maxSourceOrderDeltaPx = 0;
  let maxNonAutoStretchDeltaPx = 0;
  let maxAutoStretchDeltaPx = 0;
  let changedItemFrames = 0;
  let maxMinimumItemDeltaPx = 0;
  let maxMinimumTrackDeltaPx = 0;
  let minimumFloorViolationCount = 0;
  let automaticMinimumEligibleOccurrences = 0;

  for (const record of records) {
    const { scene } = record;
    const baseTopology = scene.topology.replace(/-(forward|reverse)$/, "");
    const direction = scene.topology.endsWith("-reverse") ? "reverse" : "forward";
    const sourceKey = `${scene.trackList}|${scene.alignContent}|${scene.minimum}|${baseTopology}`;
    const sourceGroup = sourceGroups.get(sourceKey) ?? {};
    sourceGroup[direction] = record;
    sourceGroups.set(sourceKey, sourceGroup);

    const alignmentKey = `${scene.trackList}|${scene.topology}|${scene.minimum}`;
    const alignmentGroup = alignmentGroups.get(alignmentKey) ?? {};
    alignmentGroup[scene.alignContent] = record;
    alignmentGroups.set(alignmentKey, alignmentGroup);

    const minimumKey = `${scene.trackList}|${scene.alignContent}|${scene.topology}`;
    const minimumGroup = minimumGroups.get(minimumKey) ?? {};
    minimumGroup[scene.minimum] = record;
    minimumGroups.set(minimumKey, minimumGroup);

    const capKey = JSON.stringify([
      scene.fitPositions,
      scene.tail,
      scene.alignContent,
      scene.minimum,
      baseTopology,
      direction,
    ]);
    const capGroup = capGroups.get(capKey) ?? [];
    capGroup.push(record);
    capGroups.set(capKey, capGroup);
  }

  for (const [key, group] of sourceGroups) {
    assert(group.forward && group.reverse, `source-order pair missing for ${key}`);
    assert(group.forward.rowValues.length === group.reverse.rowValues.length, `row count changed for ${key}`);
    for (let index = 0; index < group.forward.rowValues.length; index += 1) {
      const delta = Math.abs(group.forward.rowValues[index] - group.reverse.rowValues[index]);
      maxSourceOrderDeltaPx = Math.max(maxSourceOrderDeltaPx, delta);
      assert(delta <= 0.01, `source-order row changed for ${key}`);
    }
    const reverseItems = new Map(group.reverse.items.map((item) => [item.id, item]));
    for (const item of group.forward.items) {
      const reverse = reverseItems.get(item.id);
      assert(reverse, `source-order item missing for ${key}`);
      const delta = frameError(item.frame, reverse.frame);
      maxSourceOrderDeltaPx = Math.max(maxSourceOrderDeltaPx, delta);
      assert(delta <= 0.01, `source-order frame changed for ${key}: ${item.id}`);
    }
  }

  for (const [key, group] of alignmentGroups) {
    assert(group.start && group.stretch, `alignment pair missing for ${key}`);
    const autoTrackIndex = group.start.scene.trackMembers.indexOf("auto") + 1;
    assert(autoTrackIndex > 0, `auto track missing for ${key}`);
    for (let row = 0; row < 7; row += 1) {
      const delta = group.stretch.rowValues[row] - group.start.rowValues[row];
      if (row === autoTrackIndex) {
        assert(delta >= -0.01, `stretch shrank auto track for ${key}`);
        maxAutoStretchDeltaPx = Math.max(maxAutoStretchDeltaPx, delta);
      } else {
        maxNonAutoStretchDeltaPx = Math.max(maxNonAutoStretchDeltaPx, Math.abs(delta));
        assert(Math.abs(delta) <= 0.01, `stretch changed non-auto row ${row} for ${key}`);
      }
    }
    const startTotal = group.start.rowValues.reduce((sum, size) => sum + size, 0);
    const stretchTotal = group.stretch.rowValues.reduce((sum, size) => sum + size, 0);
    const expectedTotal = Math.max(startTotal, 220);
    assert(near(stretchTotal, expectedTotal, 0.02), `stretch total changed for ${key}`);
    assert(near(group.stretch.rowValues[autoTrackIndex] - group.start.rowValues[autoTrackIndex],
      expectedTotal - startTotal, 0.02), `stretch free space did not go to auto for ${key}`);
  }

  for (const [key, group] of minimumGroups) {
    assert(group.zero && group.auto, `minimum pair missing for ${key}`);
    const autoMinimumValidation = validateScene(group.auto);
    minimumFloorViolationCount += autoMinimumValidation.candidateFloorViolations;
    automaticMinimumEligibleOccurrences += autoMinimumValidation.eligibleCount;
    for (let row = 0; row < 7; row += 1) {
      maxMinimumTrackDeltaPx = Math.max(maxMinimumTrackDeltaPx,
        Math.abs(group.auto.rowValues[row] - group.zero.rowValues[row]));
    }
    const zeroItems = new Map(group.zero.items.map((item) => [item.id, item]));
    for (const item of group.auto.items) {
      const zero = zeroItems.get(item.id);
      assert(zero, `minimum pair item missing for ${key}: ${item.id}`);
      const delta = frameError(item.frame, zero.frame);
      maxMinimumItemDeltaPx = Math.max(maxMinimumItemDeltaPx, delta);
      if (delta > 0.01) changedItemFrames += 1;
    }
  }

  for (const [key, group] of capGroups) {
    assert(group.length === 6, `expected six cap permutations for ${key}`);
    const assignments = new Set(group.map((record) => record.scene.fitCaps.join("|")));
    assert(assignments.size === 6 && capAssignments().every((caps) => assignments.has(caps.join("|"))),
      `cap assignments incomplete for ${key}`);
  }

  assert(sourceGroups.size === 5_760, `expected 5,760 source-order pairs, got ${sourceGroups.size}`);
  assert(alignmentGroups.size === 5_760, `expected 5,760 alignment pairs, got ${alignmentGroups.size}`);
  assert(minimumGroups.size === 5_760, `expected 5,760 automatic-minimum pairs, got ${minimumGroups.size}`);
  assert(capGroups.size === 1_920, `expected 1,920 cap sensitivity groups, got ${capGroups.size}`);
  assert(maxNonAutoStretchDeltaPx <= 0.01, "stretch changed a non-auto track");
  assert(minimumFloorViolationCount === 0, "automatic minimum fell below eligible payload floor");
  return {
    sourceOrderPairs: sourceGroups.size,
    alignmentPairs: alignmentGroups.size,
    automaticMinimumPairs: minimumGroups.size,
    capSensitivityGroups: capGroups.size,
    changedItemFramesAfterAutomaticMinimum: changedItemFrames,
    maximumAutomaticMinimumItemFrameDeltaCssPx: maxMinimumItemDeltaPx,
    maximumAutomaticMinimumTrackDeltaCssPx: maxMinimumTrackDeltaPx,
    automaticMinimumEligibleItemOccurrences: automaticMinimumEligibleOccurrences,
    candidateContentFloorViolations: minimumFloorViolationCount,
    maximumSourceOrderDeltaCssPx: maxSourceOrderDeltaPx,
    maximumNonAutoStretchDeltaCssPx: maxNonAutoStretchDeltaPx,
    maximumAutoTrackStretchDeltaCssPx: maxAutoStretchDeltaPx,
  };
}

try {
  runBrowser(["open", pathToFileURL(fixturePath).href]);
  opened = true;
  runBrowser(["set", "viewport", "1280", "900"]);
  const sceneCount = Number(runBrowser(["eval", "window.__c114ThreeFitScenes.length"]));
  assert(sceneCount === expectedSceneCount, `expected ${expectedSceneCount} scenes, got ${sceneCount}`);

  for (let start = 0; start < sceneCount; start += batchSize) {
    const end = Math.min(start + batchSize, sceneCount);
    const output = runBrowser([
      "eval",
      `JSON.stringify(window.runC114ThreeFitBatch(${start}, ${end - start}))`,
    ]);
    const batch = parseEvalOutput(output);
    assert(batch.start === start && batch.end === end && batch.total === sceneCount,
      "batch bounds mismatch");
    captured.push(...batch.results);
    if (captured.length % 960 === 0 || captured.length === sceneCount) {
      process.stdout.write(`CAPTURED ${captured.length}/${sceneCount}\n`);
    }
  }

  assert(captured.length === expectedSceneCount, "captured scene count mismatch");
  assert(captured.every((record, index) => record.index === index), "scene indices not continuous");
  const uniqueTrackLists = new Set(captured.map((record) => record.scene.trackList));
  assert(uniqueTrackLists.size === 720, `expected 720 track lists, got ${uniqueTrackLists.size}`);
  const validations = captured.map(validateScene);
  const maximumSpanFrameErrorPx = Math.max(...validations.map((item) => item.maxSpanFrameErrorPx));
  const zeroModeEligibleOccurrences = captured
    .filter((record) => record.scene.minimum === "zero")
    .reduce((sum, record) => sum + validations[record.index].eligibleCount, 0);
  const candidateFloorViolations = captured
    .filter((record) => record.scene.minimum === "auto")
    .reduce((sum, record) => sum + validations[record.index].candidateFloorViolations, 0);
  const relationships = validateRelationships(captured);

  writeFileSync(observationPath, `${captured.map((record) => JSON.stringify(record)).join("\n")}\n`);
  const sampleIndex = captured.findIndex(({ scene }) =>
    scene.fitPositions.join(",") === "1,3,6" &&
    scene.fitCaps.join(",") === "0px,10px,20%" &&
    scene.alignContent === "start" &&
    scene.topology === "span2Span3Six-forward" &&
    scene.minimum === "auto");
  assert(sampleIndex >= 0, "sample scene not found");
  runBrowser(["eval", `JSON.stringify(window.showC114ThreeFitScene(${sampleIndex}))`]);
  runBrowser(["eval", `(() => { const grid = document.querySelector("#grid"); grid.style.transform = "scale(4)"; grid.style.transformOrigin = "top left"; return true; })()`]);
  runBrowser(["set", "viewport", "480", "900"]);
  runBrowser(["screenshot", screenshotPath]);

  const result = {
    schema: "spinon.c11-4.repeated-text-row-span-three-fit-auto-minimum-precomparison.v1",
    capturedAtUtc: new Date().toISOString(),
    browserCli,
    browserUserAgent: captured[0].userAgent,
    devicePixelRatio: captured[0].devicePixelRatio,
    viewport: [1280, 900],
    sampleScreenshotScale: 4,
    sampleScreenshotViewport: [480, 900],
    fixture: { path: fixtureRelativePath, sha256: fixtureSha256 },
    collector: {
      path: "spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-precomparison-2026-11-10/capture-chrome.mjs",
      sha256: captureScriptSha256,
    },
    sceneCount: captured.length,
    pairedKeys: captured.length / 2,
    uniqueTrackListCount: uniqueTrackLists.size,
    fitPositionTriples: 20,
    typedCapAssignments: 6,
    tailOrders: 6,
    alignments: ["start", "stretch"],
    spanTopologies: [
      "span2Six-forward",
      "span2Six-reverse",
      "span2Span3Six-forward",
      "span2Span3Six-reverse",
    ],
    minimumModes: ["zero", "auto"],
    gridSizeCssPx: [40, 220],
    expectedTrackSizeMultiset: {
      "fit-content()": 3,
      "min-content": 1,
      "max-content": 1,
      auto: 1,
    },
    automaticMinimumEligibility: {
      rule: "non-scrollable item spans at least one auto-minimum track and no flexible track",
      capValues: ["0px", "10px", "20%"],
      percentBasisCssPx: 220,
      zeroModeEligibleItemOccurrences: zeroModeEligibleOccurrences,
      candidateFloorViolations,
    },
    independentChecks: {
      allScenesHaveSevenUsedRows: true,
      authoredSixMemberTrackOrderPreserved: true,
      itemFramesEqualTheirSpannedUsedRowSums: true,
      sourceOrderReversalPreservesSemanticFrames: relationships.maximumSourceOrderDeltaCssPx <= 0.01,
      alignContentStretchChangesOnlyAutoTrack: relationships.maximumNonAutoStretchDeltaCssPx <= 0.01,
      maximumSpanFrameErrorCssPx: maximumSpanFrameErrorPx,
      ...relationships,
    },
    artifacts: {
      observations: "chrome157-observation.ndjson",
      screenshot: "chrome157-three-fit-auto-minimum-sample.png",
    },
  };
  writeFileSync(verificationPath, `${JSON.stringify(result, null, 2)}\n`);
  process.stdout.write(`VERIFIED ${JSON.stringify(result)}\n`);
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
