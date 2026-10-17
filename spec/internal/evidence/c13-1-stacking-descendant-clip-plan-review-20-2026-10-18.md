# C13.1 nested stacking descendant 계획 적대 검토 20개 · 2026-10-18

코드 구현 전에 계획과 고정 Chrome 입력을 서로 다른 실패 관점으로 점검했다. 아래 판정과 완화는 [계획](../../../plan/c13-1-stacking-descendant-clip-controls.md)과 [사전 비교](c13-1-stacking-descendant-clip-plan-2026-10-18/README.md)에 반영했다.

| # | 독립 실패 관점 | 계획상 방어 및 통과 기준 |
| ---: | --- | --- |
| 1 | `overflow:visible`이 실제 positive control이 아닌 경우 | 세 case geometry를 같게 고정하고 visible 경계 밖 target hit/blue paint를 Chrome에서 먼저 관찰한다. |
| 2 | `clip`과 `hidden` 결과를 한 행으로 합쳐 차이를 놓치는 경우 | 두 value를 각각 실행하고 각자 6 hit·6 RGBA sample을 가진다. |
| 3 | target 내부 점만 통과하고 clip 경계를 잘못 처리하는 경우 | 경계 안 `+119.5`와 밖 `+120.5`를 함께 검사한다. |
| 4 | clipped target frame 전체를 제거해 visible overflow도 깨뜨리는 경우 | 동일한 target frame에서 visible/clip/hidden을 대조한다. |
| 5 | paint는 clip되나 hit identity가 새는 경우 | `elementFromPoint()` 기준 18개 exact identity를 surface 색과 독립 비교한다. |
| 6 | hit는 clip되나 descendant paint가 밖으로 새는 경우 | Chrome PNG 및 Android PixelCopy/iOS Metal의 18개 RGBA를 별도 비교한다. |
| 7 | 자손의 큰 z-index가 조상 stacking context를 탈출하는 경우 | parent `z=0`, nested layer `z=900`, target `z=10`과 sibling overlay `z=1`을 고정한다. |
| 8 | sibling overlay가 clip 밖 descendant에 가려지는 경우 | 세 case의 동일한 overlap point에서 overlay identity와 red RGBA를 요구한다. |
| 9 | overlay가 parent의 clip ancestry에 잘못 포함되는 경우 | overlay를 root의 sibling으로 생성하고 fixture node order와 rect를 고정한다. |
| 10 | 내부 parent 배경/target stacking 순서가 다르게 동작하는 경우 | parent-only positive control, target-inside positive control과 exact node id를 각각 둔다. |
| 11 | static child가 부모 border/hit 규칙을 혼동시키는 경우 | border를 두지 않아 이번 테스트는 clip·stacking 상호작용만 분리한다. |
| 12 | case 순서가 달라져 ID digest가 우연히 같은 경우 | 고정 case ID·순서·frame·point inventory 및 hash를 입력 검증에 묶는다. |
| 13 | DPR/viewport mismatch로 경계 sample이 다른 pixel을 읽는 경우 | Chrome 기준 `540×128`, DPR 1, 각 CSS pixel의 중심과 screenshot 크기를 고정한다. |
| 14 | subpixel 좌표 반올림 차이가 색 mismatch로 오인되는 경우 | hit query 좌표는 CSS pixel center이고 paint byte는 `floor(x), floor(y)`를 명시한다. |
| 15 | root fallback identity가 body/html 중 임의로 매핑되는 경우 | fixture `#fixture[data-id=root]`를 명시하고 Chrome hit result를 해당 stable ID로 수집한다. |
| 16 | 새 fixture 경로가 기본 앱 artifact에 들어가는 경우 | 기존 C13.1 전용 fixture feature에서만 연결하고 default/product 부정 대조를 둔다. |
| 17 | 앱 query와 surface 캡처가 다른 render generation에서 합쳐지는 경우 | run ID/process/surface generation 및 document/style/render revision을 같은 summary에 묶는다. |
| 18 | 단일 process 결과를 안정성 일반화로 오해하는 경우 | Android와 iOS 각각 새 앱 process 세 개를 실행한다. |
| 19 | simulator 증거를 실제 GPU/display 또는 DOM event로 확대하는 경우 | AVD software backend와 compositor 전 iOS drawable로 한정하고 event/WPT를 제외 항목으로 고정한다. |
| 20 | 구현 실패를 fixture 재정의로 숨기는 경우 | Chrome 입력 hash와 expected ID/color를 source로 고정하고, mismatch면 fixture 재수집·plan review를 먼저 갱신한다. |

**판정:** 계획 단계 통과. 실제 runtime 통과를 의미하지 않는다. 구현 시작 전 계획 PR이 `Rebase and merge`된 뒤, 구현 변경에 대한 새 20개 failure-perspective review를 작성한다.
