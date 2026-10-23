# C11.4 · `auto-fit`과 area 확장 track 결합 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [양 축 auto-repeat과 template areas](./c11-4-two-axis-template-areas.md) · [CSS Grid 내부 계약](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 계획 PR [#548](https://github.com/ohah/spinon/pull/548)이 병합됐고 구현 PR [#549](https://github.com/ohah/spinon/pull/549)이 열렸다. 7개 신규 scene을 포함한 구현 후보는 최신 source에서 layout·projection·opt-in FFI 검사를 통과했고, fixture-boundary Node 검사는 7/7이다. 전체 336-scene route에서 정해진 41개 ID를 선택해 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 41/41을 실행했다. 두 플랫폼의 node frame·used tracks·draw/present는 41/41 exact match이며, 29 area scene의 PixelCopy/Metal 표본은 각각 116/116 exact다. 첫 Android 실행 중 empty-area scene의 item-rect host map 누락을 찾아 Android/iOS에 빈 배열을 추가하고 전 scene host mapping 회귀검사를 넣은 뒤 전체 실행을 다시 통과했다. workspace와 fixture feature Clippy, static archive의 기본/fixture negative·positive control도 통과했다. 현재 `mergeStateStatus=CLEAN`이며 `gh pr checks --required`에는 check가 보고되지 않았다. Agent Browser에서 본문 Android/iOS 이미지가 각각 1080×2400, 1206×2622로 렌더링됨을 확인했다. C11.4 checkbox와 공식 `77/213` 집계는 병합 전까지 그대로 둔다. [검증·한계·화면](../spec/internal/evidence/c11-4-auto-fit-area-expansion-runtime-2026-10-23/README.md).

## 결손과 기존 경계

구현된 제한 profile [`RuntimeBlockFlexGridTrackSizingV1`](../crates/spinon-style/src/stylo_dom/cascade/snapshot.rs)은 columns와 rows 각각에 auto-repeat을 하나씩 둘 수 있고 `grid-template-areas` metadata를 함께 projection한다. [`grid_values.rs`](../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs)의 차원 검사는 auto-repeat 축에서 area dimension 확장을 허용한다. [`grid.rs`](../crates/spinon-layout/src/grid.rs)의 validation은 metadata 상한을 유지한다. Vendored Taffy `0.15.0`은 repeat 개수 계산 뒤 area dimension과 template dimension의 큰 값을 explicit grid 크기로 쓰고, 비어 있는 auto-fit repeat 및 양쪽 gutter를 접은 뒤 area가 추가한 track을 `grid-auto-columns`/`grid-auto-rows`로 만든다.

현재 고정 runtime fixture에는 다음 교차가 빠져 있다.

- [양 축 area fixture](../tests/fixtures/css/c11/grid-auto-repeat-template-areas-two-axis-c114.html)의 `area-added-*` 세 장면은 `auto-fill`이며, `auto-fit` 장면의 areas는 반복 track보다 크지 않다.
- [single-axis area fixture](../tests/fixtures/css/c11/grid-auto-repeat-template-areas-c114.html)의 `*-area-larger`도 `auto-fill`이다.
- 직접 metadata validation은 area extension과 `auto-fit` metadata를 각각 허용하지만, area가 추가한 explicit track을 포함한 layout 결과·빈 track collapse·가터·frame을 함께 고정하지 않는다.

이 작업은 제한된 기존 profile에서 누락된 교차 동작을 고정한다. 실행 결과가 pinned Taffy 경로에서 규범과 일치하면 제품 계산 코드를 바꾸지 않고 최소 regression 및 opt-in runtime fixture만 추가한다. 차이가 재현되면 Taffy adapter 또는 전달 metadata의 최소 경계를 수정해, repeat 수와 track collapse의 새 구현을 Spinon에서 중복하지 않는다.

## 규범 및 사전 비교

