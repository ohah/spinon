# C11.4 · four-fit repeated-text row-span item별 `min-height:0|auto`

**상태:** Chrome 157 사전 비교 138,240/138,240 scenes와 독립 검증 완료 · 구현 전 계획 실패 관점 20개 검토 완료 · 계획 PR diff 리뷰/생성 대기 · C11.4/C11 미완료 · 공식 `78/214` 유지

**선행 uniform 구현:** [four-fit row-span `min-height:auto`](./c11-4-root-html-repeated-text-row-span-four-fit-auto-minimum.md) · 구현 PR [#1357](https://github.com/ohah/spinon/pull/1357)

**직전 mixed 구현:** [five-fit row-span item별 minimum mask](./c11-4-root-html-repeated-text-row-span-five-fit-mixed-auto-minimum.md) · 구현 PR [#1372](https://github.com/ohah/spinon/pull/1372)

**사전 비교:** [Chrome 157 전체 관측·독립 검증](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum-precomparison-2026-11-12/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-four-fit-mixed-auto-minimum-precomparison.html)

**계획 검토:** [별도 실패 관점 20개](./c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum-plan-review-20.md) · [계획 PR 변경분 실패 관점 30개](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum-precomparison-2026-11-12/plan-pr-diff-review-30.md)

## 결손과 제한 목표

기존 parsed Root HTML four-fit row-span profile은 모든 span text item이 `min-height:0`인 baseline 또는 모든 item이 `min-height:auto`인 candidate만 지원한다. 현재 four-fit style admission은 item별 automatic minimum mask를 허용하지 않는다. CSS Grid automatic minimum은 각 grid item의 실제 span track과 computed overflow에 따라 개별 결정되므로 item별 `0|auto` 조합을 고정 profile 안에서 추가한다.

범위는 PR #1357의 정확한 여섯-member `grid-auto-rows` multiset `[fit-content(0px), fit-content(10px), fit-content(20%), fit-content(30px), min-content, auto]`, 기존 두 row-span topology, direct-text payload grammar, fixed placement와 Root HTML owner 조건이다. Fit track 순서와 typed cap은 15 position set × 24 cap order × 두 tail order의 720개 list 전부에서 보존한다. `span2Six`의 A–E 32 masks와 `span2Span3Six`의 A–D 16 masks를 지원한다. 첫 explicit-row empty control은 모든 scene에서 `min-height:0`이다.

이 bounded slice는 네 fit profile에서만 기존 all-zero/all-auto endpoint 사이의 혼합 item mask를 허용한다. 다른 fit 개수, auto-placement, 임의 Grid, 일반 CSS admission, public API/FFI 및 상위 C11.4 완료를 추가하지 않는다.

## 규범과 비교 모델

규범 기준은 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)다. Automatic minimum eligibility는 grid item별로 판정한다. 이 fixture의 eligible item은 scroll container가 아니고, 적어도 한 span track의 minimum sizing function이 `auto`이며, 여러 track을 span할 때 flexible track을 포함하지 않는다. `fit-content()`의 minimum sizing function은 `auto`다. Cap 값을 content minimum의 상한으로 사용하거나 한 item의 mask를 이웃 item에 전파하지 않는다.

비교 기준은 Agent Browser `0.31.1`의 HeadlessChrome `157.0.0.0`, macOS arm64, DPR 1이다. 고정 입력은 기존 definite `40×220px` Grid, `40px` column, `10px` explicit row, gap 0, sparse row flow, LTR, `overflow:visible`, monospace 10px/10px line-height text를 유지한다.

| 비교 축 | 고정 범위 |
| --- | --- |
| Six-member `grid-auto-rows` | 15 fit-position sets × 24 cap orders × 2 `[min-content,auto]` tail orders = 720 track lists |
| Item topology | span2Six A–E: 32 masks · span2Span3Six A–D: 16 masks |
| Paired conditions | forward/reverse DOM order × `align-content:start|stretch` |
| 전체 Chrome oracle | 720 × (32×2 + 16×2) × 2 = 138,240 scenes |
| 기존 uniform endpoint 대조 | PR #1357의 11,520 scenes와 zero/all-auto mask geometry 비교 |

Mask bit 순서는 semantic item ID 순서 A, B, C, D, E이며 두 번째 topology는 A–D만 쓴다. Mask 0은 기존 all-`0px` baseline이고 all-bits-set mask는 기존 all-`auto` candidate다. 기존 uniform 관측과 endpoint가 일치해야 한다. 각 scene은 authored/computed track, used rows, item/payload frame, line rectangles, minimum mask와 Grid geometry를 보존한다.

### Chrome 157 사전 관측

[독립 verifier](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum-precomparison-2026-11-12/verification.json)는 모든 138,240 scenes, 720 track list, 15 fit position set, 24 cap order, 2 tail order, 32/16 masks, paired source order·alignment 및 기존 uniform endpoint 11,520개를 확인했다.

- Automatic-minimum eligible item의 payload-floor 위반은 0이다. `min-height:0` 기준선의 payload-floor 미달은 span2Six 28,032건, span2Span3Six 9,216건이다.
- Grid block-end 초과 1,392건은 모두 `min-height:0` scene에서 관측됐다. 이 결과는 Chrome fixture 관측이며 제품 동작 보장이 아니다.
- Source-order 반전과 non-auto track의 alignment 차이는 각각 0 CSS px다. span frame과 covered-row 합의 최대 차이는 0.00045 CSS px다.
- 기존 all-zero/all-auto four-fit matrix와 11,520 endpoint의 최대 geometry 차이는 0 CSS px다.

이 독립 결과는 제한된 구현을 검토할 근거다. Spinon RuntimeSession의 baseline/candidate 비교가 끝나기 전 구현 성공으로 보지 않는다.

## 구현 범위와 사전 판정 기준

1. **계획 먼저 병합:** 계획 자체를 구현 전에 20개의 서로 다른 실패 관점으로 검토하고, 실제 계획 PR diff는 별도 30개 관점으로 확인한다. 이 계획 PR이 Rebase and merge된 뒤에만 제품 구현에 들어간다.
2. **정확한 style admission:** existing four-fit row-span track-list predicate와 two topology에서만 span item별 `ComputedCssDimension::LengthPx(0)` 또는 `Auto` mask를 허용한다. Existing explicit-row control은 계속 0이어야 한다. Five-/six-fit mixed helper를 four-fit에 무검증 재사용하지 않는다.
3. **정확한 layout profile:** six implicit rows, typed cap 위치·순서, fixed span placement, direct text child, one Grid owner, writing context, existing supported item style와 revision binding을 검증한다. Unsupported child, malformed mask, 다른 fit 개수 및 stale/partial trace는 fail-closed한다.
4. **RuntimeSession mixed sample:** 전체 Chrome oracle에서 deterministic한 20 track list를 뽑아 각 list의 모든 192 scene을 실행한다(3,840 scenes). Fixture의 ascending fit-position enumeration에서 15개 position set을 모두 대표하고 24 cap order 중 20개, 두 tail order를 포함한다. Cap rank는 fixture의 swap-recursion `capPermutations` 배열 index로 정의한다. 각 position index `p=0..14`에서 `(capRank=(7p) mod 24, tailRank=p mod 2)`를 선택하고, `p={0,3,6,9,12}`에서는 기존 선택과 겹치지 않는 cap rank `{3,6,9,10,13}`과 반대 tail rank를 추가한다. Sample builder와 독립 verifier는 exact 선택, 원본 Chrome hash, 각 mask/geometry, source order, alignment와 RuntimeSession 입력 순서를 검사한다. Sample은 전체 138,240 RuntimeSession scenes를 실행했다는 뜻이 아니다.
5. **Uniform endpoint 회귀:** 같은 candidate revision에서 기존 `parsed_root_html_four_fit_row_span_matches_all_11520_chrome_scenes` RuntimeSession matrix를 다시 실행한다. Mask 0/all-auto의 rows, item/payload/line geometry와 trace가 기존 결과와 `0.01 CSS px` 안에서 일치해야 한다. Existing four-fit auto-placement 5,760, 3-/5-/6-fit row-span, 5-/6-fit mixed, fit-position·track-rounding 및 explicit-row controls도 영향받는 focused regression gate로 실행한다. 기존 `parsed_root_html_four_fit_auto_minimum_rejects_mixed_items_then_recovers` test는 mixed item을 거부하는 assertion을 없애고 이 positive mask matrix와 올바른 control/invalid-value rejection으로 대체한다.
6. **독립 geometry assertion:** 각 span frame은 실제 covered used rows의 합과 일치하고, eligible automatic minimum item은 payload content floor 아래로 내려가지 않는다. Source-order reversal은 semantic ID별 geometry를 보존한다. `align-content:stretch`는 `auto` maximum track 외의 track을 늘리지 않는다. Chrome/RuntimeSession 허용 오차는 `0.01 CSS px`다. Grid-end overflow는 별도 집계해 사전 관측과 대조하며 예상값을 선험적으로 0으로 강제하지 않는다.
7. **Rejection과 atomic recovery:** 잘못된 fit count/cap/tail/order/percentage basis, control auto, mixed non-0/auto 값, wrong span, extra owner/item, altered font/text/overflow/writing context와 unsupported placement를 거부한다. 실패 revision은 부분 output을 게시하지 않고 같은 RuntimeSession의 다음 valid revision에서 복구해야 한다.
8. **Rounding boundary:** 기존 four-fit rounding behavior가 mixed mask에서도 Chrome 결과와 맞는지 exact half/fractional edge를 확인한다. 변경된 four-fit rounding 조건이나 shared rounding policy는 재현된 mismatch, 추가 독립 oracle, rounding edge regression 없이는 넓히지 않는다.
9. **Platform smoke:** 사용 전에 현재 booted device/app 상태를 기록하고 Android API 37 ARM64 AVD와 iOS 26.2 Simulator에서 기본 Root HTML route를 빌드해 light/dark `layout-ready`와 draw/present를 확인한다. 기본 route는 mixed feature fixture를 실행하지 않으므로 플랫폼 결과는 startup smoke로만 보고한다. 사용한 task-owned simulator/browser session은 종료 후 device/session/process 상태를 재확인한다. Simulator profile이나 데이터를 삭제하지 않는다.
10. **검토·문서:** 구현 후 새로운 failure mode 20개의 implementation/runtime 검토와 별도의 30개 PR diff 검토를 수행하고 local/remote diff hash와 PR head를 대조한다. Merge 후 내부 contract 0059, `spec/STATUS.md`, `spec/public/roadmap.html`, 이 계획 및 Tailnet preview를 실제 결과로 동기화한다. C11.4/C11 완료 체크, 공식 `78/214`, 숫자 contract `0.1.0`은 이 slice만으로 바꾸지 않는다.

### 검증 명령 범주

- Chrome oracle: `node .../capture-chrome.mjs` · `python3 .../verify-chrome-matrix.py`
- RuntimeSession mixed subset: `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test parsed_root_html_four_fit_mixed_minimum_matrix_chunk -- --nocapture`
- Uniform four-fit endpoints: `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test parsed_root_html_four_fit_row_span_matches_all_11520_chrome_scenes -- --nocapture`
- Focused RuntimeSession/style/layout tests, `cargo fmt --all -- --check`, and Clippy for changed crates. Test names must match the final implementation module before they are run; a no-match/zero-test command is not evidence.

## Runtime sample rationale

The full Chrome oracle has 138,240 scenes. RuntimeSession mixed execution uses 3,840 scenes across 20 full track-list contexts because each context already crosses every topology, per-item mask, source order and alignment. The sample covers every fit-position set, both tail orders, and 20 of 24 authored cap permutations. Existing four-fit uniform endpoints independently cover every one of the 720 lists and both minimum states (11,520 scenes). These denominators stay separate in reports.

## 제외

다른 fit 개수와 arbitrary cap/list/order/topology, auto-placement, implicit placement, nested/anonymous/replaced payload, scroll/clipping behavior, 일반 CSS Grid admission, public API·FFI, 전체 WPT, 앱 surface pixels, compositor parity, 실기기, hardware GPU 및 전체 C11.4/C11 완료는 범위 밖이다.
