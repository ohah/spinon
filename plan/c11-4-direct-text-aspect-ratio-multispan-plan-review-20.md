# C11.4 direct-text aspect-ratio two-track span 계획 검토 20개

기준 계획: [`c11-4-direct-text-aspect-ratio-multispan.md`](./c11-4-direct-text-aspect-ratio-multispan.md)

사전 비교 기준: [`precomparison evidence`](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-multispan-precomparison-2026-10-30/README.md)

판정: 구현 계획으로 진행 가능. 아래에서 수정한 문제는 계획에 반영했다. 구현 전 검토이며 제품 변경이나 runtime 검증은 아니다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 100px root의 여유 폭이 두 intrinsic track을 불필요하게 늘리는가 | **통과.** Agent Browser에서 root는 100px로 유지됐고 columns 합은 기여값 40/45/60/30/35.5px에 따라 달라졌다. 독립 기대값도 resolved track 합을 비교한다. |
| 2 | Ahem load 실패가 다른 font의 glyph advance로 숨겨질 수 있는가 | **통과.** HTML이 local Ahem TTF를 고정하고 Agent Browser `document.fonts.check("10px Ahem")`가 true인 경우만 유효하다. |
| 3 | numeric ratio의 width/height 방향을 뒤집어도 기대값이 통과하는가 | **통과.** 명시적인 `2 / 1`에서 inline contribution 60px은 block 30px이며 Chrome 156과 독립 계산을 모두 기록했다. |
| 4 | CSSOM resolved pixel list를 authored track-size input으로 잘못 읽는가 | **수정.** 계획은 layout 전 Stylo typed `max-content max-content` snapshot을 확인하고 CSSOM의 post-layout pixel list와 구별하도록 명시했다. |
| 5 | `grid-column:1 / span 2`가 start/end 계산에서 다른 topology로 투영될 수 있는가 | **통과.** 정확히 column line 1에서 두 track까지 한정하고 named/negative/implicit line, 다른 span 길이와 auto placement를 거부한다. |
| 6 | 문서 절대 좌표가 독립 item frame 좌표로 오인되는가 | **수정.** expected JSON에 item 좌표가 Grid root 기준임을 명시했다. Browser의 absolute DOM 좌표는 root 좌표를 빼서 비교해야 한다. |
| 7 | 양쪽 column에 prior single-span contribution이 생겨 균등 분배 가정이 틀리는가 | **통과.** 고정 입력에는 span item만 하나 있고 다른 item이 없다. 따라서 두 같은 max-content tracks의 prior single-span size는 없다. |
| 8 | span item의 contribution 합에 gap/gutter를 빠뜨리는가 | **통과.** root gap은 0으로 고정한다. 비영 gap 입력은 이 계획에 포함하지 않는다. |
| 9 | 기본 `min-width:auto`를 0으로 취급해 max-height control이 잘못 줄어드는가 | **통과.** `max-height:15px` control은 track `[20,20]`, item `40×15`이며, 별도 `min-width:0` control은 `[15,15]`, `30×15`다. |
| 10 | min/max 축 변환에서 width/height를 혼동하는가 | **통과.** `min-height:30px`와 `max-width:30px` 장면을 각각 고정해 양방향 전이를 분리했다. |
| 11 | no-ratio control이 우연히 preferred ratio를 상속하는가 | **통과.** `span-no-ratio`는 authored/computed `aspect-ratio:auto`를 명시하고 40×10 track baseline을 분리한다. |
| 12 | 17.75px 경계가 정수 반올림 또는 f32 비교로 뭉개지는가 | **통과.** 독립값은 columns 17.75px씩, total/item width 35.5px이다. Runtime 허용오차는 계획 실행 전에 명시한다. |
| 13 | 30px item에 실제 40px text가 있어 overflow를 놓치는가 | **통과.** content Text range와 item frame을 따로 비교하며 max 사례에서 10px inline overflow를 고정했다. |
| 14 | 화면에서 보이지 않는 track seam을 pixel sample 성공으로 과장하는가 | **수정.** trace의 track 경계로 확인하고 단일 item의 연속 paint 때문에 seam 자체는 pixel assertion에서 제외하도록 바꿨다. |
| 15 | 기존 collector가 요구하는 AAAA+A 두 item profile에 단일 AAAA span을 억지로 끼워 넣는가 | **수정.** 별도 fixture profile/module로 분리하고 helper의 immutable input/owned output/NodeId contract를 편집 전에 고정했다. |
| 16 | 한 case의 세부 결과를 전체 Grid span 지원으로 확대하는가 | **통과.** 한 개 item, 두 동일 max-content columns, numeric 2:1과 제한 min/max로 한정하고 다른 topology는 제외했다. |
| 17 | scene ID나 DOM 순서가 실제 ratio owner를 대신하는가 | **통과.** 정확한 typed computed owner NodeId와 revision을 끝까지 전달하도록 수용 기준에 둔다. |
| 18 | unsupported percentage/calc/negative/non-finite min/max가 일부만 적용되는가 | **통과.** 0..=60px finite subset 밖 입력은 layout 전에 fail-closed이며 partial layout을 수용하지 않는다. |
| 19 | 새 span 경로가 기존 ratio/minmax scenes 또는 기본 artifact를 깨뜨리는가 | **통과.** 두 simulator에서 기존 42 scene과 새 9 scene을 합친 51/51 matrix를 요구하고 default/opt-in artifact 경계를 확인한다. |
| 20 | 계획 PR의 Chrome 관측이 simulator, WPT 또는 공식 C11 완료로 과장되는가 | **통과.** 사전 비교를 Chrome oracle로만 분류하고 plan/runtime/official completion의 상태를 별도 기술한다. |

Chrome 값은 독립 기대값을 고정한 뒤 대조했으며, 이 문서의 PASS는 계획 품질을 뜻한다. 아직 Spinon 구현 결과를 뜻하지 않는다.
