# PR #1351 plan and Tailnet evidence diff review: 30 perspectives

Scope reviewed: the plan's merge-state and step 9 update, plus this folder's Tailnet publication and browser evidence. This review does not substitute for GitHub CI or product tests.

1. **Merge identity:** PR #1350 is linked to the observed merge SHA `9949b1e35a90c174e4ac67f37978b46ce60873b4`.
2. **Implementation merge identity:** PR #1349 retains its actual merge SHA `b0725665e31cc50ed6eb65a4083a1b94245dea36`.
3. **Merge method:** PR #1350 is described as Rebase and merge, consistent with the recorded merge state.
4. **Date and timezone:** GitHub timestamps are recorded in UTC rather than ambiguously labeled local time.
5. **CI boundary:** Zero GitHub checks remain explicitly distinguished from a green CI result.
6. **Official denominator:** The plan leaves the official count at `78/214`.
7. **C11.4 state:** The plan continues to say C11.4/C11 is incomplete.
8. **Internal contract version:** The plan leaves contract `0.1.0` unchanged.
9. **Bounded feature scope:** The five-fit implementation remains limited to the already-reviewed profile.
10. **Next candidate:** The separate six-fit candidate is not described as implemented or complete.
11. **Roadmap claim:** The deployed roadmap visibly carries PR #1349, its merge SHA, official count, and incomplete C11.4/C11 state.
12. **Broken route regression:** The previously missing dedicated plan URL now renders in Agent Browser.
13. **Publication input:** The route manifest is tied to source commit `9949b1e...` and its tree hash.
14. **Route denominator:** The publication and verification denominator is consistently 67 selected routes.
15. **Payload size:** Selected publication size is recorded as 13,678,567 bytes.
16. **Local integrity:** All 67 local served files match their selected source sizes and hashes.
17. **HTTPS integrity:** All 67 HTTPS routes return HTTP 200 and match their source sizes and hashes.
18. **Roadmap hash:** The raw `roadmap.html` SHA-256 is recorded as `8ce99fb9...`.
19. **Backup scope:** Exactly 8 replaced routes have backups under a new, task-specific backup root.
20. **Backup integrity:** All 8 backups are hash-verified against the pre-copy versions.
21. **No route deletion:** The copy and verification reports record zero deletions.
22. **Server continuity:** Preview PID `90711` remains unchanged; the sync does not restart the server.
23. **Tailnet identity:** The publication targets the user's configured `macstudio.tailed42f2.ts.net` host.
24. **Desktop browser evidence:** Both target pages were rendered at 1280-pixel viewport width with matching document/body widths.
25. **Mobile browser evidence:** Both target pages were rendered at 390-pixel width with matching document/body widths.
26. **Async roadmap state:** The mobile roadmap check waits for the official status block before capturing its screenshot.
27. **Browser selection:** Agent Browser is used; Terminal Browser is explicitly not used.
28. **Session isolation:** The browser uses a task-specific session name; unrelated browser sessions are outside the cleanup scope.
29. **Simulator distinction:** Shutdown/shared simulator state is not conflated with disposable cache files; the shared iPhone image is preserved.
30. **Evidence boundary:** Browser rendering and HTTP/hash checks are not presented as CI, full platform testing, or product feature completion.

## Review result

The evidence supports the completed PR #1350 Tailnet publication. The first plan screenshot exposed stale “merge-state sync in progress” wording; the plan change in this PR corrects that text and links to the publication evidence. No completion counter, C11.4 checkbox, or contract version changes.
