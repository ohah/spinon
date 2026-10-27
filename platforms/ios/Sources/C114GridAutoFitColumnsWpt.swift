import Foundation

enum C114GridAutoFitColumnsWpt {
    private static let expectedMarker = "c11_4_grid_auto_fit_columns_wpt_expected="
    private static let traceMarker = "grid_track_sizing_diagnostics="
    private static let prefix = "grid-auto-fit-columns-"
    private static let epsilon = 0.02

    private struct UsedTrack {
        let names: Set<String>
        let size: Double
    }

    private struct ParsedTrackList {
        let tracks: [UsedTrack]
        let trailingNames: Set<String>
    }

    static func caseIndex(_ sceneID: String) -> Int? {
        guard sceneID.hasPrefix(prefix) else { return nil }
        let suffix = String(sceneID.dropFirst(prefix.count))
        guard suffix.count == 2, let index = Int(suffix), (1...35).contains(index),
              String(format: "%02d", index) == suffix else { return nil }
        return index
    }

    static func isScene(_ sceneID: String) -> Bool {
        caseIndex(sceneID) != nil
    }

    static func expectedBoxCount(_ sceneID: String) -> Int? {
        guard let index = caseIndex(sceneID) else { return nil }
        return index == 32 || index == 35 ? 3 : 2
    }

    static func surfacePixelSamples(
        sceneID: String
    ) -> [(String, String, UInt32, UInt32, [UInt32])]? {
        guard let index = caseIndex(sceneID) else { return nil }
        let itemX: Int
        let itemWidth: Int
        let itemSampleX: UInt32
        switch index {
        case 7:
            itemX = 0
            itemWidth = 357
            itemSampleX = 230
        case 8:
            itemX = 0
            itemWidth = 257
            itemSampleX = 230
        case 11:
            itemX = 50
            itemWidth = 297
            itemSampleX = 250
        case 14:
            itemX = 267
            itemWidth = 157
            itemSampleX = 300
        case 19:
            itemX = 80
            itemWidth = 222
            itemSampleX = 250
        case 31:
            itemX = 40
            itemWidth = 180
            itemSampleX = 210
        default: return nil
        }
        let suffix = String(format: "%02d", index)
        let rootID = "c114-grid-auto-fit-wpt-\(suffix)-root"
        let itemID = "c114-grid-auto-fit-wpt-\(suffix)-item-1"
        let itemEndAfterX = UInt32(itemX + 2 + itemWidth + 1)
        let samplePrefix = "grid-auto-fit-columns-\(suffix)"
        return [
            ("\(samplePrefix)-border", rootID, 100, 28, [255, 0, 255, 255]),
            ("\(samplePrefix)-overflow-item", itemID, itemSampleX, 10,
             [0, 255, 255, 255]),
            ("\(samplePrefix)-item-end-background", rootID, itemEndAfterX, 10,
             [0, 0, 0, 255]),
            ("\(samplePrefix)-control-background", rootID, 470, 200,
             [0, 0, 0, 255]),
        ]
    }

    static func expectedRootFrame(_ sceneID: String) -> String? {
        guard let index = caseIndex(sceneID) else { return nil }
        let width = index == 2 ? 230 : 204
        let height = index == 32 ? 54 : 29
        return "width=\(width),height=\(height)"
    }

    static func nodeFramesMatch(sceneID: String, report: String) -> Bool {
        guard isScene(sceneID) else { return true }
        guard let expected = expected(sceneID: sceneID, report: report),
              let root = expected["expectedRootFrame"] as? [String: Any],
              let items = expected["items"] as? [[String: Any]],
              let rootX = root["x"] as? Int,
              let rootY = root["y"] as? Int,
              let rootWidth = root["width"] as? Int,
              let rootHeight = root["height"] as? Int else { return false }
        var frames = "node_frames_css_px=[0:node=1,x=\(rootX),y=\(rootY),"
            + "width=\(rootWidth),height=\(rootHeight)"
        for (index, item) in items.enumerated() {
            guard let frame = item["expected"] as? [String: Any],
                  let x = frame["x"] as? Int,
                  let y = frame["y"] as? Int,
                  let width = frame["width"] as? Int,
                  let height = frame["height"] as? Int else { return false }
            frames += ";\(index + 1):node=\(index + 2),x=\(x + 2),y=\(y + 2),"
                + "width=\(width),height=\(height)"
        }
        guard items.count + 1 == expectedBoxCount(sceneID) else { return false }
        frames += "]"
        return report.contains(frames)
    }

