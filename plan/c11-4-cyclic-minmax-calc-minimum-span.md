# C11.4 · cyclic percentage `minmax(calc())` minimum과 spanning item 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [직전 양축 minimum 계획](./c11-4-cyclic-minmax-calc-minimum-two-axis.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 계획 PR [#496](https://github.com/ohah/spinon/pull/496)이 병합된 뒤 네 고정 scene의 style-to-layout·runtime fixture를 구현했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 4/4 scene, 각 플랫폼 표면 표본 8/8 exact를 확인했다. 구현/runtime 실패 관점 20개 검토와 PR diff 30개 검토를 별도로 기록한다. 현재 C11.4 전체 완료 판정은 아니며 체크와 현재 공식 완료 수 `72/213`은 유지한다. [사전 비교 근거](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-span-precomparison-2026-10-22/README.md) · [구현·Simulator 근거](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-span-runtime-2026-10-22/README.md).

## 목적과 범위

PR #493은 cyclic `minmax(calc())` minimum을 양축에 적용했지만 한 explicit track과 direct text item만 사용했다. 그 계획은 다중 track과 span을 명시적으로 제외했다. 이번 제한 slice는 같은 minimum 함수를 두 explicit track에 놓고 한 in-flow item이 두 track을 span할 때 §11.5.1 contribution distribution, cyclic intrinsic basis, final percentage resolution이 순서대로 이어지는지 확인한다.

각 축에는 두 개의 동일한 `minmax(calc(50% + length), max-content)` track과 span-2 Ahem text item 하나만 둔다. columns에서는 `aa aa`, `column-gap:4px`, cyclic/definite `50px` inline basis를 사용한다. rows에서는 `a a`, definite `20px` inline size, `row-gap:4px`, cyclic/definite `20px` block basis를 사용한다. 두 축을 한 장면에서 동시에 cyclic하게 만들지 않는다. root에는 2px solid border를 두어 content-box percentage basis와 border box를 구분하고, minimum이 root보다 큰 경우 item이 border 밖으로 나가는 색 표본을 검사한다.

정확히 네 scene ID만 opt-in C11.4 fixture 경로에 허용한다. arbitrary span groups, 복수 item/source-order 상호작용, nested/anonymous/replaced item, 다른 track grammar, writing mode, 일반 CSS admission, WPT, 내부 계약 버전 변경은 범위 밖이다.

## 기준 입력과 독립 기대값

기준 HTML은 [고정 precomparison fixture](../tests/fixtures/css/c11/grid-cyclic-minmax-calc-minimum-span-c114-precomparison.html)다. 기대값은 [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.4](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing), [§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items) 및 Ahem 고정 입력에서 계산한다. Chrome 156은 독립 관찰자다.

| Scene | Intrinsic / definite content size | Intrinsic span distribution | Final used tracks | Root border box | Item, root-relative |
|---|---:|---|---|---:|---:|
| `cyclic-minmax-calc-min-span-columns` | `50×10px` | `aa aa` max-content 50px, gap 4px를 제외해 columns `[23,23]` | Columns `[29,29]`, Rows `[10]` | `54×14px` | `(2,2) 62×10px` |
| `definite-minmax-calc-min-span-columns` | `50×10px` | 같은 두 track·span control, width definite | Columns `[29,29]`, Rows `[10]` | `54×14px` | `(2,2) 62×10px` |
| `cyclic-minmax-calc-min-span-rows` | `20×20px` | `a a`가 폭 20px에서 두 줄, 4px row gap 뒤 rows `[8,8]` | Columns `[20]`, Rows `[12,12]` | `24×24px` | `(2,2) 20×28px` |
| `definite-minmax-calc-min-span-rows` | `20×20px` | 같은 두 row·span control, height definite | Columns `[20]`, Rows `[12,12]` | `24×24px` | `(2,2) 20×28px` |

Columns의 Ahem `aa aa` min-content/max-content inline contribution은 `20/50px`이다. 두 max-content track은 span의 max-content contribution에서 4px column gap을 한 번 뺀 46px을 대칭 분배해 intrinsic width 50px을 만든다. cyclic minimum은 intrinsic basis 계산 중 `auto`처럼 취급되며, width가 정해진 뒤 각 minimum은 `50% × 50 + 4 = 29px`이다. max-content limit 23px은 minimum보다 작으므로 minmax range가 minimum으로 올라간다. 두 final track과 4px gap을 합친 spanning item width는 62px이지만 content root width는 50px로 유지된다.

