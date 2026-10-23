# C11.4 · 양 축 auto-repeat과 `grid-template-areas` 결합 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [단일 축 area 결합](./c11-4-auto-repeat-template-areas.md) · [양 축 auto-repeat](./c11-4-two-axis-auto-repeat.md) · [CSS Grid 내부 계약](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 계획 PR [#543](https://github.com/ohah/spinon/pull/543)이 병합됐다. 구현 전 소스 재추적에서 cascade gate를 추가 발견해 이 계획 수정에 반영하고, 계획 수정 관점 20개와 계획 수정 PR diff 30개를 새로 검토했다. Chrome 156 9-scene oracle을 유지했다. 구현과 Android/iOS Simulator 34-scene matrix가 통과했고 구현 PR [#545](https://github.com/ohah/spinon/pull/545)을 열었다. default/product FFI archive negative control은 simulator build 후 남은 저장 공간이 112 MiB라 다시 실행하지 않았다. 구현/runtime 실패 관점 20개와 PR 변경사항 실패 관점 30개를 별도로 기록했다. C11.4 checkbox와 공식 `77/213`은 유지한다. [구현 실행 근거](../spec/internal/evidence/c11-4-two-axis-template-areas-runtime-2026-10-23/README.md).

## 결손

PR #368은 `grid-template-columns`와 `grid-template-rows`에 auto-repeat 하나씩을 독립 적용한다. PR #539는 `RuntimeBlockFlexGridTrackSizingV1`에서 한 축 auto-repeat과 `grid-template-areas`를 함께 보존한다. 양 축 조합은 두 경계에서 각각 차단된다. [`incremental/grid.rs`](../crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs)는 template areas와 두 auto-repeat이 있으면 `ComputedCssGridLayout`을 만들기 전에 거부하고, [`grid_values.rs`](../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs)는 projected style도 거부한다. CSS Grid는 서로 다른 두 track list에서 auto-repeat 하나씩을 허용하고, area aliases를 행·열 양쪽에 만든다.

이 후속은 제한 profile 하나에서 이미 구현된 두 동작의 교차만 연결한다. 반복 수·track expansion·named line lookup·empty-track collapse를 Spinon에서 다시 계산하지 않고 pinned Taffy `0.15.0`에 전달한다. 두 축 각각의 auto-repeat과 area rectangles가 같은 `GridContainerStyle`까지 보존되는지 검증한다.

## 규범 및 사전 비교 기준

