# C11.4 merged-status and roadmap docs PR diff review · 30 points

This review covers the post-merge documentation reconciliation for implementation PR #1224. It distinguishes the checked-in standalone roadmap source from the live Tailnet preview copy.

| # | Adversarial documentation check | Evidence inspected | Result |
|---:|---|---|---|
| 1 | Merge timestamp or SHA differs from GitHub | `gh pr view 1224`: `2026-11-06T14:35:05Z`, `4039ed60327b09128503559fcc45268acbb9637e` | Matches all updated files |
| 2 | PR link points at a different change | GitHub PR #1224 URL and ledger link | Matches |
| 3 | C11 parent checkbox accidentally becomes complete | `spec/STATUS.md` C11 row | Remains unchecked |
| 4 | C11.4 completion is implied by the merged slice | Status, plan, and contract wording | Explicitly incomplete |
| 5 | Official completion counter increments for a bounded implementation | `78/214` in status, plan, contract, runtime evidence, public roadmap | Unchanged |
| 6 | Limited fixture profile is described as public CSS support | Status, plan, internal contract, roadmap | Scope explicitly bounded |
| 7 | Accepted grammar accidentally widens beyond ASCII | `[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*` references | ASCII-only internal U+002D |
| 8 | Leading, trailing, repeated, or Unicode hyphens disappear from rejection boundary | Plan and contract | Rejection boundary retained |
| 9 | U+200B marker is described as source text | Plan and contract mapping text | Normalized layout buffer only |
| 10 | Source/Text node identity guarantee is omitted | Runtime evidence and contract | Preserved and stated |
| 11 | Focused test counts drift from captured run | Status, plan, contract, runtime README | Counts agree: style 12, runtime 16/16, line-break 4, auto 3 |
| 12 | Android device is misrepresented as physical or hardware GPU | Status and public roadmap | API 37 ARM64 emulator; ANGLE/Vulkan/SwiftShader |
| 13 | iOS sample is represented as post-compositor output | Status, runtime README, roadmap | Metal drawable before compositor |
| 14 | One sampled pixel is presented as full-frame equality | Evidence and public roadmap | Exactness limited to one sample per platform |
| 15 | Visual screenshots are treated as pixel proof | Runtime README and public roadmap | Visual smoke distinguished from one-pixel readback |
| 16 | Existing Clippy failure is hidden | Status and runtime README | Two unchanged helper lints disclosed |
| 17 | Local Clippy is described as GitHub CI success | Status and runtime README | GitHub PR check rollup explicitly empty |
| 18 | Scope quietly expands to WPT or arbitrary CSS | Plan and contract | Both remain outside this slice |
| 19 | Unicode word boundaries or general hyphenation are claimed | Plan, contract, status | Explicitly excluded |
| 20 | Text selection/copy semantics are claimed from raster evidence | Contract and runtime README | Explicitly excluded |
| 21 | Plan still says implementation has not started | Plan status | Updated to merged PR #1224 |
| 22 | Internal contract heading leaves the implementation in a stale candidate state | Contract heading | Records bounded merged PR #1224 |
| 23 | Internal contract number is bumped without an interface version change | Contract version header and project rule | Fixed `0.1.0` retained |
| 24 | Static public roadmap retains the stale “implementation not started” record | `spec/public/roadmap.html` targeted C11.4 entry | Replaced by merged-slice record |
| 25 | Latest C11.4 entry lacks a stable PR link | Public roadmap entry | Links to merged PR #1224 |
| 26 | Roadmap screenshot links point at a mutable or missing asset | Raw GitHub commit URLs | Pinned to merge SHA `4039ed6…` |
| 27 | Roadmap links to the unmerged work branch | New plan/evidence links | Use `main` paths after merge |
| 28 | Repository source is claimed to update the live Tailnet copy automatically | Docs sync README | Explicitly says Tailnet copy is a separate post-merge step |
| 29 | Tailnet deployment is claimed before HTTP/browser verification | Docs sync README | No deployment claim made here |
| 30 | Documentation change adds another completion checkbox or changes counts | `git diff` and STATUS header | No checkbox/count changes |

The review found no documentation inconsistency in the bounded merged status. Live Tailnet delivery is not part of this PR's evidence and must be verified after merge.
