# Root HTML Grid column-flow mixed placement precomparison

This evidence fixes a bounded follow-up to the merged row-flow mixed cross-axis profile. It compares a column-flow Grid that combines fully definite, row-locked, column-locked, and fully-auto items.

## Independent expectation

The independent occupancy model follows CSS Grid 2025 CRD §8.5. For column flow, fully definite items reserve cells first, column-locked items are placed in source order down their fixed columns, and the remaining items use the sparse cursor down rows before advancing columns. `visibility:hidden` still occupies a cell; `display:none` does not. Negative line `-2` resolves from the end of each explicit axis. `expected.json` records the full cell/frame derivation and an implicit-track rejection control.

## Chrome observation

The separate browser observation used Agent Browser with HeadlessChrome 156, DPR 1, viewport 1280×633. `capture.json` records the computed tracks, flow mode, visibility, and scene-relative frames. The eight positive scenes and 31 participating item frames (27 CSS-visible and four `visibility:hidden`) match the independent expectations exactly. Two scenes make a hidden item the only representative of one placement class, so mode selection must count it as a Grid item while excluding `display:none`. The implicit-boundary control resolves to five columns and four rows in Chrome, beyond its authored four-column/three-row grid, and remains an expected fail-closed case for the bounded product profile.

The screenshot is `chrome-156-agent-browser.png`. The HTML input is `fixture.html`. Chrome is a cross-check; the CSS Grid algorithm and occupancy model are the independent basis.

## Review records

The plan has a separate [20-point failure review](../../../../plan/c11-root-html-grid-column-flow-mixed-cross-axis-plan-review-20.md) and this plan PR has a [30-point diff review](plan-pr-diff-review-30.md). Runtime and implementation reviews remain future work for a separate implementation PR.

## Scope

This is a browser precomparison only. It is not a Spinon runtime result, product CSS-admission claim, WPT pass, simulator result, or completion claim for C11.1/C11.
