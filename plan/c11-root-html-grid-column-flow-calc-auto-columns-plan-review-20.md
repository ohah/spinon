# C11.1 `calc()` implicit columns 계획 실패 관점 20개

검토 대상은 [계획](./c11-root-html-grid-column-flow-calc-auto-columns.md)과 [구현 전 관측](../spec/internal/evidence/c11-root-html-grid-column-flow-calc-auto-columns-precomparison-2026-11-03/README.md)이다. 계획 검토이며 기능 구현 검토나 PR diff 검토와 합산하지 않는다.

| # | 적대적 질문 | 판정 | 근거/필요한 조치 |
|---|---|---|---|
| 1 | `calc()`가 실제 implicit track의 허용 breadth인지 확인했는가? | 통과 | Grid §7.2.1의 `<track-breadth>`는 `[0,∞]` `<length-percentage>`를 포함한다. |
| 2 | 반복 목록이 explicit tracks 다음부터 순환하는 규칙을 보존하는가? | 통과 | §7.6 첫 implicit track부터 pattern을 반복하며 fixture에 네 implicit track을 만든다. |
| 3 | percentage basis에 gap을 잘못 빼지 않는가? | 통과 | 두 calc는 240px content width 기준으로 계산하며 column gap은 start 좌표에만 더한다. |
| 4 | 첫 calc의 독립 산술이 잘못됐는가? | 통과 | `0.10 × 240 + 5 = 29px`다. |
| 5 | 두 번째 calc의 소수값을 정수화하는가? | 통과 | `0.02 × 240 + 2 = 6.8px`; 독립 기대값은 소수 그대로 보존한다. |
| 6 | 두 번 순환하는 list 순서와 track 시작점이 맞는가? | 통과 | expected `[30,40,29,6.8,29,6.8]`, starts `[0,35,80,114,125.8,159.8]`다. |
| 7 | Browser 양자화 오차를 기능 오차로 오판하는가? | 통과 | Chrome `6.796875px`와 exact rational 차이는 `0.003125px`, 허용치는 `0.02px`다. |
| 8 | 음수 calc 결과가 declaration 전체를 무효 처리하는가? | 통과 | Values §10.12는 top-level result를 범위에 clamp하고 Chrome은 negative track을 `0px`로 썼다. |
| 9 | 100% 초과 계산 결과를 container 폭으로 clamp하는가? | 통과 | `calc(120% + 1px)`는 240px basis에서 289px이며 Chrome used tracks가 이를 확인했다. |
| 10 | definite `0px` width를 indefinite로 잘못 분류하는가? | 통과 | `calc(10% + 5px)`는 0px basis에서 5px; Chrome control과 plan expected가 일치한다. |
| 11 | mixed scalar/list에서 calc 이후 direct px/%/fr가 덮이거나 remaining space가 잘못 배분되는가? | 통과 | px/% mixed list는 `[10,29,36,10]px`; `calc(10% + 5px) 1fr`는 `[29,43.5,29,43.5]px`로 고정했다. |
| 12 | 단순 두 항 외 Product/Invert/Min/Max/Clamp tree가 넓은 Calc 허용으로 통과하는가? | 통과 | positive/negative operand sign만 받는 `Sum` 형태를 제한하고 다른 operator 및 arity는 거부한다. |
| 13 | CSS-invalid fallback과 typed-but-unsupported 수식이 같은 경로로 처리되는가? | 통과 | 문법상 invalid declaration fallback과 valid CSS의 out-of-slice AST 거부를 별도 검사하도록 계획에 추가했다. |
| 14 | CSS 수식이 다른 node나 property 소유자로 재사용되는가? | 통과 | calc ID, Grid node, existing `GridTemplateColumns` math binding을 교차 확인한다. |
| 15 | width의 min/max/aspect-ratio/math 변형이 percentage basis를 바꾸는가? | 통과 | 해당 width variant는 제한 profile에서 거부하도록 명시했다. |
| 16 | track list length 또는 implicit count가 10,000 cap을 넘는가? | 통과 | empty/over-limit list 및 explicit+implicit total quota를 거부하는 검증 관문이 있다. |
| 17 | 누적 track/gap extent가 layout 숫자 범위를 넘는가? | 통과 | 기존 `33,554,428 CSS px` math ceiling과 cumulative finite-f32 extent 검사를 따로 확인한다. |
| 18 | shared validator에서 Calc 허용이 다른 profile에 퍼지는가? | 통과 | Root HTML column-flow만 허용하고 Root projection 외부 profile에 대한 positive/negative 경계를 검사한다. |
| 19 | Chrome oracle screenshot을 app pixel evidence로 오인하는가? | 통과 | Chrome, layout frames, simulator surface readback을 별도 evidence로 보존하게 되어 있다. |
| 20 | 제한 구현으로 C11 또는 공식 완료 수를 앞당기는가? | 통과 | C11/C11.1 checkbox와 `72/213` 유지가 명시되어 있다. |

검토 중 계획 변경은 적용된 상태다. 지원 AST를 두 항 signed sum으로 고정하고, math ceiling과 cumulative extent를 분리하고, range-control 관측 및 재현 스크립트를 연결했다.
