import assert from 'node:assert/strict';
import { randomUUID, createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';

import {
  command, packageName, androidApkPath, androidComponent, repositoryRoot,
  simulatorAppPath, simulatorUDID,
} from './c11-2-2-grid-auto-placement-runtime-support.mjs';
import { saveSceneEvidence } from './c11-2-2-grid-auto-placement-runtime-support.mjs';
import { compareC1122GridAutoPlacementScene } from './c11-2-2-grid-auto-placement-runtime-compare.mjs';

export {
  androidApkPath, androidComponent, command, packageName, repositoryRoot,
  simulatorAppPath, simulatorUDID, saveSceneEvidence,
};

export const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
export const kstDate = () => new Intl.DateTimeFormat('en-CA', {
  timeZone: 'Asia/Seoul',
}).format(new Date());

export function newOrderRunID(platform) {
  const prefix = platform === 'android-avd' ? 'a' : 'i';
  return `c1122-order-${prefix}-${Date.now().toString(36)}-${randomUUID().slice(0, 8)}`;
}

export async function loadC1122GridItemOrderInputs() {
  const runtimeMatrixPath = 'tests/fixtures/css/c11/grid-item-order-runtime-matrix-v1.json';
  const pixelMatrixPath = 'tests/fixtures/css/c11/grid-item-order-mobile-surface-pixel-matrix-v1.json';
  const [runtimeBytes, pixelBytes] = await Promise.all([
    readFile(resolve(repositoryRoot, runtimeMatrixPath)),
    readFile(resolve(repositoryRoot, pixelMatrixPath)),
  ]);
  const runtimeMatrix = JSON.parse(runtimeBytes.toString('utf8'));
  const pixelMatrix = JSON.parse(pixelBytes.toString('utf8'));
  assert.equal(runtimeMatrix.schema, 'spinon-css-c11-2-2-grid-item-order-runtime-matrix/v1');
  assert.equal(pixelMatrix.schema, 'spinon-css-c11-2-2-grid-item-order-mobile-surface-pixel-matrix/v1');
  assert.equal(pixelMatrix.inputs.runtimeMatrix.sha256, sha256(runtimeBytes));
  assert.deepEqual(runtimeMatrix.cases.map(({ id }) => id), ['order-mixed', 'order-ties']);
  assert.deepEqual(pixelMatrix.cases.map(({ id }) => id), runtimeMatrix.cases.map(({ id }) => id));
  assert.deepEqual(runtimeMatrix.summary, { sceneCount: 2, nodeCount: 9, itemCount: 7 });
  assert.deepEqual(pixelMatrix.summary, { sceneCount: 2, sampleCount: 7 });
  for (const fixtureCase of runtimeMatrix.cases) {
    const pixelCase = pixelMatrix.cases.find(({ id }) => id === fixtureCase.id);
    assert.equal(pixelCase.runtimeMatrixSourceSha256, fixtureCase.sourceSha256,
      `${fixtureCase.id}: Pixel matrix source hash`);
    assert.equal(pixelCase.samples.length, fixtureCase.nodeIds.length - 1,
      `${fixtureCase.id}: 표본 수`);
  }
  for (const generator of [
    'build-c11-2-2-grid-item-order-runtime-matrix.mjs',
    'build-c11-2-2-grid-item-order-pixel-matrix.mjs',
  ]) {
    const result = command(process.execPath, [
      resolve(repositoryRoot, `tools/css-reference/${generator}`), '--check',
    ]);
    assert.match(result.stdout, /PASS/);
  }
  return { runtimeMatrix, pixelMatrix, runtimeHash: sha256(runtimeBytes), pixelHash: sha256(pixelBytes) };
}

export function orderFixtureCase(fixtureCase) {
  return {
    id: fixtureCase.id,
    nodeIds: fixtureCase.nodeIds,
    expected: fixtureCase.expected,
  };
}

export function chromeCaseFromMatrix(fixtureCase) {
  return {
    nodes: fixtureCase.expectedRects.map(({ id, rect }, index) => ({
      id,
      rect,
      properties: index === 0 ? {} : { order: fixtureCase.computedOrders[id] },
    })),
  };
}

export function compareOrderRuntimeScene({
  fixtureCase, lines, runID,
}) {
  const chromeCase = chromeCaseFromMatrix(fixtureCase);
  const comparison = compareC1122GridAutoPlacementScene({
    fixtureCase: orderFixtureCase(fixtureCase),
    chromeCase,
    lines,
    runID,
  });
  const summaryLine = lines.find((line) => line.includes('SPINON_C1122_GRID_SUMMARY ')
    && line.includes(`run_id=${runID}`) && line.includes(`scene_id=${fixtureCase.id}`));
  assert.ok(summaryLine, `${fixtureCase.id}: runtime summary marker 없음`);
  const fields = Object.fromEntries([...summaryLine.matchAll(/([a-z_]+)=([^\s]+)/g)]
    .map(([, key, value]) => [key, value]));
  const generation = Number(fields.generation);
  const documentRevision = Number(fields.document_revision);
  const environmentRevision = Number(fields.environment_revision);
  assert.ok(Number.isSafeInteger(generation) && generation > 0, `${fixtureCase.id}: generation`);
  assert.ok(Number.isSafeInteger(documentRevision) && documentRevision > 0,
    `${fixtureCase.id}: document revision`);
  assert.ok(Number.isSafeInteger(environmentRevision) && environmentRevision >= 0,
    `${fixtureCase.id}: environment revision`);
  return {
    ...comparison,
    revisions: { generation, document: documentRevision, environment: environmentRevision },
  };
}

export async function createOrderEvidenceDirectory(platform, runID) {
  const directory = resolve(repositoryRoot,
    `spec/internal/evidence/c11-2-2-grid-item-order-runtime-${kstDate()}/${platform}/${runID}`);
  await mkdir(directory, { recursive: true });
  return directory;
}

export async function writeOrderRuntimeSummary({
  directory, platform, runID, target, device, appSha256, inputs, observations, pixelObservations,
}) {
  assert.equal(observations.length, 2);
  assert.equal(pixelObservations.length, 2);
  assert.ok(observations.every(({ outcome, frameCount }) => outcome === 'matched' && frameCount === 5
    || outcome === 'matched' && frameCount === 6));
  assert.ok(pixelObservations.every(({ status, matchedSamples }) => status === 'matched'
    && matchedSamples >= 3));
  const summary = {
    schema: 'spinon-c11-2-2-grid-item-order-mobile-runtime/v1',
    status: 'matched',
    platform,
    runID,
    target,
    device,
    appSha256,
    inputs: { runtimeMatrixSha256: inputs.runtimeHash, pixelMatrixSha256: inputs.pixelHash },
    summary: {
      scenes: observations.length,
      frameComparisons: observations.reduce((count, item) => count + item.frameCount, 0),
      surfacePixelSamples: pixelObservations.reduce((count, item) => count + item.matchedSamples, 0),
      maximumGeometryErrorCssPx: Math.max(...observations.map(({ maximumAbsoluteErrorCssPx }) => maximumAbsoluteErrorCssPx)),
      maximumChannelError: Math.max(...pixelObservations.map(({ maximumChannelError }) => maximumChannelError)),
      freshProcessPerScene: true,
      screenshotsArePixelParity: false,
    },
    observations,
    pixelObservations,
    limitations: [
      '두 fixed row-sparse auto-placement scene만 검증했으며 일반 CSS Grid 또는 C11 완료를 뜻하지 않는다.',
      'Chrome 156 geometry와 지정 simulator runtime을 비교했다. 두 simulator는 실기기가 아니다.',
      '표면 픽셀은 Android PixelCopy 또는 iOS Metal drawable 내부 표본이며 OS compositor 출력은 확인하지 않았다.',
      '비겹침 item만 사용해 painting order, overlap, stacking을 검증하지 않았다.',
    ],
  };
  await writeFile(resolve(directory, 'comparison.json'), `${JSON.stringify(summary, null, 2)}\n`);
  const readme = [
    '# C11.2.2 Grid item order simulator runtime', '',
    `- 실행: \`${runID}\``, `- 플랫폼: ${platform}`, `- 대상: ${target}`,
    `- 앱 SHA-256: \`${appSha256}\``,
    `- runtime matrix SHA-256: \`${inputs.runtimeHash}\``,
    `- surface pixel matrix SHA-256: \`${inputs.pixelHash}\``,
    `- 결과: geometry 장면 ${observations.length}/2, surface 표본 ${summary.summary.surfacePixelSamples}/7`,
    `- 최대 geometry 오차: ${summary.summary.maximumGeometryErrorCssPx} CSS px`,
    `- 최대 RGBA 채널 오차: ${summary.summary.maximumChannelError}`,
    '- scene마다 새 앱 process를 시작했다.',
    '- 상세 결과: [comparison.json](./comparison.json), [device.json](./device.json)',
    '- 각 scene의 runtime 로그, screenshot, 표면 표본 및 비교 JSON을 같은 폴더에 보존했다.',
    '', '## 제한', '', ...summary.limitations.map((item) => `- ${item}`), '',
  ].join('\n');
  await writeFile(resolve(directory, 'README.md'), readme);
  return summary;
}

export const compareOrderScene = compareOrderRuntimeScene;
