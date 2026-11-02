# C11 Root HTML named-area + fr plan sync

This documentation update removes a stale “plan-only” summary from `plan/c11-css-grid.md` and records the already merged bounded implementation of PR #1030.

## Source of truth checked

- PR #1030 merged on 2026-11-02 with merge SHA `579435b1955e7ad157baa13d100d75f13a5dd283`.
- `spec/STATUS.md` records the same merge, the fixed 2×2 named-area map, direct `1fr 2fr` tracks on both definite axes, and the Android API 37 ARM64 AVD and iPhone 17 Pro/iOS 26.2 Simulator draw/present evidence.
- `plan/c11-root-html-grid-fr-named-areas.md` records implementation scope, simulator evidence, and limits.
- The official C11 parent remains unchecked and the aggregate remains `72/213`.

The plan summary now matches these sources. `spec/STATUS.md` did not need a change because its PR #1030 entry was already current. This documentation-only update does not claim a new implementation, CI result, pixel readback, or C11 completion.

## Validation

- Compared the PR number, merge SHA, implementation scope, simulator classes, and incomplete-state wording against the local status ledger and implementation plan.
- `git diff --check` passed.
- No build, tests, browser, or simulator run was needed for this documentation-only correction.

See [the 20-point PR diff review](./pr-diff-review-20.md).
