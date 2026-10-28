# C11.4 `fit-content()` automatic-minimum floor implementation PR diff review 30

This review checks the full implementation change set across layout/runtime code, both simulator adapters, capture tooling, archive evidence, screenshots, and roadmap records. It is separate from the plan review and implementation/runtime review.

1. **A full Taffy algorithm change can be smuggled into a two-scene fixture — PASS.** The diff adds an independent layout oracle and a fixture adapter; it does not change the shared Grid sizing algorithm.
2. **The plan remains marked as “not implemented” after runtime work — PASS.** The plan index now links the simulator evidence while preserving the narrow scope and `72/213` status.
3. **A plan change could inflate the official C11.4 completion state — PASS.** The relevant checkbox and official completion count are unchanged.
4. **The WPT path or pinned revision can drift in implementation docs — PASS.** Plan, precomparison, runtime README, and status record point to the pinned `fit-content-track-sizing.html` assertion and revision.
5. **Chrome's upstream WPT result can be reported as a Spinon WPT pass — PASS.** Documentation names the upstream baseline and explicitly separates it from the two-scene Spinon runtime fixture.
6. **The independent CSS expectation can be inferred from the runtime result — PASS.** The expected `[10,100]` and `[10,40]` vectors come from the precomparison/CRD calculation and are separately hard-coded in the layout oracle.
7. **The zero control can accidentally count as another WPT assertion — PASS.** The docs call it a differential control and do not count it as a WPT pass.
8. **The main/control sources can diverge in more than `min-height` — PASS.** Both fixture sources keep the same root, two-item topology, track, and 10px/100px contents; only the control's second item has `min-height:0`.
9. **The fixture can author a CSS property unsupported by the profile — PASS.** Both sources omit explicit `overflow`; runtime requires computed visible overflow, matching the pinned default behavior.
10. **Additional arbitrary nodes can enter the fixture through the ID route — PASS.** Runtime checks the root/element identities, ownership, child counts, and the five exact node identities before building an observation.
11. **The fixture source allowlist can accept modified or unknown IDs — PASS.** Only the two exact scene IDs map to source; source tests reject altered and unknown IDs.
12. **Computed styles can include unrelated nodes or duplicate identities — PASS.** Runtime preparation/output validation require a unique style-node set equal to the five expected nodes.
13. **The CSS style and typed Grid input can disagree — PASS.** Runtime checks the fixed profile, one explicit column, implicit fit-content rows, zero spacing, default borders, auto flow, and absence of repeat/areas/named-line extras.
14. **The automatic minimum can be reduced in the WPT scene — PASS.** Main computed item minima are `Auto/Auto`; only the second control item is `LengthPx(0)`.
15. **The two scene outputs can be combined across document versions — PASS.** Runtime observations, layout trace, style and output keys must share generation, document, render-tree, style, and environment revisions.
16. **Frames can pass with missing, duplicate, non-finite, or wrong nodes — PASS.** Runtime validates a complete unique five-node frame set and both platform matchers compare all five expected coordinates and sizes.
17. **A matching final row can hide a broken sizing stage — PASS.** The expected six stage kinds and their per-stage row-base values are checked in runtime, Android, iOS, and the capture runner.
18. **Trace gutters or collapsed boundary entries can change unnoticed — PASS.** Runtime checks the five alternating row-track/gutter entries, zero gutter bases, and collapsed boundary gutters.
19. **The FFI diagnostic can omit or mislabel the observation — PASS.** The new optional report contains computed style, typed Grid input, node IDs, frames, row trace, item min-heights, and frame-source metadata; missing frames or root style fail report generation.
20. **Rust fixture diagnostics can break clone/constructor call sites — PASS.** Every touched `RuntimeGridTrackSizingDiagnostics` construction and clone copies or initializes the new optional field; feature-enabled tests compile and pass.
21. **Android can accept malformed JSON or a stale frame report — PASS.** The strict Java matcher validates input, style, node IDs, frames, stage values, revisions, and presentation identity before accepting a scene.
22. **iOS can accept a weaker contract than Android — PASS.** The Swift matcher checks the same two IDs, revisions, item minima, nodes, frames, tracks, and ordered stage values; iOS build and simulator captures pass.
23. **The first runtime layout can be mistaken for the displayed surface — PASS.** Both platforms compare the draw identity with the surface generation and presentation sequence; the iOS local-name compile issue was fixed before the final shared-source captures.
24. **The surface samples can miss the overflow distinction — PASS.** Main uses four samples; control includes child content inside and below its 40px row, root background, and a point outside the root, for five samples per platform.
25. **Pixel coordinates/colors can be copied from the wrong platform scale — PASS.** Android PixelCopy and iOS Metal logs contain CSS/device coordinates and exact expected/actual RGBA values; summaries are 4/4 and 5/5.
26. **The capture catalog can omit or alias a new scene — PASS.** Each scene has an exact frame, track, presentation identity, box count, and pixel expectations; the Node runner selects precisely the two IDs.
27. **Historical catalog tests can keep stale scene counts — PASS.** All affected count expectations were updated from the former catalog size to 524; the fixture-boundary suite passes 16/16.
28. **The fixture route can leak into a production GPU archive — PASS.** The `c04-runtime-gpu` release archive scan has zero matching IDs, source paths, or symbols; the `c04-runtime-gpu-fixture-test` positive control contains the new source and symbols. This does not claim APK/IPA exclusion.
29. **Evidence can show a pass from a stale APK or different source revision — PASS.** The first stale APK and profile rejection remain documented; final Android/iOS captures have matching tracked-diff and source-snapshot hashes, and only those final runs support the pass claim.
30. **The docs can overclaim the supported feature or hide unverified layers — PASS.** Status, conformance, plan, and evidence state that this is a restricted opt-in slice, not a full WPT pass, general CSS admission, physical-device/hardware-GPU run, iOS post-compositor pixel check, or C11.4 completion.

**Result:** 30 PASS / 0 PARTIAL / 0 FAIL. The review found no remaining PR-diff inconsistency requiring another code change.
