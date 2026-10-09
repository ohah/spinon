import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inputPath = 'tests/fixtures/css/c04/cascade-layers.v1.json';
const htmlPath = 'tests/fixtures/css/c04/cascade-layers.html';
const baseCssPath = 'tests/fixtures/css/c04/style-layout-bridge.css';
const captureToolPath = 'tools/css-reference/capture-c04-cascade-layers.mjs';
const sessionHelperPath = 'tools/css-reference/chromium-session.mjs';
const referencePath = 'tests/fixtures/css/references/c04-cascade-layers-v1.json';

async function readRepositoryFile(path) {
  return readFile(join(repositoryRoot, path));
}

async function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

test('C04 Cascade Layers reference matches its immutable input and capture sources', async () => {
  const [inputBytes, htmlBytes, baseCssBytes, captureToolBytes, sessionHelperBytes, referenceBytes] = await Promise.all([
    readRepositoryFile(inputPath),
    readRepositoryFile(htmlPath),
    readRepositoryFile(baseCssPath),
    readRepositoryFile(captureToolPath),
    readRepositoryFile(sessionHelperPath),
    readRepositoryFile(referencePath),
  ]);
  const input = JSON.parse(inputBytes.toString('utf8'));
  const reference = JSON.parse(referenceBytes.toString('utf8'));

  assert.equal(input.schema, 'spinon-css-c04-cascade-layers-input/v1');
  assert.equal(reference.schema, 'spinon-css-c04-cascade-layers-reference/v1');
  assert.equal(reference.fixture.id, input.fixtureId);
  assert.equal(reference.fixture.inputPath, inputPath);
  assert.equal(reference.fixture.inputSha256, await sha256(inputBytes));
  assert.equal(reference.fixture.htmlPath, htmlPath);
  assert.equal(reference.fixture.htmlSha256, await sha256(htmlBytes));
  assert.equal(reference.fixture.baseCssPath, baseCssPath);
  assert.equal(reference.fixture.baseCssSha256, await sha256(baseCssBytes));
  assert.equal(reference.captureTool.path, captureToolPath);
  assert.equal(reference.captureTool.sha256, await sha256(captureToolBytes));
  assert.deepEqual(reference.captureTool.dependencies, [{
    path: sessionHelperPath,
    sha256: await sha256(sessionHelperBytes),
  }]);
  assert.equal(reference.oracle.name, 'Chromium');
  assert.deepEqual(reference.comparison, input.comparison);

  const expectedCaseIds = input.cases.map(({ id }) => id);
  const observedCaseIds = reference.observations.map(({ caseId }) => caseId);
  assert.equal(new Set(expectedCaseIds).size, expectedCaseIds.length, '입력 case ID가 중복되지 않아야 합니다');
  assert.deepEqual(observedCaseIds, expectedCaseIds);
  for (const observation of reference.observations) {
    assert.deepEqual(observation.frames.map(({ id }) => id), [
      'flex-parent',
      'flex-a',
      'flex-b',
      'flex-c',
    ]);
    assert.deepEqual(observation.itemStyles.map(({ id }) => id), ['flex-a', 'flex-b', 'flex-c']);
    assert.equal(typeof observation.computed.alignItems, 'string');
    assert.equal(typeof observation.computed.justifyContent, 'string');
    for (const frame of observation.frames) {
      for (const field of ['x', 'y', 'width', 'height']) {
        assert.equal(Number.isFinite(frame[field]), true, `${observation.caseId}.${frame.id}.${field}`);
      }
    }
  }
});