기준은 CSS Grid Layout Module Level 1 2025-03-26 CRD [§7.1 explicit grid](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#explicit-grids), [§7.2.3.2 auto-repeat](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat), [§7.5 implicit grid](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#implicit-grids)다.

고정 입력, 독립 기대, Chrome observation, screenshot 및 SHA-256은 [사전 비교 근거](../spec/internal/evidence/c11-4-auto-fit-area-expansion-precomparison-2026-10-23/README.md)에 있다. Chrome for Testing `156.0.8078.48`, Agent Browser `0.31.1`, macOS arm64, viewport `1280×633`, DPR 1을 사용했다. definite `52px` 축에 `20px` 반복 track과 `4px` gap을 적용해 auto-repeat 2개를 만든다. area는 각 축에서 3 track까지 확장하고 추가 track은 고정 `grid-auto-columns:16px`, `grid-auto-rows:12px`로 크기를 정한다. Chrome CSSOM used tracks·item frames·scroll extent 7/7이 독립 규범 계산과 일치한다.

| Scene | CSS 조합 | 기대 columns | 기대 rows | 기대 item `[x,y,w,h]` | 검증 포인트 |
| --- | --- | --- | --- | --- | --- |
| `fit-fit-empty` | 두 축 auto-fit, 3×3 area, item 없음 | `[0,0,16]` | `[0,0,12]` | 없음 | area 정의 자체가 반복 track을 점유하지 않음 |
| `fit-fit-item` | 두 축 auto-fit, item은 3행 3열 추가 area cell | `[0,0,16]` | `[0,0,12]` | `[0,0,16,12]` | 반복 track collapse 뒤 추가 cell frame 이동 |
| `fit-fit-span` | 두 축 auto-fit, item이 3×3 area 전체 span | `[20,20,16]` | `[20,20,12]` | `[0,0,64,60]` | item과 교차한 반복 유지, root 밖 12px/8px overflow |
| `fit-fill-item` | columns auto-fit, rows auto-fill, 추가 cell만 점유 | `[0,0,16]` | `[20,20,12]` | `[0,48,16,12]` | column 반복만 collapse, bottom 8px overflow |
| `fill-fit-item` | columns auto-fill, rows auto-fit, 추가 cell만 점유 | `[20,20,16]` | `[0,0,12]` | `[48,0,16,12]` | row 반복만 collapse, right 12px overflow |
| `fit-fit-column` | 두 축 auto-fit, areas가 세 번째 column만 추가 | `[0,0,16]` | `[20,0]` | `[0,0,16,20]` | 추가 column 유지, 빈 row repeat collapse |
| `fit-fit-row` | 두 축 auto-fit, areas가 세 번째 row만 추가 | `[20,0]` | `[0,0,12]` | `[0,0,20,12]` | 추가 row 유지, 빈 column repeat collapse |

모든 frame은 grid content origin 기준 CSS px다. 반복 track과 gutter collapse는 exact zero로 비교한다. 고정 정수 geometry에는 tolerance를 두지 않는다. Root는 고정 `52×52px`이며 scroll extent로 overflow를 분리 기록한다.

## 제한 구현 범위

- 기존 `RuntimeBlockFlexGridTrackSizingV1`만 dual-axis auto-repeat과 template areas 결합을 받는다. `RuntimeBlockFlexGridTemplateV1`, 이전 profile과 일반 CSS admission은 그대로 fail-closed한다.
- 두 축의 auto-repeat, area rectangles, fixed `grid-auto-*`를 기존 `GridContainerInput`에서 Taffy `0.15.0`까지 전달한다. repeat count, named-line/area placement, empty-track collapse를 Spinon에서 별도로 계산하지 않는다.
- 독립 layout test는 7 scene의 area dimension 확장, used tracks, item frames, collapsed count와 stage trace를 확인한다. cascade/projection 경계 test는 두 auto-fit mode와 3×3 area metadata를 보존하고 non-target profile 거부를 유지하는지 검사한다.
- opt-in runtime scene은 기존 [template-area auto-repeat scene JSON](../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/template_area_auto_repeat_scenes.json)에 정확히 7개 추가한다. 해당 FFI route의 fixed scene ID validator, Android/iOS host route와 exact expected ID set을 같이 갱신한다. Android PixelCopy와 iOS Metal drawable의 scene 표본을 고정한다.
- 전체 고정 matrix는 no-area dual-axis auto-repeat 12 + single-axis area 13 + dual-axis area 9 + 신규 7, 총 41 scene이다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 41/41을 실행한다. 동작 확인은 물리 Android 연결 없이 simulator/AVD만 쓴다.
- `fit-fit-span`, `fit-fill-item`, `fill-fit-item`의 grid 안·root 밖 item·clear 표본을 PixelCopy와 Metal drawable에서 비교한다. 이는 Android software/host GPU 경로와 iOS compositor 제출 전 drawable 자료이며 hardware GPU, 실제 OS compositor 이후 표면 또는 border stroke를 주장하지 않는다.
- default/product 및 fixture-test `spinon-ffi` static archive를 비교해 새 scene marker·fixture host symbol의 absence/positive control을 확인한다. static archive 결과를 APK/IPA 전체 제외로 확대 해석하지 않는다.
- 계획 표를 재사용하지 않은 새 runtime 실패 관점 20개와 PR 변경 diff 실패 관점 20개를 각각 작성한다. 사용자가 요청한 PR별 20회 검토 기준을 적용한다.

## 구현 및 병합 관문

1. 각 profile의 cascade `allow_auto_repeat`, area metadata copying, projection validation, `GridContainerInput`의 두 repeat·area 전달 및 vendored Taffy explicit-grid/collapse 경로를 다시 대조한다. 구현 코드 변경 여부는 실제 mismatch 증거로 결정한다.
2. 사전 oracle의 seven IDs와 expected table을 Rust layout regression 및 runtime scene JSON에 일치시킨다. Added-area-only, empty-area, only-one-axis-fit controls와 full-span occupancy를 모두 실행한다. 이전 no-area 12 및 area 22 scene의 ID와 결과도 유지한다.
3. focused crate tests, fixture ID validator, `mise exec -- cargo fmt --all -- --check`, `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings`를 실행한다. 실패하거나 자원 때문에 미실행인 검사는 근거와 제한을 기록한다.
4. Android target은 API 37 ARM64 AVD `emulator-5558` (`spinon_api37_0_applied_txn`), iOS target은 iPhone 17 Pro / iOS 26.2 Simulator `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`로 고정한다. USB physical device는 사용하지 않는다. 이미 실행 중인 기기·세션·build/cache 프로세스는 먼저 owner와 route를 확인하고 중단하거나 reset하지 않는다.
5. 각 플랫폼의 41-scene exact ID set, root/item frame, axis별 used track/collapse count, stage trace 및 draw/present를 비교한다. span/mixed overflow 표본은 screenshot, Android PixelCopy와 iOS drawable readback에서 확인한다.
6. source revision, simulator/app/build metadata, 원본 로그·JSON·screenshots·artifact hashes와 archive 경계 검사를 runtime evidence에 기록한다. 운영체제 compositor가 만든 최종 화면 검증으로 부풀리지 않는다.
7. 구현/runtime failure review 20개와 PR diff review 20개를 독립적으로 마친 뒤, 현재 required checks·merge state를 조회하고 `Rebase and merge`한다. PR 본문에 runtime 화면이 있으면 screenshot을 붙이고 Agent Browser로 이미지 표시를 확인한다.
8. PR review 후 본인 Agent Browser session과 child process만 종료하고 fresh session/process scan을 기록한다. 다른 Browser, tmux pane, emulator, Simulator, Tailnet server와 shared cache는 종료하지 않는다.
9. 구현이 병합된 뒤 변경된 Tailnet roadmap을 hash manifest와 backup으로 동기화한다. HTTPS response·hash·Agent Browser 렌더링을 확인하고 sync PR도 별도의 20관점 diff review 및 Rebase and merge를 거친다.

## 계획 검토

- [계획 자체의 독립 실패 관점 20개](./c11-4-auto-fit-area-expansion-plan-review-20.md)
- [Chrome 156 사전 비교·규범 기대·screen·hash](../spec/internal/evidence/c11-4-auto-fit-area-expansion-precomparison-2026-10-23/README.md)
- [계획 PR diff의 독립 실패 관점 20개](../spec/internal/evidence/c11-4-auto-fit-area-expansion-precomparison-2026-10-23/plan-pr-diff-review-20.md)

## 제외 범위

새 CSS grammar, arbitrary/non-px `grid-auto-*`, intrinsic·percentage·fraction sizing, 여러 auto-repeat이 한 track list에 있는 경우, authored line-name collision 확장, auto-placement 확장, RTL·writing mode·subgrid, public CSS support, WPT, 전체 C11.4 완료는 다루지 않는다. Android 실기기, Android hardware GPU, iOS 실기기, iOS compositor 이후 pixels와 최종 APK/IPA fixture exclusion도 주장하지 않는다. 내부 계약 `0059`의 숫자 버전 `0.1.0`, C11.4 checkbox와 공식 `77/213`은 유지한다.
