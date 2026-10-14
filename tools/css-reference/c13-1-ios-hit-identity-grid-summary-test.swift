import Foundation

@main
enum C131HitIdentityGridSummaryTests {
    private static let marker = C131HitIdentityGridSummaryValidator.marker
    private static var negativeControls = 0

    static func main() throws {
        var assertions = 0
        let exact = try makeJson()
        let parsed = try C131HitIdentityGridSummaryValidator.parse(report(exact))
        assert(parsed.isMatched, "pinned summary should match")
        assert(parsed.pointCount == 30_000 && parsed.matchedCount == 30_000, "matched counts")
        assertions += 2

        var mismatched = baseObject()
        mismatched["status"] = "mismatch"
        mismatched["matched_count"] = 29_999
        mismatched["mismatch_count"] = 1
        mismatched["actual_identity_sha256"] = String(repeating: "f", count: 64)
        mismatched["mismatch_preview"] = [[
            "xCssPx": 12,
            "yCssPx": 34,
            "expectedNodeId": "c131-surface-target-visible",
            "actualNodeId": NSNull(),
        ]]
        let mismatchReport = report(try canonicalJson(mismatched))
        let mismatch = try C131HitIdentityGridSummaryValidator.parse(mismatchReport)
        assert(!mismatch.isMatched, "valid mismatch report must never pass")
        assert(mismatch.mismatchPreview.first?.actualNodeId == nil, "null actual identity remains null")
        assertions += 2

        var escapedPreview = mismatched
        escapedPreview["mismatch_preview"] = [[
            "xCssPx": 12,
            "yCssPx": 34,
            "expectedNodeId": "c131-surface-target-visible",
            "actualNodeId": "node } with quote \" and slash \\",
        ]]
        let escaped = try C131HitIdentityGridSummaryValidator.parse(report(try canonicalJson(escapedPreview)))
        assert(escaped.mismatchPreview.first?.actualNodeId == "node } with quote \" and slash \\",
               "nested braces and escaped quotes")
        assertions += 1

        assertRejected("missing marker", "status=0 layout=ready")
        assertRejected("duplicate marker", report(exact) + " " + marker + exact)
        assertRejected("missing object", "prefix \(marker)not-json")
        assertRejected("truncated object", "prefix \(marker)\(String(exact.dropLast()))")
        assertRejected("malformed JSON", "prefix \(marker){broken}")

        var missingField = baseObject()
        missingField.removeValue(forKey: "grid_oracle_sha256")
        assertRejected("missing field", report(try canonicalJson(missingField)))
        var extraField = baseObject()
        extraField["unexpected"] = "value"
        assertRejected("extra field", report(try canonicalJson(extraField)))
        assertRejected("duplicate field", report(String(exact.dropLast()) + ",\"status\":\"matched\"}"))

        assertRejected("boolean count", report(exact.replacingOccurrences(of: "\"point_count\":30000", with: "\"point_count\":true")))
        assertRejected("fractional count", report(exact.replacingOccurrences(of: "\"point_count\":30000", with: "\"point_count\":30000.5")))
        assertRejected("string count", report(exact.replacingOccurrences(of: "\"point_count\":30000", with: "\"point_count\":\"30000\"")))

        var wrongCounts = baseObject()
        wrongCounts["matched_count"] = 29_999
        assertRejected("counts do not sum", report(try canonicalJson(wrongCounts)))
        var wrongStatus = baseObject()
        wrongStatus["status"] = "mismatch"
        assertRejected("status contradicts counts", report(try canonicalJson(wrongStatus)))
        var wrongPointCount = baseObject()
        wrongPointCount["point_count"] = 29_999
        assertRejected("point count", report(try canonicalJson(wrongPointCount)))

        for key in ["expected_identity_sha256", "actual_identity_sha256", "chrome_observation_sha256", "grid_oracle_sha256"] {
            var changed = baseObject()
            changed[key] = String(repeating: "e", count: 64)
            assertRejected("changed \(key)", report(try canonicalJson(changed)))
        }

        var previewOverflow = mismatched
        previewOverflow["mismatch_count"] = 17
        previewOverflow["matched_count"] = 29_983
        previewOverflow["mismatch_preview"] = (0..<17).map { index in
            ["xCssPx": index, "yCssPx": 1, "expectedNodeId": "node-\(index)", "actualNodeId": NSNull()] as [String: Any]
        }
        assertRejected("preview over limit", report(try canonicalJson(previewOverflow)))

        for (label, x, y) in [("x upper bound", 300, 1), ("y upper bound", 1, 100), ("negative x", -1, 1)] {
            var invalidPreview = mismatched
            invalidPreview["mismatch_preview"] = [[
                "xCssPx": x,
                "yCssPx": y,
                "expectedNodeId": "node",
                "actualNodeId": NSNull(),
            ]]
            assertRejected(label, report(try canonicalJson(invalidPreview)))
        }

        var invalidPreviewType = mismatched
        invalidPreviewType["mismatch_preview"] = [[
            "xCssPx": 1,
            "yCssPx": 1,
            "expectedNodeId": "node",
            "actualNodeId": 7,
        ]]
        assertRejected("preview actual type", report(try canonicalJson(invalidPreviewType)))
        print("C13.1 iOS hit identity grid summary parser: \(assertions) positive assertions and \(negativeControls) negative controls passed")
    }

    private static func assertRejected(_ label: String, _ report: String, file: StaticString = #file, line: UInt = #line) {
        negativeControls += 1
        do {
            _ = try C131HitIdentityGridSummaryValidator.parse(report)
            assertionFailure("\(label): malformed summary accepted", file: file, line: line)
        } catch {
            // A rejected report cannot be counted as a matching runtime result.
        }
    }

    private static func report(_ object: String) -> String {
        "status=0 layout=ready boxes=41 generation=7 hit_identity_grid_summary_json=\(object) hit_test_samples_json=[]"
    }

    private static func makeJson() throws -> String {
        try canonicalJson(baseObject())
    }

    private static func canonicalJson(_ object: [String: Any]) throws -> String {
        let data = try JSONSerialization.data(withJSONObject: object, options: [.sortedKeys])
        guard let string = String(data: data, encoding: .utf8) else {
            throw NSError(domain: "C131HitIdentityGridSummaryTests", code: 1)
        }
        return string
    }

    private static func baseObject() -> [String: Any] {
        [
            "status": "matched",
            "point_count": 30_000,
            "matched_count": 30_000,
            "mismatch_count": 0,
            "mismatch_preview_limit": 16,
            "mismatch_preview": [Any](),
            "expected_identity_sha256": C131HitIdentityGridSummaryValidator.identitySha256,
            "actual_identity_sha256": C131HitIdentityGridSummaryValidator.identitySha256,
            "chrome_observation_sha256": C131HitIdentityGridSummaryValidator.chromeObservationSha256,
            "grid_oracle_sha256": C131HitIdentityGridSummaryValidator.gridOracleSha256,
        ]
    }
}
