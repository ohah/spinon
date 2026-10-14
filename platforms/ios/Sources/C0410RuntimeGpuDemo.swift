import MetalKit
import UIKit

private let c1133GridTemplateAreasSceneIDs: Set<String> = [
    "basic-rectangles", "null-cell-dot-runs", "all-null-cells", "single-cell-area",
    "bare-area-edge-aliases", "grid-area-one-value", "grid-area-two-values",
    "grid-area-three-values", "grid-area-four-values", "row-column-longhands-area",
    "grid-template-shorthand", "grid-shorthand", "bare-area-vs-explicit-one-name",
    "explicit-line-before-area", "explicit-line-after-area", "area-names-ending-start-or-end",
    "implicit-area-from-explicit-lines", "escaped-and-unicode-identifiers",
    "invalid-l-shape-fallback", "invalid-disconnected-area-fallback",
    "invalid-row-width-fallback", "invalid-trash-token-fallback", "empty-string-fallback",
    "area-track-count-mismatch", "unknown-area-implicit-tracks", "nested-hidden-source-order",
]

private struct C124MetalPixelCase {
    let direction: String
    let blueOrder: Int
    let redOrder: Int
    let runID: String

    var caseID: String {
        "flex-\(direction)-paint-order-\(blueOrder)-\(redOrder)"
    }

    var cssSample: (x: Int, y: Int) {
        (70, direction.hasPrefix("column") && direction == "column-reverse" ? 175 : 155)
    }

    var expectedRGBA: String {
        let blueOnTop = (direction == "row-reverse" || direction == "column-reverse")
            ? blueOrder == 0 : blueOrder == 1
        return blueOnTop ? "#2866f6ff" : "#e5484dff"
    }

    static func parse(arguments: [String]) -> C124MetalPixelCase? {
        guard let raw = arguments.first(where: {
            $0.hasPrefix("--spinon-c124-metal-surface-readback=")
        }) else { return nil }
        let fields = String(raw.dropFirst("--spinon-c124-metal-surface-readback=".count))
            .split(separator: ",", omittingEmptySubsequences: false)
        guard fields.count == 3,
              let blueOrder = Int(fields[1]), let redOrder = Int(fields[2]),
              ["row", "row-reverse", "column", "column-reverse"].contains(String(fields[0])),
              (blueOrder == 0 || blueOrder == 1), redOrder == 0,
              let runArgument = arguments.first(where: {
                  $0.hasPrefix("--spinon-c124-metal-surface-run=")
              }) else { return nil }
        let runID = String(runArgument.dropFirst("--spinon-c124-metal-surface-run=".count))
        guard !runID.isEmpty,
              runID.allSatisfy({ $0.isASCII && ($0.isLetter || $0.isNumber || $0 == "-" || $0 == "_") })
        else { return nil }
        return C124MetalPixelCase(
            direction: String(fields[0]), blueOrder: blueOrder,
            redOrder: redOrder, runID: runID
        )
    }
}




private struct C111MetalPixelSample: Decodable {
    let id: String
    let nodeId: String
    let xCssPx: UInt32
    let yCssPx: UInt32
    let rgba: [UInt32]
}

private struct C111MetalPixelRun {
    let runID: String
    let sceneID: String
    let markerPrefix: String
    let samples: [C111MetalPixelSample]
    private static let c1121PixelSceneIDs: Set<String> = [
        "numeric-lines", "positive-spans", "reversed-lines",
        "shorthand-cascade", "overlap-source-order", "nested-placement-modes",
    ]
    private static let c1122PixelSceneIDs: Set<String> = [
        "definite-first-mixed", "auto-only-fill", "positive-implicit-row",
        "row-locked-auto-column", "column-locked-auto-row", "auto-column-span",
        "sparse-hole-not-backfilled", "implicit-track-size-and-gap",
        "hidden-child-excluded", "nested-independent-placement", "column-cursor-wrap",
    ]
    private static let c1123PixelSceneIDs: Set<String> = [
        "row-sparse-baseline", "row-dense-hole-fill", "row-dense-span-collision",
        "row-sparse-hole-control", "row-dense-fixed-column",
        "row-dense-positive-implicit-row", "column-sparse-explicit-fill",
        "column-sparse-positive-implicit-column", "column-sparse-fixed-row",
        "column-sparse-fixed-column", "column-dense-hole-fill",
        "column-sparse-hole-control", "column-dense-span-collision",
        "column-dense-positive-implicit-column", "definite-first-dense",
        "hidden-item-excluded", "nested-mixed-flow", "auto-track-no-implicit",
        "auto-span-at-track-limit",
    ]

    private static let c1131RepeatPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("grid-background", "c1131-multi-track-repeat-grid", 300, 120, [237, 241, 247, 255]),
        ("item-blue", "c1131-multi-track-repeat-a", 10, 10, [40, 102, 246, 255]),
        ("item-red", "c1131-multi-track-repeat-b", 40, 10, [229, 72, 77, 255]),
        ("column-gap", "c1131-multi-track-repeat-grid", 112, 10, [237, 241, 247, 255]),
        ("row-gap", "c1131-multi-track-repeat-grid", 10, 53, [237, 241, 247, 255]),
    ]

    private static let c1132NamedLinePixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("grid-background", "c1132-positive-grid", 20, 80, [237, 241, 247, 255]),
        ("item-blue", "c1132-positive-item", 100, 10, [40, 102, 246, 255]),
    ]
    private static let c1133TemplateAreaPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("grid-background", "c1133-main-grid", 150, 10, [16, 24, 32, 255]),
        ("item-blue", "c1133-main-item", 100, 50, [62, 99, 221, 255]),
    ]

    private static func hasFixedC1131RepeatSamples(_ samples: [C111MetalPixelSample]) -> Bool {
        guard samples.count == c1131RepeatPixelSamples.count else { return false }
        return c1131RepeatPixelSamples.allSatisfy { expected in
            samples.contains { sample in
                sample.id == expected.0 && sample.nodeId == expected.1
                    && sample.xCssPx == expected.2 && sample.yCssPx == expected.3
                    && sample.rgba == expected.4
            }
        }
    }

    private static func hasFixedC1132NamedLineSamples(_ samples: [C111MetalPixelSample]) -> Bool {
        guard samples.count == c1132NamedLinePixelSamples.count else { return false }
        return c1132NamedLinePixelSamples.allSatisfy { expected in
            samples.contains { sample in
                sample.id == expected.0 && sample.nodeId == expected.1
                    && sample.xCssPx == expected.2 && sample.yCssPx == expected.3
                    && sample.rgba == expected.4
            }
        }
    }

    private static func hasFixedC1133TemplateAreaSamples(_ samples: [C111MetalPixelSample]) -> Bool {
        guard samples.count == c1133TemplateAreaPixelSamples.count else { return false }
        return c1133TemplateAreaPixelSamples.allSatisfy { expected in
            samples.contains { sample in
                sample.id == expected.0 && sample.nodeId == expected.1
                    && sample.xCssPx == expected.2 && sample.yCssPx == expected.3
                    && sample.rgba == expected.4
            }
        }
    }

    static func parse(arguments: [String], sceneID: String?) -> C111MetalPixelRun? {
        guard let rawRun = arguments.first(where: {
            $0.hasPrefix("--spinon-c11-grid-metal-surface-pixels=")
        }), let rawSamples = arguments.first(where: {
            $0.hasPrefix("--spinon-c11-grid-metal-surface-samples=")
        }), let sceneID else { return nil }
        let runID = String(rawRun.dropFirst("--spinon-c11-grid-metal-surface-pixels=".count))
        let encoded = String(rawSamples.dropFirst("--spinon-c11-grid-metal-surface-samples=".count))
        let c1131Scene = arguments.first(where: {
            $0.hasPrefix("--spinon-c1131-grid-repeat-scene=")
        }).map { String($0.dropFirst("--spinon-c1131-grid-repeat-scene=".count)) }
        let isC1131Repeat = arguments.contains("--spinon-c1131-grid-repeat") || c1131Scene != nil
        if isC1131Repeat {
            let c1131RunID = arguments.first(where: {
                $0.hasPrefix("--spinon-c1131-grid-repeat-run-id=")
            }).map { String($0.dropFirst("--spinon-c1131-grid-repeat-run-id=".count)) }
            guard c1131Scene == "multi-track-repeat", c1131Scene == sceneID,
                  c1131RunID == runID,
                  !arguments.contains("--spinon-c1123-grid-auto-placement"),
                  !arguments.contains("--spinon-c1122-grid-auto-placement"),
                  !arguments.contains("--spinon-c1121-grid-placement"),
                  !arguments.contains(where: { $0.hasPrefix("--spinon-c1123-grid-scene=") }),
                  !arguments.contains(where: { $0.hasPrefix("--spinon-c1122-grid-scene=") }),
                  !arguments.contains(where: { $0.hasPrefix("--spinon-c1121-grid-scene=") }) else {
                return nil
            }
        }
        let isC1132NamedLine = arguments.contains("--spinon-c1132-grid-named-lines")
        if isC1132NamedLine {
            let c1132RunID = arguments.first(where: {
                $0.hasPrefix("--spinon-c1132-grid-run-id=")
            }).map { String($0.dropFirst("--spinon-c1132-grid-run-id=".count)) }
            guard sceneID == "positive-occurrence", c1132RunID == runID,
                  !isC1131Repeat,
                  !arguments.contains("--spinon-c1123-grid-auto-placement"),
                  !arguments.contains("--spinon-c1122-grid-auto-placement"),
                  !arguments.contains("--spinon-c1121-grid-placement") else {
                return nil
            }
        }
        let isC1133TemplateAreas = arguments.contains("--spinon-c1133-grid-template-areas")
        if isC1133TemplateAreas {
            let c1133RunID = arguments.first(where: {
                $0.hasPrefix("--spinon-c1133-grid-run-id=")
            }).map { String($0.dropFirst("--spinon-c1133-grid-run-id=".count)) }
            let c1133SceneID = arguments.first(where: {
                $0.hasPrefix("--spinon-c1133-grid-scene=")
            }).map { String($0.dropFirst("--spinon-c1133-grid-scene=".count)) }
                ?? "basic-rectangles"
            guard sceneID == c1133SceneID, sceneID == "basic-rectangles",
                  c1133GridTemplateAreasSceneIDs.contains(c1133SceneID), c1133RunID == runID,
                  !isC1131Repeat, !isC1132NamedLine,
                  !arguments.contains("--spinon-c1123-grid-auto-placement"),
                  !arguments.contains("--spinon-c1122-grid-auto-placement"),
                  !arguments.contains("--spinon-c1121-grid-placement") else {
                return nil
            }
        }
        let c1123Scene = arguments.first(where: {
            $0.hasPrefix("--spinon-c1123-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1123-grid-scene=".count)) }
        let isC1123Placement = arguments.contains("--spinon-c1123-grid-auto-placement")
            || c1123Scene != nil
        if isC1123Placement {
            let c1123RunID = arguments.first(where: {
                $0.hasPrefix("--spinon-c1123-grid-run-id=")
            }).map { String($0.dropFirst("--spinon-c1123-grid-run-id=".count)) }
            guard let c1123Scene, c1123PixelSceneIDs.contains(c1123Scene),
                  c1123Scene == sceneID, c1123RunID == runID else { return nil }
        }
        let c1122Scene = arguments.first(where: {
            $0.hasPrefix("--spinon-c1122-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1122-grid-scene=".count)) }
        let isC1122Placement = arguments.contains("--spinon-c1122-grid-auto-placement")
            || c1122Scene != nil
        if isC1123Placement && isC1122Placement { return nil }
        if isC1122Placement {
            guard let c1122Scene, c1122PixelSceneIDs.contains(c1122Scene),
                  c1122Scene == sceneID else { return nil }
        }
        let c1121Scene = arguments.first(where: {
            $0.hasPrefix("--spinon-c1121-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1121-grid-scene=".count)) }
        let isC1121Placement = arguments.contains("--spinon-c1121-grid-placement") || c1121Scene != nil
        if isC1121Placement && !isC1122Placement && !isC1123Placement {
            guard let c1121Scene, c1121PixelSceneIDs.contains(c1121Scene),
                  c1121Scene == sceneID else { return nil }
        }
        guard (8...128).contains(runID.count),
              runID.allSatisfy({ $0.isASCII && ($0.isLetter || $0.isNumber || $0 == "-" || $0 == "_") }),
              (1...16_384).contains(encoded.count),
              let sampleBytes = Data(base64Encoded: encoded),
              let samples = try? JSONDecoder().decode([C111MetalPixelSample].self, from: sampleBytes),
              (1...64).contains(samples.count),
              Set(samples.map(\.id)).count == samples.count,
              Set(samples.map { "\($0.xCssPx),\($0.yCssPx)" }).count == samples.count,
              samples.allSatisfy({ sample in
                  sample.id.range(of: "^[a-z0-9-]{1,80}$", options: .regularExpression) != nil
                      && sample.nodeId.range(of: "^[a-zA-Z0-9_-]{1,80}$", options: .regularExpression) != nil
                      && sample.xCssPx < 320 && sample.yCssPx < 240
                      && sample.rgba.count == 4 && sample.rgba.allSatisfy({ $0 <= 255 })
              }) else { return nil }
        if isC1131Repeat && !hasFixedC1131RepeatSamples(samples) { return nil }
        if isC1132NamedLine && !hasFixedC1132NamedLineSamples(samples) { return nil }
        if isC1133TemplateAreas && !hasFixedC1133TemplateAreaSamples(samples) { return nil }
        return C111MetalPixelRun(runID: runID, sceneID: sceneID,
                                 markerPrefix: isC1133TemplateAreas ? "C1133"
                                     : isC1132NamedLine ? "C1132"
                                     : isC1131Repeat ? "C1131"
                                     : isC1123Placement ? "C1123"
                                     : isC1122Placement ? "C1122" : "C111",
                                 samples: samples)
    }
}

private func c11GridMetalPixelSceneID(arguments: [String]) -> String? {
    if arguments.contains("--spinon-c1133-grid-template-areas") {
        return arguments.first(where: {
            $0.hasPrefix("--spinon-c1133-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1133-grid-scene=".count)) }
            ?? "basic-rectangles"
    }
    if arguments.contains("--spinon-c1132-grid-named-lines") {
        return "positive-occurrence"
    }
    let prefixes = [
        "--spinon-c1131-grid-repeat-scene=",
        "--spinon-c1123-grid-scene=",
        "--spinon-c1122-grid-scene=",
        "--spinon-c11-grid-case=",
    ]
    for prefix in prefixes {
        if let argument = arguments.first(where: { $0.hasPrefix(prefix) }) {
            return String(argument.dropFirst(prefix.count))
        }
    }
    return nil
}

