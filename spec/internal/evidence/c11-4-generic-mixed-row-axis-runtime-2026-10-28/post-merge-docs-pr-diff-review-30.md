# PR #713 post-merge roadmap synchronization · 30 diff failure perspectives

## Scope

This review covers the post-merge documentation diff for PR #713: `spec/STATUS.md`, the C11.4 parent and focused plans, the runtime evidence README, captured GitHub merge/protection/check records, and this review record. No implementation code or official completion checkbox changes are in scope.

The review used the captured GitHub API responses and a fresh `gh pr checks 713` observation. Three issues were corrected during review: the status ledger date was stale, the new merge SHA lacked a closing Markdown code delimiter, and the saved check-output file was empty. The final diff was checked again after those fixes.

## Failure perspectives

1. **Wrong PR association** — Confirm each new merge statement names PR #713 and links to `https://github.com/ohah/spinon/pull/713`. **PASS** — the plan, status ledger, and evidence README all link to the same PR.
2. **Wrong merge state** — Check that the PR is recorded as merged, not merely approved or closed. **PASS** — captured state says `MERGED`; docs say merged.
3. **Wrong merge method** — Check that Rebase and merge is stated accurately. **PASS** — the plan and status entry name Rebase and merge.
4. **Wrong merge timestamp** — Compare the recorded time with `pr-713-merge-state.json`. **PASS** — all use `2026-10-27T21:00:41Z`.
5. **Timezone drift** — Ensure the UTC timestamp is not silently relabeled as Korean local time. **PASS** — the exact `Z` timestamp is preserved.
6. **Wrong merge SHA** — Compare every new SHA with the GitHub `mergeCommit.oid`. **PASS** — all use `679b8d51bc3b4edc2cbd1c4f5921e3b73a12dabe`.
7. **Stale pending language** — Search the focused and parent plans for “review pending” state left over after merge. **PASS** — the current summary now says PR #713 merged.
8. **Empty check rollup misread as success** — Check that an empty rollup is not described as green CI. **PASS** — docs explicitly say no CI pass is claimed.
9. **CLI absence hidden** — Verify `gh pr checks 713` outcome is retained with its exit code and exact message. **PASS** — `pr-713-checks.txt` records exit 1 and “no checks reported”; this is absence of reported checks, not a pass.
10. **Invented required checks** — Compare branch-protection wording with the captured API response. **PASS** — the README says the response omitted configured required status checks; it does not infer a passing check.
11. **Review requirement overstated** — Check the configured approval count. **PASS** — the README reports zero required approving reviews from the captured response.
12. **Stale status-ledger date** — Check the header against the update date. **FIXED** — changed the ledger baseline from 2026-10-27 to 2026-10-28.
13. **Malformed merge-SHA markup** — Render the status entry around the SHA. **FIXED** — added the missing closing backtick and reran `git diff --check`.
14. **Official count accidentally promoted** — Search the changed status lines for an increment or new checked item. **PASS** — `72/213` remains; no checkbox was added or checked.
15. **Parent completion implied** — Check the prose around the C11.4 row slice. **PASS** — C11.4 remains explicitly incomplete.
16. **Scope inflated into general CSS support** — Check for an implication that the fixture enables author CSS generally. **PASS** — the docs retain the opt-in limitation and state that general CSS admission is unverified.
17. **Fixed and generic routes conflated** — Verify the row slice is described as preserving the fixed-ID route and adding generic routing. **PASS** — both route boundaries remain explicit.
18. **Novel IDs lost in summary** — Ensure the generic-adapter distinction survives in the official ledger. **PASS** — the status entry says it uses novel HostNode IDs.
19. **Scene count overstated** — Check the matrix description against the evidence README. **PASS** — 16 scenes per platform include fixed column, fixed row, generic column, and generic row controls.
20. **Android target drift** — Verify the target is still the API 37 ARM64 16 KB AVD, not a physical Samsung device. **PASS** — the evidence identifies the AVD and says ADB selected only `emulator-5554`.
21. **iOS target drift** — Verify simulator results are not called physical-device results. **PASS** — iPhone 17 Pro / iOS 26.2 Simulator is named.
22. **Focused runtime count drift** — Compare the ledger's 13/13 statement with the evidence README. **PASS** — both preserve 13/13.
23. **FFI count drift** — Compare the ledger's 84/84 statement with the evidence README. **PASS** — both preserve 84/84.
24. **Known Clippy failure erased** — Check that the strict Clippy failure remains visible. **PASS** — both existing `type_complexity` and `map_identity` lints remain documented.
25. **Lint rerun overclaimed** — Check that the rerun is not described as an unqualified strict pass. **PASS** — only those two lint classes were allowed; all other warnings remained denied.
26. **Archive isolation inflated** — Check that static-archive evidence is not called APK/IPA exclusion proof. **PASS** — the README explicitly limits that conclusion.
27. **Pixel evidence inflated** — Check that screenshots are not presented as glyph or image texel equality. **PASS** — the evidence describes host surface/layout boxes and preserves the rendering limitations.
28. **Algorithm distinction overclaimed** — Check the min-content/max-content statement against the fixed oracle. **PASS** — both are 20px here, so the docs say this matrix cannot distinguish the algorithms.
29. **Evidence link breakage** — Resolve the new relative links to the plan, PR state JSON, branch-protection JSON, and check log. **PASS** — all targets exist at the linked paths.
30. **Patch hygiene and unrelated-work damage** — Check whitespace and scope. **PASS** — `git diff --check` is clean; only the four intended tracked docs are modified, and unrelated untracked files were not staged or altered.

## Result

No blocking issue remains in the reviewed documentation diff after the three corrections above. This review records documentation and GitHub merge-state consistency; it does not add runtime, CI, physical-device, or full C11.4 evidence.
