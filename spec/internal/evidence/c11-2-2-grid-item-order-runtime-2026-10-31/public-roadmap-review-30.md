# Public roadmap C11.2.2 order note review · 30 perspectives

Date: 2026-10-31. This review covers the visible C11.2.2 note added to `spec/public/roadmap.html` after PR #899 merged.

1. **Correct parent section — PASS.** The note is placed between C11.2.2 and C11.2.3 descriptions.
2. **Scope label — PASS.** It says this is a limited follow-up, not broad C11 support.
3. **Merged implementation link — PASS.** PR #899 points to the actual merged implementation and runtime evidence.
4. **Existing scene matrix — PASS.** The text says the original 16-scene matrix remains unchanged.
5. **New scene count — PASS.** It reports only the two fixed order scenes.
6. **Sparse mode — PASS.** It names fixed row-sparse placement.
7. **Auto placement guard — PASS.** It restricts ordered support to cases where every visible direct child is auto-placed.
8. **Signed order — PASS.** It says signed `order` values are used.
9. **Stable ties — PASS.** It preserves source order for equal values.
10. **Chrome oracle — PASS.** It identifies Chrome 156 as the comparison baseline.
11. **Geometry count — PASS.** It reports 11 frame comparisons on each platform.
12. **Geometry tolerance — PASS.** It reports the observed maximum error as 0 CSS px.
13. **Surface sample count — PASS.** It reports seven samples per platform.
14. **Surface result — PASS.** PixelCopy and Metal samples are described as exact.
15. **Android target — PASS.** It identifies Android 17/API 37 ARM64 AVD.
16. **Android renderer — PASS.** It identifies ANGLE/SwiftShader rather than physical GPU hardware.
17. **iOS target — PASS.** It identifies iPhone 17 Pro/iOS 26.2 Simulator.
18. **iOS readback boundary — PASS.** It says Metal samples are before compositor submission.
19. **Overlap limitation — PASS.** It explicitly says the non-overlapping scenes do not verify paint/stacking.
20. **WPT boundary — PASS.** It does not claim full WPT conformance.
21. **C11 status — PASS.** It says full C11 remains incomplete.
22. **Official completion count — PASS.** It preserves `72/213`.
23. **No new checkbox — PASS.** The added note is a paragraph, so it does not increase the denominator or checked count.
24. **Checkbox source — PASS.** The roadmap still reads official checked state from main's `spec/STATUS.md` path.
25. **Status source freshness — PASS.** Agent Browser shows 213 disabled checkboxes with 72 checked.
26. **PR status consistency — PASS.** PR #899 is merged and its Rebase timestamp/SHA are recorded in the source status and plan.
27. **No contradictory “open PR” text — PASS.** The visible note says merged and the related status documentation no longer calls it open.
28. **HTML escaping — PASS.** The inserted text uses the existing note structure and does not add unescaped query or ampersand markup.
29. **Diff scope — PASS.** The only product-visible change is one C11.2.2 roadmap paragraph; no checkbox, script, or layout behavior changed.
30. **Preview publication boundary — PASS.** The note does not publish automatically; Tailnet preview synchronization is a separate explicit step after merge.

The follow-up is limited to making the merged C11.2.2 `order` evidence visible in the roadmap; it does not change completion accounting.
