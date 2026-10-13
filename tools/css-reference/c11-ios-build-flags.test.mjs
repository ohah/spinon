import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { chmod, mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const buildSimulatorScript = resolve(repositoryRoot, 'tools/build-ios-sim.sh');
const buildNativeScript = resolve(repositoryRoot, 'tools/build-ios-native.sh');
const ffiHeaderDirectory = resolve(repositoryRoot, 'crates/spinon-ffi/include');

function preprocessFfiHeader(definitions) {
  const result = spawnSync(process.env.CC ?? 'clang', [
    '-E',
    '-P',
    '-x',
    'c',
    `-I${ffiHeaderDirectory}`,
    ...Object.entries(definitions).map(([name, value]) => `-D${name}=${value}`),
    '-',
  ], {
    cwd: repositoryRoot,
    encoding: 'utf8',
    input: '#include "spinon_ffi.h"\n',
  });
  return result;
}

async function runSimulatorBuild({ includeMise, variables }) {
  const directory = await mkdtemp(join(tmpdir(), 'spinon-ios-build-flags-'));
  const binDirectory = join(directory, 'bin');
  const capturedArguments = join(directory, 'arguments.txt');
  await mkdir(binDirectory);
  const executable = `#!/bin/bash\nprintf '%s\\n' "$@" > "$SPINON_CAPTURE"\n`;
  const commandName = includeMise ? 'mise' : 'xcodebuild';
  const commandPath = join(binDirectory, commandName);
  await writeFile(commandPath, executable);
  await chmod(commandPath, 0o755);
  const result = spawnSync('/bin/bash', [buildSimulatorScript], {
    cwd: repositoryRoot,
    encoding: 'utf8',
    env: {
      ...process.env,
      PATH: `${binDirectory}:/usr/bin:/bin`,
      SPINON_CAPTURE: capturedArguments,
      ...variables,
    },
  });
  let args = '';
  try {
    args = await readFile(capturedArguments, 'utf8');
  } catch {
    // 거부 경로에서는 가짜 빌드 도구가 실행되지 않아 캡처 파일이 없습니다.
  }
  await rm(directory, { recursive: true, force: true });
  return { ...result, args };
}

function runNativeBuild(variables) {
  return spawnSync('/bin/bash', [buildNativeScript], {
    cwd: repositoryRoot,
    encoding: 'utf8',
    env: {
      ...process.env,
      SPINON_V8_DIR: '/tmp/spinon-missing-v8-for-flag-test',
      ...variables,
    },
  });
}

test('mise 유무와 관계없이 iOS simulator 빌드 인자로 Grid pixel 기능을 전달한다', async () => {
  for (const includeMise of [false, true]) {
    const result = await runSimulatorBuild({
      includeMise,
      variables: {
        SPINON_ENABLE_C04_RUNTIME_GPU: '1',
        SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST: '1',
      },
    });
    assert.equal(result.status, 0, result.stderr);
    assert.match(result.args,
      /SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1/);
    assert.match(result.args,
      /SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE=0/);
    assert.match(result.stdout,
      /SPINON_IOS_SIM_BUILD_FLAGS c04_runtime_gpu=1 c04_runtime_gpu_fixture_test=1 c04_ua_cascade_fixture_probe=0 c11_2_3_grid_fixture=1 c11_grid_metal_surface_pixels=1/);
    assert.match(result.args, /SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=1/);
  }
});

test('Grid pixel 검증은 C04 GPU fixture 없이 빌드를 시작하지 않는다', async () => {
  const result = await runSimulatorBuild({
    includeMise: false,
    variables: {
      SPINON_ENABLE_C04_RUNTIME_GPU: '0',
      SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST: '1',
    },
  });
  assert.equal(result.status, 2);
  assert.match(result.stderr, /C04\.10 runtime GPU fixture와 함께 켜야 합니다/);
  assert.equal(result.args, '');
});

test('iOS simulator 스크립트는 누락된 Grid pixel flag를 기본 비활성화로 전달한다', async () => {
  const script = await readFile(buildSimulatorScript, 'utf8');
  assert.match(script,
    /SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST:-0/);
  const result = await runSimulatorBuild({
    includeMise: false,
    variables: {
      SPINON_ENABLE_C04_RUNTIME_GPU: '1',
      SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST: '0',
    },
  });
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.args,
    /SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=0/);
});

test('두 iOS 빌드 스크립트는 잘못된 Grid pixel flag를 거부한다', async () => {
  const simulatorResult = await runSimulatorBuild({
    includeMise: false,
    variables: { SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST: 'true' },
  });
  assert.equal(simulatorResult.status, 2);
  assert.match(simulatorResult.stderr, /0 또는 1이어야 합니다/);
  assert.equal(simulatorResult.args, '');

  const nativeResult = runNativeBuild({
    SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST: '2',
  });
  assert.equal(nativeResult.status, 2);
  assert.match(nativeResult.stderr, /0 또는 1이어야 합니다/);
});