    static func traceMatches(sceneID: String, report: String) -> Bool {
        guard isScene(sceneID) else { return true }
        guard let expected = expected(sceneID: sceneID, report: report),
              let repeatExpectation = expected["repeat"] as? [String: Any],
              let repeatCount = repeatExpectation["expectedCount"] as? Int,
              let prefixCount = repeatExpectation["prefixTrackCount"] as? Int,
              let patternCount = repeatExpectation["patternTrackCount"] as? Int,
              let suffixCount = repeatExpectation["suffixTrackCount"] as? Int,
              let observation = expected["usedTrackObservation"] as? [String: Any],
              let diagnostics = jsonAfter(traceMarker, report: report),
              let traces = diagnostics["trackSizingTraces"] as? [[String: Any]],
              traces.count == 1,
              let trace = traces.first,
              let grid = trace["grid"] as? Int, grid == 1,
              let usedColumns = trace["usedColumns"] as? [String: Any],
              let usedRows = trace["usedRows"] as? [String: Any],
              let passes = trace["passes"] as? [[String: Any]],
              passes.count == 2 else { return false }
        let columns = passes[0]
        let rows = passes[1]
        guard columns["axis"] as? String == "Columns",
              rows["axis"] as? String == "Rows",
              columns["sequence"] as? Int == 0,
              rows["sequence"] as? Int == 1,
              columns["autoRepeatCount"] as? Int == repeatCount,
              rows["autoRepeatCount"] is NSNull,
              let authoredTrackList = repeatExpectation["authoredTrackList"] as? String,
              let basisString = repeatExpectation["definiteContentBasisPx"] as? String,
              let basis = Double(basisString),
              let percentageBasis = columns["percentageBasis"],
              let columnContributions = columns["contributions"] as? [[String: Any]],
              let rowContributions = rows["contributions"] as? [[String: Any]],
              contributionsMatch(columnContributions, sceneID: sceneID),
              rowContributions.isEmpty,
              let expectedColumns = observation["gridTemplateColumns"] as? String,
              let expectedRows = observation["gridTemplateRows"] as? String,
              let columnGap = observation["columnGap"] as? String,
              let rowGap = observation["rowGap"] as? String else {
            return false
        }
        if let observedBasis = percentageBasis as? Double {
            guard close(observedBasis, basis) else { return false }
        } else if !(percentageBasis is NSNull) || authoredTrackList.contains("%") {
            return false
        }
        let expectedExplicitTracks = prefixCount + patternCount * repeatCount + suffixCount
        return resolvedAxisMatches(usedColumns, observation: observation, axis: "columns",
                                   explicitTrackCount: expectedExplicitTracks,
                                   repeatMode: repeatExpectation["mode"] as? String ?? "")
            && resolvedAxisMatches(usedRows, observation: observation, axis: "rows",
                                   explicitTrackCount: 0, repeatMode: "none")
            && passMatches(columns, expectedList: expectedColumns, gapValue: columnGap,
                           repeatMode: repeatExpectation["mode"] as? String ?? "")
            && passMatches(rows, expectedList: expectedRows, gapValue: rowGap,
                           repeatMode: "none")
    }

    private static func contributionsMatch(_ contributions: [[String: Any]],
                                           sceneID: String) -> Bool {
        let expectedSpan: Int
        switch sceneID {
        case "grid-auto-fit-columns-15": expectedSpan = 4
        case "grid-auto-fit-columns-18": expectedSpan = 3
        case "grid-auto-fit-columns-19": expectedSpan = 2
        default: return contributions.isEmpty
        }
        guard contributions.count == 3 else { return false }
        var kinds = Set<String>()
        for contribution in contributions {
            guard let kind = contribution["kind"] as? String,
                  ["Minimum", "MinContent", "MaxContent"].contains(kind),
                  kinds.insert(kind).inserted,
                  contribution["item"] as? Int == 2,
                  contribution["span"] as? Int == expectedSpan,
                  contribution["crossesFlexibleTrack"] as? Bool == false,
                  let value = contribution["value"] as? Double,
                  value.isFinite, close(value, 0) else { return false }
            if kind == "Minimum" {
                guard contribution["minimumSource"] as? String != nil else { return false }
            } else if !(contribution["minimumSource"] is NSNull) {
                return false
            }
        }
        return kinds == Set(["Minimum", "MinContent", "MaxContent"])
    }

