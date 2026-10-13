import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';

import { decodeRgbaPng } from './read-png-rgba.mjs';

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function fieldsAfter(line, marker) {
  const offset = line.indexOf(marker);
  assert.notEqual(offset, -1, `Android 표면 로그 marker가 없습니다: ${marker.trim()}`);
  return Object.fromEntries([...line.slice(offset + marker.length).matchAll(/([a-z0-9_]+)=([^\s]+)/g)]
    .map(([, key, value]) => [key, value]));
}

function markerLines(log, marker, runID) {
  return log.split(/\r?\n/).filter((line) => {
    if (!line.includes(marker)) return false;
    return fieldsAfter(line, marker).run_id === runID;
  });
}

function exactInteger(value, label, minimum = 0) {
  const parsed = Number(value);
  assert.ok(Number.isSafeInteger(parsed) && parsed >= minimum, `${label} 정수가 잘못되었습니다: ${value}`);
  return parsed;
}

export function cssPixelCenterToSurfacePixel(cssPixel, surfaceSizePx, viewportSizeCssPx) {
  assert.ok(Number.isSafeInteger(cssPixel) && cssPixel >= 0, 'CSS pixel 좌표가 잘못되었습니다.');
  assert.ok(Number.isSafeInteger(surfaceSizePx) && surfaceSizePx > 0, 'surface 크기가 잘못되었습니다.');
  assert.ok(Number.isSafeInteger(viewportSizeCssPx) && viewportSizeCssPx > 0,
    'CSS viewport 크기가 잘못되었습니다.');
  const pixel = Math.floor((cssPixel + 0.5) * (surfaceSizePx / viewportSizeCssPx));
  assert.ok(pixel >= 0 && pixel < surfaceSizePx, '계산한 surface 표본이 범위를 벗어났습니다.');
  return pixel;
}

