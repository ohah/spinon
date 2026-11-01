import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));

async function source(path) {
  return readFile(resolve(repositoryRoot, path), 'utf8');
}

test('C11.4 Unicode titlecase matrix remains fixture-only and source preserving', async () => {
  const paths = {
    expected: 'spec/internal/evidence/c11-4-anonymous-grid-text-transform-unicode-precomparison-2026-11-01/runtime-matrix-expected.json',
    fullWidthExpected: 'spec/internal/evidence/c11-4-anonymous-grid-text-transform-full-width-precomparison-2026-11-02/runtime-matrix-expected.json',
    fixture: 'tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-transform-unicode.html',
    ffi: 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/anonymous_text_items/computed_text_transform.rs',
    runtime: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_text/text_transform.rs',
    runtimeTest: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/anonymous_text_text_transform.rs',
    textLayout: 'crates/spinon-text-layout/src/normalize/text_transform.rs',
    capture: 'tools/css-reference/capture-c11-4-computed-text-transform-simulators.mjs',
    android: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
    ios: 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
  };
  const entries = await Promise.all(Object.entries(paths)
    .map(async ([key, path]) => [key, await source(path)]));
  const text = Object.fromEntries(entries);
  const expected = JSON.parse(text.expected);
  const fullWidthExpected = JSON.parse(text.fullWidthExpected);
  const scenes = expected.scenes;

  assert.equal(expected.status, 'oracle-targets-only-not-a-Spinon-result');
  assert.equal(expected.browser, 'HeadlessChrome 156.0.0.0 via Agent Browser');
  assert.equal(expected.toleranceCssPx, 0.03125);
  assert.equal(scenes.length, 23);
  assert.equal(new Set(scenes.map(({ id }) => id)).size, scenes.length);
  assert.equal(new Set(scenes.map(({ runtimeSceneId }) => runtimeSceneId)).size, scenes.length);
  assert.equal(fullWidthExpected.sceneCount, 4);
  assert.equal(fullWidthExpected.scenes.length, fullWidthExpected.sceneCount);
  assert.deepEqual(fullWidthExpected.scenes.map(({ runtimeSceneId }) => runtimeSceneId), [
    'text-transform-full-width-none-max-content',
    'text-transform-full-width-max-content',
    'text-transform-full-width-min-content',
    'text-transform-full-width-collapsed-spaces-max-content',
  ]);
  assert.ok(fullWidthExpected.scenes.filter(({ textTransform }) => textTransform === 'full-width')
    .every(({ sourceText, shapedText }) => ['AB 12', 'AB   12'].includes(sourceText)
      && shapedText === 'ＡＢ　１２'));
  assert.equal(fullWidthExpected.font.language, 'ja');
  for (const scene of fullWidthExpected.scenes) {
    assert.ok(scene.runtimeSceneId.startsWith('text-transform-full-width-'), scene.id);
    for (const path of [text.ffi, text.runtime, text.runtimeTest, text.android, text.ios]) {
      assert.ok(path.includes(scene.runtimeSceneId), `${scene.id}: missing full-width runtime mapping`);
    }
  }

  const canvasMetricScenes = scenes.filter(({ canvasTextMetrics }) => canvasTextMetrics);
  assert.deepEqual(canvasMetricScenes.map(({ id }) => id), ['unicode-hyphen-boundary']);
  const canvasMetric = canvasMetricScenes[0];
  assert.equal(canvasMetric.canvasTextMetrics.widthPx, 56.165863037109375);
  assert.equal(canvasMetric.canvasTextMetrics.language, expected.font.language);
  assert.ok(Math.abs(canvasMetric.columnsPx - canvasMetric.canvasTextMetrics.widthPx)
    <= expected.toleranceCssPx);

  for (const scene of scenes) {
    assert.ok(scene.runtimeSceneId.startsWith('text-transform-'), scene.id);
    assert.ok(['none', 'uppercase', 'lowercase', 'capitalize'].includes(scene.textTransform),
      `${scene.id}: transform profile`);
    for (const key of [
      'columnsPx', 'rowsPx', 'minContentPx', 'maxContentPx',
      'minContentHeightPx', 'maxContentHeightPx',
    ]) assert.ok(Number.isFinite(scene[key]) && scene[key] >= 0, `${scene.id}: ${key}`);
    assert.ok(scene.minContentPx <= scene.maxContentPx, `${scene.id}: intrinsic width order`);
    assert.ok(Array.isArray(scene.lineAdvancesPx) && scene.lineAdvancesPx.length > 0,
      `${scene.id}: expected line advances`);
    assert.ok(scene.lineAdvancesPx.every((value) => Number.isFinite(value) && value >= 0),
      `${scene.id}: finite line advances`);
    const sourceTexts = scene.sourceTextNodes ?? [scene.sourceText];
    assert.equal(sourceTexts.join(''), scene.sourceText, `${scene.id}: source fragments concatenate`);
    for (const path of [text.ffi, text.runtime, text.runtimeTest, text.android, text.ios]) {
      assert.ok(path.includes(scene.runtimeSceneId), `${scene.id}: missing runtime mapping`);
    }
  }

  const split = scenes.find(({ id }) => id === 'unicode-source-split');
  assert.deepEqual(split.sourceTextNodes, ['ß', 'eta']);
  assert.equal(split.shapedText, 'Sseta');
  assert.ok(text.capture.includes('runtime-matrix-expected.json'));
  assert.ok(text.capture.includes('CSSOM track vs unquantized Canvas width'));
  assert.ok(text.capture.includes('expectedValues.sceneCount ?? 23'));
  assert.ok(text.capture.includes("'full-width': 'FullWidth'"));
  assert.ok(text.capture.includes("assert.equal(style.wordBreak, 'Normal')"));
  assert.ok(text.capture.includes("assert.equal(style.lineBreak, 'Strict')"));
  assert.ok(text.capture.includes("result: 'FAIL'"));
  assert.ok(text.fixture.includes('text-transform: capitalize'));
  assert.ok(text.textLayout.includes('TitlecaseMapper'));
  assert.ok(text.textLayout.includes('WordSegmenter'));
  assert.ok(text.android.includes(
    'if (c114ComputedTextTransformScene(sceneId) != null) {\n'
      + '            return c114GenericDirectTextIntrinsicInputsMatch(sceneId, report);',
  ));
  assert.ok(text.ios.includes(
    'if c114ComputedTextTransformScene(sceneID) != nil {\n'
      + '        return c114GenericDirectTextIntrinsicInputsMatch(sceneID: sceneID, report: report)',
  ));
  assert.equal((text.capture.match(/\$\{error \|\| 'none'\}/g) ?? []).length, 2);
});
