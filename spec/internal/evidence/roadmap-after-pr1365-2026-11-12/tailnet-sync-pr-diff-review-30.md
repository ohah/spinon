# PR diff failure review: post-merge Tailnet roadmap sync

## Diff identity

- Source PR: #1366, merged into `main` as `5a7ed74d197c78d287cc12a0823a2abc77050f3f`.
- Tailnet preview root: `/Users/yoonhb/Library/Application Support/SpinonPreview`.
- Preserved server: PID `90711`, listening on `127.0.0.1:8768`.
- Scope: selected Rspress outputs, the public roadmap and linked C11.4 status/plan/evidence routes; no full site publish or path deletion.

## Review perspectives

1. **Merge source:** All source files came from merged `main` at `5a7ed74d197c78d287cc12a0823a2abc77050f3f`.
2. **Target identity:** The preview root is the server's verified cwd, not an inferred similarly named directory.
3. **Server ownership:** The pre-existing PID 90711 was confirmed listening before and after copy; it was not restarted.
4. **Destination scope:** The manifest lists each of the 89 destinations; no recursive copy targets the entire repository or entire preview root.
5. **No deletion:** The manifest records `deletedFileCount: 0`; no route or old preview file was removed.
6. **Backup isolation:** Existing preview bytes for each of the 14 replaced files were saved under one unique backup directory.
7. **Backup integrity:** Backup byte counts and SHA-256 values were checked before replacement and rechecked by the served verifier.
8. **New-file behavior:** 55 new routes/assets are classified as added, distinct from the 14 replacements.
9. **No-op behavior:** 20 identical files were left in place and recorded as unchanged.
10. **Path traversal:** Relative paths reject absolute paths, dot segments, and backslashes.
11. **Symlink defense:** Source files/trees and every existing destination path component are checked before reads or writes.
12. **Atomic install:** New content is written to a unique temporary sibling, hash checked, then renamed into place.
13. **Collision behavior:** Existing backup, manifest, and temporary paths cause an assertion instead of overwrite.
14. **Current roadmap bytes:** `/spinon/roadmap.html` matches the merged `spec/public/roadmap.html` source hash.
15. **Status source:** Root and `spec/STATUS.md` copies match the merged status source with official `78/214` unchanged.
16. **Status page:** Generated `STATUS.html` and `spec/STATUS.html` were rebuilt from the selected current source.
17. **Plan state:** The Tailnet track-sizing plan says PR #1365 merged and the next bounded feature candidate is unselected.
18. **Contract state:** The rendered C11 contract is built from the merged `spec/internal/0059-c11-grid-layout.md`.
19. **Runtime evidence routes:** The PR #1365 README, platform smoke, 20-perspective implementation review, and 30-perspective PR review were included.
20. **Precomparison routes:** The already-referenced six-fit precomparison evidence was hash-checked and refreshed as part of this selected slice.
21. **Build version:** The selected build used repository-pinned Rspress 2.0.22.
22. **Selected page gate:** Eight page routes and 14 referenced static assets were checked for expected PR/status markers and file existence.
23. **Build claim:** Full documentation build and full dead-link validation are explicitly marked unverified.
24. **Build-output lifecycle:** The task-owned build root was removed after copy and hash verification; the reproducible config, input manifest, build log, and output hashes remain.
25. **Manifest completeness:** 89 source-to-destination records account for every added, replaced, or unchanged path.
26. **Local HTTP:** All 89 published paths returned HTTP 200 through the preserved local server.
27. **Tailnet HTTP:** All 89 paths returned HTTP 200 through the exact user-provided Tailnet host.
28. **End-to-end content:** Source, preview file, local HTTP body, and Tailnet HTTP body SHA-256 values matched on all 89 paths.
29. **Desktop UI:** Agent Browser confirmed the latest #1365 summary, 78/214 count, C11.4/C11 incomplete state, and next-candidate status at 1440×1000 with page width 1440px.
30. **Mobile and cleanup:** Agent Browser confirmed the same status at 390×844 with page width 390px; task-created sessions and local candidate server were closed, and no iOS simulator was booted.

## Evidence

- Selected build: `selected-root.json`, `selected-build.log`, `selected-site-verification.json`.
- Tailnet copy and serving: `tailnet-copy-manifest.json`, `served-verification.json`, `served-verification.log`.
- Live browser: `tailnet-agent-browser-verification.json`, `tailnet-roadmap-desktop-agent-browser.png`, `tailnet-roadmap-mobile-agent-browser.png`.
- Cleanup: `browser-session-lifecycle.json`.
