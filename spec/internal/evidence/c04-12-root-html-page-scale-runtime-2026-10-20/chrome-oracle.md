# Agent Browser Chrome viewport oracle

- Agent Browser CLI `0.31.1`, Chrome engine, iPhone 12 emulation, device pixel ratio `3`.
- Screenshot surface: `1170×2532` physical pixels. Inputs derive from `tests/fixtures/css/c04/root-html-page-scale-runtime.html` with exactly one viewport meta replacement.
- The prior pinned Chrome 155 oracle remains the source of comparison for browser behavior. This run re-read the same three meta cases and saved screenshots beside this record.

| Case | `innerWidth×innerHeight` | `visualViewport` width×height | visual scale | Probe physical bounds at DPR 3 |
| --- | ---: | ---: | ---: | ---: |
| `initial-scale=0.5` | `780×1688` | `780×1688` | `0.5` | `[150,180) × [75,90)` |
| `initial-scale=2` | `390×844` | `195×422` | `2` | `[600,720) × [300,360)` |
| `width=500, initial-scale=1` | `501×1083` (integer DOM readout) | `390×844` | `1` | `[300,360) × [150,180)` |

The fixed-width case's integer `innerWidth`/`innerHeight` readout is rounded. The pinned reference retains its fractional viewport observation (`500×1082.0513`). Runtime CSS viewport values are checked independently from the app's environment report.

| Case | Screenshot |
| --- | --- |
| `initial-scale=0.5` | [chrome-scale-half.png](./chrome-scale-half.png) |
| `initial-scale=2` | [chrome-scale-two.png](./chrome-scale-two.png) |
| `width=500, initial-scale=1` | [chrome-width-500.png](./chrome-width-500.png) |

Input variant SHA-256:

- `initial-scale=0.5`: `de90f7a36cd0ceae2869204601632c1662bf49ed438fb2d06679a814870c2fab`
- `initial-scale=2`: `fe7368388fbeb38a69d4fabce218568735cee890bbcb890340c8c183f75246bc`
- `width=500, initial-scale=1`: `028471f1dbd807679ef455b21ccf4b5fc832e9131a1d2e84a33790059f8db097`
