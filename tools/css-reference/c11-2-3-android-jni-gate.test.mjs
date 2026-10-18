import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const read = (path) => readFile(resolve(repositoryRoot, path), 'utf8');

function nativeFunction(source, symbol) {
  const start = source.indexOf(symbol);
  assert.notEqual(start, -1, `${symbol} JNI 진입점이 없습니다`);
  const nextFunction = source.indexOf('\nextern "C" JNIEXPORT', start);
  return source.slice(start, nextFunction === -1 ? source.length : nextFunction);
}

test('C11.2.3 Android JNI 진입점은 opt-in FFI 선언과 같은 feature gate를 쓴다', async () => {
  const [jni, header, build, gradle] = await Promise.all([
    read('platforms/android/app/src/main/cpp/spinon_jni.cc'),
    read('crates/spinon-ffi/include/spinon_ffi.h'),
    read('tools/build-android.sh'),
    read('platforms/android/app/build.gradle.kts'),
  ]);
  const feature = 'SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST';

  for (const symbol of [
    'nativeCreateC1123GridAutoPlacementHost(',
    'nativeEvalC1123GridAutoPlacementFixture(',
  ]) {
    const functionSource = nativeFunction(jni, symbol);
    assert.match(functionSource, new RegExp(`#if defined\\(${feature}\\)`));
    assert.match(functionSource, /#else[\s\S]*?#endif/);
  }
  assert.match(nativeFunction(jni, 'nativeCreateC1123GridAutoPlacementHost('),
    /#else[\s\S]*?return 0;[\s\S]*?#endif/);
  assert.match(nativeFunction(jni, 'nativeEvalC1123GridAutoPlacementFixture('),
    /#else[\s\S]*?status=-90 feature-disabled[\s\S]*?#endif/);

  assert.match(header, new RegExp(`defined\\(${feature}\\)`));
  assert.match(build, new RegExp(`-D${feature}=\\$\\{${feature}:-0\\}`));
  assert.match(build, /ffi_features\+\=\(c11-2-3-grid-fixture-test\)/);
  assert.match(gradle, new RegExp(`environmentVariable\\("${feature}"\\)`));
});
