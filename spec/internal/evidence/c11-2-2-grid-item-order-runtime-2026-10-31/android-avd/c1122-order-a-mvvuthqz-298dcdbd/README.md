# C11.2.2 Grid item order simulator runtime

- 실행: `c1122-order-a-mvvuthqz-298dcdbd`
- 플랫폼: Android API 37 ARM64 AVD
- 대상: emulator-5554 · Android 17/API 37 · sdk_gphone16k_arm64 · arm64-v8a
- 앱 SHA-256: `26fd11adb7fbe42efcfbb3786a17f30e5f98d45f56b42039a62393c66775346b`
- runtime matrix SHA-256: `369781c736eaffcf651f104b78ee28ca4742fa42b6be0c0a002033d55d69aa0c`
- surface pixel matrix SHA-256: `8d460ab34abc767acafdfffb49e10d6aed868f83916552478e6da17567e4d3a9`
- 결과: geometry 장면 2/2, surface 표본 7/7
- 최대 geometry 오차: 0 CSS px
- 최대 RGBA 채널 오차: 0
- scene마다 새 앱 process를 시작했다.
- 상세 결과: [comparison.json](./comparison.json), [device.json](./device.json)
- 각 scene의 runtime 로그, screenshot, 표면 표본 및 비교 JSON을 같은 폴더에 보존했다.

## 제한

- 두 fixed row-sparse auto-placement scene만 검증했으며 일반 CSS Grid 또는 C11 완료를 뜻하지 않는다.
- Chrome 156 geometry와 지정 simulator runtime을 비교했다. 두 simulator는 실기기가 아니다.
- 표면 픽셀은 Android PixelCopy 또는 iOS Metal drawable 내부 표본이며 OS compositor 출력은 확인하지 않았다.
- 비겹침 item만 사용해 painting order, overlap, stacking을 검증하지 않았다.