    private static func resolvedAxisMatches(_ actual: [String: Any],
                                            observation: [String: Any], axis: String,
                                            explicitTrackCount: Int,
                                            repeatMode: String) -> Bool {
        let listKey = axis == "columns" ? "gridTemplateColumns" : "gridTemplateRows"
        let explicitKey = axis == "columns" ? "explicitColumnTrackCount" : "explicitRowTrackCount"
        let negativeKey = axis == "columns"
            ? "negativeImplicitColumnTrackCount" : "negativeImplicitRowTrackCount"
        let positiveKey = axis == "columns"
            ? "positiveImplicitColumnTrackCount" : "positiveImplicitRowTrackCount"
        guard let expectedText = observation[listKey] as? String,
              let expected = parseTrackList(expectedText),
              let actualText = actual["usedTrackList"] as? String,
              let resolved = parseTrackList(actualText),
              trackListsMatch(resolved, expected),
              let expectedExplicit = observation[explicitKey] as? Int,
              let expectedNegative = observation[negativeKey] as? Int,
              let expectedPositive = observation[positiveKey] as? Int,
              actual["explicitTrackCount"] as? Int == expectedExplicit,
              actual["negativeImplicitTrackCount"] as? Int == expectedNegative,
              actual["positiveImplicitTrackCount"] as? Int == expectedPositive,
              expectedExplicit == explicitTrackCount,
              expectedNegative + expectedExplicit + expectedPositive == expected.tracks.count,
              let collapsed = actual["collapsedTrackIndices"] as? [Int] else { return false }
        let expectedCollapsed = repeatMode == "auto-fit"
            ? expected.tracks.enumerated().compactMap {
                close($0.element.size, 0) ? $0.offset : nil
            }
            : []
        return collapsed == expectedCollapsed
    }

    private static func passMatches(_ pass: [String: Any], expectedList: String,
                                    gapValue: String, repeatMode: String) -> Bool {
        guard let expected = parseTrackList(expectedList),
              let stages = pass["stages"] as? [[String: Any]], stages.count >= 2 else { return false }
        let expectedInternalCount = expected.tracks.count * 2 + 1
        let gap: Double
        if gapValue == "normal" {
            gap = 0
        } else if gapValue.hasSuffix("px"),
                  let value = Double(gapValue.dropLast(2)) {
            gap = value
        } else {
            return false
        }
        let collapsed = expected.tracks.map { repeatMode == "auto-fit" && close($0.size, 0) }
        var trailingCollapsedStart = collapsed.count
        while trailingCollapsedStart > 0 && collapsed[trailingCollapsedStart - 1] {
            trailingCollapsedStart -= 1
        }
        for (stageIndex, stage) in stages.enumerated() {
            let finalStage = stageIndex == stages.count - 1
            guard let tracks = stage["tracks"] as? [[String: Any]],
                  tracks.count == expectedInternalCount else { return false }
            for (index, track) in tracks.enumerated() {
                guard let internalIndex = track["internalIndex"] as? Int,
                      internalIndex == index,
                      let isGutter = track["gutter"] as? Bool,
                      isGutter == (index.isMultiple(of: 2)),
                      let isCollapsed = track["collapsed"] as? Bool,
                      let base = track["baseSize"] as? Double,
                      base.isFinite, base >= -epsilon,
                      track["infinitelyGrowable"] as? Bool != nil,
                      let growth = track["growthLimit"] else { return false }
                if let growthValue = growth as? Double {
                    if !growthValue.isFinite || growthValue < -epsilon { return false }
                } else if !(growth is NSNull) {
                    return false
                }
                if isGutter {
                    let lineIndex = index / 2
                    let edge = lineIndex == 0 || lineIndex == expected.tracks.count
                    let previousTrackCollapsed = !edge && collapsed[lineIndex - 1]
                    let startsTrailingCollapsedRun = lineIndex == trailingCollapsedStart
                        && trailingCollapsedStart < expected.tracks.count
                    let expectedGutterCollapsed = edge || previousTrackCollapsed
                        || startsTrailingCollapsedRun
                    let expectedGap = expectedGutterCollapsed ? 0 : gap
                    if isCollapsed != expectedGutterCollapsed || !close(base, expectedGap) {
                        return false
                    }
                } else {
                    let trackIndex = index / 2
                    let expectedCollapsed = collapsed[trackIndex]
                    if isCollapsed != expectedCollapsed
                        || (finalStage
                            && !close(base, expected.tracks[trackIndex].size)) { return false }
                    if expectedCollapsed {
                        guard close(base, 0), let growthValue = growth as? Double,
                              close(growthValue, 0) else { return false }
                    }
                }
            }
        }
        return stages.first?["stage"] as? String == "Initialize"
    }

