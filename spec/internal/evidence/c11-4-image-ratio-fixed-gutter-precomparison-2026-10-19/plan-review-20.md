# C11.4 fixed gutter 계획 검토 · 20개 실패 관점

대상: `plan/c11-4-image-ratio-fixed-gutter.md`, Chrome fixture, 사전 비교 README. 구현 전에 현재 main의 `spanned_fixed_track_limit()`과 W3C CSS Grid Level 1 §6.6을 함께 확인했다. 각 관점은 계획에서 필요한 별도 실행·계약 경계를 확인한다.

| # | 실패 관점 | 계획의 판정 기준 | 결과 |
| ---: | --- | --- | --- |
| 1 | 고정 max 함수와 단순 fixed min 함수가 혼동되는가 | 각 track의 max sizing function만 합산한다. | PASS |
| 2 | `minmax(auto, fixed)`가 fixed max로 인식되는가 | 3px max track의 실제 computed form과 unit matrix를 확인한다. | PASS |
| 3 | item width/height를 ratio 변환 전 잘못 clamp하는가 | source axis constraints → fixed grid-area cap → ratio → target constraints 순서를 trace로 검증한다. | PASS |
| 4 | gap을 잊거나 한 번 더 더하는가 | span N track일 때 N−1 내부 gutter만 합산하고 gap이 layout frame에 이미 적용되는 경계를 확인한다. | PASS |
| 5 | span 바깥 gutter가 limit에 포함되는가 | middle-span case에서 외부 track/gutter를 크게 하고 결과는 내부 3+4+3만 사용한다. | PASS |
| 6 | 여러 내부 gutter 중 마지막/첫 gutter가 빠지는가 | 3 track·2 gutter 케이스에서 3+4+5+3+3 합을 확인한다. | PASS |
| 7 | column만 고치고 row 축은 누락되는가 | row/column 대칭 fixture와 양 Simulator 실행을 요구한다. | PASS |
| 8 | single track 회귀가 바뀌는가 | PR #354의 single-track no-gutter 수치 행렬을 포함한 regression을 요구한다. | PASS |
| 9 | `fit-content()` argument를 fixed max로 잘못 취급하는가 | fit-content negative control은 clamp를 만들지 않는지 검증한다. | PASS |
| 10 | `fr`가 fixed로 오판되거나 auto minimum과 혼합되는가 | flexible max control은 all-fixed predicate와 automatic-minimum 조건을 별도 검사한다. | PASS |
| 11 | content-box에서 padding/border가 두 번 차감되는가 | 반대 축 edge를 한 번 빼고 ratio 후 target 축 edge를 한 번 더하는 값표를 둔다. | PASS |
| 12 | border-box item의 area limit이 padding 값 때문에 낮아지는가 | border-box control은 track+gutter size 그대로 적용한다. | PASS |
| 13 | image natural ratio와 authored ratio를 혼동하는가 | 사전 fixture는 natural 24×16과 authored 3/2를 기록한다. 구현 review에서 두 입력 경로를 확인한다. | PASS |
| 14 | Chrome 값을 규범적 oracle로 사용해 상호 운용 차이를 숨기는가 | 표준 기대와 Chrome 155 실제 관찰을 별도 열로 기록하며 일치를 요구하지 않는다. | PASS |
| 15 | gap percentage/calc의 basis를 임의 선택하는가 | 이 PR은 fixed px gap만 다루고 percentage/calc를 명시적으로 남긴다. | PASS |
| 16 | vertical writing mode의 logical/physical 축을 잘못 일반화하는가 | 이 PR은 기존 horizontal writing mode profile만 판정하고 vertical writing mode는 남긴다. | PASS |
| 17 | 음수·NaN·무한대 합이 정상 size로 변환되는가 | non-finite/overflow 합은 typed error로 전파하고 partial layout이 없어야 한다. | PASS |
| 18 | 10,000-track resource limit을 우회하는가 | 기존 track limit과 output atomicity를 회귀 실행하고 span iteration을 이 한도 안에 둔다. | PASS |
| 19 | §11.5 limited min/max-content span limit을 §6.6과 합쳐 버리는가 | `spanned_track_limit()`은 별도 규칙으로 유지하고 기존 C11.4 span regression을 실행한다. | PASS |
| 20 | 내부 fixture 변경이 product ABI나 상위 C11 완료를 뜻하는가 | opt-in fixture feature를 재확인하고 C11.4/C11 상태와 완료 수를 그대로 둔다. | PASS |

계획 검토 범위의 누락은 반영 후 재검토했다. 이 검토는 구현 코드나 simulator 동작을 검증하지 않는다.
