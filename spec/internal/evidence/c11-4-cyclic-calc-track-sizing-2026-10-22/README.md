# C11.4 cyclic percentage `calc()` track sizing 실행 근거

CSS Grid 2025 CRD의 cyclic percentage 계산에 `calc(percentage + length)` track을 연결한 네 고정 scene을 확인했다. 기대값은 [Agent Browser Chrome 156 사전 비교](../c11-4-cyclic-calc-track-sizing-precomparison-2026-10-22/README.md)와 spec-derived 계산으로 먼저 고정했다. 일반 CSS author admission이나 C11.4 완료 판정은 아니다.

## 결과

| Scene | Columns / Rows | Root / item | Android AVD | iOS Simulator |
| --- | --- | --- | --- | --- |
| `cyclic-calc-column` | `[16]` / `[16]` | `24×16` / `24×16` | frame·trace·draw 통과, PixelCopy `2/2` | frame·trace·draw 통과, Metal drawable `2/2` |
| `cyclic-calc-row` | `[24]` / `[10]` | `24×16` / `24×16` | frame·trace·draw 통과, PixelCopy `2/2` | frame·trace·draw 통과, Metal drawable `2/2` |
| `cyclic-calc-two-axis` | `[16]` / `[10]` | `24×16` / `24×16` | frame·trace·draw 통과, PixelCopy `2/2` | frame·trace·draw 통과, Metal drawable `2/2` |
| `definite-calc-control` | `[24]` / `[12]` | `40×20` / `24×16` | frame·trace·draw 통과, PixelCopy `2/2` | frame·trace·draw 통과, Metal drawable `2/2` |

양 플랫폼에서 `4/4` scene, root/item node frame, 축별 최종 track trace와 draw/present가 통과했다. Android PixelCopy 표본은 총 `8/8`, iOS Metal drawable 표본은 총 `8/8` RGBA exact다. 표본은 cyclic track 경계 직전과 직후의 item 색을 비교해 overflow paint를 확인한다. Android는 API 37 ARM64 AVD `emulator-5558`(SwiftShader), iOS는 iPhone 17 Pro / iOS 26.2 Simulator다. 실기기는 연결하지 않았다.

## 화면

- Android: [column](./android-avd-c114-cyclic-calc-track-sizing-android-avd-20261021T224422209Z-5ec7de6b/cyclic-calc-column.png), [row](./android-avd-c114-cyclic-calc-track-sizing-android-avd-20261021T224422209Z-5ec7de6b/cyclic-calc-row.png), [two axis](./android-avd-c114-cyclic-calc-track-sizing-android-avd-20261021T224422209Z-5ec7de6b/cyclic-calc-two-axis.png), [definite control](./android-avd-c114-cyclic-calc-track-sizing-android-avd-20261021T224422209Z-5ec7de6b/definite-calc-control.png)
- iOS: [column](./ios-simulator-c114-cyclic-calc-track-sizing-ios-simulator-20261021T224442114Z-cd934a2e/cyclic-calc-column.png), [row](./ios-simulator-c114-cyclic-calc-track-sizing-ios-simulator-20261021T224442114Z-cd934a2e/cyclic-calc-row.png), [two axis](./ios-simulator-c114-cyclic-calc-track-sizing-ios-simulator-20261021T224442114Z-cd934a2e/cyclic-calc-two-axis.png), [definite control](./ios-simulator-c114-cyclic-calc-track-sizing-ios-simulator-20261021T224442114Z-cd934a2e/definite-calc-control.png)

## 실행과 수정한 검증 누락

처음 Android PixelCopy 실행에서 새 scene이 sample-scene allowlist와 색상 table에 없어서 검증기가 표본을 거부했다. iOS `cyclic-calc-two-axis`는 공통 기대값 helper가 모든 `-two-axis` suffix를 잘라 기존 기본 frame을 선택했다. harness allowlist·색상 table을 추가하고 suffix 축약을 `nested-depth-*` scene으로 한정한 뒤 두 플랫폼을 다시 빌드해 네 scene 전체를 재실행했다. 첫 오류 로그는 [Android allowlist 실패](../c11-4-cyclic-calc-track-sizing-runtime-2026-10-22-2026-10-22/android-avd-c114-cyclic-calc-track-sizing-android-avd-20261021T224151912Z-544211da/runtime-cyclic-calc-column.log), [PixelCopy 기대 색 누락](../c11-4-cyclic-calc-track-sizing-runtime-2026-10-22-2026-10-22/android-avd-c114-cyclic-calc-track-sizing-android-avd-20261021T224239257Z-118dcfc7/runtime-cyclic-calc-column.log), [iOS 기대 frame 불일치](../c11-4-cyclic-calc-track-sizing-runtime-2026-10-22-2026-10-22/ios-simulator-c114-cyclic-calc-track-sizing-ios-simulator-20261021T224229102Z-cc5f46a1/runtime-cyclic-calc-two-axis.log)에 보존했다. 최종 Android 설치 APK SHA-256은 `bc63c3cee2fe18734b737cfe2675d337b117f2161e73612abdd08bbd44f5f06a`이고 로컬·설치 파일이 일치한다. iOS app binary SHA-256은 `8328acd5a5aeafcf65105fc1210fe606f4c9d4a600e48f1edf6c486ce72d6f76`이다.

## 코드·도구 검증

- Android fixture build와 iOS Simulator fixture build: 성공. 두 빌드에서 `c04-runtime-gpu=1`, `c04-runtime-gpu-fixture-test=1`, `c11-grid-metal-surface-pixels-test=1`을 사용했다.
- Rust focused tests: style-to-layout `1/1`, runtime `1/1`, FFI `2/2` 통과.
- `node --test tools/css-reference/fixture-module-boundary.test.mjs`: `6/6` 통과.
- `cargo fmt --all -- --check`, `git diff --check`, `node --check tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs`: 통과.
- 기본·opt-in Rust release artifact 비교: [artifact-boundary.json](./artifact-boundary.json). 비교는 Rust `.rlib`에 한정하며 최종 Android/iOS release bundle size 검증으로 확대하지 않는다.
- 새 코드/runtime 실패 관점은 [implementation-review-20.md](./implementation-review-20.md)에 있다.
- 최종 변경 diff의 별도 적대 검토는 [pr-diff-review-30.md](./pr-diff-review-30.md)에 있다.
- suffix helper 변경 뒤 기존 `nested-depth-{one,two,three}-{min,max}-content-two-axis` 여섯 scene도 Android AVD와 iOS Simulator에서 각각 `6/6` 통과했다. 원본은 [Android 회귀 폴더](../c11-4-cyclic-calc-nested-depth-regression-2026-10-22/android-avd-c114-cyclic-calc-helper-regression-android-avd-20261021T224808690Z-26a7a24b/)와 [iOS 회귀 폴더](../c11-4-cyclic-calc-nested-depth-regression-2026-10-22/ios-simulator-c114-cyclic-calc-helper-regression-ios-simulator-20261021T224808689Z-34e0a3f5/)에 있다.

## 범위와 한계

이 결과는 네 opt-in scene과 현재 C11.4 runtime profile만 확인한다. `minmax(calc())`, `fit-content(calc())`, 여러 `calc()` track, nested Grid, WPT, 일반 CSS 지원을 확인하지 않았다. Android는 AVD software renderer이며 실기기/hardware GPU가 아니다. iOS 표본은 Metal drawable 결과로 OS compositor 이후 화면을 뜻하지 않는다. C11.4 공식 checkbox와 완료 수 `72/213`은 유지한다.
