# `text-transform:capitalize` implementation/runtime review: 20 perspectives

This review evaluates the candidate implementation and its runtime boundary separately from the 20-view plan review. Each perspective checks a concrete failure mode in the code or captured app path.

1. **Mode propagation:** Verify `Capitalize` survives the text-layout model, `ParagraphStyle`, layout result, trace, and computed-style adapter. The enum and runtime assertions keep the computed mode observable end to end.
2. **First-letter behavior:** Verify only a lowercase first ASCII letter is uppercased; later characters retain their authored case. The `iIII ii` scene must shape to `IIII Ii`.
3. **Word boundary:** Verify only U+0020 resets the word-start state. Tabs/newlines and all other non-letter characters are rejected before mapping.
4. **ASCII restriction:** Verify digit, punctuation, and non-ASCII input fail with `UnsupportedTextTransformProfile` instead of silently applying a partial mapping.
5. **Existing transform regression:** The original six `none|uppercase|lowercase` scenes and their values remain in the matrix alongside six new source-specific controls.
6. **Original DOM source:** Verify `HostDocument` Text remains unchanged for all six added scenes; the runtime assertions compare the source against the scene input.
7. **UTF-16 source ranges:** Verify each source character retains its original one-unit UTF-16 span after layout-only case mapping.
8. **Collapsed whitespace mapping:** Repeated U+0020 characters collapse for shaping while all original UTF-16 ranges remain and map to the collapsed paragraph span.
9. **Inline formatting:** Verify both inline style maps and source inline ancestors still reject non-`none` transforms, including `Capitalize`.
10. **Exact computed value:** Verify the scene ID cannot spoof the value. Computed `capitalize` is accepted only for its fixed scene IDs; wrong values fail before layout publication.
11. **Language owner:** Verify direct `lang="en"` is required on the Grid owner; missing, inherited, wrong-language, and `xml:lang` paths remain rejected.
12. **Revision identity:** Verify style generation, document revision, and render-tree revision are checked against the current HostDocument snapshot before the scene is laid out.
13. **Anonymous-item identity:** Verify one exact Grid owner and one direct Text item are used; FFI scene construction does not add wrappers or extra text nodes.
14. **Intrinsic sizing:** Verify the min-content and max-content values for the base, mixed-case, and collapsed-space input against independent Noto Sans KR advances and the 1/64px tolerance.
15. **Automatic rows:** Verify min-content scenes wrap to two 10px lines and max-content scenes remain one line, producing 20px and 10px rows respectively.
16. **Fixture isolation:** Verify the 12 scene IDs stay behind the existing opt-in runtime fixture and FFI feature. The default-versus-opt-in archive check remains a required final gate.
17. **Android adapter parity:** Verify Android's scene ID, source text, transformed text, line advances, and track metrics match the shared expected matrix; API 37 ARM64 AVD captured 6/6 scenes.
18. **iOS adapter parity:** Verify iOS uses the same six new scene inputs and expected geometry; iPhone 17 Pro/iOS 26.2 Simulator captured 6/6 on the fresh-app run.
19. **Failure cohort retention:** The first iOS attempt returned `status=-12` because a newer screen request superseded the pending evaluation. Its log is retained in the `ios-simulator-pass01` attempt directory and excluded from the 6/6 pass matrix; the later pass is not treated as a repair for that race.
20. **Claim boundary:** The simulator screens show `layout=ready` with glyph paint disabled. They prove layout/runtime values only, not glyph pixels, hardware GPU behavior, general CSS admission, WPT conformance, or full C11.4/C11 completion.

**Disposition:** No implementation blocker was found in the fixed ASCII/U+0020 profile. The superseded iOS attempt remains a separately recorded runtime-harness failure. Final simulator receipts must be captured after the candidate source is committed so `sourceHead` binds to the code under review.
