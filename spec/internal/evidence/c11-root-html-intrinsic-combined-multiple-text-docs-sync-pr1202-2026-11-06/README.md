# C11.4 Root HTML intrinsic multiple-text roadmap sync · 2026-11-06

This documentation change reconciles the official status, internal contract, and public roadmap with merged implementation PR [#1202](https://github.com/ohah/spinon/pull/1202), merged at `009da4f159836139205831c80f8163cb4af52b50`.

## Changes

- `spec/STATUS.md` records the bounded merged slice and its focused/broad test results. The C11.4 and C11 parent items remain incomplete; official completion stays `78/214`.
- `spec/internal/0059-c11-grid-layout.md` changes PR #1199 from candidate to merged status and documents PR #1202's scope, oracle rows, 10,000/10,001 boundary, simulator smoke, broad baseline failures, and limitations. Internal version `0.1.0` is unchanged.
- `spec/public/roadmap.html` already contained PRs #1190, #1195, and #1199; this change adds the missing PR #1202 card and simulator screenshot links without increasing the official count.
- The merged #1202 runtime/precomparison evidence now uses pinned raw GitHub links for images and downloadable artifacts, avoiding Rspress treating local `.png`, `.json`, and `.log` files as HTML pages.
- The implementation's Android and iOS screenshots are linked from the roadmap and are also embedded in the implementation PR body.

## Review

The separate 30-case hostile review of the documentation and sync change is in [docs-pr-diff-review-30.md](./docs-pr-diff-review-30.md).

The repository docs build was attempted three times. The first run exposed several new file-extension links in this change, which were corrected. The final run still fails on many unrelated existing dead links; see [build failure summary](./docs-build-failure-summary.md). No pass claim is made.

## Tailnet preview

The selected Tailnet preview serves a copy under `~/Library/Application Support/SpinonPreview`, not the repository file. Sync, hash, desktop/mobile Agent Browser checks, and browser-process cleanup are recorded in the [Tailnet sync evidence](../roadmap-tailnet-sync-pr1202-2026-11-06/README.md).
