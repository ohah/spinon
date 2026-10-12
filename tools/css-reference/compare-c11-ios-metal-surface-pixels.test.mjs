import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import {
  compareC11IOSMetalPixelLogs,
  validateC11IOSChromeOracle,
} from './compare-c11-ios-metal-surface-pixels.mjs';

const fixture = JSON.parse(readFileSync('tests/fixtures/css/c11/metal-surface-pixel-samples.json', 'utf8'));
const referenceBytes = readFileSync(fixture.oracle.referencePath);
const screenshotBytes = readFileSync(fixture.oracle.screenshotPath);
const runID = 'c11-ios-metal-20261012-fixed';
const common = {
  run_id: runID,
  pid: '50123',
  surface_generation: '4',
  document_revision: '1',
  render_tree_revision: '1',
  environment_revision: '1',
};

function devicePoint(sample) {
  return {
    x: Math.floor((sample.xCssPx + 0.5) * 3),
    y: Math.floor((sample.yCssPx + 0.5) * 3),
  };
}

function rgba(value) { return value.join(','); }

function validLog() {
  const begin = {
    ...common,
    scene: fixture.fixtureId,
    viewport: '320x240',
    drawable: '960x720',
  };
  const samples = fixture.samples.map((sample) => {
    const point = devicePoint(sample);
    return `SPINON_C111_METAL_PIXEL_SAMPLE run_id=${runID} pid=50123 surface_generation=4 sample=${sample.id} x=${point.x} y=${point.y} rgba=${rgba(sample.rgba)} expected=${rgba(sample.rgba)} exact=true`;
  });
  const summary = {
    ...common,
    drawable: '960x720',
    scale_x: '3',
    scale_y: '3',
    status: 'matched',
    expected_samples: '7',
    observed_samples: '7',
    matched_samples: '7',
  };
  return [
    `SPINON_C111_METAL_PIXEL_BEGIN ${Object.entries(begin).map(([key, value]) => `${key}=${value}`).join(' ')}`,
    ...samples,
    `SPINON_C111_METAL_PIXEL_SUMMARY ${Object.entries(summary).map(([key, value]) => `${key}=${value}`).join(' ')}`,
  ].join('\n');
}

function mustReject(log) {
  assert.throws(() => compareC11IOSMetalPixelLogs(log, { fixture, runID }));
}

test('fixture pins the exact Chrome 154 DPR 1 reference screenshot and seven extracted pixels', () => {
  const result = validateC11IOSChromeOracle({ fixture, referenceBytes, screenshotBytes });
  assert.deepEqual(result, {
    referenceId: fixture.oracle.referenceId,
    screenshot: [360, 800],
    sampleCount: 7,
  });
});

test('accepts one current-process drawable run with all exact sample colors', () => {
  const result = compareC11IOSMetalPixelLogs(validLog(), { fixture, runID });
  assert.equal(result.status, 'matched');
  assert.equal(result.matchedSamples, 7);
  assert.equal(result.maximumChannelError, 0);
});

test('C11.2.1 runtime scene alias is distinct from its inventory fixture id', () => {
  const log = validLog().replace(`scene=${fixture.fixtureId}`, 'scene=numeric-lines');
  const placementFixture = { ...fixture, sceneId: 'numeric-lines' };
  const result = compareC11IOSMetalPixelLogs(log, { fixture: placementFixture, runID });
  assert.equal(result.status, 'matched');
});

test('accepts the valid initial environment revision zero', () => {
  const log = validLog().replaceAll('environment_revision=1', 'environment_revision=0');
  assert.equal(compareC11IOSMetalPixelLogs(log, { fixture, runID }).status, 'matched');
});
test('isolates one run when the unified log contains other process runs', () => {
  const otherRunID = 'c11-ios-metal-20261012-other';
  const interleaved = `${validLog()}\n${validLog().replaceAll(runID, otherRunID)}`;
  assert.equal(compareC11IOSMetalPixelLogs(interleaved, { fixture, runID }).status, 'matched');
});

