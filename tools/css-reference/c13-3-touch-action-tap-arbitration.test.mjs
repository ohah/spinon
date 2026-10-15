import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const read = (relativePath) => readFile(resolve(repositoryRoot, relativePath), 'utf8');

test('C13.3 tap probe stays on the explicit opt-in fixture route', async () => {
  const [android, activity, ios] = await Promise.all([
    read('platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'),
    read('platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java'),
    read('platforms/ios/Sources/C0410RuntimeGpuDemo.swift'),
  ]);
  assert.match(android,
    /SPINON_C133_GESTURE_FIXTURE_TEST\s*&&\s*activity\.getIntent\(\)\.getBooleanExtra\("spinon_c133_tap_probe"/);
  assert.match(android,
    /tap-probe-requires-fixture-route/);
  assert.match(android,
    /tap-probe-requires-scenario-3-or-4/);
  assert.match(activity,
    /spinon_c133_tap_probe[\s\S]*?SPINON_C133_GESTURE_FIXTURE_TEST/);
  assert.match(ios,
    /c133TapProbeRequested && !c133GestureFixtureRequested/);
  assert.match(ios,
    /tap-probe-requires-scenario-3-or-4/);
});

test('Android counts only guarded View.OnClickListener activations and forwards accepted touch streams', async () => {
  const android = await read('platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java');
  assert.match(android,
    /surfaceView\.setOnClickListener\(view -> onC133TapProbeClick\(\)\)/);
  assert.match(android,
    /wasForwardingTapGesture \|\| c133TapForwardingGesture \? false : handled/);
  assert.match(android,
    /action == MotionEvent\.ACTION_MOVE[\s\S]*?wasForwardingTapGesture && !panWasStarted && c133PanStarted[\s\S]*?surfaceView\.setPressed\(false\)[\s\S]*?surfaceView\.cancelLongPress\(\)[\s\S]*?return true/);
  assert.match(android,
    /c133TapDownAccepted\s*&&\s*c133TouchActive && !c133PanStarted && !c133TerminalQueued/);
  assert.match(android,
    /private void onC133TapProbeClick\(\)[\s\S]*?if \(!c133TapClickCandidate\)[\s\S]*?c133TapActivationCount\+\+/);
});

test('Android absolute input maps from the full SurfaceView origin, not its clipped accessibility bounds', async () => {
  const [android, capture] = await Promise.all([
    read('platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'),
    read('tools/css-reference/capture-c13-3-touch-action-tap-arbitration.mjs'),
  ]);
  assert.match(android,
    /surfaceView\.getLocationOnScreen\(surfaceOriginOnScreen\)[\s\S]*?surface_origin_screen_px=/);
  assert.match(android,
    /surface_local_physical_px=[\s\S]*?screen_physical_px=[\s\S]*?surface_origin_screen_px=[\s\S]*?surface_view_size_px=/);
  assert.match(capture,
    /const clippedWidth = viewSize\.width - visibleWidth;[\s\S]*?bounds\.left - clippedWidth \/ 2[\s\S]*?bounds\.top - clippedHeight \/ 2/);
  assert.match(capture,
    /function assertAndroidTouchTransform[\s\S]*?Android runtime SurfaceView origin differs[\s\S]*?Android runtime SurfaceView size differs[\s\S]*?Android runtime screen coordinate differs/);
  assert.match(capture,
    /decodeRgbaPng[\s\S]*?C13\.3 scene background is missing[\s\S]*?scrolled inner marker is not visible/);
});

test('iOS tap waits for pan failure and configures pan from computed touch-action before readiness', async () => {
  const ios = await read('platforms/ios/Sources/C0410RuntimeGpuDemo.swift');
  assert.match(ios,
    /tapRecognizer\.require\(toFail: recognizer\)/);
  assert.match(ios,
    /let panEnabled = actualTouchAction != "none"[\s\S]*?c133PanRecognizer\?\.isEnabled = panEnabled[\s\S]*?requestDraw\(\)/);
  assert.match(ios,
    /recognizer\.state == \.recognized[\s\S]*?c133TapActivationCount \+= 1/);
  assert.match(ios,
    /tap-probe-requires-fixture-route/);
});

test('tap plan has twenty distinct pre-implementation failure perspectives', async () => {
  const review = await read('spec/internal/evidence/c13-3-touch-action-tap-arbitration-plan-review-20-2026-10-15.md');
  const rows = [...review.matchAll(/^\|\s*(\d+)\s*\|/gm)].map((match) => Number(match[1]));
  assert.deepEqual(rows, Array.from({ length: 20 }, (_, index) => index + 1));
});
