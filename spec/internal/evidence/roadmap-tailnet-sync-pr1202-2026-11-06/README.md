# PR #1202 Tailnet roadmap sync · 2026-11-06

The preview at [macstudio.tailed42f2.ts.net/spinon/roadmap.html](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html) now contains the PR #1202 C11.4 card. C11.4/C11 remain incomplete and the official count stays `78/214`.

## Source and copy

- The selected source was `spec/public/roadmap.html`, 589,048 bytes, SHA-256 `92940ba1f896a096126b7ed5b5caa7fa4450abf461992f9e779c42bac090ab95`.
- Preflight confirmed Tailscale `Running`, the expected preview server PID `28040` on `127.0.0.1:8768`, no open handle for the target, and local HTTP 200.
- The previous 586,947-byte page (SHA-256 `4c56c7e6e0f23c9a2282882a720d67d09d717604f6404ccc640f85089dd97e54`) is preserved at `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1202-2026-11-06/pass01/roadmap.html`.
- The sync replaced only `roadmap.html` by atomic rename. It deleted no files and did not restart or stop the preview server.
- Local HTTP and Tailnet HTTPS both returned 200 with 589,048 bytes and the same source SHA-256. The reports are `preflight.json`, `copy-and-backup.json`, and `served-verification.json` in this directory.

## Agent Browser review

- Desktop viewport 1440×1000: document width 1440px, no horizontal overflow; the PR #1202 card is x=90px and 928px wide. [Screenshot](https://raw.githubusercontent.com/ohah/spinon/main/spec/internal/evidence/roadmap-tailnet-sync-pr1202-2026-11-06/tailnet-roadmap-agent-browser-desktop.png) · DOM report: `tailnet-agent-browser-desktop.json`.
- Mobile viewport 390×844: document/body width 390px, no horizontal overflow; the card is x=14px and 362px wide. [Screenshot](https://raw.githubusercontent.com/ohah/spinon/main/spec/internal/evidence/roadmap-tailnet-sync-pr1202-2026-11-06/tailnet-roadmap-agent-browser-mobile.png) · DOM report: `tailnet-agent-browser-mobile.json`.
- Agent Browser opened both raw simulator screenshot links from the card. Android loaded completely at 1080×2400 and iOS at 1206×2622. Report: `tailnet-image-link-browser-check.json`.
- Agent Browser also loaded all four images embedded in PR #1203's body (`complete=true`): Android 1080×2400, iOS 1206×2622, Tailnet desktop 1440×1000, and Tailnet mobile 390×844. Report: `pr-body-image-browser-check.json`.
- The linked Tailnet `STATUS.html` still displays `78/214` and leaves C11.4 incomplete. Its rendered copy does not yet include a PR #1202 detail row; that row is present in the synchronized roadmap card and the updated Markdown source `spec/STATUS.md`.
- `desktop-scroll-offset-diagnostic-not-used.png` is a retained initial scroll-position capture; it is not the desktop verification screenshot.
- The screen captures show the Tailnet page layout and links. They are not simulator pixel or CSS conformance evidence.
- `agent-browser close --all` left no Agent Browser sessions or headless browser processes. Tailnet preview server PID `28040` remains active on `127.0.0.1:8768`. See `agent-browser-process-check.json`.

The copy script is [sync-tailnet-roadmap.mjs](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1202-2026-11-06/sync-tailnet-roadmap.mjs). It uses exclusive report/backup creation, checks the serving process and target, hashes source/backup/local/remote bytes, and only replaces the selected regular file. See `SHA256SUMS.txt` for the evidence file hashes.
