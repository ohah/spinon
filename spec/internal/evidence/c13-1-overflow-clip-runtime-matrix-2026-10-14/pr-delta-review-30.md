# C13.1 runtime matrix PR delta review · 30 perspectives · 2026-10-14

Scope: only the current runtime matrix delta in PR #158, including renderer logic, tests, evidence, contract, plan, status, and PR summary. The implementation failure review is recorded separately in [implementation-review-20.md](implementation-review-20.md).

| # | PR delta perspective | Review result |
| --- | --- | --- |
| 1 | Product code scope | Change is confined to C13.1 overflow hit clipping in `overflow_clip.rs`; no CSS parser or public-support expansion. |
| 2 | Paint behavior scope | Paint interval construction is unchanged; the new boolean is consumed only by hit-bound selection. |
| 3 | In-flow compatibility | Existing positioned clip/in-flow border-strip case remains in the prior three-row matrix and passes. |
| 4 | Contained absolute behavior | New row uses positioned ancestor and absolute target whose containing block is that ancestor; visual edge is asserted. |
| 5 | Escaped absolute behavior | Existing containing-block escape behavior is retained and receives a Chrome frame/hit-range case. |
| 6 | Nested ancestor behavior | Nested clip row checks both ancestor intersections against raw Chrome evidence. |
| 7 | Sibling bleed | Implementation walks only the current target's lineage; unrelated sibling descendants cannot set the flag. |
| 8 | Fixed-position behavior | Fixed branch is not changed; review does not claim new fixed-position coverage. |
| 9 | Axis-specific semantics | The new matrix covers visible, x-only, both-axis, and mixed-axis cases; the prior y-only row remains. |
| 10 | Mixed-axis normalization | Chrome computed `auto`/`hidden` and `hidden`/`auto` rows are pinned and the test checks their observed hit ranges. |
| 11 | `overflow-clip-margin` behavior | Padding-box and content-box new rows complement existing border-box and hidden-ignore rows. |
| 12 | Content-box regression provenance | The old contradictory test now embeds a dedicated Chrome observation rather than retaining a guessed border union. |
| 13 | Positive control | x=119 is included in the captured target hit range and asserted as target. |
| 14 | Negative control | x=122 is outside the Chrome visual edge and asserted not to hit the target. |
| 15 | Raw oracle immutability signal | Chrome version/revision, UA, viewport, repository fixture hash, and baseline observation hash are asserted in tests. |
| 16 | Duplicate expected-value drift | Tests validate the captured JSON against per-case constants before comparing renderer output. |
| 17 | Observation completeness | Ten new rows include target frame, computed overflow values, clip margin, and x/y elementFromPoint ranges. |
| 18 | Test sweep completeness | Runtime checks sweep all target integer coordinates on both axes; documentation avoids calling it a full two-dimensional sweep. |
| 19 | Snapshot geometry regression | Every new row asserts frame origin and size with a 0.5 CSS px tolerance. |
| 20 | Paint/hit distinction | New helper checks paint and hit clip intervals separately; no combined assertion hides divergence. |
| 21 | Negative-coordinate / viewport boundary impact | This delta does not touch viewport logic; prior null controls remain separate evidence. |
| 22 | Allocation/error paths | No new allocation or error path is introduced by the boolean; existing fallible ancestry reservation remains unchanged. |
| 23 | Feature/API bleed | No Cargo feature, JNI, FFI symbol, public API, or application fixture flag changes are present in this delta. |
| 24 | Build reproducibility | Android command and pinned NDK use are preserved in the build log; Gradle SDK compatibility warning is recorded. |
| 25 | Android target provenance | `adb devices -l` showed the API 37 ARM64 emulator only; no physical device result is claimed. |
| 26 | Android result scope | Current AVD run proves existing 13-node route regression only, not the new contained-absolute case. |
| 27 | Android pixel evidence | PixelCopy log records 11/11 expected samples and image hash; samples are existing fixture only. |
| 28 | Stale log records | `android-route.log` is filtered to the current cold-start PID 8712; the successful hit and PixelCopy records share run ID `179b65d965f7`. |
| 29 | Documentation and completion state | Contract, roadmap, and status add this evidence while leaving C13/C13.1 incomplete and listing remaining gates. |
| 30 | PR lifecycle and claim control | PR #158 remains open and draft; no merge or broad CSS/WPT/DOM-event completion claim is added. |

The recorded verification is 8 matrix tests, 52 render crate tests (1 ignored), and focused Clippy, plus the existing Android route's 15/15 hit and 11/11 PixelCopy regression. WPT and new-case AVD/DOM event coverage remain open.
