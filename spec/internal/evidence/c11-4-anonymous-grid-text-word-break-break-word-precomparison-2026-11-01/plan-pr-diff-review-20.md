# C11.4 `word-break: break-word` plan PR diff review · 20 perspectives

**Reviewed base:** `7bc1a7710a93420dd199d9436eeae6a67264f57f` (`origin/main` before this PR). **Scope:** roadmap corrections, the proposed plan, independent browser/oracle artifacts, and this review record. No product/runtime code is in the candidate diff.

## Review

| # | Diff failure perspective | Check and finding | Result |
| ---: | --- | --- | --- |
| 1 | Scope contamination | The candidate changes plan/evidence files only; there are no Rust, native, public API, or fixture runtime implementation changes. | Pass |
| 2 | Parent C11 progress overstatement | The C11 plan keeps C11.3.3, C11.4, C11, WPT runner, and public CSS support explicitly incomplete. | Pass |
| 3 | Official completion drift | The candidate repeatedly retains official `72/213`; it does not check a C11 or C11.4 status box. | Pass |
| 4 | Stale C11.3.3 roadmap pointer | The former next-candidate line named PR #970 work as pending; it now records #967/#970 as merged and points to the C11.4 candidate. | Pass |
| 5 | Stale text-indent history | The track-sizing plan formerly described the `hanging`/`each-line` slice as a plan-only candidate; it now records merged PR #959 and its scoped evidence. | Pass |
| 6 | Historical evidence link accuracy | The changed text-indent record initially pointed its implementation review into the evidence directory, but the artifact lives under `plan/`; the link was corrected to the existing file. | Fixed; pass |
| 7 | Status-link fragment validity | The new plan initially used `#c11-css-grid` on a checklist item with no heading anchor. The fragment was removed while keeping the direct status-file link. | Fixed; pass |
| 8 | Local-link breakage | A scan of added/updated Markdown lines checked 29 relative links, including the new plan, evidence, roadmap, and review links; all resolve. | Pass |
| 9 | Plan/oracle divergence | The plan and evidence both specify 18 scenes from two `word-break` values, three `overflow-wrap` values, and three track types. | Pass |
| 10 | CSS rule reversal | The documentation correctly separates deprecated `word-break: break-word` (effective `anywhere`) from `overflow-wrap: break-word` min-content behavior. | Pass |
| 11 | Expected-value transcription | The six property-pair expectations distinguish min-content (10px/40px), max-content (40px), and fixed-track row/line behavior. | Pass |
| 12 | Missing-scene substitution | The first verifier allowed a duplicate row to replace a missing scene. It now asserts the exact unique 18-ID set and checks each ID against its property pair and track. | Fixed; pass after rerun |
| 13 | Authored/computed mismatch | The verifier compares computed values with each planned pair, and the implementation plan requires validating both computed values from one owner snapshot. | Pass |
| 14 | Geometry-only false pass | The verifier checks computed columns/rows, root dimensions, line count, line widths, and line heights, not only one outer box. | Pass |
| 15 | Font substitution | The fixture points to pinned repository Ahem; the evidence records its SHA-256 and asserts the browser loaded the font. | Pass |
| 16 | Browser provenance | Raw data records HeadlessChrome 156.0.0.0, DPR 1, CSS support, computed properties, track strings, and all scene observations. | Pass |
| 17 | Screenshot omission or unsupported claim | The PR can include the captured Agent Browser matrix screenshot; README limits it to browser precomparison and does not call it Spinon output. | Pass |
| 18 | Browser process lifecycle | The lifecycle record says the Agent Browser session was closed and the post-close process check found none. | Pass |
| 19 | Verification overclaim | The evidence reports only the 18/18 Chrome-to-independent-oracle comparison; it explicitly excludes Spinon, WPT, physical-device, hardware-GPU, and full C11 claims. | Pass |
| 20 | Review-record integrity | This file contains 20 distinct diff checks, identifies the reviewed base, and distinguishes plan/oracle review from later implementation/runtime and PR reviews. | Pass |

## Decision

Three diff issues were corrected before PR creation: exact scene-set validation, the invalid status anchor, and the text-indent review link. The corrected observation verifier was rerun successfully (18/18); changed-line relative links resolve (29/29); `git diff --cached --check` passes. No remaining diff blocker found. This is a plan PR review; it does not substitute for the implementation/runtime review or the later implementation PR diff review.
