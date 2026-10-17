import CoreFoundation
import CryptoKit
import Foundation

private struct C131MetalFullSurfacePixelArtifactManifest: Encodable {
    let schema: String
    let status: String
    let runId: String
    let generation: UInt64
    let viewportCssPx: [Int]
    let drawablePixelSize: [Int]
    let surfaceFormat: String
    let rgbaOrder: String
    let pixelCount: Int
    let rgbaByteLength: Int
    let batchSize: Int
    let batchCount: Int
    let sampleFile: String
    let sampleSha256: String
    let processID: Int32
    let elapsedMilliseconds: UInt64
}

final class C131MetalFullSurfacePixelCapture: C131MetalSurfacePixelCaptureProtocol {
    static let viewportWidth = 300
    static let viewportHeight = 100
    static let drawableWidth = 900
    static let drawableHeight = 300
    static let pixelCount = drawableWidth * drawableHeight
    static let batchSize = 64
    static let totalBatchCount = (pixelCount + batchSize - 1) / batchSize
    static let totalDeadlineSeconds: TimeInterval = 300
    static let batchDeadlineSeconds: TimeInterval = 15

    var expectedSampleCount: Int { Self.pixelCount }
    var expectedBatchCount: Int { Self.totalBatchCount }

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
    private var surfaceFormat: String?
    private var finished = false

