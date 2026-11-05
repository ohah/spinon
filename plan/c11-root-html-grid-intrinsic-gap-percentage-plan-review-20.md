# C11.1 intrinsic Grid percentage-gap plan: 20 failure perspectives

This review checks the plan before product implementation. It does not count as implementation/runtime review or PR diff review.

| # | Failure path or invariant | Result |
|---:|---|---|
| 1 | A bounded percentage-gap slice is used to mark C11.1 or C11 complete. | Pass: the plan keeps both checkboxes incomplete and retains official 78/214. |
| 2 | A percentage from an unrelated Grid or Flex profile is admitted by a shared gap helper. | Pass: admission is scoped to RootHtmlIntrinsicTrackSizingV1; other profiles and Flex remain unchanged. |
| 3 | row-gap accidentally uses the inline width basis or column-gap uses block height. | Pass: the contract binds row-gap to definite content height and column-gap to definite content width, with distinct 120×80 evidence. |
| 4 | Border, padding, or margin makes the root dimension differ from its percentage content-box basis. | Pass: this profile retains zero box edges, so the definite fixed root axis equals its content-box size. |
| 5 | The intrinsic word in the feature name is mistaken for an auto-sized, cyclic grid container. | Pass: both root axes remain fixed and definite; cyclic percentage contributions and auto axes are explicit exclusions. |
| 6 | A CSS-valid percentage gap above 100% is clamped or silently rejected. | Pass: 120% is a positive case; its 120px gutter and visible fixed-item overflow are preserved. |
| 7 | 0% changes the existing zero-gap track result. | Pass: a separate 0% control matches the no-gap geometry. |
| 8 | normal is assigned an unintended nonzero Grid gap. | Pass: the plan requires preserving existing normal/zero behavior. |
| 9 | The shorthand's first and second values are reversed. | Pass: mixed-axis cases explicitly require row 25% and column 10%. |
| 10 | Legacy grid-gap is routed through a different percentage basis path. | Pass: alias, shorthand, and longhands must resolve through the same computed longhand values. |
| 11 | A gutter is counted before the first or after the last explicit track. | Pass: aggregate count is explicit track count minus one. |
| 12 | A fractional percentage representation is rounded to whole percentages or pixels. | Pass: typed percentage is preserved and used pixels are compared as CSS-pixel values. |
| 13 | Multiplication overflows f32 before a safe range check can run. | Pass: basis multiplication and gutter aggregation use wider intermediate arithmetic and reject unrepresentable outputs. |
| 14 | Individually finite gutters combine into an unrepresentable aggregate. | Pass: the plan checks aggregate gutter extent before layout publication. |
| 15 | A finite track plus gutter produces infinite track-line positions. | Pass: existing track-line validation is extended to the resolved percentage gap before frame collection. |
| 16 | Unsupported calc or a synthetic negative/non-finite percentage becomes zero and reports success. | Pass: unsupported or invalid values produce property-specific failure; CSS-invalid declaration fallback stays at the cascade layer. |
| 17 | A rejected root leaves frames from another root in the same output. | Pass: all Grid layout output is required to fail atomically without partial frames or paint. |
| 18 | Support leaks from explicit tracks into implicit tracks, auto-repeat, areas, dense flow, or column flow. | Pass: all are named exclusions; existing profiles remain regression controls. |
| 19 | Chrome observations are copied as normative expected values. | Pass: the fixture, independent arithmetic, Chrome observation, and comparison report are separate artifacts. |
| 20 | Simulator screenshots or Agent Browser geometry are presented as device pixels, WPT conformance, or full C11 completion. | Pass: validation limits separate layout/draw evidence from PixelCopy, Metal readback, WPT, and official completion. |

No product source was modified to create this plan or review. Implementation must begin only after the plan PR is reviewed and merged.
