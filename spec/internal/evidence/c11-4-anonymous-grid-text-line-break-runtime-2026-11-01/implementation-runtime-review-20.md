# C11.4 computed `line-break` implementation/runtime review (20 perspectives)

Reviewed the candidate implementation against its fixed Chrome 156 oracle,
runtime tests, Android/iOS Simulator captures and static FFI archive controls.
Each item records the bounded result rather than a general CSS support claim.

1. **CSS oracle scope:** The only punctuation distinction is U+301C in
   `文〜文`; the expected break opportunity follows the linked CSS Text 3,
   CSS Sizing 3 and CSS Grid 1 rules. Pass; arbitrary punctuation is outside
   this candidate.
2. **Scene catalog:** Five unique IDs cover Japanese normal/strict
   min-/max-content and English normal min-content. Pass; unknown IDs are
   rejected.
3. **Direct language owner:** The runtime accepts exactly one unnamespaced
   HTML `lang` on the Grid style owner and matches it to the scene. Pass.
4. **Language inheritance boundary:** An explicit ancestor-only `lang="ja"`
   negative control is rejected because the owner itself has no language.
   Pass; this is a fixture input rule, not browser inheritance support.
5. **XML language:** `xml:lang` is rejected for the fixed fixture. Pass.
6. **Computed values:** Each scene requires its exact `normal` or `strict`
   computed value; `auto`, `loose`, `anywhere` and scene/value mismatches are
   rejected. Pass.
7. **Existing `auto` behavior:** Unrelated anonymous text fixtures retain the
   pre-existing computed `auto` expectation and Strict paragraph default.
   Pass; no global parser mapping changed.
8. **Locale delivery:** `ja`/`en` reaches the same anonymous paragraph as the
   line-break mode. Pass; positive runtime reports and text-layout tests
   verify this.
9. **Font determinism:** Noto Sans KR is pinned to SHA-256
   `9e1d729e7e2b36f9ef439da102f8c134c10aabe46f1c843bf0aca5c043b86f76` and
   registered only for the relevant opt-in scenes. Pass.
10. **Tree shape:** Generated source has one HTML Grid owner and exactly one
    direct Text node; wrong text and extra child mutations are rejected. Pass.
11. **Computed style:** Runtime tests compare each scene's computed
    `line-break` and document revision before accepting the result. Pass.
12. **Typography profile:** Font family, size, weight, line height, writing
    direction, whitespace, word-break and spacing remain fixed. Wrong font and
    property mutations are rejected. Pass.
13. **Min-content inline size:** Japanese normal resolves to 10px; Japanese
    strict and English normal resolve to 20px. Pass in focused text/runtime
    checks and both platform traces.
14. **Max-content inline size:** All three glyph advances sum to 30px. Pass in
    the fixed Noto font and the two max-content simulator scenes.
15. **Min-content block size:** At 10px line-height, three Japanese normal
    lines occupy 30px; strict Japanese and English normal occupy 20px. Pass.
16. **Max-content block size:** The one-line max-content profile occupies
    10px. Pass.
17. **Grid columns:** The scene must use the exact intrinsic track keyword;
    opposite min-/max-content and fixed 120px controls are rejected. Pass.
18. **Grid rows and frames:** Android and iOS traces verify final Columns and
    Rows sizes plus root/item geometry for all five scenes. Pass within the
    declared 0.05 CSS px text/layout tolerance.
19. **Revision and failure publication:** A stale document revision, unlisted
    scene, missing/mismatched language, XML language, wrong text/property and
    extra child are rejected before a valid layout result is returned. Pass.
20. **Platform and archive limits:** Android API 37 ARM64 AVD and iPhone 17
    Pro/iOS 26.2 Simulator each pass 5/5; default/fixture static FFI archives
    separate the host symbol, scene ID and font bytes. Pass for these targets
    only. Screenshots show route/status but not text glyph pixels, hardware GPU
    output or post-compositor output.

Focused checks: text-layout 2/2, runtime 6/6, FFI source 1/1, fixture module
boundary 27/27, feature-scoped Clippy, both simulator builds and both 5/5
matrices passed. The C11.4/C11 checklist and official `72/213` remain
unchanged.
