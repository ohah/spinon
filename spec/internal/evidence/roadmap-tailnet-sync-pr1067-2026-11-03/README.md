# PR #1067 C11.1 Tailnet roadmap synchronization — 2026-11-03

## Scope and source

After documentation PR [#1067](https://github.com/ohah/spinon/pull/1067) merged, the selected C11.1 roadmap routes were rebuilt and synchronized to the existing Tailnet preview at [https://macstudio.tailed42f2.ts.net/spinon/roadmap.html](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html). The source was `main` commit `afb68887e4b2882c5d05487196020b5e6865e1e3` (tree `522777b7c9d00cde5d3ccf0a4a2df7da17ef866b`). The implementation merge remains PR #1066, SHA `2dde385c5f240c8b8837b9e55d9c5431f5f9294b`.

The selection used Git-tracked files only. RSPress remained pinned at 2.0.22. The selected root contained 65 files (4,512,933 bytes); the built publication contained 138 routes (14,196,035 bytes). Full-site dead-link validation was disabled for this selected build and is not claimed here.

## Tailnet publication

Tailscale reported `Running`. Existing preview server PID 28040 was listening on `127.0.0.1:8768` and served HTTP 200 before and after the update. It was not restarted or terminated.

The corrected preflight found 70 new routes, 30 changed routes, and 38 already-identical routes. The existing server had open readers for `roadmap.html` and `STATUS.html`. Their old bytes were backed up under `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1067-2026-11-03/pass01`; replacement used a temporary file in the same directory followed by atomic rename. Existing readers continued using their old inode. No route was deleted or edited in place. The separate `spec/plan/c11-css-grid.html` alias was excluded because the preview server still had open readers for it; the linked `/plan/c11-css-grid.html` route was updated.

The first diagnostic invocation used an invalid `lsof` argument form and was discarded before any publication. The corrected preflight enumerated the existing readers above; no route was changed until that preflight passed.

Eleven important routes were then fetched through both local HTTP and the Tailnet HTTPS URL. Every response was HTTP 200 and its SHA-256 matched the selected publication, including the roadmap, official status page, C11 plan, new `fr` plan, contract 0059, merge/runtime records, and Android/iOS screenshots. The full selected-route hash manifests are in [preflight](preflight.json), [copy and backup](copy-and-backup.json), [selected root](selected-root.json), [selected publication](selected-publication.json), and [served verification](served-verification.json).

## Agent Browser confirmation

The isolated Agent Browser session opened the Tailnet URL. At desktop viewport 1440×1000 the PR #1066 card displayed the merge SHA and `72/213`, with document width 1440. At mobile viewport 390×844 the card fit within a 362px content box and document/body width remained 390px. The C11 checkbox was unchecked. Captures are [desktop](tailnet-roadmap-agent-browser-desktop.png) and [mobile](tailnet-roadmap-agent-browser-mobile.png); machine-readable observations are in [Agent Browser verification](agent-browser-verification.json).

After capture, only session `spinon-roadmap-pr1067` was closed. `agent-browser session info --json` reported `active=false` and `pid=null`; no process matching that session remained. Preview server PID 28040 remained running.

## State and limits

The page reports the merged bounded `fr` implicit-column slice, while C11.1/C11 remain incomplete and the official count remains `72/213`. This selected publication does not validate every site link, imply GitHub Pages deployment, or expand simulator screenshots into PixelCopy/Metal readback or compositor-pixel evidence. The Android AVD and iOS Simulator scope remains as described by the linked runtime record.

- [PR diff and Tailnet publication review, 30 perspectives](roadmap-pr-diff-review-30.md)
- [Merged C11.1 implementation evidence](../c11-root-html-grid-column-flow-fr-auto-columns-runtime-2026-11-03/README.md)
