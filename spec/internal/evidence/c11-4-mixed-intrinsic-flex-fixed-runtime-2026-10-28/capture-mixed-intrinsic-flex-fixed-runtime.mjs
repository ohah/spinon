import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { existsSync, readFileSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../../../..');
const evidenceRoot = dirname(fileURLToPath(import.meta.url));
const target = process.argv[2];
const serialOrUDID = process.argv[3];
const sceneFilter = process.argv[4];
assert.ok(['android-avd', 'ios-simulator'].includes(target),
  '사용법: node capture-mixed-intrinsic-flex-fixed-runtime.mjs android-avd <ADB serial> [scene ID] | ios-simulator <UDID> [scene ID]');
assert.ok(serialOrUDID, 'Android AVD serial 또는 iOS Simulator UDID가 필요합니다.');

const expectedJsonPath = resolve(repositoryRoot,
  'spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-precomparison-2026-10-28/expected.json');
const hiddenExpectedJsonPath = resolve(repositoryRoot,
  'spec/internal/evidence/c11-4-mixed-flex-hidden-item-precomparison-2026-10-28/expected.json');
const visibilityExpectedJsonPath = resolve(repositoryRoot,
  'spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-precomparison-2026-10-28/expected.json');
const overflowExpectedJsonPath = resolve(repositoryRoot,
  'spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-border-overflow-precomparison-2026-10-28/expected.json');
const expectedCases = JSON.parse(readFileSync(expectedJsonPath, 'utf8')).cases;
function rational(value) {
  if (value && typeof value === 'object') return value.numerator / value.denominator;
  const [numerator, denominator = '1'] = String(value).split('/');
  return Number(numerator) / Number(denominator);
}
const existingScenes = Object.entries(expectedCases).map(([id, scene]) => {
  const axis = scene.axis === 'rows' ? 'Rows' : 'Columns';
  const root = axis === 'Rows'
    ? { x: 0, y: 0, width: scene.crossAxisPx, height: scene.sizingAxisPx }
    : { x: 0, y: 0, width: scene.sizingAxisPx, height: scene.crossAxisPx };
  const itemOrder = id.endsWith('reversed') ? ['c', 'b', 'a'] : ['a', 'b', 'c'];
  const itemNodeIds = [2, 4, 5];
  const frames = [
    { node: 1, ...root },
    ...itemOrder.map((item, index) => ({
      node: itemNodeIds[index],
      ...scene.itemFramesRootRelativePx[item],
    })),
  ];
  return {
    id,
    axis,
    availableSize: scene.sizingAxisPx,
    oppositeSize: scene.crossAxisPx,
    gap: scene.gapPx,
    bases: scene.tracksPx.map(rational),
    remaining: rational(scene.remainingFreeSpacePx),
    minContentBase: scene.minContentBasePx,
    root,
    frames,
    boxCount: 4,
    drawBoxCount: 4,
  };
});
const hiddenScenes = JSON.parse(readFileSync(hiddenExpectedJsonPath, 'utf8')).scenes
  .filter(({ displayNoneChild }) => displayNoneChild)
  .map((scene) => {
    const id = scene.sceneId;
    const root = scene.rootRelativeFramesCssPx.root;
    const axis = scene.axis;
    const hiddenFrame = { x: 0, y: 0, width: 0, height: 0 };
    const frames = [
      { node: 1, ...root, semantic: 'root' },
      { node: 2, ...hiddenFrame, semantic: 'hidden-zero-frame-bookkeeping' },
      { node: 4, ...scene.rootRelativeFramesCssPx.a, semantic: 'a' },
      { node: 6, ...scene.rootRelativeFramesCssPx.b, semantic: 'b' },
      { node: 7, ...scene.rootRelativeFramesCssPx.c, semantic: 'c' },
    ];
    return {
      id,
      axis,
      availableSize: scene.trackSizing.definiteAxisCssPx,
      oppositeSize: axis === 'Rows' ? root.width : root.height,
      gap: scene.trackSizing.gapsCssPx[0],
      bases: scene.trackSizing.usedTracksCssPx,
      remaining: scene.trackSizing.remainingFreeSpaceCssPx,
      minContentBase: scene.intrinsicInputs.visibleAminContentCssPx,
      root,
      frames,
      boxCount: 4,
      frameCount: 5,
      drawBoxCount: 4,
      hiddenNode: 2,
    };
  });
const visibilityScenes = Object.entries(
  JSON.parse(readFileSync(visibilityExpectedJsonPath, 'utf8')).scenes,
).map(([id, expected]) => {
  const sceneId = {
    'column-hidden': 'column-visibility-hidden',
    'column-visible-control': 'column-visibility-visible-control',
    'row-hidden': 'row-visibility-hidden',
    'row-visible-control': 'row-visibility-visible-control',
  }[id];
  assert.ok(sceneId, `알 수 없는 visibility precomparison scene: ${id}`);
  const axis = sceneId.startsWith('row-') ? 'Rows' : 'Columns';
  const rootArray = expected.root;
  const root = {
    x: rootArray[0], y: rootArray[1], width: rootArray[2], height: rootArray[3],
  };
  const frame = (values) => ({
    x: values[0], y: values[1], width: values[2], height: values[3],
  });
  const hidden = expected.hidden_visibility === 'hidden';
  const rootId = `novel-mixed-intrinsic-flex-fixed-${sceneId}-root`;
  const spanId = `novel-mixed-intrinsic-flex-fixed-${sceneId}-visibility`;
  const sample = expected.surface_sample_root_css_px;
  const surfacePixelSamples = [
    {
      id: `${sceneId}-overlap`, nodeId: spanId,
      xCssPx: sample[0], yCssPx: sample[1],
      rgba: [...expected.surface_sample_rgb, 255],
    },
    {
      id: `${sceneId}-fixed-track-gap`, nodeId: rootId,
      xCssPx: axis === 'Rows' ? 60 : 155,
      yCssPx: axis === 'Rows' ? 155 : 60,
      rgba: [232, 237, 245, 255],
    },
  ];
  const visibleItems = Object.fromEntries(Object.entries(expected.items)
    .map(([name, values]) => [name, frame(values)]));
  return {
    id: sceneId,
    axis,
    availableSize: axis === 'Rows' ? root.height : root.width,
    oppositeSize: axis === 'Rows' ? root.width : root.height,
    gap: 10,
    bases: expected.axis_tracks_css_px,
    remaining: 0,
    minContentBase: 100,
    root,
    frames: [
      { node: 1, ...root, semantic: 'root' },
      { node: 2, ...visibleItems.a, semantic: 'a' },
      { node: 4, ...visibleItems.hidden, semantic: 'visibility-span' },
      { node: 6, ...visibleItems.b, semantic: 'image' },
      { node: 7, ...visibleItems.c, semantic: 'c' },
    ],
    boxCount: hidden ? 4 : 5,
    frameCount: 5,
    drawBoxCount: hidden ? 4 : 5,
    visibilityNode: 4,
    visibilityMinContent: 150,
    visibility: expected.hidden_visibility,
    surfacePixelSamples,
  };
});
const overflowExpected = JSON.parse(readFileSync(overflowExpectedJsonPath, 'utf8'));
const overflowSceneIds = {
  'column-hidden': 'column-visibility-hidden-border-overflow',
  'column-visible-control': 'column-visibility-visible-border-overflow-control',
  'row-hidden': 'row-visibility-hidden-border-overflow',
  'row-visible-control': 'row-visibility-visible-border-overflow-control',
};
const overflowScenes = Object.entries(overflowExpected.scenes).map(([key, expected]) => {
  const id = overflowSceneIds[key];
  assert.ok(id, `알 수 없는 border overflow precomparison scene: ${key}`);
  const axis = id.startsWith('row-') ? 'Rows' : 'Columns';
  const rootWidth = expected.rootBorderBoxCssPx[0];
  const rootHeight = expected.rootBorderBoxCssPx[1];
  const contentWidth = expected.contentBoxCssPx[0];
  const contentHeight = expected.contentBoxCssPx[1];
  const availableSize = axis === 'Rows' ? contentHeight : contentWidth;
  const oppositeSize = axis === 'Rows' ? contentWidth : contentHeight;
  const bases = axis === 'Rows'
    ? expected.gridTemplateRowsCssPx : expected.gridTemplateColumnsCssPx;
  const hidden = expected.visibility === 'hidden';
  const rootId = `novel-mixed-intrinsic-flex-fixed-${id}-root`;
  const aId = `novel-mixed-intrinsic-flex-fixed-${id}-a`;
  const spanId = `novel-mixed-intrinsic-flex-fixed-${id}-visibility`;
  const toFrame = (values) => ({
    x: values[0], y: values[1], width: values[2], height: values[3],
  });
  const borderPoint = axis === 'Rows'
    ? overflowExpected.surfaceSampleRequirements.rowBorderRootLocalCssPx
    : overflowExpected.surfaceSampleRequirements.columnBorderRootLocalCssPx;
  const overflowPoint = axis === 'Rows'
    ? overflowExpected.surfaceSampleRequirements.rowOverflowRootLocalCssPx
    : overflowExpected.surfaceSampleRequirements.columnOverflowRootLocalCssPx;
  return {
    id,
    axis,
    availableSize,
    oppositeSize,
    gap: 10,
    bases,
    remaining: availableSize - bases.reduce((sum, size) => sum + size, 0) - 20,
    minContentBase: 100,
    root: { x: 0, y: 0, width: rootWidth, height: rootHeight },
    frames: [
      { node: 1, x: 0, y: 0, width: rootWidth, height: rootHeight, semantic: 'root' },
      { node: 2, ...toFrame(expected.aAndVisibilityFrameRootLocalCssPx), semantic: 'a' },
      { node: 4, ...toFrame(expected.aAndVisibilityFrameRootLocalCssPx), semantic: 'visibility-span' },
      { node: 6, ...toFrame(expected.imageFrameRootLocalCssPx), semantic: 'image' },
      { node: 7, ...toFrame(expected.cFrameRootLocalCssPx), semantic: 'c' },
    ],
    boxCount: hidden ? 3 : 4,
    frameCount: 5,
    drawBoxCount: hidden ? 3 : 4,
    visibilityNode: 4,
    visibilityMinContent: overflowExpected.independentTrackCalculation.hiddenSpanCharacters
      * overflowExpected.independentTrackCalculation.ahemCharacterAdvanceCssPx,
    visibility: expected.visibility,
    surfacePixelSamples: [
      {
        id: `${id}-border`, nodeId: rootId,
        xCssPx: borderPoint[0], yCssPx: borderPoint[1],
        rgba: overflowExpected.colors.border.rgba,
      },
      {
        id: `${id}-overflow`, nodeId: hidden ? aId : spanId,
        xCssPx: overflowPoint[0], yCssPx: overflowPoint[1],
        rgba: hidden
          ? overflowExpected.colors.underlyingVisibleItem.rgba
          : overflowExpected.colors.visibleSpanControl.rgba,
      },
    ],
  };
});
const scenes = [...existingScenes, ...hiddenScenes, ...visibilityScenes, ...overflowScenes];
assert.ok(!sceneFilter || scenes.some(({ id }) => id === sceneFilter),
  `알 수 없는 flexible-track scene: ${sceneFilter}`);
const selectedScenes = sceneFilter ? scenes.filter(({ id }) => id === sceneFilter) : scenes;

const packageName = 'dev.spinon.bootstrap';
const bundlePath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
const apkPath = resolve(repositoryRoot,
  'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
const captureScriptPath = fileURLToPath(import.meta.url);
const timestamp = new Date().toISOString().replaceAll(/[^0-9TZ]/g, '');
const runPrefix = `c114-mixed-intrinsic-flex-fixed-${target}-${timestamp}-${randomUUID().slice(0, 8)}`;
const evidenceDirectory = resolve(evidenceRoot, 'runs', runPrefix);
const sourceInputPaths = [
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/generic_intrinsic_flex_fixed.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/generic_intrinsic_flex_fixed/hidden.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/generic_intrinsic_flex_fixed/visibility_hidden.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_intrinsic_flex_fixed.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_intrinsic_flex_fixed/hidden.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_intrinsic_flex_fixed/root_contract.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_intrinsic_flex_fixed/visibility_hidden.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_intrinsic_flex_fixed/tests.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_intrinsic_flex_fixed/tests/visibility_hidden.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_intrinsic_flex_fixed/tests/visibility_hidden/border_overflow.rs',
  'crates/spinon-layout/src/calc_tree/text.rs',
  'crates/spinon-style-to-render/src/runtime.rs',
  'crates/spinon-style-to-render/src/tests/runtime.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_nested_grid_replaced_item.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_nested_grid_rows.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_text_image.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_text_multi_image.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_grid_border_overflow.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_direct_text_image.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_direct_text_image/fixture.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_direct_text_image_row_axis.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/cyclic_minmax_mixed_text_image_span.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/generic_spanning.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/prepare.rs',
  'crates/spinon-style-to-layout/src/projection.rs',
  'crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs',
  'crates/spinon-text-layout/src/engine/intrinsic.rs',
  'crates/spinon-text-layout/src/tests.rs',
  'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
  'platforms/android/app/src/main/java/dev/spinon/bootstrap/C114FlexStage.java',
  'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
  'platforms/ios/Sources/C114FlexStage.swift',
  'platforms/ios/SpinonBootstrap.xcodeproj/project.pbxproj',
  'tools/css-reference/c11-4-mixed-intrinsic-flex-fixed-runtime/verify.py',
  'spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-precomparison-2026-10-28/expected.json',
  'spec/internal/evidence/c11-4-mixed-flex-hidden-item-precomparison-2026-10-28/expected.json',
  'spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-precomparison-2026-10-28/expected.json',
  'spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-border-overflow-precomparison-2026-10-28/expected.json',
];
const frameTolerance = 1e-4;
const traceTolerance = 1e-4;
const numericPattern = '[-+]?(?:\\d+\\.?\\d*|\\.\\d+)(?:[eE][-+]?\\d+)?';

function command(program, args, options = {}) {
  const result = spawnSync(program, args, {
    cwd: repositoryRoot,
    env: { ...process.env, ...options.env },
    encoding: Object.hasOwn(options, 'encoding') ? options.encoding : 'utf8',
    timeout: options.timeout ?? 30_000,
    maxBuffer: 32 * 1024 * 1024,
  });
  if (result.error) throw result.error;
  if (!options.allowFailure && result.status !== 0) {
    throw new Error(`${program} ${args.join(' ')} failed (${result.status}):\n${result.stderr}`);
  }
  return result;
}

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sleep = (milliseconds) => new Promise((done) => setTimeout(done, milliseconds));
const pngSignature = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);

function redactLocalPaths(value) {
  let redacted = value.replaceAll(repositoryRoot, '<REPO>');
  if (process.env.HOME) redacted = redacted.replaceAll(process.env.HOME, '<HOME>');
  return redacted;
}

function normalizeEvidenceText(value) {
  const normalized = redactLocalPaths(value)
    .replaceAll('/Users/node_modules', '<USERS>/node_modules')
    .replaceAll('\r\n', '\n')
    .split('\n')
    .map((line) => line.replace(/[ \t]+$/g, ''))
    .join('\n');
  return normalized.length === 0 ? '' : `${normalized.replace(/\n+$/g, '')}\n`;
}

function sourceRevision() {
  const commit = command('git', ['rev-parse', 'HEAD']).stdout.trim();
  const tree = command('git', ['rev-parse', 'HEAD^{tree}']).stdout.trim();
  const trackedDiff = command('git', ['diff', '--binary', 'HEAD']).stdout;
  const trackedDirtyPaths = command('git', ['diff', '--name-only', 'HEAD']).stdout
    .split(/\r?\n/).filter(Boolean);
  const sourceFileSha256 = Object.fromEntries(sourceInputPaths.map((path) => [
    path,
    sha256(readFileSync(resolve(repositoryRoot, path))),
  ]));
  return {
    commit,
    tree,
    trackedDiffSha256: sha256(trackedDiff),
    trackedDirtyPaths,
    sourceFileSha256,
    sourceFilesSha256: sha256(JSON.stringify(sourceFileSha256)),
  };
}

function assertPng(bytes, description) {
  assert.ok(Buffer.isBuffer(bytes) && bytes.subarray(0, pngSignature.length).equals(pngSignature),
    `${description}: PNG signature가 없습니다.`);
}

function pngDimensions(path) {
  const output = command('sips', ['-g', 'pixelWidth', '-g', 'pixelHeight', path]).stdout;
  const width = Number(output.match(/pixelWidth:\s*(\d+)/)?.[1]);
  const height = Number(output.match(/pixelHeight:\s*(\d+)/)?.[1]);
  assert.ok(Number.isInteger(width) && width > 0 && Number.isInteger(height) && height > 0,
    `PNG 크기를 읽을 수 없습니다: ${path}`);
  return { width, height };
}

async function buildApplication() {
  const source = sourceRevision();
  const captureScriptSha256 = sha256(readFileSync(captureScriptPath));
  const gitCommonDirectory = command('git', [
    'rev-parse', '--path-format=absolute', '--git-common-dir',
  ]).stdout.trim();
  const localV8Directory = resolve(repositoryRoot, 'build/v8-source/v8');
  const sharedV8Directory = resolve(dirname(gitCommonDirectory), 'build/v8-source/v8');
  const v8Directory = process.env.SPINON_V8_DIR
    ?? (existsSync(localV8Directory) ? localV8Directory : sharedV8Directory);
  const requiredV8Revision = readFileSync(
    resolve(repositoryRoot, 'tools/v8/v8-revision.txt'), 'utf8',
  ).trim();
  const actualV8Revision = command('git', ['-C', v8Directory, 'rev-parse', 'HEAD'])
    .stdout.trim();
  const v8Status = command('git', ['-C', v8Directory, 'status', '--porcelain=v1'])
    .stdout.trim();
  assert.equal(actualV8Revision, requiredV8Revision,
    `고정 V8 checkout revision이 다릅니다: ${actualV8Revision}`);
  assert.equal(v8Status, '', '고정 V8 checkout에 변경 파일이 있습니다.');
  const buildLogPath = resolve(evidenceDirectory,
    target === 'android-avd' ? 'android-build.log' : 'ios-build.log');
  const args = ['exec', '--', 'bun', 'run',
    target === 'android-avd' ? 'build:android' : 'build:ios-sim'];
  const environment = {
    SPINON_ENABLE_C04_RUNTIME_GPU: '1',
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST: '1',
    SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST: '1',
    SPINON_V8_DIR: v8Directory,
  };
  const result = command('mise', args, {
    allowFailure: true,
    timeout: 900_000,
    env: environment,
  });
  const buildLog = normalizeEvidenceText(`${result.stdout}${result.stderr}`);
  await writeFile(buildLogPath, buildLog);
  assert.equal(result.status, 0,
    `${target}: current-source app build 실패. 로그: ${buildLogPath}`);
  const sourceAfterBuild = sourceRevision();
  assert.equal(sourceAfterBuild.sourceFilesSha256, source.sourceFilesSha256,
    `${target}: 앱 빌드 도중 source input이 바뀌었습니다.`);
  const v8RevisionAfterBuild = command('git', ['-C', v8Directory, 'rev-parse', 'HEAD'])
    .stdout.trim();
  const v8StatusAfterBuild = command('git', ['-C', v8Directory, 'status', '--porcelain=v1'])
    .stdout.trim();
  assert.equal(v8RevisionAfterBuild, requiredV8Revision,
    '앱 빌드 뒤 고정 V8 revision이 달라졌습니다.');
  assert.equal(v8StatusAfterBuild, '', '앱 빌드가 V8 source tree를 변경했습니다.');
  assert.equal(sha256(readFileSync(captureScriptPath)), captureScriptSha256,
    `${target}: build 중 capture script가 바뀌었습니다.`);
  const artifactPath = target === 'android-avd'
    ? apkPath
    : resolve(bundlePath, 'SpinonBootstrap');
  const artifactSha256 = sha256(await readFile(artifactPath));
  return {
    command: ['mise', ...args],
    environment: {
      ...environment,
      SPINON_V8_DIR: '<shared build/v8-source/v8>',
    },
    v8: {
      checkout: '<shared build/v8-source/v8>',
      requiredRevision: requiredV8Revision,
      actualRevision: actualV8Revision,
      cleanBeforeBuild: v8Status === '',
      cleanAfterBuild: v8StatusAfterBuild === '',
    },
    source,
    captureScriptSha256,
    artifactPath: artifactPath.slice(repositoryRoot.length + 1),
    artifactSha256,
    buildLogPath: buildLogPath.slice(repositoryRoot.length + 1),
    buildLogSha256: sha256(buildLog),
  };
}

function traceParts(log, runID) {
  const marker = `SPINON_C114_GRID_TRACE_CHUNK run_id=${runID} `;
  return log.split(/\r?\n/).flatMap((line) => {
    const offset = line.indexOf(marker);
    if (offset < 0) return [];
    const match = line.slice(offset + marker.length).match(/\bpart=(\d+)\/(\d+)\s+(.*)$/);
    return match ? [{ index: Number(match[1]), total: Number(match[2]), payload: match[3] }] : [];
  });
}

function reassembleTrace(log, runID, sceneID) {
  const parts = traceParts(log, runID);
  assert.ok(parts.length > 0, `${sceneID}: trace chunk 누락`);
  const total = parts[0].total;
  assert.ok(Number.isInteger(total) && total > 0, `${sceneID}: trace 전체 chunk 수 오류`);
  assert.ok(parts.every(({ total: partTotal, index }) => (
    partTotal === total && Number.isInteger(index) && index >= 1 && index <= total
  )), `${sceneID}: trace part 번호/총수가 일치하지 않습니다.`);
  assert.equal(parts.length, total, `${sceneID}: trace chunk 중복 또는 누락`);
  const byIndex = new Map(parts.map(({ index, payload }) => [index, payload]));
  assert.equal(byIndex.size, total, `${sceneID}: 중복 trace chunk`);
  const serialized = [...Array(total)].map((_, index) => byIndex.get(index + 1)).join('');
  return {
    total,
    diagnostics: JSON.parse(serialized.replace(/^grid_track_sizing_diagnostics=/, '')),
  };
}

function assertClose(actual, expected, label, tolerance = traceTolerance) {
  assert.ok(Number.isFinite(actual) && Math.abs(actual - expected) <= tolerance,
    `${label}: expected ${expected}, got ${actual} (tolerance ${tolerance})`);
}

function expectedFrames(scene) {
  return scene.frames;
}

function parseFrameLine(line, marker) {
  const offset = line.indexOf(marker);
  if (offset < 0) return null;
  const frame = line.slice(offset + marker.length).trim();
  const expression = new RegExp(
    `^(\\d+):node=(\\d+),x=(${numericPattern}),y=(${numericPattern}),width=(${numericPattern}),height=(${numericPattern})$`);
  const match = frame.match(expression);
  assert.ok(match, `잘못된 node frame 형식: ${frame}`);
  return {
    index: Number(match[1]),
    node: Number(match[2]),
    x: Number(match[3]),
    y: Number(match[4]),
    width: Number(match[5]),
    height: Number(match[6]),
    raw: frame,
  };
}

function assertFrames(log, scene) {
  const marker = 'SPINON_C114_GRID_NODE_FRAME ';
  const actual = log.split(/\r?\n/)
    .map((line) => parseFrameLine(line, marker))
    .filter(Boolean);
  const expected = expectedFrames(scene);
  assert.equal(actual.length, expected.length, `${scene.id}: node frame 수`);
  actual.forEach((frame, index) => {
    const targetFrame = expected[index];
    assert.equal(frame.index, index, `${scene.id}: node frame 순서`);
    assert.equal(frame.node, targetFrame.node, `${scene.id}/${targetFrame.semantic}: node ID`);
    for (const property of ['x', 'y', 'width', 'height']) {
      assertClose(frame[property], targetFrame[property],
        `${scene.id}/node-${targetFrame.node}.${property}`, frameTolerance);
    }
  });
  return actual.map(({ raw }) => raw);
}

function assertFixedAxis(pass, scene) {
  assert.deepEqual(pass.stages?.map(({ stage }) => stage),
    ['Initialize', 'FixedTrackEarlyReturn'], `${scene.id}: 반대쪽 고정 축 stage 순서`);
  assert.deepEqual(pass.contributions, [], `${scene.id}: 고정 축 contribution은 비어 있어야 함`);
  for (const stage of pass.stages) {
    assert.ok(Array.isArray(stage.tracks) && stage.tracks.length === 3,
      `${scene.id}: 고정 축은 track 하나와 gutter 두 개여야 함`);
    stage.tracks.forEach((track, index) => {
      const gutter = index % 2 === 0;
      const boundary = index === 0 || index === 2;
      assert.equal(track.internalIndex, index);
      assert.equal(track.gutter, gutter);
      assert.equal(track.collapsed, boundary);
      assertClose(track.baseSize, gutter ? 0 : scene.oppositeSize,
        `${scene.id}: 고정 축 ${stage.stage} track ${index}`);
      assertClose(track.growthLimit, gutter ? 0 : scene.oppositeSize,
        `${scene.id}: 고정 축 ${stage.stage} growth ${index}`);
    });
  }
}

function assertFlexibleTracks(tracks, scene) {
  assert.ok(Array.isArray(tracks) && tracks.length === scene.bases.length * 2 + 1,
    `${scene.id}: CSS track/gutter topology가 다름`);
  let cssTrackIndex = 0;
  const actualBases = [];
  tracks.forEach((track, index) => {
    assert.equal(track.internalIndex, index, `${scene.id}: internal index ${index}`);
    assert.ok(Number.isFinite(track.baseSize), `${scene.id}: non-finite baseSize ${index}`);
    assert.equal(typeof track.gutter, 'boolean', `${scene.id}: gutter flag ${index}`);
    assert.equal(typeof track.collapsed, 'boolean', `${scene.id}: collapsed flag ${index}`);
    const gutter = index % 2 === 0;
    const boundary = index === 0 || index === tracks.length - 1;
    assert.equal(track.gutter, gutter, `${scene.id}: gutter topology ${index}`);
    assert.equal(track.collapsed, boundary, `${scene.id}: boundary sentinel ${index}`);
    if (gutter) {
      assertClose(track.baseSize, boundary ? 0 : scene.gap,
        `${scene.id}: gutter baseSize ${index}`);
    } else {
      const expected = scene.bases[cssTrackIndex++];
      assertClose(track.baseSize, expected, `${scene.id}: CSS track baseSize ${index}`);
      actualBases.push(track.baseSize);
    }
  });
  assert.equal(cssTrackIndex, scene.bases.length, `${scene.id}: CSS track 수`);
  const remaining = scene.availableSize - actualBases.reduce((sum, size) => sum + size, 0)
    - scene.gap * (scene.bases.length - 1);
  assertClose(remaining, scene.remaining, `${scene.id}: 남은 공간`);
  return { actualBases, remaining };
}

function assertTrace(diagnostics, scene) {
  const traces = diagnostics.trackSizingTraces;
  assert.ok(Array.isArray(traces) && traces.length === 1,
    `${scene.id}: Grid trace는 하나여야 합니다.`);
  const [trace] = traces;
  assert.equal(trace.grid, 1, `${scene.id}: grid node`);
  assert.ok(Array.isArray(trace.passes) && trace.passes.length === 2,
    `${scene.id}: Columns/Rows 두 pass가 필요합니다.`);
  assert.deepEqual(trace.passes.map(({ sequence }) => sequence), [0, 1],
    `${scene.id}: pass sequence가 Columns/Rows 순서와 일치해야 합니다.`);
  assert.deepEqual(trace.passes.map(({ axis }) => axis), ['Columns', 'Rows'],
    `${scene.id}: axis pass 순서`);
  const targetIndex = scene.axis === 'Rows' ? 1 : 0;
  const fixedIndex = 1 - targetIndex;
  assertFixedAxis(trace.passes[fixedIndex], scene);
  const target = trace.passes[targetIndex];
  const names = target.stages?.map(({ stage }) => stage) ?? [];
  const uniqueStageIndex = (name) => {
    const matches = names.flatMap((candidate, index) => candidate === name ? [index] : []);
    assert.equal(matches.length, 1, `${scene.id}: ${name}는 trace에 정확히 한 번 있어야 함`);
    return matches[0];
  };
  const initialize = uniqueStageIndex('Initialize');
  const maximize = uniqueStageIndex('MaximizeTracks');
  const expand = uniqueStageIndex('ExpandFlexibleTracks');
  const stretch = uniqueStageIndex('StretchAutoTracks { applied: false }');
  assert.ok(initialize < maximize && maximize < expand && expand < stretch,
    `${scene.id}: Initialize → Maximize → Expand → Stretch 순서`);
  const { actualBases, remaining } = assertFlexibleTracks(
    target.stages[expand].tracks, scene);
  assertVisibilityContributions(target, scene);
  assertIntrinsicInputs(diagnostics, scene);
  return { actualBases, remaining };
}

function assertVisibilityContributions(pass, scene) {
  if (scene.visibilityNode === undefined) return;
  const contributions = pass.contributions;
  assert.ok(Array.isArray(contributions) && contributions.length === 4,
    `${scene.id}: A와 visibility 항목의 축 contribution은 각각 Minimum/MinContent여야 함`);
  const automaticMinimum = 'AutomaticMinimum { spans_auto_min_track: false, spans_multiple_tracks: false, spans_flexible_max_track: true, content_based: false, is_compressible_replaced: false, fixed_track_sum: None, fixed_gutter_sum: None, fixed_track_limit: None }';
  const normalizeDebugValue = (value) => typeof value === 'string'
    ? value.replaceAll(/\s+/g, '') : value;
  const normalized = contributions.map(({ item, kind, span, crossesFlexibleTrack, value,
    minimumSource }) => ({ item, kind, span, crossesFlexibleTrack, value,
    minimumSource: normalizeDebugValue(minimumSource) }))
    .sort((left, right) => left.item - right.item
      || (left.kind === 'Minimum' ? 0 : 1) - (right.kind === 'Minimum' ? 0 : 1));
  assert.deepEqual(normalized, [
    { item: 2, kind: 'Minimum', span: 1, crossesFlexibleTrack: true,
      value: 0, minimumSource: normalizeDebugValue(automaticMinimum) },
    { item: 2, kind: 'MinContent', span: 1, crossesFlexibleTrack: true,
      value: 100, minimumSource: null },
    { item: 4, kind: 'Minimum', span: 1, crossesFlexibleTrack: true,
      value: 0, minimumSource: normalizeDebugValue(automaticMinimum) },
    { item: 4, kind: 'MinContent', span: 1, crossesFlexibleTrack: true,
      value: scene.visibilityMinContent, minimumSource: null },
  ], `${scene.id}: hidden visibility도 A 다음 source order로 정확히 track sizing에 참여해야 함`);
}

function assertIntrinsicInputs(diagnostics, scene) {
  const inputs = diagnostics.gridItemIntrinsicInputs;
  const visibility = scene.visibilityNode !== undefined;
  const hidden = scene.hiddenNode !== undefined;
  const expectedLength = visibility ? 4 : 3;
  assert.ok(Array.isArray(inputs) && inputs.length === expectedLength,
    `${scene.id}: direct grid intrinsic input count`);
  const itemOrder = visibility
    ? ['a', 'visibility', 'b', 'c']
    : hidden
    ? ['a', 'b', 'c']
    : (scene.id.endsWith('reversed') ? ['c', 'b', 'a'] : ['a', 'b', 'c']);
  const nodeIds = visibility ? [2, 4, 6, 7] : hidden ? [4, 6, 7] : [2, 4, 5];
  for (const [index, input] of inputs.entries()) {
    const item = itemOrder[index];
    assert.equal(input.sourceOrder, hidden ? index + 1 : index,
      `${scene.id}: source order ${index}`);
    assert.equal(input.node, nodeIds[index], `${scene.id}: intrinsic input node ${index}`);
    assert.notEqual(input.node, scene.hiddenNode,
      `${scene.id}: display:none text must not enter intrinsic inputs`);
    if (item === 'b') {
      assert.equal(input.source?.kind, 'replaced', `${scene.id}: image input kind`);
      assertClose(input.source?.natural?.width, 24, `${scene.id}: PNG natural width`);
      assertClose(input.source?.natural?.height, 16, `${scene.id}: PNG natural height`);
      continue;
    }
    assert.equal(input.source?.kind, scene.axis === 'Rows' ? 'orthogonalText' : 'text',
      `${scene.id}/${item}: text input kind`);
    const physicalAxis = scene.axis === 'Rows' ? 'height' : 'width';
    const expectedMinContent = item === 'a' ? scene.minContentBase
      : item === 'visibility' ? scene.visibilityMinContent : 40;
    assertClose(input.contributionInput?.[physicalAxis]?.minContent, expectedMinContent,
      `${scene.id}/${item}: min-content contribution`);
  }
}

function successfulDraw(log, boxCount) {
  return target === 'android-avd'
    ? log.includes(`SPINON_C0410_DRAW status=0 presented boxes=${boxCount}`)
    : new RegExp(`SPINON_C0410_DRAW generation=\\d+ status=0 presented boxes=${boxCount}`)
      .test(log);
}

function verifyScene(log, runID, scene) {
  assert.ok(!log.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`),
    `${scene.id}: native scene assertion 실패`);
  assert.ok(!log.includes('SPINON_C114_GRID_ARGUMENT_ERROR'),
    `${scene.id}: scene argument 거부`);
  assert.ok(!log.includes(`SPINON_C114_GRID_TRACE_MISSING run_id=${runID}`),
    `${scene.id}: trace 누락`);
  const summary = `SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `;
  const summaryOffset = log.indexOf(summary);
  assert.ok(summaryOffset >= 0, `${scene.id}: exact runtime summary marker 누락`);
  const boxCount = scene.boxCount;
  const frameCount = scene.frameCount ?? boxCount;
  const summaryLine = log.slice(summaryOffset).split(/\r?\n/, 1)[0];
  assert.ok(summaryLine.includes(
    `expected_root_frame=width=${scene.root.width},height=${scene.root.height}`),
    `${scene.id}: root expected frame summary 불일치`);
  assert.ok(summaryLine.includes(`status=0 layout=ready boxes=${boxCount}`),
    `${scene.id}: ready/box summary 불일치`);
  const drawBoxCount = scene.drawBoxCount ?? boxCount;
  assert.ok(successfulDraw(log.slice(summaryOffset), drawBoxCount),
    `${scene.id}: presented frame marker 누락`);

  const nodeFrames = assertFrames(log, scene);
  if (target === 'ios-simulator') {
    assert.ok(log.includes(
    `SPINON_C114_GRID_FRAME_SUMMARY run_id=${runID} count=${frameCount} marker=present`),
    `${scene.id}: iOS frame summary marker 누락`);
  }
  const { total: traceChunks, diagnostics } = reassembleTrace(log, runID, scene.id);
  const { actualBases, remaining } = assertTrace(diagnostics, scene);
  let pixelSamples;
  if (scene.surfacePixelSamples) {
    const marker = target === 'android-avd'
      ? 'SPINON_C114_GRID_PIXEL_MATRIX' : 'SPINON_C114_METAL_PIXEL';
    const summaryLine = log.split(/\r?\n/).find((line) => (
      line.includes(`${marker}_SUMMARY`) && line.includes(`run_id=${runID}`)
    ));
    assert.ok(summaryLine?.includes('status=matched'),
      `${scene.id}: ${marker} 정확한 표면 표본 summary 누락 또는 불일치`);
    pixelSamples = scene.surfacePixelSamples.map((expected) => {
      const sampleLine = log.split(/\r?\n/).find((line) => (
        line.includes(`${marker}_SAMPLE`) && line.includes(`run_id=${runID}`)
        && line.includes(`sample=${expected.id}`)
      ));
      assert.ok(sampleLine, `${scene.id}/${expected.id}: 표면 표본 로그 누락`);
      assert.ok(sampleLine.includes(`rgba=${expected.rgba.join(',')}`),
        `${scene.id}/${expected.id}: 실제 RGBA가 기대와 다름`);
      if (target === 'android-avd') {
        assert.ok(sampleLine.includes(`expected=${expected.rgba[0]},${expected.rgba[1]},${expected.rgba[2]},255 exact=true`),
          `${scene.id}/${expected.id}: PixelCopy 표본 expected/exact 불일치`);
        assert.ok(sampleLine.includes(`target_node_id=${expected.nodeId}`),
          `${scene.id}/${expected.id}: PixelCopy 표본 owner NodeId 불일치`);
      } else {
        assert.ok(sampleLine.includes(`expected=${expected.rgba.join(',')} exact=true`),
          `${scene.id}/${expected.id}: Metal 표본 expected/exact 불일치`);
      }
      return { id: expected.id, nodeId: expected.nodeId, rgba: expected.rgba };
    });
  }
  return {
    sceneId: scene.id,
    runID,
    status: 'passed',
    boxCount,
    frameCount,
    drawBoxCount,
    targetAxis: scene.axis,
    trackBasesCssPx: actualBases,
    remainingSpaceCssPx: remaining,
    nodeFrames,
    traceChunks,
    presented: true,
    ...(pixelSamples ? { pixelSamples } : {}),
  };
}

function verifyRejectedRoute(log, label) {
  assert.ok(log.includes('SPINON_C114_GRID_ARGUMENT_ERROR'),
    `${label}: C11.4 argument error marker가 없습니다.`);
  assert.ok(!log.includes('SPINON_C114_GRID_SUMMARY'),
    `${label}: 거부된 route가 정상 C11.4 scene까지 진행했습니다.`);
}

async function runAndroid() {
  assert.ok(/^emulator-\d+$/.test(serialOrUDID),
    '실기기를 대상으로 실행하지 않습니다. emulator- serial만 허용합니다.');
  const adb = (args, options = {}) => command('adb', ['-s', serialOrUDID, ...args], options);
  try {
    const bootCompleted = adb(['shell', 'getprop', 'sys.boot_completed']).stdout.trim();
    assert.equal(bootCompleted, '1', `Android AVD ${serialOrUDID}가 boot 상태가 아닙니다.`);
    const device = {
      serial: serialOrUDID,
      avdName: adb(['shell', 'getprop', 'ro.boot.qemu.avd_name']).stdout.trim(),
      apiLevel: Number(adb(['shell', 'getprop', 'ro.build.version.sdk']).stdout.trim()),
      abi: adb(['shell', 'getprop', 'ro.product.cpu.abi']).stdout.trim(),
      model: adb(['shell', 'getprop', 'ro.product.model']).stdout.trim(),
      pageSizeBytes: Number(adb(['shell', 'getconf', 'PAGE_SIZE']).stdout.trim()),
      qemu: adb(['shell', 'getprop', 'ro.kernel.qemu']).stdout.trim(),
      screenSize: adb(['shell', 'wm', 'size']).stdout.trim(),
      fingerprint: adb(['shell', 'getprop', 'ro.build.fingerprint']).stdout.trim(),
    };
    assert.equal(device.apiLevel, 37, 'Android API 37 AVD가 필요합니다.');
    assert.equal(device.abi, 'arm64-v8a', 'Android ARM64 AVD가 필요합니다.');
    assert.equal(device.pageSizeBytes, 16_384, 'Android 16 KB AVD가 필요합니다.');
    assert.equal(device.qemu, '1', '실기기가 아닌 Android AVD가 필요합니다.');
    const build = await buildApplication();
    const apkSha256 = sha256(await readFile(apkPath));
    assert.equal(apkSha256, build.artifactSha256, 'APK와 build provenance SHA-256이 다릅니다.');
    const install = adb(['install', '-r', apkPath]);
    await writeFile(resolve(evidenceDirectory, 'android-install.log'),
      normalizeEvidenceText(`${install.stdout}${install.stderr}`));
    const installedBaseApk = adb(['shell', 'pm', 'path', packageName]).stdout
      .split(/\r?\n/).map((line) => line.replace(/^package:/, '').trim())
      .find((path) => path.endsWith('/base.apk'));
    assert.ok(installedBaseApk, '설치된 base.apk 경로 없음');
    const installedApkSha256 = adb(['shell', 'sha256sum', installedBaseApk]).stdout
      .trim().split(/\s+/)[0];
    assert.equal(installedApkSha256, apkSha256, '설치된 APK hash 불일치');

    const rejectedRoutes = [];
    for (const rejected of [
      { label: 'unknown-scene', scene: 'column-freeze-unlisted', selector: [] },
      { label: 'visible-control', scene: 'column-visible-control', selector: [] },
      { label: 'conflicting-c1121-profile', scene: 'column-freeze',
        selector: ['--ez', 'spinon_c1121_grid_placement', 'true'] },
    ]) {
      const runID = `${runPrefix}-rejected-${rejected.label}`;
      const launch = adb(['shell', 'am', 'start', '-S', '-f', '0x10008000', '-n',
        `${packageName}/.MainActivity`, '--ez', 'spinon_c114_grid_track_sizing', 'true',
        '--es', 'spinon_c114_grid_scene', rejected.scene,
        '--es', 'spinon_c114_grid_run_id', runID, ...rejected.selector]);
      const launchLog = normalizeEvidenceText(`${launch.stdout}${launch.stderr}`);
      const launchPath = resolve(evidenceDirectory,
        `android-rejected-${rejected.label}-launch.log`);
      await writeFile(launchPath, launchLog);
      const deadline = Date.now() + 30_000;
      let runtimeLog = '';
      let pid = null;
      while (Date.now() < deadline) {
        pid = Number(adb(['shell', 'pidof', packageName], { allowFailure: true })
          .stdout.trim().split(/\s+/)[0]) || null;
        if (pid) {
          runtimeLog = adb(['logcat', '-d', '-T', '1000', '-v', 'threadtime',
            `--pid=${pid}`, 'SpinonBootstrap:V', '*:S'], { timeout: 30_000 }).stdout;
        }
        if (runtimeLog.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) break;
        await sleep(300);
      }
      runtimeLog = normalizeEvidenceText(runtimeLog);
      const runtimePath = resolve(evidenceDirectory,
        `android-rejected-${rejected.label}-runtime.log`);
      await writeFile(runtimePath, runtimeLog);
      verifyRejectedRoute(runtimeLog, `Android ${rejected.label}`);
      rejectedRoutes.push({
        label: rejected.label,
        sceneId: rejected.scene,
        additionalSelectorArguments: rejected.selector,
        runID,
        pid,
        launchLogSha256: sha256(await readFile(launchPath)),
        runtimeLogSha256: sha256(await readFile(runtimePath)),
        rejected: true,
      });
    }

    const observations = [];
    for (const [index, scene] of selectedScenes.entries()) {
      const runID = `${runPrefix}-${index}`;
      assert.ok(runID.length <= 80, `${scene.id}: PixelCopy run ID가 80자를 초과합니다.`);
      const pixelMatrixArguments = scene.surfacePixelSamples ? [
        '--es', 'spinon_c11_grid_pixel_matrix_run_id', runID,
        '--es', 'spinon_c11_grid_pixel_matrix_samples',
        Buffer.from(JSON.stringify(scene.surfacePixelSamples.map((sample) => ({
          id: sample.id,
          nodeId: sample.nodeId,
          xCssPx: sample.xCssPx,
          yCssPx: sample.yCssPx,
          rgba: sample.rgba,
        })))).toString('base64url'),
      ] : [];
      const launch = adb(['shell', 'am', 'start', '-S', '-f', '0x10008000', '-n',
        `${packageName}/.MainActivity`, '--ez', 'spinon_c114_grid_track_sizing', 'true',
        '--es', 'spinon_c114_grid_scene', scene.id,
        '--es', 'spinon_c114_grid_run_id', runID, ...pixelMatrixArguments]);
      const launchLog = normalizeEvidenceText(`${launch.stdout}${launch.stderr}`);
      const launchPath = resolve(evidenceDirectory, `android-launch-${scene.id}.log`);
      await writeFile(launchPath, launchLog);
      const deadline = Date.now() + 90_000;
      let runtimeLog = '';
      let pid = null;
      while (Date.now() < deadline) {
        pid = Number(adb(['shell', 'pidof', packageName], { allowFailure: true })
          .stdout.trim().split(/\s+/)[0]) || null;
        if (pid) {
          runtimeLog = adb(['logcat', '-d', '-T', '1000', '-v', 'threadtime',
            `--pid=${pid}`, 'SpinonBootstrap:V', '*:S'], { timeout: 30_000 }).stdout;
        }
        const summarySeen = runtimeLog.includes(
          `SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `);
        const drawSeen = successfulDraw(runtimeLog, scene.drawBoxCount ?? scene.boxCount);
        const pixelSeen = !scene.surfacePixelSamples || runtimeLog.includes(
          `SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=${runID}`);
        if (runtimeLog.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
            || runtimeLog.includes('SPINON_C114_GRID_ARGUMENT_ERROR')
            || (summarySeen && drawSeen && pixelSeen)) break;
        await sleep(300);
      }
      runtimeLog = normalizeEvidenceText(runtimeLog);
      const runtimeLogPath = resolve(evidenceDirectory, `android-runtime-${scene.id}.log`);
      await writeFile(runtimeLogPath, runtimeLog);
      const observation = verifyScene(runtimeLog, runID, scene);
      observation.pid = pid;
      observation.launchLogSha256 = sha256(await readFile(launchPath));
      observation.runtimeLogSha256 = sha256(await readFile(runtimeLogPath));
      const activityState = adb(['shell', 'dumpsys', 'activity', 'activities']).stdout;
      const topResumedActivity = activityState.split(/\r?\n/)
        .find((line) => line.includes('topResumedActivity='));
      assert.ok(topResumedActivity?.includes(`${packageName}/`),
        `${scene.id}: screenshot 시점 foreground가 Spinon이 아닙니다.`);
      observation.foregroundActivity = topResumedActivity.trim();
      await sleep(500);
      const screenshot = adb(['exec-out', 'screencap', '-p'], { encoding: null }).stdout;
      assertPng(screenshot, `Android screenshot ${scene.id}`);
      const screenshotPath = resolve(evidenceDirectory, `android-${scene.id}.png`);
      await writeFile(screenshotPath, screenshot);
      observation.screenshot = {
        path: screenshotPath.slice(repositoryRoot.length + 1),
        dimensions: pngDimensions(screenshotPath),
        sha256: sha256(screenshot),
      };
      if (scene.surfacePixelSamples) {
        const pixelCopy = adb(['exec-out', 'run-as', packageName, 'cat',
          `cache/c114-grid-surface-pixel-matrix/${scene.id}-${runID}.png`],
        { encoding: null });
        assertPng(pixelCopy.stdout, `Android PixelCopy 표면 캡처 ${scene.id}`);
        const pixelCopyPath = resolve(evidenceDirectory, `android-pixelcopy-${scene.id}.png`);
        await writeFile(pixelCopyPath, pixelCopy.stdout);
        observation.pixelCopy = {
          path: pixelCopyPath.slice(repositoryRoot.length + 1),
          dimensions: pngDimensions(pixelCopyPath),
          sha256: sha256(pixelCopy.stdout),
        };
      }
      observations.push(observation);
      console.log(`Android API 37 ARM64 AVD ${scene.id}: PASS`);
    }
    const finalSource = sourceRevision();
    assert.equal(finalSource.sourceFilesSha256, build.source.sourceFilesSha256,
      'Android capture source가 build source와 다릅니다.');
    assert.equal(sha256(readFileSync(captureScriptPath)), build.captureScriptSha256,
      'Android capture script가 실행 중 바뀌었습니다.');
    const cleanup = adb(['shell', 'am', 'force-stop', packageName], { allowFailure: true });
    await writeFile(resolve(evidenceDirectory, 'android-cleanup.log'),
      normalizeEvidenceText(`${cleanup.stdout}${cleanup.stderr}`));
    const manifest = {
      complete: true,
      target,
      runPrefix,
      generatedAtUtc: new Date().toISOString(),
      device,
      apkSha256,
      installedApkSha256,
      build,
      sourceRevisionAfter: finalSource,
      rejectedRoutes,
      observations,
      cleanup: { action: 'force-stop Spinon only', avdRemainsBooted: true },
    };
    await writeFile(resolve(evidenceDirectory, 'android-simulator.json'),
      `${JSON.stringify(manifest, null, 2)}\n`);
  } finally {
    let cleanup = { stdout: '', stderr: '' };
    try {
      cleanup = adb(['shell', 'am', 'force-stop', packageName], {
        allowFailure: true,
        timeout: 5_000,
      });
    } catch (error) {
      cleanup.stderr = `Spinon force-stop command failed: ${error}`;
    }
    await writeFile(resolve(evidenceDirectory, 'android-cleanup-final.log'),
      normalizeEvidenceText(`${cleanup.stdout}${cleanup.stderr}`)).catch(() => {});
  }
}

async function runIOS() {
  const devicesBefore = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json']).stdout)
    .devices;
  const initial = Object.entries(devicesBefore).flatMap(([runtime, values]) => values
    .filter(({ udid }) => udid === serialOrUDID)
    .map((device) => ({ ...device, runtime })))[0];
  assert.ok(initial, `Simulator UDID를 찾지 못했습니다: ${serialOrUDID}`);
  let bootedByThisRun = false;
  try {
    if (initial.state !== 'Booted') {
      command('xcrun', ['simctl', 'boot', serialOrUDID]);
      bootedByThisRun = true;
    }
    command('xcrun', ['simctl', 'bootstatus', serialOrUDID, '-b'], { timeout: 180_000 });
    const devicesAfterBoot = JSON.parse(command('xcrun', ['simctl', 'list', 'devices', '--json'])
      .stdout).devices;
    const simulator = Object.entries(devicesAfterBoot).flatMap(([runtime, values]) => values
      .filter(({ udid, state }) => udid === serialOrUDID && state === 'Booted')
      .map((device) => ({ ...device, runtime })));
    assert.equal(simulator.length, 1, '대상 iOS Simulator가 정확히 하나 booted여야 합니다.');
    assert.equal(simulator[0].name, 'iPhone 17 Pro', 'iPhone 17 Pro Simulator가 필요합니다.');
    assert.equal(simulator[0].deviceTypeIdentifier,
      'com.apple.CoreSimulator.SimDeviceType.iPhone-17-Pro');
    assert.match(simulator[0].runtime, /iOS-26-2$/, 'iOS 26.2 Simulator가 필요합니다.');
    const modelIdentifier = command('xcrun', [
      'simctl', 'getenv', serialOrUDID, 'SIMULATOR_MODEL_IDENTIFIER',
    ]).stdout.trim();
    const build = await buildApplication();
    const appSha256 = sha256(await readFile(resolve(bundlePath, 'SpinonBootstrap')));
    assert.equal(appSha256, build.artifactSha256, 'iOS app와 build provenance SHA-256 불일치');
    const install = command('xcrun', ['simctl', 'install', serialOrUDID, bundlePath]);
    await writeFile(resolve(evidenceDirectory, 'ios-install.log'),
      normalizeEvidenceText(`${install.stdout}${install.stderr}`));
    const rejectedRoutes = [];
    for (const rejected of [
      { label: 'unknown-scene', scene: 'column-freeze-unlisted', selector: [] },
      { label: 'visible-control', scene: 'column-visible-control', selector: [] },
      { label: 'conflicting-c1121-profile', scene: 'column-freeze',
        selector: ['--spinon-c1121-grid-placement'] },
    ]) {
      const runID = `${runPrefix}-rejected-${rejected.label}`;
      const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process',
        serialOrUDID, packageName, '--spinon-c114-grid-track-sizing',
        `--spinon-c114-grid-run-id=${runID}`,
        `--spinon-c114-grid-scene=${rejected.scene}`, ...rejected.selector]);
      const launchLog = normalizeEvidenceText(`${launch.stdout}${launch.stderr}`);
      const launchPath = resolve(evidenceDirectory,
        `ios-rejected-${rejected.label}-launch.log`);
      await writeFile(launchPath, launchLog);
      const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
      assert.ok(Number.isInteger(pid) && pid > 0,
        `iOS ${rejected.label}: launch PID 없음`);
      const predicate = 'process == "SpinonBootstrap" AND eventMessage CONTAINS[c] '
        + '"SPINON_C114_GRID"';
      const deadline = Date.now() + 60_000;
      let runtimeLog = '';
      while (Date.now() < deadline) {
        runtimeLog = command('xcrun', ['simctl', 'spawn', serialOrUDID, 'log', 'show',
          '--last', '90s', '--style', 'syslog', '--predicate', predicate],
        { timeout: 45_000 }).stdout;
        const processMarker = new RegExp(`SpinonBootstrap\\[${pid}(?::|\\])`);
        runtimeLog = runtimeLog.split(/\r?\n/)
          .filter((line) => processMarker.test(line)).join('\n');
        if (runtimeLog.includes('SPINON_C114_GRID_ARGUMENT_ERROR')) break;
        await sleep(500);
      }
      runtimeLog = normalizeEvidenceText(runtimeLog);
      const runtimePath = resolve(evidenceDirectory,
        `ios-rejected-${rejected.label}-runtime.log`);
      await writeFile(runtimePath, runtimeLog);
      verifyRejectedRoute(runtimeLog, `iOS ${rejected.label}`);
      rejectedRoutes.push({
        label: rejected.label,
        sceneId: rejected.scene,
        additionalSelectorArguments: rejected.selector,
        runID,
        pid,
        launchLogSha256: sha256(await readFile(launchPath)),
        runtimeLogSha256: sha256(await readFile(runtimePath)),
        rejected: true,
      });
    }
    const observations = [];
    for (const [index, scene] of selectedScenes.entries()) {
      const runID = `${runPrefix}-${index}`;
      assert.ok(runID.length <= 80, `${scene.id}: Metal surface run ID가 80자를 초과합니다.`);
      const pixelMatrixArguments = scene.surfacePixelSamples ? [
        `--spinon-c11-grid-metal-surface-pixels=${runID}`,
        `--spinon-c11-grid-metal-surface-samples=${Buffer.from(JSON.stringify(
          scene.surfacePixelSamples.map((sample) => ({
            id: sample.id,
            nodeId: sample.nodeId,
            xCssPx: sample.xCssPx,
            yCssPx: sample.yCssPx,
            rgba: sample.rgba,
          })),
        )).toString('base64')}`,
      ] : [];
      const launch = command('xcrun', ['simctl', 'launch', '--terminate-running-process',
        serialOrUDID, packageName, '--spinon-c114-grid-track-sizing',
        `--spinon-c114-grid-run-id=${runID}`, `--spinon-c114-grid-scene=${scene.id}`,
        ...pixelMatrixArguments]);
      const launchLog = normalizeEvidenceText(`${launch.stdout}${launch.stderr}`);
      const launchPath = resolve(evidenceDirectory, `ios-launch-${scene.id}.log`);
      await writeFile(launchPath, launchLog);
      const pid = Number(launch.stdout.match(/(?:^|:)\s+(\d+)\s*$/m)?.[1]);
      assert.ok(Number.isInteger(pid) && pid > 0, `${scene.id}: iOS launch PID 없음`);
      const predicate = 'process == "SpinonBootstrap" AND (eventMessage CONTAINS[c] "SPINON_C114_GRID" '
        + 'OR eventMessage CONTAINS[c] "SPINON_C0410_DRAW" '
        + 'OR eventMessage CONTAINS[c] "SPINON_C114_GRID_NODE_FRAME" '
        + 'OR eventMessage CONTAINS[c] "SPINON_C114_METAL_PIXEL")';
      const deadline = Date.now() + 150_000;
      let runtimeLog = '';
      while (Date.now() < deadline) {
        runtimeLog = command('xcrun', ['simctl', 'spawn', serialOrUDID, 'log', 'show', '--last',
          '90s', '--style', 'syslog', '--predicate', predicate], { timeout: 45_000 }).stdout;
        const processMarker = new RegExp(`SpinonBootstrap\\[${pid}(?::|\\])`);
        runtimeLog = runtimeLog.split(/\r?\n/).filter((line) => processMarker.test(line)).join('\n');
        const summarySeen = runtimeLog.includes(
          `SPINON_C114_GRID_SUMMARY run_id=${runID} scene_id=${scene.id} `);
        const drawSeen = successfulDraw(runtimeLog, scene.drawBoxCount ?? scene.boxCount);
        const pixelSeen = !scene.surfacePixelSamples || runtimeLog.includes(
          `SPINON_C114_METAL_PIXEL_SUMMARY status=matched run_id=${runID}`);
        if (runtimeLog.includes(`SPINON_C114_GRID_ASSERTION_ERROR run_id=${runID}`)
            || runtimeLog.includes('SPINON_C114_GRID_ARGUMENT_ERROR')
            || (summarySeen && drawSeen && pixelSeen)) break;
        await sleep(500);
      }
      runtimeLog = normalizeEvidenceText(runtimeLog);
      const runtimeLogPath = resolve(evidenceDirectory, `ios-runtime-${scene.id}.log`);
      await writeFile(runtimeLogPath, runtimeLog);
      const observation = verifyScene(runtimeLog, runID, scene);
      observation.pid = pid;
      observation.launchLogSha256 = sha256(await readFile(launchPath));
      observation.runtimeLogSha256 = sha256(await readFile(runtimeLogPath));
      await sleep(500);
      const screenshotPath = resolve(evidenceDirectory, `ios-${scene.id}.png`);
      const screenshotCommand = command('xcrun', [
        'simctl', 'io', serialOrUDID, 'screenshot', screenshotPath,
      ]);
      await writeFile(resolve(evidenceDirectory, `ios-screenshot-${scene.id}.log`),
        normalizeEvidenceText(`${screenshotCommand.stdout}${screenshotCommand.stderr}`));
      const screenshot = await readFile(screenshotPath);
      assertPng(screenshot, `iOS screenshot ${scene.id}`);
      observation.screenshot = {
        path: screenshotPath.slice(repositoryRoot.length + 1),
        dimensions: pngDimensions(screenshotPath),
        sha256: sha256(screenshot),
      };
      observations.push(observation);
      console.log(`iPhone 17 Pro / iOS 26.2 Simulator ${scene.id}: PASS`);
    }
    const finalSource = sourceRevision();
    assert.equal(finalSource.sourceFilesSha256, build.source.sourceFilesSha256,
      'iOS capture source가 build source와 다릅니다.');
    assert.equal(sha256(readFileSync(captureScriptPath)), build.captureScriptSha256,
      'iOS capture script가 실행 중 바뀌었습니다.');
    const manifest = {
      complete: true,
      target,
      runPrefix,
      generatedAtUtc: new Date().toISOString(),
      udid: serialOrUDID,
      name: simulator[0].name,
      runtime: simulator[0].runtime,
      deviceTypeIdentifier: simulator[0].deviceTypeIdentifier,
      modelIdentifier,
      bootedBeforeRun: initial.state === 'Booted',
      bootedByThisRun,
      appSha256,
      build,
      sourceRevisionAfter: finalSource,
      rejectedRoutes,
      observations,
    };
    await writeFile(resolve(evidenceDirectory, 'ios-simulator.json'),
      `${JSON.stringify(manifest, null, 2)}\n`);
  } finally {
    const terminate = command('xcrun', [
      'simctl', 'terminate', serialOrUDID, packageName,
    ], { allowFailure: true });
    await writeFile(resolve(evidenceDirectory, 'ios-cleanup.log'),
      normalizeEvidenceText(`${terminate.stdout}${terminate.stderr}`)).catch(() => {});
    if (bootedByThisRun) {
      const shutdown = command('xcrun', ['simctl', 'shutdown', serialOrUDID], {
        allowFailure: true,
      });
      await writeFile(resolve(evidenceDirectory, 'ios-shutdown.log'),
        normalizeEvidenceText(`${shutdown.stdout}${shutdown.stderr}`)).catch(() => {});
    }
  }
}

await mkdir(evidenceDirectory, { recursive: true });
try {
  if (target === 'android-avd') await runAndroid();
  else await runIOS();
} catch (error) {
  await writeFile(resolve(evidenceDirectory, 'failure.json'), `${JSON.stringify({
    complete: false,
    target,
    runPrefix,
    generatedAtUtc: new Date().toISOString(),
    selectedScenes: selectedScenes.map(({ id }) => id),
    error: String(error?.stack ?? error),
  }, null, 2)}\n`).catch(() => {});
  throw error;
}
console.log(`Capture evidence: ${evidenceDirectory.replaceAll(repositoryRoot, '<REPO>')}`);
