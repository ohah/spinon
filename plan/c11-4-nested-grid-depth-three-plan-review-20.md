# C11.4 depth-three nested Grid 계획 실패 관점 검토 20개

| # | 독립 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | “depth 3”가 Grid 개수가 아니라 세 중첩 경계로 해석되어 장면이 모호한가? | 계획은 root → middle → branch → inner, 네 Grid와 세 Grid-to-Grid 경계를 별도로 정의했다. 통과. |
| 2 | 실제 fixture에 Grid 단계가 네 개보다 적거나 많아도 검사가 같은 이름으로 통과하는가? | 두 scene의 기대 Grid 개수는 4개이며 구현 검사에 exact node identity/order를 요구한다. 통과. |
| 3 | min-content만 확인해 max-content contribution 전달을 놓치는가? | 새 깊이에서 min/max 양 축 scene 두 개를 따로 고정했다. 통과. |
| 4 | width만 intrinsic으로 두고 직전 slice를 반복하는가? | 각 Grid의 width/height와 column/row track에 동일 intrinsic keyword를 적용한다. 통과. |
| 5 | depth 효과를 비교할 기준이 없는가? | 기존 depth-one과 depth-two의 양 축 여덟 scene을 회귀 control로 유지한다. 통과. |
| 6 | 독립 기대값을 Chrome 값에서 역으로 만들었는가? | Ahem advance와 CSS Grid CRD sizing 순서로 규범값을 계산하고 Chrome observation을 별도 JSON에 둔다. 통과. |
| 7 | Ahem 미로딩 fallback이 유효한 oracle처럼 저장되는가? | capture는 document.fonts.ready를 기다리고 ahemReady를 기록하며 false를 실패 gate로 취급한다. 통과. |
| 8 | min-content가 문자열 전체 advance인 100px로 잘못 계산되는가? | 공백 없는 가장 긴 조각 XXXXXXX만으로 70px을 도출했다. 통과. |
| 9 | max-content에서 공백 advance를 빼 90px로 계산하는가? | 두 글자, 공백, 일곱 글자의 총 advance 100px을 고정했다. 통과. |
| 10 | row 기대가 column 결과와 연결되지 않는가? | columns-before-rows 절차와 70px wrap 두 줄/100px 한 줄을 기대값 근거에 연결했다. 통과. |
| 11 | intrinsic row height가 text line-height와 무관한 UA 기본값에 좌우되는가? | leaf는 Ahem 10px font와 10px line-height를 사용한다. 통과. |
| 12 | nested Grid item 자체의 기본 margin/padding/border가 재귀 단계마다 누적되는가? | 각 Grid와 leaf의 margin, padding, border를 0으로 고정하고 추가 min/max를 배제했다. 통과. |
| 13 | Grid gutter가 기댓값에 숨겨져 intrinsic contribution의 재귀 효과를 혼합하는가? | 모든 Grid gap을 0으로 두고 track은 축별 하나만 사용한다. 통과. |
| 14 | 각 부모에 실제 in-flow child 하나가 아닌 anonymous text나 두 child가 섞이는가? | 각 Grid-to-Grid parent는 direct child 한 개, inner Grid는 text leaf 한 개이며 다른 fixture 입력을 섞지 않는다. 통과. |
| 15 | style의 align/placement 때문에 track 외부의 정렬 공간이 frame에 반영되는가? | start alignment를 고정하고 intrinsic single track에 item 하나만 배치한다. 통과. |
| 16 | nested level이 잘못된 owner 아래 놓여도 같은 IDs만 확인하고 통과하는가? | 각 node의 parent ID, exact child order와 root-relative frame을 검사할 것을 요구한다. 통과. |
| 17 | 서로 다른 document/style revision에서 Grid와 text 값이 섞이는가? | source revision을 모든 contribution trace와 함께 확인한다. 통과. |
| 18 | depth-four나 잘못된 scene이 partial layout/trace를 반환하는가? | 미등록 scene, depth-four, 추가 child와 잘못된 intrinsic 조합의 fail-closed negative control을 명시했다. 통과. |
| 19 | fixture-test API가 일반 제품 CSS 지원처럼 노출되거나 기본 archive에 들어가는가? | opt-in fixture adapter만 허용하고 product/fixture archive 양쪽 artifact boundary를 검사한다. 통과. |
| 20 | Chrome screenshot·simulator draw를 glyph parity 또는 C11.4 완료 근거로 과장하는가? | screenshot은 보조자료, WPT·glyph paint·physical Android 미검증, C11.4 72/213 유지로 구분했다. 통과. |

## 판정

depth-three topology, 양 축 intrinsic 전달, 독립 기대값, 정확한 fixture 경계, negative controls와 완료 범위를 서로 다른 실패 경로로 검토했다. 계획 구현 전 비교 모델이 고정됐다. 이 표는 이후 구현/runtime 실패 관점 20개를 대신하지 않는다.
