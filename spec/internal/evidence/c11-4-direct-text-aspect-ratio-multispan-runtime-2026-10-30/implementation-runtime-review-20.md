# C11.4 direct-text aspect-ratio two-column span 구현·runtime 검토 20개

계획 검토와 다른 코드/runtime 실패 경로를 새로 검토했다. 기준 입력은 고정 9-case precomparison이며, 최종 판정은 Android API 37 ARM64 AVD와 iOS 26.2 Simulator 각각의 51-scene 성공 실행 및 focused Rust/FFI 결과다. 이 검토는 일반 CSS 적합성이나 C11.4 전체 완료 판정이 아니다.

| # | 구현/runtime 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 새 route가 C11.4 fixture-test feature 없이 일반 artifact에 들어가는가 | 기본 spinon-ffi release archive에는 새 scene suffix 0/9, helper symbol과 C11.4 fixture exports가 없다. |
| 2 | opt-in positive control에서 장면 또는 helper가 제거되는가 | fixture-test archive에서 9/9 suffix와 prepare helper, 기존 C11.4 create/eval exports가 있다. 최종 앱 번들 전체의 제외는 주장하지 않는다. |
| 3 | auto placement adapter가 명시적 span route를 먼저 받아 잘못 처리하는가 | direct-text dispatcher가 새 span collector를 기존 generic adapter 앞에서 실행한다. 기존 42 regression 장면은 같은 51-scene capture에서 보존됐다. |
| 4 | placement가 없는 장면까지 새 profile로 끌어오는가 | explicit placement가 없으면 기존 adapter 경로로 반환한다. 자동 배치 scene 42개가 Android/iOS에서 통과했다. |
| 5 | direct anonymous text 또는 빈 추가 child가 한 item처럼 통과하는가 | 의미 있는 anonymous text, 추가 element, item 수 불일치를 각각 거부한다. negative Rust tests가 추가 item과 source shape를 확인한다. |
| 6 | nested/replaced content가 text 측정으로 숨겨지는가 | 직접 Text `AAAA`와 one-item 구조를 강제하고 HTML image를 별도 탐색해 거부한다. negative runtime tests가 content/topology를 분리한다. |
| 7 | 다른 namespace의 Grid root가 허용되는가 | root namespace를 HTML로 검증한다. |
| 8 | span 1/3, 잘못된 line, row span이 two-column distribution으로 오인되는가 | column start line 1/end span 2와 자동 row를 typed placement에서 검사한다. unsupported topology negative tests가 거부를 확인한다. |
| 9 | root의 track/gap/width/position 변경이 독립 기대값을 바꾸는가 | max-content 두 열, auto 한 행, 100px width, 0 gap·edges, static Grid profile만 허용한다. |
| 10 | ratio가 실제 owner가 아닌 다른 요소에서 유입되거나 serialization과 typed 값이 다른가 | 한 item의 typed ratio, 정확한 `2 / 1`, sizing box와 ratio 값을 대조하고 geometry 유한성도 검사한다. |
| 11 | no-ratio 대조군이 ratio projection을 상속하는가 | `aspect-ratio:auto`에서 typed ratio와 ratio minmax owner가 없어야 한다. `span-no-ratio`의 두 플랫폼 frame/trace가 통과했다. |
| 12 | unsupported min/max 값이 일부만 적용되어 잘못된 layout을 내는가 | finite 0..=60px fixed 길이 subset만 받는다. percentage/calc/negative/non-finite/상한 초과와 topology negative tests가 fail-closed다. |
| 13 | Ahem font fallback이나 다른 text가 contribution을 바꾸는가 | 정확한 Text `AAAA`, Ahem font/size/line height, writing direction과 paragraph profile을 고정했다. Runtime report는 font registry revision과 측정 폭을 기록한다. |
| 14 | min-width:auto floor와 explicit zero floor를 서로 바꾸는가 | max-height auto-min control은 40px inline floor를 유지하고 zero-min sibling은 30px로 줄어든다. 기대 JSON, Rust trace, Android/iOS trace가 분리한다. |
| 15 | cross-axis min/max 전이가 column span 기여와 row 기여를 혼동하는가 | 양축 contribution을 따로 기대하고 Columns span 2와 Rows span 1 trace를 검사한다. `min-height:30px`와 max-width/height 사례가 통과했다. |
| 16 | fractional 35.5px frame이 정수 픽셀 내림 때문에 sample 좌표와 어긋나는가 | track/frame 기대값은 35.5px를 유지하며 Android sample은 ceil 기반 native 좌표를 사용한다. targeted fractional 표본 4/4 exact였다. |
| 17 | trace의 Initialize 값과 최종 row size가 같은 값으로 잘못 기대되는가 | row contribution 0인 max-height 장면에서 초기 stage와 maximize 이후 base를 구별한다. Android/iOS matcher와 전체 trace가 통과했다. |
| 18 | continuous item paint를 column seam pixel 증거라고 과장하는가 | sample은 item/root/clear 경계만 본다. seam은 per-track trace로 검사하며 glyph와 seam pixel 검증은 근거 범위에서 제외했다. |
| 19 | root 밖 색상과 OS 표면 clear 색을 혼동하는가 | Android root 밖 sample은 검정 SurfaceView clear로 따로 기대한다. iOS 표본은 Metal drawable의 실제 표본이며 compositor 제출 후라고 주장하지 않는다. |
| 20 | AVD의 지연된 stale response가 성공 장면을 실패 장면과 섞거나 재시도 성공으로 둔갑하는가 | 성공 pass15는 scene별 run ID와 matched PixelCopy summary를 요구한다. pass16의 첫 새 scene은 correct layout summary 뒤 draw/pixel이 없어 timeout했으며 별도 실패 로그로 보존해 성공 근거에 넣지 않았다. |

## 발견 및 수정 내역

Runtime bring-up에서 Android/iOS box count, `white-space` 기대값, root 바깥 clear color, row initial stage, fractional pixel coordinate, constrained intrinsic measurements, iOS route allowlist/input count 불일치를 찾았다. 각 원인을 adapter 또는 validator의 해당 profile에 고치고 51-scene simulator run과 runtime tests를 재실행했다. 상세 로그는 [runtime README](./README.md) 및 linked pass log에 있다.

검토 범위는 fixture-only route다. Android API 37 QEMU/SwiftShader는 실기기·hardware GPU가 아니며 iOS Metal readback은 compositor 전이다. 이 검토와 실행은 전체 C11.4/C11 완료를 뜻하지 않는다.
