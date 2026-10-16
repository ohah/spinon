# C19 기본 페인트 속성 계획 실패 관점 검토

구현 전에 계획이 기능 범위와 실패 기준을 놓치지 않았는지 20개의 서로 다른 관점으로 확인했다. 이 표는 구현 또는 PR 변경 검토를 대신하지 않는다.

| # | 실패 관점 | 계획의 차단 기준 | 판정 |
|---:|---|---|---|
| 1 | 계획이 Chromium oracle보다 먼저 코드를 허용하는가 | grammar 입력, browser version, environment와 hash를 고정하기 전에는 C19.1 구현을 시작하지 않는다. | 통과 |
| 2 | CSS Color 4 grammar 일부를 빠뜨려 제한 profile을 전체 색상으로 오해하는가 | named/system, transparent/currentColor, hex, rgb, hsl, hwb, Lab/LCH, OKLab/OKLCH, color() production을 항목별 fixture로 세분한다. | 통과 |
| 3 | 유효한 wide-gamut 값이 sRGB 단순 clamp로 바뀌는가 | 표준 변환·gamut 처리 결과를 Chrome tuple/pixel oracle와 비교하고 변환 실패를 진단한다. | 통과 |
| 4 | alpha를 0/1만 허용하거나 8-bit 변환을 너무 일찍 적용하는가 | fractional alpha와 색공간 변환 허용오차를 실행 전 고정하고 surface quantization을 별도로 기록한다. | 통과 |
| 5 | `currentColor`가 상속된 color 대신 선언 순서의 이전 값을 읽는가 | inherited `color`, currentColor background/border와 invalid declaration fallback을 같은 cascade fixture에서 비교한다. | 통과 |
| 6 | background-color가 잘못 상속되거나 초기 투명 대신 검정이 되는가 | CSS Backgrounds 3의 transparent initial/non-inherited 동작과 Chrome computed value를 고정한다. | 통과 |
| 7 | `color` 상속·초깃값이 강제 light environment와 다르게 처리되는가 | system color와 color-scheme/forced-colors를 fixture environment에 포함하고 oracle 값을 저장한다. | 통과 |
| 8 | alpha blending이 encoded sRGB에서 실행되어 값이 달라지는가 | sRGB surface, transfer conversion, overlapped interior samples와 browser pixels를 함께 대조한다. | 통과 |
| 9 | child별 opacity로 group compositing을 흉내 내 겹침이 어두워지는가 | 두 자식의 내부 overlap 및 nested group에서 그룹 결과와 per-child mutation을 구분한다. | 통과 |
| 10 | opacity가 stacking context 또는 sibling paint order를 바꾸지 않는가 | opacity 적용 전후로 stacking order·overlap sample과 C13 regression을 보존한다. | 통과 |
| 11 | opacity group이 parent clip 바깥까지 합성되는가 | nested clip 내부·외부 sample을 독립 좌표로 고정하고 render/hit clip을 함께 확인한다. | 통과 |
| 12 | transparent/zero-opacity element가 hit test에서 제외되는가 | same geometry의 opaque·partial·transparent 쌍에서 NodeId/hit miss가 동일해야 한다. | 통과 |
| 13 | color alpha와 element opacity를 곱하는 순서가 달라지는가 | color alpha, group opacity, backdrop을 독립 조합한 browser pixel cases를 고정한다. | 통과 |
| 14 | surface format 또는 플랫폼별 color encoding 차이를 놓치는가 | Android PixelCopy와 iOS Metal drawable에서 target format/profile을 기록하고 같은 RGBA oracle을 쓴다. | 통과 |
| 15 | WCAG contrast에서 gamma-correct relative luminance를 빼먹는가 | 독립 수식 구현과 browser fixture를 함께 두고 경계 ratio 양쪽을 비교한다. | 통과 |
| 16 | 반투명 foreground/background를 compositing 전 값으로 계산하는가 | black/white backdrop과 foreground/background alpha 조합의 최종 색을 기준으로 ratio를 계산한다. | 통과 |
| 17 | contrast fixture 결과를 전체 접근성 적합성으로 과장하는가 | 16px text ratio만 검증하며 accessibility tree·anti-aliasing·WCAG 제품 준수는 명시적으로 제외한다. | 통과 |
| 18 | text painter 부재를 color 지원 완료로 오해하는가 | 실제 glyph color의 C15/C16 common fixture가 통과하기 전에는 C19 체크를 유지한다. | 통과 |
| 19 | GPU group allocation/변환 실패가 부분 frame이나 이전 색상을 노출하는가 | 실패 시 atomic publish, node/property/stage diagnostics, negative controls를 구현 gate에 둔다. | 통과 |
| 20 | fixture 코드가 기본 제품 artifact에 포함되거나 무관한 통과를 완료 근거로 쓰는가 | opt-in positive/negative artifact 대조, 별도 플랫폼·suite gate와 한계를 완료 기준에 둔다. | 통과 |

## 판정

20개 계획 관점 모두 차단 기준을 계획에 명시했다. 이는 구현 검증이 아니다. C19 구현 뒤에는 코드·런타임에 대한 새 20개 실패 관점과 해당 PR diff의 별도 검토를 기록한다.
