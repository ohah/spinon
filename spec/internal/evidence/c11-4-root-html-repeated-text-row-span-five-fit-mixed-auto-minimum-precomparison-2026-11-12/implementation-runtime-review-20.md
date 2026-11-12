# C11.4 five-fit row-span mixed minimum 구현/runtime 검토 20개

검토 대상은 현재 branch의 five-fit style admission, layout admission 및 공유 fit-content span sizing·rounding 변경이다. Chrome 기준 전체 414,720-scene oracle과 RuntimeSession mixed sample 3,984 scenes, five-fit uniform endpoint 34,560 scenes, six-fit uniform row-span 11,520 scenes를 구분해 판정한다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | mixed mask가 five-fit 외 임의 Grid list까지 허용됨 | PASS — style/layout 각각 6-member, 다섯 typed cap 각 1회, singleton intrinsic/auto tail만 admission한다. profile tests가 모든 위치·cap order·tail 및 중복/미등록 cap을 확인한다. |
| 2 | style projection과 layout admission의 profile 조건이 서로 다름 | PASS — 두 계층이 동일한 five-fit list helper와 기존 repeated-text row-span topology gate를 함께 요구한다. RuntimeSession matrix가 실제 snapshot까지 통과했다. |
| 3 | 첫 explicit-row empty control도 automatic minimum mask에 포함됨 | PASS — style/layout 양쪽 모두 control을 `min-height:0`으로 고정하며 atomic rejection test가 `auto`를 fail-closed 처리한다. |
| 4 | span item에 `auto`와 `0` 외 다른 min-height가 허용됨 | PASS — 유효 값은 `Auto|Fixed(0)` 뿐이다. media-query로 `1px`을 활성화한 same-session rejection/recovery가 통과했다. |
| 5 | mask bit가 source order 뒤집기에 따라 다른 semantic item에 적용됨 | PASS — A–E/A–D item ID로 mask를 다시 인코딩하고 metadata test가 기록 mask와 대조한다. matrix는 forward/reverse frame pair를 확인한다. |
| 6 | sample에서 일부 intermediate mask, cap order 또는 fit/tail 문맥 누락 | PASS — 독립 verifier와 metadata test가 3,984 scenes, 120 lists, 18개 full context, 102개 rotating context와 mask 산식을 확인했다. |
| 7 | 전체 Chrome oracle을 RuntimeSession 실행 결과로 과장 | PASS — 근거 문서는 414,720 Chrome scenes, 3,984 mixed RuntimeSession scenes와 34,560 uniform endpoint scenes를 별도 denominators로 쓴다. |
| 8 | eligible automatic minimum 계산에서 실제 spanned row나 flexible track을 무시 | PASS — 관측된 scene별 span eligibility와 contribution trace를 사용한다. 3,984-scene RuntimeSession 결과가 모든 used row/frame과 대조됐다. |
| 9 | `min-height:auto` content floor 또는 visible Grid-end 동작이 Chrome과 다름 | PASS — 예상/실제 floor shortfall 및 Grid-end overflow 집계가 일치하고 auto 집계는 모두 0이다. |
| 10 | floor/overflow 집계 배열 순서를 반대로 해석 | PASS — source의 mode index는 `[min-height:0, min-height:auto]`다. 로그 문구와 문서를 바로잡고 mixed chunk 2를 재실행해 `zero/auto=[120, 0]`을 확인했다. |
| 11 | minimum/content-based minimum 기여가 max-side growth limit 또는 fit cap에 잘못 막힘 | PASS — beyond-step phase별 potential을 분리했다. 3,984 mixed scenes와 existing five-/six-fit uniform matrices가 통과했다. |
| 12 | `max-content` minimum phase가 fit-content cap을 무시함 | PASS — `MaxContentMinimum`은 해당 track의 fit-content ceiling을 계속 적용한다. max-content tail/span rounding case와 matrix 비교가 통과했다. |
| 13 | contribution phase, span, item contribution kind가 엇갈려 적용됨 | PASS — trace에 기록된 같은 span·kind만 phase별로 재생하고 영향을 받지 않는 항목은 증가량을 배정하지 않는다. mixed matrix row values 및 frames가 oracle과 일치했다. |
| 14 | growth-limit trace 보정이 mismatching stage를 조용히 통과시킴 | PASS — 최초 no-sync simulation은 각 stage base와 최종 pre-alignment values를 대조한다. 불일치할 때만 growth-limit sync fallback을 시도하고 어느 path든 match가 없으면 `None`이다. |
| 15 | 없는 trace, gutter, 잘못된 px 값 또는 예상 track 수가 부분 결과로 반환 | PASS — trace 중복/누락, nonzero gutter, nonfinite/음수 LayoutUnit 및 track count 불일치는 모두 fail-closed한다. |
| 16 | 1/64px growth potential 동점 순서가 f32 잡음에 따라 바뀜 | PASS — potential을 LayoutUnit으로 비교하고 기존 auto/후행 row tie-break를 유지한다. focused rounding cases와 mixed matrix가 통과했다. |
| 17 | `ceil` 대상 합계의 f32 오차가 잘못된 1/64px를 추가함 | PASS — 유한 raw total의 float 오차 한도 안에서만 상향 단위를 보정한다. `later_span_keeps_layout_units` 및 전체 sample이 통과했다. |
| 18 | stretched singleton `auto` row가 intrinsic row 반올림 잔여분을 흡수하지 못하거나 다른 row가 바뀜 | PASS — mixed five-fit + 실제 stretch + singleton auto-tail에서만 trace residual 조건을 허용한다. auto-tail 회귀가 통과하고 max-content tail stretch는 고정된다. |
| 19 | 공유 sizing simulator 수정이 기존 uniform endpoints를 깨뜨림 | PASS — five-fit 34,560 scenes/4 chunks 및 six-fit 11,520 scenes/4 chunks가 통과했고 candidate payload-floor 위반은 각각 0이다. |
| 20 | 실패 revision이 partial snapshot을 내거나 세션이 복구되지 않음 | PASS — explicit-row control rejection과 same-session mixed acceptance→invalid-value rejection→recovery가 통과했다. mixed sample test도 12/12 통과했다. |

구현/runtime 검토에서 수정 차단 항목은 없었다. #1369 six-fit mixed 138,240-scene 전체 matrix 중 chunk 1/12의 11,520 scenes가 통과했고 나머지 126,720 scenes는 이번 branch에서 재실행하지 않았다. Android/iOS basic Root HTML route smoke는 light/dark `layout=ready`와 draw `status=0 presented`로 확인했으나 five-fit mixed feature matrix를 화면에서 실행한 결과는 아니다. 이 검토는 실제 PR diff 30관점 검토를 대체하지 않는다.
