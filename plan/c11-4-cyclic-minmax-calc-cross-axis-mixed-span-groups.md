# C11.4 · cyclic `minmax(calc())` mixed cross-axis span groups

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [기존 two-item cross-axis 계획](./c11-4-cyclic-minmax-calc-cross-axis-span-groups.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 네 fixed scene의 기준 관측·독립 계산, 계획 실패 관점 20개와 Android/iOS runtime 행렬을 확인했다. PR #515의 네 장면은 두 플랫폼에서 각 4/4 draw/present됐고, definite 두 장면의 overflow·clear 표본은 각각 4/4 exact다. C11.4 전체 회귀 행렬도 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 263/263 draw/present, 표면 표본 219/219 exact(63개 표본 대상 scene)를 확인했다. 이 제한 fixture와 simulator 결과는 일반 CSS 지원 또는 C11.4 완료를 뜻하지 않는다. checkbox와 공식 `72/213`은 유지한다. [최종 회귀 실행·화면·로그](../spec/internal/evidence/c11-4-pr515-prereq-regression-2026-10-22/README.md)

## 목적

기존 cross-axis span-groups slice는 가운데 track을 공유하는 두 span-2 item을 검증한다. 그 계획이 제외한 세 번째 item과 다른 column span을 추가해, span-2 group의 order-independent planned increase와 뒤따르는 span-3 contribution을 확인한다. Column 결과는 각 text item의 wrapping과 row contribution을 바꾸며, 큰 span의 item이 row sizing을 결정하는 definite control도 포함한다.

CSS Grid 2025 CRD §11.3–§11.5.1은 intrinsic size contribution을 점점 큰 span 순서로 처리하고, 각 affected track의 planned increase를 item별 증가값 중 최대로 유지한다. [고정 입력과 독립 계산](../spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/README.md)은 그 규칙을 3-track/2-track fixture에서 확인한다. Chrome 156은 진단 관측으로만 둔다.

## 고정 장면

세 item은 3-column × 2-row Grid에 모두 row span 2로 배치한다. A는 columns `1 / 3`의 auto-size Ahem text `AA A A`, B는 columns `2 / 4`의 fixed `40 × 10px` `XXXX`, C는 columns `1 / 4`의 auto-size Ahem text `AA AA AA AA`다. Track expression은 각 축 `minmax(calc(10% + 2px), max-content)`, gap은 2px, root border는 2px다.

Oracle의 고정 profile은 root `box-sizing: content-box; display: grid; height: max-content; margin: 0; padding: 0; align-content: start; justify-content: start`다. root에는 2px solid `#f0c674` border와 `#101820` background가 있다. 세 item은 모두 direct block child이며 `box-sizing: border-box; min-width: auto; min-height: auto; margin: 0; padding: 0; border: 0`, 저장소 `tests/fixtures/resources/c15/Ahem.ttf`의 400 weight `10px/10px` font, `white-space: normal`, `word-break: normal`, `overflow-wrap: normal`, color/background `#e5484d`를 쓴다. A/C는 auto × auto, B는 40 × 10px다. Font bytes는 사전 비교 evidence의 SHA-256 manifest에 고정한다.

| Scene | Width | DOM order | Root border box | Columns | Rows | Text lines A/B/C |
| --- | --- | --- | --- | --- | --- | --- |
| `intrinsic-forward` | min-content | A, B, C | 55 × 24px | `[15.671875,15.671875,15.65625]` | `[9,9]` | 2 / 1 / 2 |
| `intrinsic-reverse` | min-content | C, B, A | 55 × 24px | `[15.671875,15.671875,15.65625]` | `[9,9]` | 2 / 1 / 2 |
| `definite-forward` | 40px content | A, B, C | 44 × 44px | `[12,12,12]` | `[19,19]` | 3 / 1 / 4 |
| `definite-reverse` | 40px content | C, B, A | 44 × 44px | `[12,12,12]` | `[19,19]` | 3 / 1 / 4 |

Forward/reverse는 document order만 바꾸고 node identity·placement·style을 유지한다. 독립 계산의 full contribution/frame expectations와 Chrome observations는 [사전 비교 README](../spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/README.md)와 JSON을 기준으로 한다.

## 구현 관문

1. 기존 C11.4 opt-in fixture-test profile에서 네 scene ID만 허용한다. 정확히 세 text item, span topology `2/2/3`, 고정 order variant, track expression, font revision 및 sizes를 검증하고 미등록 입력은 partial output 없이 거부한다.
2. `spinon-style-to-layout` trace에서 Columns가 Rows보다 먼저 처리되는지, A/B span-2 bases와 C span-3의 no-growth 기여, 각 item/node ID·span·contribution을 확인한다. Intrinsic constraint bases `[9,19,19]`와 final columns `[15.671875,15.671875,15.65625]`를 분리한다.
3. C의 Columns area는 intrinsic 51px에서 text lines `[50,50]`, definite 40px에서 네 20px lines가 되어 row contribution이 20→40px로 바뀌어야 한다. A는 2→3줄, B는 한 줄을 유지한다. final Rows `[9,9]`/`[19,19]`, root border box 55 × 24/44 × 44px 및 Node별 item frame을 검사한다.
4. AVD PixelCopy definite sample에서 root border 바깥의 B 색과 item 이후 surface clear를 기록한다. iOS Simulator Metal drawable에서 같은 논리 표본을 제출 전에 검사하고 좌표·scale·surface origin을 manifest에 남긴다. Android 실기기는 연결하지 않는다.
5. 기본 release FFI artifact에서 새 scene ID·fixture marker·Ahem payload가 빠지고 opt-in fixture artifact에 포함되는지 대조한다. Focused Rust tests, workspace formatter/lint, FFI checks, Android API 37 ARM64 AVD와 iOS 26.2 Simulator app builds를 실행한다.
6. 기존 C11.4 cyclic minmax cross-axis, overlapping-span, two-axis text, mixed item 및 row-order regression을 재실행한다. **새 구현/runtime 실패 관점 20개**와 별도 PR diff 실패 관점 **30개**를 기록한다. 결함을 찾으면 해당 scene, source order, contribution trace와 surface sample을 다시 확인한다.
7. `spec/internal/0059-c11-grid-layout.md`, C11.4 parent plan, `spec/STATUS.md`, 공개 로드맵 및 runtime README의 지원 제한을 일치시킨다. C11.4 checkbox, top-level item 수, 공식 `72/213`, 공개 CSS 지원 상태는 올리지 않는다.

## 성공·실패 판정

- Intrinsic min-content columns는 constraint bases `[9,19,19]`에서 시작하고 final columns는 독립 quantized expectation의 1/64 CSS px 안이어야 한다. Definite control은 `[12,12,12]`여야 한다.
- Span-2 items는 span-3 C보다 먼저 sizing되고, C의 20px intrinsic contribution은 existing bases 51px보다 작아서 추가 column growth를 만들지 않아야 한다. Three-item order를 뒤집어 track list나 Node별 geometry가 달라지면 실패다.
- Intrinsic A/C line counts는 2/2, definite는 3/4여야 한다. C contribution이 definite row sizing을 결정해 Rows `[19,19]`가 되어야 한다. Stretched frame height를 text contribution으로 잘못 기록하면 실패다.
- Definite item B는 root border를 12px 넘고 실제 표면 sample에서 B 색이 보여야 한다. B가 grid area 안에만 남거나 clear sample이 오염되면 실패다.
- 네 scene 밖의 tree, item 수, text/font, placement, track grammar는 fail-closed해야 한다. 일반 CSS admission 또는 C11.4 완료를 주장하면 실패다.

## 제외 범위

임의 item/span 수, 네 번째 item, 다른 mixed span topology, nested/anonymous/replaced item, row-axis cyclic percentage extension beyond this exact case, arbitrary author CSS, WPT, Android 실기기/hardware GPU, iOS compositor 이후 pixel, glyph raster equality, full CSS Grid support는 포함하지 않는다. 이 slice는 세 fixed direct-text item의 제한 fixture 결과다.

## 검토와 근거

- [Chrome 156 사전 관측과 독립 계산](../spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/README.md)
- [계획 실패 관점 20개](./c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-plan-review-20.md)
- [계획 PR 변경 실패 관점 30개](../spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/plan-pr-diff-review-30.md)
- 기존 [two-item 실행 계획](./c11-4-cyclic-minmax-calc-cross-axis-span-groups.md)과 [실행 evidence](../spec/internal/evidence/c11-4-cross-axis-span-groups-runtime-2026-10-22/README.md)
- [PR #515 최종 행렬·수정한 regression oracle](../spec/internal/evidence/c11-4-pr515-prereq-regression-2026-10-22/README.md)
