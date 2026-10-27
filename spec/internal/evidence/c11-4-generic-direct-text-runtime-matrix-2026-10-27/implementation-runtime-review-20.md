# C11.4 direct-text implementation and runtime review · 20 perspectives

Reviewed the fixture source, runtime diagnostics, Android and iOS scene matchers, captured runtime traces, and simulator matrix in pass02. Each check below passed; no open product-code finding remains.

| # | Failure perspective | Review result |
| ---: | --- | --- |
| 1 | A generic scene route accidentally becomes available in a non-fixture app build. | Android allowlist checks the fixture-test BuildConfig flag; iOS compiles an empty allowlist when the fixture-test condition is off. Default FFI archive omits the new route string. |
| 2 | An unknown scene ID silently falls through to another source. | Rust scene lookup returns None for unregistered IDs; the existing unknown-ID unit assertion passes. Native route admission also rejects IDs outside its allowlist. |
| 3 | The app route ID is confused with the DOM root ID. | Each of the six sources returns its route ID and root ID separately; content-variation and reorder preserve grid-9x and grid-reorder roots. Both platform matchers check their expected node frames. |
| 4 | Scene data is interpolated from caller input and can alter the fixture program. | All interpolated IDs, classes, styles, and text are fixed static scene constants; the device route selects only from the six-entry table. |
| 5 | The common Grid setup drifts between scenes. | All six sources retain the fixed 100px min-content/max-content columns, 10px column gap, Ahem 10px font and line-height, zero edges, and auto rows. Runtime matrix passed. |
| 6 | Normal wrapping produces a wrong min/max intrinsic width. | Both normal items are checked at 20/40 width, 20/10 height, and final line advances [10,20] and [40]. Android and iOS passed. |
| 7 | No-wrap is represented by a fixture label but not computed paragraph style. | Both items are checked for NoWrap, intrinsic width 40/40, height 10/10, and a single 40px line. Both platforms passed. |
| 8 | Break-all leaks across siblings or changes the wrong input. | The break-all scene checks both ABCD items and four 10px lines for the narrow first item; the reordered scene separately requires item-z to stay Normal with one 40px line. Both passed. |
| 9 | Content-variation uses the same default text accidentally. | The two items use the expected shared A BCD source but different node identities; the first and second tracks are checked at 30px and 50px with their distinct final line layouts. |
| 10 | Whitespace collapsing destroys the evidence of source whitespace. | The matcher compares raw source with leading/trailing and repeated spaces against normalized layout text A BC. Both platforms passed. |
| 11 | DOM source order and final item order are conflated. | The content-variation and reorder routes preserve their planned sourceOrder and final node identity. Frame expectations check each expected node at its item slot. |
| 12 | A text layout is attached to a sibling or a different paragraph. | Each trace requires exactly two snapshots, node equals layoutParagraph, and each direct source is the following text node with no inline ancestors. |
| 13 | Stale text layout data is presented as belonging to another layout revision. | Trace serialization validates snapshot revision against diagnostics; both native matchers compare source/style/environment/intrinsic-size revision fields to the current layout input revision. |
| 14 | A helper captures only text size and hides font registry changes. | Each input and each text layout requires font registry revision 1; each layout also checks the fixed font family, size, weight, style, and line-height. |
| 15 | Typed contribution input omits style constraints or nonzero edges. | Both axes require min/max content values, auto preferred/min/max sizes, zero margin and padding lengths, and zero borders. Missing or malformed fields reject the scene. |
| 16 | Final text layout is measured at an unrelated width or has wrong line geometry. | Each item checks available width, layout width/height, exact line count, line advances, line tops, and line heights. Both platform matchers passed all scenes. |
| 17 | Revision diagnostics change behavior for unrelated C11.4 scenes. | Text snapshots are appended only to the opt-in C11.4 diagnostic record and cloned/re-keyed alongside existing revisioned inputs. Focused trace tests and workspace Clippy passed. |
| 18 | Contribution trace checks only columns or allows wrong track sizing. | The matcher requires one grid trace, two ordered axis passes, exact final track counts/sizes/gutters, and expected width and height contribution kinds/values. Both platforms passed. |
| 19 | Item geometry or present success is inferred from an incomplete summary. | Strict native matchers compare root and both item IDs/positions/sizes. The run additionally requires trace chunks and presented boxes=3. All twelve runs passed. |
| 20 | A layout screenshot is presented as glyph-paint or broader CSS compatibility evidence. | Both captured reports state runtime_text_count=0 and glyph_batch_draw_count=0. Evidence and roadmap scope this as opt-in direct-text intrinsic input only; C11 remains incomplete at 72/213. |

Validation logs and the exact source patch hash are in the sibling pass02 directory. The only first-pass rejection was the harness-created overlong run ID, recorded separately under pass01 and corrected before the complete matrix.
