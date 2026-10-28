import CoreFoundation
import Foundation

/// C11.4 §11.7의 네 고정 앱 runtime flexible-track scene을 검증합니다.
enum C114FlexStage {
    private static let sceneIDs: Set<String> = [
        "weighted-columns", "frozen-row", "subunit-columns", "zero-fr-row",
        "column-freeze", "column-no-freeze", "column-equal-boundary", "column-freeze-reversed",
        "row-freeze", "row-no-freeze", "row-equal-boundary", "row-freeze-reversed",
    ]
    private static let tolerance = 1.0e-4

    static func isScene(_ sceneID: String) -> Bool {
        sceneIDs.contains(sceneID)
    }

    static func expectedRootFrame(_ sceneID: String) -> String? {
        guard isScene(sceneID) else { return nil }
        if isMixedScene(sceneID) {
            return isRowScene(sceneID) ? "width=120,height=300" : "width=300,height=120"
        }
        return isRowScene(sceneID) ? "width=120,height=200" : "width=200,height=120"
    }

    static func itemCount(_ sceneID: String) -> Int {
        guard isScene(sceneID) else { return 0 }
        return isMixedScene(sceneID) ? 3 : 2
    }

    static func nodeFramesMatch(_ sceneID: String, report: String) -> Bool {
        guard let expected = expectedNodeFrames(sceneID),
              let range = report.range(of: "node_frames_css_px=[") else { return false }
        let suffix = report[range.upperBound...]
        guard let end = suffix.firstIndex(of: "]") else { return false }
        let encoded = String(suffix[..<end])
        guard !report[suffix.index(after: end)...].contains("node_frames_css_px=[") else {
            return false
        }
        let frames = encoded.split(separator: ";", omittingEmptySubsequences: false)
        guard frames.count == expected.count else { return false }
        for (index, frameText) in frames.enumerated() {
            let fields = frameText.split(separator: ",", omittingEmptySubsequences: false)
            guard fields.count == 5 else { return false }
            let identity = fields[0].split(separator: ":", maxSplits: 1)
            guard identity.count == 2,
                  let frameIndex = Int(identity[0]),
                  let node = integerField(identity[1], prefix: "node="),
                  frameIndex == index, node == expectedFrameNode(sceneID, index) else { return false }
            let numericFields = [
                (fields[1], "x="), (fields[2], "y="),
                (fields[3], "width="), (fields[4], "height="),
            ]
            for (fieldIndex, field) in numericFields.enumerated() {
                guard let actual = doubleField(field.0, prefix: field.1),
                      actual.isFinite,
                      abs(actual - expected[index][fieldIndex]) <= tolerance else { return false }
            }
        }
        return true
    }

    static func traceMatches(_ sceneID: String, report: String) -> Bool {
        guard isScene(sceneID),
              let diagnostics = diagnosticsObject(report),
              let traces = diagnostics["trackSizingTraces"] as? [[String: Any]],
              traces.count == 1,
              integer(traces[0]["grid"]) == 1,
              let passes = traces[0]["passes"] as? [[String: Any]],
              passes.count == 2,
              integer(passes[0]["sequence"]) == 0,
              integer(passes[1]["sequence"]) == 1,
              string(passes[0]["axis"]) == "Columns",
              string(passes[1]["axis"]) == "Rows" else { return false }

        let targetIndex = isRowScene(sceneID) ? 1 : 0
        let fixedIndex = 1 - targetIndex
        return fixedAxisMatches(passes[fixedIndex])
            && flexibleAxisMatches(sceneID, pass: passes[targetIndex])
            && (!isMixedScene(sceneID) || intrinsicInputsMatch(sceneID, diagnostics: diagnostics))
    }

    private static func fixedAxisMatches(_ pass: [String: Any]) -> Bool {
        guard let stages = pass["stages"] as? [[String: Any]],
              stages.count == 2,
              let contributions = pass["contributions"] as? [[String: Any]],
              contributions.isEmpty,
              string(stages[0]["stage"]) == "Initialize",
              string(stages[1]["stage"]) == "FixedTrackEarlyReturn" else { return false }
        return fixedTracksMatch(stages[0]["tracks"]) && fixedTracksMatch(stages[1]["tracks"])
    }

    private static func fixedTracksMatch(_ value: Any?) -> Bool {
        guard let tracks = value as? [[String: Any]], tracks.count == 3 else { return false }
        for (index, track) in tracks.enumerated() {
            let gutter = index.isMultiple(of: 2)
            let boundary = index == 0 || index == 2
            let size = gutter ? 0.0 : 120.0
            guard integer(track["internalIndex"]) == index,
                  boolean(track["gutter"]) == gutter,
                  boolean(track["collapsed"]) == boundary,
                  near(track["baseSize"], expected: size),
                  near(track["growthLimit"], expected: size) else { return false }
        }
        return true
    }

