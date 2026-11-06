# Rspress docs build result · 2026-11-06

Command: `mise exec -- bun run docs:build` (Rspress 2.0.22).

The build exits with code 1 because the repository-wide docs link checker reports many dead links in existing status/evidence pages. The errors include unchanged links in `spec/STATUS.md` and `spec/internal/0059-c11-grid-layout.md`, plus unchanged Markdown evidence in the already merged PR #1202 runtime/precomparison folders.

The first run also caught links newly added by this documentation change to local `.png`, `.json`, and `.mjs` files. Those links were changed to raw GitHub links or plain code filenames. A later pass also converted the merged #1202 runtime/precomparison evidence's local `.png`, `.json`, and `.log` links to pinned raw GitHub URLs. The final build still exits 1 on numerous unrelated existing dead links in status/evidence pages; no dead-link entry names the new docs-sync/Tailnet-sync evidence directories or the touched #1202 runtime/precomparison paths. This PR does not claim the full build passes or repair unrelated repository links.

The Tailnet roadmap itself was separately verified in Agent Browser at desktop and mobile viewports, and its source/local/remote byte hashes match. That preview check does not replace the failed Rspress build.
