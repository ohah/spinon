# C11.4 · 네 fit repeated-text auto-placement `min-height:auto`

- **상태:** Chrome 157 사전 비교와 20/30 관점 계획 검토 완료 · 계획 PR [#1343](https://github.com/ohah/spinon/pull/1343) Rebase and merge (`2fa78846db7490d75e4d894c59eb712a32e20576`) · 구현 PR [#1344](https://github.com/ohah/spinon/pull/1344) open · 구현/runtime 20관점 및 실제 PR diff 30관점 검토 완료 · C11.4/C11 미완료 · 공식 `78/214` 유지 · 내부 계약 `0.1.0` 유지
- **직전 자동 배치:** [three-fit `min-height:auto`](./c11-4-root-html-repeated-text-autoplacement-three-fit-auto-minimum.md) · PR [#1342](https://github.com/ohah/spinon/pull/1342)
- **직전 row-span:** [three-fit `min-height:auto`](./c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum.md) · PR [#1338](https://github.com/ohah/spinon/pull/1338)
- **사전 비교:** [Chrome 157 5,760-scene matrix와 독립 검산](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-four-fit-auto-min-precomparison-2026-11-11/README.md)
- **계획 검토:** [서로 다른 실패 관점 20개](./c11-4-root-html-repeated-text-autoplacement-four-fit-auto-minimum-plan-review-20.md)
- **계획 PR diff 검토:** [실제 변경분의 서로 다른 30개 관점](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-four-fit-auto-min-precomparison-2026-11-11/plan-pr-diff-review-30.md)

## 결손과 제한 목표

기본 parsed Root HTML 반복 text auto-placement 경로에는 six-member `grid-auto-rows` list의 one-fit, two-fit, three-fit `min-height:auto` profile이 있다. 같은 9-direct-child profile에서 네 `fit-content()` member를 가진 list는 거부한다.

이 계획은 여덟 자동 배치 text item의 computed `min-height:auto`가 네 fit track에 놓일 때 content floor를 보존하는 제한 profile을 추가한다. 여섯-member list는 정확히 `fit-content()` 4개, `min-content` 1개, `max-content` 1개다. 두 tail member는 각각 한 번만 나오며 두 authored 순서를 모두 허용한다. 여덟 item 중 첫 empty explicit-row control은 계속 `min-height:0`이다. 기존 one-/two-/three-fit auto-placement 및 three-fit row-span profile은 별도 경계로 유지한다.

## 구현 전 비교 모델

- 규범 기준은 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-functions), [§11.3–11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-algorithm)다. §7.2에서 `fit-content()`의 min track sizing function은 `auto`다. §6.6 조건과 같이 non-scrollable item이 해당 축의 auto-minimum track 하나 이상을 차지하고 flexible track을 걸치지 않으면 content-based automatic minimum을 사용한다. `min-content`와 `max-content` tail은 auto-minimum eligibility와 track contribution을 구분하는 대조군이다.
- 독립 oracle은 Agent Browser 0.31.1이 실행한 HeadlessChrome `157.0.0.0`, `MacIntel`, DPR 1이다. Chrome CSS Grid 관측이며 Spinon RuntimeSession 구현 근거로 취급하지 않는다.
- Grid는 definite `40×220px`, explicit `40px` column과 빈 `10px` row, gap 0, LTR sparse row auto-flow, `overflow:visible`을 사용한다. direct child는 빈 explicit control 하나와 여덟 text item이다. text는 10px monospace/10px line-height이며 line count는 `[2,2,2,3,2,2,3,1]`이다.
- `grid-auto-rows`는 여섯 member로 고정한다. Fit 위치 quadruple 15개, tail order 2개, four-cap assignment 24개(`0px`, `10px`, `20%`, `30px`)가 720개 unique authored list를 만든다. 220px definite block-size basis에서 각 list를 `align-content:start|stretch`, DOM forward/reverse, item `min-height:0|auto`와 교차한다. 총 5,760 scenes와 2,880 paired keys다.
- 사전 비교는 fit-position 조합별 track row와 item/payload frame을 비교한다. `min-height:auto`의 fit-row payload-floor 위반은 0, paired `min-height:0`의 fit-row under-floor occurrence는 7,200, non-fit tail floor 위반은 0, Grid block end 초과는 0이었다. `align-content:stretch`와 `start`의 row sizes는 이 auto track 없는 matrix에서 같았다. 상세 값과 corpus는 사전 비교 README 및 `verification.json`에 고정했다.

## 제한 admission 계약

- 기본 parsed Root HTML text-paint RuntimeSession 경로의 단일 Grid owner만 대상으로 한다. Grid geometry, viewport, explicit track, child count/order, text payload/font/line-height, writing mode, direction, alignment, gap, overflow와 static placement는 고정 precomparison profile과 같아야 한다.
- `grid-auto-rows`는 여섯 member, 정확한 fit/min/max multiplicity, 15개 위치 quadruple 중 하나, min/max tail 두 순서 중 하나여야 한다. Fit cap은 유한하고 0 이상인 direct px 또는 definite-basis direct percentage만 받으며 authored index와 type을 보존한다. `calc()`와 indefinite percentage basis는 거부한다.
- 여덟 auto-placement text item은 같은 computed `min-height:0` 또는 같은 `min-height:auto` mode를 사용한다. Auto mode는 non-scrollable item만 허용하고, empty explicit-row control은 `min-height:0`이어야 한다. 각 text item은 row 하나에만 배치되고 dense/column flow, explicit placement, span 및 implicit column을 허용하지 않는다.
- `min-height:auto` candidate에서 fit row의 item frame은 text payload floor 이상이어야 한다. Row order, track cap, item/payload geometry, line count, Grid owner·revision·완료 trace가 같은 input revision에 속하는지 대조한다. Tail rows의 automatic minimum 적용 여부를 fit rows와 합산하지 않는다.
- 잘못된 list, cap, style, topology, owner, revision 또는 trace는 새 결과를 부분 게시하지 않고 거부한다. 같은 RuntimeSession에서 다음 유효 revision이 성공하는지 확인한다.
- 이 profile은 공개 API/FFI나 내부 숫자 계약을 추가하지 않는다. 일반 CSS Grid admission, 전체 WPT, PixelCopy/Metal surface parity, compositor 이후 pixel, 실기기/hardware GPU 또는 C11.4/C11 완료를 뜻하지 않는다.

## 구현 관문

1. 이 계획의 20개 실패 관점 검토와 계획 PR diff의 별도 30개 검토를 마치고 계획 PR을 Rebase and merge한 뒤 제품 코드를 수정한다.
2. Style projection은 정확한 four-fit list만 추가로 허용하고 fit position 15개, tail order 2개, cap index와 typed cap을 검증한다. One-/two-/three-fit auto-placement와 three-fit row-span은 기존 positive/negative controls로 보존한다.
3. 기본 parsed Root HTML RuntimeSession에서 5,760 scene 전체를 Chrome observation과 비교한다. paired 2,880 keys, 15 position sets, 720 track lists, repeated list wrapping, source order, line count, eligible fit-row floor, row/cap trace와 block-end geometry를 확인한다.
4. Rejection matrix는 이 profile에서 허용하지 않는 0 및 5–6 fit count, 다섯/일곱 member, tail multiplicity와 `auto` tail, invalid/non-finite/negative/`calc()` cap, indefinite percentage, mixed item minimum, control auto minimum, explicit placement/span/dense/column flow, scrollable overflow, nested/inline/replaced child, unsupported style, extra owner/column/row와 malformed/stale/duplicate trace를 다룬다. 기존 1–3 fit auto-placement는 각자의 positive regression으로 유지하고, 두 min/max tail order는 positive control에서 모두 확인한다. Atomic rejection 뒤 same-session recovery를 확인한다.
5. Existing one-fit auto-placement 1,440 scene, two-fit auto-placement 2,880 scene, three-fit auto-placement 5,760 scene, three-fit row-span 11,520 scene, one/two-fit row-span automatic-minimum regressions, repeated-text 810 scene과 관련 Grid track-sizing tests를 회귀한다. 변경 crate focused tests, 제품·fixture-test 대상의 package Clippy `-D warnings`, `cargo fmt --all -- --check` 결과를 기록한다. 기준선 확인 전부터 broad-suite 실패를 이번 변경의 원인으로 분류하지 않는다.
6. 동일 source SHA의 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 기본 route를 light/dark로 확인한다. Fixture/app asset hash, `layout=ready`, draw/present, OS screenshot을 보존한다. Screenshot은 visual smoke이며 pixel readback이나 hardware GPU 증거가 아니다.
7. 공유 simulator profile은 재사용 뒤 shutdown한다. 새 task-owned simulator는 캡처 뒤 삭제한다. 앱/data source는 백업 hash와 비교해 원상 복원한다. Agent Browser는 고유 session을 쓰고 종료 뒤 session/process가 사라졌는지 확인한다. Terminal Browser는 사용하지 않는다.
8. 계획과 겹치지 않는 새 20개 implementation/runtime 실패 관점과 실제 구현 diff의 별도 30개 관점을 검토한다. PR 본문에 가능한 Android/iOS 화면을 commit-pinned raw URL로 첨부하고 Agent Browser에서 로드를 확인한다.
9. Rebase and merge 뒤 내부 계약 0059, `spec/STATUS.md`, `spec/public/roadmap.html`, 구현 계획/evidence와 지정 Tailnet preview를 실제 결과·해시와 동기화한다. 이 bounded slice만으로 C11.4/C11 checkbox, 공식 `78/214`, internal numeric contract `0.1.0`을 올리지 않는다.

## 계획 완료 판정

계획 PR의 독립 20관점 검토와 실제 diff의 독립 30관점 검토, Chrome 사전 비교의 입력·결과·한계가 연결되고 구현 범위·거부 범위·회귀·platform gate가 구체적으로 고정되면 계획 단계가 완료된다. Chrome 사전 비교 자체로 제품 구현을 완료 처리하지 않는다.
