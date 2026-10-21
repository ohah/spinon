# C11.4 직접 text와 image의 mixed multi-span 기여 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** Chrome 155 사전 비교와 계획 실패 관점 20개 검토를 마쳤다. 제한된 runtime fixture 구현과 네 mixed scene의 Rust·Android API 37 AVD·Android 실기기·iOS Simulator 실행을 마쳤다. [구현 PR #419](https://github.com/ohah/spinon/pull/419)은 2026-10-21 01:25:11 UTC에 Rebase and merge됐고 merge SHA는 `1e2c5352ff9932a776420fdf533c9d040e4a0746`이다. PR check rollup은 비어 있었고 `main`의 required status check는 설정되어 있지 않았다. C11.4와 공식 완료 수는 미완료로 유지한다.

## 목적과 범위

기존 text와 image multi-span fixture는 각각 다른 Grid에서 직접 자식 item을 검사한다. 한 Grid의 span group에 text의 Parley intrinsic input과 image의 replaced-size input이 함께 들어갈 때 contribution collection과 stage grouping이 함께 올바른지는 확인하지 않았다. 이번 제한 slice는 서로 다른 source 종류를 섞어도 CSS Grid의 같은 span sizing 절차에서 결합되는지 확인한다.

고정 fixture는 하나의 definite `220×180px` Grid, 네 개 `min-content` column, 세 개 fixed `60px` row, gap 0을 사용한다. item은 Ahem direct-leaf `<div>` text와 고정 PNG direct `<img>`의 조합이다. text `XX XXXXX`의 독립 min-content 폭은 50px, 끊을 수 없는 18개의 `X`는 180px이다. image natural size는 24×16이고 CSS `height:48px;width:auto`에서 transferred inline size 72px가 된다. 각 item은 별도 row에 있어 cross-axis overflow나 overlap 없이 text span-2를 column 1–2, image span-2를 2–3, 긴 text span-3을 1–3에 놓는다.

네 scene은 두 span-2 item의 DOM 순서를 뒤집은 대조와 span-3 item의 DOM 위치를 앞뒤로 이동한 대조다. 기대 열 크기는 각각 `[25,36,36,0]`, `[52⅔,63⅔,63⅔,0]`이며 독립 계산과 Chrome 관찰은 [사전 비교 근거](../spec/internal/evidence/c11-4-mixed-text-image-multispan-precomparison-2026-10-21/README.md)에 고정했다.

이 제한 slice에서 지원하지 않는 입력은 다음과 같다.

- Grid root 외에 직접 자식 2개 또는 3개, anonymous/nested item, 여러 text node, 다른 item 종류, 다른 child order
- 다른 Ahem text, image URL/bytes/natural size, font/style, row/column placement, track/gap, writing mode 또는 CSS property
- async image replacement, mixed text/image descendant 한 item, implicit placement, auto-repeat 조합, arbitrary CSS, WPT와 공개 CSS 지원

모든 입력은 fixture allowlist로 고정한다. 알 수 없는 scene이나 topology/style/source revision은 부분 intrinsic input 또는 부분 LayoutOutput으로 전환하지 않고 기존 전체 거부 경계에서 실패해야 한다. fixture FFI 및 payload는 opt-in fixture-test artifact에만 둔다. 공개 API, CSS 지원 계약, 내부 계약 버전은 바꾸지 않는다.

## 표준 기준과 판정

CSS Grid §11.5.1의 span별 grouping과 planned-increase 규칙을 적용한다. span-2 group에서는 text 50px와 image 72px 각각의 증가를 source 종류와 무관하게 track별로 모아 `[25,36,36,0]`을 얻는다. 다음 span-3 group에서 text 180px 기여는 현재 첫 세 열의 97px을 넘어 83px을 더해 `[52⅔,63⅔,63⅔,0]`을 만든다. 이 독립 계산을 구현 결과를 보고 조정하지 않는다. Chrome 관찰은 fractional CSSOM quantization을 허용 범위 안에서 비교한다.

## 구현 및 검증 관문