test('iOS native build도 C04 GPU fixture 없이 Grid pixel feature를 거부한다', () => {
  const result = runNativeBuild({
    SPINON_ENABLE_C04_RUNTIME_GPU: '0',
    SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST: '1',
  });
  assert.equal(result.status, 2);
  assert.match(result.stderr, /C04\.10 runtime GPU fixture와 함께 켜야 합니다/);
});

test('Grid pixel 빌드 설정을 iOS native Rust feature와 C++ define으로 연결한다', async () => {
  const nativeBuild = await readFile(resolve(repositoryRoot, 'tools/build-ios-native.sh'), 'utf8');
  const project = await readFile(resolve(
    repositoryRoot,
    'platforms/ios/SpinonBootstrap.xcodeproj/project.pbxproj',
  ), 'utf8');
  const matrix = await readFile(resolve(
    repositoryRoot,
    'tools/css-reference/capture-c11-2-1-grid-placement-ios-runtime-matrix.mjs',
  ), 'utf8');

  assert.match(nativeBuild, /append_ffi_feature c11-grid-metal-surface-pixels-test/);
  assert.match(nativeBuild, /"\$c11_grid_surface_pixel_cpp_flag"/);
  assert.match(project, /GCC_PREPROCESSOR_DEFINITIONS[^\n]*SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST/);
  assert.match(project, /SWIFT_ACTIVE_COMPILATION_CONDITIONS[^\n]*SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST/);
  assert.match(matrix, /SPINON_IOS_SIM_BUILD_FLAGS c04_runtime_gpu=1 c04_runtime_gpu_fixture_test=1 c04_ua_cascade_fixture_probe=0 c11_2_3_grid_fixture=1 c11_grid_metal_surface_pixels=1/);
  assert.match(matrix, /SPINON_IOS_NATIVE_FEATURES ffi=.*c11-grid-metal-surface-pixels-test/);
});

