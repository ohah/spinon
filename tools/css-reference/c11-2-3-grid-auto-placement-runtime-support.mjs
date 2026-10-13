import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const scriptDirectory = dirname(fileURLToPath(import.meta.url));
export const repositoryRoot = resolve(scriptDirectory, '../..');
export const packageName = 'dev.spinon.bootstrap';
export const androidComponent = `${packageName}/.MainActivity`;
export const androidApkPath = resolve(repositoryRoot,
  'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
export const simulatorAppPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
export const simulatorUDID = 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
export const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
export const newRunID = (platform) =>
  `c1123-${platform === 'android-physical' ? 'a' : 'i'}-${Date.now().toString(36)}-${randomUUID().slice(0, 8)}`;
const kstDate = () => new Intl.DateTimeFormat('en-CA', {
  timeZone: 'Asia/Seoul',
}).format(new Date());

export function command(binary, args, {
  cwd = repositoryRoot, timeout = 60_000, encoding = 'utf8',
  allowFailure = false, maxBuffer = 64 * 1024 * 1024,
} = {}) {
  const result = spawnSync(binary, args, { cwd, encoding, timeout, maxBuffer });
  if (result.error || (!allowFailure && result.status !== 0)) {
    throw new Error(`${binary} ${args.join(' ')} 실패: ${result.error?.message
      ?? result.stderr?.toString() ?? result.stdout?.toString()}`);
  }
  return result;
}

export async function loadC1123Inputs() {
  const matrixPath = 'tests/fixtures/css/c11/grid-auto-placement-c1123-runtime-matrix-v1.json';
  const matrixBytes = await readFile(resolve(repositoryRoot, matrixPath));
  const matrix = JSON.parse(matrixBytes.toString('utf8'));
  assert.equal(matrix.schema, 'spinon-css-c11-2-3-grid-auto-placement-runtime-matrix/v1');
  assert.equal(matrix.profileId, 'RuntimeBlockFlexGridAutoPlacementDenseV1');
  const inventoryBytes = await readFile(resolve(repositoryRoot, matrix.inputs.inventory.path));
  const htmlBytes = await readFile(resolve(repositoryRoot, matrix.inputs.html.path));
  const referenceBytes = await readFile(resolve(repositoryRoot, matrix.inputs.chromiumReference.path));
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  assert.equal(sha256(inventoryBytes), matrix.inputs.inventory.sha256);
  assert.equal(sha256(htmlBytes), matrix.inputs.html.sha256);
  assert.equal(matrix.inputs.html.path, 'tests/fixtures/css/c11/grid-auto-placement-c1123-v6.html');
  assert.equal(sha256(referenceBytes), matrix.inputs.chromiumReference.sha256);
  assert.equal(reference.schema, 'spinon-css-c11-2-3-grid-auto-placement-precomparison/v7');
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlPath, matrix.inputs.html.path);
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.deepEqual(matrix.cases.map(({ id }) => id), inventory.cases.map(({ id }) => id));
  const dpr1 = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1);
  assert.ok(dpr1, '고정 Chromium DPR 1 관측이 없습니다.');
  assert.deepEqual(matrix.cases.map(({ id }) => id), dpr1.cases.map(({ id }) => id));
  assert.equal(matrix.summary.sceneCount, 25);
  assert.equal(matrix.summary.implementationCandidateCount, 21);
  assert.equal(matrix.summary.oracleOnlyCount, 4);
  assert.equal(matrix.summary.successCount, 19);
  assert.equal(matrix.summary.expectedRejectCount, 6);
  assert.equal(matrix.summary.nodeCount, 107);
  assert.deepEqual(matrix.viewportCssPx, {
    width: 360, height: 800, deviceScaleFactors: [1, 2],
  });
  for (const fixtureCase of matrix.cases) {
    const chromeCase = dpr1.cases.find(({ id }) => id === fixtureCase.id);
    assert.equal(fixtureCase.nodeIds.length, chromeCase.nodes.length,
      `${fixtureCase.id}: fixture와 Chromium node 수가 다릅니다.`);
    assert.deepEqual(fixtureCase.nodeIds, chromeCase.nodes.map(({ id }) => id),
      `${fixtureCase.id}: fixture와 Chromium node 순서가 다릅니다.`);
  }
  const generatorCheck = command(process.execPath, [
    resolve(repositoryRoot, 'tools/css-reference/build-c11-2-3-grid-auto-placement-runtime-matrix.mjs'),
    '--check',
  ]);
  return {
    matrix,
    inventory,
    reference,
    referenceCases: dpr1.cases,
    hashes: {
      matrix: sha256(matrixBytes), inventory: sha256(inventoryBytes),
      html: sha256(htmlBytes), reference: sha256(referenceBytes),
    },
    generatorCheck: generatorCheck.stdout.trim(),
  };
}