    private static func parseTrackList(_ value: String) -> ParsedTrackList? {
        guard !value.isEmpty, value != "none" else { return nil }
        var cursor = value.startIndex
        var names = Set<String>()
        var tracks: [UsedTrack] = []
        while cursor < value.endIndex {
            while cursor < value.endIndex, value[cursor].isWhitespace {
                cursor = value.index(after: cursor)
            }
            guard cursor < value.endIndex else { break }
            if value[cursor] == "[" {
                guard let close = value[cursor...].firstIndex(of: "]") else { return nil }
                let nameStart = value.index(after: cursor)
                names.formUnion(value[nameStart..<close]
                    .split(whereSeparator: \.isWhitespace).map(String.init))
                cursor = value.index(after: close)
                continue
            }
            let tokenStart = cursor
            while cursor < value.endIndex, !value[cursor].isWhitespace {
                cursor = value.index(after: cursor)
            }
            let token = String(value[tokenStart..<cursor])
            guard token.hasSuffix("px"),
                  let size = Double(token.dropLast(2)), size.isFinite, size >= 0 else { return nil }
            tracks.append(UsedTrack(names: names, size: size))
            names.removeAll(keepingCapacity: true)
        }
        guard !tracks.isEmpty else { return nil }
        return ParsedTrackList(tracks: tracks, trailingNames: names)
    }

    private static func trackListsMatch(_ actual: ParsedTrackList,
                                        _ expected: ParsedTrackList) -> Bool {
        guard actual.tracks.count == expected.tracks.count,
              actual.trailingNames == expected.trailingNames else { return false }
        return zip(actual.tracks, expected.tracks).allSatisfy {
            $0.names == $1.names && close($0.size, $1.size)
        }
    }

    private static func close(_ left: Double, _ right: Double) -> Bool {
        left.isFinite && right.isFinite && abs(left - right) <= epsilon
    }

    private static func expected(sceneID: String, report: String) -> [String: Any]? {
        guard let index = caseIndex(sceneID),
              let value = jsonAfter(expectedMarker, report: report),
              value["sceneId"] as? String == sceneID,
              value["wptIndex"] as? Int == index else { return nil }
        return value
    }

    private static func jsonAfter(_ marker: String, report: String) -> [String: Any]? {
        guard let range = report.range(of: marker) else { return nil }
        let suffix = report[range.upperBound...]
        guard let start = suffix.firstIndex(of: "{") else { return nil }
        var depth = 0
        var inString = false
        var escaped = false
        var cursor = start
        var end: String.Index?
        while cursor < suffix.endIndex {
            let character = suffix[cursor]
            if inString {
                if escaped {
                    escaped = false
                } else if character == "\\" {
                    escaped = true
                } else if character == "\"" {
                    inString = false
                }
            } else if character == "\"" {
                inString = true
            } else if character == "{" {
                depth += 1
            } else if character == "}" {
                depth -= 1
                if depth == 0 {
                    end = suffix.index(after: cursor)
                    break
                }
            }
            cursor = suffix.index(after: cursor)
        }
        guard let end,
              let data = String(suffix[start..<end]).data(using: .utf8),
              let value = try? JSONSerialization.jsonObject(with: data) as? [String: Any] else {
            return nil
        }
        return value
    }
}
