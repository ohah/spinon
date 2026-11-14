# C11.4 · 두 `fit-content()` row-span item별 mixed minimum 검증

**상태:** 구현 전 계획 후보. Chrome 157 Agent Browser 사전 비교 92,160/92,160 scene과 독립 verifier를 마쳤다. 제품 RuntimeSession은 아직 실행하지 않았다. C11.4/C11 체크와 공식 `78/214`, 내부 숫자 계약 `0.1.0`은 유지한다.

- **기준 profile:** [두 `fit-content()` row-span 제한 구현](./c11-4-root-html-repeated-text-row-span-two-fit-content.md), PR #1311
- **구현 전 비교:** [92,160-scene Chrome 157 matrix와 overflow 기록](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-two-fit-mixed-auto-minimum-precomparison-2026-11-14/README.md)
- **Chrome fixture:** [`two-fit mixed minimum fixture`](../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-two-fit-mixed-auto-minimum-precomparison.html)
- **계획 실패 관점 검토:** [서로 다른 20개](./c11-4-root-html-repeated-text-row-span-two-fit-mixed-auto-minimum-plan-review-20.md)
- **계획 PR diff 검토:** [서로 다른 30개](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-two-fit-mixed-auto-minimum-precomparison-2026-11-14/plan-pr-diff-review-30.md)
- **다음 후보 index:** [C11.4 track sizing 순차 계획](./c11-4-grid-track-sizing.md)

## 확인한 코드 공백

Style projection의 `allow_auto_minimum` 조건은 기존 두-fit explicit row-span profile에서 모든 span text item에 `min-height:0|auto`를 이미 허용한다. 하지만 `span_auto_minimum_items_candidate`는 fit count `1|3|4|5|6`만 다루고, layout rounding 경로도 mixed replay를 one/three/four/five/six-fit으로 나눈다. 기존 two-fit RuntimeSession 회귀는 uniform row-span 3,840 scene이며 item별 mixed mask module은 없다. 따라서 이 후보는 admission을 새로 여는 작업이라고 가정하지 않는다. 현재 허용 입력이 Chrome과 같은지 제품 실행 경로로 검증하고, 차이가 있으면 그 원인을 재현한 뒤 최소한으로 고친다. 잘못된 입력까지 이미 허용되거나 제품 동작이 보장되지 않으면 projection admission을 좁힌다.

## 구현 전 비교 모델

- Oracle은 HeadlessChrome 157.0.0.0, DPR 1, viewport `1280×900`의 Agent Browser다. 전체 입력, UA, collector, compressed observations와 screenshots는 [evidence directory](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-two-fit-mixed-auto-minimum-precomparison-2026-11-14/README.md)에 hash로 연결했다.
- definite Grid `40×220px`, explicit row `10px`, six-member `grid-auto-rows` list `fit-content()×2 + min-content×2 + max-content×1 + auto×1`, LTR, row flow, gap 0, `overflow:visible`, `align-content:start|stretch`를 고정한다.
- 두 fit 위치 pair 15개, representative cap pair 네 가지와 양방향 ordered assignment 8개, 기존 tail order 네 가지를 사용해 unique track list 480개를 만든다. Direct cap은 finite non-negative px 또는 percentage며 percentage basis는 220px다. 이 표본은 모든 cap 값 또는 모든 list 순열을 지원한다는 뜻이 아니다.
- Topology는 `span2Six`의 다섯 text item과 `span2Span3Six`의 네 text item이다. 두 topology 모두 forward/reverse DOM 순서를 사용한다. Mask bit는 항상 semantic ID `A–E` 기준이며 reverse source order에서도 owner를 유지한다. Explicit-row control은 `0px`로 고정하고 각 span item은 `0px|auto`다.
- `span2Six` 32개, `span2Span3Six` 16개 mask, 두 source order, 두 alignment를 포함한 Chrome scene 수는 `480 × 2 × 2 × (32 + 16) = 92,160`이다. All-zero와 all-auto endpoint는 각각 3,840 scene이다. 전체 관측은 46,080 source-order pair와 46,080 alignment pair에 대응한다.

Chrome 결과 92,160/92,160, 480 track list, automatic-minimum style item occurrence 215,040개다. Span frame/covered-row 최대 차이는 `0.000375 CSS px`, source-order geometry 차이는 `0 CSS px`, stretch에서 non-auto row 변화는 `0 CSS px`다. Singleton auto row는 `25–130.0002 CSS px` stretch됐다. Used row는 finite non-negative `0–170 CSS px`다.

경계 넘침도 별도로 비교했다. 320 scene에서 text payload와 line rect가 220px Grid content/border edge를 넘어 최대 `30 CSS px`까지 보였고, Grid item frame 및 전체 used-row 합은 경계를 넘지 않았다. 이 scene들은 모두 `overflow:visible`, `align-content:stretch`, zero cap pair `(0px,0%)` 또는 `(0%,0px)`의 역순, span2Six topology에 있었다. 두 ordered cap assignment는 각각 160 scene이다. 구현은 이 관측을 숨기거나 payload overflow를 Grid item frame overflow로 바꿔 기록하면 안 된다. 자세한 mask별 분포와 sample은 evidence README 및 verifier JSON을 기준으로 한다.

## 고정 지원·거부 경계

