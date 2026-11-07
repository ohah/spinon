# C11.4 finite px fit-content cap oracle

Agent Browser on HeadlessChrome 157.0.0.0 (macOS arm64, viewport 1280×633, DPR 1) captured the independent scene in [`chrome-observation.json`](./chrome-observation.json). The fixed source is [`root-html-grid-fit-content-min-content-auto-track-list-px-precomparison.html`](../../../../../tests/fixtures/css/c11/root-html-grid-fit-content-min-content-auto-track-list-px-precomparison.html), SHA-256 `1e033168adfef61579ead7a2ea0f4f7c685ab5f43060e693fb53e530fff61720`.

The computed list remains `fit-content(20px) min-content auto`; used rows are `[10,20,34,30,6]px`. The separately calculated expected frames in [`expected-values.json`](./expected-values.json) match Chrome exactly: the Grid, five items and four nested payloads (10 frames total, 0 CSS px difference). The 20px fit-content cap is exercised as a pixel length, and only the auto track receives the 20px positive free-space stretch.

The saved [`Agent Browser screenshot`](./chrome157-agent-browser.png) has SHA-256 `fc87cc40902ae65a46d6d5231d78993ad335fbdbbc4b2178686f89438f3cacc5`. This is a Chrome reference only; it is not Spinon runtime or WPT evidence.
