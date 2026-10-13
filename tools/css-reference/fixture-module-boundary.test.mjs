import assert from 'node:assert/strict';
import { existsSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const legacyFixtureSources = [
  'crates/spinon-ffi/src/runtime_gpu/c11.rs',
  'crates/spinon-ffi/src/runtime_gpu/c11_2_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/c11_2_3.rs',
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
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures.rs',
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

test('이동한 fixture 구현은 정해진 fixtures 모듈 경로에 있다', () => {
  for (const relativePath of canonicalFixtureSources) {
    assert.equal(
      existsSync(resolve(repositoryRoot, relativePath)),
      true,
      `정식 fixture 모듈 경로가 없습니다: ${relativePath}`,
    );
  }
});
