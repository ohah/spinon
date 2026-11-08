import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

const here = dirname(fileURLToPath(import.meta.url));
const repository = resolve(here, '../../../..');
const fixturePath = resolve(
  repository,
  'tests/fixtures/css/c11/root-html-grid-fit-min-max-content-auto-implicit-row-list-precomparison.html',
);
const observationPath = resolve(here, 'chrome-observation.json');
const fixtureBytes = readFileSync(fixturePath);
const observation = JSON.parse(readFileSync(observationPath, 'utf8'));
const members = ['fit', 'min', 'max', 'auto'];
const itemHeights = [20, 10, 12, 6, 20];
const expectedBaseRows = [10, ...itemHeights];
const epsilon = 0.00001;

function permutations(values) {
  if (values.length < 2) return [values];
  return values.flatMap((value, index) =>
    permutations(values.filter((_, other) => other !== index))
      .map((tail) => [value, ...tail]));
}

function close(actual, expected, label) {
  assert.ok(
    Number.isFinite(actual) && Math.abs(actual - expected) <= epsilon,
    `${label}: expected ${expected}, observed ${actual}`,
  );
}

function expectedTrackList(order, capName) {
  const fitContent = `fit-content(${capName === 'px' ? '20px' : '20%'})`;
  const value = { fit: fitContent, min: 'min-content', max: 'max-content', auto: 'auto' };
  return order.map((member) => value[member]).join(' ');
}

function validateCase(entry) {
  const match = entry.name.match(/^((?:fit|min|max|auto)-(?:fit|min|max|auto)-(?:fit|min|max|auto)-(?:fit|min|max|auto))-(px|pct)-(start|stretch)$/);
  assert.ok(match, `unrecognized scene name: ${entry.name}`);
  const order = match[1].split('-');
  const capName = match[2];
  const alignment = match[3];
  assert.deepEqual([...order].sort(), [...members].sort(), `${entry.name}: unique member order`);

  const authored = expectedTrackList(order, capName);
  assert.equal(entry.authoredRows, authored, `${entry.name}: authored track list`);
  assert.equal(entry.computedRows, authored, `${entry.name}: computed track list`);
  assert.equal(entry.alignContent, alignment, `${entry.name}: align-content computed value`);
  assert.equal(entry.computedAlignContent, alignment, `${entry.name}: align-content observation`);
  assert.equal(entry.usedRows.length, 6, `${entry.name}: explicit plus five implicit rows`);
  assert.equal(entry.items.length, 5, `${entry.name}: five implicit-row items`);
  assert.equal(entry.payloads.length, 5, `${entry.name}: five fixed-size payloads`);
  assert.equal(entry.horizontalOverflow, false, `${entry.name}: no horizontal overflow`);
  close(entry.grid.width, 40, `${entry.name}: grid width`);
  close(entry.grid.height, 100, `${entry.name}: grid height`);
  close(entry.explicit.width, 40, `${entry.name}: explicit item width`);
  close(entry.explicit.height, 10, `${entry.name}: explicit item height`);
  close(entry.explicit.y, entry.grid.y, `${entry.name}: explicit item top`);

  const expectedRows = [...expectedBaseRows];
  if (alignment === 'stretch') {
    // The implicit list repeats from its first entry. If that entry is `auto`,
    // the fifth implicit row is a second auto track and shares free space equally.
    const autoIndexes = order
      .map((member, index) => member === 'auto' ? index + 1 : null)
      .filter((index) => index !== null);
    if (order[0] === 'auto') autoIndexes.push(5);
    assert.ok(autoIndexes.length > 0, `${entry.name}: auto track exists`);
    const freeSpace = 100 - expectedRows.reduce((sum, size) => sum + size, 0);
    assert.ok(freeSpace > 0, `${entry.name}: positive definite free space`);
    const share = freeSpace / autoIndexes.length;
    for (const index of autoIndexes) expectedRows[index] += share;
  }

  for (let index = 0; index < expectedRows.length; index += 1) {
    close(entry.usedRows[index], expectedRows[index], `${entry.name}: used row ${index}`);
  }
  close(entry.usedRows[1], entry.usedRows[5], `${entry.name}: first list value cycles at implicit row five`);

  let y = entry.grid.y + expectedRows[0];
  for (let index = 0; index < entry.items.length; index += 1) {
    const item = entry.items[index];
    const payload = entry.payloads[index];
    close(item.x, entry.grid.x, `${entry.name}: item ${index + 1} x`);
    close(item.y, y, `${entry.name}: item ${index + 1} y`);
    close(item.width, 40, `${entry.name}: item ${index + 1} width`);
    close(item.height, expectedRows[index + 1], `${entry.name}: item ${index + 1} height`);
    close(payload.x, entry.grid.x, `${entry.name}: payload ${index + 1} x`);
    close(payload.y, y, `${entry.name}: payload ${index + 1} y`);
    close(payload.width, 40, `${entry.name}: payload ${index + 1} width`);
    close(payload.height, itemHeights[index], `${entry.name}: payload ${index + 1} height`);
    y += expectedRows[index + 1];
  }

  return { name: entry.name, usedRows: expectedRows };
}

