# C11.4 zero fixed maximum implementation review: 20 failure perspectives

Reviewed the final implementation, fixture catalog, simulator captures, and artifact boundary on 2026-10-29. Each item names a concrete failure mode and the evidence used to reject it.

1. **Wrong UA floor for zero-sized fixed maxima** — PASS. Independent `n × 1 + (n − 1) × gap ≤ 16` calculation yields 16/4; the Chrome catalog and Taffy assertions agree.
2. **Only one Grid axis routed** — PASS. The fixture matrix contains both columns and rows; Rust direct layout, Taffy, Android, and iOS each cover both.
3. **An intrinsic minimum omitted** — PASS. `auto`, `min-content`, and `max-content` each appear in all 24 scene combinations and the direct Taffy loop.
4. **`auto-fill` confused with `auto-fit`** — PASS. Chrome, used-track collapse flags, item offsets, and both simulator matchers distinguish them.
5. **Gap 0 and gap 4 use the same repeat formula** — PASS. Counts are independently asserted as 16 and 4; the 4px gap case distinguishes item offsets.
6. **Occupied third track loses its 12px minimum** — PASS. Chrome observation, Rust frame/track assertion, FFI scene catalog, and platform track logs preserve the 12px track.
7. **`auto-fit` collapses the occupied track** — PASS. The collapse vector leaves index 2 open and collapses all other repeated tracks in each axis.
8. **`auto-fill` incorrectly collapses empty tracks** — PASS. All repeated tracks remain uncollapsed in scene expectations and final runtime traces.
9. **Gap collapse around empty `auto-fit` tracks is wrong** — PASS. Chrome item offset is 0px for fit and 8px for fill at gap 4; both platform frame matchers enforce those offsets.
10. **Root/item frames drift from the source scene** — PASS. All 24 simulator observations match the fixture root and item frames; both platform run IDs bind each capture to its scene.
11. **Overflow outside the root is never sampled** — PASS. Every auto-fill/gap-4 scene samples the item outside the 16px root; total exact surface samples are 54 per platform.
12. **Pixel probes hit another scene node or duplicate sample** — PASS. Android/iOS matchers require exact target IDs, positions, and a unique expected sample set.
13. **Scene catalog omits or duplicates an ID** — PASS. Fixture and Chrome JSON each contain 24 unique IDs; Rust checks exact set equality and the final capture verifier checks ordered IDs.
14. **A runtime lookup silently accepts an unlisted scene** — PASS. FFI route test rejects `zero-fixed-maximum-unlisted`; platform allowlists require exact scene IDs.
15. **Fixture implementation leaks into default FFI** — PASS. Default static archive marker scan is negative and fixture-enabled archive positive.
16. **Android fixture matcher stays compiled into default product** — PASS. Java array and route are gated by `SPINON_C04_RUNTIME_GPU_FIXTURE_TEST`; default APK scan is negative and fixture APK positive.
17. **iOS fixture matcher stays compiled into default product** — PASS. Swift scene set uses the fixture macro; default executable scan is negative and fixture executable positive.
18. **A broad run kind accidentally executes the whole catalog** — PASS. `zero-fixed-maximum` without an explicit scene list is rejected; the boundary suite verifies that guard.
19. **A positive fixed maximum also uses the zero floor** — PASS. Direct vendored Taffy positive controls use 2px maxima and assert 8/3 counts for gap 0/4.
20. **A limited fixture is promoted to full C11.4** — PASS. Plan, internal contract, status ledger, and evidence explicitly keep C11.4 unchecked and `72/213` unchanged.

Platform boundary: Android is API 37 ARM64 QEMU AVD; iOS is iPhone 17 Pro/iOS 26.2 Simulator. Metal samples are pre-compositor. Neither result proves a physical Android device or Android hardware GPU.
