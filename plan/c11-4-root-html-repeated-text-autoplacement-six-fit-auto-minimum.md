# C11.4 six-fit Root HTML repeated-text auto-placement `min-height:auto`

- **상태:** Chrome 157 사전 비교 5,760/5,760 scene 및 독립 검산 완료 · 계획 20관점 검토 완료 · 계획 PR 검토 대기 · 제품 코드 미변경
- **직전 구현:** [다섯-fit auto-placement](./c11-4-root-html-repeated-text-autoplacement-five-fit-auto-minimum.md) · 구현 PR [#1349](https://github.com/ohah/spinon/pull/1349)
- **사전 비교:** [Chrome 157 six-fit matrix와 독립 검산](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-six-fit-auto-min-precomparison-2026-11-11/README.md)
- **계획 검토:** [서로 다른 실패 관점 20개](./c11-4-root-html-repeated-text-autoplacement-six-fit-auto-minimum-plan-review-20.md)

## 결손과 제한 목표

현재 기본 parsed Root HTML repeated-text auto-placement에는 여섯-member `grid-auto-rows` list에서 one-fit부터 five-fit까지의 bounded `min-height:auto` profile이 있다. six-fit, 즉 여섯 track이 모두 `fit-content()`인 profile은 아직 거부한다.

이 계획은 고정된 nine-direct-item Grid profile에서 자동 배치된 여덟 text item의 computed `min-height:auto`가 six-fit implicit rows의 content floor를 보존하는지 구현·검증한다. Chrome 157 사전 비교에서는 `min-height:0` 기준선의 7,200/23,040 item occurrence가 text payload floor 아래로 내려가고, `min-height:auto` 후보에서는 0/23,040이었다. 이 결과는 해당 입력 matrix에만 적용한다.

## 구현 전 비교 모델

- 규범 기준은 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-terminology), [§11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch)다. `fit-content()` track의 minimum sizing function은 `auto`이며, 고정 `fit-content()` argument는 §6.6의 content-based minimum을 fixed max sizing처럼 자르지 않는다.
- 독립 oracle은 Agent Browser 0.31.1, HeadlessChrome `157.0.0.0`, `MacIntel`, DPR 1이다. 규범 비교는 고정 W3C CRD이며 Chrome 관측은 이 profile의 실제 CSS 결과다. Chromium 157 Grid track-sizing source는 버전 고정 구현 참고로만 남긴다.
- [사전 비교 fixture](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-six-fit-auto-min-precomparison-2026-11-11/oracle-fixture.html)는 definite `40×220px` Grid, explicit `40px` column 및 빈 `10px` row, gap 0, LTR sparse row auto-flow, `overflow:visible`, 10px monospace/10px line-height의 고정 text 8개를 쓴다. 첫 control은 explicit row에 놓이며 계속 `min-height:0`이다.
- 여섯 implicit `grid-auto-rows` member는 모두 `fit-content()`다. direct cap `0px`, `10px`, `20%`, `30px`, `40px`, `50px`의 720 authored 순열을 definite 220px percentage basis에서 비교했다. 각 list는 `align-content:start|stretch`, DOM forward/reverse, text-item `min-height:0|auto`와 교차한다.
- 총 5,760 scenes / 2,880 paired keys에서 computed track list와 alignment, used row heights, item/payload frame, line count/top, source order를 수집했다. 독립 검산은 Chrome metadata, 전체 axis coverage, scene ordering, six-fit list/cap 순서, item-row 연결과 payload floor를 확인했다.
- 결과: candidate automatic minimum fit-track occurrence 23,040, candidate floor violation 0, baseline under-floor 7,200, baseline visible payload row overflow 7,200, Grid-end overflow 0, baseline/candidate row-size pair 차이 2,880/2,880, `start`와 `stretch`의 row 및 item geometry 차이 0이다. 전체 값과 제한은 [검산 결과 JSON](https://raw.githubusercontent.com/ohah/spinon/fc22a202944c9ebd2026d9abe30d8dc5df1caf18/spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-six-fit-auto-min-precomparison-2026-11-11/verification.json)에 있다.

## 제한 admission 계약

- 기본 parsed Root HTML text-paint `RuntimeSession` 경로의 단일 Grid owner만 대상으로 한다. Grid geometry, definite percentage basis, explicit track, control, direct child 수·순서, text payload/font/line-height, writing mode, direction, gaps, overflow, auto-flow와 placement는 사전 비교 profile에 고정한다.
- `grid-auto-rows`는 정확히 여섯 member이고 여섯 전부 direct `fit-content()`여야 한다. 이 bounded slice는 computed cap multiset `{0px, 10px, 20%, 30px, 40px, 50px}`의 720 authored order만 허용한다. 각 typed cap 값과 index/order를 같은 style revision의 trace로 보존한다. computed absolute px로 정규화된 `calc()`는 정규화 결과가 이 multiset의 해당 px 값일 때만 허용한다. duplicate/missing/other finite cap, unresolved `calc()`, indefinite percentage, negative/non-finite cap과 다른 list/member 수는 거부한다.
- 첫 빈 explicit-row control은 계속 `min-height:0`이어야 한다. 여덟 direct text item은 모두 computed `min-height:0`이거나 모두 `min-height:auto`여야 한다. candidate auto mode는 non-scrollable item에만 허용한다. mixed minimum, row-span, explicit placement, dense/column flow, implicit column, nested/replaced/inline content는 이 slice에서 허용하지 않는다.
- Candidate 각 item frame은 같은 row의 text payload floor 이상이어야 한다. Row sequence, CSS cap/order, computed `min-height`, line layout, item/payload frame, Grid owner/revision 및 completion trace가 단일 input revision에 속해야 한다. `align-content:start|stretch`의 row 및 item geometry는 고정 Chrome 관측과 각각 비교한다.
- 잘못된 count/list/cap/style/topology/owner/revision/trace는 partial layout이나 stale completion을 publish하지 않고 fail-closed해야 한다. rejection 뒤 같은 `RuntimeSession`의 새 유효 revision이 성공해야 한다.
- 이 slice는 public API/FFI, 내부 numeric contract version, 일반 CSS Grid admission, arbitrary content/cap, nested grid, dynamic mutation, row-span, full WPT, compositor pixel parity, 하드웨어 GPU 또는 C11.4/C11 완료를 추가하지 않는다.

## 구현 및 검증 관문

1. 계획 PR 병합 전 제품 코드를 수정하지 않는다. 계획 문서 자체를 서로 다른 실패 경로 20개로 검토하고, 실제 계획 PR diff도 별도의 서로 다른 20관점으로 검토한다. 결함을 찾으면 계획과 영향받은 검토 항목을 고친다.
2. CSS Grid style projection에 six-fit predicate를 별도 bounded 조건으로 추가한다. 기존 one-/two-/three-/four-/five-fit auto-placement이나 row-span admission을 대체하거나 일반화하지 않는다. 정확한 cap multiset의 720 direct cap order, missing/duplicate/unlisted cap rejection, empty control, order/position mapping, supported computed cap type를 검사한다.
3. 기본 parsed Root HTML `RuntimeSession`의 Chrome 157 matrix를 재현한다: 5,760 scenes, 2,880 paired keys. `grid-auto-rows` authored/computed order, computed min-height, used rows, 9 item frames, 8 payload frames/line boxes, alignment geometry, baseline overflow, candidate floor, Grid-end overflow를 독립 관측과 대조한다. Evidence용 capture와 product RuntimeSession 결과를 서로 대체하지 않는다.
4. Focused rejection matrix는 0–5-fit list, 7-member list, invalid/non-finite/negative/unresolved cap, indefinite percentage, mixed minimum, control auto minimum, missing/extra child, wrong text/style/geometry, nested/inline/replaced content, overflow scroll, dense/column flow, explicit row/span, extra Grid owner 및 stale trace/revision을 다룬다. Atomic rejection 뒤 같은 session recovery도 확인한다.
5. Existing one-/two-/three-/four-/five-fit auto-placement, repeated-text row-span, explicit/implicit row, `fit-content()` cap, style-revision/trace 및 unrelated Root HTML Grid tests를 regression gate로 실행한다. 변경한 Rust에 `cargo fmt --all -- --check`와 영향받은 crate Clippy를 실행하고 결과를 보존한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기본 Root HTML route의 light/dark 화면을 visual smoke로 확인한다. V8/runtime revision, app asset hash, layout-ready, draw/present 및 OS screenshot을 묶는다. 화면 smoke를 PixelCopy/Metal readback, compositor parity, physical-device/hardware-GPU 또는 full CSS proof로 확대하지 않는다.
7. 플랫폼 기기 사용 전 상태를 확인한다. 공유 iPhone 17 Pro는 작업 후 shutdown하고 Simulator.app을 종료한다. task-owned Android clone을 만들면 캡처 후 종료·삭제하며, 공유 AVD/iOS device data 및 사용하지 않은 디바이스는 건드리지 않는다. Agent Browser는 전용 session으로 실행하고 종료 뒤 해당 session이 inactive/`pid:null`인지 확인한다. Terminal Browser 및 관련 없는 HWP Agent Browser session/process는 보존한다.
8. 구현 뒤 기존 계획 검토와 겹치지 않는 새 implementation/runtime 실패 관점 20개를 작성한다. 실제 구현 PR diff를 별도의 서로 다른 20관점으로 검토하고 PR head diff와 로컬 diff가 같은지 확인한다. PR 본문에는 scope, matrix/runtime/platform 근거, 남은 범위와 가능한 Chrome/앱 화면을 넣는다. Rebase and merge 후 exact merge SHA와 PR check 상태를 기록한다.
9. 병합 후 internal contract 0059, `spec/STATUS.md`, `spec/public/roadmap.html`, 계획 및 evidence의 PR/merge 상태를 동기화한다. 해당 Tailnet preview를 현재 선택 경로와 hash 기준으로 갱신하고 Agent Browser desktop/mobile에서 확인한 뒤 전용 browser session을 닫는다.

## 계획 완료 판정

사전 비교와 독립 검산, 계획 검토 20관점, 별도 PR diff 검토 20관점, admission/rejection, runtime regression, 플랫폼 smoke 및 자원 정리 절차가 구체적으로 연결되면 계획 단계가 끝난다. Chrome 사전 비교는 Spinon 구현 결과가 아니며 C11.4/C11의 공식 완료 수를 바꾸지 않는다.
