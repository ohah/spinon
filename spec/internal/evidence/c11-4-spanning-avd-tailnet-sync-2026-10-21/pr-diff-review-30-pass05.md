# Tailnet pass05 PR diff hostile review — 30 checks

| # | Failure perspective | Result and evidence |
|---:|---|---|
| 1 | The selected site could be built from the wrong source revision | `selected-root-preparation-pass05.json` pins HEAD `44c18e61d2328753726cd120eeca3cff465a29a4` and tree `66b5bd2726dfe5184ac17aad0355a787df1734e0`; pass. |
| 2 | The pass04 base could already contain the AVD note, hiding a missing overlay | Preparation records the base root and copies tracked changes after source `4592bd77`; generated roadmap explicitly contains the API 37 AVD note; pass. |
| 3 | A changed Markdown source could be omitted from the selected root | Preparation records 38 changed source files and 109 mapped copies; each copied byte sequence matches `git show HEAD`; pass. |
| 4 | A renamed or deleted source file could silently leave a stale route | The preparation script fails closed for deletion statuses; `noDeletes=true`; pass. |
| 5 | A file path could escape the selected root | Every source path rejects absolute paths and `..`; every target is checked beneath the selected root; pass. |
| 6 | A symlink could redirect a selected build path | Base root and generated site are recursively checked for symlinks; `noSymlinks=true`; pass. |
| 7 | The selected build could use an unpinned RSPress version | Summary records RSPress `2.0.22`, matching the installed lockfile version; pass. |
| 8 | The failed full docs build could be presented as successful | README states that full `docs:build` had 695 dead-link groups and this pass is selected-root only; pass. |
| 9 | Disabled dead-link/image checks could be mistaken for conformance | Summary records `fullRepositoryDeadLinkCheck=false`; README scopes the result to required routes; pass. |
| 10 | The build could reuse stale output after a failed run | Pass05 output paths were confirmed absent before build; generated tree is recorded in the build summary; pass. |
| 11 | The new roadmap note could be absent from rendered HTML | Summary asserts the API 37 AVD note and evidence link in `roadmap.html`; Agent Browser found the note live; pass. |
| 12 | The official completion count could be increased accidentally | Rendered STATUS and roadmap retain 72/213; browser result records the same; pass. |
| 13 | C11.4 could be marked complete by the supplemental AVD run | README and summary retain the incomplete status; pass. |
| 14 | Ahem glyph paint could be implied from geometry | Roadmap and rendered README state it was not verified; pass. |
| 15 | Text overflow pixels or PixelCopy could be implied | Roadmap and rendered README state neither was checked; pass. |
| 16 | The Android AVD result could be described as a physical-device result | The roadmap calls it one Android API 37 AVD; README says no physical Android device was connected in this pass; pass. |
| 17 | The AVD image route could be missing while the README route exists | Summary requires the PNG route and the live browser loaded it at 1080×2400; pass. |
| 18 | The screenshot could differ from the hashed source artifact | Preflight and remote verification bind the served image to SHA-256 `8d127bd3b71da6b0500390d34972b018d0d91d3c63606fe6fa53cb6bc273ce21`; pass. |
| 19 | The live server could be an unrelated listener | Preflight and copy script require one PID whose command is `tailnet-preview-server.mjs` and only `127.0.0.1:8768`; pass. |
| 20 | The server could be stopped during sync | The existing PID 1191 was retained and served the preview after the copy; pass. |
| 21 | Preview files could be overwritten without an inventory | Preflight records old path, byte count and SHA for every changed target; 250 create, 194 replace, 2,776 same; pass. |
| 22 | Existing files could be lost without a backup | All 194 replaced files (28,613,007 bytes) were backed up and verified; pass. |
| 23 | The script could delete stale files outside the selected site | It performs no deletes; preflight, copy and local verification all record `no_deletes=true`; pass. |
| 24 | A target could change between preflight and copy | Copy rechecks source and target hashes against preflight immediately before replacement; all targets verified; pass. |
| 25 | Non-preview processes could hold a target open during rename | Before each replacement, the script inspects target open handles and refuses non-preview holders; pass. |
| 26 | A partial write could expose truncated HTML or assets | Writes go to temporary siblings, are fsynced, then atomically renamed; local verification passed 3,220/3,220; pass. |
| 27 | The local preview could match while Tailnet serves stale bytes | Remote HTTPS checked every file: 3,220 HTTP 200 and 3,220 matching hashes, zero failures; pass. |
| 28 | The Tailnet route prefix could be wrong | Agent Browser opened `/spinon/roadmap.html`, the evidence README and the direct PNG route; pass. |
| 29 | The PR screenshot could show an unrelated part of the roadmap or fail to load | The screenshot visibly includes the API 37 AVD note and 72/213; Agent Browser confirmed the PR raw image `complete=true`, 1600×900, matching its recorded SHA-256; pass. |
| 30 | Agent Browser could remain running after verification | `browser-close-pass05.json` records `close --all` for Tailnet and PR sessions; Agent Browser and HeadlessChrome process counts were both zero; pass. |

No unresolved issue was found in these 30 checks. This is a document-preview and evidence-sync review, not a product implementation or full C11.4 conformance claim.
