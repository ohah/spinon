# C11.1 percentage implicit columns implementation/runtime review (20)

Scope: source diff, fixed precomparison fixture, focused Rust tests, Android API 37 AVD capture, iOS 26.2 simulator capture, and default/fixture-test FFI archives. This review checks this bounded slice only; it does not promote C11.1 or C11.

| # | Failure perspective | Result and evidence |
|---:|---|---|
| 1 | Percentage lists accidentally become available to row-flow or dense-flow profiles | Pass: cascade enables them only for `auto_flow == Column`; the column-flow adapter requires sparse column flow. |
| 2 | `fr`, `calc()`, intrinsic, or nested sizing functions pass as direct percentage tracks | Pass: the profile accepts only direct finite px/percentage values; style tests retain unsupported intrinsic forms as unsupported. |
| 3 | Negative, NaN, or infinite percentages reach layout | Pass: cascade/projection/layout validation rejects them; layout tests cover negative and NaN. |
| 4 | Empty or oversized patterns reach modulo/repetition logic | Pass: empty lists and patterns over `GRID_MAX_TRACKS` are rejected before expansion. |
| 5 | A percentage basis is inferred from viewport or indefinite width | Pass: width must be definite, finite CSS px; runtime rejection tests cover auto and math-backed width. |
| 6 | `min-width`, `max-width`, or `aspect-ratio` silently changes the basis | Pass: all three are rejected for this profile; layout and default-route tests exercise them. |
| 7 | Zero width is mistaken for indefinite width | Pass: fixed `0px` is accepted and percentage tracks resolve to zero; layout test covers it. |
| 8 | Percentages are computed after subtracting column gaps | Pass: independent expected values use 24px/36px tracks from 240px, then add 5px gaps to starts; runtime tests match. |
| 9 | Values above 100% are clamped to the container | Pass: 120% yields 288px tracks and starts `[80,373,666,959]`; layout test matches. |
| 10 | Mixed px/% track order is lost while repeating the pattern | Pass: `10px 15%` yields starts `[80,95,136,151]`; layout test matches. |
| 11 | Scalar percentage auto-column input is treated differently from a one-item list | Pass: scalar `10%` repeated geometry is covered by the precomparison and layout matrix. |
| 12 | Implicit pattern starts at the wrong entry after explicit tracks | Pass: the two-entry `[10%,15%]` pattern repeats from its first entry after the two explicit columns; all six column starts match. |
| 13 | Explicit columns or rows accidentally inherit percentage auto-track support | Pass: Root HTML profile still requires fixed px explicit tracks and `grid-auto-rows:auto`. |
| 14 | Gaps are counted zero times or twice in the returned item positions | Pass: expected starts include one 5px column gap and one 3px row gap; runtime/layout positions match. |
| 15 | Required-column count divides by zero or exceeds the implicit-track cap | Pass: container validation requires nonempty rows before `div_ceil`; explicit-plus-implicit columns are capped at 10,000. |
| 16 | Hidden or non-item descendants distort auto-placement count | Pass: the adapter counts visible owned direct children; nested grids, nonempty/non-div items, and unsupported item sizes fail closed. |
| 17 | Explicit placements, `order`, or non-source-order insertion bypass sparse column placement | Pass: all four placement properties must be auto and `order` must be zero; runtime test checks source order. |
| 18 | Expansion accepts a pattern that differs from the validated repeated tracks | Pass: `expanded_columns_match` checks explicit prefix, exact count, and each repeated pattern entry before placement is published. |
| 19 | Percentage values are accepted by projection but lost before the Taffy layout result | Pass: cascade, projection, layout, parsed Root HTML runtime, and both app routes retain the same values; focused tests plus both simulator logs show the matching 240×63 grid and twelve item frames. |
| 20 | Fixture ABI or payload leaks into the default FFI archive, or app route selects a neighboring C11 pixel fixture | Pass: default archive lacks the task symbol/title; opt-in archive contains both. Android/iOS route logs show the dedicated Root HTML host and 15 boxes without the generic C11 surface-pixel capture. |

Observed outcome: no blocker found in these 20 bounded checks. Platform evidence is simulator-only and does not prove physical-device GPU behavior or drawable pixel readback; see [runtime evidence](./README.md).
