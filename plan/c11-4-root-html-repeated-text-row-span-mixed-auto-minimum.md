# C11.4 · repeated-text row-span 항목별 `min-height:auto` 조합

**상태:** mixed per-item minimum bounded slice가 구현 PR [#1369](https://github.com/ohah/spinon/pull/1369)로 병합됐다 (merge SHA `79cc6328fb61e0babbb6d55c164485145f0058f5`). 138,240 RuntimeSession scene과 기존 회귀를 확인했으며 플랫폼 확인은 기본 Root HTML boot smoke다. C11.4/C11 체크는 미완료, 공식 완료 `78/214`, 내부 contract `0.1.0`은 유지한다.

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

## 구현과 검증 결과

1. 계획 및 구현 작업은 별도 PR로 진행했고, 구현/runtime 실패 관점 20개와 실제 implementation PR diff 30개 검토를 각각 남겼다.
2. Style projection과 Layout은 per-item `0px|auto`를 보존하며 mixed mask는 exact six-fit row-span profile에서만 admission한다. Six-fit auto-placement와 1–5-fit row-span admission은 변경하지 않았다.
3. 기본 Root HTML RuntimeSession에서 720 authored cap order, 두 topology의 모든 mask, 두 alignment와 forward/reverse source order를 합쳐 138,240/138,240 scene을 확인했다. 각 item의 computed minimum, trace contribution, covered-row 합, Chrome frame/payload floor와 owner/revision을 검증했다.
4. 기존 uniform six-fit 11,520 scene, six-fit auto-placement 5,760 scene, three-fit row-span 11,520 scene 회귀 및 invalid minimum·control `auto`·unsupported cap의 fail-closed 동작과 same-session recovery를 확인했다.
5. Focused Rust tests, `cargo fmt --all -- --check`, runtime feature compile은 통과했다. Strict all-targets Clippy는 변경 밖 테스트 helper의 기존 lint 6건에서 중단됐다. 지정한 두 baseline lint 종류만 제외한 변경 범위 실행은 통과했고, broad style-to-layout/layout suite의 기존 실패는 base/candidate 증거와 함께 기록했다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 source revision으로 기본 Root HTML boot smoke를 확인했다. 이는 mixed-mask geometry나 surface-pixel parity가 아니다. 작업용 Android 설치물과 AVD는 제거·종료했고 공유 AVD와 iOS Simulator 프로필, 기존 iOS 앱 설치는 보존했다.
7. 구현/runtime 실패 관점 20개와 실제 implementation PR diff 30개를 별도로 기록했다. PR 본문 화면은 기본 Root HTML boot smoke라고 명시했고 Agent Browser에서 두 이미지가 로드됨을 확인했다.
8. 구현 PR [#1369](https://github.com/ohah/spinon/pull/1369)은 `2026-11-12T01:02:45Z` Rebase and merge로 병합됐다 (merge SHA `79cc6328fb61e0babbb6d55c164485145f0058f5`). 이어 `spec/STATUS.md`, 내부 계약 0059, 이 plan/evidence와 공개 roadmap의 병합 상태를 동기화한다. C11.4/C11 checkbox, 공식 `78/214`, contract `0.1.0`은 유지한다. Tailnet preview를 바꿀 때는 선택 경로만 backup·hash 검증으로 동기화한다.

## 완료 판정

138,240 RuntimeSession scene, 720 cap order, 모든 per-item mask, uniform control 회귀, source-order/alignment invariants, span-frame·payload floor, atomic rejection/recovery, existing profile 회귀, focused Rust/Clippy/rustfmt, 기본 Root HTML Android/iOS smoke, 별도 runtime 20개와 PR diff 30개, 명세·로드맵 동기화와 merge 결과를 구현 PR [#1369](https://github.com/ohah/spinon/pull/1369)에서 확인했다. GitHub workflow/status checks는 0개, strict Clippy에는 기존 helper lint 여섯 건, broad crate suite에는 baseline 실패가 있어 모두 green으로 확대 해석하지 않는다. 플랫폼 smoke는 mixed-minimum feature geometry 또는 surface pixels가 아니다. 이 bounded slice는 일반 CSS Grid의 임의 automatic minimum, public CSS 지원, WPT 전체 적합성 또는 C11.4/C11 완료가 아니다.
