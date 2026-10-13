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
export const simulatorUDID = 'ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D';
export const androidApkPath = resolve(repositoryRoot,
  'platforms/android/app/build/outputs/apk/debug/app-debug.apk');
export const simulatorAppPath = resolve(repositoryRoot,
  'build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app');
export const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
export const newRunID = (platform) =>
  `c1122-${platform === 'android-physical' ? 'a' : 'i'}-${Date.now().toString(36)}-${randomUUID().slice(0, 8)}`;
export const kstDate = () => new Intl.DateTimeFormat('en-CA', {
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

export async function loadC1122Inputs() {
  const matrixPath = 'tests/fixtures/css/c11/grid-auto-placement-runtime-matrix.json';
  const inventoryPath = 'tests/fixtures/css/c11/grid-auto-placement-inventory-v3.json';
  const htmlPath = 'tests/fixtures/css/c11/grid-auto-placement-v3.html';
  const referencePath = 'tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v4.json';
  const paths = { matrix: matrixPath, inventory: inventoryPath, html: htmlPath, reference: referencePath };
  const entries = await Promise.all(Object.entries(paths).map(async ([key, path]) => [
    key, await readFile(resolve(repositoryRoot, path)),
  ]));
  const bytes = Object.fromEntries(entries);
  const matrix = JSON.parse(bytes.matrix.toString('utf8'));
  const inventory = JSON.parse(bytes.inventory.toString('utf8'));
  const reference = JSON.parse(bytes.reference.toString('utf8'));
  assert.equal(matrix.schema, 'spinon-css-c11-2-2-grid-auto-placement-runtime-matrix/v1');
  assert.equal(reference.schema, 'spinon-css-c11-2-2-grid-auto-placement-precomparison/v4');
  assert.equal(matrix.inputs.inventory.sha256, sha256(bytes.inventory));
  assert.equal(matrix.inputs.html.sha256, sha256(bytes.html));
  assert.equal(matrix.inputs.chromiumReference.sha256, sha256(bytes.reference));
  assert.equal(reference.fixture.inventorySha256, sha256(bytes.inventory));
  assert.equal(reference.fixture.htmlSha256, sha256(bytes.html));
  assert.deepEqual(matrix.cases.map(({ id }) => id), inventory.cases.map(({ id }) => id));
  const dpr1 = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1);
  assert.ok(dpr1, 'Chrome 154 DPR 1 observation이 없습니다.');
  assert.deepEqual(matrix.cases.map(({ id }) => id), dpr1.cases.map(({ id }) => id));
  assert.equal(matrix.summary.sceneCount, 16);
  assert.equal(matrix.summary.candidateCount, 11);
  assert.equal(matrix.summary.expectedRejectCount, 5);
  assert.equal(matrix.summary.nodeCount, 69);
  assert.deepEqual(matrix.viewportCssPx, { width: 360, height: 800 });
  for (const fixtureCase of matrix.cases) {
    const chromeCase = dpr1.cases.find(({ id }) => id === fixtureCase.id);
    assert.equal(fixtureCase.nodeIds.length, chromeCase.nodes.length,
      `${fixtureCase.id}: fixture와 Chrome node 수가 다릅니다.`);
    assert.deepEqual(fixtureCase.nodeIds, chromeCase.nodes.map(({ id }) => id),
      `${fixtureCase.id}: fixture와 Chrome node order가 다릅니다.`);
  }
  const generatorCheck = command(process.execPath, [
    resolve(repositoryRoot, 'tools/css-reference/build-c11-2-2-grid-auto-placement-runtime-matrix.mjs'),
    '--check',
  ]);
  return {
    matrix,
    reference,
    referenceCases: dpr1.cases,
    hashes: Object.fromEntries(Object.entries(bytes).map(([key, value]) => [key, sha256(value)])),
    generatorCheck: generatorCheck.stdout.trim(),
  };
}

export async function loadC1122PixelMatrix(inputs) {
  const path = 'tests/fixtures/css/c11/grid-auto-placement-mobile-surface-pixel-matrix.json';
  const bytes = await readFile(resolve(repositoryRoot, path));
  const matrix = JSON.parse(bytes.toString('utf8'));
  assert.equal(matrix.schema, 'spinon-css-c11-2-2-grid-auto-placement-mobile-surface-pixel-matrix/v1');
  assert.equal(matrix.inputs.runtimeMatrix.sha256, inputs.hashes.matrix);
  assert.equal(matrix.inputs.inventory.sha256, inputs.hashes.inventory);
  assert.equal(matrix.inputs.html.sha256, inputs.hashes.html);
  assert.equal(matrix.inputs.chromiumReference.sha256, inputs.hashes.reference);
  assert.deepEqual(matrix.cases.map(({ id }) => id), inputs.matrix.cases.map(({ id }) => id));
  assert.equal(matrix.summary.sceneCount, 16);
  assert.equal(matrix.summary.pixelSceneCount, 11);
  assert.equal(matrix.summary.expectedRejectCount, 5);
  assert.equal(matrix.summary.sampleCount, 51);
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
    resolve(repositoryRoot, 'tools/css-reference/build-c11-2-2-grid-auto-placement-pixel-matrix.mjs'),
    '--check',
  ]);
  return { matrix, sha256: sha256(bytes), generatorCheck: generatorCheck.stdout.trim() };
}

export function referenceCase(inputs, sceneID) {
  const fixture = inputs.referenceCases.find(({ id }) => id === sceneID);
  assert.ok(fixture, `Chrome 154 reference scene 없음: ${sceneID}`);
  return fixture;
}

