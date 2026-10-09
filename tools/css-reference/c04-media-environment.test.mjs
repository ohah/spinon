import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  input: 'tests/fixtures/css/c04/media-environment.v1.json',
  html: 'tests/fixtures/css/c04/media-environment.html',
  css: 'tests/fixtures/css/c04/media-environment.css',
  capture: 'tools/css-reference/capture-c04-media-environment.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
  reference: 'tests/fixtures/css/references/c04-media-environment-v1.json',
};

async function bytes(path) {
  return readFile(join(repositoryRoot, path));
}

function sha256(value) {
  return createHash('sha256').update(value).digest('hex');
}

test('C04 media environment Chromium reference is tied to immutable fixture and capture sources', async () => {
  const [inputBytes, htmlBytes, cssBytes, captureBytes, helperBytes, referenceBytes] = await Promise.all([
    ...Object.values(paths).filter((path) => path !== paths.reference).map(bytes),
    bytes(paths.reference),
  ]);
  const [inputBytesValue, htmlBytesValue, cssBytesValue, captureBytesValue, helperBytesValue, referenceBytesValue] = [
    inputBytes, htmlBytes, cssBytes, captureBytes, helperBytes, referenceBytes,
  ];
  const input = JSON.parse(inputBytesValue.toString('utf8'));
  const reference = JSON.parse(referenceBytesValue.toString('utf8'));
  assert.equal(input.schema, 'spinon-css-c04-media-environment-input/v1');
  assert.equal(reference.schema, 'spinon-css-c04-media-environment-reference/v1');
  assert.equal(reference.fixture.id, input.fixtureId);
  assert.equal(reference.fixture.inputPath, paths.input);
  assert.equal(reference.fixture.inputSha256, sha256(inputBytesValue));
  assert.equal(reference.fixture.htmlPath, paths.html);
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytesValue));
  assert.equal(reference.fixture.cssPath, paths.css);
  assert.equal(reference.fixture.cssSha256, sha256(cssBytesValue));
  assert.equal(reference.captureTool.path, paths.capture);
  assert.equal(reference.captureTool.sha256, sha256(captureBytesValue));
  assert.deepEqual(reference.captureTool.dependencies, [{
    path: paths.helper,
    sha256: sha256(helperBytesValue),
  }]);
  assert.equal(reference.oracle.name, 'Chromium');
  assert.match(reference.oracle.product, /^Chrome\/\d+\.\d+\.\d+\.\d+$/);
  assert.match(reference.oracle.revision, /^@[0-9a-f]+$/);
  assert.deepEqual(reference.comparison, input.comparison);
  assert.deepEqual(reference.observations.map(({ caseId }) => caseId), input.cases.map(({ id }) => id));

  for (const [index, observation] of reference.observations.entries()) {
    const item = input.cases[index];
    assert.deepEqual(Object.keys(observation.mediaMatches), [
      'prefersColorSchemeLight', 'prefersColorSchemeDark',
      'pointerNone', 'pointerCoarse', 'pointerFine',
      'hoverNone', 'hoverHover',
      'anyPointerNone', 'anyPointerCoarse', 'anyPointerFine',
      'anyHoverNone', 'anyHoverHover',
    ]);
    assert.deepEqual(Object.keys(observation.displays), input.probes);
    assert.equal(observation.mediaMatches.prefersColorSchemeLight, item.colorScheme === 'light');
    assert.equal(observation.mediaMatches.prefersColorSchemeDark, item.colorScheme === 'dark');
    assert.equal(observation.mediaMatches.pointerNone, item.primaryPointer === 'none');
    assert.equal(observation.mediaMatches.pointerCoarse, item.primaryPointer === 'coarse');
    assert.equal(observation.mediaMatches.pointerFine, item.primaryPointer === 'fine');
    assert.equal(observation.mediaMatches.hoverNone, !item.primaryHover);
    assert.equal(observation.mediaMatches.hoverHover, item.primaryHover);
    assert.equal(observation.mediaMatches.anyPointerNone, !item.allPointers.coarse && !item.allPointers.fine);
    assert.equal(observation.mediaMatches.anyPointerCoarse, item.allPointers.coarse);
    assert.equal(observation.mediaMatches.anyPointerFine, item.allPointers.fine);
    assert.equal(observation.mediaMatches.anyHoverNone, !item.allPointers.hover);
    assert.equal(observation.mediaMatches.anyHoverHover, item.allPointers.hover);
    for (const [name, matched] of Object.entries(observation.mediaMatches)) {
      const probe = {
        prefersColorSchemeLight: 'scheme-light',
        prefersColorSchemeDark: 'scheme-dark',
        pointerNone: 'pointer-none',
        pointerCoarse: 'pointer-coarse',
        pointerFine: 'pointer-fine',
        hoverNone: 'hover-none',
        hoverHover: 'hover-hover',
        anyPointerNone: 'any-pointer-none',
        anyPointerCoarse: 'any-pointer-coarse',
        anyPointerFine: 'any-pointer-fine',
        anyHoverNone: 'any-hover-none',
        anyHoverHover: 'any-hover-hover',
      }[name];
      assert.equal(observation.displays[probe], matched ? 'none' : 'block', `${item.id}.${name}`);
    }
  }
});
