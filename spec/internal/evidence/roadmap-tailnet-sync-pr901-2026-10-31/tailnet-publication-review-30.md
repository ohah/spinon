# Tailnet publication evidence review · 30 perspectives

Scope: only the evidence for publishing merged PR #901's static roadmap HTML to the already-running local Tailnet preview aliases. This review does not certify the roadmap feature, C11, WPT, a simulator run, or CI.

1. **Wrong source revision:** the copied source is tied to `origin/main` `5292d162d4f11a50de69f4c7084707f732ec727d`, which contains merged PR #901.
2. **Wrong source file:** the selected repository source is `spec/public/roadmap.html`, matching the served roadmap page.
3. **Source changed during copy:** the atomic sync rechecked the selected source head and digest before replacing aliases.
4. **Stale source bytes:** the source has 437,642 bytes and SHA-256 `3330422c6a7f0ca76acca6453a1243840f0abe91113021498cd4a2ef63732e7e`.
5. **Wrong preview root:** the copy targets are under the running server's `SpinonPreview` root.
6. **Unintended file scope:** only `roadmap.html`, `public/roadmap.html`, and `spec/public/roadmap.html` were selected.
7. **Target missing or replaced:** preflight found all three targets present before the update.
8. **Symlink redirection:** preflight found regular target files rather than symlinks.
9. **Concurrent writer:** preflight found no process holding any target file open.
10. **Backup path collision:** the dedicated `pass01` backup directory was absent before the operation.
11. **Backup data loss:** all three previous files were preserved with their original 436,783-byte size and SHA-256 `f915c285c180257e14db51235e92dd8111ba0f837285c4ee8039f63ff52c539b`.
12. **Partial target write:** each replacement used a temporary file, flush/fsync, and atomic rename.
13. **Wrong destination content:** all three targets were read back and matched the source digest.
14. **Alias divergence:** the three target paths have identical byte counts and hashes.
15. **Unrequested cleanup:** no files were deleted.
16. **Wrong server process:** the preview server PID and command were recorded before the copy.
17. **Wrong listener:** the server's only observed listener was `127.0.0.1:8768`.
18. **Server restart side effects:** the existing preview server was left running; the update replaced static files only.
19. **Wrong live URL:** browser verification used `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`.
20. **HTTP failure:** direct HTTPS retrieval returned status 200.
21. **MIME mismatch:** the live response content type was `text/html; charset=utf-8`.
22. **Truncated response:** live response size was 437,642 bytes.
23. **Stale served copy:** live response SHA-256 matches the merged source digest.
24. **Missing roadmap change:** the served bytes contain the C11.2.2 computed Grid `order` note and PR #899 reference.
25. **Wrong rendered page:** Agent Browser reported the expected Korean roadmap title and Tailnet URL.
26. **Missing visible note:** the captured viewport shows the C11.2.2 order note beside the related Grid roadmap card.
27. **Incorrect completion total:** rendered text says `213개 표시 · 공식 완료 72/213`.
28. **Checkbox mutation:** rendered DOM had 213 checkboxes, 72 checked, and all 213 disabled; this publication did not edit completion controls.
29. **Overstated C11 completion:** the source status ledger keeps C11 and C11.4 unchecked; the new paragraph explicitly says full C11 remains incomplete.
30. **Browser left running:** Agent Browser was closed after the screenshot, and the follow-up process listing found no Agent Browser or Chrome browser process.

Conclusion: the publication and evidence are bounded to one merged roadmap HTML source and three pre-existing local preview aliases. The public completion count and implementation status remain unchanged.
