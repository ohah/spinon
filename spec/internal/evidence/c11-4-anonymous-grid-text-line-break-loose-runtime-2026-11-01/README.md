# C11.4 anonymous Grid text computed `line-break: loose` 실행 근거

## 범위

이 실행은 기존 computed `line-break` 5개 회귀 장면과 신규 `loose` 4개 장면을 합친 고정 9-scene opt-in 행렬을 다룬다. 일반 author CSS admission, 임의 문자열·font·locale, WPT, glyph rasterization, 실기기/hardware GPU 및 전체 C11.4/C11 완료를 뜻하지 않는다. Chrome 156 사전 관찰과 CSS Text 3·UAX #14에서 독립 유도한 입력·기대값은 [사전 비교 근거](../c11-4-anonymous-grid-text-line-break-loose-precomparison-2026-11-01/README.md)와 [expected-values.json](expected-values.json)에 고정했다.

입력 문자열 `文＄文`의 세 글자 advance는 고정 Noto Sans KR 10px에서 각각 10px다. `lang=ja; line-break:loose`는 세 경계에서 나뉘어 min-content 10px·auto row 30px이고, `lang=en; loose` 및 `lang=ja; normal` prefix control은 20px·20px다. loose max-content는 30px·10px다. 기존 `文〜文` normal/strict 장면도 같은 행렬에서 회귀 검사한다.

## Simulator 결과

| 대상 | 실행 결과 | 빌드 artifact | 설치 artifact |
| --- | --- | --- | --- |
| Android API 37 ARM64 AVD `emulator-5554` (`sdk_gphone16k_arm64`) | 9/9 scene 통과 | APK SHA-256 `b961d38e966ad8f925544aa76fbee207a380176883fbafb286a4f14a950f70bb` | 같은 SHA-256 |
| iPhone 17 Pro / iOS 26.2 Simulator (`iPhone18,1`) | 9/9 scene 통과 | executable SHA-256 `b6b144abad09e7ca39f8172debb7bb05f173872462b5b287ca37a222ac933683` | 같은 SHA-256 |

각 장면은 computed keyword와 locale, source text, paragraph metrics, intrinsic inline/block 기여, Columns/Rows 최종 track, item frame 및 전체 runtime trace를 검사했다. [Android matrix와 scene logs](android-avd/) · [iOS matrix와 scene logs](ios-simulator/). iOS 첫 장면의 시작 시도 하나는 `status=-12`로 superseded되어 원본을 보존했고, 1회 retry가 통과했다([attempts.tsv](ios-simulator/attempts.tsv)).

첫 Android 실행의 English loose 장면에서 앱 validator의 예상 line advance 순서가 source order와 반대로 적힌 것을 발견했다. 실제 trace는 `[10, 20]`이었다. [실패 원본](android-avd/line-break-en-loose-min-content.failed-attempt-01.log)을 보존하고 Android/iOS 기대값을 source order로 고친 뒤 Android 전체 9/9와 iOS 전체 9/9를 다시 실행했다. 이는 초기 앱 기대값 오류였으며 runtime trace 자체는 CSS text-layout 기대값과 일치했다.

Screenshots: [Android](android-avd/representative-screen.png) · [iOS](ios-simulator/representative-screen.png). 화면에는 fixture route/status와 GPU canvas가 보이지만 glyph ink가 그려지지 않는다. 이 캡처는 text paint, drawable pixel equality, hardware renderer 또는 iOS compositor 이후 화면을 입증하지 않는다.

## 빌드와 검사

- Android: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 tools/build-android-app.sh`
- iOS: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 tools/build-ios-sim.sh`
- Simulator capture: `node tools/css-reference/capture-c11-4-anonymous-grid-text-line-break-simulators.mjs <android-avd|ios-simulator> <serial|UDID> loose`
- `cargo test -p spinon-text-layout c114_line_break --locked` — 3 passed.
- `cargo test -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test computed_line_break --locked` — 3 passed.
- `cargo test -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test anonymous_grid_line_break --locked` — 3 passed.
- `cargo test -p spinon-ffi --features c04-runtime-gpu-fixture-test computed_line_break_scenes_generate_the_exact_allowlisted_inputs --locked` — 1 passed.
- `node --test tools/css-reference/fixture-module-boundary.test.mjs` — 30 passed.
- Feature-scoped Clippy passed for `spinon-text-layout`, `spinon-runtime`, and `spinon-ffi` with `--all-targets -- -D warnings`.
- `cargo fmt --all -- --check`, JavaScript syntax checks, Android/iOS Simulator builds passed.
- Default and fixture-test FFI archive comparison: [artifact boundary](artifact-boundary.md).
- 구현/runtime 실패 관점 검토: [20개](implementation-runtime-review-20.md).
- 최종 PR diff 실패 관점 검토: [30개](pr-diff-review-30.md).

이 근거는 해당 고정 fixture profile만 설명한다. 제품 일반 CSS cascade 지원이나 전체 C11.4/C11 완료를 주장하지 않는다.