    private static func flexibleAxisMatches(_ sceneID: String, pass: [String: Any]) -> Bool {
        guard string(pass["axis"]) == (isRowScene(sceneID) ? "Rows" : "Columns"),
              let stages = pass["stages"] as? [[String: Any]] else { return false }
        let names = stages.compactMap { string($0["stage"]) }
        guard names.count == stages.count,
              names.filter({ $0 == "Initialize" }).count == 1,
              names.filter({ $0 == "MaximizeTracks" }).count == 1,
              names.filter({ $0 == "ExpandFlexibleTracks" }).count == 1,
              names.filter({ $0 == "StretchAutoTracks { applied: false }" }).count == 1,
              let maximize = names.firstIndex(of: "MaximizeTracks"),
              let expand = names.firstIndex(of: "ExpandFlexibleTracks"),
              let stretch = names.firstIndex(of: "StretchAutoTracks { applied: false }"),
              names.firstIndex(of: "Initialize")! < maximize,
              maximize < expand, expand < stretch else { return false }
        return flexibleTracksMatch(sceneID, tracks: stages[expand]["tracks"])
    }

    private static func flexibleTracksMatch(_ sceneID: String, tracks value: Any?) -> Bool {
        guard let expected = expectedTrackBases(sceneID),
              let tracks = value as? [[String: Any]], tracks.count == expected.count * 2 + 1 else {
            return false
        }
        var cssTrackIndex = 0
        for (index, track) in tracks.enumerated() {
            let gutter = index.isMultiple(of: 2)
            let boundary = index == 0 || index == tracks.count - 1
            guard integer(track["internalIndex"]) == index,
                  boolean(track["gutter"]) == gutter,
                  boolean(track["collapsed"]) == boundary else { return false }
            let size: Double
            if gutter {
                size = boundary ? 0 : 10
            } else {
                guard cssTrackIndex < expected.count else { return false }
                size = expected[cssTrackIndex]
                cssTrackIndex += 1
            }
            guard near(track["baseSize"], expected: size) else { return false }
        }
        return cssTrackIndex == expected.count
    }

    private static func diagnosticsObject(_ report: String) -> [String: Any]? {
        guard let marker = report.range(of: "grid_track_sizing_diagnostics=") else { return nil }
        let suffix = report[marker.upperBound...]
        var depth = 0
        var inString = false
        var escaped = false
        var end: String.Index?
        for index in suffix.indices {
            let character = suffix[index]
            if inString {
                if escaped { escaped = false }
                else if character == "\\" { escaped = true }
                else if character == "\"" { inString = false }
                continue
            }
            if character == "\"" { inString = true }
            else if character == "{" { depth += 1 }
            else if character == "}" {
                depth -= 1
                if depth == 0 { end = suffix.index(after: index); break }
            }
        }
        guard let end,
              let data = String(suffix[..<end]).data(using: .utf8),
              let value = try? JSONSerialization.jsonObject(with: data) as? [String: Any] else {
            return nil
        }
        return value
    }

    private static func expectedNodeFrames(_ sceneID: String) -> [[Double]]? {
        switch sceneID {
        case "column-freeze":
            return [[0, 0, 300, 120], [0, 0, 100, 120], [110, 0, 24, 16], [160, 0, 140, 120]]
        case "column-no-freeze", "column-equal-boundary":
            return [[0, 0, 300, 120], [0, 0, 80, 120], [90, 0, 24, 16], [140, 0, 160, 120]]
        case "column-freeze-reversed":
            return [[0, 0, 300, 120], [160, 0, 140, 120], [110, 0, 24, 16], [0, 0, 100, 120]]
        case "row-freeze":
            return [[0, 0, 120, 300], [0, 0, 120, 100], [0, 110, 24, 16], [0, 160, 120, 140]]
        case "row-no-freeze", "row-equal-boundary":
            return [[0, 0, 120, 300], [0, 0, 120, 80], [0, 90, 24, 16], [0, 140, 120, 160]]
        case "row-freeze-reversed":
            return [[0, 0, 120, 300], [0, 160, 120, 140], [0, 110, 24, 16], [0, 0, 120, 100]]
        case "weighted-columns":
            return [
                [0, 0, 200, 120], [0, 0, 190.0 / 3.0, 120], [220.0 / 3.0, 0, 380.0 / 3.0, 120],
            ]
        case "frozen-row":
            return [[0, 0, 120, 200], [0, 0, 120, 80], [0, 90, 120, 110]]
        case "subunit-columns":
            return [[0, 0, 200, 120], [0, 0, 85, 120], [95, 0, 20, 120]]
        case "zero-fr-row":
            return [[0, 0, 120, 200], [0, 0, 120, 24], [0, 34, 120, 166]]
        default:
            return nil
        }
    }

