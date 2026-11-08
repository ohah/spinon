import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const expected = JSON.parse(fs.readFileSync(path.join(evidence, 'expected.json'), 'utf8'));
const raw = JSON.parse(fs.readFileSync(path.join(evidence, 'chrome-observation.json'), 'utf8'));
const observed = typeof raw === 'string' ? JSON.parse(raw) : raw;
const epsilon = 1e-6;
const failures = [];
let maxAbsError = 0;

function compareNumber(actual, wanted, field, sceneId) {
  const error = Math.abs(actual - wanted);
  maxAbsError = Math.max(maxAbsError, error);
  if (error > epsilon) failures.push({ sceneId, field, actual, expected: wanted, error });
}

assert.equal(expected.orderCount, 30);
assert.equal(expected.sceneCount, 120);
assert.equal(observed.length, expected.sceneCount);
const observedIds = new Set();

for (let index = 0; index < expected.scenes.length; index += 1) {
  const wanted = expected.scenes[index];
  const actual = observed[index];
  assert.equal(actual.id, wanted.id, `scene order mismatch at ${index}`);
  assert.ok(!observedIds.has(actual.id), `duplicate observation: ${actual.id}`);
  observedIds.add(actual.id);
  assert.equal(actual.alignContent, wanted.alignContent, `${wanted.id}: alignment`);
  assert.equal(actual.gridAutoRows, wanted.serializedTrackList, `${wanted.id}: computed track order`);
  compareNumber(actual.grid.width, 40, 'grid.width', wanted.id);
  compareNumber(actual.grid.height, 100, 'grid.height', wanted.id);
  assert.deepEqual(actual.gridTemplateRows.length, wanted.rows.length, `${wanted.id}: used row count`);
  wanted.rows.forEach((size, row) => compareNumber(actual.gridTemplateRows[row], size, `rows[${row}]`, wanted.id));
  assert.equal(actual.items.length, wanted.items.length, `${wanted.id}: item count`);
  wanted.items.forEach((box, itemIndex) => {
    const boxActual = actual.items[itemIndex];
    assert.equal(boxActual.index, box.index, `${wanted.id}: item index ${itemIndex}`);
    for (const key of ['x', 'y', 'width', 'height']) {
      compareNumber(boxActual[key], box[key], `items[${itemIndex}].${key}`, wanted.id);
    }
  });
  assert.equal(actual.payloads.length, wanted.payloads.length, `${wanted.id}: payload count`);
  wanted.payloads.forEach((box, payloadIndex) => {
    const boxActual = actual.payloads[payloadIndex];
    assert.equal(boxActual.index, box.index, `${wanted.id}: payload index ${payloadIndex}`);
    for (const key of ['x', 'y', 'width', 'height']) {
      compareNumber(boxActual[key], box[key], `payloads[${payloadIndex}].${key}`, wanted.id);
    }
  });
  compareNumber(
    actual.payloads[6].y + actual.payloads[6].height - 100,
    wanted.fitContentPayloadOverflowPastGrid,
    'fit-content overflow past grid border',
    wanted.id,
  );
  compareNumber(actual.gridTemplateRows.reduce((sum, size) => sum + size, 0), wanted.totalRows, 'track sum', wanted.id);
}

assert.equal(observedIds.size, 120);
assert.deepEqual(failures, [], `Chrome differs from independent expected geometry: ${JSON.stringify(failures.slice(0, 8))}`);
const report = {
  browser: 'Agent Browser HeadlessChrome 157.0.0.0 / macOS arm64',
  viewport: '1280x633 CSS px, DPR 1',
  uniqueOrderCount: expected.orderCount,
  sceneCount: expected.sceneCount,
  scenesCompared: observed.length,
  coordinateAndTrackChecks: observed.length * (expected.scenes[0].rows.length + expected.scenes[0].items.length * 4 + expected.scenes[0].payloads.length * 4 + 2),
  passed: failures.length === 0,
  failures,
  maxAbsError,
  toleranceCssPx: epsilon,
  capForms: ['20px', '20%'],
  alignmentValues: ['start', 'stretch'],
  computedTrackOrderPreserved: true,
  fitContentOverflowCheckedInAllScenes: true,
  sourceIsIndependentFixedPayloadModel: expected.source,
};
fs.writeFileSync(path.join(evidence, 'chrome-verification.json'), `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
