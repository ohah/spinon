# Plan PR diff failure review: C11.4 anonymous Grid author `tab-size`

Reviewed staged plan PR delta: the bounded candidate plan, its separate 20-perspective plan review, C11.4 navigation/status updates, the 13-case browser fixture, independent expectations, Agent Browser observation, screenshot, and supporting evidence README. These are 30 distinct PR-diff failure hypotheses; they are separate from the plan's 20 failure perspectives and from the later implementation/runtime review.

| # | PR-diff failure hypothesis | Review result |
|---:|---|---|
| 1 | The patch changes product behavior before plan approval. | The staged paths contain no Rust, Android, iOS, or runtime implementation source. **Pass.** |
| 2 | The patch accidentally marks C11 or C11.4 complete. | Both parent checkboxes remain unchecked; the new status note says implementation has not started. **Pass.** |
| 3 | The roadmap total silently increases above 72/213. | No checkbox or task-ID count input is added; the official `72/213` text remains unchanged. **Pass.** |
| 4 | The overall 72/213 count is presented as a C11 percentage. | Plan, evidence, and status explicitly distinguish the global count from C11 progress. **Pass.** |
| 5 | The supported numeric values drift between plan, fixture, expected values, and status. | The candidate profile is 1/2/4 plus existing default 8; the 13 fixture and expected cases agree. **Pass.** |
| 6 | The plan claims arbitrary `tab-size` support. | Zero, fractional, length, other values, and general CSS admission are explicit exclusions with fail-closed acceptance. **Pass.** |
| 7 | Valid but excluded CSS values are described as invalid CSS. | The docs call them valid CSS forms outside this bounded fixture profile. **Pass.** |
| 8 | The CSS Text reference points to a non-primary or wrong module. | The plan links the W3C CSS Text Level 3 definition and names §4.1.2/§4.2. **Pass.** |
| 9 | Browser CSSOM is used as the normative expected-value source. | Independent `10N` interval arithmetic is recorded separately from HeadlessChrome observations. **Pass.** |
| 10 | The plan confuses screenshot/range data with glyph raster proof. | Evidence explicitly excludes glyph ink and platform pixel claims. **Pass.** |
| 11 | The wrong Ahem resource is loaded or the font hash is stale. | Fixture relative URL resolves to the stored Ahem resource; recorded hash matches the file. **Pass.** |
| 12 | The Chrome fixture fails to load Ahem and falls back silently. | Raw observation reports `fontLoaded: true` and the browser is HeadlessChrome 156. **Pass.** |
| 13 | Input fixture, oracle, and screenshot hashes do not identify this patch's artifacts. | All four hashes in the README were recomputed from the staged input, expected JSON, observation JSON, and PNG. **Pass.** |
| 14 | JSON `\\t`/`\\n` sequences accidentally represent literal backslashes instead of source controls. | Decoded inputs contain actual tab, LF, and CRLF characters; independent arithmetic agrees with all expected normalized strings. **Pass.** |
| 15 | The browser observation omits a planned case or contains duplicate IDs. | The 13 unique fixture IDs, expected rows, and observation rows match one-for-one. **Pass.** |
| 16 | Computed tab-size does not match the case's claimed value. | Every observation's computed value matches the fixture profile 1, 2, or 4. **Pass.** |
| 17 | Root frame or used Grid track differs from the expected table. | An independent comparison confirmed root width/height and one-column/one-row sizes agree for 13/13 cases. **Pass.** |
| 18 | Exact-stop arithmetic is only tested at the old default value. | Value-1, value-2, and value-4 exact-stop controls each distinguish the next stop. **Pass.** |
| 19 | Consecutive tabs are conflated with one wider tab. | The value-2 scene records sequential tab fragments and a 70px total line. **Pass.** |
| 20 | LF reset is tested but CRLF normalization is not represented. | The fixture injects CR and LF code points, records the two-code-unit input, and expects one 20px-high two-line Grid. **Pass.** |
| 21 | A trailing preserved tab is omitted from max-content sizing. | A dedicated trailing-tab scene expects 40px and remains in both plan and oracle table. **Pass.** |
| 22 | Min-content or fixed-width controls silently wrap `pre` content. | Min-content is 60px with one line; the 25px fixed track separately retains 60px unwrapped text advance. **Pass.** |
| 23 | The status note reads runtime scenes as completed implementation. | It describes only precomparison, independent values, and plan review; implementation is explicitly not started. **Pass.** |
| 24 | The new plan is unreachable from the existing C11.4 documentation. | It is linked from `spec/STATUS.md` and the central C11.4 track-sizing plan. **Pass.** |
| 25 | Local links in the new plan, review records, or evidence README are broken. | All relative local links from the four new documents resolve. **Pass.** |
| 26 | The 20-perspective plan review is counted as the plan PR diff review. | This file defines a separate 30-perspective PR diff review; both review artifacts remain distinct. **Pass.** |
| 27 | The patch includes unrelated dirty evidence or broad staging. | The PR delta is the nine plan/evidence paths plus this review; no prior untracked evidence was staged. **Pass.** |
| 28 | Repository whitespace or patch errors are hidden by Markdown content. | `git diff --cached --check` reports no errors. **Pass.** |
| 29 | Terminal Browser or an orphan Agent Browser process remains. | Browser evidence was captured through Agent Browser; the session was closed and no exact `agent-browser` process remained. **Pass.** |
| 30 | Plan-only work claims simulator, CI, or implementation gates were run. | No product tests, mobile builds, simulator runs, or CI passes are claimed in this plan PR. **Pass.** |

**Plan PR diff review: 30/30 addressed.** The review applies to this staged patch before commit; verify the final pushed head still matches the reviewed paths and content before merging.
