# C11.4 §11.7 flexible track expansion test-only 구현 근거

**범위:** [계획](../../../../plan/c11-4-flex-stage-oracle.md)의 네 definite empty-grid 입력에 대해 `ExpandFlexibleTracks` trace base를 독립 유리수 결과와 비교한다. 제품 알고리즘, 공개 기능, runtime, FFI와 simulator는 변경하거나 검증하지 않았다.

## 변경

- Rust 변경은 [기존 test 전용 파일](../../../../crates/spinon-layout/src/grid_track_sizing_tests/flex_factor_bound.rs) 하나다. `expected.json`의 고정 네 case에서 typed `GridTrackSize` 입력을 만들고 axis, definite root/viewport 크기, 대상 gap, track descriptor, trace와 rational base 배열을 검사한다.
- Root trace와 대상 축 pass는 각각 하나여야 한다. 고유 `ExpandFlexibleTracks`는 `MaximizeTracks` 뒤, `StretchAutoTracks` 전에 있어야 한다. 각 stage 배열에서 CSS track, gap gutter와 양쪽 collapsed boundary sentinel을 확인한다.
- 구현 실패 관점 [20개 검토](./implementation-review-20.md)와 구현 PR 변경분 [30개 검토](./pr-diff-review-30.md)를 마쳤다. test 파일은 498줄이다.

## 실행 결과

| 명령 | 결과 | 원시 로그 |
| --- | --- | --- |
| `cargo test --locked -p spinon-layout flexible_track_expansion_stage` | 통과, 1 passed · 292 filtered | [focused-test.log](./focused-test.log) |
| `cargo fmt --all -- --check` | 통과, 진단 없음 | [fmt-check.log](./fmt-check.log) |
| `cargo clippy --locked -p spinon-layout --all-targets -- -D warnings` | 통과 | [clippy.log](./clippy.log) |
| `git diff --check` | 통과, 진단 없음 | 실행 당시 shell 결과 |

독립 산출 스크립트를 다시 실행했으며 `expected.json`은 사전 비교와 같은 SHA-256 `07bedf1b3366ef9252cc80eb123c44a14dab50d876c24f398441f3baa9529000`이다. 최종 Rust source SHA-256은 `5e81ba84ff2f812c98be874dbe1ab4da037e8f8784e13142eca53e40fb38b3b4`다. 실행 전 main revision은 `c2d652754fa9e439cd56465b512b9f6119203e17`이었다.

## 한계와 상태

이 검증은 고정 네 test input의 Rust trace만 확인한다. HTML/CSS admission, WPT, item contribution/span, intrinsic sizing, app runtime, Android/iOS simulator, 화면 pixel equality와 hardware device는 확인하지 않았다. C11.4는 미완료이며 공식 `72/213`을 유지한다.
