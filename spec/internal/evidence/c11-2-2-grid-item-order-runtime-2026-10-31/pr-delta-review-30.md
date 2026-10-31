# C11.2.2 sparse-row `order` PR delta review · 30 perspectives

Date: 2026-10-31. Review covers the implementation, versioned fixture tooling, platform routes, evidence, and roadmap edits in this PR candidate.

1. **Product scope — PASS.** The new capability is restricted to one existing runtime profile and two fixed scenes.
2. **CSS claim boundary — PASS.** Documentation calls this a fixture candidate, not general CSS Grid or WPT conformance.
3. **Official status — PASS.** `spec/STATUS.md` records candidate evidence while preserving the parent checkbox and `72/213` count.
4. **Internal contract version — PASS.** Contract `0059` remains `0.1.0`.
5. **Old matrix preservation — PASS.** Existing 16-scene JSON is not modified; the new input and pixel observations have separate versioned files.
6. **FFI selector — PASS.** Exact constant IDs choose the new matrix, and unknown/suffixed values are rejected.
7. **Matrix metadata — PASS.** Runtime loader checks schema, profile, exact scene order/count, aggregate node/item counts, outcome, and root identity.
8. **Scene source limits — PASS.** Loader rejects oversized source, missing expected rectangles, duplicate IDs, malformed parent/node arrays, and mismatched scene identity.
9. **Pixel matrix scope — PASS.** Samples identify item interiors for the two fixed scenes and do not imply overlap or border coverage.
10. **Computed property type — PASS.** `order` conversion remains checked at the typed `i32` boundary with exact error context.
11. **Stable tie source — PASS.** The comparison fixture includes tied negative orders and validates the resulting geometry.
12. **Independent expected values — PASS.** Chrome 156 observation and input hashes are retained separately from implementation results.
13. **Host tree order — PASS.** Style/runtime tests verify the DOM child sequence stays in source order.
14. **Dense profile regression — PASS.** A style-to-layout negative fixture confirms nonzero order still fails under the dense profile.
15. **Partially placed child regression — PASS.** Layout-core negative coverage rejects a visible partial-placement sibling.
16. **Existing C11.2.2 scenes — PASS.** Existing focused placement tests passed before and after the narrow implementation path.
17. **Android admission — PASS after correction.** The initial launch and marker-timeout evidence is preserved; source tracing found the allowlist gap, and both scene and pixel allowlists now contain the two IDs.
18. **iOS admission — PASS after correction.** Both iOS auto-placement and Metal pixel sample allowlists contain the same IDs.
19. **Android device assertion — PASS.** Runner requires connected API 37 ARM64 QEMU AVD and stores device/render metadata.
20. **iOS device assertion — PASS.** Runner requires a booted, available iPhone 17 Pro on iOS 26.2 Simulator.
21. **Fresh app processes — PASS.** Each scene starts with a fresh Android process or `simctl launch --terminate-running-process` process.
22. **Capture termination — PASS.** Android runner force-stops and iOS runner terminates the app on success and failure.
23. **Geometry assertion — PASS.** Both platform summaries require matching frame set and report zero CSS-pixel error.
24. **Pixel assertion — PASS.** PixelCopy and Metal comparison each validate the exact expected sample count and channel values.
25. **Screenshot interpretation — PASS.** Screenshots are stored as scene evidence and explicitly not treated as pixel parity.
26. **Build configuration — PASS.** Both builds enable the opt-in runtime fixture; iOS additionally asserts the Metal sample build flag.
27. **Default artifact boundary — PASS.** The core `c04-runtime-gpu` archive lacks both new C11 fixture symbols; the opt-in fixture archive contains both. [Inspection evidence](./artifact-symbol-boundary.md).
28. **Lint/format — PASS.** Workspace Clippy, rustfmt, and `git diff --check` passed after correcting the reported conditional style.
29. **Workspace test accounting — PASS WITH EXISTING FAILURE DISCLOSED.** One C07.3 pinned computed-string mismatch reproduced in isolation; it is outside the diff. The other 1,431 workspace tests pass with 4 ignored when only that exact mismatch is filtered. [Failure and rerun log](./workspace-test-c07-3-baseline-failure.md).
30. **Screenshot links in PR — PASS.** PR #899 renders both branch-hosted captures in the conversation; Agent Browser verified image load at 1080×2400 (Android AVD) and 1206×2622 (iPhone Simulator). The browser session was closed and no Agent Browser/Chromium process remained.

All reviewed delta items have a recorded result. The review records the existing unrelated C07.3 test mismatch and the repository's empty check rollup separately from the passing filtered workspace run. PR #899 was merged using Rebase and merge at `2026-10-31T03:51:58Z`, merge SHA `0450007c3fd0b021658f45ffb6b4b413ca28a0f2`.
