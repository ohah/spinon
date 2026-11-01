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
    sceneTable: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_text/text_indent/scenes.rs',
    runtimeTest: 'crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/anonymous_text_text_indent.rs',
    trace: 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs',
    androidHost: 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
    iosHost: 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
    capture: 'tools/css-reference/capture-c11-4-computed-text-indent-simulators.mjs',
    expected: 'spec/internal/evidence/c11-4-anonymous-grid-text-indent-runtime-2026-11-01/spec-derived-expectations.json',
    flagsExpected: 'spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-precomparison-2026-11-01/expected-values.json',
    oracle: 'spec/internal/evidence/c11-4-anonymous-grid-text-indent-precomparison-2026-11-01/chrome156-observation.json',
    negativeExpected: 'spec/internal/evidence/c11-4-anonymous-grid-text-negative-indent-precomparison-2026-11-01/expected.json',
    artifactScanner: 'spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/artifact-boundary/scan-artifact.mjs',
    plan: 'plan/c11-4-anonymous-grid-text-computed-text-indent.md',
    negativePlan: 'plan/c11-4-anonymous-grid-text-negative-indent.md',
  };
  const values = await Promise.all(Object.entries(paths)
    .map(async ([key, path]) => [key, await source(path)]));
  const text = Object.fromEntries(values);
  const expected = JSON.parse(text.expected);
  const flagsExpected = JSON.parse(text.flagsExpected);
  const cases = expected.cases;
  const uniqueIDs = new Set(cases.map(({ id }) => id));
  assert.equal(cases.length, 18);
  assert.equal(uniqueIDs.size, cases.length);
  assert.equal(expected.ahem.fontSizeCssPx, 10);

  for (const item of cases) {
    assert.ok(text.ffiScenes.includes(item.id), `FFI generator is missing ${item.id}`);
    assert.ok(text.sceneTable.includes(item.id), `runtime allowlist is missing ${item.id}`);
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
  assert.equal(flagsExpected.cases.length, 20);
  assert.equal(new Set(flagsExpected.cases.map(({ id }) => id)).size, 20);
  for (const item of flagsExpected.cases) {
    const sceneID = `text-indent-flags-${item.id}`;
    assert.ok(text.ffiScenes.includes(sceneID), `FFI generator is missing ${sceneID}`);
    assert.ok(text.sceneTable.includes(sceneID), `runtime allowlist is missing ${sceneID}`);
    const forcedBreak = item.whiteSpace === 'pre-line';
    const minContentTrack = item.id.includes('-min-content-');
    const indentPx = Number.parseFloat(item.computedIndent);
    const eachLine = item.computedIndent.includes('each-line');
    const hanging = item.computedIndent.includes('hanging');
    const firstIndent = hanging ? 0 : indentPx;
    const secondIndent = forcedBreak
      ? ((eachLine !== hanging) ? indentPx : 0)
      : (hanging ? indentPx : 0);
    assert.deepEqual(item.lineOffsetsPx,
      minContentTrack || forcedBreak ? [firstIndent, secondIndent] : [firstIndent],
      `${sceneID}: line indentation`);
    assert.deepEqual(item.lineAdvancesPx,
      minContentTrack || forcedBreak ? [30, 10] : [50], `${sceneID}: line advances`);
    const minContent = Math.max(30 + firstIndent, 10 + secondIndent);
    const maxContent = forcedBreak ? minContent : 50 + firstIndent;
    const counterpartID = item.id.replace(
      minContentTrack ? '-min-content-' : '-max-content-',
      minContentTrack ? '-max-content-' : '-min-content-');
    const counterpart = flagsExpected.cases.find(({ id }) => id === counterpartID);
    assert.ok(counterpart, `${sceneID}: paired intrinsic track`);
    assert.equal(item.usedColumnsPx,
      minContentTrack ? minContent : maxContent, `${sceneID}: selected track`);
    assert.equal(counterpart.usedColumnsPx,
      minContentTrack ? maxContent : minContent, `${sceneID}: paired track`);
    assert.equal(item.rootHeightPx, minContentTrack || forcedBreak ? 20 : 10,
      `${sceneID}: auto row height`);
  }

  assert.match(text.ffi,
    /#\[path = "anonymous_text_items\/computed_text_indent\.rs"\]\s*mod computed_text_indent;/);
  assert.match(text.runtime,
    /TEXT_INDENT_SCENE_IDS[\s\S]*validate_computed_text_indent[\s\S]*text_indent,/);
  assert.ok(text.runtimeTest.includes('anonymous_grid_computed_text_indent_reaches_intrinsic_tracks_and_first_line_position'));
  assert.match(text.validation, /parse_computed_text_indent[\s\S]*strip_suffix\("px"\)/);
  assert.match(text.validation,
    /stylo_flags_fallback_allowed[\s\S]*computed_value == Some\("0px"\)/);
  assert.ok(text.trace.includes('"textIndent": style.text_indent'));
  assert.ok(text.trace.includes('"indent": line.indent'));
  assert.ok(text.trace.includes('"textIndentEachLine": style.text_indent_flags.each_line'));
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
  assert.match(text.iosHost,
    /minContentHeight:\s*textIndent\.minContentHeight,\s*maxContentHeight:\s*textIndent\.whiteSpace == "PreLine"\s*\? textIndent\.minContentHeight : 10/);
  assert.ok(text.androidHost.includes('"textIndent", expected.textIndent'));
  assert.ok(text.androidHost.includes('"indent", expected.lineIndents[lineIndex]'));
  assert.ok(text.androidHost.includes('"textIndentEachLine"'));
  assert.ok(text.iosHost.includes('expected: expected.textIndent'));
  assert.ok(text.iosHost.includes('expected.lineIndents[lineIndex]'));
  assert.ok(text.iosHost.includes('textIndentEachLine'));
  assert.equal((text.iosHost.match(/c114IsMinContentTextTrackScene\(sceneID\)/g) ?? []).length, 2);
  assert.match(text.iosHost,
    /private func c114IsMinContentTextTrackScene\([\s\S]*?text-indent-flags-[\s\S]*?contains\("-min-content-"\)/);
  assert.match(text.iosHost, /let plainIndent = sceneID\.hasSuffix\("-indent"\)/);
  assert.ok(text.iosHost.includes(String.raw`SPINON_C114_GRID_ARGUMENT_ERROR run_id=\(c114GridTrackSizingRunID) scene_id=\(c114GridTrackSizingSceneID) rejected=true`));
  assert.ok(text.capture.includes("target: 'Android API 37 ARM64 AVD'"));
  assert.ok(text.capture.includes("target: 'iPhone 17 Pro / iOS 26.2 Simulator'"));
  assert.ok(text.capture.includes('expectedValues.cases'));
  assert.ok(text.capture.includes("['positive', 'negative', 'flags']"));
  assert.ok(text.capture.includes('flagsPrecomparison.cases'));
  assert.ok(text.capture.includes('scene.computedTextIndent'));
  assert.match(text.capture, /maxContentHeightPx: forcedBreak \? 20 : 10/);
  assert.ok(text.capture.includes("finalTrackSizes('Columns')"));
  assert.ok(text.capture.includes("finalTrackSizes('Rows')"));
  assert.ok(text.capture.includes("report.includes('status=0 presented boxes=1')"));
  assert.ok(text.capture.includes('root_css_px=120x${scene.rowsPx}'));
  assert.ok(text.capture.includes('function iosLog(udid, pid)'));
  assert.ok(text.capture.includes('processIdentifier == ${pid}'));
  assert.ok(text.capture.includes('draw_superseded_retries'));
  assert.match(text.artifactScanner,
    /expectedValues\.cases\.map\(\(\{ id \}\) => `text-indent-flags-\$\{id\}`\)/);
  assert.match(text.artifactScanner, /expectedPresent \? 2 : 0/);
  assert.match(text.artifactScanner, /expectedPresent \? 20 : 0/);
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
    assert.match(text.capture,
      /lineCount: candidate\.trackKind === 'min-content'\s*\? metric\.minContentRows : metric\.maxContentRows/);
    assert.equal(item.trackPx, expectedTrack, `${item.sceneId}: selected Grid track`);
    assert.ok(text.ffiScenes.includes(item.sceneId), `FFI generator is missing ${item.sceneId}`);
    assert.ok(text.sceneTable.includes(item.sceneId), `runtime allowlist is missing ${item.sceneId}`);
  }
  assert.match(text.validation, /-10\.0\.\.=10\.0/);
  assert.match(text.androidHost, /text-indent-negative-five-[\s\S]*text-indent-negative-ten-/);
  assert.match(text.iosHost, /text-indent-negative-five-[\s\S]*text-indent-negative-ten-/);
  assert.match(text.negativePlan, /12개/);
  assert.match(text.negativePlan, /-30px/);
});
