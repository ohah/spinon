# C11.4 · repeated-text row-span six-fit `min-height:auto`

**상태:** 계획 PR 병합 · 제한된 six-fit row-span 구현과 RuntimeSession 검증 완료 · 구현 PR 준비 중 · 공식 `78/214`, C11.4/C11 미완료 유지

**직전 row-span 구현:** [five-fit `min-height:auto`](./c11-4-root-html-repeated-text-row-span-five-fit-auto-minimum.md) · 구현 PR [#1360](https://github.com/ohah/spinon/pull/1360)

**별도 six-fit predecessor:** [auto-placement `min-height:auto`](./c11-4-root-html-repeated-text-autoplacement-six-fit-auto-minimum.md) · 구현 PR [#1354](https://github.com/ohah/spinon/pull/1354). 이 input은 nine-child non-spanning auto-placement이며 본 row-span slice를 지원하지 않는다.

**사전 비교:** [Chrome 157 matrix와 독립 검증](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-six-fit-auto-minimum-precomparison.html) · [계획 실패 관점 20개](./c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-review-20.md)

**구현·실행 근거:** [RuntimeSession matrix·플랫폼 smoke·한계](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-runtime-2026-11-12/README.md) · [구현/runtime 실패 관점 20개](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-runtime-2026-11-12/implementation-runtime-review-20.md)

## 결손과 제한 목표

현재 `RootHtmlFitContentAutoRowsV1`의 repeated-text row-span `min-height:auto` 경로는 1–5 `fit-content()` track profile을 제한적으로 다루지만, six-fit row-span은 허용하지 않는다. Style projection에는 five-fit span predicate와 six-fit auto-placement predicate가 별도다. Layout 쪽 repeated span automatic-minimum admission은 fit 수 `1 | 3 | 4 | 5`이며 6을 포함하지 않는다. 따라서 six-fit auto-placement이 merged라는 사실만으로 span topology를 허용하지 않는다.

목표는 다음 **고정된 row-span profile만** parsed Root HTML style projection, layout, RuntimeSession에 추가하는 것이다.

- `grid-auto-rows`는 정확히 여섯 member이며 여섯 track 모두 `fit-content()`다.
- direct cap의 typed multiset은 `[0px, 10px, 20%, 30px, 40px, 50px]`이며 720 authored order를 보존한다.
- definite Grid는 `40×220px`, explicit column은 `40px`, 첫 explicit row는 빈 `10px` control이다. Grid는 row direction/LTR, zero gap, `overflow:visible`, `align-content:start|stretch`다.
- Topology A는 control 외 direct text item 다섯 개를 row 2–6에 각각 span 2로 둔다. Topology B는 row 2–3의 span 2 item 두 개와 row 4–5의 span 3 item 두 개를 둔다. 두 topology 모두 implicit row 여섯 개를 서로 겹치게 덮는다.
- 모든 span text item의 computed `min-height`는 한 scene 안에서 전부 `0` 또는 전부 `auto`여야 한다. 첫 explicit-row control은 항상 `min-height:0`이다.

## 구현 전 비교 모델

- 규범 기준: [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items). §6.6 automatic minimum eligibility와 §11.5 spanning contribution은 별도로 판정한다. §11.2에 따라 `fit-content()`의 minimum sizing function은 `auto`다.
- 독립 browser oracle은 Agent Browser 0.31.1 / HeadlessChrome `157.0.0.0`, `MacIntel`, DPR 1이다. 비교 기하는 `0.01 CSS px` 이내, payload floor는 candidate에서 위반 0건이어야 한다.
- 사전 비교는 720 cap order × 2 topology × 2 DOM order × 2 alignment × 2 minimum mode = 11,520 scenes, 5,760 baseline/candidate pair다. 구현 RuntimeSession도 같은 scene identity와 order를 재생해 list/cap order, rows, span/item/payload geometry, line boxes, floor와 revision owner를 독립 observation에 대조한다.
- Chrome 157 결과: baseline `min-height:0`은 4,800/25,920 item occurrences에서 payload floor 아래였고, candidate `min-height:auto`는 0/25,920이었다. 모든 5,760 pair의 geometry가 달랐다. 두 mode 모두 220px Grid 바깥으로 payload가 나가지 않았으며 span-frame 대 covered rows 최대 오차는 `0.00005px`, source-order reversal과 `start|stretch` 차이는 0이었다. 전체 수치는 [사전 비교 verifier](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12/verification.json)를 기준으로 한다.

## 제한 admission과 구현 검증 관문

1. 계획 PR이 병합되기 전 제품 code를 수정하지 않는다. 계획 자체를 서로 다른 실패 경로 20개로 검토하고, 실제 계획 PR diff는 별도의 프로젝트 필수 30개 관점으로 검토한다.
2. Style projection과 layout에 six-fit **row-span 전용** predicate/profile을 추가한다. six-fit auto-placement, five-fit span 또는 generic six-member track admission으로 우회 허용하지 않는다. 기존 1–5 fit span, 1–6 fit auto-placement 결과는 유지한다.
3. Positive RuntimeSession matrix는 11,520 scenes / 5,760 baseline/candidate pairs다. Track order와 typed cap mapping, 7 used rows (explicit row 포함), control, computed minimum, item span, text payload/line boxes, floor, Grid-end boundary, DOM order, alignment, owner/style/layout revision을 검사한다. Cap percentages는 definite `220px` basis에 대해 계산하며 `20%`는 `44px`다.
4. Rejection matrix는 track count 5/7, fit 수 0–5, duplicate/missing/other cap, wrong typed cap, negative/non-finite/unresolved calc, indefinite percent, tail/extra list member, mixed minimum, control auto, missing/extra direct child, wrong text/font/line-height, topology/span start 오류, scrollable overflow, flexible sizing, dense/column flow, explicit columns/owner, nested/inline/replaced child 및 stale revision/trace를 포함한다. Rejection은 일부 geometry/trace를 게시하지 않아야 하며 같은 session의 다음 유효 revision에서 회복해야 한다.
5. Existing 1–5 fit row-span, 1–6 fit auto-placement, track rounding, span distribution, repeated-text content, style revision과 Root HTML Grid 회귀를 별도로 돌린다. 변경 Rust는 `cargo fmt --all -- --check`와 영향 crate Clippy를 기록한다. 전체 suite에 사전 실패가 있으면 변경 전 baseline과 분리한다.
6. Android API 37 ARM64 AVD와 iOS 26.2 Simulator에서 기본 Root HTML route의 light/dark build/runtime smoke를 수행할 수 있으면 결과와 기기 식별자를 기록한다. 기본 route는 six-fit fixture를 실행하지 않으므로 이 smoke를 feature geometry, PixelCopy/Metal readback 또는 pixel parity로 세지 않는다. 기기 상태를 먼저 확인하고 전용 AVD clone·Simulator·Agent Browser 세션은 완료 후 닫고 소유 프로세스 부재를 확인한다.
7. 구현 뒤 계획 검토와 겹치지 않는 runtime failure 관점 20개를 작성한다. 구현 PR diff는 별도 프로젝트 필수 30개 관점으로 검토하고 PR head와 local diff를 비교한다. PR 본문에는 feature matrix, browser/RuntimeSession 결과, 가능한 화면, 남은 범위를 기록한다.
8. 병합 후 internal contract 0059와 roadmap/plan/evidence를 실제 결과에 동기화한다. 제한 profile만으로 `spec/STATUS.md` 완료 checkbox, 공식 `78/214`, C11.4/C11 또는 내부 숫자 계약 `0.1.0`을 승격하지 않는다. Tailnet 미리보기는 source hash와 backup을 기록한 뒤 사용자가 정한 preview에 동기화한다.

## 로컬 구현 결과

- Style projection과 layout에 six-fit row-span 전용 exact cap profile을 추가했다. Six-fit auto-placement 및 기존 1–5 fit row-span profile은 별도 경계로 유지한다.
- 기존 five-fit phase-level rounding replay를 five/six-fit span simulator로 일반화했다. Six-fit replay 또는 trace-stage 대조가 실패하면 fail-closed하고, five-fit fallback은 유지한다.
- 최종 RuntimeSession 실행은 720 cap orders × 16 scenes = 11,520/11,520 scene, 5,760 baseline/candidate pairs다. Candidate payload-floor violation 0건, baseline 4,800건이다. Pair별 geometry change, source-order 및 `start/stretch` invariants를 확인했다.
- Profile/rejection/trace, five-fit regressions, format, workspace Clippy 및 Android/iOS Simulator app builds를 기록했다. 전체 `spinon-layout --lib`에는 미수정 C15 HostDocument assertion failure 1건이 남아 있고 이번 실행에서 baseline tree 재현은 하지 않았다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 캡처는 기본 root HTML route smoke로 한정한다. Six-fit fixture geometry나 PixelCopy/Metal readback은 확인하지 않는다.
- 상세 증거와 PR diff 검토는 implementation PR 준비가 끝나면 [실행 근거](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-runtime-2026-11-12/README.md)에 기록한다. 공식 `78/214` 및 C11.4/C11 상태는 유지한다.

## 남은 범위

이 slice는 arbitrary six-fit cap/topology, general Grid/CSS admission, WPT, public API/FFI, 앱 화면의 six-fit fixture geometry, hardware GPU, surface pixel parity 또는 전체 C11.4/C11 완료를 포함하지 않는다. 제한 구현을 병합해도 이 제한 profile 밖의 기능을 지원한다고 간주하지 않는다.
