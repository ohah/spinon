# C11.4 · 자동 반복 `minmax(inflexible, 0px)` 경계

**상태:** 계획 및 Chrome 사전 비교만 완료했다. 구현, Spinon runtime, Android/iOS Simulator는 아직 확인하지 않았다. C11.4와 상위 C11 체크박스는 미완료로 두며 공식 완료 집계 `72/213`을 변경하지 않는다.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [직전 fixed-maximum slice](./c11-4-auto-repeat-inflexible-fixed-maximum.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

## 목표와 이미 확인된 코드

직전 제한 slice는 `minmax(auto|min-content|max-content, 40px | definite percentage | supported calc)`를 다뤘고 최대 `0px`은 장면에서 제외했다. 이번 범위는 이 0px 경계를 열·행, `auto-fill`·`auto-fit`, 0px·4px gap으로 고정 비교해 기존 입력·layout 경로가 CSS Grid 결과를 보존하는지 확인하는 것이다.

현재 `crates/spinon-layout/src/grid.rs`의 fixed-maximum preflight는 유한한 음이 아닌 `LengthPx`를 허용한다. Vendored Taffy `compute_explicit_grid_size_in_axis()`는 automatic repeat 횟수에만 1px 바닥값을 적용하고 원래 track sizing function은 유지한다. 따라서 이 slice의 첫 목적은 새 문법이나 일반 CSS admission 추가가 아니라, `minmax(inflexible, 0px)`을 실제 layout/runtime 경로로 실행해 반복 수·track sizing·`auto-fit` gutter collapse를 검증하는 것이다. 테스트에서 차이가 발견될 때만 그 경계를 최소 수정한다.

## 규범·oracle·재현

CSS Grid Level 1 §7.2.3.2는 automatic repeat 산정 시 0으로 나누는 상황을 피하도록 track size를 UA 지정 최소값으로 바닥 처리하며, 1px을 제안한다. 고정된 oracle은 [24-scene Chrome 156 사전 비교](../spec/internal/evidence/c11-4-auto-repeat-zero-fixed-maximum-precomparison-2026-10-29/README.md)와 [HTML fixture](../tests/fixtures/css/c11/grid-auto-repeat-zero-fixed-maximum-c114-precomparison.html)다.

장면은 반복 axis `columns|rows` × minimum `auto|min-content|max-content` × `auto-fill|auto-fit` × gap `0|4px` 조합이다. definite content axis는 16px이고 반복 pattern은 `minmax(<minimum>, 0px)` 한 track이다. 12×12px item 하나를 세 번째 repeated track에 배치한다. 독립 산정값은 가장 큰 양의 정수 `n` satisfying `n × 1px + (n − 1) × gap ≤ 16px`이며, gap 0px은 16개, gap 4px은 4개다.

Chrome oracle에서 기대하는 추가 결과는 다음과 같다.

- 사용 track list에서 세 번째 track은 item의 12px 최소 크기를 보존하고 나머지 repeated track은 0px이다.
- `auto-fill`은 gap 4px일 때 세 번째 item을 root 시작점에서 8px에 둔다.
- `auto-fit`은 item 앞의 두 빈 track과 연결 gutter를 접어 item을 시작점 0px에 둔다.
- gap 0px에서는 두 repeat mode의 item offset이 모두 0px이다.
- row/column 결과는 각각 수평/수직축에서 대칭이다.

## 구현·runtime 계획

1. Chrome JSON을 fixed runtime scene 목록의 source로 연결하고 Rust 검증기에서 정확한 24 scene ID, 입력, expanded repeated count, used track sizes, item/root frame 및 `auto-fit` collapse offset을 검증한다. 새로운 author CSS admission은 추가하지 않는다.
2. `spinon-layout`의 direct track sizing test와 Taffy explicit-grid test에 zero maximum을 명시해 기존 UA count floor가 `minmax(auto, 0px)` 및 `minmax(min-content|max-content, 0px)`에서도 유지되는지 확인한다. 열·행과 gap 0/4를 덮고, `2px` positive maximum control에서는 gap 0px에 8개, gap 4px에 3개가 되는지 확인해 0px 전용 floor를 구분한다.
3. Opt-in fixture-test FFI adapter에서 root, item, `GridContainerInput`, trace, document/style/environment/layout revision을 교차 확인한다. Unsupported grammar, indefinite axis, non-finite value의 admission을 확대하지 않는다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 scene IDs를 실행한다. layout/draw/present identity를 대조하고 표면 픽셀 표본은 item, root 안의 빈 영역, root 바깥을 구분한다. AVD는 hardware GPU나 실기기로 확대 해석하지 않으며, iOS drawable readback은 compositor 이후 표시 증거가 아니다.
5. 기본 artifact와 fixture-test positive control을 빌드해 새 adapter/payload의 opt-in 경계를 확인한다. 정적 archive 결과를 APK/IPA 전체 제외로 확대하지 않는다.
6. 구현/runtime 실패 관점 20개와 PR diff 실패 관점 20개를 별도로 기록한다. 리뷰 발견을 고치면 영향받은 관점을 다시 검증한다.
7. `spec/internal/0059-c11-grid-layout.md`, C11.4 계획, `spec/STATUS.md`, public roadmap을 확인한 결과와 맞춘다. 이 한 slice로 C11.4나 `72/213`을 승격하지 않는다.

## 종료 조건과 남는 범위

- Chrome oracle 24 scene의 repeat count, used track serialization, item offset이 독립 계산과 일치한다.
- Rust direct/layout projection/fixture FFI 및 양 simulator에서 24 scene 결과가 oracle과 맞거나, 확인된 구현 차이를 수정하고 원인을 기록한다.
- Android PixelCopy와 iOS drawable에서 선택한 고정 RGBA 표본이 같은 scene revision의 위치에 대해 일치한다.
- artifact opt-in 경계, 구현/runtime 검토 20개, PR diff 검토 20개, 병합 후 roadmap 상태를 증거로 남긴다.
- PR은 required checks와 merge state를 재확인한 뒤 Rebase and merge하며, Tailnet roadmap을 갱신하고 Agent Browser를 닫는다.

이 slice는 일반 author CSS admission, 모든 track minimum의 intrinsic semantics, 임의 gap/value, indefinite axis, 전체 auto-repeat 조합, 전체 WPT 및 C11.4 완료를 뜻하지 않는다.
