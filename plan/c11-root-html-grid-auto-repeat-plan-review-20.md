# Parsed Root HTML Grid auto-repeat 계획 실패 관점 검토 20개 · 2026-11-05

대상은 [`c11-root-html-grid-auto-repeat.md`](./c11-root-html-grid-auto-repeat.md) 및 구현 전 고정한 [Chrome 156 비교](../spec/internal/evidence/c11-root-html-grid-auto-repeat-precomparison-2026-11-05/README.md)다. 아래는 서로 다른 실패 벡터를 적용해 계획 범위를 검토한 기록이다. 이 검토는 구현 코드/runtime 검토나 구현 PR diff 검토를 대체하지 않는다.

| # | 실패 관점 | 계획에서 요구한 방어와 판정 |
|---:|---|---|
| 1 | `auto-fill` count가 169/170px 경계에서 1회 또는 2회로 어긋남 | 양쪽 축 경계 fixture와 exact expanded-track 비교를 포함한다. 통과 기준에 명시함. |
| 2 | pattern 내부 gutter와 반복 사이 gutter를 count 산식에서 한 번 누락 | 40px+30px minimum, 10px gap의 규범 유도값과 Chrome 관찰을 함께 고정했다. 구현은 Taffy count 경계와 별도 대조해야 한다. |
| 3 | `fr` free space를 min base 위에 나눠 첫 oracle처럼 잘못 계산 | 초기 실패 기대 파일을 보존하고 §11.7.1의 freeze/restart 규칙으로 corrected oracle을 고정했다. |
| 4 | `auto-fill`이 비점유 repeated track을 collapse | empty pattern track까지 computed track list에서 비교한다. auto-fill positive 장면에는 비점유 반복분이 남는다. |
| 5 | `auto-fit`이 repeat count 자체를 줄이거나 empty gutter를 남김 | occupied/empty cases에서 repeat count와 0px collapsed tracks/gutters를 각각 비교한다. |
| 6 | item이 하나도 없는 `auto-fit`에서 모든 repeated track을 collapse하지 않음 | columns·rows empty-grid 장면을 fixture에 추가했다. |
| 7 | `display:none` 아이템을 점유나 capacity에 포함 | hidden child case에서 computed `display`, zero frame, 첫 visible item의 200px sizing을 대조한다. |
| 8 | authored pattern length만 세어 item capacity가 확장 수보다 작아져 유효 입력을 거부 | plan은 expanded count 기반 cell capacity 계산과 2-item positive cases를 요구한다. |
| 9 | capacity 계산이 부족해 implicit row/column을 허용 | explicit capacity를 넘는 item count는 배치 전에 fail-closed하도록 계약에 명시한다. |
| 10 | subpixel minimum이 count 계산에서 UA 1px floor를 받지 않거나 그 floor가 실제 track sizing까지 바뀜 | 0.5px minimum·2px axis case로 repeat count 두 개와 actual 1px sizing을 고정하고, count-only floor와 track sizing을 구분한다. |
| 11 | repeat axis가 auto/percentage/calc라 definite content basis가 불명확 | definite positive px axis, auto min-size, none max-size, no aspect-ratio로 범위를 제한한다. |
| 12 | border-box/padding이 track-count basis와 projection content size를 다르게 만듦 | Grid root의 padding·border·margin을 0으로 고정하고 실제 definite basis와 frame을 대조한다. |
| 13 | percentage gap 또는 percentage minimum이 count의 cyclic basis를 만듦 | percentage/calc gaps와 percentage/intrinsic minima를 명시적으로 거부한다. |
| 14 | repeat 앞뒤 track 또는 line names가 Taffy insertion point/line mapping과 상호작용 | auto-repeat fragment를 authored list 전체로 제한하고 prefix/suffix/named lines/areas를 거부한다. |
| 15 | integer repeat, 두 auto-repeat, 또는 양 축 auto-repeat 혼합을 조용히 통과 | 한 축, 한 fragment만 허용한다. projection validator와 negative control에서 확인한다. |
| 16 | rows 계산에 width 또는 columns 계산에 height를 잘못 연결 | row/column 각 fixture에 서로 다른 definite axis와 track geometry를 고정한다. |
| 17 | column/dense flow와 implicit auto tracks가 다른 placement 경로를 탐 | default row auto-placement만 승인하고 그 밖의 flow와 implicit tracks는 거부한다. |
| 18 | visible overflow item을 root/track frame에 clamp 또는 clip | 170px root 밖 70px item overflow의 geometry와 simulator 화면을 확인한다. |
| 19 | profile 확대가 C11.4 opt-in path나 기본 artifact에 영향을 줌 | admission은 `RuntimeRootHtmlTextPaintV1` 후보 입력만 허용하고 C11.4 profile 회귀 및 기본 artifact 구성을 별도 확인한다. |
| 20 | 실패/할당 오류 뒤 일부 node frame이 scene으로 publish되거나 simulator가 다른 route를 검증 | unsupported/cap/allocation 실패의 no-partial-scene, 기본 Root HTML 앱 route와 Android/iOS Simulator 증거를 pass gate에 둔다. |

## 판정

계획에 정의된 subset으로 구현 전 진행 가능하다. 구현을 시작하기 전에는 이 계획 PR을 병합하고, 구현 시 Taffy count 식과 Root HTML capacity 계산의 경계 일치를 검증한다. 이 검토만으로 계획 병합이나 기능 구현이 자동 승인되는 것은 아니며 별도 PR diff review를 남긴다.
