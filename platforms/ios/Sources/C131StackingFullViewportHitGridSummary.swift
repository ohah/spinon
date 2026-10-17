import CoreFoundation
import Foundation

struct C131StackingFullViewportHitGridSummary {
    struct MismatchPreview {
        let xCssPx: Int
        let yCssPx: Int
        let expectedNodeId: String
        let actualNodeId: String?
    }

    let status: String
    let pointCount: Int
    let matchedCount: Int
    let mismatchCount: Int
    let edgeControlCount: Int
    let edgeControlsMatched: Int
    let mismatchPreview: [MismatchPreview]
    let expectedIdentitySha256: String
    let actualIdentitySha256: String
    let oracleSha256: String

    var isMatched: Bool {
        status == "matched"
            && pointCount == 69_120
            && matchedCount == 69_120
            && mismatchCount == 0
            && edgeControlCount == 11
            && edgeControlsMatched == 11
            && mismatchPreview.isEmpty
            && expectedIdentitySha256 == C131StackingFullViewportHitGridSummaryValidator.identitySha256
            && actualIdentitySha256 == expectedIdentitySha256
            && oracleSha256 == C131StackingFullViewportHitGridSummaryValidator.oracleSha256
    }
}

enum C131StackingFullViewportHitGridSummaryValidator {
    enum ValidationError: Error, CustomStringConvertible {
        case missingMarker
        case duplicateMarker
        case missingObject
        case unbalancedObject
        case invalidJson
        case nonCanonicalJson
        case invalidSchema
        case invalidCounts
        case invalidPreview
        case invalidDigest

        var description: String {
            switch self {
            case .missingMarker: return "missing-marker"
            case .duplicateMarker: return "duplicate-marker"
            case .missingObject: return "missing-object"
            case .unbalancedObject: return "unbalanced-object"
            case .invalidJson: return "invalid-json"
            case .nonCanonicalJson: return "non-canonical-json"
            case .invalidSchema: return "invalid-schema"
            case .invalidCounts: return "invalid-counts"
            case .invalidPreview: return "invalid-preview"
            case .invalidDigest: return "invalid-digest"
            }
        }
    }

    static let marker = "hit_identity_grid_summary_json="
    static let identitySha256 = "866073e0c6f092d4b2459fa3b5cfd2ca38dfb8935f582ee3c9614dba3bfc7483"
    static let oracleSha256 = "279364905336ef53d225c781bae68ec26d3fe4d50cfc7d0cfce689ab5354eee2"

    private static let summaryKeys: Set<String> = [
        "status", "point_count", "matched_count", "mismatch_count",
        "mismatch_preview_limit", "mismatch_preview", "edge_control_count",
        "edge_controls_matched", "expected_identity_sha256", "actual_identity_sha256",
        "oracle_sha256",
    ]
    private static let previewKeys: Set<String> = [
        "xCssPx", "yCssPx", "expectedNodeId", "actualNodeId",
    ]
    private static let nodeIds: Set<String> = [
        "root", "visible-parent", "visible-layer", "visible-target", "visible-overlay",
        "clip-parent", "clip-layer", "clip-target", "clip-overlay", "hidden-parent",
        "hidden-layer", "hidden-target", "hidden-overlay",
    ]
    private static let previewLimit = 16
    private static let pointCount = 69_120

