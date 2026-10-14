# C13.1 overflow clip runtime matrix · 2026-10-14

## Oracle and fixture

The browser capture used agent-browser 0.31.1 with Google Chrome for Testing `155.0.8059.40`, revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}`, HeadlessChrome/155 user agent, 1280×1100 CSS px, DPR 1. The original repository fixture is `tests/fixtures/css/c13/overflow-clip-c13-1.html`, SHA-256 `b7acd7e63f79355b28cd480511b9be82e0f2bef478f1aadf685eb3b0bf4ef16a`. Its pinned precomparison observation is SHA-256 `fd6d9e66825bc453609245950635f4fc56054ae9011b14d2f8901d490ecde4b5`.

[`chrome155-runtime-matrix-observation.json`](chrome155-runtime-matrix-observation.json) records ten live Chrome rows. The prior three selected rows (`clip-y`, `clip-margin-border`, `clip-margin-hidden`) remain in [the earlier observation](../c13-1-overflow-clip-matrix-2026-10-14/chrome155-selected-observation.json). Together, the Rust module checks 13 target cases. For each case it compares frame edges and each integer CSS-pixel target hit interval on both axes; this is not a two-dimensional sweep of the whole viewport.

## 17행 런타임 판정 범위

| 행 유형 | 수 | Rust 판정 | 남은 표면 검증 |
| --- | ---: | --- | --- |
| 유효 C13.1 geometry case | 13 | frame, 축별 target interval, target frame 안의 Chrome topmost NodeId를 비교한다. 이전 exact identity는 [축별 근거](../c13-1-overflow-clip-hit-identity-2026-10-14/README.md), target frame의 모든 정수 `(x,y)` 표본은 [2D 후속 근거](../c13-1-overflow-clip-hit-identity-2d-2026-10-14/README.md)에 있다. | 유효행 전체 PixelCopy/Metal surface, target frame 밖 전체 viewport paint·hit·event 표본은 미완료다. |
| 음수 `overflow-clip-margin` | 1 | cascade diagnostic 뒤 layout 전에 reject한다. [검증 로그](cargo-negative-margin-test.log)는 1 passed다. | Chrome의 paint/event 동작을 제품 의미로 채택하지 않았다. |
| float/BFC 변형 | 3 | Chrome 비교 3행 모두를 [`c131_float_formatting_comparison_cases_fail_closed`](../../../../crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c13_1_float_boundary_tests.rs) 한 테스트가 순회해 `unsupported_inline_property`로 reject한다. [정정된 실행 로그](cargo-float-failclosed-test.log)는 1 passed다. | float geometry parity는 C26 소유 범위다. |

따라서 17행의 런타임 disposition은 확인했지만 17행 surface matrix나 C13.1 완료를 뜻하지 않는다. 처음 추가한 float 실행 로그는 잘못된 package/filter로 테스트 0개를 선택했으며 [`기록`](cargo-float-filter-no-match.log)은 그대로 보존했다. 올바른 package와 filter의 실행 결과는 위 정정 로그를 따른다.

| New Chrome row | Overflow / margin | target frame (x, y, w, h) | target hit x / y, inclusive integer CSS px |
| --- | --- | --- | --- |
| `visible` | visible / 0px | 34, 98, 140, 120 | 34–173 / 98–217 |
| `clip-x` | clip / visible | 232, 98, 140, 120 | 232–337 / 98–217 |
| `clip-both` | clip / 0px | 34, 266, 140, 120 | 34–139 / 266–347 |
| `clip-margin-padding` | clip / 8px | 232, 266, 140, 120 | 232–341 / 266–351 |
| `clip-margin-content` | clip / content-box 8px | 430, 266, 140, 120 | 430–535 / 266–347 |
| `mixed-visible-hidden` | auto / hidden | 34, 602, 140, 120 | 34–139 / 602–683 |
| `mixed-clip-auto` | hidden / auto, border-box 8px | 232, 602, 140, 120 | 232–337 / 602–683 |
| `positioned-contained` | clip / 0px, contained absolute child | 504, 822, 42, 30 | 504–531 / 822–851 |
| `positioned-escape` | clip / 0px, outer containing block | 114, 968, 42, 30 | 114–155 / 968–997 |
| `nested-clip` | nested clip x/y axes | 253, 961, 140, 120 | 253–323 / 961–1015 |

The separate [`content-box-positioned-hit-control.html`](content-box-positioned-hit-control.html) is a 200×160 CSS px controlled case for the previous test whose expectation claimed that positioned descendants retained the full border strip. The corresponding live observation records target frame x=74..134, y=54..94 and target hit x=74..119, y=54..89 with `overflow-clip-margin: content-box 8px`. Its fixture SHA-256 is `cf635425b6717a565da37f29649bc04723ab3358fa8f2bf8783bd4721697217f`; the observation is [`chrome155-content-box-positioned-hit-observation.json`](chrome155-content-box-positioned-hit-observation.json).

## Failure and correction

The expanded matrix exposed a renderer hit-test error: a positioned `overflow: clip` ancestor used its border-box union for a contained absolute descendant. Before the fix, the `positioned-contained` row admitted x=416..535 where Chrome admitted x=504..531; the outer clip in `nested-clip` admitted y=924..1019 where Chrome admitted y=928..1015. Both red failures were reproduced against the same fixed Chrome capture.

`runtime_clips_for_node` now distinguishes an absolute descendant whose containing block is inside the clip lineage. For that descendant, `clip` hit bounds use the visual clip interval, matching paint. A positioned clip with only in-flow descendants still uses the prior paint-interval/border-box union, and an absolute descendant whose containing block lies outside the clip ancestor still skips that ancestor. Paint clip calculation did not change. The content-box control updates the stale prior assertion and checks a positive point at x=119 plus a negative border-strip point at x=122.

## Verification

- [`cargo-matrix-test.log`](cargo-matrix-test.log): `cargo test --locked -p spinon-style-to-render c13_1_clip_matrix`, 8 passed.
- [`cargo-contentbox-test.log`](cargo-contentbox-test.log): the dedicated controlled content-box hit regression, 1 passed.
- [`cargo-render-test.log`](cargo-render-test.log): `cargo test --locked -p spinon-style-to-render`, 52 passed, 0 failed, 1 ignored.
- [`cargo-clippy.log`](cargo-clippy.log): `cargo clippy --locked -p spinon-style-to-render -p spinon-style-to-layout --all-targets -- -D warnings`, passed.
- [`cargo-fmt.log`](cargo-fmt.log) and [`git-diff-check.log`](git-diff-check.log): `cargo fmt --all -- --check` and `git diff --check`, both passed.
- The Android APK was rebuilt with `SPINON_ENABLE_C04_RUNTIME_GPU=1` and `SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST=1`; [`android-build.log`](android-build.log) ends with `BUILD SUCCESSFUL in 6s`. The log contains an Android Gradle Plugin compile SDK 37.2 compatibility warning and records use of pinned NDK 27.1.12297006.
- [`android-install.log`](android-install.log) and [`android-start.log`](android-start.log) record APK installation and a cold route start with `Status: ok`. `adb devices -l` listed one target, `emulator-5554` (`sdk_gphone16k_arm64`, API 37 ARM64). No physical device was connected. The current process log uses `adb logcat --pid=8712`; run `179b65d965f7` evaluated 13 nodes, matched 15/15 hit queries, and matched 11/11 PixelCopy color samples. The selected renderer was ANGLE/SwiftShader software GL.

The app rebuild command was:

```sh
SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 \
SPINON_ENABLE_C04_RUNTIME_GPU=1 \
SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST=1 \
  mise exec -- bash tools/build-android-app.sh
```

The APK was installed on the already booted AVD with `adb -s emulator-5554 install -r platforms/android/app/build/outputs/apk/debug/app-debug.apk`, then launched with `adb -s emulator-5554 shell am start -S -W -n dev.spinon.bootstrap/.MainActivity --ez spinon_c131_overflow_clip true`.

After the capture, the app was stopped with `adb -s emulator-5554 shell am force-stop dev.spinon.bootstrap`. The pre-existing AVD remains booted.

The Android route is a regression check of the existing 13-node fixture. It does not contain the newly added contained-absolute case, so it is not independent Android verification of the new behavior. [`android-route.log`](android-route.log) contains only the current app PID and its timestamped runtime evidence.

## Limits

Chrome and Rust compare target frames and axis-wise integer hit intervals, not full-surface paint. The AVD PixelCopy samples belong to the existing fixture only. No WPT runner, DOM event dispatch, full viewport hit sweep, display scanout, physical device, or hardware GPU was tested. C13.1 and C13 remain incomplete.

## Adversarial reviews

[Implementation failure review: 20 perspectives](implementation-review-20.md) and [PR delta review: 30 perspectives](pr-delta-review-30.md) are separate reviews for this matrix, hit-test fix, controlled regression, evidence, and status changes.