final class C0410RuntimeGpuDemoViewController: UIViewController {
    private let runtimeQueue = DispatchQueue(label: "spinon.c0410.runtime")
    private let renderQueue = DispatchQueue(label: "spinon.c0410.render")
    private let stateLock = NSLock()
    private let presentationUpdateLock = NSLock()
    private let hostLifetime = C0410RuntimeGpuHostLifetime()
    private let canvasView = C0410RuntimeGpuCanvasView()
    private let resizeButton = UIButton(type: .system)
    private let customPropertiesButton = UIButton(type: .system)
    private let statusLabel = UILabel()
    private var canvasWidthConstraint: NSLayoutConstraint?
    private var canvasHeightConstraint: NSLayoutConstraint?
    private var hostHandle: UInt64 = 0
    private var rendererReady = false
    private var rendererCreationPending = false
    private var surfaceConfigurationPending = false
    private var surfaceGeneration: UInt64 = 0
    private var rendererSurfaceGeneration: UInt64?
    private var lastDrawableSize = CGSize.zero
    private var c111MetalDrawableSize = CGSize.zero
    private var viewportWidthCssPx: Float = 301
    private var viewportHeightCssPx: Float = 100
    private var presentationSequence: UInt64 = 0
    private var expandedSurface = false
    private var automaticResizeScheduled = false
    private let failureProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c0410-failure-probe")
    private let shutdownProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c0410-shutdown-probe")
    private let runtimeResultCacheProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c053-runtime-result-cache")
    private let incrementalRestyleProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c054-incremental-restyle")
    private let percentageDimensionsProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c061-percentage-dimensions")
    private let spacingPercentagesProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c062-spacing-percentages")
    private let absoluteLengthsProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c063-absolute-lengths")
    private let fontRelativeUnitsProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c064-font-relative-units")
    private let typedCssMathProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c065-typed-css-math")
    private let viewportUnitsProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c066-viewport-units")
    private let minMaxSizingProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c071-min-max-sizing")
    private let borderWidthProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c072-border-width")
    private let aspectRatioProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c073-aspect-ratio")
    private let blockPaintProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c08-block-paint")
    private let marginCollapseProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c092-margin-collapse")
    private let flowRootProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c093-flow-root")
    private let flexWrapProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c101-flex-wrap")
    private let flexDistributionProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c102-flex-distribution")
    private let flexReverseProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1031-flex-reverse")
    private let flexOrderProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1032-flex-order")
    private let flexAlignmentProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1033-flex-alignment")
    private let flexBaselineProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1034-flex-baseline")
    private let c121PositioningProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c121-static-relative")
    private let c1121GridPlacementProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1121-grid-placement")
        || ProcessInfo.processInfo.arguments.contains(where: {
            $0.hasPrefix("--spinon-c1121-grid-scene=")
        })
    private var c1121GridSceneID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1121-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1121-grid-scene=".count)) } ?? "numeric-lines"
    }
    private var c1121GridRunID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1121-grid-run-id=")
        }).map { String($0.dropFirst("--spinon-c1121-grid-run-id=".count)) } ?? "manual"
    }
    private let c1122GridAutoPlacementSceneIDs: Set<String> = [
        "definite-first-mixed", "auto-only-fill", "positive-implicit-row",
        "row-locked-auto-column", "column-locked-auto-row", "auto-column-span",
        "sparse-hole-not-backfilled", "implicit-track-size-and-gap",
        "hidden-child-excluded", "nested-independent-placement", "reject-implicit-column",
        "reject-auto-sized-implicit-row", "reject-repeated-auto-row-pattern",
        "reject-dense-flow", "reject-column-flow", "column-cursor-wrap",
    ]
    private let c1122GridAutoPlacementProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1122-grid-auto-placement")
        || ProcessInfo.processInfo.arguments.contains(where: {
            $0.hasPrefix("--spinon-c1122-grid-scene=")
        })
    private var c1122GridSceneID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1122-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1122-grid-scene=".count)) } ?? ""
    }
    private var c1122GridRunID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1122-grid-run-id=")
        }).map { String($0.dropFirst("--spinon-c1122-grid-run-id=".count)) } ?? "manual"
    }
    private let c1123GridAutoPlacementSceneIDs: Set<String> = [
        "row-sparse-baseline", "row-dense-hole-fill", "row-dense-span-collision",
        "row-sparse-hole-control", "row-dense-fixed-row", "row-dense-fixed-column",
        "row-dense-positive-implicit-row", "column-sparse-explicit-fill",
        "column-sparse-positive-implicit-column", "column-sparse-fixed-row",
        "column-sparse-fixed-column", "column-dense-hole-fill", "column-sparse-hole-control",
        "column-dense-span-collision", "column-dense-positive-implicit-column",
        "definite-first-dense", "hidden-item-excluded", "nested-mixed-flow",
        "auto-track-no-implicit", "reject-repeated-auto-track", "reject-negative-implicit-line",
        "reject-nonzero-order", "reject-indefinite-axis", "auto-span-at-track-limit",
        "auto-span-over-track-limit",
    ]
    private let c1123GridAutoPlacementProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1123-grid-auto-placement")
        || ProcessInfo.processInfo.arguments.contains(where: {
            $0.hasPrefix("--spinon-c1123-grid-scene=")
        })
    private var c1123GridSceneID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1123-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1123-grid-scene=".count)) } ?? ""
    }
    private var c1123GridRunID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1123-grid-run-id=")
        }).map { String($0.dropFirst("--spinon-c1123-grid-run-id=".count)) } ?? "manual"
    }
    private let c1131GridRepeatSceneID = "multi-track-repeat"
    private let c1131GridRepeatProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1131-grid-repeat")
        || ProcessInfo.processInfo.arguments.contains(where: {
            $0.hasPrefix("--spinon-c1131-grid-repeat-scene=")
        })
    private var c1131GridRepeatSceneArgument: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1131-grid-repeat-scene=")
        }).map { String($0.dropFirst("--spinon-c1131-grid-repeat-scene=".count)) }
            ?? c1131GridRepeatSceneID
    }
    private var c1131GridRepeatRunID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1131-grid-repeat-run-id=")
        }).map { String($0.dropFirst("--spinon-c1131-grid-repeat-run-id=".count)) } ?? "manual"
    }
    private let c1132GridNamedLineSceneID = "positive-occurrence"
    private let c1132GridNamedLineRejectSceneID = "missing-name-implicit-line"
    private let c1132GridNamedLinesProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1132-grid-named-lines")
    private var c1132GridNamedLinesRunID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1132-grid-run-id=")
        }).map { String($0.dropFirst("--spinon-c1132-grid-run-id=".count)) } ?? "manual"
    }
    private var c1133GridTemplateAreasSceneID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1133-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1133-grid-scene=".count)) }
            ?? "basic-rectangles"
    }
    private let c1133GridTemplateAreasRejectSceneID = "area-track-count-mismatch"
    private let c1133GridTemplateAreasProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1133-grid-template-areas")
    private var c1133GridTemplateAreasRunID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1133-grid-run-id=")
        }).map { String($0.dropFirst("--spinon-c1133-grid-run-id=".count)) } ?? "manual"
    }
    private var c11GridMetalPixelStatusName: String {
        c1133GridTemplateAreasProbeRequested ? "C11.3.3"
            : c1132GridNamedLinesProbeRequested ? "C11.3.2"
            : c1131GridRepeatProbeRequested ? "C11.3.1"
            : c1123GridAutoPlacementProbeRequested ? "C11.2.3"
            : c1122GridAutoPlacementProbeRequested ? "C11.2.2"
            : c1121GridPlacementProbeRequested ? "C11.2.1" : "C11.1"
    }
    private let c11GridSceneIDs = [
        "fixed-tracks-gap", "percentage-tracks", "fractional-tracks-gap",
        "mixed-fixed-percent-fr", "row-auto-placement", "empty-grid",
        "single-item-grid", "grid-under-block", "grid-under-flex", "flex-under-grid",
    ]
    private var c11GridSceneArgument: String? {
        ProcessInfo.processInfo.arguments.first { $0.hasPrefix("--spinon-c11-grid-case=") }
            .map { String($0.dropFirst("--spinon-c11-grid-case=".count)) }
    }
    private var c11GridSceneIndex: Int? {
        if ProcessInfo.processInfo.arguments.contains("--spinon-c11-grid") { return 0 }
        guard let sceneId = c11GridSceneArgument else { return nil }
        return c11GridSceneIDs.firstIndex(of: sceneId)
    }
    private let c111MetalPixelRun = C111MetalPixelRun.parse(
        arguments: ProcessInfo.processInfo.arguments,
        sceneID: c11GridMetalPixelSceneID(arguments: ProcessInfo.processInfo.arguments)
    )
    private var c111MetalPixelArgumentRequested: Bool {
        ProcessInfo.processInfo.arguments.contains(where: {
            $0.hasPrefix("--spinon-c11-grid-metal-surface-pixels=")
        })
    }
    private var c111MetalPixelArgumentRunID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c11-grid-metal-surface-pixels=")
        }).map { String($0.dropFirst("--spinon-c11-grid-metal-surface-pixels=".count)) }
            ?? "missing"
    }
    private var c111MetalPixelRequestIssued = false
    private var c111MetalPixelReadbackFinished = false
    private var c11GridFixtureRequested: Bool {
        c1121GridPlacementProbeRequested || c1122GridAutoPlacementProbeRequested
            || c1123GridAutoPlacementProbeRequested || c1131GridRepeatProbeRequested
            || c1132GridNamedLinesProbeRequested || c1133GridTemplateAreasProbeRequested
            || ProcessInfo.processInfo.arguments.contains("--spinon-c11-grid")
            || c11GridSceneArgument != nil || c111MetalPixelRun != nil
    }
    private let c131OverflowClipProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c131-overflow-clip")
    private let c131BorderStripProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c131-overflow-clip-border")
    private let c122AbsoluteBlockProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c122-absolute-block")
    private let c123FixedPositionProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c123-fixed-position")
    private let c124StackingProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c124-stacking-context")
        || ProcessInfo.processInfo.arguments.contains("--spinon-c124-depth-safety")
        || ProcessInfo.processInfo.arguments.contains(where: {
            $0.hasPrefix("--spinon-c124-metal-surface-readback=")
        })
    private let c124MetalPixelCase = C124MetalPixelCase.parse(
        arguments: ProcessInfo.processInfo.arguments
    )
    private var c124MetalPixelReadbackFinished = false
    private let c124DepthSafetyProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c124-depth-safety")
    private let c1035PositionedFlexProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1035-positioned-flex")
    private let blockFormattingProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c091-block-formatting")
        || ProcessInfo.processInfo.arguments.contains("--spinon-c092-margin-collapse")
        || ProcessInfo.processInfo.arguments.contains("--spinon-c093-flow-root")
    private let registeredPropertiesProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c052-registered-properties")
        || ProcessInfo.processInfo.arguments.contains("--spinon-c053-runtime-result-cache")
    private var authorStylesheetsProbeRequested: Bool {
        !registeredPropertiesProbeRequested && ProcessInfo.processInfo.arguments
            .contains("--spinon-c0411-runtime-author-stylesheets")
    }
    private var customPropertiesProbeRequested: Bool {
        !registeredPropertiesProbeRequested && !authorStylesheetsProbeRequested
            && ProcessInfo.processInfo.arguments.contains("--spinon-c051-custom-properties")
    }
    private var fixedSizeCssFixtureRequested: Bool {
        c131OverflowClipProbeRequested || c131BorderStripProbeRequested || c11GridFixtureRequested
            || c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
            || c124StackingProbeRequested
            || c1035PositionedFlexProbeRequested
            || blockFormattingProbeRequested
            || flexWrapProbeRequested || flexDistributionProbeRequested
            || flexReverseProbeRequested || flexOrderProbeRequested || flexAlignmentProbeRequested
            || flexBaselineProbeRequested
    }
    private var failureProbeStarted = false
    private var shutdownProbeStarted = false
    private var closing = false
    private var c1121GridRuntimeSummaryEmitted = false
    private var c1122GridRuntimeSummaryEmitted = false
    private var c1123GridRuntimeSummaryEmitted = false
    private var displayScale: Float = 1
    private var darkMode = false
    private var runtimePresentationLane: C0410LatestTaskLane?
    private var renderDrawLane: C0410LatestTaskLane?

    override func viewDidLoad() {
        super.viewDidLoad()
        displayScale = Float(UIScreen.main.scale)
        darkMode = traitCollection.userInterfaceStyle == .dark
        view.backgroundColor = UIColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)

        let title = UILabel()
        title.text = c131BorderStripProbeRequested
            ? "SPINON · C13.1 paint/hit border"
            : c131OverflowClipProbeRequested
            ? "SPINON · C13.1 overflow: clip"
            : c1133GridTemplateAreasProbeRequested
            ? "SPINON · C11.3.3 Grid template areas"
            : c1132GridNamedLinesProbeRequested
            ? "SPINON · C11.3.2 named Grid lines"
            : c1131GridRepeatProbeRequested
            ? "SPINON · C11.3.1 Grid repeat()"
            : c1123GridAutoPlacementProbeRequested
            ? "SPINON · C11.2.3 Grid"
            : c1122GridAutoPlacementProbeRequested
            ? "SPINON · C11.2.2 Grid"
            : c1121GridPlacementProbeRequested
            ? "SPINON · C11.2.1 Grid"
            : c11GridFixtureRequested
            ? "SPINON · C11.1 CSS Grid"
            : c1035PositionedFlexProbeRequested
            ? "SPINON · C10.3.5 positioned Flex"
            : c123FixedPositionProbeRequested
            ? "SPINON · C12.3 viewport fixed"
            : c124StackingProbeRequested
            ? "SPINON · C12.4 stacking context"
            : c122AbsoluteBlockProbeRequested
            ? "SPINON · C12.2 Block absolute"
            : c121PositioningProbeRequested
            ? "SPINON · C12.1 정적·상대 위치"
            : flexBaselineProbeRequested
            ? "SPINON · C10.3.4 Flex baseline"
            : flexAlignmentProbeRequested
            ? "SPINON · C10.3.3 Flex 정렬"
            : flexReverseProbeRequested
            ? "SPINON · C10.3.1 reverse Flex"
            : flexOrderProbeRequested
            ? "SPINON · C10.3.2 Flex order"
            : flexDistributionProbeRequested
            ? "SPINON · C10.2 flex distribution"
            : flexWrapProbeRequested
            ? "SPINON · C10.1 flex-wrap"
            : flowRootProbeRequested
            ? "SPINON · C09.3 flow-root"
            : marginCollapseProbeRequested
            ? "SPINON · C09.2 margin collapse"
            : blockFormattingProbeRequested
            ? "SPINON · C09.1 Block formatting"
            : viewportUnitsProbeRequested
            ? "SPINON · C06.6 viewport units"
            : blockPaintProbeRequested
            ? "SPINON · C08 Block 흐름"
            : aspectRatioProbeRequested
            ? "SPINON · C07.3 aspect ratio"
            : borderWidthProbeRequested
            ? "SPINON · C07.2 border width"
            : minMaxSizingProbeRequested
            ? "SPINON · C07.1 min/max sizing"
            : typedCssMathProbeRequested
            ? "SPINON · C06.5 typed CSS math"
            : fontRelativeUnitsProbeRequested
            ? "SPINON · C06.4 font-relative units"
            : absoluteLengthsProbeRequested
            ? "SPINON · C06.3 absolute lengths"
            : spacingPercentagesProbeRequested
            ? "SPINON · C06.2 spacing percentages"
            : percentageDimensionsProbeRequested
            ? "SPINON · C06.1 percentage dimensions"
            : incrementalRestyleProbeRequested
            ? "SPINON · C05.4 incremental restyle"
            : registeredPropertiesProbeRequested
            ? runtimeResultCacheProbeRequested ? "SPINON · C05.3 runtime cache"
                : "SPINON · C05.2 @property"
            : authorStylesheetsProbeRequested
                ? "SPINON · C04.11 CSS → WGPU" : "SPINON · C04.10 CSS → WGPU"
        title.textColor = UIColor(red: 0.92, green: 0.95, blue: 0.99, alpha: 1)
        title.font = .systemFont(ofSize: 22, weight: .bold)
        title.translatesAutoresizingMaskIntoConstraints = false
        title.isHidden = c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
        view.addSubview(title)

        let description = UILabel()
        description.text = c131BorderStripProbeRequested
            ? "static/absolute clip border-strip hit oracle → WGPU Metal · 400×160 CSS px"
            : c131OverflowClipProbeRequested
            ? "V8 overflow: clip·clip margin·positioned escape → Stylo → Taffy → WGPU Metal · 320×240 CSS px"
            : c1133GridTemplateAreasProbeRequested
            ? "Grid template area placement → Stylo → Taffy → WGPU Metal · 320×240 CSS px"
            : c1132GridNamedLinesProbeRequested
            ? "named Grid line occurrence·span → Stylo → Taffy → WGPU · 320×240 CSS px"
            : c1131GridRepeatProbeRequested
            ? "정수 Grid repeat() 확장·line-name metadata → Stylo → Taffy → WGPU · 320×240 CSS px"
            : c1123GridAutoPlacementProbeRequested
            ? "dense·column Grid 자동 배치 → Stylo → Taffy → WGPU Metal · 360×800 CSS px"
            : c1122GridAutoPlacementProbeRequested
            ? "V8 행 우선 sparse auto-placement·implicit row → Stylo → Taffy → WGPU · 360×800 CSS px"
            : c1121GridPlacementProbeRequested
            ? "V8 numeric Grid line/span → Stylo → Taffy → WGPU · 320×240 CSS px"
            : c11GridFixtureRequested
            ? "V8 DOM display:grid → Stylo → Taffy → Rust 장면 → WGPU · 320×240 CSS px"
            : c1035PositionedFlexProbeRequested
            ? "V8 직접 Flex 자식·wrapper·paint 순서 → Stylo → Taffy → WGPU · 320×240 CSS px"
            : c123FixedPositionProbeRequested
            ? "V8 fixed viewport owner·inset·resize → Stylo → Taffy → WGPU · 360×800 CSS px"
            : c122AbsoluteBlockProbeRequested
            ? "V8 absolute·containing block·inset → Stylo → Taffy → WGPU · 360×800 CSS px"
            : c121PositioningProbeRequested
            ? "V8 position·inset·flow/visual frame → Stylo → Taffy → WGPU"
            : flexBaselineProbeRequested
            ? "V8 first·last baseline·중첩 전파 → Stylo → Taffy → WGPU · 320×240 CSS px"
            : flexAlignmentProbeRequested
            ? "V8 align-items/self/content·place-* → Stylo → Taffy → WGPU · 320×240 CSS px"
            : flexReverseProbeRequested
            ? "V8 row-reverse·wrap-reverse → Stylo → Taffy → WGPU · 320×240 CSS px"
            : flexOrderProbeRequested
            ? "V8 Flex order·stable ties·overlap paint → Stylo → Taffy → WGPU · 320×240 CSS px"
            : flexDistributionProbeRequested
            ? "V8 flex basis·grow·shrink·min/max freeze → Stylo → Taffy → WGPU · 320×240 CSS px"
            : flexWrapProbeRequested
            ? "V8 CSS flex-wrap row·gap → Stylo → Taffy → WGPU · 320×240 CSS px"
            : flowRootProbeRequested
            ? "V8 DOM flow-root BFC 경계 → Stylo → Taffy → WGPU · 320×240 CSS px"
            : marginCollapseProbeRequested
            ? "V8 DOM signed vertical margins → Stylo → Taffy → WGPU · 320×240 CSS px"
            : blockFormattingProbeRequested
            ? "V8 DOM Block·auto margin → Stylo → Taffy → WGPU · 320×240 CSS px"
            : viewportUnitsProbeRequested
            ? "V8 vw/svw/lvw/dvw → Stylo CSS px → Taffy → WGPU"
            : blockPaintProbeRequested
            ? "V8 DOM Block 흐름 → Stylo → Taffy → WGPU"
            : aspectRatioProbeRequested
            ? "V8 CSS aspect-ratio → Stylo → Taffy → WGPU"
            : borderWidthProbeRequested
            ? "V8 CSS border widths → Stylo → Taffy → WGPU · 테두리 페인트 제외"
            : minMaxSizingProbeRequested
            ? "V8 min/max size → Stylo typed values → Taffy → WGPU"
            : typedCssMathProbeRequested
            ? "V8 calc/min/max/clamp → Stylo typed values → Taffy → WGPU"
            : fontRelativeUnitsProbeRequested
            ? "V8 CSS em/rem → Stylo computed CSS px → Taffy → WGPU"
            : absoluteLengthsProbeRequested
            ? "V8 CSS absolute lengths → Stylo CSS px → Taffy → WGPU"
            : spacingPercentagesProbeRequested
            ? "V8 CSS spacing % → Stylo → Taffy → WGPU"
            : percentageDimensionsProbeRequested
            ? "V8 CSS % → typed dimensions → Taffy → WGPU"
            : incrementalRestyleProbeRequested
            ? "V8 inline style → dirty subtree → Stylo → Taffy → WGPU"
            : registeredPropertiesProbeRequested
            ? runtimeResultCacheProbeRequested
                ? "V8 detached DOM → worker cache → WGPU"
                : "V8 DOM <style> → Stylo → Taffy → WGPU"
            : authorStylesheetsProbeRequested
                ? "실제 V8 DOM <style> → Stylo → Taffy → wgpu surface"
            : "실제 V8 DOM → Stylo → Taffy → Rust 장면 → wgpu surface"
        description.textColor = UIColor(red: 0.78, green: 0.83, blue: 0.90, alpha: 1)
        description.font = .systemFont(ofSize: 14)
        description.numberOfLines = 2
        description.translatesAutoresizingMaskIntoConstraints = false
        description.isHidden = c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
        view.addSubview(description)

        canvasView.translatesAutoresizingMaskIntoConstraints = false
        canvasView.isAccessibilityElement = true
        canvasView.accessibilityLabel = c131BorderStripProbeRequested
            ? "C13.1 paint/hit border-strip Chrome 경계 WGPU Metal 장면"
            : c131OverflowClipProbeRequested
            ? "C13.1 overflow clip과 positioned escape WGPU Metal 장면"
            : c1123GridAutoPlacementProbeRequested
            ? "C11.2.3 dense·column Grid 자동 배치 WGPU 장면"
            : c1122GridAutoPlacementProbeRequested
            ? "C11.2.2 행 우선 Grid 자동 배치 WGPU 장면"
            : c1121GridPlacementProbeRequested
            ? "C11.2.1 numeric Grid line 배치 WGPU 장면"
            : c1035PositionedFlexProbeRequested
            ? "C10.3.5 positioned Flex 정렬·wrapper·paint 순서 WGPU 장면"
            : c11GridFixtureRequested
            ? "C11.1 고정 CSS Grid 트랙·gap 배치 WGPU 장면"
            : c123FixedPositionProbeRequested
            ? "C12.3 viewport fixed 배치 WGPU 장면"
            : c124StackingProbeRequested
            ? "C12.4 z-index 쌓임 맥락 WGPU 장면"
            : c122AbsoluteBlockProbeRequested
            ? "C12.2 Block absolute WGPU 장면"
            : c121PositioningProbeRequested
            ? "C12.1 정적·상대 위치 WGPU 장면"
            : flexBaselineProbeRequested
            ? "C10.3.4 Flex first·last baseline WGPU 장면"
            : flexAlignmentProbeRequested
            ? "C10.3.3 Flex Box Alignment WGPU 장면"
            : flexReverseProbeRequested
            ? "C10.3.1 row-reverse와 wrap-reverse WGPU 장면"
            : flexOrderProbeRequested
            ? "C10.3.2 Flex order와 겹친 색상 WGPU 장면"
            : flexDistributionProbeRequested
            ? "C10.2 flex grow·shrink·min/max freeze WGPU 장면"
            : flexWrapProbeRequested
            ? "C10.1 세 줄 flex-wrap·gap WGPU 장면"
            : flowRootProbeRequested
            ? "C09.3 flow-root 내부 margin 격리와 외부 collapse WGPU 장면"
            : marginCollapseProbeRequested
            ? "C09.2 signed vertical margin collapse WGPU 장면"
            : viewportUnitsProbeRequested
            ? "C06.6 viewport units WGPU 장면"
            : blockFormattingProbeRequested
            ? "C09.1 일반 Block 흐름·auto margin WGPU 장면"
            : blockPaintProbeRequested
            ? "C08 Block 흐름 기본 페인트 WGPU 장면"
            : aspectRatioProbeRequested
            ? "C07.3 종횡비 WGPU 장면"
            : borderWidthProbeRequested
            ? "C07.2 테두리 폭 레이아웃 WGPU 장면"
            : minMaxSizingProbeRequested
            ? "C07.1 min/max sizing WGPU 장면"
            : typedCssMathProbeRequested
            ? "C06.5 calc min max clamp WGPU 장면"
            : fontRelativeUnitsProbeRequested
            ? "C06.4 em/rem font-relative units WGPU 장면"
            : absoluteLengthsProbeRequested
            ? "C06.3 absolute CSS length units WGPU 장면"
            : spacingPercentagesProbeRequested
            ? "C06.2 percentage margin, padding, gap WGPU 장면"
            : percentageDimensionsProbeRequested
            ? "C06.1 percentage width, height, flex-basis WGPU 장면"
            : incrementalRestyleProbeRequested
            ? "C05.4 inline style 하위 트리 재계산 검증 WGPU 장면"
            : registeredPropertiesProbeRequested
            ? runtimeResultCacheProbeRequested
                ? "C05.3 detached-node 결과 재사용 검증 WGPU 장면"
                : "C05.2 등록 사용자 지정 속성 Chromium fixture의 WGPU 장면"
            : authorStylesheetsProbeRequested
                ? "C04.11 Chromium stylesheet fixture의 WGPU 장면"
            : "C04.10 Chromium fixture의 WGPU 장면"
        view.addSubview(canvasView)

        resizeButton.setTitle("표면 크기 전환 · 301×100 CSS px", for: .normal)
        resizeButton.addTarget(self, action: #selector(toggleSurfaceSize), for: .touchUpInside)
        resizeButton.translatesAutoresizingMaskIntoConstraints = false
        if fixedSizeCssFixtureRequested { resizeButton.isHidden = true }
        view.addSubview(resizeButton)

        customPropertiesButton.setTitle(
            typedCssMathProbeRequested
                ? "C06.5 typed CSS math fixture 다시 실행"
                : fontRelativeUnitsProbeRequested
                ? "C06.4 em/rem fixture 실행 완료"
                : absoluteLengthsProbeRequested
                ? "C06.3 absolute length fixture 실행 완료"
                : spacingPercentagesProbeRequested
                ? "C06.2 spacing percentage fixture 실행 완료"
                : percentageDimensionsProbeRequested
                ? "C06.1 percentage fixture 실행 완료"
                : incrementalRestyleProbeRequested
                ? "C05.4 · 왼쪽 branch style 전환"
                : registeredPropertiesProbeRequested
                ? runtimeResultCacheProbeRequested
                    ? "C05.3 · detached / 연결 변경 실행"
                    : "C05.2 · 등록 사용자 지정 속성 다시 적용"
                : "C05 · 사용자 지정 속성 다시 적용",
            for: .normal
        )
        customPropertiesButton.addTarget(
            self,
            action: typedCssMathProbeRequested
                ? #selector(evaluateTypedCssMathFixture)
                : fontRelativeUnitsProbeRequested
                ? #selector(evaluateFontRelativeUnitsFixture)
                : absoluteLengthsProbeRequested
                ? #selector(evaluateAbsoluteLengthsFixture)
                : spacingPercentagesProbeRequested
                ? #selector(evaluateSpacingPercentagesFixture)
                : percentageDimensionsProbeRequested
                ? #selector(evaluatePercentageDimensionsFixture)
                : incrementalRestyleProbeRequested
                ? #selector(evaluateIncrementalRestyleFixture)
                : registeredPropertiesProbeRequested
                ? runtimeResultCacheProbeRequested
                    ? #selector(evaluateRuntimeResultCacheFixture)
                    : #selector(evaluateRegisteredPropertiesFixture)
                : #selector(evaluateCustomPropertiesFixture),
            for: .touchUpInside
        )
        customPropertiesButton.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(customPropertiesButton)
        if c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
            || c121PositioningProbeRequested
            || authorStylesheetsProbeRequested
            || fixedSizeCssFixtureRequested || blockPaintProbeRequested
            || minMaxSizingProbeRequested || borderWidthProbeRequested
            || aspectRatioProbeRequested
            || typedCssMathProbeRequested
            || fontRelativeUnitsProbeRequested
            || absoluteLengthsProbeRequested
            || percentageDimensionsProbeRequested
            || spacingPercentagesProbeRequested {
            customPropertiesButton.isHidden = true
        }

        statusLabel.text = "V8·CSS runtime 준비 중…"
        statusLabel.textColor = UIColor(red: 0.38, green: 0.73, blue: 1, alpha: 1)
        statusLabel.font = .monospacedSystemFont(ofSize: 11, weight: .regular)
        statusLabel.numberOfLines = 0
        statusLabel.translatesAutoresizingMaskIntoConstraints = false
        statusLabel.isHidden = c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
        view.addSubview(statusLabel)

        let canvasWidth = canvasView.widthAnchor.constraint(
            equalToConstant: c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
                ? 360 : c131BorderStripProbeRequested ? 400 : fixedSizeCssFixtureRequested ? 320 : 301
        )
        let canvasHeight = canvasView.heightAnchor.constraint(
            equalToConstant: c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
                ? 800 : c131BorderStripProbeRequested ? 160 : fixedSizeCssFixtureRequested ? 240 : 100
        )
        canvasWidthConstraint = canvasWidth
        canvasHeightConstraint = canvasHeight
        NSLayoutConstraint.activate([
            title.leadingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.leadingAnchor, constant: 22),
            title.trailingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.trailingAnchor, constant: -22),
            title.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 24),
            description.leadingAnchor.constraint(equalTo: title.leadingAnchor),
            description.trailingAnchor.constraint(equalTo: title.trailingAnchor),
            description.topAnchor.constraint(equalTo: title.bottomAnchor, constant: 8),
            canvasView.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            canvasView.centerYAnchor.constraint(equalTo: view.centerYAnchor),
            canvasWidth,
            canvasHeight,
            resizeButton.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            resizeButton.topAnchor.constraint(equalTo: canvasView.bottomAnchor, constant: 12),
            customPropertiesButton.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            customPropertiesButton.topAnchor.constraint(equalTo: resizeButton.bottomAnchor, constant: 4),
            statusLabel.leadingAnchor.constraint(equalTo: title.leadingAnchor),
            statusLabel.trailingAnchor.constraint(equalTo: title.trailingAnchor),
            statusLabel.bottomAnchor.constraint(equalTo: view.safeAreaLayoutGuide.bottomAnchor, constant: -18),
        ])
        if failureProbeRequested && shutdownProbeRequested {
            statusLabel.text = "draw 복구와 종료 검증은 나눠 실행해야 합니다."
            log("SPINON_C0410_PROBE_ARGUMENT_ERROR · failure와 shutdown 검증을 동시에 요청했습니다")
            return
        }
        runtimePresentationLane = C0410LatestTaskLane(
            queue: runtimeQueue,
            work: { [weak self] in
                guard let self else { return }
                try self.applyLatestEnvironment()
            },
            onFailure: { [weak self] message in self?.reportLaneFailure("runtime", message) }
        )
        renderDrawLane = C0410LatestTaskLane(
            queue: renderQueue,
            work: { [weak self] in
                guard let self else { return }
                try self.drawLatestIfReady()
            },
            onFailure: { [weak self] message in self?.reportLaneFailure("render", message) }
        )
        enqueueRuntime { [weak self] in self?.initializeRuntime() }
    }

    override func viewDidDisappear(_ animated: Bool) {
        super.viewDidDisappear(animated)
        beginShutdown()
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        guard !automaticResizeScheduled,
              ProcessInfo.processInfo.arguments.contains("--spinon-c0410-auto-resize") else {
            return
        }
        automaticResizeScheduled = true
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) { [weak self] in
            self?.runAutomaticResizeProbe(step: 0)
        }
    }

    private func runAutomaticResizeProbe(step: Int) {
        guard step < 3, !isClosing else { return }
        toggleSurfaceSize()
        guard step + 1 < 3 else { return }
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.75) { [weak self] in
            self?.runAutomaticResizeProbe(step: step + 1)
        }
    }

    override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()
        let scale = view.window?.screen.scale ?? UIScreen.main.scale
        let viewportSize = c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
            ? CGSize(width: 360, height: 800) : canvasView.bounds.size
        guard viewportSize.width > 0, viewportSize.height > 0 else { return }
        stateLock.lock()
        let scaleChanged = displayScale != Float(scale)
        let viewportChanged = viewportWidthCssPx != Float(viewportSize.width)
            || viewportHeightCssPx != Float(viewportSize.height)
        displayScale = Float(scale)
        viewportWidthCssPx = Float(viewportSize.width)
        viewportHeightCssPx = Float(viewportSize.height)
        stateLock.unlock()
        let size = CGSize(
            width: (canvasView.bounds.width * scale).rounded(),
            height: (canvasView.bounds.height * scale).rounded()
        )
        let sizeChanged = size != lastDrawableSize
        guard size.width > 0, size.height > 0 else { return }
        stateLock.lock()
        c111MetalDrawableSize = size
        stateLock.unlock()
        if viewportChanged || sizeChanged {
            log("SPINON_C0410_VIEWPORT_CHANGED css=\(viewportSize.width)x\(viewportSize.height) scale=\(scale) drawable=\(Int(size.width))x\(Int(size.height))")
        }
        if scaleChanged || sizeChanged {
            if sizeChanged {
                stateLock.lock()
                surfaceGeneration &+= 1
                let generation = surfaceGeneration
                stateLock.unlock()
                log("SPINON_C0410_SURFACE_CHANGED generation=\(generation) size=\(Int(size.width))x\(Int(size.height))")
            }
            _ = beginPresentationUpdate()
        }
        guard sizeChanged else {
            if scaleChanged || viewportChanged { refreshEnvironment() }
            return
        }
        lastDrawableSize = size
        (canvasView.layer as? CAMetalLayer)?.drawableSize = size
        if rendererIsReady {
            resizeRenderer(to: size)
        } else {
            ensureRenderer(width: Int(size.width), height: Int(size.height))
        }
    }

    override func traitCollectionDidChange(_ previousTraitCollection: UITraitCollection?) {
        super.traitCollectionDidChange(previousTraitCollection)
        guard previousTraitCollection?.userInterfaceStyle != traitCollection.userInterfaceStyle else {
            return
        }
        stateLock.lock()
        darkMode = traitCollection.userInterfaceStyle == .dark
        stateLock.unlock()
        refreshEnvironment()
    }

    private var rendererIsReady: Bool {
        stateLock.lock()
        defer { stateLock.unlock() }
        return rendererReady
    }

    private var isClosing: Bool {
        stateLock.lock()
        defer { stateLock.unlock() }
        return closing
    }

    private func currentEnvironment() -> (width: Float, height: Float, scale: Float, dark: Bool) {
        stateLock.lock()
        defer { stateLock.unlock() }
        return (viewportWidthCssPx, viewportHeightCssPx, displayScale, darkMode)
    }

    private func runtimeStatusSummary(_ report: String) -> String {
        let status = String(report.split(separator: " ", maxSplits: 1).first ?? "")
        guard report.contains(" layout=") else {
            return String(report.prefix(180))
        }
        return [
            status,
            "layout=\(reportField(report, "layout="))",
            "boxes=\(reportField(report, "boxes="))",
            "generation=\(reportField(report, "generation="))",
            "document_revision=\(reportField(report, "document_revision="))",
            "environment_revision=\(reportField(report, "environment_revision="))"
        ].joined(separator: " ")
    }

    private func logC131FixtureReport(_ report: String) -> Bool {
        let expectedHitIDs = [
            "visible-target", "clip-inside", "clip-outside", "clip-margin-expanded",
            "clip-margin-outside", "nested-clip-inside", "nested-clip-outside",
            "nested-y-visible", "nested-y-clip-outside", "positioned-escape",
            "outer-clip-outside", "viewport-left-outside", "viewport-top-outside",
            "viewport-right-outside", "viewport-bottom-outside",
        ]
        guard report.hasPrefix("status=0 "),
              report.contains(" layout=ready "),
              reportField(report, "boxes=") == "13",
              let frameMarker = report.range(of: "node_frames_css_px=["),
              let frameEnd = report[frameMarker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C131_FIXTURE_SUMMARY status=error stage=runtime-report \(runtimeStatusSummary(report))")
            return false
        }
        let frames = report[frameMarker.upperBound..<frameEnd].split(separator: ";")
        for frame in frames {
            log("SPINON_C131_NODE_FRAME \(frame)")
        }
        log("SPINON_C131_NODE_FRAME_COUNT count=\(frames.count)")

        guard let sampleMarker = report.range(of: "hit_test_samples_json="),
              let sampleStart = report[sampleMarker.upperBound...].firstIndex(of: "["),
              let sampleEnd = report[report.index(after: sampleStart)...].firstIndex(of: "]") else {
            log("SPINON_C131_FIXTURE_SUMMARY status=error stage=hit-test-report")
            return false
        }
        let sampleJSON = String(report[sampleStart...sampleEnd])
        guard let sampleData = sampleJSON.data(using: .utf8),
              let samples = try? JSONSerialization.jsonObject(with: sampleData) as? [[String: Any]] else {
            log("SPINON_C131_FIXTURE_SUMMARY status=error stage=hit-test-json")
            return false
        }
        let sampleIDs = samples.compactMap { $0["id"] as? String }
        let matched = samples.filter { $0["status"] as? String == "matched" }
        for sample in samples {
            let id = sample["id"] as? String ?? "missing"
            let element = sample["elementId"] as? String ?? "null"
            let x = sample["xCssPx"] ?? "missing"
            let y = sample["yCssPx"] ?? "missing"
            let status = sample["status"] as? String ?? "missing"
            log("SPINON_C131_HIT_TEST id=\(id) x=\(x) y=\(y) element=\(element) status=\(status)")
        }
        let passed = frames.count == 13 && samples.count == 15
            && matched.count == 15 && sampleIDs == expectedHitIDs
        log("SPINON_C131_HIT_TEST_SUMMARY status=\(passed ? "ok" : "error") matched=\(matched.count) total=\(samples.count) ids=\(sampleIDs == expectedHitIDs ? "fixed" : "unexpected")")
        log("SPINON_C131_FIXTURE_SUMMARY status=\(passed ? "ok" : "error") boxes=13 frames=\(frames.count) hit_samples=\(samples.count) hit_matches=\(matched.count)")
        return passed
    }

    private func logC131BorderFixtureReport(_ report: String) -> Bool {
        let expectedSampleIDs = [
            "static-inside-clip", "static-border-strip", "static-outside-border",
            "positioned-inside-clip", "positioned-border-strip", "positioned-outside-border",
        ]
        let expectedTargets = [
            "static-target", "static-clip", "root", "positioned-target",
            "positioned-target", "root",
        ]
        let expectedPoints: [(Double, Double)] = [
            (125, 40), (128, 40), (131, 40), (315, 40), (318, 40), (321, 40),
        ]
        let expectedFrames: [(Double, Double, Double, Double)] = [
            (0, 0, 380, 120), (10, 10, 120, 96), (24, 24, 140, 120),
            (200, 10, 120, 96), (214, 24, 140, 120),
        ]
        guard report.hasPrefix("status=0 "),
              report.contains(" layout=ready "),
              reportField(report, "boxes=") == "5",
              let frameMarker = report.range(of: "node_frames_css_px=["),
              let frameEnd = report[frameMarker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C131_BORDER_FIXTURE_SUMMARY status=error stage=runtime-report \(runtimeStatusSummary(report))")
            return false
        }
        let frames = report[frameMarker.upperBound..<frameEnd].split(separator: ";")
        for frame in frames {
            log("SPINON_C131_BORDER_NODE_FRAME \(frame)")
        }
        let geometryMatches = frames.count == expectedFrames.count
            && frames.enumerated().allSatisfy { index, frame in
                var coordinates: [String: Double] = [:]
                for field in frame.split(separator: ",") {
                    let pair = field.split(separator: "=", maxSplits: 1)
                    guard pair.count == 2,
                          let value = Double(pair[1]),
                          ["x", "y", "width", "height"].contains(String(pair[0])) else {
                        continue
                    }
                    coordinates[String(pair[0])] = value
                }
                let expected = expectedFrames[index]
                return coordinates["x"] == expected.0
                    && coordinates["y"] == expected.1
                    && coordinates["width"] == expected.2
                    && coordinates["height"] == expected.3
            }
        log("SPINON_C131_BORDER_GEOMETRY_SUMMARY status=\(geometryMatches ? "ok" : "error") frames=\(frames.count)")

        guard let sampleMarker = report.range(of: "hit_test_samples_json="),
              let sampleStart = report[sampleMarker.upperBound...].firstIndex(of: "["),
              let sampleEnd = report[report.index(after: sampleStart)...].firstIndex(of: "]") else {
            log("SPINON_C131_BORDER_FIXTURE_SUMMARY status=error stage=hit-test-report")
            return false
        }
        let sampleJSON = String(report[sampleStart...sampleEnd])
        guard let sampleData = sampleJSON.data(using: .utf8),
              let samples = try? JSONSerialization.jsonObject(with: sampleData) as? [[String: Any]] else {
            log("SPINON_C131_BORDER_FIXTURE_SUMMARY status=error stage=hit-test-json")
            return false
        }
        let sampleIDs = samples.compactMap { $0["id"] as? String }
        let matched = samples.filter { $0["status"] as? String == "matched" }
        let samplesAgree = samples.count == expectedSampleIDs.count
            && samples.enumerated().allSatisfy { index, sample in
                (sample["id"] as? String) == expectedSampleIDs[index]
                    && (sample["elementId"] as? String) == expectedTargets[index]
                    && (sample["xCssPx"] as? Double) == expectedPoints[index].0
                    && (sample["yCssPx"] as? Double) == expectedPoints[index].1
                    && (sample["status"] as? String) == "matched"
            }
        for sample in samples {
            let id = sample["id"] as? String ?? "missing"
            let element = sample["elementId"] as? String ?? "null"
            let x = sample["xCssPx"] ?? "missing"
            let y = sample["yCssPx"] ?? "missing"
            let status = sample["status"] as? String ?? "missing"
            log("SPINON_C131_BORDER_HIT_TEST id=\(id) x=\(x) y=\(y) element=\(element) status=\(status)")
        }
        let passed = frames.count == 5 && geometryMatches && samplesAgree && matched.count == 6
            && sampleIDs == expectedSampleIDs
        log("SPINON_C131_BORDER_HIT_TEST_SUMMARY status=\(passed ? "ok" : "error") matched=\(matched.count) total=\(samples.count) ids=\(sampleIDs == expectedSampleIDs ? "fixed" : "unexpected")")
        log("SPINON_C131_BORDER_FIXTURE_SUMMARY status=\(passed ? "ok" : "error") boxes=5 frames=\(frames.count) hit_samples=\(samples.count) hit_matches=\(matched.count)")
        return passed
    }

    private func logC1031NodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1031_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1031_NODE_FRAME \(frame)")
        }
        log("SPINON_C1031_FRAME_SUMMARY frames=\(frames.count) marker=present")
    }

    private func logC121NodeFrames(_ state: String, _ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C121_FRAME_SUMMARY state=\(state) marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C121_NODE_FRAME state=\(state) \(frame)")
        }
        log("SPINON_C121_FRAME_SUMMARY state=\(state) frames=\(frames.count) marker=present")
    }

    private func logC122NodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C122_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C122_NODE_FRAME \(frame)")
        }
        log("SPINON_C122_FRAME_SUMMARY frames=\(frames.count) marker=present")
    }

    private func logC123NodeFrames(_ state: String, _ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C123_FRAME_SUMMARY state=\(state) marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C123_NODE_FRAME state=\(state) \(frame)")
        }
        log("SPINON_C123_FRAME_SUMMARY state=\(state) frames=\(frames.count) marker=present")
    }

    private func logC123FailureDetails(_ report: String) {
        let chunkSize = 600
        let characters = Array(report)
        let chunkCount = (characters.count + chunkSize - 1) / chunkSize
        for index in 0..<chunkCount {
            let start = index * chunkSize
            let end = min(characters.count, start + chunkSize)
            log("SPINON_C123_EVAL_DETAIL part=\(index + 1)/\(chunkCount) \(String(characters[start..<end]))")
        }
    }

    private func runC123EnvironmentMatrix(
        host: UInt64,
        dark: Bool,
        deviceScaleFactor: Float,
        initialEnvironmentRevision: UInt64
    ) -> Bool {
        let steps: [(String, Float, Float, Float)] = [
            ("matrix-360x800-dpr-2", 360, 800, 2),
            ("matrix-360x800-dpr-2.625", 360, 800, 2.625),
            ("matrix-360x800-dpr-3", 360, 800, 3),
            ("matrix-390x844-dpr-1", 390, 844, 1),
            ("matrix-390x844-dpr-2", 390, 844, 2),
            ("matrix-390x844-dpr-2.625", 390, 844, 2.625),
            ("matrix-390x844-dpr-3", 390, 844, 3),
            ("resize-start", 360, 800, 1),
            ("resize-outbound", 390, 844, 1),
            ("resize-noop", 390, 844, 1),
            ("resize-return", 360, 800, 1),
            ("device-final", 360, 800, deviceScaleFactor),
        ]
        var expectedRevision = initialEnvironmentRevision
        for (state, width, height, scale) in steps {
            if state != "resize-noop" { expectedRevision += 1 }
            let report = SpinonRunner.setRuntimeGpuEnvironment(
                host, width: width, height: height, scale: scale, dark: dark
            )
            guard let report, report.hasPrefix("status=0 ") else {
                log("SPINON_C123_MATRIX_FAILURE state=\(state) report=\(report ?? "missing")")
                return false
            }
            let actualRevision = reportField(report, "environment_revision=")
            guard actualRevision == String(expectedRevision) else {
                log("SPINON_C123_MATRIX_FAILURE state=\(state) expected_revision=\(expectedRevision) actual_revision=\(actualRevision)")
                return false
            }
            log("SPINON_C123_SUMMARY state=\(state) \(runtimeStatusSummary(report))")
            logC123NodeFrames(state, report)
        }
        return true
    }

    private func logC1035NodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1035_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1035_NODE_FRAME \(frame)")
        }
        log("SPINON_C1035_FRAME_SUMMARY frames=\(frames.count) marker=present")
    }

    private func logC111GridNodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C111_GRID_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C111_GRID_NODE_FRAME_SWIFT \(frame)")
        }
        log("SPINON_C111_GRID_FRAME_SUMMARY count=\(frames.count) marker=present")
    }

    private func logC1121GridNodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1121_GRID_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1121_GRID_NODE_FRAME_RUN run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) frame=\(frame)")
        }
        log("SPINON_C1121_GRID_FRAME_SUMMARY run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) count=\(frames.count) marker=present")
    }

    private func logC1132GridNamedLineFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1132_GRID_FRAME_SUMMARY run_id=\(c1132GridNamedLinesRunID) marker=missing")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1132_GRID_NODE_FRAME_RUN run_id=\(c1132GridNamedLinesRunID) scene_id=\(c1132GridNamedLineSceneID) frame=\(frame)")
        }
        log("SPINON_C1132_GRID_FRAME_SUMMARY run_id=\(c1132GridNamedLinesRunID) scene_id=\(c1132GridNamedLineSceneID) count=\(frames.count) marker=present")
    }

    private func logC1133GridTemplateAreaFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1133_GRID_FRAME_SUMMARY run_id=\(c1133GridTemplateAreasRunID) marker=missing")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1133_GRID_NODE_FRAME_RUN run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasSceneID) frame=\(frame)")
        }
        log("SPINON_C1133_GRID_FRAME_SUMMARY run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasSceneID) count=\(frames.count) marker=present")
    }

    private func logGridAutoPlacementReadySummaryIfNeeded(
        _ report: String,
        profile: String,
        runID: String,
        sceneID: String
    ) {
        guard report.hasPrefix("status=0 "), report.contains(" layout=ready ") else { return }
        stateLock.lock()
        let shouldLog: Bool
        if profile == "C1123" {
            shouldLog = !c1123GridRuntimeSummaryEmitted
            c1123GridRuntimeSummaryEmitted = true
        } else {
            shouldLog = !c1122GridRuntimeSummaryEmitted
            c1122GridRuntimeSummaryEmitted = true
        }
        stateLock.unlock()
        guard shouldLog else { return }
        log("SPINON_\(profile)_GRID_SUMMARY run_id=\(runID) scene_id=\(sceneID) \(runtimeStatusSummary(report))")
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_\(profile)_GRID_FRAME_SUMMARY run_id=\(runID) scene_id=\(sceneID) marker=missing")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_\(profile)_GRID_NODE_FRAME_RUN run_id=\(runID) scene_id=\(sceneID) frame=\(frame)")
        }
        log("SPINON_\(profile)_GRID_FRAME_SUMMARY run_id=\(runID) scene_id=\(sceneID) count=\(frames.count) marker=present")
        let paintMarker = "paint_node_order=["
        guard let start = report.range(of: paintMarker),
              let end = report[start.upperBound...].firstIndex(of: "]"),
              report.distance(from: start.lowerBound, to: end) <= 512 else { return }
        log("SPINON_\(profile)_GRID_PAINT_ORDER run_id=\(runID) scene_id=\(sceneID) \(report[start.lowerBound...end])")
    }

    private func logC1121GridPaintOrder(_ report: String) {
        let marker = "paint_node_order=["
        guard let start = report.range(of: marker),
              let end = report[start.upperBound...].firstIndex(of: "]"),
              report.distance(from: start.lowerBound, to: end) <= 512 else {
            log("SPINON_C1121_GRID_PAINT_ORDER run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) marker=missing-or-too-large")
            return
        }
        log("SPINON_C1121_GRID_PAINT_ORDER run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) \(report[start.lowerBound...end])")
    }

    private func logC1121GridReadySummaryIfNeeded(_ report: String) {
        guard report.hasPrefix("status=0 "), report.contains(" layout=ready ") else { return }
        stateLock.lock()
        let shouldLog = !c1121GridRuntimeSummaryEmitted
        c1121GridRuntimeSummaryEmitted = true
        stateLock.unlock()
        guard shouldLog else { return }
        log("SPINON_C1121_GRID_SUMMARY run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) \(runtimeStatusSummary(report))")
        logC1121GridNodeFrames(report)
        logC1121GridPaintOrder(report)
    }

    private func evalC121PositionState(
        host: UInt64,
        state: UInt32
    ) -> (report: String, attempts: Int) {
        var report = ""
        for attempt in 1...5 {
            report = SpinonRunner.evalRuntimeGpuC12_1PositionState(host, state: state)
            let superseded = report.hasPrefix("status=-12 ")
                && report.contains("op=eval status=0 ")
            if !superseded || attempt == 5 {
                return (report, attempt)
            }
            Thread.sleep(forTimeInterval: 0.025)
        }
        return (report, 5)
    }

    private func logC1032NodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1032_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1032_NODE_FRAME \(frame)")
        }
        log("SPINON_C1032_FRAME_SUMMARY frames=\(frames.count) marker=present")
    }

    private func logC1033NodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1033_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1033_NODE_FRAME \(frame)")
        }
        log("SPINON_C1033_FRAME_SUMMARY frames=\(frames.count) marker=present")
    }

    private func logC1033RuntimeSummary(_ report: String) {
        let fields = report.split(whereSeparator: \.isWhitespace)
        func field(_ name: String) -> String {
            fields.first(where: { $0.hasPrefix(name) }).map(String.init) ?? "\(name)missing"
        }
        log("SPINON_C1033_RUNTIME_SUMMARY \(field("status=")) \(field("layout=")) \(field("boxes=")) \(field("error="))")
    }

    private func logC1034NodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1034_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1034_NODE_FRAME \(frame)")
        }
        log("SPINON_C1034_FRAME_SUMMARY frames=\(frames.count) marker=present")
    }

    private func logC1034RuntimeSummary(_ report: String) {
        let fields = report.split(whereSeparator: \.isWhitespace)
        func field(_ name: String) -> String {
            fields.first(where: { $0.hasPrefix(name) }).map(String.init) ?? "\(name)missing"
        }
        log("SPINON_C1034_RUNTIME_SUMMARY \(field("status=")) \(field("layout=")) \(field("boxes=")) \(field("error="))")
    }

    @objc private func toggleSurfaceSize() {
        guard !isClosing else { return }
        expandedSurface.toggle()
        let width: CGFloat = expandedSurface ? 341 : 301
        let height: CGFloat = expandedSurface ? 128 : 100
        canvasWidthConstraint?.constant = width
        canvasHeightConstraint?.constant = height
        resizeButton.setTitle(
            "표면 크기 전환 · \(Int(width))×\(Int(height)) CSS px",
            for: .normal
        )
        log("SPINON_C0410_RESIZE_REQUEST css=\(Int(width))x\(Int(height))")
        view.setNeedsLayout()
        view.layoutIfNeeded()
    }

    private func initializeRuntime() {
        guard !isClosing else { return }
        if c131OverflowClipProbeRequested && c131BorderStripProbeRequested {
            log("SPINON_C131_ARGUMENT_ERROR conflicting=both-overflow-fixture-routes")
            postStatus("실패 · C13.1 route를 하나만 선택해야 합니다")
            return
        }
        if c131OverflowClipProbeRequested || c131BorderStripProbeRequested,
           let conflictingArgument = ProcessInfo.processInfo.arguments.first(where: {
               $0.hasPrefix("--spinon-c")
                   && $0 != "--spinon-c131-overflow-clip"
                   && $0 != "--spinon-c131-overflow-clip-border"
                   && $0 != "--spinon-c0410-runtime-gpu"
           }) {
            log("SPINON_C131_ARGUMENT_ERROR conflicting=\(conflictingArgument)")
            postStatus("실패 · C13.1 전용 route와 다른 CSS fixture를 함께 선택했습니다")
            return
        }
        if c1131GridRepeatProbeRequested
            && (c1131GridRepeatSceneArgument != c1131GridRepeatSceneID
                || c1131GridRepeatRunID.range(
                    of: "^[A-Za-z0-9_-]{1,80}$", options: .regularExpression) == nil
                || c1123GridAutoPlacementProbeRequested
                || c1122GridAutoPlacementProbeRequested
                || c1121GridPlacementProbeRequested) {
            log("SPINON_C1131_GRID_ARGUMENT_ERROR rejected=true scene_id=\(c1131GridRepeatSceneArgument)")
            postStatus("실패 · 알 수 없거나 중복 선택된 C11.3.1 Grid scene")
            return
        }
        if c1132GridNamedLinesProbeRequested
            && (c1132GridNamedLinesRunID.range(
                of: "^[A-Za-z0-9_-]{1,80}$", options: .regularExpression) == nil
                || c1131GridRepeatProbeRequested
                || c1123GridAutoPlacementProbeRequested
                || c1122GridAutoPlacementProbeRequested
                || c1121GridPlacementProbeRequested) {
            log("SPINON_C1132_GRID_ARGUMENT_ERROR rejected=true")
            postStatus("실패 · C11.3.2 Grid fixture 또는 profile 선택이 올바르지 않습니다")
            return
        }
        if c1133GridTemplateAreasProbeRequested
            && (c1133GridTemplateAreasRunID.range(
                of: "^[A-Za-z0-9_-]{1,80}$", options: .regularExpression) == nil
                || !c1133GridTemplateAreasSceneIDs.contains(c1133GridTemplateAreasSceneID)
                || c1131GridRepeatProbeRequested || c1132GridNamedLinesProbeRequested
                || c1123GridAutoPlacementProbeRequested
                || c1122GridAutoPlacementProbeRequested
                || c1121GridPlacementProbeRequested) {
            log("SPINON_C1133_GRID_ARGUMENT_ERROR rejected=true")
            postStatus("실패 · C11.3.3 Grid fixture 또는 profile 선택이 올바르지 않습니다")
            return
        }
        if c1123GridAutoPlacementProbeRequested
            && (!c1123GridAutoPlacementSceneIDs.contains(c1123GridSceneID)
                || c1123GridRunID.range(
                    of: "^[A-Za-z0-9_-]{1,80}$", options: .regularExpression) == nil
                || c1122GridAutoPlacementProbeRequested || c1121GridPlacementProbeRequested) {
            log("SPINON_C1123_GRID_ARGUMENT_ERROR rejected=true scene_id=\(c1123GridSceneID)")
            postStatus("실패 · 알 수 없거나 중복 선택된 C11.2.3 Grid scene")
            return
        }
        if c1122GridAutoPlacementProbeRequested
            && !c1122GridAutoPlacementSceneIDs.contains(c1122GridSceneID) {
            log("SPINON_C1122_GRID_ARGUMENT_ERROR rejected=true scene_id=\(c1122GridSceneID)")
            postStatus("실패 · 알 수 없는 C11.2.2 Grid scene")
            return
        }
        if c11GridFixtureRequested && !c1121GridPlacementProbeRequested
            && !c1122GridAutoPlacementProbeRequested
            && !c1123GridAutoPlacementProbeRequested
            && !c1131GridRepeatProbeRequested
            && !c1132GridNamedLinesProbeRequested
            && !c1133GridTemplateAreasProbeRequested
            && c11GridSceneIndex == nil {
            log("SPINON_C111_GRID_ARGUMENT_ERROR rejected=true")
            postStatus("실패 · 알 수 없는 C11.1 Grid scene")
            return
        }
        if c111MetalPixelArgumentRequested && c111MetalPixelRun == nil {
            let markerPrefix = c1133GridTemplateAreasProbeRequested ? "SPINON_C1133"
                : c1132GridNamedLinesProbeRequested ? "SPINON_C1132"
                : c1131GridRepeatProbeRequested ? "SPINON_C1131"
                : c1123GridAutoPlacementProbeRequested ? "SPINON_C1123"
                : c1122GridAutoPlacementProbeRequested ? "SPINON_C1122" : "SPINON_C111"
            let invalidSceneID = c1133GridTemplateAreasProbeRequested
                ? c1133GridTemplateAreasSceneID
                : c1131GridRepeatProbeRequested
                ? c1131GridRepeatSceneArgument
                : c1132GridNamedLinesProbeRequested ? c1132GridNamedLineSceneID
                : c11GridSceneArgument ?? "unknown"
            log("\(markerPrefix)_METAL_PIXEL_ERROR run_id=\(c111MetalPixelArgumentRunID) scene_id=\(invalidSceneID) reason=invalid-run-scene-or-samples")
            postStatus("실패 · \(c11GridMetalPixelStatusName) Metal 표본 실행 인수·장면·표본이 올바르지 않습니다")
            return
        }
        if c124StackingProbeRequested && c124MetalPixelCase == nil
            && ProcessInfo.processInfo.arguments.contains(where: {
                $0.hasPrefix("--spinon-c124-metal-surface-readback=")
            }) {
            log("SPINON_C124_METAL_PIXEL status=error reason=invalid-case-arguments")
            postStatus("실패 · C12.4 Metal 표본 case 인수가 올바르지 않습니다")
            return
        }
        let handle = c131BorderStripProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC13_1OverflowClipBorderFixture()
            : c131OverflowClipProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC13_1OverflowClipFixture()
            : c1133GridTemplateAreasProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC11_3_3GridTemplateAreasFixture()
            : c1132GridNamedLinesProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC11_3_2GridNamedLinesFixture()
            : c1131GridRepeatProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC11_3_1GridRepeatFixture()
            : c1123GridAutoPlacementProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC11_2_3GridAutoPlacementFixture()
            : c1122GridAutoPlacementProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC11_2_2GridAutoPlacementFixture()
            : c1121GridPlacementProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC11_2_1GridPlacementFixture()
            : c111MetalPixelRun != nil
            ? SpinonRunner.createRuntimeGpuHostWithC11_1GridSurfacePixelReadbackFixture()
            : c11GridFixtureRequested
            ? SpinonRunner.createRuntimeGpuHostWithC11_1GridFixture()
            : c124MetalPixelCase != nil
            ? SpinonRunner.createRuntimeGpuHostWithC12_4SurfaceReadbackFixture()
            : c1035PositionedFlexProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC10_3_5PositionedFlexFixture()
            : c123FixedPositionProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC12_3FixedFixture()
            : c124StackingProbeRequested || c124DepthSafetyProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC12_4StackingFixture()
            : c122AbsoluteBlockProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC12_2AbsoluteBlockFixture()
            : c121PositioningProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC12_1PositionFixture()
            : blockFormattingProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithBlockFormattingFixture()
            : blockPaintProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithBlockPaintFixture()
            : SpinonRunner.createRuntimeGpuHost(
                withRegisteredPropertiesFixture: registeredPropertiesProbeRequested
            )
        guard handle != 0 else {
            postStatus("실패 · V8 runtime host를 만들지 못했습니다")
            return
        }
        log("SPINON_C0410_HOST_RETURNED handle=\(handle)")
        stateLock.lock()
        hostHandle = handle
        hostLifetime.store(handle)
        let shouldContinue = !closing
        stateLock.unlock()
        log("SPINON_C0410_HOST_STORED continue=\(shouldContinue)")
        guard shouldContinue else { return }

        let environmentInputs = currentEnvironment()
        let viewportWidth: Float = c131BorderStripProbeRequested
            ? 400 : c131OverflowClipProbeRequested
            ? 320 : c122AbsoluteBlockProbeRequested
            || c123FixedPositionProbeRequested
            || c1122GridAutoPlacementProbeRequested
            || c1123GridAutoPlacementProbeRequested
            ? 360 : c1131GridRepeatProbeRequested || fixedSizeCssFixtureRequested ? 320 : environmentInputs.width
        let viewportHeight: Float = c131BorderStripProbeRequested
            ? 160 : c131OverflowClipProbeRequested
            ? 240 : c122AbsoluteBlockProbeRequested
            || c123FixedPositionProbeRequested
            || c1122GridAutoPlacementProbeRequested
            || c1123GridAutoPlacementProbeRequested
            ? 800 : c1133GridTemplateAreasProbeRequested
                || c1132GridNamedLinesProbeRequested || c1131GridRepeatProbeRequested
                || fixedSizeCssFixtureRequested ? 240 : environmentInputs.height
        log("SPINON_C0410_ENVIRONMENT_REQUEST width=\(viewportWidth) height=\(viewportHeight)")
        let environment = SpinonRunner.setRuntimeGpuEnvironment(
            handle, width: viewportWidth, height: viewportHeight,
            scale: c123FixedPositionProbeRequested || c1132GridNamedLinesProbeRequested
                || c1133GridTemplateAreasProbeRequested
                ? 1 : environmentInputs.scale,
            dark: environmentInputs.dark
        )
        guard environment?.hasPrefix("status=0 ") == true else {
            postStatus("실패 · \(environment ?? "환경 보고 없음")")
            return
        }
        log("SPINON_C0410_ENVIRONMENT \(environment ?? "")")

        var result: String? = nil
        if c131BorderStripProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC13_1OverflowClipBorderFixture(handle)
        } else if c131OverflowClipProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC13_1OverflowClipFixture(handle)
        } else if c1133GridTemplateAreasProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC11_3_3GridTemplateAreasFixture(
                handle, sceneID: c1133GridTemplateAreasSceneID,
                runID: c1133GridTemplateAreasRunID)
        } else if c1132GridNamedLinesProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC11_3_2GridNamedLinesFixture(
                handle, sceneID: c1132GridNamedLineSceneID, runID: c1132GridNamedLinesRunID)
        } else if c1131GridRepeatProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC11_3_1GridRepeatFixture(
                handle, sceneID: c1131GridRepeatSceneArgument, runID: c1131GridRepeatRunID)
        } else if c1123GridAutoPlacementProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC11_2_3GridAutoPlacementFixture(
                handle, sceneID: c1123GridSceneID, runID: c1123GridRunID)
        } else if c1122GridAutoPlacementProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC11_2_2GridAutoPlacementFixture(
                handle, sceneID: c1122GridSceneID, runID: c1122GridRunID)
        } else if c1121GridPlacementProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC11_2_1GridPlacementFixture(
                handle, sceneID: c1121GridSceneID, runID: c1121GridRunID)
        } else if c11GridFixtureRequested {
            result = SpinonRunner.evalRuntimeGpuC11_1GridSceneFixture(
                handle, sceneIndex: UInt32(c11GridSceneIndex ?? 0))
        } else if c1035PositionedFlexProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC10_3_5PositionedFlexFixture(handle)
        } else if c123FixedPositionProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC12_3FixedFixture(handle)
        } else if c124StackingProbeRequested || c124DepthSafetyProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC12_4StackingFixture(handle)
        } else if c122AbsoluteBlockProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC12_2AbsoluteBlockFixture(handle)
        } else if c121PositioningProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC12_1PositionFixture(handle)
        } else if flexBaselineProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFlexBaselineFixture(handle)
        } else if flexAlignmentProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFlexAlignmentFixture(handle)
        } else if flexOrderProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFlexOrderFixture(handle)
        } else if flexReverseProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFlexReverseFixture(handle)
        } else if flexDistributionProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFlexDistributionFixture(handle)
        } else if flexWrapProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFlexWrapFixture(handle)
        } else if flowRootProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFlowRootFixture(handle)
        } else if marginCollapseProbeRequested {
            result = SpinonRunner.evalRuntimeGpuMarginCollapseFixture(handle)
        } else if viewportUnitsProbeRequested {
            result = SpinonRunner.evalRuntimeGpuViewportUnitsFixture(handle)
        } else if blockFormattingProbeRequested {
            result = SpinonRunner.evalRuntimeGpuBlockFormattingFixture(handle)
        } else if blockPaintProbeRequested {
            result = SpinonRunner.evalRuntimeGpuBlockPaintFixture(handle)
        } else if aspectRatioProbeRequested {
            result = SpinonRunner.evalRuntimeGpuAspectRatioFixture(handle)
        } else if borderWidthProbeRequested {
            result = SpinonRunner.evalRuntimeGpuBorderWidthFixture(handle)
        } else if minMaxSizingProbeRequested {
            result = SpinonRunner.evalRuntimeGpuMinMaxSizingFixture(handle)
        } else if typedCssMathProbeRequested {
            result = SpinonRunner.evalRuntimeGpuTypedCssMathFixture(handle)
        } else if fontRelativeUnitsProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFontRelativeUnitsFixture(handle)
        } else if absoluteLengthsProbeRequested {
            result = SpinonRunner.evalRuntimeGpuAbsoluteLengthsFixture(handle)
        } else if spacingPercentagesProbeRequested {
            result = SpinonRunner.evalRuntimeGpuSpacingPercentagesFixture(handle)
        } else if percentageDimensionsProbeRequested {
            result = SpinonRunner.evalRuntimeGpuPercentageDimensionsFixture(handle)
        } else if incrementalRestyleProbeRequested {
            result = SpinonRunner.evalRuntimeGpuIncrementalRestyleFixture(handle)
        } else if registeredPropertiesProbeRequested {
            result = SpinonRunner.evalRuntimeGpuRegisteredPropertiesFixture(handle)
        } else if authorStylesheetsProbeRequested {
            result = SpinonRunner.evalRuntimeGpuAuthorStylesheetsFixture(handle)
        } else {
            result = SpinonRunner.evalRuntimeGpuFixture(handle)
        }
        if c1133GridTemplateAreasProbeRequested,
           c1133GridTemplateAreasSceneID == "basic-rectangles",
           result?.hasPrefix("status=0 ") == true {
            let rejectionHost = SpinonRunner.createRuntimeGpuHostWithC11_3_3GridTemplateAreasFixture()
            let rejection: String
            if rejectionHost == 0 {
                rejection = "status=-2 isolated rejection host creation failed"
            } else {
                let rejectionEnvironment = SpinonRunner.setRuntimeGpuEnvironment(
                    rejectionHost, width: 320, height: 240, scale: 1,
                    dark: environmentInputs.dark
                ) ?? "status=-1 missing-environment-report"
                rejection = rejectionEnvironment.hasPrefix("status=0 ")
                    ? SpinonRunner.evalRuntimeGpuC11_3_3GridTemplateAreasFixture(
                        rejectionHost,
                        sceneID: c1133GridTemplateAreasRejectSceneID,
                        runID: c1133GridTemplateAreasRunID
                    ) ?? "status=-1 missing-report"
                    : rejectionEnvironment
                SpinonRunner.freeRuntimeGpuHost(rejectionHost)
            }
            let expectedRejection = rejection.hasPrefix("status=-11 ")
                && rejection.contains("unsupported_computed_value")
                && rejection.contains("property=Some(\"grid-template-areas\")")
                && !rejection.contains("node_frames_css_px=[")
            log("SPINON_C1133_GRID_REJECTION run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasRejectSceneID) expected=\(expectedRejection) \(rejection)")
            let rejectionStatus = String(rejection.split(separator: " ", maxSplits: 1).first ?? "status=-1")
            let rejectionCode = rejection.contains("unsupported_computed_value")
                ? "unsupported_computed_value" : "missing"
            let rejectionProperty = rejection.contains("property=Some(\"grid-template-areas\")")
                ? "grid-template-areas" : "missing"
            let rejectionFrames = rejection.contains("node_frames_css_px=[") ? "present" : "absent"
            log("SPINON_C1133_GRID_REJECTION_SUMMARY run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasRejectSceneID) expected=\(expectedRejection) \(rejectionStatus) code=\(rejectionCode) property=\(rejectionProperty) frames=\(rejectionFrames)")
            guard expectedRejection else {
                postStatus("실패 · C11.3.3 area track 수 불일치 거부 계약 오류")
                return
            }
        }
        if c1132GridNamedLinesProbeRequested, result?.hasPrefix("status=0 ") == true {
            let rejectionHost = SpinonRunner.createRuntimeGpuHostWithC11_3_2GridNamedLinesFixture()
            let rejection: String
            if rejectionHost == 0 {
                rejection = "status=-2 isolated rejection host creation failed"
            } else {
                let rejectionEnvironment = SpinonRunner.setRuntimeGpuEnvironment(
                    rejectionHost, width: 320, height: 240, scale: 1,
                    dark: environmentInputs.dark
                ) ?? "status=-1 missing-environment-report"
                rejection = rejectionEnvironment.hasPrefix("status=0 ")
                    ? SpinonRunner.evalRuntimeGpuC11_3_2GridNamedLinesFixture(
                        rejectionHost,
                        sceneID: c1132GridNamedLineRejectSceneID,
                        runID: c1132GridNamedLinesRunID
                    ) ?? "status=-1 missing-report"
                    : rejectionEnvironment
                SpinonRunner.freeRuntimeGpuHost(rejectionHost)
            }
            let expectedRejection = rejection.hasPrefix("status=-11 ")
                && rejection.contains("unsupported_named_grid_line")
                && rejection.contains("property=Some(\"grid-column-start\")")
                && !rejection.contains("node_frames_css_px=[")
            log("SPINON_C1132_GRID_REJECTION run_id=\(c1132GridNamedLinesRunID) scene_id=\(c1132GridNamedLineRejectSceneID) expected=\(expectedRejection) \(rejection)")
            let rejectionStatus = String(rejection.split(separator: " ", maxSplits: 1).first ?? "status=-1")
            let rejectionCode = rejection.contains("unsupported_named_grid_line")
                ? "unsupported_named_grid_line" : "missing"
            let rejectionProperty = rejection.contains("property=Some(\"grid-column-start\")")
                ? "grid-column-start" : "missing"
            let rejectionFrames = rejection.contains("node_frames_css_px=[") ? "present" : "absent"
            log("SPINON_C1132_GRID_REJECTION_SUMMARY run_id=\(c1132GridNamedLinesRunID) scene_id=\(c1132GridNamedLineRejectSceneID) expected=\(expectedRejection) \(rejectionStatus) code=\(rejectionCode) property=\(rejectionProperty) frames=\(rejectionFrames)")
            guard expectedRejection else {
                postStatus("실패 · C11.3.2 implicit named line 거부 계약 불일치")
                return
            }
        }
        if c131BorderStripProbeRequested {
            let report = result ?? "status=-1 missing-report"
            log("SPINON_C131_BORDER_EVAL_RUN \(runtimeStatusSummary(report))")
            guard logC131BorderFixtureReport(report) else {
                postStatus("실패 · C13.1 border-strip frame·hit-test 검증 결과 불일치")
                return
            }
        } else if c131OverflowClipProbeRequested {
            let report = result ?? "status=-1 missing-report"
            log("SPINON_C131_EVAL_RUN \(runtimeStatusSummary(report))")
            guard logC131FixtureReport(report) else {
                postStatus("실패 · C13.1 frame·hit-test 검증 결과 불일치")
                return
            }
        } else if c1133GridTemplateAreasProbeRequested {
            log("SPINON_C1133_GRID_EVAL_RUN run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasSceneID) \(runtimeStatusSummary(result ?? "status=-1 missing-report"))")
            if let result, result.hasPrefix("status=0 ") {
                log("SPINON_C1133_GRID_SUMMARY run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasSceneID) \(runtimeStatusSummary(result))")
                logC1133GridTemplateAreaFrames(result)
            } else if let result {
                let code = result.contains("unsupported_computed_value")
                    ? "unsupported_computed_value"
                    : result.contains("unsupported_named_grid_line")
                        ? "unsupported_named_grid_line" : "missing"
                let property = result.contains("property=Some(\"grid-template-areas\")")
                    ? "grid-template-areas"
                    : result.contains("property=Some(\"grid-column-start\")")
                        ? "grid-column-start" : "missing"
                let nodeID = result.range(of: #"node=Some\(\d+\)"#, options: .regularExpression).map {
                    String(result[$0].dropFirst("node=Some(".count).dropLast())
                } ?? "missing"
                let expectedNodeID = c1133GridTemplateAreasSceneID == "area-track-count-mismatch"
                    ? "1" : c1133GridTemplateAreasSceneID == "unknown-area-implicit-tracks"
                        ? "2" : "missing"
                let status = String(result.split(separator: " ", maxSplits: 1).first ?? "status=-1")
                let expected = status == "status=-11"
                    && ((c1133GridTemplateAreasSceneID == "area-track-count-mismatch"
                        && code == "unsupported_computed_value" && property == "grid-template-areas"
                        && nodeID == expectedNodeID)
                    || (c1133GridTemplateAreasSceneID == "unknown-area-implicit-tracks"
                        && code == "unsupported_named_grid_line" && property == "grid-column-start"
                        && nodeID == expectedNodeID))
                let frames = result.contains("node_frames_css_px=[") ? "present" : "absent"
                log("SPINON_C1133_GRID_REJECTION_SUMMARY run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasSceneID) expected=\(expected) \(status) code=\(code) property=\(property) node_id=\(nodeID) frames=\(frames)")
            }
        } else if c1132GridNamedLinesProbeRequested {
            log("SPINON_C1132_GRID_EVAL_RUN run_id=\(c1132GridNamedLinesRunID) scene_id=\(c1132GridNamedLineSceneID) \(runtimeStatusSummary(result ?? "status=-1 missing-report"))")
            if let result, result.hasPrefix("status=0 ") {
                log("SPINON_C1132_GRID_SUMMARY run_id=\(c1132GridNamedLinesRunID) scene_id=\(c1132GridNamedLineSceneID) \(runtimeStatusSummary(result))")
                logC1132GridNamedLineFrames(result)
            }
        } else if c1131GridRepeatProbeRequested {
            log("SPINON_C1131_GRID_EVAL_RUN run_id=\(c1131GridRepeatRunID) scene_id=\(c1131GridRepeatSceneArgument) \(runtimeStatusSummary(result ?? "status=-1 missing-report"))")
        } else if c1123GridAutoPlacementProbeRequested || c1122GridAutoPlacementProbeRequested {
            let profile = c1123GridAutoPlacementProbeRequested ? "C1123" : "C1122"
            let runID = c1123GridAutoPlacementProbeRequested ? c1123GridRunID : c1122GridRunID
            let sceneID = c1123GridAutoPlacementProbeRequested ? c1123GridSceneID : c1122GridSceneID
            log("SPINON_\(profile)_GRID_EVAL_RUN run_id=\(runID) scene_id=\(sceneID) \(runtimeStatusSummary(result ?? "status=-1 missing-report"))")
            if let result, result.hasPrefix("status=0 ") {
                logGridAutoPlacementReadySummaryIfNeeded(
                    result, profile: profile, runID: runID, sceneID: sceneID)
            } else if let result,
                      result.hasPrefix("status=-12 "),
                      result.contains("op=eval status=0 ") {
                log("SPINON_\(profile)_GRID_SUPERSEDED run_id=\(runID) scene_id=\(sceneID) \(runtimeStatusSummary(result))")
            } else if let result {
                log("SPINON_\(profile)_GRID_SUMMARY run_id=\(runID) scene_id=\(sceneID) \(runtimeStatusSummary(result))")
                let marker = "CSS runtime 장면 생성 실패:"
                let diagnostic = result.range(of: marker).map { String(result[$0.lowerBound...]) }
                    ?? "diagnostic-marker-missing"
                log("SPINON_\(profile)_GRID_DIAGNOSTIC run_id=\(runID) scene_id=\(sceneID) \(diagnostic.prefix(768))")
            } else {
                log("SPINON_\(profile)_GRID_SUMMARY run_id=\(runID) scene_id=\(sceneID) status=-1 missing-report")
            }
        } else if c1121GridPlacementProbeRequested {
            log("SPINON_C1121_GRID_EVAL_RUN run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) \(runtimeStatusSummary(result ?? "status=-1 missing-report"))")
            if let result, result.hasPrefix("status=0 ") {
                logC1121GridReadySummaryIfNeeded(result)
            } else if let result,
                      result.hasPrefix("status=-12 "),
                      result.contains("op=eval status=0 ") {
                log("SPINON_C1121_GRID_SUPERSEDED run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) \(runtimeStatusSummary(result))")
            } else if let result {
                log("SPINON_C1121_GRID_SUMMARY run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) \(runtimeStatusSummary(result))")
                let marker = "CSS runtime 장면 생성 실패:"
                let diagnostic = result.range(of: marker).map { String(result[$0.lowerBound...]) }
                    ?? "diagnostic-marker-missing"
                log("SPINON_C1121_GRID_DIAGNOSTIC run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) \(diagnostic.prefix(768))")
            } else {
                log("SPINON_C1121_GRID_SUMMARY run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) status=-1 missing-report")
            }
        } else if c11GridFixtureRequested {
            let sceneId = c11GridSceneIndex.map { c11GridSceneIDs[$0] } ?? "invalid"
            log("SPINON_C111_GRID_SUMMARY scene_id=\(sceneId) \(runtimeStatusSummary(result ?? "status=-1 missing-report"))")
            if let result, result.hasPrefix("status=0 ") {
                logC111GridNodeFrames(result)
            }
        }
        let sceneWasSupersededAfterCommit = result?.hasPrefix("status=-12 ") == true
            && result?.contains("op=eval status=0 ") == true
        if c123FixedPositionProbeRequested,
           let result,
           !result.hasPrefix("status=0 ") {
            logC123FailureDetails(result)
        }
        if c123FixedPositionProbeRequested && sceneWasSupersededAfterCommit {
            postStatus("실패 · C12.3 초기 장면이 environment matrix 전에 교체됐습니다")
            log("SPINON_C123_MATRIX_FAILURE reason=initial-scene-superseded report=\(result ?? "missing")")
            return
        }
        guard result?.hasPrefix("status=0 ") == true || sceneWasSupersededAfterCommit else {
            postStatus("실패 · \(result ?? "JavaScript 보고 없음")")
            return
        }
        if let pixelCase = c124MetalPixelCase {
            let mutation = SpinonRunner.evalRuntimeGpuC12_4Source(
                handle, source: c124MetalPixelMutationSource(for: pixelCase)
            ) ?? "status=-1 missing-report"
            guard mutation.hasPrefix("status=0 ") else {
                log("SPINON_C124_METAL_PIXEL status=error run_id=\(pixelCase.runID) case=\(pixelCase.caseID) stage=fixture-mutation report=\(mutation)")
                postStatus("실패 · C12.4 reverse Flex fixture 변경 · \(mutation)")
                return
            }
            result = mutation
            log("SPINON_C124_METAL_FIXTURE run_id=\(pixelCase.runID) case=\(pixelCase.caseID) direction=\(pixelCase.direction) blue_order=\(pixelCase.blueOrder) red_order=\(pixelCase.redOrder) expected=\(pixelCase.expectedRGBA) \(runtimeStatusSummary(mutation))")
        }
        if c124DepthSafetyProbeRequested {
            guard let fixtureNodeCount = Int(reportField(result ?? "", "document_nodes=")) else {
                postStatus("실패 · C12.4 fixture 기본 노드 수를 읽지 못했습니다")
                return
            }
            let depthProbe = runC124DepthSafetyProbe(
                host: handle, fixtureNodeCount: fixtureNodeCount
            )
            guard depthProbe.passed else {
                postStatus("실패 · C12.4 깊이 검증 · \(depthProbe.failure)")
                return
            }
        }
        if flexReverseProbeRequested, result?.hasPrefix("status=0 ") == true {
            logC1031NodeFrames(result ?? "")
        }
        if flexAlignmentProbeRequested, result?.hasPrefix("status=0 ") == true {
            logC1033RuntimeSummary(result ?? "")
            logC1033NodeFrames(result ?? "")
        }
        if flexBaselineProbeRequested, result?.hasPrefix("status=0 ") == true {
            logC1034RuntimeSummary(result ?? "")
            logC1034NodeFrames(result ?? "")
        }
        if flexOrderProbeRequested, result?.hasPrefix("status=0 ") == true {
            logC1032NodeFrames(result ?? "")
        }
        if c121PositioningProbeRequested, result?.hasPrefix("status=0 ") == true {
            logC121NodeFrames("initial", result ?? "")
        }
        if c122AbsoluteBlockProbeRequested, result?.hasPrefix("status=0 ") == true {
            log("SPINON_C122_SUMMARY \(runtimeStatusSummary(result ?? ""))")
            logC122NodeFrames(result ?? "")
        }
        if c123FixedPositionProbeRequested, result?.hasPrefix("status=0 ") == true {
            guard let initialEnvironmentRevision = UInt64(
                reportField(environment ?? "", "environment_revision=")
            ) else {
                log("SPINON_C123_MATRIX_FAILURE state=initial missing_environment_revision")
                postStatus("실패 · C12.3 초기 환경 revision이 없습니다")
                return
            }
            let initialEvalRevision = reportField(result ?? "", "environment_revision=")
            guard initialEvalRevision == String(initialEnvironmentRevision) else {
                log("SPINON_C123_MATRIX_FAILURE state=initial expected_revision=\(initialEnvironmentRevision) actual_revision=\(initialEvalRevision)")
                postStatus("실패 · C12.3 초기 environment revision 불일치")
                return
            }
            log("SPINON_C123_SUMMARY state=matrix-360x800-dpr-1 \(runtimeStatusSummary(result ?? ""))")
            logC123NodeFrames("matrix-360x800-dpr-1", result ?? "")
            guard runC123EnvironmentMatrix(
                host: handle,
                dark: environmentInputs.dark,
                deviceScaleFactor: environmentInputs.scale,
                initialEnvironmentRevision: initialEnvironmentRevision
            ) else {
                postStatus("실패 · C12.3 environment matrix")
                return
            }
        }
        if c1035PositionedFlexProbeRequested, result?.hasPrefix("status=0 ") == true {
            log("SPINON_C1035_SUMMARY \(runtimeStatusSummary(result ?? ""))")
            logC1035NodeFrames(result ?? "")
        }
        if sceneWasSupersededAfterCommit {
            let scope = c1133GridTemplateAreasProbeRequested
                ? "SPINON_C1133_GRID"
                : c1132GridNamedLinesProbeRequested
                ? "SPINON_C1132_GRID"
                : c1131GridRepeatProbeRequested
                ? "SPINON_C1131_GRID"
                : c1121GridPlacementProbeRequested
                ? "SPINON_C1121_GRID"
                : c11GridFixtureRequested
                ? "SPINON_C111_GRID"
                : c1035PositionedFlexProbeRequested
                ? "SPINON_C1035"
                : c122AbsoluteBlockProbeRequested
                ? "SPINON_C122"
                : c124StackingProbeRequested
                ? "SPINON_C124"
                : c121PositioningProbeRequested
                ? "SPINON_C121"
                : flexAlignmentProbeRequested
                ? "SPINON_C1033"
                : flexBaselineProbeRequested
                ? "SPINON_C1034"
                : flexOrderProbeRequested
                ? "SPINON_C1032"
                : flexReverseProbeRequested
                ? "SPINON_C1031"
                : flexDistributionProbeRequested
                ? "SPINON_C102"
                : flexWrapProbeRequested
                ? "SPINON_C101"
                : marginCollapseProbeRequested
                ? "SPINON_C092"
                : viewportUnitsProbeRequested
                ? "SPINON_C066A"
                : blockFormattingProbeRequested
                ? "SPINON_C091"
                : blockPaintProbeRequested
                ? "SPINON_C08"
                : aspectRatioProbeRequested
                ? "SPINON_C073"
                : borderWidthProbeRequested
                ? "SPINON_C072"
                : minMaxSizingProbeRequested
                ? "SPINON_C071"
                : typedCssMathProbeRequested
                ? "SPINON_C065"
                : fontRelativeUnitsProbeRequested
                ? "SPINON_C064"
                : absoluteLengthsProbeRequested
                ? "SPINON_C063"
                : spacingPercentagesProbeRequested
                ? "SPINON_C062"
                : percentageDimensionsProbeRequested
                ? "SPINON_C061"
                : incrementalRestyleProbeRequested
                ? "SPINON_C054"
                : registeredPropertiesProbeRequested
                ? runtimeResultCacheProbeRequested ? "SPINON_C053" : "SPINON_C052"
                : authorStylesheetsProbeRequested ? "SPINON_C0411" : "SPINON_C0410"
            log("\(scope)_EVAL_SCENE_SUPERSEDED \(result ?? "")")
            postStatus("JavaScript 적용 완료 · 최신 CSS 장면 다시 계산 중")
        } else {
            let scope = c1133GridTemplateAreasProbeRequested
                ? "SPINON_C1133_GRID"
                : c1132GridNamedLinesProbeRequested
                ? "SPINON_C1132_GRID"
                : c1131GridRepeatProbeRequested
                ? "SPINON_C1131_GRID"
                : c1121GridPlacementProbeRequested
                ? "SPINON_C1121_GRID"
                : c11GridFixtureRequested
                ? "SPINON_C111_GRID"
                : c1035PositionedFlexProbeRequested
                ? "SPINON_C1035"
                : c122AbsoluteBlockProbeRequested
                ? "SPINON_C122"
                : c124StackingProbeRequested
                ? "SPINON_C124"
                : c121PositioningProbeRequested
                ? "SPINON_C121"
                : flexAlignmentProbeRequested
                ? "SPINON_C1033"
                : flexBaselineProbeRequested
                ? "SPINON_C1034"
                : flexOrderProbeRequested
                ? "SPINON_C1032"
                : flexReverseProbeRequested
                ? "SPINON_C1031"
                : flexDistributionProbeRequested
                ? "SPINON_C102"
                : flexWrapProbeRequested
                ? "SPINON_C101"
                : marginCollapseProbeRequested
                ? "SPINON_C092"
                : viewportUnitsProbeRequested
                ? "SPINON_C066A"
                : blockFormattingProbeRequested
                ? "SPINON_C091"
                : blockPaintProbeRequested
                ? "SPINON_C08"
                : aspectRatioProbeRequested
                ? "SPINON_C073"
                : borderWidthProbeRequested
                ? "SPINON_C072"
                : minMaxSizingProbeRequested
                ? "SPINON_C071"
                : typedCssMathProbeRequested
                ? "SPINON_C065"
                : fontRelativeUnitsProbeRequested
                ? "SPINON_C064"
                : absoluteLengthsProbeRequested
                ? "SPINON_C063"
                : spacingPercentagesProbeRequested
                ? "SPINON_C062"
                : percentageDimensionsProbeRequested
                ? "SPINON_C061"
                : incrementalRestyleProbeRequested
                ? "SPINON_C054"
                : registeredPropertiesProbeRequested
                ? runtimeResultCacheProbeRequested ? "SPINON_C053" : "SPINON_C052"
                : authorStylesheetsProbeRequested ? "SPINON_C0411" : "SPINON_C0410"
            log("\(scope)_EVAL \(result ?? "")")
            postStatus(runtimeStatusSummary(result ?? "runtime scene 준비 완료"))
            if c121PositioningProbeRequested {
                log("SPINON_C121_SUMMARY state=initial \(runtimeStatusSummary(result ?? ""))")
            } else if aspectRatioProbeRequested {
                log("SPINON_C073_SUMMARY \(runtimeStatusSummary(result ?? ""))")
            } else if borderWidthProbeRequested {
                log("SPINON_C072_SUMMARY \(runtimeStatusSummary(result ?? ""))")
            }
        }
        if c121PositioningProbeRequested, !sceneWasSupersededAfterCommit {
            for state in UInt32(1)...UInt32(2) {
                let stateName = state == 1 ? "target-relative" : "ancestor-relative"
                let stateRun = evalC121PositionState(host: handle, state: state)
                let stateReport = stateRun.report
                guard stateReport.hasPrefix("status=0 ") else {
                    postStatus("실패 · C12.1 \(stateName) · \(stateReport)")
                    return
                }
                log("SPINON_C121_SUMMARY state=\(stateName) attempts=\(stateRun.attempts) \(runtimeStatusSummary(stateReport))")
                logC121NodeFrames(stateName, stateReport)
                result = stateReport
            }
            postStatus(runtimeStatusSummary(result ?? "C12.1 mutation 보고 없음"))
        }
        if customPropertiesProbeRequested && !authorStylesheetsProbeRequested {
            result = SpinonRunner.evalRuntimeGpuCustomPropertiesFixture(handle)
            guard result?.hasPrefix("status=0 ") == true else {
                postStatus("실패 · C05 사용자 지정 속성 · \(result ?? "보고 없음")")
                return
            }
            log("SPINON_C051_EVAL \(result ?? "")")
            postStatus(runtimeStatusSummary(result ?? "C05 장면 준비 완료"))
        }
        if c124DepthSafetyProbeRequested {
            postStatus("C12.4 깊이 검증 통과 · 1024 통과, 1025 거부, 얕은 revision 복구")
        }
        DispatchQueue.main.async { [weak self] in
            guard let self else { return }
            let size = self.lastDrawableSize
            if size.width > 0 && size.height > 0 {
                self.ensureRenderer(width: Int(size.width), height: Int(size.height))
            }
            self.refreshEnvironment()
        }
    }

    private func runC124DepthSafetyProbe(
        host: UInt64,
        fixtureNodeCount: Int
    ) -> (passed: Bool, failure: String) {
        let maximumStarted = ProcessInfo.processInfo.systemUptime
        let maximum = SpinonRunner.evalRuntimeGpuC12_4Source(
            host, source: c124DepthBuildSource(maximumDepth: 1024)
        ) ?? "status=-1 missing-report"
        let maximumMillis = Int((ProcessInfo.processInfo.systemUptime - maximumStarted) * 1000)
        let maximumPassed = maximum.hasPrefix("status=0 ")
            && reportField(maximum, "layout=") == "ready"
            && reportField(maximum, "boxes=") == String(fixtureNodeCount + 1023)
            && reportField(maximum, "cascadeRecomputedStyleElements=")
                == String(fixtureNodeCount + 1023)
        log("SPINON_C124_DEPTH_STAGE stage=maximum expected=1024 passed=\(maximumPassed) elapsed_ms=\(maximumMillis) report_length=\(maximum.utf8.count)")
        guard maximumPassed else {
            log("SPINON_C124_DEPTH_SUMMARY status=failed stage=maximum report=\(maximum)")
            return (false, "1024 깊이를 계산하지 못했습니다")
        }

        let overflowStarted = ProcessInfo.processInfo.systemUptime
        let overflow = SpinonRunner.evalRuntimeGpuC12_4Source(
            host, source: c124DepthOverflowSource()
        ) ?? "status=-1 missing-report"
        let overflowMillis = Int((ProcessInfo.processInfo.systemUptime - overflowStarted) * 1000)
        let overflowPassed = !overflow.hasPrefix("status=0 ")
            && overflow.contains("tree_depth_exceeded")
            && overflow.contains("1025") && overflow.contains("1024")
        log("SPINON_C124_DEPTH_STAGE stage=overflow expected=reject-1025 passed=\(overflowPassed) elapsed_ms=\(overflowMillis) report=\(overflow)")
        guard overflowPassed else {
            log("SPINON_C124_DEPTH_SUMMARY status=failed stage=overflow")
            return (false, "1025 초과 요청이 지정된 오류로 거부되지 않았습니다")
        }

        let recoveryStarted = ProcessInfo.processInfo.systemUptime
        let recovery = SpinonRunner.evalRuntimeGpuC12_4Source(
            host, source: c124DepthRecoverySource()
        ) ?? "status=-1 missing-report"
        let recoveryMillis = Int((ProcessInfo.processInfo.systemUptime - recoveryStarted) * 1000)
        let recoveryPassed = recovery.hasPrefix("status=0 ")
            && reportField(recovery, "layout=") == "ready"
            && reportField(recovery, "boxes=") == String(fixtureNodeCount)
        log("SPINON_C124_DEPTH_STAGE stage=recovery expected=shallow-ready passed=\(recoveryPassed) elapsed_ms=\(recoveryMillis) report_length=\(recovery.utf8.count)")
        let passed = maximumPassed && overflowPassed && recoveryPassed
        log("SPINON_C124_DEPTH_SUMMARY status=\(passed ? "passed" : "failed") maximum=1024 overflow=1025 recovery=\(recoveryPassed) platform=ios-simulator")
        return (passed, recoveryPassed ? "" : "초과 거부 후 얕은 revision 계산이 복구되지 않았습니다")
    }

    private func c124DepthBuildSource(maximumDepth: Int) -> String {
        """
        (() => {
          var parent = globalThis.spinonC124NodeRefs['c124-root'];
          var root = null;
          for (let index = 0; index < \(maximumDepth - 1); index += 1) {
            const node = document.createElement('div');
            node.setAttribute('style', 'position:relative;z-index:0;width:1px;height:1px');
            parent.appendChild(node);
            if (root === null) root = node;
            parent = node;
          }
          globalThis.__spinonC124DepthRoot = root;
          globalThis.__spinonC124DepthLeaf = parent;
          return 'SPINON_C124_DEPTH_MAX=\(maximumDepth)';
        })()
        """
    }

    private func c124MetalPixelMutationSource(for pixelCase: C124MetalPixelCase) -> String {
        let column = pixelCase.direction.hasPrefix("column")
        let reverse = pixelCase.direction.hasSuffix("reverse")
        let itemSize = column
            ? "flex:0 0 60px;width:100px;height:60px"
            : "flex:0 0 80px;width:80px;height:60px"
        let overlapMargin: String
        switch (column, reverse) {
        case (false, false): overlapMargin = "margin-right:-40px"
        case (false, true): overlapMargin = "margin-left:-40px"
        case (true, false): overlapMargin = "margin-bottom:-40px"
        case (true, true): overlapMargin = "margin-top:-40px"
        }
        let containerStyle = "display:flex;position:absolute;left:20px;top:125px;width:100px;height:100px;flex-direction:\(pixelCase.direction);align-items:flex-start;justify-content:flex-start"
        let blueStyle = "position:static;\(itemSize);\(overlapMargin);order:\(pixelCase.blueOrder);background-color:#2866f6"
        let redStyle = "position:static;\(itemSize);\(overlapMargin);order:\(pixelCase.redOrder);background-color:#e5484d"
        return """
        (() => {
          const nodes = globalThis.spinonC124NodeRefs;
          if (!nodes) throw new Error('C12.4 surface readback fixture node map is missing');
          const targets = [
            ['c124-static-flex-container', '\(containerStyle)'],
            ['c124-static-flex-z-one', '\(blueStyle)'],
            ['c124-static-flex-z-two', '\(redStyle)'],
          ];
          for (const [id, style] of targets) {
            const element = nodes[id];
            if (!element) throw new Error('C12.4 surface readback target missing: ' + id);
            element.setAttribute('style', style);
          }
          return { direction: '\(pixelCase.direction)', blueOrder: \(pixelCase.blueOrder), redOrder: \(pixelCase.redOrder) };
        })()
        """
    }

    private func c124DepthOverflowSource() -> String {
        """
        (() => {
          const node = document.createElement('div');
          node.setAttribute('style', 'position:relative;z-index:0;width:1px;height:1px');
          globalThis.__spinonC124DepthLeaf.appendChild(node);
          globalThis.__spinonC124DepthOverflow = node;
          return 'SPINON_C124_DEPTH_OVERFLOW=1025';
        })()
        """
    }

    private func c124DepthRecoverySource() -> String {
        """
        (() => {
          globalThis.__spinonC124DepthLeaf.removeChild(globalThis.__spinonC124DepthOverflow);
          globalThis.spinonC124NodeRefs['c124-root'].removeChild(
            globalThis.__spinonC124DepthRoot
          );
          delete globalThis.__spinonC124DepthOverflow;
          delete globalThis.__spinonC124DepthRoot;
          delete globalThis.__spinonC124DepthLeaf;
          return 'SPINON_C124_DEPTH_RECOVERED';
        })()
        """
    }

    @objc private func evaluateCustomPropertiesFixture() {
        enqueueRuntime { [weak self] in
            guard let self else { return }
            stateLock.lock()
            let handle = closing ? 0 : hostHandle
            stateLock.unlock()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuCustomPropertiesFixture(handle)
            log("SPINON_C051_EVAL \(report ?? "보고 없음")")
            postStatus(runtimeStatusSummary(report ?? "사용자 지정 속성 실행 결과가 없습니다"))
            guard report?.hasPrefix("status=0 ") == true else { return }
            requestDraw()
        }
    }

    @objc private func evaluateRegisteredPropertiesFixture() {
        enqueueRuntime { [weak self] in
            guard let self else { return }
            stateLock.lock()
            let handle = closing ? 0 : hostHandle
            stateLock.unlock()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuRegisteredPropertiesFixture(handle)
            log("SPINON_C052_EVAL \(report ?? "보고 없음")")
            postStatus(runtimeStatusSummary(report ?? "등록 사용자 지정 속성 실행 결과가 없습니다"))
            guard report?.hasPrefix("status=0 ") == true else { return }
            requestDraw()
        }
    }

    @objc private func evaluateRuntimeResultCacheFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuRuntimeResultCacheFixture(handle)
            self.log("SPINON_C053_EVAL \(report ?? "runtime cache fixture 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "runtime cache fixture 보고 없음"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    @objc private func evaluateIncrementalRestyleFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuIncrementalRestyleFixture(handle)
            self.log("SPINON_C054_EVAL \(report ?? "incremental restyle fixture 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "incremental restyle 결과가 없습니다"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    @objc private func evaluatePercentageDimensionsFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuPercentageDimensionsFixture(handle)
            self.log("SPINON_C061_EVAL \(report ?? "percentage fixture 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "percentage fixture 보고 없음"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    @objc private func evaluateSpacingPercentagesFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuSpacingPercentagesFixture(handle)
            self.log("SPINON_C062_EVAL \(report ?? "spacing percentage 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "spacing percentage 보고 없음"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    @objc private func evaluateAbsoluteLengthsFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuAbsoluteLengthsFixture(handle)
            self.log("SPINON_C063_EVAL \(report ?? "absolute length fixture 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "absolute length fixture 보고 없음"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    @objc private func evaluateFontRelativeUnitsFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuFontRelativeUnitsFixture(handle)
            self.log("SPINON_C064_EVAL \(report ?? "font-relative fixture 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "font-relative fixture 보고 없음"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    @objc private func evaluateTypedCssMathFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuTypedCssMathFixture(handle)
            self.log("SPINON_C065_EVAL \(report ?? "typed CSS math fixture 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "typed CSS math fixture 보고 없음"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    private func ensureRenderer(width: Int, height: Int) {
        dispatchPrecondition(condition: .onQueue(.main))
        guard width > 0, height > 0 else { return }
        stateLock.lock()
        guard hostHandle != 0, !rendererReady, !rendererCreationPending, !closing else {
            stateLock.unlock()
            return
        }
        rendererCreationPending = true
        let handle = hostHandle
        let generation = surfaceGeneration
        let viewPointer = Unmanaged.passUnretained(canvasView).toOpaque()
        stateLock.unlock()

        let surfaceReport = SpinonRunner.prepareRuntimeGpuWgpuSurface(
            handle, view: viewPointer
        )
        log("SPINON_C0410_SURFACE_PREPARE \(surfaceReport ?? "")")
        guard surfaceReport?.hasPrefix("status=0 ") == true else {
            stateLock.lock()
            rendererCreationPending = false
            stateLock.unlock()
            postStatus(surfaceReport ?? "UIKit surface 준비 보고 없음")
            return
        }

        enqueueRender { [weak self] in
            guard let self else { return }
            guard self.isCurrentSurfaceGeneration(generation) else {
                SpinonRunner.destroyRuntimeGpuRenderer(handle)
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
                self.retryRendererCreation()
                return
            }
            let report = SpinonRunner.createRuntimeGpuWgpu(
                handle, width: UInt32(width), height: UInt32(height)
            )
            self.log("SPINON_C0410_RENDERER \(report ?? "")")
            guard report?.hasPrefix("status=0 ") == true else {
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
                self.postStatus(report ?? "WGPU renderer 보고 없음")
                return
            }
            guard self.isCurrentSurfaceGeneration(generation) else {
                SpinonRunner.destroyRuntimeGpuRenderer(handle)
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
                self.retryRendererCreation()
                return
            }
            DispatchQueue.main.async { [weak self] in
                self?.configureRendererOnMain(
                    handle: handle, generation: generation, rendererReport: report ?? ""
                )
            }
        }
    }

    private func configureRendererOnMain(
        handle: UInt64, generation: UInt64, rendererReport: String
    ) {
        dispatchPrecondition(condition: .onQueue(.main))
        guard isCurrentSurfaceGeneration(generation) else {
            enqueueRender { [weak self] in
                SpinonRunner.destroyRuntimeGpuRenderer(handle)
                guard let self else { return }
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
                self.retryRendererCreation()
            }
            return
        }

        let configureReport = SpinonRunner.configureRuntimeGpuWgpuSurface(handle)
        let configureThread = Thread.isMainThread ? "main" : "other"
        log("SPINON_C0410_SURFACE_CONFIGURE thread=\(configureThread) \(configureReport ?? "")")
        let ready = configureReport?.hasPrefix("status=0 ") == true
        stateLock.lock()
        let stale = closing || surfaceGeneration != generation
        rendererReady = ready && !stale
        rendererSurfaceGeneration = ready && !stale ? generation : nil
        if ready && !stale { rendererCreationPending = false }
        stateLock.unlock()
        if ready && stale {
            enqueueRender { [weak self] in
                SpinonRunner.destroyRuntimeGpuRenderer(handle)
                guard let self else { return }
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
                self.retryRendererCreation()
            }
            return
        }
        guard ready else {
            postStatus(configureReport ?? "UIKit surface 구성 보고 없음")
            enqueueRender { [weak self] in
                SpinonRunner.destroyRuntimeGpuRenderer(handle)
                guard let self else { return }
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
            }
            return
        }
        postStatus("\(rendererReport) · \(configureReport ?? "surface configured")")
        refreshEnvironment()
    }

    private func resizeRenderer(to size: CGSize) {
        dispatchPrecondition(condition: .onQueue(.main))
        guard size.width > 0, size.height > 0 else { return }
        stateLock.lock()
        guard rendererReady, !surfaceConfigurationPending, !closing else {
            stateLock.unlock()
            return
        }
        surfaceConfigurationPending = true
        rendererSurfaceGeneration = nil
        let generation = surfaceGeneration
        stateLock.unlock()
        enqueueRender { [weak self] in
            guard let self else { return }
            DispatchQueue.main.async { [weak self] in
                self?.configureResizeOnMain(size: size, generation: generation)
            }
        }
    }

    private func configureResizeOnMain(size: CGSize, generation: UInt64) {
        dispatchPrecondition(condition: .onQueue(.main))
        stateLock.lock()
        let current = !closing && surfaceGeneration == generation
        let handle = hostHandle
        stateLock.unlock()
        guard current, handle != 0 else {
            finishSurfaceResize(generation: generation, success: false)
            return
        }

        let result = SpinonRunner.resizeRuntimeGpuWgpu(
            handle, width: UInt32(size.width), height: UInt32(size.height)
        )
        log("SPINON_C0410_RESIZE status=\(result) thread=main")
        guard result == 0 else {
            finishSurfaceResize(generation: generation, success: false)
            postStatus("실패 · WGPU 표면 크기 변경 status=\(result)")
            return
        }
        finishSurfaceResize(generation: generation, success: true)
    }

    private func finishSurfaceResize(generation: UInt64, success: Bool) {
        stateLock.lock()
        let current = !closing && surfaceGeneration == generation
        surfaceConfigurationPending = false
        if success && current { rendererSurfaceGeneration = generation }
        let retrySize = lastDrawableSize
        stateLock.unlock()
        guard !isClosing else { return }
        guard current else {
            if rendererIsReady, retrySize.width > 0, retrySize.height > 0 {
                resizeRenderer(to: retrySize)
            }
            return
        }
        guard success else { return }
        scheduleLatestPresentation()
    }

    private func refreshEnvironment() {
        guard beginPresentationUpdate() != 0 else { return }
        scheduleLatestPresentation()
    }

    private func scheduleLatestPresentation() {
        guard let runtimePresentationLane else { return }
        if case .closed = runtimePresentationLane.request() {
            postStatus("실패 · 닫힌 runtime presentation lane에 요청했습니다")
        }
    }

    private func applyLatestEnvironment() throws {
        let generation: UInt64
        let sequence: UInt64
        let handle: UInt64
        let environmentInputs: (width: Float, height: Float, scale: Float, dark: Bool)
        stateLock.lock()
        guard !closing, hostHandle != 0 else {
            stateLock.unlock()
            return
        }
        generation = surfaceGeneration
        sequence = presentationSequence
        handle = hostHandle
        environmentInputs = (viewportWidthCssPx, viewportHeightCssPx, displayScale, darkMode)
        stateLock.unlock()
        guard isCurrentPresentation(generation: generation, sequence: sequence) else { return }
        let report = SpinonRunner.setRuntimeGpuEnvironment(
            handle, width: environmentInputs.width, height: environmentInputs.height,
            scale: environmentInputs.scale, dark: environmentInputs.dark
        )
        log("SPINON_C0410_ENVIRONMENT viewport=\(environmentInputs.width)x\(environmentInputs.height) scale=\(environmentInputs.scale) dark=\(environmentInputs.dark) sequence=\(sequence) \(report ?? "")")
        if c1131GridRepeatProbeRequested, let report {
            log("SPINON_C1131_GRID_ENVIRONMENT_RESULT run_id=\(c1131GridRepeatRunID) scene_id=\(c1131GridRepeatSceneArgument) \(runtimeStatusSummary(report))")
        } else if c1123GridAutoPlacementProbeRequested, let report {
            logGridAutoPlacementReadySummaryIfNeeded(
                report, profile: "C1123", runID: c1123GridRunID, sceneID: c1123GridSceneID)
        } else if c1122GridAutoPlacementProbeRequested, let report {
            logGridAutoPlacementReadySummaryIfNeeded(
                report, profile: "C1122", runID: c1122GridRunID, sceneID: c1122GridSceneID)
        } else if c1121GridPlacementProbeRequested, let report {
            logC1121GridReadySummaryIfNeeded(report)
        }
        postStatus(runtimeStatusSummary(report ?? "환경 보고 없음"))
        guard report?.hasPrefix("status=0 ") == true else {
            throw C0410LatestTaskLaneError.operationFailed(report ?? "환경 갱신 보고 없음")
        }
        if report?.hasPrefix("status=0 ") == true
            && isCurrentPresentation(generation: generation, sequence: sequence) {
            requestDraw()
        }
    }

    private func requestDraw() {
        stateLock.lock()
        guard !closing, rendererReady, rendererSurfaceGeneration == surfaceGeneration else {
            stateLock.unlock()
            return
        }
        stateLock.unlock()
        guard let renderDrawLane else { return }
        if case .closed = renderDrawLane.request() {
            postStatus("실패 · 닫힌 render draw lane에 요청했습니다")
        }
    }

    private func drawLatestIfReady() throws {
        stateLock.lock()
        guard !closing, rendererReady, rendererSurfaceGeneration == surfaceGeneration,
              hostHandle != 0, let generation = rendererSurfaceGeneration else {
            stateLock.unlock()
            return
        }
        let handle = hostHandle
        stateLock.unlock()
        if shutdownProbeRequested && !shutdownProbeStarted {
            shutdownProbeStarted = true
            runShutdownProbeGate()
        }
        guard canUseSurfaceGeneration(generation) else { return }
        if let pixelRun = c111MetalPixelRun, !c111MetalPixelRequestIssued {
            let pixelMarker = "SPINON_\(pixelRun.markerPrefix)_METAL_PIXEL"
            stateLock.lock()
            let drawableSize = c111MetalDrawableSize
            let viewportWidth = viewportWidthCssPx
            let viewportHeight = viewportHeightCssPx
            stateLock.unlock()
            guard viewportWidth == 320, viewportHeight == 240,
                  drawableSize.width > 0, drawableSize.height > 0 else {
                let detail = "C11 Metal pixel viewport/drawable이 준비되지 않았습니다"
                log("\(pixelMarker)_ERROR run_id=\(pixelRun.runID) stage=surface-size")
                c111MetalPixelReadbackFinished = true
                throw C0410LatestTaskLaneError.operationFailed(detail)
            }
            let scaleX = Float(drawableSize.width) / viewportWidth
            let scaleY = Float(drawableSize.height) / viewportHeight
            guard scaleX.isFinite, scaleY.isFinite, scaleX > 0,
                  abs(scaleX - scaleY) < 0.000001 else {
                let detail = "C11 Metal pixel drawable scale의 가로·세로 비율이 다릅니다"
                log("\(pixelMarker)_ERROR run_id=\(pixelRun.runID) stage=surface-scale")
                c111MetalPixelReadbackFinished = true
                throw C0410LatestTaskLaneError.operationFailed(detail)
            }
            var coordinates: [UInt32] = []
            coordinates.reserveCapacity(pixelRun.samples.count * 2)
            for sample in pixelRun.samples {
                let x = floor((Float(sample.xCssPx) + 0.5) * scaleX)
                let y = floor((Float(sample.yCssPx) + 0.5) * scaleY)
                guard x >= 0, y >= 0, x < Float(drawableSize.width),
                      y < Float(drawableSize.height) else {
                    let detail = "C11 Metal pixel 표본이 drawable 범위를 벗어났습니다: \(sample.id)"
                    log("\(pixelMarker)_ERROR run_id=\(pixelRun.runID) stage=sample-bounds sample=\(sample.id)")
                    c111MetalPixelReadbackFinished = true
                    throw C0410LatestTaskLaneError.operationFailed(detail)
                }
                coordinates.append(UInt32(x))
                coordinates.append(UInt32(y))
            }
            let request = coordinates.withUnsafeBufferPointer { buffer in
                if pixelRun.markerPrefix == "C1133" {
                    SpinonRunner.requestRuntimeGpuC11_3_3GridSurfacePixels(
                        forTest: handle,
                        sampleXY: buffer.baseAddress,
                        sampleCount: UInt(coordinates.count / 2)
                    )
                } else if pixelRun.markerPrefix == "C1132" {
                    SpinonRunner.requestRuntimeGpuC11_3_2GridSurfacePixels(
                        forTest: handle,
                        sampleXY: buffer.baseAddress,
                        sampleCount: UInt(coordinates.count / 2)
                    )
                } else if pixelRun.markerPrefix == "C1131" {
                    SpinonRunner.requestRuntimeGpuC11_3_1GridSurfacePixels(
                        forTest: handle,
                        sampleXY: buffer.baseAddress,
                        sampleCount: UInt(coordinates.count / 2)
                    )
                } else if pixelRun.markerPrefix == "C1123" {
                    SpinonRunner.requestRuntimeGpuC11_2_3GridSurfacePixels(
                        forTest: handle,
                        sampleXY: buffer.baseAddress,
                        sampleCount: UInt(coordinates.count / 2)
                    )
                } else if pixelRun.markerPrefix == "C1122" {
                    SpinonRunner.requestRuntimeGpuC11_2_2GridSurfacePixels(
                        forTest: handle,
                        sampleXY: buffer.baseAddress,
                        sampleCount: UInt(coordinates.count / 2)
                    )
                } else {
                    SpinonRunner.requestRuntimeGpuC11_1GridSurfacePixels(
                        forTest: handle,
                        sampleXY: buffer.baseAddress,
                        sampleCount: UInt(coordinates.count / 2)
                    )
                }
            } ?? "status=-1 missing-request-report"
            guard request.hasPrefix("status=0 ") else {
                log("\(pixelMarker)_ERROR run_id=\(pixelRun.runID) stage=request report=\(request)")
                c111MetalPixelReadbackFinished = true
                throw C0410LatestTaskLaneError.operationFailed(request)
            }
            c111MetalPixelRequestIssued = true
            log("\(pixelMarker)_REQUESTED run_id=\(pixelRun.runID) scene=\(pixelRun.sceneID) generation=\(generation) drawable=\(Int(drawableSize.width))x\(Int(drawableSize.height)) scale_x=\(scaleX) scale_y=\(scaleY) samples=\(pixelRun.samples.count)")
        }
        if let pixelCase = c124MetalPixelCase, !c124MetalPixelReadbackFinished {
            stateLock.lock()
            let scale = displayScale
            stateLock.unlock()
            let pixelX = UInt32(((Float(pixelCase.cssSample.x) + 0.5) * scale).rounded(.down))
            let pixelY = UInt32(((Float(pixelCase.cssSample.y) + 0.5) * scale).rounded(.down))
            let request = SpinonRunner.requestRuntimeGpuSurfacePixelReadback(
                forTest: handle,
                x: pixelX, y: pixelY
            ) ?? "status=-1 missing-request-report"
            guard request.hasPrefix("status=0 ") else {
                log("SPINON_C124_METAL_PIXEL status=error run_id=\(pixelCase.runID) case=\(pixelCase.caseID) stage=request report=\(request)")
                c124MetalPixelReadbackFinished = true
                throw C0410LatestTaskLaneError.operationFailed(request)
            }
            log("SPINON_C124_METAL_PIXEL_REQUESTED run_id=\(pixelCase.runID) case=\(pixelCase.caseID) css_x=\(pixelCase.cssSample.x) css_y=\(pixelCase.cssSample.y) scale=\(scale) pixel_x=\(pixelX) pixel_y=\(pixelY)")
        }
        let result = SpinonRunner.drawRuntimeGpuWgpu(handle)
        log("SPINON_C0410_DRAW generation=\(generation) \(result ?? "")")
        if result?.hasPrefix("status=-12 ") == true {
            log("SPINON_C0410_DRAW_SUPERSEDED generation=\(generation)")
            requestDraw()
            return
        }
        if result?.hasPrefix("status=0 ") != true {
                postStatus("실패 · \(result ?? "WGPU draw 보고 없음")")
            throw C0410LatestTaskLaneError.operationFailed(result ?? "WGPU draw 실패 보고 없음")
        }
        if let pixelRun = c111MetalPixelRun, !c111MetalPixelReadbackFinished {
            try verifyC111MetalPixelReadback(
                result: result ?? "",
                run: pixelRun,
                generation: generation
            )
        }
        if let pixelCase = c124MetalPixelCase, !c124MetalPixelReadbackFinished {
            let sampleXValue = reportField(result ?? "", "surface_pixel_x=")
            let sampleYValue = reportField(result ?? "", "surface_pixel_y=")
            let rgba = reportField(result ?? "", "surface_pixel_rgba=")
            let surfaceSize = reportField(result ?? "", "surface_texture=")
            guard let sampleX = UInt32(sampleXValue),
                  let sampleY = UInt32(sampleYValue),
                  !rgba.isEmpty, !surfaceSize.isEmpty else {
                log("SPINON_C124_METAL_PIXEL status=error run_id=\(pixelCase.runID) case=\(pixelCase.caseID) stage=missing-sample report=\(result ?? "missing")")
                c124MetalPixelReadbackFinished = true
                throw C0410LatestTaskLaneError.operationFailed("Metal surface pixel 표본이 draw 보고에 없습니다")
            }
            stateLock.lock()
            let scale = displayScale
            stateLock.unlock()
            let passed = rgba == pixelCase.expectedRGBA
                && sampleX == UInt32(((Float(pixelCase.cssSample.x) + 0.5) * scale).rounded(.down))
                && sampleY == UInt32(((Float(pixelCase.cssSample.y) + 0.5) * scale).rounded(.down))
            c124MetalPixelReadbackFinished = true
            let status = passed ? "ok" : "error"
            log("SPINON_C124_METAL_PIXEL status=\(status) run_id=\(pixelCase.runID) case=\(pixelCase.caseID) direction=\(pixelCase.direction) blue_order=\(pixelCase.blueOrder) red_order=\(pixelCase.redOrder) css_x=\(pixelCase.cssSample.x) css_y=\(pixelCase.cssSample.y) scale=\(scale) pixel_x=\(sampleX) pixel_y=\(sampleY) rgba=\(rgba) expected=\(pixelCase.expectedRGBA) surface=\(surfaceSize) renderer=metal-surface")
            guard passed else {
                postStatus("실패 · C12.4 Metal surface 색상·좌표 비교")
                throw C0410LatestTaskLaneError.operationFailed("Metal surface sample이 고정 Chrome 표본과 다릅니다")
            }
            postStatus("C12.4 Metal surface 표본 통과 · \(pixelCase.caseID)")
        }
        if failureProbeRequested && !failureProbeStarted {
            failureProbeStarted = true
            runFailureRecoveryProbe(handle: handle, baseline: result ?? "")
        }
    }

    private func verifyC111MetalPixelReadback(
        result: String,
        run: C111MetalPixelRun,
        generation: UInt64
    ) throws {
        let pixelMarker = "SPINON_\(run.markerPrefix)_METAL_PIXEL"
        func failure(_ stage: String, _ detail: String) -> C0410LatestTaskLaneError {
            c111MetalPixelReadbackFinished = true
            log("\(pixelMarker)_ERROR run_id=\(run.runID) stage=\(stage) detail=\(detail)")
            postStatus("실패 · \(c11GridMetalPixelStatusName) Metal surface 표본 \(stage)")
            return .operationFailed(detail)
        }

        let surface = reportField(result, "surface_texture=")
            .split(separator: "x").compactMap { UInt32($0) }
        guard surface.count == 2, surface[0] > 0, surface[1] > 0 else {
            throw failure("surface-size", "WGPU draw report에 실제 surface 크기가 없습니다")
        }
        let viewport = (width: UInt32(320), height: UInt32(240))
        stateLock.lock()
        let drawableSize = c111MetalDrawableSize
        let viewportWidth = UInt32(viewportWidthCssPx)
        let viewportHeight = UInt32(viewportHeightCssPx)
        stateLock.unlock()
        guard viewportWidth == viewport.width, viewportHeight == viewport.height,
              drawableSize.width == CGFloat(surface[0]),
              drawableSize.height == CGFloat(surface[1]) else {
            throw failure("surface-generation", "viewport 또는 surface drawable 크기가 요청과 달라졌습니다")
        }
        let scaleX = Float(surface[0]) / Float(viewportWidth)
        let scaleY = Float(surface[1]) / Float(viewportHeight)
        guard scaleX.isFinite, scaleY.isFinite, scaleX > 0,
              abs(scaleX - scaleY) < 0.000001 else {
            throw failure("surface-scale", "drawable 가로·세로 pixel scale이 일치하지 않습니다")
        }
        let documentRevision = reportField(result, "document_revision=")
        let renderTreeRevision = reportField(result, "render_tree_revision=")
        let environmentRevision = reportField(result, "environment_revision=")
        let layoutGeneration = reportField(result, "generation=")
        guard UInt64(documentRevision) != nil, UInt64(renderTreeRevision) != nil,
              UInt64(environmentRevision) != nil, UInt64(layoutGeneration) != nil else {
            throw failure("revision", "draw report에 문서·트리·환경 revision이 없습니다")
        }
        guard let encodedSamples = reportField(result, "surface_pixel_samples_json=").data(using: .utf8),
              let values = try? JSONSerialization.jsonObject(with: encodedSamples) as? [[String: Any]],
              values.count == run.samples.count else {
            throw failure("sample-count", "draw report의 surface sample 개수가 실행 fixture와 다릅니다")
        }

        let pid = ProcessInfo.processInfo.processIdentifier
        let drawableText = "\(surface[0])x\(surface[1])"
        log("\(pixelMarker)_BEGIN run_id=\(run.runID) pid=\(pid) surface_generation=\(generation) layout_generation=\(layoutGeneration) document_revision=\(documentRevision) render_tree_revision=\(renderTreeRevision) environment_revision=\(environmentRevision) scene=\(run.sceneID) viewport=\(viewport.width)x\(viewport.height) drawable=\(drawableText)")

        var matchedCount = 0
        var records: [(String, UInt32, UInt32, [UInt32], [UInt32], Bool)] = []
        for (index, expected) in run.samples.enumerated() {
            guard let value = values[index] as? [String: Any],
                  let xNumber = value["x"] as? NSNumber,
                  let yNumber = value["y"] as? NSNumber,
                  let channels = value["rgba"] as? [NSNumber], channels.count == 4 else {
                throw failure("sample-shape", "surface 표본 JSON 형태가 올바르지 않습니다: \(expected.id)")
            }
            let actualX = xNumber.uint32Value
            let actualY = yNumber.uint32Value
            let actualRGBA = channels.map(\.uint32Value)
            let expectedX = UInt32(floor((Float(expected.xCssPx) + 0.5) * scaleX))
            let expectedY = UInt32(floor((Float(expected.yCssPx) + 0.5) * scaleY))
            let exact = actualX == expectedX && actualY == expectedY
                && actualRGBA == expected.rgba
            if exact { matchedCount += 1 }
            records.append((expected.id, actualX, actualY, actualRGBA, expected.rgba, exact))
        }

        for (id, x, y, actual, expected, exact) in records {
            let actualText = actual.map(String.init).joined(separator: ",")
            let expectedText = expected.map(String.init).joined(separator: ",")
            log("\(pixelMarker)_SAMPLE run_id=\(run.runID) pid=\(pid) surface_generation=\(generation) sample=\(id) x=\(x) y=\(y) rgba=\(actualText) expected=\(expectedText) exact=\(exact)")
        }
        let status = matchedCount == run.samples.count ? "matched" : "mismatch"
        log("\(pixelMarker)_SUMMARY status=\(status) run_id=\(run.runID) pid=\(pid) surface_generation=\(generation) document_revision=\(documentRevision) render_tree_revision=\(renderTreeRevision) environment_revision=\(environmentRevision) scene=\(run.sceneID) drawable=\(drawableText) scale_x=\(scaleX) scale_y=\(scaleY) expected_samples=\(run.samples.count) observed_samples=\(values.count) matched_samples=\(matchedCount)")
        c111MetalPixelReadbackFinished = true
        guard matchedCount == run.samples.count else {
            throw failure("pixel-mismatch", "실제 Metal surface 표본이 고정 Chrome RGBA와 다릅니다")
        }
        postStatus("\(c11GridMetalPixelStatusName) Metal surface 표본 통과 · \(matchedCount)/\(run.samples.count)")
    }

    private func runShutdownProbeGate() {
        let release = DispatchSemaphore(value: 0)
        for _ in 0..<10_000 { _ = renderDrawLane?.request() }
        let snapshot = renderDrawLane?.snapshot()
        log("SPINON_C0410_SHUTDOWN_PROBE_PENDING requests=10000 "
            + "scheduled=\(snapshot?.scheduled ?? false) "
            + "dirty=\(snapshot?.dirty ?? false) pending=\(snapshot?.pendingDrains ?? -1)")
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) { [weak self] in
            self?.beginShutdown()
        }
        DispatchQueue.global(qos: .utility).asyncAfter(deadline: .now() + 0.35) {
            release.signal()
        }
        release.wait()
        log("SPINON_C0410_SHUTDOWN_PROBE_GATE_RELEASED")
    }

    private func runFailureRecoveryProbe(handle: UInt64, baseline: String) {
#if SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE_1
        let armed = SpinonRunner.injectNextRuntimeGpuDrawFailure(handle)
        log("SPINON_C0410_FAILURE_PROBE_ARM \(armed ?? "")")
        guard armed?.hasPrefix("status=0 ") == true else {
            log("SPINON_C0410_FAILURE_PROBE_FAILED stage=arm")
            postStatus("실패 · draw 오류 주입을 설정하지 못했습니다")
            return
        }

        let expectedFailure = SpinonRunner.drawRuntimeGpuWgpu(handle)
        log("SPINON_C0410_FAILURE_PROBE_EXPECTED_ERROR \(expectedFailure ?? "")")
        guard expectedFailure?.hasPrefix("status=0 ") != true,
              expectedFailure?.contains("시험용으로 다음 draw를 실패") == true else {
            log("SPINON_C0410_FAILURE_PROBE_FAILED stage=expected-error")
            postStatus("실패 · 시험용 draw 오류가 예상대로 전달되지 않았습니다")
            return
        }

        let recovered = SpinonRunner.drawRuntimeGpuWgpu(handle)
        let baselineRevision = reportField(baseline, "environment_revision=")
        let recoveredRevision = reportField(recovered ?? "", "environment_revision=")
        let sameScene = !baselineRevision.isEmpty && baselineRevision == recoveredRevision
        let passed = recovered?.hasPrefix("status=0 ") == true && sameScene
        log("SPINON_C0410_FAILURE_PROBE_RECOVERY passed=\(passed) "
            + "baseline_revision=\(baselineRevision) recovered_revision=\(recoveredRevision) "
            + "\(recovered ?? "")")
        if passed {
            postStatus("draw 오류가 장면 revision을 바꾸지 않았고 다음 draw가 회복됐습니다")
        } else {
            log("SPINON_C0410_FAILURE_PROBE_FAILED stage=recovery")
            postStatus("실패 · draw 오류 뒤 정상 장면 복구가 확인되지 않았습니다")
        }
#else
        log("SPINON_C0410_FAILURE_PROBE_DISABLED · 내부 실패 fixture 빌드가 아닙니다")
        postStatus("실패 · draw 실패 검증용 빌드 옵션이 꺼져 있습니다")
#endif
    }

    private func reportField(_ report: String, _ field: String) -> String {
        guard let range = report.range(of: field) else { return "" }
        let value = report[range.upperBound...]
        return String(value.prefix { !$0.isWhitespace })
    }

    private func parseSurfaceDimensions(_ value: String) -> (width: UInt32, height: UInt32)? {
        let fields = value.split(separator: "x", omittingEmptySubsequences: false)
        guard fields.count == 2,
              let width = UInt32(fields[0]), let height = UInt32(fields[1]),
              width > 0, height > 0 else { return nil }
        return (width, height)
    }

    private func isCurrentSurfaceGeneration(_ generation: UInt64) -> Bool {
        stateLock.lock()
        defer { stateLock.unlock() }
        return !closing && surfaceGeneration == generation
    }

    private func canUseSurfaceGeneration(_ generation: UInt64) -> Bool {
        stateLock.lock()
        defer { stateLock.unlock() }
        return !closing && rendererReady && surfaceGeneration == generation
            && rendererSurfaceGeneration == generation
    }

    private func isCurrentPresentation(generation: UInt64, sequence: UInt64) -> Bool {
        stateLock.lock()
        defer { stateLock.unlock() }
        return !closing && surfaceGeneration == generation && presentationSequence == sequence
    }

    private func retryRendererCreation() {
        DispatchQueue.main.async { [weak self] in
            guard let self, !self.isClosing else { return }
            let size = self.lastDrawableSize
            guard size.width > 0, size.height > 0 else { return }
            self.ensureRenderer(width: Int(size.width), height: Int(size.height))
        }
    }

    private func postStatus(_ value: String) {
        DispatchQueue.main.async { [weak self] in
            guard let self, !self.isClosing else { return }
            self.statusLabel.text = value
        }
    }

    private func log(_ value: String) {
        NSLog("%@", value)
    }

    private func reportLaneFailure(_ lane: String, _ message: String) {
        log("SPINON_C0410_QUEUE_FAILURE lane=\(lane) message=\(message)")
        postStatus("실패 · \(lane) queue: \(message)")
    }

    private func beginPresentationUpdate() -> UInt64 {
        presentationUpdateLock.lock()
        defer { presentationUpdateLock.unlock() }
        stateLock.lock()
        guard !closing, hostHandle != 0 else {
            stateLock.unlock()
            return 0
        }
        let handle = hostHandle
        stateLock.unlock()

        let sequence = SpinonRunner.beginRuntimeGpuPresentationUpdate(handle)
        stateLock.lock()
        if !closing && hostHandle == handle && sequence != 0 {
            presentationSequence = sequence
        }
        stateLock.unlock()
        return sequence
    }

    private func enqueueRuntime(_ work: @escaping () -> Void) {
        stateLock.lock()
        guard !closing else {
            stateLock.unlock()
            return
        }
        runtimeQueue.async(execute: work)
        stateLock.unlock()
    }

    private func enqueueRender(_ work: @escaping () -> Void) {
        stateLock.lock()
        guard !closing else {
            stateLock.unlock()
            return
        }
        renderQueue.async(execute: work)
        stateLock.unlock()
    }

    private func beginShutdown() {
        stateLock.lock()
        guard !closing else {
            stateLock.unlock()
            return
        }
        stateLock.unlock()
        _ = beginPresentationUpdate()
        stateLock.lock()
        guard !closing else {
            stateLock.unlock()
            return
        }
        surfaceGeneration &+= 1
        let destroyedGeneration = surfaceGeneration
        closing = true
        let runtimeQueue = self.runtimeQueue
        let renderQueue = self.renderQueue
        let hostLifetime = self.hostLifetime
        let retainedSurfaceView = canvasView
        stateLock.unlock()
        runtimePresentationLane?.close()
        renderDrawLane?.close()
        log("SPINON_C0410_SURFACE_DESTROYED generation=\(destroyedGeneration)")

        runtimeQueue.async {
            renderQueue.async {
                let handle = hostLifetime.take()
                if handle != 0 {
                    SpinonRunner.destroyRuntimeGpuRenderer(handle)
                    SpinonRunner.freeRuntimeGpuHost(handle)
                }
                _ = retainedSurfaceView
                NSLog("SPINON_C0410_DRAINED generation=%llu", destroyedGeneration)
            }
        }
    }

    deinit {
        beginShutdown()
    }
}

private enum C0410LatestTaskLaneError: Error, CustomStringConvertible {
    case operationFailed(String)

    var description: String {
        switch self {
        case let .operationFailed(message): message
        }
    }
}

private final class C0410RuntimeGpuHostLifetime {
    private let lock = NSLock()
    private var handle: UInt64 = 0

    func store(_ value: UInt64) {
        lock.lock()
        handle = value
        lock.unlock()
    }

    func load() -> UInt64 {
        lock.lock()
        defer { lock.unlock() }
        return handle
    }

    func take() -> UInt64 {
        lock.lock()
        defer { lock.unlock() }
        let value = handle
        handle = 0
        return value
    }
}

private final class C0410RuntimeGpuCanvasView: UIView {
    override class var layerClass: AnyClass { CAMetalLayer.self }
}
