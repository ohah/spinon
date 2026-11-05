# C11.4 parsed Root HTML intrinsic implicit rows · implementation/runtime review 20

Candidate under review: current checkout `codex/c11-intrinsic-auto-rows`, before implementation PR creation. This review covers the implementation and bounded RuntimeSession behavior; it is separate from the 20-item plan review and the actual PR diff review that must be made against the final PR head.

1. **Reachability:** the route is selected from the existing `RuntimeRootHtmlTextPaintV1` style profile and parsed Root HTML fixture. No fixture-only success path or new public selector was added. The Android/iOS app route exercised the default bootstrap path.
2. **Typed cascade value:** Stylo preserves `grid-auto-rows:min-content` as typed `ComputedCssGridAutoRows::TrackList([MinContent])`; the focused cascade test passes 1/1.
3. **Single-token admission:** only a singleton computed `MinContent` implicit-row track selects the candidate. `max-content` and a multi-track list reject.
4. **Route priority:** direct-text aspect-ratio scene owners are excluded before candidate item admission, preserving their earlier route decision.
5. **Explicit track boundary:** each explicit axis remains one positive fixed-px track. Combining intrinsic explicit tracks with this auto-row candidate rejects.
6. **Owner topology:** the candidate requires one visible Grid owner. A second visible owner rejects before layout publication.
7. **Item topology:** only visible direct HTML `div` elements with no child nodes are admitted. Direct formatting whitespace is ignored as non-item input.
8. **Text boundaries:** non-empty `div` descendants reject as `grid-auto-rows`; direct anonymous text rejects in the existing Root HTML tree preparation path. Both cases publish no layout or render snapshot.
9. **Element boundaries:** a different direct element name rejects; a nested Grid item rejects. Existing unsupported nested Grid CSS can also fail earlier during cascade.
10. **Item sizes:** computed width and height must each be finite, non-negative fixed CSS px. An auto-sized item rejects.
11. **Item constraints:** min/max sizes must remain at their defaults, aspect ratio must be unset, and item positioning must be static. A non-default minimum-size input rejects.
12. **Placement semantics:** all four typed row/column placement longhands must be `auto`; explicit or partial placement rejects instead of being silently normalized to auto-placement.
13. **Flow and implicit axes:** row flow and no implicit columns are required. Dense/column flow, repeats, named areas/lines, and other track functions remain outside the candidate.
14. **Gaps:** row and column gaps must be zero. Both nonzero-row-gap and nonzero-column-gap controls reject.
15. **Box and alignment limits:** definite positive container axes, zero margin/padding/border, visible overflow, LTR horizontal writing, and the existing `normal`/`start` alignment subset remain enforced by the layout contract.
16. **Typed projection:** the projection test passes 1/1 and the Root HTML grid-capacity tests pass 2/2; the bounded `MinContent` track reaches `RootHtmlIntrinsicTrackSizingV1` without CSS text reparsing.
17. **Track sizing:** focused layout coverage passes 11/11, including `[10,22,34]` track bases, implicit row placement, overflow, invalid track rejection, and the existing 10,000-track ceiling.
18. **Frames and paint:** RuntimeSession passes exact one-row and two-row frame checks against the independent expected JSON; paint boxes preserve DOM order and item identity.
19. **Atomic failure and revision:** all 13 unsupported RuntimeSession variants end in `Failed` with `completed=None`; the supported source checks matching cascade/layout keys and retains the third item through y=66px.
20. **Native app evidence:** Android API 37 ARM64 AVD and iPhone 17 Pro/iOS 26.2 Simulator both reached `layout=ready` and draw/present for the same fixture hash; DPR-scaled screenshot color runs match the three row heights. These are OS screenshot checks, not PixelCopy/Metal drawable readback, physical Android GPU evidence, iOS post-compositor parity, or WPT evidence.

Focused logs, simulator artifacts, hashes, and broad-filter baseline are indexed in [runtime evidence](README.md). The candidate broad Root HTML filter is 133 passed/9 failed; the three-file `HEAD` baseline is 131 passed/9 failed with the same nine C11.5 failures. Neither broad run is green. PR diff review against the final GitHub PR head remains a separate required review.
