import Foundation
import OSLog
import UIKit

private struct C124MatrixInput: Decodable {
    struct Inventory: Decodable {
        let path: String
        let sha256: String
    }

    struct Expected: Decodable {
        let kind: String
        let property: String?
        let api: String?
    }

    struct MatrixCase: Decodable {
        let id: String
        let expected: Expected
        let source: String
    }

    struct Summary: Decodable {
        let cases: Int
        let nodes: Int
        let expectedSupported: Int
        let expectedRejected: Int
    }

    let schema: String
    let inventory: Inventory
    let cases: [MatrixCase]
    let summary: Summary
}

private struct C124MatrixCaseResult: Encodable {
    let index: Int
    let id: String
    let expected: String
    let actual: String
    let reasonMatched: Bool
    let matched: Bool
    let report: String
}

private struct C124MatrixResult: Encodable {
    let schema: String
    let runID: String
    let platform: String
    let device: String
    let osVersion: String
    let inventorySHA256: String
    let status: String
    let cases: Int
    let nodes: Int
    let expectedReady: Int
    let expectedRejected: Int
    let matched: Int
    let results: [C124MatrixCaseResult]
    let error: String?
}

enum C124RuntimeMatrixProbe {
    private static let logger = Logger(subsystem: "dev.spinon.bootstrap", category: "c124-matrix")
    private static let inputName = "c12-4-runtime-matrix-input.json"

    static func run(runID: String) {
        guard runID.range(of: "^[A-Za-z0-9_-]{1,80}$", options: .regularExpression) != nil else {
            logger.error("SPINON_C124_IOS_MATRIX status=error reason=invalid-run-id")
            return
        }

        let directory = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
        let outputURL = directory.appendingPathComponent("c124-runtime-matrix-\(runID).json")
        do {
            try? FileManager.default.removeItem(at: outputURL)
            let inputURL = directory.appendingPathComponent(inputName)
            let inputData = try Data(contentsOf: inputURL)
            let matrix = try JSONDecoder().decode(C124MatrixInput.self, from: inputData)
            guard matrix.schema == "spinon-css-c12-4-runtime-matrix/v1",
                  matrix.cases.count == 26,
                  matrix.summary.cases == 26,
                  matrix.summary.nodes == 77,
                  matrix.summary.expectedSupported == 21,
                  matrix.summary.expectedRejected == 5,
                  Set(matrix.cases.map(\.id)).count == 26 else {
                throw ProbeError.invalidMatrix
            }

            var results: [C124MatrixCaseResult] = []
            for (index, fixtureCase) in matrix.cases.enumerated() {
                results.append(evaluate(fixtureCase, index: index))
            }

            let matched = results.filter(\.matched).count
            let result = C124MatrixResult(
                schema: "spinon-c124-ios-runtime-matrix/v1",
                runID: runID,
                platform: "iOS Simulator",
                device: UIDevice.current.model,
                osVersion: UIDevice.current.systemVersion,
                inventorySHA256: matrix.inventory.sha256,
                status: results.count == 26 && matched == 26 ? "matched" : "mismatch",
                cases: results.count,
                nodes: matrix.summary.nodes,
                expectedReady: matrix.summary.expectedSupported,
                expectedRejected: matrix.summary.expectedRejected,
                matched: matched,
                results: results,
                error: nil
            )
            try write(result, to: outputURL)
            logger.notice("SPINON_C124_IOS_MATRIX status=\(result.status, privacy: .public) run_id=\(runID, privacy: .public) cases=\(result.cases, privacy: .public) nodes=\(result.nodes, privacy: .public) matched=\(result.matched, privacy: .public)")
        } catch {
            let result = C124MatrixResult(
                schema: "spinon-c124-ios-runtime-matrix/v1",
                runID: runID,
                platform: "iOS Simulator",
                device: UIDevice.current.model,
                osVersion: UIDevice.current.systemVersion,
                inventorySHA256: "",
                status: "error",
                cases: 0,
                nodes: 0,
                expectedReady: 0,
                expectedRejected: 0,
                matched: 0,
                results: [],
                error: String(describing: error)
            )
            do {
                try write(result, to: outputURL)
            } catch {
                logger.error("SPINON_C124_IOS_MATRIX status=error run_id=\(runID, privacy: .public) output_write_failed=\(String(describing: error), privacy: .public)")
            }
            logger.error("SPINON_C124_IOS_MATRIX status=error run_id=\(runID, privacy: .public) reason=\(String(describing: error), privacy: .public)")
        }
    }

    private static func evaluate(
        _ fixtureCase: C124MatrixInput.MatrixCase,
        index: Int
    ) -> C124MatrixCaseResult {
        let supported = fixtureCase.expected.kind == "supported"
        let expected = supported ? "ready" : "reject"
        let handle = SpinonRunner.createRuntimeGpuHostWithC12_4StackingFixture()
        guard handle != 0 else {
            return C124MatrixCaseResult(
                index: index, id: fixtureCase.id, expected: expected,
                actual: "host-create-failed", reasonMatched: false, matched: false,
                report: "status=-1 host-create-failed"
            )
        }
        defer { SpinonRunner.freeRuntimeGpuHost(handle) }

        let environment = SpinonRunner.setRuntimeGpuEnvironment(
            handle, width: 320, height: 240, scale: 1, dark: false
        ) ?? "status=-2 environment-report-unavailable"
        guard environment.hasPrefix("status=0 ") else {
            return C124MatrixCaseResult(
                index: index, id: fixtureCase.id, expected: expected,
                actual: "environment-failed", reasonMatched: false, matched: false,
                report: environment
            )
        }

        let report = SpinonRunner.evalRuntimeGpuC12_4Source(handle, source: fixtureCase.source)
            ?? "status=-3 evaluation-report-unavailable"
        let ready = report.hasPrefix("status=0 ") && report.contains("layout=ready")
        let rejected = report.contains(" op=eval ") && !report.hasPrefix("status=0 ")
        let reasonMatched = supported || matchesExpectedDiagnostic(fixtureCase.expected, report: report)
        let matched = supported ? ready : rejected && reasonMatched
        let actual = ready ? "ready" : rejected
            ? reasonMatched ? "reject" : "reject-wrong-diagnostic"
            : "invalid-layout"

        logger.notice("SPINON_C124_IOS_MATRIX_CASE index=\(index, privacy: .public) id=\(fixtureCase.id, privacy: .public) expected=\(expected, privacy: .public) actual=\(actual, privacy: .public) matched=\(matched, privacy: .public)")
        return C124MatrixCaseResult(
            index: index, id: fixtureCase.id, expected: expected,
            actual: actual, reasonMatched: reasonMatched, matched: matched,
            report: report
        )
    }

    private static func matchesExpectedDiagnostic(
        _ expected: C124MatrixInput.Expected,
        report: String
    ) -> Bool {
        if expected.property == "pseudo-element" {
            return report.contains("cascade_error=")
                && report.contains("지원하지 않는 CSS 선언 content")
        }
        if let property = expected.property {
            return report.contains("node=Some(")
                && report.contains("property=Some(\"\(property)\")")
        }
        return expected.api == "showModal"
            && report.contains("TypeError:")
            && report.contains("showModal is not a function")
    }

    private static func write(_ result: C124MatrixResult, to url: URL) throws {
        let encoder = JSONEncoder()
        encoder.outputFormatting = [.prettyPrinted, .sortedKeys]
        try encoder.encode(result).write(to: url, options: .atomic)
    }

    private enum ProbeError: Error {
        case invalidMatrix
    }
}
