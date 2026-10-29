# C11.3.3 leading named implicit lines · plan PR diff review 30

**Reviewed:** 2026-10-29
**Scope:** staged plan PR delta only; no implementation code is included.
**Decision:** no blocking documentation, oracle, fixture, or scope defect found. The implementation gates below remain mandatory.

| # | Adversarial review angle | Result |
|---:|---|---|
| 1 | Parent C11.3.3 and C11 IDs match the official ledger | PASS — plan links the existing C11 entry and leaves its checkbox untouched. |
| 2 | The plan implies public CSS Grid support | PASS — it explicitly keeps public support and both parent completion states incomplete. |
| 3 | The normative link is stable and section-specific | PASS — archived 2025-03-26 CRD §8.3 URL and related §7.5/§8.3.1 are named. |
| 4 | The plan quotes too much normative text | PASS — behavior is paraphrased and linked; the short example shorthand is only an identifier. |
| 5 | Pinned WPT context is mistaken for a WPT execution | PASS — revision is identified and the document says no mapped leading-line WPT case or runner pass exists. |
| 6 | Browser version evidence is overstated as a binary-pinned oracle | PASS — exact browser binary revision/hash absence is stated in the plan and JSON. |
| 7 | The working-tree source revision for the capture is explicit | PASS — reference binds to origin/main HEAD `d26a0cfd6d6383dbf9fffccef65a07f9647c320e`. |
| 8 | Fixture bytes can drift without detection | PASS — HTML SHA-256 is recorded in the reference JSON. |
| 9 | Inventory and screenshot can drift independently | PASS — inventory hash and screenshot hash are recorded with repository-relative paths. |
| 10 | Screenshot might be mistaken for Spinon output | PASS — it is labeled browser precomparison and the record says no implementation result is implied. |
| 11 | Seven scenes are missing or duplicated across files | PASS — HTML, inventory, reference, plan and capture list the same seven scene IDs. |
| 12 | Expected rectangles mix viewport and root-relative coordinates | PASS — JSON names rect order and coordinate reference; plan table is explicitly root-relative. |
| 13 | CSSOM track order is confused with the numeric expected vector | PASS — captured serialized columns and numeric vectors preserve physical left-to-right track order. |
| 14 | Positive named occurrence falls back on an explicit unnamed line | PASS — expectation accounts for implicit lines beyond the explicit end boundary. |
| 15 | Negative named occurrence falls on the trailing side | PASS — negative case expects two leading tracks and an item at the leading edge. |
| 16 | Backward span uses a trailing implicit line contrary to the oracle | PASS — §8.3 example and one/two-span cases require leading line generation. |
| 17 | Partial explicit name match is not represented | PASS — the partial-explicit scene requires one explicit match plus one leading implicit match. |
| 18 | Authored-line precedence lacks a control | PASS — the final scene expects an authored ghost match and no leading implicit track. |
| 19 | Existing forward span behavior could regress unnoticed | PASS — forward-span-two remains in the same fixed inventory and is a required compatibility check. |
| 20 | Repeated implicit track list order is ambiguous | PASS — `[15px, 7px]` and the captured leading/trailing lists establish physical sequence and reverse-from-explicit semantics. |
| 21 | Non-pixel track list members could leak into the profile | PASS — only non-empty fixed CSS-pixel lists are permitted; invalid members remain explicit negative tests. |
| 22 | The signed normalizer could admit TrackSizing profile behavior | PASS — plan requires retaining fixed item/track restrictions and auto-placement rejection. |
| 23 | General numeric implicit placement could be accidentally advertised | PASS — numeric-only placement remains out of scope and receives a rejection test. |
| 24 | A numeric endpoint needed by a named span is not clearly bounded | PASS — the narrow opposite-endpoint exception is stated separately from general numeric support. |
| 25 | Arithmetic or allocation limit moves after track allocation | PASS — checked arithmetic, reserve ordering and 10,000-track boundary are implementation gates. |
| 26 | Area aliases or existing named-line projection change | PASS — precedence is explicit and the existing 26-scene C11.3.3 regression matrix is mandatory. |
| 27 | Invalid combinations could publish partial frames | PASS — failure atomicity and absence of draw output are required negative controls. |
| 28 | Android/iOS evidence boundaries are inflated | PASS — AVD software renderer, simulator and pre-compositor limits are required in the result. |
| 29 | Existing unrelated untracked work is swept into this PR | PASS — the proposed delta is limited to the plan, fixture, precomparison and their review records; unrelated files remain unstaged. |
| 30 | Plan review is treated as implementation or merge evidence | PASS — plan-failure review is separate; implementation review, CI/check state and merge proof remain later gates. |

## Remaining implementation gate

The new fixture has a browser-derived baseline, not a Spinon pass. Before implementation merge, exercise the actual opt-in FFI/app route on both requested simulators, rerun the existing C11.3.3 matrix on the final code, review implementation failure modes, and verify the implementation PR's final checks and merge state.
