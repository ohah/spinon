import Foundation

enum C114FitContentImplicitColumnWpt {
    static let sceneID = "fit-content-implicit-column-wpt"
    private static let rootID = "c114-fit-content-implicit-column-grid"
    private static let itemID = "c114-fit-content-implicit-column-item"
    private static let childID = "c114-fit-content-implicit-column-child"
    private static let traceMarker = "grid_track_sizing_diagnostics="

    static func isScene(_ sceneID: String) -> Bool {
        sceneID == Self.sceneID
    }

    static func expectedRootFrame(_ sceneID: String) -> String? {
        isScene(sceneID) ? "width=300,height=300" : nil
    }

    static func expectedItemFrame(_ sceneID: String) -> String? {
        isScene(sceneID) ? "width=40,height=10" : nil
    }

    static func expectedBoxCount(_ sceneID: String) -> Int? {
        isScene(sceneID) ? 3 : nil
    }

    static func expectedNodeFrames(_ sceneID: String) -> String? {
        guard isScene(sceneID) else { return nil }
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=300,height=300;"
            + "1:node=2,x=60,y=0,width=40,height=10;"
            + "2:node=3,x=60,y=0,width=400,height=10]"
    }

    static func nodeFramesMatch(sceneID: String, report: String) -> Bool {
        guard isScene(sceneID) else { return true }
        guard let expectedNodeFrames = expectedNodeFrames(sceneID) else { return false }
        return report.contains(expectedNodeFrames)
    }

    static func surfacePixelSamples(
        sceneID: String
    ) -> [(String, String, UInt32, UInt32, [UInt32])]? {
        guard isScene(sceneID) else { return nil }
        return [
            ("implicit-column-root-background", rootID, 30, 5, [23, 33, 43, 255]),
            ("implicit-column-child-inside-item", childID, 80, 5, [255, 209, 102, 255]),
            ("implicit-column-child-outside-root", childID, 350, 5, [255, 209, 102, 255]),
            ("implicit-column-surface-clear", rootID, 470, 5, [0, 0, 0, 255]),
        ]
    }

    static func traceMatches(sceneID: String, report: String) -> Bool {
        guard isScene(sceneID) else { return true }
        guard let diagnostics = diagnostics(report),
              let observation = diagnostics["fitContentImplicitColumn"] as? [String: Any],
              let style = observation["styleSnapshot"] as? [String: Any],
              let computed = style["computedProperties"] as? [String: Any],
              let input = observation["layoutInputGridContainer"] as? [String: Any],
              let nodeIDs = observation["nodeIds"] as? [String: Any],
              let frames = observation["framesCssPx"] as? [String: Any],
              let revision = diagnostics["layoutInputRevision"] as? [String: Any],
              let overflow = observation["overflowCssPx"] as? [String: Any],
              let generation = integer(style["generation"]),
              let document = integer(style["documentRevision"]),
              let renderTree = integer(style["renderTreeRevision"]),
              let styleRevision = integer(style["styleRevision"]),
              let environment = integer(style["environmentRevision"]),
              let revisionSource = revision["source"] as? String else { return false }

        guard observation["sceneId"] as? String == Self.sceneID,
              observation["frameSource"] as? String == "pre-render LayoutOutput CSS px",
              style["profile"] as? String == "RuntimeBlockFlexGridTrackSizingV1",
              style["rootNode"] as? Int == 1,
              computed["display"] as? String == "grid",
              computed["width"] as? String == "LengthPx(300.0)",
              computed["height"] as? String == "LengthPx(300.0)",
              computed["gridTemplateColumns"] as? String == "60px",
              computed["gridTemplateRows"] as? String == "none",
              computed["gridAutoColumns"] as? String == "fit-content(40px)",
              computed["gridAutoRows"] as? String == "fit-content(40px)",
              computed["alignContent"] as? String == "normal",
              (computed["itemColumnStart"] as? String)?.contains("Line(2)") == true,
              computed["itemMinWidth"] as? String == "LengthPx(0.0)",
              input["rootNode"] as? Int == 1,
              input["columns"] as? String == "[LengthPx(60.0)]",
              input["explicitRows"] as? String == "[]",
              input["autoColumns"] as? String == "TrackList([FitContent(LengthPx(40.0))])",
              input["autoRows"] as? String == "TrackList([FitContent(LengthPx(40.0))])",
              input["autoFlow"] as? String == "Row",
              input["placementMode"] as? String == "TrackSizing",
              nodeIDs.count == 3,
              nodeIDs["root"] as? Int == 1,
              nodeIDs["item"] as? Int == 2,
              nodeIDs["child"] as? Int == 3,
              frameMatches(frames["root"], [0, 0, 300, 300]),
              frameMatches(frames["item"], [60, 0, 40, 10]),
              frameMatches(frames["child"], [60, 0, 400, 10]),
              numberArrayMatches(observation["expectedColumnsCssPx"], [60, 40]),
              numberArrayMatches(observation["expectedRowsCssPx"], [10]),
              integer(observation["expectedItemMinimumContributionCssPx"]) == 0,
              integer(observation["expectedItemMaxContentContributionCssPx"]) == 400,
              integer(overflow["childPastItem"]) == 360,
              integer(overflow["childPastRoot"]) == 160,
              stagesMatch(observation["columnSizingStages"],
                          [[60, 0], [60, 0], [60, 0], [60, 40], [60, 40], [60, 40]],
                          stretchApplied: false),
              stagesMatch(observation["rowSizingStages"],
                          [[0], [10], [10], [10], [10], [10]], stretchApplied: true),
              contributionsMatch(observation["columnContributions"],
                                 sizes: [0, 400, 400], minimumSource: "SpecifiedMinimum"),
              contributionsMatch(observation["rowContributions"],
                                 sizes: [10, 10, 10], minimumSource: "AutomaticMinimum { spans_auto_min_track: true, spans_multiple_tracks: false, spans_flexible_max_track: false, content_based: true, is_compressible_replaced: false, fixed_track_sum: None, fixed_gutter_sum: None, fixed_track_limit: None }"),
              generation > 0, document > 0, renderTree > 0,
              styleRevision >= 0, environment >= 0,
              integer(revision["style"]) == styleRevision,
              integer(revision["environment"]) == environment,
              revisionSource.hasPrefix("HostDocument {"),
              revisionSource.contains("DocumentGeneration(\(generation))"),
              revisionSource.contains("DocumentRevision(\(document))"),
              revisionSource.contains("RenderTreeRevision(\(renderTree))") else { return false }
        return true
    }

