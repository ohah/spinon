# C11.4 overlapping span PR diff review · 20 checks

**Review scope:** the implementation, simulator evidence, plan and status changes in the implementation PR. The conclusions below refer to the final diff and linked artifacts, not to unrelated untracked material in the shared checkout.

| # | Diff risk | Finding |
|---:|---|---|
| 1 | Unrelated working-tree files get included | Only the 12 implementation paths, focused plan/status edits, final run directories, and named logs are intended for staging. Existing evidence trees stay untouched. Pass. |
| 2 | New Rust modules sit outside the fixture boundary | FFI, runtime, and style-to-layout files are under their established fixture directories; module-boundary check passes 6/6. Pass. |
| 3 | Scene-count edits hide a missing or duplicate case | Six IDs are unique in fixture registries and six capture cases are appended; capture asserts the new total 245. Pass. |
| 4 | Simulator harness accepts arbitrary IDs by prefix alone | Prefix checks select expected-value logic, while exact ID sets and runtime fixture allowlists gate execution. Pass. |
| 5 | Android and iOS encode different scene matrices | Both platforms contain the same six IDs, widths, expected frames, intrinsic inputs, and sample coordinates. Pass. |
| 6 | Reversed scenes report the wrong source order | Expected node ordering and intrinsic input arrays are reversed consistently with the fixture scene. Pass. |
| 7 | Expected track totals conflict with the plan | Rust tests, native harnesses, capture JSON, and independent plan agree on `[42,47,47]` and `[32,32,32]`. Pass. |
| 8 | Pixel samples omit the in-root control or beyond-root evidence | Both small scenes include red `(4,7)`, red `(126,7)`, and clear `(150,7)` points with node IDs. Pass. |
| 9 | Pixel oracle uses a fallback/default color for new IDs | Android expected RGB maps all three suffixes; iOS binds exact RGBA per sample. Final simulator logs are exact. Pass. |
| 10 | Tailnet or PR image links refer to transient local files | The PR body uses committed raw GitHub screenshot paths after the push. Tailnet sync is a separate follow-up and no local preview-cache paths are added to this implementation diff. Pass. |
| 11 | Screenshots are from an earlier source revision | Both simulator manifests contain the same 12-path tracked diff digest; the final image paths are in those run directories. Pass. |
| 12 | Build evidence omits the opt-in route that was actually executed | Android and iOS final build logs record fixture-enabled builds; AVD was rerun after enabling the missing Android fixture flag. Pass. |
| 13 | Build or runtime failures are silently called successes | The README distinguishes the initial feature-disabled route and first incomplete sample oracle from the final successful rebuild and rerun. Pass. |
| 14 | Focused checks are presented as full workspace CI | The README names focused tests and crate Clippy only; it does not claim full CI or a full Rust workspace run. Pass. |
| 15 | Default release artifact accidentally includes fixture payload | Static archive string scan finds all six IDs and marker absent in default and present in opt-in control. Pass. |
| 16 | Artifact evidence claims more than it proves | The README calls it a static-archive string scan and excludes bundle and symbol-table audit claims. Pass. |
| 17 | Plan/status converts a candidate into full C11.4 support | C11.4 remains unchecked; official count remains `77/213`; public CSS admission and full Grid remain out of scope. Pass. |
| 18 | Simulator evidence is described as physical or compositor output | AVD software renderer and iOS pre-submit Metal boundaries are explicit; no physical Android device is claimed. Pass. |
| 19 | Overflow evidence is confused with CSS border painting | `(126,7)` is described as red text outside the root border; border stroke pixels are explicitly excluded. Pass. |
| 20 | PR description omits user-visible simulator screens | PR body includes one final Android AVD and one final iOS Simulator screenshot, with run links and limits. Pass. |

No blocking diff issue was found. After push, the screenshot links and PR rendering still require Agent Browser confirmation; that check is not inferred from local file existence.
