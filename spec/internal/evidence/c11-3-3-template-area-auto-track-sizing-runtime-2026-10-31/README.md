# C11.3.3 area-backed typed `grid-auto-*` 후보 실행 근거

## 판정 범위

이 변경은 `RuntimeBlockFlexGridTemplateV1`의 area-backed `grid-auto-rows`·`grid-auto-columns`에서 이미 구현된 C11.4 typed track 값을 허용하는 제한 후속 후보이다. 기준은 [CSS Grid Layout Module Level 1, 2025-03-26 CRD §§7.3, 7.5–7.6, 8.5, 11.4–11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)과 [Chrome 156 Agent Browser 사전 비교](../c11-3-3-template-area-auto-track-sizing-precomparison-2026-10-31/chrome/README.md)다.

후보 브랜치의 기준 HEAD는 `7bcc6a6641a2d95e607a6f805578a19a5fa84f0a`이며, 아래 결과는 그 HEAD에 현재 dirty source를 적용해 만든 앱·아카이브에서 수집했다. PR/병합 전 상태다.

## Android·iOS Simulator

| 플랫폼 | 실행 | 범위와 결과 | 한계 |
| --- | --- | --- | --- |
| Android API 37 ARM64 AVD | [실행 README](../c11-3-3-grid-template-areas-runtime-2026-10-31/android-avd-c1133-matrix-20261031T075025965Z-f6357ddc/README.md) · [result.json](../c11-3-3-grid-template-areas-runtime-2026-10-31/android-avd-c1133-matrix-20261031T075025965Z-f6357ddc/result.json) · [화면](../c11-3-3-grid-template-areas-runtime-2026-10-31/android-avd-c1133-matrix-20261031T075025965Z-f6357ddc/typed-area-minmax-row-screen.png) | Chrome DPR 1의 39 scene·126 node frame 비교, 최대 geometry 오차 0 CSS px. typed area scene 2개·7개 PixelCopy 표본 exact. | ANGLE/SwiftShader software renderer; 실기기·hardware GPU 근거가 아니다. |
| iPhone 17 Pro / iOS 26.2 Simulator | [실행 README](../c11-3-3-grid-template-areas-runtime-2026-10-31/ios-simulator-c1133-ios-matrix-20261031T075232596Z-bc687ad2/README.md) · [result.json](../c11-3-3-grid-template-areas-runtime-2026-10-31/ios-simulator-c1133-ios-matrix-20261031T075232596Z-bc687ad2/result.json) · [화면](../c11-3-3-grid-template-areas-runtime-2026-10-31/ios-simulator-c1133-ios-matrix-20261031T075232596Z-bc687ad2/typed-area-minmax-row-screen.png) | Chrome DPR 1의 39 scene·126 node frame 비교, 최대 geometry 오차 0 CSS px. typed area scene 2개·7개 Metal drawable 표본 exact. | Metal readback은 OS compositor 제출 전이다. 실기기·hardware GPU 근거가 아니다. |

두 행렬은 34 success, 5 cascade fallback, 0 expected rejection을 기록한다. typed row는 `minmax(20px, 1fr)`의 area 생성 track, typed column은 `25%`의 area 생성 track이다. 기존 fixed-track unknown-area overflow scene은 각 플랫폼에서 Chrome geometry 3개와 root 밖 item 표면 sample까지 3/3 비교했다. 이 overflow 장면은 새 typed scene의 overflow 검증으로 세지 않는다.

## Positive trailing named-line track

`crates/spinon-layout/src/grid_tests.rs`의 `trailing_template_area_named_lines_use_typed_auto_tracks_after_area_pattern_phase`는 row·column 양축에서 `[10px, 25%, 8px]` pattern을 사용한다. area가 각 축의 첫 두 pattern entry를 소비한 뒤 positive trailing named-line expansion이 `8px`, `10px` 순서로 이어지는지, final item frame `(70, 60, 18, 18)`이 되는지 확인하며 통과했다. 이 조합은 Rust layout test에서 직접 검증했다. 두 앱 matrix의 typed scenes에는 이 named-line 조합이 포함되지 않는다.

## 로컬 검증

- `cargo fmt --all -- --check` — [로그](./builds/fmt-check.log), 통과.
- `cargo test --locked -p spinon-layout template_area_ -- --nocapture` — 9/9, [로그](./builds/layout-template-area-tests.log).
- `cargo test --locked -p spinon-style template_profile_preserves_fixed_lists_and_area_backed_typed_auto_tracks -- --nocapture` — 1/1, [로그](./builds/style-cascade-test.log).
- `cargo test --locked -p spinon-style-to-layout template_area_typed_track_lists -- --nocapture` — 1/1, [로그](./builds/style-projection-test.log).
- `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test fixed_runtime_matrix_selects_every_scene_and_rejection_boundary -- --nocapture` — 1/1, [로그](./builds/ffi-runtime-matrix-test.log).
- `cargo clippy --locked --workspace --all-targets -- -D warnings` — [로그](./builds/workspace-clippy.log), 통과.
- Node C11.3.3 matrix, surface fixture, simulator script 및 fixture module-boundary 검사 — 50/50, [로그](./builds/node-c1133-and-boundary-tests.log).
- Android AVD와 iOS Simulator는 각각 위 matrix를 최신 Rust source로 다시 빌드해 실행했다. typed area 표면 sample은 양쪽 각각 7/7 exact다.

## 기본/opt-in artifact 대조

macOS ARM64 Release `spinon-ffi` static archive만 검사했다. 기본 `c04-runtime-gpu` product archive는 75,261,008 bytes, SHA-256 `06a79d41e185bb9a90210b113dd875addb5f45299182112d1b35776ebf8898cf`이며 [scene/source marker](./artifacts/default-product-markers.txt)와 [C11.3.3 FFI export](./artifacts/default-product-symbols.txt)가 모두 비어 있다. `c04-runtime-gpu-fixture-test` positive control archive는 81,142,496 bytes, SHA-256 `a46d0bd730d653bc13afcaef551e9a71774fa848c8a6f5b4e608969b132c9aff`이며 두 typed scene ID, runtime fixture source marker 및 `new`/`eval` export를 포함한다. build logs, hash, byte count, marker와 symbol 결과는 [`artifacts/`](./artifacts/)에 보존했다. 이 검사는 최종 APK/IPA 전체 fixture exclusion을 증명하지 않는다.

## 남은 경계

area 없는 profile, signed/leading implicit track, 일반 CSS author admission, 전체 WPT, Grid §11 전체 sizing matrix, 실기기·hardware GPU와 iOS compositor 이후 pixel은 이 후보에서 검증하지 않았다. 기존 C11.3.3/C11 상태도 완료로 바꾸지 않는다. 공식 완료 수는 `72/213`이며 C11 상위와 C11.4는 미완료다. 구현/runtime 실패 관점은 [별도 검토 20개](./implementation-runtime-review-20.md), 실제 PR 변경 검토는 [30개 독립 관점](./pr-diff-review-30.md)에 있다.
