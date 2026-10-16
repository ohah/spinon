import assert from 'node:assert/strict';
import { existsSync } from 'node:fs';
import { readdir, readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const legacyFixtureSources = [
  'crates/spinon-ffi/src/runtime_gpu/c11_3_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/c11_3_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/c11.rs',
  'crates/spinon-ffi/src/runtime_gpu/c11_2_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/c11_2_3.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c11_3_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c11_3_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c06.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c07.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c08.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c09.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c10.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c10_3_5.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c11.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c11_2_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c11_2_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c11_2_3.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c12.rs',
  'crates/spinon-ffi/src/runtime_ua_cascade/probe.rs',
  'crates/spinon-ffi/src/runtime_ua_cascade/layout_probe.rs',
];
const canonicalFixtureSources = [
  'crates/spinon-ffi/src/runtime_gpu/fixtures.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_3_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_3_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_2_scroll.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_2_scroll_host.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_3_scroll_gesture.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_3_scroll_gesture_host.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11_3_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11_3_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_3.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c18_1.rs',
  'crates/spinon-ffi/src/runtime_ua_cascade/fixtures.rs',
  'crates/spinon-ffi/src/runtime_ua_cascade/fixtures/probe.rs',
  'crates/spinon-ffi/src/runtime_ua_cascade/fixtures/layout_probe.rs',
];
test('fixture 이동 뒤 이전 경로에 Rust 사본이 남지 않는다', () => {
  for (const relativePath of legacyFixtureSources) {
    assert.equal(
      existsSync(resolve(repositoryRoot, relativePath)),
      false,
      `이전 fixture 경로에 소스가 남았습니다: ${relativePath}`,
    );
  }
});

test('CSS reference 도구가 삭제된 Rust fixture 경로를 참조하지 않는다', async () => {
  const entries = await readdir(resolve(repositoryRoot, 'tools/css-reference'));
  const sources = await Promise.all(entries
    .filter((entry) => entry.endsWith('.mjs') && entry !== 'fixture-module-boundary.test.mjs')
    .map((entry) => readFile(resolve(repositoryRoot, 'tools/css-reference', entry), 'utf8')));
  const staleReferences = legacyFixtureSources.filter((legacyPath) =>
    sources.some((source) => source.includes(legacyPath)));

  assert.deepEqual(staleReferences, []);
});

test('이동한 fixture 구현은 정해진 fixtures 모듈 경로에 있다', () => {
  for (const relativePath of canonicalFixtureSources) {
    assert.equal(
      existsSync(resolve(repositoryRoot, relativePath)),
      true,
      `정식 fixture 모듈 경로가 없습니다: ${relativePath}`,
    );
  }
});

test('작업별 FFI fixture 모듈은 runtime 소스 루트에 남지 않는다', async () => {
  const misplaced = [];
  const queue = [['crates/spinon-ffi/src', '']];
  while (queue.length > 0) {
    const [absoluteDirectory, relativeDirectory] = queue.pop();
    const entries = await readdir(resolve(repositoryRoot, absoluteDirectory), { withFileTypes: true });
    for (const entry of entries) {
      const relativePath = relativeDirectory ? `${relativeDirectory}/${entry.name}` : entry.name;
      if (entry.isDirectory()) {
        queue.push([`${absoluteDirectory}/${entry.name}`, relativePath]);
      } else if (
        entry.isFile()
        && /^c\d+(?:_\d+)*\.rs$/.test(entry.name)
        && !relativePath.split('/').includes('fixtures')
      ) {
        misplaced.push(`${absoluteDirectory}/${entry.name}`);
      }
    }
  }
  assert.deepEqual(misplaced, [], `fixture 모듈을 fixtures/ 아래로 옮기세요: ${misplaced.join(', ')}`);
});

test('앱 fixture adapter와 include 데이터는 opt-in Cargo feature에 격리된다', async () => {
  const runtimeSource = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu.rs'),
    'utf8',
  );
  const ffiSource = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi.rs'),
    'utf8',
  );
  const runtimeFixtureSource = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures.rs'),
    'utf8',
  );
  const ffiFixtureSource = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures.rs'),
    'utf8',
  );
  const c13FixtureSource = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_1.rs'),
    'utf8',
  );
  const c13FfiFixtureSource = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_1.rs'),
    'utf8',
  );
  const c13BorderRuntimeFixture = await readFile(
    resolve(repositoryRoot, 'tests/fixtures/css/c13/runtime-overflow-clip-hit-border-c13-1.js'),
    'utf8',
  );
  const c13ScrollHostFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_2_scroll_host.rs'),
    'utf8',
  );
  const c13ScrollFfiFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_2.rs'),
    'utf8',
  );
  const c13ScrollRuntimeFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_2_scroll.rs'),
    'utf8',
  );
  const c13GestureRuntimeFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_3_scroll_gesture.rs'),
    'utf8',
  );
  const c13GestureHostFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_3_scroll_gesture_host.rs'),
    'utf8',
  );
  const c13GestureFfiFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_3.rs'),
    'utf8',
  );
  const runtimeFfiHeader = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/include/spinon_ffi.h'),
    'utf8',
  );
  const c13GestureRuntimeScript = await readFile(
    resolve(repositoryRoot, 'tests/fixtures/css/c13/runtime-scroll-gesture-c13-3.js'),
    'utf8',
  );
  const c18RuntimeFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_1.rs'),
    'utf8',
  );
  const c18FfiFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c18_1.rs'),
    'utf8',
  );
  const c18RuntimeScript = await readFile(
    resolve(repositoryRoot, 'tests/fixtures/css/c18/runtime-image-overflow-c18-1.js'),
    'utf8',
  );
  const cargoManifest = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/Cargo.toml'),
    'utf8',
  );
  const androidJniSource = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/main/cpp/spinon_jni.cc'),
    'utf8',
  );
  const androidGradleSource = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/build.gradle.kts'),
    'utf8',
  );
  const androidActivitySource = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java'),
    'utf8',
  );
  const androidGpuDemoSource = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'),
    'utf8',
  );
  const androidImagePaintCaptureAdapter = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/c18_1_image_paint_fixture_test/java/dev/spinon/bootstrap/ImagePaintFixtureCaptureAdapter.java'),
    'utf8',
  );
  const androidImagePaintNativeAdapter = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/c18_1_image_paint_fixture_test/java/dev/spinon/bootstrap/ImagePaintFixtureNativeAdapter.java'),
    'utf8',
  );
  const androidBuildScript = await readFile(
    resolve(repositoryRoot, 'tools/build-android.sh'),
    'utf8',
  );
  const iosRunnerHeader = await readFile(
    resolve(repositoryRoot, 'platforms/ios/Sources/SpinonRunner.h'),
    'utf8',
  );
  const iosRunnerImplementation = await readFile(
    resolve(repositoryRoot, 'platforms/ios/Sources/SpinonRunner.mm'),
    'utf8',
  );
  const iosGpuDemoSource = await readFile(
    resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'),
    'utf8',
  );
  const iosProject = await readFile(
    resolve(repositoryRoot, 'platforms/ios/SpinonBootstrap.xcodeproj/project.pbxproj'),
    'utf8',
  );
  assert.match(runtimeSource, /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*mod fixtures;/);
  assert.match(ffiSource, /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*mod fixtures;/);
  assert.match(cargoManifest, /c04-runtime-gpu-fixture-test\s*=\s*\["c04-runtime-gpu"\]/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c13-1-overflow-clip-fixture-test"\)\]\s*pub\(crate\) mod c13_1;/);
  assert.match(ffiFixtureSource,
    /#\[cfg\(feature = "c13-1-overflow-clip-fixture-test"\)\]\s*mod c13_1;/);
  assert.match(cargoManifest,
    /c13-1-overflow-clip-fixture-test\s*=\s*\["c04-runtime-gpu-fixture-test",\s*"dep:sha2"\]/);
  assert.match(c13FixtureSource,
    /RUNTIME_CSS_C13_1_BORDER_FIXTURE_SOURCE[\s\S]*runtime-overflow-clip-hit-border-c13-1\.js/);
  assert.match(c13FfiFixtureSource,
    /spinon_runtime_gpu_host_new_c13_1_overflow_clip_border_fixture/);
  assert.match(c13FfiFixtureSource,
    /spinon_runtime_gpu_host_eval_c13_1_overflow_clip_border_fixture/);
  assert.match(c13BorderRuntimeFixture,
    /data-c131-border-node/);
  assert.match(androidGpuDemoSource,
    /nativeCreateC131OverflowClipBorderHost\(\)/);
  assert.match(androidGpuDemoSource,
    /nativeEvalC131OverflowClipBorderFixture\(long host\)/);
  assert.match(androidJniSource,
    /Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC131OverflowClipBorderHost/);
  assert.match(androidJniSource,
    /Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC131OverflowClipBorderFixture/);
  assert.match(androidActivitySource,
    /spinon_c131_overflow_clip_border/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c13-2-scroll-fixture-test"\)\]\s*pub\(crate\) mod c13_2_scroll;/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c13-2-scroll-fixture-test"\)\]\s*mod c13_2_scroll_host;/);
  assert.match(ffiFixtureSource,
    /#\[cfg\(feature = "c13-2-scroll-fixture-test"\)\]\s*mod c13_2;/);
  assert.match(cargoManifest,
    /c13-2-scroll-fixture-test\s*=\s*\["c04-runtime-gpu-fixture-test",\s*"dep:spinon-core"\]/);
  assert.match(c13ScrollRuntimeFixture,
    /runtime-scroll-container-c13-2\.js/);
  assert.match(c13ScrollHostFixture,
    /border_escape_outside_parent=true[\s\S]*border_escape_hit=matched/);
  assert.match(c13ScrollFfiFixture,
    /spinon_runtime_gpu_host_new_c13_2_scroll_fixture/);
  assert.match(c13ScrollFfiFixture,
    /spinon_runtime_gpu_host_eval_c13_2_scroll_fixture/);
  assert.match(androidGpuDemoSource,
    /c132ScrollFixtureRequested\s*\?\s*"SPINON · C13\.2 scroll state"/);
  assert.match(androidGpuDemoSource,
    /c132ScrollFixtureRequested\s*\?\s*"V8 scroll owner·offset·paint\/hit → WGPU · 5 PixelCopy 표본 · 400×240 CSS px"/);
  assert.match(androidGpuDemoSource,
    /c132ScrollFixtureRequested\s*\?\s*"C13\.2 scroll owner·offset·paint\/hit WGPU 장면"/);
  assert.match(androidGpuDemoSource,
    /c132ScrollFixtureRequested\s*\|\|\s*c133GestureFixtureRequested\s*\?\s*400/);
  assert.match(androidGpuDemoSource,
    /if \(c132ScrollFixtureRequested \|\| c133GestureFixtureRequested\) return 400\.0f;/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c13-3-gesture-fixture-test"\)\]\s*pub\(crate\) mod c13_3_scroll_gesture;/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c13-3-gesture-fixture-test"\)\]\s*pub\(super\) mod c13_3_scroll_gesture_host;/);
  assert.match(ffiFixtureSource,
    /#\[cfg\(feature = "c13-3-gesture-fixture-test"\)\]\s*pub\(super\) mod c13_3;/);
  assert.match(cargoManifest,
    /c13-3-gesture-fixture-test\s*=\s*\[[\s\S]*?"c04-runtime-gpu-fixture-test"[\s\S]*?"dep:spinon-core"[\s\S]*?"spinon-render\/c13-3-gesture-fixture-test"[\s\S]*?"spinon-runtime\/c13-3-gesture-fixture-test"[\s\S]*?\]/);
  assert.match(c13GestureRuntimeFixture,
    /RUNTIME_CSS_C13_3_SCROLL_GESTURE_SOURCE[\s\S]*runtime-scroll-gesture-c13-3\.js/);
  assert.match(c13GestureHostFixture,
    /C13_3GestureFixtureScenario[\s\S]*RUNTIME_CSS_C13_3_SCROLL_GESTURE_SOURCE/);
  assert.match(c13GestureFfiFixture,
    /spinon_runtime_gpu_host_new_c13_3_gesture_fixture/);
  assert.match(c13GestureFfiFixture,
    /spinon_runtime_gpu_host_eval_c13_3_gesture_fixture/);
  assert.match(c13GestureFfiFixture,
    /spinon_runtime_gpu_host_enqueue_c13_3_gesture/);
  assert.match(c13GestureFfiFixture,
    /spinon_runtime_gpu_host_enqueue_c13_3_wheel/);
  assert.match(runtimeFfiHeader,
    /#if defined\(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_enqueue_c13_3_wheel/);
  assert.match(c13GestureFfiFixture,
    /spinon_runtime_gpu_host_process_c13_3_gesture_queue/);
  assert.match(c13GestureRuntimeScript,
    /data-c133-gesture-node/);
  assert.match(androidBuildScript,
    /ffi_features\+=\(c13-3-gesture-fixture-test\)/);
  assert.match(androidBuildScript,
    /c133_gesture_fixture_cpp_flag=.*SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST=.*SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST/);
  assert.match(androidJniSource,
    /#if defined\(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c13_3_gesture_fixture/);
  assert.match(androidGpuDemoSource,
    /nativeCreateC133GestureHost\(int scenarioId\)/);
  assert.match(androidGpuDemoSource,
    /nativeProcessC133GestureQueue\(long host\)/);
  assert.match(androidGpuDemoSource,
    /nativeEnqueueC133Wheel\([\s\S]*?sourceUnit, float coordinateScale, int source/);
  assert.match(androidGpuDemoSource,
    /KeyEvent\.normalizeMetaState\(metaState\)/);
  assert.match(androidGpuDemoSource,
    /normalizedMetaState & KeyEvent\.META_CTRL_ON/);
  assert.match(androidJniSource,
    /#if defined\(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_enqueue_c13_3_wheel/);
  assert.doesNotMatch(androidGpuDemoSource, /SPINON_C133_SURFACE_BOUNDS/);
  assert.match(androidGpuDemoSource,
    /new StringBuilder\("SPINON_C133_SURFACE"\)[\s\S]*?append\("_BOUNDS"\)/);
  assert.doesNotMatch(androidGpuDemoSource, /SPINON_C133_STYLE_UPDATE /);
  assert.doesNotMatch(androidGpuDemoSource, /c13_3_style_invalidation=applied/);
  assert.match(androidGpuDemoSource,
    /\.append\("SPINON_C133_"\)[\s\S]*?\.append\("STYLE_UPDATE "\)/);
  assert.match(androidActivitySource,
    /spinon_c133_gesture_fixture/);
  assert.match(iosProject,
    /SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST=\$\(SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST\)/);
  assert.match(iosProject,
    /SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_\$\(SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST\)/);
  assert.match(iosRunnerHeader,
    /#if defined\(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST\)[\s\S]*?createRuntimeGpuHostWithC13_3GestureFixture/);
  assert.match(iosRunnerImplementation,
    /#if defined\(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c13_3_gesture_fixture/);
  assert.match(iosRunnerHeader,
    /enqueueRuntimeGpuC13_3Wheel:[\s\S]*?modifierFlags:/);
  assert.match(iosRunnerImplementation,
    /enqueueRuntimeGpuC13_3Wheel:[\s\S]*?spinon_runtime_gpu_host_enqueue_c13_3_wheel/);
  assert.match(iosGpuDemoSource,
    /#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1[\s\S]*?withC13_3GestureFixture:/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c18-1-image-paint-fixture-test"\)\]\s*pub\(super\) mod c18_1;/);
  assert.match(ffiFixtureSource,
    /#\[cfg\(feature = "c18-1-image-paint-fixture-test"\)\]\s*mod c18_1;/);
  assert.match(cargoManifest,
    /c18-1-image-paint-fixture-test\s*=\s*\[[\s\S]*?"c04-runtime-gpu-fixture-test"[\s\S]*?"dep:spinon-image"[\s\S]*?"spinon-runtime\/c14-1-live-v8-fixture-test"[\s\S]*?\]/);
  assert.match(c18RuntimeFixture,
    /RUNTIME_CSS_C18_1_IMAGE_PAINT_FIXTURE_SOURCE[\s\S]*runtime-image-overflow-c18-1\.js/);
  assert.match(c18RuntimeFixture,
    /include_bytes![\s\S]*c14-1-png-basic\.png/);
  assert.match(runtimeSource,
    /fn set_environment\([\s\S]*?#\[cfg\(feature = "c18-1-image-paint-fixture-test"\)\][\s\S]*?let attach_c18_1_images = self\.c18_1_image_paint_fixture[\s\S]*?c18_1_image_document_ready[\s\S]*?Ordering::Acquire[\s\S]*?self\.wait_and_publish\([\s\S]*?WaitAndPublishOptions\s*\{[\s\S]*?attach_c18_1_images,/);
  assert.match(c18FfiFixture,
    /spinon_runtime_gpu_host_new_c18_1_image_paint_fixture[\s\S]*spinon_runtime_gpu_host_request_c18_1_surface_pixels_for_test/);
  assert.match(c18RuntimeScript,
    /spinon-fixture:c18-1\/static-png-visible[\s\S]*spinon-fixture:c18-1\/static-png-hidden/);
  assert.match(runtimeFfiHeader,
    /#if defined\(SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_request_c18_1_surface_pixels_for_test/);
  assert.match(androidBuildScript,
    /ffi_features\+=\(c18-1-image-paint-fixture-test\)/);
  assert.match(androidBuildScript,
    /c181_image_paint_cpp_flag=.*SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST/);
  assert.match(androidJniSource,
    /#if defined\(SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c18_1_image_paint_fixture/);
  assert.match(androidImagePaintCaptureAdapter,
    /SurfaceViewPixelCapture\.requestForCssViewport\(surfaceView,[\s\S]*?"c181-image-overflow"/);
  assert.match(androidImagePaintCaptureAdapter,
    /getLocationOnScreen\(contentOrigin\)[\s\S]*?surfaceView\.getLocationOnScreen\(surfaceOrigin\)[\s\S]*?SPINON_C181_SCREEN_GEOMETRY[\s\S]*?surface_view_size_px=/);
  assert.doesNotMatch(androidGpuDemoSource,
    /nativeRequestC181SurfacePixelsForTest/);
  assert.doesNotMatch(androidGpuDemoSource,
    /private\s+static\s+native\s+[^;]*(?:C181|c18_1)/i);
  assert.match(androidGpuDemoSource,
    /cssViewportWidthLocked\(\)\s*\{[\s\S]*?imagePaintProbeRequested\) return 80\.0f/);
  assert.match(androidGpuDemoSource,
    /cssViewportHeightLocked\(\)\s*\{[\s\S]*?imagePaintProbeRequested\) return 40\.0f/);
  assert.match(androidActivitySource,
    /BuildConfig\.SPINON_IMAGE_PAINT_FIXTURE_TEST[\s\S]*spinon_c181_image_paint/);
  assert.match(androidGradleSource,
    /buildConfigField\([\s\S]*?"SPINON_IMAGE_PAINT_FIXTURE_TEST"[\s\S]*?spinonC181ImagePaintFixtureTestValue/);
  assert.match(androidGradleSource,
    /if \(spinonC181ImagePaintFixtureTestValue == "1"\)\s*\{\s*java\.srcDir\("src\/c18_1_image_paint_fixture_test\/java"\)/);
  assert.match(androidImagePaintCaptureAdapter,
    /SPINON_C181_PIXELCOPY_BEGIN[\s\S]*SPINON_C181_PIXELCOPY_SUMMARY/);
  assert.match(androidImagePaintNativeAdapter,
    /native long createHost\(\);[\s\S]*native byte\[\] evalFixture\(long host\);/);
  assert.match(iosProject,
    /SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST=\$\(SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST\)/);
  assert.match(iosRunnerImplementation,
    /#if defined\(SPINON_ENABLE_C04_RUNTIME_GPU\)[\s\S]*?SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST[\s\S]*?spinon_runtime_gpu_host_new_c18_1_image_paint_fixture/);
  assert.match(iosGpuDemoSource,
    /imagePaintProbeRequested[\s\S]*?requestC181MetalPixelReadback/);
});
