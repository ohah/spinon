# C11.1 Root HTML Grid auto-repeat 구현·runtime 적대 검토 20개

검토 범위는 이 제한 후보의 구현 diff, fixed Chrome 156 oracle, Rust runtime 및 Android/iOS 기본 앱 시뮬레이터 실행이다. 각 항목은 별도의 입력·불변 조건을 공격한다. 코드 경계 검토와 실제 실행 검증을 구분했다. 전체 CSS Grid, C11.1/C11 완료 또는 WPT 적합성을 승인하지 않는다.

| # | 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | `auto-fill`과 `auto-fit`이 같은 개수 규칙·collapse 동작으로 잘못 합쳐지는가? | typed repeat mode를 보존하고 13-scene Chrome/runtime 비교에서 두 mode 및 빈/collapsed track 관측을 대조했다. 통과. |
| 2 | 한 grid에서 columns와 rows에 동시에 auto-repeat을 허용하는가? | cascade grammar와 layout contract가 양 축 동시 repeat를 거부한다. negative runtime matrix 통과. |
| 3 | repeat 앞·뒤의 fixed track 또는 fixed-repeat neighbor가 silently 받아들여지는가? | 전체 axis track list를 repeat 하나로 제한한다. neighbor negative case 통과. |
| 4 | repeat pattern에 0개 또는 3개 이상 track을 넣어 count·line metadata를 어긋나게 하는가? | pattern 길이 1–2와 exact start/end를 cascade, projection, layout에서 검사한다. 3-track negative case 통과. |
| 5 | named line이나 repeat 내부 line name이 expanded track과 다른 line에 붙는가? | authored/repeat line-name group을 모두 거부한다. 두 negative cases 통과. |
| 6 | 0, 음수 또는 non-finite minimum이 repeat 분모로 들어가는가? | positive finite px minimum만 허용한다. count overflow/invalid input tests와 grammar negative test 통과. |
| 7 | `2fr` 또는 비-flex maximum이 제한 adapter를 통과하는가? | `minmax(positive px, 1fr)`만 admission한다. 2fr negative runtime case 통과. |
| 8 | UA 1px minimum floor가 실제 track sizing까지 적용되어 subpixel authored minimum을 바꾸는가? | floor는 count 함수에만 적용하고 원래 minimum을 Taffy에 전달한다. subpixel count tests 통과. |
| 9 | 169/170px 경계 또는 gap 포함 count가 1열 어긋나는가? | 1-track 경계와 2-track·gutter 산식을 독립 테스트하고 Chrome 13-scene 비교를 통과했다. |
| 10 | count 중간 합산·곱셈 overflow, Infinity 또는 0 denominator가 allocation으로 번지는가? | checked arithmetic와 finite guards로 `None`/fail-closed 처리한다. overflow tests 통과. |
| 11 | 10,000 track 초과 요청이 과도한 allocation이나 Taffy 범위 위반을 일으키는가? | expanded count를 `GRID_MAX_TRACKS`로 제한한다. 상한 test 통과. |
| 12 | 반복축 auto/percentage/zero size가 불명확한 basis로 count를 계산하는가? | definite positive finite CSS px 크기 및 미지원 math 부재를 검사한다. indefinite runtime case 통과. |
| 13 | 반복축 min/max override가 count basis와 used size를 분리하는가? | `min-size:auto`, `max-size:none`만 허용한다. min/max negative cases 통과. |
| 14 | 반대 축의 여러 track·repeat·line name이 grid sizing을 바꾸는가? | 이름 없는 fixed px track 하나만 허용한다. rows/columns native fixture로 각각 실행했다. |
| 15 | percentage/math/negative gutter 또는 box edge가 content basis와 cell frame을 밀어내는가? | finite non-negative px gutter, 0 margin/padding/border profile로 제한한다. percentage-gap과 non-zero edge negative cases 통과. |
| 16 | non-default alignment, overflow, RTL 또는 dense flow가 셀 좌표/순서를 바꾸는가? | 기본/start alignment, visible overflow, LTR, row sparse flow만 허용한다. 5종 negative cases 통과. |
| 17 | implicit track sizing 또는 explicit placement/span이 조용히 추가 track을 만드는가? | implicit rows/columns auto 및 auto-placement만 허용하고 numeric placement를 거부한다. negative cases 통과. |
| 18 | item 수가 확장된 explicit cells를 넘어가면 partial grid가 publish되는가? | 확장 capacity를 checked 계산하고 초과 시 실패시킨다. capacity-overflow runtime case 통과. |
| 19 | `display:none`, `order`, positioned item, aspect ratio 또는 self-alignment가 occupancy를 바꾸는가? | hidden subtree는 occupancy에서 제외하며, 나머지 style은 거부한다. 13-scene positive matrix와 item-style negative cases 통과. |
| 20 | Rust fixture만 통과하고 기본 앱 route의 row repeat 또는 화면 표시가 빠지는가? | native fixture가 column 1-track, column 2-track, row repeat를 검증한다. Android/iOS default route 모두 `layout=ready boxes=14`, `draw/present boxes=14`; OS screenshot은 visual smoke만 확인한다. |

세부 실행 결과, simulator 범위와 artifact hash는 [runtime evidence](README.md)에 기록했다. 별도의 candidate diff review는 [20개 공격 관점](pr-diff-review-20.md)에 기록했으며, PR 생성 후 GitHub diff와의 동일성을 확인한다.
