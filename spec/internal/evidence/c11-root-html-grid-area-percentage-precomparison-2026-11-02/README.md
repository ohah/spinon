# Root HTML named areas × percentage tracks precomparison

## Fixed input and oracle

- Browser: Agent Browser with `HeadlessChrome/156.0.0.0`.
- Viewport: 1280×633 CSS px at DPR 1.
- Four source pages: both-axis percentages, mixed px/percentage tracks, overflowing columns above 100%, and a zero-width first percentage track.
- The three item colors use `background-color` declarations. They are visually and semantically equivalent to the original shorthand, while keeping these Grid scenes within the runtime's existing CSS input profile; Grid declarations and geometry are unchanged.
- Raw computed-style strings, DOMRects, and case IDs: [Chrome observation](./chrome-observation.json).
- Normative independent calculation: [expected values](./expected.json).
- Chrome screenshots: [`both axes`](./chrome-both-axes.png), [`mixed px and percentage`](./chrome-mixed-px-percent.png), [`overflowing columns`](./chrome-overflowing-columns.png), [`zero track`](./chrome-zero-track.png).
- Source and screenshot SHA-256 values: [hash manifest](./hashes.txt).

## Independent calculation

All pages use the same 120×70px definite root and rectangular area map `"header header" "side content"`. The source order is deliberately `content`, `header`, `side`; each item is placed by its `grid-area` name.

- `both-axes`: columns are `[120×0.25, 120×0.75] = [30,90]`; rows are `[70×0.20, 70×0.80] = [14,56]`.
- `mixed-px-percent`: columns `[40,120×0.50] = [40,60]`; rows `[70×0.20,50] = [14,50]`. The remaining 20px width and 6px height stay unused.
- `overflowing-columns`: columns `[120×1.20,120×0.10] = [144,12]`; their 156px sum remains wider than the 120px root. Visible overflow places `content` at x=144px.
- `zero-track`: columns `[120×0,120×1] = [0,120]`; the first area has zero width and the second starts at x=0.

The expected values are calculated from the definite content-box axes and area boundaries, not copied from Chrome output. Existing Root HTML percentage validation already checks definite-axis basis, finite resolved sizes, overflow, and fixed-pixel gap constraints. CSS Grid 2025 CRD §7.2.1 defines track sizes and §7.3 defines named areas; the implementation plan preserves the existing exact 2×2 area map and does not add general CSS admission.

## Chrome observation

Chrome returned computed tracks `30px 90px` / `14px 56px` for the both-axis case, `40px 60px` / `14px 50px` for the mixed case, `144px 12px` / `14px 56px` for the overflowing case, and `0px 120px` / `14px 56px` for the zero-track case. All item frames match the independent arithmetic exactly at these integer boundaries. The overflowing page reports `root.scrollWidth=156` while the grid root remains 120px wide.

These browser observations are an independent oracle only. They do not establish Spinon runtime behavior or standards conformance. The default Root HTML runtime comparison is covered by [`c11_1_default_root_html_runtime_combines_named_areas_with_percentage_tracks`](../../../../crates/spinon-runtime/src/session/tests/root_html_grid_template_areas.rs); projection accepts only the exact 2×2 map and two direct px/percentage tracks per axis. Separate runtime rejection cases keep mixed `%`/`fr`, indefinite bases, nonzero percentage gap, math tracks, and negative percentages closed.
