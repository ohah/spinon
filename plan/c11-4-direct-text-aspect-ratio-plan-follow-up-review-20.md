# C11.4 direct-text aspect-ratio 100px-root plan follow-up — 20 checks

**Review target:** the plan and precomparison input revision after merged plan PR #833, before implementation changes.

| # | Failure angle introduced by the input revision | Result |
| --- | --- | --- |
| 1 | Chrome fixture and app source use different root widths | PASS — both now use a definite 100px root. |
| 2 | `max-content` columns stretch to consume the 100px root | PASS — Chrome capture records content tracks and unused inline space. |
| 3 | Ratio row expected root width remains 50px | PASS — expected root frame is updated to 100×20px; item-local frames stay 10/40px wide. |
| 4 | No-ratio row control inherits old intrinsic root width | PASS — control root frame is 100×10px. |
| 5 | Definite-row ratio scene exceeds the root width | PASS — columns total 90px inside the 100px root, leaving 10px unused. |
| 6 | Definite-row no-ratio columns are still expected at 40px+10px | PASS — capture and independent calculation both record those tracks inside 100px. |
| 7 | Auto row changed when only root width changed | PASS — Chrome still reports row tracks 20px with ratio and 10px without ratio. |
| 8 | Root width revision changes the CSS grid track syntax | PASS — both positive/control pairs still use two `max-content` columns. |
| 9 | Row and column test directions collapse into one input | PASS — auto-row and definite-40px-row remain separate scenes. |
| 10 | Item-local frame positions accidentally retain document coordinates | PASS — README table and JSON use root-local coordinates. |
| 11 | The screenshot no longer matches JSON after fixture edits | PASS — screenshot was recaptured from the 100px-root fixture. |
| 12 | The screenshot or fixture hash points to the previous version | PASS — hashes were recomputed and recorded after the rerun. |
| 13 | Ahem loading is assumed after changing the base commit | PASS — Agent Browser reports the Ahem font face loaded in this run. |
| 14 | Chrome version/viewport changes are hidden | PASS — the rerun records HeadlessChrome 156.0.0.0, 1280×633, DPR 1. |
| 15 | The app root's definite width changes the content contribution basis | PASS — the plan pins the same computed width and requires cross-checking typed input. |
| 16 | The feature is broadened to arbitrary root widths | PASS — only 100px is accepted in this fixture profile. |
| 17 | The changed root width invalidates ratio calculations | PASS — width-to-height remains 40÷2=20; height-to-width remains 40×2=80. |
| 18 | The revision quietly reuses replaced-image transfer logic | PASS — the plan continues to exclude that shortcut for non-replaced text. |
| 19 | The new precomparison is mistaken for a Spinon runtime result | PASS — README says Chrome-only and the plan keeps simulator runs pending. |
| 20 | The plan correction changes official status or completion count | PASS — C11.4 remains unchecked and official completion stays `72/213`. |

**Result:** 20/20 input-revision checks pass. Implementation may proceed against the 100px-root contract; original plan and merged plan-PR review reports remain historical records for PR #833.
