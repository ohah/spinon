# C11.4 span-stage trace test implementation evidence

## 범위

이 evidence는 [계획 PR #641](https://github.com/ohah/spinon/pull/641)에서 승인된 four-case, four-column span-2/span-3 oracle의 test-only implementation을 기록한다. 변경 코드는 [`grid_multispan_chrome_oracle_tests.rs`](../../../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests.rs) 한 파일이다. CSS/runtime/FFI/renderer/native code와 C11.4 public support state는 변경하지 않는다.

`assert_stage_snapshot`은 각 stage에서 CSS track 수와 값을 gutter에서 분리해 확인한다. Internal vector는 CSS track `N`개에 대해 `2N+1`개의 alternating gutter/track entries다. Taffy `initialize_grid_tracks`는 first/last boundary gutter를 collapsed sentinel로 두며, 이 no-auto-fit fixture의 inner zero-gap gutters와 CSS tracks는 expanded다. Test는 이 exact trace flag pattern, batch span/count/order, finite base/growth values, final remaining space를 검사한다. Expected unbounded trace limit은 serializer `None` projection만 대조하며 내부 `+∞`와 NaN/`−∞` 구분을 주장하지 않는다.

## 실행 결과

Implementation branch base: `a33b245983fae050b66efb9ff259b3fe1132a051` (PR #641 merge commit).

| Command/check | Result |
| --- | --- |
| `cargo test --locked -p spinon-layout grid_multispan_chrome_oracle_tests` | 2 passed |
| `cargo fmt --all -- --check` | passed |
| `cargo clippy --locked -p spinon-layout --all-targets -- -D warnings` | passed |
| `derive_expected.py` output vs committed `expected.json` | exact match |
| `git diff --check` | passed |

The same-line invalid placement control uses `GridPlacementMode::Existing` because `TrackSizing` mode intentionally bypasses `normalize_grid_item_placements`; it expects `LayoutError::InvalidGridPlacement` and no successful `LayoutOutput`.

The implementation/runtime failure review first found that boundary gutter `is_collapsed` flags were not asserted. The implementation now checks all `2N+1` entry kinds and collapse flags; the targeted regression passed. The follow-up review changed the effective result from 19 PASS/1 PARTIAL to **20/20 PASS**. See [implementation review](../c11-4-span-stage-oracle-implementation-review-20-2026-10-26.md) and [targeted recheck addendum](../c11-4-span-stage-oracle-implementation-review-addendum-2026-10-26.md).

## 제한

This is a Rust layout trace test. It does not run CSS cascade, an app, browser capture, Android/iOS simulator, pixel comparison, hardware GPU, WPT, or general C11.4 matrix. The existing Chrome 155 screenshot is a separate final-geometry baseline, not a new intermediate trace capture. C11.4 Stage B and official `72/213` remain incomplete.
