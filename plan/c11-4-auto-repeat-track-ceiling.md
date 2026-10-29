# C11.4 auto-repeat 10,000 track 상한 경계

## 목표

고정된 Taffy 0.15.0 의존성에서 auto-repeat이 확장되는 track 수를 직접 관찰하는 `spinon-layout` 회귀 행렬을 추가한다. Columns와 Rows에서 각각 9,999·10,000·10,001 요청 크기를 확인하고, `auto-fill`과 빈 `auto-fit`의 collapse 차이를 구분한다.

범위가 정해진 test slice다. 이 결과만으로 C11.4나 C11을 완료 처리하거나 공개 CSS 지원을 추가하지 않는다.

## 기준 구현과 판정값

- Spinon authored-grid 상한은 [`crates/spinon-layout/src/grid.rs`](../crates/spinon-layout/src/grid.rs)의 `GRID_MAX_TRACKS = 10_000`이다.
- adapter는 이미 authored track 10,000개 초과를 거부하고 auto-repeat flex-factor 사전 검사에 상한을 적용한다. 기존 테스트는 이 두 검사를 다루지만 auto-repeat이 경계에서 실제로 몇 개의 track을 내는지는 직접 확인하지 않는다.
- 고정 vendored Taffy 0.15.0은 auto-repeat 확장을 남은 slot 수로 제한하고 반복 앞 track을 보존하며 상한에 닿으면 뒤쪽 track을 잘라낸다. 관련 코드는 [`vendor/taffy/src/compute/grid/explicit_grid.rs`](../vendor/taffy/src/compute/grid/explicit_grid.rs)에 있다.
- 판정 oracle은 기존 `TrackSizing` mode에서 캡처한 `Initialize` stage의 gutter 제외 track 상태다. 1px 반복 track, 0 gap, definite 9,999px·10,000px·10,001px content axis에서 기대 track 수는 각각 9,999·10,000·10,000이다. 빈 `auto-fit`도 개수는 같지만 모든 repeated track이 collapse되어야 하며, `auto-fill`은 collapse되지 않아야 한다.
- 별도 source-order control은 2px prefix, 1px auto-repeat, 3px suffix를 쓴다. 10,003px basis에서는 2px prefix + 9,998개 1px repeated track + 3px suffix(총 10,000개)를 기대한다. 10,004px basis에서는 제한 전 10,001개가 필요하므로 상한 결과는 2px prefix와 9,999개 repeated track이며 뒤의 3px suffix는 잘린다.
- 고정 track 하나가 10,000개인 경우 stage trace는 track state 100,000개 예산보다 작다. 최대 축은 한 번에 하나만 실행해 두 축 동시 최대 입력으로 예산을 넘기지 않는다.

## 범위

1. 각 축·크기·repeat mode별로 1px 반복 track 하나, zero gap, definite content basis, grid item이 없는 `LayoutInput`을 만든다.
2. target axis의 마지막 sizing pass에서 `Initialize` stage가 정확히 한 번 있는지 검사한다. resolved explicit track 수와 stage의 gutter 제외 track 수가 기대값과 같아야 한다.
3. 10,003px/10,004px source-order control을 두 축에서 실행해 상한 도달 전에는 suffix가 남고 상한에서는 prefix가 보존되며 suffix만 잘리는지 검사한다.
4. 작업은 Rust unit test에 둔다. 10,000 track 경계는 시각적인 앱 장면이 아니라 algorithm trace assertion이므로 Android/iOS screenshot은 이 test-only slice에 해당하지 않는다.
5. 실제 test 출력, source revision, trace-budget 결과를 `spec/internal/evidence/c11-4-auto-repeat-track-ceiling-2026-10-30/`에 보존하고 C11.4 계획과 상태 대장에서 연결한다.

## 범위 밖

- 10,000 상한이나 Taffy의 clamp 정책 변경.
- 제품 로직, fixture ABI, 앱 장면 또는 CSS admission 추가.
- repeated pattern 복수 track, explicit placement/line resolution, named-line 확장, gutter, intrinsic size, percentage basis, implicit track과의 상호작용.
- 이 고정 행렬을 넘어 10,000 track layout의 보편적인 성공을 보장하는 주장. Taffy의 100,000 captured-track-state 예산과 내부 allocation 동작은 별도 제한이다.
- C11.4, C11.3, C11 또는 WPT 적합성 완료 표시.

## 완료 기준

- 12개 matrix 입력(3개 크기 × 2개 축 × 2개 repeat mode)이 정확한 track 수와 collapse 상태를 낸다.
- 네 개 prefix/suffix control(2개 크기 × 2개 축)이 상한에서 source order를 확인한다.
- 관련 `spinon-layout` Grid 테스트 선택, `cargo fmt --all -- --check`, `git diff --check`가 통과한다. 무관한 기존 실패가 있으면 별도로 기록한다.
- 구현/runtime 동작을 서로 다른 실패 관점 20개로 검토하고, 최종 PR diff를 서로 다른 실패 관점 30개로 검토한다.
- 공식 C11.4와 C11 체크는 미완료로 유지한다.
