# Tailnet roadmap sync after PR #666 · 2026-10-27

PRs [#664](https://github.com/ohah/spinon/pull/664), [#665](https://github.com/ohah/spinon/pull/665), and [#666](https://github.com/ohah/spinon/pull/666) are merged. This sync uses exact `origin/main` HEAD `fb3f1e5a307c9678d6b70af017858fe7e03f435a` and tree `3e7c6e643d501a70e53aae1780729c1c25c81898`. C11.4 remains unchecked; official completion remains `72/213`.

## Selected build

- The pre-fix Rspress 2.0.22 build failed because Markdown images without `./` were resolved as package imports. The original failure log is [preserved here, compressed without edits](./rspress-build-image-path-blocker-pr666-pass01.log.raw.gz); PR #666 fixes the four links and updates the precomparison README checksum.
- The corrected selected-root Rspress build exited `0`; Pages rendered in `11149 ms`. The command, source head, config limits, and log hashes are in [build result](./rspress-build-result-pr666-pass01.json), with the compressed raw build log alongside it. It generated 791 HTML pages, while publication stages only the focused route set. Dead-link and dead-image checks are disabled in this selected config, so this is not a full documentation or repository-wide link check.
- The selected root is based on the prior pass08 selected site as a read-only hardlink tree. Eighty tracked main sources were compared byte-for-byte with `origin/main` and overlaid as 307 aliases. The [root manifest](./selected-root-pr666-pass01.json) records source hashes and every overlay.
- The publication allowlist contains 40 rendered HTML routes and 253 total routes, `32,309,889` bytes. The [selected-build manifest](./selected-build-pr666-pass02.json) records every file hash. The staging script collects only selected pages, linked raw evidence and images, and page-referenced static assets; it does not copy all 791 generated pages.

## Tailnet copy and verification

- The [preflight](./tailnet-preflight-pr666-pass01.json) ran before the copy: Tailscale was `Running`, Mac Studio online, preview server PID `28040` listening on `127.0.0.1:8768`, and the existing roadmap returned HTTP 200. At that point it made no preview-target writes. The later copy is recorded separately; no server restart or file deletion occurred. The [target handle inventory](./target-open-handles-pr666-pass01.json) records the same pre-copy window.
- Three open static-asset handles belonged to PID `28040`. Their current bytes matched the selected build exactly, so those three files were left untouched. See [open-handle hash comparison](./open-handle-route-comparison-pr666-pass01.json).
- The copy created 202 routes, replaced 22, and left 29 already-identical routes in place. All 22 replaced files were backed up byte-for-byte under `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr666-2026-10-27/pass01`. No preview files were deleted. The [copy report](./tailnet-copy-pr666-pass01.json) and [run log](./tailnet-copy-run-pr666-pass01.log) list the changes.
- Post-copy verification matched local selected output, preview files, backup hashes, and Tailnet HTTP response bodies: local `253/253`, backup `22/22`, Tailnet `253/253`, plus two existing shared assets `2/2`. The [verification manifest](./tailnet-verify-pr666-pass01.json) contains each served hash.

## Agent Browser verification

- Agent Browser 0.31.1 opened the public roadmap, the new focused plan, and both evidence READMEs. It confirmed the roadmap's PR #664 merge SHA, the expected `[10,290]` rows, and C11.4's unchanged `72/213` incomplete status.
- Runtime Android and iOS evidence images loaded at `1080×2400` and `1206×2622`. Both Chrome precomparison images loaded at `1280×633`. These checks confirm the corrected local image imports on the Tailnet site; no new app or simulator run was made for this documentation sync.
- The live roadmap screen is [captured here](./tailnet-roadmap-agent-browser-pass02.png); the exact observations and image URLs are in [Agent Browser results](./tailnet-agent-browser-observation-pr666-pass01.json). Terminal Browser was not used.

## Scope

This sync publishes the selected Tailnet routes for the merged roadmap and C11.4 plans/evidence. It does not change repository source, simulator captures, the official status ledger, the `72/213` count, or the C11.4 completion checkbox. Existing simulator and precomparison snapshots were retained.
