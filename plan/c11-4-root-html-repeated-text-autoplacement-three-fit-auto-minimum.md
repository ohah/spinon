# C11.4 · three-fit auto-placement 반복 텍스트의 `min-height:auto`

- **상태:** 계획 및 Chrome 157 사전 비교 완료 · 구현 시작 전 · C11.4/C11 미완료 · 공식 `78/214` 유지 · 내부 계약 `0.1.0` 유지
- **직전 구현:** [three-fit row-span `min-height:auto`](./c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum.md) · 구현 PR [#1338](https://github.com/ohah/spinon/pull/1338)
- **기존 자동 배치:** [one-fit 자동 최소 크기](./c11-4-root-html-repeated-text-autoplacement-one-fit-auto-minimum.md) · PR [#1322](https://github.com/ohah/spinon/pull/1322) · [two-fit 자동 최소 크기](./c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum.md) · PR [#1318](https://github.com/ohah/spinon/pull/1318)
- **세 fit 위치 row-span 범위:** [계획](./c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum.md) · 별도 row-span profile만 지원하며 three-fit auto-placement는 의도적으로 거부한다.
- **Chrome 사전 비교:** [5,760 scene matrix와 독립 검산](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-three-fit-auto-min-precomparison-2026-11-11/README.md)
- **계획 실패 관점 검토:** [서로 다른 20개](./c11-4-root-html-repeated-text-autoplacement-three-fit-auto-minimum-plan-review-20.md)
- **계획 PR diff 적대 검토:** [실제 변경분의 서로 다른 30개 관점](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-three-fit-auto-min-precomparison-2026-11-11/plan-pr-diff-review-30.md)

## 결손과 제한 목표

기본 parsed Root HTML의 반복 텍스트 auto-placement는 여섯 member `grid-auto-rows` list 중 one-fit 및 two-fit의 `min-height:auto` profile을 지원한다. 현재 `root_html_fit_content_repeated_min_max_content_auto_placement_track_list`는 기존 one-/two-fit list만 받아들이며, three-fit list는 별도의 explicit row-span profile에서만 허용된다. projection 회귀 테스트도 같은 세 fit list에 대해 span predicate는 true, auto-placement predicate는 false임을 120개 position/tail 조합 전체에서 확인한다.

이 계획은 기존 nine-item Root HTML Grid의 여덟 repeated-text auto-placement item에 한해, three-fit list에서도 모든 text item이 computed `min-height:auto`인 profile을 추가하는 bounded slice를 정한다. Explicit-row 빈 control은 계속 `min-height:0`이며 one-fit/two-fit auto-placement 및 three-fit row-span 경계는 각각 유지한다.

## 구현 전 비교 모델

- 규범 근거는 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-functions), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)다. §6.6의 content-based automatic minimum 적용 조건과 §7.2의 `fit-content()`·`auto` minimum sizing function이 eligible row를 구분한다. `min-content`와 `max-content` row는 이 profile에서 automatic-minimum row로 분류하지 않는다.
- 독립 기준은 Agent Browser가 실행한 HeadlessChrome `157.0.0.0`, `MacIntel`, DPR 1이다. 사전 비교는 제품 RuntimeSession이 아니라 브라우저 CSS Grid 동작이다.
- 입력은 definite `40×220px` Grid, `40px` explicit column, 빈 `10px` explicit row, gap 0, LTR, row sparse auto-flow, `overflow:visible`이다. 여덟 implicit row item은 기존 고정 ASCII text payload와 10px monospace/10px line-height를 쓴다. Source order는 forward와 reverse를 모두 관측한다.
- 여섯-member `grid-auto-rows` list는 정확히 `fit-content()` 3개, `min-content` 1개, `max-content` 1개, `auto` 1개다. 세 fit 위치의 20개 조합, tail의 여섯 순서, `0px`·`10px`·`20%` cap을 각 한 번씩 배치한 여섯 순열을 교차해 고유 track list 720개를 만든다. Chrome 입력의 percentage basis는 definite 220px이다.
- 각 list에 `align-content:start|stretch`, `min-height:0|auto`, forward/reverse DOM order를 적용한다. 총 5,760 scene이며, 2,880개 동일 key에서 `min-height:0`과 `auto`를 paired comparison한다. 첫 explicit-row control은 모든 scene에서 `min-height:0`을 유지한다.
- 독립 verifier는 track list multiplicity·순서·cap 위치, computed CSSOM, 8개 implicit row 순환, source-order item/payload frame 및 line count, eligible row의 text content floor, definite grid 끝 밖으로의 item frame, start/stretch row 합과 auto-only free-space 분배를 검사한다.

### 사전 비교 결과

- 20 fit-position triple × 6 tail order × 6 cap assignment, 720개의 authored list를 포함한 Chrome 관측 5,760/5,760 scene과 2,880 paired key를 수집했다. 계획 fixture/content와 collector SHA-256은 `verification.json`에 고정했다.
- `min-height:0` baseline에서 payload content floor보다 작은 row는 7,200개였고, 그중 `fit-content()` 또는 `auto` minimum track에 있는 eligible row 역시 7,200개였다. `min-height:auto` candidate는 모든 row가 floor를 지켰으며, eligible row 7,200개 전부가 paired baseline보다 회복됐다.
- Candidate `align-content:start`의 전체 row 합은 180px, `stretch`는 220px다. Baseline start 합은 입력에 따라 110–150px였다. Stretch에서 `auto` row occurrence 1개/2개에 각 40px/20px가 배분됐으며 non-auto row의 최대 차이는 0px다.
- Baseline 각 장면은 하나 이상의 item payload가 해당 row frame보다 커서 visible item overflow가 있었고, candidate에는 없었다. 양쪽 모두 마지막 item frame이 220px Grid 끝을 넘은 장면은 없었다. Item 내부 visible overflow와 Grid 경계 overflow를 별개로 검사했다.
- 대표 Chrome 화면은 사전 비교 README에 포함한 fixed candidate 장면이다. 이 결과는 제품 RuntimeSession, Android/iOS, WPT 또는 일반 CSS admission의 근거가 아니다.

