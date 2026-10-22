# C11.4 cyclic `minmax(calc())` mixed text and replaced-image span groups

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [text/image mixed-span precedent](./c11-4-mixed-text-image-multispan-contributions.md) · [cyclic replaced-image precedent](./c11-4-cyclic-minmax-replaced-image-span.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** Chrome 156 Agent Browser 사전 관측과 CSS Grid 2025 CRD 독립 기대값을 네 fixed scene으로 기록했고, 계획 실패 관점 20개를 검토했다. Runtime 구현은 시작하지 않았다. 이 제한 slice는 C11.4 checkbox와 공식 `77/213`을 올리지 않는다.

## 목적

기존 cyclic `minmax(calc())` mixed cross-axis fixture는 text item 세 개를, cyclic replaced-image fixture는 image item 세 개를 사용한다. 기존 mixed text/image multispan fixture는 둘을 한 Grid에 넣었지만 cyclic percentage를 쓰지 않는다. 이 계획은 그 조합 공백만 닫는다. 한 3×2 Grid에서 span-2 Ahem text와 span-2 image가 같은 planned-increase group에 들어가고 span-3 Ahem text가 뒤에 sizing되는지, Columns 결과와 replaced-image aspect ratio가 Rows contribution에 전달되는지 검사한다.

PR #515의 topology·text와 PR #520의 image resource/replaced-item 경로를 바탕으로 한다. 이는 두 기존 PR을 다시 검증하거나 일반 mixed-content CSS를 지원하는 작업이 아니다. 새 scene은 `RuntimeBlockFlexGridTrackSizingV1` opt-in fixture-test 경계에만 추가한다.

## 고정 입력과 장면

사전 비교 파일은 [CSS oracle](../tests/fixtures/css/c11/grid-cyclic-minmax-mixed-text-image-span-c114-precomparison.html)이다. 네 scene은 3-column × 2-row Grid, 모든 track의 `minmax(calc(10% + 2px), max-content)`, 양 축 2px gap, 2px border를 사용한다. A와 C는 저장소 Ahem 400 10px/10px direct text다. B는 `c14-1-png-basic.png` direct `<img>`이며 bytes의 natural size는 24×16px, CSS `width`/`min-width`는 40px, `height`는 `auto`다. 명시한 minimum size가 image inline contribution을 40px로 고정하고, 자연 3:2 aspect ratio가 block contribution `80/3px`을 만든다. A는 columns `1 / 3`, B는 `2 / 4`, C는 `1 / 4`; 세 item 모두 rows `1 / 3`이다.

| Scene | Root content width | DOM order | 독립 Columns | 독립 Rows | Root border box | A/B/C line count |
| --- | ---: | --- | --- | --- | --- | --- |
| `intrinsic-forward` | 51px min-content | A, B, C | `[15.671875, 15.671875, 15.65625]` | `[37/3, 37/3]` | `55 × 92/3px` | 2 / image / 2 |
| `intrinsic-reverse` | 51px min-content | C, B, A | same | same | same | same |
| `definite-forward` | 40px | A, B, C | `[12, 12, 12]` | `[19, 19]` | `44 × 44px` | 3 / image / 4 |
| `definite-reverse` | 40px | C, B, A | same | same | same | same |

Forward/reverse changes only source order. Item identity, placement, style, font revision, image revision and all other values remain fixed. Chrome's complete CSSOM values, line rectangles, frames, viewport and screenshot are separate from [spec-derived-expectations.json](../spec/internal/evidence/c11-4-cyclic-minmax-mixed-text-image-span-precomparison-2026-10-23/spec-derived-expectations.json).

## 독립 계산과 Chrome 관측

CSS Grid 2025 CRD §7.2.1 treats the cyclic percentage as auto during intrinsic sizing and resolves it against the resulting axis size. Under §11.3–§11.5.1, the same-span contributions propose per-track increases before the per-track maximum is applied: A's 20px min-content text proposes `[7,7,0]`, B's 40px image contribution proposes `[0,17,17]`, and the combined increase is `[7,17,17]`. Adding these to the initial `[2,2,2]` bases gives `[9,19,19]`. C's 20px min-content contribution is smaller than the existing 51px span including gutters and causes no further growth. The independent intrinsic content width is therefore 51px.

Columns then determine text wrapping. At intrinsic width, A has 20px and 30px lines, C has two 50px lines, and the image contributes block size `80/3px`. Rows start at `[2,2]`; with the 2px gutter, the image's 80/3px contribution proposes `31/3px` per track, yielding rational Rows `[37/3,37/3]` and intrinsic content height `80/3px`. The 40px definite-width control wraps A to three lines and C to four; C's 40px block contribution gives Rows `[19,19]`. The image ends 12px beyond the definite root's right border, so the simulator gate includes a sample in the image outside the border and a sample after its painted extent.

Chrome 156 Agent Browser observed the same track sizes, root boxes, item frames, text line widths and source-order invariance. CSSOM values for the rational `80/3px` image-driven height are quantized to 1/64 CSS px; every used value is compared against the independent calculation with at most 1/64 CSS px tolerance. The browser capture is an observation, not the source of the expectations. See the [observation JSON](../spec/internal/evidence/c11-4-cyclic-minmax-mixed-text-image-span-precomparison-2026-10-23/chrome156-observation.json), [environment and asset revisions](../spec/internal/evidence/c11-4-cyclic-minmax-mixed-text-image-span-precomparison-2026-10-23/browser-environment.json), and [Chrome screenshot](../spec/internal/evidence/c11-4-cyclic-minmax-mixed-text-image-span-precomparison-2026-10-23/chrome-agent-browser-precomparison.png).

## 구현 관문

1. 이 계획을 구현과 별개인 20개 실패 관점으로 검토하고 계획 자체의 누락을 먼저 수정한다. 계획 PR 변경도 별도 20개 관점으로 검토한다.
2. 최신 `main`에서 같은 checkout을 사용하고 새 worktree를 만들지 않는다. 네 scene ID, source order, exact three-item tree, Ahem bytes, image bytes/natural size, CSS track list, placement와 style을 allowlist하고 변형은 부분 입력 없이 거부한다.
3. Text intrinsic input 두 개와 replaced-image input 하나가 같은 document/layout revision에 들어가는지 확인한다. Node identity·item order·source kind·image revision·aspect ratio·axis를 trace에서 구별하고, span-2 A/B의 planned-increase maximum과 뒤 span-3 C를 분리해 검증한다.
4. Columns가 확정된 뒤 text line widths로 Rows contribution을 측정한다. intrinsic rational Rows와 definite `[19,19]`를 구분하고, stretched item frame을 intrinsic contribution으로 재사용하지 않는다. Reverse source order는 item-keyed geometry와 track sizes를 바꾸지 않아야 한다.
5. 기존 fixture-test feature만 사용한다. 기본 product artifact에 새 FFI symbol·scene marker·image/text payload가 없고 opt-in positive control에는 모두 있는지 확인한다. 기존 cyclic minmax, replaced-image, mixed text/image, text wrapping, cross-axis span regression을 실행한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 새 네 scene과 관련 C11.4 regression을 실행한다. 물리 Android 연결은 하지 않는다. Root/item frame·track stage·image draw·draw/present를 확인하고, PixelCopy 및 compositor 제출 전 Metal 표본을 서로 구분한다.
7. Definite forward/reverse scene에서 root border 밖의 B image pixel과 image 뒤 surface clear를 각 simulator 표면에서 검사한다. Renderer가 CSS border stroke를 그리지 않는 경계와 iOS compositor 제출 이후 미검증을 명시한다.
8. Formatter, focused Rust/style/layout/FFI checks, workspace Clippy, Android/iOS fixture build와 fixture artifact 경계를 실행한다. 실패를 수정하고 영향 scene 및 표본을 다시 확인한다.
9. 구현과 runtime을 계획과 독립된 새 실패 관점 20개로 검토한다. PR diff에는 사용자가 요청한 별도 20개 관점을 적용한다. 재사용·중복한 관점은 세지 않는다.
10. PR 본문에는 실제 AVD/iOS Simulator screenshot raw image를 각각 연결하고, required check 및 merge state를 확인한 뒤 Rebase and merge한다. 병합 후 Tailnet roadmap을 동기화하고 Agent Browser로 화면을 확인한 뒤 브라우저 세션과 임시 로컬 HTTP server를 종료한다.

## 실패 판정과 제외 범위

- 같은 span-2 group의 text/image contribution이 item 순서에 따라 track bases를 바꾸거나 image source kind/revision이 사라지면 실패다.
- Image 40px inline contribution, `80/3px` block contribution, intrinsic constraint bases `[9,19,19]`, final intrinsic Columns 및 Rows, definite controls가 독립 기대값의 1/64 CSS px를 넘으면 실패다.
- Text line count·width, root/item frame, overflow edge와 clear sample을 fixture 기대값과 대조한다. Geometry만 통과하고 image가 그려지지 않거나 border 밖 sample이 다른 색이면 실패다.
- Unknown scene/tree/font/image/style, incomplete revision, partial contribution list, unsupported profile 또는 output overflow는 전체 fixture를 fail-closed해야 한다.

네 fixed direct-child scene 외의 arbitrary text/image content, nested/anonymous/out-of-flow item, 다른 image decoder·URL·font, 여러 span topology, general CSS admission, WPT, 물리 Android/hardware GPU, iOS compositor 이후 pixel, CSS border stroke 및 전체 C11.4/C11 완료는 제외한다. C11.4 checkbox와 `77/213`은 계속 유지한다.

## 근거와 검토

- [Chrome 156 관측·독립 기대값·asset hash](../spec/internal/evidence/c11-4-cyclic-minmax-mixed-text-image-span-precomparison-2026-10-23/README.md)
- [계획 실패 관점 20개](./c11-4-cyclic-minmax-mixed-text-image-span-plan-review-20.md)
- [text/image mixed multi-span 선행](./c11-4-mixed-text-image-multispan-contributions.md)
- [cyclic cross-axis text precedent](./c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups.md) · [cyclic replaced-image precedent](./c11-4-cyclic-minmax-replaced-image-span.md)
- [PR #515 precondition/regression evidence](../spec/internal/evidence/c11-4-pr515-prereq-regression-2026-10-22/README.md) · [PR #520 runtime evidence](../spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-runtime-2026-10-23/README.md)
