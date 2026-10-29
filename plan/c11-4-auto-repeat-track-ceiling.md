# C11.4 auto-repeat track ceiling boundary

## Goal

Add a focused `spinon-layout` regression matrix for the pinned Taffy 0.15.0 auto-repeat expansion limit. The tests must observe actual initialized track states at requested sizes 9,999, 10,000, and 10,001, for both columns and rows, and distinguish `auto-fill` from empty `auto-fit` collapse.

This is a bounded implementation/test slice. It does not complete C11.4 or C11 and does not add public CSS support.

## Baseline and oracle

- Spinon's authored-grid limit is `GRID_MAX_TRACKS = 10_000` in `crates/spinon-layout/src/grid.rs`.
- The current adapter already rejects more than 10,000 authored tracks and bounds the flex-factor preflight using the auto-repeat ceiling. Existing tests cover those two checks, but do not directly assert the number of tracks emitted by auto-repeat at the boundary.
- The pinned dependency is vendored Taffy 0.15.0. `vendor/taffy/src/compute/grid/explicit_grid.rs` limits auto-repeat expansion to the remaining slots up to `MAX_GRID_TRACKS`, preserves tracks before the repetition, and truncates later tracks when the resulting explicit grid reaches the limit.
- The test oracle is the captured `Initialize` stage from the existing `TrackSizing` mode, filtered to non-gutter tracks. For one fixed 1px repeated track, a definite 9,999px, 10,000px, or 10,001px content axis expects respectively 9,999, 10,000, and 10,000 initialized tracks. Empty `auto-fit` keeps the same count but marks all repeated tracks collapsed; empty `auto-fill` keeps them uncollapsed.
- Separate source-order controls use a 2px prefix, a 1px auto-repeat, and a 3px suffix. At a 10,003px basis the expected sequence is one 2px prefix, 9,998 repeated 1px tracks, and one 3px suffix (10,000 total). At a 10,004px basis the uncapped fit would require 10,001 tracks; the capped sequence is one 2px prefix followed by 9,999 repeated 1px tracks, and the later 3px suffix is truncated. Run both controls on both axes.
- The pinned `TrackSizing` trace records track states across all captured stages. One 10,000-track axis uses fewer than the 100,000-state trace budget; rows and columns are run independently so this test does not exceed the budget by tracing two maximum axes at once.

## Scope

1. Build direct `LayoutInput` cases for each axis, size, and repeat mode using one 1px repeated track, zero gap, a definite content basis, and no grid items.
2. Assert that the `Initialize` stage exists exactly once for the target axis, contains the expected non-gutter track count, and reports the expected empty `auto-fit` collapse state.
3. Assert the 10,003/10,004 source-order controls retain the prefix and drop the suffix only when the cap is reached, on both axes.
4. Keep the work in Rust unit tests. The 10,000-track boundary is an algorithm trace assertion with no visible app scene; Android/iOS screenshots are not applicable to this test-only slice.
5. Record the test output, exact source revision, and trace-budget result in `spec/internal/evidence/c11-4-auto-repeat-track-ceiling-2026-10-30/` and link it from the C11.4 plan and status ledger.

## Out of scope

- Changing the 10,000 limit or Taffy’s clamping policy.
- Adding production logic, fixture ABI, app scenes, or CSS admission.
- Auto-repeat patterns with multiple repeated tracks, explicit placement/line resolution, named-line expansion, gutters, intrinsic sizes, percentage bases, or interaction with implicit tracks.
- Claiming the 10,000-track layout always succeeds outside this trace-budgeted fixed-size matrix; Taffy’s 100,000 captured-track-state budget and its internal allocation behavior remain separate limits.
- Marking C11.4, C11.3, C11, or WPT conformance complete.

## Completion checks

- The 12-case matrix (3 sizes × 2 axes × 2 repeat modes) matches the exact counts and collapse states above.
- The four prefix/suffix controls (2 sizes × 2 axes) confirm source order at the cap.
- The full relevant `spinon-layout` Grid test selection, `cargo fmt --all -- --check`, and `git diff --check` pass; report unrelated pre-existing failures separately.
- Review the implementation/runtime behavior from 20 distinct failure angles, then review the final PR delta from 30 distinct failure angles.
- Keep the official C11.4 and C11 checkboxes open.
