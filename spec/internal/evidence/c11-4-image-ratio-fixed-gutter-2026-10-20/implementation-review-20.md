# C11.4 fixed gutter 구현 적대 검토 · 20개 실패 관점

**대상:** definite preferred-size suggestion의 fixed-grid-area clamp, track/gutter span 합산, direct HTML `img` fixture, Android·iOS simulator 검증 경로. 계획 검토 표와 PR 검토 표를 재사용하지 않았다.

| # | 실패 관점 | 확인한 경계와 근거 | 결과 |
| ---: | --- | --- | --- |
| 1 | definite preferred size가 조기 반환하며 §6.6 cap을 건너뛰는가 | `GridItem::minimum_contribution()`에서 author min/max 적용 뒤 all-fixed area limit을 적용한다. | PASS |
| 2 | author min/max와 area cap의 순서가 뒤바뀌는가 | owner 2712의 preferred 36, min 20, max 30, area 10은 최종 preferred contribution 10을 검증한다. | PASS |
| 3 | span 경계 밖의 선·gutter가 area limit에 포함되는가 | `track_range_excluding_lines()`가 placement 인덱스의 양쪽 경계 line을 제외한다. | PASS |
| 4 | 내부 gutter가 span 계산에서 빠지는가 | `GridTrackVec`의 gutter도 같은 span range에 있으며 `GridTrackKind::Gutter`로 따로 기록된다. | PASS |
| 5 | 기존 span range에 gutter를 다시 더해 중복 계산하는가 | 3px track + 3px gutter + 4px track의 합이 10px로 검증된다. | PASS |
| 6 | 3-track span에서 두 내부 gutter 중 하나를 놓치는가 | track 합 12px와 gutter 합 6px을 분리해 총 18px을 검사한다. | PASS |
| 7 | middle span 밖의 큰 track이 합산되는가 | 바깥 80px·90px track을 둔 사례에서 가운데 3px + 3px gutter + 4px만 합산한다. | PASS |
| 8 | gap 0에서 존재하지 않는 gutter가 더해지는가 | zero-gap control의 track 합과 limit이 모두 7px, gutter 합은 0px이다. | PASS |
| 9 | rows와 columns에서 축별 gap을 혼동하는가 | row/column 장면이 서로 다른 row-gap·column-gap 입력과 기대 합을 사용한다. | PASS |
| 10 | 2-track과 3-track span의 단일 gutter 결과만 우연히 맞는가 | 각 축에 2-track 및 3-track span을 두고 합을 별도로 검사한다. | PASS |
| 11 | inline→block ratio 변환이 곱셈으로 잘못되는가 | column preferred size의 clamp를 row transferred contribution에서 나눗셈으로 확인한다. | PASS |
| 12 | block→inline ratio 변환이 나눗셈으로 잘못되는가 | row preferred size의 clamp를 column transferred contribution에서 곱셈으로 확인한다. | PASS |
| 13 | `fr` max track이 fixed limit으로 오인되는가 | flexible max track control은 limit과 trace 구성요소를 모두 `None`으로 기대한다. | PASS |
| 14 | `fit-content()` 인수가 fixed max처럼 사용되는가 | `fit-content(3px)` control은 fixed limit을 만들지 않는다. | PASS |
| 15 | indefinite/non-finite track 값이 유효한 cap으로 남는가 | helper는 `definite_value()` 실패와 non-finite 값을 모두 거부한다. | PASS |
| 16 | f32 합산 overflow가 무한대 또는 잘린 limit으로 변하는가 | 2e38 track 두 개와 2e38 gutter의 합은 f64 trace로 유지하고 f32 cap은 생략하며 최종 frame 유한성을 검사한다. | PASS |
| 17 | content-box edge를 source/target 축에서 이중 차감하는가 | owner 2711은 source 10px border-box limit에서 4px edge를 한 번 차감하고 target edge를 한 번 더한다. | PASS |
| 18 | border-box item에 edge 보정이 잘못 적용되는가 | edge가 없는 border-box controls는 10px limit과 ratio frame을 그대로 검사한다. | PASS |
| 19 | native fixture의 scene allowlist·frame·trace·pixel 표본이 서로 다른 입력을 검증하는가 | 같은 scene ID별 root/item frame, trace 구성요소, Android PixelCopy·iOS Metal 4/4 표본을 연결했다. | PASS |
| 20 | 제한 fixture 결과를 C11.4 전체 지원으로 과장하는가 | 구현은 fixed-pixel span 두 장면으로 제한하고 전체 C11.4 체크와 완료 수를 유지한다. | PASS |

**범위 한계:** percentage/calc track, implicit placement, vertical writing mode, nested·anonymous item, WPT 적합성, compositor 이후 화면 및 실기기는 이 slice의 테스트에 포함되지 않는다. Android API 37 AVD와 iOS 26.2 Simulator 결과만 근거로 삼는다. Chrome 관찰값은 규범 기대값을 대체하지 않는다.

**결함:** 이 20개 구현 관점에서 추가 결함을 찾지 못했다. 계획 단계에서 확인한 preferred-size 조기 반환 결함은 구현과 focused tests에서 별도로 다뤘다.
