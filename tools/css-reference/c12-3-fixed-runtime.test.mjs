import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import { compareRuntimeLog } from './compare-c12-3-fixed-runtime.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const evidenceRoot = join(repositoryRoot, 'spec/internal/evidence/c12-3-fixed-positioning');
const [inventory, reference, androidLog, iosLog] = await Promise.all([
  readFile(join(repositoryRoot, 'tests/fixtures/css/c12/position-fixed-inventory.json'), 'utf8')
    .then(JSON.parse),
  readFile(join(repositoryRoot, 'tests/fixtures/css/references/c12-3-position-fixed-v1.json'), 'utf8')
    .then(JSON.parse),
  readFile(join(evidenceRoot, 'android-physical.log'), 'utf8'),
  readFile(join(evidenceRoot, 'ios-simulator.log'), 'utf8'),
]);

test('C12.3 Android 실기기 V8 runtime의 전체 viewport/DPR/resize frame이 Chrome 기준과 일치한다', () => {
  const result = compareRuntimeLog({
    log: androidLog,
    platform: 'android-physical',
    inventory,
    reference,
  });
  assert.equal(result.matchedStates, 13);
  assert.equal(result.matchedFrames, 364);
});

test('C12.3 iOS Simulator V8 runtime의 전체 viewport/DPR/resize frame이 Chrome 기준과 일치한다', () => {
  const result = compareRuntimeLog({
    log: iosLog,
    platform: 'ios-simulator',
    inventory,
    reference,
  });
  assert.equal(result.matchedStates, 13);
  assert.equal(result.matchedFrames, 364);
});

test('C12.3 comparator는 frame 누락·잘못된 revision·이전 실패 로그를 통과시키지 않는다', () => {
  const base = {
    platform: 'android-physical',
    inventory,
    reference,
  };
  assert.throws(() => compareRuntimeLog({
    ...base,
    log: androidLog.replace(/SPINON_C123_NODE_FRAME state=resize-noop[^\n]*\n/, ''),
  }));
  const staleRevisionLog = androidLog.replace(
    /(SPINON_C123_SUMMARY state=resize-outbound[^\n]*environment_revision=)9/,
    (_match, prefix) => `${prefix}19`,
  );
  assert.notEqual(staleRevisionLog, androidLog, 'revision negative control must mutate a canonical matrix state');
  assert.throws(() => compareRuntimeLog({
    ...base,
    log: staleRevisionLog,
  }));
  assert.throws(() => compareRuntimeLog({
    ...base,
    log: `${androidLog}\nSPINON_C123_MATRIX_FAILURE state=negative-control\n`,
  }));
});
