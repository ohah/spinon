import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { decodeRgbaPng } from './read-png-rgba.mjs';

const scriptDirectory = path.dirname(fileURLToPath(import.meta.url));
export const repositoryRoot = path.resolve(scriptDirectory, '../..');
const planEvidence = path.join(
  repositoryRoot,
  'spec/internal/evidence/c13-2-visible-overflow-surface-pixels-plan-2026-10-15',
);
const runEvidence = path.join(
  repositoryRoot,
  'spec/internal/evidence/c13-2-visible-overflow-surface-pixels-2026-10-15',
);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

export const C132_SURFACE_SAMPLES = [
  { id: 'visible-escape', cssX: 150, cssY: 30, rgba: [227, 84, 78, 255] },
  { id: 'parent-gap', cssX: 136, cssY: 30, rgba: [16, 24, 39, 255] },
  { id: 'parent-interior', cssX: 20, cssY: 20, rgba: [52, 66, 86, 255] },
  { id: 'parent-border', cssX: 11, cssY: 11, rgba: [240, 198, 116, 255] },
  { id: 'root-background', cssX: 50, cssY: 110, rgba: [16, 24, 39, 255] },
];

const markers = {
  android: {
    begin: 'SPINON_C132_PIXELCOPY_BEGIN ',
    sample: 'SPINON_C132_PIXEL ',
    summary: 'SPINON_C132_PIXELCOPY_SUMMARY ',
    surfaceKey: 'surface',
    xKey: 'x_px',
    yKey: 'y_px',
  },
  ios: {
    begin: 'SPINON_C132_METAL_PIXEL_REQUESTED ',
    sample: 'SPINON_C132_METAL_PIXEL_SAMPLE ',
    summary: 'SPINON_C132_METAL_PIXEL_SUMMARY ',
    surfaceKey: 'drawable',
    xKey: 'x',
    yKey: 'y',
  },
};

function markerFields(line, marker) {
  const markerIndex = line.indexOf(marker);
  assert.ok(markerIndex >= 0, `로그 marker가 없습니다: ${marker.trim()}`);
  const fields = {};
  const fieldPattern = /(?:^|\s)([a-z][a-z0-9_]*)=([^\s]+)/g;
  const text = line.slice(markerIndex + marker.length);
  for (const match of text.matchAll(fieldPattern)) fields[match[1]] = match[2];
  return fields;
}

function numericList(value, label) {
  assert.equal(typeof value, 'string', `${label}이 없습니다.`);
  const values = value.split(',').map(Number);
  assert.ok(values.length > 0 && values.every(Number.isSafeInteger), `${label} 형식이 잘못됐습니다.`);
  return values;
}

function dimensions(value, label) {
  const match = /^(\d+)x(\d+)$/.exec(value ?? '');
  assert.ok(match, `${label} 크기 형식이 잘못됐습니다: ${value}`);
  return [Number(match[1]), Number(match[2])];
}

export function parseC132SurfacePixelLog(platform, logText) {
  assert.ok(Object.hasOwn(markers, platform), `알 수 없는 platform입니다: ${platform}`);
  const config = markers[platform];
  const lines = logText.split(/\r?\n/);
  const findOne = (marker) => {
    const matches = lines.filter((line) => line.includes(marker));
    assert.equal(matches.length, 1, `${marker.trim()} 로그가 정확히 한 줄이어야 합니다.`);
    return markerFields(matches[0], marker);
  };
  const begin = findOne(config.begin);
  const summary = findOne(config.summary);
  const sampleLines = lines.filter((line) => line.includes(config.sample));
  const samples = sampleLines.map((line) => {
    const fields = markerFields(line, config.sample);
    return {
      id: fields.sample,
      runId: fields.run_id,
      pid: Number(fields.pid),
      surfaceGeneration: Number(fields.surface_generation),
      x: Number(fields[config.xKey]),
      y: Number(fields[config.yKey]),
      rgba: numericList(fields.rgba, 'rgba'),
      exact: fields.exact,
    };
  });
  const drawLine = lines.filter((line) => line.includes('SPINON_C0410_DRAW ')
    && line.includes('boxes=26'));
  assert.ok(drawLine.length > 0, '26-box draw 로그가 없습니다.');
  const draw = markerFields(drawLine.at(-1), 'SPINON_C0410_DRAW ');
  if (platform === 'ios') {
    assert.ok(draw.surface_pixel_samples_json, 'iOS final draw에 Metal 표본 JSON이 없습니다.');
    draw.surfacePixelSamples = JSON.parse(draw.surface_pixel_samples_json);
  }
  const assertions = lines.filter((line) => line.includes('SPINON_C132_SCROLL_ASSERTIONS '));
  assert.equal(assertions.length, 1, 'C13.2 scroll assertion 로그가 정확히 한 줄이어야 합니다.');
  return { platform, begin, summary, samples, draw, assertions: markerFields(
    assertions[0], 'SPINON_C132_SCROLL_ASSERTIONS ',
  ) };
}

