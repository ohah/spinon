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
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11_3_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11_3_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11.rs',
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
  const cargoManifest = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/Cargo.toml'),
    'utf8',
  );
  const androidJniSource = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/main/cpp/spinon_jni.cc'),
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
    /c132ScrollFixtureRequested\s*\?\s*400\s*:\s*fixedSizeCssFixtureRequested\s*\?\s*320/);
  assert.match(androidGpuDemoSource,
    /if \(c132ScrollFixtureRequested\) return 400\.0f;/);
});
