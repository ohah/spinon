# C11.4 direct mixed intrinsic Tailnet roadmap sync

Date: 2026-10-25

## Source state

Implementation PR [#611](https://github.com/ohah/spinon/pull/611) merged to main at 2c4c278f0facd31ac3e0cc60dce1f27575628b35. Documentation PR [#612](https://github.com/ohah/spinon/pull/612) then merged at 25ecb4f2ced6442f85e09bcdbc33106c02beaa3b. The official completion count remains 72/213, and C11/C11.4 remain unchecked.

The runtime evidence page had eight Markdown image paths without an explicit ./ prefix. The selected Rspress build rejected those module specifiers. The docs-only correction is commit e32b4e97; all eight links now use explicit relative paths. The corrected source README hash is recorded in [selected-root-pass02.json](selected-root-pass02.json), and its rendered page is in the selected-build manifest.

## Selected build

Rspress 2.0.22 built a bounded documentation root, not the full site. It selected the roadmap, status, C11 Grid plan pages, contract 0059, precomparison, runtime evidence and their linked files. The root preparation copied 203 source files. The initial build stopped because imported screenshots were only in the public asset tree. The next pass copied 30 images to the Markdown import root and then exposed the missing ./ prefixes. Pass03 succeeded after the source correction. Readable stdout and exit code are in [selected-build-pass03.log](selected-build-pass03.log) and [selected-build-pass03.exit](selected-build-pass03.exit); exact captured output is compressed in [selected-build-pass03-raw.log.gz](selected-build-pass03-raw.log.gz). The three readable logs only trim trailing spaces/tabs from fixed-width Rspress table output; [normalization hashes](selected-build-log-normalization-pass03.json) connect each readable log to its raw archive.

The final package combines the 47 Rspress output routes with the static roadmap and original evidence files. Its route manifest contains 236 unique paths and requires the new roadmap, STATUS, contract, plan, runtime page, Android/iOS representative screenshots and post-merge test logs.

## Copy and verification

The existing preview server at PID 1180 served https://macstudio.tailed42f2.ts.net/spinon/ with HTTP 200 and was not restarted. The target root was /Users/yoonhb/Library/Application Support/SpinonPreview. Copy is create/replace only; it deletes no files.

Preflight found 212 new routes, 10 replacements and 14 unchanged routes (236 total). The 10 replaced files, totaling 4,668,813 previous bytes, were backed up under SpinonPreviewBackups/c11-4-direct-mixed-text-image-tailnet-roadmap-sync-2026-10-25/pass03. Local and Tailnet responses match the selected source route SHA-256 hashes for 236/236 paths, all Tailnet responses returned HTTP 200, and all 10 backups match their preflight hashes. See [preflight](tailnet-preflight-pass03.json), [copy report](tailnet-copy-pass03.json) and [final verification](tailnet-verification-pass03.json).

## Agent Browser verification

Agent Browser 0.31.1 opened the Tailnet roadmap, official STATUS page and runtime evidence page. The rendered roadmap shows PR #611, its merge SHA, and 72 checked items out of 213. The official STATUS C11 checkbox is unchecked and disabled. The runtime page loaded all eight Android/iOS fixture screenshots plus the 150×150 site logo: 9/9 images loaded, with Android images at 1080×2400 and iOS images at 1206×2622.

The browser screenshots are [roadmap](roadmap-agent-browser-pass03.png) and [runtime evidence](runtime-agent-browser-pass03.png); both are 1280×633. Machine-readable page/image results and hashes are in [Agent Browser verification](agent-browser-verification-pass03.json). The named session was closed; the post-close process table found no Agent Browser, terminal-browser or session Chrome process in [browser-process-check-pass03.txt](browser-process-check-pass03.txt). The existing Chrome and Tailnet preview server were left running.

## Evidence files

- prepare-selected-root.mjs, prepare-selected-root-pass02.mjs, repair-image-imports.mjs, and repair-image-imports-pass02.mjs: source selection and image import placement.
- rspress.config.ts and rspress-pass02.config.ts: bounded Rspress roots.
- selected-build.log / selected-build.exit, selected-build-pass02.log / .exit, and selected-build-pass03.log / .exit: failed attempts retained separately and final successful build. The raw logs are gzip archives with hashes recorded in selected-build-log-normalization-pass03.json.
- scan-selected-site-pass03.mjs, selected-build-pass03.json: Rspress output inventory.
- prepare-selected-output-pass03.mjs, selected-output-preparation-pass03.json, scan-selected-output-pass03.mjs, selected-output-pass03.json: final 236-route package and hashes.
- source-version-pass03.json: merge commits and equality between the committed runtime README and the selected-root copy.
- sync-tailnet.mjs, sync-tailnet-pass03.mjs, tailnet-preflight-pass03.json, tailnet-copy-pass03.json, and tailnet-verification-pass03.json: guarded create/replace sync, backups and end-to-end hashes.

This verifies the selected Tailnet documentation routes and linked images. It is not a full-site Rspress build, a full dead-link scan, or new CSS support evidence. Simulator screenshot limits and the unchanged C11.4 status remain as documented in the runtime evidence.
