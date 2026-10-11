import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { decodeRgbaPng } from './read-png-rgba.mjs';

function parseCaptureLog(log, caseId, runId) {
  const lines = log.split(/\r?\n/);
  const errorLine = lines.find((line) => line.includes('SPINON_C124_SURFACE_CAPTURE status=error')
    && line.includes(`run_id=${runId}`));
  assert.equal(errorLine, undefined, `${caseId}: PixelCopy가 실패했습니다: ${errorLine ?? ''}`);

  const captureLines = lines.filter((line) => line.includes('SPINON_C124_SURFACE_CAPTURE status=ok'));
  assert.equal(captureLines.length, 1, `${caseId}: PixelCopy 성공 기록은 정확히 하나여야 합니다.`);
  const capture = captureLines[0].match(
    /SPINON_C124_SURFACE_CAPTURE status=ok run_id=([^ ]+) case=([^ ]+) width=(\d+) height=(\d+) density=([^ ]+) attempts=(\d+) inventory_sha256=([0-9a-f]{64}) sha256=([0-9a-f]{64})/,
  );
  assert.ok(capture, `${caseId}: PixelCopy 성공 기록의 형식이 잘못되었습니다.`);
  assert.equal(capture[1], runId, `${caseId}: 이전 실행의 PixelCopy 기록입니다.`);
  assert.equal(capture[2], caseId, `${caseId}: 다른 장면의 PixelCopy 기록입니다.`);
  const pixels = lines.filter((line) => line.includes('SPINON_C124_SURFACE_PIXEL '))
    .map((line) => {
      const match = line.match(
        /SPINON_C124_SURFACE_PIXEL run_id=([^ ]+) case=([^ ]+) sample=([^ ]+) x=(\d+) y=(\d+) rgba=(\d+),(\d+),(\d+),(\d+)/,
      );
      assert.ok(match, `${caseId}: surface pixel 로그 형식이 잘못되었습니다: ${line}`);
      return {
        runId: match[1],
        caseId: match[2],
        id: match[3],
        x: Number(match[4]),
        y: Number(match[5]),
        rgba: match.slice(6).map(Number),
      };
    });
  return {
    width: Number(capture[3]),
    height: Number(capture[4]),
    density: Number(capture[5]),
    attempts: Number(capture[6]),
    inventorySha256: capture[7],
    imageSha256: capture[8],
    pixels,
  };
}

function rgbaError(actual, expected) {
  return Math.max(...actual.map((channel, index) => Math.abs(channel - expected[index])));
}

