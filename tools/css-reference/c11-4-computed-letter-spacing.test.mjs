import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));

async function source(path) {
  return readFile(resolve(repositoryRoot, path), 'utf8');
}

test('C11.4 computed letter-spacing remains a fixed opt-in anonymous Grid profile', async () => {
  const paths = {
    ffi: 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/anonymous_text_items.rs',
    ffiScenes: 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/anonymous_text_items/computed_letter_spacing.rs',
    runtime: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_text.rs',
    validation: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_text/letter_spacing.rs',
    runtimeTest: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/anonymous_text_letter_spacing.rs',
    trace: 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs',
    androidHost: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
    iosHost: 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
    capture: 'tools/css-reference/capture-c11-4-computed-letter-spacing-simulators.mjs',
    expected: 'spec/internal/evidence/c11-4-anonymous-grid-text-letter-spacing-precomparison-2026-10-31/expected-values.json',
    plan: 'plan/c11-4-anonymous-grid-text-letter-spacing.md',
  };
  const entries = Object.entries(paths);
  const values = await Promise.all(entries.map(async ([key, path]) => [key, await source(path)]));
  const text = Object.fromEntries(values);
  const expected = JSON.parse(text.expected);
  const cases = expected.cases;
  const uniqueIDs = new Set(cases.map(({ id }) => id));
  assert.equal(cases.length, 16);
  assert.equal(uniqueIDs.size, cases.length);
  assert.equal(expected.fontSizePx, 10);

  for (const item of cases) {
    assert.ok(text.ffiScenes.includes(item.id), `FFI generator is missing ${item.id}`);
    assert.ok(text.validation.includes(item.id), `runtime allowlist is missing ${item.id}`);
    assert.ok(text.runtimeTest.includes(item.id), `runtime test is missing ${item.id}`);
    assert.ok(text.androidHost.includes(item.id), `Android scene allowlist is missing ${item.id}`);
    assert.ok(text.iosHost.includes(item.id), `iOS scene allowlist is missing ${item.id}`);
    assert.equal(item.expectedTrackPx,
      item.track === 'min-content' ? item.expectedMinContentPx : item.expectedMaxContentPx,
      `${item.id}: track oracle`);
    assert.equal(item.expectedRowPx,
      item.text === 'XX XX' && item.track === 'min-content' ? 20 : 10,
      `${item.id}: row oracle`);
  }

  assert.match(text.ffi,
    /#\[path = "anonymous_text_items\/computed_letter_spacing\.rs"\]\s*mod computed_letter_spacing;/);
  assert.match(text.runtime,
    /LETTER_SPACING_SCENE_IDS[\s\S]*validate_computed_letter_spacing[\s\S]*letter_spacing,/);
  assert.match(text.validation,
    /parse_computed_letter_spacing[\s\S]*value == "normal"[\s\S]*strip_suffix\("px"\)/);
  assert.ok(text.validation.includes('(-2.0..=2.0).contains(&number)'));
  assert.ok(text.trace.includes('"letterSpacing": style.letter_spacing'));
  assert.ok(text.capture.includes("target: 'Android API 37 ARM64 AVD'"));
  assert.ok(text.capture.includes("target: 'iPhone 17 Pro / iOS 26.2 Simulator'"));
  assert.ok(text.capture.includes('expectedValues.cases'));
  assert.ok(text.capture.includes('scene.id'));
  assert.ok(text.capture.includes("finalTrackSizes('Columns')"));
  assert.ok(text.capture.includes("finalTrackSizes('Rows')"));
  assert.match(text.plan, /authored `0px`도 `normal`로 직렬화/);
  assert.match(text.plan, /Chrome parity를 주장하지 않는다/);
});
