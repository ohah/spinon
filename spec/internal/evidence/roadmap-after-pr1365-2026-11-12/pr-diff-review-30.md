# PR diff failure review: roadmap after PR #1365

## Diff identity

- Base: `origin/main` at `943a75d5d4812cd2e788684541261c3627db5589`.
- Scope: `spec/public/roadmap.html`, `plan/c11-4-grid-track-sizing.md`, plus this review and Agent Browser screenshots.
- Purpose: reflect merged PR #1365 in the public roadmap and leave the next C11.4 implementation candidate explicitly unselected.
- Review status: 30 distinct failure perspectives checked; the candidate summary wrapper was restored after Agent Browser exposed a flex-layout regression during review.

## Review perspectives

1. **Merge identity:** The roadmap links to PR #1365 and uses main merge SHA `943a75d5d4812cd2e788684541261c3627db5589`, verified against `origin/main`.
2. **Feature boundary:** The entry names six-fit repeated-text row-span `min-height:auto`; it does not imply auto-placement or general CSS Grid support.
3. **Typed profile:** The displayed cap multiset is `[0px, 10px, 20%, 30px, 40px, 50px]`, matching the merged scope.
4. **Authored permutations:** The 720 supported authored orders are described as the fixed profile, not arbitrary six-fit lists.
5. **Runtime denominator:** The summary says 11,520/11,520 RuntimeSession scenes, matching the final matrix.
6. **Baseline denominator:** The 4,800 floor violations are attributed to baseline observations, not candidate behavior.
7. **Candidate result:** Candidate payload-floor violations are shown as zero without extending that claim beyond the tested profile.
8. **Platform wording:** Android API 37 ARM64 AVD and iPhone 17 Pro/iOS 26.2 Simulator are identified accurately.
9. **Smoke limitation:** The roadmap says platform captures used the default Root HTML route and did not execute six-fit fixture geometry.
10. **Pixel evidence:** No PixelCopy, Metal drawable readback, compositor parity, or physical-device claim was added.
11. **Broad suite result:** The history paragraph retains `spinon-layout --lib` at 544 passed / 1 unchanged C15 assertion failure.
12. **CI state:** Zero GitHub checks/workflows is described as not CI green.
13. **Official completion count:** `78/214` is preserved in both summary and plan.
14. **C11 status:** C11.4 and C11 remain incomplete; no checkbox or completion claim was promoted.
15. **Contract version:** Internal contract `0.1.0` remains unchanged.
16. **Current summary placement:** PR #1365 is now the first bounded-slice summary rather than PR #1360.
17. **Current summary link:** The summary links to the six-fit plan and PR #1365.
18. **Evidence link:** The current runtime README and PR diff review links target existing merged evidence sources.
19. **Stale-stage removal:** The roadmap no longer labels the six-fit implementation as planned or not yet started.
20. **Next candidate honesty:** No unreviewed follow-up feature is presented as selected; the next card asks for remaining-scope review.
21. **Queue semantics:** The next card is a planning step, not an official implementation checkbox or a completion metric.
22. **Kanban count:** The official task count remains 214 with 78 complete; only the explanatory detail card text changes.
23. **WIP count:** The roadmap continues to show no item in progress and respects its two-item WIP limit.
24. **Historical ordering:** The #1365 history entry precedes #1360 while earlier merged history remains intact.
25. **Sequential plan:** `plan/c11-4-grid-track-sizing.md` now records #1365 as complete and leaves the next bounded candidate unselected.
26. **Cross-document consistency:** The public page and sequential plan both say C11.4 remains incomplete and the next feature candidate has not been chosen.
27. **Link route availability:** All new page links point to the merged plan, contract, and runtime evidence paths; the Tailnet sync will publish those pages before final live verification.
28. **Desktop layout:** Agent Browser at 1440×1000 measured a 1440px document/body width; the heading and summary retain a usable 171.8px / 1068.2px split.
29. **Mobile layout:** Agent Browser at 390×844 measured document/body width 390px with no page-level horizontal overflow.
30. **Flex regression check:** An initial render exposed separately flexed summary nodes after the wrapper changed; restoring the `<span>` wrapper fixed the desktop layout, and the final desktop/mobile DOM and screenshots were rechecked.

## Verification evidence

- Agent Browser screenshots: `roadmap-desktop-agent-browser.png`, `roadmap-mobile-agent-browser.png`.
- Browser metrics: desktop and mobile page widths matched their viewport widths; the dynamic board reported 78/214 and the updated C11.4 next card.
- `git diff --check` passed after the final markup correction.
