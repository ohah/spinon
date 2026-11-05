# C11.4 intrinsic auto rows plan PR diff review · 30 adversarial perspectives

Reviewed the actual plan PR diff containing the Chrome 156 fixture and observation, independent expected values, screenshot, plan, and separate 20-point plan review. This review does not inspect or approve implementation code.

1. **Input identity — PASS.** Fixture path and recorded SHA-256 refer to the added HTML file.
2. **Chrome identity — PASS.** Observation records HeadlessChrome 156, the requested browser oracle.
3. **Expected versus observed — PASS.** Independent arithmetic is stored separately from browser measurements.
4. **Fixed track arithmetic — PASS.** Expected track sizes sum to the reported item origins and overflow endpoint.
5. **Implicit-row count — PASS.** One explicit row plus two implicit rows explains the three observed row sizes.
6. **Visible overflow — PASS.** The Grid bottom is 40px; the last item bottom is 66px; evidence records the 26px difference.
7. **Box sizing — PASS.** Grid and item use border-box with zero border, padding, and margin, so CSS-pixel boxes match expected sizes.
8. **Gap ambiguity — PASS.** Both row and column gaps are fixed to zero in the fixture.
9. **Placement ambiguity — PASS.** Row flow, one explicit column, one explicit row, and three source-ordered direct children yield the intended placements.
10. **Unsupported CSS ambiguity — PASS.** The plan distinguishes a valid but unsupported computed value from CSS-invalid declaration fallback.
11. **Product route — PASS.** Planned success path is the existing parsed Root HTML runtime route.
12. **New API surface — PASS.** The plan adds no parser, FFI selector, profile, or public API.
13. **Other Grid modes — PASS.** Dense/column flow, areas, repeats, lines, and implicit columns remain outside the new gate.
14. **Existing auto-row behavior — PASS.** The revision clarifies that `auto` behavior in the explicit-intrinsic route remains unchanged.
15. **Track resource cap — PASS.** Existing 10,000-track ceiling and checked counts are retained.
16. **Computed ownership — PASS.** Candidate selection uses typed computed values and preserves owner/property diagnostics.
17. **Placement-profile precedence — PASS.** The plan preserves current direct-text aspect-ratio and other candidate precedence.
18. **Item sizing contract — PASS.** Success items have finite fixed non-negative dimensions and no extra box-edge inputs.
19. **Grid size contract — PASS.** Both container dimensions are definite positive fixed CSS pixels.
20. **Alignment contract — PASS.** The plan keeps the current normal/start subset and fixes the precomparison to start.
21. **Trace acceptance — PASS.** Runtime acceptance requires finite non-negative track bases, origins, DOM order, and overflow.
22. **Failure atomicity — PASS.** Unsupported combinations must reject before partial frame or paint publication.
23. **Stale output — PASS.** Runtime evidence must bind fixture/source revision through layout and presentation.
24. **Android evidence — PASS.** AVD execution is scoped and does not claim physical-device or hardware GPU parity.
25. **iOS evidence — PASS.** Simulator geometry, drawable readback, and OS screenshot are kept as separate evidence classes.
26. **Screenshot meaning — PASS.** The Chrome capture is explicitly browser-only and is not represented as Spinon or WPT evidence.
27. **Default artifact boundary — PASS.** Existing product-versus-test fixture boundary checks are required; no selector/export is introduced.
28. **Plan-review separation — PASS.** The 20-point pre-implementation review is separate from future implementation/runtime and PR reviews.
29. **Official status — PASS.** The plan explicitly keeps C11.4/C11 open and preserves the current 78/214 roadmap count.
30. **Process and workspace handling — PASS.** The evidence records Agent Browser and loopback-server shutdown; the plan requires one checkout and no worktree.

Result: **30/30 PASS.** The plan PR contains no product implementation. The implementation PR remains subject to fresh implementation/runtime and actual-diff reviews after its final diff exists.
