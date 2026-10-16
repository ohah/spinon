import CryptoKit
import CoreFoundation
import Foundation

private struct C192MetalExpectedSample {
    let id: String
    let cssX: Int
    let cssY: Int
    let productRGBA: [Int]
    let chromeRGBA: [Int]
}

private struct C192MetalArtifactSample: Encodable {
    let id: String
    let cssX: Int
    let cssY: Int
    let drawableX: Int
    let drawableY: Int
    let rgba: [Int]
    let productExpectedRGBA: [Int]
    let chromeExpectedRGBA: [Int]
    let productExact: Bool
    let chromeExact: Bool
    let runID: String
    let generation: UInt64
}

private struct C192MetalArtifactManifest: Encodable {
    let schema: String
    let status: String
    let runID: String
    let generation: UInt64
    let viewportCssPx: [Int]
    let drawablePixelSize: [Int]
    let surfaceFormat: String
    let rgbaOrder: String
    let scaleX: Double
    let scaleY: Double
    let sampleCount: Int
    let productMatched: Int
    let chromeExact: Int
    let knownChromeDelta: String
    let sampleFile: String
    let sampleSha256: String
    let processID: Int32
}

struct C192MetalPixelCaptureSummary {
    let sampleCount: Int
    let productMatched: Int
    let chromeExact: Int
    let sampleFileName: String
    let manifestFileName: String
    let sampleSha256: String
    var productPassed: Bool { productMatched == sampleCount }
}

final class C192MetalSurfacePixelCapture {
    static let viewportWidth = 360
    static let viewportHeight = 240
    private static let expectedSamples: [C192MetalExpectedSample] = [
        .init(id: "rootBackdrop", cssX: 340, cssY: 232, productRGBA: [32, 49, 75, 255], chromeRGBA: [32, 49, 75, 255]),
        .init(id: "sourceAlpha", cssX: 26, cssY: 26, productRGBA: [143, 25, 38, 255], chromeRGBA: [143, 25, 38, 255]),
        .init(id: "groupSingle", cssX: 64, cssY: 26, productRGBA: [16, 89, 165, 255], chromeRGBA: [16, 89, 165, 255]),
        .init(id: "groupOverlap", cssX: 80, cssY: 26, productRGBA: [143, 121, 38, 255], chromeRGBA: [143, 121, 38, 255]),
        .init(id: "nestedOpacity", cssX: 128, cssY: 24, productRGBA: [24, 101, 56, 255], chromeRGBA: [24, 101, 56, 255]),
        .init(id: "alphaTimesGroupOpacity", cssX: 168, cssY: 24, productRGBA: [88, 37, 56, 255], chromeRGBA: [88, 37, 56, 255]),
        .init(id: "wideGamutSrgb", cssX: 208, cssY: 24, productRGBA: [0, 142, 196, 255], chromeRGBA: [0, 142, 200, 255]),
        .init(id: "opacityZero", cssX: 16, cssY: 72, productRGBA: [32, 49, 75, 255], chromeRGBA: [32, 49, 75, 255]),
        .init(id: "opacityHalfSingle", cssX: 84, cssY: 90, productRGBA: [143, 25, 38, 255], chromeRGBA: [143, 25, 38, 255]),
        .init(id: "opacityHalfOverlap", cssX: 68, cssY: 68, productRGBA: [16, 25, 165, 255], chromeRGBA: [16, 25, 165, 255]),
        .init(id: "opacityOneOverlap", cssX: 124, cssY: 68, productRGBA: [0, 0, 255, 255], chromeRGBA: [0, 0, 255, 255]),
        .init(id: "clippedZOverlap", cssX: 202, cssY: 76, productRGBA: [16, 25, 165, 255], chromeRGBA: [16, 25, 165, 255]),
        .init(id: "clippedRed", cssX: 212, cssY: 76, productRGBA: [143, 25, 38, 255], chromeRGBA: [143, 25, 38, 255]),
        .init(id: "clippedAway", cssX: 220, cssY: 76, productRGBA: [32, 49, 75, 255], chromeRGBA: [32, 49, 75, 255]),
        .init(id: "currentColorBorder", cssX: 234, cssY: 58, productRGBA: [28, 121, 102, 255], chromeRGBA: [28, 121, 102, 255]),
        .init(id: "borderInterior", cssX: 256, cssY: 80, productRGBA: [32, 49, 75, 255], chromeRGBA: [32, 49, 75, 255]),
    ]

