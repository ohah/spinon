# C11.3.3 area-created fit-content track 분류 정정 검토 20개

초기 계획의 “implicit column” 분류를 CSS Grid 규범과 첫 typed runtime diagnostic에 대조해 수정했다. 검토 대상은 [계획](../../../../plan/c11-3-3-template-area-fit-content-cap.md), [기대값](expected-values.json), [Chrome oracle](chrome/README.md), [정정 기록](runtime-model-correction-2026-10-31.md)이다. 이 표는 구현 검토나 PR diff 검토를 대신하지 않는다.

| # | 실패 관점 | 정정 검토와 결과 |
| ---: | --- | --- |
| 1 | `grid-template-areas` matrix dimension을 explicit-grid 크기에 반영하지 않음 | 규범 §7.1에 따라 matrix 1×2가 explicit area grid 크기를 정하도록 계획과 기대값을 수정했다. |
| 2 | authored template track 수만으로 explicit grid 크기를 고정함 | 1개 authored column과 2개 area column을 구분하고 effective explicit count 2를 기대한다. |
| 3 | area가 추가한 explicit track을 implicit track으로 오인함 | area-created unsized second column은 explicit grid track이라고 명시했다. |
| 4 | `grid-auto-columns`가 positive implicit track에만 적용된다고 가정함 | 규범 §7.5–7.6에 따라 explicit grid 내 unsized area-created track의 sizing 함수로 기록했다. |
| 5 | area 밖 item이 있어 implicit track이 함께 생기는 입력을 사용함 | fixture에는 2개 area 밖 placement가 없고 positive implicit count 0이라고 확인했다. |
| 6 | auto-track sizing과 implicit track 개수를 하나의 assertion으로 혼합함 | typed `columns`와 `auto-columns`를 각각 확인하고 trace explicit/implicit 카운트를 따로 둔다. |
| 7 | CSSOM `grid-template-columns` used value를 authored list와 혼동함 | Chrome computed columns `20px 40px`는 used explicit list의 oracle로 기록했다. |
| 8 | track trace가 CSSOM 관측을 대체한다고 간주함 | browser oracle, typed adapter input, runtime trace를 별개 근거로 요구한다. |
| 9 | `fit-content(40px)` upper limit만 적용하고 auto minimum floor를 생략함 | 계산을 `max(auto minimum, min(limit, max-content))`로 고치고 intrinsic floor도 검증하도록 했다. |
| 10 | min-content가 limit보다 커 cap 효과가 보이지 않음 | 독립 Ahem min-content 20px을 고정해 40px limit보다 작게 둔다. |
| 11 | max-content가 limit보다 작아 limit effect를 보이지 않음 | 독립 Ahem max-content 110px을 고정해 40px limit보다 크게 둔다. |
| 12 | expected final width를 intrinsic probe에서 추론만 함 | actual used track, item frame, trace를 각각 40px로 검사하도록 남겼다. |
| 13 | 첫 번째 column의 20px가 second column에도 복제됨 | authored columns `[20px]`, area dimension 2, used track list `[20px, fit-content(40px)]`를 분리했다. |
| 14 | area rectangle이 2번째 column을 차지하지 않음 | named area `fit` 범위 `[1,2)`와 item x=20,width=40을 함께 고정했다. |
| 15 | track 폭은 맞지만 item stretch/frame이 다른 값임 | track trace와 item frame `[20,0,40,60]`을 별도 관측한다. |
| 16 | `grid-template-rows`의 60px가 unsized column 해석을 흐림 | row axis를 fixed `[60px]`로 둬 비교 범위를 inline axis 하나로 제한했다. |
| 17 | writing mode 또는 direction 때문에 column/physical x 대응이 달라짐 | fixture를 horizontal-tb/LTR로 고정하고 지원범위 밖 writing mode를 추가 주장하지 않는다. |
| 18 | zero gap/margin/padding 누락으로 40px track 경계를 이동시킴 | root와 items의 spacing/border reset을 fixture/source 계약으로 고정했다. |
| 19 | track boundary sample이 DPR 변환 또는 경계 포함 규칙에 민감함 | Chrome DPR 1 CSS sample과 native origin/scale 환산을 별도로 저장하고 59/60px를 비교한다. |
| 20 | 규범 정정으로 일반 area 또는 전체 `fit-content()` 지원을 과장함 | 고정 단일 scene과 제한 typed subset만 주장하고 C11.3.3/C11/72/213은 미완료로 유지한다. |

CSS Grid 근거는 [Level 1 §7.1, §7.5–7.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)이다. 구현 결과는 계획 correction review가 아니라 별도의 [implementation/runtime review 20개](../c11-3-3-template-area-fit-content-cap-runtime-2026-10-31/implementation-runtime-review-20.md)에서 검토한다.
