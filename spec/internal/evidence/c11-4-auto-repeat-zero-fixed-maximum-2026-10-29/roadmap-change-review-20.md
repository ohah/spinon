# C11.4 zero fixed maximum public roadmap update: 20 failure perspectives

Reviewed the `spec/public/roadmap.html` entry for merged PR #801 against the official status ledger and the merged simulator evidence on 2026-10-29.

1. **Wrong implementation PR number** — PASS. The new card links to merged PR #801.
2. **Wrong merge time** — PASS. It records `2026-10-29T08:55:22Z` from GitHub's merged PR state.
3. **Wrong merge SHA** — PASS. It records `5ff030f18146e17f0a7d5e17f5afb350d0fe31e3` from the Rebase and merge commit.
4. **C11.4 is accidentally presented as complete** — PASS. The card says the parent remains incomplete and links the official status ledger.
5. **Official completion total is inflated** — PASS. It retains `72/213`, matching `spec/STATUS.md`.
6. **Zero maximum is confused with a zero repeat count** — PASS. The text distinguishes the 1px UA repeat floor and reports 16/4 tracks for gap 0/4.
7. **A Grid axis is omitted** — PASS. The card names Columns and Rows.
8. **An intrinsic minimum is omitted** — PASS. It says three intrinsic minimum values and the evidence page lists each one.
9. **`auto-fill` and `auto-fit` are conflated** — PASS. Both modes are named; the evidence gives their collapse/offset differences.
10. **Gap inputs are overstated** — PASS. Only the tested 0px and 4px gaps are claimed.
11. **Scene count is misstated** — PASS. Both simulator runs report 24/24 from `verification.json`.
12. **Android result is described as physical-device or hardware-GPU proof** — PASS. The card calls out the API 37 ARM64 QEMU AVD boundary.
13. **iOS Metal readback is described as final displayed pixels** — PASS. It is explicitly marked pre-compositor.
14. **Pixel sample counts are misstated** — PASS. The card reports PixelCopy and Metal 54/54 as verified.
15. **Artifact controls become a universal packaging claim** — PASS. The linked README limits negative/positive markers to inspected artifacts.
16. **General author CSS support is implied** — PASS. The card says this is a fixed opt-in fixture, not general CSS admission.
17. **A screenshot link points at an old scene or platform** — PASS. Android and iOS links point to the exact `auto-fill`, gap-4 PNGs from the final verified runs.
18. **Plan/evidence/review links point to a nonexistent route** — PASS. Each source path exists on main and is present in the Rspress source tree.
19. **Existing roadmap history is rewritten or removed** — PASS. The change inserts a new C11.4 progress paragraph and leaves existing cards intact.
20. **This local diff is described as published before deployment** — PASS. The PR records the source change only; Tailnet publication and Agent Browser verification are separate post-merge steps.

The update changes only the public progress card. It does not alter roadmap checkboxes, official status counts, or implementation scope.

`mise exec -- bun run docs:build` was attempted twice after the source edits. Rspress exits nonzero on 1,041 dead-link source groups elsewhere in the existing `spec/` corpus. After removing relative Markdown image imports from the evidence README, the final build log has no error references to the new zero-fixed-maximum evidence directory. The PR does not claim the full docs build passed.
