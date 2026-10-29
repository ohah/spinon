# C11.3.3 template-area track-list cycling implementation/runtime review · 20 cases

Reviewed the final projection and layout implementation, fixed Chrome oracle, opt-in runtime route, simulator logs, and default/fixture static archives. This review is separate from the plan review and plan PR review.

| # | Failure path | Check and result |
| ---: | --- | --- |
| 1 | Area-created rows accidentally use the column list | The distinct two-axis scene and projection assertion preserve row values `[5,7,5]` and column values `[11,13,11]`; pass. |
| 2 | Area-created columns accidentally use the row list | Column-only scene has unique `[9,10]` values; its frame and pixel samples match Chrome on both simulators; pass. |
| 3 | First generated track starts at list index one | Row-only and column-only first-implicit scenes start at index zero; focused projection and runtime comparisons pass. |
| 4 | Cycling fails when generated count exceeds pattern length | `both-axis-cycle` requires wrap to the first value; 7 Android and 7 iOS samples match exactly. |
| 5 | Authored tracks are overwritten by the generated pattern | Projection vectors retain each authored prefix before appended tracks; pass. |
| 6 | An authored grid longer than its area is truncated or expanded | `authored-tracks-exceed-area` retains both authored axes and adds no implicit track; runtime samples pass. |
| 7 | Rows and columns share one pattern phase | Projection tests independently rotate the row and column lists; pass. |
| 8 | Later named implicit placement restarts the cycle after area expansion | The row and column continuation scenes use the existing named-line path and require the second value; layout frames and both simulator captures pass. |
| 9 | Numeric implicit-line placement is accidentally admitted | Continuation fixtures use `area-name-end / span 1 ghost`; numeric placement remains outside the scene and implementation contract. |
| 10 | Scalar `auto` or px extension changes behavior | Existing scenes remain in the 37-scene matrix; total outcome counts are 31 success, 5 cascade fallback, and 1 expected rejection. |
| 11 | Empty typed track list causes modulo-zero or silent fallback | Empty row/column lists are rejected by projection and layout validation; focused tests pass. |
| 12 | Percentage, intrinsic, flex, calc, or other non-px list member enters the bounded profile | The profile predicate admits only non-empty fixed-px lists; invalid member and error-context tests pass. |
| 13 | Area-less `TemplateGrid` admits track lists | Layout requires validated `template_areas` metadata for the new admission; negative layout test rejects the no-area case. |
| 14 | New predicate widens named implicit modes | Existing named implicit modes retain px-list support; the regression test also rejects non-px lists there. |
| 15 | Row/column property context is lost on reservation failure | Extension context carries the matching `grid-auto-*` property and node ID into the error; injected failure tests pass. |
| 16 | Track allocation partially mutates the projection | Reservation occurs before extending the vector, and injected reservation failure followed by a valid projection recovers; pass. |
| 17 | Line-name groups no longer match track boundaries | Projection ensures at least `track_count + 1` line groups and tests the final lengths on both axes; pass. |
| 18 | Counts or metadata overflow bypass the existing caps | Checked arithmetic and existing reservation/cap paths remain in use; track and combined-metadata boundary tests pass. |
| 19 | New runtime IDs diverge between FFI, Android, iOS, or matrix summaries | The six IDs, strict 37/118 counts, unknown-ID rejection, app allowlists, and builder check agree; pass. |
| 20 | Fixture symbols or payload leak into the product archive | Default static archive has 0 C11.3.3 exports and no cycle-scene strings; opt-in positive control has the three expected exports and cycle payload. This is static-library evidence only. |

## Findings fixed during execution

- Android initially classified new pixel sample IDs as the default background RGB. The scene-color table and a structural assertion now cover every added sample ID; the final AVD run is exact.
- A first continuation fixture used numeric implicit placement, which is outside the selected supported subset. It was changed to the existing named implicit-line route, and its expected later-track phase was rechecked in Chrome, Rust, and both simulators.
- The fixture-module boundary suite had a stale C11.4 catalog expectation of 618 while the current capture catalog is 622. The assertion was synced to the current catalog and the complete boundary suite passes.
- Clippy found repeat-iterator/conversion issues and argument-count warnings. The implementation uses `repeat_n`, direct `usize`, and small option/context structs; focused Clippy now passes without lint suppressions.

## Limits

Android evidence is API 37 ARM64 AVD on SwiftShader, not a physical device or hardware GPU. iOS evidence reads Metal drawable pixels before compositor submission. Neither result establishes WPT conformance, general CSS Grid support, or C11.3.3 completion.
