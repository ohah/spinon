# C11.4 fit-content implicit column 계획 적대 검토 20개

대상: [계획](./c11-4-fit-content-implicit-column-wpt.md), [사전 비교](../spec/internal/evidence/c11-4-fit-content-implicit-column-precomparison-2026-10-29/README.md)

| # | 실패 관점 | 검증과 판정 |
|---:|---|---|
| 1 | 다른 WPT revision에서 기대값을 가져온다. | 계획·source pin의 revision과 checkout revision `c999c58338ee1d223df5ad62e48be1202ced0d37`를 대조했다. 일치한다. |
| 2 | live WPT와 pinned source가 달라 target이 이동했다. | `wpt.live` response와 pinned file SHA-256이 동일하다. PASS. |
| 3 | WPT의 target markup에서 `grid-auto-columns` 값이 다르다. | pinned source lines 64–66은 `fit-content(40px)`이다. 계획 입력과 일치한다. |
| 4 | explicit prefix track을 빠뜨리거나 다른 값으로 둔다. | WPT의 `grid-template-columns:60px`를 그대로 둔다. Expected output은 prefix `60px`을 보존한다. |
| 5 | column 2 placement가 implicit column을 만들지 않는다. | explicit grid에 column 1만 선언되어 있고 item은 `grid-column:2`다. Target은 양의 implicit column을 요구한다. |
| 6 | `grid-column:2`를 `2 / 3`로 바꿔 원본 placement 의미가 달라진다. | 고정 scene은 start line `2`를 보존하고 end는 auto로 둔다. Matcher는 used area를 second column 하나로 검사한다. |
| 7 | item automatic minimum이 40px cap을 무시한다. | WPT가 `min-width:0`을 지정한다. Matcher는 computed `min-width:0` 및 zero minimum contribution을 확인한다. |
| 8 | child 400px max-content contribution이 잘못 계산된다. | 고정 block child width 400px로 text shaping·font fallback 변수를 제거하고 max-content contribution 400px을 검사한다. |
| 9 | fit-content limit을 minimum clamp로 오해한다. | 계획은 auto minimum, max-content contribution, 40px limit을 각각 분리해 둔다. |
| 10 | track growth가 400px까지 허용된다. | §11.5 growth limit clamp를 고정하고 final implicit column 40px을 검사한다. |
| 11 | 남은 definite inline space가 fit-content track에 stretch된다. | §11.8의 auto max track eligibility와 별도 200px remainder를 확인한다. |
| 12 | root width가 viewport·body sizing 때문에 달라진다. | WPT target width를 300px로 고정하고 앱 viewport를 480×320px로 둔다. Root-relative frames를 비교한다. |
| 13 | implicit row가 column oracle을 오염시킨다. | Row axis를 10px single implicit row로 고정해 cross-axis regression만 감시한다. Column result와 분리한다. |
| 14 | root border/padding/gap이 track basis를 바꾼다. | 고정 입력은 zero border, padding, gap을 요구한다. Scene matcher에서 computed values를 검사한다. |
| 15 | overflow가 viewport 밖으로 나가 pixel evidence가 불가능하다. | 480px viewport는 root 300px과 child endpoint 460px을 모두 포함한다. Root-outside sample x=350.5px을 요구한다. |
| 16 | diagnostic color가 sizing input을 바꾼다. | Background colors만 추가하고 border, padding, min/max, size declarations는 추가하지 않는다. |
| 17 | screenshot overlay를 WPT 원본 관측으로 오해한다. | README는 overlay가 harness와 geometry 관측 뒤에 들어갔음을 표시한다. JSON raw geometry는 overlay style과 분리한다. |
| 18 | CSSOM output을 authored track list와 혼동한다. | WPT가 단언한 resolved `gridTemplateColumns` string `60px 40px`을 비교한다. |
| 19 | default archive에 test fixture ABI가 들어간다. | 계획은 product/default marker 부재와 opt-in fixture positive control을 각각 요구한다. |
| 20 | 제한 assertion 통과로 C11.4 완료를 올린다. | 수용 기준은 WPT runner/full support/C11 completion을 제외하고 `72/213` 및 checkbox를 유지한다. |

**판정:** 20/20 관점에서 계획 입력과 판정 기준이 분리되어 있다. 계획은 기능 구현 완료나 C11.4 승격을 주장하지 않는다.
