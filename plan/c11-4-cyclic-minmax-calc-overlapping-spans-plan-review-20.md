# `minmax(calc())` 겹치는 span 계획 실패 관점 검토 20개

2026-10-22에 고정 HTML, Agent Browser 관찰, 독립 계산, 계획 판정 기준을 검토했다. 이 문서는 계획 전용 검토이며 runtime 구현이나 PR diff 검토가 아니다.

| # | 별도 실패 경로 | 계획에서 확인한 방어 |
|---:|---|---|
| 1 | Chrome 결과를 규범 기대값으로 복사 | Ahem advance와 §7.2.1·§11.5.1에서 기여·분배를 독립 계산하고 browser 값을 별도 JSON에 둔다. |
| 2 | border-box 너비를 cyclic percentage basis로 사용 | `width`는 content-box 입력이며 border 2px씩은 root border box에만 더한다. |
| 3 | 세 track의 calc minimum을 intrinsic root size에 바로 더함 | cyclic percentage가 intrinsic container sizing에서 auto처럼 처리된 뒤 final size로 resolve되는 단계를 분리한다. |
| 4 | column gap을 span contribution에서 누락하거나 두 번 뺌 | span-2는 내부 gutter 2px 하나, span-3은 2px 두 개를 명시한다. |
| 5 | 같은 span-2 그룹을 DOM 순서대로 누적 | §11.5.1의 track별 planned increase와 forward/reverse controls를 기준으로 한다. |
| 6 | 두 번째 span-2 item이 겹치는 가운데 track을 더해야 할 값으로 누적 | 가운데 track에는 item별 계획의 최대 `[34,39,39]`를 기대한다. |
| 7 | span-3 contribution을 span-2 그룹보다 먼저 계산 | 다른 span 길이의 증가 순서를 `[34,39,39]` 이후 `+8`씩으로 계산한다. |
| 8 | span line indexing에서 끝 line을 off-by-one으로 계산 | span-2는 `1/3`, `2/4`; span-3은 `1/4`로 고정하고 used column list를 확인한다. |
| 9 | Ahem 대신 fallback font가 로드되어 glyph 폭이 달라짐 | browser evidence에서 `document.fonts.status`와 Ahem loaded 검사를 보존하고 font hash를 고정한다. |
| 10 | 공백 wrapping으로 min-content와 max-content가 달라짐 | 모든 본문을 unbreakable Ahem 글자 run으로 두고 `white-space:nowrap`을 allowlist한다. |
| 11 | grid item automatic minimum이 track span 값을 바꿈 | `min-width:0`·`min-height:0`를 입력에 고정하고 contribution source를 별도로 검사한다. |
| 12 | 세 item의 row sizing이 column 결과에 영향을 줌 | 모든 item을 row 1에 고정하고 row track은 고정 10px이다. |
| 13 | forward/reverse 비교가 placement도 바꿔 버림 | DOM 순서만 뒤집고 각 kind의 numeric grid-column은 유지한다. |
| 14 | definite 140px control이 cyclic root와 다른 basis를 사용 | 같은 140px content width와 동일 track expression으로 대응시킨다. |
| 15 | definite 100px control에서 max-content upper size를 최종 base로 강제 | available width100, calc minimum12, 균등 남은 60px 분배의 `[32,32,32]`를 요구한다. |
| 16 | item frame을 text intrinsic width로 오해 | `span3`의 100px frame과 140px `scrollWidth`를 별도 관찰한다. |
| 17 | overflow 지점을 border 안이나 글자 바깥에 잘못 선택 | root-relative `(126,7)`은 Ahem text 안, `(150,7)`은 그 뒤 surface clear다. |
| 18 | browser page white를 app native host clear로 사용 | Chrome page 배경과 Android/iOS native surface sample을 별도 환경으로 기록한다. |
| 19 | browser screenshot의 border stroke를 앱 border 지원으로 해석 | screenshot은 oracle evidence이며 CSS border paint는 acceptance에서 제외한다. |
| 20 | fixed fixture 통과를 일반 CSS Grid 지원이나 C11.4 완료로 승격 | six scene allowlist, default/fixture artifact 분리, `77/213`과 C11.4 미완료 상태를 유지한다. |

## 판정

20개 항목이 서로 다른 입력·규범 계산·layout·pixel boundary·계약 과장을 다루도록 계획에 반영했다. 미해결 항목은 구현 전에 oracle와 계획을 다시 수정해야 한다.
