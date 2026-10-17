# C13.1 Android API 37 AVD full surface regression

- 실행 ID: `1b511357b3fa`
- 대상: emulator-5554 · sdk_gphone16k_arm64 · API 37 · arm64-v8a
- C13.1 compact fixture 앱에서 41/41 frame, 52/52 fixed PixelCopy sample, 30,000/30,000 hit identity를 확인했다.
- 900×300 PixelCopy에서 CSS pixel center 30,000개를 exact 비교했고, 전체 물리 표면 270,000 texel / 1,080,000 RGBA channel도 Chrome 155와 대조했다. 각 결과의 불일치는 0/0개다.
- Chrome target의 container border 밖 paint 좌표도 물리 표면 전체 비교에 포함했다.
- 세 실행은 서로 다른 앱 PID와 surface generation을 가진 독립 run으로 기록한다. API 37 ARM64 AVD software renderer만 검증한다. 실기기·hardware GPU·DOM event dispatch·WPT 및 C13.1 완료를 뜻하지 않는다.
- [실행 요약](./summary.json) · build/install/launch/runtime 로그, PixelCopy와 앱 화면을 함께 보존했다.
