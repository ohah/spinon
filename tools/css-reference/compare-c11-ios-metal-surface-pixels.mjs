import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';

import { decodeRgbaPng } from './read-png-rgba.mjs';

const BEGIN = 'SPINON_C111_METAL_PIXEL_BEGIN ';
const SAMPLE = 'SPINON_C111_METAL_PIXEL_SAMPLE ';
const SUMMARY = 'SPINON_C111_METAL_PIXEL_SUMMARY ';

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function fieldsAfter(line, marker) {
  const index = line.indexOf(marker);
  assert.notEqual(index, -1, `로그 marker가 없습니다: ${marker.trim()}`);
  return Object.fromEntries(
    [...line.slice(index + marker.length).matchAll(/([a-z_]+)=([^\s]+)/g)]
      .map(([, key, value]) => [key, value]),
  );
}

function pixel(value) {
  const components = value.split(',').map(Number);
  assert.equal(components.length, 4, `RGBA 값이 잘못되었습니다: ${value}`);
  assert.ok(components.every((component) => Number.isInteger(component) && component >= 0 && component <= 255),
    `RGBA 채널 범위가 잘못되었습니다: ${value}`);
  return components;
}

function expectedDevicePoint(sample, fixture, drawable) {
  const viewport = fixture.runtime.viewportCssPx;
  const scaleX = drawable.width / viewport.width;
  const scaleY = drawable.height / viewport.height;
  assert.ok(Number.isFinite(scaleX) && scaleX > 0, 'drawable x scale이 잘못되었습니다.');
  assert.ok(Number.isFinite(scaleY) && scaleY > 0, 'drawable y scale이 잘못되었습니다.');
  assert.ok(Math.abs(scaleX - scaleY) < 1e-9, 'drawable x/y scale이 일치하지 않습니다.');
  const x = Math.floor((sample.xCssPx + 0.5) * scaleX);
  const y = Math.floor((sample.yCssPx + 0.5) * scaleY);
  assert.ok(x < drawable.width && y < drawable.height, `${sample.id}: drawable 표면 밖의 좌표입니다.`);
  return {
    x,
    y,
    scaleX,
    scaleY,
  };
}

export function validateC11IOSChromeOracle({ fixture, referenceBytes, screenshotBytes }) {
  assert.equal(sha256(referenceBytes), fixture.oracle.referenceSha256,
    'Chrome reference JSON hash가 fixture와 다릅니다.');
  assert.equal(sha256(screenshotBytes), fixture.oracle.screenshotSha256,
    'Chrome DPR 1 screenshot hash가 fixture와 다릅니다.');
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  assert.equal(reference.referenceId, fixture.oracle.referenceId, 'Chrome reference ID가 다릅니다.');
  assert.equal(reference.oracle.product, fixture.oracle.product, 'Chrome 버전이 다릅니다.');
  const observation = reference.observations.find(({ deviceScaleFactor }) =>
    deviceScaleFactor === fixture.oracle.deviceScaleFactor);
  assert.ok(observation, '고정 DPR의 Chrome 표본이 없습니다.');
  assert.equal(reference.environment.viewportCssPx.width, fixture.oracle.viewportCssPx.width, 'Chrome viewport 너비가 다릅니다.');
  assert.equal(reference.environment.viewportCssPx.height, fixture.oracle.viewportCssPx.height, 'Chrome viewport 높이가 다릅니다.');

  const image = decodeRgbaPng(screenshotBytes);
  assert.equal(image.width, fixture.oracle.viewportCssPx.width, 'Chrome PNG 너비가 viewport와 다릅니다.');
  assert.equal(image.height, fixture.oracle.viewportCssPx.height, 'Chrome PNG 높이가 viewport와 다릅니다.');
  for (const sample of fixture.samples) {
    assert.deepEqual(image.getPixel(sample.xCssPx, sample.yCssPx), sample.rgba,
      `${sample.id}: fixture 색이 고정 Chrome PNG와 다릅니다.`);
  }
  return { referenceId: reference.referenceId, screenshot: [image.width, image.height], sampleCount: fixture.samples.length };
}

