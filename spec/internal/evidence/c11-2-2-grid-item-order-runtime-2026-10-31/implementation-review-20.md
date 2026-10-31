# C11.2.2 sparse-row `order` implementation/runtime review · 20 perspectives

Date: 2026-10-31. Each item checks a distinct failure boundary in the implementation and fixed runtime path.

1. **Signed sort direction — PASS.** `ItemPlacement.order` is the computed `i32`; sparse normalization sorts ascending, including negative values.
2. **Tie determinism — PASS.** Each visible source child receives a unique ordinal before sorting; `(order, source_order)` preserves ties without depending on sort stability.
3. **Source DOM immutability — PASS.** `HostDocument` children are not rewritten; the style-to-layout matrix compares the original child sequence after layout.
4. **Visible-child boundary — PASS.** The admission gate and normalization skip hidden children; their placement cannot change the order of visible siblings.
5. **All-auto requirement — PASS.** When any visible child has nonzero order, every visible child must have no authored placement or a fully auto `GridItemInput`.
6. **Partial-placement negative — PASS.** A visible partially placed sibling makes nonzero order fail with `UnsupportedGridOrder` before a successful layout output.
7. **Hidden partial sibling — PASS by gate inspection.** Hidden children are exempt from the all-auto check and do not enter the sparse placement list.
8. **Sparse-only admission — PASS.** The early gate admits nonzero order only for `GridPlacementMode::SparseRow`; dense and other Grid profiles remain rejected.
9. **Unsupported display/mode ordering — PASS.** The nonzero-order gate runs before normal placement normalization, so an unsupported mode cannot consume the new order path.
10. **Zero-order compatibility — PASS.** The new gate only activates when a visible direct child has nonzero order; existing zero-order inputs retain the original path.
11. **Typed range lower bound — PASS.** `parse_order` accepts `i32::MIN` and rejects one below it with node/property/value context.
12. **Typed range upper bound — PASS.** `parse_order` accepts `i32::MAX` and rejects one above it with node/property/value context.
13. **Computed style forwarding — PASS.** The style fixture checks the serialized computed `order` values against the independent matrix.
14. **Frame oracle — PASS.** The fixed two-scene expected rectangles are compared in Rust and independently at runtime against Chrome 156 coordinates.
15. **Taffy child ordering — PASS for layout input.** Grid children use the same stable order sort path as Flex, while flow text paragraphs retain their existing special handling.
16. **Flex regression boundary — PASS.** Existing workspace Flex order tests pass in the workspace run; this change broadens the condition to Grid without changing the Flex branch.
17. **Versioned matrix isolation — PASS.** The order scenes use a separate `.../v1` matrix; the previous 16-scene matrix and its reference are not edited.
18. **Scene selector injection — PASS.** Only the two constant scene IDs select the new matrix; unknown or suffixed IDs fail closed.
19. **Android route evidence — PASS after correction.** The first run timed out before runtime/draw/PixelCopy markers; source tracing found the platform admission omission, both Android allowlists were corrected, and the AVD rerun matched 2/2 scenes.
20. **Surface evidence boundary — PASS.** Seven non-overlap item interiors matched exactly on each simulator; evidence labels PixelCopy/Metal drawable samples and does not claim compositor, physical-device, or overlap proof.

No remaining implementation defect was found in this bounded path. The initial Android route omission and a Clippy `collapsible_if` finding were corrected before the final simulator and lint runs. The unrelated C07.3 workspace test mismatch is recorded in the parent README and is not represented as a passing test.
