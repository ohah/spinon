# Tailnet sync evidence PR diff review · 30 independent checks

1. **Implementation status inflation** — verify the evidence does not mark C11 or C11.4 complete. The README and manifests preserve `72/213` and `c11Complete: false`.
2. **Wrong source revision** — compare every report's source HEAD and tree. All manifests identify main `39f57f7a6eed4ddd49e06836e89579d0514cc591` and tree `05ad3c22259547128b462a4f6ce0e99ea4b68597`.
3. **Source diff identity drift** — compare tracked diff hashes across manifests. They match `e1716157534632371333f0f55fd8ea9896aa9d65b77051826f8ba111d4924658`.
4. **Selected-root count mismatch** — compare the 271 staged inputs to publication inputs. The report records 271 files and 9,549,482 bytes; selected publication contains 350 final routes after generated pages and aliases.
5. **Publication total mismatch** — recompute route and byte totals from `selected-publication.json`. It records 350 routes and 19,017,880 bytes.
6. **Preflight action arithmetic** — ensure create, replace, and same counts total 350. The recorded values are 179 + 29 + 142.
7. **Copy action arithmetic** — compare copy counts with preflight. The copy report records 179 created, 29 replaced, and 142 identical.
8. **Backup completeness** — ensure every replacement has a pre-copy backup and matching previous hash. All 29 backup checks are recorded as verified.
9. **Unexpected deletion** — inspect copy logic and reports for unlink/rmdir or delete actions. No delete operation is present and all reports set `noDeletes` true.
10. **Preview root aliasing** — check the preview root is a real directory rather than a symlink. `checkServer` compares `realpath` and the configured root.
11. **Source symlink traversal** — inspect source staging. Each path component is checked with `lstat`; symlink sources are rejected.
12. **Generated-site symlink traversal** — inspect publication traversal. Symlink entries and unsupported file types are rejected.
13. **Route traversal** — probe absolute paths, backslashes, empty components, `.` and `..`. `safeRoute` rejects them before filesystem joins.
14. **Duplicate route content conflict** — verify aliases cannot silently shadow different bytes. `add` accepts duplicate routes only when SHA-256 matches.
15. **Untracked source inclusion** — ensure source-tree selection uses the Git index. `git ls-files` supplies tracked files only.
16. **Source content mutation after staging** — compare staged byte length and SHA-256 during publication creation. Both must still match the root manifest.
17. **Roadmap card links missing from package** — validate each local card href against the publication route map. Six links are recorded and required routes are asserted.
18. **Unintended full-site claim** — inspect exclusions and validation flags. Two C11 parent plan aliases are excluded, and full-document link validation is explicitly false.
19. **Preflight server identity** — require the existing port 8768 listener and expected preview server command. The run records PID `28040`.
20. **Preview server restart claim without evidence** — the original report field alone was insufficient. Added an explicit post-sync observation at 2026-11-01 09:41:18 UTC: Tailscale `Running`, preflight and post-sync PID both `28040`.
21. **Tailscale unavailable** — require backend state `Running` during preflight and the recorded post-sync observation.
22. **Open replaced files** — check changed routes before writing and again immediately before replacement. The preflight count is zero; replacement checks refuse open targets.
23. **Target changes between preflight and copy** — re-read each destination and compare its prior bytes/hash. Added an existence-state assertion for both create and replace actions so a concurrently created path is not overwritten.
24. **Backup overwrite** — use exclusive creation for backup files and refuse an existing backup root. Earlier versions are preserved.
25. **Partial destination writes** — write to a unique exclusive temporary file and rename into place; verify the resulting destination hash.
26. **Replacement mode loss** — preserve existing permission bits for replaced routes and use `0644` for newly created routes.
27. **Local verification overclaim** — distinguish preview-root file hash comparison from HTTP serving. README names the former accurately; HTTPS checks separately verify served bytes.
28. **Remote status-only false positive** — require both HTTP 200 and exact length/hash for each route. 350/350 passed.
29. **Unbounded remote fetch fan-out** — limit concurrent requests to six and apply a 20-second per-request timeout.
30. **Browser process leak** — Agent Browser verified the public page, then `close --all` ran; a process scan found no Agent Browser or dedicated Chrome process.

No unresolved blocker was found for this evidence-only change. The post-sync PID field is a point-in-time observation; the review does not claim continuous process telemetry. No build or test result is inferred from this document PR.
