export function selectAndroidPhysicalSerial(deviceList, requestedSerial) {
  const connected = deviceList
    .split(/\r?\n/)
    .map((line) => {
      const match = line.match(/^(\S+)\s+device\b(.*)$/);
      if (!match) return null;
      const model = match[2].match(/(?:^|\s)model:([^\s]+)/)?.[1]?.replaceAll('_', '-');
      return { serial: match[1], model };
    })
    .filter(Boolean);

  if (requestedSerial) {
    const selected = connected.find(({ serial }) => serial === requestedSerial);
    if (!selected) {
      throw new Error(`요청한 Android 대상이 연결되지 않았습니다: ${requestedSerial}`);
    }
    if (selected.model !== 'SM-S731N') {
      throw new Error(`Android 실기기 SM-S731N이 필요합니다: ${requestedSerial} (${selected.model ?? 'model unknown'})`);
    }
    return requestedSerial;
  }

  const candidates = connected.filter(({ model }) => model === 'SM-S731N');
  if (candidates.length === 1) return candidates[0].serial;
  if (candidates.length === 0) {
    throw new Error('ADB에서 Android SM-S731N 실기기를 찾지 못했습니다. adb devices -l과 ANDROID_SERIAL을 확인하세요.');
  }
  throw new Error('SM-S731N 실기기가 여러 대 연결되어 있습니다. capture 명령에 ADB serial을 지정하세요.');
}

export function selectAndroidAVDSerial(deviceList, requestedSerial) {
  const connected = deviceList
    .split(/\r?\n/)
    .map((line) => {
      const match = line.match(/^(emulator-\d+)\s+device\b/);
      return match?.[1] ?? null;
    })
    .filter(Boolean);

  if (requestedSerial) {
    if (!/^emulator-\d+$/.test(requestedSerial)) {
      throw new Error(`Android AVD serial 형식이 아닙니다: ${requestedSerial}`);
    }
    if (!connected.includes(requestedSerial)) {
      throw new Error(`요청한 Android AVD가 연결되지 않았습니다: ${requestedSerial}`);
    }
    return requestedSerial;
  }

  if (connected.length === 1) return connected[0];
  if (connected.length === 0) {
    throw new Error('ADB에서 실행 중인 Android AVD를 찾지 못했습니다. emulator-* serial을 확인하세요.');
  }
  throw new Error('Android AVD가 여러 대 연결되어 있습니다. capture 명령에 AVD serial을 지정하세요.');
}
