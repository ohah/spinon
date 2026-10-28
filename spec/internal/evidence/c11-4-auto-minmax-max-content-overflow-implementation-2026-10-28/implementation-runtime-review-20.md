# C11.4 `minmax(auto, max-content)` 구현 검토 · 실패 관점 20개

| # | 독립 실패 관점 | 최종 코드 판정 |
|---:|---|---|
| 1 | 새 테스트 모듈이 `#[cfg(test)]` 경계를 벗어나 기본 library에 들어감 | 통과 — 기존 `grid_auto_min_scrollable_overflow_tests` 테스트 모듈의 하위로만 등록된다. |
| 2 | 기존 `minmax(auto, 1fr)` 14개 regression 입력이나 기대값이 바뀜 | 통과 — 기존 case 배열과 실행 경로는 수정하지 않고 새 child module만 추가했다. |
| 3 | 열/행 두 축 중 한 축이 빠지거나 overflow 값 하나가 누락됨 | 통과 — 10개 고정 case가 두 축 × `visible`, `clip`, `hidden`, `auto`, `scroll`을 전부 포함한다. |
| 4 | 같은 overflow를 두 축에 넣어 축별 속성 선택 오류를 숨김 | 통과 — Columns는 X만 바꾸고 Y는 `visible`; Rows는 Y만 바꾸고 X는 `visible`로 둔다. |
| 5 | `visible`과 `clip`의 automatic minimum 판정이 잘못됨 | 통과 — 양쪽은 content-based source/value를 기대한다. |
| 6 | `hidden`, `auto`, `scroll`이 0 minimum 대신 content-based 값으로 남음 | 통과 — 세 값 모두 `OverflowAutomaticMinimum` source와 0을 각각 검사한다. |
| 7 | Minimum trace 값만 검사하고 source 분류 오류를 허용함 | 통과 — Minimum 기여 trace는 정확히 하나이며 source enum도 확인한다. |
| 8 | auto minimum에 flexible maximum 전제 플래그가 잘못 붙음 | 통과 — 이 `max-content` track에서 `spans_flexible_max_track=false`를 요구한다. |
| 9 | 단일 track span이 여러 track 또는 비-auto minimum span으로 기록됨 | 통과 — content-based source에서 auto min track=true, multiple tracks=false를 검사한다. |
| 10 | `content_based` source flag가 false로 내려옴 | 통과 — `visible`/`clip` control에서 true를 검사한다. |
| 11 | scrollable minimum이 0이 되며 MaxContent 기여도도 0으로 잘못 감소함 | 통과 — 모든 Columns case의 MaxContent trace는 100px을 유지한다. |
| 12 | row 축의 MaxContent 기여도가 transferred size와 다름 | 통과 — 모든 Rows case의 MaxContent trace는 10px이다. |
| 13 | Minimum 및 MaxContent trace가 같은 종류로 조회되거나 합쳐짐 | 통과 — `Minimum`과 `MaxContent`를 독립 목록으로 정확히 하나씩 검사한다. |
| 14 | final track 값 대신 중간 stage 또는 gutter 값을 검사함 | 통과 — 마지막 stage에서 gutter/collapsed가 아닌 단일 track의 base size를 검사한다. |
| 15 | definite root size가 흐려져 track maximize 경계가 바뀜 | 통과 — Columns 60×20px, Rows 30×5px를 layout input과 frame에서 확인한다. |
| 16 | Ahem 글꼴 부재나 system font fallback이 100px 텍스트 기여도를 바꿈 | 통과 — system font 없는 engine에 고정 Ahem TTF를 등록하고 10 glyph 입력을 사용한다. |
| 17 | image aspect ratio가 row 10px 계산에 사용되지 않거나 다른 width에 매핑됨 | 통과 — 24×16 natural size, 15px CSS width, 10px frame/track을 함께 확인한다. |
| 18 | image가 5px row track으로 stretch되어 item overflow control이 사라짐 | 통과 — `align-self:flex-start`로 image frame을 15×10px로 유지한다. |
| 19 | item의 위치 또는 크기가 root/track과 다른데 크기 일부만 확인함 | 통과 — 두 축 root/item frame의 x/y/width/height를 검사한다. |
| 20 | pure layout unit 결과를 app/runtime, simulator 또는 전체 CSS 지원으로 확대함 | 통과 — 테스트는 Taffy layout unit 경로에만 있고 문서/PR에서 simulator 및 전체 지원을 주장하지 않는다. |

첫 focused compile에서 부모 `node` helper import 누락이 드러나 `use super` import를 보완했다. 최종 revision `2fdccabe8e77fc4bbf67bf59b0dabf309b4947eb`에서 focused 2/2와 package 297/297을 재실행해 통과했다. 구현/runtime 검토는 계획의 20개 검토와 별도로 작성했다.
