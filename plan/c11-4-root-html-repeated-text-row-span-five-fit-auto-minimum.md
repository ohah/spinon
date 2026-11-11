# C11.4 · repeated-text row-span five-fit `min-height:auto`

**상태:** 계획 PR [#1359](https://github.com/ohah/spinon/pull/1359)과 제한 구현 PR [#1360](https://github.com/ohah/spinon/pull/1360) Rebase and merge · merge SHA `6b3475a0ee48741b7f2aaa30e8ee62b19618ef8d` · C11.4/C11 미완료 · 공식 `78/214` 유지

**직전 row-span:** [four-fit `min-height:auto`](./c11-4-root-html-repeated-text-row-span-four-fit-auto-minimum.md) · 구현 PR [#1357](https://github.com/ohah/spinon/pull/1357)

**직전 자동 배치:** [five-fit `min-height:auto`](./c11-4-root-html-repeated-text-autoplacement-five-fit-auto-minimum.md) · 구현 PR [#1349](https://github.com/ohah/spinon/pull/1349)

**사전 비교:** [Chrome 157 관측과 독립 검증](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-five-fit-auto-minimum-precomparison-2026-11-11/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-five-fit-auto-minimum-precomparison.html) · [계획 실패 관점 검토 20개](./c11-4-root-html-repeated-text-row-span-five-fit-auto-minimum-plan-review-20.md)

**병합된 제한 구현 검증:** [2026-11-12 실행 근거](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-five-fit-auto-minimum-runtime-2026-11-12/README.md)에는 Chrome 157 RuntimeSession matrix 34,560/34,560, five-fit trace·rejection 회귀와 Android/iOS 기본 route light/dark smoke가 기록되어 있다. [구현/runtime 실패 관점 20개 검토](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-five-fit-auto-minimum-runtime-2026-11-12/implementation-runtime-review-20.md)와 실제 PR diff 30개 검토를 마친 뒤 [PR #1360](https://github.com/ohah/spinon/pull/1360)을 병합했다. [병합 기록](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-five-fit-auto-minimum-runtime-2026-11-12/merge-PR1360.md)을 참조한다. 이 bounded profile의 병합은 C11.4/C11 완료나 공식 집계 승격이 아니다.

## 결손과 제한 목표

구현 전 결손은 기본 parsed Root HTML repeated-text row-span에서 다섯 `fit-content()` track이 들어간 여섯 implicit row list가 거부된다는 점이었다. PR #1360은 이 결손 중 아래에 고정한 row-span profile만 추가했다. 다섯-fit auto-placement은 별도 owner와 placement 규칙을 가지므로 row-span 구현과 합치지 않는다.

이 계획은 **정확히** 다섯 `fit-content()` track과 `min-content`, `max-content`, `auto` 중 하나인 singleton tail로 이루어진 여섯-member `grid-auto-rows` list를 조사하고 지원한다. 각 fit cap은 `0px`, `10px`, `20%`, `30px`, `40px`를 한 번씩 갖는다. Candidate text items의 computed `min-height:auto`와 동일 scene의 `min-height:0` 기준선을 비교한다. 첫 explicit-row empty control은 양쪽 모두 `min-height:0`으로 고정한다.

## 구현 전 비교 모델

- 규범 기준은 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)다. Automatic minimum eligibility와 spanning contribution을 구분한다. `fit-content()`의 minimum track sizing function은 `auto`이며 cap을 content floor 상한으로 취급하지 않는다.
- 기준 구현은 Agent Browser 0.31.1이 실행한 HeadlessChrome `157.0.0.0`이다. macOS arm64, DPR 1에서 관측했다. Chrome은 CSS 기준 비교이며 Spinon 구현 결과가 아니다.
- 고정 Grid는 definite `40×220px`, explicit column `40px`, explicit row `10px`, gap 0, LTR sparse row flow, `overflow:visible`이다. Text payload·font·line-height는 four-fit row-span 기준 fixture와 동일하다.
- 여섯 implicit row list는 fit 위치 quintuple 6개 × cap authored order 120개 × singleton tail 종류 3개로 2,160개다. Candidate `min-height:auto`와 baseline `min-height:0`, `align-content:start|stretch`, span-2 다섯 item 또는 span-2/3 네 item topology와 DOM forward/reverse를 교차해 34,560 scene과 17,280 pair를 비교했다. 각 scene은 explicit row 하나와 implicit row 여섯 개를 가진다.
- 독립 판정은 여섯 authored track의 순서·typed cap 위치, seven used rows의 finite/non-negative 값, item frame과 covered row 합, payload·line count, automatic-minimum eligibility/floor, source-order 반전, tail 종류별 stretch 관계다. Geometry tolerance는 `0.01 CSS px`다.

## 사전 관측

[독립 verifier](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-five-fit-auto-minimum-precomparison-2026-11-11/verification.json)는 34,560 scene, 2,160 track list, 6 fit-position quintuple, 120 cap order, 3 tail value, 17,280 source/alignment/minimum pair와 288 cap permutation group을 확인했다.

- Candidate automatic-minimum 대상은 77,760 item occurrence다. 고정 Chrome fixture에서 candidate payload-floor violation은 0이고 `min-height:0` baseline violation은 13,248이었다.
- Item frame과 occupied row 합의 최대 차이는 `0.00025` CSS px이며 DOM source-order 반전의 의미 geometry 차이는 0이다.
- `align-content:stretch`는 `auto` tail에만 남은 공간을 배분했다(최대 `130.0002` CSS px). `min-content`/`max-content` tail의 used row는 바뀌지 않았다. 두 결과를 한 metric으로 합산하지 않는다.
- 이 수치는 Chrome 157 고정 fixture 관측이다. Spinon RuntimeSession, Android/iOS app, 표면 pixel 또는 실기기 결과를 대신하지 않는다.

## 제한 admission과 구현 검증 관문

1. 계획 자체를 별도의 20개 실패 관점으로 검토하고 실제 계획 PR diff를 서로 다른 30개 관점으로 검토한 뒤 계획 PR을 병합한다. 계획 PR 병합 전에는 제품 Rust/runtime 코드를 수정하지 않는다.
2. Parsed Root HTML style projection과 layout admission에 다섯-fit row-span predicate를 독립 추가한다. 정확히 여섯 implicit member, 다섯 fit cap의 typed multiset `[0px, 10px, 20%, 30px, 40px]`, singleton tail `[min-content|max-content|auto]`만 허용한다. Tail position과 cap authored index/type를 보존한다. Existing one-/two-/three-/four-fit row-span, five-fit auto-placement, six-fit auto-placement 및 position별 profile을 일반화·교체하지 않는다.
3. Fixed Root HTML owner, `40×220px` definite Grid, `40px` column, explicit `10px` row, zero gap, LTR sparse row flow, visible overflow, fixed text/font/line-height, two topology families와 4개 DOM order, `align-content:start|stretch`를 검증한다. Text items는 동일한 `min-height:0` 또는 `auto` mode를 써야 한다. Empty explicit-row control은 `min-height:0`이다.
4. 각 positive candidate의 automatic-minimum eligibility를 CSS 조건으로 계산한다. Scroll container, flexible spanned track, fit count·tail·cap·track order 불일치 및 span outside the fixed topology는 admission하지 않는다. Cap은 finite non-negative px 또는 definite-basis percentage여야 한다. 기존 projection에서 절대 길이 px로 정규화되는 `calc()`만 허용 cap과 같은 typed value로 처리하고, unresolved calculation과 indefinite percentage는 거부한다.
5. RuntimeSession에서 Chrome의 같은 34,560 scene을 재실행한다. Track list·used rows·item/payload frames·line count·source order·tail-specific stretch·Grid-end overflow와 trace/revision owner를 독립 observation에 대조한다. Candidate payload-floor violation은 0이어야 한다. Baseline 차이는 관측값으로 기록하며 제품 결과를 미리 강제하지 않는다.
6. Negative matrix에는 fit 수 0–6, list 5/7 member, fit 위치 이동, cap 중복·누락·순서/typed value 오류, tail 누락·중복·기타 값, negative/non-finite/unresolved cap, indefinite percentage, mixed minimum, control auto minimum, scrollable overflow, dense/column flow, explicit placement, span·item 수·text/font 오류, extra/nested owner, inline/replaced child, stale trace·revision 및 atomic rejection 뒤 same-session recovery를 포함한다. 실패 응답은 일부 frame/trace를 게시하지 않아야 한다.
7. Existing one-/two-/three-/four-fit row-span, five-/six-fit auto-placement, position, track-list, rounding, revision과 Root HTML regression을 실행한다. 변경 Rust는 `cargo fmt --all -- --check`와 영향 crate Clippy를 기록한다. 전체 suite 실패가 있으면 변경 전 baseline과 분리해 보고한다.
8. iOS 26.2 Simulator와 Android API 37 ARM64 AVD에서 기본 Root HTML route를 light/dark로 빌드해 layout-ready/draw-presented smoke를 확인한다. 이 app route는 row-span fixture를 싣지 않으므로 feature matrix의 증거로 부르지 않는다. PixelCopy/Metal readback, compositor pixel parity, 실기기·hardware GPU·full WPT 주장을 추가하지 않는다. 사용 전에 simulator/device 상태를 조사하고 task-owned clone·simulator·Agent Browser session을 끝난 즉시 종료한다. 현재 비교는 Agent Browser에서 했으며 종료 뒤 전용 session/process가 없는지 확인했다.
9. 계획 검토와 다른 실패 경로 20개의 implementation/runtime 검토를 작성하고 실제 구현 PR diff는 별도 30개 관점으로 검토한다. PR head와 local diff를 대조한다. PR 설명에는 matrix, 플랫폼 smoke 범위, 화면 근거와 남은 한계를 기록한다.
10. Merge 후 internal contract 0059, `spec/STATUS.md`, `spec/public/roadmap.html`, plan/evidence와 Tailnet preview를 실제 결과에 맞춘다. 이 제한 slice만으로 C11.4/C11 checkbox, 공식 `78/214` 또는 internal numeric contract `0.1.0`을 완료 처리하지 않는다.

## 남은 범위

이 병합 profile은 arbitrary Grid track/cap/topology, general Root HTML Grid admission, public API/FFI, 앱 화면에서 five-fit row-span fixture 실행, 표면 pixel parity, 실기기/hardware GPU, full WPT 또는 C11.4/C11 완료를 추가하지 않는다. 이 계획의 제한 profile은 구현됐으며, 남은 범위는 별도 작업으로 추적해야 한다.
