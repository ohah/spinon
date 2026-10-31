import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));

async function source(path) {
  return readFile(resolve(repositoryRoot, path), 'utf8');
}

test('C11.4 anonymous Grid text-transform stays fixed, opt-in, and source preserving', async () => {
  const paths = {
    ffi: 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/anonymous_text_items.rs',
    ffiScenes: 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/anonymous_text_items/computed_text_transform.rs',
    trace: 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs',
    runtime: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_text.rs',
    validation: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_text/text_transform.rs',
    runtimeTests: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests.rs',
    runtimeTest: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/anonymous_text_text_transform.rs',
    android: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
    ios: 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
    capture: 'tools/css-reference/capture-c11-4-computed-text-transform-simulators.mjs',
    expected: 'spec/internal/evidence/c11-4-anonymous-grid-text-transform-precomparison-2026-11-01/expected.json',
    plan: 'plan/c11-4-anonymous-grid-text-computed-text-transform.md',
  };
  const entries = await Promise.all(Object.entries(paths)
    .map(async ([key, path]) => [key, await source(path)]));
  const text = Object.fromEntries(entries);
  const expected = JSON.parse(text.expected);
  const cases = expected.scenes;
  const sceneIDs = cases.map((scene) =>
    `text-transform-${scene.textTransform}-${scene.track === 'min-content' ? 'min' : 'max'}-content`);
  assert.equal(cases.length, 6);
  assert.deepEqual(cases.map(({ id }) => id), [
    'transform-none-min', 'transform-none-max',
    'transform-uppercase-min', 'transform-uppercase-max',
    'transform-lowercase-min', 'transform-lowercase-max',
  ]);
  assert.deepEqual(sceneIDs, [
    'text-transform-none-min-content', 'text-transform-none-max-content',
    'text-transform-uppercase-min-content', 'text-transform-uppercase-max-content',
    'text-transform-lowercase-min-content', 'text-transform-lowercase-max-content',
  ]);
  assert.equal(new Set(sceneIDs).size, sceneIDs.length);
  assert.equal(expected.font.family, 'Noto Sans KR');
  assert.equal(expected.font.language, 'en');

  for (const scene of cases) {
    const sceneID = `text-transform-${scene.textTransform}-${scene.track === 'min-content' ? 'min' : 'max'}-content`;
    assert.ok(text.ffiScenes.includes(sceneID), `FFI scene missing ${sceneID}`);
    assert.ok(text.validation.includes(sceneID), `runtime allowlist missing ${sceneID}`);
    assert.ok(text.runtimeTest.includes(sceneID), `runtime verifier missing ${sceneID}`);
    assert.ok(text.android.includes(sceneID), `Android scene expectation missing ${sceneID}`);
    assert.ok(text.ios.includes(sceneID), `iOS scene expectation missing ${sceneID}`);
    const glyphAdvance = (value) => [...value].reduce((sum, char) => {
      const advance = expected.font.glyphAdvances[char];
      assert.ok(Number.isFinite(advance), `${scene.id}: no independent advance for ${char}`);
      return sum + advance * 10 / expected.font.unitsPerEm;
    }, 0);
    const words = scene.shapedText.split(' ');
    const minContent = Math.max(...words.map(glyphAdvance));
    const maxContent = words.reduce((sum, word) => sum + glyphAdvance(word), 0)
      + expected.font.glyphAdvances.space * 10 / expected.font.unitsPerEm;
    const selected = scene.track === 'min-content' ? minContent : maxContent;
    assert.ok(Math.abs(scene.columnsPx - selected) <= expected.toleranceCssPx,
      `${scene.id}: selected track must match the independent font metric`);
    assert.equal(scene.rowsPx, scene.track === 'min-content' ? 20 : 10,
      `${scene.id}: automatic row height`);
  }

  assert.match(text.ffi,
    /#\[path = "anonymous_text_items\/computed_text_transform\.rs"\]\s*mod computed_text_transform;/);
  assert.match(text.runtimeTests,
    /#\[cfg\(feature = "c11-4-grid-track-sizing-fixture-test"\)\][\s\S]*anonymous_text_text_transform;/);
  assert.match(text.runtime,
    /TEXT_TRANSFORM_SCENE_IDS[\s\S]*validate_computed_text_transform[\s\S]*text_transform,/);
  assert.match(text.runtimeTest,
    /text_layout\.input\.sources\[0\]\.text, "IIII ii"[\s\S]*text_layout\.layout\.text, case\.expected_text/);
  assert.ok(text.validation.includes('xml:lang is outside the fixture profile'));
  assert.ok(text.validation.includes('Grid style owner lang must be exactly en'));
  assert.ok(text.trace.includes('"textTransform": format!("{:?}", style.text_transform)'));
  assert.ok(text.android.includes('BuildConfig.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST'));
  assert.ok(text.ios.includes('#if SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST_1'));
  assert.ok(text.capture.includes('style.textTransform'));
  assert.ok(text.capture.includes('installedApkSha256'));
  assert.ok(text.capture.includes('installedExecutableSha256'));
  assert.ok(text.capture.includes('finalTrackSizes(columnPass)'));
  assert.ok(text.capture.includes('finalTrackSizes(rowPass)'));
  assert.match(text.plan, /ASCII/);
  assert.match(text.plan, /Noto Sans KR/);
  assert.match(text.plan, /12\/?213|72\/?213/);
});
