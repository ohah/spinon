# C11.4 `fit-content()` automatic-minimum floor implementation/runtime review 20

This review targets implementation and runtime failure paths. It is separate from the 20-point plan review and the implementation PR-diff review.

1. **WPT fixture drifts to a neighboring assertion — PASS.** The main scene uses the pinned `fit-content(40px)` implicit rows, 10px/100px children, and default item `min-height:auto`; the exact WPT revision/source hash remain linked in the precomparison.
2. **The CSS formula is copied from current Taffy output — PASS.** The Rust oracle hard-codes the independent `[10,100]` and `[10,40]` control expectations and checks their row stages and node frames.
3. **The differential control changes more than the minimum — PASS.** Both sources share root geometry, track list, topology, and child heights; only the second item's `min-height:0` changes.
4. **The WPT auto outer width makes simulator frames nondeterministic — PASS.** The planned runtime adaptation fixes only the root width to 60px and checks the block-axis row result separately.
5. **Unknown scene IDs can select fixture content — PASS.** `runtime_source` returns a source only for the two exact IDs; a suffix/payload variant is rejected by the FFI unit test.
6. **A matching root ID hides a different DOM tree — PASS.** Runtime preparation checks HTML namespace, `div` tag, document-root ownership, exactly two direct items, one direct child each, parent links, and all five exact node IDs.
7. **An extra or missing computed-style node is silently ignored — PASS.** Preparation and output validation require a unique computed-style node set equal to the five fixture nodes.
8. **The root can enter a different profile or track grammar — PASS.** Computed style and typed input require the fixture profile, one 60px column, no explicit rows, one `fit-content(40px)` auto row, row auto-flow, and no repeat/template areas/named lines.
9. **An unsupported explicit overflow property causes a false runtime rejection — PASS.** Fixture source authors no overflow declaration, matching WPT's default-visible condition; runtime verifies computed visible overflow for root and items.
10. **Box spacing or borders alter the contribution/frame — PASS.** Root/items/children must have zero margin and padding and default border; the grid has zero row and column gaps.
11. **The control's automatic minimum remains `auto` — PASS.** The computed item `min-height` pair is checked as `[Auto, Auto]` for main and `[Auto, LengthPx(0)]` for control.
12. **Extra Grid containers contaminate the fixture trace — PASS.** The runtime observation requires exactly one root Grid input and rejects changed node ownership or root identity.
13. **Styles, frames, and trace can be combined across revisions — PASS.** Runtime output validation matches HostDocument generation, document/render-tree revisions, style revision, and environment revision to the completed layout key.
14. **Missing, duplicate, reordered, or non-finite frames pass — PASS.** Runtime validation requires all five unique node frames and finite coordinates/sizes within the pinned tolerance; the platform matchers check the exact node IDs and coordinates.
15. **A partial or wrong-axis trace is accepted — PASS.** Runtime validation requires one root trace with exactly one Columns pass and one Rows pass.
16. **The contribution-stage change is hidden by a matching final height — PASS.** Six stage names and each intermediate/final base-size vector are checked; the control must transition through intrinsic `[10,0]` and final `[10,40]`.
17. **Synthetic gutters or collapsed boundary entries distort row checks — PASS.** The runtime matcher requires five trace entries with the expected alternating gutter/track identity, collapsed boundary gutters, and zero gutter bases.
18. **Android accepts incomplete report JSON or stale output — PASS.** The Java matcher fails closed on missing/malformed diagnostics, style/input/frame IDs, stage sequence, source revision, dimensions, or presentation identity; PixelCopy verifies 4/4 and 5/5 samples.
19. **iOS accepts incomplete report JSON or stale output — PASS.** The Swift matcher checks the same scene/style/input/node/frame/trace/revision contract; the Metal drawable identity is tied to the same surface generation/sequence and its 4/4 and 5/5 samples match exactly.
20. **Fixture payload leaks into the production GPU archive or the result is overstated — PASS.** With `c04-runtime-gpu` enabled and fixture-test disabled, string scan found zero scene IDs/source markers/symbols; the opt-in archive contains them. The evidence records emulator/Simulator scope and leaves C11.4 and official `72/213` unchanged.

**Finding:** no remaining implementation/runtime failure was found in these 20 paths. The earlier stale Android APK, unsupported explicit overflow declaration, and Swift local-name collision were corrected and retained in the evidence README; they are not counted as final passing runs.
