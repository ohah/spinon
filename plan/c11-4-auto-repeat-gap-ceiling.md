# C11.4 · positive-gap auto-repeat track ceiling 계획

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [단일 track 상한 검사](./c11-4-auto-repeat-track-ceiling.md) · [두-track pattern named-line 경계](./c11-4-auto-repeat-pattern-ceiling.md) · [C11 Grid 내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

## 목표

기존 10,000-track 경계 trace는 0 gap만 사용한다. 이 후속은 `1px` positive gap이 두-track auto-repeat의 repetition count, 상한 절단, 마지막 repeated named-line frame에 반영되는지 Columns와 Rows에서 고정한다.

이는 고정 vendored Taffy `0.15.0`과 `spinon-layout`의 내부 `GRID_MAX_TRACKS = 10_000` 정책을 검사하는 test-only slice다. Explicit auto-repeat clamp는 Taffy 내부 경계로 다루며 CSS 일반 지원이나 C11.4/C11 완료를 주장하지 않는다.

## 기준과 독립 기대값

- CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.2의 definite-size auto-repeat 계산을 사용한다. 반복 단위는 `[1px, 2px]`, prefix는 `2px`, suffix는 `3px`, gap은 `1px`이다. 모든 track 크기는 auto-repeat 계산의 1px UA floor 이상이다.
- 각 완전 입력의 `R`회 반복에는 `2 + 2R`개 track과 그 사이 `1 + 2R`개 gutter가 있다. 사용 공간은 `2 + 3R + 3 + (1 + 2R) = 6 + 5R` px다.
- definite content-axis basis `25,001px`은 `R = 4,999`회의 정확한 fit이다. 명시 track은 prefix 1개, 반복 9,998개, suffix 1개로 총 10,000개다.
- basis `25,006px`은 `R = 5,000`회를 요청한다. track 상한 적용 뒤에는 prefix 1개와 반복 pattern 9,999개가 남는다. 반복 pattern의 4,999개 완전한 `[1px, 2px]` 쌍 다음에 `1px` track만 남고 suffix는 잘린다.
- target axis의 마지막 완전 `pattern-start -1 / pattern-middle -1` child frame은 exact-fit에서 offset `24,993px`, size `1px`다. cap case에서는 잘린 마지막 `1px` track에 놓이며 offset `24,998px`, size `1px`다. Offset은 선행 track 크기와 `1px` gutter를 모두 포함한다.
- 상한 정책과 source-order clamp는 [Taffy `explicit_grid.rs`](../vendor/taffy/src/compute/grid/explicit_grid.rs)에 있고 입력 제한은 [`GRID_MAX_TRACKS`](../crates/spinon-layout/src/grid.rs)이다. 이 내부 cap을 CSS Grid의 일반 UA limit이나 implicit-grid §5.4의 동작으로 확대하지 않는다.

## 구현·검증 경계

1. 같은 checkout에서 최신 `main` 위에 작업하며 별도 worktree를 만들지 않는다.
2. Rust test-only `LayoutInput`에 2px prefix, `[pattern-start] 1px [pattern-middle] 2px [pattern-end]` auto-repeat, 3px suffix, target axis 1px gap을 넣는다. 다른 축에는 1px 고정 track 하나와 0px gap을 둔다.
3. 두 basis(25,001px exact-fit, 25,006px cap) × 두 축으로 총 네 실행을 고정한다. 각 실행에서 resolved explicit count, 마지막 target-axis sizing pass의 단일 `Initialize` stage, gutter 제외 track 수와 전체 track-size 순서를 확인한다.
4. `pattern-start -1 / pattern-middle -1`에 놓인 1px child의 target-axis frame을 확인해 named-line occurrence와 gutter 누적을 검증한다. 반복 pattern에는 세 line-name group을 넣고 outer line-name vector에는 길이 네 개의 빈 group을 둔다.
5. 최대 10,000-track 축은 한 번에 하나만 실행하고 전체 captured trace state가 100,000 미만인지 검사한다. 이 계산 trace에는 앱 surface가 없으므로 Android/iOS screenshot, PixelCopy 또는 Metal drawable 검증은 적용하지 않는다.
6. 기존 ceiling 및 `spinon-layout` Grid 테스트, 해당 Clippy와 rustfmt를 실행한다. 구현/runtime 실패 관점 20개와 최종 PR 변경 실패 관점 30개는 각각 별도 기록한다.
7. 결과와 범위를 C11.4 계획·공식 상태·공개 roadmap에 기록하되 C11.4/C11 체크와 공식 `72/213`은 유지한다.

## 제외 범위

- zero gap이 아닌 임의 gutter 크기 행렬, 퍼센트·calc gap, 음수/비유한 gap.
- `auto-fit` collapse, 3개 이상 repeated track, 다른 line-name 구성, implicit-grid 배치, item intrinsic contribution.
- Taffy 상한 또는 clamp 정책 변경, CSS admission, 앱 FFI route, 전체 WPT 실행·적합성.
- Android 실기기·hardware GPU, iOS compositor 이후 pixels, C11.4 또는 C11 완료 주장.

## 완료 기준

- 네 실행에서 track 수·반복 pattern·suffix 보존/절단이 위 기대값과 일치한다.
- 네 실행에서 named-line child frame이 각각 `24,993px` 또는 `24,998px` offset과 1px 크기를 낸다.
- trace budget이 100,000 미만이며 기존 관련 Grid 테스트·Clippy·rustfmt가 통과한다.
- 구현/runtime 실패 관점 20개와 PR diff 실패 관점 30개를 별도로 남긴다.
