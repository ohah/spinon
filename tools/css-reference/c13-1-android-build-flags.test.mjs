import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(import.meta.dirname, '../..');

test('Android accepts the shared C13.1 opt-in name and preserves the legacy alias', async () => {
  const gradle = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/build.gradle.kts'),
    'utf8',
  );
  const nativeBuild = await readFile(resolve(repositoryRoot, 'tools/build-android.sh'), 'utf8');

  assert.match(gradle, /environmentVariable\("SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST"\)/);
  assert.match(gradle, /environmentVariable\("SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST"\)/);
  assert.match(gradle, /"SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST"\s*,\s*spinonC131OverflowClipFixtureTestValue/);
  assert.match(nativeBuild, /canonical_c131_fixture_flag="\$\{SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST:-\}"/);
  assert.match(nativeBuild, /legacy_c131_fixture_flag="\$\{SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST:-\}"/);
  assert.match(nativeBuild, /c131_overflow_clip_cpp_flag="-DSPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST=\$c131_fixture_flag"/);
  assert.match(nativeBuild, /ffi_features\+=\(c13-1-overflow-clip-fixture-test\)/);
});
