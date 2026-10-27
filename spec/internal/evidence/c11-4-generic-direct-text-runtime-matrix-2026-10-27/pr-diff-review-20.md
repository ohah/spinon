# C11.4 direct-text implementation PR diff review · 20 perspectives

Reviewed the complete proposed implementation diff independently from the implementation/runtime checklist. This review covers the Rust fixture and diagnostic changes, both native routes and matchers, the simulator runner, and submitted evidence. No unresolved source or evidence-integrity issue remains.

| # | Diff failure perspective | Review result |
| ---: | --- | --- |
| 1 | Product CSS admission or public APIs are added incidentally. | The diff changes only opt-in fixture sources, runtime fixture diagnostics, and app fixture routing. It adds no product CSS admission rule or public API. |
| 2 | Fixture code is linked by default. | The route allowlists remain behind the runtime GPU fixture-test gate. Default versus opt-in Rust static archive comparison excludes/includes the route string as intended. |
| 3 | One platform omits or renames a route. | Android and iOS use the same six canonical route IDs. Both simulator matrix tables show all six IDs passing. |
| 4 | Rust source generation changes the existing normal control. | The existing generic-root-normal source retains its root, items, styles, source text, and normal wrapping expectations. Its Android/iOS regression control passed. |
| 5 | Dynamic source formatting omits an argument or changes the generated JS object. | The six-entry unit test checks each route/root pair, element and text-node counts, sizing style, raw whitespace, break-all class and nowrap styles. Device execution covers all generated programs. |
| 6 | A source-level test treats the scene table as a generic collector special case. | The collector regression test remains based on six computed-style/intrinsic/C15 geometries; it does not add route-ID-specific collector logic. |
| 7 | Owned source lifetime is invalid after switching from a borrowed string. | The FFI dispatch now returns Cow::Owned for the generated String; the compiled app and Rust fixture test exercise this path. |
| 8 | Revisioned diagnostics omit snapshots or fail to carry them through cloning. | Runtime diagnostics capture text snapshots keyed by intrinsic input node and re-key snapshot revisions in with_revision, matching the handling of other revisioned diagnostics. |
| 9 | The diagnostic map silently accepts duplicate paragraph owners. | Trace assembly rejects duplicate text-layout node keys, paragraph-owner mismatch, and layouts without matching item frames. |
| 10 | JSON diagnostics silently erase a conversion error. | contributionInput serializes as null on conversion failure, while both strict native matchers require a structured two-axis object; such failure rejects every new scene. |
| 11 | New serialized layout values are ambiguous for enum variants. | Dimension and length/percentage values carry explicit kind tags and include numeric values where needed; trace-report tests pass. |
| 12 | Raw source and collapsed layout text share one field. | The trace exposes source text separately from normalized paragraph text; whitespace assertions compare both independently. |
| 13 | Platform implementations drift in revision or style checks. | Android and Swift assert the same revision tuple, direct source node, no inline ancestors, Ahem properties, wrapping mode, line geometry, and contribution values. |
| 14 | Numeric checks accept NaN, infinities, or unrelated integer values. | Native JSON readers reject non-finite numbers; frame and intrinsic checks use explicit tolerances and exact route/node identities. |
| 15 | Error handling converts malformed diagnostics into success. | Parse/cast failures return false; scene argument errors, missing trace chunks, assertion errors, and absent draw/present reports fail the simulator runner. |
| 16 | Trace arrays rely on callback order that the runtime does not promise. | Contribution records are matched by item, kind, value and span, while axis order is the explicit Columns then Rows contract. The native matcher does not index contributions by callback sequence. |
| 17 | Frame assertions check only width and height. | Generic node-frame strings include root and both items with node IDs, x/y positions, widths, and heights on both platforms. |
| 18 | Evidence output overwrites unrelated prior runs or targets physical devices. | The runner writes a unique run directory and checks the explicit emulator serial/AVD/API/ABI plus the selected simulator UDID/model before installation. It does not start or stop simulators. |
| 19 | Runner cleanup terminates user or unrelated processes. | The trap force-stops only the Spinon app package on emulator-5554 and terminates only dev.spinon.bootstrap on the selected iPhone simulator. Matrix completion confirmed the app processes are absent. |
| 20 | Evidence claims exceed what the screens and logs prove. | Screenshots are labeled representative layout/present evidence; zero glyph counts are recorded; iOS is simulator-only; Android hardware GPU and release package exclusion are not claimed. The official completion count and C11 checkbox remain unchanged. |

The first simulator-run attempt and initial archive-check false negative are preserved as labeled harness diagnostics. The accepted pass02 rerun, completed corrected archive boundary, test logs, and screenshots are separate artifacts. The planned merge is Rebase and merge after current-base and GitHub check status inspection.
