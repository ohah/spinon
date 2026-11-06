# C11.4 Root HTML intrinsic columns + multiple direct text rows precomparison

## 입력과 관찰 경계

- Fixture: `tests/fixtures/css/c11/root-html-grid-intrinsic-combined-multiple-text-precomparison.html`
- SHA-256: `ef0cff8d4e53a162cc7a5dfdb0c8c5ec0d7b7f3942ba9985edc4f1f7101e33e6`
- Browser: Agent Browser HeadlessChrome 156.0.0.0 on macOS arm64, DPR 1, viewport 1280×633; `document.fonts.status=loaded`.
- Raw observations: [`chrome156-observation.json`](chrome/chrome156-observation.json). Screenshots: [`min-content`](chrome/min-content.png), [`max-content`](chrome/max-content.png), [`auto`](chrome/auto.png), [`minmax(min-content, 1fr)`](chrome/minmax-min-fr.png).
- The same single Grid owner was observed in both text source orders for `min-content`, `max-content`, `auto`, and `minmax(min-content, 1fr)`. The browser script changed only the explicit column value and, for reverse controls, swapped the two text item DOM nodes.

## 결과와 기준

Grid is 40×40px with a 10px explicit first row, one min-content implicit-row pattern, and four direct items: a 1×10px empty control, `A A A A A A`, `A`, and a 1×34px empty control. The two text items use 10px `monospace`, 10px line-height, and normal wrapping.

| Explicit column | Used column in Chrome | Forward rows | Text lines long/short | Reverse rows |
| --- | ---: | --- | --- | --- |
| `min-content` | 6.03125px | `[10,60,10,34]px` | 6 / 1 | `[10,10,60,34]px` |
| `max-content` | 66.234375px | `[10,10,10,34]px` | 1 / 1 | `[10,10,10,34]px` |
| `auto` | 40px | `[10,20,10,34]px` | 2 / 1 | `[10,10,20,34]px` |
| `minmax(min-content,1fr)` | 40px | `[10,20,10,34]px` | 2 / 1 | `[10,10,20,34]px` |

The independently expected behavior follows CSS Grid Layout Level 1, 2025-03-26 CRD §7.6, §11.1 and §11.5: explicit columns resolve before rows; those used columns determine available inline space for each text item; each item then contributes its own min-content block size to its implicit row. The fixed sibling controls do not set an intrinsic column minimum because their width is auto. Chrome observations are recorded separately from this derivation.

`min-content` leaves each `A` token on a separate line (6 × 10px), while `auto` and the flexible control resolve to 40px and wrap into two lines. `max-content` keeps the long text on one line and visibly overflows the 40px Grid. Reversing the text items changes the two implicit row sizes only where their contributions differ; item identity and DOM order remain aligned. The longest overflow reaches y=114px in the min-content case, y=74px in the auto/flexible cases, and y=64px in the max-content case.

## 한계

This is an implementation precomparison, not Spinon runtime evidence or WPT conformance. The `monospace` advance is specific to the recorded browser environment; Android/iOS native font metrics are measured independently at runtime. The matrix does not establish arbitrary text, Unicode, fonts, Grid topology, general CSS admission, GPU pixels, or completion of C11/C11.4.

The initial comparison draft used auto-width empty controls, which the existing Root HTML item contract does not accept. Before implementation, they were corrected to fixed 1px widths and the fixture hash, observations, screenshots and plan review were updated. This prevents the comparison input from silently describing an unsupported runtime scene.
