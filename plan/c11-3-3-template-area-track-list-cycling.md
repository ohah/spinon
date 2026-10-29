# C11.3.3 · template-area implicit track-list cycling

**상위:** [C11 전체 계획](./c11-css-grid.md) · [C11.3.3 area implicit-track 계획](./c11-3-3-template-area-implicit-tracks.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**사전 비교:** [Chrome 156 관측과 CSS 규범에서 계산한 독립 기대값](../spec/internal/evidence/c11-3-3-template-area-track-list-cycle-precomparison-2026-10-30/README.md) · [고정 fixture](../tests/fixtures/css/c11/grid-template-areas-auto-track-list-cycling-c1133-precomparison.html)

**계획 실패 관점:** [독립 20개 검토](./c11-3-3-template-area-track-list-cycling-plan-review-20.md)

**계획 PR 변경 검토:** [별도 20개 관점](./c11-3-3-template-area-track-list-cycling-pr-diff-review-20.md)

**상태:** 계획과 Chrome 사전 비교만 완료했다. 이 계획 PR이 병합되기 전에는 구현하지 않는다. 구현되더라도 C11.3.3·C11 또는 공식 완료 수를 승격하지 않는다.

## 결손

기존 [C11.3.3 area 확장](./c11-3-3-template-area-implicit-tracks.md)은 area 행·열 수가 authored template track보다 많으면 부족한 track을 생성한다. 이 제한 profile은 `grid-auto-rows`·`grid-auto-columns`의 단일 `auto` 또는 px 값만 연결하고, fixed-`px` track-size 목록은 Style projection 단계의 영역 확장 함수에서 거부한다. `RuntimeBlockFlexGridTemplateV1`의 projection은 고정 px 목록을 typed `GridAutoRows::TrackList`/`GridAutoColumns::TrackList`로 이미 보존하지만, area expansion이 목록 순환을 하지 않고 layout validation도 `TemplateGrid`에서 목록을 거부한다.

CSS Grid 2025 CRD §7.6은 이 속성의 값을 `<track-size>+`로 정의하고, 여러 값이면 패턴을 반복한다. explicit grid 뒤 첫 implicit track은 목록 첫 값으로 시작한다. §7.3/§7.5에 따라 `grid-template-areas`가 authored explicit track보다 더 많은 축을 요구하면 해당 축에 implicit track이 필요하다. 따라서 한 개의 px 값을 같은 크기로 복제하는 현재 동작은 유지하면서, 두 개 이상의 fixed-`px` 값이 authored prefix 뒤에서 순서대로 순환하는 경로를 추가한다.

## 고정 사전 oracle

규범은 [CSS Grid Layout Module Level 1 · 2025-03-26 CRD §§7.3, 7.5, 7.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-tracks)다. [Chrome 156 Agent Browser 비교](../spec/internal/evidence/c11-3-3-template-area-track-list-cycle-precomparison-2026-10-30/README.md)는 used track list와 각 item의 root-relative frame을 독립 기대값과 비교했다. 모든 길이와 area 셀은 정수 CSS px라 tolerance는 0 CSS px다.

| Scene | Authored rows / columns | `grid-auto-rows` / `grid-auto-columns` | Expected used rows / columns |
| --- | --- | --- | --- |
| `both-axis-cycle` | `[20]` / `[30]` | `[5, 7]` / `[11, 13]` | `[20, 5, 7, 5]` / `[30, 11, 13, 11]` |
| `row-cycle-from-first-implicit` | `[17, 19]` / `[22, 24]` | `[6, 8]` / `[10, 12]` | `[17, 19, 6, 8, 6]` / `[22, 24]` |
| `column-cycle-from-first-implicit` | `[12, 14]` / `[21, 23, 25]` | `[4, 5]` / `[9, 10]` | `[12, 14]` / `[21, 23, 25, 9, 10]` |
| `row-cycle-continues-after-area` | `[20]` / `[30]` | `[6, 8]` / `[11, 13]` | `[20, 6, 8, 6]` / `[30]` |
| `column-cycle-continues-after-area` | `[30]` / `[20]` | `[11, 13]` / `[5, 7]` | `[30]` / `[20, 5, 7, 5]` |
| `authored-tracks-exceed-area` | `[15, 16, 17]` / `[18, 19, 20]` | `[5, 7]` / `[9, 11]` | Authored rows and columns remain unchanged |

The [independent expectation](../spec/internal/evidence/c11-3-3-template-area-track-list-cycle-precomparison-2026-10-30/expected.json) records every root-relative item frame as well as the used track strings. Chrome observation is stored separately. The continuation scenes place an item after an area has already consumed one value from a two-value pattern; the later track must use the second value, then wrap. This catches an adapter that materializes area tracks and restarts the pattern for subsequent explicit placement. The authored-longer scene prevents truncating trailing authored tracks or applying the auto list when no area-created track is needed.

## Scope and contract

1. Extend only `RuntimeBlockFlexGridTemplateV1` when `template_areas` is present. `grid-auto-rows` and `grid-auto-columns` may be `auto`, one non-negative finite px value, or a non-empty list of non-negative finite px track sizes. Keep existing scalar results unchanged.
2. When the area dimension exceeds the authored track count, append exactly the missing count. For a list, generated positive implicit tracks cycle from index zero independently on each axis. Preserve every authored track and line-name group. If the same Grid later creates more positive implicit tracks through explicit placement, preserve the CSS pattern phase after the materialized area tracks (the continuation scenes require the list to rotate by the number of area-created tracks modulo list length). The two-axis scene detects row/column list swaps.
3. Keep the C11.3.3 named implicit-line modes and all other placement/style profiles on their current supported grammar. Do not permit intrinsic keywords, percentages, `fr`, `minmax()`, `fit-content()`, `calc()`, auto-repeat, negative implicit extension, or generic author CSS admission through this change.
4. Fail closed for empty or invalid typed lists, unsupported list members, track caps, arithmetic overflow, and injected track/metadata reservation failures. On failure return no partially projected or laid-out scene; a subsequent valid request on the same worker must recover.
5. Treat auto track lists as invalid in `TemplateGrid` unless the same validated `template_areas` route is active and every list item is a fixed px track. The layout validation change must not weaken `TemplateGridImplicitTracks`, `TemplateGridSignedImplicitTracks`, C11.2.2/2.3, or C11.4 TrackSizing gates.
6. No public ABI, exported product fixture symbol, parser grammar, or style-profile identifier is added. Extend the existing opt-in C11.3.3 fixture route and strict matrix only. Existing 31 scene IDs, DOM topology, expected outcomes, and output checks remain unchanged; add the four scene IDs and update schema/hash/count consumers together.

## Implementation and verification gates

1. After this plan PR is Rebase and merge merged, create a feature branch in the existing Spinon checkout. Do not create a worktree or delegate parallel work.
2. Add Style projection tests for row-only, column-only and two-axis pattern cycling, scalar regressions, longer authored lists, allocation failure, invalid/empty list and unsupported list item. Assert the exact typed `GridContainerInput` and error property/node.
3. Add layout tests for all six independent scene geometries and `TemplateGrid` list admission. Negative controls must prove the same list is rejected without area metadata, in named implicit modes and in unrelated profiles. Test row and column pattern-phase continuation after area-generated tracks.
4. Extend the existing feature-gated C11.3.3 route. Android API 37 ARM64 AVD and iPhone 17 Pro/iOS 26.2 Simulator must each run the existing 31 scenes plus six new scenes (37 total), checking scene allowlist, used rows/columns, all node frames, layout/source/style revision, draw/present, and fixed PixelCopy/Metal drawable samples for the six new scenes. Keep simulator/software-renderer and pre-compositor limitations explicit.
5. Verify focused Style/layout/runtime tests, relevant fixture/FFI boundary tests, `cargo fmt --all -- --check`, affected Clippy targets, and Android/iOS Simulator fixture builds. Compare the default/product FFI archive against the opt-in fixture-test positive control; do not infer complete APK/IPA exclusion from a static archive.
6. Produce a new implementation/runtime failure review with 20 distinct cases and a separate implementation PR diff review with 20 distinct cases. The PR diff review is the PR's 20-case adversarial gate; do not add a duplicate third review. Reviews must inspect the actual final source/diff, not reuse this plan review.
7. Put both Simulator screenshots in the implementation PR when captures are available. Verify the PR page/images with Agent Browser, then close its session and verify no Agent Browser process remains. Merge only after checking live required-check configuration and PR merge state; use Rebase and merge.
8. After the implementation merge, sync `spec/STATUS.md`, internal contract 0059, the C11 plans, and public/Tailnet roadmap to the verified bounded result. Keep C11.3.3/C11 unchecked and official completion `72/213`. Do not deploy GitHub Pages.

## Completion boundary

This slice establishes forward cyclic, fixed-px implicit track lists for area-expanded positive tracks in one opt-in profile. It does not complete C11.3.3 or C11. It does not establish backward/negative implicit sizing, arbitrary track-list grammar, general CSS behavior, WPT conformance, physical-device GPU behavior, iOS post-compositor pixel equality, or every C11.4 track-sizing combination.