독립 기대값, Chrome 관찰 JSON, 입력 fixture, screenshot과 SHA-256은 [사전 비교 근거](../spec/internal/evidence/c11-4-two-axis-template-areas-precomparison-2026-10-23/README.md)에 고정했다. 기준은 CSS Grid Level 1 2025-03-26 CRD [§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat), [§7.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#named-areas), [§7.5–§7.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#implicit-grids), [§8.3–§8.4](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement)다.

Chrome for Testing `156.0.8078.48`, Agent Browser `0.31.1`, macOS arm64, viewport `1280×633`, DPR 1에서 fixture를 실행했다. CSSOM used track lengths와 root/item `getBoundingClientRect()`를 수집했다. 양 축 반복 단위는 fixed `20px`, gap은 `4px`다. definite content size 100/76/40px에서 반복 수는 각각 4/3/1이다. 비교값은 CSS Grid normative rule로 먼저 계산했고 Chrome은 교차 확인용 observation으로만 사용했다. 9개 모두 일치한다.

## 제한 구현 범위

- cascade와 style projection 모두에서 `RuntimeBlockFlexGridTrackSizingV1`만 columns와 rows의 track list에 각각 auto-repeat 하나와 `grid-template-areas`를 함께 허용한다. cascade의 `allow_auto_repeat`가 실제로 true인 profile인지 확인하고, `RuntimeBlockFlexGridTemplateV1` 및 이전 profile의 거부는 유지한다.
- 각 축은 `auto-fill` 또는 `auto-fit`을 독립 선택한다. repeat count, repeated line range, area-generated `hero-start`/`hero-end`, item placement, empty-track collapse는 기존 Taffy 0.15.0 경로를 따른다.
- 두 axis의 expanded repeat count보다 area dimension이 큰 경우도 허용한다. area가 추가한 explicit track은 이 고정 fixture에 한해 기존 fixed `grid-auto-columns:16px` 및 `grid-auto-rows:12px`를 사용한다. 이 구현에서 신규 `grid-auto-*` grammar를 열지 않는다.
- `grid-template-areas`의 유효한 직사각형 cell metadata를 layout-owned type으로 복사하고 checked dimension conversion 및 기존 byte/track 상한을 유지한다. Spinon-owned allocation 실패, 잘못된 area와 coordinate overflow는 전체 layout publication 전에 오류로 끝난다.
- 기존 no-area dual-axis auto-repeat 12 scene과 single-axis auto-repeat+areas 13 scene은 각각 고정 회귀로 유지한다. 신규 9 scene은 [fixture](../tests/fixtures/css/c11/grid-auto-repeat-template-areas-two-axis-c114.html)에서 읽으며 새 matrix와 simulator app route에 정확히 한 번씩 연결한다.
- root/item frame, both-axis used track list, auto-fit collapsed track, repeat count, grid extent, stage trace, draw/present를 ID별 비교한다. `area-added-both`는 40×40 root 밖으로 item을 12px씩 넘겨 PixelCopy와 Metal drawable에서 grid 안·밖 색 표본을 함께 확인한다.

### 고정 scene

모든 item frame은 grid content origin 기준 `[x,y,width,height]` CSS px다. used tracks와 정수 frame은 exact 비교한다.

| Scene ID | CSS 조합과 기대 결과 |
| --- | --- |
| `area-short` | 양 축 `auto-fill`; 100×76 root, area 2×2가 expanded 4×3보다 짧음. Columns `[20,20,20,20]`, Rows `[20,20,20]`, item `[24,0,8,8]`. |
| `area-added-columns` | 양 축 `auto-fill`; 40×76 root에서 repeat은 columns 1개를 만들고 area는 explicit columns를 총 3개로 늘린다. Columns `[20,16,16]`, Rows `[20,20,20]`, item `[44,0,8,8]`. |
| `area-added-rows` | 양 축 `auto-fill`; 76×40 root에서 repeat은 rows 1개를 만들고 area는 explicit rows를 총 3개로 늘린다. Columns `[20,20,20]`, Rows `[20,12,12]`, item `[24,40,8,8]`. |
| `area-added-both` | 양 축 `auto-fill`; 40×40 root에서 각 repeat은 track 1개를 만들고 area는 두 축을 각각 총 3개로 늘린다. Columns `[20,16,16]`, Rows `[20,12,12]`, item `[44,40,8,8]`. 오른쪽·아래 root overflow surface 색을 확인한다. |
| `fill-fit` | columns `auto-fill`, rows `auto-fit`; area item 한 개. Columns `[20,20,20,20]`, Rows `[0,20,0]`, item `[24,0,8,8]`. |
| `fit-fill` | columns `auto-fit`, rows `auto-fill`; area item 한 개. Columns `[0,20,0,0]`, Rows `[20,20,20]`, item `[0,24,8,8]`. |
| `fit-fit-empty` | 양 축 `auto-fit`; named area는 정의하지만 in-flow item은 없다. 모든 repeated track은 `[0,…]`으로 collapse한다. |
| `fit-fit-span` | 양 축 `auto-fit`; 한 item이 2×2 `hero` area를 채운다. Columns `[20,20,0,0]`, Rows `[20,20,0]`, item `[0,0,44,44]`. |
| `area-longhand` | 양 축 `auto-fill`; 두 축의 generated line alias를 `grid-column`/`grid-row` longhand로 배치한다. item `[0,24,8,8]`. |

`grid-template-areas` 행·열 수가 더 큰 경우 area가 expanded explicit grid를 늘리고, track list가 제공하지 않은 추가 explicit track은 각각 `grid-auto-*` sizing을 받는 것이 독립 계산 기준이다. `auto-fit`은 placement 뒤 빈 반복 track 및 인접 gutter를 접는다. area 선언 자체는 track을 occupied로 만들지 않고, item span과 교차하는 반복 track은 유지한다. 상세 관찰과 root scroll extent는 사전 비교 JSON을 기준으로 한다.

## 구현 및 검증 관문

1. 변경 전에 `incremental/grid.rs`의 cascade guard와 `grid_values.rs`의 projection guard를 각각 추적한다. `element.rs`의 `allow_auto_repeat`가 true인 유일 profile이 `RuntimeBlockFlexGridTrackSizingV1`인지 확인한다. 두 guard 뒤에 `ComputedCssGridLayout` → `GridContainerInput` → `CompactTaffyStyle` → vendored Taffy `NamedLineResolver`로 두 axis repeat metadata 및 area rectangles가 한 번씩 도달하는지 pinned source에서 추적한다.
2. cascade dual-axis-area reject는 `allow_auto_repeat`이 true인 경우에만 제거하고 projection reject도 `RuntimeBlockFlexGridTrackSizingV1`에서만 제거한다. `RuntimeBlockFlexGridTemplateV1` 및 이전 profile, 두 repeat이 한 track list에 있는 입력, malformed area, coordinate/metadata limit 초과, unsupported placement는 기존 구체 오류로 fail-closed한다. 오류 path가 computed snapshot·이전 layout·partial frames를 게시하지 않는 regression control을 둔다.
3. `spinon-style` cascade, style projection, layout focused tests에서 각 9개 geometry case와 profile gate를 검증한다. target profile은 두 `ComputedCssGridAutoRepeat`과 `ComputedCssGridTemplateAreas`를 동시에 보존해야 하고, non-target profile은 기존처럼 reject해야 한다. area rectangle/alias, repeat count, collapse/gutter, one-axis/no-area 회귀를 검사한다. dual-axis auto-repeat fixture 12 scene 및 area fixture 13 scene의 ID set도 별도 정확히 유지한다.
4. 기존 fixture boundary에서 scene IDs·CSS marker·FFI symbol을 갱신한다. default/product `spinon-ffi` static archive에서 fixture sentinel·payload·전용 symbol이 빠지고 `c04-runtime-gpu-fixture-test` positive control에서 들어오는지 각각 확인한다.
5. 관련 Rust tests, fixture validator, `mise exec -- cargo fmt --all -- --check`, workspace all-target Clippy를 실행한다. 실제 검사를 실행하지 않았으면 PR에 pass라고 적지 않는다.
6. Android API 37 ARM64 AVD `emulator-5558`과 iPhone 17 Pro / iOS 26.2 Simulator `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`에서 기존 12+13 scene과 신규 9 scene을 각각 실행한다. Android target은 `emulator-*` serial로 고정하고 USB physical device는 사용하지 않는다. 각 route의 exact ID set, full node frame, axes track list, trace, draw/present와 surface sample을 검사한다.
7. `area-added-both`의 표본은 captured grid origin에 대한 CSS point `[38,38]`의 `#e5e7eb`, root 오른쪽·아래로 넘친 item point `[47,43]`의 `#e5484d`, item 바깥 `[53,43]`/`[47,49]`의 host-surface clear를 Android PixelCopy와 iOS compositor 제출 전 Metal drawable에서 실제 readback한다. item은 오른쪽 12px·아래 8px 넘친다. 화면 좌표는 simulator scale에 맞게 변환한다. 해당 표본은 CSS border stroke, hardware GPU, 물리 기기 또는 iOS compositor-after 화면으로 확대 해석하지 않는다.
8. 신규 code/runtime은 계획 표를 재사용하지 않은 새 실패 관점 20개로 검토한다. 구현 PR diff는 별도 30관점, Tailnet preview sync PR이 필요하면 그 diff도 별도 30관점으로 검토한다. 계획 PR diff도 독립된 30관점으로 기록한다.
9. 구현 PR은 `.github/pull_request_template.md`에 상태 ID, 제한 명세, oracle, Android/iOS Simulator 화면, artifact hash와 남은 범위를 기록한다. 가능한 simulator screenshots는 PR 본문에 올리고 Agent Browser에서 이미지가 실제로 보이는지 확인한다. Browser 세션과 자식 process를 닫고 fresh process scan으로 종료를 확인한다.
10. 구현 병합 후 사용자가 요청한 Tailnet roadmap 동기화를 별도 PR로 진행한다. 선택 파일만 hash manifest와 함께 sync하고 HTTPS 응답, bytes/hash 및 Agent Browser 렌더링을 확인한다. 계획·구현·sync PR은 current required check과 merge state를 확인하고 Rebase and merge한다.

## 계획 검토

- [계획 자체의 독립 실패 관점 20개](./c11-4-two-axis-template-areas-plan-review-20.md)
- [cascade gate 소스 재추적 뒤 계획 수정의 독립 실패 관점 20개](./c11-4-two-axis-template-areas-cascade-plan-review-20.md)
- [cascade guard와 profile call site 소스 추적](../spec/internal/evidence/c11-4-two-axis-template-areas-precomparison-2026-10-23/cascade-source-audit.md)
- [계획 PR diff 실패 관점 30개](../spec/internal/evidence/c11-4-two-axis-template-areas-precomparison-2026-10-23/plan-pr-diff-review-30.md)
- [cascade gate 계획 수정 PR diff 실패 관점 30개](../spec/internal/evidence/c11-4-two-axis-template-areas-precomparison-2026-10-23/plan-revision-pr-diff-review-30.md)
- [Chrome 156 oracle와 독립 기대값](../spec/internal/evidence/c11-4-two-axis-template-areas-precomparison-2026-10-23/README.md)

## 제외 범위

한 track list의 복수 auto-repeat, integer repeat 혼합, repeated line names, authored/generated alias collision의 추가 조합, subgrid, non-fixed `grid-auto-*`, intrinsic/percentage/fraction track, implicit placement 확장, RTL·writing mode, arbitrary runtime CSS, 공개 CSS admission, WPT와 전체 C11.4 완료는 다루지 않는다. Android 물리 기기·hardware GPU, iOS 물리 기기와 compositor 제출 후 픽셀도 검증하지 않는다. 내부 계약 `0059` 숫자 버전 `0.1.0`, C11.4 checkbox와 공식 `77/213`은 그대로 유지한다.
