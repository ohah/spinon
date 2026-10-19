# C11.4 fixed-span 계획 수정 검토 · 20개 실패 관점

**대상:** `plan/c11-4-image-ratio-fixed-gutter.md` 수정안, 고정 Chrome fixture 및 §6.6 기대값. 제품 계산 코드 수정 전 검토다. 기존 계획 PR의 검토 표는 재사용하지 않았다.

| # | 실패 관점 | 판정 기준 | 결과 |
| ---: | --- | --- | --- |
| 1 | fixed max span에서 specified size suggestion이 누락되는가 | §6.6 본문과 대조해 preferred-size 조기 반환도 grid-area maximum으로 제한하도록 요구한다. | PASS |
| 2 | 내부 gutter가 기존 span range에 포함되는가 | `GridTrackVec`, `track_range_excluding_lines()`, `GridTrack::gutter()`를 대조하고 중복 합산 금지를 명시한다. | PASS |
| 3 | gutter를 별도 재가산해 이중 적용하는가 | `3px + 3px gutter + 4px`를 `10px`로 유지하고 geometry 합산은 금지한다. | PASS |
| 4 | span 양 끝 grid line의 gutter가 포함되는가 | 두 track placement에서 내부 gutter 한 개만 range에 들어오는지 trace로 확인한다. | PASS |
| 5 | definite preferred-size 경로가 기존 fixed span limit을 건너뛰는가 | `width:36px` case의 baseline trace가 `PreferredSize / 36px`임을 확인했다. 구현 후 `10px`이어야 한다. | BASELINE RED 재현 |
| 6 | min/max size 적용 순서가 바뀌는가 | author min/max clamp 뒤 grid-area cap이 적용되며 definite 최대 크기 제한을 계속 존중한다. | 검증 필요 |
| 7 | content-box padding/border가 cap에서 이중 차감되는가 | source axis edge를 한 번 빼고 target axis edge를 한 번 복구한다. | 검증 필요 |
| 8 | border-box에서 item edge를 잘못 차감하는가 | border-box case는 grid-area border-box limit을 그대로 쓴다. | 검증 필요 |
| 9 | 반대 축 transferred-size clamp가 사라지거나 두 번 적용되는가 | 기존 ratio 변환 전 source-axis fixed limit과 기존 단일 track 회귀를 비교한다. | 검증 필요 |
| 10 | ratio 방향이 row/column에서 뒤바뀌는가 | columns→rows는 나눗셈, rows→columns는 곱셈 기대를 각 축 trace로 비교한다. | 검증 필요 |
| 11 | 3-track span에서 내부 gutter 하나가 빠지는가 | max 함수 3·4·5와 3px gutter 두 개 합 `18px`을 확인한다. | 검증 필요 |
| 12 | span 외부 track/gutter가 포함되는가 | 큰 외부 track을 둔 middle-span에서 내부 합 `10px`만 나온다. | 검증 필요 |
| 13 | zero-gap control이 fixed span 합을 바꾸는가 | gap `0px`에서 limit이 track max 합 `7px`이다. | 검증 필요 |
| 14 | `fr` maximum이 fixed predicate를 통과하는가 | flexible control은 limit을 생성하지 않는다. | 검증 필요 |
| 15 | `fit-content()` 인수가 fixed max로 오인되는가 | fit-content control은 limit을 생성하지 않는다. | 검증 필요 |
| 16 | Chrome item frame을 내부 suggestion 값으로 오해하는가 | frame과 intermediate contribution을 분리하고 Chrome 결과에서 transfer 값을 단정하지 않는다. | PASS |
| 17 | Chrome 관찰이 규범적 oracle을 대체하는가 | expected cap은 W3C §6.6에서 독립 도출하고 browser 표는 별도 열로 둔다. | PASS |
| 18 | trace가 limit 구성요소와 stage를 잃는가 | fixed track sum, gutter sum, combined limit, source contribution, stage를 동일 revision으로 기록한다. | 검증 필요 |
| 19 | overflow sum이 NaN/무한대 기여나 partial output으로 남는가 | f32 overflow 입력과 final output 유한성 및 typed `LayoutError` 경계를 재현한다. | 검증 필요 |
| 20 | 하위 작업 근거가 C11.4 전체 완료로 과장되는가 | 공식 C11.4 체크와 완료 수는 그대로 두고 Simulator·WPT·비대상 조합 범위를 분리한다. | PASS |

검토에서 gutter 누락이라는 최초 원인은 부정됐고, preferred-size 조기 반환 결함은 baseline RED로 재현됐다. 남은 검증 관점은 구현 검토와 별도로 처리한다. 이 표는 구현이나 PR diff 검토로 재사용하지 않는다.
