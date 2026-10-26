# C11.4 generic direct-text app matcher review — pass02 (20 views)

**Date:** 2026-10-26  
**Scope:** Read-only hostile review of the Android and iOS `generic-root-normal` matchers against the current plan and latest Android API 37 / iPhone 17 Pro simulator logs. No source edits, builds, or tests were performed for this review.

## Findings

### P1 — Refresh the evidence README hashes before treating the evidence bundle as final

The latest matrix reran at 12:04 and passed, but the top-level evidence README still has hashes from the earlier 11:57 capture. Current values are:

- Android APK: `c46439a58b4a7f5ee3e287def55874f61a17cb7f451552116504200d81401773`; README: `90ecb9103e14100be9de771ad2cb1f5ce1be35ce478aefe917b72ff398104f02`.
- Android screenshot: `61026041f6f2743fe6cefe72804b64cde15bef2dd00a479950710ae373338b0f`; README: `2073913f59e523408816ae4b3308b74a0e206f008a4acf0dd8ccff34091be74b`.
- iOS screenshot: `ec1f791bb08643105acdf68f370b8f31d04c5105f507ef5fbfa6b5504edae50e`; README: `68a8b783280917a7433b284ffd9a3510349c3c089c1248677a7d7d7cf5c04275`.

Dimensions remain 1080×2400 and 1206×2622. The iOS executable and both V8 archive hashes still match the README. `source-head.txt` is still base commit `12741e4546c855d29967044a62025a57c74caf54`; the README says it will be refreshed after implementation commit and another matrix run. This is transparent, but source attribution is not complete yet.

The current Android matcher SHA-256 is `e4653bffedd0714936d4f026b9ed66fb17f89d69cd5453fd331398e9cec35f1b`; iOS matcher SHA-256 is `2007028254b6e67fe5cfa1a6595941369eff2956dc55af8220819899d9707692`. File times place the Android build (12:04:16) and runtime (12:04:25) after its source (12:03:54). The iOS source (11:56:13), successful build (11:57:06), and runtime (12:04:28) are also ordered correctly. **The latest Android runtime evidence does not predate the current Android matcher**, though the source hashes are not yet recorded in the run manifest.

### Non-blocking parity and parser-hardening gaps

- **P2 — Android pass metadata is less strict than iOS.** Android checks the two axes in order but does not validate `sequence`, null `phase`, `autoRepeatCount`, or `percentageBasis` (`C0410RuntimeGpuDemo.java:2482-2492`). iOS validates those fields (`C0410RuntimeGpuDemo.swift:3155-3171`). The recorded runtime values are correct; malformed metadata could still pass Android only.
- **P2 — Android validates only the final stage.** Its track matcher takes `stages.last()` (`C0410RuntimeGpuDemo.java:2513-2518`), while iOS checks stage order and every stage’s track/gutter shape (`C0410RuntimeGpuDemo.swift:3186-3231`). This exceeds the plan’s selected-final-track requirement but leaves platform acceptance asymmetric for malformed intermediate stages.
- **P2 — Both diagnostics parsers ignore trailing data after a valid object.** Android calls `JSONTokener.nextValue()` without checking that the tokener is exhausted (`C0410RuntimeGpuDemo.java:2370-2375`). iOS extracts the first balanced object and stops without validating the remaining suffix (`C0410RuntimeGpuDemo.swift:2623-2658`). The actual runtime report has no such suffix; malformed-input injection was not run.
- **Coverage limit — callback `contentSize` values are not compared to exact expected sizes by either app matcher.** Both require finite nonnegative dimensions and per-node MinContent/MaxContent probes; Android has now been brought to the same condition as iOS (`C0410RuntimeGpuDemo.java:2417-2452`, `C0410RuntimeGpuDemo.swift:3096-3121`). Raw intrinsic source metrics and track contributions are compared exactly within tolerance. The focused Rust tests cover the deeper text-layout results. Avoid claiming the app matchers alone prove each callback’s numeric output.

## 20-view matrix

