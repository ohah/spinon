# C11.4 app surface 표본 oracle 교정 계획 검토 20관점

**검토 대상 계획 blob:** `plan/c11-4-image-auto-max-border-overflow.md`<br>
**SHA-1:** `30922642f0bd8c6763742457699222f20010723e`<br>
**검토 기준 source:** `origin/main` `29449e4f794e53255909b58ed8fb2fd4995dd5d2`<br>
**관련 evidence:** [Chrome 사전 비교](../c11-4-image-auto-max-border-overflow-precomparison-2026-10-26/README.md), [최초 iOS runtime 결과](../c11-4-image-auto-max-border-overflow-runtime-2026-10-26/README.md)<br>
**원래 계획 검토:** [초기 20관점 PASS](../c11-4-image-auto-max-border-overflow-plan-review-2026-10-26/plan-review-20.md)<br>
**검토 결과:** 이 변경 범위는 outside-root app sample의 계층 오라클만 교정한다. 아래 20개 확인 결과, 기존 geometry/track/sample mapping과 product renderer 동작을 바꾸지 않는다. 코드 gate 및 C9 재실행은 이 새 계획 blob에 대한 root PASS가 난 뒤 허용한다.

**후속 상태:** 아래 항목은 계획 교정 검토 당시의 판정이다. 승인 후 iOS Simulator와 Android API 37 AVD 재실행을 완료했고, 결과는 [runtime evidence](../c11-4-image-auto-max-border-overflow-runtime-2026-10-26/README.md)에 기록했다.

| # | 적대적으로 확인한 실패 가능성 | 판정 및 근거 |
|---:|---|---|
| 1 | 계획 SHA가 새 outside-root 기대값 수정 이후의 blob을 가리키지 않음 | SHA-1 `30922642f0bd8c6763742457699222f20010723e`를 대상 계획의 정확한 blob에서 고정했다. |
| 2 | 계획의 기준이 origin/main이 아니거나 다른 checkout의 source와 섞임 | 계획에 `29449e4f794e53255909b58ed8fb2fd4995dd5d2`를 계속 고정했다. |
| 3 | iOS black 표본 하나를 CSS body 배경의 일반 결과로 과잉 해석 | Chrome HTML oracle의 `(26.5,8.5)` `[16,24,32,255]`와 앱 표면의 `[0,0,0,255]`를 명시적으로 분리했다. |
| 4 | `(26.5,8.5)`가 실제로 root box 안쪽이어서 root 배경을 읽음 | root border-box는 폭 12px이므로 x=26.5는 root 밖이다. 좌표 및 기존 scene frame은 유지된다. |
| 5 | outside-root sample이 root clipping을 기대하는데 surface clear와 혼동 | 기대를 root paint에서 surface clear로 바꾸고 mutation 설명도 동일하게 교정했다. |
| 6 | app surface clear가 임의 기본값 추측으로 설정됨 | 첫 iOS Metal run에서 실제 `[0,0,0,255]`가 측정됐고 공용 draw 코드도 `root_html_default_render=false`일 때 이를 지정한다. |
| 7 | root HTML 경로의 `root_html_canvas_color`를 C11.4 app fixture에 잘못 적용 | 계획은 두 renderer mode가 다른 경로임을 분리하며 root HTML body/canvas 값은 app clear 입력으로 사용하지 않는다. |
| 8 | C11.4 fixture가 일반 document compositing 또는 DOM body backdrop을 실행한다고 주장 | 앱 fixture가 native opt-in surface path임을 설명하고 일반 웹 document parity 주장을 하지 않는다. |
| 9 | 제품 renderer를 바꾸어 local expected를 맞춤 | renderer clear 동작 변경을 명시적으로 제외했다. 이번 변경은 fixture oracle만 고친다. |
| 10 | fixture-level whole-surface backdrop API가 이미 있는데 무시 | C11.4 fixture path에는 이를 구성하는 API가 없음을 조사했고 새 backdrop API도 추가하지 않는다. |
| 11 | geometry나 track sizing 값을 다시 튜닝해 pixel mismatch를 숨김 | columns `[24,17]`, row `[16]`, root `12×40`, item `(1,1,24,16)`은 그대로다. |
| 12 | item overflow marker의 outside-root visibility가 함께 기대값 변경으로 약화 | image/item 표본 `(12.5,8.5)`의 red marker `[229,72,77,255]`와 overflow geometry 검사는 유지한다. |
| 13 | root border 표본까지 surface clear로 재분류 | 좌·우 border 표본은 root border paint `[34,197,94,255]` 그대로다. |
| 14 | CSS pixel center를 정수 좌표로 바꾸어 표본 위치가 이동 | runner 정수 index `[26,8]` 및 center `(26.5,8.5)`는 바뀌지 않는다. |
| 15 | scale/origin 수식을 변경해 다른 surface pixel을 읽음 | Android/iOS mapping `floor((origin + index + 0.5) * scale)` 및 origin `[0,0]`은 바뀌지 않는다. |
| 16 | Android clear 경로가 iOS와 다를 가능성을 확인 없이 배제 | 공용 native renderer path는 동일하나 Android runtime 증거는 미검증으로 남기고 API 37 simulator capture를 요구한다. |
| 17 | PixelCopy가 view background나 root CSS를 따로 합성해 clear와 다른 색을 읽음 | Android adapter는 SurfaceView 대상 PixelCopy bitmap을 읽는다. API 37 capture를 남은 runtime gate로 둔다. |
| 18 | iOS sample이 compositor 제출 후 사용자 화면을 의미한다고 과장 | 계획은 Metal drawable readback이 compositor 제출 전임을 계속 명시한다. |
| 19 | 기존 실패 로그를 새 expected value로 다시 해석하거나 덮어씀 | 최초 iOS `3/4 mismatch` 로그를 별도 evidence로 보존하고 새 run은 새 run ID/evidence로 기록한다. |
| 20 | 이 표본 교정을 C11.4 기능 완료로 계산 | 공식 checkbox와 `72/213`은 유지되고 전체 overflow/clip 지원이나 C11.4 완료 주장을 제외한다. |

**자체 계획 교정 판정:** PASS; 단, 재빌드 및 simulator capture는 root의 이 SHA 독립 PASS 이후에만 실행한다.
