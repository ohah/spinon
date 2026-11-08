# C11.4 four-member fit-content cap 정규화 계획 수정 · 실패 관점 20개

원래 계획은 authored `calc()` cap을 모두 거부했다. RuntimeSession 관측에서 absolute CSS math가 profile 입력 전에 computed px 값으로 정규화되는 점이 확인됐다. 아래 수정은 computed typed cap 경계를 유지하도록 계획을 갱신한다. 이는 사전 계획 검토 20개 및 구현/runtime 검토와 별도다.

| # | 적대적 실패 경로 | 필요한 경계 | 결과 |
|---:|---|---|---|
| 1 | computed-value 정규화 뒤에도 authored syntax를 복원할 수 있다고 오해한다 | 이 profile은 raw declaration이 아닌 typed computed value를 받는다고 명시했다. | PASS |
| 2 | `calc(12px + 8px)`가 유한한 `20px`로 계산되어도 거부한다 | 정규화된 `FitContent(LengthPx(20))` 결과만 허용한다. | PASS |
| 3 | absolute calc 정규화 뒤 computed serialization이 이전 `calc()`를 유지한다고 가정한다 | canonical `fit-content(20px)` serialization을 확인한다. | PASS |
| 4 | percentage가 든 `calc()`를 direct percentage로 오인한다 | unresolved `calc(12px + 8%)`를 fail-closed한다. | PASS |
| 5 | absolute calc 허용이 나머지 track member도 넓힌다 | fit-content·min-content·max-content·auto를 각 하나씩 요구한다. | PASS |
| 6 | 음수 computed cap을 track sizing에 전달한다 | finite non-negative cap 조건과 음수 negative control을 유지한다. | PASS |
| 7 | NaN 또는 무한 cap이 enum 모양만으로 통과한다 | projection·layout 경계에서 finite 조건을 확인한다. | PASS |
| 8 | definite containing block 없이 percentage cap을 허용한다 | 정확한 100px block-size basis를 요구하고 indefinite Grid height는 거부한다. | PASS |
| 9 | percentage cap이 px로 바뀌어 typed identity를 잃는다 | `Percentage(0.2)`를 px cap과 별도로 보존한다. | PASS |
| 10 | 정규화로 cap의 track-list 위치가 바뀐다 | 24개 typed member 순서를 모두 보존한다. | PASS |
| 11 | 다섯 번째 implicit row가 첫 member를 반복하지 않는다 | 5개 row cycle과 첫 row payload 반복을 확인한다. | PASS |
| 12 | 정규화 허용이 기존 3-member/min-content profile에 적용된다 | four-member 인식은 정확히 유지하고 기존 item 수는 바꾸지 않는다. | PASS |
| 13 | singleton 또는 무관한 intrinsic placement mode까지 admission된다 | `RootHtmlFitContentAutoRowsV1`만 허용 owner로 유지한다. | PASS |
| 14 | `align-content:stretch`가 정규화된 fit/min/max track을 늘린다 | 남는 공간은 auto track에만 분배한다. | PASS |
| 15 | `align-content:start`에서 free space가 사라진다 | 78px track 합과 남는 22px을 유지한다. | PASS |
| 16 | cap 정규화 때문에 payload allowlist가 넓어져 다른 값을 받는다 | 위치별 다섯 payload height를 제한한다. | PASS |
| 17 | mixed calc 거부가 partial layout/render를 남긴다 | 원자 실패와 같은 세션 복구를 확인한다. | PASS |
| 18 | 문서 수정이 새 parser·공개 API·FFI selector 추가를 뜻한다 | 공개 및 FFI surface는 그대로 둔다. | PASS |
| 19 | 계획 정정을 릴리스 capability로 표시한다 | C11.4/C11 미완료, 공식 `78/214`, 계약 `0.1.0`을 유지한다. | PASS |
| 20 | runtime 관측을 브라우저 또는 표준 적합성으로 확대한다 | Stylo computed-value 경로의 관측으로만 귀속한다. | PASS |

Absolute cap 결과는 focused RuntimeSession regression에서 Ready 상태, typed `FitContent(LengthPx(20))`, computed serialization과 같은 layout/style revision을 검사한다. Mixed percentage calc와 indefinite-height controls는 atomic rejection test에 둔다. 이 리뷰는 cap-normalization 계획 수정만 검토하며 구현/runtime 또는 PR diff 검토를 대신하지 않는다.
