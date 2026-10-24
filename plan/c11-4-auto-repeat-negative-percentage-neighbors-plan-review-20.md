# C11.4 음수 percentage auto-repeat 이웃 계획 실패 관점 검토 20개

대상 계획: [signed percentage `calc()` 이웃](./c11-4-auto-repeat-negative-percentage-neighbors.md) · [사전 비교](../spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/README.md)

계획 PR 전에 계획의 범위·규범·oracle·오류 경계를 서로 다른 실패 관점으로 점검했다. 아래 항목은 계획 검토이며 구현이나 앱 runtime을 검증하지 않는다.

| # | 실패 관점 | 공격 검토와 판정 |
|---:|---|---|
| 1 | 완료 상태를 계획 단계에서 승격 | 계획은 C11.4 체크와 `72/213`을 유지하고, #583의 merge SHA만 상태 문구에서 바로잡는다. 통과. |
| 2 | `calc(20px - 5%)`를 parse-time invalid로 오판 | CSS Values §10.12와 Chrome `CSS.supports()` positive control이 선언 유효성을 확인한다. unsupported profile 진단과 CSS 문법 오류를 구분한다. 통과. |
| 3 | 음수 결과의 허용 범위 누락 | Grid track breadth `[0,∞]`와 top-level math result clamp를 연결해 negative final을 0px로 정의한다. 통과. |
| 4 | clamp 시점을 합산 전에 적용 | 계획은 signed terms를 먼저 percentage basis로 계산하고 final value에 nonnegative range를 적용한다. 통과. |
| 5 | 실제 Stylo AST가 예상과 다름 | 구현 수용 전에 cascade test에서 `Sum` term shape를 확인하고 다르면 계획 재검토를 요구한다. 통과. |
| 6 | 음수 Percentage의 두 동등 typed 표현 | finite signed `Percentage`와 finite `Negate(Percentage)` 한 단계만 허용하고 일반 nested math는 제외한다. 통과. |
| 7 | 이중 negate·중첩 AST가 whitelist를 우회 | 한 번만 감싼 node와 정확히 두 top-level term을 고정하고 `Negate(Negate(...))`는 거부 대상으로 둔다. 통과. |
| 8 | Sum 항 순서에 따른 비대칭 | length-first와 percentage-first를 cascade·projection·layout 양쪽에서 계획한다. 통과. |
| 9 | Percentage가 열 축에서 잘못된 크기를 참조 | Grid §7.2.1의 inner inline size와 column property context를 명시했다. 통과. |
| 10 | Percentage가 행 축에서 inline size를 참조 | Grid §7.2.1의 inner block size와 `grid-template-rows` property context를 별도로 고정했다. 통과. |
| 11 | border-box outer size를 basis로 사용 | oracle은 outer 136px에서 inline padding 16px와 border 4px를 제외한 116px를 독립 계산한다. 통과. |
| 12 | auto-repeat count에서 gutter 수 누락 | 40px repeated minimum과 4px gap을 고정하고 prefix/suffix 각각의 한 neighbor case에서 count를 독립 산출한다. 통과. |
| 13 | fit 불가일 때 최소 1회 규칙 변경 | C11.4 CSS Grid repeat-count 규칙과 현재 Taffy 경로를 유지하며 이 계획에서 count 알고리즘을 수정하지 않는다. 통과. |
| 14 | signed neighbor가 fr free space를 바꿈 | Chrome oracle가 neighbor 8px/0px 이후의 반복 track used size를 기록하며 independent formula로 대조한다. 통과. |
| 15 | auto-fit이 auto-fill처럼 처리 | occupied repeat 1개와 collapsed repeat 4개를 고정하고 `8px 228px 0px × 4` Chrome control을 추가했다. 통과. |
| 16 | calc binding의 node/property 혼선 | 기존 calc ID unique성, 같은 node, column/row property identity와 definite-axis basis gate를 그대로 acceptance에 둔다. 통과. |
| 17 | invalid AST에서 partial layout 반환 | unsupported math shape는 projection 또는 layout에서 구체 오류로 닫고 partial output 부재를 검사한다. 통과. |
| 18 | 음수 입력이 overflow 검사를 우회 | finite/layout-representability 및 aggregate guard를 유지하고 지원 가능한 final negative만 0px로 clamp한다. 초과값의 clamp를 주장하지 않는다. 통과. |
| 19 | Chrome 값을 normative expected로 역산 | fixture term과 basis에서 독립 track model을 계산하고 Chrome CSSOM 양자화에 1/64px 허용값만 적용한다. validator 9/9 통과. |
| 20 | fixture 전용 코드를 제품 artifact에 유입 | 구현 관문에 기본 product vs opt-in fixture-test archive positive/negative control을 두고 simulator 결과의 플랫폼 한계를 구분했다. 통과. |

## 판정

계획 검토 차단 항목은 없다. 적용한 보정은 계획에서 signed percentage AST 표현을 명확히 하고, finite/layout-representable 경계 밖 수치까지 clamp를 보장한다고 과장하지 않도록 한정한 것이다. 검토는 위 계획 문서에 반영된 revision을 대상으로 한다.
