# C11.4 computed `line-break` PR diff review (30 perspectives)

Reviewed the complete staged candidate diff independently from the
implementation/runtime review. The review includes Rust, platform harnesses,
capture tooling, contract text and evidence files.

1. **Change scope:** The diff adds one fixed anonymous Grid text profile and
   does not modify general CSS parsing, HTML loading, or a public API. Pass.
2. **FFI fixture isolation:** The new source module is under the existing
   `c11_4/anonymous_text_items` fixture module and its parent route is already
   fixture-gated. Pass.
3. **Scene catalog integrity:** FFI and runtime catalogs contain the same five
   unique fixed scene IDs. Pass; source-generation and boundary tests check
   this.
4. **Input serialization:** The FFI source records the exact text, style owner,
   owner language, track keyword and CSS line-break value. Pass.
5. **Typography assumptions:** Noto Sans KR, 10px size/line-height, 400 weight,
   horizontal-tb/LTR, normal whitespace and word-break are fixed. Pass.
6. **Owner-language policy:** Runtime checks an exact owner `lang`, rejects
   XML language, and does not silently substitute inherited language. Pass.
7. **Typed value mapping:** `normal` and `strict` map to the existing
   `LineBreakMode` variants; existing `auto` scenes retain their previous
   Strict default. Pass.
8. **Paragraph locale:** The scene language is applied to the same anonymous
   paragraph as its line-break mode. Pass in runtime and text-layout evidence.
9. **Font payload scope:** The existing pinned font is registered only for
   Noto-backed allowlisted scenes. Release archive scan confirms the exact
   font bytes occur only in the fixture archive. Pass.
10. **Intrinsic text sizing:** Tests compare all four min/max inline/block
    metrics for Japanese normal/strict and English normal. Pass.
11. **Grid contribution selection:** Runtime checks both intrinsic column
    keywords, auto rows and rejects an opposite/fixed column. Pass.
12. **Revision ownership:** Runtime positive controls bind computed styles and
    trace input to the current HostDocument revision; a stale key is rejected.
    Pass.
13. **Failure controls:** Missing/mismatched owner language, XML language,
    wrong/unsupported line-break, wrong text, extra child, ancestor-only
    language and unknown scene are rejected. Pass.
14. **Android expectation adapter:** Existing direct-text expectations default
    to the previous Strict value; new scene IDs select Normal/Strict explicitly.
    Pass; Android build and five-scene AVD matrix are green.
15. **iOS expectation adapter:** Defaults and new scene mode selection match
    Android. Pass; iOS build and five-scene Simulator matrix are green.
16. **Installed artifact identity:** The captured APK and iOS executable hashes
    match their installed counterparts for the recorded runs. Pass.
17. **Trace completeness:** Capture checks every trace chunk, source revision,
    intrinsic input, line metrics, item frame and final Columns/Rows tracks.
    Pass.
18. **Capture lifecycle:** Android is force-stopped and iOS terminated in
    `finally`; only an iOS startup `status=-12` supersession gets one retry,
    with the original attempt log retained. Pass by code inspection; the final
    recorded run needed no retry.
19. **Evidence wording:** The docs distinguish route screenshots from glyph
    paint, software/Metal Simulator output from hardware GPU evidence, static
    archive scope from full APK/IPA exclusion, and this candidate from full C11.
    Pass.
20. **Repository hygiene and status:** Only the explicit candidate files were
    staged. Unrelated prior evidence remains unstaged; `spec/STATUS.md` and the
    roadmap checkbox/count are not promoted. Pass; official `72/213` remains.

The focused tests, Clippy and fixture boundary check passed after the final
source changes. No blocking defect was found in this staged diff. This review
does not claim CI, PR image rendering, merge state or completion of C11.4/C11.

21. **Fail-closed scene selection:** Unknown scene IDs and unlisted language,
    line-break, and track combinations do not fall through to a nearby valid
    profile. The runtime publishes a precise rejection and no layout frame.
    Pass.
22. **FFI decoding boundary:** Scene source construction preserves UTF-8 text
    and explicit property values; malformed or unsupported scene data cannot
    be normalized into the valid Japanese/English cases. Pass in the route
    tests and source review.
23. **Default compatibility:** The existing computed `auto` route continues
    through the previous Strict default, while only the five new IDs opt into
    explicit Normal or Strict. The Android and iOS adapters preserve that
    behavior. Pass.
24. **Property-owner identity:** `lang` and computed `line-break` are read
    from the direct text's exact Grid style owner; ancestor-only language and
    XML language do not accidentally satisfy the profile. Pass in negative
    runtime controls.
25. **Text-to-layout axis mapping:** Inline and block min/max metrics are
    checked independently, then compared with the column contribution and
    auto row result. A swapped-axis implementation would fail the fixed
    expected values. Pass.
26. **Oracle boundary:** The pinned Chrome 156 precomparison and its independent
    expected values precede the implementation result. Simulator results are
    compared to those fixed expectations; screenshots are not treated as a
    glyph-pixel oracle. Pass.
27. **Capture result attribution:** Each platform matrix records the simulator,
    installed binary identity, scene ID and complete trace. This prevents a
    successful result from a different build or scene being attributed to the
    candidate. Pass.
28. **Retry isolation:** iOS retries only the known startup supersession
    status, retains the failed attempt log, and requires the new attempt to
    pass the same full trace checks. Other failures remain failures. Pass by
    code review and recorded attempt data.
29. **Artifact claim scope:** The release-vs-fixture scan proves only static
    FFI archive symbol/string/font separation. The evidence explicitly avoids
    extrapolating that result to complete APK or IPA bundles. Pass.
30. **Completion and publication boundary:** Contract text describes this as
    one opt-in candidate; C11.4 and C11 remain unchecked, the roadmap is not
    promoted, and the official count stays 72/213 until broader criteria are
    met. Pass.

All 30 perspectives found no blocking defect in the candidate diff. The review
does not claim CI, PR image rendering, merge state or completion of C11.4/C11.
