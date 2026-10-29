# C11.3.3 unknown-area implicit `auto` track 구현 근거

이 변경은 기존 `unknown-area-implicit-tracks` fixture에 한정된 확장이다. 이전 기대 거부는 runtime matrix v1에 보존하고, Chrome 154 기준 성공을 source hash가 고정된 v2에 기록한다. C11.3.3 또는 C11 전체를 완료로 표시하지 않으며 공식 집계(`72/213`)도 바꾸지 않는다.

## Oracle 및 시뮬레이터 실행

- 계획: [C11.3.3 unknown-area implicit auto track](../../../../plan/c11-3-3-unknown-area-auto-implicit-tracks.md). 계획 PR [#818](https://github.com/ohah/spinon/pull/818)은 `a234244ba3ab88ac324e122a1c61a8ff143b3c70`으로 병합됐다.
- 변경 전 보존 matrix v1 SHA-256: `62baa58bab5af9fd9cfb498ed4a5a979202a5e43bba5438952358cfc204358ee`.
- Runtime matrix v2 SHA-256: `6064bafe2e53c51696e13992a6d0bc0a9bc96188ea46a5f82052063ab47cbd6e`. v1과 기존 inventory v1을 수정하지 않고 과거 자료로 연결한다.
- Chrome 154 기대 geometry: root `(0,0,160,120)`, 비어 있는 unknown-area item `(160,120,0,0)`.
- Android API 37 ARM64 AVD/SwiftShader: [최종 실행 README](../c11-3-3-grid-template-areas-runtime-2026-10-30/android-avd-c1133-matrix-20261029T203832317Z-36dd5796/README.md), [결과](../c11-3-3-grid-template-areas-runtime-2026-10-30/android-avd-c1133-matrix-20261029T203832317Z-36dd5796/result.json), [화면](../c11-3-3-grid-template-areas-runtime-2026-10-30/android-avd-c1133-matrix-20261029T203832317Z-36dd5796/android-screen.png).
- iPhone 17 Pro/iOS 26.2 Simulator: [최종 실행 README](../c11-3-3-grid-template-areas-runtime-2026-10-30/ios-simulator-c1133-ios-matrix-20261029T204016316Z-f19f0361/README.md), [결과](../c11-3-3-grid-template-areas-runtime-2026-10-30/ios-simulator-c1133-ios-matrix-20261029T204016316Z-f19f0361/result.json), [화면](../c11-3-3-grid-template-areas-runtime-2026-10-30/ios-simulator-c1133-ios-matrix-20261029T204016316Z-f19f0361/ios-screen.png).

두 결과 모두 source commit `92c8502be33ada9e2a71e32bea69ff048ea3f109`에서 생성됐다. Android 결과에는 runtime matrix, Java source, capture script, APK hash가 기록되고 iOS 결과에는 해당 platform source와 app hash가 기록된다.

두 플랫폼에서 37 scene·118 node를 대조했다. 결과는 성공 32, cascade fallback 5, 기대 거부 0, 최대 geometry 오차 0 CSS px이다. 대상 scene의 두 node도 Chrome geometry와 정확히 일치한다. 기본 surface sample은 각 플랫폼 2/2, fixed-track overflow control은 각 3/3, track-list cycle 표본은 각 28/28 exact다. iOS Metal readback은 OS compositor 제출 전이며 AVD는 SwiftShader를 사용하므로 실기기 또는 hardware GPU 동작을 증명하지 않는다.

## 로컬 검사

- `cargo fmt --all -- --check` — 통과.
- `cargo test -p spinon-layout --lib grid::tests` — 19/19.
- `cargo test -p spinon-style-to-layout --lib c11_3_3_grid_template_areas_tests` — 19/19.
- `cargo test -p spinon-ffi --features c11-grid-metal-surface-pixels-test c11_3_3` — 7/7.
- `cargo test -p spinon-layout --lib auto_track_sizing_tests::track_sizing` — 13/13.
- `cargo test -p spinon-style-to-layout --lib c11_4_track_sizing_areas_tests` — 20/20.
- feature-gated C11.4 mixed text/image 회귀 — 3/3.
- CSS reference/iOS matrix Node tests — 25/25, fixture-module boundary — 23/23.
- Runtime matrix builder `--check` — 통과, 37 scene·118 node.
- `cargo clippy --locked --workspace --all-targets -- -D warnings` — 통과.
- 기본 및 opt-in Release FFI archive의 fixture 심볼/payload 분리는 [artifact 경계 근거](./artifact-boundary.md)에 기록했다.

첫 Android 실행은 기본 scene probe가 과거 거부를 요구해 중단됐다. 중복된 obsolete probe를 제거한 뒤 전체 AVD matrix가 통과했다. 첫 iOS 시도는 legacy pixel fixture의 v1 hash를 신규 v2 matrix와 직접 비교해 실패했다. runner를 고쳐 과거 fixture hash는 v1과 별도로 확인하고, 전체 iOS matrix를 다시 통과했다. 실패 실행은 성공 결과로 합산하지 않았다.

## 검토 자료

- [구현/runtime 실패 관점 20개](./implementation-runtime-review-20.md).
- [구현 PR diff 실패 관점 30개](./implementation-pr-diff-review-30.md).

이 제한 slice는 임의 CSS admission, px 이외 auto-track list, negative/leading implicit sizing, Spinon WPT 적합성, hardware GPU/실기기, iOS compositor 이후 pixel, 일반 CSS Grid 지원을 추가하지 않는다.
