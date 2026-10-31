# Plan failure review: C11.4 anonymous Grid text author `tab-size`

Reviewed plan: [`c11-4-anonymous-grid-text-pre-author-tab-size.md`](./c11-4-anonymous-grid-text-pre-author-tab-size.md) and its pinned input/oracle artifacts. These are 20 separate failure hypotheses; this plan review is not the later implementation/runtime review or a PR-diff review.

| # | Failure perspective | Review result |
|---:|---|---|
| 1 | The plan changes a public CSS or general HTML admission path by accident. | Scope fixes the route to the existing opt-in C11.4 anonymous-text fixture and forbids public API, general admission, and product-wide policy changes. **Pass.** |
| 2 | Existing initial `tab-size:8` behavior changes while adding author values. | Value 8 remains an explicit regression control; the prior default-tab slice stays in the matrix. **Pass.** |
| 3 | The implementation guesses a value from a scene ID or authored declaration. | Plan requires the computed-style snapshot, a validated typed value, and rejects inference from ID, source text, or declaration spelling. **Pass.** |
| 4 | A unitless number is interpreted as pixels instead of a multiple of U+0020 advance. | CSS Text §4.2 and the Ahem space-advance formula are pinned; cases 1/2/4 expect 10/20/40px stops. **Pass.** |
| 5 | Exact-stop tabs remain at the current stop rather than moving to the following stop. | Values 1, 2, and 4 each have an exact-stop scene; independent arithmetic and Chrome ranges agree. **Pass.** |
| 6 | A leading tab uses an unexpected origin or inherited ancestor offset. | Root and inline origin are fixed at x=0 with zero edges, and each tab-size has a leading control. Offset ancestors are excluded and must be rejected. **Pass.** |
| 7 | Consecutive tabs reuse the same stop or fail to advance in sequence. | The value-2 consecutive-tab control expects two sequential stop advances and a 70px line. **Pass.** |
| 8 | A forced line break carries the prior line's tab cursor into the next line. | LF and CRLF controls separately expect a reset to inline x=0; the CRLF browser observation preserves both source code units as one break. **Pass.** |
| 9 | A trailing tab is discarded as collapsible whitespace. | The `XX\t` control expects a preserved 40px final line at value 2. **Pass.** |
| 10 | `white-space:pre` is accidentally soft-wrapped for min-content sizing. | The min-content case expects min=max=60px and one 60px line. **Pass.** |
| 11 | A narrow fixed Grid track silently clips or wraps text instead of retaining overflow. | The 25px track control separately checks the 25px root/track and 60px unwrapped text advance. **Pass.** |
| 12 | A size-1 tab is treated as an empty tab or as a 1px interval. | The leading and exact-stop value-1 cases expect a 10px stop interval derived from the 10px space advance. **Pass.** |
| 13 | A size-4 tab ignores the computed value and keeps the old 8-stop result. | Leading and exact-stop value-4 controls distinguish the expected 40px interval from 80px. **Pass.** |
| 14 | Valid CSS zero, fractional, or length values are falsely claimed as supported by this bounded candidate. | The plan explicitly excludes these valid CSS forms from this profile and requires rejection without partial output. **Pass.** |
| 15 | Tab normalization loses the original tab's UTF-16 range or expanded paragraph range. | Runtime/unit acceptance requires one source code unit to map to the complete normalized tab expansion, with provenance preserved. **Pass.** |
| 16 | Tabs split across adjacent text sources restart the paragraph cursor. | Multi-source runtime tests must prove the tab cursor continues and source NodeIds remain distinct. **Pass.** |
| 17 | The inherited value is read from an unrelated node or a stale style snapshot. | Root/profile shape, computed property, paragraph owner, and revision are checked together; stale or absent style fails closed. **Pass.** |
| 18 | The new value leaks into `PreWrap`, `BreakSpaces`, or ordinary text paths. | Those modes are named regression boundaries and their tab behavior remains separate. **Pass.** |
| 19 | The oracle is mistaken for normative truth or for glyph/pixel evidence. | README separates CSS Text arithmetic from Chrome observation and explicitly limits Range/screenshot evidence to browser layout, not glyph pixels. **Pass.** |
| 20 | A fixture-only result is presented as C11.4 completion or enters the default artifact. | Plan forbids checkbox/count promotion and requires default-vs-opt-in archive marker, ABI, and payload checks. **Pass.** |

During review, the expected JSON was decoded to verify tab, LF, and CRLF inputs are actual control characters rather than literal backslash sequences. The plan and expected-value artifact were corrected before this review was finalized. Current Chrome observation records 13 cases with Ahem loaded at HeadlessChrome 156, DPR 1. **Plan review: 20/20 addressed.**
