# C11.3.3 public roadmap note · adversarial diff review 30

- Review target: one new `spec/public/roadmap.html` progress entry, its local-source Agent Browser screenshot, and the evidence links, based on the already merged PR #816.
- Base commit: `19b5bf218b6595cd8d54c437cdda2152d02428b8`.
- Purpose: show the classifier correction on the Tailnet roadmap without changing support or completion state.
- Visual evidence: `roadmap-pr-preview.png` (1440×1100), captured from the modified local HTML source after confirming the title, PR link, and evidence links in Agent Browser. It is not a Tailnet-after-sync screenshot.
- Result: 30 distinct failure perspectives passed.

| # | Hostile review view | Result and evidence |
| ---: | --- | --- |
| 1 | Wrong PR referenced | PASS — note links to merged PR #816, the inventory correction. |
| 2 | Wrong merge identity | PASS — timestamp and merge SHA match the live PR record. |
| 3 | Date ordering | PASS — the note is inserted before the prior newest C11.3.3 entry. |
| 4 | Counts claim all WPT passed in Spinon | PASS — text explicitly calls it the fixed Chrome oracle. |
| 5 | Wrong WPT test count | PASS — current report records 4 tests and 73 subtests. |
| 6 | Chrome report rerun implied | PASS — wording describes the fixed report and classification correction only. |
| 7 | Original WPT runner implied | PASS — text says no Spinon WPT execution or conformance result. |
| 8 | WPT result confused with fixture support | PASS — wording says named mapping to derived scenes, not original test execution. |
| 9 | Mapping generalized to every case | PASS — it is bounded to the four pinned `.grid` subtests. |
| 10 | Trailing behavior generalized | PASS — text names positive trailing implicit named-line/span scenes. |
| 11 | Arbitrary implicit tracks implied | PASS — no general track-generation claim is made. |
| 12 | `unknown-area-implicit-tracks` status lost | PASS — its expected rejection is explicitly retained. |
| 13 | C11.3.3 promoted to complete | PASS — it remains a candidate. |
| 14 | C11 parent promoted | PASS — C11 remains incomplete. |
| 15 | Official count changed | PASS — `72/213` is explicitly unchanged. |
| 16 | C11.4 state changed | PASS — the entry does not claim or modify C11.4. |
| 17 | Public API support implied | PASS — entry describes inventory classification only. |
| 18 | Android/iOS reruns implied | PASS — no platform execution claims appear. |
| 19 | Historical PR #393 scenes misrepresented as new | PASS — the note labels them as previously verified derived fixture scenes. |
| 20 | Evidence path typo | PASS — internal README route matches the committed directory and `README.md` route convention. |
| 21 | Status link broken | PASS — link uses the existing `/spinon/STATUS.html#css-구현-체크리스트` anchor. |
| 22 | PR link destination malformed | PASS — link resolves to the canonical GitHub PR URL. |
| 23 | HTML nesting broken | PASS — entry is a single paragraph inserted between existing progress paragraphs. |
| 24 | HTML text injection | PASS — text uses escaped-safe plain content and existing `<code>`/`<a>` markup. |
| 25 | Underscore in rejection identifier misparsed | PASS — `unknown-area-implicit-tracks` is displayed as literal code text. |
| 26 | Evidence-only change changes completion checkbox | PASS — no checkbox, denominator, or status source is edited. |
| 27 | Duplicate public event entry | PASS — no existing entry for PR #816 was present at insertion. |
| 28 | Root route mismatch | PASS — the note lives in the source served as `/spinon/roadmap.html`. |
| 29 | Link to unmerged evidence | PASS — evidence README and PR #816 were merged before this change. |
| 30 | Diff exceeds intended scope or screenshot is misrepresented | PASS — source change is one roadmap paragraph; screenshot shows the local source preview and is labeled separately from Tailnet verification. |
