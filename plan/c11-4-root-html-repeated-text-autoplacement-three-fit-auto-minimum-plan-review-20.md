# C11.4 three-fit auto-placement 계획 적대 검토 20개

검토 대상은 [three-fit auto-placement `min-height:auto` 계획](./c11-4-root-html-repeated-text-autoplacement-three-fit-auto-minimum.md)과 Chrome 157 사전 비교다. 각 항목은 별도의 실패 경로와 수용 조건을 다룬다. 구현은 아직 시작하지 않았다.

| # | 적대 관점 | 검토 및 계획의 방어 | 판정 |
|---:|---|---|---|
| 1 | one explicit item과 eight text item의 구조가 바뀌어 일반 Grid admission으로 새는가 | exact nine-item profile, 첫 빈 explicit row, 뒤 eight direct text item으로 고정한다. | 통과 |
| 2 | six-member list가 아닌데 track trace가 순환 해석되는가 | list length 6만 허용하고 여덟 implicit row의 6-member cycle을 별도 검사한다. | 통과 |
| 3 | three-fit predicate의 multiplicity가 1/2/4 fit에도 열리는가 | 정확히 세 fit와 min/max/auto 각 1개만 새 predicate에 허용하고 1/2/4-fit mutation은 rejection control로 둔다. | 통과 |
| 4 | position coverage에서 경계 위치 `[1,2,3]` 또는 `[4,5,6]`이 빠지는가 | 6개 위치 중 3개를 고르는 20개 triple 전체를 Chrome와 projection matrix에서 검사한다. | 통과 |
| 5 | tail의 authored order가 정렬·정규화 과정에서 손실되는가 | 남은 min/max/auto의 여섯 순서를 모두 기록하고 CSSOM 및 typed projection 순서를 대조한다. | 통과 |
| 6 | `fit-content()` cap이 다른 위치에 붙거나 단위가 바뀌는가 | `[0px,10px,20%]`의 여섯 순열을 fit position과 교차하고 authored index·computed value를 확인한다. | 통과 |
| 7 | 20%가 indefinite basis에서 잘못 계산되는가 | Grid block size를 definite 220px로 고정하고 indefinite percentage basis는 fail-closed 항목으로 둔다. | 통과 |
| 8 | `calc()`, 음수, NaN/무한 cap이 finite direct px/%와 섞여 허용되는가 | product admission은 finite non-negative direct px/%만 허용하고 invalid/calc cap mutation을 거부한다. | 통과 |
| 9 | 첫 explicit-row control에도 `min-height:auto`가 적용되어 기준이 오염되는가 | control은 모든 baseline/candidate에서 `min-height:0`을 요구한다. | 통과 |
| 10 | 여덟 item 중 일부에만 auto minimum이 적용되는 혼합 상태가 통과하는가 | positive profile은 여덟 item 전부 0 또는 전부 auto만 허용하고 mixed case를 거부한다. | 통과 |
| 11 | min-content/max-content row를 자동 최소 크기 eligible로 잘못 계산하는가 | §6.6/§7.2 기반으로 fit-content와 auto minimum track만 eligible로 집계한다. | 통과 |
| 12 | `fit-content()`의 authored cap이 content floor를 낮추는 것으로 오해되는가 | Chrome 157에서 cap 0px/10px도 item floor를 유지하는 관계를 baseline paired observation으로 분리한다. | 통과 |
| 13 | text payload 변화가 item minimum 차이로 오인되는가 | 10px monospace, 10px line-height, 고정 ASCII payload/line count, 동일 scene key를 고정한다. | 통과 |
| 14 | reverse DOM에서 payload ID와 implicit row 순서가 잘못 연결되는가 | forward/reverse 각각 row ID, payload frame과 line count를 독립적으로 검증한다. 두 순서의 결과 동일성을 잘못 요구하지 않는다. | 통과 |
| 15 | min-height baseline과 candidate가 서로 다른 track/cap/order를 비교하는가 | 동일 `(trackList, alignment, DOM order)` 2,880 key를 paired key로 검사한다. | 통과 |
| 16 | visible payload overflow와 Grid 전체 경계 overflow가 하나로 뭉개지는가 | undersized row의 payload spill과 마지막 item의 220px Grid 경계 통과를 별도 관측한다. | 통과 |
| 17 | `align-content:stretch` 자유 공간이 fit/min/max row까지 잘못 배분되는가 | candidate start/stretch의 전체 합과 각 authored row 차이를 검사하고 auto row occurrence에만 40px를 나눈다. | 통과 |
| 18 | style/layout trace가 다른 owner·revision에서 섞이거나 일부만 게시되는가 | 구현 관문에서 node owner, revision, complete row trace를 결합하고 누락/중복/stale trace는 atomic rejection 및 same-session recovery로 확인한다. | 통과 |
| 19 | span profile 변경이 auto-placement profile을 넓히거나 반대로 거부 경계가 깨지는가 | three-fit row-span을 독립 regression으로 두고 auto-placement exact 9-item route에만 새 predicate를 적용한다. | 통과 |
| 20 | Chrome oracle, app screenshot 또는 simulator smoke가 완제품/일반 CSS 지원으로 과장되는가 | 비교 모델·RuntimeSession·Android/iOS visual smoke의 증거 분류를 분리하고 C11.4/C11/공식 집계를 유지한다. | 통과 |

## 결론

계획의 독립 입력·oracle·paired key·지원 경계·rejection/atomic recovery·platform evidence가 각 실패 경로를 수용한다. 구현 전에 보완할 차단 결함은 찾지 못했다. 이 검토는 계획 검토이며 구현/runtime 검토나 실제 plan PR diff 검토를 대신하지 않는다.
