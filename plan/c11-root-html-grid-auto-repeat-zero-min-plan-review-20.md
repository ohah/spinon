# C11.1 Root HTML auto-repeat zero minimum · 계획 실패 관점 검토 20개

검토 대상: [계획](./c11-root-html-grid-auto-repeat-zero-min.md), [Chrome/독립 사전 비교](../spec/internal/evidence/c11-root-html-grid-auto-repeat-zero-min-precomparison-2026-11-06/README.md), 고정 HTML 입력과 expected output.

| # | 서로 다른 실패 관점 | 판정 |
|---:|---|---|
| 1 | CSS Grid 자동 반복 grammar가 zero fixed minimum을 허용하지 않는다고 잘못 가정할 수 있음 | PASS — auto-repeat는 definite fixed-size를 요구하며 `0px`는 definite fixed breadth다. |
| 2 | UA floor 권고를 규범상 반드시 1px이라고 확대할 수 있음 | PASS — 계획은 1px을 이번 profile의 선택값으로 고정하고 명세의 “suggested” 상태를 구분한다. |
| 3 | count floor가 실제 `minmax(0px,1fr)` minimum을 바꿀 수 있음 | PASS — count-only floor와 used track sizing의 0px minimum을 별도 acceptance로 분리한다. |
| 4 | two-track pattern을 단일-track count 공식으로 계산할 수 있음 | PASS — `P`개의 내부 track minimum과 pattern 내부/반복 사이 gutter를 모두 식에 포함한다. |
| 5 | gap을 count formula에서 한 번 빠뜨리거나 반복마다 중복할 수 있음 | PASS — denominator의 `P × G`와 numerator의 경계 `+ G`로 전체 gutter 수를 반영한다. |
| 6 | container가 너무 작을 때 0 repetition을 만들어 규범과 어긋날 수 있음 | PASS — `max(1, floor(...))` 최소 한 repetition 규칙을 명시한다. |
| 7 | `0px` 대신 음수·NaN·무한·percentage를 zero profile로 축약할 수 있음 | PASS — finite non-negative px만 열고 unsupported computed values는 reject한다. |
| 8 | parser가 authored source만 보고 cascade된 실제 값을 무시할 수 있음 | PASS — 계산된 Root HTML typed style을 기준으로 검증하도록 계획한다. |
| 9 | auto-fill과 auto-fit가 같은 결과로 처리될 수 있음 | PASS — auto-fill은 빈 track을 보존하고 auto-fit은 placement 뒤 빈 track을 collapse하는 별도 장면이 있다. |
| 10 | 빈 auto-fit에서 track이 남거나 gutter가 유지될 수 있음 | PASS — 빈 auto-fit과 점유 auto-fit을 각각 expected에 고정한다. |
| 11 | collapse를 placement 전에 적용해 occupancy가 잘못될 수 있음 | PASS — 계획이 item placement 뒤 collapse 순서를 명시한다. |
| 12 | rows 축이 columns 경로를 그대로 가정해 계산을 바꿀 수 있음 | PASS — rows와 columns에 독립된 positive·mixed fixtures를 둔다. |
| 13 | subpixel gap/track size가 정수로 반올림되어 count 경계가 바뀔 수 있음 | PASS — 0.5px gap 및 1.125px used tracks를 독립 비교에 고정한다. |
| 14 | mixed `[0px, 2px]` pattern에서 각 minimum에 1px floor가 적용되지 않을 수 있음 | PASS — count basis `[1px, 2px]`와 used `[1px,2px]` 결과를 따로 검증한다. |
| 15 | zero track 두 개 pattern의 pattern 크기와 expanded count를 혼동할 수 있음 | PASS — 5 repetitions가 10 expanded tracks임을 별도 기대값에 적었다. |
| 16 | repeat count는 제한 안이지만 pattern 배수의 expanded tracks가 limit을 넘을 수 있음 | PASS — quota는 repetition count가 아니라 expanded track 수에 적용한다. |
| 17 | 10,000-track 경계 초과 때 partial layout이 게시될 수 있음 | PASS — overflow-safe preflight와 pre-publication rejection을 acceptance로 요구한다. |
| 18 | 기존 positive minimum, integer repeat, C11.4 sizing 회귀가 깨질 수 있음 | PASS — 기존 positive profile과 C11.1/C11.4 회귀를 유지한다. |
| 19 | Chrome 157 관찰을 규범 기대값이나 WPT 적합성으로 잘못 취급할 수 있음 | PASS — independent formula와 browser observation, WPT 제외를 분리한다. |
| 20 | 제한 구현을 C11.1/C11 전체 지원이나 공식 완료 수 증가로 오인할 수 있음 | PASS — C11 checkbox와 `78/214`를 유지하고 exclusions를 명시한다. |

## 최종 판정

구현 전 계획 관문은 20개 관점 모두 PASS다. Plan PR의 실제 diff는 별도 30관점에서 다시 검토한다. 이 검토는 아직 구현/runtime 검토나 PR diff 검토를 대신하지 않는다.
