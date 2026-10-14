import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';

import { validateObservation } from './capture-c13-1-compact-surface-matrix.mjs';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const observationPath = path.join(
  repositoryRoot,
  'spec/internal/evidence/c13-1-compact-surface-matrix-2026-10-14/chrome155-observation.json',
);

test('Chrome 155 C13.1 compact surface oracle pins 13 frames and 52 pixels', () => {
  const observation = JSON.parse(readFileSync(observationPath, 'utf8'));
  assert.equal(validateObservation(observation), true);
});

test('the oracle rejects changed hit identity, sample color, duplicate points, and geometry', () => {
  const baseline = JSON.parse(readFileSync(observationPath, 'utf8'));
  const mutations = [
    (value) => { value.cases[0].samples[0].hitId = 'c131-surface-box-visible'; },
    (value) => { value.cases[0].samples[0].rgba[0] ^= 1; },
    (value) => { value.cases[0].samples[1].x = value.cases[0].samples[0].x; },
    (value) => { value.cases[1].frame.box.width += 1; },
  ];
  for (const mutate of mutations) {
    const changed = structuredClone(baseline);
    mutate(changed);
    assert.throws(() => validateObservation(changed));
  }
});
