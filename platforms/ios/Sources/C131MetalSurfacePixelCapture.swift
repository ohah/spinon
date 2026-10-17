import CoreFoundation
import CryptoKit
import Foundation

struct C131MetalPixelCoordinate {
    let index: Int
    let cssX: Int?
    let cssY: Int?
    let drawableX: Int
    let drawableY: Int
}

protocol C131MetalSurfacePixelCaptureProtocol {
    var expectedSampleCount: Int { get }
    var expectedBatchCount: Int { get }

    func nextBatch(
        drawableWidth: Int,
        drawableHeight: Int,
        generation: UInt64,
        format: String
    ) throws -> [C131MetalPixelCoordinate]?

    func acceptDrawReport(
        _ report: String,
        generation: UInt64,
        drawableWidth: Int,
        drawableHeight: Int,
        format: String
    ) throws -> C131MetalPixelCaptureProgress

    func abort()
}

private struct C131MetalPixelArtifactSample: Encodable {
    let index: Int
    let cssX: Int
    let cssY: Int
    let drawableX: Int
    let drawableY: Int
    let rgba: [Int]
    let runId: String
    let generation: UInt64
}

private struct C131MetalPixelArtifactManifest: Encodable {
    let schema: String
    let status: String
    let runId: String
    let generation: UInt64
    let viewportCssPx: [Int]
    let drawablePixelSize: [Int]
    let surfaceFormat: String
    let rgbaOrder: String
    let scaleX: Double
    let scaleY: Double
    let sampleCount: Int
    let batchSize: Int
    let batchCount: Int
    let sampleFile: String
    let sampleSha256: String
    let processID: Int32
    let elapsedMilliseconds: UInt64
}

struct C131MetalPixelCaptureProgress {
    let capturedSamples: Int
    let totalSamples: Int
    let capturedBatches: Int
    let totalBatches: Int
    let complete: Bool
    let sampleFileName: String?
    let manifestFileName: String?
}

final class C131MetalSurfacePixelCapture: C131MetalSurfacePixelCaptureProtocol {
    static let viewportWidth = 300
    static let viewportHeight = 100
    static let sampleCount = viewportWidth * viewportHeight
    static let batchSize = 64
    static let totalBatchCount = (sampleCount + batchSize - 1) / batchSize
    static let totalDeadlineSeconds: TimeInterval = 300
    static let batchDeadlineSeconds: TimeInterval = 15
    var expectedSampleCount: Int { Self.sampleCount }
    var expectedBatchCount: Int { Self.totalBatchCount }

    enum CaptureError: Error, CustomStringConvertible {
        case invalidRunID
        case artifactDirectoryUnavailable
        case invalidSurfaceSize
        case invalidSurfaceFormat
        case scaleTooSmall
        case duplicateDrawableCoordinate
        case generationChanged
        case surfaceChanged
        case batchAlreadyPending
        case batchMissing
        case totalDeadlineExceeded
        case batchDeadlineExceeded
        case missingReportField(String)
        case invalidReportField(String)
        case invalidSampleCount
        case invalidSampleShape(Int)
        case sampleCoordinateMismatch(Int)
        case sampleColorInvalid(Int)
        case artifactWriteFailed(String)

        var description: String {
            switch self {
            case .invalidRunID: return "invalid-run-id"
            case .artifactDirectoryUnavailable: return "artifact-directory-unavailable"
            case .invalidSurfaceSize: return "invalid-surface-size"
            case .invalidSurfaceFormat: return "invalid-surface-format"
            case .scaleTooSmall: return "drawable-scale-below-one"
            case .duplicateDrawableCoordinate: return "duplicate-drawable-coordinate"
            case .generationChanged: return "surface-generation-changed"
            case .surfaceChanged: return "surface-size-or-format-changed"
            case .batchAlreadyPending: return "batch-already-pending"
            case .batchMissing: return "batch-missing"
            case .totalDeadlineExceeded: return "total-deadline-exceeded"
            case .batchDeadlineExceeded: return "batch-deadline-exceeded"
            case let .missingReportField(field): return "missing-report-field-\(field)"
            case let .invalidReportField(field): return "invalid-report-field-\(field)"
            case .invalidSampleCount: return "invalid-sample-count"
            case let .invalidSampleShape(index): return "invalid-sample-shape-\(index)"
            case let .sampleCoordinateMismatch(index): return "sample-coordinate-mismatch-\(index)"
            case let .sampleColorInvalid(index): return "sample-color-invalid-\(index)"
            case let .artifactWriteFailed(reason): return "artifact-write-failed-\(reason)"
            }
        }
    }

