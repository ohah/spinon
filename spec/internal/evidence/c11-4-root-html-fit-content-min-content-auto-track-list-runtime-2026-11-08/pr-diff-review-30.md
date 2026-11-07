# C11.4 fit-content/min-content/auto candidate diff review (30 paths)

Pre-PR adversarial review of the implementation, tests, fixed oracle, simulator evidence, and plan changes in this candidate. Each item examines a separate failure path or contract boundary. Result: 30/30 reviewed, no blocking diff defect found. The final PR body must bind simulator screenshots to the pushed commit and link this review.

1. **Unrelated profiles gain `min-content` admission.** Checked the mode predicates in `grid_values.rs` and `grid.rs`; the exception is tied to `RootHtmlFitContentAutoRowsV1` and an exact list.
2. **Other track-list order accepted.** The computed-style helper permits only the two documented sequences; both are tested separately.
3. **Track-list length drifts from the contract.** Three-member matching prevents missing or appended track functions; four-member negatives cover the adjacent case.
4. **Duplicate fit-content anchor.** A second fit-content cannot fit either exact three-member pattern and fails closed.
5. **Duplicate min-content or auto.** Style/runtime negatives verify duplicate min-content and missing required auto cases.
6. **Invalid pixel cap.** Negative, NaN and infinite px values are rejected in projection; the independent Chrome fixture covers a finite 20px cap.
7. **Invalid percentage cap.** Negative, NaN and infinite percentage values are rejected; the primary oracle uses 20% and 120% limits.
8. **Cap is interpreted in the wrong unit.** The px oracle records computed `fit-content(20px)` and `[10,20,34,30,6]px`; the runtime suite checks the typed `LengthPx` route.
9. **Percentage basis is indefinite or wrong.** The plan fixes a definite 100px content box; 20% resolves to 20px and 120% to 120px in the overflow oracle.
10. **Fit-content behaves as an unconditional fixed size.** The 120% scene uses 6px and 40px repeated rows from contributions/available space instead of forcing the cap.
11. **Min-content uses max-content sizing.** The two order scenes preserve the individual 34px and 10px min-content contributions; style projection checks typed member identity.
12. **Auto track does not receive positive free space.** Chrome, layout and RuntimeSession agree on 30px/54px auto rows in the two orders.
13. **Stretch affects min-content or fit-content too.** Layout frame assertions prove only the auto track receives the 20px free-space increase.
14. **Track-list repetition begins at the second member.** The fourth implicit row is asserted against the repeated fit-content member in both positive and overflow scenes.
15. **`align-content:stretch` leaks to another list.** Non-exact list plus stretch fails in layout and RuntimeSession; the projection scope is based on the exact computed list.
16. **Another content-distribution value becomes accepted.** Runtime changes stretch to center and expects a failed revision with no completed result.
17. **Existing start alignment regresses.** The previous fit-content-auto suite remains in the focused 11-test layout, 5-test projection and 11-test runtime runs.
18. **`max-content` is accepted as a side effect.** Explicit projection/runtime negative inputs and the exact typed-list predicate keep it unsupported.
19. **`calc()`, `minmax()` or `fr` enters the intrinsic exception.** The pattern only matches `FitContent(LengthPx|Percentage)`, `MinContent`, and `Auto`; other enum variants fail the track guard.
20. **Explicit template tracks are incorrectly included.** Only `grid-auto-rows` is extended; explicit rows/columns stay under their pre-existing shape checks.
21. **Implicit columns inherit the new sizing.** The existing contract continues to require `grid-auto-columns:auto`; no column track code is changed.
22. **Dense/column auto-placement inherits the pattern.** Normalization keys on the existing sparse Root HTML fit-content placement mode; other placement branches are unchanged.
23. **Spanning or changed DOM topology enters the profile.** The existing validator still requires the bounded direct-child topology; new tests retain one explicit row and four one-payload implicit items.
24. **Item frames look right while payloads drift.** Chrome, layout and RuntimeSession compare each item and each nested payload independently.
25. **Visible overflow is clipped at the Grid boundary.** The 50px fourth payload remains 50px while its track is 40px and extends 10px below the border.
26. **Malformed list partially publishes layout/render state.** Runtime negatives assert `Failed` and absence of `completed`; there is no partially published render snapshot.
27. **Recovery only works in a fresh session.** Dark-media failure is followed by a light-media revision in the same `RuntimeSession`, with full recovered frames asserted.
28. **Computed values reorder or lose the authored members.** Runtime checks the typed `ComputedCssGridAutoRows` vector and computed serialization string before checking geometry.
29. **Evidence binds to a different fixture or an early blank screen.** Fixture/build SHA is recorded; Android current-PID evidence gates the final capture on layout-ready and draw-presented. The initial blank screenshot is retained and labeled as early.
30. **PR claims surface parity or app-data continuity without evidence.** The README labels screenshots as visual smoke, excludes PixelCopy/Metal readback, records the changed iOS data-container path and unknown cause, and does not claim preserved old app data.

Focused Rust, scoped Clippy, rustfmt, diff checks, Chrome 157 px/% captures, and the bounded Android/iOS Simulator run are summarized in [`README.md`](./README.md). C11.4/C11 and official `78/214` remain unchanged.
