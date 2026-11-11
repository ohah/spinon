# C11.4 · repeated-text row-span 항목별 `min-height:auto` 조합

**상태:** 계획 PR에서 범위와 Chrome 비교를 검토 중 · 제품 코드는 아직 변경하지 않음 · C11.4/C11 미완료 · 공식 완료 `78/214` 유지

**선행 구현:** [six-fit row-span `min-height:auto`](./c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum.md) · PR [#1365](https://github.com/ohah/spinon/pull/1365)

**구현 전 비교:** [138,240-scene Chrome 157 행렬과 독립 검증](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-mixed-auto-minimum-precomparison-2026-11-12/README.md)

**계획 검토:** [서로 다른 실패 관점 20개](./c11-4-root-html-repeated-text-row-span-mixed-auto-minimum-plan-review-20.md)

## 결손과 목표

PR #1365가 추가한 six-fit repeated-text row-span profile은 모든 span text item의 computed `min-height`가 한 장면에서 전부 `0` 또는 전부 `auto`인 경우만 허용한다. 현재 `0`과 `auto`를 item별로 섞은 장면은 atomic rejection 대상이다. CSS Grid automatic minimum은 item별 computed minimum과 span이 덮는 track을 각각 판정하므로, 균일 모드만으로는 실제 유효한 혼합 cascade를 표현하지 못한다.

목표는 기존 고정 six-fit row-span profile 안에서 span text item마다 `min-height:0|auto`를 독립 적용하는 것이다. 지원하지 않는 track·cap·topology·style 범위는 넓히지 않는다. 새 FFI/API나 fixture-only 제품 경로는 추가하지 않고 기본 parsed Root HTML RuntimeSession에서 검증한다.

## 고정 입력과 비교 모델

- 규범 기준은 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)다. `fit-content()` track의 minimum sizing function은 `auto`다. 각 visible, non-scrollable span item이 자동 최소값을 사용하는지 개별 판정한다.
- 독립 oracle은 Agent Browser 0.31.1 / HeadlessChrome 157.0.0.0, MacIntel, DPR 1, viewport `1280×900`이다. definite `40×220px` Grid, `40px` column, 빈 explicit `10px` row, zero gap, LTR sparse row flow, `overflow:visible`, `align-content:start|stretch`를 고정한다.
- `grid-auto-rows`는 six-fit list이며 cap multiset은 `[0px, 10px, 20%, 30px, 40px, 50px]`이다. authored order 720개를 모두 검사하고 percentage basis는 definite 220px block size로 둔다.
- Topology A는 A–E 다섯 text item이 각각 여섯 implicit row 중 두 개를 span한다. Topology B는 A/B가 span 2, C/D가 span 3으로 같은 여섯 implicit row를 덮는다. 각 topology에서 source order forward/reverse를 모두 검사한다.
- Topology A의 32개, Topology B의 16개 `min-height` bit mask를 전부 교차한다. bit 순서는 A–E 또는 A–D이고 bit `1`은 해당 item의 `auto`, `0`은 `0px`다. Explicit-row control은 항상 `0px`다.
- 모든 720 cap order에서 두 alignment와 두 DOM order를 검사한다. 총 `720 × (32 + 16) × 2 × 2 = 138,240` scene이며 mask당 2,880 scene이다. Uniform all-zero/all-auto 11,520 scene은 이전 [six-fit uniform capture](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12/README.md)와 교차 비교한다.
- 각 item의 computed minimum, span이 덮는 used-row 합, item/payload frame, line box와 Grid-end overflow를 기록한다. geometry 허용 오차는 `0.01 CSS px`다. `auto` item만 자신의 payload floor를 보존해야 하며 `0px` item의 payload floor 부족은 별도 결과로 기록한다.

## Chrome 사전 비교 결과

Agent Browser가 138,240 scene 전부를 기록했고 독립 verifier가 입력 조합·computed minimum·row/frame 관계를 확인했다.

| 검증 | 결과 |
| --- | ---: |
| 전체 scene / comparison group | 138,240 / 34,560 |
| 여섯-cap authored order | 720 / 720 |
| per-item mask | A topology 32/32, B topology 16/16 |
| 기존 uniform matrix와 일치한 control | 11,520 / 11,520 |
| `auto` item payload-floor violation | 0 |
| `0px` item payload-floor shortfall | A 34,944, B 12,672 |
| Grid-end overflow | 두 topology 모두 0 |
| source-order·alignment geometry 오차 | 0 CSS px |
| span frame와 covered-row 합 최대 차이 | 0.0001 CSS px |
| mixed mask가 all-zero와 geometry가 달랐던 cap order | 모든 mask에서 540–720 / 720 |

