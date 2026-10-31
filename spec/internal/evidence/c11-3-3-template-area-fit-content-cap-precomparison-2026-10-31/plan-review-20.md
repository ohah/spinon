# C11.3.3 area-backed fit-content cap 계획 실패 관점 검토 20개

계획 자체를 구현 전에 검토했다. 기준 입력과 Chrome 관찰은 [고정 precomparison](README.md) 및 [Chrome observation](chrome/observation.json)에 묶었다. 아래 항목은 서로 다른 실패 원인을 다룬다.

| # | 실패 관점 | 계획상 방어·판정 |
| ---: | --- | --- |
| 1 | Ahem 파일이 바뀌어 glyph advance가 10px가 아님 | pinned Ahem SHA-256을 runtime/test 근거에 기록하고 font loaded를 확인한다. |
| 2 | Chrome 캡처 전에 web font가 로드되지 않아 fallback font를 사용함 | `document.fonts.ready`, `document.fonts.check`, loaded 상태를 oracle gate로 둔다. |
| 3 | white-space collapse 변경으로 공백 advance/줄바꿈이 달라짐 | 고정 HTML과 실행 source에서 `white-space:normal` 의미를 보존하고 text content를 고정한다. |
| 4 | min-content와 max-content가 같은 값이라 cap이 구분되지 않음 | 독립 probe를 각각 고정해 20px와 110px가 40px limit 양쪽에 놓이는지 검사한다. |
| 5 | probe 측정만 통과하고 Grid item contribution은 text를 사용하지 않음 | Spinon intrinsic contribution trace의 min/max source와 값도 직접 확인한다. |
| 6 | text source가 matrix generator에서 생략됨 | 생성된 runtime JS에 `AA AA AA AA`가 남는지 source contract test를 둔다. |
| 7 | fixture가 area 크기와 explicit track 수를 같게 지정해 implicit track을 만들지 않음 | explicit columns 1개, area columns 2개, typed auto column 1개를 고정하고 track trace를 확인한다. |
| 8 | `grid-area:fit`이 계산되지 않아 item이 다른 column에 놓임 | CSS named-area placement와 expected root-relative x=20을 함께 assertion한다. |
| 9 | track 40px이 cap이 아니라 item의 explicit width에서 유래함 | item width를 auto로 유지하고 `fit-content(40px)` 외 별도 width constraint를 두지 않는다. |
| 10 | auto minimum floor가 40px 이상이라 cap 경로를 시험하지 않음 | min-content 20px의 contribution과 final 40px track을 분리해 비교한다. |
| 11 | grid track 폭은 40px지만 stretched item frame은 track과 달라짐 | root-relative item frame `[20,0,40,60]`을 track trace와 별도로 확인한다. |
| 12 | zero gap 또는 margin reset이 빠져 좌표와 계산 폭이 이동함 | root, items, body의 margin/padding/border/gap을 0으로 고정한다. |
| 13 | text item의 line-height가 달라 max-content/intrinsic height에 영향을 줌 | Ahem font size와 line-height를 모두 10px로 고정한다. |
| 14 | CSS px와 native physical pixel을 혼동함 | Chrome은 DPR 1로 고정하고 native 좌표는 surface origin과 scale 변환을 기록한다. |
| 15 | 화면 색상 선택자가 잘못 연결되어 sample이 배경색만 검사함 | fixture의 실제 element ID와 color rule을 일치시키고 내부/외부 경계 표본을 사용한다. |
| 16 | root 폭이 track 폭과 같아 cap 이후의 빈 영역을 구분하지 못함 | root 200px, used tracks 20px+40px를 고정해 60px 바깥의 root 배경을 비교한다. |
| 17 | adapter 추가가 일반 C11.3.3 text/area support를 조용히 넓힘 | C11.4 opt-in feature와 단일 scene/node allowlist로 제한하고 feature-off archive를 확인한다. |
| 18 | C11.3.3 기존 area fallback/unknown-area 동작 회귀 | 기존 39-scene matrix를 그대로 실행하고 새 case는 별도 assertion을 둔다. |
| 19 | Android/iOS 화면이 Chrome oracle를 대체하거나 iOS compositor 증거로 오해됨 | geometry, PixelCopy, Metal drawable을 분리 기록하고 Chrome-only·SwiftShader·pre-compositor 한계를 표기한다. |
| 20 | 한 scene의 pass가 C11.3.3/C11.4/C11 전체 완료로 집계됨 | STATUS.md 체크와 공식 72/213을 변경하지 않고 제한 fixture 근거로만 기록한다. |

계획 단계에서 차단 결함을 찾지 못했다. 구현 전에 위 각 경계를 소스와 runtime fixture에서 다시 확인하고, 새 발견은 구현 계획 또는 범위를 수정한다.
