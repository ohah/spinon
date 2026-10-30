# C11.4 · auto-repeat 다중 track 패턴 상한과 named line 경계 계획

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [단일 track 상한 검사](../spec/internal/evidence/c11-4-auto-repeat-track-ceiling-2026-10-30/README.md) · [C11 Grid 내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

## 목표

기존 auto-repeat 10,000-track 경계 검사는 한 track짜리 반복 패턴을 사용한다. 이 후속은 `[1px, 2px]` 두 track 패턴이 정확히 10,000개로 맞는 경우와 상한을 넘겨 마지막 반복 패턴의 첫 track에서 잘리는 경우를 Columns와 Rows에서 고정한다. 반복 경계의 named line이 실제 남은 마지막 track에 붙어 있는지도 item frame으로 확인한다.

이는 vendored Taffy `0.15.0`과 `spinon-layout`의 내부 `GRID_MAX_TRACKS = 10_000` 정책을 위한 테스트 전용 slice다. W3C §5.4의 UA 정의 한도는 implicit grid를 다루며, 여기서 검사하는 explicit auto-repeat 절단의 oracle은 아니다. 이 값과 구체적인 clamp 결과는 내부 Taffy 경계로만 취급한다. 공개 CSS 지원, 전체 WPT 적합성, C11.4 또는 C11 완료를 주장하지 않는다.

## 기준 모델과 독립 기대값

- CSS Grid 2025 CRD §7.2.3.2에서 `auto-fill`은 definite 크기와 gap을 고려해 넘치지 않는 최대 반복 횟수를 고른다. §5.4의 UA 정의 한도는 implicit grid에 관한 별도 규칙이며, 이 계획은 그 규칙을 explicit track cap의 근거로 사용하지 않는다. [auto-fill 반복](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat) · [큰 grid 제한](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#limiting-large-grids)
- CSS Grid §8.3에서 named line의 음수 occurrence는 explicit grid의 끝에서 역방향으로 센다. [line-based placement](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement)
- 현재 내부 상한은 [`GRID_MAX_TRACKS`](../crates/spinon-layout/src/grid.rs)이고 확장·source-order 절단은 [Taffy `explicit_grid.rs`](../vendor/taffy/src/compute/grid/explicit_grid.rs)에 있다. 상한 정책은 Taffy 고정 버전의 구현 경계로 취급하고 CSS 일반 규칙으로 확대하지 않는다.
- 입력 track list는 앞의 `2px` track, `[pattern-start] 1px [pattern-middle] 2px [pattern-end]` auto-repeat, 뒤의 `3px` track이다. gap은 0이다. 반복 한 번의 크기는 3px이며 non-repeat track 합은 5px다.
- Rust `GridAutoRepeat.line_names`에는 repeat pattern의 세 line-name group을 넣고, 해당 축의 outer line-name vector에는 빈 group 네 개를 둔다. 네 개는 `tracks.len() - repeated_track_count + 2`이며, 반복 track 사이 이름을 outer line 이름으로 중복 기록하지 않는다.
- definite content-axis 크기 `15,002px`에서는 `(15,002 - 5) / 3 = 4,999`회 반복된다. 완전한 9,998 repeated tracks와 앞·뒤 track을 합쳐 explicit track 10,000개가 되고 suffix `3px`가 남는다.
- `15,005px`에서는 `(15,005 - 5) / 3 = 5,000`회 반복을 요청한다. 상한을 적용하면 앞의 track 1개 뒤에 repeated pattern 9,999개가 남는다. 이는 4,999개의 온전한 `[1px, 2px]` 쌍 뒤에 마지막 `1px` 하나만 남기는 결과이며 suffix는 잘린다.
- `pattern-start -1 / pattern-middle -1` 배치는 각각 마지막 완전 반복의 첫 track과 상한에서 잘린 마지막 `1px` track을 가리켜야 한다. 예상 target-axis frame은 각각 offset `14,996px`, size `1px`와 offset `14,999px`, size `1px`이다.

계산, 양 축 scene과 독립 기대값은 [precomparison 기록](../spec/internal/evidence/c11-4-auto-repeat-pattern-ceiling-precomparison-2026-10-30/README.md)에 고정한다.

## 구현과 검증 경계

1. 같은 checkout에서 최신 `main` 위에 구현한다. 별도 worktree를 만들지 않는다.
2. Rust 테스트 전용 입력에서 Columns와 Rows를 각각 실행한다. 한 축만 10,000개 track으로 채우고 다른 축은 `1px` 고정 track 하나를 쓴다.
3. 두 장면(정확히 맞는 `15,002px`, 상한을 넘는 `15,005px`) × 두 축, 총 네 실행을 고정한다. 각 결과에서 resolved explicit count, 마지막 sizing pass의 단일 `Initialize` stage, non-gutter track 수와 전체 track-size 순서를 비교한다.
4. 각 실행에 `pattern-start -1 / pattern-middle -1`로 놓인 `1px` child를 포함해 repeated line occurrence가 올바른 마지막 track에 연결되는지 frame으로 검사한다. 반복 내부 세 group과 별도로 outer line-name vector의 네 빈 group을 입력하고, exact-fit에서 suffix가 남는지, cap case에서 suffix가 잘리는지도 마지막 track size로 검증한다.
5. 최대 축 한 번에 하나만 실행하고 기존 100,000개 captured-track-state 예산을 넘지 않는지 확인한다. 앱 장면·FFI route·PixelCopy·iOS drawable이 없는 알고리즘 trace 테스트이므로 Android/iOS simulator와 screenshot은 사용하지 않는다.
6. 구현/runtime 실패 관점 20개와 최종 PR 변경 실패 관점 30개를 별도 기록한다. 구현에서 실제 결함이 발견되면 원인을 고치고 이 계획의 기대값 및 제외 범위를 다시 검토한다.
7. 공식 C11.4/C11 체크와 `72/213` 집계는 유지한다. 결과를 C11.4 상위 계획과 test evidence에만 기록한다.

## 제외 범위

auto-fit collapse, gap이 있는 auto-repeat, 세 개 이상 track 패턴, 임의 line-name 병합, implicit-grid 배치, numeric/negative track line 일반 지원, Taffy 상한 변경, 일반 author CSS admission, 앱 runtime, Android 실기기·hardware GPU, iOS compositor 이후 픽셀, WPT 전체 실행, C11.4/C11 완료는 포함하지 않는다.

## 완료 기준

- 네 실행 모두 explicit count와 `Initialize` track 수가 10,000이며 예상 track sequence와 suffix 보존/절단이 정확하다.
- named-line 배치가 네 실행의 axis frame에서 독립 기대값과 맞는다.
- 기존 C11.4 auto-repeat ceiling test와 `spinon-layout` Grid 테스트, 해당 Clippy와 rustfmt가 통과한다.
- 구현/runtime 검토 20개와 PR diff 검토 30개를 별도로 남긴다.
