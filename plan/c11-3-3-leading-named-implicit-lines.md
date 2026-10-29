# C11.3.3 · Leading and trailing implicit named lines

**Status:** implementation plan; no implementation or official completion claim yet.
**Parent:** [C11 Grid plan](./c11-css-grid.md) · [official status](../spec/STATUS.md#c11-css-grid) · [internal contract 0059](../spec/internal/0059-c11-grid-layout.md)
**Normative source:** [CSS Grid Layout Module Level 1 · 2025-03-26 CRD §8.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement), §7.5 and §8.3.1.
**Pinned WPT context:** revision `c999c58338ee1d223df5ad62e48be1202ced0d37`. The existing unknown-area WPT cases do not directly cover leading named-line fallback; this plan is derived from the normative §8.3 example and its named-occurrence rules.

## Goal

Extend the existing C11.3.3 opt-in named-line profile so an insufficient named-line occurrence or named span can resolve against implicit lines on the relevant side of the explicit grid. Preserve authored line names and area aliases first. Keep the accepted input narrow: named placement must cause the implicit extent, grid-auto tracks must be fixed CSS-pixel sizes, and items must remain definitely placed.

The current projection resolves unknown bare names and trailing named-span shortages only. It rejects missing numeric named-line occurrences and backward named-span shortages. The current layout profile expands only a trailing extent. The existing signed-origin track normalizer and reverse leading grid-auto track-list expansion provide a candidate implementation path, but must be used without admitting the broader TrackSizing profile or its auto-placement behavior.

## Normative behavior and implementation boundary

- For a numbered named line, count authored matches in the requested direction. If the count is short, synthesize the remaining named lines as implicit lines in that counting direction. Positive occurrences extend after the explicit grid; negative occurrences extend before it.
- For a named span, first count authored matching lines in the span search direction. If the count remains short, synthesize named implicit lines only on that direction's side of the explicit grid. A backward search must skip endward implicit lines even when its opposite line lies beyond the explicit grid; the §8.3 `span foo / 4` example fixes this case.
- Preserve explicit authored-name and template-area alias precedence. An authored matching line satisfies the placement before fallback creates any implicit line.
- Use the existing C11.3.3 opt-in route. General numeric line placement does not become supported. A numeric line outside the explicit grid is admitted only as the definite opposite endpoint needed by a named span in this route; an unpaired numeric extent remains rejected.
- Permit only fixed CSS-pixel values in grid-auto-rows and grid-auto-columns, including a non-empty fixed-pixel list. Leading list expansion follows CSS Grid's reverse-from-the-explicit-grid order. Keep the 10,000-track and checked-arithmetic limits.
- Keep automatic placement, auto/intrinsic implicit sizes, auto-repeat, implicit area generation, general negative numeric placement, WPT conformance, and public CSS support out of scope. The fixture must reject or avoid these inputs rather than accidentally routing them through a broader profile.

## Oracle fixture

The fixed fixture is `tests/fixtures/css/c11/grid-leading-named-implicit-lines-c1133.html`. Its inputs, scene identities, expected computed track lists and root-relative rectangles are recorded in `tests/fixtures/css/c11/grid-leading-named-implicit-lines-c1133-inventory.json` and `tests/fixtures/css/references/c11-3-3-leading-named-implicit-lines-precomparison-v1.json`.

| Scene | Placement under test | Computed columns in CSS pixels | Root size | Item rect relative to root |
|---|---|---|---|---|
| positive-missing-occurrences | `3 ghost / 4 ghost` | `[30, 40, 15, 7, 15, 7]` | `[114, 20]` | `[107, 0, 7, 20]` |
| negative-missing-occurrences | `-1 ghost / -2 ghost` | `[15, 7, 30, 40]` | `[92, 20]` | `[0, 0, 15, 20]` |
| backward-span-one | `span ghost / 4` | `[7, 100, 15, 7]` | `[129, 20]` | `[0, 0, 129, 20]` |
| backward-span-two | `span 2 ghost / 4` | `[15, 7, 100, 15, 7]` | `[144, 20]` | `[0, 0, 144, 20]` |
| backward-span-partial-explicit | one authored `ghost` plus `span 2 ghost / 4` | `[7, 40, 100, 15]` | `[162, 20]` | `[0, 0, 162, 20]` |
| forward-span-two | `4 / span 2 ghost` | `[50, 15, 7, 15, 7]` | `[94, 20]` | `[72, 0, 22, 20]` |
| backward-span-uses-authored-name | authored `ghost` plus `span ghost / 4` | `[30, 40, 15]` | `[85, 20]` | `[30, 0, 55, 20]` |

The precomparison was captured from the local fixture with Agent Browser 0.31.1 and HeadlessChrome 156.0.0.0 at DPR 1. Its JSON and screenshot are under `spec/internal/evidence/c11-3-3-leading-named-implicit-lines-precomparison-2026-10-29/`. The browser binary revision/hash was not exposed by Agent Browser, so this is a version-bounded browser observation, not a binary-pinned Chrome 154 capture. The normative CRD remains the behavior authority.

## Implementation and verification gates

1. Merge this plan and its precomparison record before implementation. The 20-point plan review is `spec/internal/evidence/c11-3-3-leading-named-implicit-lines-plan-review-20-2026-10-29.md`.
2. Extend the named-line resolver with checked signed line arithmetic and directional implicit-name fallback. Keep the existing area/authored-name resolution precedence and old wrappers unchanged.
3. Connect the named-line opt-in placement to signed-origin track expansion while preserving its fixed-size, definite-placement restrictions. Reject auto-placement and any non-fixed-pixel implicit track-list member before allocation.
4. Add focused resolver, style-to-layout and layout-core tests for each inventory scene, explicit-name precedence, negative numeric occurrences, backward/forward spans, leading track-list reversal, numeric-only rejection, area regression, limit boundaries and failure atomicity.
5. Add the new scene set only to the opt-in C11.3.3 fixture route. Verify root and item geometry, resolved track sizes, DOM node identity, draw boxes and pixels on Android API 37 ARM64 AVD and iPhone 17 Pro / iOS 26.2 Simulator. Re-run the existing C11.3.3 26-scene matrix after the last code change. State simulator/software-renderer and pre-compositor boundaries exactly; do not claim a physical-device or general CSS result.
6. Run the repository-required Rust formatting and Clippy gates. Record initial failures, corrective runs, final HEAD, local checks, required CI state and merge proof separately. Keep C11 and the official 72/213 count unchecked.
7. Before the implementation PR, write 20 distinct implementation-failure reviews and the repository-required PR diff review with at least 30 independent points. Recheck all affected points after corrections.

## Explicit non-goals

This slice does not implement arbitrary numeric implicit placement, CSS Grid auto-placement with signed extents, intrinsic grid-auto track sizing, general named-line parsing, all WPT cases, or a public CSS Grid support declaration. C11.3.3 and C11 remain incomplete after this slice.
