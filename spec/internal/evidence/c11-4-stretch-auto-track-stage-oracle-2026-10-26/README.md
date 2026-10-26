# C11.4 §11.8 auto-track stretch stage oracle

## Scope

This evidence covers one test-only `spinon-layout` slice. It verifies the `StretchAutoTracks` snapshot exposed by the existing trace. It does not change track sizing, style projection, FFI, or Android/iOS adapters.

The normative rule is [CSS Grid Layout Module Level 1, 2025 CRD §11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch): with `normal` or `stretch` content distribution, remaining positive definite free space is divided equally among tracks whose maximum sizing function is `auto`.

## Fixed input and expected values

- Root and viewport: `20×100px`, `RootSizingPolicy::Match`, zero border/padding.
- Columns: `[20px]`; rows: `[auto, auto, 5px]`; row gap: `10px`; grid placement mode: `TrackSizing`.
- Three direct children have fixed block sizes `[10,20,5]px` in DOM/row order.
- Before §11.8, row bases are `[10,20,5]px`; two internal gaps use `20px`. Remaining free space is `100 - 35 - 20 = 45px`.
- `align-content:stretch` divides `45px` between the two auto-max rows. The expected stage rows are `[32.5,42.5,5]px`, with gap bases `[0,10,10,0]px` including the two collapsed boundary gutters. The fixed third row stays `5px`; all row tracks and gaps occupy `100px`.
- Stretch child frames are `(0,0,20,10)`, `(0,42.5,20,20)`, `(0,95,20,5)`.
- `align-content:start` is a negative control: `StretchAutoTracks { applied:false }`, rows `[10,20,5]px`, `45px` unused, and child y positions `[0,20,50]`.

The reference is the rule and arithmetic above, not a Taffy output used as an oracle. The test inspects the `Rows` pass and checks `IntrinsicSizingComplete → MaximizeTracks → ExpandFlexibleTracks → StretchAutoTracks` order, pre-stretch snapshots, stretch result, gutter sizes, root frame, and child frames.

## Verification

| Check | Result |
| --- | --- |
| `mise exec -- cargo test --locked -p spinon-layout css_grid_2025_stretch_auto_tracks_distributes_definite_free_space_equally` | 1/1 passed |
| `mise exec -- cargo fmt --all -- --check` | passed |
| `mise exec -- cargo clippy --locked -p spinon-layout --all-targets -- -D warnings` | passed |
| `git diff --check` | passed |

Raw command output is in [focused-test.log](./focused-test.log), [fmt-check.log](./fmt-check.log), and [clippy.log](./clippy.log). The first focused-test attempt exposed a missing `LayoutEngine` trait import in the new test; the import was added and the final rerun passed.

The plan and implementation each passed a separate 20-view review: [plan review](../../../../plan/c11-4-stretch-auto-track-stage-plan-review-20-2026-10-26.md) and [implementation review](./implementation-review-20.md). The separate PR diff review is still pending.

No app/runtime production path changed, so Android/iOS app builds and simulator captures were not part of this test-only gate. This does not establish browser or platform parity, WPT conformance, or C11.4 completion. The official checkbox and `72/213` count remain unchanged.
