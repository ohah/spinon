import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, statSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const evidenceDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(evidenceDir, '../../../..');
const androidDir = resolve(evidenceDir, 'android-avd');
const iosDir = resolve(evidenceDir, 'ios-simulator');
const oraclePath = resolve(
  repoRoot,
  'spec/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/chrome/chrome155-observation.json',
);
const oracle = JSON.parse(readFileSync(oraclePath, 'utf8'));
const scenes = [
  'auto-image-definite',
  'minmax-auto-image',
  'minmax-min-max-image',
  'fit-content-below-minimum',
  'fit-content-above-intrinsic',
  'fit-content-percent',
  'fixed-small-track-overflow',
  'spanning-min-content-items',
  'min-content-row-image',
];
const pixelExpected = [
  { id: 'item-inside-root', xCssPx: 5, yCssPx: 8, rgba: [229, 72, 77, 255] },
  { id: 'item-overflow-beyond-root', xCssPx: 20, yCssPx: 8, rgba: [229, 72, 77, 255] },
  { id: 'root-background-control', xCssPx: 6, yCssPx: 25, rgba: [16, 24, 32, 255] },
];

function readTrace(platformDir, scene, platformRunName) {
  const logPath = resolve(platformDir, `runtime-${scene}.log`);
  const log = readFileSync(logPath, 'utf8');
  assert.doesNotMatch(log, /SPINON_C114_GRID_ASSERTION_ERROR|SPINON_C114_GRID_TRACE_MISSING/);
  const runId = `c114-image-${platformRunName}-${scene}-20261019-r4`;
  const chunks = log.split(/\r?\n/).flatMap((line) => {
    const marker = line.indexOf('SPINON_C114_GRID_TRACE_CHUNK ');
    if (marker < 0) return [];
    const match = line.slice(marker + 'SPINON_C114_GRID_TRACE_CHUNK '.length)
      .match(/^run_id=(\S+) part=(\d+)\/(\d+) (.*)$/);
    if (!match || match[1] !== runId) return [];
    return [{ part: Number(match[2]), total: Number(match[3]), text: match[4] }];
  }).sort((left, right) => left.part - right.part);
  assert.ok(chunks.length > 0, `${scene}: trace chunks missing in ${logPath}`);
  assert.equal(chunks.length, chunks[0].total, `${scene}: incomplete trace chunks`);
  chunks.forEach((chunk, index) => {
    assert.equal(chunk.part, index + 1, `${scene}: trace part sequence has a gap`);
    assert.equal(chunk.total, chunks[0].total, `${scene}: trace part count changed`);
  });
  const serialized = chunks.map((chunk) => chunk.text).join('');
  const prefix = 'grid_track_sizing_diagnostics=';
  assert.ok(serialized.startsWith(prefix), `${scene}: trace report prefix missing`);
  const diagnostics = JSON.parse(serialized.slice(prefix.length));
  assert.equal(diagnostics.layoutInputRevision.intrinsicSize, 1);
  assert.equal(diagnostics.gridItemIntrinsicInputs.length, 1);
  const input = diagnostics.gridItemIntrinsicInputs[0];
  assert.deepEqual(
    { grid: input.grid, node: input.node, sourceOrder: input.sourceOrder },
    { grid: 1, node: 2, sourceOrder: 0 },
  );
  assert.equal(input.source.kind, 'replaced');
  assert.equal(input.source.natural.width, 24);
  assert.equal(input.source.natural.height, 16);
  assert.equal(diagnostics.trackSizingTraces.length, 1);
  const passes = diagnostics.trackSizingTraces[0].passes;
  assert.deepEqual(passes.map((pass) => pass.axis), ['Columns', 'Rows']);
  return { logPath, log, runId, chunks, diagnostics };
}

function actualTracks(pass) {
  const finalStage = pass.stages.at(-1);
  assert.ok(finalStage, `${pass.axis}: final sizing stage missing`);
  return finalStage.tracks
    .filter((track) => !track.collapsed)
    .map((track) => ({
      size: track.baseSize,
      gutter: track.gutter,
    }));
}

