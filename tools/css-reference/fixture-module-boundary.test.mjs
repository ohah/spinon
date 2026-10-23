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
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_named_lines.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_template_area_auto_repeat.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_4.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_4/fixture.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_4/scene.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_4/tests.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c15_3.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11_3_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11_3_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_3.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c18_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c18_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c18_4.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c15_3.rs',
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

test('양 축 template-area auto-repeat 아홉 scene이 두 플랫폼 경로에 고정된다', async () => {
  const expectedIDs = [
    'template-area-repeat-two-axis-area-short',
    'template-area-repeat-two-axis-area-added-columns',
    'template-area-repeat-two-axis-area-added-rows',
    'template-area-repeat-two-axis-area-added-both',
    'template-area-repeat-two-axis-fill-fit',
    'template-area-repeat-two-axis-fit-fill',
    'template-area-repeat-two-axis-fit-fit-empty',
    'template-area-repeat-two-axis-fit-fit-span',
    'template-area-repeat-two-axis-area-longhand',
  ];
  const [fixtureText, android, ios] = await Promise.all([
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/template_area_auto_repeat_scenes.json'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'),
    'utf8'),
    readFile(resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8'),
  ]);
  const fixtures = JSON.parse(fixtureText);
  assert.equal(fixtures.length, 22, 'template-area fixture는 기존 13개와 신규 9개다.');
  assert.deepEqual(
    fixtures.map(({ id }) => id).filter((id) => id.startsWith('template-area-repeat-two-axis-')),
    expectedIDs,
  );
  const overflow = fixtures.find(({ id }) => id === 'template-area-repeat-two-axis-area-added-both');
  assert.deepEqual(overflow.expect.root, [40, 40]);
  assert.deepEqual(overflow.expect.items, [{ name: 'hero', frame: [44, 40, 8, 8] }]);
  assert.deepEqual(overflow.expect.tracks, {
    columns: [20, 16, 16],
    rows: [20, 12, 12],
  });
  for (const id of expectedIDs) {
    assert.ok(android.includes(`"${id}"`), `Android allowlist is missing ${id}`);
    assert.ok(ios.includes(`"${id}"`), `iOS allowlist is missing ${id}`);
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
  const c114RuntimeFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'),
    'utf8',
  );
  const c114NamedLineRuntimeFixture = await readFile(
    resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_named_lines.rs'),
    'utf8',
  );
  const c114TemplateAreaRuntimeFixture = await readFile(
    resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_template_area_auto_repeat.rs'),
    'utf8',
  );
  const c114RuntimeFixtureHost = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'),
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
  const c182RuntimeFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_2.rs'),
    'utf8',
  );
  const c182FfiFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c18_2.rs'),
    'utf8',
  );
  const c184RuntimeFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_4.rs'),
    'utf8',
  );
  const c184FixtureImplementation = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_4/fixture.rs'),
    'utf8',
  );
  const c184FfiFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c18_4.rs'),
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
  const c182RuntimeScript = await readFile(
    resolve(repositoryRoot, 'tests/fixtures/css/c18/runtime-image-alpha-c18-2.js'),
    'utf8',
  );
  const c184RuntimeScript = await readFile(
    resolve(repositoryRoot, 'tests/fixtures/css/c18/runtime-image-async-replacement-c18-4.js'),
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
  const androidAlphaImageCaptureAdapter = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/c18_2_alpha_image_fixture_test/java/dev/spinon/bootstrap/AlphaImageFixtureCaptureAdapter.java'),
    'utf8',
  );
  const androidAsyncImageCaptureAdapter = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/c18_4_async_image_fixture_test/java/dev/spinon/bootstrap/AsyncImageFixtureCaptureAdapter.java'),
    'utf8',
  );
  const androidAsyncImageNativeAdapter = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/c18_4_async_image_fixture_test/java/dev/spinon/bootstrap/AsyncImageFixtureNativeAdapter.java'),
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
  const iosAppDelegateSource = await readFile(
    resolve(repositoryRoot, 'platforms/ios/Sources/AppDelegate.swift'),
    'utf8',
  );
  const iosBuildSimulatorScript = await readFile(
    resolve(repositoryRoot, 'tools/build-ios-sim.sh'),
    'utf8',
  );
  const iosProject = await readFile(
    resolve(repositoryRoot, 'platforms/ios/SpinonBootstrap.xcodeproj/project.pbxproj'),
    'utf8',
  );
  assert.match(runtimeSource, /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*mod fixtures;/);
  assert.match(ffiSource, /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*mod fixtures;/);
  assert.match(cargoManifest, /c04-runtime-gpu\s*=\s*\[[^\]]*"dep:spinon-render-wgpu"[^\]]*\]/);
  const runtimeGpuFixtureFeature = cargoManifest.match(
    /^c04-runtime-gpu-fixture-test\s*=\s*\[([\s\S]*?)^\]/m,
  );
  assert.ok(runtimeGpuFixtureFeature, 'Runtime GPU fixtures stay behind the fixture-test feature');
  assert.match(runtimeGpuFixtureFeature[1], /"c04-runtime-gpu"/);
  assert.match(runtimeGpuFixtureFeature[1], /"spinon-runtime\/c11-4-grid-track-sizing-fixture-test"/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c13-1-overflow-clip-fixture-test"\)\]\s*pub\(crate\) mod c13_1;/);
  assert.match(runtimeFixtureSource,
    /pub\(crate\) mod c11_4_auto_repeat_named_lines;/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*pub\(crate\) mod c11_4_template_area_auto_repeat;/);
  assert.match(c114RuntimeFixtureHost,
    /super::c11_4_template_area_auto_repeat::runtime_source\(scene_id\)/);
  assert.match(c114TemplateAreaRuntimeFixture,
    /include_str!\("c11_4\/template_area_auto_repeat_scenes\.json"\)/);
  assert.match(c114RuntimeFixture,
    /super::c11_4_auto_repeat_named_lines::runtime_source/);
  assert.match(c114NamedLineRuntimeFixture,
    /include_str!\("c11_4\/auto_repeat_named_line_scenes\.json"\)/);
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
    /cssViewportWidthLocked\(\)\s*\{[\s\S]*?imagePaintProbeRequested\)\s*\{\s*return imageAsyncReplacementProbeRequested \|\| imageObjectFitPositionProbeRequested\s*\? 224\.0f : 80\.0f/);
  assert.match(androidGpuDemoSource,
    /cssViewportHeightLocked\(\)\s*\{[\s\S]*?imagePaintProbeRequested\)\s*\{\s*return imageAsyncReplacementProbeRequested \? 88\.0f\s*:\s*imageObjectFitPositionProbeRequested \? 72\.0f : 40\.0f/);
  assert.match(androidActivitySource,
    /BuildConfig\.SPINON_IMAGE_PAINT_FIXTURE_TEST[\s\S]*spinon_c181_image_paint/);
  assert.match(androidGradleSource,
    /buildConfigField\([\s\S]*?"SPINON_IMAGE_PAINT_FIXTURE_TEST"[\s\S]*?spinonC181ImagePaintFixtureTestValue/);
  assert.match(androidGradleSource,
    /if \(spinonImagePaintFixtureTestValue\)\s*\{\s*java\.srcDir\("src\/c18_1_image_paint_fixture_test\/java"\)/);
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
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c18-2-alpha-image-compositing-fixture-test"\)\]\s*pub\(super\) mod c18_2;/);
  assert.match(ffiFixtureSource,
    /#\[cfg\(feature = "c18-2-alpha-image-compositing-fixture-test"\)\]\s*mod c18_2;/);
  assert.match(cargoManifest,
    /c18-2-alpha-image-compositing-fixture-test\s*=\s*\["c18-1-image-paint-fixture-test"\]/);
  assert.match(c182RuntimeFixture,
    /include_str![\s\S]*runtime-image-alpha-c18-2\.js[\s\S]*include_bytes![\s\S]*c18-2-alpha-source-over\.png/);
  assert.match(c182RuntimeFixture,
    /C18\.2 img src가 고정 입력과 다릅니다[\s\S]*C18\.2 img 수가 다릅니다/);
  assert.match(c182FfiFixture,
    /spinon_runtime_gpu_host_new_c18_2_alpha_image_fixture[\s\S]*spinon_runtime_gpu_host_request_c18_2_surface_pixels_for_test/);
  assert.match(c182RuntimeScript,
    /spinon-fixture:c18-2\/alpha-source-over/);
  assert.match(runtimeFfiHeader,
    /#if defined\(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_request_c18_2_surface_pixels_for_test/);
  assert.match(androidBuildScript,
    /ffi_features\+=\(c18-2-alpha-image-compositing-fixture-test\)/);
  assert.match(androidBuildScript,
    /c182_alpha_image_cpp_flag=.*SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST/);
  assert.match(androidJniSource,
    /#if defined\(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c18_2_alpha_image_fixture/);
  assert.match(iosBuildSimulatorScript,
    /c18_image_paint_shared=\$\{c18_image_paint_shared\} c18_1_image_paint_fixture=\$\{SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST:-0\} c18_2_alpha_image_fixture=\$\{SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST:-0\}/);
  assert.match(iosProject,
    /SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST=\$\(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST\)/);
  assert.match(iosProject,
    /SPINON_ENABLE_C18_2_ALPHA_IMAGE_FIXTURE_TEST_\$\(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST\)/);
  assert.match(iosRunnerImplementation,
    /#if defined\(SPINON_ENABLE_C04_RUNTIME_GPU\)[\s\S]*?SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST[\s\S]*?spinon_runtime_gpu_host_new_c18_2_alpha_image_fixture/);
  assert.match(androidAlphaImageCaptureAdapter,
    /SPINON_C182_PIXELCOPY_SAMPLE[\s\S]*SPINON_C182_PIXELCOPY_SUMMARY[\s\S]*SPINON_C182_SCREEN_GEOMETRY/);
  assert.match(androidActivitySource,
    /BuildConfig\.SPINON_C18_2_ALPHA_IMAGE_FIXTURE_TEST[\s\S]*spinon_c182_alpha_image[\s\S]*imageAlphaFixtureRequested/);
  assert.match(iosAppDelegateSource,
    /#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1[\s\S]*runRuntimeGpu = runRuntimeGpuBase[\s\S]*--spinon-c181-image-paint[\s\S]*--spinon-c182-alpha-image[\s\S]*--spinon-c183-object-fit-position/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c18-4-image-fixture-test"\)\]\s*pub\(super\) mod c18_4;/);
  assert.match(ffiFixtureSource,
    /#\[cfg\(feature = "c18-4-image-fixture-test"\)\]\s*mod c18_4;/);
  assert.match(cargoManifest,
    /c18-4-image-fixture-test\s*=\s*\[[\s\S]*?"c04-runtime-gpu-fixture-test"[\s\S]*?"dep:spinon-image"[\s\S]*?"dep:spinon-layout"[\s\S]*?\]/);
  assert.match(c184RuntimeFixture,
    /RUNTIME_CSS_C18_4_ASYNC_IMAGE_FIXTURE_SOURCE[\s\S]*runtime-image-async-replacement-c18-4\.js/);
  assert.match(c184FixtureImplementation,
    /include_bytes![\s\S]*c18-4-async-[abcd]-(?:slow|fast)\.png/);
  assert.match(c184FfiFixture,
    /spinon_runtime_gpu_host_new_c18_4_async_image_fixture[\s\S]*spinon_runtime_gpu_host_request_c18_4_surface_pixels_for_test/);
  assert.match(c184RuntimeScript,
    /spinon-fixture:c18-4\/b-fast[\s\S]*spinon-fixture:c18-4\/c-slow[\s\S]*spinon-fixture:c18-4\/broken[\s\S]*spinon-fixture:c18-4\/d-fast/);
  assert.match(runtimeFfiHeader,
    /#if defined\(SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c18_4_async_image_fixture/);
  assert.match(androidBuildScript,
    /ffi_features\+=\(c18-4-image-fixture-test\)/);
  assert.match(androidBuildScript,
    /c184_async_image_cpp_flag=.*SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST/);
  assert.match(androidJniSource,
    /#if defined\(SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c18_4_async_image_fixture/);
  assert.match(androidGradleSource,
    /if \(spinonC184AsyncImageFixtureTestValue == "1"\)\s*\{\s*java\.srcDir\("src\/c18_4_async_image_fixture_test\/java"\)/);
  assert.match(androidActivitySource,
    /BuildConfig\.SPINON_C18_4_ASYNC_IMAGE_FIXTURE_TEST[\s\S]*spinon_c184_async_image/);
  assert.match(androidAsyncImageNativeAdapter,
    /native long createHost\(\);[\s\S]*native byte\[\] evalFixture\(long host\);/);
  assert.match(androidAsyncImageCaptureAdapter,
    /SPINON_C184_PIXELCOPY_SAMPLE[\s\S]*SPINON_C184_PIXELCOPY_SUMMARY/);
  assert.match(iosBuildSimulatorScript,
    /c18_4_async_image_fixture=\$\{SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST:-0\}/);
  assert.match(iosProject,
    /SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST=\$\(SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST\)/);
  assert.match(iosRunnerImplementation,
    /#if defined\(SPINON_ENABLE_C04_RUNTIME_GPU\)[\s\S]*?SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST[\s\S]*?spinon_runtime_gpu_host_new_c18_4_async_image_fixture/);
  assert.match(iosAppDelegateSource,
    /runC184ImageRoute = arguments\.contains\("--spinon-c184-async-image"\)[\s\S]*runC184ImageRoute/);
});

test('C15.3 glyph paint 앱 adapter와 고정 글꼴은 opt-in feature 경계에 묶인다', async () => {
  const read = (path) => readFile(resolve(repositoryRoot, path), 'utf8');
  const [runtimeModules, ffiModules, runtimeSource, ffiSource, cargoManifest,
    c15RuntimeFixture, c15HostDocumentFixture, c15HostDocumentLayout, c15HostDocumentRaster,
    c15CssFixture, c15FfiFixture, ffiHeader,
    androidBuild, androidGradle, androidActivity, androidDemo, androidJni, androidAdapter,
    iosBuild, iosProject, iosHeader,
    iosRunner, iosAppDelegate, iosDemo] = await Promise.all([
    read('crates/spinon-ffi/src/runtime_gpu/fixtures.rs'),
    read('crates/spinon-ffi/src/runtime_gpu/ffi/fixtures.rs'),
    read('crates/spinon-ffi/src/runtime_gpu.rs'),
    read('crates/spinon-ffi/src/runtime_gpu/ffi.rs'),
    read('crates/spinon-ffi/Cargo.toml'),
    read('crates/spinon-ffi/src/runtime_gpu/fixtures/c15_3.rs'),
    read('crates/spinon-ffi/src/runtime_gpu/fixtures/c15_3_hostdocument_text_paint.rs'),
    read('crates/spinon-ffi/src/runtime_gpu/fixtures/c15_3_hostdocument_text_paint/layout.rs'),
    read('crates/spinon-ffi/src/runtime_gpu/fixtures/c15_3_hostdocument_text_paint/raster.rs'),
    read('tests/fixtures/css/c15/c15-3-wgpu-glyph-paint.js'),
    read('crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c15_3.rs'),
    read('crates/spinon-ffi/include/spinon_ffi.h'),
    read('tools/build-android.sh'),
    read('platforms/android/app/build.gradle.kts'),
    read('platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java'),
    read('platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'),
    read('platforms/android/app/src/main/cpp/spinon_jni.cc'),
    read('platforms/android/app/src/c15_3_wgpu_glyph_paint_fixture_test/java/dev/spinon/bootstrap/GlyphPaintFixtureNativeAdapter.java'),
    read('tools/build-ios-native.sh'),
    read('platforms/ios/SpinonBootstrap.xcodeproj/project.pbxproj'),
    read('platforms/ios/Sources/SpinonRunner.h'),
    read('platforms/ios/Sources/SpinonRunner.mm'),
    read('platforms/ios/Sources/AppDelegate.swift'),
    read('platforms/ios/Sources/C0410RuntimeGpuDemo.swift'),
  ]);

  assert.match(runtimeModules,
    /#\[cfg\(feature = "c15-3-wgpu-glyph-paint-fixture-test"\)\]\s*pub\(super\) mod c15_3;/);
  assert.match(runtimeModules,
    /#\[cfg\(feature = "c15-3-wgpu-glyph-paint-fixture-test"\)\]\s*pub\(super\) mod c15_3_hostdocument_text_paint;/);
  assert.match(c15HostDocumentFixture, /mod layout;\s*mod raster;/);
  assert.match(ffiModules,
    /#\[cfg\(feature = "c15-3-wgpu-glyph-paint-fixture-test"\)\]\s*mod c15_3;/);
  assert.match(cargoManifest,
    /c15-3-wgpu-glyph-paint-fixture-test\s*=\s*\[[\s\S]*?"c04-runtime-gpu-fixture-test"[\s\S]*?"dep:spinon-core"[\s\S]*?"dep:spinon-layout"[\s\S]*?"dep:spinon-text-layout"[\s\S]*?spinon-runtime\/c19-2-box-paint-opacity-fixture-test[\s\S]*?\]/);
  assert.match(c15RuntimeFixture,
    /include_bytes![\s\S]*NotoSansKR-VF\.ttf[\s\S]*c15-3-wgpu-glyph-paint\.js[\s\S]*TextLayoutEngine::new_without_system_fonts[\s\S]*RuntimePaintCommand::DrawText/);
  assert.match(c15HostDocumentLayout,
    /from_host_document_with_text_paragraphs[\s\S]*compute_with_text_layout_engine[\s\S]*text_layouts/);
  assert.match(c15HostDocumentFixture,
    /rasterize_glyphs[\s\S]*validate_raster_snapshot[\s\S]*RuntimeRenderText::new/);
  assert.match(c15HostDocumentLayout,
    /LayoutSourceRevision::HostDocument[\s\S]*source_utf16/);
  assert.match(c15HostDocumentRaster, /overflow_ink_samples[\s\S]*encode_sample_points/);
  assert.match(c15CssFixture,
    /spinon-c15-3-wgpu-glyph-paint\/v2/);
  assert.match(c15CssFixture, /width:360px;height:240px/);
  assert.match(c15CssFixture, /left:160px/);
  assert.match(c15FfiFixture,
    /spinon_runtime_gpu_host_new_c15_3_glyph_paint_fixture[\s\S]*spinon_runtime_gpu_host_new_c15_3_hostdocument_text_paint_fixture[\s\S]*spinon_runtime_gpu_host_eval_c15_3_glyph_paint_fixture/);
  assert.match(runtimeSource,
    /#\[cfg\(feature = "c15-3-wgpu-glyph-paint-fixture-test"\)\][\s\S]*fixtures::c15_3_hostdocument_text_paint::attach_hostdocument_text_paint[\s\S]*fixtures::c15_3::attach_glyph_paint/);
  assert.match(ffiModules,
    /#\[cfg\(feature = "c15-3-wgpu-glyph-paint-fixture-test"\)\][\s\S]*c15_3/);
  assert.match(ffiHeader,
    /#if defined\(SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c15_3_hostdocument_text_paint_fixture[\s\S]*?spinon_runtime_gpu_host_eval_c15_3_glyph_paint_fixture/);

  assert.match(androidBuild,
    /ffi_features\+=\(c15-3-wgpu-glyph-paint-fixture-test\)/);
  assert.match(androidBuild,
    /c153_wgpu_glyph_paint_cpp_flag=.*SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST/);
  assert.match(androidGradle,
    /src\/c15_3_wgpu_glyph_paint_fixture_test\/java/);
  assert.match(androidActivity,
    /spinon_c153_glyph_paint[\s\S]*SPINON_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST/);
  assert.match(androidDemo,
    /GlyphPaintFixtureNativeAdapter[\s\S]*createHostDocumentTextPaintHost[\s\S]*hostdocument_text_samples_css_px=/);
  assert.match(androidJni,
    /#if defined\(SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c15_3_glyph_paint_fixture[\s\S]*?spinon_runtime_gpu_host_new_c15_3_hostdocument_text_paint_fixture[\s\S]*?spinon_runtime_gpu_host_eval_c15_3_glyph_paint_fixture/);
  assert.match(androidAdapter,
    /createHost\(\);[\s\S]*createHostDocumentTextPaintHost\(\);[\s\S]*evalFixture\(long host\);/);

  assert.match(iosBuild,
    /append_ffi_feature c15-3-wgpu-glyph-paint-fixture-test/);
  assert.match(iosProject,
    /SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST=\$\(SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST\)/);
  assert.match(iosHeader,
    /SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST[\s\S]*createRuntimeGpuHostWithC15_3GlyphPaintFixture[\s\S]*createRuntimeGpuHostWithC15_3HostDocumentTextPaintFixture/);
  assert.match(iosRunner,
    /SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST[\s\S]*spinon_runtime_gpu_host_new_c15_3_glyph_paint_fixture[\s\S]*spinon_runtime_gpu_host_new_c15_3_hostdocument_text_paint_fixture[\s\S]*spinon_runtime_gpu_host_eval_c15_3_glyph_paint_fixture/);
  assert.match(iosAppDelegate,
    /--spinon-c153-glyph-paint[\s\S]*--spinon-c153-hostdocument-text-paint/);
  assert.match(iosDemo,
    /\$0 != "--spinon-c153-glyph-paint"[\s\S]*\$0 != "--spinon-c153-hostdocument-text-paint"/);
  assert.match(iosDemo,
    /SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST_1[\s\S]*createRuntimeGpuHostWithC15_3HostDocumentTextPaintFixture[\s\S]*hostdocument_text_samples_css_px=/);
});
