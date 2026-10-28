# C11.4 `minmax(auto, max-content)` overflow 계획 · 실패 관점 20개

| # | 실패 경로 | 판정 |
|---:|---|---|
| 1 | CSS Grid automatic minimum 조건을 잘못 인용함 | 통과 — §6.6의 computed overflow, `auto` 최소 track, multi-span flexible-track 조건을 적었다. |
| 2 | `overflow: clip`을 scrollable 값과 묶음 | 통과 — `visible`과 함께 non-scrollable control로 분리한다. |
| 3 | `hidden`, `auto`, `scroll`이 content-based minimum을 유지함 | 통과 — 두 축에서 각각 독립 case로 두고 0을 기대한다. |
| 4 | track base와 item minimum contribution을 혼동함 | 통과 — trace source/value와 최종 used track을 모두 검사한다. |
| 5 | `max-content` maximum을 실제로 검사하지 않음 | 통과 — 모든 case에 해당 maximum을 쓰고 그 contribution을 확인한다. |
| 6 | minimum이 0이면 max-content growth limit도 사라진다고 가정함 | 통과 — max-content contribution을 100px/10px으로 유지하고 definite available space까지만 track을 키운다. |
| 7 | Columns 기대값이 Chrome 관측에서 복사됨 | 통과 — Ahem intrinsic width와 60px definite limit을 독립 계산한다. |
| 8 | Rows intrinsic size가 aspect ratio 변경으로 달라짐 | 통과 — natural 24×16 비율과 CSS width 15px에서 transferred row size 10px을 고정한다. |
| 9 | Row image stretch가 5px track 밖의 item overflow를 숨김 | 통과 — `align-self:start`로 15×10px frame을 유지한다. |
| 10 | Rows 기대값이 text layout에 의존함 | 통과 — replaced-image snapshot 경로를 쓰고 text height gate는 범위 밖으로 명시한다. |
| 11 | overflow가 Grid item이 아니라 root에 지정됨 | 통과 — 각 matrix 값은 item에 지정하고 computed 값을 관측한다. |
| 12 | root 크기가 indefinite가 되어 max-content 동작이 바뀜 | 통과 — 두 축의 root 크기를 고정하고 `expected.json`에 기록한다. |
| 13 | item style의 명시 min-size가 automatic minimum을 덮음 | 통과 — 관련 minimum은 `auto`로 두고 computed minimum을 기록한다. |
| 14 | scroll width를 item border-box width로 오해함 | 통과 — used track/item frame을 기대값으로 삼고 scrollSize는 진단값으로만 둔다. |
| 15 | computed style이 선언 track과 다름을 놓침 | 통과 — 원 Chrome observation에 computed track, root/item rect를 함께 보존한다. |
| 16 | 폰트 또는 이미지가 준비되기 전에 관측함 | 통과 — Ahem 준비를 확인하고 모든 image decode 완료 후 수집한다. |
| 17 | 페이지 좌표를 root-relative frame으로 비교함 | 통과 — 원 관측에서 두 좌표계를 분리한다. |
| 18 | screenshot을 Spinon render로 표현함 | 통과 — evidence와 PR 자료에 Chrome browser baseline이라고 표시한다. |
| 19 | pure layout unit test를 앱/simulator 근거로 확대함 | 통과 — 앱 경로 변경이 없어 simulator 결과를 주장하지 않는다. |
| 20 | 제한 matrix를 C11.4 또는 공식 집계 완료로 사용함 | 통과 — 계획과 상위 상태에서 C11.4와 `72/213`을 미완료로 유지한다. |

고정된 입력에서 규범 계산과 기대값 사이의 모순은 발견되지 않았다. Chrome 관측은 독립 계산을 대체하지 않는 비교 자료다.
