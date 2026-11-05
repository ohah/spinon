# C11.1 fixed-px gap post-merge documentation · PR diff review 30

**Review scope:** the post-merge status, internal contract, plan, and public roadmap edits that record PR #1172's merge metadata and preserve the bounded feature scope. The implementation PR's 30-point review is separate.

| # | Hostile perspective | Check and result |
|---:|---|---|
| 1 | Does the merge SHA identify the actual GitHub merge commit? | Compared with `gh pr view 1172`; `ee31e1607eda4d80f0a0dcc49f7656c3b693f249`. **Pass.** |
| 2 | Is the reported merge timestamp exact and in UTC? | Compared with `mergedAt`; `2026-11-05T04:18:10Z` is retained verbatim. **Pass.** |
| 3 | Could the feature commit be mistaken for the merge commit? | Docs label the merge SHA explicitly; the PR head SHA remains separately available on GitHub. **Pass.** |
| 4 | Does the status still say the implementation is under review? | The stale phrase is replaced with the merged state and timestamp. **Pass.** |
| 5 | Does the roadmap say the PR is open or draft? | The card labels #1172 “Rebase and merge” and gives the merge SHA. **Pass.** |
| 6 | Does the internal contract still describe the merged follow-up as unmerged? | Its section title and final paragraph now say bounded merged follow-up and include the merge record. **Pass.** |
| 7 | Does the focused plan still imply no implementation exists? | A merge record is added before the plan details, with the exact bounded result. **Pass.** |
| 8 | Did the docs incorrectly mark C11.1 complete? | Status, plan, contract, and roadmap all keep C11.1 incomplete. **Pass.** |
| 9 | Did the docs incorrectly mark C11 complete? | The parent C11 checkbox remains unchecked and all four records say C11 is incomplete. **Pass.** |
| 10 | Did the docs incorrectly mark C11.4 complete? | The C11.4 checkbox remains unchecked and the bounded slice is not promoted. **Pass.** |
| 11 | Did this limited slice increase the official completion count? | Each record retains `78/214`; no checklist item was promoted. **Pass.** |
| 12 | Does the updated status retain the allowed CSS values? | It keeps finite non-negative fixed-px shorthand and longhand support. **Pass.** |
| 13 | Does any summary accidentally imply percentage-gap support? | Percentage remains an explicit property-level rejection. **Pass.** |
| 14 | Does any summary accidentally imply `calc()` gap support? | Math-backed `calc()` remains rejected by the bounded profile. **Pass.** |
| 15 | Does the docs sync erase CSS-invalid negative fallback behavior? | The internal contract still distinguishes Stylo fallback from runtime rejection. **Pass.** |
| 16 | Is gutter aggregate overflow omitted from the merged record? | Status, contract, and implementation evidence retain the axis-specific fail-closed guard. **Pass.** |
| 17 | Is track-plus-gutter overflow reduced to aggregate-only checking? | The docs retain the resolved finite track-position validation before frame collection. **Pass.** |
| 18 | Could the status imply partial frames are published on failure? | The contract retains atomic rejection and no partial scene publication. **Pass.** |
| 19 | Does the roadmap overstate the Android result as a physical-device result? | It identifies API 37 ARM64 AVD and excludes PixelCopy/hardware claims. **Pass.** |
| 20 | Does the roadmap overstate the iOS result as a device or post-compositor pixel result? | It identifies iOS Simulator visual smoke and excludes Metal readback. **Pass.** |
| 21 | Are simulator boxes and scene counts inconsistent with evidence? | The card says `layout=ready boxes=23`; runtime evidence records the same. **Pass.** |
| 22 | Is the Chrome comparison conflated with Spinon runtime? | The roadmap and plan keep Chrome 156 as comparison evidence, separate from app runs. **Pass.** |
| 23 | Are focused test counts changed by this docs-only sync? | Layout 6/6 and runtime 10/10 remain identical to their recorded logs. **Pass.** |
| 24 | Is the broad C15 failure omitted or reported green? | Runtime evidence and PR body retain 447/448 and the C15 failure. **Pass.** |
| 25 | Are GitHub checks falsely described as passing? | The merge record says there were no check runs and no required checks. **Pass.** |
| 26 | Do relative links resolve from the plan and contract directories? | Checked the plan, runtime evidence, precomparison, and review paths against tracked source files. **Pass.** |
| 27 | Does the public roadmap link to a review file that is absent from source? | The link resolves to the tracked Markdown review file; the later selected publication and Agent Browser checks will verify the rendered route before Tailnet sync. **Pass.** |
| 28 | Does the new post-merge evidence contradict the implementation review? | It links to the implementation review and repeats only its bounded scope and recorded merge state. **Pass.** |
| 29 | Does contract versioning change without a public-interface change? | Internal contract version `0.1.0` is retained; no API or parser claim was added. **Pass.** |
| 30 | Does the new evidence conceal the failed full docs build or claim broader validation? | The runtime evidence records the full build failure outside this diff; the sync does not call it a pass. **Pass.** |

## Result

All 30 distinct documentation failure perspectives passed. No broader CSS Grid support, official completion, CI success, or simulator pixel-readback claim was introduced.
