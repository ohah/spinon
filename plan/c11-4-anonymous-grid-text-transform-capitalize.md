# C11.4 anonymous Grid text computed `text-transform:capitalize`

## State and scope

This is a bounded, opt-in runtime fixture profile for anonymous direct Text Grid items. It adds the computed `capitalize` value to the existing `none|uppercase|lowercase` text-transform fixture path. It does not add general CSS admission, WPT support, or mark C11.4/C11 complete. Internal contract `0059` remains version `0.1.0`.

CSS Text Level 3 defines `capitalize` by changing the first typographic letter unit in each word to titlecase when it is lowercase, while leaving the other characters unchanged. Word boundaries are UA-dependent. This profile fixes the boundary to U+0020 and limits input to direct `lang="en"` text containing ASCII letters and U+0020 only. Punctuation, non-ASCII text, other whitespace, inherited or missing language, and inline runs fail closed. This profile does not claim general English word segmentation.

The fixed Noto Sans KR 10px/400 Chrome 156 oracle and independent advances are recorded in [the precomparison evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-precomparison-2026-11-01/README.md). Its six scenes cover `none` min/max controls, `capitalize` min/max for `iiii ii`, mixed case `iIII ii`, and repeated-space collapse `ii   ii`. Expected shaping strings and track sizes are in that evidence's `expected.json`; CSSOM track comparison tolerance is 0.03125 CSS px.

## Implementation contract

- Add a `Capitalize` mode to text layout and retain the computed mode in the layout result.
- For this mode only, validate the complete direct source as ASCII letters and U+0020 before layout. Map the first lowercase ASCII letter after the start or a U+0020 to uppercase. Preserve all other ASCII letters as authored; do not lowercase the rest of a word.
- Keep HostDocument Text content and UTF-16/source ranges unchanged. Apply the transform only to the normalized layout buffer, without changing its character count or source mapping. Existing whitespace collapse remains responsible for shaping repeated spaces.
- Accept `capitalize` only for the exact fixture owner, exact computed style snapshot/document/layout revision, direct `lang="en"`, and scene-specific computed value. A scene ID never substitutes for the computed value. Reject unsupported values and any owner, revision, language, input, or source-shape mismatch before publishing a layout.
- Add positive and negative coverage at text-layout, runtime fixture, and FFI dispatcher/boundary levels. Fixture-only scene markers and payload must remain absent from the default product archive and present in the opt-in test archive.
- Update internal contract `0059`, the C11 plan, official status notes, and the public roadmap to describe only the bounded slice. Keep C11/C11.4 unchecked and official completion at 72/213.

## Verification gates

1. Run text-layout unit tests for the six source/mode behaviors, source-range identity, repeated-space mapping, and rejection of non-ASCII/unsupported characters or inline runs.
2. Run focused runtime fixture tests for all six Chrome scenes and negative cases for missing/wrong/inherited language, wrong computed value, scene/owner mismatch, and stale style revisions. Confirm rejection happens before layout publication.
3. Run FFI dispatcher and fixture-boundary tests, including default-versus-opt-in archive marker checks.
4. Run focused Rust tests, feature-scoped Clippy, formatting, Android API 37 ARM64 AVD, and iPhone 17 Pro/iOS 26.2 Simulator matrices on the same source revision. Report simulator geometry separately from glyph paint and surface pixels; this slice has no glyph-pixel claim.
5. Complete a distinct 20-view implementation/runtime review and a distinct 30-view PR diff review under the project rules. Review and fix findings before merge. Rebase and merge only after required checks and stated evidence are verified; update the Tailnet roadmap and close Agent Browser processes after any browser review.

## Explicit exclusions

Unicode casing and case expansion, punctuation-based or locale-specific word boundaries, UAX #29 conformance, language inheritance, inline formatting, arbitrary CSS author stylesheets, other `text-transform` values, general WPT conformance, glyph-pixel parity, physical-device/hardware-GPU claims, and completion of C11.4/C11.
