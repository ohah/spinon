import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const evidenceDir = dirname(fileURLToPath(import.meta.url));
const repositoryRoot = resolve(evidenceDir, "../../../..");
const fixturePath = resolve(
  repositoryRoot,
  "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-two-fit-content-precomparison.html",
);
const observationPath = resolve(evidenceDir, "chrome-observation.ndjson");
const verificationPath = resolve(evidenceDir, "verification.json");
const screenshotPath = resolve(evidenceDir, "chrome157-dual-fit-content-sample.png");
const lifecyclePath = resolve(evidenceDir, "agent-browser-lifecycle.json");
const browserCli = process.env.AGENT_BROWSER_PATH ?? "/opt/homebrew/bin/agent-browser";
const session = "spinon-c114-dual-fit-precomparison-20261109";
const expectedSceneCount = 3_840;
const batchSize = 120;
const expectedText = {
  A: "AAAA BBBB CCCC DDDD EEEE FFFF",
  B: "AAAAAA BBBBB CCCC DDD",
  C: "AAAAA BBBB CCC DDDD",
  D: "AAAA BBB CCCCC DDDD",
  E: "AAA BBBB CCCCCC",
};
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
const fixture = readFileSync(fixturePath);
const fixtureSha256 = createHash("sha256").update(fixture).digest("hex");
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

function near(actual, expected, tolerance = 0.01) {
  return Number.isFinite(actual) && Math.abs(actual - expected) <= tolerance;
}

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

function frameError(actual, expected) {
  return Math.max(...actual.map((value, index) => Math.abs(value - expected[index])));
}

