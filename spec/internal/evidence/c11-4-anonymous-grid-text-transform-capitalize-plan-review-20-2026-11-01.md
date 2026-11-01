# `text-transform:capitalize` plan failure review: 20 perspectives

The review checks whether the bounded C11.4 fixture plan can be implemented without overclaiming CSS semantics or weakening the existing anonymous-text path. Each item records a failure condition and the guard retained in the plan.

1. **Standards semantics:** `capitalize` could incorrectly lowercase later uppercase letters. The plan only uppercases an initial lowercase ASCII letter and preserves all other letters; the mixed-case oracle checks `iIII ii` → `IIII Ii`.
2. **Word segmentation:** UAs can disagree on word boundaries. The profile explicitly fixes U+0020 as the only separator and disclaims general word segmentation.
3. **Punctuation boundary:** Punctuation could introduce unmodeled word-start rules. The profile rejects punctuation before layout.
4. **Locale-sensitive casing:** Locale-specific titlecase could be guessed from a scene ID. The plan requires direct `lang="en"` and limits the mapping to ASCII.
5. **Unicode expansion:** Unicode uppercase can change character count and invalidate source offsets. Non-ASCII input is rejected and the mapping is one-to-one.
6. **Wrong source mutation:** Transforming HostDocument text would corrupt DOM-visible content. The plan changes only the normalized layout buffer and asserts source remains unchanged.
7. **Source-range drift:** Rewriting UTF-16 mappings while changing case could misattribute text. The plan preserves character count and source mapping and adds mapping assertions.
8. **Repeated-space ordering:** Capitalizing after collapse could lose word boundaries, while collapsing first could alter source mapping. The profile transforms the pre-collapse normalized buffer and tests repeated spaces separately.
9. **Other whitespace:** Tabs, newlines, or non-breaking spaces could become accidental word separators. Only U+0020 is admitted; other whitespace fails closed.
10. **Leading/trailing space ambiguity:** Unspecified edge whitespace can hide incorrect word-start state. The fixed matrix does not rely on leading/trailing whitespace; any added case must follow the same U+0020 rule and get an explicit oracle.
11. **Computed-style spoofing:** A scene marker alone could select capitalization while style says `none`. Runtime validation must match exact computed `capitalize` and reject mismatches.
12. **Wrong owner:** A matching scene ID on another node could receive the profile. Validate exact Grid owner and direct anonymous text relationship.
13. **Stale snapshot:** A style snapshot from a prior document/layout revision could be reused. Reuse the existing generation/document/render-tree revision gate and add stale-revision negatives.
14. **Language inheritance:** Ancestor language could silently broaden the profile. Require exact direct `lang="en"`; reject missing, wrong, or inherited language.
15. **Incorrect min-content:** Capitalized initials have different advances and can change the longest word. Compare the min-content scene against independent Noto Sans KR glyph advances.
16. **Incorrect max-content:** Spaces and preserved uppercase characters affect the unbroken width. Compare all three max-content scenes and use the stated 1/64px quantization tolerance.
17. **Fixture overreach:** A code path might enable the mode for all Grid text. Restrict it to the existing opt-in fixture adapter and test default archive exclusion.
18. **FFI leakage:** Test-only scene IDs or payload could enter the default static archive. Keep and run default/opt-in symbol and marker boundary controls.
19. **Oracle overclaim:** Chrome layout values or Range boxes could be mistaken for Spinon behavior or glyph ink. Label them as CSSOM track observations and treat Range rectangles as ancillary only.
20. **Completion overclaim:** A passing six-scene slice could mark C11.4 complete. Contract, status, plan, and roadmap must retain C11/C11.4 unchecked and official completion at 72/213.

**Disposition:** No plan blocker remains within the fixed ASCII/U+0020 profile. Implementation must stop and revise the plan if it requires punctuation, Unicode, inherited language, or a general word-boundary rule.