const observedCases = observation.cases;
assert.equal(observedCases.length, 96, '24 permutations × 2 cap forms × 2 alignments');
assert.equal(new Set(observedCases.map((entry) => entry.name)).size, 96, 'unique scene names');
assert.match(observation.userAgent, /HeadlessChrome\/157\./, 'pinned browser major version');
assert.equal(observation.platform, 'MacIntel', 'Chrome host platform');
assert.deepEqual(observation.viewport, { width: 1280, height: 633, dpr: 1 }, 'viewport and DPR');

const expectedOrderNames = new Set(permutations(members).map((order) => order.join('-')));
const observedOrderNames = new Set(observedCases.map((entry) => entry.name.replace(/-(?:px|pct)-(?:start|stretch)$/, '')));
assert.deepEqual(observedOrderNames, expectedOrderNames, 'all 24 distinct orders are represented');

const verified = observedCases.map(validateCase);
const byName = new Map(observedCases.map((entry) => [entry.name, entry]));
for (const entry of observedCases) {
  const name = entry.name.replace(/-(px|pct)-(start|stretch)$/, '');
  const [, capName, alignment] = entry.name.match(/-(px|pct)-(start|stretch)$/);
  const equivalentCap = byName.get(`${name}-${capName === 'px' ? 'pct' : 'px'}-${alignment}`);
  assert.deepEqual(entry.usedRows, equivalentCap.usedRows, `${entry.name}: 20px and 20% share the definite 100px basis`);
  if (alignment === 'stretch') {
    const start = byName.get(`${name}-${capName}-start`);
    const autoIndexes = entry.name
      .replace(/-(px|pct)-stretch$/, '')
      .split('-')
      .map((member, index) => member === 'auto' ? index + 1 : null)
      .filter((index) => index !== null);
    if (entry.name.startsWith('auto-')) autoIndexes.push(5);
    const expectedShare = 22 / autoIndexes.length;
    for (let index = 1; index < entry.usedRows.length; index += 1) {
      const delta = entry.usedRows[index] - start.usedRows[index];
      if (autoIndexes.includes(index)) close(delta, expectedShare, `${entry.name}: auto-only stretch delta ${index}`);
      else close(delta, 0, `${entry.name}: non-auto track unchanged by stretch ${index}`);
    }
  }
}

const fixtureSha256 = createHash('sha256').update(fixtureBytes).digest('hex');
const result = {
  status: 'PASS',
  browser: observation.userAgent,
  platform: observation.platform,
  viewport: observation.viewport,
  fixture: 'tests/fixtures/css/c11/root-html-grid-fit-min-max-content-auto-implicit-row-list-precomparison.html',
  fixtureSha256,
  sceneCount: verified.length,
  orderCount: expectedOrderNames.size,
  capForms: ['20px', '20%'],
  alignments: ['start', 'stretch'],
  expectation: {
    rowsWithStart: expectedBaseRows,
    totalStartUsed: expectedBaseRows.reduce((sum, size) => sum + size, 0),
    remainingDefiniteFreeSpace: 100 - expectedBaseRows.reduce((sum, size) => sum + size, 0),
    stretchedTrackRule: 'Only implicit auto max-sizing tracks share positive free space equally; the fifth implicit row repeats the first list member.',
    minMaxGeometryLimit: 'These fixed-height payloads have equal min-content and max-content contributions; the computed CSS list is checked to preserve each distinct track function.',
  },
  allCasesMatched: true,
};
writeFileSync(resolve(here, 'oracle-verification.json'), `${JSON.stringify(result, null, 2)}\n`);
console.log(JSON.stringify(result, null, 2));
