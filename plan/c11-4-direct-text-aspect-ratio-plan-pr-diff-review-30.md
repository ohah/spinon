# C11.4 direct-text aspect-ratio plan PR delta review — 30 checks

**Scope:** plan-only PR delta on top of `origin/main` tree `57156c15a1d5c66a2ea2db57aac3d61ba2b35a5f`. This is separate from the 20-item plan failure review and does not assess implementation or simulator behavior.

| # | Adversarial PR-delta check | Result |
| --- | --- | --- |
| 1 | Does the PR contain production CSS admission or layout implementation hidden among docs? | PASS — changed product code is absent. |
| 2 | Does the scope explicitly say this is a new candidate rather than an already approved plan? | PASS — plan header says plan-only candidate. |
| 3 | Is the C11.4 parent plan linked from the child? | PASS — parent plan link is present. |
| 4 | Does the parent plan link back to the child? | PASS — focused-plan link is present in the dated parent entry. |
| 5 | Does the official status ledger link the child plan? | PASS — the C11.4 note links the plan. |
| 6 | Does the ledger preserve C11.4's unchecked state? | PASS — the parent checkbox remains unchecked. |
| 7 | Does this candidate alter the displayed completion count? | PASS — `72/213` is explicitly retained. |
| 8 | Does plan-only text claim successful Spinon execution? | PASS — no Spinon implementation or runtime is claimed. |
| 9 | Is the normative source a CSS Grid source at a named version? | PASS — CSS Grid 2025 CRD links are pinned. |
| 10 | Does the plan distinguish Grid item sizing from intrinsic track sizing? | PASS — §§6.2 and 11.5 are named separately. |
| 11 | Is the replaced-item limitation stated clearly? | PASS — the image transferred-size shortcut is explicitly excluded. |
| 12 | Does the plan link the independent precomparison artifact? | PASS — child plan, parent plan, and status ledger point to it. |
| 13 | Is browser observation labeled separately from the normative calculation? | PASS — README identifies Chrome as observation and shows independent expected values. |
| 14 | Is the browser's exact observed version recorded? | PASS — HeadlessChrome 156.0.0.0 appears in JSON and README. |
| 15 | Is the viewport/DPR recorded? | PASS — 1280×633 and DPR 1 are in JSON and README. |
| 16 | Is Ahem font loading verified rather than assumed? | PASS — the recorded font face status is `loaded`. |
| 17 | Do all four scene IDs agree across HTML, JSON, and README? | PASS — row ratio/control and column ratio/control names match. |
| 18 | Are ratio and no-ratio scenes paired? | PASS — row and column controls use the same content without a ratio. |
| 19 | Are both conversion directions independently covered? | PASS — 40 inline→20 block and 40 block→80 inline are listed. |
| 20 | Do expected dimensions match normalized captured tracks? | PASS — local track and item frames match the JSON capture. |
| 21 | Are root document positions mistaken for local item positions? | PASS — README explicitly normalizes local geometry and preserves raw capture. |
| 22 | Is the fixed ratio overgeneralized to arbitrary authored ratios? | PASS — only computed `2.0` is positive. |
| 23 | Does the plan accidentally accept arbitrary fonts, text, or descendants? | PASS — Ahem, text characters, direct-child shape, and item count are bounded. |
| 24 | Are dimensions, min/max, edges, and alignment interactions uncontrolled? | PASS — the accepted profile fixes each or excludes it. |
| 25 | Are simulator expectations represented as completed evidence? | PASS — Android/iOS runs are explicitly pending. |
| 26 | Is Android AVD renderer evidence likely to be misreported as device GPU? | PASS — planned gate names the AVD and limits the claim. |
| 27 | Is the iOS Metal capture likely to be described as post-compositor proof? | PASS — plan restricts the pixel claim to the sampled boundary. |
| 28 | Does the plan include fixture artifact boundary checks? | PASS — feature-off and fixture-test positive controls are required. |
| 29 | Does the plan state separate implementation/runtime and PR review gates? | PASS — independent 20-item implementation review and 30-item PR delta review are required. |
| 30 | Does the PR template scope support and remaining work accurately? | PASS — the plan and status distinguish a restricted fixture from public CSS support and C11.4 completion. |

**Result:** 30/30 PR-delta checks pass for the plan-only candidate. Any implementation or runtime change requires its own review and evidence; this report cannot be reused as that review.
