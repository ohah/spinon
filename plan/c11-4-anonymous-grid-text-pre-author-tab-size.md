# C11.4 anonymous Grid text `white-space: pre` numeric author `tab-size`

**Parent:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [internal Grid contract 0059](../spec/internal/0059-c11-grid-layout.md) · [inline text contract 0070](../spec/internal/0070-c15-1-inline-text-layout.md) · [CSS Text Level 3](https://www.w3.org/TR/css-text-3/#tab-size-property)

**Status:** Plan failure review 20/20 complete; implementation has not started. The preceding [default tab-size slice](./c11-4-anonymous-grid-text-pre-tabs.md) implements only the inherited initial value 8 and rejects computed `tab-size:2`. This plan considers a bounded numeric author-value extension for that same opt-in anonymous Grid `pre` fixture. It does not change the C11.4 checkbox or official `72/213` count. [Plan review](./c11-4-anonymous-grid-text-pre-author-tab-size-plan-review-20.md).

## Problem and goal

The existing text normalizer expands preserved tabs at eight-space stops. The C11.4 runtime style snapshot already exposes the computed `tab-size` serialization, but the text-layout path does not consume it. A fixed negative scene proves that the current opt-in route rejects `tab-size:2`. The candidate should pass the computed numeric value to the tab expansion algorithm and preserve the existing default-eight behavior.

The candidate is limited to one direct anonymous Grid text paragraph, stored Ahem 10px/10px LTR, `white-space:pre`, zero letter/word spacing, and numeric `tab-size` values 1, 2, and 4. The fixed values exercise one-space stops, ordinary stops, and a larger interval while keeping each tab expansion expressible as whole Ahem spaces. The inherited initial value 8 remains the existing control. Other valid CSS values remain outside this fixture profile and must fail closed.

## Normative reference, oracle, and expected values

CSS Text Level 3 §4.2 defines `tab-size` as inherited, with initial value 8; a unitless number multiplies the U+0020 advance. CSS Text Level 3 §4.1.2 defines preserved tab processing and its inline-content-relative stops. A forced segment break starts the next line's tab position from its inline content start. The checked-in [Chrome 156 precomparison](../spec/internal/evidence/c11-4-anonymous-pre-author-tab-size-precomparison-2026-10-31/README.md) records the browser observation separately from independent expectations.

The stored Ahem font at 10px has a 10px space advance. Therefore a numeric value `N` gives a `10N` CSS-pixel interval. At an exact stop, a tab advances to the following stop. The expected cases cover sizes 1, 2, and 4; leading, exact-stop, consecutive, LF/CRLF line-reset, trailing, min-content, and fixed-track overflow behavior. The detailed expected rows and per-line advances are in [`expected-values.json`](../spec/internal/evidence/c11-4-anonymous-pre-author-tab-size-precomparison-2026-10-31/expected-values.json). Existing value-8 behavior stays as a regression control.

## Candidate contract and scope

- Route remains the opt-in `RuntimeBlockFlexGridTrackSizingV1` C11.4 anonymous-text fixture. No general HTML admission, public CSS API, CSS declaration policy, or product-wide text behavior is added.
- The root is a zero-edge horizontal-tb/LTR Grid with a single `pre` anonymous text item and fixed Ahem style. The line-start offset, letter spacing, word spacing, and text origin are zero.
- The runtime collector must read the computed `tab-size` from the computed-style snapshot, validate it against the candidate profile, and pass a typed tab-stop value into the text-layout input. It must not infer the value from a scene ID, source text, or authored declaration string.
- The candidate supports computed numeric values 1, 2, and 4, plus the existing computed initial value 8. Computed zero, fractional values, lengths, values outside the profile, absent/stale computed style, and unsupported roots are rejected without partial paragraph, track, frame, or draw results.
- Tab expansion uses the current inline advance within the paragraph and resets only at a preserved segment break. Across multiple `TextSource` nodes, tab position continues while each source node and UTF-16 provenance remains intact. A source tab maps to the full normalized space range while retaining its one-code-unit source range.
- The implementation must preserve all existing `Normal`, `NoWrap`, `PreLine`, `PreWrap`, `BreakSpaces`, and default-eight `Pre` behavior. `PreWrap` remains governed by its existing separate tab profile.
- Update internal contracts 0059/0070 only if the implementation changes their described input or ownership boundary; keep version `0.1.0` unless a semantic contract version change is required.

## Verification plan

1. Freeze the fixture, independent expected values, browser observations, Ahem resource hash, and source hashes. The 20-perspective plan review must pass before implementation begins.
2. Add focused Rust coverage for values 1, 2, and 4, default 8 regression, exact-stop behavior, LF and CRLF line reset, trailing and consecutive tabs, min-content non-wrapping, narrow-track overflow, and source-range/node provenance across joined sources.
3. Extend the opt-in C11.4 anonymous-text collector and fixed scene validator. Positive cases must compare computed style, typed value, normalized paragraph, per-line advance, intrinsic contributions, track trace, root/item frame, revision, draw/present, and retry count. Negative values and structure changes must fail before partial output.
4. Run focused text-layout, HostDocument/layout, runtime and FFI tests; CSS fixture module-boundary checks; relevant Clippy and formatting. Record initial and corrected failures separately. Do not claim unrelated workspace tests passed if the known C07.3 computed-string precision mismatch remains.
5. Run the new scenes and the existing anonymous-text regression matrix on Android API 37 ARM64 AVD and iPhone 17 Pro/iOS 26.2 Simulator. Verify the app hashes match the tested source build, and distinguish Android software rendering and pre-compositor iOS samples from physical-device or post-compositor proof. Attach screenshots to the implementation PR when available.
6. Verify the fixture ABI, scene marker, and payload are absent from default product artifacts and present only in the opt-in fixture-test build. Recheck the `tab-size:2` rejection control becomes a positive only after the computed style and typed input agree.
7. Record implementation/runtime review from 20 distinct failure perspectives and a separate final implementation PR review from 30 distinct diff failure perspectives. Keep both separate from this plan review and the plan PR's 30-perspective diff review.

## Explicit exclusions

`<length>` values, zero, fractional numbers, arbitrary numeric ranges, nonzero spacing, offset inline origins, bidi, orthogonal writing modes, arbitrary fonts/text/scripts, inline formatting contexts, tabs under `PreWrap`/`BreakSpaces`, general CSS cascade admission, root HTML parsing, glyph ink pixel equality, full WPT, physical hardware validation, and completion of C11.4/C11 are not included. The 12 fixed browser cases are evidence for this candidate only.