    init(runID: String) throws {
        guard runID.range(of: "^[A-Za-z0-9_-]{1,80}$", options: .regularExpression) != nil else {
            throw C131MetalSurfacePixelCapture.CaptureError.invalidRunID
        }
        self.runID = runID
        guard let caches = FileManager.default.urls(
            for: .cachesDirectory,
            in: .userDomainMask
        ).first else {
            throw C131MetalSurfacePixelCapture.CaptureError.artifactDirectoryUnavailable
        }
        let directory = caches.appendingPathComponent(
            "c13-1-metal-surface-pixels",
            isDirectory: true
        )
        try FileManager.default.createDirectory(at: directory, withIntermediateDirectories: true)
        directoryURL = directory
        let stem = "c131-metal-full-surface-\(runID)"
        partURL = directory.appendingPathComponent("\(stem).rgba.part")
        sampleURL = directory.appendingPathComponent("\(stem).rgba")
        manifestPartURL = directory.appendingPathComponent("\(stem).manifest.json.part")
        manifestURL = directory.appendingPathComponent("\(stem).manifest.json")
        guard FileManager.default.createFile(atPath: partURL.path, contents: nil) else {
            throw C131MetalSurfacePixelCapture.CaptureError.artifactWriteFailed("create-rgba-file")
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
        try checkTotalDeadline()
        guard pendingBatch == nil else {
            throw C131MetalSurfacePixelCapture.CaptureError.batchAlreadyPending
        }
        guard proposedWidth == Self.drawableWidth, proposedHeight == Self.drawableHeight,
              proposedGeneration > 0 else {
            throw C131MetalSurfacePixelCapture.CaptureError.invalidSurfaceSize
        }
        guard Self.supportedFormats.contains(proposedFormat) else {
            throw C131MetalSurfacePixelCapture.CaptureError.invalidSurfaceFormat
        }
        if let surfaceGeneration, surfaceGeneration != proposedGeneration {
            throw C131MetalSurfacePixelCapture.CaptureError.generationChanged
        }
        if surfaceGeneration == nil {
            surfaceGeneration = proposedGeneration
            surfaceFormat = proposedFormat
        } else if surfaceFormat != proposedFormat {
            throw C131MetalSurfacePixelCapture.CaptureError.surfaceChanged
        }
        guard nextIndex < Self.pixelCount else { return nil }

        let endIndex = min(nextIndex + Self.batchSize, Self.pixelCount)
        var coordinates: [C131MetalPixelCoordinate] = []
        coordinates.reserveCapacity(endIndex - nextIndex)
        for index in nextIndex..<endIndex {
            coordinates.append(C131MetalPixelCoordinate(
                index: index,
                cssX: nil,
                cssY: nil,
                drawableX: index % proposedWidth,
                drawableY: index / proposedWidth
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
        guard !finished, let pendingBatch else {
            throw C131MetalSurfacePixelCapture.CaptureError.batchMissing
        }
        try checkTotalDeadline()
        guard let pendingAt,
              ProcessInfo.processInfo.systemUptime - pendingAt <= Self.batchDeadlineSeconds else {
            throw C131MetalSurfacePixelCapture.CaptureError.batchDeadlineExceeded
        }
        guard surfaceGeneration == observedGeneration else {
            throw C131MetalSurfacePixelCapture.CaptureError.generationChanged
        }
        guard observedWidth == Self.drawableWidth, observedHeight == Self.drawableHeight,
              surfaceFormat == observedFormat else {
            throw C131MetalSurfacePixelCapture.CaptureError.surfaceChanged
        }
        guard let surfaceSize = field(report, "surface_texture=") else {
            throw C131MetalSurfacePixelCapture.CaptureError.missingReportField("surface_texture")
        }
        let dimensions = surfaceSize.split(separator: "x").compactMap { Int($0) }
        guard dimensions == [Self.drawableWidth, Self.drawableHeight] else {
            throw C131MetalSurfacePixelCapture.CaptureError.surfaceChanged
        }
        guard let encoded = field(report, "surface_pixel_samples_json="),
              let data = encoded.data(using: .utf8),
              let samples = try? JSONSerialization.jsonObject(with: data) as? [[String: Any]] else {
            throw C131MetalSurfacePixelCapture.CaptureError.missingReportField(
                "surface_pixel_samples_json"
            )
        }
        guard samples.count == pendingBatch.count else {
            throw C131MetalSurfacePixelCapture.CaptureError.invalidSampleCount
        }

        var packedRgba = Data()
        packedRgba.reserveCapacity(pendingBatch.count * 4)
        for (offset, coordinate) in pendingBatch.enumerated() {
            let object = samples[offset]
            guard Set(object.keys) == Set(["x", "y", "rgba"]),
                  let x = integer(object["x"]), let y = integer(object["y"]),
                  let channels = object["rgba"] as? [NSNumber], channels.count == 4 else {
                throw C131MetalSurfacePixelCapture.CaptureError.invalidSampleShape(coordinate.index)
            }
            guard x == coordinate.drawableX, y == coordinate.drawableY else {
                throw C131MetalSurfacePixelCapture.CaptureError.sampleCoordinateMismatch(
                    coordinate.index
                )
            }
            let rgba = channels.compactMap(integer)
            guard rgba.count == 4, rgba.allSatisfy({ (0...255).contains($0) }) else {
                throw C131MetalSurfacePixelCapture.CaptureError.sampleColorInvalid(coordinate.index)
            }
            packedRgba.append(contentsOf: rgba.map(UInt8.init))
        }

        do {
            guard let fileHandle else {
                throw C131MetalSurfacePixelCapture.CaptureError.artifactWriteFailed(
                    "rgba-file-closed"
                )
            }
            try fileHandle.write(contentsOf: packedRgba)
        } catch let error as C131MetalSurfacePixelCapture.CaptureError {
            throw error
        } catch {
            throw C131MetalSurfacePixelCapture.CaptureError.artifactWriteFailed(
                String(describing: error)
            )
        }

        nextIndex += pendingBatch.count
        self.pendingBatch = nil
        self.pendingAt = nil
        let complete = nextIndex == Self.pixelCount
        if complete { try finalize() }
        return C131MetalPixelCaptureProgress(
            capturedSamples: nextIndex,
            totalSamples: Self.pixelCount,
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

    private static let supportedFormats: Set<String> = [
        "Rgba8Unorm", "Rgba8UnormSrgb", "Bgra8Unorm", "Bgra8UnormSrgb",
    ]

    private func checkTotalDeadline() throws {
        guard ProcessInfo.processInfo.systemUptime - startedAt <= Self.totalDeadlineSeconds else {
            throw C131MetalSurfacePixelCapture.CaptureError.totalDeadlineExceeded
        }
    }

    private func finalize() throws {
        guard let generation = surfaceGeneration, let format = surfaceFormat else {
            throw C131MetalSurfacePixelCapture.CaptureError.invalidSurfaceSize
        }
        do {
            guard let activeFileHandle = fileHandle else {
                throw C131MetalSurfacePixelCapture.CaptureError.artifactWriteFailed(
                    "rgba-file-closed-before-finalize"
                )
            }
            try activeFileHandle.synchronize()
            try activeFileHandle.close()
            fileHandle = nil
            let rgbaData = try Data(contentsOf: partURL)
            let expectedByteLength = Self.pixelCount * 4
            guard rgbaData.count == expectedByteLength else {
                throw C131MetalSurfacePixelCapture.CaptureError.artifactWriteFailed(
                    "rgba-byte-length-\(rgbaData.count)-expected-\(expectedByteLength)"
                )
            }
            let digest = SHA256.hash(data: rgbaData).map { String(format: "%02x", $0) }.joined()
            try FileManager.default.moveItem(at: partURL, to: sampleURL)
            let encoder = JSONEncoder()
            encoder.outputFormatting = [.sortedKeys]
            let manifest = C131MetalFullSurfacePixelArtifactManifest(
                schema: "spinon-c13-1-ios-metal-full-surface-pixels/v1",
                status: "complete",
                runId: runID,
                generation: generation,
                viewportCssPx: [Self.viewportWidth, Self.viewportHeight],
                drawablePixelSize: [Self.drawableWidth, Self.drawableHeight],
                surfaceFormat: format,
                rgbaOrder: "RGBA8",
                pixelCount: Self.pixelCount,
                rgbaByteLength: expectedByteLength,
                batchSize: Self.batchSize,
                batchCount: Self.totalBatchCount,
                sampleFile: sampleURL.lastPathComponent,
                sampleSha256: digest,
                processID: ProcessInfo.processInfo.processIdentifier,
                elapsedMilliseconds: UInt64(
                    (ProcessInfo.processInfo.systemUptime - startedAt) * 1000
                )
            )
            try encoder.encode(manifest).write(to: manifestPartURL, options: .atomic)
            try FileManager.default.moveItem(at: manifestPartURL, to: manifestURL)
            finished = true
        } catch let error as C131MetalSurfacePixelCapture.CaptureError {
            throw error
        } catch {
            throw C131MetalSurfacePixelCapture.CaptureError.artifactWriteFailed(
                String(describing: error)
            )
        }
    }

    private func field(_ report: String, _ marker: String) -> String? {
        guard let range = report.range(of: marker) else { return nil }
        return report[range.upperBound...].split(whereSeparator: \.isWhitespace)
            .first.map(String.init)
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
