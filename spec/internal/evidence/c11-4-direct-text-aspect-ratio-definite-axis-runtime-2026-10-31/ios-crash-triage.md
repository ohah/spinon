# iOS Simulator timeout root cause

The initial full-matrix run stopped at `grid-text-aspect-ratio-multispan-span-baseline`. The app had reached host creation and an empty initial environment, but emitted no Grid evaluation result. Crash reports from the failed launches identified the cause:

| Time (KST) | PID | Exception | Triggered stack frame |
| --- | ---: | --- | --- |
| 2026-10-31 06:13:44 | 77872 | `EXC_BREAKPOINT` / Swift runtime `Index out of range` | `c114GenericDirectTextIntrinsicInputsMatch(sceneID:report:)` |
| 2026-10-31 06:19:47 | 81396 | `EXC_BREAKPOINT` / Swift runtime `Index out of range` | `c114GenericDirectTextIntrinsicInputsMatch(sceneID:report:)` |
| 2026-10-31 06:22:28 | 83541 | `EXC_BREAKPOINT` / Swift runtime `Index out of range` | `c114GenericDirectTextIntrinsicInputsMatch(sceneID:report:)` |

The new definite-axis check evaluated `node == expectedNodes[1]` before checking the scene ID. Existing multispan scenes provide one expected text item, so Swift evaluated an out-of-range access even though the scene was not a definite-axis scene. The condition now checks the definite-axis prefix and `expectedNodes.count > 1` before reading index 1.

After the fix, the iOS app was rebuilt and the full 55-scene run passed. The four definite-axis scenes also produced their expected Metal pixel samples. The original reports remain in the local macOS DiagnosticReports directory; this file retains only the relevant exception and stack evidence.
