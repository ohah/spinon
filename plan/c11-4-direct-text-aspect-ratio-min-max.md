# C11.4 direct-text preferred aspect-ratio min/max 계획

**상태:** 구현, 최종 로컬 검증, 구현/runtime 실패 관점 검토 20개와 구현 PR diff 실패 관점 검토 30개를 완료했다. 구현 PR [#850](https://github.com/ohah/spinon/pull/850)은 2026-10-30T10:31:12Z Rebase and merge됐고 merge SHA는 `4959332936a6ad599e1355d18161b90a9fea05bf`다. PR check rollup은 비어 있고 main required status checks가 설정되어 있지 않아 CI 성공으로 기록하지 않는다. 계획·사전 비교 PR [#849](https://github.com/ohah/spinon/pull/849)도 Rebase and merge로 병합했다. 구현은 기존 C11.4 opt-in fixture에 한정하며 C11/C11.4 공식 상태를 완료로 바꾸지 않는다.

**상위:** [C11.4 Grid track sizing](c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태 대장](../spec/STATUS.md)

## 문제와 목표

[C11.4 direct-text aspect-ratio profile](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text/generic_direct_text.rs)은 숫자 ratio와 `auto <ratio>`의 제한 사례를 처리하지만 item `min-width`, `max-width`, `min-height`, `max-height`가 초기값이 아니면 거부한다. 반면 CSS Grid §6.6의 non-replaced item `content size suggestion`은 해당 축의 min-content size를 사용하고, preferred aspect ratio가 있으면 반대 축의 definite min/max를 ratio로 환산해 제한한다. 이는 replaced item 전용 `transferred size suggestion`과 다른 규칙이다.

목표는 고정 Ahem direct-text profile에서 **AAAA 텍스트 Grid item 하나의 preferred aspect ratio와 px min/max 제약이 intrinsic contribution에 미치는 영향**을 구현하고, 다른 `RuntimeBlockFlexGridTrackSizingV1` 사용자나 제품 CSS admission에는 노출하지 않는 것이다. 구현 경계는 [고정 C11.4 fixture 경로](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text/)와 vendored Taffy Grid intrinsic contribution이다.

기준 문서는 [CSS Grid Layout Level 2 §6.6](https://drafts.csswg.org/css-grid/#min-size-auto) 및 ratio basis를 설명하는 [CSS Sizing Level 4 §4](https://drafts.csswg.org/css-sizing-4/#aspect-ratio)다. 두 문서는 작업 시점의 공개 초안이다. 명세 해석은 독립 계산과 아래에 고정한 Chrome 156 관측을 함께 대조하며, 결과를 WPT 적합성이나 전체 CSS 지원으로 확대하지 않는다.

## 사전 비교 입력과 관측

고정 입력·Chrome 관측·화면·hash는 [사전 비교 evidence](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-min-max-precomparison-2026-10-30/README.md)에 있다. Canonical HTML fixture와 embedded-font 사본은 각각 Agent Browser HeadlessChrome 156에서 실행했고 17개 case 결과가 정확히 같았다. 양쪽에서 Ahem 10px font readiness가 true였다. 구현 review에서 0..=60px의 중간값도 연속 지원해야 함을 확인해 별도 [추가 Chrome 관측](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-min-max-precomparison-2026-10-30/README.md#연속-px-범위-추가-oracle) 두 개를 더했다. `min-width:45px`는 columns `[45,10]px`, row `22.5px`, item `45×22.5px`이고 `max-height:17.75px; min-width:0; min-height:0`는 columns `[35.5,10]px`, row `17.75px`, item `35.5×17.75px`다.

- Root: HTML Grid, width `100px`, height `auto`, columns `max-content max-content`, rows `auto`, gap `0`, normal alignment.
- Item: 정확히 두 개의 HTML block child이며 direct text는 `AAAA`와 `A`다. Ahem `10px/10px`, `horizontal-tb`, LTR, wrapping 기본 조건을 유지한다.
- Ratio owner: `AAAA` 하나에 `2 / 1` 또는 `auto 2 / 1`. 나머지 item은 `aspect-ratio:auto`다.
- Edge controls: zero-edge 기본, 그리고 `box-sizing:border-box; padding:2px 4px; border:1px solid`인 numeric ratio 및 auto-ratio 대조.
- Constraints: min/max 각각의 가로·세로 방향, min/max 동시 조건, no-ratio control, numeric border-box와 auto-ratio content-box control을 포함한다.

독립 계산과 브라우저 결과를 혼동하지 않는다. 표의 Chrome 값은 고정 oracle 값이며 implementation acceptance는 동일 입력의 전체 columns, rows, root/item frame, typed ratio basis와 intrinsic contribution trace를 검사한다.

| 고정 장면 | Chrome 156 columns | rows | ratio item | 판별점 |
|---|---:|---:|---:|---|
| `numeric-baseline` | `40px 10px` | `20px` | `40×20` | ratio 기준 |
| `min-width-60` | `60px 10px` | `30px` | `60×30` | inline min이 ratio를 통해 block 기여에 반영 |
| `max-width-30-min-width-0` | `30px 10px` | `15px` | `30×15` | inline max 전달; min-content floor를 명시적으로 해제 |
| `max-height-15-min-height-0` | `40px 10px` | `15px` | `40×15` | 반대 축 max와 기본 auto minimum 구별 |
| `max-height-15-min-height-0-min-width-0` | `30px 10px` | `15px` | `30×15` | `min-width:0`이 min-content floor를 없앰 |
| `min-width-60-clamped-by-max-height-20` | `60px 10px` | `20px` | `60×20` | 반대 축 최대값이 환산된 최소 기여를 제한 |
| `min-height-20-clamps-transferred-max-width-30` | `30px 10px` | `20px` | `30×20` | 반대 축 최소값이 환산된 최대 기여를 제한 |
| `source-min-width-60-over-max-width-40` | `60px 10px` | `30px` | `60×30` | 동일 축의 min이 max보다 큰 CSS constraint 순서 |
| `auto-content-box-min-width-60` | `60px 10px` | `31px` | `60×31` | content-box ratio와 10px/6px edge 반영 |
| `auto-content-box-max-height-15-min-controls` | `28px 10px` | `15px` | `28×15` | content-box max transfer 후 edge 복원 |

나머지 7 control과 전 case exact expected values는 [`expected-values.json`](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-min-max-precomparison-2026-10-30/expected-values.json)에 있다. 특히 `max-height-15-min-height-0`은 default `min-width:auto`가 min-content width `40px`을 지키므로 width `40px`을 낸다. `min-width:0`을 추가한 control은 ratio 환산 max width `30px`을 드러낸다. 이 둘은 하나의 기대값으로 합치지 않는다.

## 제한 입력 계약

- Route는 기존 `RuntimeBlockFlexGridTrackSizingV1` 및 `c11-4-grid-track-sizing-fixture-test`의 C11.4 direct-text aspect-ratio collector에서만 허용한다.
- Root는 위 사전 fixture의 단일 in-flow Grid다. computed track size가 `max-content max-content`/`auto`이고, 다른 정렬·gap·root edge·explicit placement·implicit track·nested Grid는 거부한다.
- 정확히 두 item은 direct text `AAAA`와 `A`를 각각 하나만 소유한다. 다른 문자·공백·multiline·extra/nested/replaced item은 거부한다. Font 및 writing mode는 사전 fixture와 동일해야 한다.
- Ratio는 유일한 `AAAA` owner에만 둘 수 있다. Min/max가 모두 초기값이면 기존 finite-positive ratio subset을 유지한다. Min/max가 하나라도 지정된 ratio positive는 숫자 `2 / 1` 또는 `auto 2 / 1`로 제한하며, no-ratio control은 computed value가 정확히 `auto`여야 한다. `A` item ratio, ratio owner 중복, serialization과 typed value 불일치는 fail-closed다.
- `width`와 `height`는 계속 `auto`, margin은 0이다. Item min/max는 `auto`/`none` 또는 유한한 `0..=60px`만 허용한다. 이 slice에서 oracle에 없는 percentage, `calc()`, intrinsic keyword, 음수, 60px 초과와 비유한 값은 거부한다. Root min/max는 초기값이어야 한다.
- zero edge는 기존 profile을 유지한다. nonzero edge는 사전 비교의 padding과 border 값만 허용한다. 숫자 ratio는 `box-sizing`으로 고른 sizing box를, `auto <ratio>`는 content box를 유지한다. 변환 전후 padding/border를 중복 가산하거나 누락하면 거부한다.
- 구현은 non-replaced Grid §6.6 `content size suggestion` 경로를 명시적으로 다룬다. `transferred size suggestion`을 non-replaced item에 잘못 적용하지 않는다. Replaced-image contribution 경로와 다른 runtime profile은 결과가 바뀌지 않아야 한다.
- Adapter가 허용하는 min/max typed value는 scene ID나 authored CSS 문자열에서 추론하지 않는다. computed value snapshot의 node ownership, ratio, basis, constraints와 revision을 함께 검사한다.

## 구현 및 수용 기준

1. Vendored Taffy 0.15.0 Grid 코드를 대조한다. 현재 `non_replaced_ratio_content_contribution`은 min/max가 전부 auto일 때만 ratio-derived intrinsic contribution을 쓰고, `transferred_size_suggestion`은 replaced item 전용이다. 구현은 replaced-only 분기를 유지하면서 non-replaced content-size suggestion의 min-content, 반대 축 min/max ratio conversion, affected-axis max clamp 및 box-sizing edge를 별도 처리한다. 기존 `minimum_contribution`/min-content/max-content 경로의 실제 trace를 확인해 어느 경로가 관측된 track을 결정하는지 증명한다.
2. Direct-text collector와 `paragraph_style_for_aspect_ratio`에서 지원 subset의 px min/max를 typed value로 허용한다. 미지원 값은 paragraph/layout 시작 전에 오류로 끝나야 한다. Style-to-layout projection은 기존 정확한 ratio owner NodeId 및 ratio sizing-box contract를 재사용한다.
3. 17개 사전 비교 장면과 연속 px 범위 추가 oracle 2개를 고정 C11.4 runtime matrix에 둔다. 19개 각각의 resolved track list, min-content/ratio-adjusted suggestion, row/column contribution stage, item/root frame과 overflow를 같은 style revision으로 기록한다. 전체 19개 expected값은 고정 Chrome 관측과 정확히 일치해야 한다. Ahem/embedded source의 기존 17개 oracle 동등성도 유지한다.
4. Negative controls: row/column axis swap, ratio 역수 오류, min/max 적용 순서 역전, max가 min을 잘못 덮음, `min-width:auto`를 0으로 처리, min-width 0 control 누락, content-box edge 중복/누락, numeric ratio를 content-box 처리, `auto <ratio>`를 border-box 처리, no-ratio도 전이, wrong/multiple ratio owner, stale style revision, unsupported dimension/percentage/calc/negative/non-finite input, unsupported root track, extra item, wrong Ahem metrics를 각각 거부하거나 반례 출력으로 검출한다.
5. 기존 23개 direct-text aspect-ratio/runtime 회귀와 신규 19개를 합친 42개를 Android API 37 ARM64 AVD/SwiftShader 및 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 실행한다. Scene geometry와 표면 표본을 oracle와 비교한다. 최종 dirty source 기준 두 플랫폼 모두 42/42 scene과 129/129 surface sample이 exact다. AVD는 software renderer, iOS 표본은 compositor submission 전 Metal drawable이라는 한계를 보고하며 실기기/hardware GPU 증거로 확대하지 않는다.
6. Focused Rust tests, `spinon-layout` 및 runtime 관련 gates, `cargo fmt --all -- --check`, default/fixture-test Clippy와 default/positive-control artifact boundary를 기록한다. Fixture-only FFI, string, matrix payload는 product/default artifact에서 빠져야 한다. 실행하지 않은 CI는 통과로 보고하지 않는다.
7. 내부 계약 0059의 제한 direct-text fixture 범위와 오류 경계를 갱신하고 숫자 버전 `0.1.0`을 유지한다. 공식 STATUS에서 C11/C11.4 checkbox와 `72/213`은 그대로 둔다. 계획 PR 병합 뒤 구현 PR도 별도 branch/PR로 진행한다.

## 검증 순서와 한계

- 이 계획 자체는 구현 전에 별도의 20개 실패 관점으로 검토한다. 계획 변경 PR은 프로젝트 규칙에 따라 계획 검토와 별개로 실제 diff의 30개 관점을 검토한다.
- 구현 뒤에는 새 코드와 runtime 동작에 대한 20개 관점을 새로 검토하고, 구현 PR diff도 별도 30개 관점으로 검토한다. 이전 검토를 재사용하지 않는다.
- PR은 rebase merge만 사용한다. 계획 PR에는 `planning` 및 `area: layout`, 구현 PR에는 기존 `enhancement` 및 `area: layout` 라벨을 사용한다.
- Scope는 Ahem direct-text 제한 fixture다. Public CSS admission, 일반 paragraph, arbitrary font/script, replaced/natural ratio, writing-mode, percentage/calc, arbitrary Grid tracks, full WPT conformance, C11.4 전체 및 C11 전체 완료를 포함하지 않는다.
