# C11.4 · three-fit repeated-text row-span item별 mixed `min-height:0|auto`

**상태:** 다음 제한 Grid slice로 선정 · Chrome 157 사전 oracle 138,240/138,240 scenes와 독립 검증 완료 · 계획 실패 관점 20개 검토 완료 · 구현 시작 전 · C11.4/C11 미완료 · 공식 `78/214` 유지

**선행 uniform 구현:** [three-fit row-span `min-height:auto`](./c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum.md) · 구현 PR [#1338](https://github.com/ohah/spinon/pull/1338)

**인접 mixed 구현:** [four-fit](./c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum.md) · 구현 PR [#1375](https://github.com/ohah/spinon/pull/1375) · [five-fit](./c11-4-root-html-repeated-text-row-span-five-fit-mixed-auto-minimum.md) · 구현 PR [#1372](https://github.com/ohah/spinon/pull/1372) · [six-fit](./c11-4-root-html-repeated-text-row-span-mixed-auto-minimum.md) · 구현 PR [#1369](https://github.com/ohah/spinon/pull/1369)

**사전 비교:** [Chrome 157 138,240-scene oracle과 verifier](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-precomparison-2026-11-14/README.md) · [고정 fixture](../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-three-fit-mixed-auto-minimum-precomparison.html)

**계획 검토:** [서로 다른 실패 관점 20개](./c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-plan-review-20.md) · [계획 PR diff 실패 관점 30개](./c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-pr-review-30.md).

## 결손과 제한 목표

현재 parsed Root HTML three-fit repeated-text row-span profile은 모든 span item이 `min-height:0`인 baseline과 모든 span item이 `min-height:auto`인 candidate만 받는다. 서로 다른 item의 automatic minimum 상태를 한 장면에 섞으면 거부한다. CSS Grid의 automatic minimum eligibility는 item별로 판정되므로, 기존 세 fit-content track list와 두 row-span topology 안에서 semantic item마다 `0px` 또는 `auto`를 고정 mask로 허용하는 제한 slice를 추가한다.

허용 입력은 정확히 여섯 개의 implicit row track으로 된 `[fit-content(), fit-content(), fit-content(), min-content, max-content, auto]` multiset, direct text item, 두 개의 기존 six-row row-span topology, definite 220px block-size percentage basis, visible overflow와 기존 Root HTML owner 조건이다. Fit track의 위치 20개, cap 순서 여섯 개, intrinsic tail 순서 여섯 개를 보존한다. `span2Six`는 A–E 32개 mask, `span2Span3Six`는 A–D 16개 mask를 사용한다. 첫 explicit-row empty control은 모든 scene에서 계속 `min-height:0`이다.

이 slice는 세 fit track의 두 uniform endpoint 사이에 item별 mixed mask만 추가한다. 다른 fit 개수, arbitrary track list·cap·topology, auto-placement, implicit placement, public CSS admission, public API/FFI 또는 상위 C11.4 완료를 추가하지 않는다.

## 규범 기준과 구현 전 비교 모델

- 규범 기준은 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)다. §6.6의 automatic minimum은 computed overflow가 scrollable이 아니고 span 안에 `auto` minimum track이 있으며, 여러 track을 span하면 그 span에 flexible track이 없을 때만 content-based minimum이다. §7.2.1에 따라 `fit-content()`의 minimum sizing function은 `auto`다. §11.5.1의 spanning contribution 순서는 cap별 used row와 item geometry의 비교 기준이다.
- 독립 oracle은 Agent Browser `0.31.1`이 구동한 HeadlessChrome `157.0.0.0`, macOS arm64, DPR 1이다. Chrome 관측은 제품 RuntimeSession 구현 결과가 아니다.
- Fixture Grid의 content box는 `40×220 CSS px`, border box는 실제 1px red border를 포함한 `42×222 CSS px`다. `box-sizing:border-box`로 content 크기와 20% percentage basis를 고정한다. `overflow:visible`, `align-content:start|stretch`, LTR, sparse row flow, zero gap과 10px monospace/line-height ASCII direct text를 유지한다.
- Track list는 `20 fit-position triples × 6 cap permutations × 6 tail permutations = 720`개다. Caps는 `0px`, `10px`, `20%`이고 tails는 `min-content`, `max-content`, `auto`의 모든 순서다.
- Item families는 span-2 5개로 여섯 implicit row를 채우는 topology와 span-2 2개 및 span-3 2개로 같은 여섯 row를 채우는 topology다. 각 topology를 DOM forward/reverse로 실행한다. 두 alignment에서 각 semantic item mask를 전부 교차해 `720 × (32+32+16+16) × 2 = 138,240` scenes를 관측한다.
- Mask bit 순서는 DOM 순서가 아닌 semantic ID 정렬(A, B, C, D, E 또는 A, B, C, D)이다. Mask 0은 기존 all-`0px`, all-bits-set mask는 기존 all-`auto` endpoint다. 기존 uniform Chrome oracle 11,520 scenes와 두 endpoint의 row·item·payload geometry를 비교한다.
- [독립 검증 결과](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-precomparison-2026-11-14/verification.json)는 720 track list와 네 DOM topology의 138,240 scenes, 69,120 source-order pairs, 69,120 alignment pairs를 확인했다. Eligible auto-minimum item 304,128회 중 payload content floor 위반은 0이고, 최대 span-frame/covered-row 오차는 `0.00045 CSS px`다. Source-order의 최대 geometry 차와 non-auto row의 stretch 차이는 각각 `0 CSS px`다. 기존 uniform endpoint 11,520 scenes는 모두 `0 CSS px` 차이다.
- Chrome에서 payload가 Grid content box 아래로 나간 장면은 전체 1,344개, 그중 mixed-mask 장면은 1,236개이며 최대 초과는 20px이다. 대표 장면 index 54,448은 track list `fit-content(20%) min-content auto fit-content(0px) fit-content(10px) max-content`, `align-content:stretch`, span-2 five-item topology, E만 `min-height:auto`다. D는 `min-height:0`인 채 payload가 10px border/content 끝을 넘어가므로 visible-overflow 경계를 확인하는 화면 사례로 사용한다. [Agent Browser 캡처](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-precomparison-2026-11-14/chrome157-three-fit-mixed-sample.png).

## 구현 범위와 사전 판정 기준

1. **계획 선행:** 제품 코드를 바꾸기 전에 이 계획을 서로 다른 실패 관점 20개로 검토한다. 계획 PR의 실제 diff는 별도 30개 관점으로 검토하고, 계획 PR이 병합된 뒤에만 구현한다.
2. **정확한 style admission:** 기존 three-fit row-span track-list predicate, 두 topology와 semantic item ID 집합에서만 각 span item의 computed `min-height:0px|auto` mask를 허용한다. Explicit-row control은 `0px`이어야 한다. Four-/five-/six-fit mixed 구현을 검증 없이 재사용하지 않는다.
3. **정확한 layout profile:** six implicit rows, cap의 authored track index·순서, fixed spans, direct-text payload, 하나의 Grid owner, writing context, supported item style와 document/style/environment revision을 검증한다. Missing/duplicate/stale trace, unsupported child, 다른 fit count, overflow 또는 placement profile은 fail-closed한다.
4. **RuntimeSession mixed sample:** 720 list 중 36개의 track list context를 고정해 각 context의 모든 192 scene을 실행한다(6,912 scenes). Sample rank `k=0..35`에 대해 position-set rank `k mod 20`, cap-permutation rank `floor(k/6)`, tail-order rank `k mod 6`을 선택한다. 따라서 20개 fit-position set 모두와 여섯 cap 순서·여섯 tail 순서를 전부 포함하고, 이 36 paired assignments의 모든 topology/mask/source-order/alignment를 실행한다. Sample builder와 독립 verifier는 exact list key, source hash, 각 mask·geometry와 실행 순서를 대조한다. 이는 138,240개 RuntimeSession 전체 실행을 뜻하지 않는다.
5. **Uniform endpoint와 이웃 profile 회귀:** 같은 candidate revision에서 기존 three-fit uniform row-span 11,520-scene RuntimeSession matrix를 다시 실행하고 mask 0/all-auto endpoint의 rows, item/payload/line geometry를 기존 결과와 `0.01 CSS px` 안에서 비교한다. Existing three-fit auto-placement, one-/two-fit row-span, four-/five-/six-fit mixed matrices, cap-order/rounding, explicit-row controls 및 unsupported mutation/recovery 테스트도 영향받은 범위에서 재실행한다.
6. **독립 geometry assertions:** 모든 used row는 finite/non-negative이고, span item frame은 실제 covered row 합과 `0.01 CSS px` 안에서 맞아야 한다. Eligible `auto` item은 payload content floor보다 작아지면 안 된다. Source-order reverse는 semantic ID별 geometry를 보존해야 한다. `align-content:stretch`는 auto maximum track 외 track을 바꾸면 안 된다. Payload의 Grid content box 밖 overflow는 별도로 집계해 oracle 및 screenshot과 대조하며, 예상 결과를 선험적으로 0으로 고정하지 않는다.
7. **Fail-closed와 atomic recovery:** 잘못된 fit count·cap·tail·order·percentage basis, invalid mask/nonzero fixed minimum, control auto, wrong span, extra owner/item, altered font/text/overflow/writing context와 unsupported placement를 거부한다. 실패 revision은 partial layout/render output을 게시하지 않고 같은 RuntimeSession의 다음 valid revision에서 복구한다.
8. **Rounding boundary:** Chrome과 RuntimeSession의 three-fit tie/remainder behavior를 fractional·half boundary에서 비교한다. Shared rounding policy를 넓히지 않으며, mismatch가 발견되면 재현 scene·oracle 근거·별도 regression을 먼저 만든다.
9. **Simulator 화면 검증:** Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 실제 Root HTML route에 mask-16 대표 scene과 red Grid border를 넣는다. `min-height:0`인 D의 text가 Grid 끝 10px 밖까지 paint되는지, E의 `auto`가 자신의 payload floor를 유지하는지 OS screenshots와 runtime logs로 확인한다. 이는 browser oracle이나 PixelCopy/Metal pixel parity를 대체하지 않는다. Android AVD가 없으면 task-owned disposable AVD를 만들고 사용 후 삭제한다. 앱·시뮬레이터·Agent Browser 세션은 실행 후 종료하고 상태를 재확인한다.
10. **구현 검토와 문서:** 구현 뒤 새로운 실패 관점 20개의 implementation/runtime review와 별도 PR diff 30개 검토를 수행하고 source revision·hash·PR head를 대조한다. PR에는 내부 계약 0059와 실행 근거를 연결한다. 병합 후 `spec/STATUS.md`, `spec/public/roadmap.html`, 이 계획, 내부 계약 설명과 Tailnet preview를 실제 병합 결과로 동기화한다. C11.4/C11 완료 체크, 공식 `78/214`, 숫자 계약 `0.1.0`은 이 slice만으로 올리지 않는다.

## 검증 명령 범주

- Chrome oracle: [capture script](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-precomparison-2026-11-14/capture-chrome.mjs) · `python3 .../verify-chrome-matrix.py`
- RuntimeSession mixed sample: 변경된 실제 테스트 이름을 확인한 뒤 `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test <three-fit-mixed-test-filter> -- --nocapture`로 실행한다. 0-test/no-match 출력은 근거로 세지 않는다.
- Uniform three-fit endpoints와 세 fit auto-placement, adjacent row-span/mixed profiles, rejection/recovery, `cargo fmt --all -- --check`, 변경 Rust crate Clippy 및 Android/iOS simulator fixture builds.

## 한계

이 계획과 Chrome 관측은 API 지원 완료가 아니다. General author CSS admission, flexible tracks, scrollable overflow, alternate writing modes, nested/anonymous/replaced payload, arbitrary span, auto-placement, whole WPT, production mobile screenshot parity, real-device/hardware GPU, compositor pixel parity, public API/FFI와 전체 C11.4/C11은 제외한다.
