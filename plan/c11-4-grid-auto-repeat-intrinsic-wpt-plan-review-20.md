# C11.4 `grid-auto-repeat-intrinsic-001.html` 계획 실패 관점 검토 20개

구현 전에 [집중 계획](c11-4-grid-auto-repeat-intrinsic-wpt.md), Chrome 사전 관찰, 고정 WPT 입력과 독립 기대값을 서로 다른 실패 경계에서 검토했다. 계획에는 전체 8개 WPT grid를 포함하며, 이 표는 구현/runtime 또는 PR 변경분 review를 대신하지 않는다.

| # | 실패 관점 | 계획의 예방 기준 | 판정 |
|---:|---|---|---|
| 1 | 비슷한 이름의 `grid-change-intrinsic-size-with-auto-repeat-tracks-001.html`을 이번 파일로 착각한다. | 계획에 고정된 정확한 WPT 경로·revision·hash를 기록했다. | PASS |
| 2 | auto-fit/auto-fill 둘 중 하나 또는 min/max constraint 중 하나가 빠진다. | 네 조합의 교차로 8개 고정 scene ID와 각각의 기대값을 표에 전부 적었다. | PASS |
| 3 | 반복 count 기대값을 Chrome의 computed style에서 역으로 복사한다. | CSS Grid §5.2·§7.2.3.2·§11.5 규칙에서 track 폭과 count를 독립 계산한다. | PASS |
| 4 | `auto-fit`가 모든 empty track을 collapse한다고 기계적으로 적용한다. | 전체 반복 영역을 span하는 in-flow item이 각 repeated track을 non-empty로 만든다는 조건을 명시했다. | PASS |
| 5 | `minmax(min-content,40px)`의 trailing track을 repeated pattern 일부로 센다. | repeat pattern 20px과 한 suffix 40px을 분리해 count 3 및 used tracks를 기록했다. | PASS |
| 6 | `minmax(60px,auto)`에서 60px minimum을 count 계산에서 누락한다. | definite minimum을 사용해 max-content count 4, min-content count 1을 계산했다. | PASS |
| 7 | root border를 빼먹거나 content width와 border-box width를 혼동한다. | 100/70px content width와 104/74px border-box width를 별도 필드로 고정했다. | PASS |
| 8 | root 상단 padding 10px을 item y offset 또는 root height에서 잃는다. | 여덟 scene의 item y=10px, probe y=0px, root height=34px를 분리해 기록했다. | PASS |
| 9 | `justify-items:start` 없이 100px item을 stretch 결과와 혼동한다. | 원본 declaration과 computed typed style을 exact source fixture 조건에 포함한다. | PASS |
| 10 | absolute probe가 intrinsic contribution이나 repeat count에 참여한다. | 모든 probe를 out-of-flow로 분류하고 contribution/track-count에서 제외한다고 정의했다. | PASS |
| 11 | 범위 밖 `grid-column` line이 새 implicit track을 만든다. | §9.1 fallback을 고정하고 cases 4/8의 line 3/4/5 geometry를 별도로 예상한다. | PASS |
| 12 | 마지막 `4/5` probe의 auto fallback을 `x=70,width=0`으로 오해한다. | cases 4/8에서 third probe `(70,0)`와 fourth probe `(0,70)`을 각각 기록했다. | PASS |
| 13 | 0px probe를 그려진 box나 pixel 표본으로 세어 off-by-one 오류를 낸다. | trace/frame에는 0-width node를 유지하고 painted/sample count에서는 제외한다. | PASS |
| 14 | min-content item overflow를 frame clip으로 숨긴다. | cases 4/8의 item frame과 28px border-edge overflow를 비교하고 실제 surface positive sample을 요구한다. | PASS |
| 15 | overflow 좌표를 root-content 원점과 border-box 원점 중 잘못된 기준으로 계산한다. | WPT offset은 padding-edge-relative, simulator 표면점은 root border-box-relative로 별도 정의한다. | PASS |
| 16 | 색상 표본이 border 안쪽으로 이동하거나 item 밖 대조가 사라진다. | root border, border 밖 item pixel, item 바깥 negative control을 서로 다른 표본으로 요구한다. | PASS |
| 17 | WPT source의 defaults/helper CSS가 앱 normalization에서 빠진다. | 원본 helper 파일 hash와 root/item defaults, colors, placement를 고정한다. | PASS |
| 18 | 한 scene만 통과한 결과를 전체 WPT matrix로 확대한다. | 각 PR 전과 runtime에서 8개 ID를 각각 정확히 한 번씩 검사하도록 기준을 둔다. | PASS |
| 19 | fixture matcher가 이 입력 밖의 CSS를 제품 경로로 허용한다. | feature gate 아래 exact root, child topology, style, track/repeat/line allowlist 및 거부 대조를 둔다. | PASS |
| 20 | simulator draw/screenshot, archive 검사 또는 PR review를 전체 C11.4 완료로 부풀린다. | compositor·실기기·APK/IPA·일반 CSS·전체 WPT의 한계와 checkbox/72/213 유지 조건을 명시했다. | PASS |

**계획 검토:** PASS 20/20, 미해결 0. 이 기록은 구현/runtime 실패 관점 20개 및 계획 PR 변경분의 별도 20개 검토와 구별한다.
