# C13.1 runtime matrix implementation failure review · 20 perspectives · 2026-10-14

Scope: renderer hit clipping for contained absolute descendants, the expanded Chrome/Rust matrix, and the corrected content-box regression. The oracle and pass criteria are pinned in [the evidence README](README.md) and raw Chrome observations. This review is separate from the PR delta review.

| # | Failure perspective | Adversarial check and result |
| --- | --- | --- |
| 1 | Stale oracle mistaken for current Chrome | Captured the fixture in the pinned Chrome build and recorded browser revision, viewport, and fixture hashes. |
| 2 | User agent or viewport drift changes hit behavior | Test helper asserts the exact Chrome user agent, 1280×1100 viewport, DPR 1, version, and revision. |
| 3 | Captured row silently edits expected semantics | Rust checks each selected observation against separately pinned per-row values before using it. |
| 4 | Only in-flow regression gets changed | Existing positioned `clip-margin-border` row still expects the border-box union; the new absolute row expects the visual edge. |
| 5 | Contained absolute child gets parent border hits | Red case reproduced x=416..535 versus Chrome x=504..531; after fix its target interval matches Chrome. |
| 6 | Nested ancestor reintroduces wider hit range | Red nested case reproduced y=924..1019 versus Chrome y=928..1015; both outer and inner clip bounds now match. |
| 7 | A clip on one axis clips the other axis | `clip-x` and the earlier `clip-y` independently assert the unrestricted axis and its full hit interval. |
| 8 | Two-axis clip is accidentally treated as one edge | `clip-both` asserts x/y paint edges and independent integer hit intervals. |
| 9 | Padding/content margin expansion uses the wrong box | Both margin rows pin paint bounds and Chrome target hit ranges; content-box uses its own visual edge. |
| 10 | `hidden`/`auto` accidentally consume clip margin | Mixed-axis Chrome rows assert normalized computed values and that scrollable-axis paint/hit clips ignore the margin. |
| 11 | Escaped absolute descendant gets clipped by an unrelated ancestor | `positioned-escape` asserts no paint or hit clip from the static clip ancestor whose containing block is outside it. |
| 12 | Containment scan uses an unrelated positioned sibling | Detection is limited to positioned nodes on the current target's ancestor lineage; the nested and sibling-free fixture makes the tested ownership explicit. |
| 13 | Escape rule applied to unrelated overflow ancestors | `positioned-escape` proves this static clip ancestor is skipped; the row has no second overflow ancestor, so cross-ancestor escape composition remains outside this case. Fixed descendants are unchanged by this delta. |
| 14 | Nested clips union instead of intersect | `nested-clip` asserts exact x/y intersection bounds for paint and hit separately. |
| 15 | Half-open bounds are rounded incorrectly at edges | Bounds use CSS interval edges; the oracle assertion checks every integer CSS point intersecting the target on each axis. |
| 16 | Hit range is tested only at one lucky sample | Each axis sweeps the target's entire integer coordinate span at the Chrome-pinned orthogonal coordinate. It does not claim a full two-dimensional viewport sweep. |
| 17 | Content-box old test still encodes border-union behavior | A fresh controlled capture pins x=74..119 and y=54..89; the Rust test has positive x=119 and negative x=122 controls. |
| 18 | Paint behavior changes while fixing hit bounds | Product change only selects hit-test bounds; paint clip fields remain unchanged and exact clip-bound assertions run across the matrix. |
| 19 | Android route is used to imply new absolute coverage | AVD evidence is explicitly limited to the existing 13-node fixture; the new contained-absolute row is host Chrome/Rust evidence only. |
| 20 | Limited profile is reported as standards completion | WPT, DOM events, full surface sweep, scanout, physical hardware, and remaining C13.1 gates remain stated as unverified. |

The selected Rust module passed 8 tests; the render crate passed 52 tests with 1 ignored, and focused Clippy passed. These results do not close C13.1.
