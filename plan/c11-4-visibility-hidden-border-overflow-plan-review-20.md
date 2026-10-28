# C11.4 visibility-hidden border overflow 계획 · 실패 관점 20개

대상은 [후속 계획](./c11-4-visibility-hidden-border-overflow.md), [Chrome 관측](../spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-border-overflow-precomparison-2026-10-28/README.md), 고정 HTML과 독립 기대값이다. 구현에 들어가기 전 계획 자체를 서로 다른 실패 경로로 검토했다. 아래의 PASS는 계획 범위·oracle·판정 기준이 해당 위험을 직접 다룬다는 뜻이며 Spinon 구현 통과를 뜻하지 않는다.

| # | 공격 관점 | 검토 결과 | 판정 |
| --- | --- | --- | --- |
| 1 | 기존 #738 장면과 새 계획이 같은 동작을 중복했다고 오인할 수 있는가? | #738 root 축은 300px로 overflow가 없고 이번 root는 100px이다. 기존 anonymous text/image border fixture도 topology가 달라 별도 근거로 취급한다. | PASS |
| 2 | hidden 기대값을 Chrome 관측에서 역산했는가? | 96px content, 40px fixed track, gap 20px, 150px min-content를 먼저 계산해 `[150,40,0]`을 고정하고 CSSOM은 대조 자료로만 둔다. | PASS |
| 3 | `visibility:hidden`이 layout에서 빠지는 `display:none`으로 잘못 설정될 수 있는가? | 새 hidden span은 `display:block`, `visibility:hidden`, client rect 1, full-size frame과 contribution을 각각 요구한다. | PASS |
| 4 | visible control이 여러 CSS 차이를 함께 바꾸어 비교가 무효가 될 수 있는가? | 각 hidden/control pair는 동일 HTML order, text, background, placement, track과 root style을 공유하고 visibility 하나만 다르게 한다. | PASS |
| 5 | stacking order 때문에 표본이 span 아래/위의 의도한 표면을 읽지 못할 수 있는가? | A가 먼저, span이 나중에 오며 겹치는 root-local 표본에서 hidden은 파랑, visible은 보라를 기대한다. | PASS |
| 6 | 2px border를 content size 계산에서 빼먹을 수 있는가? | `border-box` 100px root에서 target content는 96px이며 rows도 같은 축 크기다. 이 값이 독립 계산과 JSON에 있다. | PASS |
| 7 | gap이나 fixed track을 빼먹어 flex space가 틀릴 수 있는가? | target axis의 두 10px gap과 40px track을 명시하고, flex 공간 36px 계산을 계획에 고정한다. | PASS |
| 8 | min-content 기여자가 10자 A와 15자 span 중 잘못 선택될 수 있는가? | 둘 다 동일 첫 cell에 겹치고 later source-order span의 15자 Ahem word가 150px base를 결정한다. | PASS |
| 9 | row 축이 단순한 가로 text 결과를 재사용해 orthogonal sizing을 놓칠 수 있는가? | Rows 장면은 `vertical-rl` Ahem text, row tracks와 bottom overflow 표본을 별도로 정의한다. | PASS |
| 10 | zero-size third track이 150px 첫 item의 overflow 판정을 가릴 수 있는가? | C frame은 0px target size로 기록하고 overflow 표본은 first item 내부의 root 밖 위치에 둔다. | PASS |
| 11 | border sample이 overflow item 아래에 가려져 gold 대신 item color를 읽을 수 있는가? | 초안의 right/bottom 표본이 first item에 덮인 것을 Chrome 화면에서 발견했다. 표본을 가려지지 않는 left/top border로 옮겼고 RGB `240,198,116`을 다시 확인했다. | PASS · 수정 완료 |
| 12 | root 바깥 sample이 실제로 border box 바깥인가? | Root border box는 100px이고 overflow sample은 target-local 120px이므로 경계 밖 20px이며, first item은 local 152px까지 간다. | PASS |
| 13 | root 자체 background가 item overflow로 오인될 수 있는가? | `overflow:visible`의 root 배경은 root box로 제한되고, sample 색은 item 배경에 고정된 파랑/보라다. Simulator에서는 root-local geometry와 surface 표본을 함께 요구한다. | PASS |
| 14 | image가 overflow 표본을 우연히 덮어 visibility 결과를 왜곡할 수 있는가? | Image는 local x/y 162px에서 시작하며 sample 축 좌표는 120px이다. natural size와 frame도 별도로 고정했다. | PASS |
| 15 | sample 좌표에서 DPR나 surface offset 변환을 생략할 수 있는가? | Chrome raster mapping은 DPR 1에서만 고정한다. Android/iOS는 각 실행의 frame, scale과 offset으로 root-local 좌표를 변환하고 manifest에 기록한다. | PASS |
| 16 | 한 지점의 색을 전체 screen pixel parity로 과장할 수 있는가? | 기대값과 계획은 지정한 border/overflow 표본만 다룬다. screenshot은 보조 시각 자료로 분리한다. | PASS |
| 17 | `overflow:visible`이 없는 root에서 우연히 overflow paint만 검사할 수 있는가? | 계획은 computed root overflow와 CSS root `overflow:visible`을 고정하고 visible box exterior rendering을 별도 기준으로 둔다. | PASS |
| 18 | 이전 계획의 회귀 scene 수가 stale해져 검증 범위를 잘못 계산할 수 있는가? | 상위 plan의 12-scene 예정 수가 병합 evidence의 14-scene baseline과 달랐다. 계획을 8 mixed-freeze + 2 `display:none` + 4 hidden = 14로 고치고 후속 target을 18로 고정했다. | PASS · 문서 수정 완료 |
| 19 | Android/iOS simulator 근거를 실기기나 GPU parity로 넓혀 말할 수 있는가? | AVD PixelCopy와 iOS compositor 제출 전 Metal 표본만 요구하고 physical/hardware/compositor-after 한계를 명시한다. | PASS |
| 20 | 이 제한 fixture를 전체 C11.4 완료 또는 일반 CSS support로 올릴 수 있는가? | public roadmap은 Chrome 계획 상태만 표시하고 `spec/STATUS.md` checkbox와 `72/213`은 유지한다. | PASS |

계획 검토에서 고친 두 결함은 (1) 가려진 right/bottom border sample 좌표, (2) 상위 계획의 기존 matrix 개수다. 두 수정 뒤 계획·expected JSON·Chrome screenshot sample을 다시 대조했다. 구현은 계획 PR 병합 이후 시작한다.
