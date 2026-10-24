# C11.4 cross-axis PR #607 Tailnet roadmap sync

Date: 2026-10-25

## Scope

This sync publishes the documentation update for merged PR [#607](https://github.com/ohah/spinon/pull/607) to the existing Spinon preview at `https://macstudio.tailed42f2.ts.net/spinon/`. PR #607 merged at `2026-10-24T19:06:42Z` as `2f00c2c95061db635facece5233327f895c99319`. The official completion count remains `72/213`; C11.4 remains incomplete.

The bounded Rspress 2.0.22 build selected 18 document/direct-asset routes and 25 Rspress `static/` assets. It was not a full documentation build. Dead-link and dead-image checks were disabled for this selected build because the preview update is scoped to the routes listed in the manifest.

## Build and sync result

The first selected build attempt stopped before any preview writes. Its errors identify the configured `public/icon.svg` favicon path and four PNGs imported by the runtime evidence page as missing from the selected root. Pass02 source preparation included `public/icon.svg`, `public/logo.svg`, and all four imported screenshots at their expected paths; that selected build completed and recorded the merge SHA, `72/213`, `11/11` scenes, and `35/35` exact surface samples.

After the existing 30-point status/roadmap review was corrected to `30/30 PASS`, pass03 rebuilt the selected site so its linked review page would not remain stale. The final document preflight found 10 replacements and 8 already-current routes; all 10 previous files were backed up and hash-verified. The 25 `static/` routes had 12 new hashed assets and 13 already-current assets. No routes were deleted.

The pass03 final verifier checked 43 distinct current routes: 18 document/direct-asset routes and 25 Rspress static assets. All 43 matched their local preview-file SHA-256 hashes and returned HTTP 200 through Tailnet with matching SHA-256 hashes. A separate local HTTP verifier checked the same 43 routes; all 43 returned HTTP 200 with matching byte counts and SHA-256 hashes. All 10 pass03 replacement backups matched their recorded hashes.

## Browser verification

Agent Browser checked the final Tailnet roadmap, status page, plan page, and updated 30-point review for PR #607, merge SHA `2f00c2c95061db635facece5233327f895c99319`, and `72/213`. The updated review page showed its follow-up as passed. On the runtime evidence page, all four linked simulator screenshots loaded:

- Android empty-area: 1080 × 2400
- iOS empty-area: 1206 × 2622
- Android overflow: 1080 × 2400
- iOS overflow: 1206 × 2622

The roadmap capture is [`roadmap-pr607-agent-browser.png`](roadmap-pr607-agent-browser.png); it is centered on the PR #607 entry. Its SHA-256 is `757c16c3ceb23db4b0ca519f411ad64611c3ee1dc0b170cb8f33cabb5c2b0b1a`. Compact page and image checks are in `tailnet-agent-browser-verification-pass03.log`.

## Evidence files

- `selected-build-pass02.log`, `selected-root-pass02.json`, and the pass02 verification files: the first successful selected build and sync record.
- `selected-build-pass03.log`, `selected-root-pass03.json`, `selected-build-verification-pass03.json`, and `selected-static-assets-pass03.json`: the final selected build and 43 route manifests.
- `tailnet-preflight-pass03.json`, `tailnet-copy-pass03.json`, `tailnet-verify-pass03.json`, and `tailnet-final-verification-pass03.json`: final document/asset copy, Tailnet HTTP/hash checks, and backup verification.
- `tailnet-assets-preflight-pass03.json`, `tailnet-assets-copy-pass03.json`, and `tailnet-assets-verify-pass03.json`: final static asset preflight and verification.
- `local-http-verification-pass03.json` and `local-http-verification-output-pass03.log`: local preview HTTP and SHA-256 checks for all 43 routes.
- `tailnet-agent-browser-verification-pass03.log` and `roadmap-pr607-agent-browser.png`: final rendered page/image checks and the roadmap capture.
- `prepare-selected-root-pass03.mjs`, `rspress-pass03.config.ts`, `sync-tailnet-pass03.mjs`, `sync-tailnet-assets-pass03.mjs`, `verify-local-http-pass03.mjs`, and `verify-tailnet-final-pass03.mjs`: final selected build, copy, and verification commands.

Rspress build stdout is preserved verbatim. Its fixed-width bundle table has trailing padding, so `git diff --cached --check` reports those generated log lines; the source documents and other evidence files pass the whitespace check. The adjacent JSON manifests retain the machine-readable route and hash results.

These results verify the selected documentation preview and its linked images. They do not establish full-site build coverage or broader CSS conformance beyond the simulator evidence recorded for PR #607.
