# C11.1 fractional `grid-auto-rows` PR #1070 merge record

- PR: [#1070](https://github.com/ohah/spinon/pull/1070)
- Merge method: Rebase and merge
- Merged at: `2026-11-02T16:37:30Z`
- Merge commit: `f40cb3b93d2555dafecd352bd35fb349b6bf7a64`
- Fetched `origin/main` at verification and confirmed it points to the merge commit.
- PR check rollup was empty. GitHub reports main required status checks are not enabled; this is not recorded as CI success.
- The two PR screenshots loaded in Agent Browser: Android `1080×2400`, iOS Simulator `1206×2622`. They are visual smoke captures, not surface readback.
- The known style-to-layout Root HTML regression remains `36/37` because of the unchanged negative auto-column fallback assertion documented in the PR and runtime README.
- The bounded implementation does not complete C11.1 or C11; official status remains `72/213`.
