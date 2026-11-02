# PR #1041 planning diff adversarial review 20

Reviewed PR: [#1041](https://github.com/ohah/spinon/pull/1041), base `4408c88e513cf04fba4ce219176042bdc13d9a3a`, initial head `bc67db719442d452e027ef1829969202b0c48015`. The review covers the actual GitHub PR diff, linked oracle artifacts, and rendered PR body.

1. **Branch and base identity drift — PASS.** PR base is current `main` at `4408c88`; head is the plan branch commit `bc67db7`.
2. **Implementation code accidentally included in plan PR — PASS.** The eight-file diff contains planning, fixture, observation, screenshot, and roadmap pointers only.
3. **C11 completion promoted without implementation — PASS.** The plan explicitly says candidate/not implemented; no `spec/STATUS.md`, public roadmap, or interface contract completion status is changed.
4. **Official aggregate changed by a planning candidate — PASS.** `72/213` is documented as unchanged in both parent-plan references and PR description.
5. **Candidate is not discoverable from C11 roadmap — PASS.** The C11 master plan and Root HTML runtime plan link the dedicated follow-up.
6. **Existing scalar path regressions omitted — PASS.** The implementation plan preserves `LengthPx` and requires the existing scalar path as a regression.
7. **Root profile boundary too broad — PASS.** Only `RuntimeRootHtmlTextPaintV1` / `RootHtmlImplicitRowsV1` receives fixed-px list support; other profile boundaries remain closed.
8. **List cycle starts at an ambiguous track — PASS.** The plan anchors pattern index 0 to the first implicit row after explicit rows.
9. **Unsupported grammar admitted unintentionally — PASS.** The plan rejects percentages, fractions, auto/intrinsic breadths, `minmax`, `fit-content`, auto-repeat and calculated values.
10. **Zero-length track semantics lost — PASS.** Zero is explicitly a valid boundary and must receive its own regression.
11. **Invalid, empty, or non-finite list reaches layout — PASS.** The plan requires non-empty, finite, non-negative validation at projection and core admission.
12. **Implicit rows interact with named areas without a defined contract — PASS.** The existing unsupported combination remains rejected.
13. **Existing explicit px/percentage/fr composition regresses — PASS.** The plan identifies the #1027 combination and keeps it in the regression matrix.
14. **Overflow, row gaps, or align-content change the oracle accidentally — PASS.** The fixture fixes gaps and `align-content:start`; independent arithmetic accounts for all three gutters and the 10px remainder.
15. **Chrome output is presented as the normative source — PASS.** The plan separates CSS rule-based expected geometry from one Chrome observation and limits the browser claim.
16. **Observation does not bind to its source fixture — PASS.** JSON contains fixture path and SHA-256; local integrity check matched both fixture and screenshot hashes.
17. **Observation has incomplete geometry — PASS.** The JSON records computed track list, expanded rows, root frame and all eight item frames; the independent verifier matched each expected frame.
18. **PR screenshot link is broken or omitted — PASS.** Agent Browser opened PR #1041; the body image loaded at 1280×633 with `complete=true` and nonzero natural dimensions.
19. **Screenshot is overstated as pixel or runtime proof — PASS.** Evidence and PR body call it a visual Chrome capture; Spinon/mobile/surface pixel execution remains explicitly outstanding.
20. **Plan PR claims unrun CI or runtime verification — PASS.** PR check rollup is empty; the body states Android/iOS runtime is not run and remains for implementation PR.

**Result:** 20/20 review points pass for the planning diff. No blocker found. GitHub reports PR #1041 as open with merge state `CLEAN` and no check runs; those facts are not CI evidence. This review does not validate the future implementation.
