# PR diff hostile review · 30 independent cases

Scope: the PR #1202 documentation reconciliation, its Agent Browser/Tailnet evidence, and the bounded preview-copy script. Each case checks a distinct failure path or contract boundary.

| # | Failure-oriented check | Result |
|---:|---|---|
| 1 | Official status accidentally marks C11.4 complete | PASS — the top-level C11.4 checkbox remains unchecked. |
| 2 | C11 parent is accidentally promoted | PASS — the C11 parent checkbox remains unchecked. |
| 3 | Completion count increases because a slice merged | PASS — status and roadmap remain at `78/214`. |
| 4 | Internal version changes without a versioned contract decision | PASS — internal contract `0.1.0` is preserved. |
| 5 | PR #1202 merge date is stale or copied from another PR | PASS — `2026-11-06T00:43:14Z` matches GitHub. |
| 6 | PR #1202 merge SHA is not the actual merge commit | PASS — `009da4f159836139205831c80f8163cb4af52b50` matches GitHub and the checked-out base. |
| 7 | Merged PR #1199 is still described as an unmerged candidate | PASS — its internal heading and closing status now say merged. |
| 8 | PR #1199's merge identity is wrong | PASS — timestamp and merge SHA match GitHub PR data. |
| 9 | Roadmap cards are duplicated or omitted | PASS — #1187, #1190, #1195, #1199, and #1202 each occur once. |
| 10 | New roadmap card is placed after older C11.1 entries | PASS — #1202 follows #1199 and precedes the older #1156 card. |
| 11 | PR #1202 card implies general CSS Grid admission | PASS — it describes the bounded default parsed Root HTML route and leaves C11.4/C11 incomplete. |
| 12 | Row expectations lose the reverse-source-order control | PASS — both `[10,20,10,34]px` and `[10,10,20,34]px` are recorded. |
| 13 | Track-cap behavior is omitted or reversed | PASS — 10,000 rows are accepted and 10,001 is rejected before layout. |
| 14 | Style-to-layout focused count is misstated | PASS — the changed suite is 7/7; broad suite is separately reported 422/10. |
| 15 | Runtime focused count is misstated | PASS — the changed RuntimeSession suite is 11/11; broad suite is separately reported 288/8/3 ignored. |
| 16 | Layout focused count is misstated | PASS — intrinsic layout tests are 12/12; unchanged broad package is 453/1. |
| 17 | Clippy/rustfmt claims exceed recorded evidence | PASS — only the recorded workspace Clippy and rustfmt checks are described as passing. |
| 18 | Broad Rust suites or the Rspress docs build are presented as green | PASS — all broad Rust failure counts are explicit and the docs build exit 1 from repository-wide dead links is summarized; neither is called green. |
| 19 | Base-source recheck is overgeneralized to every failure | PASS — only style/runtime failures are attributed to base replay; the layout C15 failure is separately identified. |
| 20 | Android evidence is mislabeled as a physical-device run | PASS — it is identified as API 37 ARM64 AVD `emulator-5554`. |
| 21 | iOS evidence is mislabeled as physical-device evidence | PASS — it is identified as iPhone 17 Pro Simulator on iOS 26.2. |
| 22 | Fixture-only route is confused with product route | PASS — evidence says default Root HTML route, both native C11 fixture flags off, and `layout=ready boxes=8`. |
| 23 | Screenshot dimensions or platform assignment are swapped | PASS — Android is 1080×2400 and iOS is 1206×2622. |
| 24 | Roadmap and PR-body screenshot links point to missing assets | PASS — both simulator assets load from merge SHA `009da4f…`; Agent Browser confirmed all four images in PR #1203 load completely at the recorded dimensions. |
| 25 | Screenshot evidence is represented as pixel readback | PASS — screenshots are called visual smoke; PixelCopy and Metal drawable readback are explicitly excluded. |
| 26 | Hardware/compositor parity is implied | PASS — physical Android GPU, hardware GPU, and post-compositor pixels are excluded. |
| 27 | WPT or CI success is implied | PASS — neither WPT nor CI success is claimed. |
| 28 | New parser, public API, FFI, or runtime profile is implied | PASS — the records say these were not added and preserve the existing bounded admission. |
| 29 | Internal evidence or roadmap status links point to missing paths/anchors | PASS — new Markdown targets and `#c11-css-grid` exist; raw screenshot assets load from the merged SHA. Agent Browser also confirmed Tailnet `STATUS.html` resolves and still shows `78/214` with C11.4 incomplete. Rspress still reports unrelated repository dead links, summarized separately. |
| 30 | Tailnet sync can overwrite an unrelated file, lose the old page, or leave browser processes running | PASS — sync uses the checked `roadmap.html` target, refuses existing backup/report paths, saves a verified backup, atomically renames one file, and does not restart/delete; Agent Browser sessions are closed and process-checked after capture. |

No C11.4 completion or official-count update is implied by these documentation changes.
