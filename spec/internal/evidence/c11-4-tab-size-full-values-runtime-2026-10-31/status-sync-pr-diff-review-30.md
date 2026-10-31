# C11.4 merge-state documentation diff review · 30 checks

**Scope:** documentation-only follow-up on `docs/c11-4-tab-size-pr893-merge-record`, based on `origin/main` after PR #893. The implementation was already merged; this change records its actual state and does not change product code or completion checkboxes.

| # | Documentation failure path checked | Finding |
| --- | --- | --- |
| 1 | Candidate plan still says PR creation is pending | Updated to identify PR #893 as merged. |
| 2 | Candidate plan omits merge timestamp | `2026-10-31T02:54:29Z` is recorded in UTC. |
| 3 | Candidate plan omits merge SHA | Full SHA `ae292fff135a925ef30a3e2437082e96c2386e9c` is recorded. |
| 4 | Status ledger still says merge validation is pending | Updated to the same merged state and timestamp. |
| 5 | Internal contract still says merge is pending | Updated to the same merged state and SHA. |
| 6 | Runtime evidence README still says PR creation is pending | Updated to the merged state. |
| 7 | Different documents disagree on PR number | All point to PR #893. |
| 8 | Documents disagree on merge method | All record Rebase merge. |
| 9 | Documents claim CI passed from an empty rollup | They state the rollup was empty and do not call it a CI pass. |
| 10 | Main required-check state is omitted | They state required checks were not enabled. |
| 11 | PR head is confused with the merge commit | Head `9db4300f8b4e1bc37f91e9264b0b13c5abb35ab5` and merge `ae292fff135a925ef30a3e2437082e96c2386e9c` remain distinct in review evidence. |
| 12 | PR base is incorrect | Base is `e3ec8a24b2ff463519dc79968f4cbba35d1226e3`. |
| 13 | `spec/STATUS.md` accidentally checks C11 | Parent C11 remains unchecked. |
| 14 | `spec/STATUS.md` accidentally checks C11.4 | Parent C11.4 remains unchecked. |
| 15 | Official counter changes because a limited fixture merged | `72/213` remains unchanged. |
| 16 | Merge is described as public CSS support | The candidate remains opt-in fixture support only. |
| 17 | Internal contract version is bumped | Version remains `0.1.0`. |
| 18 | Android matrix result is altered during status edit | 140/140, rejection 0, retry 0 remains unchanged. |
| 19 | Android target is described as a physical device | It remains an API 37 ARM64 16 KB QEMU AVD. |
| 20 | Android software renderer is generalized to hardware | The software-renderer limitation remains explicit. |
| 21 | iOS result is altered during status edit | 140/140, rejection 0, retry 0 remains unchanged. |
| 22 | iOS Simulator is described as post-compositor proof | The pre-compositor drawable boundary remains explicit. |
| 23 | Installed binary identity is lost | Local/installed hashes remain in simulator evidence. |
| 24 | Representative images are presented as full-matrix captures | README says each came from a separate one-scene capture run. |
| 25 | Black canvas is presented as glyph rendering proof | README explicitly says glyph ink equality is not established. |
| 26 | Failed API 37.0 AVD evidence is removed | ANR record remains linked. |
| 27 | Accidental partial run is counted as passed | Its 20/140 partial status remains excluded. |
| 28 | Implementation review 20 is dropped | Link remains in README and plan. |
| 29 | PR diff review 30 is dropped | Link remains in README and plan. |
| 30 | Status evidence links point outside the candidate evidence folder | All relative links resolve within the current repository layout. |

**Disposition:** no status mismatch remains in the reviewed documentation. This follow-up records the merge state without promoting the limited candidate to official C11.4/C11 completion.
