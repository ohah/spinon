# PR #1352 Tailnet publication evidence review: 30 perspectives

Scope reviewed: the final PR #1351 publication records and Agent Browser screenshots. This review checks evidence consistency and cleanup boundaries; it does not substitute for GitHub CI or product tests.

1. **Source commit:** The selected publication is tied to merged `main` at `07cfec2968c571e3cd150fe3a32ec91a3d4841d4`.
2. **Source tree:** The selected root records tree `af4390492087543242371be2b781b01f5d32d619`.
3. **Merge state:** PR #1351 is described as Rebase and merge with its observed timestamp.
4. **Build version:** The selective documentation build records Rspress v2.0.22.
5. **Input denominator:** The selected root includes 115 tracked source paths and excludes untracked files.
6. **Build output:** Selected-root file count and bytes are recorded from the generated manifest.
7. **Publication denominator:** The publication manifest and sync reports consistently use 75 routes.
8. **Publication bytes:** The selected route payload totals 13,978,392 bytes.
9. **Unchanged roadmap source:** `roadmap.html` hash matches the source manifest and the previous publication.
10. **Changed plan source:** The newly merged plan page is present in the publication set.
11. **Evidence link target:** The PR #1350 sync README is included in the publication, so the plan link resolves.
12. **No route deletion:** The sync records zero deletions.
13. **Create denominator:** Copy reports 36 newly created routes.
14. **Replace denominator:** Copy reports 25 replacements.
15. **Identical denominator:** Copy reports 14 already-identical routes.
16. **Backup isolation:** Replaced routes use a fresh PR #1351-specific backup root.
17. **Backup denominator:** All 25 replaced routes have a corresponding backup record.
18. **Backup bytes:** Backup integrity is checked against the pre-copy hash for each replacement.
19. **Local served integrity:** All 75 local preview files match source size and SHA-256.
20. **HTTPS response status:** All 75 HTTPS responses return 200.
21. **HTTPS served integrity:** All 75 HTTPS bodies match source size and SHA-256.
22. **Server continuity:** Existing Tailnet preview PID `90711` remains unchanged.
23. **No server restart:** Preflight, copy, and served verification record no restart.
24. **Desktop roadmap layout:** Agent Browser confirms document/body width 1280 at a 1280 viewport.
25. **Mobile roadmap layout:** Agent Browser confirms document/body width 390 at a 390 viewport after async content loads.
26. **Desktop plan layout:** Agent Browser confirms document/body width 1280 and sees the new #1350 completion state.
27. **Mobile plan layout:** Agent Browser confirms document/body width 390, finds the evidence link, and sees no stale phrase.
28. **Evidence route:** Agent Browser opens the linked sync README and sees its publication summary.
29. **Lifecycle cleanup:** The task browser session closes and reports inactive; the unrelated HWP session remains active.
30. **Product-status boundary:** The publication does not change `78/214`, C11.4/C11 completion, contract `0.1.0`, or CI status.

## Review result

The publication is verified against its source manifest at both local and HTTPS layers. The corrected plan page and linked evidence render at desktop and mobile widths. This is documentation delivery evidence only.
