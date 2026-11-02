# C11 Root HTML fixed-px implicit row list precomparison

- Oracle: Agent Browser with HeadlessChrome `156.0.0.0`, DPR 1. The isolated browser session was closed after capture; no agent-browser/Chrome remote-debugging process remained.
- Fixture: [root-html-grid-auto-row-list-precomparison.html](../../../../tests/fixtures/css/c11/root-html-grid-auto-row-list-precomparison.html), SHA-256 `63a5aad195c91531f1b79d15f5caf3104f93897c3125f9c0fb9aa1f415270e4f`.
- Input: 100×100px Grid root, explicit columns 40px 55px, explicit row 20px, 5px row/column gaps, `align-content:start`, `grid-auto-rows:15px 25px`, and eight fixed-size direct items.
- Independent expected calculation: after the explicit 20px row, three implicit rows cycle from the first list entry to `[15px,25px,15px]`. Starts are y `[0,25,45,75]`; columns start at x `[0,45]`. Track plus gutter height is 90px, leaving 10px at the bottom of the root.
- Observation: computed implicit list `15px 25px`; computed used rows `20px 15px 25px 15px`; root `[0,0,100,100]`; all eight item frames match the independently calculated geometry. Raw observation is [chrome156-observation.json](./chrome156-observation.json); screenshot is [chrome156-cycle.png](./chrome156-cycle.png), SHA-256 `38fb232e47890531df3c9cdf4d153d1d78746258521e749968226f1e26b6133d`.
- Limits: this is one Chrome observation and one visual capture. It does not test Spinon, mobile runtimes, pixel equality, hardware GPU, WPT conformance, or any implementation behavior.
