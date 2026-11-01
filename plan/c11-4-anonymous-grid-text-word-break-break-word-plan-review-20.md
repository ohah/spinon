# C11.4 anonymous Grid text `word-break: break-word` plan review · 20 perspectives

**Scope:** review the proposed contract, independent oracle, implementation boundary, and verification sequence before product changes. This does not review implementation code or count as a PR diff review.

**Inputs:** [implementation plan](./c11-4-anonymous-grid-text-word-break-break-word.md), [precomparison README](../spec/internal/evidence/c11-4-anonymous-grid-text-word-break-break-word-precomparison-2026-11-01/README.md), [independent values](../spec/internal/evidence/c11-4-anonymous-grid-text-word-break-break-word-precomparison-2026-11-01/expected-values.json), Chrome 156 observation and verifier, current text-layout model/engine, anonymous-text runtime adapter, fixture FFI source generator, project rules, and contract 0059.

## Review

| # | Failure perspective | Adversarial check and finding | Result |
| ---: | --- | --- | --- |
| 1 | Normative property mapping | Could the plan accidentally treat `word-break: break-word` as the same property value as `overflow-wrap: break-word`? CSS Text 3 §5.1 gives it the effect of `normal` plus effective `anywhere`, independently of actual `overflow-wrap`; the plan states this explicitly. | Pass |
| 2 | Actual-value cross product | Could one of the six computed pairs be omitted or silently normalized? The fixed oracle covers `normal|break-word` crossed with `normal|break-word|anywhere`; the runtime allowlist is specified as 18 scenes. | Pass |
| 3 | Min-content distinction | Could `normal + overflow-wrap: break-word` incorrectly inherit the `anywhere` min-content reduction? The plan requires 40px for that pair and 10px only for `anywhere` or effective `word-break: break-word`. | Pass |
| 4 | Max-content invariance | Could emergency breaks shrink max-content? The pinned Ahem input has a 40px unbroken advance, and the plan requires 40px max-content in every pair. | Pass |
| 5 | Fixed-track line layout | Could intrinsic sizing pass while final layout still overflows or emits the wrong line count? The 10px track control distinguishes one overflowing 40px line from four 10px lines for every pair. | Pass |
| 6 | Font nondeterminism | Could a fallback font change advances and invalidate the independent values? The fixture pins Ahem, size, weight, line height, and records the fixture font SHA-256. | Pass |
| 7 | Independent oracle bias | Could the expected values merely copy Chrome output? The expected matrix is derived from CSS Text/Sizing/Grid rules and fixed Ahem advances; raw Chrome measurements are kept as cross-check evidence. | Pass |
| 8 | Observation completeness | The first verifier checked row count but did not reject a duplicate replacing a missing scene. Added an exact unique 18-ID set check and verified that every ID matches its computed pair and track; the corrected verifier passes. | Fixed; pass after rerun |
| 9 | CSS value provenance | Could scene IDs manufacture the CSS inputs instead of validating computed values? The implementation plan requires reading both properties from the exact computed-style snapshot and using scene IDs only to select expected values. | Pass |
| 10 | Missing or unexpected computed style | Could absent, unsupported, or mismatched values fall through to defaults? The plan requires rejection of missing/mismatched properties, unlisted scenes, and unsupported profile inputs before publication. | Pass |
| 11 | Intrinsic versus final layout mapping | Could the effective override be applied only during min-content measurement or only during final wrapping? The plan requires effective `anywhere` for both while retaining the actual property value separately. | Pass |
| 12 | Existing `overflow-wrap: break-word` semantics | Could the new `WordBreakMode::BreakWord` make ordinary `overflow-wrap: break-word` reduce min-content? The plan preserves the existing 40px min-content behavior unless computed `word-break` itself is `break-word`. | Pass |
| 13 | Parley mapping | Could the new model value be passed as an unsupported Parley word-break value or be confused with `break-all`? The plan maps it to Parley `WordBreak::Normal` and represents the effective emergency opportunities through `OverflowWrap::Anywhere`. | Pass |
| 14 | Diagnostic and snapshot provenance | Could computed `overflow-wrap` be overwritten so diagnostics falsely report `anywhere`? The plan says to retain the actual `ParagraphStyle.overflow_wrap` while applying an effective layout override. | Pass |
| 15 | Source and owner identity | Could text, computed style, and Grid owner come from different revisions? The plan requires a single exact owner snapshot, matching source and revision, and rejects stale snapshots before publication. | Pass |
| 16 | Anonymous-text shape | Could an inline descendant, duplicate Text node, or altered string accidentally exercise a different shaping path? The plan fixes one direct anonymous Text node with `XXXX` and explicitly rejects changed/duplicate input. | Pass |
| 17 | Geometry and line-height controls | Could the root row height pass despite different line breaks or inherited edges? The fixed input removes gaps/edges, fixes horizontal-tb/LTR and 10px line height, and checks track, row, and text lines. | Pass |
| 18 | Regression surface | Could the change regress existing `word-break` or `overflow-wrap` scenes while the new matrix passes? The plan keeps the existing 12-scene and 9-scene matrices as regression gates and calls for focused text-layout/FFI checks. | Pass |
| 19 | Fixture isolation | Could the new scene generator or ABI leak into default product artifacts? The plan confines changes to the existing opt-in fixture route and requires default-artifact exclusion plus fixture positive-control checks during implementation. | Pass |
| 20 | Verification and claim boundary | Could simulator evidence be overstated as physical-device, GPU, WPT, or full C11 completion? The sequence distinguishes Android AVD/iOS Simulator from physical hardware, requires source/app hashes and retained evidence, and keeps C11.4/C11 plus official `72/213` unchanged. | Pass |

## Decision

One oracle-verifier gap was found and fixed before implementation: duplicate rows could previously conceal a missing scene. The corrected exact-ID verifier passes. No remaining plan-level blocker was found; the implementation gate is ready subject to preserving the listed intrinsic/final-layout distinction, computed-style provenance, and fixture-only artifact boundary. Implementation/runtime review and the separate PR diff review remain required after code changes; this plan review is not either of those reviews.
