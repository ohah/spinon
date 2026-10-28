# C11.4 · generic HostNode ID 기반 mixed text/image spanning runtime

**상위 계획:** [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 구현 및 Android/iOS Simulator 검증 완료, [PR #725](https://github.com/ohah/spinon/pull/725) 검토 대기. 기존 mixed text/image multi-span의 Chrome 155 관측과 §11.5.1 독립 기대값을 재사용해, 네 fixed scene이 새 HostNode ID를 사용해도 opt-in runtime adapter에 도달함을 확인했다. 일반 CSS admission, WPT 적합성, C11.4 완료 또는 공식 완료 수 변경은 범위에 포함하지 않는다.

## 목적

현재 `c11_4_mixed::prepare`의 mixed spanning scene은 root와 item HostNode ID가 `SCENES`의 고정 ID와 일치해야 한다. 같은 mixed adapter의 direct column/row 경로는 ID 대신 document content와 computed style을 검사한다. 이 계획은 기존 multi-span 계산 입력을 그대로 유지하면서 그 identity 경계를 같은 제한 fixture profile 안에서 보완한다.

기존 CSS·content·placement 및 독립 기대값은 [mixed multi-span 계획](./c11-4-mixed-text-image-multispan-contributions.md), [고정 Chrome 155 관측](../spec/internal/evidence/c11-4-mixed-text-image-multispan-precomparison-2026-10-21/chrome155-observation.json), [원본 fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-mixed-multispan-precomparison.html)에 고정되어 있다. authored style이나 layout 알고리즘을 바꾸지 않으므로 이 후속에서 새 브라우저 관측을 기존 observation으로 가장하지 않는다.

## 고정 입력과 독립 기대값

네 장면 모두 기존 Android/iOS fixture launcher scene ID를 유지해 이전 app matcher를 재사용한다. HTML root/item `id` attribute는 `generic-host-` 접두사를 사용해 기존 `c114-mixed-text-image-span*` 고정-ID route와 분리한다. 따라서 app launcher의 scene ID가 같아도 `c11_4_mixed::prepare`에서는 고정 scene으로 잡히지 않고 generic route를 통과해야 한다. HTML `div` grid root는 `220×180px`, `box-sizing:border-box`, 네 `min-content` columns, 세 `60px` rows, gap·border·padding 0, `align-items/content`와 `justify-items/content` start를 사용한다.

- A: Ahem 10px/10px `div` text `XX XXXXX`, `grid-column:1 / span 2`, row 1. 독립 min-content contribution은 50px.
- B: leaf HTML `img`, 고정 PNG `c14-1-png-basic.png`의 natural size 24×16px, CSS `height:48px;width:auto`, `grid-column:2 / span 2`, row 2. transferred inline contribution은 72px.
- C: Ahem 10px/10px `div` text `XXXXXXXXXXXXXXXXXX`, `grid-column:1 / span 3`, row 3. 독립 min-content contribution은 180px.
- Source order는 A,B / B,A 및 A,B,C / C,B,A의 네 조합만 허용한다. Placement는 DOM 순서와 무관하게 고정한다.

고정 asset SHA-256은 Ahem `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`, PNG `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`다. 실행 중 같은 tracked asset hash와 generated app source hash를 runtime evidence에 기록한다.

CSS Grid 2025 CRD §11.5.1의 span 길이별 group 및 track별 최대 planned increase에서 두 span-2 contribution은 columns `[25,36,36,0]px`를 만든다. 이어지는 span-3 contribution은 `[158/3,191/3,191/3,0]px`를 만든다. 네 scene의 rows는 `[60,60,60]px`, root frame은 `(0,0,220,180)`이다. Root-relative item 기대 frame은 span-2 scene에서 A `(0,0,61,20)`, B `(25,60,72,48)`이고, span-3 scene에서 A `(0,0,116⅓,10)`, B `(52⅔,60,72,48)`, C `(0,120,180,10)`이다. Reverse source order에서도 item 종류와 내용별 frame은 같아야 한다. 구현은 독립 rational 값과 기존 Chrome 관측을 각각 대조하며 Chrome 값을 규범 oracle로 사용하지 않는다. 기존 사전 비교 README의 root-relative 변환과 tolerances를 그대로 쓴다.

## 구현 계약

1. 기존 네 opt-in launcher scene의 HTML source가 고정 route와 다른 root/item HostNode ID를 생성하게 한다. 기존 fixed-ID scene unit tests는 유지해 fixed/generic 두 adapter를 각각 확인한다. 기본·제품 artifact에는 fixture scene/payload/ABI가 남지 않고 fixture-test artifact에는 positive control로 포함되어야 한다.
2. runtime adapter는 profile, generation/document/render-tree/style revision, root namespace/tag/parent, exact child count/order, text content/font/style, PNG source/decode/natural size, transferred height, grid track/placement/style을 함께 검사한다. CSS sizing 알고리즘이나 public input 허용 범위는 변경하지 않는다.
3. Known fixed-ID scene은 기존 adapter가 계속 처리한다. Unknown ID지만 mixed signature와 spanning shape가 있는 잘못된 입력은 text-only/image-only/부분 contribution으로 강등하지 않고 거부한다. non-mixed unrelated root는 기존 `Ok(None)` fallback 동작을 유지한다.
4. Rust tests는 네 scene의 새 root/item NodeId, 같은 document/style revision, text 2개와 image 1개의 intrinsic input 집합, span group trace, rational track values, root/item frame을 검사한다. DOM source order를 바꾸어도 identity별 geometry와 tracks가 유지되어야 한다.
5. Negative controls는 stale revision, 다른 profile, 누락·추가·재정렬 child, 변경된 text, 두 번째 text/image, 잘못된 PNG/height, placement span/row 변경, nested/non-leaf image 및 unsupported track style을 포함한다. 실패 시 부분 mixed input/layout을 게시하지 않는다. 기존 direct generic row/column 및 fixed mixed multi-span controls도 회귀 검사한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 새 장면을 새 source commit으로 빌드해 실행한다. 4/4 scene별 trace, columns/rows, root/item frame, draw/present와 정확한 app/source hash를 보존한다. 실기기는 범위 밖이다. 화면 증거는 geometry 배치만 보조하며 glyph/PNG texel 또는 compositor 이후 결과를 뜻하지 않는다.
7. 기본/product 및 fixture-test FFI archive 경계를 negative/positive control로 확인한다. 적용되는 focused Rust/FFI tests, `cargo fmt`, feature-specific Clippy, Android/iOS fixture app build를 실행하고 미실행 검증은 미검증으로 표시한다.
8. 구현/runtime 변경은 계획 검토와 다른 20개 실패 관점으로 검토한다. 실제 PR diff도 20개 서로 다른 실패 가설로 검토한다. 결함 수정 뒤 영향 관점을 다시 실행한다. PR은 Rebase and merge한다.
9. 성공해도 `spec/STATUS.md`의 C11.4 checkbox와 공식 완료 집계 `72/213`을 유지한다. 이 slice만으로 일반 CSS support, WPT 통과 또는 전체 C11.4를 주장하지 않는다.

## 범위 밖

Arbitrary direct item 수와 source tree, anonymous/nested Grid, 다른 font/script/image, dynamic image replacement, 다른 writing mode/track function/gap, 일반 CSS author admission, WPT suite, physical device, Android hardware GPU, iOS compositor 이후 pixel, public API와 전체 C11.4는 범위 밖이다.

## 검토 기록

- 서로 다른 계획 실패 관점 20개: [계획 검토](./c11-4-generic-mixed-spanning-runtime-plan-review-20.md)
- 기존 Chrome 관측·독립 기대값·asset hash: [사전 비교](../spec/internal/evidence/c11-4-mixed-text-image-multispan-precomparison-2026-10-21/README.md)
- 최종 runtime·시뮬레이터·archive 근거: [evidence](../spec/internal/evidence/c11-4-generic-mixed-spanning-runtime-2026-10-28/README.md)
- 구현/runtime 적대 검토 20개: [검토 기록](../spec/internal/evidence/c11-4-generic-mixed-spanning-runtime-2026-10-28/implementation-runtime-review-20.md)
- PR diff 적대 검토 20개: [검토 기록](../spec/internal/evidence/c11-4-generic-mixed-spanning-runtime-2026-10-28/pr-diff-review-20.md)
