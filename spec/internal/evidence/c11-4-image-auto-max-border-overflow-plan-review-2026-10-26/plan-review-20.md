# C11.4 image auto/max border overflow 계획 적대 검토 20개

**검토 대상 계획 blob:** `plan/c11-4-image-auto-max-border-overflow.md`<br>
**계획 blob SHA-1:** `ac08230270d78971c561fc9f7001d2ded0019082`<br>
**기준 source:** `29449e4f794e53255909b58ed8fb2fd4995dd5d2`<br>
**비교 자료:** [Chrome·규범 사전 비교](../c11-4-image-auto-max-border-overflow-precomparison-2026-10-26/README.md)<br>
**검토 시점:** 구현 전. 이 검토는 코드나 simulator runtime 결과를 확인하지 않는다.

| # | 독립 실패 관점 | 계획의 방어 또는 판정 |
|---:|---|---|
| 1 | 계획이 오래된 통합 후보 worktree의 코드나 증거를 새 main source인 것처럼 사용 | 기준 revision을 `origin/main` `29449e4f…`로 고정했고, 새 계획·fixture만 비교 자료로 추가한다. 오래된 worktree는 재사용하지 않는다. |
| 2 | `border-box` root의 content width 계산에서 border를 빠뜨림 | 12px border-box에서 좌우 1px border를 빼 content width 10px로 계산하고 Chrome `clientWidth=10`으로 대조한다. |
| 3 | auto minimum 조건이 성립하지 않아 24px track 기대값이 무효 | CSS Grid §6.6 조건을 분리했다. item overflow는 scrollable이 아니고, 첫 축에서 auto-min track을 span하며, 그 축의 span이 하나라 flexible track이 없다. |
| 4 | `img` 자체의 `overflow: clip`을 부모 Grid clipping으로 잘못 해석 | computed `clip/clip`은 img의 image-content overflow로 기록한다. Chrome hit-test와 root 밖 pixel로 parent clip 여부를 별도 판정한다. |
| 5 | PNG 자연 크기 대신 authored CSS 크기나 반대 축의 definite size가 contribution을 바꿈 | width/height는 auto, min/max와 반대 축 definite size는 없으며, 고정 PNG 24×16의 hash를 보존한다. |
| 6 | auto-min base contribution을 min-content와 다른 값으로 계산 | span-1 auto minimum contribution을 24px로 고정하고, 구현 결과가 다르면 fixture 기대를 바꾸지 말고 원인을 조사한다. |
| 7 | `max-content` maximum의 growth limit이 24px과 달라 track 합이 달라짐 | §11.5의 non-spanning max-content 기여를 독립 산술로 24px로 계산하고 Chrome track serialization과 비교한다. |
| 8 | 이웃 track 또는 gap이 빠져 overflow 양이 달라짐 | fixed 17px neighbor와 5px gutter를 분리해 기록한다. total 46px은 10px content width보다 36px 크다. |
| 9 | `justify-self: normal`이 replaced image 폭이나 시작 위치를 바꿈 | CSS Box Alignment §6.1에서 normal은 replaced grid item을 start sizing/alignment로 처리한다. Chrome은 item rect `(1,1,24,16)` 및 `elementFromPoint`를 관찰한다. |
| 10 | auto-placement가 다른 implicit row/column이나 span을 만들어 기대 경로를 바꿈 | 직접 자식은 단일 img 하나이며 첫 column에 자동 배치한다. 명시 배치·span·추가 자식은 validator에서 거부한다. |
| 11 | definite container height 또는 auto row sizing이 세로 기하를 바꿈 | 40px border-box에서 38px content height, 16px auto row를 따로 산술화하고 `align-items/align-content:start`를 고정한다. |
| 12 | track 정렬이 overflow를 분산해 image 시작 좌표가 달라짐 | `justify-content:start`을 고정한다. 첫 track과 image는 content 시작점인 `(1,1)`에서 출발한다. |
| 13 | grid의 overflow/clip 값을 새 CSS admission과 혼동 | 일반 overflow support를 추가하지 않는다. root 바깥 item marker가 보이지 않으면 이 slice에서 clip 정책을 고치지 않고 원인을 별도 분리한다. |
| 14 | root right edge와 item right edge의 차이를 잘못 산출 | root border-box 오른쪽 `x=12`, image 오른쪽 `x=25`이므로 item은 13 CSS px 밖으로 이어진다. |
| 15 | 네 표본의 경계 위치가 image 또는 다른 border paint와 겹치거나 runner index를 center로 해석 | 고정 표본은 item box, root 바깥 배경, 좌측 border, 우측 border의 네 개다. CSS pixel centers는 `(12.5,8.5)`, `(26.5,8.5)`, `(0.5,30.5)`, `(11.5,30.5)`이다. 기존 API에는 integer indexes `[12,8]`, `[26,8]`, `[0,30]`, `[11,30]`를 전달하고 center는 index+0.5로 계산한다. |
| 16 | Chrome PNG texel과 앱의 image box background marker를 동일한 이미지 paint로 오해 | 브라우저의 PNG texel `[255,255,64,255]`과 앱 item box `[229,72,77,255]`를 다른 oracle로 둔다. image draw가 없다는 표지를 그대로 기록하고 texture paint를 주장하지 않는다. |
| 17 | CSS pixel point를 Android/iOS surface pixel로 직접 사용하거나 scale/origin을 두 번 적용 | `originCssPx`를 더하고 `floor((originCssPx + index + 0.5) * scale)`로 device-pixel index를 구한다. 이 fixture의 origin은 `(0,0)`이며 iOS drawable scale도 같은 center convention을 쓴다. 각 캡처에는 실제 scale, origin, pixel format 및 source/app hash를 기록한다. |
| 18 | 새 scene이 기본 제품 artifact에 노출되거나 실패 뒤 부분 결과를 성공으로 보고 | opt-in fixture 경계와 feature-off negative control을 검사한다. 변경·validator 오류 뒤 frame/trace/draw/present/sample 성공 표지를 내지 않는 fail-closed 조건을 명시한다. |
| 19 | 시뮬레이터 실행이 사용 중인 앱/세션을 종료하거나 플랫폼 근거를 과장 | 기존 기기에서 runner가 install/force-stop/terminate할지 먼저 확인하고 조율한다. 물리 기기는 사용하지 않으며 Android AVD software path와 iOS pre-compositor readback 한계를 기록한다. |
| 20 | 제한 scene이 C11.4 전체 지원으로 표시되거나 PR screenshot이 잘못된 링크로 사라짐 | C11.4 checkbox와 `72/213`을 유지한다. PR 본문에 실제 raw Markdown screenshots를 넣고 Agent Browser에서 렌더를 확인한 뒤 task-owned browser process를 종료한다. |

## 검토 결과

**계획 판정: PASS — 구현 진행 가능.** 네 고정 표본의 수와 목적을 명시한 최종 plan blob에 대해 20개 관점을 다시 확인했다. 계획은 단일 opt-in scene, 규범 산술, Chrome geometry/pixel 관측, app API의 zero-based index→CSS center→surface index 변환, 앱 box marker의 별도 의미, 실패 시 중단 경계, simulator 안전 및 C11.4 미완료 상태를 고정한다. CSS Grid 2025 CRD §§6.6/11.5와 CSS Box Alignment §6.1은 Agent Browser로 원문을 대조했다. 계획된 Android/iOS simulator run과 실제 앱 표면 결과는 구현 후 별도 증거로 남아야 하며 이 검토는 이를 통과한 것으로 간주하지 않는다.