## 허용 및 거부 경계

- **허용:** 기존 Root HTML nine-direct-item Grid profile, 첫 빈 explicit-row `min-height:0` control, 뒤의 여덟 supported direct-text payload item, 여섯-member list의 정확한 three-fit + one min-content + one max-content + one auto multiset, 모든 세 fit-position 조합과 tail 순서, finite non-negative direct px 또는 direct percentage cap, `start|stretch`, 여덟 item 모두 `min-height:0` 또는 모두 `min-height:auto`.
- **비교 matrix cap:** `0px`, `10px`, `20%`를 세 fit track에 중복 없이 모두 배치한다. 이는 authored index와 typed cap의 교차를 분리하는 대표 입력이며, 임의의 cap 값에 대한 제품 수식 전체나 일반 Grid 지원으로 확대하지 않는다.
- **거부:** one/two/four-fit 변형을 새 three-fit predicate에 포함하지 않는다. 잘못된 list 길이·multiplicity, negative/non-finite/`calc()` cap, indefinite percentage basis, mixed item minimum, control auto minimum, row-span·explicit item placement, dense/column flow, scrollable overflow, nested/inline/replaced child, unsupported text/style, 추가 Grid owner·implicit column·unsupported row topology, 누락·중복·stale owner/revision/trace는 fail-closed한다.
- 실패 시 기존 published layout/render output을 부분 갱신하지 않으며, 같은 RuntimeSession에서 다음 유효 revision으로 복구되는지 확인한다. New admission은 Root HTML의 기존 fixed profile 안에서만 적용한다.
- Public API/FFI와 내부 숫자 계약을 추가하지 않는다. 일반 CSS Grid admission, 전체 WPT, hardware GPU/compositor pixel parity 및 C11.4/C11 완료는 범위 밖이다.

## 구현 및 완료 관문

1. 이 계획의 20개 검토와 계획 PR diff의 별도 30개 검토를 완료하고 계획 PR을 Rebase and merge한 뒤 제품 코드를 바꾼다.
2. Projection은 정확한 three-fit repeated-text auto-placement profile만 새로 허용한다. 모든 20 position triple과 여섯 tail order, cap의 authored 위치, computed min-height, 첫 control, grid owner/revision 관계를 검사한다. Existing one-/two-fit auto-placement, three-fit row-span은 별도 positive/negative control로 유지한다.
3. RuntimeSession matrix에서 같은 2,880 key의 min-height baseline/candidate를 비교해 Chrome 관측·독립 관계와 대조한다. 모든 eligible item의 content floor, typed track/cap trace와 row cycle을 확인한다.
4. Negative matrix는 1/2/4-fit, invalid multiplicity/order/length/cap/basis, mixed minimum, wrong control, child/text/style/flow/alignment, span placement, extra/missing owner, missing/stale/duplicate/mutated trace를 포함한다. Rejection atomicity와 same-session recovery를 검증한다.
5. Existing one-fit auto-placement 1,440 scene, two-fit auto-placement 2,880 scene, three-fit row-span 11,520 scene, corresponding zero-minimum baselines, row-span and repeated-text regressions, focused projection/layout/RuntimeSession tests를 회귀한다. 영향 crate의 Clippy `-D warnings`와 `cargo fmt --all -- --check`를 실행한다.
6. 동일 source SHA를 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 Root HTML route에서 light/dark로 실행해 `layout=ready`, draw/present와 screenshot을 기록한다. 이것은 visual smoke이며 pixel readback이나 실제 기기 검증을 대신하지 않는다.
7. 재사용한 공유 AVD/iPhone 17 Pro Simulator는 실행 후 shutdown하고, task 전용 앱/browser/임시 profile만 소유권과 active handle을 확인해 제거한다. 공유 user data와 일반 Xcode device template은 보존한다. Agent Browser는 고유 session으로 실행하고 닫은 뒤 session/process가 종료됐음을 확인한다. Terminal Browser는 사용하지 않는다.
8. 구현/runtime 변경은 계획 검토와 다른 실패 관점 20개로, 실제 구현 PR diff는 별도 30개 관점으로 검토한다. 가능한 commit-pinned 앱 화면은 PR 본문에 연결하고 Agent Browser에서 image load를 확인한다.
9. 구현 PR 병합 후 상태 대장, internal contract 0059, public roadmap 및 지정 Tailnet preview를 실제 결과와 맞춘다. 이 slice만으로 C11.4/C11 checkbox, 공식 `78/214`, contract version `0.1.0`은 바꾸지 않는다.

## 완료 판정

5,760-scene Chrome paired comparison와 독립 검산, RuntimeSession matrix/atomic recovery, 기존 profile 회귀, focused Rust/Clippy/rustfmt, Android/iOS 기본-route visual smoke, 별도의 implementation/runtime 20관점 및 PR diff 30관점 검토, simulator/browser 종료 확인, merge 후 status/spec/roadmap/Tailnet sync가 끝난 뒤에만 이 bounded slice를 구현 완료로 기록한다. Chrome 사전 비교만으로 implementation 완료를 표시하지 않는다.
