import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { compareC1121GridPlacementScene } from './c11-2-1-grid-placement-runtime-compare.mjs';

export const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
export const packageName = 'dev.spinon.bootstrap';
export const component = `${packageName}/.MainActivity`;
export const simulatorUDID = 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
export const simulatorAppPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
export const androidApkPath = resolve(repositoryRoot,
  'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
export const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
export const newRunID = (platform) =>
  `c1121-${platform}-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
export const kstDate = () => new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Seoul' }).format(new Date());

export function command(binary, args, { cwd = repositoryRoot, timeout = 60_000,
  encoding = 'utf8', allowFailure = false, maxBuffer = 64 * 1024 * 1024 } = {}) {
  const result = spawnSync(binary, args, { cwd, encoding, timeout, maxBuffer });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message
      ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
  }
  return result;
}

export async function loadFixedInputs() {
  const matrixPath = resolve(repositoryRoot, 'tests/fixtures/css/c11/grid-placement-runtime-matrix.json');
  const inventoryPath = resolve(repositoryRoot, 'tests/fixtures/css/c11/grid-placement-inventory.json');
  const htmlPath = resolve(repositoryRoot, 'tests/fixtures/css/c11/grid-placement.html');
  const referencePath = resolve(repositoryRoot, 'tests/fixtures/css/references/c11-grid-placement-precomparison-v2.json');
  const [matrixBytes, inventoryBytes, htmlBytes, referenceBytes] = await Promise.all([
    readFile(matrixPath), readFile(inventoryPath), readFile(htmlPath), readFile(referencePath),
  ]);
  const matrix = JSON.parse(matrixBytes.toString('utf8'));
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  assert.equal(matrix.schema, 'spinon-css-c11-2-1-grid-placement-runtime-matrix/v1');
  assert.equal(matrix.inputs.inventory.sha256, sha256(inventoryBytes));
  assert.equal(matrix.inputs.html.sha256, sha256(htmlBytes));
  assert.equal(matrix.inputs.chromiumReference.sha256, sha256(referenceBytes));
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.deepEqual(matrix.cases.map(({ id }) => id), inventory.cases.map(({ id }) => id));
  const dpr1 = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1);
  assert.ok(dpr1);
  assert.deepEqual(matrix.cases.map(({ id }) => id), dpr1.cases.map(({ id }) => id));
  assert.equal(matrix.summary.sceneCount, 10);
  assert.equal(matrix.summary.candidateCount, 6);
  assert.equal(matrix.summary.expectedRejectCount, 4);
  assert.equal(matrix.summary.nodeCount, 30);
  const generatedCheck = command(process.execPath, [
    resolve(repositoryRoot, 'tools/css-reference/build-c11-2-1-grid-placement-runtime-matrix.mjs'), '--check',
  ]);
  return {
    matrix, reference, referenceCases: dpr1.cases,
    hashes: {
      inventory: sha256(inventoryBytes), html: sha256(htmlBytes),
      reference: sha256(referenceBytes), matrix: sha256(matrixBytes),
    },
    generatorCheck: generatedCheck.stdout.trim(),
  };
}

export async function loadPixelMatrix(inputs) {
  const path = 'tests/fixtures/css/c11/grid-placement-mobile-surface-pixel-matrix.json';
  const bytes = await readFile(resolve(repositoryRoot, path));
  const matrix = JSON.parse(bytes.toString('utf8'));
  matrix.fixtureSha256 = sha256(bytes);
  assert.equal(matrix.schema, 'spinon-css-c11-2-1-mobile-surface-pixel-matrix/v1');
  assert.equal(matrix.inputs.runtimeMatrix.sha256, inputs.hashes.matrix);
  assert.equal(matrix.summary.sceneCount, 10);
  assert.equal(matrix.summary.pixelSceneCount, 6);
  assert.equal(matrix.summary.expectedRejectCount, 4);
  assert.deepEqual(matrix.cases.map(({ id }) => id), inputs.matrix.cases.map(({ id }) => id));
  assert.equal(matrix.cases.filter(({ outcome }) => outcome === 'surface-pixel-compare').length, 6);
  assert.equal(matrix.cases.filter(({ outcome }) => outcome === 'expected-rejection').length, 4);
  const check = command(process.execPath, [
    resolve(repositoryRoot, 'tools/css-reference/build-c11-2-1-grid-placement-pixel-matrix.mjs'), '--check',
  ]);
  return { matrix, bytesSha256: sha256(bytes), generatorCheck: check.stdout.trim() };
}

export async function createEvidenceDirectory(platform, runID) {
  const directory = resolve(repositoryRoot,
    `spec/internal/evidence/c11-2-1-grid-placement-${platform}-runtime-matrix-${kstDate()}/${runID}`);
  await mkdir(directory, { recursive: true });
  return directory;
}

export function referenceCase(referenceCases, sceneId) {
  const fixture = referenceCases.find(({ id }) => id === sceneId);
  assert.ok(fixture, `Chrome 154 reference scene이 없습니다: ${sceneId}`);
  return fixture;
}

export async function saveSceneEvidence({ directory, fixtureCase, chromeCase, lines,
  screenshotBytes, runID, platform }) {
  const runtimeLogPath = resolve(directory, `${fixtureCase.id}-runtime.log`);
  await writeFile(runtimeLogPath, `${lines.join('\n')}\n`);
  let screenshotEvidence = {};
  if (screenshotBytes) {
    assert.deepEqual([...screenshotBytes.subarray(0, 8)], [137, 80, 78, 71, 13, 10, 26, 10],
      `${fixtureCase.id}: PNG signature`);
    const screenshotName = `${fixtureCase.id}-${platform}.png`;
    await writeFile(resolve(directory, screenshotName), screenshotBytes);
    screenshotEvidence = { screenshot: screenshotName, screenshotSha256: sha256(screenshotBytes) };
  }
  const result = compareC1121GridPlacementScene({ fixtureCase, chromeCase, lines, runID });
  return { ...result, ...screenshotEvidence };
}

export async function writeMatrixSummary({ directory, platform, runID, target,
  appSha256, device, inputs, pixelMatrix, pixelIntentNegativeControl, observations }) {
  const pixelComparisons = observations.flatMap(({ surfacePixelComparison }) =>
    surfacePixelComparison ? [surfacePixelComparison] : []);
  const pixelScenes = pixelComparisons.length;
  const pixelSamples = pixelComparisons.reduce((total, { matchedSamples = 0 }) => total + matchedSamples, 0);
  const pixelStatus = pixelScenes === pixelMatrix.summary.pixelSceneCount
      && pixelSamples === pixelMatrix.summary.sampleCount
    ? 'matched' : 'incomplete';
  assert.equal(pixelStatus, 'matched', '지원 가능한 C11.2.1 surface pixel scene이 모두 통과하지 않았습니다.');
  const summary = {
    schema: 'spinon-c11-2-1-grid-placement-mobile-runtime-matrix/v1',
    status: 'matched',
    platform,
    runID,
    target,
    device,
    renderer: device?.renderer ?? (platform.startsWith('iOS')
      ? 'WGPU Metal drawable texture readback before compositor submission'
      : 'Android SurfaceView PixelCopy'),
    appSha256,
    inputs: inputs.hashes,
    summary: {
      scenes: observations.length,
      successfulScenes: observations.filter(({ outcome }) => outcome === 'matched').length,
      rejectedScenes: observations.filter(({ outcome }) => outcome === 'expected-rejection').length,
      nodes: observations.reduce((total, { frameCount = 0 }) => total + Math.max(0, frameCount - 1), 0),
      maximumAbsoluteErrorCssPx: Math.max(0, ...observations.map(({ maximumAbsoluteErrorCssPx = 0 }) => maximumAbsoluteErrorCssPx)),
      geometryToleranceCssPx: 0.5,
      freshProcessPerScene: true,
      screenshotsArePixelParity: false,
      surfacePixelMatrixStatus: pixelStatus,
      surfacePixelScenes: pixelScenes,
      expectedSurfacePixelScenes: pixelMatrix.summary.pixelSceneCount,
      surfacePixelSamples: pixelSamples,
      expectedSurfacePixelSamples: pixelMatrix.summary.sampleCount,
      maximumSurfacePixelChannelError: Math.max(0,
        ...pixelComparisons.map(({ maximumChannelError = 0 }) => maximumChannelError)),
    },
    surfacePixelMatrix: {
      schema: pixelMatrix.schema,
      fixtureId: pixelMatrix.fixtureId,
      fixtureSha256: pixelMatrix.fixtureSha256,
      expectedRejectScenesWithoutPixelParity: pixelMatrix.summary.expectedRejectCount,
      comparisons: pixelComparisons,
    },
    pixelIntentNegativeControl,
    observations,
    limitations: [
      ...(platform.startsWith('Android') || platform === 'android-avd'
        ? ['Android AVD 결과는 AVD software renderer이며 Android 실기기를 대체하지 않는다.']
        : []),
      ...(platform.startsWith('iOS') || platform === 'ios-simulator'
        ? ['iOS 결과는 지정 Simulator이며 iOS 실기기를 대체하지 않는다.']
        : []),
      '표면 픽셀 비교는 C11.2.1 제한 profile의 성공 후보 6 scene에 한정된다. 네 기대 거부 scene은 픽셀 parity 대상이 아니다.',
      '고정 fixture 결과는 Spinon WPT 적합성이나 전체 CSS Grid 지원을 뜻하지 않는다.',
    ],
  };
  assert.equal(summary.summary.scenes, 10);
  assert.equal(summary.summary.successfulScenes, 6);
  assert.equal(summary.summary.rejectedScenes, 4);
  await writeFile(resolve(directory, 'comparison.json'), `${JSON.stringify(summary, null, 2)}\n`);
  const lines = [
    '# C11.2.1 모바일 runtime matrix', '',
    `- run ID: \`${runID}\``, `- target: ${target}`, `- app SHA-256: \`${appSha256}\``,
    `- device metadata: [device.json](./device.json)`,
    `- renderer: ${device?.renderer ?? (platform.startsWith('iOS')
      ? 'WGPU Metal drawable texture readback before compositor submission'
      : 'Android SurfaceView PixelCopy')}`,
    `- inventory SHA-256: \`${inputs.hashes.inventory}\``,
    `- HTML SHA-256: \`${inputs.hashes.html}\``,
    `- Chrome 154 reference SHA-256: \`${inputs.hashes.reference}\``,
    `- 결과: 성공 ${summary.summary.successfulScenes}/6 · 기대 거부 ${summary.summary.rejectedScenes}/4`,
    `- surface pixel: ${pixelScenes}/6 scene · ${pixelSamples}/${pixelMatrix.summary.sampleCount} sample exact · 최대 channel 오차 ${summary.summary.maximumSurfacePixelChannelError}`,
    `- geometry 최대 절대 오차: ${summary.summary.maximumAbsoluteErrorCssPx} CSS px (허용 0.5 CSS px)`,
    '- scene마다 새 앱 process와 HostDocument를 사용했다.',
    '- 거부 장면은 정확한 code·NodeId·property를 확인하고, 비어 있지 않은 WGPU 제출과 frame을 거부했다.',
    '- 화면 PNG는 실행 장면을 확인하는 보조 자료다. 표면 픽셀 비교는 성공 scene의 PixelCopy PNG 또는 Metal drawable readback만 사용했다.',
    '- [세부 결과](./comparison.json). 각 scene의 process-filtered runtime log와 PNG를 함께 보존했다.',
    '- 네 기대 거부 장면은 정확한 오류·partial frame 없음·non-empty WGPU present 없음으로 별도 확인하며 Chrome과 pixel parity를 주장하지 않는다.',
    '- 지원 scene allowlist를 벗어난 pixel capture 요청은 플랫폼 앱 경계에서 거부했고 표면 수집 marker가 나오지 않았다.',
    '- Android 실기기, Spinon WPT 적합성 및 전체 CSS Grid 지원은 별도 범위다.', '',
  ];
  await writeFile(resolve(directory, 'README.md'), `${lines.join('\n')}\n`);
  return summary;
}