    let runID: String
    let directoryURL: URL
    private let startedAt = ProcessInfo.processInfo.systemUptime
    private let partURL: URL
    private let sampleURL: URL
    private let manifestPartURL: URL
    private let manifestURL: URL
    private var fileHandle: FileHandle?
    private var nextIndex = 0
    private var pendingBatch: [C131MetalPixelCoordinate]?
    private var pendingAt: TimeInterval?
    private var surfaceGeneration: UInt64?
    private var drawableWidth: Int?
    private var drawableHeight: Int?
    private var surfaceFormat: String?
    private var finished = false

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
        let directory = caches.appendingPathComponent("c13-1-metal-surface-pixel-centers", isDirectory: true)
        try FileManager.default.createDirectory(at: directory, withIntermediateDirectories: true)
        directoryURL = directory
        let stem = "c131-metal-pixel-centers-\(runID)"
        partURL = directory.appendingPathComponent("\(stem).jsonl.part")
        sampleURL = directory.appendingPathComponent("\(stem).jsonl")
        manifestPartURL = directory.appendingPathComponent("\(stem).manifest.json.part")
        manifestURL = directory.appendingPathComponent("\(stem).manifest.json")
        guard FileManager.default.createFile(atPath: partURL.path, contents: nil) else {
            throw CaptureError.artifactWriteFailed("create-sample-file")
        }
        fileHandle = try FileHandle(forWritingTo: partURL)
    }

    deinit {
        abort()
    }

    func nextBatch(
        drawableWidth proposedWidth: Int,
        drawableHeight proposedHeight: Int,
        generation proposedGeneration: UInt64,
        format proposedFormat: String
    ) throws -> [C131MetalPixelCoordinate]? {
        guard !finished else { return nil }
        guard ProcessInfo.processInfo.systemUptime - startedAt <= Self.totalDeadlineSeconds else {
            throw CaptureError.totalDeadlineExceeded
        }
        guard pendingBatch == nil else { throw CaptureError.batchAlreadyPending }
        guard proposedWidth > 0, proposedHeight > 0 else {
            throw CaptureError.invalidSurfaceSize
        }
        guard ["Rgba8Unorm", "Rgba8UnormSrgb", "Bgra8Unorm", "Bgra8UnormSrgb"]
            .contains(proposedFormat) else {
            throw CaptureError.invalidSurfaceFormat
        }
        let scaleX = Double(proposedWidth) / Double(Self.viewportWidth)
        let scaleY = Double(proposedHeight) / Double(Self.viewportHeight)
        guard scaleX.isFinite, scaleY.isFinite, scaleX >= 1, scaleY >= 1 else {
            throw CaptureError.scaleTooSmall
        }
        if let surfaceGeneration, surfaceGeneration != proposedGeneration {
            throw CaptureError.generationChanged
        }
        if surfaceGeneration == nil {
            surfaceGeneration = proposedGeneration
            drawableWidth = proposedWidth
            drawableHeight = proposedHeight
            surfaceFormat = proposedFormat
        } else if drawableWidth != proposedWidth || drawableHeight != proposedHeight
                    || surfaceFormat != proposedFormat {
            throw CaptureError.surfaceChanged
        }
        guard nextIndex < Self.sampleCount else { return nil }

        let endIndex = min(nextIndex + Self.batchSize, Self.sampleCount)
        var coordinates: [C131MetalPixelCoordinate] = []
        coordinates.reserveCapacity(endIndex - nextIndex)
        var seen = Set<String>()
        for index in nextIndex..<endIndex {
            let cssX = index % Self.viewportWidth
            let cssY = index / Self.viewportWidth
            let drawableX = Int(floor((Double(cssX) + 0.5) * scaleX))
            let drawableY = Int(floor((Double(cssY) + 0.5) * scaleY))
            guard drawableX >= 0, drawableY >= 0,
                  drawableX < proposedWidth, drawableY < proposedHeight else {
                throw CaptureError.invalidSurfaceSize
            }
            guard seen.insert("\(drawableX),\(drawableY)").inserted else {
                throw CaptureError.duplicateDrawableCoordinate
            }
            coordinates.append(C131MetalPixelCoordinate(
                index: index,
                cssX: cssX,
                cssY: cssY,
                drawableX: drawableX,
                drawableY: drawableY
            ))
        }
        pendingBatch = coordinates
        pendingAt = ProcessInfo.processInfo.systemUptime
        return coordinates
    }

    func acceptDrawReport(
        _ report: String,
        generation observedGeneration: UInt64,
        drawableWidth observedWidth: Int,
        drawableHeight observedHeight: Int,
        format observedFormat: String
    ) throws -> C131MetalPixelCaptureProgress {
        guard !finished, let pendingBatch else { throw CaptureError.batchMissing }
        guard ProcessInfo.processInfo.systemUptime - startedAt <= Self.totalDeadlineSeconds else {
            throw CaptureError.totalDeadlineExceeded
        }
        guard let pendingAt,
              ProcessInfo.processInfo.systemUptime - pendingAt <= Self.batchDeadlineSeconds else {
            throw CaptureError.batchDeadlineExceeded
        }
        guard surfaceGeneration == observedGeneration else { throw CaptureError.generationChanged }
        guard drawableWidth == observedWidth, drawableHeight == observedHeight,
              surfaceFormat == observedFormat else {
            throw CaptureError.surfaceChanged
        }
        guard let surfaceSize = field(report, "surface_texture=") else {
            throw CaptureError.missingReportField("surface_texture")
        }
        let dimensions = surfaceSize.split(separator: "x").compactMap { Int($0) }
        guard dimensions == [observedWidth, observedHeight] else {
            throw CaptureError.surfaceChanged
        }
        guard let encoded = field(report, "surface_pixel_samples_json="),
              let data = encoded.data(using: .utf8),
              let samples = try? JSONSerialization.jsonObject(with: data) as? [[String: Any]] else {
            throw CaptureError.missingReportField("surface_pixel_samples_json")
        }
        guard samples.count == pendingBatch.count else { throw CaptureError.invalidSampleCount }

        do {
            for (offset, coordinate) in pendingBatch.enumerated() {
                let object = samples[offset]
                guard Set(object.keys) == Set(["x", "y", "rgba"]),
                      let x = integer(object["x"]), let y = integer(object["y"]),
                      let channels = object["rgba"] as? [NSNumber], channels.count == 4 else {
                    throw CaptureError.invalidSampleShape(coordinate.index)
                }
                guard x == coordinate.drawableX, y == coordinate.drawableY else {
                    throw CaptureError.sampleCoordinateMismatch(coordinate.index)
                }
                guard let cssX = coordinate.cssX, let cssY = coordinate.cssY else {
                    throw CaptureError.invalidSampleShape(coordinate.index)
                }
                let rgba = channels.compactMap(integer)
                guard rgba.count == 4, rgba.allSatisfy({ (0...255).contains($0) }) else {
                    throw CaptureError.sampleColorInvalid(coordinate.index)
                }
                let artifactSample = C131MetalPixelArtifactSample(
                    index: coordinate.index,
                    cssX: cssX,
                    cssY: cssY,
                    drawableX: coordinate.drawableX,
                    drawableY: coordinate.drawableY,
                    rgba: rgba,
                    runId: runID,
                    generation: observedGeneration
                )
                let encoder = JSONEncoder()
                encoder.outputFormatting = [.sortedKeys]
                var line = try encoder.encode(artifactSample)
                line.append(0x0a)
                guard let fileHandle else {
                    throw CaptureError.artifactWriteFailed("sample-file-closed")
                }
                try fileHandle.write(contentsOf: line)
            }
        } catch let error as CaptureError {
            throw error
        } catch {
            throw CaptureError.artifactWriteFailed(String(describing: error))
        }

        nextIndex += pendingBatch.count
        self.pendingBatch = nil
        self.pendingAt = nil
        let complete = nextIndex == Self.sampleCount
        if complete {
            try finalize()
        }
        return C131MetalPixelCaptureProgress(
            capturedSamples: nextIndex,
            totalSamples: Self.sampleCount,
            capturedBatches: (nextIndex + Self.batchSize - 1) / Self.batchSize,
            totalBatches: Self.totalBatchCount,
            complete: complete,
            sampleFileName: complete ? sampleURL.lastPathComponent : nil,
            manifestFileName: complete ? manifestURL.lastPathComponent : nil
        )
    }

    func abort() {
        guard !finished else { return }
        try? fileHandle?.close()
        fileHandle = nil
        try? FileManager.default.removeItem(at: partURL)
        try? FileManager.default.removeItem(at: manifestPartURL)
        try? FileManager.default.removeItem(at: sampleURL)
        try? FileManager.default.removeItem(at: manifestURL)
        finished = true
    }

    private func finalize() throws {
        guard let generation = surfaceGeneration,
              let width = drawableWidth, let height = drawableHeight,
              let format = surfaceFormat else {
            throw CaptureError.invalidSurfaceSize
        }
        do {
            guard let activeFileHandle = fileHandle else {
                throw CaptureError.artifactWriteFailed("sample-file-closed-before-finalize")
            }
            try activeFileHandle.synchronize()
            try activeFileHandle.close()
            fileHandle = nil
            let sampleDigest = SHA256.hash(data: try Data(contentsOf: partURL))
                .map { String(format: "%02x", $0) }
                .joined()
            try FileManager.default.moveItem(at: partURL, to: sampleURL)
            let encoder = JSONEncoder()
            encoder.outputFormatting = [.sortedKeys]
            let manifest = C131MetalPixelArtifactManifest(
                schema: "spinon-c13-1-ios-metal-pixel-centers/v1",
                status: "complete",
                runId: runID,
                generation: generation,
                viewportCssPx: [Self.viewportWidth, Self.viewportHeight],
                drawablePixelSize: [width, height],
                surfaceFormat: format,
                rgbaOrder: "RGBA8",
                scaleX: Double(width) / Double(Self.viewportWidth),
                scaleY: Double(height) / Double(Self.viewportHeight),
                sampleCount: nextIndex,
                batchSize: Self.batchSize,
                batchCount: (nextIndex + Self.batchSize - 1) / Self.batchSize,
                sampleFile: sampleURL.lastPathComponent,
                sampleSha256: sampleDigest,
                processID: ProcessInfo.processInfo.processIdentifier,
                elapsedMilliseconds: UInt64((ProcessInfo.processInfo.systemUptime - startedAt) * 1000)
            )
            try encoder.encode(manifest).write(to: manifestPartURL, options: .atomic)
            try FileManager.default.moveItem(at: manifestPartURL, to: manifestURL)
            finished = true
        } catch {
            throw CaptureError.artifactWriteFailed(String(describing: error))
        }
    }

    private func field(_ report: String, _ marker: String) -> String? {
        guard let range = report.range(of: marker) else { return nil }
        let value = report[range.upperBound...]
        return value.split(whereSeparator: \.isWhitespace).first.map(String.init)
    }

    private func integer(_ value: Any?) -> Int? {
        guard let number = value as? NSNumber,
              CFGetTypeID(number) != CFBooleanGetTypeID() else {
            return nil
        }
        let type = String(cString: number.objCType)
        guard "cCsSiIlLqQ".contains(type) else { return nil }
        let value = number.int64Value
        guard value >= Int64(Int.min), value <= Int64(Int.max) else { return nil }
        return Int(value)
    }
}
