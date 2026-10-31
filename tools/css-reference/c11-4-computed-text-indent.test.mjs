import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));

async function source(path) {
  return readFile(resolve(repositoryRoot, path), 'utf8');
}

test('C11.4 text-indent remains a fixed opt-in anonymous Grid profile', async () => {
  const paths = {
    ffi: 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/anonymous_text_items.rs',
    ffiScenes: 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/anonymous_text_items/computed_text_indent.rs',
    runtime: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_text.rs',
    validation: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_text/text_indent.rs',
    runtimeTest: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/anonymous_text_text_indent.rs',
    trace: 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs',
    androidHost: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
    iosHost: 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
    capture: 'tools/css-reference/capture-c11-4-computed-text-indent-simulators.mjs',
    expected: 'spec/internal/evidence/c11-4-anonymous-grid-text-indent-runtime-2026-11-01/spec-derived-expectations.json',
    oracle: 'spec/internal/evidence/c11-4-anonymous-grid-text-indent-precomparison-2026-11-01/chrome156-observation.json',
    negativeExpected: 'spec/internal/evidence/c11-4-anonymous-grid-text-negative-indent-precomparison-2026-11-01/expected.json',
    plan: 'plan/c11-4-anonymous-grid-text-computed-text-indent.md',
    negativePlan: 'plan/c11-4-anonymous-grid-text-negative-indent.md',
  };
  const values = await Promise.all(Object.entries(paths)
    .map(async ([key, path]) => [key, await source(path)]));
  const text = Object.fromEntries(values);
  const expected = JSON.parse(text.expected);
  const cases = expected.cases;
  const uniqueIDs = new Set(cases.map(({ id }) => id));
  assert.equal(cases.length, 18);
  assert.equal(uniqueIDs.size, cases.length);
  assert.equal(expected.ahem.fontSizeCssPx, 10);

  for (const item of cases) {
    assert.ok(text.ffiScenes.includes(item.id), `FFI generator is missing ${item.id}`);
    assert.ok(text.validation.includes(item.id), `runtime allowlist is missing ${item.id}`);
    const words = item.text.trim().split(/\s+/);
    const indent = Number.parseFloat(item.computedTextIndent);
    const minContent = Math.max(...words.map((word) => [...word].length)) * 10 + indent;
    const maxContent = words.reduce((sum, word) => sum + [...word].length, 0) * 10
      + Math.max(0, words.length - 1) * 10 + indent;
    const expectedTrack = item.track === 'min' ? minContent : maxContent;
    const expectedRows = item.track === 'min' ? words.length * 10 : 10;
    assert.equal(item.columnsPx, expectedTrack, `${item.id}: independent Grid track`);
    assert.equal(item.rowsPx, expectedRows, `${item.id}: independent auto row`);
    assert.equal(item.minContentPx, minContent, `${item.id}: min-content formula`);
    assert.equal(item.maxContentPx, maxContent, `${item.id}: max-content formula`);
  }

  assert.match(text.ffi,
    /#\[path = "anonymous_text_items\/computed_text_indent\.rs"\]\s*mod computed_text_indent;/);
  assert.match(text.runtime,
    /TEXT_INDENT_SCENE_IDS[\s\S]*validate_computed_text_indent[\s\S]*text_indent,/);
  assert.ok(text.runtimeTest.includes('anonymous_grid_computed_text_indent_reaches_intrinsic_tracks_and_first_line_position'));
  assert.match(text.validation, /parse_computed_text_indent[\s\S]*strip_suffix\("px"\)/);
  assert.ok(text.trace.includes('"textIndent": style.text_indent'));
  assert.ok(text.androidHost.includes('c114ComputedTextIndentScene'));
  assert.match(text.androidHost,
    /double\[\] textIndent = c114ComputedTextIndentScene\(sceneId\)[\s\S]*?textIndent\[3\],\s*textIndent\[9\]/);
  assert.match(text.androidHost,
    /advances\[index\] = minContentTrack\s*\?\s*index == 1 \? textIndent\[8\] : textIndent\[7\]\s*:\s*textIndent\[9\]/);
  assert.match(text.androidHost,
    /private void applyLatestEnvironment\(\)[\s\S]*?environmentScale = c153GlyphPaintProbeRequested[\s\S]*?\|\| c114GridTrackSizingRequested/);
  assert.ok(text.iosHost.includes('c114ComputedTextIndentScene'));
  assert.match(text.iosHost,
    /private func c114IsGridTrackSizingScene\([\s\S]*?c114ComputedTextIndentScene\(sceneID\)\s*!=\s*nil\s*\{\s*return true\s*\}/);
  assert.match(text.iosHost,
    /if let textIndent = c114ComputedTextIndentScene\(sceneID\)[\s\S]*?textWidth:\s*textIndent\.unindentedLineAdvance/);
  assert.ok(text.androidHost.includes('"textIndent", expected.textIndent'));
  assert.ok(text.iosHost.includes('expected: expected.textIndent'));
  assert.ok(text.capture.includes("target: 'Android API 37 ARM64 AVD'"));
  assert.ok(text.capture.includes("target: 'iPhone 17 Pro / iOS 26.2 Simulator'"));
  assert.ok(text.capture.includes('expectedValues.cases'));
  assert.ok(text.capture.includes('scene.computedTextIndent'));
  assert.ok(text.capture.includes("finalTrackSizes('Columns')"));
  assert.ok(text.capture.includes("finalTrackSizes('Rows')"));
  assert.equal(JSON.parse(text.oracle).gridProbe.observations.length, 7);
  assert.match(text.plan, /18 scene/);
  assert.match(text.plan, /Chrome 156/);
  const negative = JSON.parse(text.negativeExpected);
  assert.equal(negative.candidateScenes.length, 12);
  assert.equal(new Set(negative.candidateScenes.map(({ sceneId }) => sceneId)).size, 12);
  for (const item of negative.candidateScenes) {
    const metric = negative.candidateMetrics.find((candidate) => candidate.text === item.text
      && candidate.indentPx === item.indentPx);
    assert.ok(metric, `${item.sceneId}: independent negative-indent metrics`);
    const words = item.text.split(/\s+/);
    const minContent = Math.max(...words.map((word, index) => [...word].length * 10
      + (index === 0 ? item.indentPx : 0)));
    const maxContent = words.reduce((sum, word) => sum + [...word].length * 10, 0)
      + (words.length - 1) * 10 + item.indentPx;
    const expectedTrack = item.trackKind === 'min-content' ? minContent : maxContent;
    assert.equal(metric.minContentPx, minContent, `${item.sceneId}: min-content formula`);
    assert.equal(metric.maxContentPx, maxContent, `${item.sceneId}: max-content formula`);
    assert.equal(item.trackPx, expectedTrack, `${item.sceneId}: selected Grid track`);
    assert.ok(text.ffiScenes.includes(item.sceneId), `FFI generator is missing ${item.sceneId}`);
    assert.ok(text.validation.includes(item.sceneId), `runtime allowlist is missing ${item.sceneId}`);
  }
  assert.match(text.validation, /-10\.0\.\.=10\.0/);
  assert.match(text.androidHost, /text-indent-negative-five-[\s\S]*text-indent-negative-ten-/);
  assert.match(text.iosHost, /text-indent-negative-five-[\s\S]*text-indent-negative-ten-/);
  assert.match(text.negativePlan, /12개/);
  assert.match(text.negativePlan, /-30px/);
});
