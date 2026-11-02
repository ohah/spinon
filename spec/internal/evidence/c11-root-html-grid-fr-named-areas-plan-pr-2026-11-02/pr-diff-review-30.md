# Plan PR diff adversarial review · 30 distinct viewpoints

Review target: plan-only diff for `c11-root-html-grid-fr-named-areas`. These checks challenge the changed plan and precomparison package; implementation/runtime review is a separate later gate.

| # | Independent diff viewpoint | Result |
|---:|---|---|
| 1 | Does the parent C11 plan distinguish a proposal from merged behavior? | PASS — the inserted paragraph labels it plan-only and preserves current official state. |
| 2 | Does the plan link resolve from the parent plan's directory? | PASS — the relative link targets `plan/c11-root-html-grid-fr-named-areas.md`. |
| 3 | Is the product entry point named precisely? | PASS — scope says `RuntimeSession::new_runtime_gpu_with_root_html()` parsed HTML route. |
| 4 | Is the existing failure reproduced by a current test? | PASS — evidence points to the named-areas-with-fraction rejection case. |
| 5 | Does the plan identify the exact restricting validator? | PASS — `validate_root_html_named_area_template()` and its px-only pair are named. |
| 6 | Does the proposal accidentally broaden all Root HTML grids? | PASS — only the current 2×2 map and direct two-axis `1fr 2fr` composition is proposed. |
| 7 | Can root dimensions be indefinite while tests pass? | PASS — fixed 120×70 definite axes are required and axis validators are retained. |
| 8 | Can min-size alter the flex fraction? | PASS — min-size override is a rejection control under the existing gate. |
| 9 | Can max-size alter the flex fraction? | PASS — max-size override is a rejection control under the existing gate. |
| 10 | Can aspect ratio affect the fraction basis unnoticed? | PASS — aspect ratio remains disallowed on fraction-bearing axes. |
| 11 | Do the independent expected rows rely on rounded browser values? | PASS — exact rational 70/3 and 140/3 are recorded separately. |
| 12 | Is the Chrome quantization tolerance chosen after seeing the delta? | PASS — measured maximum delta is documented and threshold is fixed at 0.02 CSS px. |
| 13 | Are the screenshot pixels treated as the geometry oracle? | PASS — evidence labels the screenshot visual-only and prioritizes computed tracks/DOMRects. |
| 14 | Does screenshot viewport metadata match captured browser metadata? | PASS — viewport 1280×633 and DPR 1 are in both JSON records. |
| 15 | Does the HTML fixture keep area names and the map explicit? | PASS — source fixes the 2×2 `header/side/content` matrix. |
| 16 | Is source order arranged to catch positional item matching? | PASS — content, header, side order differs from visual map order. |
| 17 | Is successful owner identity allowed to depend on class or NodeId? | PASS — acceptance calls for selector renaming and source-order controls. |
| 18 | Are `grid-area` semantics preserved instead of text parsing? | PASS — typed computed area/placement flow is explicitly required. |
| 19 | Is the fixed-px area behavior a regression baseline? | PASS — old exact input and expected behavior must remain unchanged. |
| 20 | Could percentage tracks silently enter this new combination? | PASS — percentage + area is explicitly excluded and rejected. |
| 21 | Could `minmax()` or intrinsic sizing sneak in via computed tracks? | PASS — direct fraction subset and negative controls are explicit. |
| 22 | Could named lines, repeat, or extra tracks expand scope? | PASS — they are listed as exclusions and negative cases. |
| 23 | Could nonzero gap change the free-space calculation? | PASS — new oracle uses zero gap; existing gap rules are retained. |
| 24 | Could item intrinsic contribution determine a different track size? | PASS — items remain empty block items and text/image scope is excluded. |
| 25 | Are source, expected JSON, observed JSON, and image all hash-linked? | PASS — hashes file covers each artifact and expected JSON embeds source hash. |
| 26 | Does browser evidence identify which browser/session produced it? | PASS — user agent, session name and close-after-capture are recorded. |
| 27 | Does the plan claim WPT or general CSS conformance? | PASS — WPT and general admission are excluded. |
| 28 | Does Android AVD evidence get mistaken for a physical device or GPU? | PASS — plan requests AVD only and preserves simulator limitations. |
| 29 | Does simulator draw/present get mistaken for exact surface pixels? | PASS — pixel equality is conditional on a separately run readback. |
| 30 | Does the plan PR update official completion counts before implementation? | PASS — no STATUS checkbox or aggregate changes; `72/213` is expressly preserved. |

Result: 30/30 PASS. This is a plan-diff review; it does not substitute for the required implementation/runtime 20-point review or implementation PR 30-point review.
