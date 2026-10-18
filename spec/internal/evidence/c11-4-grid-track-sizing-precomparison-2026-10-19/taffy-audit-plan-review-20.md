# C11.4 Taffy 감사 반영 계획 검토 · 20개 독립 관점

**검토 대상:** `plan/c11-4-grid-track-sizing.md`의 A/B 단계 경계와 `taffy-0.14-automatic-minimum-audit.md`.
**결론:** 계획은 자동 최소를 item별 placed span에서 계산하고 Taffy 후보 결과를 독립 판정에 종속시킨다. 아래 구현 controls가 구현 전 oracle에 추가되기 전에는 B단계 적합성을 주장할 수 없다.

| # | 독립 실패 관점 | 검토 결과와 필요한 gate |
| ---: | --- | --- |
| 1 | span 밖의 `auto`가 최소 크기를 오염 | span 밖 auto track 유무만 바꾼 쌍에서 item의 automatic minimum 및 track 결과가 같아야 한다. |
| 2 | span 안의 `auto`가 무시됨 | item span 안 auto-min track 하나에서 content-based minimum 적용 여부와 source를 기록한다. |
| 3 | span 밖의 `fr`가 최소 크기를 0으로 만듦 | span 밖 flex track 유무만 바꾼 쌍에서 최소 기여가 불변이어야 한다. |
| 4 | span 안의 flex 예외 누락 | multi-track span 안에 auto-min과 flex track이 함께 있는 경우 automatic minimum 예외를 검증한다. |
| 5 | 한 track span에서 flex 예외를 과잉 적용 | item이 auto-min track 하나만 차지할 때 다른 곳의 flex track은 eligibility에 영향을 주지 않아야 한다. |
| 6 | `minmax(auto, 1fr)`의 minimum 함수 오해 | max function이 flex여도 실제 min function의 auto 여부 및 span 예외가 구분되어야 한다. |
| 7 | `minmax(min-content, auto)`를 auto minimum으로 오인 | track의 min 함수와 max 함수, item automatic minimum을 별도 입력·출력으로 검사한다. |
| 8 | specified size suggestion 유실 | definite preferred size가 있을 때 min-content보다 우선하는 suggestion과 관련 축을 기록한다. |
| 9 | transferred size suggestion 비대체 이미지에 적용 | aspect ratio와 반대축 definite size가 없는 이미지에서는 transferred suggestion을 만들지 않는다. |
| 10 | aspect ratio 변환 방향 오류 | natural ratio 및 CSS ratio의 width→height/height→width 계산과 min/max clamp를 양 축에서 확인한다. |
| 11 | content size suggestion이 max-content로 대체 | automatic minimum의 content suggestion은 min-content 입력을 사용하고 image natural size 경계를 별도 기록한다. |
| 12 | max-size 제약 누락 | definite `max-width`/`max-height`가 자동 최소 suggestion을 clamp하는 축과 값이 정확해야 한다. |
| 13 | indefinite percentage를 definite로 오해 | opposite-axis min/max percentage가 indefinite일 때 normative zero treatment와 cyclic final layout을 분리한다. |
| 14 | fixed max track clamp를 fit-content에 적용 | fixed max track 함수 합과 intervening gutter만 적용하며 `fit-content()` limit은 item automatic minimum clamp로 재사용하지 않는다. |
| 15 | padding/border 또는 box-sizing 중복 반영 | content-box/border-box 모두 content contribution과 outer contribution에 edge가 한 번만 더해져야 한다. |
| 16 | gap이 span 기여에 중복 포함 | spanning item contribution과 track free space에서 gutter가 각각 어느 단계에 들어가는지 수치로 검사한다. |
| 17 | row·column 축 혼합 | 두 축에서 track span, ratio transfer, preferred/min/max constraints가 다른 축 입력을 참조하지 않아야 한다. |
| 18 | hidden·empty·positioned item이 계산에 유입 | 계산 대상 집합과 배치 후 span이 `display:none`, empty 및 positioned item 경계에서 명세와 일치해야 한다. |
| 19 | callback 측정 오류가 0 기여로 은폐 | text/image measure error, invalid ratio, overflow 미지원에서 partial track/frame 없이 정확한 오류를 내야 한다. |
| 20 | Taffy differential을 독립 기대값으로 오용 | W3C rule과 고정 Chrome oracle을 먼저 판정하고, Taffy 차이는 결과를 숨기지 않고 기록한다. |

## 검토 한계

이 검토는 계획 경계를 읽고 필요한 반례와 판정 조건을 확인한 문서 검토다. Rust 구현, Taffy 결과, Chrome 추가 control, WPT, Android/iOS Simulator를 실행하지 않았다. 계획의 기존 20개 검토나 향후 구현 후 20개 검토를 대체하지 않는다.
