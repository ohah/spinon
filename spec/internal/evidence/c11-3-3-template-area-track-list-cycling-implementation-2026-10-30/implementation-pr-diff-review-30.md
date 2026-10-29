# C11.3.3 template-area track-list cycling implementation PR diff review · 30 cases

This final-diff review checks distinct contract, source, fixture, platform, artifact, and reporting boundaries. It is separate from the 20-case implementation/runtime review. Review status is based on the final local diff and final simulator captures.

| # | Diff failure mode | Final check |
| ---: | --- | --- |
| 1 | Projection changes every profile instead of the selected template-area profile | List projection remains gated by `RuntimeBlockFlexGridTemplateV1`; other profiles retain their existing paths. |
| 2 | Lists with unsupported members enter through projection | The admission predicate requires a non-empty list consisting entirely of fixed px track sizes. |
| 3 | Empty lists pass projection but fail later unpredictably | Empty-list guard and explicit projection/layout tests reject them. |
| 4 | Row expansion reads the column auto-track value | Row helper call receives `auto_rows`; unique row oracle and projection vectors pass. |
| 5 | Column expansion reads the row auto-track value | Column helper call receives `auto_columns`; unique column oracle and projection vectors pass. |
| 6 | Single-value scalar behavior changes during list support | `Auto` and `LengthPx` retain separate branches with the same generated values; old matrix scenes still pass. |
| 7 | Repeated values stop before wrap-around | The 2-value patterns generate 3 implicit tracks in the two-axis fixture; exact expected tracks and pixels pass. |
| 8 | Pattern is rotated by authored track count rather than area-created track count | The helper returns only the number actually appended, which is used to rotate the remaining implicit pattern phase. |
| 9 | Pattern phase is shared across axes | Each axis is extended and rotated independently; focused projection tests pass. |
| 10 | Authored tracks or their line names are dropped | Existing prefixes remain in vectors; line-name metadata reaches at least tracks plus one. |
| 11 | Track allocation happens after mutation begins | Fallible reserve precedes extension for Auto, px, and list branches. |
| 12 | Empty or unsupported branch returns a partially built scene | Projection returns an error before producing `GridContainerInput`; recovery test passes. |
| 13 | Area metadata alone enables unsupported `TemplateGrid` lists | Layout gate requires `template_areas`, non-empty values, and fixed px members. |
| 14 | New validation silently removes named-mode px lists | Separate existing validator arm and regression test retain named-mode px support. |
| 15 | C11.4 or C11.2 grammar is widened by the new validator arm | New `TemplateGrid` pattern is scoped to its placement mode; named and unrelated-mode negative controls pass. |
| 16 | Cascade option refactor changes an unrelated capability flag | `GridTrackComputeOptions` only groups the four prior booleans; columns and rows pass the same values as before. |
| 17 | C11.3.3 FFI matrix advertises the wrong number of scenes/nodes | Rust scene allowlist and matrix summary both require 37 scenes, 118 nodes, 31 successes, 5 fallbacks, and 1 rejection. |
| 18 | Six new scene IDs are missing from a runtime route | FFI source, Android activity and demo allowlists, iOS set, matrix builder, and Node structural tests include all six IDs. |
| 19 | Old 31 scene IDs or results are replaced while extending the matrix | The previous cases remain; only six scenes are appended and the final matrix outcome counts preserve old fallback/rejection totals. |
| 20 | Fixture data is edited without updating generated hash/count consumers | Builder `--check`, C11.3.3 Node tests, FFI selection test, and Rust summary validator pass. |
| 21 | Expected vectors are copied from Spinon output, creating a circular oracle | Expected used tracks and frame rectangles come from the separately stored Chrome observation and CSS Grid rule-based JSON. |
| 22 | Continuation fixture accidentally tests unsupported numeric placement | Both continuation items use the existing named implicit-line route; numeric placement remains explicitly out of scope. |
| 23 | Android accepts incorrect pixel coordinates or node IDs | Sample validator compares exact count, sample ID, target node ID, and CSS coordinate; final PixelCopy is exact. |
| 24 | Android assigns a new item the background color | Explicit RGB cases cover each new sample identifier; structural assertion and 28/28 final PixelCopy samples pass. |
| 25 | Android matrix silently chooses a physical device | Command selected `android-avd emulator-5554`; result records an API 37 QEMU AVD and SwiftShader renderer. |
| 26 | iOS allowlist omits a scene or runs the wrong pixel fixture | Scene set has all six IDs; capture selects per-scene sample coordinates/RGBA and final six-scene Metal comparison is 28/28. |
| 27 | iOS marks a scene complete before required drawable samples exist | `requiresMetalResult` is derived from selected sample presence and is passed through completion/superseded checks; comparator asserts exact count and zero error. |
| 28 | Negative sample or unknown scene reaches drawing/readback | Android and iOS results report rejected-before-readback or rejected-before-runtime-draw; final result JSONs retain these controls. |
| 29 | PR screenshot is stale, empty, or mistaken for post-compositor evidence | The two images are from the final run IDs and visibly show the cycle scene; Android is AVD screen capture and iOS is simulator screen capture, while Metal pixel proof remains pre-compositor. |
| 30 | Fixture symbols or payload leak into the product archive, or evidence overclaims bundle exclusion | Default static archive has 0 C11.3.3 exports and no cycle-scene strings; the opt-in positive control has exactly the `new`, `eval`, and surface-pixel request exports plus cycle payload. README limits the result to the static archive. |

## Final review result

30 distinct diff risks were checked. No open implementation blocker remains in this bounded slice. The C11.3.3 and C11 official checkboxes and completion count remain unchanged pending merge and bounded documentation sync.
