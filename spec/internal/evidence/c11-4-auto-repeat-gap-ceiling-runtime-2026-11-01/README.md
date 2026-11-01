# C11.4 positive-gap auto-repeat ceiling runtime evidence

## Scope

This test-only slice checks whether a positive `1px` gap is included in the repetition count and final named-line frame for a two-track auto-repeat pattern at Spinon's internal 10,000-track ceiling. It exercises the pure Rust `spinon-layout` Grid sizing trace with Columns and Rows, an exact-fit basis, and a cap case.

It does not add CSS admission, an app/FFI route, or public CSS support. It does not establish full WPT conformance or complete C11.4/C11. The official count remains `72/213`.

## Independent expectation

The pattern is `[pattern-start] 1px [pattern-middle] 2px [pattern-end]`, with a `2px` prefix, a `3px` suffix, and a `1px` gap. For `R` repetitions, the unbounded content-axis size is:

```text
prefix + suffix + repeated tracks + gutters
= 2 + 3 + 3R + (1 + 2R) * 1
= 6 + 5R
```

Thus `25,001px` fits 4,999 repetitions exactly and yields 10,000 explicit tracks. `25,006px` requests 5,000 repetitions; after the internal cap, it keeps the prefix, 4,999 complete `[1px, 2px]` pairs, and the first `1px` track of the final pattern. The suffix is truncated.

The final `pattern-start -1 / pattern-middle -1` item frame includes all preceding gaps: offset `24,993px` in the exact-fit case and `24,998px` in the cap case, with a `1px` target-axis size. CSS Grid Layout Module Level 1, 2025 CRD [§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat) says definite `auto-fill` chooses the largest positive repetition count that does not overflow the content box, taking `gap` into account. See the [plan](../../../../plan/c11-4-auto-repeat-gap-ceiling.md).

## Results

- Four executions: Columns/Rows × exact-fit/cap, all matched the expected 10,000-track sequence and item frame.
- Captured track state stayed below 100,000 in each execution.
- Focused regression: 1 passed.
- Auto-repeat ceiling module: 4 passed.
- Grid track-sizing suite: 53 passed.
- `cargo fmt --all -- --check`, all-target `spinon-layout` Clippy with `-D warnings`, and `git diff --check` passed.
- No Android/iOS simulator or screenshot was used: this algorithm trace has no app surface or fixture ABI.
- The plan has a separate [20-point failure review](../../../../plan/c11-4-auto-repeat-gap-ceiling-plan-review-20.md); the executed implementation has a new [20-point runtime review](./implementation-runtime-review-20.md).

Detailed output: [focused test](./checks/focused-test.log), [ceiling tests](./checks/ceiling-tests.log), [Grid track-sizing suite](./checks/grid-track-sizing-tests.log), [rustfmt](./checks/rustfmt.log), [Clippy](./checks/clippy.log), and [diff check](./checks/diff-check.log). [`source-revision.json`](./source-revision.json) binds the run to the base commit and candidate source hash.
