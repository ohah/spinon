# C11.4 · repeated-text row-span four-fit `min-height:auto`

**상태:** Chrome 157 사전 비교 11,520/11,520 장면, 계획 실패 관점 20개 및 계획 PR #1356 병합 완료 · 제품 구현 RuntimeSession matrix 11,520/11,520, four-fit auto-placement regression 5,760/5,760 통과 · 구현/runtime 검토 20개, 양 플랫폼 기본 앱 smoke 및 구현 PR 준비 완료 · C11.4/C11 미완료 · 공식 `78/214` 유지

**직전 병합 구현:** [six-fit auto-placement `min-height:auto`](./c11-4-root-html-repeated-text-autoplacement-six-fit-auto-minimum.md) · 구현 PR [#1354](https://github.com/ohah/spinon/pull/1354)

**기존 row-span 기준:** [three-fit row-span `min-height:auto`](./c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum.md) · 구현 PR [#1338](https://github.com/ohah/spinon/pull/1338)

**사전 비교:** [Chrome 157 관측과 독립 검증](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-auto-minimum-precomparison-2026-11-11/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-four-fit-auto-minimum-precomparison.html) · [계획 검토 20개](./c11-4-root-html-repeated-text-row-span-four-fit-auto-minimum-plan-review-20.md)

## 결손과 제한 목표

현재 parsed Root HTML의 고정 six-row repeated-text row-span 경로는 one-, two-, three-fit list를 제한적으로 지원한다. 네 fit track을 넣은 span list는 style projection과 layout profile에서 거부된다. 여섯 개 implicit track 중 네 개가 `fit-content()`이고 나머지가 `min-content`와 `auto`인 경우의 spanning contribution 및 `min-height:auto`는 아직 확인되지 않았다.

이 계획은 **정확히** 네 개 `fit-content()` track, `min-content` 한 개, `auto` 한 개로 이루어진 여섯-member `grid-auto-rows` list만 조사하고 지원한다. Candidate의 모든 span text item은 computed `min-height:auto`이고, 비교 기준은 같은 scene에서 `min-height:0`이다. 첫 explicit-row empty control은 두 모드 모두 `min-height:0`으로 고정한다.

## 구현 전 비교 모델

- 규범 기준은 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)다. §6.6 automatic minimum의 eligibility와 §11.5 spanning contribution을 구분한다. `fit-content()`의 minimum sizing function은 `auto`이며 cap을 content floor 상한으로 취급하지 않는다.
- 기준 구현은 Agent Browser 0.31.1이 실행한 HeadlessChrome `157.0.0.0`이다. macOS 26.5.1 arm64, DPR 1에서 관측했다. Chrome은 CSS 기준 비교이며 Spinon 구현 결과가 아니다.
- 고정 Grid는 definite `40×220px`, explicit column `40px`, explicit row `10px`, gap 0, LTR, sparse row flow, `overflow:visible`이다. 각 text item은 기존 ASCII payload와 10px monospace / 10px line-height를 쓴다.
- 여섯 implicit row list는 네 개의 서로 다른 cap `0px`, `10px`, `20%`, `30px`을 갖는 `fit-content()`와 `min-content`, `auto` 각 한 개다. Percentage basis는 definite 220px다. Fit position quadruple 15개, cap authored order 24개, 두 tail order를 모두 대조해 720개 track list를 만든다. 구현 admission은 이 computed typed multiset으로 제한한다. absolute `calc()`가 기존 typed projection에서 허용 cap으로 정규화되면 같은 cap으로 취급하고, unresolved 계산·중복 cap·그 외 값을 허용하지 않는다.
- Topology는 span-2 item 다섯 개가 여섯 row를 채우는 가족과 span-2/3 item 네 개가 여섯 row를 채우는 가족이다. 각 가족의 DOM source order를 정방향·역방향으로 실행한다. 여기에 `align-content:start|stretch`와 `min-height:0|auto`를 교차해 총 11,520 scene, 5,760 paired keys를 비교한다.
- 독립 판정은 여섯 authored track의 순서·typed cap 위치 보존, 일곱 used row의 finite/non-negative 값, item frame과 span track 합, 같은 payload의 line count, automatic minimum eligibility/floor, source-order 반전, auto row에만 남는 공간을 배분하는 stretch다. 기하 허용 오차는 CSS px `0.01`이다.

## 사전 관측

[독립 verifier](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-auto-minimum-precomparison-2026-11-11/verification.json)는 11,520 scene, 720 track list, 15 fit-position quadruple, 24 cap order, 2 tail order, 5,760 source/alignment/minimum pair 및 480 cap permutation group을 확인했다.

- Candidate 자동 최소 크기 대상은 25,920 item occurrence였고 payload floor 위반은 0이었다. `min-height:0` 기준선에서는 3,648 occurrence가 payload 높이보다 낮았다.
- item frame과 span track 합의 최대 차이는 `0.000225` CSS px였다. DOM source-order 반전의 의미 geometry 차이는 `0` CSS px다.
- `align-content:stretch`는 non-auto track을 바꾸지 않았고 최대 130 CSS px를 유일한 `auto` track에 배분했다.
- 이 결과는 고정 Chrome fixture의 관측이며 Spinon RuntimeSession·Android/iOS 앱의 구현 결과가 아니다.

## 제한 admission과 구현 검증 관문

1. 제품 코드 변경 전 계획 PR을 병합한다. 구현 전에 계획 자체를 별도 20개 실패 관점으로 검토하고 실제 계획 PR diff를 서로 다른 30개 관점으로 검토한다.
2. Style projection과 layout profile에 네 fit track predicate를 독립 추가한다. exact six-member list와 `[fit-content ×4, min-content, auto]` multiset만 허용한다. 이전 one-/two-/three-fit span, auto-placement, row-span position 프로파일을 일반화하거나 교체하지 않는다.
3. `grid-auto-rows` authored/computed order, cap type·position, six positive implicit row, first explicit control, span placement와 same-revision owner를 확인한다. computed `min-height`가 전 span item 모두 `0` 또는 모두 `auto`일 때만 각각의 기존/새 profile을 admission한다.
4. RuntimeSession에서 이 11,520-scene Chrome matrix에 대응하는 baseline/candidate를 재실행한다. 각 item frame·payload line box·occupied used rows·eligible floor·track trace를 independent Chrome observation과 대조한다. Capture 관측은 product RuntimeSession 결과를 대신하지 않는다.
5. 거부 matrix에는 wrong fit count, 5/6 fits, tail 누락·중복·대체, fit position 오프셋, cap 중복/누락/미등록, unsupported cap type, negative/non-finite/unresolved cap, indefinite percentage, mixed item minimum, control auto minimum, scrollable overflow, dense/column flow, explicit placement, row-span topology 오류, extra owner, nested/inline/replaced payload, stale trace와 stale revision을 포함한다. Rejection은 결과를 부분 게시하지 않고 같은 RuntimeSession의 다음 유효 revision에서 회복해야 한다.
6. Existing one-/two-/three-fit row-span, four-fit auto-placement, one- through six-fit auto-placement, row-span position·track-list·rounding·revision tests를 regression gate로 실행한다. 변경 Rust는 rustfmt와 영향 crate Clippy를 실행하고 원본 로그를 보존한다.
7. iOS 26.2 Simulator와 Android API 37 ARM64 AVD에서 기본 Root HTML app route를 빌드하고 light/dark layout-ready/draw-presented smoke를 실행한다. 이 app route는 four-fit fixture를 로드하지 않으므로 화면은 앱 시작 회귀 확인만 증명한다. Feature geometry는 RuntimeSession matrix가 판정한다. PixelCopy/Metal readback, compositor pixel parity, 실기기, hardware GPU 또는 full WPT 주장을 추가하지 않는다. 기기와 앱 상태는 사용 전에 조사하고 task-owned clone·전용 Agent Browser session은 완료 즉시 닫아 확인한다.
8. 구현 뒤 계획 검토와 다른 failure mode 20개의 implementation/runtime 검토를 작성한다. 구현 PR diff는 별도 30개 관점으로 검토하고 PR head와 local diff hash가 같은지 확인한다. PR 본문에는 matrix, 시뮬레이터 범위, 남은 한계와 동작 화면을 넣는다.
9. Merge 후 internal contract 0059, `spec/STATUS.md`, `spec/public/roadmap.html`, 계획·evidence·Tailnet preview를 실제 결과와 동기화한다. C11.4/C11 checkbox, 공식 `78/214` 또는 internal numeric contract `0.1.0`을 이 limited slice만으로 완료 처리하지 않는다.

## 남은 범위

이 slice는 일반 CSS Grid admission, 임의 track/cap/topology, 모든 WPT, public API/FFI, 앱 화면에서 four-fit fixture 실행, pixel parity, hardware GPU 또는 C11.4/C11 완료를 추가하지 않는다. 위 관문을 충족하기 전까지 계획과 사전 비교만 완료된 상태로 기록한다.