export function compareC11IOSMetalPixelLogs(log, { fixture, runID }) {
  assert.equal(typeof runID, 'string');
  assert.match(runID, /^[A-Za-z0-9_-]{8,128}$/, 'run ID 형식이 잘못되었습니다.');
  const lines = log.split(/\r?\n/);
  const belongsToRun = (line, marker) => line.includes(marker)
    && fieldsAfter(line, marker).run_id === runID;
  const begins = lines.filter((line) => belongsToRun(line, BEGIN));
  const sampleLines = lines.filter((line) => belongsToRun(line, SAMPLE));
  const summaries = lines.filter((line) => belongsToRun(line, SUMMARY));
  assert.equal(begins.length, 1, '같은 실행의 begin marker가 정확히 하나여야 합니다.');
  assert.equal(summaries.length, 1, '같은 실행의 summary marker가 정확히 하나여야 합니다.');
  assert.equal(sampleLines.length, fixture.samples.length, '표본 개수가 고정 fixture와 다릅니다.');

  const begin = fieldsAfter(begins[0], BEGIN);
  const summary = fieldsAfter(summaries[0], SUMMARY);
  for (const [name, record] of [['begin', begin], ['summary', summary]]) {
    assert.equal(record.run_id, runID, `${name}: run ID가 다릅니다.`);
    assert.ok(Number(record.pid) > 0, `${name}: app PID가 없습니다.`);
    assert.ok(Number(record.surface_generation) > 0, `${name}: surface generation이 없습니다.`);
    assert.ok(Number(record.document_revision) > 0, `${name}: document_revision가 없습니다.`);
    assert.ok(Number(record.render_tree_revision) > 0, `${name}: render_tree_revision가 없습니다.`);
    assert.ok(Number.isInteger(Number(record.environment_revision))
      && Number(record.environment_revision) >= 0, `${name}: environment_revision가 잘못되었습니다.`);
  }
  for (const field of ['pid', 'surface_generation', 'document_revision', 'render_tree_revision', 'environment_revision']) {
    assert.equal(summary[field], begin[field], `begin/summary ${field}가 바뀌었습니다.`);
  }
  assert.equal(begin.scene, fixture.sceneId ?? fixture.fixtureId, '예상하지 않은 Grid scene입니다.');
  assert.equal(begin.viewport, `${fixture.runtime.viewportCssPx.width}x${fixture.runtime.viewportCssPx.height}`,
    'Spinon draw viewport가 다릅니다.');

  const [drawableWidth, drawableHeight] = begin.drawable.split('x').map(Number);
  assert.ok(Number.isInteger(drawableWidth) && drawableWidth > 0, 'drawable 너비가 잘못되었습니다.');
  assert.ok(Number.isInteger(drawableHeight) && drawableHeight > 0, 'drawable 높이가 잘못되었습니다.');
  assert.equal(summary.drawable, `${drawableWidth}x${drawableHeight}`, 'begin/summary drawable이 다릅니다.');
  const drawable = { width: drawableWidth, height: drawableHeight };
  const firstExpected = expectedDevicePoint(fixture.samples[0], fixture, drawable);
  assert.equal(Number(summary.scale_x), firstExpected.scaleX, 'drawable x scale이 잘못 기록되었습니다.');
  assert.equal(Number(summary.scale_y), firstExpected.scaleY, 'drawable y scale이 잘못 기록되었습니다.');
  assert.equal(summary.status, 'matched', 'Metal surface summary가 matched가 아닙니다.');
  assert.equal(Number(summary.expected_samples), fixture.samples.length, 'summary 기대 표본 수가 다릅니다.');
  assert.equal(Number(summary.observed_samples), fixture.samples.length, 'summary 관측 표본 수가 다릅니다.');
  assert.equal(Number(summary.matched_samples), fixture.samples.length, 'summary 통과 표본 수가 다릅니다.');

  const seen = new Map();
  const records = sampleLines.map((line) => fieldsAfter(line, SAMPLE));
  for (const record of records) {
    assert.equal(record.run_id, runID, '다른 실행의 표본이 섞였습니다.');
    assert.equal(record.pid, begin.pid, '다른 앱 process의 표본이 섞였습니다.');
    assert.equal(record.surface_generation, begin.surface_generation, '다른 surface generation 표본입니다.');
    assert.ok(!seen.has(record.sample), `중복 표본입니다: ${record.sample}`);
    seen.set(record.sample, record);
  }
  assert.deepEqual([...seen.keys()].sort(), fixture.samples.map(({ id }) => id).sort(),
    '고정 sample ID 집합이 다릅니다.');

  const verified = fixture.samples.map((sample) => {
    const record = seen.get(sample.id);
    const point = expectedDevicePoint(sample, fixture, drawable);
    const actual = pixel(record.rgba);
    const expected = sample.rgba;
    assert.equal(Number(record.x), point.x, `${sample.id}: drawable x가 다릅니다.`);
    assert.equal(Number(record.y), point.y, `${sample.id}: drawable y가 다릅니다.`);
    assert.deepEqual(pixel(record.expected), expected, `${sample.id}: 로그 기대 RGBA가 다릅니다.`);
    assert.deepEqual(actual, expected, `${sample.id}: Metal surface 실제 RGBA가 Chrome oracle과 다릅니다.`);
    assert.equal(record.exact, 'true', `${sample.id}: 표본 exact marker가 아닙니다.`);
    return {
      id: sample.id,
      css: [sample.xCssPx, sample.yCssPx],
      drawable: [point.x, point.y],
      rgba: actual,
      maximumChannelError: Math.max(...actual.map((channel, index) => Math.abs(channel - expected[index]))),
    };
  });

  return {
    schema: 'spinon-c11-ios-metal-surface-pixels/v1',
    status: 'matched',
    runID,
    processId: Number(begin.pid),
    surfaceGeneration: Number(begin.surface_generation),
    revisions: {
      document: Number(begin.document_revision),
      renderTree: Number(begin.render_tree_revision),
      environment: Number(begin.environment_revision),
    },
    runtimeViewportCssPx: [fixture.runtime.viewportCssPx.width, fixture.runtime.viewportCssPx.height],
    drawable: [drawable.width, drawable.height],
    scale: [Number(summary.scale_x), Number(summary.scale_y)],
    expectedSamples: fixture.samples.length,
    matchedSamples: verified.length,
    maximumChannelError: 0,
    samples: verified,
  };
}