원본 관측과 검증 결과는 [evidence README](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-mixed-auto-minimum-precomparison-2026-11-12/README.md)에 있다. 이 값은 browser CSS 결과이며 Spinon 실행 결과가 아니다.

## 입력 경계와 실패 동작

- 기존 exact six-fit typed cap multiset, 여섯 positive implicit row, 두 span topology, direct-text grammar, fixed owner/revision, fixed dimensions, source order, alignment와 LTR/visible overflow 조건을 유지한다.
- span item의 minimum 조합은 해당 topology item 모두의 `0px|auto` bit mask만 허용한다. 명시적 control item은 `0px`만 허용한다. 양수 fixed length, percentage, unresolved value 또는 다른 CSS minimum은 거부한다.
- track count·cap·type, row/span·column placement, child 수, nested/inline/replaced content, font/text grammar, scrollable overflow, writing mode, gap, dimensions, owner/revision/trace 등 기존 #1365 경계는 계속 fail-closed한다.
- 잘못되거나 stale한 mixed revision은 partial geometry/trace를 게시하지 않고, 같은 RuntimeSession의 다음 valid revision에서 회복해야 한다.

## 구현과 검증 관문

1. 계획 PR을 병합하고 계획 검토 20개와 실제 계획 PR diff 검토 30개를 각각 기록한 뒤 제품 코드를 바꾼다.
2. Style projection과 Layout은 per-item `0px|auto` 값을 보존하고 기존 six-fit row-span 전용 admission 안에서만 mask를 허용한다. Six-fit auto-placement, 1–5-fit row-span, 다른 cap multiset 또는 generic Grid admission을 넓히지 않는다.
3. 기본 Root HTML RuntimeSession에서 Chrome과 같은 138,240-scene mask/cap/topology/alignment/source-order matrix를 재생한다. 720 authored cap order를 모두 유지하고 item별 computed minimum·trace contribution·covered row·payload floor·owner/revision을 확인한다.
4. 기존 six-fit uniform 11,520 scene, 1–5-fit span, 1–6-fit auto-placement, repeated-text, row rounding, trace/rejection/recovery와 Grid layout 회귀를 별도 재실행한다. Invalid minimum, control `auto`, unsupported child/topology, stale trace는 atomic reject 후 같은 session에서 valid scene으로 회복해야 한다.
5. 변경 crate의 focused Rust tests, `cargo fmt --all -- --check`, 영향 crate Clippy를 실행한다. broader suite에 baseline failure가 있으면 후보와 base tree를 분리해 기록한다.
6. 가능하면 동일 source revision으로 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 기본 Root HTML light/dark smoke를 확인한다. 기본 route smoke는 이 mask matrix geometry나 surface-pixel parity로 해석하지 않는다. 이번 작업이 시작한 Simulator와 Agent Browser 세션은 종료하고 전용 테스트 앱/AVD가 있으면 제거한다.
7. 구현 후 계획 검토와 겹치지 않는 runtime 실패 관점 20개, 실제 implementation PR diff 30개를 각각 기록한다. PR 본문에 실행 경계·가능한 화면을 넣고 Agent Browser에서 이미지 표시를 확인한다.
8. 병합 후 `spec/STATUS.md`, 내부 계약 0059, 이 plan/evidence와 공개 roadmap을 실제 merge SHA·실행 결과와 동기화한다. C11.4/C11 checkbox, 공식 `78/214`, contract `0.1.0`은 유지한다. Tailnet preview를 바꿀 때 선택 파일만 백업·hash 검증해 동기화한다.

## 완료 판정

138,240 RuntimeSession scene, 720 cap order, 모든 per-item mask, uniform control 회귀, source-order/alignment invariants, span-frame·payload floor, atomic rejection/recovery, existing profile 회귀, focused Rust/Clippy/rustfmt, 가능한 Android/iOS smoke, 별도 runtime 20개와 PR diff 30개, 명세·로드맵 동기화와 merge 결과를 확인한 경우에만 이 bounded slice를 구현 완료로 기록한다. 이는 일반 CSS Grid의 임의 automatic minimum, public CSS 지원, WPT 전체 적합성 또는 C11.4/C11 완료가 아니다.
