# Tailnet sync PR diff hostile review · 20 points

| # | Independent failure angle | Result |
|---:|---|---|
| 1 | A stale plan header still claims PR #1320 is pending. | Fixed in this diff; it names the merge commit and keeps implementation pending. |
| 2 | The plan accidentally reports C11 or C11.4 complete. | No; both remain incomplete. |
| 3 | The official completion count drifts from the agreed baseline. | No; all selected manifests keep `78/214`. |
| 4 | The internal contract version changes without a contract revision. | No; `0.1.0` is preserved. |
| 5 | A wrong branch or source revision is published. | Manifests bind the selection to `6003e286f7bb5d36b3715a7836f4c358943bf229` and its recorded tree. |
| 6 | The corrected plan page is omitted from selected routes. | It is present in both `plan/` and `spec/plan/` route families. |
| 7 | The roadmap next-plan link points to a missing page. | The selected publication manifest includes the linked plan route. |
| 8 | The one-fit evidence page is omitted or points outside the published roots. | The README and referenced evidence files are included in both route families. |
| 9 | The README's clickable screenshot link points to the wrong filename. | Agent Browser followed the Tailnet link and loaded `chrome157-one-fit-auto-min-sample.png` at 1280×900. |
| 10 | The screenshot is only present locally but missing from Tailnet. | The image route was among the 203 HTTP-200, hash-verified routes. |
| 11 | RSPress output does not build after the plan metadata correction. | RSPress 2.0.22 completed the selected build before publication staging. |
| 12 | The selected publication manifest differs from the generated site. | Staging validated route counts, byte sizes, and SHA-256 values for all 203 routes. |
| 13 | The sync overwrites user-edited preview files without backup. | Preflight found 40 replacements; copy created and hash-verified all 40 prior-file backups. |
| 14 | A changed target is open while it is replaced. | Preflight and copy reported zero changed-route open handles. |
| 15 | The Tailnet preview server is stale, misidentified, or silently restarted. | Preflight identified PID 90711, verified the current local roadmap response, and served verification recorded no restart. |
| 16 | Unrelated existing routes are deleted or the preview index is replaced. | Sync reports `noDeletes: true`; selected staging explicitly preserves `index.html` and `404.html`. |
| 17 | Local copy writes partial or corrupt files. | All 203 local files matched their publication hashes after copy. |
| 18 | Tailnet HTTPS serves a proxy/cache copy with stale content. | All 203 HTTPS routes returned HTTP 200 and matched the publication hashes. |
| 19 | Mobile layout clips the document horizontally or hides the intended link. | Agent Browser measured 390px document width at a 390px viewport and found the next-plan link. |
| 20 | Browser or simulator processes remain after visual verification. | Agent Browser session is inactive with PID null; no iOS simulator was booted. The pre-existing Terminal Browser helper was not touched. |

This review is specific to the Tailnet documentation synchronization diff. It does not claim product runtime completion or CI coverage.