function normalizedTrace(diagnostics) {
  return diagnostics.trackSizingTraces[0].passes.map((pass) => ({
    axis: pass.axis,
    contributions: pass.contributions.map((item) => ({
      item: item.item,
      kind: item.kind,
      value: item.value,
      span: item.span,
      crossesFlexibleTrack: item.crossesFlexibleTrack,
      minimumSource: typeof item.minimumSource === 'string'
        ? item.minimumSource.replace(/\s+/g, '')
        : item.minimumSource,
    })),
    stages: pass.stages.map((stage) => ({
      stage: stage.stage.replace(/\s+/g, ''),
      tracks: stage.tracks.map((track) => ({
        internalIndex: track.internalIndex,
        gutter: track.gutter,
        collapsed: track.collapsed,
        baseSize: track.baseSize,
        growthLimit: track.growthLimit,
        infinitelyGrowable: track.infinitelyGrowable,
      })),
    })),
  }));
}

function cssPxList(value) {
  return value.trim().split(/\s+/).map((item) => {
    assert.match(item, /^\d+(?:\.\d+)?px$/);
    return Number(item.slice(0, -2));
  });
}

function closeEnough(actual, expected) {
  return Math.abs(actual - expected) <= 0.05;
}

function readSummary(log, scene) {
  const line = log.split(/\r?\n/).find((item) =>
    item.includes('SPINON_C114_GRID_SUMMARY ') && item.includes(`scene_id=${scene} `));
  assert.ok(line, `${scene}: native app assertion summary missing`);
  assert.match(line, /status=0 layout=ready boxes=2/);
  const match = line.match(
    /expected_root_frame=width=(\d+(?:\.\d+)?),height=(\d+(?:\.\d+)?) expected_item_frame=width=(\d+(?:\.\d+)?),height=(\d+(?:\.\d+)?)/,
  );
  assert.ok(match, `${scene}: expected frame summary missing`);
  return {
    root: { width: Number(match[1]), height: Number(match[2]) },
    item: { width: Number(match[3]), height: Number(match[4]) },
    line: line.trim(),
  };
}

function verifyAndroidNodeFrames(log, scene, expected) {
  const lines = log.split(/\r?\n/).filter((line) =>
    line.includes('SPINON_C114_GRID_NODE_FRAME '));
  const frames = lines.map((line) => {
    const match = line.match(/SPINON_C114_GRID_NODE_FRAME (\d+):node=(\d+),x=([^,]+),y=([^,]+),width=([^,]+),height=([^\s]+)/);
    assert.ok(match, `${scene}: malformed Android node frame`);
    return {
      index: Number(match[1]), node: Number(match[2]), x: Number(match[3]), y: Number(match[4]),
      width: Number(match[5]), height: Number(match[6]),
    };
  });
  assert.equal(frames.length, 2, `${scene}: expected two Android frame records`);
  assert.deepEqual(frames, [
    { index: 0, node: 1, x: 0, y: 0, ...expected.root },
    { index: 1, node: 2, x: 0, y: 0, ...expected.item },
  ]);
  return frames;
}

function verifyPng(platformDir, scene) {
  const name = `screenshot-${scene}.png`;
  const path = resolve(platformDir, name);
  const bytes = readFileSync(path);
  assert.equal(bytes.subarray(0, 8).toString('hex'), '89504e470d0a1a0a');
  return {
    path: path.replace(`${repoRoot}/`, ''),
    bytes: statSync(path).size,
    sha256: createHash('sha256').update(bytes).digest('hex'),
  };
}