export async function loadC1123PixelMatrix(inputs) {
  const path = 'tests/fixtures/css/c11/grid-auto-placement-c1123-mobile-surface-pixel-matrix.json';
  const bytes = await readFile(resolve(repositoryRoot, path));
  const matrix = JSON.parse(bytes.toString('utf8'));
  assert.equal(matrix.schema, 'spinon-css-c11-2-3-grid-auto-placement-mobile-surface-pixel-matrix/v1');
  assert.equal(matrix.inputs.runtimeMatrix.sha256, inputs.hashes.matrix);
  assert.equal(matrix.inputs.inventory.sha256, inputs.hashes.inventory);
  assert.equal(matrix.inputs.chromiumReference.sha256, inputs.hashes.reference);
  assert.equal(matrix.inputs.html.path, inputs.reference.fixture.htmlPath);
  assert.equal(matrix.inputs.html.sha256, inputs.reference.fixture.htmlSha256);
  assert.deepEqual(matrix.cases.map(({ id }) => id), inputs.matrix.cases.map(({ id }) => id));
  assert.equal(matrix.summary.sceneCount, 25);
  assert.equal(matrix.summary.pixelSceneCount, 19);
  assert.equal(matrix.summary.expectedRejectCount, 6);
  assert.equal(matrix.summary.sampleCount, 99);
  for (const fixtureCase of matrix.cases) {
    const expectedOutcome = inputs.matrix.cases.find(({ id }) => id === fixtureCase.id).expected.outcome;
    const samples = fixtureCase.samples ?? [];
    assert.equal(samples.length > 0, expectedOutcome === 'success',
      `${fixtureCase.id}: runtime outcome과 pixel sample 유무가 다릅니다.`);
    for (const sample of samples) {
      assert.equal(sample.rgba.length, 4, `${fixtureCase.id}/${sample.id}: RGBA 채널 수`);
      assert.ok(sample.rgba.every((channel) => Number.isInteger(channel) && channel >= 0 && channel <= 255),
        `${fixtureCase.id}/${sample.id}: RGBA 범위`);
    }
  }
  const generatorCheck = command(process.execPath, [
    resolve(repositoryRoot, 'tools/css-reference/build-c11-2-3-grid-auto-placement-pixel-matrix.mjs'),
    '--check',
  ]);
  return { matrix, sha256: sha256(bytes), generatorCheck: generatorCheck.stdout.trim() };
}

export async function createEvidenceDirectory(platform, runID) {
  const directory = resolve(repositoryRoot,
    `spec/internal/evidence/c11-2-3-dense-column-placement-${platform}-runtime-matrix-${kstDate()}/${runID}`);
  await mkdir(directory, { recursive: true });
  return directory;
}

export async function saveSceneEvidence({ directory, fixtureCase, chromeCase, lines,
  screenshotBytes, runID, platform, compareScene }) {
  await writeFile(resolve(directory, `${fixtureCase.id}-runtime.log`), `${lines.join('\n')}\n`);
  let screenshot = null;
  if (screenshotBytes) {
    assert.deepEqual([...screenshotBytes.subarray(0, 8)], [137, 80, 78, 71, 13, 10, 26, 10],
      `${fixtureCase.id}: PNG signature`);
    screenshot = `${fixtureCase.id}-${platform}.png`;
    await writeFile(resolve(directory, screenshot), screenshotBytes);
  }
  const comparison = compareScene({ fixtureCase, chromeCase, lines, runID });
  return {
    ...comparison,
    ...(screenshot ? { screenshot, screenshotSha256: sha256(screenshotBytes) } : {}),
  };
}

