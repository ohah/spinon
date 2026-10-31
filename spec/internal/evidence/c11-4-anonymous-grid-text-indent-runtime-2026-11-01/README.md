# C11.4 anonymous Grid `text-indent` runtime evidence

## 범위

이 증거는 C11.4 `RuntimeBlockFlexGridTrackSizingV1` opt-in fixture의 anonymous direct-text Grid item 하나에 한정된다. Ahem 10px/400, horizontal-tb/LTR, `white-space:normal`, `word-break:normal`, 0 gap/edge에서 `0px`, `5px`, `10px`과 두 단어·collapse된 세 단어·한 단어, `min-content`/`max-content`를 조합한 고정 18 scenes를 사용한다. CSS 전반의 `text-indent`, arbitrary unit/percentage/negative/hanging/each-line, inline run, WPT 적합성, glyph paint 또는 제품 CSS admission을 주장하지 않는다.

Chrome 156 Agent Browser의 사전 관찰과 Ahem font table에 기반한 독립 산식은 [precomparison](../c11-4-anonymous-grid-text-indent-precomparison-2026-11-01/chrome156-observation.json)과 [scene 기대값](spec-derived-expectations.json)에 있다. Chrome observation은 Spinon 실행 결과를 대신하지 않는다.

## Simulator 결과

- Android API 37 ARM64 AVD `emulator-5554`, model `sdk_gphone16k_arm64`: 18/18 PASS. 빌드 APK와 설치 APK SHA-256은 `b98725d6446cefa10b54e302e670ea208684d5ab15a2b13d27da06d1eb4f1d2e`로 일치한다. [matrix](android-avd-c114-computed-text-indent-android-avd-1793463802457-25e59203/matrix.tsv) · [simulator metadata](android-avd-c114-computed-text-indent-android-avd-1793463802457-25e59203/simulator.json) · [대표 화면](android-avd-c114-computed-text-indent-android-avd-1793463802457-25e59203/representative-screen.png)
- iPhone 17 Pro / iOS 26.2 Simulator, `iPhone18,1`: 18/18 PASS. 빌드 executable과 설치 executable SHA-256은 `26573ce7035f6fcd2f80981806fc3fd8888fd27d69083adcb6a10a7eff281345`로 일치한다. [matrix](ios-simulator-c114-computed-text-indent-ios-simulator-1793463450444-9a3928d9/matrix.tsv) · [simulator metadata](ios-simulator-c114-computed-text-indent-ios-simulator-1793463450444-9a3928d9/simulator.json) · [대표 화면](ios-simulator-c114-computed-text-indent-ios-simulator-1793463450444-9a3928d9/representative-screen.png)

각 scene 로그는 computed property와 HostDocument revision, intrinsic min/max/height, shaped line advance, item frame, Columns/Rows 최종 track을 비교한다. 두 simulator 앱은 실행 후 종료했다. 캡처에는 앱 상태 화면이 보이나 renderer 영역은 검게 표시되므로 glyph paint나 PixelCopy/Metal drawable/compositor 이후 pixel 근거가 아니다.

## Test, lint, artifact 근거

- `cargo test --locked -p spinon-text-layout`: 76 passed. [로그](text-layout-pass02.log)
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test`: 356 passed, 3 ignored. [로그](runtime-pass02.log)
- `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test`: 177 passed, 2 ignored. [로그](ffi-pass02.log)
- `node --test tools/css-reference/c11-4-computed-text-indent.test.mjs tools/css-reference/fixture-module-boundary.test.mjs`: 26 passed. [최종 Node 결과](node-tests-pass02.log). Rust fmt, diff whitespace check, relevant `spinon-text-layout`/`spinon-runtime`/`spinon-ffi` all-target Clippy `-D warnings`, Android app build, iOS Simulator app build도 통과했다. [Android build](android-build-pass02.log) · [iOS build](ios-build-pass04.log) · [Clippy logs](clippy-pass01/) · [최종 text-layout Clippy](clippy-pass02/text-layout.log)
- `spinon-ffi` macOS release static archive boundary: 기본 archive에는 신규 scene ID 0개와 adapter/validator symbol 0개, `c04-runtime-gpu-fixture-test` positive control에는 18 scene ID와 두 symbol이 있다. 해시·문자열 근거는 [artifact-boundary](binary-boundary-pass01/)에 있다. 이는 APK/IPA 또는 release app 전체 경계 검증이 아니다.
- `cargo test --locked -p spinon-style-to-layout`는 변경 바깥의 C07.3 aspect-ratio serialization 기대값 차이 1건으로 267/268이다. 자세한 실패는 [로그](style-to-layout-pass01.log)에 남겼으며 이번 slice의 통과 결과로 합산하지 않는다.

**상태 경계:** C11.4 `text-indent`는 이 고정 fixture slice만 검증했다. C11/C11.4 공식 완료 check은 미완료이며 공식 집계는 `72/213`이다.