export function validateC132ChromeOracle({ observation, screenshotBytes, fixtureBytes, sourceFixtureBytes }) {
  assert.equal(observation.schema, 'spinon-c13-2-visible-border-pixel-probe/v1');
  assert.equal(observation.browser.version, '155.0.8059.40');
  assert.equal(observation.viewport.width_css_px, 400);
  assert.equal(observation.viewport.height_css_px, 240);
  assert.equal(observation.viewport.device_pixel_ratio, 1);
  assert.equal(observation.fixture_sha256, sha256(fixtureBytes));
  assert.equal(observation.source_fixture_sha256, sha256(sourceFixtureBytes));
  assert.equal(observation.screenshot.sha256, sha256(screenshotBytes));
  const image = decodeRgbaPng(screenshotBytes);
  assert.deepEqual([image.width, image.height], [400, 240]);
  assert.equal(observation.screenshot.width, image.width);
  assert.equal(observation.screenshot.height, image.height);
  assert.deepEqual(observation.geometry_css_px.visible_parent_border_box.right, 130);
  assert.deepEqual(observation.samples.map((sample) => sample.css_cell),
    C132_SURFACE_SAMPLES.map(({ cssX, cssY }) => [cssX, cssY]));
  for (const [index, expected] of C132_SURFACE_SAMPLES.entries()) {
    const sample = observation.samples[index];
    assert.deepEqual(sample.expected_rgba, expected.rgba, `${expected.id} Chrome RGBA`);
    assert.deepEqual(image.getPixel(expected.cssX, expected.cssY), expected.rgba,
      `${expected.id} Chrome PNG RGBA`);
  }
  assert.equal(observation.samples[0].outside_parent_right, true);
  assert.equal(observation.samples[1].element, 'c132-root');
  return {
    browser: observation.browser.version,
    viewport: [image.width, image.height],
    fixtureSha256: observation.fixture_sha256,
    sourceFixtureSha256: observation.source_fixture_sha256,
    screenshotSha256: sha256(screenshotBytes),
  };
}

export function decodeC132SurfacePng(bytes) {
  const decoded = decodeRgbaPng(bytes);
  return { width: decoded.width, height: decoded.height, sha256: sha256(bytes), getPixel: decoded.getPixel };
}

function assertSame(actual, expected, label) {
  assert.equal(actual, expected, `${label}: ${actual} != ${expected}`);
}

function assertRevisionFields(capture, expectedRunId) {
  const { platform, begin, summary, draw, assertions } = capture;
  const layoutKey = platform === 'android' ? 'draw_generation' : 'layout_generation';
  const beginIdentity = ['run_id', 'pid', 'surface_generation', 'presentation_sequence'];
  const requiredSummary = [
    ...beginIdentity,
    'boxes',
    'document_revision', 'render_tree_revision', 'style_revision', 'environment_revision',
    'scroll_state_revision',
  ];
  for (const key of requiredSummary) {
    assert.ok(summary[key] !== undefined,
      `${platform} ${key} revision/identity가 없습니다.`);
  }
  for (const key of beginIdentity) {
    assert.ok(begin[key] !== undefined, `${platform} begin ${key}가 없습니다.`);
    assertSame(summary[key], begin[key], `${platform} begin/summary ${key}`);
  }
  assertSame(begin.run_id, expectedRunId, `${platform} run ID`);
  assertSame(summary.boxes, '26', `${platform} summary box count`);
  assertSame(begin.viewport, '400x240', `${platform} viewport`);
  assertSame(summary.viewport, '400x240', `${platform} summary viewport`);
  assertSame(summary.status, 'matched', `${platform} result`);
  assertSame(summary.expected_samples, '5', `${platform} expected sample count`);
  assertSame(summary.observed_samples, '5', `${platform} observed sample count`);
  assertSame(summary.matched_samples, '5', `${platform} matched sample count`);
  assertSame(summary.escape_outside_parent, 'true', `${platform} escaped sample geometry`);
  assert.ok(Number(begin.pid) > 0 && Number(summary.surface_generation) > 0,
    `${platform} process/surface generation must be positive.`);
  assert.ok(Number(summary.presentation_sequence) >= 0,
    `${platform} presentation sequence must be nonnegative.`);
  assert.ok(Number(summary[layoutKey]) > 0,
    `${platform} draw generation must be positive.`);
  assertSame(assertions.border_escape_outside_parent, 'true', `${platform} scroll fixture escape`);
  assertSame(assertions.border_escape_hit, 'matched', `${platform} scroll fixture hit target`);
  for (const [key, drawKey] of [
    ['document_revision', 'document_revision'],
    ['render_tree_revision', 'render_tree_revision'],
    ['style_revision', 'style_revision'],
    ['environment_revision', 'environment_revision'],
    ['scroll_state_revision', 'scroll_state_revision'],
  ]) assertSame(summary[key], draw[drawKey], `${platform} final draw ${key}`);
  assertSame(summary[layoutKey], draw.generation, `${platform} final draw generation`);
}

