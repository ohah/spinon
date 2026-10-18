export function classifyC124AndroidTarget({ serial, qemu, apiLevel }) {
  if (!serial || typeof serial !== 'string') {
    throw new Error('Android serial이 비어 있습니다.');
  }
  if (qemu !== '0' && qemu !== '1') {
    throw new Error(`Android 가상 기기 식별값을 판독하지 못했습니다: ${qemu}`);
  }
  if (!Number.isInteger(apiLevel) || apiLevel < 29) {
    throw new Error(`Android API level이 지원 범위 밖입니다: ${apiLevel}`);
  }

  if (qemu === '1') {
    if (!/^emulator-\d+$/.test(serial)) {
      throw new Error(`에뮬레이터 serial 형식이 잘못되었습니다: ${serial}`);
    }
    if (apiLevel !== 37) {
      throw new Error(`고정 Android API 37 AVD가 아닙니다: ${apiLevel}`);
    }
    return {
      kind: 'avd',
      classification: 'Android API 37 AVD · 시뮬레이터 표면 결과',
    };
  }

  if (/^emulator-\d+$/.test(serial)) {
    throw new Error(`에뮬레이터 serial에서 실기기 식별값을 받았습니다: ${serial}`);
  }
  return {
    kind: 'physical-device',
    classification: 'Android 실기기 · 기기 GPU 표면 결과',
  };
}
