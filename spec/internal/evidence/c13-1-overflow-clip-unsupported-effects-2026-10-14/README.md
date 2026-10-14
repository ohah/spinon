# C13.1 clip radius / transform fail-closed boundary

## Acceptance criteria recorded before code changes

The C13 plan assigns rounded clip geometry to C22 and transform geometry to C23. `RuntimeBlockFlexOverflowClipV1` must not silently treat either effect as an untransformed rectangular clip.

The runtime boundary check will cover:

1. `overflow:clip` with authored `border-radius` on the clip ancestor.
2. `overflow:clip` with `transform` on the clip ancestor.
3. An absolutely positioned transformed descendant inside a clip ancestor.
4. `border-radius` and `transform` authored through stylesheet rules as well as inline style.

Each inline case must produce a cascade root and then fail layout with `unsupported_inline_property`, the node carrying the effect, and a radius-related or exact `transform` property. Each stylesheet case must return an explicit unsupported-author-CSS error containing the authored property. No case may publish a layout/render result. This work will not add radius or transform support to C13.1, and C13 remains incomplete.

## Runtime boundary test

`c131_clip_radius_and_transform_fail_closed` runs the C13 runtime calculation with a rounded clip ancestor, a transformed clip ancestor, and a transformed absolute descendant. Each inline case creates one cascade root and then fails with `unsupported_inline_property`, the node carrying the effect, and a radius-related or exact `transform` property. Separate author stylesheet controls reject radius and transform declarations before layout. No geometry or render snapshot is published for these inputs.

The focused test passed. `cargo test --locked --workspace` passed with 730 tests, 0 failures, and 4 ignored across 24 suites; workspace Clippy, rustfmt, and `git diff --check` passed. Raw output is in [`runtime-targeted-test.log`](runtime-targeted-test.log), [`workspace-cargo-test.log`](workspace-cargo-test.log), and [`workspace-clippy.log`](workspace-clippy.log). No app or product runtime source changed, so this test-only boundary update did not require another simulator build; no physical device was connected.

## Adversarial reviews

- [Implementation failure review: 20 perspectives](../c13-1-overflow-clip-unsupported-effects-implementation-review-2026-10-14.md)
- [PR delta review: 30 perspectives](../c13-1-overflow-clip-unsupported-effects-pr-review-2026-10-14.md)
