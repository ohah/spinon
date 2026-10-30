# C11.4 direct-text ratio span simulator pixel 기준 변경 계획 검토 20개

기준 계획: [`c11-4-direct-text-aspect-ratio-multispan.md`](./c11-4-direct-text-aspect-ratio-multispan.md)

기존 simulator evidence에서 C11.4 text fixture가 glyph를 paint하지 않는 점을 확인해 계획의 pixel acceptance를 정정했다. Track/frame/text overflow geometry와 scene 범위는 바꾸지 않았다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | screenshot에서 검증하지 않은 glyph가 그려졌다고 볼 수 있는가 | **통과.** 기존 Android screenshot은 red/blue/white item 배경만 보이며 Ahem glyph는 보이지 않는다. |
| 2 | text overhang geometry까지 픽셀 증거 제거와 함께 삭제되는가 | **통과.** Chrome Text range, paragraph layout, track trace와 item frame으로 width excess를 계속 고정한다. |
| 3 | PixelCopy/Metal sample이 다른 item의 glyph로 오인되는가 | **통과.** 표본 대상을 item background edge와 item/root 바깥 clear pixel로 제한했다. |
| 4 | item/root clear 점이 같은 색이라 경계 검사가 무의미해지는가 | **통과.** 독립 geometry로 box 안/밖 좌표를 정하고 표면 표본에서는 채움/clear 차이를 검사한다. |
| 5 | 한 item이 연속 paint하는 track seam을 pixel 표본으로 증명하는가 | **통과.** track 경계는 trace의 per-track geometry로만 검증한다. |
| 6 | Chrome visible text overflow를 Spinon glyph paint 결과로 확대하는가 | **통과.** Chrome DOM range는 oracle이고 simulator는 text paint 증거가 아니라고 분리했다. |
| 7 | 기존 9개 case의 독립 track/row 기대값이 바뀌는가 | **통과.** CSS expected values는 그대로 유지된다. |
| 8 | 기존 42개 regression scene 실행 기준이 낮아지는가 | **통과.** Android/iOS 각각 51/51 scene 요구는 그대로다. |
| 9 | item background가 text node의 overflow box를 따라 확장한다고 착각하는가 | **통과.** Item border-box와 direct Text range를 별개 geometry로 계속 비교한다. |
| 10 | 화면 screenshot 한 장을 전체 scene sample receipt로 간주하는가 | **통과.** simulator 수용은 scene별 PixelCopy/Metal sample과 별도 runtime summary를 요구한다. |
| 11 | Android software renderer를 hardware GPU proof라고 표현하는가 | **통과.** AVD software-rendering 한계가 유지된다. |
| 12 | iOS pre-compositor drawable을 final-screen parity라고 표현하는가 | **통과.** compositor submission 전 표본이라는 한계가 유지된다. |
| 13 | glyph 미지원 한계가 이 feature의 CSS track contribution을 불필요하게 막는가 | **통과.** scope는 intrinsic sizing; text metrics/overflow geometry는 별도 측정한다. |
| 14 | item box 외부의 text overhang을 box overflow와 혼동하는가 | **통과.** plan은 layout overflow와 painted item background bounds를 각각 검증한다. |
| 15 | 고정 HTML의 Chrome oracle와 Android/iOS scene source가 서로 다른 입력이 되는가 | **통과.** 동일한 nine CSS constraint case와 dimensions를 따르며 surface sample 역할만 다르다. |
| 16 | screenshot 한계 수정이 simulator matrix를 생략할 근거가 되는가 | **통과.** 51 scene per platform과 PixelCopy/Metal surface checks는 계속 요구한다. |
| 17 | 계획 변경이 제품 코드/내부 계약을 건드리는가 | **통과.** 이번 PR은 planning/evidence scope만 수정하고 0059 구현 갱신은 feature merge 뒤에 한다. |
| 18 | C11.4/C11 checkbox 또는 공식 `72/213`이 달라졌는가 | **통과.** 공식 상태 대장은 변경하지 않고 두 값을 그대로 둔다. |
| 19 | 예전 evidence의 긍정적 화면을 과장 없이 인용하는가 | **통과.** [Android screenshot](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/android-avd/c114-text-ratio-mvut1ldd-e32f3c4c/grid-text-aspect-ratio-minmax-min-width-45.png)과 해당 runtime README를 연결했다. |
| 20 | implementation이 sample coordinate를 기대 결과를 본 뒤 정할 수 있는가 | **통과.** 좌표는 simulator 실행 전에 independent geometry에서 고정하도록 유지한다. |

이 검토는 acceptance criteria 수정 범위만 대상으로 했다. 제품 구현, simulator 재실행 또는 C11.4 완료 판정이 아니다.