function validateScene(record) {
  const { scene } = record;
  assert(
    scene.fitPositions.length === 2 &&
      scene.fitCaps.length === 2 &&
      scene.fitPositions[0] >= 1 &&
      scene.fitPositions[1] <= 6 &&
      scene.fitPositions[0] < scene.fitPositions[1],
    `scene ${record.index}: invalid fit-content track positions`,
  );
  assert(
    scene.trackMembers.filter((track) => track.startsWith("fit-content(")).length === 2 &&
      scene.trackMembers.filter((track) => track === "min-content").length === 2 &&
      scene.trackMembers.filter((track) => track === "max-content").length === 1 &&
      scene.trackMembers.filter((track) => track === "auto").length === 1,
    `scene ${record.index}: unsupported track-size multiplicity`,
  );
  for (let index = 0; index < scene.fitPositions.length; index += 1) {
    assert(
      scene.trackMembers[scene.fitPositions[index] - 1] ===
        `fit-content(${scene.fitCaps[index]})`,
      `scene ${record.index}: fit cap does not match its authored track position`,
    );
  }
  const fitPositionSet = new Set(scene.fitPositions);
  const observedTail = scene.trackMembers.filter((_, index) => !fitPositionSet.has(index + 1));
  assert(
    JSON.stringify(observedTail) === JSON.stringify(scene.tail),
    `scene ${record.index}: remaining track order changed`,
  );
  const baseTopology = scene.topology.replace(/-(forward|reverse)$/, "");
  const expectedItems = expectedTopologyItems[baseTopology];
  assert(expectedItems, `scene ${record.index}: unknown topology`);
  const orderedExpectedItems = scene.topology.endsWith("-reverse")
    ? [...expectedItems].reverse()
    : expectedItems;
  assert(
    JSON.stringify(scene.items) === JSON.stringify(orderedExpectedItems),
    `scene ${record.index}: topology placement changed`,
  );
  assert(record.rowValues.length === 7, `scene ${record.index}: expected 7 used rows`);
  assert(
    record.rowValues.every((value) => Number.isFinite(value) && value >= 0),
    `scene ${record.index}: used row sizes must be finite and non-negative`,
  );
  const cssRows = record.usedRows.trim().split(/\s+/).map(Number.parseFloat);
  assert(cssRows.length === 7, `scene ${record.index}: used row serialization count`);
  assert(
    cssRows.every((value, index) => near(value, record.rowValues[index], 0.001)),
    `scene ${record.index}: parsed used rows disagree with numeric observations`,
  );
  assert(
    record.computedAutoRows.trim().split(/\s+/).length === 6,
    `scene ${record.index}: computed grid-auto-rows must preserve six members`,
  );
  assert(
    record.computedAutoRows === scene.trackList,
    `scene ${record.index}: computed grid-auto-rows changed authored member order`,
  );
  assert(
    record.computedAlignContent === scene.alignContent,
    `scene ${record.index}: computed align-content changed`,
  );
  assert(
    (record.computedAutoRows.match(/fit-content\(/g) ?? []).length === 2,
    `scene ${record.index}: computed grid-auto-rows must preserve two fit-content members`,
  );
  assert(
    record.gridFrame.length === 4 && frameError(record.gridFrame, [0, 0, 40, 220]) <= 0.001,
    `scene ${record.index}: definite Grid frame changed`,
  );

  let maxSpanFrameErrorPx = 0;
  for (const item of record.items) {
    if (item.id === "empty") {
      assert(frameError(item.frame, [0, 0, 40, 10]) <= 0.01, `scene ${record.index}: explicit row`);
      continue;
    }
    const placement = scene.items.find(([id]) => id === item.id);
    assert(placement, `scene ${record.index}: missing placement for ${item.id}`);
    assert(
      near(item.frame[0], 0, 0.01) && near(item.frame[2], 40, 0.01),
      `scene ${record.index}: ${item.id} changed its explicit column frame`,
    );
    const [, startLine, span] = placement;
    const firstTrack = startLine - 1;
    const expectedY = record.rowValues.slice(0, firstTrack).reduce((sum, size) => sum + size, 0);
    const expectedHeight = record.rowValues
      .slice(firstTrack, firstTrack + span)
      .reduce((sum, size) => sum + size, 0);
    const error = Math.max(
      Math.abs(item.frame[1] - expectedY),
      Math.abs(item.frame[3] - expectedHeight),
    );
    maxSpanFrameErrorPx = Math.max(maxSpanFrameErrorPx, error);
    assert(error <= 0.01, `scene ${record.index}: ${item.id} span frame != used-row sum`);
    assert(item.payloadFrame, `scene ${record.index}: ${item.id} payload frame missing`);
    assert(
      /^[A-Z]{1,6}( [A-Z]{1,6})*$/.test(item.text) && new TextEncoder().encode(item.text).length <= 32,
      `scene ${record.index}: ${item.id} content grammar changed`,
    );
    assert(item.text === expectedText[item.id], `scene ${record.index}: ${item.id} payload changed`);
    assert(item.lineRects.length > 0, `scene ${record.index}: ${item.id} text line rects missing`);
    const expectedLineCount = { A: 6, B: 4, C: 4, D: 4, E: 3 }[item.id];
    assert(
      item.lineRects.length === expectedLineCount,
      `scene ${record.index}: ${item.id} expected ${expectedLineCount} text lines, got ${item.lineRects.length}`,
    );
    const lineTops = item.lineRects.map((line) => line[1]);
    assert(
      lineTops.every((top, index) => index === 0 || top > lineTops[index - 1]),
      `scene ${record.index}: ${item.id} line positions must increase`,
    );
  }
  return maxSpanFrameErrorPx;
}

function validatePairs(records) {
  const sourceOrderPairs = new Map();
  const alignmentPairs = new Map();
  let maxSourceOrderFrameDeltaPx = 0;
  let maxNonAutoStretchDeltaPx = 0;
  let maxAutoTrackStretchDeltaPx = 0;

  for (const record of records) {
    const { scene } = record;
    const baseTopology = scene.topology.replace(/-(forward|reverse)$/, "");
    const direction = scene.topology.endsWith("-reverse") ? "reverse" : "forward";
    const sourceKey = `${scene.trackList}|${scene.alignContent}|${baseTopology}`;
    const sourceGroup = sourceOrderPairs.get(sourceKey) ?? {};
    sourceGroup[direction] = record;
    sourceOrderPairs.set(sourceKey, sourceGroup);

    const alignKey = `${scene.trackList}|${scene.topology}`;
    const alignGroup = alignmentPairs.get(alignKey) ?? {};
    alignGroup[scene.alignContent] = record;
    alignmentPairs.set(alignKey, alignGroup);
  }

  for (const [key, pair] of sourceOrderPairs) {
    assert(pair.forward && pair.reverse, `source-order pair missing for ${key}`);
    assert(
      pair.forward.rowValues.length === pair.reverse.rowValues.length,
      `source-order row count changed for ${key}`,
    );
    for (let index = 0; index < pair.forward.rowValues.length; index += 1) {
      const delta = Math.abs(pair.forward.rowValues[index] - pair.reverse.rowValues[index]);
      maxSourceOrderFrameDeltaPx = Math.max(maxSourceOrderFrameDeltaPx, delta);
      assert(delta <= 0.01, `source-order row sizing changed for ${key}, row ${index}`);
    }
    const reverseById = new Map(pair.reverse.items.map((item) => [item.id, item]));
    for (const item of pair.forward.items) {
      const reverse = reverseById.get(item.id);
      assert(reverse, `source-order item missing for ${key}: ${item.id}`);
      const delta = frameError(item.frame, reverse.frame);
      maxSourceOrderFrameDeltaPx = Math.max(maxSourceOrderFrameDeltaPx, delta);
      assert(delta <= 0.01, `source-order item frame changed for ${key}: ${item.id}`);
      if (item.payloadFrame) {
        assert(reverse.payloadFrame, `source-order payload missing for ${key}: ${item.id}`);
        const payloadDelta = frameError(item.payloadFrame, reverse.payloadFrame);
        maxSourceOrderFrameDeltaPx = Math.max(maxSourceOrderFrameDeltaPx, payloadDelta);
        assert(payloadDelta <= 0.01, `source-order payload frame changed for ${key}: ${item.id}`);
        assert(item.lineRects.length === reverse.lineRects.length, `line count changed for ${key}: ${item.id}`);
        for (let index = 0; index < item.lineRects.length; index += 1) {
          const lineDelta = frameError(item.lineRects[index], reverse.lineRects[index]);
          maxSourceOrderFrameDeltaPx = Math.max(maxSourceOrderFrameDeltaPx, lineDelta);
          assert(lineDelta <= 0.01, `source-order text line changed for ${key}: ${item.id}`);
        }
      }
    }
  }

  for (const [key, pair] of alignmentPairs) {
    assert(pair.start && pair.stretch, `alignment pair missing for ${key}`);
    const { trackMembers } = pair.start.scene;
    const autoTrackIndices = trackMembers
      .map((track, index) => (track === "auto" ? index : -1))
      .filter((index) => index >= 0);
    assert(autoTrackIndices.length === 1, `expected exactly one auto track for ${key}`);
    const autoTrackRowIndex = autoTrackIndices[0] + 1;
    let autoTrackStretchDeltaPx = 0;
    for (let row = 0; row < 7; row += 1) {
      const delta = pair.stretch.rowValues[row] - pair.start.rowValues[row];
      if (row === autoTrackRowIndex) {
        assert(delta >= -0.01, `stretch shrank auto row for ${key}`);
        autoTrackStretchDeltaPx = delta;
      } else {
        maxNonAutoStretchDeltaPx = Math.max(maxNonAutoStretchDeltaPx, Math.abs(delta));
        assert(Math.abs(delta) <= 0.01, `stretch changed a non-auto track for ${key}, row ${row}`);
      }
    }
    const startTotal = pair.start.rowValues.reduce((sum, size) => sum + size, 0);
    const stretchTotal = pair.stretch.rowValues.reduce((sum, size) => sum + size, 0);
    const expectedStretchTotal = Math.max(startTotal, 220);
    assert(near(stretchTotal, expectedStretchTotal, 0.02), `stretch total mismatch for ${key}`);
    const expectedAutoTrackDeltaPx = expectedStretchTotal - startTotal;
    assert(
      near(autoTrackStretchDeltaPx, expectedAutoTrackDeltaPx, 0.02),
      `stretch free space was not assigned to the auto track for ${key}`,
    );
    maxAutoTrackStretchDeltaPx = Math.max(
      maxAutoTrackStretchDeltaPx,
      autoTrackStretchDeltaPx,
    );
  }

  assert(sourceOrderPairs.size === 1_920, "expected 1,920 source-order pairs");
  assert(alignmentPairs.size === 1_920, "expected 1,920 alignment pairs");
  return {
    sourceOrderPairs: sourceOrderPairs.size,
    alignmentPairs: alignmentPairs.size,
    maxSourceOrderFrameDeltaPx,
    maxNonAutoStretchDeltaPx,
    maxAutoTrackStretchDeltaPx,
  };
}

function summarizeCapSensitivity(records) {
  const groups = new Map();
  const fitRowRanges = new Map();
  const capTokens = new Set();
  for (const record of records) {
    const { scene } = record;
    const baseTopology = scene.topology.replace(/-(forward|reverse)$/, "");
    const direction = scene.topology.endsWith("-reverse") ? "reverse" : "forward";
    const key = JSON.stringify([
      scene.fitPositions,
      scene.tail,
      scene.alignContent,
      baseTopology,
      direction,
    ]);
    const group = groups.get(key) ?? [];
    group.push(record);
    groups.set(key, group);
    for (const cap of scene.fitCaps) capTokens.add(cap);
    for (const position of scene.fitPositions) {
      const range = fitRowRanges.get(position) ?? { min: Infinity, max: -Infinity };
      const usedSize = record.rowValues[position];
      range.min = Math.min(range.min, usedSize);
      range.max = Math.max(range.max, usedSize);
      fitRowRanges.set(position, range);
    }
  }

  let capSensitiveGroups = 0;
  let maximumTrackRangeAcrossCapVariantsCssPx = 0;
  for (const [key, group] of groups) {
    assert(group.length === 8, `expected eight cap assignments for ${key}`);
    const assignments = new Set(group.map((record) => record.scene.fitCaps.join("|")));
    const expectedAssignments = [
      "0px|0%",
      "0%|0px",
      "10px|20px",
      "20px|10px",
      "10px|20%",
      "20%|10px",
      "20%|120%",
      "120%|20%",
    ];
    assert(
      assignments.size === 8 && expectedAssignments.every((assignment) => assignments.has(assignment)),
      `cap assignments are incomplete or duplicated for ${key}`,
    );
    let groupRange = 0;
    for (let row = 0; row < 7; row += 1) {
      const values = group.map((record) => record.rowValues[row]);
      const range = Math.max(...values) - Math.min(...values);
      groupRange = Math.max(groupRange, range);
      maximumTrackRangeAcrossCapVariantsCssPx = Math.max(
        maximumTrackRangeAcrossCapVariantsCssPx,
        range,
      );
    }
    if (groupRange > 0.01) capSensitiveGroups += 1;
  }

  assert(groups.size === 480, `expected 480 cap-sensitivity groups, got ${groups.size}`);
  return {
    capSensitivityGroups: capSensitiveGroups,
    totalCapSensitivityGroups: groups.size,
    maximumTrackRangeAcrossCapVariantsCssPx,
    fitRowSizeRangesByTrackListPosition: Object.fromEntries(
      [...fitRowRanges].map(([position, range]) => [position, [range.min, range.max]]),
    ),
    capTokens: [...capTokens].sort(),
    definiteBlockBasisForPercentCapsCssPx: { "20%": 44, "120%": 264 },
  };
}

try {
  runBrowser(["open", pathToFileURL(fixturePath).href]);
  opened = true;
  runBrowser(["set", "viewport", "1280", "900"]);
  const sceneCount = Number(runBrowser(["eval", "window.__c114DualFitScenes.length"]));
  assert(sceneCount === expectedSceneCount, `expected ${expectedSceneCount} scenes, got ${sceneCount}`);

  for (let start = 0; start < sceneCount; start += batchSize) {
    const end = Math.min(start + batchSize, sceneCount);
    const output = runBrowser([
      "eval",
      `JSON.stringify(window.runC114DualFitBatch(${start}, ${end - start}))`,
    ]);
    const batch = parseEvalOutput(output);
    assert(batch.start === start && batch.end === end && batch.total === sceneCount, "batch bounds mismatch");
    captured.push(...batch.results);
    process.stdout.write(`CAPTURED ${captured.length}/${sceneCount}\n`);
  }

  assert(captured.length === expectedSceneCount, "scene count mismatch after capture");
  assert(
    captured.every((record, index) => record.index === index),
    "scene indices are not continuous and unique",
  );
  const uniqueTrackLists = new Set(captured.map((record) => record.scene.trackList));
  assert(uniqueTrackLists.size === 480, `expected 480 unique track lists, got ${uniqueTrackLists.size}`);
  const maxSpanFrameErrorPx = Math.max(...captured.map(validateScene));
  const pairChecks = validatePairs(captured);
  const capSensitivity = summarizeCapSensitivity(captured);

  writeFileSync(
    observationPath,
    `${captured.map((record) => JSON.stringify(record)).join("\n")}\n`,
  );
  const sampleIndex = captured.findIndex(
    ({ scene }) =>
      scene.fitPositions[0] === 2 &&
      scene.fitPositions[1] === 5 &&
      scene.fitCaps[0] === "10px" &&
      scene.fitCaps[1] === "20px" &&
      scene.alignContent === "stretch" &&
      scene.topology === "span2Span3Six-forward",
  );
  assert(sampleIndex >= 0, "sample scene not found");
  runBrowser(["eval", `JSON.stringify(window.showC114DualFitScene(${sampleIndex}))`]);
  runBrowser(["screenshot", screenshotPath]);

  const result = {
    schema: "spinon.c11-4.repeated-text-row-span-two-fit-content-precomparison.v1",
    capturedAtUtc: new Date().toISOString(),
    browserCli: browserCli,
    browserUserAgent: captured[0].userAgent,
    devicePixelRatio: captured[0].devicePixelRatio,
    viewport: [1280, 900],
    fixture: {
      path: "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-two-fit-content-precomparison.html",
      sha256: fixtureSha256,
    },
    collector: {
      path: "spec/internal/evidence/c11-4-root-html-repeated-text-row-span-two-fit-content-precomparison-2026-11-09/capture-chrome.mjs",
      sha256: captureScriptSha256,
    },
    sceneCount: captured.length,
    uniqueTrackListCount: uniqueTrackLists.size,
    trackPositionPairs: 15,
    capPairsIncludingBothAssignments: 8,
    tailOrders: 4,
    alignments: ["start", "stretch"],
    spanTopologies: ["span2Six-forward", "span2Six-reverse", "span2Span3Six-forward", "span2Span3Six-reverse"],
    gridSizeCssPx: [40, 220],
    expectedTrackSizeMultiset: {
      "fit-content()": 2,
      "min-content": 2,
      "max-content": 1,
      auto: 1,
    },
    independentChecks: {
      allScenesHaveSevenUsedRows: true,
      authoredSixMemberTrackOrderPreserved: true,
      itemFramesEqualTheirSpannedUsedRowSums: true,
      sourceOrderReversalPreservesSemanticFrames: true,
      alignContentStretchChangesOnlyAutoTrack: true,
      maximumSpanFrameErrorCssPx: maxSpanFrameErrorPx,
      ...pairChecks,
      capSensitivity,
    },
    artifacts: {
      observations: "chrome-observation.ndjson",
      screenshot: "chrome157-dual-fit-content-sample.png",
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
    `${JSON.stringify({
      browserCli,
      session,
      opened,
      closed,
      observedAtUtc: new Date().toISOString(),
    }, null, 2)}\n`,
  );
  if (closeError) throw new Error(`Agent Browser session cleanup failed: ${closeError}`);
}
