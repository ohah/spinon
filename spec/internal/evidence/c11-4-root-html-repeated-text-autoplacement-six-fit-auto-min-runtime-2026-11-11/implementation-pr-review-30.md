# C11.4 six-fit implementation PR review: 30 failure perspectives

**PR:** [#1354](https://github.com/ohah/spinon/pull/1354)
**Reviewed head:** `84cfd095b82a48c8b401728e0cbb0373e064c572`
**Base:** `main` at `94548de2e0f28b7cc27839a7f1071cf9e499b2bf`
**Diff identity:** `gh pr diff 1354` and `git diff origin/main...HEAD` were byte-for-byte identical as plain text; both SHA-256 values were `1769d27081da2daeb151a416b638fe04a76eda1fba4daf54be7bec3ea685b3b4` (3,538 lines). The Git binary patch representation is intentionally excluded from this textual comparison; the PR includes the two checked-in PNGs.

| # | Adversarial check | PR diff result |
| ---: | --- | --- |
| 1 | PR targets the intended feature branch and current `main` base | PR #1354 is open from `feat/c11-4-six-fit-auto-minimum-20261111` to `main`; head/base SHAs above match local refs. |
| 2 | Remote PR diff omits a local changed file or adds an unreviewed file | The 56-file GitHub path list matches the local base-to-head path list; the plain diff hash matches exactly. |
| 3 | A public API, FFI, or versioned ABI changes accidentally | No public API, FFI, ABI, or interface version file is modified. |
| 4 | The change broadens ordinary CSS Grid admission | The new route is gated by the exact Root HTML repeated-text profile and the separate six-fit predicate. |
| 5 | Six-fit accepts a list with the wrong member count | Both style projection and layout predicates require exactly six members; wrong lengths are rejected. |
| 6 | Any non-fit member is accepted | All six members must be typed `FitContent`; layout and style tests reject an `Auto` member. |
| 7 | Authored order must match one hard-coded order | The predicate maps caps to slots independent of sequence; tests cover all 720 orders and runtime scenes verify computed order. |
| 8 | Duplicate cap substitutes for a missing member | The seen-slot array rejects repeats; duplicate px and percentage cases are explicitly tested. |
| 9 | 20% is confused with a px value or a different percentage | The percentage slot requires typed `Percentage(0.2)`; matrix checks typed and serialized computed values. |
| 10 | An unlisted finite cap is admitted | Exact-cap tests and RuntimeSession cases reject 5px, 60px, and unsupported percentages. |
| 11 | Negative, NaN, or infinity bypasses the numeric gate | Predicate tests cover negative and non-finite caps; runtime rejection covers negative values. |
| 12 | An unresolved or wrong `calc()` is normalized into the profile | Runtime accepts only normalized absolute 30px; unresolved percentage calculation is rejected. |
| 13 | Existing one- through five-fit support regresses | Corrected one-, two-, three-, four-, and five-fit regression logs pass. Stale five-fit rejection assertions were replaced with genuinely unlisted six-fit inputs. |
| 14 | Four-fit or row-span behavior is silently routed through six-fit | Existing row-span and four-fit controls/rejections remain separate; their focused regression gates pass. |
| 15 | More than one visible Grid owner receives the bounded route | Root HTML validators require exactly one owner; second-owner mutation fails without layout publication. |
| 16 | The exception accepts the wrong nine-item tree | The root requires one bare 10px control plus eight single-payload text items; missing/replaced/nested items reject. |
| 17 | The first control contributes automatic minimum | Control remains `min-height:0`; a control-auto mutation rejects. |
| 18 | Mixed minimum values are accepted for repeated items | All eight repeated items must be `auto`; the mixed-minimum mutation rejects. |
| 19 | Text payload support leaks to arbitrary font/content topology | Text, font-size, line-height and direct payload shape remain bounded; unsupported font and line height reject. |
| 20 | Scrollable overflow incorrectly receives the auto minimum | Grid and item overflow must remain visible; both scrollable overflow cases reject. |
| 21 | Style validation and layout validation admit different cap sets | Both layers use the same exact six typed values; exact profile tests pass in both crates. |
| 22 | The Root HTML route publishes partial output after invalid input | Invalid layouts publish no completed frames/render output; rejection tests assert fail-closed behavior. |
| 23 | A rejected revision leaves stale output or blocks recovery | Same-session dark mutation fails, carries the invalid environment revision in computed styles, then light recovery publishes a new key and expected frames. |
| 24 | Track-sizing trace is attached to the wrong Grid/revision | Trace test matches the single owner and completed cascade key for the same revision. |
| 25 | One or more text items lacks an automatic-minimum contribution | Trace verifies exactly one contribution for each of eight items and checks each content-based floor. |
| 26 | The final row base size still truncates a contribution | Trace finds the final stretch stage and confirms each eligible row base is at least its contribution. |
| 27 | Matrix claims parity while skipping a cap order, alignment, DOM order, or min mode | Matrix asserts 720 track lists, 2,880 paired baseline/candidate keys, `start|stretch`, `forward|reverse`, and `0px|auto`. |
| 28 | Frame parity is shallow or compares mismatched revisions | Every scene compares same-revision typed/serialized styles, Grid, nine item frames/rows, eight payload frames, glyph count and line count to Chrome records. |
| 29 | Aggregate counts hide a floor or overflow failure | Per-item comparisons precede totals; matrix asserts 23,040 candidate items, zero candidate floor/overflow/Grid-end failures and the expected baseline failures. |
| 30 | PR screenshots or platform evidence imply six-fit GUI verification | Agent Browser loaded both image URLs in the PR page (`complete=true`, 1206×2622); PR body labels them default-app smoke. Android AVD startup failure and the lack of six-fit-specific app geometry are stated explicitly. |

**Result:** no PR diff defect found. PR has no configured status checks (`statusCheckRollup: []`); local evidence is recorded separately and this review does not mark C11.4/C11 complete.