| # | View | Result |
|---:|---|---|
| 1 | Feature-gated registration of `generic-root-normal` | PASS |
| 2 | Android/iOS route uses the same one-scene scope | PASS |
| 3 | Exactly two intrinsic inputs | PASS |
| 4 | Node IDs 2/4 and source order 0/1 | PASS |
| 5 | Text source kind and Ahem registry revision | PASS |
| 6 | Anonymous `textSources` rejected when non-empty | PASS |
| 7 | Raw min/max inline/block values checked with finite tolerance | PASS |
| 8 | HostDocument revision and nonnegative scalar checks | PASS |
| 9 | Runtime `intrinsicSize=0` accepted on both platforms | PASS |
| 10 | Intrinsic measurement entries restricted to expected text nodes | PASS |
| 11 | MinContent and MaxContent probes required per node | PASS |
| 12 | Callback content sizes only checked finite/nonnegative | PARTIAL |
| 13 | One trace for Grid node 1 | PASS |
| 14 | Exactly Columns then Rows passes | PASS |
| 15 | Pass sequence/null metadata parity | PARTIAL |
| 16 | Final content track sizes `[20,40]` and `[20]` | PASS |
| 17 | Outer collapsed gutters and 10px column gutter | PASS |
| 18 | Columns contributions and repeated identical tuple policy | PASS |
| 19 | Rows contributions at assigned 20px/40px widths | PASS |
| 20 | Screenshots, draw/present and evidence hashes | PARTIAL: claims are scoped correctly, but README hashes are stale |

## Evidence boundaries

- Latest `simulator-matrix.log` says both platforms passed. Both runtime logs contain the matching `status=0`, `layout=ready`, `boxes=3` summary and successful draw/present records; neither contains `SPINON_C114_GRID_ASSERTION_ERROR`.
- The observed trace has `intrinsicSize=0`; Columns include the expected node 2 MinContent and node 4 MinContent/MaxContent tuples with repeated identical tuples; Rows use 20px for node 2 and 10px for node 4. Serialized tracks include zero-size collapsed outer gutters and a 10px interior column gutter. Current matchers and plan agree with those values.
- Android build log reports `BUILD SUCCESSFUL`; iOS’s first build log retains a Swift compile failure, and `build-pass02.log` reports `BUILD SUCCEEDED`. The final runtime uses the successful builds.
- Screenshots show the app surface and layout boxes. Both logs report `runtime_text_count=0` and `glyph_batch_draw_count=0`; the README correctly does not present them as glyph-paint or pixel-equality evidence.
- No automatic or full-suite CI result was inspected in this matcher review.

## 2026-10-26 12:14 KST — axis-specific measurement matcher recheck

Read-only targeted recheck of the axis-specific `availableSpace` matcher update:

- Reconstructed the complete saved Android and iOS `grid_track_sizing_diagnostics` objects from all trace chunks in the run above. Both logs produce exactly: width MinContent `{2,4}`, width MaxContent `{4}`, height MinContent `{2,4}`, height MaxContent `{2,4}`.
- Java and Swift require width MinContent and both height probe kinds for nodes 2 and 4. Their checks match the saved trace. Both require node 4 for width MaxContent but do **not** reject node 2 if it also appears: Java checks `maxContentWidthNodes[1]` only (`C0410RuntimeGpuDemo.java:2457-2463`); Swift checks `.contains(4)` only (`C0410RuntimeGpuDemo.swift:3127-3131`). This is a remaining P2 false-accept gap against the exact expected width-MaxContent set `{4}`.
- Both implementations identify probe kinds with substring matching on `availableSpace`; they reject unknown nodes and malformed/nonfinite content dimensions, but do not require the entire formatted value to match a known `LayoutIntrinsicMeasureAvailableSpace` shape.
- Current matcher source SHA-256: Java `e59bae17722df997e0a0c2f4a1d1431c9d505e4bb3d7bff115bcf1cba378d5e6`; Swift `7e52dfd0af25753eefed4f375b3de5d9fa7c7ba3144fc8a1604c165990648f9c`.
- Both matcher files are timestamped 12:13:40 KST, later than the saved runtime logs at 12:04:25/12:04:28 KST. The saved simulator pass therefore confirms the trace values, but does not exercise this axis-specific matcher revision. No build or simulator rerun was performed in this recheck.

### 2026-10-26 12:15 KST — exact width-MaxContent set recheck

The follow-up matcher edit now enforces width MaxContent exactly `{4}` on both platforms: Java rejects node 2 or missing node 4 (`C0410RuntimeGpuDemo.java:2462`), and Swift compares the set to `Set([4])` (`C0410RuntimeGpuDemo.swift:3127-3130`). The saved full trace has width MaxContent nodes `{4}`, so the code matches the observed expectation; width MinContent and both height sets remain exact as recorded above. Runtime validation remains pending until the implementation is committed and rebuilt with a fresh Android/iOS matrix. No build or test was run for this recheck.
