import CoreFoundation
import Foundation

struct C131HitIdentityGridSummary {
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
    let mismatchPreview: [MismatchPreview]
    let expectedIdentitySha256: String
    let actualIdentitySha256: String
    let chromeObservationSha256: String
    let gridOracleSha256: String

    var isMatched: Bool {
        status == "matched"
            && pointCount == 30_000
            && matchedCount == 30_000
            && mismatchCount == 0
            && mismatchPreview.isEmpty
            && expectedIdentitySha256 == C131HitIdentityGridSummaryValidator.identitySha256
            && actualIdentitySha256 == expectedIdentitySha256
            && chromeObservationSha256 == C131HitIdentityGridSummaryValidator.chromeObservationSha256
            && gridOracleSha256 == C131HitIdentityGridSummaryValidator.gridOracleSha256
    }
}

enum C131HitIdentityGridSummaryValidator {
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
    static let identitySha256 = "93cccd042e82d8cd30cb7a8ab34a5154073d4a9eccbc4038294d3ca5b9dfce87"
    static let gridOracleSha256 = "35e88e57edc16e526e389317b0d18ef227e35d1356a2353b92476c75ef6414f6"
    static let chromeObservationSha256 = "515fc95f811399fda9e336a13e7583f2d22555fce4727f1fe7f734981abd0299"

    private static let summaryKeys: Set<String> = [
        "status", "point_count", "matched_count", "mismatch_count",
        "mismatch_preview_limit", "mismatch_preview", "expected_identity_sha256",
        "actual_identity_sha256", "chrome_observation_sha256", "grid_oracle_sha256",
    ]
    private static let previewKeys: Set<String> = [
        "xCssPx", "yCssPx", "expectedNodeId", "actualNodeId",
    ]
    private static let previewLimit = 16
    private static let pointCount = 30_000

    static func parse(_ report: String) throws -> C131HitIdentityGridSummary {
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
        // Rust serde_json emits this fixed summary as compact, sorted JSON. Comparing the full
        // object also rejects duplicate keys that JSONSerialization would otherwise overwrite.
        guard canonicalObject == rawObject else {
            throw ValidationError.nonCanonicalJson
        }

        guard let status = object["status"] as? String,
              let pointCount = integer(object["point_count"]),
              let matchedCount = integer(object["matched_count"]),
              let mismatchCount = integer(object["mismatch_count"]),
              let previewLimit = integer(object["mismatch_preview_limit"]),
              let rawPreview = object["mismatch_preview"] as? [[String: Any]],
              let expectedIdentitySha256 = object["expected_identity_sha256"] as? String,
              let actualIdentitySha256 = object["actual_identity_sha256"] as? String,
              let chromeObservationSha256 = object["chrome_observation_sha256"] as? String,
              let gridOracleSha256 = object["grid_oracle_sha256"] as? String else {
            throw ValidationError.invalidSchema
        }

        guard pointCount == Self.pointCount,
              (0...pointCount).contains(matchedCount),
              (0...pointCount).contains(mismatchCount),
              matchedCount + mismatchCount == pointCount,
              previewLimit == Self.previewLimit,
              rawPreview.count == min(mismatchCount, Self.previewLimit),
              (mismatchCount == 0 && status == "matched")
                  || (mismatchCount > 0 && status == "mismatch") else {
            throw ValidationError.invalidCounts
        }

        let preview = try rawPreview.map(parsePreview)
        guard expectedIdentitySha256 == Self.identitySha256,
              chromeObservationSha256 == Self.chromeObservationSha256,
              gridOracleSha256 == Self.gridOracleSha256,
              isSha256(actualIdentitySha256),
              (mismatchCount == 0
                  ? actualIdentitySha256 == Self.identitySha256
                  : actualIdentitySha256 != Self.identitySha256) else {
            throw ValidationError.invalidDigest
        }

        return C131HitIdentityGridSummary(
            status: status,
            pointCount: pointCount,
            matchedCount: matchedCount,
            mismatchCount: mismatchCount,
            mismatchPreview: preview,
            expectedIdentitySha256: expectedIdentitySha256,
            actualIdentitySha256: actualIdentitySha256,
            chromeObservationSha256: chromeObservationSha256,
            gridOracleSha256: gridOracleSha256
        )
    }

    private static func parsePreview(_ object: [String: Any]) throws -> C131HitIdentityGridSummary.MismatchPreview {
        guard Set(object.keys) == previewKeys,
              let x = integer(object["xCssPx"]),
              let y = integer(object["yCssPx"]),
              (0..<300).contains(x),
              (0..<100).contains(y),
              let expectedNodeId = object["expectedNodeId"] as? String,
              !expectedNodeId.isEmpty else {
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

        return C131HitIdentityGridSummary.MismatchPreview(
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
                    return source.index(after: index)
                }
                if depth < 0 {
                    return nil
                }
            }
            index = source.index(after: index)
        }
        return nil
    }
}
