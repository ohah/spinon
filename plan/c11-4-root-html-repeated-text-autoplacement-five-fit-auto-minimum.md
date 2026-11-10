# C11.4 다섯 fit repeated-text auto-placement `min-height:auto`

- **상태:** Chrome 157 사전 비교 17,280/17,280 scene 및 독립 검산 완료 · 계획 20관점 검토 완료 · 계획 PR 전 · C11.4/C11 미완료 · 공식 `78/214` 유지 · 내부 계약 `0.1.0` 유지
- **직전 자동 배치:** [네 fit `min-height:auto`](./c11-4-root-html-repeated-text-autoplacement-four-fit-auto-minimum.md) · PR [#1344](https://github.com/ohah/spinon/pull/1344)
- **사전 비교:** [Chrome 157 17,280-scene matrix와 독립 검산](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-min-precomparison-2026-11-11/README.md)
- **계획 검토:** [서로 다른 실패 관점 20개](./c11-4-root-html-repeated-text-autoplacement-five-fit-auto-minimum-plan-review-20.md)
- **계획 PR diff 검토:** [실제 변경분의 서로 다른 30개 관점](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-min-precomparison-2026-11-11/plan-pr-diff-review-30.md)

## 결손과 제한 목표

기본 parsed Root HTML 반복 text auto-placement에는 여섯-member `grid-auto-rows` list의 one-fit부터 four-fit까지 `min-height:auto` 제한 profile이 있다. 다섯 `fit-content()` member인 list는 거부한다.

이 slice는 여덟 auto-placed text item의 computed `min-height:auto`가 다섯 fit track에 놓일 때 content floor를 보존하는 profile을 검토한다. 고정 list에는 `fit-content()` 다섯 개와 `min-content`, `max-content`, `auto` 가운데 하나인 tail 한 개가 있다. tail 종류와 authored 위치는 제한적으로 허용하며, 여덟 번째 item은 list wrapping에 따라 앞쪽 track으로 되돌아온다. 첫 빈 explicit-row control은 계속 `min-height:0`이다. 기존 one-/two-/three-/four-fit auto-placement 및 row-span profile은 별도 경계로 유지한다.

## 구현 전 비교 모델

- 규범 기준은 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-functions), [§11.3–11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-algorithm)다. `fit-content()`의 minimum track sizing function은 `auto`다. Automatic minimum은 item의 overflow와 span되는 track sizing function 등 CSS 조건에 따라 판단하며, 모든 tail track에 무차별 적용하지 않는다.
- 독립 oracle은 Agent Browser 0.31.1의 HeadlessChrome `157.0.0.0`, `MacIntel`, DPR 1이다. 이는 browser CSS Grid 결과이며 Spinon RuntimeSession의 결과가 아니다.
- Grid와 text 입력은 [사전 비교 README](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-min-precomparison-2026-11-11/README.md)에 고정했다. definite `40×220px` Grid, explicit `40px` column과 빈 `10px` row, gap 0, LTR sparse row auto-flow, `overflow:visible`, 고정 10px monospace text를 쓴다.
- 여섯-member list의 fit-position quintuple 6가지, tail 종류 3가지, 다섯 cap (`0px`, `10px`, `20%`, `30px`, `40px`)의 authored-order 순열 120가지로 2,160 track lists를 구성한다. definite 220px basis에서 `align-content:start|stretch`, DOM forward/reverse, item `min-height:0|auto`를 교차해 17,280 scenes 및 8,640 paired keys를 관측했다.
- Candidate `min-height:auto`의 fit-track payload-floor violation은 57,600 occurrence 중 0이었다. Baseline `min-height:0`은 fit-track floor 아래 21,600 occurrence, tail별 own-row payload overflow 7,200건을 보였다. Tail floor violation과 item-frame Grid-end overflow는 0이다. `auto` tail에서만 stretch row result가 start와 달랐고 baseline payload 600건이 Grid 끝 밖에 있었다. 따라서 tail 종류·minimum mode·alignment를 따로 비교한다.

## 제한 admission 계약

