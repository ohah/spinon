# C11.3.3 leading named implicit lines · plan failure review 20

**Reviewed:** 2026-10-29
**Target:** [plan](../../../plan/c11-3-3-leading-named-implicit-lines.md), [inventory](../../../tests/fixtures/css/c11/grid-leading-named-implicit-lines-c1133-inventory.json), [Chrome observation](./c11-3-3-leading-named-implicit-lines-precomparison-2026-10-29/README.md), and CSS Grid 2025 CRD §8.3.
**Decision:** all 20 plan risks have an explicit acceptance boundary or verification gate. This review approves a plan only; it is not implementation evidence.

| # | Failure perspective | Review result and implementation gate |
|---:|---|---|
| 1 | Positive named occurrence counts the explicit end boundary as a named implicit line | PASS — only authored matches count; positive fallback starts after explicit boundary N. Test occurrences 3 and 4. |
| 2 | Negative occurrence is anchored at the wrong side of the explicit grid | PASS — negative fallback counts startward from the explicit end and emits a negative CSS line. Assert leading extent and reversed tracks. |
| 3 | First generated line is off by one because a line is confused with a track | PASS — expected CSSOM tracks and rects pin boundary/track distinction for one and two generated lines. |
| 4 | Backward span consumes a trailing implicit line created by its numeric opposite endpoint | PASS — CRD example and span-one/span-two cases require a leading line; trailing line 3 is excluded. |
| 5 | Partial authored matches are counted twice or discarded | PASS — partial-explicit case must use its authored ghost line and synthesize exactly one leading line. |
| 6 | Existing authored names lose precedence to generated implicit names | PASS — backward-span-uses-authored-name is a no-leading-generation control. Preserve authored match search before fallback. |
| 7 | Template-area aliases are shadowed by named occurrence fallback | PASS — preserve current area alias path and rerun all existing C11.3.3 area scenes. |
| 8 | Duplicate names at one explicit boundary count as multiple occurrences | PASS — count a boundary once per line-name occurrence semantics; add duplicate-name regression before implementation merge. |
| 9 | The start/end property role changes positive or negative occurrence counting | PASS — test row/column symmetry and both endpoints; resolver uses the signed occurrence, not the property name, to choose direction. |
| 10 | Forward named span changes while adding backward support | PASS — keep the existing forward case and add exact row/column regression against the 26-scene suite. |
| 11 | A bare numeric out-of-range placement gains access to the opt-in route | PASS — allow out-of-range numeric anchor only paired with a supported named span; add numeric-only expected rejection. |
| 12 | CSS Grid conflict handling swaps or drops endpoints after fallback incorrectly | PASS — retain conflict normalization order and test equal lines, reversed definite lines and two-span conflict. |
| 13 | Negative line conversion differs between projection and signed layout coordinates | PASS — test CSS negative line, normalized boundary, leading extent and final Taffy item frame as one chain. |
| 14 | Leading grid-auto track-list expansion uses forward order instead of reverse order | PASS — `[15px, 7px]` must produce physical CSSOM order `[15px, 7px]` for two leading tracks, with `7px` nearest the explicit grid; verify one/two leading tracks. |
| 15 | Track count cap is checked after allocation or integer arithmetic wraps | PASS — checked arithmetic and 10,000-track cap must reject before reserve; boundary and over-limit tests required. |
| 16 | Auto placement becomes accepted through the signed TrackSizing normalizer | PASS — preserve TemplateGridImplicitTracks definite-placement rejection before delegation; auto item remains an expected rejection. |
| 17 | Track-list support admits intrinsic, percentage, flex or empty members | PASS — only non-empty fixed CSS-pixel lists pass profile validation; test each unsupported member and empty list. |
| 18 | Row/column mapping or writing direction changes the leading side | PASS — validate both axes in LTR/horizontal-tb; keep RTL and alternate writing modes outside this slice. |
| 19 | Failure in one item leaves partial frames, tracks or draw submissions | PASS — resolver/normalizer failure must be atomic; check no published partial layout and no draw output for negative controls. |
| 20 | Browser evidence or simulator result is overstated as a WPT or product-support pass | PASS — evidence labels this as Agent Browser precomparison; implementation requires opt-in simulator runs; C11.3.3, C11 and 72/213 stay incomplete. |

## Open checks before implementation

- Agent Browser reports HeadlessChrome 156 but did not expose its binary hash; the plan records this limitation and treats the 2025 CRD as normative.
- The fixture is a specification-derived test set, not a mapped WPT assertion. Do not call it a WPT pass.
- The implementation must show that signed track normalization preserves existing C11.3.3 scenes and does not accept automatic placement or general numeric implicit placement.
