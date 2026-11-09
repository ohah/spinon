# PR #1311 diff adversarial review (30 points)

Reviewed the complete PR #1311 diff on 2026-11-09: implementation, focused tests, status/plan changes, simulator evidence, and embedded PR screenshot. These checks are separate from the 20 implementation/runtime review points.

1. **Style admission could enable the profile for any six-track list.** The predicate requires the established six-member span-track candidate and exact multiplicities; other track sizes fail closed. Pass.
2. **Two fit members could be accepted with a malformed tail multiset.** The new branch permits only two min-content, one max-content, and one auto alongside two fits. Pass.
3. **A valid multiset could bypass the four authored tail-order boundary.** Both projection and Layout require one of the four reviewed tail orders; profile tests enumerate these orders. Pass.
4. **Three fits could accidentally enter the new path.** The profile predicate returns false outside fit counts one or two; the RuntimeSession mutation test rejects three and checks atomic recovery. Pass.
5. **Fit caps could admit negative, non-finite, calc, or indirect values.** Each member still passes the existing finite nonnegative direct-px/percentage cap validator. No cap parser was broadened. Pass.
6. **Fit positions could be duplicated, unordered, or outside the list.** Positions come from distinct authored indices and are checked as an ascending pair in 1–6. The full pair matrix and unit profile test cover all 15 combinations. Pass.
7. **The two-fit profile could create more or fewer implicit rows than sized.** Its item placements are restricted to the two fixed six-row span topologies; matrix observations assert one explicit plus six used rows. Pass.
8. **The new helper could change prior one-fit position support.** Single-fit 1–3, 4–6, and existing 960-scene row-span matrices passed after the change. Pass.
9. **Auto-placement could accidentally inherit span-only behavior.** The route still distinguishes the auto-placement candidate and explicit row-span topology; no two-fit auto-placement profile is admitted. Existing auto-placement 1,440-scene regression passed before this patch. Pass by code path and prior regression.
10. **A second Grid or hidden owner could be normalized under the wrong trace.** Admission retains the single visible owner requirement and trace lookup requires exactly one trace for its node. Pass.
11. **An item could create an implicit column while exercising row sizing.** Column line placement remains fixed to 1/2 and frame comparisons cover all items. Pass.
12. **The `min-height:auto` admission could affect the previous one-fit grammar.** It is enabled only by the recognized two-fit span list in both Style and Layout; previous single-fit matrices pass. Pass.
13. **The cap could incorrectly clamp the min-content contribution.** A separate Chrome 157 control puts each min-content floor above its 0.5px cap; RuntimeSession verifies rows, item frames, payload frames and text descriptors. Pass.
14. **Text grammar could broaden while adding the min-content control.** Existing direct-text payload collection and restrictions remain; only the computed minimum-height condition is extended for this profile. Pass.
15. **Track sizing traces could be emitted for unrelated Root HTML grids.** The projection flag adds only the recognized six-member span candidate to the existing trace condition. Pass.
16. **A track's typed cap could move to a neighboring row during projection.** The full RuntimeSession matrix compares computed typed tracks in authored order and each cap stays attached to its indexed position. Pass.
17. **An unrelated or stale trace could satisfy the new normalizer.** Trace ownership, uniqueness, used track count, negative implicit rows, and collapsed rows are checked before normalization. Pass.
18. **A missing, duplicate, or out-of-order sizing stage could be accepted.** The verifier requires a unique latest row pass and unique intrinsic-complete/maximize stages followed by the final stretch stage. Pass.
19. **Gutters or internal indices could corrupt row attribution.** The verifier strips gutters and validates odd sequential internal indices, finite base sizes, and expected count. Pass.
20. **The serialized used row list could disagree with stage values.** Each parsed used-row value is compared with final trace base sizes before any frame normalization. Pass.
21. **Auto alignment could resize fit/min/max-content rows.** Pre- and post-alignment stages require non-auto deltas to be zero; the matrix checks start/stretch per track and both topologies. Pass.
22. **A stretch stage with no auto row could fail an otherwise unchanged prefix.** The updated rule accepts it only when every row remains equal; a dedicated focused test passed. Pass.
23. **Fractional rounding could use final rows instead of pre-alignment rows.** The normalizer distributes intrinsic growth from intrinsic and maximize stages, then applies auto stretch and checks final stage sizes. Pass.
24. **The rounding mode could change unrelated one-fit or intrinsic rows.** `Up` is selected only for exactly two fit members; all other profiles retain nearest rounding. Single-fit regression matrices pass. Pass.
25. **A fit cap could be exceeded by rounding remainder assignment.** Growth potentials include the row's fit cap and saturated fit rows cannot receive a residual unit. The cap-floor control and 3,840 scenes pass. Pass.
26. **Equal growth ties could depend on map or traversal order.** Tie-breaking is deterministic by growth potential, auto status, and row index. The fractional-remainder tests passed. Pass.
27. **Source-order changes could reorder frames.** Forward/reverse topology observations are compared per item across the 3,840 scenes. Pass.
28. **The scene matrix could omit an axis or combination.** Test assertions cover 480 lists, 15 position pairs, eight cap assignments, four tails, two alignments, and four topology/order values. Pass.
29. **Failure handling could publish partial render output or poison a session.** The three-fit rejection publishes no completed layout and the same session recovers to identical frames with valid input. Pass.
30. **PR text could overstate native verification or completion.** The PR embeds the actual iOS screenshot and labels it visual smoke; Agent Browser confirmed the 1206×2622 image loads. Android 37.2.12, 37.0, and updated 37.2.13 AVD attempts remain blocked before app launch, and both PR/status explicitly leave Android, WPT, pixel parity, and C11.4 completion open. Pass with Android runtime gate outstanding.

## Review result

No code or documentation defect was found in the reviewed diff. PR #1311 remains draft; Android app execution is still required before merge/completion.
