# C11.2.2 sparse row `order` 계획 적대 검토 20관점

검토 대상: [계획](./c11-2-2-sparse-row-order.md), [Chrome 사전 비교](../spec/internal/evidence/c11-2-2-grid-item-order-precomparison-2026-10-31/README.md), 현재 `origin/main` 코드. 구현은 시작하지 않았다.

| # | 독립 실패 관점 | 결과 |
| --- | --- | --- |
| 1 | 규범 근거가 Grid item의 `order`를 auto-placement에도 적용하는가? | PASS — CSS Grid 2025 CRD §6.3 및 §8.5를 링크하고 적용 범위를 행 sparse로 한정한다. |
| 2 | 음수와 양수 값의 정렬 방향이 명확한가? | PASS — signed computed integer 오름차순과 `A=2, B=-1, C=0`을 입력·예상 위치에 고정한다. |
| 3 | order 동률에서 안정성이 검증되는가? | PASS — `B`와 `C`의 `-1` 동률 및 DOM 순서가 유지되는 두 번째 장면을 둔다. |
| 4 | Source/DOM/accessibility 순서와 계산 순서를 혼동하는가? | PASS — HostDocument child list와 source/accessibility 순서를 보존한다고 계획에 명시한다. |
| 5 | 모든 visible item이 auto placement인지 아닌지 잘못 판정할 수 있는가? | PASS — 계획은 원본 `LayoutInput.grid_items`의 `None | is_auto()`만 허용하고, hidden item을 제외한 뒤 판정하도록 보완했다. normalized set을 쓰지 않아 partial-auto와 구분된다. |
| 6 | 부분-auto (`row` definite 또는 `column` definite) placement를 all-auto로 오인하는가? | PASS — complete `auto/auto` 계약과 mixed placement rejection을 적었다. 관점 5의 검증 방법도 보완한다. |
| 7 | Dense/column flow 또는 track-sizing profile에서 order를 실수로 허용하는가? | PASS — `SparseRow` 외 placement mode는 `UnsupportedGridOrder`로 유지한다. |
| 8 | `display:none` direct child의 order가 placement cursor를 소비하는가? | PASS — visible list에서 제거한 뒤 정렬해야 함을 계획과 hidden-child test에 적었다. |
| 9 | hidden child가 order sort 중 원본 visible 순서를 바꾸는가? | PASS — hidden filtering 후 stable sort를 적용하고 해당 negative/control을 실행한다. |
| 10 | definite placement 단계가 auto order 처리보다 앞서야 하는가? | PASS — 새 기능은 all-auto에 한정하고 혼합/definite placement를 거부해 기존 definite-first 계약을 유지한다. |
| 11 | CalcLayoutTree의 Taffy child vector도 order-modified 순서로 맞춰지는가? | PASS — `calc_tree.rs`의 Grid child stable sort를 계획해 placement와 Taffy child source order를 일치시킨다. |
| 12 | Taffy 내부 순서 변경이 NodeId/frame attribution을 바꾸는가? | PASS — node/frame은 NodeId로 검증하고 output order와 tree identity를 별도로 확인한다. |
| 13 | 동률 sorting이 unstable 알고리즘으로 바뀔 위험이 있는가? | PASS — Rust stable `sort_by_key` 사용과 equal-order scene을 acceptance에 요구한다. |
| 14 | anonymous text synthetic item이 다른 order를 상속하는가? | PASS — synthetic node의 기본 order 0과 기존 C11.4 text control 회귀를 명시한다. |
| 15 | CSS authored 문자열을 직접 해석해 Stylo computed 값을 우회하는가? | PASS — computed property와 typed `LayoutStyle.order`를 projection test에서 확인한다. |
| 16 | CSS `order` 정수 범위가 `i32` projection과 맞지 않을 때 성공 처리되는가? | PASS — `parse_order`의 `i32` 변환 실패를 `UnsupportedComputedValue`로 검증하는 style-to-layout negative test를 계획에 추가했다. |
| 17 | order의 paint/stacking 의미를 layout 위치 검증으로 과장하는가? | PASS — 겹치는 item, paint order, stacking을 제외하고 두 장면은 cell 내부 표본만 사용한다. |
| 18 | margin/box overflow가 non-overlap 가정을 깨뜨리는가? | PASS — 고정 10×10 leaf, 10px tracks, 0 margin/padding/border, transform 없음으로 scene 입력을 고정한다. |
| 19 | 기존 C11.2.2 matrix를 변경해 과거 evidence hash를 무효화하는가? | PASS — 별도 versioned order matrix를 사용하고 기존 16-scene matrix를 유지한다. |
| 20 | 앱 surface 관측을 Chrome/WPT/실기기 증거로 오인하는가? | PASS — Android API 37 AVD 및 iOS 26.2 Simulator만 실행하고 simulator·surface·compositor 경계를 분리한다. |

## 수정 내역

- 관점 5와 16은 계획 문구 및 검사 조건을 보완한 뒤 PASS로 닫았다. 계획 PR을 만들기 전 실제 `parse_order` 구현과 `GridItemInput::is_auto()` visibility를 다시 확인한다.
