# PR #1350 post-merge Tailnet roadmap sync

## Source and publication

- GitHub `main` and publication source commit: `9949b1e35a90c174e4ac67f37978b46ce60873b4` (PR #1350, Rebase and merge, 2026-11-11 04:12:56 UTC).
- Tailnet host: `macstudio.tailed42f2.ts.net`, serving from the existing Spinon preview root.
- Selected publication: 67 routes, 13,678,567 bytes. The source roadmap SHA-256 is `8ce99fb9bd408d88f1080d65247857989f45de5e418456020f023fc45b6bb118`.
- Copy result: 48 new routes, 8 replaced routes, 11 already-identical routes. All 8 replaced files were backed up and verified under `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1350-2026-11-11/pass01`.
- Served verification: 67/67 local files and 67/67 HTTPS responses matched the selected publication size and SHA-256; HTTPS status was 200 for each. All 8 backups matched their recorded hashes. No routes were deleted.
- Preview server PID remained `90711` throughout preflight, copy, and served verification. It was not restarted.

Machine-readable details are in [preflight.json](./preflight.json), [copy-and-backup.json](./copy-and-backup.json), and [served-verification.json](./served-verification.json).

## Agent Browser confirmation

The user-specified Agent Browser session opened the deployed roadmap and its dedicated C11.4 plan URL. That plan URL, which had returned 404 before publication, rendered after the copy. The roadmap showed PR #1349 and merge SHA `b0725665e31cc50ed6eb65a4083a1b94245dea36`, official status `78/214`, and C11.4/C11 still incomplete.

| Page | Viewport | Document/body width | Result |
|---|---:|---:|---|
| Roadmap | 1280 × 1200 | 1280 / 1280 | Rendered; PR #1349, merge SHA, 78/214 and incomplete C11.4/C11 visible |
| Roadmap | 390 × 844 | 390 / 390 | Rendered after async status loaded; no horizontal page overflow |
| Dedicated plan | 1280 × 1200 | 1280 / 1280 | Rendered with title, sections and linked evidence |
| Dedicated plan | 390 × 844 | 390 / 390 | Rendered; no horizontal page overflow |

Screenshots were captured by Agent Browser:

- [Roadmap, desktop](./tailnet-roadmap-desktop-agent-browser.png)
- [Roadmap, mobile](./tailnet-roadmap-mobile-agent-browser.png)
- [Dedicated plan, desktop](./tailnet-plan-desktop-agent-browser.png)
- [Dedicated plan, mobile](./tailnet-plan-mobile-agent-browser.png)

The plan screenshots record the first post-#1350 publication. During that check its summary still said the merge-state documentation sync was in progress. This was stale wording; the follow-up documentation change records PR #1350 and this Tailnet sync as completed. The screenshots establish that the formerly missing route rendered, not that the stale status wording was current.

The browser check used the Agent Browser CLI and did not use Terminal Browser. This README records the first publication from #1350; the follow-up documentation publication is verified separately after it is merged.

## Resource cleanup

- The task-owned Agent Browser session `spinon-tailnet-pr1350-20261111` is closed and reports `active: false`; the unrelated active HWP session is preserved.
- The task-owned Android AVD clone and temporary diff/capture files were removed after use. The original Android AVD and shared iPhone 17 Pro Simulator data image were preserved.
- All registered iOS simulators were shut down and `Simulator.app` was not running. There was no registered simulator named S03 or Pilot to delete.
- The existing Tailnet preview server and unrelated repository work remain untouched.

## Completion boundary

This publication changes documentation availability only. It does not mark C11.4/C11 complete, change official completion `78/214`, change internal contract `0.1.0`, or claim GitHub CI passed; PR #1349 had zero GitHub checks.
