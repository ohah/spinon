# C11.4 four-fit row-span `min-height:auto` 계획 검토 20개

**검토 대상:** [four-fit row-span 계획](./c11-4-root-html-repeated-text-row-span-four-fit-auto-minimum.md) 및 [Chrome 157 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-auto-minimum-precomparison-2026-11-11/README.md)

계획 PR 전 실패 관점 검토다. 20개 항목은 서로 다른 admission·track sizing·item sizing·revision·증거 경계를 다룬다. 실제 계획 PR diff와 구현/runtime 검토에는 재사용하지 않는다.

| # | 실패 관점 | 검토한 방어 | 판정 |
|---:|---|---|---|
| 1 | 네-fit 예외가 일반 six-row Grid admission으로 번짐 | 여섯 track, 네 fit, `min-content` 하나, `auto` 하나의 exact typed multiset으로 한정한다. | 통과 |
| 2 | fit position 1-based와 Rust index가 한 칸 어긋남 | 15 quadruple 전체와 track member index/cap mapping을 Chrome observation으로 재검증한다. | 통과 |
| 3 | fit position 하나가 중복되거나 빠져 실제 fit 수가 3/5가 됨 | 순서 증가·유일성·15 조합을 확인하고 5/6-fit 변형은 별도 거부한다. | 통과 |
| 4 | tail track 한 개가 대체·중복되거나 허용 순서 밖으로 이동 | 남은 두 index의 `[min-content, auto]` 및 역순만 admission한다. | 통과 |
| 5 | `max-content` 또는 flexible track이 네-fit row list에 섞임 | exact tail 검사를 통해 max-content·`fr`·다른 track type을 거부한다. | 통과 |
| 6 | 네 cap 값은 맞지만 authored position과 typed value 연결이 바뀜 | 네 fit position별 computed typed cap, authored order와 같은 revision trace를 비교한다. | 통과 |
| 7 | cap 순열 행렬에서 특정 4! assignment가 누락 | 고유 cap 24순열과 독립 verifier의 각 480 cap group을 확인한다. | 통과 |
| 8 | percentage cap이 indefinite basis에서 임의 px로 계산됨 | definite 220px Grid block size를 고정하고 indefinite percentage는 fail-closed한다. | 통과 |
| 9 | resolved `calc()`와 unresolved/음수/비유한 cap이 같은 값으로 처리됨 | 기존 typed projection이 exact cap으로 normalize한 계산만 값으로 분류하고 unresolved/invalid는 거부한다. | 통과 |
| 10 | fit-content cap이 automatic minimum content floor를 자름 | Chrome baseline/candidate payload floor를 각각 재계산하고 cap과 min contribution을 분리한다. | 통과 |
| 11 | `min-height:auto`가 scroll container에도 자동으로 content floor를 강제 | profile을 visible non-scrollable overflow로 제한하고 scrollable 값은 거부한다. | 통과 |
| 12 | span이 자동 최소 track을 포함하지 않아도 floor를 계산 | `fit-content()`/`auto` minimum track을 span한 item만 eligibility cohort로 분리한다. | 통과 |
| 13 | span에 flexible track이 들어와 자동 최소 크기 조건을 위반 | 지원 track multiset에 flex track을 두지 않고 추가 flex member도 거부한다. | 통과 |
| 14 | 네 text item 중 일부만 `auto`가 되어 paired profile을 오염 | 모든 span item은 같은 `0` 또는 `auto`; mixed mode는 rejection matrix로 둔다. | 통과 |
| 15 | 빈 explicit-row control이 candidate `auto` minimum의 영향을 받음 | control은 두 모드에서 모두 row 1 placement와 `min-height:0`으로 고정한다. | 통과 |
| 16 | span frame이 다른 row를 더하거나 explicit row를 span 합에 포함 | explicit row를 제외한 여섯 implicit used row와 각 item occupied-row sum을 비교한다. | 통과 |
| 17 | source order reverse가 동일 item 의미 geometry를 바꿈 | 두 topology 가족 각각 forward/reverse semantic item ID와 track value를 pair한다. | 통과 |
| 18 | stretch가 fit/min-content track을 늘려 다른 sizing 결과를 감춤 | `auto` row만 free-space recipient로 검사하고 나머지 used row 불변을 요구한다. | 통과 |
| 19 | invalid trace/revision이 일부 layout을 게시한 뒤 후속 요청을 오염 | 모든 rejection 뒤 동일 RuntimeSession의 새 valid revision 복구와 atomic output 부재를 요구한다. | 통과 |
| 20 | Chrome oracle 또는 기본 앱 smoke를 four-fit 제품 실행·C11.4 완료로 표현 | Chrome matrix, RuntimeSession, 기본-route simulator smoke와 public support 범위를 별도로 기록한다. | 통과 |

## 판정

계획 범위를 바꿀 결함은 찾지 못했다. Chrome 사전 비교는 기준 동작이며 제품 구현 증거가 아니다. 코드 변경 전에 계획 PR을 병합하고, 실제 PR diff는 별도의 30개 관점으로 검토한다. 구현 뒤에는 새 20개 runtime review와 별도의 PR diff review를 수행한다.
