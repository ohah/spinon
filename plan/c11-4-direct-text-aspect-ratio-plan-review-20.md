# C11.4 direct-text aspect-ratio plan failure review — 20 items

**Review target:** `plan/c11-4-direct-text-aspect-ratio.md` and its pinned Chrome 156 precomparison, before implementation.

| # | Failure angle | Review result |
| --- | --- | --- |
| 1 | Applying image/replaced transferred-size rules to direct text | PASS — plan explicitly avoids Grid §6.6 replaced-item shortcut and scopes this to ratio-aware Grid item sizing. |
| 2 | Using Chrome output as the normative expected value | PASS — Ahem dimensions and ratio math are separately calculated; browser data is labeled observation only. |
| 3 | Ratio multiplication and division reversed | PASS — 40px inline × 1/2 = 20px block; 40px block × 2 = 80px inline. |
| 4 | Font not actually loaded in the browser comparison | PASS — `document.fonts.load("10px Ahem")` completed and the face reports `loaded`. |
| 5 | Browser capture includes nonlocal root offsets | PASS — raw capture keeps document positions; the acceptance table compares root-local coordinates. |
| 6 | No-ratio controls accidentally include the ratio declaration | PASS — both control scenes have computed `aspect-ratio:auto`; geometry is recorded separately. |
| 7 | Row contribution is tested only by final item geometry | PASS — acceptance requires both Rows trace and root/item frame to match. |
| 8 | Column transfer is inferred from row behavior | PASS — separate definite-40px-row scene checks the 80px inline contribution and column trace. |
| 9 | Grid self-alignment changes from the fixed input | PASS — fixture pins normal `align-items` and `justify-items`, matching the restricted profile. |
| 10 | Root sizing hides track contributions | PASS — the root is max-content sized and the two max-content columns are asserted directly. |
| 11 | CSS precomparison differs from intended runtime structure | PASS — both use two direct block text items, one `AAAA` ratio item, one short control item, and no descendant content. |
| 12 | A DOM ID or fixture ID becomes the acceptance key | PASS — collector acceptance must inspect HostDocument structure and computed style values. |
| 13 | Arbitrary ratios leak into the profile | PASS — only computed `2.0` on one four-character item is positive; all other ratios fail closed. |
| 14 | Other CSS dimensions silently interact with the ratio | PASS — width/height/min constraints, max constraints, box edges, and margin are restricted. |
| 15 | Ahem control relies on glyph raster output | PASS — expected geometry uses Ahem advances and line-height; pixel gate samples boxes only. |
| 16 | Unsupported rich text or replaced content is partly collected | PASS — descendant, anonymous, image, and >2 item structures remain rejected. |
| 17 | Existing no-ratio direct-text and one-item fallback regressions | PASS — both are explicit unchanged regression gates. |
| 18 | Simulator evidence claims product support | PASS — Android/iOS routes are fixture-test-only; product artifact must exclude fixture symbols. |
| 19 | Simulator screen evidence overstates capture layer | PASS — PixelCopy and Metal drawable evidence must name their actual boundary; no compositor-after or hardware claim. |
| 20 | A limited fixture increments official completion or contract version | PASS — C11.4/C11 stay unchecked, `72/213` remains, contract numeric version remains `0.1.0`. |

**Result:** 20/20 plan failure angles pass. This review authorizes moving to implementation only after the plan change is merged; it does not validate code or runtime behavior.
