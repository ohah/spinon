# C11.4 · one-fit auto-placement text의 `min-height:auto`

- **상태:** 계획 및 Chrome 사전 비교 완료 · 계획 PR 대기 · 제품 구현 전 · C11.4/C11 미완료 · 공식 `78/214` 유지
- **직전 구현:** [two-fit auto-placement `min-height:auto`](./c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum.md) · PR [#1318](https://github.com/ohah/spinon/pull/1318)
- **기준 profile:** [one-fit auto-placement fit-content 위치 1–6](./c11-4-root-html-repeated-text-autoplacement-fit-content-positions.md) · 구현 PR [#1302](https://github.com/ohah/spinon/pull/1302)
- **Chrome 사전 비교:** [1,440-scene `min-height:auto`와 `min-height:0` 대조](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-one-fit-auto-min-precomparison-2026-11-10/README.md)
- **계획 실패 관점:** [서로 다른 20개](./c11-4-root-html-repeated-text-autoplacement-one-fit-auto-minimum-plan-review-20.md)
- **계획 PR diff 적대 검토:** [서로 다른 20개](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-one-fit-auto-min-precomparison-2026-11-10/plan-pr-diff-review-20.md)

## 결손과 제한 목표

기존 one-fit repeated-text auto-placement profile은 여덟 auto-placement item 모두 `min-height:0`인 입력을 지원한다. 두-fit auto-placement에서는 최근 `min-height:auto`를 허용했지만 one-fit auto-placement에 같은 item style을 적용하면 projection이 거부한다. 이 제한 slice는 기존 one-fit track list·placement·content를 유지하면서 여덟 text item의 computed `min-height:auto`만 추가로 허용한다. 첫 explicit-row control은 `min-height:0`로 유지한다.

검증할 동작은 fit-content track의 automatic minimum이 작은 authored cap에 의해 content-based minimum 아래로 줄어들지 않는다는 관계다. 예를 들어 `fit-content(0%)` 행은 해당 text의 content floor를 유지해야 한다. Chrome 157 사전 비교는 이 관계를 확인했지만 Spinon 구현 근거는 아니다.

## 고정 입력과 Chrome 기준

- 기본 parsed Root HTML route의 기존 9-direct-item Grid profile을 사용한다. Grid는 definite `40×220px`, `40px` explicit column 하나, `10px` explicit row 하나, gap 0, static positioning, LTR, row sparse auto-flow, `overflow:visible`, `align-content:start|stretch`다.
- Explicit row는 빈 control item 하나가 차지한다. 뒤의 auto-placement item 여덟 개는 각자 direct text payload div 하나만 가진다. 기존 고정 ASCII uppercase payload와 10px monospace/10px line-height를 유지하고 Chrome line-count `[2,2,2,3,2,2,3,1]`을 입력 hash 및 관측에 고정한다.
- `grid-auto-rows`는 정확히 여섯 member다. `fit-content(cap)`은 하나만 두고 authored 위치 1–6을 각각 확인한다. 나머지는 `min-content` 두 개, `max-content` 두 개, `auto` 한 개이며 고유 tail order 30개만 다룬다.
- Cap은 기존 비교와 같은 `20px`, `20%`, `0%`, `120%`다. Definite 220px block-size에서 percentage used cap은 각각 44px, 0px, 264px다. CSS 값의 authored/computed type과 순서를 보존한다.
- 기존 [one-fit `min-height:0` Chrome corpus](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-fit-content-positions-precomparison-2026-11-09/README.md)와 candidate를 같은 scene key로 비교한다. 새 Chrome 157 Agent Browser matrix는 `6 positions × 30 tails × 4 caps × 2 alignments = 1,440 scenes`다.
- 후보 사전 비교에서 1,920 fit-track cycle occurrence를 확인했다. 기준 `min-height:0`에서 content floor 아래였던 600 paired occurrence가 auto minimum에서 회복됐다(같은 authored input의 start/stretch pair를 각각 셈). Content floor 위반과 visible overflow는 각각 0개였다. `start` row 합은 모든 list에서 180px이고 `stretch`는 40px를 auto row occurrence에만 분배해 220px가 됐다. [전체 관측과 독립 verifier](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-one-fit-auto-min-precomparison-2026-11-10/README.md)를 구현 기대값의 기준으로 삼되, Chrome 결과를 RuntimeSession 성공으로 간주하지 않는다.
- CSS Grid [§6.6](https://www.w3.org/TR/css-grid-1/#min-size-auto)에 따라 scrollable overflow는 automatic minimum 조건을 만족하지 않는다. Fit-content의 automatic minimum은 그 authored cap으로 content-based minimum 아래 clamp되지 않는다. 이 계획은 고정 visible-overflow fixture에 한정하며 일반 Grid 수식 전체를 구현하지 않는다.

## 허용·거부 경계

- 9-item one-fit repeated-text auto-placement profile에서 여덟 direct text item에 computed `min-height:auto`를 허용한다. Explicit-row control은 계속 `min-height:0`이며 기존 여덟-item `min-height:0` profile도 그대로 지원한다.
- 여섯-member track list에는 정확히 한 fit-content, 두 min-content, 두 max-content, 한 auto를 둔다. Fit 위치 1–6, 30개 tail order, 유한한 0 이상 direct px/% cap, 고정 40×220/40×10 geometry와 기존 child/text grammar만 허용한다.
- `overflow:visible`, non-scrollable item minimum, 단일 explicit column, explicit row 뒤에서 시작하는 6-member implicit-row cycle, `start|stretch` alignment와 현재 Grid owner/revision/complete row trace 조건을 유지한다.
- scrollable `overflow:auto|scroll`, 숨김 overflow, 두 개 이상의 fit track, fit 누락/중복, 잘못된 tail/list 길이·multiplicity, 음수/비유한/계산식 cap, indefinite percentage basis, `min-height`의 다른 값, control item의 auto minimum, child 수·순서·topology·font·text·flow·alignment 변형은 계속 거부한다.
- two-fit auto-placement와 two-fit row-span profile은 각각 기존 경계를 유지한다. 새 admission은 다른 profile이나 일반 CSS cascade로 전파하지 않는다. Owner·revision·trace 누락 및 모순된 sizing 결과는 output을 부분 발행하지 않고 fail-closed여야 한다.
- public API/FFI, arbitrary Grid sizing, 일반 author CSS admission, WPT 적합성, Android/iOS surface pixel parity, 실기기/hardware GPU, C11.4/C11 완료는 목표 범위가 아니다.

## 구현·검증 관문

1. 이 계획의 실패 관점 20개 검토와 별도 계획 PR diff 20개 적대 검토를 마치고 계획 PR을 병합한 다음에만 제품 코드를 수정한다.
2. Style projection에서는 one-fit track list와 정확한 repeated-text auto-placement topology에서만 여덟 text item의 computed `min-height:auto`를 추가 허용한다. Control `min-height:0`, one-fit `min-height:0`, two-fit auto-placement auto minimum, two-fit row-span auto minimum을 각각 분리한 positive/regression control로 둔다.
3. Layout sizing은 fit-content의 auto minimum이 0%와 20px cap 아래로 줄지 않는지 확인한다. 1–6 cycle index, repeated row 7/8, definite percentage basis, non-fit row sizing, start/stretch의 auto-only distribution과 complete trace를 검증한다.
4. 기본 Root HTML RuntimeSession에서 Chrome의 1,440 scene을 동일 key로 대조한다. Computed minimum과 authored/computed track order, 9 item frame, 8 payload frame·line count, content floor, fit cap, row cycle, total height 및 overflow를 검사한다.
5. Negative tests에는 two-/zero-/multiple-fit list, 잘못된 multiplicity/tail/cap/percentage basis, scrollable·hidden overflow, 잘못된 control/item min-height, child/text/font/flow/alignment 변형, extra owner, stale revision과 불완전·변조 track trace를 포함한다. 거부 시 기존 published layout/render revision과 frames를 유지하고 같은 RuntimeSession에서 다음 valid 입력이 성공하는지 확인한다.
6. 기존 one-fit `min-height:0` 1,440-scene corpus, two-fit auto-placement auto-minimum 2,880-scene corpus, two-fit row-span auto-minimum 및 direct-text 810 corpus와 fit-content track-size regressions를 다시 확인한다. 영향 crate focused tests, scoped Clippy `-D warnings`, `cargo fmt --all -- --check`를 실행하고 base에서 재현되는 기존 실패는 따로 기록한다.
7. 동일 source revision을 Android API 37.x ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 Root HTML route에서 실행한다. `fit-content(0%)` floor, 20px cap의 3-line item, fit 위치 1–6의 대표 장면, start/stretch를 포함해 `layout=ready`, draw/present, HTML·app hash와 screenshot을 기록한다. Screenshot은 visual smoke이며 PixelCopy/Metal drawable readback이 아니다.
8. 실행 전에 simulator/device, Agent Browser, 앱 process와 output 디렉터리를 식별한다. 이번 검증이 만든 브라우저 session·앱·임시 simulator profile/data는 종료하고 제거한다. 다음 slice에서 사용할 공용 iPhone 17 Pro와 공유 Android AVD, user-owned simulator profile/data는 보존하며 정확한 task ownership이 확인되지 않은 리소스는 삭제하지 않는다.
9. 구현/runtime은 계획 검토와 다른 실패 관점 20개로 검토한다. Implementation PR diff도 서로 다른 20개 관점으로 검토한다. 검증된 Android/iOS screenshot이 있으면 PR 본문에 commit 고정 raw 링크를 넣고 merge 전 Agent Browser로 링크와 화면을 확인한다. Terminal Browser는 사용하지 않는다.
10. 구현 병합 후 `spec/STATUS.md`, internal contract `0059`, 이 계획과 공개 roadmap을 실제 결과 및 미지원 경계와 동기화한다. 범위상 C11.4/C11 체크, 공식 `78/214`, contract version `0.1.0`은 유지한다. source/hash를 확인한 뒤 사용자가 지정한 Tailnet roadmap preview를 업데이트한다.

## 완료 판정

이 bounded slice는 1,440-scene RuntimeSession 비교, independent content-floor/cap/cycle/frame/stretch checks, atomic rejection와 same-session recovery, 기존 one-/two-fit/span/text regressions, focused Rust/Clippy/rustfmt, Android/iOS 기본-route smoke, 별도 implementation/runtime 20관점 검토, implementation PR diff 20관점 검토와 merge 후 status/spec/roadmap/Tailnet sync를 마친 뒤에만 구현 완료로 기록한다. Chrome 사전 비교나 simulator screenshot만으로 구현 완료로 표시하지 않는다.
