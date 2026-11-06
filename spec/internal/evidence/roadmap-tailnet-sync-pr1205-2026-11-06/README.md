# PR #1205 Tailnet roadmap synchronization · 2026-11-06

The Tailnet preview now contains the merged PR #1205 C11.4 progress card. C11.4 and C11 remain incomplete; the official count is `78/214`.

## Source and safe copy

The final selected source `spec/public/roadmap.html` was 590,947 bytes (SHA-256 `02c61ff2923610bd7b7c9359af0fc2ec6c47651e063d86ffe734a2e2d0207acc`). The original page was 589,048 bytes (SHA-256 `92940ba1f896a096126b7ed5b5caa7fa4450abf461992f9e779c42bac090ab95`) and remains preserved at `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1205-2026-11-06/pass01/roadmap.html`. A first synced wording pass was preserved in `pass02/roadmap.html` under the same backup root before the final wording was atomically applied.

Final preflight found Tailscale `Running`, the expected preview server PID `28040` on `127.0.0.1:8768`, local HTTP 200, and no open handle for `roadmap.html`. The final copy replaced only that regular file using atomic rename; it deleted nothing and did not restart the server. Local HTTP and Tailnet HTTPS both returned 200 with 590,947 bytes and the source SHA-256. Final reports: `preflight-pass02.json`, `copy-and-backup-pass02.json`, and `served-verification-pass02.json`.

- Scripts: [initial sync](./sync-tailnet-roadmap.mjs) · [final wording sync](./sync-tailnet-roadmap-pass02.mjs)
- The pass02 backup hash matches the immediately previous served-page hash.
- Tailnet preview server remains active.

## Agent Browser review

The final live page was checked with Agent Browser at desktop 1440×1000 and mobile 390×844. Both widths have no horizontal overflow. The #1205 card is 928px wide at desktop and 362px at mobile, and it visibly retains `78/214`. Reports and screenshots: [desktop report](./tailnet-agent-browser-desktop-pass02.json) · [mobile report](./tailnet-agent-browser-mobile-pass02.json) · [desktop capture](./tailnet-roadmap-agent-browser-desktop-pass02.png) · [mobile capture](./tailnet-roadmap-agent-browser-mobile-pass02.png).

Both commit-pinned simulator screenshot links loaded completely in Agent Browser: Android 1080×2400 and iOS 1206×2622 ([Tailnet page report](./tailnet-image-link-browser-check-pass02.json)). The four images embedded in PR #1206 also loaded completely: Android 1080×2400, iOS 1206×2622, Tailnet desktop 1440×1000, and Tailnet mobile 390×844 ([PR body report](./pr-body-image-browser-check.json)). `agent-browser close --all` completed; no Agent Browser or HeadlessChrome processes remained ([process report](./agent-browser-process-check-pass02.json)).

## Scope and review

The card records PR #1205 at merge SHA `e35e49bba4bc14a9b40492d62ced01289d71bca8`. It documents only ASCII SPACE/TAB/LF/CR separators before, after, and between standalone `A` tokens in the existing default parsed Root HTML `grid-auto-rows:min-content` path. It does not mark C11.4 or C11 complete, change `78/214`, or claim surface pixel readback or CI success.

The 30-point review is [roadmap-pr-diff-review-30.md](./roadmap-pr-diff-review-30.md). No discrepancy was found. File hashes are recorded in `SHA256SUMS.txt`.
