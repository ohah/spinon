# C11.4 Root HTML fit-content implicit-row list precomparison · 2026-11-08

The fixture is pinned at SHA-256 `51c1afc9fccc90f30e42ffebceaa9e1108aa3e27904df9b8a12d6632176ef321`. It exercises two- and three-entry `grid-auto-rows` patterns, a mixed `fit-content()`/fixed-px/fit-content pattern, a mixed fit-content/fixed-px/direct-percentage pattern, a minimum contribution above its cap, and zero/above-100% cap boundaries. Percentage values use the definite 100px Grid content-box height.

Chrome reference: Agent Browser HeadlessChrome 157.0.0.0, macOS arm64, viewport 1280×633, DPR 1. Observed computed values and used rows are in [chrome-observation.json](./chrome-observation.json); the full-page screenshot is [chrome157-agent-browser.png](./chrome157-agent-browser.png). The six scenes were compared against calculations derived from CSS Grid track-sizing rules before implementation; [expected values](./expected-values.json) and [comparison result](./independent-vs-chrome-check.json) record the input and exact frames. All six scenes match with 0 CSS px geometry difference.

The normative list behavior is [CSS Grid Layout Module Level 2 §7.6](https://www.w3.org/TR/css-grid-2/#auto-tracks): `grid-auto-rows` accepts one or more track sizes and repeats that pattern for additional implicit tracks, starting immediately after the last explicit track. The current published W3C text captured by Agent Browser is [css-grid-spec-observation.json](./css-grid-spec-observation.json). Fit-content track limits preserve the track minimum while capping max-content growth; see [§12.3 Track Sizing Algorithm](https://www.w3.org/TR/css-grid-2/#algo-track-sizing).

This is a Chrome reference matrix, not a Spinon result or a WPT conformance claim.