const comparisons = scenes.map((scene) => {
  const chrome = oracle.cases.find((item) => item.id === scene);
  assert.ok(chrome, `${scene}: Chrome oracle case missing`);
  assert.equal(chrome.naturalImage?.width, 24);
  assert.equal(chrome.naturalImage?.height, 16);
  const android = readTrace(androidDir, scene, 'android');
  const ios = readTrace(iosDir, scene, 'ios');
  const androidSummary = readSummary(android.log, scene);
  const iosSummary = readSummary(ios.log, scene);
  const androidPasses = android.diagnostics.trackSizingTraces[0].passes;
  const iosPasses = ios.diagnostics.trackSizingTraces[0].passes;
  const chromeColumns = cssPxList(chrome.columns);
  const chromeRows = cssPxList(chrome.rows);
  const androidColumns = actualTracks(androidPasses[0]);
  const androidRows = actualTracks(androidPasses[1]);
  const iosColumns = actualTracks(iosPasses[0]);
  const iosRows = actualTracks(iosPasses[1]);
  const spinonColumns = (tracks) => tracks.filter((track) => !track.gutter).map((track) => track.size);
  const spinonRows = (tracks) => tracks.filter((track) => !track.gutter).map((track) => track.size);
  const androidGaps = androidColumns.filter((track) => track.gutter).map((track) => track.size);
  const iosGaps = iosColumns.filter((track) => track.gutter).map((track) => track.size);
  const chromeColumnGap = Number.parseFloat(chrome.columnGap);
  for (const [platform, observed] of [
    ['Android', androidSummary],
    ['iOS', iosSummary],
  ]) {
    assert.ok(closeEnough(observed.root.width, chrome.grid.width), `${scene}: ${platform} root width differs`);
    assert.ok(closeEnough(observed.root.height, chrome.grid.height), `${scene}: ${platform} root height differs`);
    assert.ok(closeEnough(observed.item.width, chrome.item.width), `${scene}: ${platform} item width differs`);
    assert.ok(closeEnough(observed.item.height, chrome.item.height), `${scene}: ${platform} item height differs`);
  }
  for (const [platform, columns, rows, gaps] of [
    ['Android', spinonColumns(androidColumns), spinonRows(androidRows), androidGaps],
    ['iOS', spinonColumns(iosColumns), spinonRows(iosRows), iosGaps],
  ]) {
    assert.equal(columns.length, chromeColumns.length, `${scene}: ${platform} column count differs`);
    assert.equal(rows.length, chromeRows.length, `${scene}: ${platform} row count differs`);
    columns.forEach((value, index) => assert.ok(closeEnough(value, chromeColumns[index]), `${scene}: ${platform} column differs`));
    rows.forEach((value, index) => assert.ok(closeEnough(value, chromeRows[index]), `${scene}: ${platform} row differs`));
    const nonzeroGaps = gaps.filter((value) => value > 0);
    if (chromeColumnGap > 0) {
      assert.deepEqual(nonzeroGaps, [chromeColumnGap], `${scene}: ${platform} column gap differs`);
    } else {
      assert.deepEqual(nonzeroGaps, [], `${scene}: unexpected ${platform} column gap`);
    }
  }
  const expectedColumnContribution = [
    'fixed-small-track-overflow', 'min-content-row-image',
  ].includes(scene) ? null : 24;
  for (const [platform, passes] of [['Android', androidPasses], ['iOS', iosPasses]]) {
    const rowInput = passes[1].contributions.some((item) =>
      item.item === 2 && item.span === 1 && item.kind === 'MinContent' && item.value === 16);
    assert.ok(rowInput, `${scene}: ${platform} lacks replaced row min-content contribution`);
    if (expectedColumnContribution !== null) {
      const span = scene === 'spanning-min-content-items' ? 2 : 1;
      const columnInput = passes[0].contributions.some((item) =>
        item.item === 2 && item.span === span && item.kind === 'MinContent'
          && item.value === expectedColumnContribution);
      assert.ok(columnInput, `${scene}: ${platform} lacks replaced column min-content contribution`);
    }
  }
  const androidNormalized = normalizedTrace(android.diagnostics);
  const iosNormalized = normalizedTrace(ios.diagnostics);
  assert.deepEqual(androidNormalized, iosNormalized, `${scene}: Android/iOS sizing traces differ`);
  const androidNodeFrames = verifyAndroidNodeFrames(android.log, scene, androidSummary);
  const image = {
    naturalSize: android.diagnostics.gridItemIntrinsicInputs[0].source.natural,
    intrinsicRevision: android.diagnostics.layoutInputRevision.intrinsicSize,
  };
  assert.deepEqual(image, {
    naturalSize: ios.diagnostics.gridItemIntrinsicInputs[0].source.natural,
    intrinsicRevision: ios.diagnostics.layoutInputRevision.intrinsicSize,
  });
  return {
    scene,
    chrome: {
      columns: chromeColumns,
      rows: chromeRows,
      columnGap: chromeColumnGap,
      root: { width: chrome.grid.width, height: chrome.grid.height },
      item: { width: chrome.item.width, height: chrome.item.height },
      naturalImage: chrome.naturalImage,
      overflow: chrome.overflow,
    },
    android: {
      runId: android.runId,
      frames: androidSummary,
      nodeFrames: androidNodeFrames,
      columns: androidColumns,
      rows: androidRows,
      gaps: androidGaps,
      image,
      traceChunks: android.chunks.length,
      screenshot: verifyPng(androidDir, scene),
    },
    ios: {
      runId: ios.runId,
      frames: iosSummary,
      columns: iosColumns,
      rows: iosRows,
      gaps: iosGaps,
      image,
      traceChunks: ios.chunks.length,
      screenshot: verifyPng(iosDir, scene),
    },
    traceEquivalentAcrossPlatforms: true,
    chromeGeometryMatched: true,
  };
});

