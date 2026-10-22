# C11.4 overlapping span implementation review · 20 failure paths

**Review scope:** fixed scene allowlist, typed-style validation, text ownership, track-sizing assertions, native sample gates, and release feature boundary. This is a source-level failure review; it does not replace the runtime captures in [README](README.md).

| # | Failure path | Finding |
|---:|---|---|
| 1 | An arbitrary scene ID reaches fixture source generation | The FFI table resolves only the six fixed IDs; its registry test rejects an unlisted ID. Pass. |
| 2 | A typo under the new ID prefix falls through to ordinary text rendering | The runtime fixture treats unknown IDs with either overlapping-span prefix as unsupported. Pass. |
| 3 | Duplicate scene IDs make lookup order select a different scene | The FFI and runtime tests require six unique IDs. Pass. |
| 4 | Fixture entry leaks into a default release | Default and opt-in release archives were compared; only the opt-in archive contains the marker and six IDs. Pass. |
| 5 | Different columns or track grammar are accepted under the same ID | Runtime checks three typed `MinMax(Calc(10% + 2px), MaxContent)` columns in order. Pass. |
| 6 | Extra implicit columns, auto-repeat, named lines, or template areas change the case | Track count and the supported metadata fields are checked; mismatches fail closed. Pass. |
| 7 | Root box sizing, width, height, gap, or border changes expected outer geometry | Root dimensions, border, zero margin/padding, 2px column gap, and 10px row are checked against the selected case. Pass. |
| 8 | A non-root element or wrong HTML namespace claims the scene ID | Runtime root selection checks XHTML namespace, root parent, `div`, profile, and marker. Pass. |
| 9 | Root or item identity marker is absent or non-empty | The fixture requires the expected IDs and empty `data-node` attributes. Pass. |
| 10 | Child wrappers or nested text alter intrinsic ownership | Exactly three direct element children and one direct text node per item are required. Pass. |
| 11 | Text content changes while retaining expected frames | Exact Ahem strings are matched before paragraph layout. Pass. |
| 12 | Reordered DOM changes item placement but retains scene ID | Forward/reverse IDs encode exact order and numeric placement; the runtime checks each child in that order. Pass. |
| 13 | Span start/end or row placement differs from the fixed case | Each typed grid placement and computed span length is checked. Pass. |
| 14 | Text wraps or font metrics depend on host-installed fonts | Ahem 10px, nowrap-related styles and bundled Ahem font registration are fixed; font registration errors fail. Pass. |
| 15 | Text contribution uses min-width or authored size instead of measured intrinsic text | The test checks 70/80/140px min/max-content sources and each span contribution. Pass. |
| 16 | Same-span contributions accumulate in DOM order | Forward/reverse cases assert span batch order `[2,3]`, track sizing state, final columns, and identical frames. Pass. |
| 17 | Cyclic and definite widths accidentally share an unchecked path | Intrinsic max-content and definite 140px cases are separate; definite 100px is a constrained positive control. Pass. |
| 18 | Multi-owner glyph paint drops a text owner or duplicates its color entry | Shared rendering input checks non-empty owners, styles, handles, and duplicate owner IDs before building the color map. Pass. |
| 19 | Pixel readback samples the wrong scene/node/coordinate | Android and iOS require exactly the three expected sample IDs, target nodes, and `(4,7)`, `(126,7)`, `(150,7)` coordinates. Both 100px scenes pass 3/3 per platform. Pass. |
| 20 | Sample success is overstated as border-stroke or final-screen validation | Evidence separates red text outside the root from the clear-after-text control; it explicitly excludes border stroke and iOS post-compositor output. Pass. |

The six scenes are fixture-only. This review does not assert generic CSS Grid behavior, WPT conformance, physical-device output, or full C11.4 completion.
