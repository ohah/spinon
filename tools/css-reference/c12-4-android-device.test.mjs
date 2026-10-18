import assert from 'node:assert/strict';
import test from 'node:test';

import { classifyC124AndroidTarget } from './c12-4-android-device.mjs';

test('고정 API 37 AVD를 시뮬레이터로 분류한다', () => {
  assert.deepEqual(classifyC124AndroidTarget({
    serial: 'emulator-5554',
    qemu: '1',
    apiLevel: 37,
  }), {
    kind: 'avd',
    classification: 'Android API 37 AVD · 시뮬레이터 표면 결과',
  });
});

test('API 36 USB 기기를 실기기로 분류한다', () => {
  assert.deepEqual(classifyC124AndroidTarget({
    serial: 'R5KYB06SMPE',
    qemu: '0',
    apiLevel: 36,
  }), {
    kind: 'physical-device',
    classification: 'Android 실기기 · 기기 GPU 표면 결과',
  });
});

test('AVD API와 serial을 고정 계약에서 벗어나게 바꾸면 거부한다', () => {
  assert.throws(() => classifyC124AndroidTarget({
    serial: 'emulator-5554', qemu: '1', apiLevel: 36,
  }), /고정 Android API 37 AVD/);
  assert.throws(() => classifyC124AndroidTarget({
    serial: 'R5KYB06SMPE', qemu: '1', apiLevel: 37,
  }), /에뮬레이터 serial 형식/);
  assert.throws(() => classifyC124AndroidTarget({
    serial: 'emulator-5554', qemu: '0', apiLevel: 36,
  }), /실기기 식별값/);
});

test('미확인 가상 기기 상태와 지원 범위 밖 API를 허용하지 않는다', () => {
  assert.throws(() => classifyC124AndroidTarget({
    serial: 'R5KYB06SMPE', qemu: '', apiLevel: 36,
  }), /가상 기기 식별값/);
  assert.throws(() => classifyC124AndroidTarget({
    serial: 'R5KYB06SMPE', qemu: '0', apiLevel: 28,
  }), /지원 범위 밖/);
});
