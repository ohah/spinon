# C11.4 · five-fit repeated-text row-span item별 `min-height:0|auto`

**상태:** Chrome 사전 비교 검증 완료 · 후보 구현 진행 중 · mixed RuntimeSession sample 3,984/3,984 통과 · five-fit uniform 34,560/34,560, six-fit uniform 11,520/11,520, six-fit mixed 대표 11,520/138,240 scenes 통과 · focused tests/Clippy/platform smoke 통과 · PR diff 30관점 검토 완료 · 원격 PR/CI/merge gate 대기 · C11.4/C11 미완료 · 공식 `78/214` 유지

**기준 구현:** [five-fit row-span `min-height:auto`](./c11-4-root-html-repeated-text-row-span-five-fit-auto-minimum.md) · PR [#1360](https://github.com/ohah/spinon/pull/1360)

**선행 mixed 구현:** [six-fit row-span item별 minimum mask](./c11-4-root-html-repeated-text-row-span-mixed-auto-minimum.md) · PR [#1369](https://github.com/ohah/spinon/pull/1369)

**사전 입력:** [five-fit uniform Chrome matrix](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-five-fit-auto-minimum-precomparison-2026-11-11/README.md) · [기본 fixture](../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-five-fit-auto-minimum-precomparison.html)

**혼합 mask 사전 결과:** [Chrome 157 관측과 독립 검증](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-five-fit-mixed-auto-minimum-precomparison-2026-11-12/README.md)

## 결손과 목표

기본 parsed Root HTML은 six-row repeated-text row-span의 five-fit profile에서 span item 전부가 `min-height:0`이거나 전부 `min-height:auto`인 두 상태를 지원한다. 현재 한 item만 값을 바꾼 mixed profile은 `RuntimeSession`이 atomic rejection한다. CSS Grid의 automatic minimum은 grid item마다 계산된다. 따라서 다섯 `fit-content()` track을 갖는 기존 profile에서도 각 span item의 0/auto 조합을 개별적으로 표현해야 한다.

이 slice는 PR #1360의 다섯 typed cap과 singleton tail, 두 row-span topology, 고정 direct-text payload를 유지한다. 다섯 span item topology에는 32개 mask, span-2/span-3 topology에는 16개 mask를 연결한다. 첫 explicit-row empty control은 모든 경우 `min-height:0`으로 고정한다. six-fit, auto-placement, 다른 fit 개수·owner·text grammar와 일반 CSS admission은 바꾸지 않는다.

## 규범과 비교 모델

규범 기준은 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)다. 이 fixture에서는 item의 computed overflow가 scrollable이 아니고, item이 `auto` minimum track을 span하며, span 내부에 flexible track이 없을 때만 `min-height:auto`의 content-based minimum이 적용된다. `fit-content()` 인자는 이 content-based minimum의 상한이 아니다. Mask bit별 eligibility를 계산하며 item끼리 minimum 값을 공유하지 않는다.

기준 구현은 Agent Browser 0.31.1이 실행하는 HeadlessChrome 157, macOS arm64, DPR 1이다. 기존 definite `40×220px` Grid, `40px` column, explicit `10px` row, zero gap, LTR sparse row flow, visible overflow, monospace 10px/line-height 10px direct-text 입력을 유지한다. 고정 item placement는 다섯 span-2 item 또는 span-2/span-3 혼합 네 item이고 각 family의 DOM source order를 forward/reverse로 바꾼다.

| 축 | 전체 고정 조합 |
| --- | --- |
| 여섯-member `grid-auto-rows` list | fit 위치 조합 6 × 다섯 typed cap 순열 120 × singleton tail 3 = 2,160 lists |
| minimum masks | span2Six A–E: 32 · span2Span3Six A–D: 16 |
| DOM order / alignment | family별 forward·reverse × `start`·`stretch` |
| 전체 관측 | 2,160 × (32×2 + 16×2) × 2 = 414,720 scenes |

Mask bit 순서는 semantic item ID 오름차순 A, B, C, D, E이며 span2Span3Six은 A–D만 쓴다. `minimumMask=0`은 기존 `min-height:0` baseline이고 all-bits-set mask는 기존 uniform `min-height:auto` candidate다. 이 두 endpoint가 기존 34,560-scene 관측과 일치해야 한다. 중간 mask는 브라우저에서 새로 관측하고 독립 검증한다. 모든 scene의 grid/item/payload frame, used row, computed track serialization, line count, overflow, mask와 owner metadata를 보존한다.

### Chrome 157 사전 관측 결과

414,720개 장면과 모든 mask·track list가 수집됐고 독립 verifier가 전체 scene을 확인했다. 기존 uniform five-fit 관측과 endpoint 34,560개를 재비교해 최대 geometry 차이 `0 CSS px`를 확인했다. Automatic-minimum eligible item의 payload content-floor 위반은 0건이었다. `min-height:0`에서는 payload-floor 미달 132,480건과 Grid block-end 초과 1,440건을 관측했고, `min-height:auto`에서는 Grid block-end 초과가 없었다. 이 차이는 후보 코드의 보장이 아니라 기준 브라우저 관측이며 RuntimeSession에서 재검증한다.

Source-order 반전과 non-auto alignment의 최대 geometry 차이는 각각 `0 CSS px`; span frame과 covered rows 합의 최대 오차는 `0.00045 CSS px`였다. 따라서 현재 범위와 판정 기준으로 제한 구현을 시작할 수 있다. 이 결과는 Chrome oracle일 뿐 Spinon 구현 결과나 일반 CSS 지원을 뜻하지 않는다.

## 구현 전 독립 판정 기준

- Observation 수는 정확히 414,720이며 track list 2,160개, topologies 4개, alignment 두 값, 32/16 masks 전부를 빠짐·중복 없이 포함한다.
- 각 mask에서 item별 computed `min-height`가 mask bit와 일치하고 empty explicit-row control은 0이다. 모든 candidate item은 기존 visible, non-scrollable overflow다.
- Automatic-minimum eligibility는 해당 item의 실제 spanned tracks에서 `auto` minimum 또는 `fit-content()`의 `auto` minimum을 찾아 계산한다. span track 중 flexible track이 있으면 eligible로 세지 않는다.
- `min-height:auto` eligible item의 content floor와 visible payload clipping/overflow를 검사한다. Baseline과 candidate의 기대 차이를 사전에 0으로 가정하지 않는다.
- Source order reversal은 semantic item ID별 row/frame/payload frame을 보존해야 한다. `stretch`는 singleton `auto` tail에서만 남은 공간을 늘릴 수 있으며 `min-content`/`max-content` tail을 늘리지 않는다.
- 기존 five-fit uniform matrix의 mask 0/all-auto endpoint, #1369 six-fit mixed matrix, three-/four-fit row-span과 five-fit auto-placement controls를 회귀로 둔다.
- Geometry 비교 오차는 최대 `0.01 CSS px`다. Chrome 관측은 독립 기준 구현일 뿐 제품 결과·시뮬레이터 결과를 대신하지 않는다.

## 제한 admission과 구현 검증 관문

1. 이 계획을 별개의 실패 경로 20개로 구현 전 검토하고 그 결과를 반영한다.
2. Style projection과 Layout admission은 exact existing five-fit row-span topology/list에서만 span item의 mixed `min-height:0|auto`를 추가 허용한다. 첫 empty explicit-row control은 0으로 고정한다.
3. Five typed caps `[0px, 10px, 20%, 30px, 40px]`는 각각 한 번, tail은 `min-content|max-content|auto` 중 하나여야 한다. List는 정확히 여섯 member며 cap authored position/type을 보존한다.
4. Uniform all-zero/all-auto input과 32/16 mixed masks를 모두 허용한다. Control auto, extra item, unsupported placement, changed text/font/edge/overflow, indefinite percentage, wrong cap/tail/list, another Grid owner와 non-five-fit row-span은 fail-closed rejection한다.
5. Chrome 기준 observation은 414,720 scenes 전체를 유지한다. RuntimeSession은 독립 검증된 균형 sample 3,984 scenes와 기존 uniform endpoint 34,560 scenes를 실행한다. Sample은 120개 typed-cap 순서를 모두 포함하고, 18개 fit-position/tail 대표 문맥에서는 두 topology family의 모든 중간 mask를 교차하며, 나머지 102개 cap 순서는 topology별 회전 mask를 사용한다. 기존 endpoint matrix는 2,160개 track list 전체에서 mask 0과 all-auto를 확인한다. Sample subset 자체를 원본 oracle과 다시 대조한 뒤 frame, row, text line, source order, alignment, automatic minimum, content floor, grid-end overflow 및 revision owner를 비교한다. 전체 Chrome matrix와 RuntimeSession sample을 같은 크기의 결과로 표현하지 않는다. Negative fixture rejection은 partial snapshot을 내지 않으며 같은 세션의 다음 유효 revision은 recovery해야 한다.
6. Track trace rounding이 mixed five-fit에서 baseline/all-auto endpoint와 맞는지 검사한다. 필요하면 rounding 정책은 새 독립 oracle와 edge tests 없이 넓히지 않는다.
7. Layout, Style projection, RuntimeSession focused tests와 existing one-/two-/three-/four-/five-/six-fit span·placement·rounding·rejection 회귀를 실행한다. 변경 Rust는 `cargo fmt --all -- --check`와 영향 crate Clippy를 확인하고 기존 baseline failure는 별도로 남긴다.
8. Android API 37 ARM64 AVD와 iOS 26.2 Simulator에서 기본 Root HTML route를 light/dark로 빌드해 layout-ready/draw-present smoke를 확인한다. 이 route 화면은 414,720 feature matrix나 표면 pixel parity의 증거가 아니다. 사용한 task-owned emulator/simulator/Agent Browser session은 끝난 즉시 닫고 shared state는 보존한다.
9. 구현/runtime 동작을 새로운 실패 관점 20개로 검토하고 PR diff는 필수인 30개 독립 관점으로 따로 검토한다. Head SHA와 실제 PR diff를 대조하며 미통과 리뷰를 고친다.
10. Merge 후 plan, 내부 계약 0059, `spec/STATUS.md`, public roadmap과 Tailnet preview를 실제 결과에 맞춘다. C11.4/C11과 공식 `78/214`, 내부 numeric contract `0.1.0`은 변경하지 않는다.

## 제외

일반 CSS parser/admission, public API·FFI, auto-placement, six-fit 및 다른 fit-count row-span, arbitrary cap/order/topology, replaced/anonymous/nested items, scroll/clipping, WPT 전체, 실기기, hardware GPU, surface/compositor pixel parity, C11.4/C11 완료는 범위 밖이다.

## RuntimeSession sample 조정 기록

Chrome의 전체 oracle은 414,720 scenes로 보존한다. 첫 전체 RuntimeSession matrix 실행은 3,840 scenes 처리 후 약 5분 30초에 수동 중단했고, 그 실행은 완료된 검증이 아니었다. 관측 처리율 기준 전체 RuntimeSession matrix는 약 10시간으로 추정되어 반복 개발 검증에 부적합했다. 이에 RuntimeSession 혼합-mask 비교를 3,984개 균형 sample로 제한하고, 전체 Chrome oracle과 기존 34,560개 uniform endpoint RuntimeSession 회귀를 별도로 유지한다. sample 생성기와 verifier는 원본 oracle hash, 선택 규칙, 120개 cap 순서, 18개 완전 대표 문맥, 102개 회전 문맥을 고정 검증한다.

이후 sample 3,984 scenes를 40개 track-list씩 세 청크로 나눠 실행했고 세 청크가 모두 통과했다. 각 scene의 used row·item/payload frame·eligible automatic minimum·Grid-end overflow와 source-order/alignment pair를 검증했다. 집계는 `[min-height:0, min-height:auto]` 순서이며 floor shortfall 예상/실제 `[1388, 0]`, Grid-end overflow 예상/실제 `[4, 0]`으로 같았다. 즉 auto item에는 shortfall이나 Grid-end overflow가 없고 zero item의 비영 집계는 Chrome baseline 관측과 일치한다. 공유 rounding 변경 뒤 기존 five-fit uniform endpoint matrix 34,560/34,560과 six-fit uniform row-span matrix 11,520/11,520 scenes도 통과했고 두 matrix에서 candidate payload-floor 위반은 0이었다. #1369 six-fit mixed-mask matrix는 chunk 1/12의 11,520 scenes를 재실행해 rows, item/payload frames, source-order와 alignment pair가 일치했다. 전체 138,240-scene matrix 중 나머지 126,720 scenes는 재실행하지 않았다. Android/iOS 기본 Root HTML smoke는 light/dark에서 layout-ready 및 draw-presented를 확인했으나 feature matrix가 아니다. 각 범위와 실행 로그는 [RuntimeSession 근거](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-five-fit-mixed-auto-minimum-precomparison-2026-11-12/README.md)에 기록했다. 구현/runtime 20개와 PR diff 30개 검토는 완료했으며, 실제 원격 PR diff·CI·merge gate는 아직 남아 있어 이를 slice 완료로 표시하지 않는다.
