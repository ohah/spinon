# C11.4 익명 Grid 텍스트 computed `word-break` 런타임 검증

이 기록은 [독립 사전 비교](../c11-4-anonymous-grid-text-word-break-precomparison-2026-11-01/README.md)에서 고정한 12개 입력을 Android와 iOS 앱의 opt-in runtime fixture로 실행한 결과다. 제품 CSS 지원 선언이나 C11/C11.4 완료 판정은 아니다.

## 실행과 결과

- Android: API 37 ARM64 AVD `spinon_api37_1_compat16k` (`emulator-5554`), SwiftShader. `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 tools/build-android-app.sh` 빌드 후 `node tools/css-reference/capture-c11-4-anonymous-grid-text-word-break-simulators.mjs android-avd emulator-5554`를 실행했다. 12/12 장면에서 입력 해시와 설치 APK 해시가 같고 runtime assertion, frame, text layout, intrinsic contribution, track trace 검사가 통과했다. [장면별 로그](./android-avd/) · [행렬](./android-avd/matrix.tsv) · [기기·APK 해시](./android-avd/simulator.json).
- iOS: iPhone 17 Pro / iOS 26.2 Simulator (`iPhone18,1`, UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`). `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 tools/build-ios-sim.sh` 빌드 후 `node tools/css-reference/capture-c11-4-anonymous-grid-text-word-break-simulators.mjs ios-simulator 9DCCE836-7659-4914-B6C1-13F54BFFA9DB`를 실행했다. 설치 executable과 빌드 executable SHA-256이 같고 12/12 장면이 통과했다. [장면별 로그](./ios-simulator/) · [행렬](./ios-simulator/matrix.tsv) · [기기·executable 해시](./ios-simulator/simulator.json).
- 행렬은 Ahem의 `normal`/`break-all`과 한국어 Noto Sans KR의 `normal`/`keep-all`을 min-/max-content track에서 확인한다. 각 trace에서 HostDocument revision, anonymous text style owner, font family·locale·computed keyword, text line advances, intrinsic min/max 입력, 최종 행·열 track size를 검사한다.
- `lang=ko`는 paragraph locale metadata로 보존되지만 pinned Parley 0.12.0에서 이 입력의 `line-break:auto`는 strict segmenter를 사용하고, 그 segmenter의 locale별 분할 차이는 중국어·일본어에만 적용된다. 한국어 `normal` geometry는 locale 유무 대조에서 같으며 `keep-all` 차이는 word-break mode에 따른다. 따라서 이 결과는 한국어 locale별 줄 분할 지원을 입증하지 않는다.
- 앱 엔진의 Noto Sans KR glyph advance 산식은 9.2px이며 런타임은 float 표현으로 기록한다. Chrome 156의 Grid 직렬화 값 9.203125/36.8125px은 브라우저 양자화 관측으로 따로 보존했다. 두 값의 차이를 런타임 오차나 규범 산식에 합치지 않는다.
- FFI static archive 경계도 확인했다. `cargo build --locked -p spinon-ffi --lib --no-default-features --features c04-runtime-gpu` 결과 `target/debug/libspinon_ffi.a`(663,833,160 bytes)에는 고유 scene 문자열과 Noto Sans KR payload의 64-byte 표본이 모두 없었다. `c04-runtime-gpu-fixture-test` positive-control build 결과(742,749,552 bytes)에는 두 항목이 모두 포함됐다. 이는 debug static archive의 fixture 누출 여부만 확인하며 Android/iOS release artifact 크기를 뜻하지 않는다.
- 두 capture 프로세스는 행렬이 끝난 뒤 앱을 각각 force-stop/terminate했다. 시뮬레이터는 사용자 소유 상태를 보존하기 위해 종료하지 않았다.

## 화면 캡처의 범위

[Android 캡처](./android-avd/representative-screen.png)와 [iOS 캡처](./ios-simulator/representative-screen.png)는 fixture 앱의 ready 상태와 viewport를 식별한다. 현재 GPU surface의 텍스트 영역이 검정색이므로 두 이미지는 글리프 rasterization, 화면 색상, compositor 제출 후 픽셀을 증명하지 않는다. 텍스트 배치 판정은 앱 로그의 layout/track diagnostics로 했다. 시각적인 줄바꿈 oracle은 사전 비교의 [Chrome 156 캡처](../c11-4-anonymous-grid-text-word-break-precomparison-2026-11-01/chrome156-word-break-precomparison.png)다.

## 남은 범위

검증한 것은 이 고정 fixture subset의 Android API 37 AVD와 iOS 26.2 Simulator 동작이다. 실기기, 일반 author CSS admission, 다른 언어·글꼴·writing mode·`overflow-wrap` 결합, 글리프 painting, 전체 CSS Text 및 CSS Grid 적합성을 입증하지 않는다. 공식 상태 대장의 C11/C11.4 미체크와 로드맵 집계 72/213은 이 slice로 바꾸지 않는다.
