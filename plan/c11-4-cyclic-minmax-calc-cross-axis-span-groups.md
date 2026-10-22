# C11.4 · cyclic `minmax(calc())` 교차 축 span groups

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [양축 span text 계획](./c11-4-cyclic-minmax-calc-two-axis-spanning-text.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** Chrome 156 사전 관측과 CSS Grid 2025 CRD에서 독립 계산한 네 고정 scene을 확정했다. 계획 실패 관점 검토 20개와 계획 PR 변경분 별도 검토 20개를 마쳤다. Runtime 구현은 시작하지 않았다. C11.4 상태와 공식 `77/213`은 유지한다.

## 목적

PR #506은 하나의 text item이 두 columns와 두 rows를 동시에 span하는 경우 columns-first sizing에 따른 wrapping과 row contribution을 확인했다. 기존 cyclic overlapping-span slice는 여러 item의 same-span planned-increase 경계를 다뤘지만, 그 item들이 양축을 span하면서 각각 다른 line count와 block contribution을 낼 때의 교차 축 동작은 결합하지 않았다.

이번 후속은 가운데 열에서 겹치는 두 span-2 text item의 Columns contribution을 CSS Grid §11.5.1 규칙으로 함께 계산하고, 그 결과 item별 wrapping이 각자 다른 Rows contribution으로 이어지는지 확인한다. reverse-order scene은 두 item의 배치를 그대로 두고 DOM 순서만 바꾼다. definite-width control에서는 item background가 root의 오른쪽 border box 바깥으로 그려지는 sample을 Android/iOS Simulator surface에서 직접 검사한다.

## 고정 입력과 독립 기대값

두 item은 동일한 `minmax(calc(10% + 2px), max-content)` track 3 columns × 2 rows Grid 안에 놓인다. gap은 양 축 2px, root border는 2px, item sizing은 `min-width:auto; min-height:auto`, text는 Ahem 10px/10px다.

| Item | Columns | Rows | Text | Min/max-content inline | 예상 inline wrapping |
| --- | --- | --- | --- | --- | --- |
| A | `1 / 3` | `1 / 3` | `AA A A` | 20 / 60px | intrinsic span width 33.34375px에서 `[20,30]` 두 줄, definite 40px root의 26px span width에서 `[20,10,10]` 세 줄 |
| B | `2 / 4` | `1 / 3` | `XXXX` | 40 / 40px | 고정 40×10px box, 40px 한 줄 |

| Case | Root width constraint | Columns | Rows | Root border box | Item frames, root-relative |
| --- | --- | --- | --- | --- | --- |
| intrinsic-forward | min-content | `[15.671875,15.671875,15.65625]` | `[9,9]` | `55×24px` | A `[2,2,33.34375,20]`, B `[19.671875,2,40,10]` |
| intrinsic-reverse | min-content | `[15.671875,15.671875,15.65625]` | `[9,9]` | `55×24px` | intrinsic-forward와 동일 |
| definite-forward | definite 40px content width | `[12,12,12]` | `[14,14]` | `44×34px` | A `[2,2,26,30]`, B `[16,2,40,10]` |
| definite-reverse | definite 40px content width | `[12,12,12]` | `[14,14]` | `44×34px` | definite-forward와 동일 |

규범 계산, Chrome 관측과 1/64 CSS px quantization tolerance는 [사전 비교 기록](../spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-span-groups-precomparison-2026-10-22/README.md)에 있다. Min-content constraint에서 먼저 계산되는 track bases `[9,19,19]`와 그 결과 51px container에서 cyclic percentage를 풀어 실제 배치에 쓰는 Columns `[15.671875,15.671875,15.65625]`는 서로 다른 sizing 단계다. 이 구분을 생략하거나 Chrome output을 독립 기대값의 근거로 사용하면 안 된다. `minmax(calc(10% + 2px), max-content)`의 cyclic minimum에서 2px 절대항을 잃거나, §11.5.1의 같은-track planned increase를 합산하거나, contribution gap을 두 번 더하면 판정 실패다.

## 구현 관문

1. 기존 C11.4 opt-in fixture-test profile에서 네 scene만 허용한다. 3×2 tracks, 두 item ID와 source order, grid placement, Ahem text/font revision, typed row/column track, gap, root dimensions 및 style profile을 exact 검사하고 다른 input은 부분 layout 없이 거부한다.
2. Rust trace는 Columns를 Rows보다 먼저 실행했음을 확인하고, 각 item의 `Minimum`, `MinContent`, `MaxContent` contribution, span=2, planned increase, intrinsic source/revision, final track arrays를 보존한다. 각 축의 contribution은 item ID를 구분한다.
3. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 scene 모두 실행한다. Intrinsic case는 min-content track bases `[9,19,19]`가 content width 51px로 이어진 뒤 최종 Columns가 percentage 재해석을 거쳐 `[15.671875,15.671875,15.65625]`가 되는 단계도 trace한다. 모든 root/item frame과 contribution을 대조하고 source revision, app/build hash, draw/present 결과를 기록한다. 실기기는 사용하지 않는다.
4. Definite-width 두 scene에서 Android PixelCopy와 iOS Metal drawable을 사용해 root-relative `(50,4)`의 `#e5484d` item B background와 `(60,4)`의 native surface clear를 검사한다. scale, viewport origin과 physical sample 좌표를 기록한다. `(50,4)`는 44px root border box 바깥, 40px item B 안이어야 한다. iOS 표본은 compositor 제출 전 결과다.
5. 기본 release FFI artifact에서 신규 scene ID·fixture marker·Ahem payload가 제외되고 opt-in fixture-test positive control에만 들어가는지 각각 확인한다. 변경한 Rust crate의 focused tests, formatter/lint, FFI tests와 Android/iOS Simulator app builds를 실행한다. 실기기·hardware GPU·WPT·iOS compositor 이후 pixels는 범위 밖이다.
6. 기존 양축 span, overlapping span, minmax(calc()) 및 multi-span ordering tests를 재실행한다. 새 구현/runtime 실패 관점 20개와 구현 PR diff 실패 관점 20개를 각각 독립적으로 기록한다. 발견한 결함은 영향 scene과 surface sample까지 다시 실행한다.
7. 내부 계약 0059의 existing opt-in fixture 경계를 따른다. 새 일반 CSS admission이나 public API를 추가하지 않는다. `spec/STATUS.md`, `plan/c11-4-grid-track-sizing.md`, conformance evidence와 공개 로드맵에는 제한 slice와 미검증 범위만 기록하고 C11.4 checkbox와 `77/213`을 올리지 않는다.

## 성공·오류 판정

- 네 scene에서 intrinsic min-content bases `[9,19,19]`, 최종 Columns `[15.671875,15.671875,15.65625]` 또는 definite `[12,12,12]`, Rows `[9,9]`/`[14,14]`와 root-relative frames가 기대값 허용치 안에 있어야 한다.
- A의 line count와 Rows contribution은 Columns가 결정한 item span width에 반응해야 한다. Intrinsic case는 `[20,30]` 두 줄·20px block contribution, definite 40px case는 `[20,10,10]` 세 줄·30px contribution이어야 한다. B는 두 case 모두 40×10px 한 줄이다.
- Reverse DOM order는 placement를 바꾸지 않는다. Track arrays, root size, 각 Node ID에 연결된 frame와 contribution이 forward case와 같아야 한다.
- A의 두 줄 contribution 20px block, B의 한 줄 contribution 10px block이 별도로 trace되어야 한다. 한 item의 contribution을 두 item에 재사용하거나 Columns 전에 Rows를 계산하면 실패다.
- definite case의 item B는 root right border를 12px 넘는다. PixelCopy와 drawable sample은 (50,4)에서 `#e5484d`를, (60,4)에서 `#000000`을 정확히 읽어야 한다.
- allowlist 외 tree/style/font/track input은 unsupported sizing으로 실패하고 새/이전 C11 profile 모두 partial frame을 내면 안 된다.

## 제외 범위

이 slice는 Ahem direct text 두 개, fixed overlap topology, 3×2 explicit grid, 하나의 cyclic calc expression, 두 width constraint로 한정한다. 임의 item count·span count, nested/anonymous/replaced items, row-axis cyclic percentage 조합 beyond these inputs, author CSS admission, full WPT, text glyph raster equality, physical devices, Android hardware GPU, iOS compositor-after pixels 또는 전체 C11.4/CSS Grid 완료를 증명하지 않는다.

## 검토와 근거

- [계획 실패 관점 20개](./c11-4-cyclic-minmax-calc-cross-axis-span-groups-plan-review-20.md)
- [CSS Grid CRD 규범 계산과 Chrome 156 사전 비교](../spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-span-groups-precomparison-2026-10-22/README.md)
- [계획 PR 변경분 별도 실패 관점 검토 20개](../spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-span-groups-precomparison-2026-10-22/plan-pr-diff-review-20.md)
