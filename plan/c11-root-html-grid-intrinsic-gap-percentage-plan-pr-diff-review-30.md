# C11.1 intrinsic Grid percentage-gap plan PR: 30-point hostile review

This review targets the plan PR diff and its precomparison artifacts. It is separate from the plan's 20-item design review.

| # | Attack / invariant | Result |
|---:|---|---|
| 1 | The plan could be based on a pre-#1172 checkout. | Pass: the plan names the merged fixed-px gap predecessor and uses its current profile boundary. |
| 2 | The candidate could be filed under C11.4 although it changes the basic Root HTML profile. | Pass: it is classified as C11.1 Root HTML parsed runtime work. |
| 3 | The PR could change the official completion count by implying the candidate is done. | Pass: plan and review explicitly retain 78/214 and incomplete C11.1/C11. |
| 4 | The internal interface numeric version could drift with the added behavior. | Pass: internal contract 0059 remains 0.1.0. |
| 5 | The plan could promise a new public API, parser, fixture-only success path, or FFI route. | Pass: the scope adds none of them. |
| 6 | The plan could expand percentage support to Flex or every Grid profile. | Pass: only RootHtmlIntrinsicTrackSizingV1 is in scope. |
| 7 | The plan could use the wrong basis for one gap axis. | Pass: both percentage bases and the mixed 120×80 case are explicit. |
| 8 | The grid's outer box could be mistaken for its content box. | Pass: zero margin, padding, and border remain profile requirements. |
| 9 | The plan could conflate intrinsic track sizing with cyclic intrinsic container sizing. | Pass: definite fixed axes and cyclic-contribution exclusions are stated separately. |
| 10 | A cyclic percentage gap might be treated as a resolved content-box percentage during intrinsic contribution. | Pass: the normative cyclic-zero rule is documented and excluded from these noncyclic fixtures. |
| 11 | Percentage values could be clamped to 100% or rounded before track sizing. | Pass: the plan requires unclamped direct percentages and includes 120% scenes. |
| 12 | A zero percentage could produce a nonzero gutter or different control geometry. | Pass: separate 0% and no-gap controls are included. |
| 13 | grid-gap and gap shorthand could differ in row/column ordering. | Pass: the mixed-basis cases cover the shorthand and legacy alias using expected row/column values. |
| 14 | CSS-invalid negative declarations could be reported as unsupported computed CSS. | Pass: cascade fallback is distinguished from typed invalid-value diagnostics. |
| 15 | A typed NaN, infinity, or negative percentage could enter Taffy. | Pass: validation and property-specific rejection are required before layout. |
| 16 | Percentage multiplication or aggregate gutters could overflow f32. | Pass: the plan requires wider intermediates and finite-range validation. |
| 17 | Track plus gutter could overflow after track sizing despite a finite gap. | Pass: finite track-line positions are checked before frame collection. |
| 18 | An overflow scene could be rejected only because it extends beyond the root. | Pass: finite visible overflow is a positive result and is not clamped. |
| 19 | The new profile could accidentally gain implicit tracks, auto-repeat, named areas, dense flow, or column flow. | Pass: each remains outside this slice and remains a regression boundary. |
| 20 | Chrome data could serve as both expected and actual, hiding an oracle mismatch. | Pass: independent expected values, raw observation, and comparison report are separate. |
| 21 | The fixture hash could be absent or refer to a different fixture than the observation. | Pass: expected.json records the exact fixture path and SHA-256. |
| 22 | The nine-scene comparison could ignore one scene or one geometry axis. | Pass: comparison.json checks every scene's root, computed gaps, both track lists, and all item rectangles. |
| 23 | Computed percentage strings could be misreported as resolved pixel gaps. | Pass: README separates computed syntax from used gaps inferred from tracks and positions. |
| 24 | Browser version or viewport could be missing, making the observation irreproducible. | Pass: the observation and expected manifest record HeadlessChrome 156.0.0.0 and 1280×720. |
| 25 | Browser evidence could be mistaken for a Spinon runtime result. | Pass: precomparison README labels the artifacts as reference-only and explicitly excludes Spinon runtime. |
| 26 | Browser screenshot colors could be presented as pixel-perfect implementation evidence. | Pass: the plan does not call this image a Spinon pixel or simulator result. |
| 27 | A simulator draw screenshot could be called PixelCopy or Metal drawable evidence without readback. | Pass: later validation requires explicit readback boundaries and keeps screenshots as visual smoke. |
| 28 | A local Chrome oracle could be mislabeled as Spinon WPT conformance. | Pass: WPT conformance is explicitly excluded. |
| 29 | Layout errors could publish partial frames from other Grid owners. | Pass: atomic no-partial-output behavior is an acceptance requirement. |
| 30 | Post-merge records or Tailnet output could claim more than the implemented slice. | Pass: status, contract, public roadmap, exact merge SHA, preview verification, and browser-process cleanup are bounded to the confirmed result. |

The review does not approve implementation by itself. Product code changes require the post-merge plan gate and new implementation/runtime and PR-diff reviews.
