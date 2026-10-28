import Foundation
import Vision

guard CommandLine.arguments.count == 2 else {
    fatalError("usage: verify-ios-screenshot.swift <screenshot.png>")
}

let request = VNRecognizeTextRequest()
request.recognitionLevel = .accurate
request.usesLanguageCorrection = false
try VNImageRequestHandler(url: URL(fileURLWithPath: CommandLine.arguments[1])).perform([request])

let recognizedText = (request.results ?? []).compactMap { $0.topCandidates(1).first?.string }
let combined = recognizedText.joined(separator: "\n")
let normalized = combined.uppercased()
for marker in ["SPINON", "C11.4", "GRID TRACK SIZING"] where !normalized.contains(marker) {
    fputs("Screenshot is missing expected app title marker: \(marker)\n\(combined)\n", stderr)
    exit(1)
}

print(combined)