1. 계획 PR을 Rebase and merge한 뒤 최신 `main`의 같은 checkout에서 구현 branch를 만든다. 별도 worktree는 만들지 않는다.
2. 기존 `RuntimeBlockFlexGridTrackSizingV1` fixture 경계를 사용해 text/image item이 한 revision의 같은 Grid intrinsic-input 목록에 모두 들어오는 네 개의 고정 scene을 연결한다. scene/root/item ID, source order, text, image source/revision, style, placement, track 수를 검증한다.
3. Rust 검사에서 source 종류별 intrinsic input과 revision, item/node mapping, span group 순서, contribution trace, final track과 모든 root-relative frame을 독립 기대값에 대조한다. forward/reversed DOM order가 같은 track 값을 내는지 확인한다.
4. Android API 37 ARM64 AVD, Samsung SM-S731N Android 16/API 36 실기기(serial 명시), iPhone 17 Pro/iOS 26.2 Simulator에서 네 scene을 실행한다. 각 실행은 전체 scene 수, finite frame/track 수치, trace completeness, draw/present, APK hash(실기기)를 확인한다. 기존 실행 자료와 새 코드는 분리한다.
5. 기본 product artifact에서 fixture FFI symbol·scene 문자열·payload가 빠지고 opt-in positive-control에는 포함되는지 대조한다. C11.4 기존 profiles의 회귀, `cargo fmt`, focused Rust/runtime/FFI checks, workspace Clippy, Android/iOS fixture build를 실행한다.
6. 구현/runtime 코드를 새로운 실패 관점 20개로 검토한다. 구현과 runtime에서 찾은 결함은 고친 뒤 영향 관점을 다시 실행하고 원인을 evidence에 적는다.
7. 구현 PR 변경분은 이 계획 검토와 중복하지 않는 20개 실패 관점으로 별도 검토한다. PR body에는 실제 발견·수정과 남은 한계를 기록하고, required checks가 있으면 모두 통과한 뒤 Rebase and merge한다.

## 완료 판정과 한계

- 네 scene의 Chrome observation, 독립 rational track 기대값, Ahem/image decode 준비 조건과 input hash가 고정돼야 한다.
- text 2개 intrinsic measurement와 image 1개 natural/transferred-size input이 한 layout revision에 함께 연결되고, contribution trace에서 종류·node·span이 정확히 보존돼야 한다.
- Rust 및 Android AVD, Android 실기기, iOS Simulator에서 각 네 scene의 track/frame/stage/draw/present가 통과해야 한다. 실기기 APK identity와 기본/fixture artifact 경계를 기록한다.
- PixelCopy 색 표본을 추가하면 해당 SurfaceView의 sample 위치만 주장한다. PNG texel paint, 모든 CSS border stroke, iOS compositor 이후 pixel은 확인 전까지 미검증이다.
- 이 fixture는 direct-leaf Ahem text/image 세 item에 한정된다. 일반 text/image matrix, nested/anonymous item, hidden/out-of-flow item, all track sizing stages, WPT, 전체 C11.4/C11 또는 공개 CSS Grid 지원 완료를 주장하지 않는다.

## 근거

- [Chrome 사전 비교, observation, screenshot, hashes](../spec/internal/evidence/c11-4-mixed-text-image-multispan-precomparison-2026-10-21/README.md)
- [실행 fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-mixed-multispan-precomparison.html)
- [계획 적대 검토 20개](./c11-4-mixed-text-image-multispan-plan-review-20.md)
- [계획 PR 변경분 적대 검토 20개](../spec/internal/evidence/c11-4-mixed-text-image-multispan-precomparison-2026-10-21/plan-pr-diff-review-20.md)
- [구현/runtime 실패 관점 검토 20개](../spec/internal/evidence/c11-4-mixed-text-image-multispan-2026-10-21/implementation-review-20.md)
- [구현 PR diff 실패 관점 검토 20개](../spec/internal/evidence/c11-4-mixed-text-image-multispan-2026-10-21/pr-diff-review-20.md)
- [구현 병합 후 roadmap 문서 PR 적대 검토 30개](../spec/internal/evidence/c11-4-mixed-text-image-multispan-2026-10-21/postmerge-docs-pr-review-30.md)
- [roadmap Agent Browser 화면](/spinon/internal/evidence/c11-4-mixed-text-image-multispan-2026-10-21/roadmap-agent-browser-pass34.png) · [공식 완료 집계 72/213 화면](/spinon/internal/evidence/c11-4-mixed-text-image-multispan-2026-10-21/roadmap-counter-agent-browser-pass34.png)