- Existing exact two-fit row-span profile, six authored track member, two direct px/percentage caps, 480 track lists, two existing topology families, forward/reverse source order와 `start|stretch` 이외의 grammar는 넓히지 않는다.
- 모든 span item별 `0px|auto` mask를 다룬다. 첫 explicit-row item은 empty child, 10px height와 `min-height:0`을 계속 요구한다. 각 item's computed minimum은 mask의 semantic ID와 일치해야 한다.
- Automatic minimum eligibility는 authored `min-height:auto`만 보고 모든 item에 적용하지 않는다. Span이 덮는 track type, scrollable overflow 조건, intrinsic contribution trace를 함께 비교한다. Min-content floor와 fit cap의 관계는 0px cap에서도 기록한다.
- `overflow:visible`을 그대로 비교하고 payload line box, payload frame, Grid item frame, Grid content edge를 각각 구분한다. 320-scene zero-cap overflow는 고정 regression이다. Hidden/scrollable overflow, 다른 cap, style 또는 topology는 새 지원으로 유추하지 않는다.
- Public CSS/FFI API, 신규 contract version, arbitrary Grid track lists, arbitrary text/span topology, WPT 전체 적합성은 범위 밖이다.

## 구현 및 검증 관문

1. 이 계획은 별도 20개 실패 관점 검토를 통과하고, 계획 PR의 실제 변경 diff도 그와 독립적인 30개 관점으로 다시 검토한 뒤에만 병합한다. PR에는 Chrome matrix·overflow screenshots를 올려 계획 oracle과 구현 실행을 구분한다.
2. RuntimeSession fixture는 고정 observation hash에서 92,160 scene을 모두 읽고 입력 조합·semantic mask·source order·alignment·cap·track multiset을 다시 확인한다. 작은 대표 표본으로 전체 실행을 대체하지 않는다.
3. 기본 parsed Root HTML RuntimeSession에서 Chrome의 모든 scene을 비교한다. Computed style, track order, per-item minimum, automatic-minimum eligibility, track contribution/trace, used row, span frame, payload frame, text line rect, overflow boundary와 revision/owner를 확인한다. Chrome observation과 RuntimeSession output의 비교 허용 오차는 frame/row별 `0.01 CSS px` 이하이며 1/64px quantization과 trace 기반 예외를 별도 기록한다.
4. Per-mask item ID는 DOM order와 독립이어야 한다. 46,080 source-order pair는 각 ID별 frame/payload/text line과 row vector가 일치해야 한다. 46,080 start/stretch pair는 auto row만 늘고 non-auto row 변화가 `0.01px` 이하여야 한다. Span box와 covered used-row 합의 최대 차이는 `0.01px` 이하다.
5. Chrome의 320 visible-overflow scenes를 같은 browser/runtime fixture로 실행해 zero-cap assignment, `span2Six` topology, forward/reverse, mask별 count와 최대 30px payload/line overflow를 재현한다. item frame이 Grid edge 밖으로 나가지 않고 used-row 합은 220px 이하여야 한다. Min-height:auto가 적용된 item의 비eligible span에서 overflow가 남는 경우도 item별 trace와 함께 확인한다.
6. Negative controls는 one/three/four/five/six-fit multiplicity, 0/3/6 이외의 list length와 tail mutation, `min-height:min-content|1px`, explicit control `auto`, mask ID reorder, wrong span/source order, unsupported cap/percentage basis/alignment/overflow, extra owner, missing/duplicate/reordered/wrong-owner/stale trace를 포함한다. Rejection 뒤 published layout/render revision과 frames가 그대로이고 같은 RuntimeSession이 valid scene으로 회복하는지 확인한다.
7. Existing two-fit uniform span 3,840 scene, two-fit min-content-floor control, one-/three-/four-/five-/six-fit row-span mixed/uniform matrices, one-/two-fit auto-placement, one-fit row-span, repeated-text 810 corpus와 Grid sizing/projection/trace regressions를 다시 실행한다. 실패 artifact를 보존하고 base failure와 candidate failure를 구분한다.
8. 영향받은 Rust tests와 projection tests, 변경 crate Clippy `-D warnings`, `cargo fmt --all -- --check`를 실행한다. 전체 workspace test나 Clippy에서 발견한 baseline 문제를 이 후보 통과로 섞지 않는다.
9. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 source revision을 기본 Root HTML route로 연다. Mixed mask sentinel과 zero-cap visible-overflow sentinel의 layout-ready/draw-present, 앱 입력 hash와 화면을 기록한다. 이는 visual smoke이며 모든 92,160 scene, PixelCopy/Metal drawable exactness, compositor parity, 실기기나 hardware GPU 증거가 아니다. 시작 전에 booted device/profile을 기록하고 이번 실행이 켠 simulator/app/browser session만 종료한다. 표준 재사용 프로필은 유지하고 이번 작업용 임시 프로필·capture는 검증 뒤 제거한다.
10. 기능 수정 뒤 이전 plan review와 다른 20개 implementation/runtime failure perspective를 새로 작성한다. 실제 구현 PR diff는 별도의 30개 관점으로 검토하고, 발견된 결함은 수정한 뒤 해당 관점을 다시 확인한다.
11. 병합 전에 공개/internal contract와 `spec/STATUS.md`, 이 parent plan, public roadmap을 동기화한다. 아직 unresolved profile을 완료로 표시하지 않으며 공식 `78/214`, C11.4/C11 checkbox, internal contract `0.1.0`을 유지한다. 병합 후 사용자가 지정한 Tailnet preview와 desktop/mobile responsive rendering을 Agent Browser로 확인하고 해당 세션은 닫는다.

## 완료 판정

Chrome 92,160-scene oracle와 같은 분모의 RuntimeSession, all-mask semantic ownership, source-order/alignment invariants, zero-cap visible overflow controls, eligibility/trace/rounding, atomic reject/recovery, existing profile regressions, focused test/lint/format, Android/iOS representative product-path smoke, 별도 implementation/runtime 20관점과 PR diff 30관점 review, contract/roadmap sync, PR checks 상태 및 Rebase and merge state를 모두 기록한 뒤에만 이 bounded slice를 완료로 기록한다. 이 완료도 C11.4/C11의 공식 완료나 `78/214` 증가가 아니다.