export function compareC11AndroidPixelCopyScene(log, {
  scene, runID, pngBytes, viewportCssPx,
  markerPrefix = 'C111', cacheDirectory = 'c11-grid-surface-pixel-matrix',
}) {
  assert.ok(['C111', 'C1122', 'C1123', 'C1131'].includes(markerPrefix), 'Android 표면 marker prefix가 잘못되었습니다.');
  assert.ok(['c11-grid-surface-pixel-matrix', 'c1122-grid-surface-pixel-matrix',
    'c1123-grid-surface-pixel-matrix', 'c1131-grid-surface-pixel-matrix']
    .includes(cacheDirectory), 'Android 표면 cache 경로가 허용되지 않습니다.');
  const marker = `SPINON_${markerPrefix}_GRID_PIXEL_MATRIX`;
  const BEGIN = `${marker}_BEGIN `;
  const CAPTURE = `${marker}_CAPTURE `;
  const SAMPLE = `${marker}_SAMPLE `;
  const SUMMARY = `${marker}_SUMMARY `;
  assert.match(runID, /^[A-Za-z0-9_-]{8,128}$/, 'run ID 형식이 잘못되었습니다.');
  assert.ok(scene && /^[a-z0-9-]{1,80}$/.test(scene.id), 'scene ID가 잘못되었습니다.');
  assert.ok(Array.isArray(scene.samples) && scene.samples.length >= 1 && scene.samples.length <= 64,
    'fixture sample 수가 1–64 범위 밖입니다.');
  assert.ok(Buffer.isBuffer(pngBytes), 'PixelCopy PNG가 Buffer가 아닙니다.');
  assert.ok(viewportCssPx && Number.isSafeInteger(viewportCssPx.width) && viewportCssPx.width > 0
    && Number.isSafeInteger(viewportCssPx.height) && viewportCssPx.height > 0,
  'C11 runtime CSS viewport가 잘못되었습니다.');

  const begins = markerLines(log, BEGIN, runID);
  const captures = markerLines(log, CAPTURE, runID);
  const samples = markerLines(log, SAMPLE, runID);
  const summaries = markerLines(log, SUMMARY, runID);
  assert.equal(begins.length, 1, '같은 실행의 begin marker가 정확히 하나여야 합니다.');
  assert.equal(captures.length, 1, '같은 실행의 성공 capture marker가 정확히 하나여야 합니다.');
  assert.equal(summaries.length, 1, '같은 실행의 summary marker가 정확히 하나여야 합니다.');
  assert.equal(samples.length, scene.samples.length, '같은 실행의 표본 개수가 다릅니다.');

  const begin = fieldsAfter(begins[0], BEGIN);
  const captureLine = captures[0];
  assert.ok(captureLine.includes(`${CAPTURE}status=ok `), 'PixelCopy capture status가 ok가 아닙니다.');
  const capture = fieldsAfter(captureLine, CAPTURE);
  const summary = fieldsAfter(summaries[0], SUMMARY);
  for (const [label, fields] of [['begin', begin], ['capture', capture], ['summary', summary]]) {
    assert.equal(fields.run_id, runID, `${label}: run ID가 다릅니다.`);
    assert.equal(fields.scene, scene.id, `${label}: scene이 다릅니다.`);
  }
  const pid = exactInteger(begin.pid, 'app PID', 1);
  const generation = exactInteger(begin.generation, 'surface generation', 1);
  const width = exactInteger(begin.width, 'begin width', 1);
  const height = exactInteger(begin.height, 'begin height', 1);
  const viewportWidth = exactInteger(begin.viewport_width, 'begin CSS viewport width', 1);
  const viewportHeight = exactInteger(begin.viewport_height, 'begin CSS viewport height', 1);
  assert.deepEqual([viewportWidth, viewportHeight], [viewportCssPx.width, viewportCssPx.height],
    'Android PixelCopy viewport가 고정 C11 runtime viewport와 다릅니다.');
  const scaleX = width / viewportWidth;
  const scaleY = height / viewportHeight;
  assert.equal(Number(begin.scale_x), scaleX, 'begin surface x scale이 실제 크기에서 계산되지 않았습니다.');
  assert.equal(Number(begin.scale_y), scaleY, 'begin surface y scale이 실제 크기에서 계산되지 않았습니다.');
  const density = Number(begin.density);
  assert.ok(Number.isFinite(density) && density > 0, 'Android density가 잘못되었습니다.');
  assert.equal(exactInteger(begin.samples, 'begin sample count', 1), scene.samples.length);
  assert.equal(exactInteger(capture.generation, 'capture generation', 1), generation);
  assert.equal(exactInteger(capture.width, 'capture width', 1), width);
  assert.equal(exactInteger(capture.height, 'capture height', 1), height);
  assert.equal(exactInteger(capture.viewport_width, 'capture CSS viewport width', 1), viewportWidth);
  assert.equal(exactInteger(capture.viewport_height, 'capture CSS viewport height', 1), viewportHeight);
  assert.equal(Number(capture.scale_x), scaleX, 'capture surface x scale이 실제 크기에서 계산되지 않았습니다.');
  assert.equal(Number(capture.scale_y), scaleY, 'capture surface y scale이 실제 크기에서 계산되지 않았습니다.');
  assert.equal(Number(capture.density), density, 'PixelCopy begin/capture density가 다릅니다.');
  assert.equal(exactInteger(capture.attempts, 'PixelCopy attempts', 1) <= 3, true,
    'PixelCopy retry 수가 제한을 넘었습니다.');
  assert.equal(exactInteger(summary.generation, 'summary generation', 1), generation);
  assert.equal(summary.status, markerPrefix === 'C1131' ? 'matched' : 'captured',
    'PixelCopy summary 상태가 예상한 값이 아닙니다.');
  assert.equal(exactInteger(summary.expected_samples, 'expected sample count', 1), scene.samples.length);
  assert.equal(exactInteger(summary.observed_samples, 'observed sample count', 1), scene.samples.length);
  if (markerPrefix === 'C1131') {
    assert.equal(exactInteger(summary.matched_samples, 'matched sample count'), scene.samples.length,
      'C11.3.1 runtime의 exact 표본 수가 fixture와 다릅니다.');
  }

  const pngPath = capture.png;
  const escapedCacheDirectory = cacheDirectory.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  assert.match(pngPath ?? '', new RegExp(`^/data/user/0/dev\\.spinon\\.bootstrap/cache/${escapedCacheDirectory}/[a-z0-9-]+-[A-Za-z0-9_-]+\\.png$`),
    'PixelCopy PNG 경로가 앱 cache 경로/run 규칙과 다릅니다.');
  const image = decodeRgbaPng(pngBytes);
  assert.deepEqual([image.width, image.height], [width, height], 'PixelCopy PNG 크기가 marker와 다릅니다.');
  const pngSha256 = sha256(pngBytes);
  assert.equal(capture.sha256, pngSha256, 'PixelCopy PNG SHA-256이 marker와 다릅니다.');

  const records = new Map();
  for (const line of samples) {
    const record = fieldsAfter(line, SAMPLE);
    assert.equal(record.run_id, runID, '표본 run ID가 다릅니다.');
    assert.equal(record.scene, scene.id, '표본 scene이 다릅니다.');
    assert.equal(exactInteger(record.pid, 'sample PID', 1), pid, '표본 PID가 begin과 다릅니다.');
    assert.equal(exactInteger(record.generation, 'sample generation', 1), generation,
      '표본 surface generation이 begin과 다릅니다.');
    assert.ok(!records.has(record.sample), `중복 표본 ID입니다: ${record.sample}`);
    records.set(record.sample, record);
  }
  assert.deepEqual([...records.keys()].sort(), scene.samples.map(({ id }) => id).sort(),
    '표본 ID 집합이 고정 fixture와 다릅니다.');

  const checkedSamples = scene.samples.map((expected) => {
    const actual = records.get(expected.id);
    const cssX = exactInteger(actual.css_x, `${expected.id} CSS x`);
    const cssY = exactInteger(actual.css_y, `${expected.id} CSS y`);
    assert.deepEqual([cssX, cssY], [expected.xCssPx, expected.yCssPx], `${expected.id}: CSS 좌표`);
    const x = cssPixelCenterToSurfacePixel(cssX, width, viewportWidth);
    const y = cssPixelCenterToSurfacePixel(cssY, height, viewportHeight);
    assert.deepEqual([exactInteger(actual.x, `${expected.id} pixel x`),
      exactInteger(actual.y, `${expected.id} pixel y`)], [x, y], `${expected.id}: device pixel 좌표`);
    const rgba = actual.rgba.split(',').map((value) => exactInteger(value, `${expected.id} RGBA`, 0));
    assert.equal(rgba.length, 4, `${expected.id}: RGBA 채널 수`);
    assert.ok(rgba.every((value) => value <= 255), `${expected.id}: RGBA 범위`);
    assert.deepEqual(rgba, expected.rgba, `${expected.id}: PixelCopy 결과가 pinned Chrome과 다릅니다.`);
    assert.deepEqual(image.getPixel(x, y), rgba, `${expected.id}: PNG와 로그 RGBA가 다릅니다.`);
    if (markerPrefix === 'C1131') {
      assert.deepEqual(actual.expected?.split(',').map((value) => exactInteger(value,
        `${expected.id} expected RGBA`, 0)), expected.rgba,
      `${expected.id}: runtime의 expected RGBA가 고정 fixture와 다릅니다.`);
      assert.equal(actual.exact, 'true', `${expected.id}: runtime exact marker가 true가 아닙니다.`);
    }
    return { id: expected.id, css: [cssX, cssY], pixel: [x, y], rgba };
  });

  return { status: 'matched', scene: scene.id, runID, processId: pid,
    markerPrefix,
    surfaceGeneration: generation, density, surface: [width, height],
    runtimeViewportCssPx: [viewportWidth, viewportHeight], scale: [scaleX, scaleY],
    expectedSamples: scene.samples.length, matchedSamples: checkedSamples.length,
    maximumChannelError: 0, pngSha256, samples: checkedSamples };
}