test('C04 GPU 핵심과 fixture 검증 feature를 분리하고 앱 검증 빌드에서만 둘 다 켠다', async () => {
  const androidBuild = await readFile(resolve(repositoryRoot, 'tools/build-android.sh'), 'utf8');
  const iosBuild = await readFile(buildNativeScript, 'utf8');
  const ffiManifest = await readFile(resolve(repositoryRoot, 'crates/spinon-ffi/Cargo.toml'), 'utf8');
  const iosProject = await readFile(resolve(
    repositoryRoot,
    'platforms/ios/SpinonBootstrap.xcodeproj/project.pbxproj',
  ), 'utf8');

  assert.match(androidBuild,
    /if \[\[ "\$\{SPINON_ENABLE_C04_RUNTIME_GPU:-0\}" == "1" \]\]; then\s+ffi_features\+=\(c04-runtime-gpu-fixture-test c11-2-3-grid-fixture-test\)\s+fi/);
  assert.match(androidBuild, /s04_runtime_gpu_fixture_test_cpp_flag="-DSPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=/);
  assert.match(androidBuild, /"\$s04_runtime_gpu_fixture_test_cpp_flag"/);
  assert.match(androidBuild,
    /s04_c1123_grid_fixture_cpp_flag="-DSPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=\$\{SPINON_ENABLE_C04_RUNTIME_GPU:-0\}"/);
  assert.match(androidBuild, /"\$s04_c1123_grid_fixture_cpp_flag"/);
  assert.match(iosBuild,
    /if \[\[ "\$\{SPINON_ENABLE_C04_RUNTIME_GPU:-0\}" == "1" \]\]; then\s+append_ffi_feature c04-runtime-gpu-fixture-test\s+append_ffi_feature c11-2-3-grid-fixture-test\s+fi/);
  assert.match(iosBuild, /runtime_gpu_fixture_test_cpp_flag="-DSPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=/);
  assert.match(iosBuild, /"\$runtime_gpu_fixture_test_cpp_flag"/);
  assert.match(iosBuild,
    /c1123_grid_fixture_cpp_flag="-DSPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=\$\{SPINON_ENABLE_C04_RUNTIME_GPU:-0\}"/);
  assert.match(iosBuild, /"\$c1123_grid_fixture_cpp_flag"/);
  assert.match(iosProject,
    /GCC_PREPROCESSOR_DEFINITIONS[^\n]*SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST[^\n]*SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST/);
  assert.match(ffiManifest,
    /c04-runtime-gpu-fixture-test\s*=\s*\["c04-runtime-gpu"\]/);
  assert.match(ffiManifest,
    /c11-2-3-grid-fixture-test\s*=\s*\["c04-runtime-gpu-fixture-test"\]/);
  const runtimeGpu = await readFile(resolve(
    repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu.rs',
  ), 'utf8');
  const runtimeGpuFfi = await readFile(resolve(
    repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/ffi.rs',
  ), 'utf8');
  const fixtureData = await readFile(resolve(
    repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/data.rs',
  ), 'utf8');
  const runtimeGpuFixtures = await readFile(resolve(
    repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures.rs',
  ), 'utf8');
  const ffiFixtures = await readFile(resolve(
    repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures.rs',
  ), 'utf8');
  const ffiHeader = await readFile(resolve(
    repositoryRoot,
    'crates/spinon-ffi/include/spinon_ffi.h',
  ), 'utf8');
  assert.match(runtimeGpu, /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*mod fixtures;/);
  assert.match(runtimeGpuFfi, /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*mod fixtures;/);
  assert.match(runtimeGpuFixtures, /mod c11_3_1;/);
  assert.match(ffiFixtures, /mod c11_3_1;/);
  assert.doesNotMatch(runtimeGpu, /mod c11_3_1;/);
  assert.doesNotMatch(runtimeGpuFfi, /mod c11_3_1;/);
  assert.match(fixtureData, /include_str!\(/);
  assert.match(ffiHeader,
    /#if defined\(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_registered_properties_fixture/);
});

test('UA cascade fixture probe는 기본 off이고 명시한 경우에만 Rust와 네이티브 경계를 켠다', async () => {
  const androidBuild = await readFile(resolve(repositoryRoot, 'tools/build-android.sh'), 'utf8');
  const iosBuild = await readFile(buildNativeScript, 'utf8');
  const ffiManifest = await readFile(resolve(repositoryRoot, 'crates/spinon-ffi/Cargo.toml'), 'utf8');
  const ffiHeader = await readFile(resolve(repositoryRoot, 'crates/spinon-ffi/include/spinon_ffi.h'), 'utf8');
  const androidGradle = await readFile(resolve(repositoryRoot, 'platforms/android/app/build.gradle.kts'), 'utf8');
  const iosProject = await readFile(resolve(
    repositoryRoot,
    'platforms/ios/SpinonBootstrap.xcodeproj/project.pbxproj',
  ), 'utf8');

  assert.match(androidBuild, /SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE:-0/);
  assert.match(androidBuild, /ffi_features\+=\(c04-runtime-ua-cascade-fixture-test\)/);
  assert.match(androidBuild, /s04_ua_cascade_probe_cpp_flag/);
  assert.match(iosBuild, /SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE:-0/);
  assert.match(iosBuild, /append_ffi_feature c04-runtime-ua-cascade-fixture-test/);
  assert.match(iosBuild, /ua_cascade_probe_cpp_flag/);
  assert.match(ffiManifest, /c04-runtime-ua-cascade-fixture-test\s*=\s*\[\]/);
  assert.match(ffiHeader,
    /#if defined\(SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE\)[\s\S]*?spinon_runtime_ua_cascade_probe/);
  assert.match(androidGradle, /spinonC04UaCascadeFixtureProbe/);
  assert.match(iosProject,
    /GCC_PREPROCESSOR_DEFINITIONS[^\n]*SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE/);
  const result = await runSimulatorBuild({
    includeMise: false,
    variables: { SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE: '1' },
  });
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.args, /SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE=1/);
  assert.match(result.stdout, /c04_ua_cascade_fixture_probe=1/);
});

test('C11 iOS pixel 캡처는 출력 플래그 순서와 무관하게 기능 값을 확인한다', async () => {
  const capture = await readFile(resolve(
    repositoryRoot,
    'tools/css-reference/capture-c11-3-1-grid-repeat-pixels.mjs',
  ), 'utf8');

  assert.match(capture,
    /const buildFlagLine = buildOutput\.match\(\/\^SPINON_IOS_SIM_BUILD_FLAGS/);
  assert.match(capture,
    /buildFlags\.c04_runtime_gpu, '1'/);
  assert.match(capture,
    /buildFlags\.c11_grid_metal_surface_pixels, '1'/);
  assert.doesNotMatch(capture,
    /SPINON_IOS_SIM_BUILD_FLAGS c04_runtime_gpu=1 c11_grid_metal_surface_pixels=1/);
});

test('Android와 iOS UA probe 호출은 opt-in feature gate 안에만 둔다', async () => {
  const android = await readFile(resolve(
    repositoryRoot,
    'platforms/android/app/src/main/cpp/spinon_jni.cc',
  ), 'utf8');
  const ios = await readFile(resolve(
    repositoryRoot,
    'platforms/ios/Sources/SpinonRunner.mm',
  ), 'utf8');
  const androidMethod = android.match(
    /Java_dev_spinon_bootstrap_MainActivity_nativeSessionUaCascadeProbe\([\s\S]*?\n}\n/,
  )?.[0];
  const iosMethod = ios.match(
    /\+ \(NSString \*\)runUaCascadeProbe \{[\s\S]*?\n\}/,
  )?.[0];

  assert.ok(androidMethod, 'Android JNI UA probe wrapper exists');
  assert.ok(iosMethod, 'iOS UA probe wrapper exists');
  for (const [platform, method] of [
    ['Android', androidMethod],
    ['iOS', iosMethod],
  ]) {
    assert.match(method,
      /#if defined\(SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE\) && \\?\s*SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE[\s\S]*?spinon_runtime_ua_cascade_probe[\s\S]*?#else[\s\S]*?ua_cascade_probe=DISABLED[\s\S]*?#endif/,
      `${platform} keeps the optional symbol behind its opt-in macro`);
    assert.equal(
      (method.match(/spinon_runtime_ua_cascade_probe\(/g) ?? []).length,
      1,
      `${platform} contains one gated native probe call`,
    );
  }
});

test('C04 runtime core만으로는 fixture ABI를 노출하지 않는다', () => {
  const base = { SPINON_ENABLE_C04_RUNTIME_GPU: '1' };
  const fixtureFeatureOff = preprocessFfiHeader({
    ...base,
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST: '0',
    SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST: '1',
  });
  assert.equal(fixtureFeatureOff.status, 0, fixtureFeatureOff.stderr);
  assert.match(fixtureFeatureOff.stdout, /spinon_runtime_gpu_host_new\(/);
  assert.doesNotMatch(fixtureFeatureOff.stdout,
    /spinon_runtime_gpu_host_new_registered_properties_fixture/);
  assert.doesNotMatch(fixtureFeatureOff.stdout,
    /spinon_runtime_gpu_host_(?:new|eval)_c11_2_3_grid_auto_placement_fixture/);
  assert.doesNotMatch(fixtureFeatureOff.stdout,
    /spinon_runtime_gpu_host_(?:new|eval)_c11_3_1_grid_repeat_fixture/);
  assert.doesNotMatch(fixtureFeatureOff.stdout,
    /spinon_runtime_gpu_host_request_c11_2_3_grid_surface_pixels_for_test/);

  const runtimeOff = preprocessFfiHeader({
    SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST: '1',
  });
  assert.equal(runtimeOff.status, 0, runtimeOff.stderr);
  assert.doesNotMatch(runtimeOff.stdout,
    /spinon_runtime_gpu_host_new_c11_2_3_grid_auto_placement_fixture/);
  assert.doesNotMatch(runtimeOff.stdout,
    /spinon_runtime_gpu_host_eval_c11_2_3_grid_auto_placement_fixture/);
  assert.doesNotMatch(runtimeOff.stdout,
    /spinon_runtime_gpu_host_(?:new|eval)_c11_3_1_grid_repeat_fixture/);

  const pixelFeatureOff = preprocessFfiHeader({
    ...base,
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST: '1',
    SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST: '1',
  });
  assert.equal(pixelFeatureOff.status, 0, pixelFeatureOff.stderr);
  assert.match(pixelFeatureOff.stdout,
    /spinon_runtime_gpu_host_new_c11_2_3_grid_auto_placement_fixture/);
  assert.match(pixelFeatureOff.stdout,
    /spinon_runtime_gpu_host_eval_c11_2_3_grid_auto_placement_fixture/);
  assert.match(pixelFeatureOff.stdout,
    /spinon_runtime_gpu_host_new_c11_3_1_grid_repeat_fixture/);
  assert.match(pixelFeatureOff.stdout,
    /spinon_runtime_gpu_host_eval_c11_3_1_grid_repeat_fixture/);
  assert.doesNotMatch(pixelFeatureOff.stdout,
    /spinon_runtime_gpu_host_request_c11_2_3_grid_surface_pixels_for_test/);

  const pixelFeatureOn = preprocessFfiHeader({
    ...base,
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST: '1',
    SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST: '1',
    SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST: '1',
  });
  assert.equal(pixelFeatureOn.status, 0, pixelFeatureOn.stderr);
  assert.match(pixelFeatureOn.stdout,
    /spinon_runtime_gpu_host_request_c11_2_3_grid_surface_pixels_for_test/);
  assert.match(pixelFeatureOn.stdout,
    /spinon_runtime_gpu_host_request_c11_3_1_grid_surface_pixels_for_test/);
});
