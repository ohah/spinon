# C11.4 anonymous Grid text negative `text-indent` runtime evidence

## Implementation PR

Implementation PR [#938](https://github.com/ohah/spinon/pull/938) was Rebase and merged at `2026-10-31T18:34:54Z` with merge SHA `8c5ca0c62ed030642455f53122928b25bba80fda`. GitHub reported an empty check rollup, so this record does not claim CI passed. C11/C11.4 remain incomplete and the official count remains `72/213`.

[병합 후 상태 검토 20개](post-merge-status-review-20.md)도 공식 checkbox, 72/213 집계, PR metadata, 실행 범위와 artifact 표현을 다시 확인했다.

## 범위

기존 fixture 전용 computed `text-indent` profile에 `-5px`와 `-10px`를 추가한 12개 고정 장면이다. 기존 `0px`·`5px`·`10px` 장면 18개를 회귀로 유지해 전체 30개다. Ahem 10px/400, horizontal-tb/LTR, normal whitespace, line-height 10px, anonymous direct text 하나, edge/gap 없는 Grid의 단일 min-content 또는 max-content column만 다룬다. 기대값은 [CSS 규범·Chrome 156 Agent Browser 관찰·독립 Ahem 계산](../c11-4-anonymous-grid-text-negative-indent-precomparison-2026-11-01/README.md)에 연결한다. `-30px`는 oracle-only 경계다.

## 시뮬레이터 실행

- Android API 37 ARM64 AVD `emulator-5554`에서 30/30 PASS. Android 기대값 어댑터의 max-content line advance를 수정한 뒤 다시 빌드·실행한 최종 APK와 설치된 APK SHA-256은 `76aa0a6264a0930b2c2d182825dd69dc41b4030231aa42de87229c83259167ac`으로 일치한다. [최종 matrix](android-avd-c114-computed-text-indent-negative-android-avd-1793471314021-0e2d635a/matrix.tsv) · [장치 정보와 APK hash](android-avd-c114-computed-text-indent-negative-android-avd-1793471314021-0e2d635a/simulator.json) · [화면](android-avd-c114-computed-text-indent-negative-android-avd-1793471314021-0e2d635a/representative-screen.png) · [최종 30-scene 실행 로그](android-capture-review-fix.log) · [수정 후 Android build](android-build-review-fix.log)
- iPhone 17 Pro / iOS 26.2 Simulator `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`에서 30/30 PASS. 빌드와 설치 executable SHA-256은 `6d89068adf493136ff95f78db3b109ca8ef958a0b1bc6316b70c9df4d31393da`로 일치한다. [matrix](ios-simulator-c114-computed-text-indent-negative-ios-simulator-1793470656069-b87d34b9/matrix.tsv) · [시뮬레이터 정보](ios-simulator-c114-computed-text-indent-negative-ios-simulator-1793470656069-b87d34b9/simulator.json) · [화면](ios-simulator-c114-computed-text-indent-negative-ios-simulator-1793470656069-b87d34b9/representative-screen.png)

각 장면에서 computed value와 HostDocument revision, intrinsic min/max/height, line count와 각 line advance, item frame, Columns/Rows 최종 track을 검사했다. 앱 실행이 끝난 뒤 AVD 앱은 force-stop했고 iOS 앱은 terminate했다. 캡처 화면은 glyph pixel 검증이 아니며 실기기나 Android hardware GPU 결과로 확대 해석하지 않는다.

PR diff 검토 중 Android expectation adapter가 max-content scene의 유일한 line advance에 첫 단어 폭을 사용하고 있음을 발견했다. expected width를 전체 unindented max-content advance로 바꾸고 CSS reference static test에 이 분기를 고정했다. 수정 뒤 fixture app build, 26개 Bun 검사를 다시 통과했고, 새 설치 hash가 빌드 APK와 일치하는 상태에서 Android 30/30을 재실행했다. 앞선 Android capture는 최종 근거로 사용하지 않는다.

## 확인 중 수정한 실행 결함

첫 AVD 시도에서 앱이 시작 직후 종료됐다. logcat 원인은 Android fixture 기대 배열이 9개인데 새 검증기가 index 9를 읽은 `ArrayIndexOutOfBoundsException`이었다. `minContentHeight`를 포함해 기대 배열을 10개 항목으로 바로잡고 앱을 다시 빌드했다. 다음 시도에서는 고정 C11.4 viewport의 초기 scale `1.0`을 `applyLatestEnvironment`가 기기 density `2.625`로 덮어써 초기 eval이 surface 변경에 supersede됐다. C11.4 fixture에도 동일한 고정 scale을 적용하도록 수정한 뒤 전체 AVD matrix가 30/30 통과했다. 이는 ADB 연결 문제가 아니었다.

## 코드·빌드 검증

- `cargo test --locked -p spinon-text-layout`: 76 passed. [로그](text-layout-tests.log)
- `cargo test --locked -p spinon-runtime --lib --features c11-4-grid-track-sizing-fixture-test text_indent`: 5 passed. [로그](runtime-text-indent-tests.log)
- `cargo test --locked -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test computed_text_indent_matrix_generates_thirty`: 1 passed. [로그](ffi-text-indent-tests.log)
- `bun test tools/css-reference/c11-4-computed-text-indent.test.mjs tools/css-reference/fixture-module-boundary.test.mjs`: 26 passed. [기준 로그](node-tests.log) · [수정 후 최종 로그](node-tests-review-fix.log)
- `cargo fmt --all -- --check`, runtime/FFI fixture-feature Clippy `--all-targets -- -D warnings`, Android fixture app build 및 iOS Simulator fixture app build가 통과했다. [fmt](rustfmt-check.log) · [runtime Clippy](runtime-clippy.log) · [FFI Clippy](ffi-clippy.log) · [Android build](android-build.log) · [iOS build](ios-build.log)

## static archive feature 경계

macOS release `spinon-ffi` 기본 archive에서는 30개 scene ID와 `computed_text_indent`/`validate_computed_text_indent` symbol이 모두 0개였다. fixture-test positive control에는 30개 scene ID와 두 symbol이 있다. SHA-256과 검색 결과는 [boundary 근거](binary-boundary-pass01/)에 있다. 이 검사는 전체 APK/IPA fixture exclusion을 뜻하지 않는다.

## 상태 경계

이 결과는 `-5px`·`-10px`의 고정 fixture slice다. 임의 음수 길이, `-30px`, percentage, `hanging`/`each-line`, 일반 CSS admission, glyph paint/pixels, WPT 적합성, 실기기 또는 전체 C11.4/C11을 추가하지 않는다. 공식 C11/C11.4 체크와 완료 수 `72/213`은 유지한다.
