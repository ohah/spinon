import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

export const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
export const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
export const inputPaths = {
  html: 'tests/fixtures/css/c13/c13-1-compact-surface-matrix.html',
  script: 'tests/fixtures/css/c13/c13-1-compact-surface-matrix.js',
  chromeObservation: 'spec/internal/evidence/c13-1-compact-surface-matrix-2026-10-14/chrome155-observation.json',
};

const expectedSourceSha256 = {
  html: '431f4b4452969adcae80c4e5f89e5e45b1e8756fc3c4d8995b7a5eccfbc6f355',
  script: 'd36bc1f7d3c791cdc91f6d878c3961e067efc2f88ca1486db8a065c5fd69c760',
  chromeObservation: '515fc95f811399fda9e336a13e7583f2d22555fce4727f1fe7f734981abd0299',
};

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

export function validateHitIdentityGrid(oracle, { root = repositoryRoot } = {}) {
  assert.equal(oracle.status, 'captured', 'Chrome hit grid capture status must be captured');
  assert.deepEqual(oracle.viewport, {
    width: 300,
    height: 100,
    devicePixelRatio: 1,
    scrollX: 0,
    scrollY: 0,
  }, 'Chrome hit grid viewport must remain the fixed 300x100 CSS viewport');
  assert.equal(oracle.queryConvention, 'elementFromPoint(xCssPx, yCssPx), x=0..299, y=0..99',
    'queryConvention must retain the integer CSS point grid');
  assert.equal(oracle.browser.userAgent, 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/155.0.0.0 Safari/537.36',
    'browser userAgent must match pinned Chrome');
  assert.deepEqual(oracle.root, { x: 0, y: 0, width: 300, height: 100, top: 0, right: 300, bottom: 100, left: 0 });
  assert.deepEqual(oracle.unexpected, [], 'every CSS grid query must hit a registered fixture node');

  const sourceBytes = Object.fromEntries(Object.entries(inputPaths).map(([key, relativePath]) => [
    key,
    readFileSync(path.resolve(root, relativePath)),
  ]));
  for (const [key, bytes] of Object.entries(sourceBytes)) {
    assert.equal(sha256(bytes), expectedSourceSha256[key], `${key} source bytes changed after Chrome capture`);
  }

  const observation = JSON.parse(sourceBytes.chromeObservation.toString('utf8'));
  assert.equal(observation.browser.agentBrowserVersion, '0.31.1');
  assert.equal(observation.browser.chromeVersion, 'Google Chrome 155.0.8059.40');
  assert.equal(observation.browser.revision, 'cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}');
  assert.deepEqual(oracle.browser, { userAgent: observation.browser.userAgent });
  assert.deepEqual(observation.viewport, oracle.viewport);

  const expectedNodeIds = ['c131-surface-root'];
  for (const caseId of observation.caseIds) {
    expectedNodeIds.push(`c131-surface-case-${caseId}`, `c131-surface-box-${caseId}`);
    if (caseId === 'nested-clip') expectedNodeIds.push('c131-surface-inner-nested-clip');
    expectedNodeIds.push(`c131-surface-target-${caseId}`);
  }
  assert.deepEqual(oracle.nodeIds, expectedNodeIds, 'fixture node identity order changed');
  assert.equal(new Set(oracle.nodeIds).size, 41, 'fixture node IDs must be unique');
  assert.equal(oracle.rowCount, 100);
  assert.equal(oracle.rows.length, 100);
  assert.equal(oracle.queryCount, 30_000);

  const histogram = new Array(oracle.nodeIds.length).fill(0);
  const identityBytes = new Uint8Array(30_000);
  let runCount = 0;
  for (let y = 0; y < 100; y += 1) {
    const runs = oracle.rows[y];
    assert.ok(Array.isArray(runs) && runs.length > 0, `row ${y} must have at least one run`);
    let nextX = 0;
    for (const [startX, endX, nodeIndex] of runs) {
      assert.equal(startX, nextX, `row ${y} has a gap or overlap at x=${nextX}`);
      assert.ok(Number.isSafeInteger(endX) && endX > startX && endX <= 300, `row ${y} run end is invalid`);
      assert.ok(Number.isSafeInteger(nodeIndex) && nodeIndex >= 0 && nodeIndex < oracle.nodeIds.length,
        `row ${y} run references an unknown node`);
      for (let x = startX; x < endX; x += 1) {
        identityBytes[y * 300 + x] = nodeIndex;
        histogram[nodeIndex] += 1;
      }
      nextX = endX;
      runCount += 1;
    }
    assert.equal(nextX, 300, `row ${y} does not cover the full CSS viewport width`);
  }
  assert.equal(runCount, oracle.runCount, 'run summary does not match the row data');
  assert.equal(histogram.reduce((sum, count) => sum + count, 0), 30_000);
  assert.deepEqual(oracle.histogram, histogram, 'node histogram does not match the row data');
  assert.equal(sha256(identityBytes), oracle.pixelGridIdentitySha256, 'expanded identity digest does not match');

  let checkedFixedSamples = 0;
  for (const item of observation.cases) {
    for (const sample of item.samples) {
      assert.ok(Number.isInteger(sample.x) && Number.isInteger(sample.y));
      assert.ok(sample.x >= 0 && sample.x < 300 && sample.y >= 0 && sample.y < 100);
      const run = oracle.rows[sample.y].find(([startX, endX]) => sample.x >= startX && sample.x < endX);
      assert.ok(run, `fixed sample ${item.id}/${sample.role} is not covered by the full grid`);
      assert.equal(oracle.nodeIds[run[2]], sample.hitId, `fixed sample ${item.id}/${sample.role} identity changed`);
      checkedFixedSamples += 1;
    }
  }
  assert.equal(checkedFixedSamples, 52, 'the existing fixed oracle must remain a complete subset control');

  return {
    status: 'valid',
    viewportCssPx: [300, 100],
    pointCount: 30_000,
    rowCount: 100,
    nodeCount: 41,
    runCount,
    fixedSampleSubsetCount: checkedFixedSamples,
    pixelGridIdentitySha256: sha256(identityBytes),
    sources: Object.fromEntries(Object.entries(sourceBytes).map(([key, bytes]) => [key, sha256(bytes)])),
  };
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const oraclePath = path.join(evidenceDirectory, 'chrome155-hit-identity-grid.json');
  const oracle = JSON.parse(readFileSync(oraclePath, 'utf8'));
  console.log(JSON.stringify(validateHitIdentityGrid(oracle), null, 2));
}
