# C11.4 span-stage trace test 구현 근거

## 범위

이 근거는 [계획 PR #641](https://github.com/ohah/spinon/pull/641)에서 승인한 네 가지 span-2/span-3 입력의 test-only 구현을 기록한다. 변경 코드는 [`grid_multispan_chrome_oracle_tests.rs`](../../../../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests.rs) 한 파일이다. CSS/runtime/FFI/renderer/native 코드와 C11.4 공개 지원 상태는 변경하지 않는다.

`assert_stage_snapshot`은 각 stage의 CSS track 수와 값을 gutter에서 분리해 검사한다. Internal vector는 CSS track `N`개에 대해 `2N+1`개의 gutter/track 항목을 교대로 담는다. Taffy `initialize_grid_tracks`는 첫/마지막 boundary gutter를 collapsed sentinel로 표시한다. 이 no-auto-fit fixture에서 내부 zero-gap gutter와 CSS track은 collapsed 상태가 아니다. 테스트는 이 항목 구조와 각 collapse flag, batch span/count/order, finite base/growth 값, 최종 남은 공간을 검사한다. Expected unbounded trace limit은 serializer의 `None` 투영만 대조하며 내부 `+∞`와 NaN/`−∞`의 구분을 주장하지 않는다.

## 실행 결과

구현 branch의 기준 commit은 PR #641 merge commit `a33b245983fae050b66efb9ff259b3fe1132a051`이다.

| 명령/검사 | 결과 |
| --- | --- |
| `cargo test --locked -p spinon-layout grid_multispan_chrome_oracle_tests` | 2개 통과 |
| `cargo fmt --all -- --check` | 통과 |
| `cargo clippy --locked -p spinon-layout --all-targets -- -D warnings` | 통과 |
| `derive_expected.py` 출력과 저장된 `expected.json` 비교 | 정확히 일치 |
| `git diff --check` | 통과 |

같은 start/end line의 잘못된 배치 control은 `TrackSizing` mode가 `normalize_grid_item_placements`를 건너뛰므로 `GridPlacementMode::Existing`에서 실행한다. 예상 결과는 `LayoutError::InvalidGridPlacement`이며 성공한 `LayoutOutput`은 없어야 한다.

구현/runtime 리뷰에서 boundary gutter의 `is_collapsed` flag 검사가 빠진 점을 발견했다. 구현은 모든 `2N+1` 항목의 kind/collapse flag를 확인하도록 보완했고 targeted regression이 통과했다. [구현 리뷰](../c11-4-span-stage-oracle-implementation-review-20-2026-10-26.md)와 [수정 확인 addendum](../c11-4-span-stage-oracle-implementation-review-addendum-2026-10-26.md)을 참고한다.

## 제한

이 변경은 Rust layout trace test다. CSS cascade, app/browser 실행, Android/iOS simulator, pixel 비교, hardware GPU, WPT 또는 전체 C11.4 조합은 실행하지 않는다. 기존 Chrome 155 화면은 최종 geometry의 참고 baseline이며 새 중간 trace 캡처가 아니다. C11.4 Stage B와 공식 `72/213`은 미완료로 유지한다.
