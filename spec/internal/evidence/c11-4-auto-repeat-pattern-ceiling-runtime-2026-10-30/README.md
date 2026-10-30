# C11.4 auto-repeat two-track pattern ceiling runtime evidence

## 범위

`spinon-layout`의 순수 Rust Grid track sizing 경로에서 두 track auto-repeat pattern과 끝쪽 named-line 배치를 검증한다. 정확히 상한에 맞는 경우와 상한에서 pattern 중간이 잘리는 경우를 Columns/Rows 양축으로 확인한다. 앱 runtime, CSS admission, WPT 전체 또는 C11.4/C11 완료 근거가 아니다.

## 기준과 source

- 계획: [`c11-4-auto-repeat-pattern-ceiling`](../../../../plan/c11-4-auto-repeat-pattern-ceiling.md)
- 사전 비교: [`precomparison`](../c11-4-auto-repeat-pattern-ceiling-precomparison-2026-10-30/README.md)
- 대상 테스트: `crates/spinon-layout/src/grid_track_sizing_tests/auto_repeat_track_ceiling.rs`
- 구현 기준 revision: `945c9781be074231bc4163d4847be4aa78b8fc3f` (`origin/main`), 구현 diff 미커밋

## 초기 실패와 수정

[`checks/initial-review-findings.md`](checks/initial-review-findings.md)에 잘못된 테스트 필터와 첫 입력 검증 실패를 보존했다. 첫 필터는 0개를 선택했으므로 통과로 세지 않았다. 함수 이름 기준 실행은 `InvalidGridTrackList`로 실패했다. 원인은 반복 line-name metadata가 있는 입력에서 outer line-name vector도 `tracks.len() - repeated_track_count + 2` 길이를 가져야 한다는 입력 계약 누락이었다. 네 빈 outer group을 명시한 뒤 동일 테스트가 통과했다.

## 검증 기록

수정 뒤 최종 검증:

- `mise exec -- cargo test --locked -p spinon-layout --lib auto_repeat_track_ceiling`: 3 passed, 0 failed.
- `mise exec -- cargo test --locked -p spinon-layout --lib grid::`: 135 passed, 0 failed.
- `mise exec -- cargo test --locked -p spinon-layout --lib`: 314 passed, 0 failed.
- `mise exec -- cargo clippy --locked -p spinon-layout --all-targets -- -D warnings`: 통과.
- `mise exec -- cargo fmt --all -- --check`: 통과.
- `git diff --check`: 통과.

원본 출력은 [`checks/`](checks/)에 보존했다. 테스트 결과는 Rust trace 및 frame assertion 범위에 한정한다. 이 slice는 앱 화면을 렌더링하지 않아 시뮬레이터 screenshot evidence를 만들지 않는다. 구현/runtime 실패 관점은 [별도 검토 20개](implementation-runtime-review-20.md)에 있다.

## 제한

Taffy의 현재 explicit-grid cap 동작을 고정하는 테스트다. auto-fit, gap, 3개 이상 반복 track, 임의 named-line 병합, implicit-grid 일반 배치, public CSS support, Android/iOS app runtime, 실기기 및 GPU 검증은 포함하지 않는다. 공식 C11.4 체크와 `72/213` 집계는 유지한다.
