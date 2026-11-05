# Tailnet C11 gap synchronization evidence PR: 30-point hostile review

Each point checks a different failure path or claim boundary in this documentation-only change.

| # | Attack / invariant | Result |
|---:|---|---|
| 1 | A stale branch could publish evidence against the wrong main revision. | Pass: all manifests bind to source HEAD ca69e93c262f42178b3a600164302ee0cbc188dc. |
| 2 | A tree hash could disagree with the named commit. | Pass: selected-root, publication, preflight, copy, and served manifests all record tree 34b008933ccd65b2c2cd939a20a076f16c44a27e. |
| 3 | A modified tracked file could leak into the selected publication while claiming a commit-bound source. | Pass: tracked-diff digest is the empty-input SHA-256; the selected root is index-backed. |
| 4 | Thousands of unrelated untracked workspace files could enter the generated site. | Pass: the selector records 336 Git-index source files, not the untracked workspace inventory. |
| 5 | A symlink could escape the selected documentation root. | Pass: the selected-root manifest records noSymlinks true. |
| 6 | Source-file count could be confused with rendered-route count. | Pass: 336 input files and 451 output routes are recorded as separate quantities. |
| 7 | Selected input byte size could be misstated. | Pass: the source manifest records 8,710,764 bytes. |
| 8 | Rendered byte size could be inferred from source bytes. | Pass: the publication manifest independently records 21,279,555 output bytes. |
| 9 | The evidence could say build passed when Rspress did not render pages. | Pass: the selected build log records successful page rendering and completion. |
| 10 | A selective build could be misrepresented as full documentation validation. | Pass: the README discloses the manifest's fullDocsLinkValidation false value and the Rspress dead-link/dead-image checks disabled by config; it limits the claim to this selected build. |
| 11 | A known project completion count could be changed or copied from a stale preview. | Pass: source and publication manifests both record 78/214. |
| 12 | A bounded C11 slice could be mislabeled as C11 completion. | Pass: manifests and the rendered card mark C11 incomplete. |
| 13 | The card could show a PR head SHA instead of the merge SHA. | Pass: the recorded card SHA is the merged #1172 SHA ee31e1607eda4d80f0a0dcc49f7656c3b693f249. |
| 14 | The published card could omit the merge evidence or completion state. | Pass: Agent Browser evaluation found the merge SHA, 78/214, and incomplete C11 marker on the live route. |
| 15 | The route plan could silently delete an existing preview file. | Pass: publication, preflight, copy, and served manifests all record no deletions. |
| 16 | Preflight could proceed while changed targets were open. | Pass: changedRouteOpenHandleCount is zero. |
| 17 | Copy could create or replace a different number of routes than preflight planned. | Pass: preflight and copy agree on 148 created, 42 replaced, and 261 identical. |
| 18 | Replaced preview files could be overwritten without a recoverable copy. | Pass: copy records a dedicated backup location and verification records 42 checked backups. |
| 19 | Backup verification could count files without matching their content. | Pass: served verification records all 42 backup hash checks as verified. |
| 20 | A target file could differ locally after copy while the site looked correct in one screenshot. | Pass: the local content hash verifier covers all 451 routes. |
| 21 | HTTPS responses could be unavailable or stale while local files pass. | Pass: the HTTPS verifier records HTTP 200 and matching hashes for all 451 routes. |
| 22 | The preview server could have been restarted, invalidating the claimed pre-existing process state. | Pass: served verification records previewServerRestarted false; preflight records PID 28040. |
| 23 | Tailnet could have been offline during the copy. | Pass: preflight records the backend running and the local roadmap returning HTTP 200. |
| 24 | Desktop screenshot dimensions could disagree with the claimed viewport. | Pass: the Agent Browser desktop evaluation records 1440×1000 and the screenshot is 1440×1000. |
| 25 | Desktop could show a stale card or clip the target outside the viewport. | Pass: desktop evaluation finds the merged SHA and records the target card visible. |
| 26 | Mobile could have horizontal overflow hidden by a narrow screenshot crop. | Pass: mobile evaluation compares document client and scroll widths; both are 390 px and horizontalOverflow is false. |
| 27 | The target could exist in the DOM but not be visible on mobile. | Pass: mobile evaluation records the card visible at x=14 px, width 362 px, within the 390 px viewport. |
| 28 | Mobile could show a different roadmap state from desktop. | Pass: both evaluations find the same merge SHA, 78/214, and incomplete C11 marker. |
| 29 | Browser automation could be left running after evidence capture. | Pass: close-all completed, session list reported no active sessions, and the subsequent process check returned none. |
| 30 | Private preview or screenshots could be presented as production release or pixel-readback proof. | Pass: the README labels this as Tailnet preview evidence, discloses the selective link-validation limit, and describes screenshots as visual evidence. |

No source changes are included in this PR. The selected route inventory and hashes describe the already completed Tailnet synchronization.
