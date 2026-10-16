# C15 0px inline width observation

Agent Browser opened the existing `inline-text-precomparison.html` fixture in Chrome `155.0.8059.85` on macOS 26.5.1 arm64. The Ahem font loaded successfully. The source file remained unchanged; the live DOM changed only `#long-word.style.width` from 15px to 0px.

At viewport 1280×633 CSS px and DPR 1, Chrome returned a parent rect of `[8, 138, 0, 10]` and a text Range rect of `[8, 138, 20, 10]` for the unbreakable `XX`. A definite 0px content width therefore produces one 20×10px text line whose advance extends beyond the parent width.

The machine-readable observation, Chromium revision and fixture SHA-256 are in `zero-width-long-word-observation.json`. This is a DOM geometry reference only. It is not a Spinon runtime, simulator, screenshot, or pixel-paint result. The Agent Browser session was closed after capture and no Agent Browser/Chrome process remained.
