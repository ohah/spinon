# C11.4 zero-length percentage `calc()` 계획 PR 변경 실패 관점 20개

구현 전 계획 PR의 실제 변경 파일과 기준 근거를 확인했다. 계획 검토 20개와 별개이며 구현 코드/runtime 리뷰를 대신하지 않는다.

| # | 변경 실패 관점 | 판정 |
|---:|---|---|
| 1 | 계획이 구현 완료처럼 읽히는가? | 계획 전용이라고 표기하고 C11.4 체크와 `72/213` 유지 문장을 넣었다. 통과. |
| 2 | CSS Grid 문법에서 zero length 항이 invalid라고 오해하는가? | §7.2.1의 nonnegative 범위와 링크된 CRD 원문을 대조했다. 통과. |
| 3 | auto-repeat `<track-size>`와 `<fixed-size>`를 혼동하는가? | §7.2.3.1의 fixed-size 조건과 `1fr` 최대 함수를 분리했다. 통과. |
| 4 | repeat count를 12px resolved minimum이 아닌 1fr maximum으로 계산하는가? | 독립 JSON의 `floor(120/12)=10` 및 expanded Chrome tracks가 일치한다. 통과. |
| 5 | auto-fit과 auto-fill 결과를 동일하게 취급하는가? | 10 fill tracks와 1 occupied + 9 collapsed fit tracks를 별도 기록했다. 통과. |
| 6 | columns와 rows percentage basis를 뒤바꾸는가? | 네 calc scene에서 각 relevant content 축을 120px로 고정했다. 통과. |
| 7 | control이 calc case와 다른 box 조건을 쓰는가? | root size, gap, decorations, item 수와 12px minimum을 맞췄다. 통과. |
| 8 | Chrome 결과를 독립 규범 기대값으로 재사용하는가? | spec-derived JSON과 Chrome observation JSON을 별도 파일로 보존했다. 통과. |
| 9 | observation scene 수나 반복 track 수가 달라지는가? | JSON parse 및 8 scene, 각 해당 축의 10 tracks를 검증했다. 통과. |
| 10 | calc/control pair가 item geometry까지 대조하는가? | 같은 축/모드별 computed tracks와 item size가 대응 control과 일치한다. 통과. |
| 11 | 브라우저 버전·viewport·DPR이 다른 결과를 기록하는가? | Chrome 156.0.8078.48, viewport 650×280, DPR 1을 기록했다. 통과. |
| 12 | Chrome screenshot이 다른 viewport 결과거나 장면이 잘리는가? | Agent Browser에서 650×280 캡처를 직접 보고 8개 scene을 확인했다. 통과. |
| 13 | screenshot 제목이 겹쳐 결과를 오인하는가? | 첫 겹친 캡처를 폐기하고 caption 배치를 고친 이미지로 교체했다. 통과. |
| 14 | 화면이 border 바깥 overflow pixel을 입증한다고 과장하는가? | plan은 캡처를 frame 보조 근거로만 두고 border/pixel equality를 주장하지 않는다. 통과. |
| 15 | fixture나 plan의 relative links가 끊기는가? | precomparison README, plan, umbrella plan의 relative targets를 확인했다. 통과. |
| 16 | plan PR이 코드 또는 API 버전 계약까지 바꾸는가? | 변경은 계획·oracle fixture/evidence에 한정되고 `0059`는 그대로다. 통과. |
| 17 | Stylo zero term 정규화를 감추고 직접 percentage까지 승인하는가? | bare percentage 정규화가 나오면 scope review 전 구현을 멈추도록 기록했다. 통과. |
| 18 | zero definite basis가 Taffy 0분모 경로로 전달되는가? | explicit positive fixed axis/zero decorations 이외를 typed error로 막도록 계획했다. 통과. |
| 19 | 계획에서 simulator 화면이나 archive 결과를 과장하는가? | 두 simulator와 release/fixture archive gates를 예정 범위로만 쓰고 완료 주장은 하지 않는다. 통과. |
| 20 | 기존 untracked evidence나 Cargo target cache가 PR에 들어가는가? | 의도한 신규 경로만 stage하고 기존 untracked 파일은 보존한다. `target`은 cache 정리 후 없다. 통과. |

검토 표는 20개 distinct item이며 각 판정은 현재 plan diff 또는 명시된 precomparison 산출물에서 확인했다. 이후 수정된 plan 자체의 row count와 links도 확인한다.
