# C11.1 Root HTML Grid cross-axis partial-placement precomparison

## 비교 입력

- Pinned browser oracle: Agent Browser로 연 HeadlessChrome `156.0.0.0`.
- Viewport: `360×800` CSS px, DPR 1.
- Grid: no-area, `grid-auto-flow: row`, sparse packing, four `40px` columns, three `40px` rows, zero gap, `160×120px` content box.
- Items: direct empty block children with fixed `40×40px` dimensions. No text, replaced/nested/positioned item, edges, order or transform.
- The fixture contains DOM-interleaved row-locked, column-locked, fully definite and fully auto items; span, `visibility:hidden`, `display:none`, reversed DOM interleaving, and one implicit-row control.

## 독립 판정

The fixed expectations in [`expected.json`](./expected.json) were derived from [CSS Grid Layout Module Level 1, 2025-03-26 CRD §8.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-placement-algo): definite placements reserve occupancy first; row-locked items are processed next in order-modified document order; remaining items are processed afterward, with definite-column items holding their column while the row cursor advances and fully auto items using the sparse cursor. `visibility:hidden` keeps a grid area and `display:none` contributes no grid item. The browser capture is kept separate so an observed frame does not define its own expected value.

The `dom-cross-axis` case has a column-locked item before row-locked items in DOM. Chrome places both row-locked items in row 1 first, then the column-locked items in later rows, then the fully auto item. This distinguishes the three placement phases and verifies that a single-axis profile cannot be reused unchanged for the mixed set.

The `mixed-spans` case verifies occupied cells across two rows and two columns. `hidden-controls` checks hidden occupancy, while `display-none-control` isolates the non-participating `display:none` item. `reverse-order` puts column-locked items before row-locked items in markup. The `implicit-row-control` is valid CSS and Chrome creates a fourth row at `y=120px`; the planned Spinon slice must reject this unsupported implicit extent atomically.

## 산출물과 한계

- [`fixture.html`](./fixture.html) is the frozen source input.
- [`capture.js`](./capture.js) extracts item frames relative to each grid.
- [`capture.json`](./capture.json) is the Agent Browser output.
- [`verify_precomparison.py`](./verify_precomparison.py) compares the capture with the independent expectations.
- [`chrome-156-agent-browser.png`](./chrome-156-agent-browser.png) is the full-page screenshot.
- `SHA256SUMS.txt` binds these files and the independent expected values.

This is a browser baseline only. No Spinon implementation, simulator, physical-device, hardware-GPU, WPT-runner, or product-admission result is claimed here.
