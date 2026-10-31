import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));

async function source(path) {
  return readFile(resolve(repositoryRoot, path), 'utf8');
}

test('C11.4 computed word-spacing remains a fixed opt-in anonymous Grid profile', async () => {
  const paths = {
    ffi: 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/anonymous_text_items.rs',
    ffiScenes: 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/anonymous_text_items/computed_word_spacing.rs',
    runtime: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_text.rs',
    validation: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_text/word_spacing.rs',
    runtimeTest: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/anonymous_text_word_spacing.rs',
    trace: 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs',
    androidHost: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
    iosHost: 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
    capture: 'tools/css-reference/capture-c11-4-computed-word-spacing-simulators.mjs',
    expected: 'spec/internal/evidence/c11-4-anonymous-grid-text-word-spacing-precomparison-2026-10-31/spec-derived-expectations.json',
    plan: 'plan/c11-4-anonymous-grid-text-computed-word-spacing.md',
  };
  const entries = Object.entries(paths);
  const values = await Promise.all(entries.map(async ([key, path]) => [key, await source(path)]));
  const text = Object.fromEntries(values);
  const expected = JSON.parse(text.expected);
  const cases = expected.cases;
  const uniqueIDs = new Set(cases.map(({ id }) => id));
  assert.equal(cases.length, 24);
  assert.equal(uniqueIDs.size, cases.length);
  assert.equal(expected.ahem.fontSizeCssPx, 10);

  for (const item of cases) {
    assert.ok(text.ffiScenes.includes(item.id), `FFI generator is missing ${item.id}`);
    assert.ok(text.validation.includes(item.id), `runtime allowlist is missing ${item.id}`);
    const words = item.text.trim().split(/\s+/);
    const spacing = Number.parseFloat(item.computedWordSpacing);
    const minContent = Math.max(...words.map((word) => [...word].length)) * 10;
    const maxContent = words.reduce((sum, word) => sum + [...word].length, 0) * 10
      + Math.max(0, words.length - 1) * (10 + spacing);
    assert.equal(item.columnsPx, item.track === 'min' ? minContent : maxContent,
      `${item.id}: independent track formula`);
    assert.equal(item.rowsPx, item.track === 'min' ? words.length * 10 : 10,
      `${item.id}: independent row formula`);
  }

  assert.match(text.ffi,
    /#\[path = "anonymous_text_items\/computed_word_spacing\.rs"\]\s*mod computed_word_spacing;/);
  assert.match(text.runtime,
    /WORD_SPACING_SCENE_IDS[\s\S]*validate_computed_word_spacing[\s\S]*word_spacing,/);
  assert.ok(text.runtimeTest.includes('fn cases()'));
  assert.ok(text.runtimeTest.includes('for (index, case) in cases.iter().enumerate()'));
  assert.ok(text.runtimeTest.includes(
    'anonymous_grid_computed_word_spacing_reaches_intrinsic_tracks_and_text_layout'));
  assert.match(text.validation,
    /parse_computed_word_spacing[\s\S]*strip_suffix\("px"\)/);
  for (const value of ['normal', 'zero', 'minus-one', 'plus-one']) {
    assert.ok(text.androidHost.includes(`word-spacing-${value}-`),
      `Android scene adapter is missing ${value}`);
    assert.ok(text.iosHost.includes(`word-spacing-${value}-`),
      `iOS scene adapter is missing ${value}`);
  }
  assert.ok(text.trace.includes('"wordSpacing": style.word_spacing'));
  assert.ok(text.capture.includes("target: 'Android API 37 ARM64 AVD'"));
  assert.ok(text.capture.includes("target: 'iPhone 17 Pro / iOS 26.2 Simulator'"));
  assert.ok(text.capture.includes('expectedValues.cases'));
  assert.ok(text.capture.includes('scene.computedWordSpacing'));
  assert.ok(text.capture.includes("finalTrackSizes('Columns')"));
  assert.ok(text.capture.includes("finalTrackSizes('Rows')"));
  assert.match(text.plan, /normal.*0px/);
  assert.match(text.plan, /Chrome.*oracle/);
});