Rows의 Ahem `a a`는 definite 20px inline size에서 두 줄, 20px block max-content contribution을 만든다. span distribution은 4px row gap을 한 번 빼고 `[8,8]px` intrinsic rows를 만든다. cyclic block percentage는 max-content height 20px을 basis로 resolve해 각 minimum이 `50% × 20 + 2 = 12px`이 된다. final item height는 12+4+12=28px이며 content root height 20px를 넘는다. Definite controls는 같은 50px/20px basis를 직접 지정한다.

계산과 브라우저 관찰은 [precomparison README](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-span-precomparison-2026-10-22/README.md), [Chrome 156 JSON](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-span-precomparison-2026-10-22/chrome156-observation.json) 및 [SHA-256 manifest](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-span-precomparison-2026-10-22/SHA256SUMS.txt)에 분리 보존한다.

## 구현과 판정 관문

1. 계획 PR을 병합하기 전에 별도의 20개 실패 관점 검토를 유지한다. 계획 merge 전 runtime 구현은 시작하지 않는다.
2. 직접 layout regression에서 cyclic percentage minimum을 intrinsic contribution 단계에서 auto처럼 취급한 값과 final resolve 값을 분리한다. columns/rows의 span contribution, gap subtraction, min/max flooring, final track 및 root/item frame을 각각 검사한다.
3. 기존 `c04-runtime-gpu-fixture-test` C11.4 경로에 정확히 네 scene, 두 explicit track, 한 span-2 item, Ahem text와 지정 gap/border만 허용한다. 추가·누락 item, 변경된 ID/DOM/source order/style/track expression은 partial output 없이 거부한다.
4. Rust focused tests, fixture module boundary, `cargo fmt --all -- --check`, workspace all-target Clippy, Android/iOS fixture builds를 실행한다. 기본 product archive에서 새 fixture ID/marker/symbol/payload가 빠지고 opt-in archive에서 positive control로 들어오는지 비교한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 scene 각각의 trace, used tracks, node frame, draw/present를 확인한다. Android PixelCopy와 iOS compositor 제출 전 Metal drawable에서 root-relative `(58,6)` column sample은 border 오른쪽 바깥/item 내부 `#e5484d`, `(66,6)`은 item 밖 native surface clear `#000000`이다. Row sample `(8,27)`은 아래 border 바깥/item 내부 `#e5484d`, `(8,31)`은 item 밖 native surface clear `#000000`이다. Chrome precomparison의 white page backdrop은 별도 browser oracle이며 앱 표면 표본의 기대색이 아니다. 이 표본은 border stroke 픽셀 검증이 아니다.
6. 새 코드/runtime 동작을 계획 리뷰와 별도의 실패 관점 20개로 검토한다. 최종 PR diff는 30개의 독립 관점으로 확인한다. 발견을 수정하면 영향받은 regression·artifact·simulator 표본을 다시 확인한다.
7. 구현 PR 본문에 C11.4 미완료, 지원 경계, 실행 화면과 미검증 범위를 기록한다. PR screenshot은 Agent Browser에서 열어 로드 여부를 확인하고 사용한 session과 process를 닫는다. Live merge state/checks를 확인하고 Rebase and merge를 사용한다.

## 범위 밖과 한계

네 고정 scene은 span-2 item 하나의 cyclic min calc만 검증한다. span이 다른 여러 item의 순서, span-3 이상, `fit-content()`, flex track, nested/anonymous/replaced item, WPT, Android 실기기/hardware GPU, iOS compositor 이후 pixel 및 일반 CSS Grid 지원은 검증하지 않는다. C11.4 checkbox는 그대로 미완료이며 현재 공식 완료 수 `72/213`도 이 slice에서 바꾸지 않는다.

## 근거와 검토

- [Chrome 156 precomparison·독립 기대값](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-span-precomparison-2026-10-22/README.md)
- [계획 실패 관점 검토 20개](./c11-4-cyclic-minmax-calc-minimum-span-plan-review-20.md)
- [계획 PR 변경 검토 30개](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-span-precomparison-2026-10-22/plan-pr-diff-review-30.md)
- [구현/runtime 실패 관점 검토 20개](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-span-runtime-2026-10-22/implementation-runtime-review-20.md)
- [최종 구현 PR diff 검토 30개](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-span-runtime-2026-10-22/pr-diff-review-30.md)
- [Android/iOS Simulator 실행·화면·artifact 경계](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-span-runtime-2026-10-22/README.md)
- [CSS Grid 2025 CRD §7.2.1, §11.4, §11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)
