# Public roadmap PR #713 update · 30 diff failure perspectives

## Scope

This review covers the single new C11.4 note in `spec/public/roadmap.html`, which records the already merged PR #713 and links its plan, runtime evidence, reviews, and Android/iOS screenshots. It does not change the implementation, support checkbox, or completion counter.

The first wording pass left the 16-scene matrix ambiguous and blurred the strict Clippy failure with the GitHub check state. Both statements were clarified before this final review.

## Failure perspectives

1. **Wrong section placement** — Verify the new note is inside the C11.4 implementation progress section. **PASS** — it is adjacent to the latest #710 entry and before the section closes.
2. **Chronology regression** — Check that the row follows existing #706 and #710 notes. **PASS** — #713 is appended after them.
3. **Wrong PR target** — Check the GitHub href. **PASS** — it links to `https://github.com/ohah/spinon/pull/713`.
4. **Wrong merge claim** — Compare text with the merged PR state. **PASS** — it says Rebase and merge, matching GitHub.
5. **Timestamp drift** — Compare the displayed UTC time with `pr-713-merge-state.json`. **PASS** — `2026-10-27T21:00:41Z` matches.
6. **Merge SHA typo** — Compare the displayed SHA with the captured merge commit. **PASS** — `679b8d51bc3b4edc2cbd1c4f5921e3b73a12dabe` matches.
7. **Generic route overstated** — Check whether the note preserves the limited novel-ID adapter scope. **PASS** — it describes four mixed row scenes and the existing route boundaries.
8. **Fixed route erased** — Ensure the fixed-ID row route is described as retained. **PASS** — the note states that it was retained.
9. **Generic column regression implied** — Ensure the existing generic column route is still represented. **PASS** — the note names it as a retained comparison route.
10. **Ambiguous 16-scene count** — Check whether the reader could interpret 16 scenes per category. **FIXED** — the note now says each platform passed 16/16 total, including fixed and generic column/row controls.
11. **Android platform drift** — Verify the Android result is attributed to the API 37 ARM64 16 KB AVD. **PASS** — the exact target is stated.
12. **Physical-device overclaim** — Check that simulator evidence is not described as hardware-device evidence. **PASS** — both targets are explicitly simulators.
13. **iOS platform drift** — Verify the named iPhone 17 Pro / iOS 26.2 Simulator target. **PASS** — it matches the runtime evidence.
14. **Runtime test count drift** — Compare 13/13 with the runtime README. **PASS** — the count matches.
15. **FFI test count drift** — Compare 84/84 with the runtime README. **PASS** — the count matches.
16. **Strict Clippy failure hidden** — Check that the two existing lint failures remain visible. **PASS** — `type_complexity` and `map_identity` are named.
17. **CI state conflated with Clippy** — Check that local Clippy and GitHub checks are reported separately. **FIXED** — the note now states that the GitHub check rollup was empty and makes no CI-pass claim.
18. **Empty checks called green** — Check whether empty rollup is described as success. **PASS** — it explicitly avoids claiming CI passed.
19. **Official count incremented** — Verify the note keeps `72/213`. **PASS** — no completion count change is claimed.
20. **C11.4 marked complete** — Check parent status wording. **PASS** — it explicitly says C11.4 remains incomplete.
21. **Algorithm distinction overclaimed** — Check the min/max row assertion against the fixed oracle. **PASS** — the note says this input does not prove a difference.
22. **General CSS admission implied** — Check whether the fixture is presented as broad author CSS support. **PASS** — it remains a limited fixed-input runtime slice.
23. **Plan route broken** — Resolve the focused plan link to the source plan and planned `.html` route. **PASS** — both exist in the merged repository/build inputs.
24. **Evidence route broken** — Resolve the evidence README link. **PASS** — the source README exists and the selected publication includes its generated page.
25. **Implementation review link broken** — Resolve the 20-perspective review link. **PASS** — the Markdown source exists.
26. **PR diff review link broken** — Resolve the 30-perspective review link. **PASS** — the Markdown source exists.
27. **Android screenshot route broken** — Resolve the linked PNG to the 16-scene Android run. **PASS** — the PNG exists and has a valid image signature.
28. **iOS screenshot route broken** — Resolve the linked PNG to the iOS Simulator run. **PASS** — the PNG exists and has a valid image signature.
29. **HTML structure damaged** — Check the inserted note's paragraph, bold, code, and anchor closing tags and surrounding section boundary. **PASS** — tags are balanced and the existing section close remains after the note.
30. **Unrelated roadmap content changed** — Inspect the final diff. **PASS** — only one C11.4 note line was added; no other roadmap content or counts changed.

## Result

No blocking issue remains in the final one-line roadmap addition. `git diff --check` is clean. The review validates the summary against the already merged PR #713 and its captured evidence; it adds no new runtime or CI result.
