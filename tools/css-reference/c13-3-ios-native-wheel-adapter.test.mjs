import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const iosSourcePath = resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
const readIosSource = () => readFile(iosSourcePath, 'utf8');

test('iOS native wheel adapter launch markers are emitted only in the opt-in fixture build', async () => {
  const ios = await readIosSource();
  const marker = '--spinon-c133-native-wheel-adapter';
  const occurrences = [...ios.matchAll(/--spinon-c133-native-wheel-adapter/g)];
  assert.equal(occurrences.length, 2, 'the adapter marker should have only its route and allowlist occurrences');
  for (const occurrence of occurrences) {
    const featureStart = ios.lastIndexOf('#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1', occurrence.index);
    const featureEnd = ios.indexOf('#endif', occurrence.index);
    assert.ok(featureStart >= 0 && featureEnd > occurrence.index, `${marker} must be compile-time gated`);
  }
  assert.ok(
    /#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1\s+let c133NativeWheelAdapterArguments: Set<String> = \[\s*"--spinon-c133-native-wheel-adapter",\s*\]\s*#else\s+let c133NativeWheelAdapterArguments: Set<String> = \[\]\s*#endif/.test(ios),
    'default builds must use an empty adapter argument allowlist',
  );
  assert.ok(
    /c133GestureFixtureRequested\s*&& c133NativeWheelAdapterArguments\.contains\(\$0\)/.test(ios),
    'adapter argument must be allowlisted only for the fixture route',
  );
});

test('iOS recognizers separate mouse and trackpad scroll without accepting touch input', async () => {
  const ios = await readIosSource();
  assert.match(ios,
    /discreteRecognizer\.allowedScrollTypesMask = \.discrete[\s\S]*?discreteRecognizer\.allowedTouchTypes = \[\][\s\S]*?c133NativeWheelDiscreteRecognizer = discreteRecognizer/);
  assert.match(ios,
    /continuousRecognizer\.allowedScrollTypesMask = \.continuous[\s\S]*?continuousRecognizer\.allowedTouchTypes = \[\][\s\S]*?c133NativeWheelContinuousRecognizer = continuousRecognizer/);
  assert.match(ios,
    /if recognizer === c133NativeWheelDiscreteRecognizer\s*\{\s*source = 0[\s\S]*?\} else if recognizer === c133NativeWheelContinuousRecognizer\s*\{\s*source = 1/);
});

test('iOS wheel adapter differences cumulative translation and rebases one sample on surface replacement', async () => {
  const ios = await readIosSource();
  const handlerStart = ios.indexOf('@objc private func handleC133NativeWheel(');
  const handlerEnd = ios.indexOf('\n    private func beginC133Pan(', handlerStart);
  assert.notEqual(handlerStart, -1);
  assert.notEqual(handlerEnd, -1);
  const handler = ios.slice(handlerStart, handlerEnd);
  assert.match(handler,
    /let previousTranslation = previous\?\.translation \?\? \.zero[\s\S]*?x: translation\.x - previousTranslation\.x[\s\S]*?y: translation\.y - previousTranslation\.y/);
  assert.match(handler,
    /c133NativeWheelTranslations\[identifier\] = \([\s\S]*?surfaceGeneration: generation[\s\S]*?guard !generationChanged/);
  assert.match(handler, /case \.began, \.changed, \.ended:/);
  assert.match(handler, /defer \{[\s\S]*?if isTerminal \{[\s\S]*?removeValue\(forKey: identifier\)/);
});

test('iOS wheel adapter rejects control, invalid scale, outside points, and zero deltas before FFI enqueue', async () => {
  const ios = await readIosSource();
  const handlerStart = ios.indexOf('@objc private func handleC133NativeWheel(');
  const handlerEnd = ios.indexOf('\n    private func beginC133Pan(', handlerStart);
  const handler = ios.slice(handlerStart, handlerEnd);
  const enqueue = handler.indexOf('SpinonRunner.enqueueRuntimeGpuC13_3Wheel(');
  assert.notEqual(enqueue, -1);
  for (const rejection of [
    'reason=nonfinite-translation',
    'reason=surface-generation-changed',
    'reason=control-wheel',
    'reason=surface-or-runtime-not-ready',
    'reason=invalid-coordinate-scale',
    'reason=point-outside-surface',
    'reason=nonfinite-normalized-sample',
    'reason=zero-delta',
  ]) {
    assert.ok(handler.indexOf(rejection) >= 0, `missing fail-closed rejection ${rejection}`);
    assert.ok(handler.indexOf(rejection) < enqueue, `${rejection} must be checked before enqueue`);
  }
  assert.match(handler, /deltaXCss = Float\(-delta\.x \/ pointsPerCssX\)/);
  assert.match(handler, /deltaYCss = Float\(-delta\.y \/ pointsPerCssY\)/);
  assert.match(handler, /location\.x >= bounds\.minX[\s\S]*?location\.x < bounds\.maxX/);
});

test('iOS wheel adapter preserves normalized input metadata and schedules drain only after admission', async () => {
  const ios = await readIosSource();
  const handlerStart = ios.indexOf('@objc private func handleC133NativeWheel(');
  const handlerEnd = ios.indexOf('\n    private func beginC133Pan(', handlerStart);
  const handler = ios.slice(handlerStart, handlerEnd);
  assert.match(handler,
    /sourceUnit: 2,[\s\S]*?coordinateScale: coordinateScale,[\s\S]*?source: source,[\s\S]*?momentumPhase: 0,[\s\S]*?modifierFlags: normalizedModifiers/);
  assert.match(handler,
    /report\.hasPrefix\("status=0 "\) && report\.contains\("admitted=true"\)\s*\{\s*scheduleC133Drain\(\)/);
  assert.match(handler, /momentum_phase=None/);
});
