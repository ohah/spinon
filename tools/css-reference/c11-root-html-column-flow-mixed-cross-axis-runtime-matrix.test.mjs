import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import test from 'node:test';
import { androidDensityFromWmOutput } from './compare-c11-android-surface-pixel-matrix.mjs';

const repositoryRoot = resolve(new URL('../..', import.meta.url).pathname);
const matrixPath = 'tests/fixtures/css/c11/root-html-grid-column-flow-mixed-cross-axis-runtime-matrix-v1.json';
const matrix = JSON.parse(readFileSync(resolve(repositoryRoot, matrixPath), 'utf8'));

test('runtime matrix artifact is reproducible from pinned Chrome and fixture inputs', () => {
  const output = execFileSync(process.execPath, [
    'tools/css-reference/build-c11-root-html-column-flow-mixed-cross-axis-runtime-matrix.mjs',
    '--check',
  ], { cwd: repositoryRoot, encoding: 'utf8' });
  assert.match(output, /9 scenes · 8 success · 1 expected reject · 31 items · 31 pixel samples · verified/);
});

test('runtime matrix pins all eight positives and one implicit-boundary rejection', () => {
  assert.equal(matrix.schema,
    'spinon-css-c11-root-html-column-flow-mixed-cross-axis-runtime-matrix/v1');
  assert.deepEqual(matrix.summary, {
    sceneCount: 9,
    successfulScenes: 8,
    expectedRejectScenes: 1,
    participatingItems: 31,
    pixelSampleCount: 31,
    geometryToleranceCssPx: 0,
  });
  assert.deepEqual(matrix.scenes.map(({ id }) => id), [
    'dom-cross-axis', 'mixed-spans', 'hidden-controls',
    'hidden-row-lock-qualification', 'hidden-column-lock-qualification',
    'display-none-control', 'implicit-boundary-control', 'reverse-order',
    'negative-line-control',
  ]);
  assert.equal(matrix.scenes.filter(({ outcome }) => outcome === 'success').length, 8);
  assert.equal(matrix.scenes.reduce((count, scene) => count + scene.pixelSamples.length, 0), 31);
});

test('both platform rejection adapters pin code, property, and runtime node identity', () => {
  const rejection = matrix.scenes.find(({ outcome }) => outcome === 'expected-rejection');
  assert.equal(rejection.id, 'implicit-boundary-control');
  assert.deepEqual(rejection.expectedDiagnostic, {
    code: 'invalid_grid_placement',
    property: 'grid-row-start',
    nodeId: 16,
  });

  const android = readFileSync(resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'), 'utf8');
  const ios = readFileSync(resolve(repositoryRoot,
    'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8');
  const runner = readFileSync(resolve(repositoryRoot,
    'tools/css-reference/capture-c11-root-html-column-flow-mixed-cross-axis-runtime-matrix.mjs'), 'utf8');
  const denseSceneSet = ios.match(
    /private let c111RootHtmlDensePlacementSceneIDs: Set<String> = \[([\s\S]*?)\n    \]/,
  )?.[1];
  const columnFlowSceneSet = ios.match(
    /private let c111RootHtmlColumnFlowMixedCrossAxisSceneIDs: Set<String> = \[([\s\S]*?)\n    \]/,
  )?.[1];
  assert.ok(android.includes('"invalid_grid_placement|grid-row-start|16"'));
  assert.ok(ios.includes('("invalid_grid_placement", "grid-row-start", 16)'));
  assert.ok(runner.includes('assert.equal(nodeId, diagnostic.nodeId'));
  assert.ok(denseSceneSet && columnFlowSceneSet, 'iOS C11.1 scene allowlists are missing');
  assert.doesNotMatch(denseSceneSet, /dom-cross-axis|mixed-spans|implicit-boundary-control/);
  assert.match(columnFlowSceneSet, /dom-cross-axis/);
  assert.match(columnFlowSceneSet, /implicit-boundary-control/);
});

test('Android PixelCopy and iOS Metal CFM routes use device density for viewport samples', () => {
  const androidCapture = readFileSync(resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'), 'utf8');
  const androidComparator = readFileSync(resolve(repositoryRoot,
    'tools/css-reference/compare-c11-android-surface-pixel-matrix.mjs'), 'utf8');
  const iosCapture = readFileSync(resolve(repositoryRoot,
    'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8');
  const iosComparator = readFileSync(resolve(repositoryRoot,
    'tools/css-reference/compare-c11-ios-metal-surface-pixels.mjs'), 'utf8');
  assert.ok(androidCapture.includes('SurfaceViewPixelCapture.request(surfaceView, width, height, density'));
  assert.ok(androidComparator.includes("const usesDensityPixelScale = markerPrefix === 'C111CFM'"));
  assert.ok(iosCapture.includes('let usesDensityMappedC111 = pixelRun.markerPrefix == "C111CFM"'));
  assert.ok(iosComparator.includes("markerPrefix === 'C111CFM'"));
});

test('Android CFM evidence binds app PixelCopy density to independent ADB wm density', () => {
  assert.equal(androidDensityFromWmOutput('Physical density: 420'), 2.625);
  assert.equal(androidDensityFromWmOutput(
    'Physical density: 440\nOverride density: 420'), 2.625);
  assert.throws(() => androidDensityFromWmOutput('Physical density: 0'));
  assert.throws(() => androidDensityFromWmOutput('density unavailable'));
  const runner = readFileSync(resolve(repositoryRoot,
    'tools/css-reference/capture-c11-root-html-column-flow-mixed-cross-axis-runtime-matrix.mjs'), 'utf8');
  assert.ok(runner.includes('androidDensityFromWmOutput(device.density)'));
});