export async function writeRuntimeSummary({
  directory, platform, runID, target, device, appSha256, inputs, observations,
  pixelMatrix = null, pixelObservations = [],
}) {
  assert.equal(observations.length, inputs.matrix.summary.sceneCount);
  const successfulScenes = observations.filter(({ outcome }) => outcome === 'matched').length;
  const rejectedScenes = observations.filter(({ outcome }) => outcome === 'expected-rejection').length;
  const nodes = observations.reduce((total, { nodeCount = 0 }) => total + nodeCount, 0);
  assert.equal(successfulScenes, 19);
  assert.equal(rejectedScenes, 6);
  assert.equal(nodes, inputs.matrix.summary.nodeCount);
  if (pixelMatrix) {
    assert.equal(pixelObservations.length, pixelMatrix.matrix.summary.pixelSceneCount);
    assert.equal(new Set(pixelObservations.map(({ scene }) => scene)).size, pixelObservations.length);
    assert.ok(pixelObservations.every(({ status, maximumChannelError }) =>
      status === 'matched' && maximumChannelError === 0));
  } else {
    assert.equal(pixelObservations.length, 0);
  }
  const summary = {
    schema: 'spinon-c11-2-3-grid-auto-placement-mobile-runtime-matrix/v1',
    status: 'matched', platform, runID, target, device, appSha256,
    inputs: inputs.hashes,
    summary: {
      scenes: observations.length,
      successfulScenes,
      rejectedScenes,
      nodes,
      maximumAbsoluteErrorCssPx: Math.max(0,
        ...observations.map(({ maximumAbsoluteErrorCssPx = 0 }) => maximumAbsoluteErrorCssPx)),
      geometryToleranceCssPx: 0.5,
      freshProcessPerScene: true,
      screenshotsArePixelParity: false,
      surfacePixelReadback: pixelMatrix ? {
        status: 'matched',
        matrixSha256: pixelMatrix.sha256,
        sceneCount: pixelObservations.length,
        sampleCount: pixelObservations.reduce((total, scene) => total + scene.expectedSamples, 0),
        maximumChannelError: Math.max(0,
          ...pixelObservations.map(({ maximumChannelError }) => maximumChannelError)),
      } : 'not-performed',
    },
    observations,
    limitations: [
      '고정 C11.2.3 Chromium fixture의 runtime geometry 비교이며 Spinon WPT 적합성이나 전체 CSS Grid 지원을 뜻하지 않는다.',
      '화면 PNG는 실행 상태 확인용이며 GPU surface 또는 OS compositor pixel parity를 증명하지 않는다.',
      '이 결과는 기록된 Android 기기 또는 지정 iOS Simulator 한 대에서만 유효하다.',
      ...(pixelMatrix ? [
        '표면 pixel은 기록된 Android 실기기 PixelCopy 또는 iOS Simulator drawable readback에서만 비교했다.',
        'iOS drawable 결과는 OS compositor 제출 전이며 최종 화면 pixel을 뜻하지 않는다.',
      ] : ['Metal/WGPU surface pixel readback은 이 행렬에서 수행하지 않았다.']),
      'oracle-only 4 scene은 runtime 성공으로 요구하지 않고 계약된 오류 동작을 확인한다.',
    ],
  };
  await writeFile(resolve(directory, 'comparison.json'), `${JSON.stringify(summary, null, 2)}\n`);
  if (pixelMatrix) {
    await writeFile(resolve(directory, 'surface-pixel-comparison.json'), `${JSON.stringify({
      schema: 'spinon-c11-2-3-grid-auto-placement-surface-pixel-comparison/v1',
      status: 'matched',
      fixtureId: pixelMatrix.matrix.fixtureId,
      matrixSha256: pixelMatrix.sha256,
      maximumChannelError: 0,
      observations: pixelObservations,
    }, null, 2)}\n`);
  }
  const lines = [
    '# C11.2.3 모바일 runtime geometry 행렬', '',
    `- run ID: \`${runID}\``, `- 대상: ${target}`, `- 앱 SHA-256: \`${appSha256}\``,
    '- 결과: 성공 19/19 · 기대 거부 6/6 · 전체 scene 25개',
    `- 전체 node: ${nodes}개`,
    `- geometry 최대 절대 오차: ${summary.summary.maximumAbsoluteErrorCssPx} CSS px (허용 0.5 CSS px)`,
    '- 장면마다 새 앱 process와 HostDocument를 사용했다.',
    '- 거부 장면은 오류 code·NodeId·CSS property·frame 부재·양수 draw 부재를 확인했다.',
    '- 성공 장면은 DOM 순서·geometry·paint order·양수 WGPU draw를 확인했다.',
    pixelMatrix
      ? `- 표면 pixel: ${pixelObservations.length}개 성공 scene, ${summary.summary.surfacePixelReadback.sampleCount}개 표본, 최대 RGBA 채널 오차 0.`
      : '- 화면 PNG는 실행 상태 보조 자료이며 surface pixel readback은 수행하지 않았다.',
    '- [세부 결과](./comparison.json)와 각 scene의 process-filtered runtime log를 보존했다.',
    '- 이 행렬은 고정 Chromium fixture 결과이며 WPT 적합성 또는 전체 CSS Grid 지원을 뜻하지 않는다.', '',
  ];
  await writeFile(resolve(directory, 'README.md'), `${lines.join('\n')}\n`);
  return summary;
}
