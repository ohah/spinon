# C11.4 ratio span row placement 보정 PR diff 검토 30개

**검토 대상:** canonical HTML, 독립 expected values, Agent Browser 관측/verifier, evidence README, 집중 계획과 두 검토 문서의 실제 diff.

| # | PR diff 검토 관점 | 결과 |
|---:|---|---|
| 1 | canonical item에 explicit `grid-row:1`이 남는가 | **통과.** inline `grid-row` 선언을 제거했다. |
| 2 | canonical column start가 바뀌었는가 | **통과.** `grid-column:1 / span 2`를 유지한다. |
| 3 | column span end가 바뀌었는가 | **통과.** `span 2`를 유지하고 oracle이 확인한다. |
| 4 | row start computed value가 HTML과 다르게 선언되는가 | **통과.** canonical HTML은 row property를 지정하지 않으며 결과는 auto다. |
| 5 | row end computed value를 빠뜨렸는가 | **통과.** JSON과 verifier가 `gridRowEnd:auto`를 포함한다. |
| 6 | expected-values가 root row track과 item placement를 혼합하는가 | **통과.** root row track auto와 item row longhands를 별도 필드로 둔다. |
| 7 | 아홉 fixture의 case list가 줄거나 재정렬되는가 | **통과.** 기존 순서와 아홉 case가 유지된다. |
| 8 | 독립 columns/rows/item 기대 geometry가 달라지는가 | **통과.** JSON 기대값은 변경하지 않았다. |
| 9 | text width 또는 overflow 계산을 덮어쓰는가 | **통과.** text range와 item width 차의 기존 assertion을 유지한다. |
| 10 | ratio/control computed aspect-ratio 검사가 빠지는가 | **통과.** 기존 ratio assertion이 그대로다. |
| 11 | font shorthand 값 검증이 제거되는가 | **통과.** `10px / 10px Ahem` assertion이 유지된다. |
| 12 | row placement를 한 case만 검증하는가 | **수정됨.** verifier는 case loop 안에서 아홉 모두를 검사한다. |
| 13 | Browser JSON이 새 row fields 없이 stale 상태인가 | **수정됨.** HeadlessChrome 156에서 JSON을 다시 캡처했다. |
| 14 | observation user agent가 고정된 Chrome oracle와 다른가 | **통과.** pinned HeadlessChrome 156을 유지했다. |
| 15 | local Ahem font readiness를 잃었는가 | **통과.** 재관측에서 `fontReady:true`를 확인했다. |
| 16 | screenshot이 수정 전 DOM state일 가능성이 있는가 | **통과.** 동일한 새 browser session에서 screenshot을 다시 저장했다. |
| 17 | screenshot 해시가 README의 새 실행과 맞지 않는가 | **통과.** 새 PNG 해시는 기존과 같아 화면이 변하지 않았음을 확인했다. |
| 18 | canonical HTML README hash가 이전 값인가 | **수정됨.** 새 HTML 파일 해시로 교체했다. |
| 19 | expected JSON README hash가 이전 값인가 | **수정됨.** placement metadata가 반영된 JSON 해시로 교체했다. |
| 20 | observation JSON README hash가 이전 값인가 | **수정됨.** 새 Browser 관측 파일 해시로 교체했다. |
| 21 | unchanged Ahem binary hash를 불필요하게 바꾸는가 | **통과.** Ahem 파일은 변경하지 않았다. |
| 22 | evidence verifier의 hash check 자체가 제거되는가 | **통과.** 5개 artifact hash 대조가 유지된다. |
| 23 | verifier의 기존 1px negative control이 사라지는가 | **통과.** geometry mutation 거부 check를 유지했다. |
| 24 | plan의 root row auto 설명이 canonical fixture와 다른가 | **수정됨.** plan과 HTML의 row placement가 일치한다. |
| 25 | plan status가 local draft나 simulator 검증을 과장하는가 | **수정됨.** local draft와 미실행 runtime 범위를 구분한다. |
| 26 | plan correction이 official status/count를 올리는가 | **통과.** `spec/STATUS.md`와 `72/213`은 변경하지 않았다. |
| 27 | 이전 pixel limitation amendment와 중복/충돌하는가 | **통과.** glyph pixel 기준은 그대로이며 이번 diff는 row 입력 정합성만 다룬다. |
| 28 | 새 review document 링크가 없는 파일을 가리키는가 | **통과.** PR 생성 전 모든 새 relative Markdown link를 확인한다. |
| 29 | PR 변경이 제품/runtime 코드나 내부 계약에 섞이는가 | **통과.** plan/oracle/evidence 파일만 포함한다. |
| 30 | 비어 있는 GitHub checks를 성공이라고 보고하는가 | **통과.** PR check rollup은 별도 확인하고 성공으로 승격하지 않는다. |

이 표는 계획 보정 PR의 실제 diff 검토다. 제품 구현·simulator 검증·CI 성공 판정은 포함하지 않는다.
