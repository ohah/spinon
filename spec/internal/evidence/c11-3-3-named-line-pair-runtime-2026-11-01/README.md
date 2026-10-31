# C11.3.3 양 끝 positive implicit named-line pair 구현 근거

**범위:** 고정 크기 Grid item 하나의 Rows 또는 Columns 한 축에서 양 끝 placement가 모두 positive trailing implicit named line으로 해석되는 no-area 제한 profile. 반대 축은 explicit definite placement다.

**구현 계획:** [PR #932 병합](https://github.com/ohah/spinon/pull/932) · merge `dd4477f911a8dcefd6857d0e9b0a942781f72079` · [계획](../../../../plan/c11-3-3-both-implicit-named-line-pairs.md)

**기준 구현:** [Chrome 156 사전 비교](../c11-3-3-both-implicit-named-lines-precomparison-2026-11-01/README.md). HeadlessChrome `156.0.8078.80`, Agent Browser `0.31.1`, macOS, viewport `720×400` CSS px, DPR 1. Chrome 비교 입력은 Spinon 구현 전 실행했다.

**공식 상태:** `spec/STATUS.md`의 `72/213` 및 C11/C11.3 체크는 그대로 둔다. 이 제한 후보는 C11 완료나 WPT 적합성 선언이 아니다.

## 고정된 동작

Chrome reference는 columns/rows 양 축, 반복 occurrence, equal endpoint, 역순 endpoint, 다중 implicit track 분배, fixed-px control의 9 scene을 보존한다. 기준 JSON은 [`c11-3-3-positive-named-line-pairs-reference-v1.json`](../../../../tests/fixtures/css/references/c11-3-3-positive-named-line-pairs-reference-v1.json)이며 SHA-256은 `049247f72ee6f53522fdb2b3096741ce1ed4942e568e5739aaba9013252f5d07`이다. 관찰 JSON SHA-256은 `85e72190c10b4f540d9c322a1d90ef9962ee1717b0f3dd6db4e994e12df71520`, Chrome screenshot SHA-256은 `5c6fddcb06a2eb20069abd135bd0d59d33d0351a7f911986a6988ba35cd784e1`이다.

Android/iOS 앱 경로는 고정된 다중 track 장면 두 개를 실행한다. Rust profile tests는 나머지 CSSOM cases와 invalid shape를 추가로 확인한다.

| 플랫폼 | Scene | Item frame (CSS px) | 표면 비교 |
| --- | --- | --- | --- |
| Android API 37 ARM64 AVD, emulator `emulator-5554` | columns multi-track | `[80, 0, 30, 10]` | SurfaceView PixelCopy 1/1 exact, 최대 채널 오차 0 |
| Android API 37 ARM64 AVD, emulator `emulator-5554` | rows multi-track | `[0, 40, 20, 30]` | SurfaceView PixelCopy 1/1 exact, 최대 채널 오차 0 |
| iPhone 17 Pro / iOS 26.2 Simulator | columns multi-track | `[80, 0, 30, 10]` | Metal drawable 1/1 exact, 최대 채널 오차 0 |
| iPhone 17 Pro / iOS 26.2 Simulator | rows multi-track | `[0, 40, 20, 30]` | Metal drawable 1/1 exact, 최대 채널 오차 0 |

두 축 모두 item 표본은 grid root의 오른쪽 또는 아래쪽 바깥에 있다. 앱 표면 표본의 RGBA는 Chrome reference와 `[229, 72, 77, 255]`로 일치한다.

- [Android columns 실행·화면](./android-avd-c1133-android-avd-20261031T171802734Z-2c0d7a87/README.md) · [PNG](./android-avd-c1133-android-avd-20261031T171802734Z-2c0d7a87/android-screen.png) · [PixelCopy](./android-avd-c1133-android-avd-20261031T171802734Z-2c0d7a87/pixelcopy.png)
- [Android rows 실행·화면](./android-avd-c1133-android-avd-20261031T171819520Z-31b93052/README.md) · [PNG](./android-avd-c1133-android-avd-20261031T171819520Z-31b93052/android-screen.png) · [PixelCopy](./android-avd-c1133-android-avd-20261031T171819520Z-31b93052/pixelcopy.png)
- [iOS columns 실행·화면](./ios-simulator-c1133-ios-simulator-20261031T171829815Z-2b13e7d6/README.md) · [PNG](./ios-simulator-c1133-ios-simulator-20261031T171829815Z-2b13e7d6/ios-screen.png)
- [iOS rows 실행·화면](./ios-simulator-c1133-ios-simulator-20261031T171941587Z-3d6b1b59/README.md) · [PNG](./ios-simulator-c1133-ios-simulator-20261031T171941587Z-3d6b1b59/ios-screen.png)

Android는 ANGLE/Vulkan/SwiftShader software renderer다. iOS 픽셀은 Metal drawable에서 compositor 제출 전 수집했다. 두 결과는 실기기 GPU 또는 OS compositor 이후의 화면 근거가 아니다.

최종 앱 바이너리 SHA-256은 Android `529612a1635630e755445243e85bf9f384c3e531bc5b96306e77676dc74dcdae`, iOS `92e2e14a9c42ee1fcea2650acfcf37a9b1719b79ccb0fa508b4639e9773171d6`다. 초기 Android 시도에서 scene/pixel allowlist 불일치를 발견해 수정했다. 중간 실행 산출물은 checkout에 보존하고 PR에는 최종 실행 근거를 포함했다. 최종 네 실행은 성공 scene과 좌표 거부 경로, Android scene-route 거부 경로를 확인했다.

## 검증

- `cargo test --locked -p spinon-layout`: 326개 통과 ([로그](./cargo-test-layout-final.log)). 전체 두 패키지 실행에서 `spinon-layout` 325개 및 `spinon-style-to-layout` 270개는 통과했고, 변경 파일과 무관한 C07 `aspect_ratio_computed_values_and_frames_match_pinned_chromium_at_both_dprs` 한 건이 computed-style 문자열 `1.7777778 / 1` 대 `1.77778 / 1` 비교에서 실패했다. [전체 실행 로그](./cargo-test-layout-style-after-fix.log).
- C11.3.3 style regression subset: 22개 통과 ([로그](./cargo-test-c11-3-3-style-final.log)).
- `spinon-layout` 10,000-track ceiling candidate: 정확히 10,000 track 배치는 통과하고 한도 초과 배치는 error로 끝난다. trace allocation이 cap 검증을 가리지 않도록 이 한 테스트에서는 선택적 track trace를 껐다.
- opt-in FFI scene allowlist test: 1개 통과 ([로그](./cargo-test-ffi-pair-allowlist.log)).
- CSS reference/runtime static suite: 17개 통과 ([로그](./bun-test-c1133.log)); fixture-module boundary: 25개 통과 ([로그](./node-test-fixture-boundary.log)).
- `cargo fmt --all -- --check`, runtime capture module `node --check`, workspace `cargo clippy --locked --workspace --all-targets -- -D warnings`: 통과 ([서식/구문 확인](./format-and-node-check-final.log), [Clippy 로그](./cargo-clippy-workspace-final.log)).
- `spinon-ffi` default static archive에는 새 scene marker와 C11.3.3 fixture ABI가 없고, `c11-grid-metal-surface-pixels-test` opt-in archive에는 둘 다 있다 ([기본 artifact](./default-artifact-fixture-boundary.log), [opt-in artifact](./fixture-positive-artifact-boundary.log)). 이는 static archive 검사이며 최종 APK/IPA 크기 검증으로 확대하지 않는다.

구현 및 runtime 실패 관점 검토 20개는 [별도 기록](./implementation-review-20.md)에 있다. PR 실제 diff는 PR 생성 뒤 별도로 검토한다.

## 제한

이 slice는 no-area Grid 하나와 direct layout-participating item 하나를 대상으로 한다. 양 끝 endpoint 모두 이름 지정이어야 하고, resolver가 둘 다 positive trailing implicit line으로 계산해야 한다. item의 width·height는 fixed여야 하며 반대 축은 definite placement다. 양 축 implicit, 한쪽 endpoint만 implicit, signed/leading line, span, 숫자 placement, auto-size item, 추가 layout-participating child, template areas, 지원 밖 auto-track 문법은 허용하지 않는다. 실기기 GPU, iOS compositor 이후 픽셀, WPT 적합성, 일반 CSS Grid 전체는 검증되지 않았다.