test('rejects wrong run id', () => mustReject(validLog().replaceAll(`run_id=${runID}`, 'run_id=stale-run-id')));
test('rejects missing begin marker', () => mustReject(validLog().replace(/SPINON_C111_METAL_PIXEL_BEGIN .*\n/, '')));
test('rejects duplicate begin marker', () => {
  const log = validLog();
  mustReject(log.replace(/SPINON_C111_METAL_PIXEL_BEGIN .*\n/, (line) => `${line}${line}`));
});
test('rejects missing summary marker', () => mustReject(validLog().replace(/SPINON_C111_METAL_PIXEL_SUMMARY .*$/, '')));
test('rejects duplicate summary marker', () => {
  const log = validLog();
  mustReject(`${log}\n${log.split('\n').at(-1)}`);
});
test('rejects missing sample', () => mustReject(validLog().replace(/SPINON_C111_METAL_PIXEL_SAMPLE .*\n/, '')));
test('rejects duplicate sample ID', () => {
  const lines = validLog().split('\n');
  mustReject([...lines, lines[1]].join('\n'));
});
test('rejects unknown sample ID', () => mustReject(validLog().replace('sample=item-blue', 'sample=unknown')));
test('rejects a different application process', () => mustReject(validLog().replace('sample=item-blue x=', 'sample=item-blue pid=50124 x=')));
test('rejects sample from another surface generation', () => mustReject(validLog().replace('surface_generation=4 sample=item-blue', 'surface_generation=5 sample=item-blue')));
test('rejects mismatched begin and summary revisions', () => mustReject(validLog().replace('document_revision=1 render_tree_revision=1', 'document_revision=2 render_tree_revision=1')));
test('rejects nonpositive revision', () => mustReject(validLog().replace('render_tree_revision=1', 'render_tree_revision=0')));
test('rejects unexpected scene', () => mustReject(validLog().replace('scene=fixed-tracks-gap', 'scene=percentage-tracks')));
test('rejects Chrome-sized runtime viewport in place of the app draw viewport', () => mustReject(validLog().replace('viewport=320x240', 'viewport=360x800')));
test('rejects drawable aspect ratio mismatch', () => mustReject(validLog().replaceAll('960x720', '960x600')));
test('rejects wrong horizontal pixel scale', () => mustReject(validLog().replace('scale_x=3', 'scale_x=2')));
test('rejects wrong vertical pixel scale', () => mustReject(validLog().replace('scale_y=3', 'scale_y=4')));
test('rejects off-by-one pixel coordinate', () => mustReject(validLog().replace('x=61 y=31', 'x=62 y=31')));
test('rejects changed Metal surface color', () => mustReject(validLog().replace('rgba=40,102,246,255', 'rgba=41,102,246,255')));
test('rejects changed expected color in runtime log', () => mustReject(validLog().replace('expected=40,102,246,255', 'expected=40,103,246,255')));
test('rejects non-exact sample marker', () => mustReject(validLog().replace('exact=true', 'exact=false')));
test('rejects mismatch summary status', () => mustReject(validLog().replace('status=matched', 'status=mismatch')));
test('rejects inconsistent summary sample counts', () => mustReject(validLog().replace('matched_samples=7', 'matched_samples=6')));
test('rejects malformed RGBA channels', () => mustReject(validLog().replace('rgba=40,102,246,255', 'rgba=40,102,999,255')));
test('rejects altered Chrome reference bytes', () => assert.throws(() => validateC11IOSChromeOracle({
  fixture, referenceBytes: Buffer.from('wrong'), screenshotBytes,
})));
test('rejects altered Chrome screenshot bytes', () => assert.throws(() => validateC11IOSChromeOracle({
  fixture, referenceBytes, screenshotBytes: Buffer.from('wrong'),
})));
