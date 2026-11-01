import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));

async function source(path) {
  return readFile(resolve(repositoryRoot, path), 'utf8');
}

test('C11.4 capitalize keeps the fixed English/ASCII boundary and layout-only source mapping', async () => {
  const paths = {
    expected: 'spec/internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-precomparison-2026-11-01/expected.json',
    plan: 'plan/c11-4-anonymous-grid-text-transform-capitalize.md',
    review: 'spec/internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-plan-review-20-2026-11-01.md',
    fixture: 'tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-transform-capitalize.html',
    textLayout: 'crates/spinon-text-layout/src/normalize.rs',
    textLayoutTest: 'crates/spinon-text-layout/src/tests/c11_4_text_transform.rs',
    ffi: 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/anonymous_text_items/computed_text_transform.rs',
    runtime: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_text/text_transform.rs',
    runtimeTest: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/anonymous_text_text_transform.rs',
    capture: 'tools/css-reference/capture-c11-4-computed-text-transform-simulators.mjs',
    boundary: 'tools/css-reference/fixture-module-boundary.test.mjs',
    android: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
    ios: 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
  };
  const entries = await Promise.all(Object.entries(paths)
    .map(async ([key, path]) => [key, await source(path)]));
  const text = Object.fromEntries(entries);
  const expected = JSON.parse(text.expected);
  const scenes = expected.scenes;
  assert.equal(expected.browser, 'HeadlessChrome 156.0.0.0 via Agent Browser');
  assert.equal(expected.font.language, 'en');
  assert.equal(scenes.length, 6);
  assert.deepEqual(scenes.map(({ id }) => id), [
    'transform-none-min', 'transform-none-max',
    'transform-capitalize-min', 'transform-capitalize-max',
    'transform-capitalize-mixed-case', 'transform-capitalize-collapsed-spaces',
  ]);
  assert.equal(new Set(scenes.map(({ runtimeSceneId }) => runtimeSceneId)).size, scenes.length);

  for (const scene of scenes) {
    const advances = expected.font.glyphAdvances;
    const wordWidth = (word) => [...word].reduce((sum, character) => {
      assert.ok(Number.isFinite(advances[character]), `${scene.id}: missing ${character} advance`);
      return sum + advances[character] * 10 / expected.font.unitsPerEm;
    }, 0);
    const words = scene.shapedText.split(' ');
    const minContent = Math.max(...words.map(wordWidth));
    const maxContent = words.reduce((sum, word) => sum + wordWidth(word), 0)
      + advances.space * 10 / expected.font.unitsPerEm;
    const selected = scene.track === 'min-content' ? minContent : maxContent;
    assert.ok(Math.abs(scene.columnsPx - selected) <= expected.toleranceCssPx,
      `${scene.id}: track differs from independent glyph advances`);
    assert.equal(scene.rowsPx, scene.track === 'min-content' ? 20 : 10);
    for (const path of [text.ffi, text.runtime, text.runtimeTest, text.android, text.ios]) {
      assert.ok(path.includes(scene.runtimeSceneId), `${scene.id}: runtime scene is not wired`);
    }
  }

  assert.match(text.textLayout, /TextTransformMode::Capitalize/);
  assert.match(text.textLayout, /is_ascii_alphabetic\(\)/);
  assert.match(text.textLayout, /capitalize_word_start/);
  assert.match(text.textLayoutTest, /capitaliz(?:e|es).*source.ranges|capitalize.*preserves.ranges/is);
  assert.match(text.capture, /process\.argv\[4\]/);
  assert.match(text.capture, /scene\.runtimeSceneId/);
  assert.match(text.fixture, /text-transform: capitalize/);
  assert.match(text.boundary, /text-transform-capitalize-collapsed-spaces/);
  assert.match(text.plan, /U\+0020/);
  assert.match(text.plan, /does not add general CSS admission/);
  assert.match(text.review, /20 perspectives/);
  assert.match(text.review, /Unicode expansion/);
});
