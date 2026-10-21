# C11.4 Android API 37 AVD Tailnet roadmap sync — pass05

## Source and selected build

- Source commit is merge commit `44c18e61d2328753726cd120eeca3cff465a29a4`; source tree is `66b5bd2726dfe5184ac17aad0355a787df1734e0`. The selected root starts from the Tailnet pass04 root and overlays 38 tracked `plan/` and `spec/` changes since `4592bd77c58c59547fc3c6740f6a8afa1e4ea7e2`, including the Android API 37 AVD roadmap note, status, plan and evidence.
- RSPress `2.0.22` built 3,220 files (151,601,371 bytes), with site tree SHA-256 `a85d29e09605aabd4ae1d9c8f28009b3345ea8fb660593f04216433f0f5ae78c`.
- For a clean diff, trailing spaces and the final blank line in RSPress's size table were removed from the log; rows and values were preserved.
- This is a selected-root build with dead-link and dead-image checks disabled. It verifies the roadmap, STATUS, C11.4 plan and AVD evidence routes; it is not a full documentation-link conformance result. The repository full `docs:build` previously reported 695 dead-link groups.
- The rendered AVD note keeps the observed scope precise: root/item and tracks, trace and draw/present are recorded; Ahem glyph paint, overflow pixels and PixelCopy remain unverified. C11.4 stays incomplete and official count stays 72/213.

## Tailnet sync

- Updated the existing preview rooted at `/Users/yoonhb/Library/Application Support/SpinonPreview`; server PID 1191 continues listening on `127.0.0.1:8768`.
- Preflight found 250 new files, 194 replacements and 2,776 unchanged files, with zero deletions. The 194 old files (28,613,007 bytes) were backed up to the pass05 backup root before atomic replacements.
- Local byte/SHA verification passed for all 3,220 files and all 194 backups. Remote Tailnet HTTPS returned HTTP 200 and matching SHA-256 for all 3,220 files, with zero failures.
- Agent Browser showed the new roadmap note and its evidence link, opened the evidence README, and loaded the AVD capture at 1080×2400. The [roadmap screenshot](./tailnet-roadmap-note-agent-browser-pass05.png) is 1600×900. Agent Browser and HeadlessChrome were closed afterward; both process counts were zero.
- The screenshot was also included in PR #439 and loaded from its raw GitHub image URL at 1600×900; [PR image verification](./pr439-image-agent-browser-pass05.json).
- tmux and simulator sessions were left unchanged. No physical Android device was connected for this pass.

Manifests and scripts with `pass05` in the filename record root preparation, selected build, preflight, backup/copy, local verification, remote verification, browser observations and artifact hashes. The pass does not claim WPT conformance or full C11.4 completion.
