# C11.4 computed `tab-size` full values implementation/runtime review · 20 failure paths

**Review scope:** candidate diff on `feat/c11-4-tab-size-full-values`, source HEAD `e3ec8a24b2ff463519dc79968f4cbba35d1226e3`, plus focused Rust/native checks and the Android/iOS runtime matrices linked from the [evidence README](README.md). This reviews the limited opt-in anonymous Grid text profile only.

| # | Failure path examined | Evidence and disposition |
| --- | --- | --- |
| 1 | Existing `white-space:pre` tab behavior regresses when the enum changes | `Pre` retains its eight-space path; the new anchor path is selected only by `PreWithTabSize`. Existing focused text tests passed. |
| 2 | Representative screenshot is silently omitted by a stale scene identifier | Found: the capture ID lacked the generated `anonymous-pre-tab-value-` prefix. Corrected the runner and reran that scene on Android and iOS; both produced a PNG and passed runtime validation. |
| 3 | Scene ID supplies the runtime `tab-size` instead of the computed style | Runtime validation reads the computed-style snapshot and rejects a mismatch with the fixture expectation. The runtime negative tests cover mismatched and missing properties. |
| 4 | Unitless number is confused with a CSS-pixel length | `parse_computed_tab_size` preserves `Number` versus `LengthPx`; the full number and length scene groups both pass. |
| 5 | Authored `em` or `calc()` is parsed before cascade resolution | FFI supplies authored declarations; runtime consumes the computed absolute value. The `1.5em` and `calc(2px + 3px)` scenes pass using their computed values. |
| 6 | Negative, non-finite, malformed, or absent values publish partial layout | Style parsing and `ParagraphStyle::validate` reject invalid values before a layout is returned; focused runtime/text tests cover missing and invalid input. |
| 7 | Finite input overflows while computing interval or stop coordinates | `pre_tab_advance` checks interval, position, next stop, and resulting width for finiteness; a `f32::MAX` regression test expects an error. |
| 8 | `tab-size:0` creates a visible space or positive inline box | Zero advances are retained as zero-width anchors and skipped when inline boxes are built; zero number/length scenes pass. |
| 9 | A tab exactly on a stop uses that same stop rather than the next strict stop | The stop index is `floor(position / interval) + 1`; exact-stop cases exist in the 140-scene matrix for every value. |
| 10 | A tab less than `0.5ch` from the next stop misses the following-stop adjustment | `pre_tab_advance` applies the half-space threshold; focused tests cover the fallback and the browser reference matrix covers it across values. |
| 11 | Fractional number/length advances are rounded to whole spaces | Tabs use fractional inline-box widths; `0.3`, `0.25`, `3.25`, and `0.3px` cases pass expected line-advance checks on both platforms. |
| 12 | A consecutive tab run recalculates every tab from current position | The first tab in a run uses the current stop; later tabs add one interval. Consecutive cases and focused text tests pass. |
| 13 | A normal character fails to terminate a consecutive-tab run | The normal character path clears `in_pre_tab_run`; interrupted-run tests pass. |
| 14 | LF does not reset the tab origin | The segment-break path resets inline position and run state; LF scenes pass on both platforms. |
| 15 | CRLF creates a duplicate newline or corrupts source ranges | Normalization emits one newline range for CRLF while retaining each source code unit; focused provenance tests and both runtime validators pass. |
| 16 | Removing the tab from normalized text loses source-node ownership | The tab's source range remains with an empty normalized range, while the inline advance is recorded separately. Joined-source provenance tests pass. |
| 17 | A trailing tab is dropped because it has no following glyph | The anchor becomes an end-edge inline box; trailing-tab cases pass with their expected line advance. |
| 18 | Intrinsic min-content wraps text despite `white-space:pre` | The `PreWithTabSize` path measures unwrapped lines; min-content and max-content cases pass with the expected contribution. |
| 19 | A fixed 25px Grid track clips or wraps the 45px text advance | Fixed-overflow scenes check the 25px root/item frame, 45px text line advance, track trace, draw/present, and zero retries on both platforms. |
| 20 | Native validators accept only frame geometry while ignoring style, source, track, or app identity | Android/iOS checks validate computed value, normalized source ranges, line advances, frame, track contributions, complete trace, draw/present, and installed app hash. Each 140-scene matrix passes; screenshots are explicitly limited to route/present evidence. |

**Disposition:** the screenshot identifier issue was corrected and verified with one-scene Android/iOS reruns. No additional blocking defect was found within this profile. The review does not prove glyph rasterization, general CSS admission, physical-device/hardware-GPU behavior, WPT conformance, or full C11.4/C11. A separate implementation PR diff review remains required before merge.
