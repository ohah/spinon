# C11.1 Root HTML mixed column-flow implementation review

Implementation review before PR. Each item attacks a separate admission, layout, platform, or evidence boundary.

1. **Can a non-Root-HTML runtime profile enter this route?** Pass — route discovery and placement-mode selection are guarded by `RuntimeRootHtmlTextPaintV1` in `crates/spinon-style-to-layout/src/projection/style_values.rs`.
2. **Can row flow, dense flow, areas, repeats, or named lines be selected as sparse column flow?** Pass — the candidate selector requires computed `grid-auto-flow:column`, excludes the dense route, and rejects areas, repeats, and line names in `root_html_one_axis_placement.rs`.
3. **Can a grid containing only one locked axis qualify?** Pass — both candidate discovery and layout validation require at least one row-locked and one column-locked item; layout tests cover the one-axis rejection.
4. **Are fully definite items incorrectly sent through the sparse cursor?** Pass — they are admitted and pre-reserved; the layout tests assert their placement before locked and auto items.
5. **Are fully auto items omitted after the locked phases?** Pass — the matrix and layout tests include fully auto items after definite, row-locked, and column-locked occupancy.
6. **Can named, mixed-line, or nonnumeric placement values slip through typed admission?** Pass — candidate classification only accepts numeric line/span and auto values; layout validates every endpoint and rejects named placement.
7. **Can `visibility:hidden` items fail to qualify their lock class or stop occupying cells?** Pass — dedicated hidden-row and hidden-column qualification scenes pass; runtime tests verify their frames remain while paint is suppressed.
8. **Does `display:none` consume an auto-placement cell or a pixel sample?** Pass — the runtime matrix verifies it has no layout frame and excludes it from the 31 participating-item samples.
9. **Do spans in the locked axis and auto axis preserve both dimensions?** Pass — the layout test covers two-axis spans and cursor occupancy; the `mixed-spans` simulator scene matches exact frames and pixels.
10. **Can nonzero `order` or reversed DOM order silently change the bounded algorithm?** Pass — unsupported nonzero order is rejected; the reverse-order scene pins DOM/source-order placement on both platforms.
11. **Do negative explicit lines resolve against the correct explicit grid boundary?** Pass — focused layout and runtime negative-line controls match the pinned frames on both simulators.
12. **Can placement expand beyond authored rows/columns or publish a partial scene?** Pass — row and column implicit-boundary controls fail atomically with no completed snapshot; both platform matrices require `invalid_grid_placement / grid-row-start / node 16`.
13. **Can invalid dimensions, tracks, or huge explicit grids bypass resource limits?** Pass — container and track validation require finite nonnegative supported dimensions and enforce `GRID_MAX_TRACKS`; compilation catches incomplete placement-mode matches.
14. **Can implicit tracks be created through `grid-auto-rows` or `grid-auto-columns`?** Pass — this mixed profile requires both to be `auto`, while explicit-only line bounds are checked after normalization.
15. **Can unsupported grid alignment, gap, or container styling alter sampled geometry?** Pass — the shared Root HTML one-axis validator rejects unsupported container styles; accepted fixed-grid geometry is compared at zero CSS-pixel tolerance.
16. **Can nested grids, intrinsic text, or non-static positioned children bypass item restrictions?** Pass — the shared validator rejects nested Grid containers, unsupported item contents/sizes, and positioned children.
17. **Does error reporting identify the offending property and runtime node rather than merely fail?** Pass — Rust, Android, iOS, and matrix checks pin the implicit-boundary diagnostic to property `grid-row-start` and node 16.
18. **Can this fixture leak into default product builds or collide with C11.1 dense fixtures?** Pass — Cargo/FFI/native adapters are opt-in; default and feature-boundary checks pass; Swift allowlists and runtime markers keep dense and mixed routes separate.
19. **Can pixel checks pass with a false device scale?** Pass after review fixes — Android's app density is compared with independent ADB `wm density`; iOS's reported scale must equal both actual Metal drawable axes divided by CSS viewport. Tests reject mismatched Android density and an oversized iOS drawable with a forged scale.
20. **Can stale processes, stale scenes, or an incomplete oracle make the matrix look green?** Pass — each scene uses a fresh process and run ID, the matrix pins 8 positive and 1 rejection scene, and both Android/iOS verifiers require all 31 exact RGBA samples plus exact geometry.

## Findings fixed during this review

- Column-flow invalid-placement errors could use row-flow wording. The shared validator now recognizes both mixed cross-axis modes when formatting diagnostics.
- The iOS CFM comparator trusted the logged density without binding it to the actual Metal drawable dimensions. It now checks both axes against the fixed CSS viewport, and the test rejects a forged scale.
- The Android matrix did not independently compare the app-reported density with device density. It now parses `adb shell wm density` and requires exact agreement.

## Verification evidence

- Android API 37 ARM64 AVD run `c111cfm-avd-mw23bnua-bf812a8f`: 9 scenes, 8 success, 1 expected rejection, 31/31 items, geometry error 0 CSS px, 31/31 exact surface samples, maximum channel error 0. ADB reports 420 dpi and PixelCopy reports 2.625×.
- iPhone 17 Pro / iOS 26.2 Simulator run `c111cfm-ios-mw23cgwv-f0048937`: 9 scenes, 8 success, 1 expected rejection, 31/31 items, geometry error 0 CSS px, 31/31 exact Metal drawable samples, maximum channel error 0. Drawable scale is 3× and the readback is before OS compositor submission.
- `node --test` for the CFM runtime matrix, Android/iOS pixel comparators, fixture-module boundary, and iOS build flags: 129 passed, 0 failed.
- Rust layout 4/4, style projection 1/1, parsed Root HTML runtime 2/2, `cargo fmt --all -- --check`, and combined C14/C11 runtime `cargo check` passed.

This bounded implementation slice does not mark C11.1, C11, public CSS support, Android hardware GPU rendering, or post-compositor iOS rendering complete.
