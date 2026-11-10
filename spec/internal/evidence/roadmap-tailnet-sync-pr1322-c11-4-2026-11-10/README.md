# Tailnet roadmap sync after implementation PR #1322

**Source slice:** C11.4 one-fit repeated-text auto-placement `min-height:auto`.
**Implementation PR:** [#1322](https://github.com/ohah/spinon/pull/1322), merged by Rebase and merge at `2026-11-10T00:37:12Z`; merge SHA `727229aea05e33537962fe9953ee206a37befecb`.
**Scope:** synchronize the updated public roadmap, official status, internal Grid contract, implementation plan, and selected evidence routes to the existing Tailnet preview. Preserve its index and 404 routes; delete no served route.

The official completion count stays `78/214`; C11.4 and C11 remain incomplete; internal numeric contract version stays `0.1.0`. PR #1322 had zero checks, so this sync does not claim CI success.

## Publication evidence

- `selected-root.json` and `selected-publication.json` bind the selected documents and generated pages to the source commit and SHA-256 values.
- `preflight.json` records the Tailscale and expected preview-server checks, all target routes, their prior hashes, open-handle checks, and create/replace/identical totals.
- `copy-and-backup.json` records the changed routes and per-file backups under `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1322-c11-4-2026-11-10/pass04`.
- `served-verification.json` records local and HTTPS HTTP 200/hash checks for every selected route, plus backup hash checks.
- `agent-browser-verification.json` and `agent-browser-lifecycle.json` record desktop/mobile inspection and browser-session closure. Screenshots are included below.
- The preview server remains on its existing process and port; the sync does not restart it or delete routes.

## Agent Browser captures

Agent Browser is used for the live preview. No Terminal Browser is used. The session is closed after inspection.

### Roadmap · desktop

![Tailnet roadmap desktop](./tailnet-roadmap-desktop-agent-browser.png)

### Roadmap · mobile

![Tailnet roadmap mobile](./tailnet-roadmap-mobile-agent-browser.png)

## Reproduction and cleanup

From the repository root, run `prepare-selected-root.mjs`, run the selected RSPress 2.0.22 build using `rspress.config.ts` with `SPINON_DOC_PASS=pass01`, then run `stage-publication.mjs`. Run `sync-tailnet.mjs preflight`, `copy`, and `verify` in order. The copy step checks the recorded source/target hashes and open handles, preserves replaced content in the task-specific backup folder, and creates no deletes.

The generated tree is isolated at `build/spinon/roadmap-tailnet-sync-pr1322-c11-4-2026-11-10/pass04`. After browser verification, remove only this task-owned build tree once `lsof` confirms that no process has it open. Keep the selected manifests, reports, and screenshots in this evidence directory. Preserve shared simulator profiles and prior evidence snapshots.

Preview URL: <https://macstudio.tailed42f2.ts.net/spinon/roadmap.html>

Preview root: `~/Library/Application Support/SpinonPreview`

Existing listener: TCP 8768, `tailnet-preview-server.mjs`.
