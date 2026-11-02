# C11 root HTML status-sync PR diff review · 20 failure perspectives

Scope: the summary and status paragraph in `plan/c11-css-grid.md`, plus this evidence note. Each check targets a separate documentation or evidence failure.

| # | Failure perspective | Check and result |
|---:|---|---|
| 1 | The PR number could point to a different change | Checked PR #1030 against the local status ledger; it is the named-area plus two-axis `fr` implementation. PASS. |
| 2 | A closed PR could be described as merged | Verified PR #1030 is merged. PASS. |
| 3 | The merge SHA could be copied from the head commit | Compared `579435b1955e7ad157baa13d100d75f13a5dd283` with the merge SHA recorded in `spec/STATUS.md` and the implementation plan. PASS. |
| 4 | The merge date could be wrong | Both records identify the merge on 2026-11-02. PASS. |
| 5 | The area map could be described too broadly | The summary names only the fixed 2×2 map and points to its implementation plan. PASS. |
| 6 | The `fr` track factors could be misreported | The summary states direct `1fr 2fr`, matching the implementation record. PASS. |
| 7 | The supported axes could be overstated | It says both definite axes; it does not claim indefinite-axis sizing. PASS. |
| 8 | The implementation could be mistaken for the C11 fixture profile | The summary explicitly names the default parsed Root HTML route. PASS. |
| 9 | A plan-only claim could survive in the current summary | Replaced the stale sentence with the merged implementation state. PASS. |
| 10 | An evidence link could resolve to a plan without runtime evidence or point outside the repository | Confirmed the implementation plan and its runtime evidence link resolve; also checked the new status-sync evidence path exists. PASS. |
| 11 | Android AVD evidence could be mislabeled as a physical device | It is labeled Android API 37 ARM64 AVD. PASS. |
| 12 | iOS evidence could be mislabeled as a physical device | It is labeled iPhone 17 Pro/iOS 26.2 Simulator. PASS. |
| 13 | Draw/present could be represented as pixel readback | The summary says draw/present and does not claim PixelCopy or Metal drawable readback. PASS. |
| 14 | The bounded slice could imply all of C11.1 is complete | The summary explicitly keeps C11.1 incomplete. PASS. |
| 15 | A C11.1 slice could imply parent C11 completion | The summary explicitly keeps C11 incomplete. PASS. |
| 16 | The overall completion denominator could be changed accidentally | The official `72/213` is stated unchanged, matching `spec/STATUS.md`. PASS. |
| 17 | The earlier direct `fr` slice could be omitted from the parent status | Added PR #1015 to the Root HTML progress list. PASS. |
| 18 | The percentage-track slice could be omitted or attached to the wrong feature | Added PR #1019 alongside PR #1015 as the bounded percentage-track work. PASS. |
| 19 | The fixed implicit-row slice could be confused with arbitrary implicit sizing | Added PR #1024 as a limited fixed implicit-row slice. PASS. |
| 20 | The `fr`/percentage and implicit-row composition could be omitted | Added PR #1027 separately from its prerequisites and PR #1030. PASS. |

Result: 20/20 PASS. No code behavior or platform validation was inferred from this documentation review.
