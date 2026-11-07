# C11.4 `safe|unsafe flex-start` 구현·RuntimeSession 실패 관점 검토 20개

검토 대상은 computed-style projection, `RootHtmlAutoRowStretchV1` gate, Taffy layout, RuntimeSession publication/recovery와 두 Simulator 관측이다. 검토는 새 구현 후보에서 수행했다.

| # | 실패 관점 | 확인한 계약·검사 | 결과 |
|---:|---|---|---|
| 1 | 안전 modifier 없이 `flex-start`만 허용될 수 있음 | style allowlist 변경은 exact `safe flex-start`와 `unsafe flex-start`만 추가 | 이상 없음 |
| 2 | computed style에서 safety modifier가 유실될 수 있음 | projection matrix가 `align-content` 전체 문자열의 exact serialization 검사 | 이상 없음 |
| 3 | enum mapping이 `flex-start`를 `start`와 다르게 배치할 수 있음 | Layout matrix와 독립 Chrome 기대 frame 6 scene 대조 | 이상 없음 |
| 4 | safe와 unsafe가 positive free space에서 다른 좌표를 낼 수 있음 | positive 3-alignment matrix와 Android/iOS positive route | 이상 없음 |
| 5 | overflow에서 safe가 start-side 위치를 보존하지 못할 수 있음 | 46px gutters·세 10px rows·item y `[0,56,112]` 확인 | 이상 없음 |
| 6 | overflow에서 unsafe가 다른 위치를 낼 수 있음 | `unsafe flex-start` 독립 기대값, RuntimeSession 및 양쪽 Simulator | 이상 없음 |
| 7 | row-gap 때문에 track base가 바뀔 수 있음 | Layout trace에서 rows `[10,10,10]`, overflow gutters `[46,46]` 확인 | 이상 없음 |
| 8 | positive scene에서 빈 gutter가 남을 수 있음 | row-gap `0px` trace에서 gutters `[0,0]` 확인 | 이상 없음 |
| 9 | 기존 `start` 회귀 | 기존 safe-start regression과 새 matrix의 plain `start` control | 이상 없음 |
| 10 | safe/unsafe center 또는 end 회귀 | 기존 safe-center/end focused tests 실행 | 이상 없음 |
| 11 | `safe flex-end`가 실수로 열릴 수 있음 | style projection·RuntimeSession negative control을 safe flex-end로 유지 | 거부됨 |
| 12 | `unsafe flex-end`가 실수로 열릴 수 있음 | style allowlist negative values와 end profile regression | 거부됨 |
| 13 | baseline 안전 modifier가 허용될 수 있음 | baseline rejection control 및 allowlist closure test | 거부됨 |
| 14 | 알 수 없는 computed serialization이 통과할 수 있음 | `allowlist_keeps_baseline_safety_and_unknown_values_closed` | 거부됨 |
| 15 | CSS parser/general style admission 범위까지 변경될 수 있음 | 변경 경로가 typed projection allowlist와 profile gate로 한정됨을 확인 | 이상 없음 |
| 16 | 다른 item/track profile로 적용될 수 있음 | layout admission은 기존 `RootHtmlAutoRowStretchV1` profile 분기 내부 | 이상 없음 |
| 17 | gutter가 alignment value에 따라 바뀔 수 있음 | 각 alignment에서 trace의 gap 값이 같음을 assertion | 이상 없음 |
| 18 | 지원 요청이 partial snapshot만 publish할 수 있음 | RuntimeSession `Ready`, complete snapshot/key 일치 확인 | 이상 없음 |
| 19 | unsupported 입력 실패가 같은 session을 오염시킬 수 있음 | 안전 flex-end 원자 거부 뒤 같은 session valid request recovery | 이상 없음 |
| 20 | 앱 route가 실제 값과 다른 fixture를 쓸 수 있음 | APK/iOS app fixture SHA, runtime layout/draw logs, screenshot bounds 대조 | 이상 없음 |

남은 CSS topology·WPT·실기기·surface pixel 경계는 [실행 근거](./README.md)에 명시했으며 기능 지원으로 확대 해석하지 않는다.
