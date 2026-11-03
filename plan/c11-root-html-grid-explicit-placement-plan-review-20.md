# C11 parsed Root HTML numeric placement 계획 실패 관점 검토 20개

검토 대상은 [`c11-root-html-grid-explicit-placement.md`](./c11-root-html-grid-explicit-placement.md)와 연결된 precomparison이다. 계획 PR diff 검토 및 이후 구현/runtime 검토와 별개로, 계획이 빠뜨린 거부·소유권·검증 경계를 찾는다.

| # | 독립 실패 관점 | 계획 검토 결과와 방어 |
|---:|---|---|
| 1 | root-html profile이라는 이유만으로 모든 Grid에 `ExplicitOnly`를 적용 | **통과.** computed placement가 있는 no-area row-flow owner만 기존 fail-closed validator를 통과시키고 owner별로 모드를 결정한다. |
| 2 | visible item 하나의 placement만 투영하고 나머지를 암묵 auto로 배치 | **통과.** selected owner의 visible child 전부를 projection하고 기존 all-definite validator가 mixed auto를 거부한다. |
| 3 | 한 문서의 all-auto Grid가 다른 Grid의 authored placement 때문에 거부 | **통과.** route와 mode가 Root HTML document 전역이 아니라 Grid owner별이다. |
| 4 | named-area typed mapping이 numeric route에 의해 덮어씀 | **통과.** named-area owner는 후보에서 제외하고 기존 area route의 경계를 유지한다. |
| 5 | column-flow나 dense placement에서 authored line이 auto-flow 결과와 혼합 | **통과.** 기본 `grid-auto-flow:row` 후보만 선택하고 기존 별도 route가 검증을 계속 소유한다. |
| 6 | `display:none` 자식의 범위 밖 line이 전체 scene을 잘못 거부 | **통과.** hidden subtree는 layout-participating child 집합에서 제외하는 대조 scene을 둔다. |
| 7 | CSSOM element scan이 anonymous direct text Grid item을 누락 | **통과.** HostDocument layout child 전체를 검사하고 mixed auto rejection 및 text precomparison을 요구한다. |
| 8 | partial axis placement를 definite placement로 잘못 분류 | **통과.** 양 축 모두 definite 조건과 column-only/row-auto negative가 있다. |
| 9 | shorthand `grid-area` 결과를 다시 parse하여 Stylo cascade와 어긋남 | **통과.** computed longhand만 입력으로 쓰고 span·longhand winner를 관측한다. |
| 10 | line 1-based와 layout 0-based indexing을 혼동 | **통과.** 기존 C11.2.1 normalizer를 재사용하고 기대 frame에서 각 explicit line을 고정한다. |
| 11 | 음수 line의 explicit-grid 끝 기준 환산이 잘못됨 | **통과.** negative line pair의 2×2 끝 cell을 독립 기대값에 둔다. |
| 12 | reversed start/end가 빈 track 또는 다른 cell로 처리 | **통과.** reversed positive endpoints를 관측하고 기존 conflict normalization을 활용한다. |
| 13 | span 한정값 또는 shorthand placement end가 빠짐 | **통과.** positive span을 각 축에서 포함하고 computed span longhand를 별도 기대값으로 둔다. |
| 14 | line number가 explicit grid 밖으로 나가 implicit track을 암묵 생성 | **통과.** 3/4 row의 Chrome implicit geometry와 runtime whole-scene rejection을 분리한다. |
| 15 | 동일 cell overlap에서 source order/paint snapshot 회귀 | **통과.** 동일 frame 두 개와 browser hit-test의 마지막 item을 oracle에 둔다. |
| 16 | overflow integer, line 0, zero span 처리에서 panic·wrap | **통과.** C11.2.1 checked arithmetic와 invalid line/span 및 overflow rejection을 구현 gate에 둔다. |
| 17 | route 검사 전 일부 item/frame을 publish | **통과.** style/layout error atomicity, stale revision, 기존 render snapshot 유지와 same-session recovery를 요구한다. |
| 18 | percentage/fr/repeat track 경계까지 새로 확장해 explicit placement로 혼합 | **통과.** 현재 Root HTML track projection을 그대로 쓰고 새 track grammar를 제외한다. |
| 19 | C11.5 alignment admission이 grid placement mode를 먼저 바꿔 explicit lines가 빠지거나 alignment value가 사라짐 | **통과.** explicit mode 승격 순서를 검증하고 fixed-size `align-self:end`·`justify-self:end` 교차 scene을 기대값에 포함한다. |
| 20 | 제한 profile 결과를 CSS Grid 전체 또는 공식 완료 수로 확대 | **통과.** 미지원 범위, `0059` 버전 유지, checkbox와 `72/213` 보존을 명시한다. |

## 결론

이 계획은 기존 C11.2.1 numeric normalizer를 Root HTML default route에 owner 단위로 연결하는 제한된 후속이다. 혼합·부분 auto, anonymous text auto-placement 및 implicit extent를 성공처럼 처리하지 않도록 구현 단계에서 전체 scene 거부와 route isolation을 검증한다. 이 검토는 코드, Android/iOS runtime 또는 PR diff 검증을 대신하지 않는다.
