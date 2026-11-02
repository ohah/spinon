# C11.1 parsed Root HTML named areas × percentage tracks

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [parsed Root HTML percentage tracks](./c11-root-html-grid-percentage-tracks.md) · [parsed Root HTML named areas](./c11-root-html-grid-template-areas.md) · [named areas × fractional tracks](./c11-root-html-grid-fr-named-areas.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**현재 상태:** four-case Chrome 156 사전 비교와 독립 기대값을 고정했다. 현재 기본 Root HTML projection은 named area와 percentage track의 조합을 `grid-template-areas`에서 거부한다. 구현은 시작하지 않았다. 이 계획 및 구현 slice만으로 C11.1/C11 완료, 일반 CSS Grid 지원 또는 공식 `72/213` 집계를 바꾸지 않는다.

**사전 비교:** [Chrome observation과 독립 계산](../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/README.md) · [expected.json](../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/expected.json) · [SHA-256 manifest](../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/hashes.txt)

**계획 실패 관점 검토:** [20개](./c11-root-html-grid-area-percentage-plan-review-20.md) · **계획 PR diff 검토:** [20개](../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/plan-pr-diff-review-20.md)

## 현재 결손

`RuntimeRootHtmlTextPaintV1` 기본 parsed Root HTML route는 direct percentage track을 definite-axis 조건으로 처리하고, 고정 `grid-template-areas` map도 별도 제한 slice로 처리한다. [`validate_root_html_named_area_template()`](../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs)는 named-area route에서 고정 `40px 80px` columns + `20px 50px` rows 또는 기존 양축 `1fr 2fr` 조합만 받는다. 따라서 `25% 75%` named-area 입력은 `grid-template-areas` 진단으로 거부된다. 현재 동작은 focused projection test [`root_html_named_area_profile_still_rejects_percentage_tracks`](../crates/spinon-style-to-layout/src/projection/style_values/grid_values/root_html_grid_track_tests.rs) 1/1로 재확인했다.

## 독립 기준과 고정 입력

기준 명세는 [CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes) 및 [§7.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-template-areas)다. 사전 비교는 Agent Browser HeadlessChrome 156에서 직접 실행했으며, computed track 문자열·DOMRect와 screenshot을 독립 산술에 대조했다.

모든 scene은 definite `120×70px` content box, zero gap, area map `"header header" "side content"`, 세 semantic item을 사용한다. DOM child source order는 `content`, `header`, `side` 순서라 class/node order를 area 위치로 혼동하지 않는지도 본다.

| Scene | Explicit columns | Explicit rows | 기대 columns / rows (px) | 기대 item frame (px) |
|---|---|---|---|---|
| `both-axes` | `25% 75%` | `20% 80%` | `[30,90]` / `[14,56]` | header `[0,0,120,14]`; side `[0,14,30,56]`; content `[30,14,90,56]` |
| `mixed-px-percent` | `40px 50%` | `20% 50px` | `[40,60]` / `[14,50]` | header `[0,0,100,14]`; side `[0,14,40,50]`; content `[40,14,60,50]` |
| `overflowing-columns` | `120% 10%` | `20% 80%` | `[144,12]` / `[14,56]` | header `[0,0,156,14]`; side `[0,14,144,56]`; content `[144,14,12,56]` |
| `zero-track` | `0% 100%` | `20% 80%` | `[0,120]` / `[14,56]` | header `[0,0,120,14]`; side `[0,14,0,56]`; content `[0,14,120,56]` |

The independent values are recorded in [`expected.json`](../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/expected.json); Chrome results are separately recorded in [`chrome-observation.json`](../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/chrome-observation.json). The existing fixed-px named-area and both-axis `fr` pages are regression controls, not output from this new matrix.

## 구현 경계

- Extend only the default parsed `RuntimeSession::new_runtime_gpu_with_root_html()` route. Do not route through C11.3.3/C11.4 fixture constructors, add a fixture scene ID, or change product ABI.
- Preserve the current exact rectangular 2×2 area map and the three direct empty block items selected by computed `grid-area`. Keep zero gap, default sparse row flow, LTR/horizontal writing mode, visible overflow, zero edges, and definite `120×70px` root.
- Add a named-area combination that permits exactly two direct explicit tracks on each axis. In the new percentage path, tracks may be finite non-negative direct `px` and `%` values, mixed per axis. When the map uses no percentage, preserve the existing fixed-px and exact `1fr 2fr` paths. Do not admit `fr` and `%` in the same named-area input in this slice.
- Reuse the existing root percentage-axis checks: columns require a definite finite non-negative width basis, rows require a definite finite non-negative height basis; min-size stays `auto`, max-size `none`, aspect-ratio unset. Resolved track lengths and track aggregates must remain finite and within the existing f32 boundary.
- Keep fixed-px gap behavior unchanged and use `gap:0` in the new matrix so percentage basis, named-area placement, and gap distribution are not conflated. Keep `grid-auto-rows`/`grid-auto-columns:auto` and no implicit tracks.
- A negative authored percentage track is CSS-invalid and must not be treated as a computed zero or as evidence that negative values are supported. The parser/cascade and runtime admission must still fail closed when invalid declarations leave this route without the exact explicit tracks.
- Do not clamp finite percentages to 100%. The overflow case must retain the `[144,12]` columns and place the blue content item at x=144px outside the 120px root. Do not clip or clamp that frame in layout or screenshot processing.
- Keep invalid/unlisted values, indefinite/cyclic percentage axes, computed min/max/aspect-ratio overrides, `calc()`, `minmax()`, `fit-content()`, `repeat()`, named lines, unsupported area maps, missing/duplicate area items, child mutation and stale revisions fail-closed before publishing a partial scene.

## 구현 및 검증 관문

1. Add the four fixed source pages and expected geometry to the Root HTML focused runtime/projection matrix. Assert the exact computed area map, direct track values, definite basis, root/item identity and same document/style/layout revision. Compare tracks and frames at 0.02 CSS px tolerance.
2. Keep failure controls for indefinite width/height, percentage plus named-area `fr`, `calc()`/`minmax()`, percentage gap, min/max/aspect-ratio override, invalid area dimension/name/item topology, CSS-invalid negative percentage declarations, stale revision and track-range overflow. Every rejection must identify the property and leave no partial layout/frame/draw result.
3. Re-run the existing fixed-px named-area and named-area `1fr 2fr` regressions, Root HTML percentage-track matrix, Root HTML base/gap/implicit-row regressions, and C11.3.3 area matrix. Keep all other computed-style profiles unchanged.
4. Run the default Root HTML source on Android API 37 ARM64 AVD and iPhone 17 Pro/iOS 26.2 Simulator for the positive percentage scenes. Bind both runs to the same committed source hash; record computed tracks, root/item frames, revisions, layout readiness, draw/present, build hashes and screenshots. The overflow screenshot must visibly preserve item content outside the 120px root. Describe AVD as software-rendered and Simulator captures as visual evidence; do not claim PixelCopy/Metal pixel equality or physical-device/hardware-GPU results unless separately measured.
5. Run focused Rust/projection/runtime checks, `cargo fmt --all -- --check`, scoped Clippy, Android/iOS Simulator builds, and the applicable default/fixture archive boundary checks. Separate related baseline failures and unrun gates from candidate results.
6. Review implementation/runtime with 20 new failure perspectives. Review the actual implementation PR diff with 20 independent perspectives as requested by the user. Recheck affected perspectives after any corrections.
7. After implementation merge, update this plan, `plan/c11-css-grid.md`, `plan/c11-root-html-grid-runtime.md`, internal contract 0059, and `spec/STATUS.md` with exact scope, merge SHA, runtime evidence and limits. Keep C11.1/C11 and official `72/213` unchanged.

## 이 slice로 완료를 주장하지 않는 범위

This does not support arbitrary CSS Grid syntax, named-area auto-placement beyond the fixed three items, named areas combined with `fr`+`%`, intrinsic or indefinite percentage sizing, percentage gap, general implicit tracks, nested/text/image content, WPT runner/conformance, public CSS admission, full C11.1/C11.3.3/C11.4, Android physical devices/hardware GPU, or iOS post-compositor pixel equality.
