# C11.4 visibility-hidden border overflow · roadmap sync review (30)

## Scope

This review covers one stale paragraph in `spec/public/roadmap.html`. It previously described the `visibility:hidden` border-overflow case as Chrome-only and said Spinon implementation and simulator validation were still pending. PR #747 merged that exact fixed-scene implementation. The corrected paragraph records the merged evidence without changing the official C11.4 completion state or `72/213` count.

## Independent checks

| # | Failure perspective | Check and result |
|---:|---|---|
| 1 | Wrong work item | The title still names the `visibility:hidden` border-overflow slice, matching the focused plan. PASS. |
| 2 | Wrong PR association | The implementation is linked to PR #747, confirmed merged. PASS. |
| 3 | Wrong merge time | The recorded timestamp is `2026-10-28T08:42:37Z`, matching GitHub. PASS. |
| 4 | Wrong merge revision | The row records merge SHA `cb3c9a50b9a10d1d441cf15a584bacefcd91630d`. PASS. |
| 5 | Stale pending claim | The paragraph no longer says implementation and simulator verification are pending. PASS. |
| 6 | Chrome oracle promoted to implementation | Chrome 156 is explicitly described as the pre-implementation baseline. PASS. |
| 7 | Columns root dimensions | The row distinguishes the `100×120px` Columns root through linked evidence; it does not invent new dimensions. PASS. |
| 8 | Rows root dimensions | The row distinguishes the `120×100px` Rows root through linked evidence. PASS. |
| 9 | Border/content confusion | The record refers to a sampled border point and preserves the root-border/content-box distinction in the linked run evidence. PASS. |
| 10 | Wrong target tracks | The four new scenes use `[150,40,0]px`; the text matches the run record. PASS. |
| 11 | Wrong remaining space | The negative remaining content-axis space is `-114px`; the text matches the run record. PASS. |
| 12 | Frame claim exceeds evidence | The paragraph makes no new frame claim; linked evidence holds the per-scene frames. PASS. |
| 13 | Wrong scene total | The recorded result is 14 existing regressions plus four new scenes, 18 per platform. PASS. |
| 14 | Android platform inflation | Android is labeled API 37 ARM64 16 KB AVD, not a physical device. PASS. |
| 15 | iOS platform inflation | iPhone 17 Pro / iOS 26.2 is labeled Simulator. PASS. |
| 16 | PixelCopy evidence overstated | The linked README identifies Android samples as PixelCopy and limits them to selected coordinates. PASS. |
| 17 | Metal evidence overstated | The linked README identifies iOS samples as drawable readback before compositor submission. PASS. |
| 18 | Hidden sample color reversed | The outside-root hidden sample is recorded as blue. PASS. |
| 19 | Visible-control sample color reversed | The outside-root visible control is recorded as purple. PASS. |
| 20 | Border sample claim unsupported | The linked sample table records the gold border sample. PASS. |
| 21 | Point sample generalized | The new text calls these limited samples and does not imply a full-frame comparison. PASS. |
| 22 | Full-surface parity implied | The paragraph explicitly says full-surface parity was not verified. PASS. |
| 23 | General CSS support implied | The paragraph explicitly leaves general CSS admission unverified. PASS. |
| 24 | Clipping or scrolling implied | Clipping and scrolling are explicitly outside this evidence. PASS. |
| 25 | Physical-device coverage implied | Real devices and hardware GPU are explicitly unverified. PASS. |
| 26 | Android hardware path implied | The row identifies an AVD and makes no hardware-GPU claim. PASS. |
| 27 | iOS scanout implied | Pixels after iOS compositor submission are explicitly unverified. PASS. |
| 28 | Official completion count changed | C11.4 remains incomplete and `72/213` is unchanged. PASS. |
| 29 | Broken local evidence path | Plan, runtime README, 18-scene matrix, and Chrome baseline links resolve to the existing files/routes. PASS. |
| 30 | CI or merge policy invented | The row reports the merge revision and runtime evidence but makes no CI-success claim. PASS. |

## Result

All 30 distinct documentation failure perspectives pass. No product code, API, fixture, simulator artifact, or completion checkbox changed. The full document link graph was not revalidated by this review.
