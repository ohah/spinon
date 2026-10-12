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
    assert.match(result.stdout,
      /SPINON_IOS_SIM_BUILD_FLAGS c04_runtime_gpu=1 c11_grid_metal_surface_pixels=1/);
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
  assert.match(matrix, /SPINON_IOS_SIM_BUILD_FLAGS c04_runtime_gpu=1 c11_grid_metal_surface_pixels=1/);
  assert.match(matrix, /SPINON_IOS_NATIVE_FEATURES ffi=.*c11-grid-metal-surface-pixels-test/);
});