export function compareC124AndroidSurfaceCase({
  log,
  pngBytes,
  caseId,
  runId,
  fixtureCase,
  referenceSamples,
  stage,
  chromeOrigin,
  viewportCssPx,
  inventorySha256,
  tolerance,
  expectedDensity,
}) {
  assert.equal(fixtureCase.id, caseId, '요청한 fixture case와 입력 case가 다릅니다.');
  assert.equal(fixtureCase.expected.kind, 'supported', `${caseId}: 거부 대상 장면입니다.`);
  assert.ok(fixtureCase.samples.length > 0, `${caseId}: 고정 픽셀 표본이 없습니다.`);

  const capture = parseCaptureLog(log, caseId, runId);
  assert.equal(capture.inventorySha256, inventorySha256,
    `${caseId}: APK asset과 현재 inventory hash가 다릅니다.`);
  assert.equal(capture.density, expectedDensity, `${caseId}: Android display density가 바뀌었습니다.`);
  assert.equal(capture.attempts >= 1 && capture.attempts <= 3, true,
    `${caseId}: PixelCopy 재시도 수가 허용 범위 밖입니다.`);
  const expectedWidth = Math.round(viewportCssPx.width * expectedDensity);
  const expectedHeight = Math.round(viewportCssPx.height * expectedDensity);
  assert.deepEqual({ width: capture.width, height: capture.height },
    { width: expectedWidth, height: expectedHeight }, `${caseId}: surface bitmap 크기가 다릅니다.`);
  const decodedPng = decodeRgbaPng(pngBytes);
  assert.deepEqual({ width: decodedPng.width, height: decodedPng.height },
    { width: capture.width, height: capture.height }, `${caseId}: PNG 크기와 PixelCopy 결과가 다릅니다.`);
  const imageSha256 = createHash('sha256').update(pngBytes).digest('hex');
  assert.equal(imageSha256, capture.imageSha256, `${caseId}: 저장 PNG hash가 로그와 다릅니다.`);

  assert.equal(capture.pixels.length, fixtureCase.samples.length,
    `${caseId}: sample 수가 fixture와 다릅니다.`);
  assert.equal(new Set(capture.pixels.map(({ id }) => id)).size, capture.pixels.length,
    `${caseId}: sample ID가 중복되었습니다.`);
  const expectedIds = fixtureCase.samples.map(({ id }) => id).sort();
  assert.deepEqual(capture.pixels.map(({ id }) => id).sort(), expectedIds,
    `${caseId}: sample ID 집합이 fixture와 다릅니다.`);

  const compared = fixtureCase.samples.map((sample) => {
    const pixel = capture.pixels.find(({ id }) => id === sample.id);
    assert.equal(pixel.runId, runId, `${caseId}/${sample.id}: 다른 실행의 sample입니다.`);
    assert.equal(pixel.caseId, caseId, `${caseId}/${sample.id}: 다른 장면의 sample입니다.`);
    const expectedX = Math.floor((stage.x + sample.xCssPx + 0.5) * expectedDensity);
    const expectedY = Math.floor((stage.y + sample.yCssPx + 0.5) * expectedDensity);
    assert.deepEqual({ x: pixel.x, y: pixel.y }, { x: expectedX, y: expectedY },
      `${caseId}/${sample.id}: surface 표본 위치가 잘못되었습니다.`);
    assert.deepEqual(decodedPng.getPixel(pixel.x, pixel.y), pixel.rgba,
      `${caseId}/${sample.id}: log RGBA와 PixelCopy PNG의 실제 픽셀이 다릅니다.`);

    const reference = referenceSamples.find(({ fixtureId, sampleId }) =>
      fixtureId === caseId && sampleId === sample.id);
    assert.ok(reference, `${caseId}/${sample.id}: 고정 Chrome pixel oracle이 없습니다.`);
    assert.deepEqual(reference.expectedRgba, sample.rgba,
      `${caseId}/${sample.id}: inventory와 Chrome expected color가 다릅니다.`);
    assert.deepEqual(reference.rgba, reference.expectedRgba,
      `${caseId}/${sample.id}: 고정 Chrome screenshot 자체가 inventory 색과 다릅니다.`);
    assert.equal(reference.xPixel,
      Math.floor((chromeOrigin.x + sample.xCssPx + 0.5) * 1),
      `${caseId}/${sample.id}: DPR 1 Chrome 기준 좌표가 fixture와 다릅니다.`);
    assert.equal(reference.yPixel,
      Math.floor((chromeOrigin.y + sample.yCssPx + 0.5) * 1),
      `${caseId}/${sample.id}: DPR 1 Chrome 기준 좌표가 fixture와 다릅니다.`);
    assert.ok(pixel.rgba.every((channel) => Number.isInteger(channel) && channel >= 0 && channel <= 255),
      `${caseId}/${sample.id}: RGBA 채널이 byte 범위를 벗어났습니다.`);
    assert.equal(pixel.rgba[3], 255, `${caseId}/${sample.id}: 불투명 sample alpha가 255가 아닙니다.`);
    const maxChannelError = rgbaError(pixel.rgba, reference.expectedRgba);
    assert.ok(maxChannelError <= tolerance,
      `${caseId}/${sample.id}: Chrome pixel 차이가 허용치를 넘었습니다: actual=${pixel.rgba} expected=${reference.expectedRgba}`);
    return {
      id: sample.id,
      x: pixel.x,
      y: pixel.y,
      actualRgba: pixel.rgba,
      expectedRgba: reference.expectedRgba,
      maxChannelError,
    };
  });

  return {
    caseId,
    runId,
    width: capture.width,
    height: capture.height,
    density: capture.density,
    attempts: capture.attempts,
    imageSha256,
    samples: compared,
    maximumChannelError: Math.max(...compared.map(({ maxChannelError }) => maxChannelError)),
  };
}
