# PR #1351 post-merge Tailnet roadmap sync

## Publication

- GitHub `main` and selected publication source: `07cfec2968c571e3cd150fe3a32ec91a3d4841d4` (PR #1351, Rebase and merge, 2026-11-11 04:53:46 UTC).
- Source tree: `af4390492087543242371be2b781b01f5d32d619`.
- Tailnet host: `macstudio.tailed42f2.ts.net`. The existing preview server PID `90711` stayed unchanged; it was not restarted.
- Selective Rspress v2.0.22 build: 115 tracked source paths, 214 selected-root files, 71,659,299 bytes. The build output is task-owned and removed after verification.
- Selected publication: 75 routes, 13,978,392 bytes. It included the corrected plan page and the linked PR #1350 sync evidence page.
- Copy result: 36 new routes, 25 replaced routes, 14 already-identical routes. All 25 previous files were backed up and hash-verified at `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1351-2026-11-11/pass01`.
- Served verification: all 75 local files and HTTPS responses matched selected source byte sizes and SHA-256 hashes; all HTTPS responses were HTTP 200. All 25 backups matched their previous hashes. No route was deleted.
- The source `roadmap.html` hash stayed `8ce99fb9bd408d88f1080d65247857989f45de5e418456020f023fc45b6bb118`; its content was unchanged by PR #1351.

See [preflight.json](./preflight.json), [copy-and-backup.json](./copy-and-backup.json), [served-verification.json](./served-verification.json), [selected-root.json](./selected-root.json), and [selected-publication.json](./selected-publication.json) for full route-level records.

## Agent Browser confirmation

The post-merge plan page now shows PR #1350 as merged and the Tailnet sync as complete. Its stale “merge-state 문서 동기화 중” phrase is absent. The linked PR #1350 sync evidence route also renders. The roadmap continues to show PR #1349 merge SHA `b0725665e31cc50ed6eb65a4083a1b94245dea36`, official `78/214`, and C11.4/C11 incomplete.

| Page | Viewport | Document/body width | Result |
|---|---:|---:|---|
| Roadmap | 1280 × 1200 | 1280 / 1280 | Rendered; PR #1349 merge, `78/214`, and incomplete C11.4/C11 visible |
| Roadmap | 390 × 844 | 390 / 390 | Rendered after async status loaded; no horizontal page overflow |
| Dedicated plan | 1280 × 1200 | 1280 / 1280 | Rendered; PR #1350 merge SHA and completed sync text visible; stale wording absent |
| Dedicated plan | 390 × 844 | 390 / 390 | Rendered; evidence link present; stale wording absent; no horizontal page overflow |
| Linked sync evidence | 1280 × 1200 | 1280 / 1280 | Rendered with publication metadata and screenshot links |

Screenshots were captured by Agent Browser:

- [Roadmap, desktop](./tailnet-roadmap-desktop-agent-browser.png)
- [Roadmap, mobile](./tailnet-roadmap-mobile-agent-browser.png)
- [Corrected plan, desktop](./tailnet-plan-desktop-agent-browser.png)
- [Corrected plan, mobile](./tailnet-plan-mobile-agent-browser.png)

The exact route/text/width checks are in [agent-browser-verification.json](./agent-browser-verification.json). The browser session was `spinon-tailnet-pr1351-final-20261111`; it was closed after capture, and its Agent Browser session info reports inactive. An unrelated HWP Agent Browser session was left running.

The Agent Browser CLI was used for page checks. Terminal Browser was not used.

## Completion boundary

This is a selected documentation preview sync. It does not claim that every repository document is deployed. It does not change the official count `78/214`, C11.4/C11 checkbox, or internal contract `0.1.0`; it does not claim CI passed or expand the five-fit implementation scope.