    enum CaptureError: Error, CustomStringConvertible {
        case invalidRunID
        case artifactDirectoryUnavailable
        case invalidSurfaceSize
        case nonUniformScale
        case unsupportedScale
        case missingField(String)
        case invalidReport(String)
        case artifactWriteFailed(String)

        var description: String {
            switch self {
            case .invalidRunID: return "invalid-run-id"
            case .artifactDirectoryUnavailable: return "artifact-directory-unavailable"
            case .invalidSurfaceSize: return "invalid-surface-size"
            case .nonUniformScale: return "non-uniform-drawable-scale"
            case .unsupportedScale: return "unsupported-drawable-scale"
            case let .missingField(field): return "missing-report-field-\(field)"
            case let .invalidReport(reason): return "invalid-report-\(reason)"
            case let .artifactWriteFailed(reason): return "artifact-write-failed-\(reason)"
            }
        }
    }

    let runID: String
    let directoryURL: URL
    private let sampleURL: URL
    private let manifestURL: URL

    init(runID: String) throws {
        guard runID.range(of: "^[A-Za-z0-9_-]{1,80}$", options: .regularExpression) != nil else {
            throw CaptureError.invalidRunID
        }
        self.runID = runID
        guard let caches = FileManager.default.urls(
            for: .cachesDirectory,
            in: .userDomainMask
        ).first else {
            throw CaptureError.artifactDirectoryUnavailable
        }
        directoryURL = caches.appendingPathComponent("c19-2-metal-surface-pixels", isDirectory: true)
        try FileManager.default.createDirectory(at: directoryURL, withIntermediateDirectories: true)
        sampleURL = directoryURL.appendingPathComponent("c192-metal-pixels-\(runID).jsonl")
        manifestURL = directoryURL.appendingPathComponent("c192-metal-pixels-\(runID).manifest.json")
    }

    func coordinates(drawableWidth: Int, drawableHeight: Int) throws -> [UInt32] {
        guard drawableWidth >= Self.viewportWidth, drawableHeight >= Self.viewportHeight else {
            throw CaptureError.invalidSurfaceSize
        }
        let scaleX = Double(drawableWidth) / Double(Self.viewportWidth)
        let scaleY = Double(drawableHeight) / Double(Self.viewportHeight)
        guard scaleX.isFinite, scaleY.isFinite, abs(scaleX - scaleY) < 0.000001 else {
            throw CaptureError.nonUniformScale
        }
        guard (1...8).contains(Int(scaleX.rounded())),
              abs(scaleX - scaleX.rounded()) < 0.000001 else {
            throw CaptureError.unsupportedScale
        }
        var result: [UInt32] = []
        result.reserveCapacity(Self.expectedSamples.count * 2)
        for sample in Self.expectedSamples {
            let x = floor((Double(sample.cssX) + 0.5) * scaleX)
            let y = floor((Double(sample.cssY) + 0.5) * scaleY)
            guard x >= 0, y >= 0, x < Double(drawableWidth), y < Double(drawableHeight),
                  let drawableX = UInt32(exactly: x), let drawableY = UInt32(exactly: y) else {
                throw CaptureError.invalidSurfaceSize
            }
            result.append(drawableX)
            result.append(drawableY)
        }
        return result
    }

