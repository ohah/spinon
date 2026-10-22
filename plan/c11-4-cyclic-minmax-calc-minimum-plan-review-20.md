# C11.4 cyclic `minmax(calc())` 최솟값 계획 검토 20개

검토 대상: [집중 계획](./c11-4-cyclic-minmax-calc-minimum.md), [Chrome 156 사전 입력](../tests/fixtures/css/c11/grid-cyclic-minmax-calc-minimum-c114-precomparison.html), [규범 및 기대값 근거](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-precomparison-2026-10-22/README.md). 각 항목은 서로 다른 입력 또는 판정 실패 경로다.

| # | 실패 관점 | 계획에서 요구하는 판정 | 결과 |
| --- | --- | --- | --- |
| 1 | percentage basis를 border-box 14px로 계산 | `box-sizing:content-box`, 10px content box와 2px border를 별도로 기록한다. | 반영 |
| 2 | cyclic percentage를 intrinsic phase에서 바로 200%로 resolve | 규범상 intrinsic container size 단계에서 cyclic track percentage를 auto처럼 취급하고, final phase에서 10px basis로 resolve한다. | 반영 |
| 3 | `minmax(calc())` 최솟값을 직전 최댓값 테스트와 혼동 | 신규 입력은 min이 `calc()`, max가 `max-content`인 형태로 고정한다. | 반영 |
| 4 | calc minimum을 제거해도 item max-content가 track을 결정해 테스트가 통과 | 200% minimum 결과 24/22px이 10px max-content보다 커지도록 선택한다. | 반영 |
| 5 | `max-content`보다 큰 min에서 max를 그대로 사용하는 구현 오류 | §7.2.1에 따른 max-floor-to-min 규칙을 기대값과 runtime assertion에 둔다. | 반영 |
| 6 | Ahem이 로드되지 않아 fallback 글꼴 폭이 기대값을 오염 | Chrome observation에서 `document.fonts.check("10px Ahem")` 및 loaded 상태를 저장한다. | 반영 |
| 7 | 한 글자의 intrinsic width/height가 10px이 아닌 fixture 차이 | Ahem `10px/10px`, 단일 `a`, item/root contribution을 고정하고 root content size를 기록한다. | 반영 |
| 8 | column 결과가 row의 definite inline size에 의해 정해짐 | cyclic column은 max-content width, cyclic row는 고정 10px inline size와 max-content height로 분리한다. | 반영 |
| 9 | definite control이 cyclic 결과를 대신해 intrinsic phase를 검증했다고 오인 | cyclic column/row와 definite column/row를 별도 ID로 실행한다. | 반영 |
| 10 | row-axis calc basis가 inline-size 또는 border-box에서 취해짐 | row fixture의 content height, root border box, row track을 별도 기록한다. | 반영 |
| 11 | border가 intrinsic box 크기에 미치는 4px 합이 누락 | root outer box 14px와 content box 10px을 따로 비교한다. | 반영 |
| 12 | item auto min-size가 calc minimum 대신 track을 늘림 | Ahem min/max-content contribution 10px과 최종 track 24/22px을 trace로 비교한다. | 반영 |
| 13 | stretch/alignment가 track 자체의 크기처럼 보임 | `align-content:start`, `justify-content:start`를 고정하고 used track list와 item frame을 함께 단언한다. | 반영 |
| 14 | overflow가 `overflow:visible` 선언에만 의존 | 기본 visible 동작을 쓰고 미지원 declaration을 추가하지 않는다. | 반영 |
| 15 | 앱이 track frame만 검사하고 실제 pixel clipping을 놓침 | Android PixelCopy와 iOS Metal drawable에서 root 경계 바깥 pixel을 읽는다. | 반영 |
| 16 | 배경색 overlay가 CSS border 검증을 가장함 | border는 runtime CSS/layout/paint가 생성하며 테스트 전용 border overlay는 금지한다. | 반영 |
| 17 | overflow item이 border를 덮는 순서 때문에 border 존재를 잘못 판정 | 가려지지 않는 left/top border control과 가려지는 right/bottom sample을 각각 둔다. | 반영 |
| 18 | CSS px 좌표를 device px로 그대로 샘플해 잘못된 색을 읽음 | density/DPR 변환과 PixelCopy/Metal drawable 원점·scale을 실행 근거에 저장한다. | 반영 |
| 19 | Chrome screenshot color management 차이를 CSS sRGB 의미로 오인 | Chrome screenshot은 위치·paint order 기준으로 쓰고 computed CSS color와 pixel RGB를 분리한다. | 반영 |
| 20 | fixed fixture 성공을 전체 C11.4 또는 CSS Grid 지원 완료로 승격 | 상태 checkbox와 `72/213`을 유지하고 미지원 경계를 계획·PR에 명시한다. | 반영 |

판정: Chrome 156에서 Ahem 10px 글리프가 만든 10px content size, 최종 24/22px minimum track, 14px border box와 border/item overflow 위치를 관찰했다. CSS Grid 2025 CRD §7.2.1·§10.6의 순서로 독립 계산했다. 계획은 네 fixed scene의 계획 검토를 통과했으며 구현 전용 20개와 PR diff 30개는 별도로 수행한다.
