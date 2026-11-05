# C11.4 intrinsic auto rows plan review · 20 independent failure perspectives

Scope: review the plan for adding only `grid-auto-rows:min-content` to the default parsed Root HTML intrinsic-track route. This is a pre-implementation review; these perspectives do not replace implementation/runtime or PR-diff review.

1. **Computed token drift — PASS.** The plan requires a typed single `MinContent` value and avoids reparsing computed CSS strings.
2. **Wrong owner routing — PASS.** Candidate detection is limited to the owner whose computed Grid contains the intrinsic auto row and preserves the aspect-ratio route exclusion.
3. **Placement mode collision — PASS.** The plan says to preserve candidate precedence and existing placement profiles.
4. **Explicit-grid regression — PASS.** The change reuses `RootHtmlIntrinsicTrackSizingV1` with existing explicit track validation intact.
5. **Implicit column escape — PASS.** `grid-auto-columns` remains `auto`; implicit columns are explicitly outside scope.
6. **Track-list cycling ambiguity — PASS.** Only a single typed `MinContent` pattern is accepted and repeated for each implicit row.
7. **Fixed-size item contribution — PASS.** The precomparison uses fixed item block sizes, giving independent 10/22/34px contributions without font-dependent geometry.
8. **Intrinsic height misreported as a preflight bound — PASS.** The plan forbids treating content-derived track sizes as known fixed extents and requires finite final traces/frames.
9. **Required-row undercount — PASS.** Acceptance explicitly tests one and two implicit rows plus the 10,001-row cap boundary, and requires sizing from the actual required row count.
10. **Gap multiplication — PASS.** Zero gaps are required, so the existing explicit-track gap preflight cannot silently undercount implicit gutters.
11. **Track-count denial of service — PASS.** Existing 10,000-track and checked-count limits remain required, with a 10,001-row rejection case.
12. **Partial scene after rejection — PASS.** Unsupported patterns and runtime failures must reject the whole layout/paint publication.
13. **CSS-valid versus CSS-invalid conflation — PASS.** Valid but unsupported auto-row values fail as adapter diagnostics; invalid declarations retain cascade fallback semantics.
14. **Overflow clipping regression — PASS.** The expected third item bottom is 66px under a 40px visible Grid; runtime gates inspect the outside portion separately.
15. **DOM paint order — PASS.** Source order is retained and explicitly checked across multiple implicit rows.
16. **Cross-platform evidence overclaim — PASS.** AVD, iOS Simulator, native readback and screenshots are separate evidence, and no hardware/compositor claim is made.
17. **Stale revision mixing — PASS.** The runtime verification requires the same fixture/source revision through layout and presentation.
18. **Default artifact leakage — PASS.** The implementation reuses the product route and requires existing default-vs-fixture artifact boundary checks; no selector/export is added.
19. **Scope creep into C11 completion — PASS.** The plan expressly preserves C11.4/C11 checkboxes and official 78/214 count.
20. **Browser evidence/process residue — PASS.** The Chrome observation, independent expected, screenshot and hashes are separate, and Agent Browser/server shutdown is explicit.

Review result: **20/20 PASS for plan structure and scope.** Implementation must still verify that the existing Taffy pipeline produces the independently fixed row contributions and that each negative input fails at the intended layer.