    func accept(
        report: String,
        generation: UInt64,
        drawableWidth: Int,
        drawableHeight: Int,
        surfaceFormat: String
    ) throws -> C192MetalPixelCaptureSummary {
        let surfaceText = try field(report, "surface_texture=")
            .split(whereSeparator: { $0.isWhitespace })
            .first ?? ""
        let dimensions = surfaceText
            .split(separator: "x")
            .compactMap { Int($0) }
        guard dimensions == [drawableWidth, drawableHeight] else {
            throw CaptureError.invalidReport("surface-size")
        }
        let encodedSamples = try field(report, "surface_pixel_samples_json=")
        guard let data = encodedSamples.data(using: .utf8),
              let values = try? JSONSerialization.jsonObject(with: data) as? [[String: Any]],
              values.count == Self.expectedSamples.count else {
            throw CaptureError.invalidReport("sample-count-or-json")
        }
        let coordinates = try coordinates(drawableWidth: drawableWidth, drawableHeight: drawableHeight)
        let scaleX = Double(drawableWidth) / Double(Self.viewportWidth)
        let scaleY = Double(drawableHeight) / Double(Self.viewportHeight)
        var productMatched = 0
        var chromeExact = 0
        var lines: [Data] = []
        let encoder = JSONEncoder()
        encoder.outputFormatting = [.sortedKeys]
        for (index, expected) in Self.expectedSamples.enumerated() {
            let object = values[index]
            guard Set(object.keys) == Set(["x", "y", "rgba"]),
                  let x = integer(object["x"]), let y = integer(object["y"]),
                  let channels = object["rgba"] as? [NSNumber], channels.count == 4 else {
                throw CaptureError.invalidReport("sample-shape-\(index)")
            }
            let rgba = channels.map(\.intValue)
            let expectedX = Int(coordinates[index * 2])
            let expectedY = Int(coordinates[index * 2 + 1])
            let productExact = x == expectedX && y == expectedY && rgba == expected.productRGBA
            let chromeMatches = x == expectedX && y == expectedY && rgba == expected.chromeRGBA
            if productExact { productMatched += 1 }
            if chromeMatches { chromeExact += 1 }
            lines.append(try encoder.encode(C192MetalArtifactSample(
                id: expected.id,
                cssX: expected.cssX,
                cssY: expected.cssY,
                drawableX: x,
                drawableY: y,
                rgba: rgba,
                productExpectedRGBA: expected.productRGBA,
                chromeExpectedRGBA: expected.chromeRGBA,
                productExact: productExact,
                chromeExact: chromeMatches,
                runID: runID,
                generation: generation
            )))
        }
        let jsonl = lines.reduce(into: Data()) { result, line in
            result.append(line)
            result.append(10)
        }
        do {
            try jsonl.write(to: sampleURL, options: .atomic)
        } catch {
            throw CaptureError.artifactWriteFailed("samples")
        }
        let hash = SHA256.hash(data: jsonl).map { String(format: "%02x", $0) }.joined()
        let manifest = C192MetalArtifactManifest(
            schema: "spinon-c19-2-metal-pixel-capture/v1",
            status: productMatched == Self.expectedSamples.count ? "product-matched" : "mismatch",
            runID: runID,
            generation: generation,
            viewportCssPx: [Self.viewportWidth, Self.viewportHeight],
            drawablePixelSize: [drawableWidth, drawableHeight],
            surfaceFormat: surfaceFormat,
            rgbaOrder: "RGBA8",
            scaleX: scaleX,
            scaleY: scaleY,
            sampleCount: values.count,
            productMatched: productMatched,
            chromeExact: chromeExact,
            knownChromeDelta: "wideGamutSrgb product blue=196 Chrome155 blue=200",
            sampleFile: sampleURL.lastPathComponent,
            sampleSha256: hash,
            processID: ProcessInfo.processInfo.processIdentifier
        )
        do {
            let manifestData = try encoder.encode(manifest)
            try manifestData.write(to: manifestURL, options: .atomic)
        } catch {
            throw CaptureError.artifactWriteFailed("manifest")
        }
        return C192MetalPixelCaptureSummary(
            sampleCount: values.count,
            productMatched: productMatched,
            chromeExact: chromeExact,
            sampleFileName: sampleURL.lastPathComponent,
            manifestFileName: manifestURL.lastPathComponent,
            sampleSha256: hash
        )
    }

    private func field(_ report: String, _ name: String) throws -> String {
        guard let marker = report.range(of: name) else { throw CaptureError.missingField(name) }
        let value = String(report[marker.upperBound...]).trimmingCharacters(in: .whitespacesAndNewlines)
        guard !value.isEmpty else { throw CaptureError.missingField(name) }
        return value
    }

    private func integer(_ value: Any?) -> Int? {
        guard let number = value as? NSNumber,
              CFGetTypeID(number) != CFBooleanGetTypeID() else { return nil }
        return number.intValue
    }
}
