# C11.5 auto-size/stretch Tailnet roadmap preview sync

The user-requested Tailnet preview now shows the bounded implementation merged as PR #1132. The preview server remained running; only its static `roadmap.html` file was atomically replaced. This preview update was made from the documentation branch before its PR merged, then checked against the committed source hash.

## Source and served route

- Source file: `spec/public/roadmap.html`.
- Source branch: `docs/c11-5-public-roadmap-merge-state`, based on main revision `a793473191053b226c0596307207c24a59e403a3` before this PR.
- Source SHA-256, byte count: `5a9f64c386fa2d6fb7b509594f24a5609013c909c68a1a0453768f31cfac9048`, 552055 bytes.
- Route: [Tailnet roadmap](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html).
- Preview root and served file: `/Users/yoonhb/Library/Application Support/SpinonPreview/roadmap.html`.
- Before SHA-256: `0301e2c299cbebd31811f405381883cec6dbdf124b1dd3a346643829e455e87e`.
- Served file and HTTPS response body SHA-256: `5a9f64c386fa2d6fb7b509594f24a5609013c909c68a1a0453768f31cfac9048`, 552055 bytes; HTTP status 200.
- Previous served content backup: `/tmp/spinon-c11-5-public-roadmap-sync-2026-11-04/roadmap.html.before`, SHA-256 `0301e2c299cbebd31811f405381883cec6dbdf124b1dd3a346643829e455e87e`.
- The existing preview server PID 28040 stayed running on 127.0.0.1:8768.

## Agent Browser verification

Agent Browser HeadlessChrome opened the Tailnet route at a 1280×633 CSS-pixel viewport. The page showed the PR #1132 merge SHA/time, the bounded Android/iOS results, both evidence links, and C11/C11.5 incomplete with `72/213`. Screenshot: [Agent Browser preview](./roadmap-postmerge-agent-browser.png), SHA-256 `2944b39113321e0eb98c0b5b843d58cac512f2a62f2602974a224b106755baa6`.

The Agent Browser session was closed. A follow-up process check found no remaining Agent Browser process; the Tailnet preview server was preserved.

## Preview scope

This is the selected static roadmap page, not a full Rspress site rebuild. Other pages and `STATUS.html` were not rebuilt or replaced. The PR #1132 merge itself does not complete C11.5/C11 or change official completion `72/213`.
