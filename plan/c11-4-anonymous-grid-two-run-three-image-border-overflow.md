# C11.4 두 anonymous text run과 세 spanning image의 bordered Grid plan

**상태:** opt-in fixture 구현과 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 검증을 마쳤다. 기존 bordered two-image 장면을 포함한 네 장면이 두 플랫폼 모두 4/4 통과했고 PixelCopy·Metal drawable 표본은 각각 8/8 exact다. Rust focused tests, workspace·fixture Clippy, 두 앱 빌드와 기본/opt-in archive boundary도 통과했다. [실행 근거](../spec/internal/evidence/c11-4-anonymous-text-two-run-three-image-border-overflow-2026-10-21/README.md) · [구현 실패 관점 20개](../spec/internal/evidence/c11-4-anonymous-text-two-run-three-image-border-overflow-2026-10-21/implementation-review-20.md). PR diff의 별도 30관점 검토는 [검토 기록](../spec/internal/evidence/c11-4-anonymous-text-two-run-three-image-border-overflow-2026-10-21/pr-diff-review-30.md)에 있다. C11.4 checkbox와 공식 완료 수는 바꾸지 않는다.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [anonymous Grid text와 세 image](./c11-4-anonymous-text-multispan-images.md) · [두 anonymous text run과 두 image border/overflow](./c11-4-anonymous-grid-border-overflow.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

## 목적

별도 제한 fixture에서 두 anonymous text run + 두 image의 border/overflow, 한 anonymous text run + 세 spanning image의 intrinsic sizing을 확인했다. 두 text run, 세 spanning image, border와 root 바깥 overflow를 한 Grid 계산에 합친 실행 근거는 없다. 이 후속은 세 조건을 같은 document/style revision에 결합하고 span-3 image의 34px border-box overflow 및 border 오른쪽 22px 밖의 실제 색 표본을 두 source order로 확인한다.

## 고정 입력과 oracle

입력은 [Chrome fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-two-run-three-image-border-overflow.html)에 고정한다.

- Root: `box-sizing:border-box`, `40×184px`, 네 변 `2px solid #f0c674`, 배경 `#101820`, `overflow:visible`, 네 개 `min-content` columns, 세 개 `60px` rows, gap 0.
- Text: 직접 자식 Ahem `10px/10px`의 `XX` run 두 개. 각각 20px min-content를 기여하고, Grid item placement는 anonymous item auto-placement에 맡긴다.
- Images: 같은 `24×16` PNG, `width:auto`, heights `16px/32px/48px`, natural ratio `3:2`, inline min-content contributions `24px/48px/72px`.
- Placement: A=`column 1 / span 2, row 1`; B=`column 2 / span 2, row 2`; C=`column 1 / span 3, row 3`. 명시 배치된 image 이후 text run은 row 1 columns 3과 4에 놓인다.
- Source-order controls: `two-anonymous-runs-three-images-forward`, `three-images-two-anonymous-runs-reversed`. DOM 자식 순서만 뒤집는다.
- Chrome 156 Agent Browser screenshot, CSSOM JSON, 입력 hash, PNG와 Ahem hash는 [사전 비교 근거](../spec/internal/evidence/c11-4-anonymous-text-two-run-three-image-border-overflow-precomparison-2026-10-21/README.md)에 고정했다. Chrome observed first three columns `[16,32,24]`는 CSS Grid 2025 CRD §11.4–§11.5.1 독립 기대값과 분리해 유지한다.

기준은 [CSS Grid Layout Module Level 1, 26 March 2025 CRD §6, §8.5, §11.4–§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)이다. 직접 text sequence는 별도 anonymous block-container Grid item을 만들며, placement 속성이 없으므로 두 run은 image가 점유한 cell 다음 row 1 columns 3과 4에 자동 배치된다. Text track의 min-content base/growth limit은 20px이다.

첫 span-2 image의 24px contribution은 columns 1–2를 12px씩 만든다. 두 번째 image의 48px contribution은 columns 2–3 합 32px에 대해 남은 16px을 growth limit이 무한한 column 2에 준다. 따라서 첫 두 image 이후 columns는 `[12,28,20]`이다. 이 span-2 contribution들의 growth-limit pass에서 무한 limit을 base sizes로 치환하면 `12+12=24`, `28+20=48`이어서 추가 공간이 남지 않으므로 columns 1–2의 growth limit은 무한으로 유지된다. Span-3 image의 contribution 72px은 첫 세 base sizes 합 60px에서 12px을 더 요구한다. Up-to-limit 단계는 columns 1–2에 6px씩 더하고 20px limit에 도달한 column 3은 그대로 둔다. 두 번째 anonymous text run은 column 4를 20px로 만든다. 독립 기대 track은 두 source order에서 `[18,34,20,20]`이다.

Chrome 156 observed track은 `[16,32,24,20]`이며 browser CSSOM에는 중간 growth-limit state가 없다. 이 plan은 차이를 앱 판정에서 숨기지 않지만 Chrome 결과를 표준 기대값으로 쓰지 않는다. 원인이나 전체 Chrome/CSS Grid 적합성을 이 fixture 하나로 단정하지 않는다.

## 구현 및 검증 관문

1. 기존 opt-in `RuntimeBlockFlexGridTrackSizingV1` fixture-test 경로에서 두 고정 장면만 allowlist한다. Unsupported topology/style/source와 알 수 없는 scene은 fail closed한다.
2. 같은 document/style/layout revision에서 두 anonymous text node와 anchor/source order, 세 image node ID·natural size·resource revision, Columns/Rows stage와 contribution trace를 검사한다. Image request 누락·중복 ID·decode 실패에서 부분 snapshot/layout을 게시하지 않는다.
3. 각 scene에서 네 used columns `[18,34,20,20]`, 세 row tracks `[60,60,60]`, root border-box `40×184`, content width `36`, anonymous item placement/frame, image frame과 34px 오른쪽 overflow를 검사한다. 두 source order의 output은 같아야 한다.
4. Chrome 156 사전 관측 `[16,32,24,20]`을 결과에 별도로 보존하고, CSSOM evidence와 screenshot은 Spinon app이나 simulator evidence로 재분류하지 않는다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 두 장면 모두 `drawPresented`, frame, trace/revision, 전체 ID 집합을 확인한다. 사용자가 simulator를 선택했으므로 physical Android/iOS 기기는 이 계획의 범위가 아니다.
6. 앱 SurfaceView PixelCopy와 iOS Metal drawable에서 border 표본 `(root-local x=10,y=0) = #f0c674`, border 오른쪽 22px 바깥 image 표본 `(root-local x=62,y=128) = #00ff40`을 확인한다. Root-local-to-surface transform, DPR, scale과 screenshot을 보존한다. iOS readback은 compositor submission 전이며 OS scanout proof가 아니다.
7. Runtime/FFI focused tests, `cargo fmt --all -- --check`, workspace Clippy, Android/iOS fixture app build를 수행한다. 기본 `spinon-ffi` artifact에는 fixture symbol/scene/payload가 빠지고 opt-in fixture-test positive control에만 있어야 한다.
8. 구현 코드/runtime 결과는 이 계획 검토와 다른 20개 실패 관점으로, PR 전체 변경은 프로젝트 규칙에 따라 별도 30개 관점으로 검토한다. PR 본문에는 Chrome 및 Android/iOS simulator 화면을 넣고, live required checks가 있으면 모두 통과한 뒤 Rebase and merge한다.

## 범위 밖

이 slice는 고정 Ahem `XX`, 고정 24×16 PNG, 두 anonymous run, 세 direct placed image, 두 source order, 네 `min-content` columns, 단색 opaque solid border 및 `overflow:visible` 한 장면에 한정한다. Arbitrary HTML/CSS, hidden/whitespace/comments 조합, nested Grid, dynamic image replacement, 다른 track function/gap/writing mode, clipping/scroll, full PNG equality, WPT, physical device, compositor 이후 pixel과 전체 C11.4/CSS Grid 지원은 주장하지 않는다. C11.4 checkbox와 공식 `72/213` 완료 집계는 유지한다.

## 근거

- [Chrome 156 사전 비교·화면·hash](../spec/internal/evidence/c11-4-anonymous-text-two-run-three-image-border-overflow-precomparison-2026-10-21/README.md)
- [고정 Chrome fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-two-run-three-image-border-overflow.html)
- [계획 실패 관점 20개](./c11-4-anonymous-grid-two-run-three-image-border-overflow-plan-review-20.md)
- [표준 track 계산 수정](../spec/internal/evidence/c11-4-anonymous-text-multimage-precomparison-2026-10-21/README.md#2026-10-21-표준-계산-재검토)
- [Simulator 실행·검증 근거](../spec/internal/evidence/c11-4-anonymous-text-two-run-three-image-border-overflow-2026-10-21/README.md)
- [구현 실패 관점 20개](../spec/internal/evidence/c11-4-anonymous-text-two-run-three-image-border-overflow-2026-10-21/implementation-review-20.md)
- [PR diff 실패 관점 30개](../spec/internal/evidence/c11-4-anonymous-text-two-run-three-image-border-overflow-2026-10-21/pr-diff-review-30.md)