const overflowAndroidLog = readFileSync(resolve(androidDir, 'runtime-fixed-small-track-overflow.log'), 'utf8');
const overflowIosLog = readFileSync(resolve(iosDir, 'runtime-fixed-small-track-overflow.log'), 'utf8');
assert.match(overflowAndroidLog, /SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched .*expected_samples=3 observed_samples=3 matched_samples=3/);
assert.match(overflowIosLog, /SPINON_C114_METAL_PIXEL_SUMMARY status=matched .*expected_samples=3 observed_samples=3 matched_samples=3/);
const pixelSamples = pixelExpected.map((expected) => {
  const androidLine = overflowAndroidLog.split(/\r?\n/).find((line) =>
    line.includes(`SPINON_C114_GRID_PIXEL_MATRIX_SAMPLE `) && line.includes(`sample=${expected.id} `));
  const iosLine = overflowIosLog.split(/\r?\n/).find((line) =>
    line.includes(`SPINON_C114_METAL_PIXEL_SAMPLE `) && line.includes(`sample=${expected.id} `));
  assert.ok(androidLine, `${expected.id}: Android PixelCopy sample missing`);
  assert.ok(iosLine, `${expected.id}: iOS Metal sample missing`);
  const rgbaText = expected.rgba.join(',');
  assert.ok(androidLine.includes(`rgba=${rgbaText} expected=${rgbaText} exact=true`));
  assert.ok(iosLine.includes(`rgba=${rgbaText} expected=${rgbaText} exact=true`));
  if (expected.id === 'item-overflow-beyond-root') {
    assert.ok(androidLine.includes(`css_x=${expected.xCssPx}.0 css_y=${expected.yCssPx}.0`));
  }
  return {
    ...expected,
    beyondGridBorder: expected.id === 'item-overflow-beyond-root',
    android: { rgba: expected.rgba, exact: true },
    ios: { rgba: expected.rgba, exact: true },
  };
});

const output = {
  schemaVersion: 1,
  scope: 'C11.4 direct-leaf HTML img intrinsic contribution matrix; not full CSS Grid or image paint conformance',
  chrome: {
    userAgent: oracle.userAgent,
    devicePixelRatio: oracle.devicePixelRatio,
    observationPath: oraclePath.replace(`${repoRoot}/`, ''),
  },
  platforms: {
    android: 'Android API 37 ARM64 AVD emulator-5554, SwiftShader',
    ios: 'iPhone 17 Pro Simulator, iOS 26.2, Metal drawable before compositor',
  },
  toleranceCssPx: 0.05,
  sceneCount: comparisons.length,
  comparisons,
  overflowSurfacePixels: {
    android: 'PixelCopy',
    ios: 'Metal drawable before OS compositor',
    samples: pixelSamples,
  },
  limitations: [
    'The PNG is decoded for intrinsic metadata; these captures do not prove image pixel painting.',
    'No physical device, hardware GPU, OS compositor result, WPT runner, or full C11.4 matrix is claimed.',
  ],
};
const outputPath = resolve(evidenceDir, 'cross-platform-comparison.json');
writeFileSync(outputPath, `${JSON.stringify(output, null, 2)}\n`);
console.log(`PASS: ${comparisons.length} Chrome/Android/iOS image scenes; 3/3 overflow surface samples per simulator`);
console.log(`Wrote ${outputPath}`);