    private static func expectedTrackBases(_ sceneID: String) -> [Double]? {
        switch sceneID {
        case "column-freeze", "column-freeze-reversed", "row-freeze", "row-freeze-reversed":
            return [100, 40, 140]
        case "column-no-freeze", "column-equal-boundary", "row-no-freeze", "row-equal-boundary":
            return [80, 40, 160]
        case "weighted-columns": return [190.0 / 3.0, 380.0 / 3.0]
        case "frozen-row": return [80, 110]
        case "subunit-columns": return [85, 20]
        case "zero-fr-row": return [24, 166]
        default: return nil
        }
    }

    private static func isRowScene(_ sceneID: String) -> Bool {
        sceneID == "frozen-row" || sceneID == "zero-fr-row" || sceneID.hasPrefix("row-")
    }

    private static func isMixedScene(_ sceneID: String) -> Bool {
        sceneID.hasPrefix("column-") || sceneID.hasPrefix("row-")
    }

    private static func intrinsicInputsMatch(
        _ sceneID: String,
        diagnostics: [String: Any]
    ) -> Bool {
        guard let inputs = diagnostics["gridItemIntrinsicInputs"] as? [[String: Any]],
              inputs.count == 3 else { return false }
        let reversed = sceneID.hasSuffix("reversed")
        let aMinContent: Double = sceneID.contains("equal-boundary")
            ? 80 : (sceneID.contains("no-freeze") ? 40 : 100)
        for (index, input) in inputs.enumerated() {
            guard integer(input["sourceOrder"]) == index,
                  integer(input["node"]) == expectedIntrinsicNode(index),
                  let source = input["source"] as? [String: Any],
                  let kind = string(source["kind"]) else { return false }
            if index == 1 {
                guard kind == "replaced",
                      let natural = source["natural"] as? [String: Any],
                      near(natural["width"], expected: 24),
                      near(natural["height"], expected: 16) else { return false }
                continue
            }
            guard kind == (isRowScene(sceneID) ? "orthogonalText" : "text"),
                  let contribution = input["contributionInput"] as? [String: Any],
                  let targetSize = contribution[isRowScene(sceneID) ? "height" : "width"]
                    as? [String: Any] else { return false }
            let itemA = reversed ? index == 2 : index == 0
            let expected = itemA ? aMinContent : 40
            guard near(targetSize["minContent"], expected: expected) else { return false }
        }
        return true
    }

    private static func expectedFrameNode(_ sceneID: String, _ frameIndex: Int) -> Int {
        guard isMixedScene(sceneID) else { return frameIndex + 1 }
        switch frameIndex {
        case 0: return 1
        case 1: return 2
        case 2: return 4
        case 3: return 5
        default: return -1
        }
    }

    private static func expectedIntrinsicNode(_ inputIndex: Int) -> Int {
        switch inputIndex {
        case 0: return 2
        case 1: return 4
        case 2: return 5
        default: return -1
        }
    }

    private static func integerField(_ field: Substring, prefix: String) -> Int? {
        guard field.hasPrefix(prefix) else { return nil }
        return Int(field.dropFirst(prefix.count))
    }

    private static func doubleField(_ field: Substring, prefix: String) -> Double? {
        guard field.hasPrefix(prefix) else { return nil }
        return Double(field.dropFirst(prefix.count))
    }

    private static func integer(_ value: Any?) -> Int? {
        guard let number = value as? NSNumber,
              CFGetTypeID(number) != CFBooleanGetTypeID(),
              number.doubleValue.isFinite,
              number.doubleValue == Double(number.intValue) else { return nil }
        return number.intValue
    }

    private static func string(_ value: Any?) -> String? {
        value as? String
    }

    private static func boolean(_ value: Any?) -> Bool? {
        guard let number = value as? NSNumber,
              CFGetTypeID(number) == CFBooleanGetTypeID() else { return nil }
        return number.boolValue
    }

    private static func near(_ value: Any?, expected: Double) -> Bool {
        guard let number = value as? NSNumber,
              CFGetTypeID(number) != CFBooleanGetTypeID(),
              number.doubleValue.isFinite else { return false }
        return abs(number.doubleValue - expected) <= tolerance
    }
}