export async function createEvidenceDirectory(platform, runID) {
  const directory = resolve(repositoryRoot,
    `spec/internal/evidence/c11-2-2-sparse-row-placement-${platform}-runtime-matrix-${kstDate()}/${runID}`);
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
  pixelMatrix, pixelObservations = [],
}) {
  assert.equal(observations.length, inputs.matrix.summary.sceneCount);
  assert.ok(pixelMatrix, 'C11.2.2 pixel matrix fixture를 제공해야 합니다.');
  assert.equal(pixelObservations.length, pixelMatrix.matrix.summary.pixelSceneCount,
    '성공 scene 전체의 surface pixel readback이 필요합니다.');
  assert.ok(pixelObservations.every(({ status }) => status === 'matched'),
    '모든 surface pixel readback이 Chrome 표본과 일치해야 합니다.');
  const summary = {
    schema: 'spinon-c11-2-2-grid-auto-placement-mobile-runtime-matrix/v1',
    status: 'matched',
    platform,
    runID,
    target,
    device,
    appSha256,
    inputs: inputs.hashes,
    summary: {
      scenes: observations.length,
      successfulScenes: observations.filter(({ outcome }) => outcome === 'matched').length,
      rejectedScenes: observations.filter(({ outcome }) => outcome === 'expected-rejection').length,
      nodes: observations.reduce((total, { nodeCount = 0 }) => total + nodeCount, 0),
      maximumAbsoluteErrorCssPx: Math.max(0,
        ...observations.map(({ maximumAbsoluteErrorCssPx = 0 }) => maximumAbsoluteErrorCssPx)),
      geometryToleranceCssPx: 0.5,
      freshProcessPerScene: true,
      screenshotsArePixelParity: false,
      surfacePixelReadback: pixelObservations.length === inputs.matrix.summary.candidateCount
        ? {
          status: pixelObservations.every(({ status }) => status === 'matched') ? 'matched' : 'mismatch',
          scenes: pixelObservations.length,
          samples: pixelObservations.reduce((total, { matchedSamples = 0 }) => total + matchedSamples, 0),
          maximumChannelError: Math.max(0,
            ...pixelObservations.map(({ maximumChannelError = 0 }) => maximumChannelError)),
          fixtureSha256: pixelMatrix?.sha256 ?? null,
        }
        : { status: 'incomplete', scenes: pixelObservations.length,
          expectedScenes: inputs.matrix.summary.candidateCount, fixtureSha256: pixelMatrix?.sha256 ?? null },
    },
    observations,
    limitations: [
      '화면 PNG는 실행 상태 확인용이며 표면 픽셀 parity를 증명하지 않는다.',
      '이 결과는 고정 C11.2.2 fixture의 runtime geometry 비교이며 Spinon WPT 적합성이나 전체 CSS Grid 지원을 뜻하지 않는다.',
      platform === 'Android physical device'
        ? '단일 Android 실기기의 결과이며 다른 기기·OS 버전을 대표하지 않는다.'
        : '지정 iOS Simulator 결과이며 iOS 실기기를 대표하지 않는다.',
      '픽셀 비교는 지정 Android 기기 또는 iOS Simulator의 draw surface 내부 표본이며 OS compositor 최종 출력·다른 기기·전체 CSS Grid 적합성을 증명하지 않는다.',
    ],
  };
  assert.equal(summary.summary.successfulScenes, 11);
  assert.equal(summary.summary.rejectedScenes, 5);
  assert.equal(summary.summary.nodes, inputs.matrix.summary.nodeCount);
  await writeFile(resolve(directory, 'comparison.json'), `${JSON.stringify(summary, null, 2)}\n`);
  const lines = [
    '# C11.2.2 모바일 runtime geometry 행렬', '',
    `- run ID: \`${runID}\``, `- 대상: ${target}`, `- 앱 SHA-256: \`${appSha256}\``,
    `- 기기 정보: [device.json](./device.json)`,
    `- Chrome 154 기준 SHA-256: \`${inputs.hashes.reference}\``,
    `- 결과: 성공 ${summary.summary.successfulScenes}/11 · 기대 거부 ${summary.summary.rejectedScenes}/5`,
    `- geometry 최대 절대 오차: ${summary.summary.maximumAbsoluteErrorCssPx} CSS px (허용 0.5 CSS px)`,
    '- 장면마다 새 앱 process와 HostDocument를 사용했다.',
    '- 거부 장면은 오류 code·NodeId·CSS property, frame 부재, 양수 draw 부재를 확인했다.',
    '- 성공 장면은 source-order NodeId·geometry·paint order·양수 WGPU draw를 확인했다.',
    `- 화면 PNG는 실행 상태 보조 자료다. 표면 pixel 비교: ${summary.summary.surfacePixelReadback.status} (${summary.summary.surfacePixelReadback.scenes ?? 0}/${inputs.matrix.summary.candidateCount} 성공 장면).`,
    '- [세부 결과](./comparison.json). 각 장면의 process-filtered runtime log와 PNG를 보존했다.',
    '- 이 행렬은 고정 fixture 결과이며 WPT 적합성 또는 전체 CSS Grid 지원을 뜻하지 않는다.',
    pixelMatrix ? `- Pixel matrix fixture SHA-256: \`${pixelMatrix.sha256}\`; 최대 RGBA 채널 오차: ${summary.summary.surfacePixelReadback.maximumChannelError ?? '미확정'}.` : '- Pixel matrix fixture를 제공하지 않았다.',
    '- 표면 표본은 OS compositor 최종 출력이 아닌 renderer surface 결과다.', '',
  ];
  await writeFile(resolve(directory, 'README.md'), `${lines.join('\n')}\n`);
  return summary;
}
