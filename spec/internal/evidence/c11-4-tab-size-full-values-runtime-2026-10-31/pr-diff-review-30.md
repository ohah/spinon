# C11.4 full computed `tab-size` candidate PR diff review · 30 failure paths

**Review scope:** staged candidate diff on `feat/c11-4-tab-size-full-values`, source HEAD `e3ec8a24b2ff463519dc79968f4cbba35d1226e3`, including Rust text/runtime/FFI, Android and iOS validators, runner, plans, contract, status, and simulator evidence. This is separate from the [implementation/runtime review](implementation-runtime-review-20.md).

| # | Diff failure path examined | Finding |
| --- | --- | --- |
| 1 | New fixture behavior leaks into a public CSS/API surface | No public `@spinon/*` API or general CSS admission is added; the change stays in the opt-in C11.4 fixture. |
| 2 | Default `TabSize` changes the previous initial value | `Default` remains unitless 8, preserving the existing initial fixture path. |
| 3 | Floating values silently lose numeric type distinction | `Number` and `LengthPx` stay distinct through runtime parsing and the text-layout model. |
| 4 | A stale enum comparison or equality assumption remains after `f32` adoption | Relevant derives/call sites compile; comparisons use `PartialEq` and finite-value validation. |
| 5 | A computed value is inferred from scene text or an authored declaration | Runtime consumes `tab-size` from the computed-style snapshot and compares it with a fixed fixture expectation. |
| 6 | `em`/`calc()` bypass computed-style resolution | The fixture supplies authored CSS while runtime parses computed absolute values; both scenes are in the 140-case matrices. |
| 7 | Invalid negative or non-finite values reach layout | Runtime and paragraph-style gates reject them; focused negative tests cover missing, mismatched, malformed, and non-finite values. |
| 8 | Arbitrary scene prefixes accidentally enter this fixture route | Full-value scene recognition requires both a known value ID and one of ten known case IDs; unknown IDs are rejected. |
| 9 | Scene generation produces duplicate or missing cases | The FFI generator test checks uniqueness and all 140 scene IDs; the captured manifest reports exactly 140. |
| 10 | FFI authored input differs from the expected CSS contract | Generated runtime source includes the exact declared value, `white-space-collapse:preserve`, `text-wrap-mode:nowrap`, Ahem 10px, and fixed Grid dimensions. |
| 11 | Test oracle accidentally reads its expected result from the runtime output | The runner reads the pinned expected-values JSON and compares each observed computed value and geometry against it. |
| 12 | Source tab deletion loses the original UTF-16 source range | Normalization keeps an empty paragraph range for each source tab; Rust and native report validators inspect source ownership and offsets. |
| 13 | Inline tab anchors are ordered on the wrong text edge | Start/end edges are selected from the normalized text boundary; leading, interior, and trailing scenes pass. |
| 14 | Multiple tabs at one normalized text index reorder or collapse | Stable event sorting preserves same-index anchor order; consecutive-tab scenes pass on both platforms. |
| 15 | Zero-width tab still creates a positive inline box | Inline-box construction skips zero width; zero-number and zero-length cases pass. |
| 16 | Exact tab stop advances by zero | Stop selection uses the next strict stop; every exact-stop scene is checked. |
| 17 | The half-space fallback is missing or applied more than once per run | The implementation applies one next-stop fallback, then the run path adds one base interval per following tab; focused and matrix cases pass. |
| 18 | Fractional number/length widths are rounded away | Inline advances remain fractional through normalization and are checked at runtime, including `0.3` and `0.3px`. |
| 19 | A normal character does not reset tab-run state | The `X`/space path resets `in_pre_tab_run`; an interrupted-run test exercises it. |
| 20 | LF or CRLF leaves the previous line's tab position active | Both newline paths reset position and run state; CRLF source offsets are checked separately. |
| 21 | Intrinsic sizing reintroduces soft wrapping for preserved text | `PreWithTabSize` measures without a finite wrap width; min-content/max-content cases pass. |
| 22 | Tab-only and trailing-tab edges create a false anonymous Grid item or drop width | Tab-only controls remain outside the 140 positive scene set; trailing tabs are checked in an existing text item. |
| 23 | Fixed overflow is reported as an expanded track instead of overflowing text | The fixed scene checks a 25px root/track while retaining its independently expected line advance. |
| 24 | Android fixture validator accepts layout without validating its trace | Android checks computed style, source mapping, text layout, frame, track pass/contributions, complete trace, and draw/present. |
| 25 | iOS fixture validator applies different expected geometry or normalization | Swift uses the same fixed 14×10 inputs and independently checks lines, frames, source ranges, and track trace. |
| 26 | iOS unified-log escaping corrupts runtime JSON validation | The collector normalizes the captured `\134` escape before decoding; the complete 140-scene iOS matrix passed. |
| 27 | A screenshot is absent despite a successful runtime matrix | Review found and fixed a representative scene ID mismatch; a separate one-scene Android/iOS rerun produced both PNGs. |
| 28 | The screenshot is presented as proof of glyph ink or compositor output | README labels the blank canvas captures as fixture-route/present evidence only and explicitly excludes glyph pixel equality. |
| 29 | Candidate fixture symbols enter the default product archive | Default static archive has no full-tab scene/fixture markers; the opt-in archive contains the fixture symbol and markers. Boundary tests pass 24/24. |
| 30 | Docs or evidence overstate C11 completion, simulator coverage, or CI | Plan, contract, and `spec/STATUS.md` keep the item as a candidate, identify simulator limits, and retain official `72/213`; no CI success or merge is claimed. |

**Disposition:** no blocking diff issue remains in the scoped candidate. The capture ID defect is corrected and the rerun evidence is linked. The separate full run's captured Android/iOS binaries match installed hashes. The 140-scene matrices, fixture boundary, `cargo fmt`, `git diff --cached --check`, and runner syntax check passed. The accidental extra matrix attempt is retained but explicitly excluded from validation. This review does not replace GitHub CI or merge-state verification.