- 기본 parsed Root HTML text-paint RuntimeSession 경로의 단일 Grid owner만 허용한다. Geometry, viewport, explicit track, direct child 수·순서, text payload/font/line-height, writing mode, direction, gap, overflow, flow와 placement는 fixed precomparison profile에 맞아야 한다.
- `grid-auto-rows`는 여섯 member여야 하며 정확히 다섯 `fit-content()`와 한 tail member로 구성한다. Tail 후보는 `min-content`, `max-content`, `auto` 각각 한 번만 허용하고, 여섯 authored position을 모두 검토한다. Fit cap은 유한하고 0 이상인 direct px 또는 definite-basis direct percentage만 받으며 cap type/order를 보존한다. `calc()` 및 indefinite percentage는 거부한다.
- 여덟 auto-placement text item은 같은 computed `min-height:0` 또는 같은 computed `min-height:auto` mode를 사용한다. Candidate auto mode는 non-scrollable item만 허용한다. 첫 empty explicit-row control은 `min-height:0`이어야 한다. 각 text item은 한 row에만 놓이며 dense/column flow, explicit placement, span 및 implicit column은 허용하지 않는다.
- Candidate에서 각 fit row의 item frame은 해당 text payload floor 이상이어야 한다. Grid row sequence, authored cap, item/payload frame, line count, Grid owner/revision 및 completion trace는 같은 input revision이어야 한다. `auto` tail의 `align-content:stretch` 결과는 별도 expected values와 대조하며 min/max tail과 합산하지 않는다.
- 잘못된 list/count/multiplicity/cap/style/topology/owner/revision/trace 입력은 결과를 부분 게시하지 않고 거부한다. 같은 RuntimeSession의 다음 유효 revision이 성공해야 한다.
- 이 profile은 공개 API/FFI 또는 내부 숫자 계약을 추가하지 않는다. 일반 CSS Grid admission, 전체 WPT, compositor pixel parity, PixelCopy/Metal readback, 실기기/hardware GPU 또는 C11.4/C11 완료를 뜻하지 않는다.

## 구현 관문

1. 계획의 독립 20 실패 관점 검토와 계획 PR의 별도 30 diff 관점 검토를 마친 뒤 계획 PR을 Rebase and merge한다. 계획 PR 병합 전에는 제품 코드를 수정하지 않는다.
2. Style projection에서 정확한 five-fit list만 추가한다. 여섯 fit position, 세 tail value, tail position, cap authored index/type를 검사한다. One-/two-/three-/four-fit auto-placement 및 row-span은 positive/negative controls로 보존한다.
3. 기본 parsed Root HTML RuntimeSession에서 17,280 scene을 Chrome observation과 비교한다. 8,640 paired keys, 6 fit-position quintuples, 2,160 track lists, 120 cap permutations, list wrapping, source order, line count, fit-row floor, track/cap trace 및 Grid-end geometry를 검사한다. `auto` tail의 stretch 차이와 baseline의 Grid-end overflow 600건을 별도 assertion으로 유지한다.
4. Rejection matrix는 0 및 6 fit count, 다섯 member/일곱 member, tail 누락·중복·기타 값, invalid/non-finite/negative/`calc()` cap, indefinite percentage, mixed minimum, control auto minimum, explicit placement/span/dense/column flow, scrollable overflow, nested/inline/replaced child, unsupported style, 추가 owner/column/row, malformed/stale/duplicate trace를 다룬다. 1–4 fit positive regressions 및 각기 다른 기존 rejection을 유지한다. Atomic rejection 뒤 same-session recovery를 확인한다.
5. Existing one-fit 1,440 scene, two-fit 2,880 scene, three-fit 5,760 scene, four-fit 5,760 scene auto-placement profile, three-fit row-span 11,520 scene, one/two-fit row-span auto-minimum, repeated-text 810 scene 및 관련 Grid track-sizing regressions를 실행한다. 변경 crate focused tests, 제품·fixture-test package Clippy `-D warnings`, `cargo fmt --all -- --check`를 기록한다. 실행하지 않은 suite는 통과로 보고하지 않으며, broad-suite 기준선 실패는 분리한다.
6. 동일 source SHA의 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 기본 Root HTML route를 light/dark로 확인한다. Fixture/app asset hash, `layout=ready`, draw/present 및 OS screenshot을 보존한다. 화면은 visual smoke이며 pixel readback 또는 hardware GPU 증거가 아니다.
7. Simulator는 사용 전에 현재 공유 profile/device를 기록한다. 공유 기기는 재사용 뒤 shutdown하고, task-owned로 새로 만든 기기는 캡처 뒤 삭제한다. 앱/data source는 사전 hash와 비교해 복원한다. Agent Browser는 고유 session을 쓰고 종료 뒤 해당 session/process가 닫혔는지 확인한다. Terminal Browser와 관련 없는 session/process는 건드리지 않는다.
8. 계획 검토와 겹치지 않는 새로운 20개 implementation/runtime 실패 관점 및 실제 implementation diff에 대한 별도 30개 검토를 수행한다. 가능한 화면은 commit-pinned raw URL로 PR에 첨부하고 Agent Browser에서 로드되는지 확인한다.
9. Rebase and merge 이후 internal contract 0059, `spec/STATUS.md`, `spec/public/roadmap.html`, 계획/runtime evidence와 지정 Tailnet preview를 실제 merge SHA와 hash에 맞춘다. 이 bounded slice로 C11.4/C11 checkbox, 공식 `78/214`, internal numeric contract `0.1.0`을 올리지 않는다.

## 계획 완료 판정

독립 20관점 계획 검토와 별도 30관점 PR diff 검토, Chrome 사전 비교의 재현 입력·결과·한계가 연결되고 admission/rejection, regression 및 platform gate가 구체적으로 고정되면 계획 단계가 끝난다. Chrome 사전 비교 자체로 Spinon 제품 동작을 완료 처리하지 않는다.
