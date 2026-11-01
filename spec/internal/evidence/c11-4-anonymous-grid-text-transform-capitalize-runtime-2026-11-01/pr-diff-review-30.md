# `text-transform:capitalize` PR diff review: 30 failure perspectives

Scope: PR [#986](https://github.com/ohah/spinon/pull/986), source commit `9b124aeab080711028f1198686b8818f26aed122`, documentation/evidence head `9ff2143820b5998975da1aa5b909806cda44166`. This review covers the code, fixture routing, platform adapters, evidence, and status/roadmap diff. It is separate from the plan and implementation/runtime reviews.

1. **CSS value semantics:** `iIII ii` must become `IIII Ii`, preserving later uppercase letters. The fixed oracle and text-layout assertions agree.
2. **Word boundary:** A separator must reset capitalization exactly at U+0020. Other ASCII punctuation and whitespace are rejected by the candidate normalizer.
3. **Input alphabet:** Digits, punctuation, non-ASCII, tabs, and newlines must not be partially transformed. Text-layout negative cases exercise representative members of each rejected class.
4. **First character casing:** An uppercase first character must remain unchanged; the implementation only calls `to_ascii_uppercase` at a word start and does not lowercase later letters.
5. **Language binding:** `lang="en"` must be present directly on the fixture Grid owner. Runtime owner validation rejects missing, inherited, wrong, and alternate language forms; the profile does not claim locale-sensitive behavior.
6. **Source ownership:** Transformation must not mutate HostDocument Text. Runtime cases compare the snapshot's direct text against the authored scene string.
7. **UTF-16 mapping:** ASCII case mapping must preserve one source unit per character. Text-layout tests assert original source ranges for every character.
8. **Collapsed spaces:** Repeated source spaces must still identify the following word start before shaping collapse. The `ii   ii` scene expects `Ii Ii` and keeps each original range mapped to the collapsed span.
9. **Inline content:** A nested inline ancestor or inline run style must not enter this direct-text-only path. Existing guards remain and the implementation review records the rejection boundary.
10. **Computed-style spoofing:** A scene ID must not enable a different computed value. The runtime maps each allowlisted ID to its exact expected `text-transform` serialization and rejects mismatches.
11. **Scene/input pairing:** Android, iOS, FFI, and Runtime use the same six new scene IDs and authored inputs; the precomparison JSON checks scene IDs and inputs independently.
12. **Anonymous item shape:** The FFI fixture creates one element owner and one direct Text node, with no wrapper that could change anonymous-item construction.
13. **Revision freshness:** The fixture reuses the existing generation, document revision, and render-tree revision checks before layout publication; the plan and runtime review preserve this gate.
14. **Intrinsic min-content:** The capitalized `Iiii` word controls the expected min-content width. Chrome CSSOM and independent glyph-advance sums are within the recorded 0.03125 CSS px tolerance.
15. **Intrinsic max-content:** Space plus both transformed words are included in the expected unbroken width. The independent advance equation and Chrome observation are stored separately.
16. **Whitespace collapse sizing:** Triple U+0020 becomes one shaping separator while authored source text remains `ii   ii`; the 13.59375px expected track is consistent with the collapsed `Ii Ii` advance.
17. **Line and row geometry:** Min-content cases wrap to two 10px lines and max-content cases stay on one 10px line; runtime receipts record row height separately from track width.
18. **Tolerance discipline:** The oracle allows 1/64px quantization (0.03125 CSS px), and the captured platform values differ from Chrome only within that bound.
19. **Existing transform regression:** The original `none`, `uppercase`, and `lowercase` scene IDs remain in the 12-scene Runtime/FFI matrix with their preexisting authored text and expected values.
20. **Feature isolation:** The new scene route remains in the opt-in C04 runtime GPU fixture path. Default and opt-in static archive raw-marker scans report absence/presence respectively; this is not presented as whole APK/IPA exclusion evidence.
21. **Android adapter parity:** Android's mode, source text, transformed layout text, and expected advances are selected per exact scene ID. The commit-bound API 37 ARM64 AVD matrix records 6/6 scenes.
22. **iOS adapter parity:** iOS has matching scene-to-mode/source/layout mappings and expected values. The commit-bound iPhone 17 Pro/iOS 26.2 Simulator matrix records 6/6 scenes with matching build/install executable hashes.
23. **Artifact identity:** Simulator receipts record `sourceHead`; installed APK and iOS executable hashes match their captured build artifacts. This binds the evidence to source commit `9b124ae`, while PR head `9ff2143` adds documentation/evidence only.
24. **Failed-run accounting:** iOS `status=-12` pass01 and pass03 receipts/logs remain retained and excluded from the passing matrix. Pass04 and the supplemental 15/15 startup comparison are not described as repairs or root-cause analysis.
25. **Screenshot claim:** The PR body embeds Chrome, Android, and iOS images. The two app screenshots show `layout=ready` with glyph paint disabled, so they do not claim text-pixel or compositor-surface verification.
26. **Oracle provenance:** Chrome 156 is identified as the expected-behavior oracle, not as evidence that Spinon itself produced those values. The local fixture, observation JSON, expected JSON, and font hash are retained.
27. **Contract versioning:** Internal contract 0059 remains at `0.1.0`; the added profile is explicitly bounded and does not silently revise a versioned contract.
28. **Official status accounting:** C11 and C11.4 remain unchecked, and `spec/STATUS.md` retains official completion at 72/213. The roadmap note repeats the same boundary.
29. **Verification reporting:** Focused tests and both simulator builds are recorded as successful; the empty GitHub check rollup is not called CI success. Runtime Clippy's existing warning and the unrelated repository docs-build errors are disclosed.
30. **Scope closure:** General CSS admission, Unicode/titlecase expansion, UAX #29, WPT, glyph pixels, hardware GPU, and full C11.4/C11 remain excluded. The current evidence does not imply any of those are complete.

**Disposition:** No blocking mismatch was found within the fixed ASCII/U+0020 fixture profile. The intermittent iOS supersede failure remains an unresolved, retained limitation; the clean same-commit runs do not establish a fix. Merge evidence must keep the failure cohort and empty check rollup visible.
