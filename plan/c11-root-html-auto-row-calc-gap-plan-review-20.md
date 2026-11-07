# C11.4 auto-row `calc()` `row-gap` 계획 실패 관점 검토 20개

대상 계획: [C11 Root HTML auto-row alignment `calc()` `row-gap`](./c11-root-html-auto-row-calc-gap.md) · 기준 입력/관측: [사전 비교](../spec/internal/evidence/c11-root-html-auto-row-calc-gap-precomparison-2026-11-07/README.md)

이 검토는 계획 경계, oracle, 제안된 오류 조건과 검증 관문을 별개의 실패 경로에서 확인한다. 구현 코드를 검토한 것으로 세지 않는다.

| # | 적대적 실패 관점 | 계획상 방어 및 구현 검증 |
|---:|---|---|
| 1 | 계획이 arbitrary CSS math를 암묵적으로 허용 | two-term, one-length/one-percentage grammar만 허용하고 nested/function AST는 거부한다. |
| 2 | 항 순서를 고정해 같은 합계의 유효 식을 다르게 처리 | percentage-first와 length-first 두 입력을 oracle matrix에 포함한다. |
| 3 | 단항 부호를 통한 미검증 계산 | 한 항에만 한 번 적용되는 negation으로 범위를 제한하고 40px 결과를 range-check한다. |
| 4 | row percentage basis에 width 또는 border-box를 잘못 사용 | border/padding이 없는 고정 40px content box의 block-size를 basis로 삼는다. |
| 5 | definite basis를 layout 이후 해석해 순환 의존 발생 | 고정 height만 허용하고 auto/min/max/aspect-ratio/cyclic basis는 제외·거부한다. |
| 6 | two-item Grid에서 row/gutter 수를 3-item 기대값으로 계산 | 2개와 3개 child를 각각 10개 alignment와 교차한 120개 장면을 고정한다. |
| 7 | gutter 수를 item 수와 같게 계산 | 독립 식은 row 수 − 1개의 gutter만 더하고 실제 track/item geometry를 비교한다. |
| 8 | CSS computed math의 node ownership를 무시 | `LayoutCssMathValue` node id와 대상 Grid node가 같은지 확인한다. |
| 9 | row-gap math id에 다른 CSS property를 붙임 | `RowGap` 바인딩을 요구하고 ColumnGap/track property 교차 입력을 거부한다. |
| 10 | 누락·중복·잘못된 id가 첫 항목이나 이전 입력으로 fallback | ID index와 exact binding 검사를 실패 주입하며 silent fallback을 허용하지 않는다. |
| 11 | 음수 계산 결과를 negative gutter로 유지 | CSS non-negative range clamp를 0px로 적용하고 `calc(1px - 10%)` 관측 control로 확인한다. |
| 12 | NaN/infinity가 clamp를 통과하거나 실패를 숨김 | 모든 항과 최종 결과·총 extent의 finite 조건을 단계별 검사한다. |
| 13 | 매우 큰 유한 gap이 f32/Taffy 범위를 넘김 | gutter 2개와 base rows를 합한 실제 extent를 검사하고 f32 표현 경계 테스트를 둔다. |
| 14 | percentage를 두 번 적용하거나 이미 px로 바뀐 값을 다시 basis에 곱함 | typed math는 한 번만 resolve하고 브라우저의 모든 track/item 좌표와 대조한다. |
| 15 | geometry preflight와 Taffy가 gap을 각각 한 번씩 더함 | 기존 Taffy sizing을 단일 used gutter 원천으로 유지하고 수동 보정을 금지한다. |
| 16 | `normal`/`stretch`가 fixed row까지 늘어남 | explicit 10px row 고정, auto implicit rows만 양의 free space에서 성장하는 expected를 둔다. |
| 17 | negative free space에서 distributed alignment가 남은 공간을 나눔 | end/center positional overflow와 distributed fallback을 양/음 free-space matrix로 구분한다. |
| 18 | Chrome을 expected oracle로 복사해 같은 버그를 비교 | Python 기대식은 `expectedUsedGap`/브라우저 기하를 읽지 않고 40×percentage+px로 계산한다. |
| 19 | subpixel serialization 오차로 정상 장면을 거부하거나 큰 차이를 놓침 | coordinate별 1/32px tolerance와 관측 최대 0.020834px를 사전에 기록한다. |
| 20 | 이 bounded profile이 다른 C11/Grid 입력을 열거나 실패 시 이전 scene을 훼손 | 기존 item/track/alignment allowlist 유지, default RuntimeSession atomic reject/recovery gate와 제외 범위를 명시한다. |

## 결과

계획은 위 20개 관점에서 구현 진입 가능하다. 사전 비교는 120/120 scene, 1,500/1,500 coordinate이며 최대 오차 0.0208333334 CSS px다. 검토에서 implementation/runtime review로 넘긴 조건은 math id ownership, unsupported AST, representability, two/three-item distinction, negative-space alignment, 실패 뒤 scene 보존이다. 구현·PR 상태는 이 문서가 증명하지 않는다.
