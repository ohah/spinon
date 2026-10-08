# Android 실기기 재검증 · 2026-10-09

## 실행 결과

- **기기:** Samsung SM-S731N, Android 16 / API 36 / SDK_INT_FULL 36.1 / ARM64, 1080×2340, density 450.
- **앱:** `dev.spinon.bootstrap` Debug, version `0.1.0-bootstrap`. 실행 APK SHA-256은 `1639d939a34c7a7dfcf99b53b906d73c12951eae99b87757ecc2e8589b1e572e`이며 빌드 산출물과 기기 설치본을 각각 추출해 비교했다.
- **렌더 경로:** R08 `SurfaceView`와 WGPU/Vulkan, 기기 GPU `Samsung Xclipse 940`을 앱 로그에서 확인했다.
- **합성 입력 음성 대조:** 새 프로세스에서 ADB synthetic tap 1회가 `input_seq=1` → revision 1 submit → 실제 `TransactionStats` callback → 유효한 복제 fence signal로 연결됐다. 동시에 수집한 `sec_touchscreen` raw event는 0건이었다.
- **반복 블록:** 별도의 새 프로세스에서 ADB synthetic tap 10회를 보냈다. input·submit·transaction callback·usable fence signal이 모두 10/10이며, `input_seq`·revision·request ID는 1–10, generation은 모두 1이다. 각 완료 시 `pending=0`, `active=0`, `queue_depth=0`이었다.
- **화면:** 시작 캡처는 파랑 `(71, 123, 237)`, 음성 대조 한 번 뒤는 주황 `(232, 109, 81)`, 반복 10회 뒤는 파랑으로 돌아왔고 화면의 활성화 수는 10이었다. 10회가 짝수이므로 마지막 색이 시작 색과 같은 것이 맞다.
- **직접 입력:** 별도 새 프로세스를 foreground에 두고 30초간 `sec_touchscreen` direct 장치와 앱 로그를 함께 수집했으나 둘 다 접촉 0건이었다. 따라서 실제 손가락 입력 positive sample은 이번에도 얻지 못했다.
- **표시 시각:** API 36은 `target_vsync_id=-1`을 반환했다. fence signal 연결은 확인했지만 event→present 지연, VSync, panel scanout, 광자 시각, 성능 순위는 산출하지 않았다.
- **복구:** 테스트 전에 저장한 APK를 `adb install -r`로 복구했고, 설치본 digest가 테스트 전과 동일한 `2ab336443bba5aabfcd6971ddcf90f7f8118d5e2bcc20435a5a6e18491c34d4a`임을 재확인했다. Chrome을 foreground로 복귀시켰다. 화면 timeout 30,000ms, 밝기 248, 자동 회전, 해상도와 density는 테스트 전후 동일했다. 앱 데이터 삭제나 화면 설정 변경은 없었다.

## 판정 범위

이 실행은 **실제 Samsung 기기에서의 Debug WGPU/Vulkan 렌더 surface와 합성 탭→OS transaction callback→복제 fence 관찰 경로**를 재확인했다. 앱 이벤트의 `input_source`는 `unknown`이므로 raw touchscreen 접촉 없이 실제 손가락 입력으로 분류하지 않는다. 이 결과는 제품 입력 지연이나 표시 완료를 증명하지 않으며 R05.3을 완료 처리하지 않는다.

## 원본

- [환경과 APK 식별](environment.txt)
- [입력 장치 capability](input-device.txt)
- [sequence별 결과 요약](capture-summary.txt)
- [Spinon tag 앱 로그](app-logcat.txt)
- [합성 대조 및 반복 구간 raw touchscreen](raw-touchscreen.txt)
- [실제 입력 관찰 구간 앱 로그](physical-app-logcat.txt)
- [실제 입력 관찰 구간 raw touchscreen](physical-raw-touchscreen.txt)
- [시작 화면](launch.png) · [합성 탭 1회 뒤](synthetic-control.png) · [10회 뒤](after-10-synthetic.png) · [직접 입력 관찰 화면](physical-ready.png)
- [실행 후 독립 실패 관점 20개](runtime-review.md)
- [파일 checksum](SHA256SUMS)
