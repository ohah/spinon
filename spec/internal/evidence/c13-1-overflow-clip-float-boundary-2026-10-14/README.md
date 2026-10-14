# C13.1 float / formatting-context boundary comparison

## Implementation gate recorded before code changes

The C13.1 plan requires a `clip` versus `hidden` / `flow-root` float comparison. This comparison pins the browser distinction and verifies Spinon's current fail-closed boundary; it does not add float layout to C13.1. C09 explicitly assigns float and clear layout to C26 and states that C09.3 `flow-root` support does not imply float containment or external-float support.

### Pinned browser observation

Captured through agent-browser `0.31.1` from `tests/fixtures/css/c13/overflow-clip-c13-1.html` on 2026-10-14. Chrome `155.0.8059.40` revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}` reported macOS `26.5.1 (25F80)` and V8 `15.5.35.20`; the binary SHA-256, fixture SHA-256, viewport, and measurements are in [`live-observation.json`](live-observation.json).

Expected relation from the pinned browser:

| case | computed display / overflow | parent height | float frame |
| --- | --- | ---: | --- |
| `hidden-float` | `block` / `hidden` | 64px | 48×36px |
| `clip-float` | `block` / `clip` | 28px | 48×36px |
| `clip-flow-root-float` | `flow-root` / `clip` | 64px | 48×36px |

The parent has a 14px padding-plus-border contribution at each block edge, giving an empty-box height of 28px. `hidden` and `flow-root` contain the 36px float; plain `clip` does not, leaving the 28px parent height. All float origins remain at the 14px inner offset. The observed CSSOM values (`clientHeight` / `scrollHeight`) are retained in the JSON and are not used as a C13 scroll-support claim.

### Acceptance criteria before implementation

1. The live browser observation matches the fixed Chrome `155.0.8059.40` relation above at viewport `1280×633`, DPR 1.
2. For all three parent variants, Spinon's `BlockFlexOverflowClip` calculation may produce a cascade snapshot but must return a layout failure with code `unsupported_inline_property`, node equal to the float child, and property `float`.
3. No parent variant may bypass the float rejection; no case may publish a successful layout/render snapshot.
4. This work must not add float support to C13.1, change C09/C26 ownership, or mark C13.1 complete. Geometry parity is not claimed until the owning float layout work exists.

## Runtime boundary test

`c131_float_formatting_comparison_cases_fail_closed` exercises all three inline-style parent variants through `BlockFlexOverflowClip`. Each cascade succeeds, then layout fails with `unsupported_inline_property`, the exact float child node, and property `float`. A separate author stylesheet control also rejects `float`. It does not assert Spinon float geometry parity.

Focused and workspace Cargo test runs, workspace Clippy, rustfmt, and `git diff --check` passed. Raw output is in [`runtime-targeted-test.log`](runtime-targeted-test.log), [`workspace-cargo-test.log`](workspace-cargo-test.log), and [`workspace-clippy.log`](workspace-clippy.log). No app or product runtime source changed, so this test-only boundary update did not require another simulator build; no physical device was connected.

## Adversarial reviews

- [Implementation failure review: 20 perspectives](../c13-1-overflow-clip-float-boundary-implementation-review-2026-10-14.md)
- [PR delta review: 30 perspectives](../c13-1-overflow-clip-float-boundary-pr-review-2026-10-14.md)