    static func presentationIdentity(
        _ report: String, drawReport: Bool
    ) -> String? {
        let rootFrameField = drawReport ? "root_css_px=" : "root_frame_css_px="
        let viewportField = drawReport ? "viewport_css_px=" : "render_viewport_css_px="
        guard report.hasPrefix("status=0 "),
              reportField(report, "boxes=") == "3",
              reportField(report, rootFrameField) == "300x300",
              reportField(report, viewportField) == "480x320",
              drawReport
                ? (report.contains("presented boxes=3 ")
                    && reportField(report, "surface_texture=")
                        .range(of: "^[0-9]+x[0-9]+$", options: .regularExpression) != nil)
                : reportField(report, "layout=") == "ready" else { return nil }
        let fields = [
            "generation=", "document_revision=", "render_tree_revision=",
            "style_revision=", "environment_revision="
        ]
        let values = fields.map { reportField(report, $0) }
        guard values.allSatisfy({ UInt64($0) != nil }),
              (UInt64(values[0]) ?? 0) > 0,
              (UInt64(values[1]) ?? 0) > 0,
              (UInt64(values[2]) ?? 0) > 0 else { return nil }
        return "300x300/480x320/" + values.joined(separator: "/")
    }

    private static func diagnostics(_ report: String) -> [String: Any]? {
        guard let markerRange = report.range(of: traceMarker),
              let data = report[markerRange.upperBound...].data(using: .utf8),
              let value = try? JSONSerialization.jsonObject(with: data),
              let diagnostics = value as? [String: Any] else { return nil }
        return diagnostics
    }

    private static func reportField(_ report: String, _ field: String) -> String {
        guard let range = report.range(of: field) else { return "" }
        let suffix = report[range.upperBound...]
        let end = suffix.firstIndex(where: { $0 == " " || $0 == "\n" }) ?? suffix.endIndex
        return String(suffix[..<end])
    }

    private static func integer(_ value: Any?) -> Int? {
        guard let number = value as? NSNumber else { return nil }
        return number.intValue
    }

    private static func double(_ value: Any?) -> Double? {
        guard let number = value as? NSNumber else { return nil }
        let result = number.doubleValue
        return result.isFinite ? result : nil
    }

    private static func frameMatches(_ value: Any?, _ expected: [Double]) -> Bool {
        guard let frame = value as? [String: Any], expected.count == 4,
              let x = double(frame["x"]), let y = double(frame["y"]),
              let width = double(frame["width"]), let height = double(frame["height"]) else {
            return false
        }
        return zip([x, y, width, height], expected).allSatisfy { pair in
            abs(pair.0 - pair.1) <= 0.000001
        }
    }

    private static func numberArrayMatches(_ value: Any?, _ expected: [Double]) -> Bool {
        guard let values = value as? [Any], values.count == expected.count else { return false }
        return zip(values, expected).allSatisfy { pair in
            guard let actual = double(pair.0) else { return false }
            return abs(actual - pair.1) <= 0.000001
        }
    }

    private static func stagesMatch(
        _ value: Any?, _ expected: [[Double]], stretchApplied: Bool
    ) -> Bool {
        let names = [
            "Initialize",
            "IntrinsicItemBatch { span: 1, crosses_flexible_track: false }",
            "IntrinsicSizingComplete", "MaximizeTracks", "ExpandFlexibleTracks",
            "StretchAutoTracks { applied: \(stretchApplied) }",
        ]
        guard let stages = value as? [[String: Any]],
              stages.count == names.count, expected.count == names.count else { return false }
        return stages.indices.allSatisfy { index in
            stages[index]["stage"] as? String == names[index]
                && numberArrayMatches(stages[index]["baseSizesCssPx"], expected[index])
        }
    }

    private static func contributionsMatch(
        _ value: Any?, sizes: [Double], minimumSource: String
    ) -> Bool {
        guard let contributions = value as? [[String: Any]], contributions.count == 3 else {
            return false
        }
        let kinds = ["Minimum", "MinContent", "MaxContent"]
        for index in kinds.indices {
            let contribution = contributions[index]
            guard integer(contribution["item"]) == 2,
                  contribution["kind"] as? String == kinds[index],
                  integer(contribution["span"]) == 1,
                  contribution["crossesFlexibleTrack"] as? Bool == false,
                  let value = double(contribution["valueCssPx"]),
                  abs(value - sizes[index]) <= 0.000001 else { return false }
        }
        guard contributions.dropFirst().allSatisfy({ $0["minimumSource"] is NSNull }) else {
            return false
        }
        return contributions[0]["minimumSource"] as? String == minimumSource
    }
}
