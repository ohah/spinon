# C11.4 다섯 fit auto-placement `min-height:auto` 계획 검토 20개

**검토 대상:** [계획](./c11-4-root-html-repeated-text-autoplacement-five-fit-auto-minimum.md) 및 [Chrome 157 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-min-precomparison-2026-11-11/README.md)

계획 PR 전의 적대 검토다. 각 항목은 별도의 실패 조건을 확인한다. 이 목록은 구현/runtime 20관점 검토나 계획 PR diff 30관점 검토로 재사용하지 않는다.

| # | 실패 관점 | 확인 및 계획의 방어 |
|---:|---|---|
| 1 | `fit-content()`를 auto-minimum 대상이 아닌 track으로 오분류 | CSS Grid 2025 CRD §7.2의 min sizing `auto`와 §6.6 eligibility를 함께 적었다. |
| 2 | scrollable overflow에도 automatic minimum을 적용 | oracle은 `overflow:visible`로 고정하고 scrollable overflow는 rejection matrix에 둔다. |
| 3 | spanning 또는 flexible track 교차 item에 §6.6 단일-track 결과를 확대 | 각 반복 text는 한 implicit row에만 놓이며 flexible track과 span을 admission에서 거부한다. |
| 4 | min/max/auto tail을 다섯 fit member와 함께 세거나 같은 floor 결과로 취급 | 각 tail 종류를 독립 profile·비교 key로 보존하고 fit occurrence에서 제외한다. |
| 5 | 여섯 1-based fit-position quintuple 중 일부를 누락하거나 0-based로 해석 | `C(6,5)=6` 조합을 모두 수집하고 position metadata와 used rows를 검증한다. |
| 6 | 세 tail 값 중 특정 종류 또는 tail이 놓인 authored index를 누락 | `min-content`, `max-content`, `auto` 각각과 여섯 위치를 고정 admission으로 검사한다. |
| 7 | 다섯 cap permutation을 누락하거나 cap과 fit-position을 잘못 연결 | 다섯 cap의 `5! = 120` authored-order assignment를 모두 비교하고 typed value/index trace를 보존한다. |
| 8 | `%` cap을 indefinite block basis에서 계산 | definite 220px basis만 positive며 indefinite basis를 rejection matrix에 둔다. |
| 9 | cap이 automatic minimum content floor를 자른다고 잘못 가정 | baseline/candidate item frame과 payload floor를 별도 비교하며 candidate floor 0건을 oracle 결과로 고정한다. |
| 10 | 7·8번째 implicit item에서 6-member list wrapping을 잘못 계산 | 8개의 item ID와 6-member modulo placement를 forward/reverse DOM order에서 검증한다. |
| 11 | 빈 explicit-row control이 candidate `min-height:auto`를 상속 | fixture와 contract에서 첫 control은 `min-height:0`으로 고정한다. |
| 12 | baseline/candidate를 list 외 조건이 다른 채 pair 처리 | pairing key는 authored list, tail, alignment, DOM order를 포함하고 minimum mode만 비교 축으로 둔다. |
| 13 | `auto` tail의 `align-content:stretch` 동작을 다른 tail에 일반화 | Chrome에서 차이가 auto tail에만 1,440 pair/mode로 나타나므로 tail×minimum 비교를 분리한다. |
| 14 | 기준선의 Grid-end payload overflow를 candidate 위반으로 계산하거나 무시 | 600건을 `auto` tail + baseline `min-height:0`에 귀속하고 candidate는 별도로 검사한다. |
| 15 | row의 크기와 payload line-box floor를 혼동 | 각 item frame과 payload frame, line count/top, row sequence를 따로 기록한다. |
| 16 | CSSOM 정규화 결과를 authored cap/list order로 역추정 | authored track list/cap 순서와 computed list, used row sequence를 모두 보존한다. |
| 17 | 잘못된 list length/count, tail multiplicity, cap 또는 unsupported styles가 admission에 새어듦 | profile grammar, non-finite/negative/calc/indefinite cap, topology, style, owner 및 trace rejection을 열거한다. |
| 18 | rejection 뒤 이전 revision 일부가 게시되거나 같은 session이 복구되지 않음 | atomic rejection과 같은 RuntimeSession의 다음 유효 revision 성공을 구현 gate에 둔다. |
| 19 | 이미 지원하는 1–4-fit auto-placement, row-span 또는 repeated-text 경계가 회귀 | 각 이전 scene matrix와 row-span/repeated-text 회귀를 별도 gate로 지정한다. |
| 20 | headless Chrome 관측을 app/runtime, pixel parity, 실기기, WPT 완료로 과장 | oracle·runtime·simulator evidence를 구분하고 미포함 route/readback/device/WPT 범위를 명시한다. |

## 판정

계획은 fixed five-fit profile에 한정하며 C11.4/C11 완료 및 공식 `78/214`를 바꾸지 않는다. 17,280-scene corpus, 8,640 paired keys, matrix verifier, browser lifecycle 근거를 실제 계획 PR diff의 별도 30관점 검토와 연결한다. 구현 후에는 이 검토와 겹치지 않는 새로운 implementation/runtime 20관점 검토를 수행한다.
