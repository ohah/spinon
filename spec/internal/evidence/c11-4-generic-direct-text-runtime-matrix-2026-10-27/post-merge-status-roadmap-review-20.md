# C11.4 post-merge status and roadmap review · 20 perspectives

Reviewed the post-merge status, plan, evidence README, and public roadmap update against PR #679 and its recorded simulator artifacts. No status-count or scope discrepancy remains. Tailnet deployment is deliberately verified after the documentation merge and is not claimed by this source review.

| # | Failure perspective | Review result |
| ---: | --- | --- |
| 1 | A partial fixture result accidentally checks C11 complete. | The parent C11 checkbox remains unchecked. |
| 2 | The completion counter rises from simulator evidence alone. | The official count remains 72/213; no counter source or checklist IDs changed. |
| 3 | The matrix is described as five scenes rather than six including the control. | Status and roadmap both state six total scenes and 6/6 per platform. |
| 4 | Android results are described as physical-device or hardware-GPU evidence. | The platform is explicitly API 37 ARM64 AVD; hardware GPU is excluded. |
| 5 | iOS simulator results are described as physical iPhone evidence. | The platform is explicitly iPhone 17 Pro / iOS 26.2 Simulator. |
| 6 | A missing or failed scene is hidden by a total-only result. | The six-scene matrix tables and run README link the exact per-platform scene records. |
| 7 | Layout boxes are described as painted text glyphs. | The status and roadmap state runtime_text_count=0 and glyph_batch_draw_count=0. |
| 8 | Draw/present is overstated as pixel equality. | The documents identify box/surface present only and exclude pixel parity claims. |
| 9 | Narrow opt-in routing is presented as general CSS support. | General CSS admission and arbitrary HTML/text behavior remain explicitly unverified. |
| 10 | Archive marker checks are expanded into final APK/IPA fixture exclusion. | The docs say Rust static archive only and exclude whole APK/IPA claims. |
| 11 | Source validation and merge identity are confused. | The tested tree matches the PR head source commit; the GitHub merge SHA is recorded separately. |
| 12 | Merge time or SHA is copied from an older plan PR. | The entry records PR #679 and merge SHA 093a2adfa846f0ad0e7218074d34821f1825d771 at the observed UTC merge time. |
| 13 | Empty check rollup is described as CI success. | The entry says the PR rollup was empty and main required status checks were not enabled. |
| 14 | Old planning-only text remains for this slice. | The status row and plan status now say implementation and simulator matrix complete. |
| 15 | README and public roadmap link to an uncommitted or missing artifact. | Every referenced file exists in the merged PR tree and the GitHub PR body image URLs loaded successfully. |
| 16 | A static image URL is malformed or points at another run. | Both links resolve to the pass02 screenshots captured by the accepted 6/6 run. |
| 17 | The plan update implies all Stage A or C11.4 work is complete. | It limits the result to six fixed direct-text positive scenes and preserves the remaining scope. |
| 18 | The public roadmap and official STATUS disagree on completion state. | Both preserve C11.4/C11 incomplete and 72/213. |
| 19 | Tailnet publication is claimed before the docs merge is deployed. | This report explicitly leaves deployment verification to the post-merge Agent Browser sync. |
| 20 | Unrelated row-axis run artifacts or status entries are swept into this update. | The docs diff changes the generic direct-text entry, its plan status, its evidence README, the related roadmap paragraph, and this review artifact; unrelated row-axis runs are not included. |

The review is based on the merged PR tree and GitHub metadata. It does not claim that the Tailnet preview has already been updated; that requires its own preflight, selected build, safe copy/backup, served-hash check, Agent Browser verification, and browser-process close record.