    static func parse(_ report: String) throws -> C131StackingFullViewportHitGridSummary {
        guard let markerRange = report.range(of: marker) else {
            throw ValidationError.missingMarker
        }
        guard report.range(of: marker, range: markerRange.upperBound..<report.endIndex) == nil else {
            throw ValidationError.duplicateMarker
        }

        var objectStart = markerRange.upperBound
        while objectStart < report.endIndex, report[objectStart].isWhitespace {
            objectStart = report.index(after: objectStart)
        }
        guard objectStart < report.endIndex, report[objectStart] == "{" else {
            throw ValidationError.missingObject
        }
        guard let objectEnd = matchingObjectEnd(in: report, from: objectStart) else {
            throw ValidationError.unbalancedObject
        }
        let rawObject = String(report[objectStart..<objectEnd])
        guard let data = rawObject.data(using: .utf8) else {
            throw ValidationError.invalidJson
        }
        guard let object = try? JSONSerialization.jsonObject(with: data) as? [String: Any] else {
            throw ValidationError.invalidJson
        }
        guard Set(object.keys) == summaryKeys else {
            throw ValidationError.invalidSchema
        }
        guard let canonicalData = try? JSONSerialization.data(
            withJSONObject: object,
            options: [.sortedKeys]
        ), let canonicalObject = String(data: canonicalData, encoding: .utf8) else {
            throw ValidationError.invalidJson
        }
        guard canonicalObject == rawObject else {
            throw ValidationError.nonCanonicalJson
        }

        guard let status = object["status"] as? String,
              let pointCount = integer(object["point_count"]),
              let matchedCount = integer(object["matched_count"]),
              let mismatchCount = integer(object["mismatch_count"]),
              let previewLimit = integer(object["mismatch_preview_limit"]),
              let edgeControlCount = integer(object["edge_control_count"]),
              let edgeControlsMatched = integer(object["edge_controls_matched"]),
              let rawPreview = object["mismatch_preview"] as? [[String: Any]],
              let expectedIdentitySha256 = object["expected_identity_sha256"] as? String,
              let actualIdentitySha256 = object["actual_identity_sha256"] as? String,
              let oracleSha256 = object["oracle_sha256"] as? String else {
            throw ValidationError.invalidSchema
        }
        guard pointCount == Self.pointCount,
              (0...pointCount).contains(matchedCount),
              (0...pointCount).contains(mismatchCount),
              matchedCount + mismatchCount == pointCount,
              edgeControlCount == 11,
              edgeControlsMatched == 11,
              previewLimit == Self.previewLimit,
              rawPreview.count == min(mismatchCount, Self.previewLimit),
              (mismatchCount == 0 && status == "matched")
                  || (mismatchCount > 0 && status == "mismatch") else {
            throw ValidationError.invalidCounts
        }

        var coordinates = Set<String>()
        let preview = try rawPreview.map { item in
            try parsePreview(item, coordinates: &coordinates)
        }
        guard expectedIdentitySha256 == Self.identitySha256,
              oracleSha256 == Self.oracleSha256,
              isSha256(actualIdentitySha256),
              (mismatchCount == 0
                  ? actualIdentitySha256 == Self.identitySha256
                  : actualIdentitySha256 != Self.identitySha256) else {
            throw ValidationError.invalidDigest
        }

        return C131StackingFullViewportHitGridSummary(
            status: status,
            pointCount: pointCount,
            matchedCount: matchedCount,
            mismatchCount: mismatchCount,
            edgeControlCount: edgeControlCount,
            edgeControlsMatched: edgeControlsMatched,
            mismatchPreview: preview,
            expectedIdentitySha256: expectedIdentitySha256,
            actualIdentitySha256: actualIdentitySha256,
            oracleSha256: oracleSha256
        )
    }

    private static func parsePreview(
        _ object: [String: Any],
        coordinates: inout Set<String>
    ) throws -> C131StackingFullViewportHitGridSummary.MismatchPreview {
        guard Set(object.keys) == previewKeys,
              let x = integer(object["xCssPx"]),
              let y = integer(object["yCssPx"]),
              (0..<540).contains(x),
              (0..<128).contains(y),
              let expectedNodeId = object["expectedNodeId"] as? String,
              nodeIds.contains(expectedNodeId),
              coordinates.insert("\(x),\(y)").inserted else {
            throw ValidationError.invalidPreview
        }

        let actualNodeId: String?
        if object["actualNodeId"] is NSNull {
            actualNodeId = nil
        } else if let value = object["actualNodeId"] as? String, !value.isEmpty {
            actualNodeId = value
        } else {
            throw ValidationError.invalidPreview
        }
        return C131StackingFullViewportHitGridSummary.MismatchPreview(
            xCssPx: x,
            yCssPx: y,
            expectedNodeId: expectedNodeId,
            actualNodeId: actualNodeId
        )
    }

    private static func integer(_ value: Any?) -> Int? {
        guard let number = value as? NSNumber,
              CFGetTypeID(number) != CFBooleanGetTypeID() else {
            return nil
        }
        let type = String(cString: number.objCType)
        guard "cCsSiIlLqQ".contains(type) else {
            return nil
        }
        let value = number.int64Value
        guard value >= Int64(Int.min), value <= Int64(Int.max) else {
            return nil
        }
        return Int(value)
    }

    private static func isSha256(_ value: String) -> Bool {
        value.range(of: "^[0-9a-f]{64}$", options: .regularExpression) != nil
    }

    private static func matchingObjectEnd(in source: String, from start: String.Index) -> String.Index? {
        var depth = 0
        var inString = false
        var escaped = false
        var index = start
        while index < source.endIndex {
            let character = source[index]
            if inString {
                if escaped { escaped = false }
                else if character == "\\" { escaped = true }
                else if character == "\"" { inString = false }
            } else if character == "\"" {
                inString = true
            } else if character == "{" {
                depth += 1
            } else if character == "}" {
                depth -= 1
                if depth == 0 { return source.index(after: index) }
                if depth < 0 { return nil }
            }
            index = source.index(after: index)
        }
        return nil
    }
}