export function compareC132SurfacePixelCapture(capture, { expectedRunId, imageInfo } = {}) {
  assert.ok(capture && (capture.platform === 'android' || capture.platform === 'ios'),
    'Android 또는 iOS capture가 필요합니다.');
  assertRevisionFields(capture, expectedRunId);
  const config = markers[capture.platform];
  const widthKey = capture.platform === 'android' ? 'surface' : 'drawable';
  const [width, height] = dimensions(capture.summary[widthKey], `${capture.platform} drawable`);
  const scaleX = width / 400;
  const scaleY = height / 240;
  assert.ok(width > 0 && height > 0 && Math.abs(scaleX - scaleY) < 1e-6,
    `${capture.platform} surface scale가 viewport와 맞지 않습니다.`);
  assertSame(capture.begin[widthKey], capture.summary[widthKey], `${capture.platform} surface 크기`);
  assert.ok(Math.abs(Number(capture.summary.scale_x) - scaleX) < 1e-6,
    `${capture.platform} scale_x 값이 surface와 맞지 않습니다.`);
  assert.ok(Math.abs(Number(capture.summary.scale_y) - scaleY) < 1e-6,
    `${capture.platform} scale_y 값이 surface와 맞지 않습니다.`);
  assert.equal(capture.samples.length, C132_SURFACE_SAMPLES.length,
    `${capture.platform} 고정 표본 개수`);
  const seen = new Set();
  const samples = capture.samples.map((actual, index) => {
    const expected = C132_SURFACE_SAMPLES[index];
    assertSame(actual.id, expected.id, `${capture.platform} sample 순서/ID`);
    assert.equal(seen.has(actual.id), false, `${capture.platform} 중복 sample ${actual.id}`);
    seen.add(actual.id);
    assertSame(actual.runId, expectedRunId, `${capture.platform} ${actual.id} run ID`);
    assertSame(String(actual.pid), capture.summary.pid, `${capture.platform} ${actual.id} PID`);
    assertSame(String(actual.surfaceGeneration), capture.summary.surface_generation,
      `${capture.platform} ${actual.id} surface generation`);
    const x = Math.floor((expected.cssX + 0.5) * scaleX);
    const y = Math.floor((expected.cssY + 0.5) * scaleY);
    assert.deepEqual([actual.x, actual.y], [x, y],
      `${capture.platform} ${actual.id} surface coordinate`);
    assert.ok(x >= 0 && y >= 0 && x < width && y < height,
      `${capture.platform} ${actual.id} sample이 surface 밖입니다.`);
    assert.deepEqual(actual.rgba, expected.rgba, `${capture.platform} ${actual.id} RGBA`);
    assertSame(actual.exact, 'true', `${capture.platform} ${actual.id} exact marker`);
    return { id: actual.id, css: [expected.cssX, expected.cssY], pixel: [x, y], rgba: actual.rgba };
  });
  if (capture.platform === 'ios') {
    assert.equal(capture.draw.surfacePixelSamples.length, samples.length,
      'iOS final draw의 Metal 표본 개수');
    for (const [index, drawSample] of capture.draw.surfacePixelSamples.entries()) {
      assert.deepEqual(
        { x: drawSample.x, y: drawSample.y, rgba: drawSample.rgba },
        { x: samples[index].pixel[0], y: samples[index].pixel[1], rgba: samples[index].rgba },
        `iOS final draw ${samples[index].id} 표본`,
      );
    }
  }
  const escape = samples[0];
  assert.ok(escape.pixel[0] + 0.5 > 130 * scaleX,
    `${capture.platform} visible escape 표본이 parent 오른쪽 border 안쪽입니다.`);

  let png;
  if (capture.platform === 'android') {
    assert.ok(imageInfo, 'Android PixelCopy PNG와 decoded image 정보가 필요합니다.');
    assert.deepEqual([imageInfo.width, imageInfo.height], [width, height],
      'Android PixelCopy PNG 크기가 SurfaceView와 다릅니다.');
    assertSame(capture.summary.sha256, imageInfo.sha256, 'Android PixelCopy PNG SHA-256');
    for (const sample of samples) {
      assert.deepEqual(imageInfo.getPixel(sample.pixel[0], sample.pixel[1]), sample.rgba,
        `Android PNG ${sample.id} 저장 픽셀 RGBA`);
    }
    png = { width, height, sha256: imageInfo.sha256 };
  } else {
    assertSame(capture.summary.renderer, 'metal-drawable-before-compositor',
      'iOS readback provenance');
  }

  return {
    platform: capture.platform,
    runId: expectedRunId,
    pid: Number(capture.summary.pid),
    surfaceGeneration: Number(capture.summary.surface_generation),
    presentationSequence: Number(capture.summary.presentation_sequence),
    layoutGeneration: Number(capture.summary[ capture.platform === 'android'
      ? 'draw_generation' : 'layout_generation']),
    revisions: {
      document: Number(capture.summary.document_revision),
      renderTree: Number(capture.summary.render_tree_revision),
      style: Number(capture.summary.style_revision),
      environment: Number(capture.summary.environment_revision),
      scroll: Number(capture.summary.scroll_state_revision),
    },
    drawable: [width, height],
    scale: [scaleX, scaleY],
    samples,
    escapeOutsideParent: true,
    ...(png ? { png } : { renderer: capture.summary.renderer }),
  };
}

