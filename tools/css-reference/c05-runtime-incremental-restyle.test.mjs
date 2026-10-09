import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  reference: 'tests/fixtures/css/references/c05-runtime-incremental-restyle-v1.json',
  html: 'tests/fixtures/css/c05/runtime-incremental-restyle.html',
  runtimeFixture: 'tests/fixtures/css/c05/runtime-incremental-restyle.js',
  capture: 'tools/css-reference/verify-c05-runtime-incremental-restyle-precomparison.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
};

async function read(path) {
  return readFile(join(repositoryRoot, path));
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

test('C05.4 Chromium fixture pins input bytes and inherited restyle boundary', async () => {
  const [referenceBytes, htmlBytes, runtimeBytes, captureBytes, helperBytes] = await Promise.all([
    read(paths.reference), read(paths.html), read(paths.runtimeFixture), read(paths.capture), read(paths.helper),
  ]);
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  const html = htmlBytes.toString('utf8');
  const runtimeFixture = runtimeBytes.toString('utf8');

  assert.equal(reference.schema, 'spinon-css-runtime-incremental-restyle-reference/v1');
  assert.equal(reference.fixture.htmlPath, paths.html);
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(reference.fixture.runtimeFixturePath, paths.runtimeFixture);
  assert.equal(reference.fixture.runtimeFixtureSha256, sha256(runtimeBytes));
  assert.equal(reference.captureTool.path, paths.capture);
  assert.equal(reference.captureTool.sha256, sha256(captureBytes));
  assert.deepEqual(reference.captureTool.dependencies, [{ path: paths.helper, sha256: sha256(helperBytes) }]);
  assert.deepEqual(reference.oracle, {
    name: 'Google Chrome',
    product: 'Chrome/154.0.8037.98',
    cliVersion: 'Google Chrome 154.0.8037.98',
    revision: '@b859317bf11f6be47f9b7799ec690a0a42a1fb33',
  });
  assert.equal(reference.viewport.widthCssPx, 320);
  assert.equal(reference.viewport.heightCssPx, 180);
  assert.equal(reference.viewport.deviceScaleFactor, 1);
  assert.doesNotMatch(html, /<style\b/i);
  assert.doesNotMatch(runtimeFixture, /createElement\(["']style["']\)/);

  const { before, after } = reference.observation;
  assert.deepEqual(Object.keys(before), Object.keys(after));
  for (const id of reference.comparison.unchangedNodes) {
    assert.deepEqual(after[id], before[id], `${id} must remain unchanged after the left branch update`);
  }
  for (const id of reference.comparison.changedNodes) {
    assert.equal(before[id].width, '32px');
    assert.equal(after[id].width, '46px');
    for (const field of ['x', 'y', 'width', 'height']) {
      assert.ok(Number.isFinite(after[id].rect[field]), `${id}.${field} must be finite`);
    }
  }
});
