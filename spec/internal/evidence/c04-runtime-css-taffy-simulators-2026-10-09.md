# C04.9 · Runtime CSS→Taffy 시뮬레이터 실행 근거

**실행일:** 2026-10-09 · **상태:** Android·iOS Simulator 실제 V8 probe 통과 · **실기기:** 미실행

## 실행한 변경

고정된 V8 checkout revision 7b50b62cb18f28617959e8452e2cd18195b38bcf를 지정해 Android와 iOS Simulator 앱을 각각 다시 빌드했다. 두 실행 모두 JS에서 실제 DOM 노드와 inline style을 만들고, 명시 viewport/media 환경을 등록한 뒤 CSS worker의 layout JSON을 1-byte buffer 실패·필요 용량 재조회·정확한 buffer 재시도 순서로 읽는다.

    SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 mise exec -- bun run build:android
    adb install -r platforms/android/app/build/outputs/apk/debug/app-debug.apk
    adb shell am start -n dev.spinon.bootstrap/.MainActivity --ez spinon_c048_ua_cascade true
    SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 mise exec -- bun run build:ios-sim
    xcrun simctl install booted build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app
    xcrun simctl launch --terminate-running-process booted dev.spinon.bootstrap --spinon-c048-ua-cascade

## 기기별 결과

| 실행 대상 | 결과 |
|---|---|
| Android ARM64 emulator, sdk_gphone16k_arm64, Android 17 / API 37 | 2026-10-09 19:01 KST 빌드·설치·실행 성공. Logcat: status 0, 기존 UA cascade PASS, runtime_layout PASS, schema spinon.runtime.layout, frames 5, chromium_geometry PASS, documentRevision 20. |
| iPhone 17 Pro Simulator, iOS 26.2, arm64 | 2026-10-09 19:02 KST 빌드·설치·실행 성공. Unified log: status 0, 기존 UA cascade PASS, runtime_layout PASS, schema spinon.runtime.layout, frames 5, chromium_geometry PASS, documentRevision 20. |

각 플랫폼의 상세 원본은 [Android log](c04-runtime-css-taffy-android-2026-10-09.log), [iOS log](c04-runtime-css-taffy-ios-2026-10-09.log), [Android 화면](c04-runtime-css-taffy-android-2026-10-09.png), [iOS 화면](c04-runtime-css-taffy-ios-2026-10-09.png)이다. 두 화면은 진단용 native app의 PASS 상태와 JSON readback 결과를 보여준다. CSS frame이 GPU 화면에 그려졌다는 증거는 아니다.

## 비교한 프레임

[구현 전 고정 Chromium 기준](css-c04-runtime-style-layout-precomparison-2026-10-09.md)의 5개 노드를 실제 V8 probe와 Rust fixture가 DOM 순서로 확인한다. 노드 ID 순서를 일부러 DOM 순서와 다르게 만들었다.

| DOM 순서 | nodeId | 예상 Chromium CSS px frame | 판정 |
|---:|---:|---|---|
| 1 | 1 | (0, 0, 300, 140) | 0.5 CSS px 이하 |
| 2 | 3 | (7, 52, 46, 36) | 0.5 CSS px 이하 |
| 3 | 2 | (62, 28, 104, 84) | 0.5 CSS px 이하 |
| 4 | 4 | (64, 33, 30, 11) | 0.5 CSS px 이하 |
| 5 | 5 | (0, 0, 0, 0), display:none subtree | 0.5 CSS px 이하 |

고정 입력/reference/tool hash와 수집 환경은 Chromium 비교 근거에 기록되어 있고 이 실행에서 덮어쓰지 않았다. property는 고정 reference와 Rust fixture가 문자열로 비교하고, 각 x/y/width/height는 좌표별 절대 오차 0.5 CSS px 이하를 요구한다.

## 부가 관측값의 경계

이번 최종 재실행의 Android 단일 진단 표본은 css_worker_ready_us 5445, session_startup_us 10216, 256 detached node에서 snapshot_clone_large_us 18, snapshot_submit_large_us 14, cascade_compute_large_us 260이었다. iOS 표본은 각각 899, 23807, 8, 10, 143이었다. Android의 이전 실행 및 iOS의 서로 다른 단일 값은 실행마다 달라질 수 있으며, 이 표본은 첫 실행·시뮬레이터 값이고 실행 순서와 warm-up 영향을 분리하지 않았다. 두 플랫폼 성능을 비교하거나 사용자 성능을 주장하는 데 사용하지 않는다.

Android 빌드는 성공했지만 현재 Android Gradle Plugin 8.13.2가 compile SDK 37.2를 검증한 버전이 아니라는 안내가 출력됐다. 스크립트는 환경의 Android NDK 설정이 고정값과 달라 SDK NDK 27.1.12297006을 선택했다. iOS link는 성공했으며 V8 archive의 중복 debug-map symbol 경고를 출력했다. 앱 실행 probe에는 영향을 주지 않았다.

## 검증 경계

- 이 근거는 V8 DOM mutation → Stylo cascade → Taffy layout → C ABI JSON copy의 시뮬레이터 실행만 확인한다.
- GPU scene 제출, 실제 CSS paint, 화면상 node geometry, input/hit-test, 화면 presentation callback, 실기기, 장기 경합, 성능 우위는 검증하지 않았다.
- 버튼 터치나 ADB injection을 이용한 수집은 하지 않았다. fixture는 앱 실행 시 자체 수행된다.
- Rust 전체 workspace와 Bun 통합 suite 결과는 PR 본문과 별도 작업 결과에 기록한다. 이 문서의 platform log는 앱에서 자동 실행된 probe의 원본이다.

## 캡처와 원본 checksum

| 파일 | SHA-256 |
|---|---|
| Android log | f938bc1451dc55ef0cc86994edcc9fb0c904696494a57f028afbdab415fb3b6a |
| iOS log | b68f9249541097d33eae16546220265342ce9e4fc9ed2a5554692e242f7f1b8b |
| Android screenshot | 2071f91caf1568afddd92e5e6fbce2e4403b23bcdca4f1abb92c480957bc81c9 |
| iOS screenshot | 2ba86b742c1106ffac59c75a36e96fbc65ea2931fbadd8bf98774c1fa27750b9 |
