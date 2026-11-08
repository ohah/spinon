# C11.4 반복 intrinsic row text 계획 검토 · 20 관점

계획 구현 전 독립 실패 경로를 확인했다. 계획 검토는 구현/runtime 검토와 합산하지 않는다.

| # | 별도 실패 관점 | 판정과 계획상 대응 |
|---:|---|---|
| 1 | Chrome fixture와 앱 fixture가 서로 다른 tree일 수 있는가? | PASS — 동일한 체크인 HTML과 SHA를 고정한다. |
| 2 | text가 grid item이 아니라 wrapper payload에서 측정되는 점을 놓치는가? | PASS — 8 item과 8 payload frame을 별도로 비교한다. |
| 3 | grid의 indentation whitespace가 anonymous text item을 추가하는가? | PASS — 각 Grid child는 명시 div 하나이며 text는 payload 안에만 둔다. |
| 4 | empty/collapsible paragraph가 유효 텍스트로 오인되는가? | PASS — 정확히 한 source와 정확한 문자열을 요구한다. |
| 5 | 다른 monospace fallback이 줄 수를 바꾸는가? | PASS — Chrome 기준을 고정하고 양 Simulator 결과가 다르면 범위를 막는다. |
| 6 | font shorthand의 기본값이 플랫폼마다 달라지는가? | PASS — family, size, weight, style, line-height, spacing을 모두 지정한다. |
| 7 | `white-space:normal`을 `nowrap` 측정으로 대체하는가? | PASS — normal과 1·2·3 line payload를 기록한다. |
| 8 | line-height normal이 OS font metrics에 따라 바뀌는가? | PASS — 10px explicit line-height만 받는다. |
| 9 | 줄 수 대신 text payload의 고정 높이를 사용해 false pass하는가? | PASS — 높이 하드코딩을 금지하고 각 payload frame과 row를 확인한다. |
| 10 | 30개 tail order 중 일부만 projection에서 보존되는가? | PASS — 순서 생성 검증과 30 order × 4 cap × 2 align scene을 사용한다. |
| 11 | definite percentage cap의 basis가 잘못 잡히는가? | PASS — 220px definite block-size에서 20%와 120%를 20px 및 0%와 대조한다. |
| 12 | `0%` cap의 zero-size 경계가 누락되는가? | PASS — 30 order 모두에서 0%를 start/stretch 양쪽으로 검사한다. |
| 13 | stretch 여유가 intrinsic track에 잘못 배분되는가? | PASS — start와 stretch를 같은 scene으로 비교한다. |
| 14 | 100px 컨테이너에서 row 합이 넘쳐도 item paint를 확인하지 않는가? | PASS — item과 payload frame을 Grid frame과 각각 비교한다. |
| 15 | fit-content row 안에서 payload가 넘치는 경우를 놓치는가? | PASS — 220px 및 100px 장면의 row/payload 크기를 함께 기록한다. |
| 16 | CSS px subpixel roundoff가 오차로 숨겨지거나 과장되는가? | PASS — 수치는 1e-5 단위 저장, 허용오차는 명시적으로 0.01 CSS px다. |
| 17 | 다른 text, inline span, multiple source가 우연히 허용되는가? | PASS — 잘못된 문장·inline·복수 source 음성 검사를 요구한다. |
| 18 | style/node 오류 뒤 partial layout 또는 draw state가 남는가? | PASS — atomic rejection과 같은 세션 valid revision recovery를 요구한다. |
| 19 | simulator 화면을 실제 표면 픽셀 동등성으로 과장하는가? | PASS — OS screenshot은 visual smoke로 한정하고 readback과 구분한다. |
| 20 | bounded plan이 공식 완료 수를 바꾸거나 일반 CSS 지원을 뜻하는가? | PASS — C11.4/C11 미완료, `78/214`, 계약 `0.1.0`을 유지한다. |

이 계획은 20개의 독립 관점에서 누락을 찾지 못했다. 이는 제품 코드 승인이나 구현/runtime 검토 통과를 뜻하지 않는다.
