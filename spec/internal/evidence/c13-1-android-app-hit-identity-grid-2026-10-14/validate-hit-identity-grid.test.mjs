import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import { evidenceDirectory, validateHitIdentityGrid } from './validate-hit-identity-grid.mjs';

const pinned = JSON.parse(readFileSync(`${evidenceDirectory}/chrome155-hit-identity-grid.json`, 'utf8'));
const clone = () => structuredClone(pinned);

test('full CSS integer grid pins the source, viewport, identities and 52-sample subset', () => {
  const result = validateHitIdentityGrid(pinned);
  assert.deepEqual(result, {
    status: 'valid',
    viewportCssPx: [300, 100],
    pointCount: 30_000,
    rowCount: 100,
    nodeCount: 41,
    runCount: 870,
    fixedSampleSubsetCount: 52,
    pixelGridIdentitySha256: '93cccd042e82d8cd30cb7a8ab34a5154073d4a9eccbc4038294d3ca5b9dfce87',
    sources: {
      html: '431f4b4452969adcae80c4e5f89e5e45b1e8756fc3c4d8995b7a5eccfbc6f355',
      script: 'd36bc1f7d3c791cdc91f6d878c3961e067efc2f88ca1486db8a065c5fd69c760',
      chromeObservation: '515fc95f811399fda9e336a13e7583f2d22555fce4727f1fe7f734981abd0299',
    },
  });
});

test('rejects a different viewport or coordinate convention', () => {
  const wrongSize = clone();
  wrongSize.viewport.width = 301;
  assert.throws(() => validateHitIdentityGrid(wrongSize), /viewport/);
  const wrongConvention = clone();
  wrongConvention.queryConvention = 'elementFromPoint(x + 0.5, y + 0.5)';
  assert.throws(() => validateHitIdentityGrid(wrongConvention), /queryConvention/);
});

test('rejects gaps, overlaps and empty rows in run-length data', () => {
  const gap = clone();
  gap.rows[0][0][1] -= 1;
  assert.throws(() => validateHitIdentityGrid(gap), /gap or overlap/);
  const overlap = clone();
  overlap.rows[0][1][0] -= 1;
  assert.throws(() => validateHitIdentityGrid(overlap), /gap or overlap/);
  const empty = clone();
  empty.rows[1] = [];
  assert.throws(() => validateHitIdentityGrid(empty), /at least one run/);
});

test('rejects unknown nodes, duplicate IDs and changed expanded-grid digest', () => {
  const badIndex = clone();
  badIndex.rows[0][0][2] = 255;
  assert.throws(() => validateHitIdentityGrid(badIndex), /unknown node/);
  const duplicateId = clone();
  duplicateId.nodeIds[1] = duplicateId.nodeIds[0];
  assert.throws(() => validateHitIdentityGrid(duplicateId), /identity order changed/);
  const badDigest = clone();
  badDigest.pixelGridIdentitySha256 = '0'.repeat(64);
  assert.throws(() => validateHitIdentityGrid(badDigest), /digest/);
});

test('rejects changes to source files, browser provenance and fixed sample identities', () => {
  assert.throws(() => validateHitIdentityGrid(clone(), { root: '/tmp' }), /ENOENT/);
  const badBrowser = clone();
  badBrowser.browser.userAgent = 'different browser';
  assert.throws(() => validateHitIdentityGrid(badBrowser), /userAgent/);
  const badFixedSample = clone();
  const sampleRun = badFixedSample.rows[6].find(([startX, endX]) => 5 >= startX && 5 < endX);
  const previousIndex = sampleRun[2];
  const changedIndex = (previousIndex + 1) % badFixedSample.nodeIds.length;
  const changedWidth = sampleRun[1] - sampleRun[0];
  sampleRun[2] = changedIndex;
  badFixedSample.histogram[previousIndex] -= changedWidth;
  badFixedSample.histogram[changedIndex] += changedWidth;
  const identityBytes = new Uint8Array(30_000);
  for (let y = 0; y < badFixedSample.rows.length; y += 1) {
    for (const [startX, endX, nodeIndex] of badFixedSample.rows[y]) {
      identityBytes.fill(nodeIndex, y * 300 + startX, y * 300 + endX);
    }
  }
  badFixedSample.pixelGridIdentitySha256 = createHash('sha256').update(identityBytes).digest('hex');
  assert.throws(() => validateHitIdentityGrid(badFixedSample), /fixed sample/);
});