export function compareC132SurfacePixelRuns({ androidCapture, androidImage, iosCapture, expectedRunId }) {
  const android = compareC132SurfacePixelCapture(androidCapture, {
    expectedRunId,
    imageInfo: androidImage,
  });
  const ios = compareC132SurfacePixelCapture(iosCapture, { expectedRunId });
  for (const key of ['layoutGeneration']) {
    assertSame(android[key], ios[key], `Android/iOS ${key}`);
  }
  for (const key of ['document', 'renderTree', 'style', 'scroll']) {
    assertSame(android.revisions[key], ios.revisions[key], `Android/iOS ${key} revision`);
  }
  return {
    status: 'matched',
    runId: expectedRunId,
    oracle: 'Chrome 155.0.8059.40',
    platforms: { android, ios },
  };
}

function loadDefaultEvidence() {
  const observation = JSON.parse(readFileSync(path.join(planEvidence, 'chrome155-observation.json'), 'utf8'));
  const chromeScreenshot = readFileSync(path.join(planEvidence, observation.screenshot.file));
  const fixtureBytes = readFileSync(path.join(repositoryRoot, observation.fixture));
  const sourceFixtureBytes = readFileSync(path.join(repositoryRoot, observation.source_fixture));
  const androidLog = readFileSync(path.join(runEvidence, 'android-pixelcopy-run5.log'), 'utf8');
  const iosLog = readFileSync(path.join(runEvidence, 'ios-metal-run5.log'), 'utf8');
  const androidPng = readFileSync(path.join(runEvidence, 'android-api37-avd-pixelcopy-run5.png'));
  return {
    expectedRunId: 'c132-surface-20261015-run5',
    oracle: validateC132ChromeOracle({
      observation,
      screenshotBytes: chromeScreenshot,
      fixtureBytes,
      sourceFixtureBytes,
    }),
    androidCapture: parseC132SurfacePixelLog('android', androidLog),
    androidImage: decodeC132SurfacePng(androidPng),
    iosCapture: parseC132SurfacePixelLog('ios', iosLog),
  };
}

function main() {
  const [, , outputPath] = process.argv;
  const evidence = loadDefaultEvidence();
  const result = compareC132SurfacePixelRuns(evidence);
  const report = { ...result, oracle: evidence.oracle };
  if (outputPath) writeFileSync(path.resolve(outputPath), `${JSON.stringify(report, null, 2)}\n`);
  console.log(JSON.stringify({
    status: report.status,
    runId: report.runId,
    oracle: report.oracle,
    platforms: Object.fromEntries(Object.entries(report.platforms).map(([name, value]) => [name, {
      pid: value.pid,
      surfaceGeneration: value.surfaceGeneration,
      layoutGeneration: value.layoutGeneration,
      revisions: value.revisions,
      drawable: value.drawable,
      matchedSamples: value.samples.length,
      escapeOutsideParent: value.escapeOutsideParent,
      png: value.png ?? null,
      renderer: value.renderer ?? null,
    }])),
    output: outputPath ? path.relative(repositoryRoot, path.resolve(outputPath)) : null,
  }, null, 2));
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) main();
