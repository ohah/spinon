# C11.4 zero fixed maximum PR diff review: 20 failure perspectives

Final-diff review on 2026-10-29, after simulator recapture and documentation updates. Scope is the zero-fixed-maximum slice; unrelated working-tree snapshots and generated Taffy files are excluded from the proposed change.

1. **New FFI fixture module is unconditionally compiled** — PASS. Module declaration is gated by `c04-runtime-gpu-fixture-test`.
2. **Scene routing is missing or shadowed by another fixture** — PASS. The new exact-ID route is in `fixtures/c11_4.rs`; its FFI test checks both listed and unlisted IDs.
3. **Runtime JavaScript is built from untrusted or mismatched scene data** — PASS. The source is generated from the checked-in 24-scene JSON and the test checks track inputs, placements, and route success.
4. **The scene JSON and Chrome oracle disagree** — PASS. Rust asserts field agreement and ordered Android/iOS verification compares the exact 24 IDs.
5. **Rust layout test accidentally changes supported production grammar** — PASS. It builds `GridTrackSize` directly and adds no author admission path.
6. **Taffy test validates columns but not rows** — PASS. Explicit column and row helpers call the sizing function with horizontal and vertical axes.
7. **The Taffy test passes due to an unrepresentative maximum** — PASS. It covers 0px and 2px, with gap-specific expected counts.
8. **A scene crosses more than one fixture profile** — PASS. The new IDs have a dedicated run kind and an exact 24-ID allowlist.
9. **Default Android build embeds the new IDs** — PASS. The default APK scan is negative; the fixture-test APK positive control contains both markers.
10. **Default iOS build embeds the new IDs** — PASS. The default iOS executable scan is negative; the fixture-test executable positive control contains both markers.
11. **Android pixel samples accept wrong targets or positions** — PASS. The matcher checks sample count, ID uniqueness, target node ID, and exact CSS-pixel coordinates.
12. **iOS pixel samples accept wrong targets or positions** — PASS. The Swift matcher applies the same exact ID/target/coordinate constraints.
13. **Border-exterior overflow is lost from evidence** — PASS. The overflow sample is included only for auto-fill with 4px gap and is checked in each platform run.
14. **Capture can select the entire 618-scene catalog** — PASS. The run kind requires explicit selection; the Node boundary test checks rejection without it.
15. **The test catalog count breaks neighboring fixture tests** — PASS. All 22 fixture-boundary tests pass with the updated 618-scene catalog count.
16. **Swift frame helper fails to compile after guard addition** — PASS. The explicit-return correction is present; final iOS fixture build succeeded and the 24-scene run passed.
17. **Evidence hashes do not describe the installed app** — PASS. Both latest run receipts report installed-app hash equality; verifier checks the hash fields.
18. **Screenshot evidence is missing or unreadable** — PASS. Verifier checks all 24 PNG headers/dimensions per platform and writes per-run SHA-256 manifests; representative captures were visually inspected.
19. **Documentation overstates simulator or support coverage** — PASS. Evidence limits AVD, pre-compositor Metal readback, fixed scenes, and the incomplete parent explicitly.
20. **Unrelated untracked files enter the PR** — PASS. Only listed source, test, plan/status/spec, and dedicated evidence paths are to be staged; no blanket add is used.

Checks recorded: focused layout/FFI/Taffy tests pass 1/1 each, fixture boundary passes 22/22, Rust formatting and diff whitespace checks pass, and final Android/iOS simulator verification passes 24/24 each. No CI or merge state is implied by these local results.
