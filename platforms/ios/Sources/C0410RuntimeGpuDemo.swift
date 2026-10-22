import CryptoKit
import MetalKit
import UIKit

private let c1133GridTemplateAreasSceneIDs: Set<String> = [
    "basic-rectangles", "null-cell-dot-runs", "all-null-cells", "single-cell-area",
    "bare-area-edge-aliases", "grid-area-one-value", "grid-area-two-values",
    "grid-area-three-values", "grid-area-four-values", "row-column-longhands-area",
    "grid-template-shorthand", "grid-shorthand", "bare-area-vs-explicit-one-name",
    "explicit-line-before-area", "explicit-line-after-area", "area-names-ending-start-or-end",
    "implicit-area-from-explicit-lines", "escaped-and-unicode-identifiers",
    "invalid-l-shape-fallback", "invalid-disconnected-area-fallback",
    "invalid-row-width-fallback", "invalid-trash-token-fallback", "empty-string-fallback",
    "area-track-count-mismatch", "unknown-area-implicit-tracks", "nested-hidden-source-order",
    "unknown-area-fixed-implicit-tracks",
]
private let c114GridTrackSizingSceneIDs: Set<String> = [
    "cyclic-percent-column", "cyclic-percent-row", "definite-percent-column",
    "cyclic-minmax-column", "cyclic-minmax-row", "definite-minmax-column",
    "cyclic-fit-content-column", "cyclic-fit-content-row", "definite-fit-content-column",
    "cyclic-fit-content-calc-column", "cyclic-fit-content-calc-row",
    "definite-fit-content-calc-column", "definite-fit-content-calc-small-limit",
    "cyclic-minmax-calc-column", "cyclic-minmax-calc-row",
    "definite-minmax-calc-column", "definite-minmax-calc-small-limit",
    "cyclic-minmax-calc-min-column", "cyclic-minmax-calc-min-row",
    "definite-minmax-calc-min-column", "definite-minmax-calc-min-row",
    "cyclic-minmax-calc-min-both-axes",
    "cyclic-minmax-calc-min-column-definite-row",
    "definite-column-cyclic-minmax-calc-min-row",
    "definite-minmax-calc-min-both-axes",
    "cyclic-percent-two-axis-multitrack", "cyclic-percent-columns-multitrack",
    "cyclic-percent-rows-multitrack", "definite-percent-two-axis-multitrack",
    "cyclic-calc-column", "cyclic-calc-row", "cyclic-calc-two-axis",
    "definite-calc-control",
    "auto-fixed-width-item", "minmax-auto-fixed-width-item",
    "fit-content-below-item-size", "spanning-auto-tracks",
    "spanning-auto-min-flex-tracks",
    "auto-fill-repeat", "auto-fit-collapse", "auto-fill-track-overflow",
    "auto-fill-fixed-neighbors", "auto-fit-fixed-neighbors",
    "auto-fill-pattern-neighbors",
    "auto-fit-span-neighbors", "auto-fit-empty-neighbors",
    "auto-fill-prefix-neighbors", "auto-fill-suffix-neighbors",
    "auto-fill-indefinite-neighbors", "auto-fill-max-size-neighbors",
    "auto-fill-min-size-neighbors", "auto-fill-overflow-neighbors",
    "auto-fill-row-neighbors", "auto-fit-row-neighbors",
    "auto-fill-two-track-columns-both-190", "auto-fill-two-track-columns-both-200",
    "auto-fill-two-track-columns-both-342", "auto-fill-two-track-columns-prefix-200",
    "auto-fill-two-track-columns-suffix-200", "auto-fill-two-track-rows-both-190",
    "auto-fill-two-track-rows-both-200", "auto-fill-two-track-rows-both-342",
    "auto-fit-min-columns-both-190", "auto-fit-min-columns-both-200",
    "auto-fit-min-columns-both-342", "auto-fit-min-rows-both-190",
    "auto-fit-min-rows-both-200", "auto-fit-min-rows-both-342",
    "auto-fit-min-columns-prefix-200", "auto-fit-min-columns-suffix-200",
    "auto-fit-min-columns-none-200", "auto-fit-min-rows-prefix-200",
    "auto-fit-min-rows-suffix-200", "auto-fit-min-rows-none-200",
    "auto-fit-min-columns-single-200", "auto-fit-min-rows-single-200",
    "auto-fit-min-columns-span-two-200", "auto-fit-min-rows-span-two-200",
    "auto-fit-min-columns-second-repeat-200", "auto-fit-min-rows-second-repeat-200",
    "auto-fit-min-columns-fixed-pattern-200", "auto-fit-min-rows-fixed-pattern-200",
    "columns-fill-control", "rows-fill-control", "both-fill-square", "both-fill-asymmetric",
    "both-fit-empty", "both-fit-single", "both-fit-span-two", "both-fit-second-repeat",
    "columns-fit-rows-fill-empty", "columns-fill-rows-fit-empty",
    "columns-fit-rows-fill-second-repeat", "columns-fill-rows-fit-second-repeat",
    "min-content-text-normal", "auto-text-max-content",
    "max-content-text-normal", "fit-content-text-minimum",
    "fit-content-calc-limit", "min-content-text-edge",
    "min-content-text-min-width", "min-content-text-max-width",
    "min-content-text-break-all", "min-content-text-nowrap",
    "auto-image-definite", "minmax-auto-image", "minmax-min-max-image",
    "fit-content-below-minimum", "fit-content-above-intrinsic",
    "fit-content-percent", "fixed-small-track-overflow",
    "spanning-min-content-items", "min-content-row-image",
    "column-auto-natural-ratio", "column-target-max-width-clamp",
    "row-target-max-height-clamp", "column-opposite-min-over-max",
    "column-border-box-transfer", "column-content-box-transfer",
    "column-target-min-width-clamp",
    "row-transfer-input-capped-by-fixed-column-span",
    "column-transfer-input-capped-by-fixed-row-span",
    "fixed-gutter-column-suggestion-clamp",
    "fixed-gutter-row-suggestion-clamp",
    "grid-gap-definite-percent", "grid-gap-cyclic-percent",
    "grid-gap-definite-calc", "grid-gap-cyclic-calc",
    "grid-gap-cyclic-percent-min-basis", "grid-gap-cyclic-calc-min-basis",
    "row-min-content", "row-max-content", "column-min-content", "column-max-content",
    "row-auto-placement-min-content", "column-auto-placement-max-content",
    "row-auto-placement-auto", "column-auto-placement-auto",
    "two-implicit-columns-min-content", "cycled-intrinsic-columns",
    "cycled-fixed-max-rows", "implicit-percentage-column", "implicit-fraction-column",
    "implicit-minmax-column", "implicit-fit-content-column",
    "implicit-min-content-overflow-border", "implicit-calc-column",
    "explicit-track-control", "row-positive-implicit-locked-item",
    "column-positive-implicit-locked-item",
    "columns-one-negative", "columns-two-negative", "columns-three-negative",
    "columns-four-negative", "columns-negative-and-positive-cycle",
    "columns-negative-fixed-single", "columns-zero-track-cycle",
    "columns-explicit-boundary-span", "columns-reversed-lines-normalize",
    "columns-negative-line-span", "columns-no-explicit-tracks",
    "columns-negative-minmax-track", "columns-negative-min-content-track",
    "columns-negative-max-content-track", "rows-one-negative", "rows-two-negative",
    "rows-three-negative", "rows-negative-and-positive-cycle",
    "both-axes-negative-independent-patterns", "hidden-negative-does-not-expand",
    "negative-column-auto-row-placement", "columns-negative-grid-auto-items",
    "rows-negative-grid-auto-items", "columns-negative-row-dense-auto-items",
    "columns-negative-row-sparse-cursor", "columns-negative-row-dense-cursor",
    "columns-negative-column-flow-auto-items", "columns-negative-column-dense-auto-items",
    "rows-negative-row-dense-auto-items", "rows-negative-column-flow-auto-items",
    "rows-negative-column-dense-auto-items", "columns-negative-fixed-root-overflow",
    "rows-negative-fixed-root-overflow",
    "columns-fill-one-leading", "columns-fill-two-repetitions",
    "columns-fill-four-repetitions", "columns-fit-one-leading-empty-repeat",
    "columns-fill-two-leading-cycle", "columns-fill-leading-span-prefix",
    "columns-fit-negative-and-repeat-occupied", "columns-fit-third-repeat-only",
    "columns-fill-negative-positive-cycle", "columns-fill-reversed-negative-lines",
    "columns-fill-repeat-only-leading", "rows-fill-one-leading",
    "rows-fill-three-repetitions", "rows-fill-five-repetitions",
    "rows-fit-one-leading-empty-repeat", "rows-fill-two-leading-cycle",
    "both-axes-fit-negative-leading", "both-axes-fill-independent-cycles",
    "typed-fixed-cycle", "typed-fixed-cycle-three-leading-trailing",
    "typed-mixed-cycle", "typed-auto", "typed-min-content", "typed-max-content",
    "typed-minmax-max-content", "typed-fit-content", "typed-fraction",
    "typed-percentage", "typed-calc", "typed-auto-fit-occupied",
    "typed-auto-fit-empty", "typed-auto-fit-gutter",
    "competing-span2-forward", "competing-span2-reversed",
    "span2-before-span3", "span3-before-span2",
    "text-span2-forward", "text-span2-reversed",
    "text-span2-before-span3", "text-span3-before-span2",
    "image-span2-forward", "image-span2-reversed",
    "image-span2-before-span3", "image-span3-before-span2",
    "mixed-text-image-span2-forward", "mixed-text-image-span2-reversed",
    "mixed-text-image-span2-before-span3", "mixed-text-image-span3-before-span2",
    "anonymous-min-content-wrap", "anonymous-comment-continuation",
    "anonymous-runs-around-block", "anonymous-hidden-child-continuation",
    "whitespace-does-not-create-item", "anonymous-text-image-text",
    "image-before-comment-group", "comment-group-before-image",
    "whitespace-around-image", "hidden-child-around-image",
    "anonymous-text-two-images-forward", "two-images-anonymous-text-reversed",
    "anonymous-text-three-images-forward", "three-images-anonymous-text-reversed",
    "anonymous-runs-two-images-forward", "two-images-anonymous-runs-reversed",
    "two-anonymous-runs-three-images-forward",
    "three-images-two-anonymous-runs-reversed",
    "nested-grid-span-two", "nested-grid-one-track",
    "nested-grid-definite-flex-control",
    "nested-grid-text-image-span-two", "nested-grid-text-image-one-track",
    "nested-grid-text-image-definite-flex-control",
    "nested-grid-text-image-row-span-two", "nested-grid-text-image-row-one-track",
    "nested-grid-text-image-row-definite-flex-control",
    "nested-grid-implicit-row-span-two", "nested-grid-implicit-row-one-row",
    "nested-depth-one-min-content", "nested-depth-two-min-content",
    "nested-depth-one-max-content", "nested-depth-two-max-content",
    "nested-depth-one-min-content-two-axis", "nested-depth-two-min-content-two-axis",
    "nested-depth-one-max-content-two-axis", "nested-depth-two-max-content-two-axis",
    "nested-depth-three-min-content-two-axis", "nested-depth-three-max-content-two-axis",
    "nested-text-span2-forward", "nested-text-span2-reversed",
    "nested-text-span2-before-span3", "nested-text-span3-before-span2",
    "depth-three-min-content-text-before-image",
    "depth-three-min-content-image-before-text",
    "depth-three-max-content-text-before-image",
    "depth-three-max-content-image-before-text",
]
private let c114AutoRepeatNegativeLineSceneIDs: Set<String> = [
    "columns-fill-one-leading", "columns-fill-two-repetitions",
    "columns-fill-four-repetitions", "columns-fit-one-leading-empty-repeat",
    "columns-fill-two-leading-cycle", "columns-fill-leading-span-prefix",
    "columns-fit-negative-and-repeat-occupied", "columns-fit-third-repeat-only",
    "columns-fill-negative-positive-cycle", "columns-fill-reversed-negative-lines",
    "columns-fill-repeat-only-leading", "rows-fill-one-leading",
    "rows-fill-three-repetitions", "rows-fill-five-repetitions",
    "rows-fit-one-leading-empty-repeat", "rows-fill-two-leading-cycle",
    "both-axes-fit-negative-leading", "both-axes-fill-independent-cycles",
    "typed-fixed-cycle", "typed-fixed-cycle-three-leading-trailing",
    "typed-mixed-cycle", "typed-auto", "typed-min-content", "typed-max-content",
    "typed-minmax-max-content", "typed-fit-content", "typed-fraction",
    "typed-percentage", "typed-calc", "typed-auto-fit-occupied",
    "typed-auto-fit-empty", "typed-auto-fit-gutter",
]

private struct C114NegativeTrackExpectation {
    let rootWidth: String
    let rootHeight: String
    let itemWidth: String
    let itemHeight: String
    let boxCount: Int
}

private let c114NegativeTrackExpectations: [String: C114NegativeTrackExpectation] = [
    "columns-one-negative": C114NegativeTrackExpectation(rootWidth: "189", rootHeight: "22", itemWidth: "47", itemHeight: "20", boxCount: 4),
    "columns-two-negative": C114NegativeTrackExpectation(rootWidth: "223", rootHeight: "22", itemWidth: "31", itemHeight: "20", boxCount: 5),
    "columns-three-negative": C114NegativeTrackExpectation(rootWidth: "273", rootHeight: "22", itemWidth: "47", itemHeight: "20", boxCount: 6),
    "columns-four-negative": C114NegativeTrackExpectation(rootWidth: "273", rootHeight: "22", itemWidth: "31", itemHeight: "20", boxCount: 6),
    "columns-negative-and-positive-cycle": C114NegativeTrackExpectation(rootWidth: "273", rootHeight: "22", itemWidth: "31", itemHeight: "20", boxCount: 6),
    "columns-negative-fixed-single": C114NegativeTrackExpectation(rootWidth: "185", rootHeight: "22", itemWidth: "37", itemHeight: "20", boxCount: 4),
    "columns-zero-track-cycle": C114NegativeTrackExpectation(rootWidth: "161", rootHeight: "22", itemWidth: "0", itemHeight: "20", boxCount: 3),
    "columns-explicit-boundary-span": C114NegativeTrackExpectation(rootWidth: "155", rootHeight: "45", itemWidth: "90", itemHeight: "20", boxCount: 4),
    "columns-reversed-lines-normalize": C114NegativeTrackExpectation(rootWidth: "223", rootHeight: "22", itemWidth: "81", itemHeight: "20", boxCount: 4),
    "columns-negative-line-span": C114NegativeTrackExpectation(rootWidth: "223", rootHeight: "22", itemWidth: "81", itemHeight: "20", boxCount: 4),
    "columns-no-explicit-tracks": C114NegativeTrackExpectation(rootWidth: "167", rootHeight: "22", itemWidth: "31", itemHeight: "20", boxCount: 5),
    "columns-negative-minmax-track": C114NegativeTrackExpectation(rootWidth: "230", rootHeight: "22", itemWidth: "72", itemHeight: "20", boxCount: 4),
    "columns-negative-min-content-track": C114NegativeTrackExpectation(rootWidth: "218", rootHeight: "22", itemWidth: "60", itemHeight: "20", boxCount: 4),
    "columns-negative-max-content-track": C114NegativeTrackExpectation(rootWidth: "254", rootHeight: "22", itemWidth: "96", itemHeight: "20", boxCount: 4),
    "rows-one-negative": C114NegativeTrackExpectation(rootWidth: "22", rootHeight: "157", itemWidth: "20", itemHeight: "29", boxCount: 4),
    "rows-two-negative": C114NegativeTrackExpectation(rootWidth: "22", rootHeight: "177", itemWidth: "20", itemHeight: "17", boxCount: 5),
    "rows-three-negative": C114NegativeTrackExpectation(rootWidth: "22", rootHeight: "209", itemWidth: "20", itemHeight: "29", boxCount: 6),
    "rows-negative-and-positive-cycle": C114NegativeTrackExpectation(rootWidth: "22", rootHeight: "209", itemWidth: "20", itemHeight: "17", boxCount: 6),
    "both-axes-negative-independent-patterns": C114NegativeTrackExpectation(rootWidth: "189", rootHeight: "137", itemWidth: "47", itemHeight: "29", boxCount: 4),
    "hidden-negative-does-not-expand": C114NegativeTrackExpectation(rootWidth: "139", rootHeight: "22", itemWidth: "40", itemHeight: "20", boxCount: 3),
    "negative-column-auto-row-placement": C114NegativeTrackExpectation(rootWidth: "189", rootHeight: "22", itemWidth: "47", itemHeight: "20", boxCount: 4),
    "columns-negative-grid-auto-items": C114NegativeTrackExpectation(rootWidth: "189", rootHeight: "45", itemWidth: "31", itemHeight: "20", boxCount: 6),
    "rows-negative-grid-auto-items": C114NegativeTrackExpectation(rootWidth: "22", rootHeight: "177", itemWidth: "20", itemHeight: "17", boxCount: 6),
    "columns-negative-row-dense-auto-items": C114NegativeTrackExpectation(rootWidth: "189", rootHeight: "45", itemWidth: "31", itemHeight: "20", boxCount: 6),
    "columns-negative-row-sparse-cursor": C114NegativeTrackExpectation(rootWidth: "155", rootHeight: "45", itemWidth: "47", itemHeight: "20", boxCount: 5),
    "columns-negative-row-dense-cursor": C114NegativeTrackExpectation(rootWidth: "155", rootHeight: "45", itemWidth: "47", itemHeight: "20", boxCount: 5),
    "columns-negative-column-flow-auto-items": C114NegativeTrackExpectation(rootWidth: "189", rootHeight: "45", itemWidth: "31", itemHeight: "20", boxCount: 6),
    "columns-negative-column-dense-auto-items": C114NegativeTrackExpectation(rootWidth: "189", rootHeight: "45", itemWidth: "31", itemHeight: "20", boxCount: 6),
    "rows-negative-row-dense-auto-items": C114NegativeTrackExpectation(rootWidth: "22", rootHeight: "177", itemWidth: "20", itemHeight: "17", boxCount: 6),
    "rows-negative-column-flow-auto-items": C114NegativeTrackExpectation(rootWidth: "45", rootHeight: "157", itemWidth: "20", itemHeight: "17", boxCount: 6),
    "rows-negative-column-dense-auto-items": C114NegativeTrackExpectation(rootWidth: "45", rootHeight: "157", itemWidth: "20", itemHeight: "17", boxCount: 6),
    "columns-negative-fixed-root-overflow": C114NegativeTrackExpectation(rootWidth: "100", rootHeight: "22", itemWidth: "31", itemHeight: "20", boxCount: 6),
    "rows-negative-fixed-root-overflow": C114NegativeTrackExpectation(rootWidth: "22", rootHeight: "100", itemWidth: "20", itemHeight: "17", boxCount: 6),
]

private func c114NegativeTrackExpectation(_ sceneID: String) -> C114NegativeTrackExpectation? {
    c114NegativeTrackExpectations[sceneID]
}

private func c114ExpectedRootFrame(_ sceneID: String) -> String {
    let sceneID = c114BaseNestedGridSceneID(sceneID)
    if sceneID == "cyclic-fit-content-column" { return "width=50,height=20" }
    if sceneID == "cyclic-fit-content-row" { return "width=20,height=40" }
    if sceneID == "definite-fit-content-column" { return "width=40,height=20" }
    if sceneID == "cyclic-fit-content-calc-column" { return "width=50,height=20" }
    if sceneID == "cyclic-fit-content-calc-row" { return "width=20,height=40" }
    if sceneID == "definite-fit-content-calc-column" { return "width=40,height=20" }
    if sceneID == "definite-fit-content-calc-small-limit" { return "width=10,height=20" }
    if sceneID == "cyclic-minmax-calc-column" { return "width=50,height=20" }
    if sceneID == "cyclic-minmax-calc-row" { return "width=20,height=40" }
    if sceneID == "cyclic-minmax-calc-min-column"
        || sceneID == "cyclic-minmax-calc-min-row"
        || sceneID == "definite-minmax-calc-min-column"
        || sceneID == "definite-minmax-calc-min-row" { return "width=14,height=14" }
    if sceneID == "cyclic-minmax-calc-min-both-axes"
        || sceneID == "cyclic-minmax-calc-min-column-definite-row" {
        return "width=54,height=14"
    }
    if sceneID == "definite-column-cyclic-minmax-calc-min-row" {
        return "width=14,height=24"
    }
    if sceneID == "definite-minmax-calc-min-both-axes" { return "width=14,height=14" }
    if sceneID == "definite-minmax-calc-column" { return "width=40,height=20" }
    if sceneID == "definite-minmax-calc-small-limit" { return "width=10,height=20" }
    if sceneID == "cyclic-percent-column" || sceneID == "cyclic-percent-row"
        || sceneID == "cyclic-minmax-column" || sceneID == "cyclic-minmax-row" {
        return "width=24,height=16"
    }
    if sceneID == "definite-percent-column" || sceneID == "definite-minmax-column" {
        return "width=40,height=16"
    }
    if sceneID == "cyclic-percent-two-axis-multitrack"
        || sceneID == "cyclic-percent-columns-multitrack"
        || sceneID == "cyclic-percent-rows-multitrack"
        || sceneID == "definite-percent-two-axis-multitrack" {
        return "width=40,height=20"
    }
    if sceneID == "cyclic-calc-column" || sceneID == "cyclic-calc-row"
        || sceneID == "cyclic-calc-two-axis" { return "width=24,height=16" }
    if sceneID == "definite-calc-control" { return "width=40,height=20" }
    if sceneID.hasPrefix("nested-text-span") {
        return sceneID.contains("before-span3") || sceneID.contains("span3-before-span2")
            ? "width=180,height=60" : "width=95,height=60"
    }
    if c114IsNestedGridDepthAnonymousImageScene(sceneID) {
        return sceneID.contains("-min-content-") ? "width=94,height=20" : "width=124,height=16"
    }
    if sceneID == "nested-depth-one-min-content"
        || sceneID == "nested-depth-two-min-content"
        || sceneID == "nested-depth-three-min-content" { return "width=70,height=20" }
    if sceneID == "nested-depth-one-max-content"
        || sceneID == "nested-depth-two-max-content"
        || sceneID == "nested-depth-three-max-content" { return "width=100,height=10" }
    if sceneID == "nested-grid-implicit-row-span-two" { return "width=24,height=40" }
    if sceneID == "nested-grid-implicit-row-one-row" { return "width=24,height=44" }
    if sceneID == "nested-grid-text-image-span-two" { return "width=98,height=20" }
    if sceneID == "nested-grid-text-image-one-track" { return "width=102,height=20" }
    if sceneID == "nested-grid-text-image-definite-flex-control" { return "width=120,height=20" }
    if sceneID == "nested-grid-text-image-row-span-two" { return "width=98,height=40" }
    if sceneID == "nested-grid-text-image-row-one-track" { return "width=98,height=44" }
    if sceneID == "nested-grid-text-image-row-definite-flex-control" { return "width=98,height=60" }
    if sceneID == "nested-grid-span-two" { return "width=134,height=20" }
    if sceneID == "nested-grid-one-track" { return "width=138,height=20" }
    if sceneID == "nested-grid-definite-flex-control" { return "width=120,height=20" }
    if c114AutoRepeatNegativeLineSceneIDs.contains(sceneID) { return "" }
    if sceneID == "anonymous-min-content-wrap" { return "width=120,height=20" }
    if c114IsAnonymousGridBorderOverflowScene(sceneID) { return "width=40,height=184" }
    if c114IsAnonymousTextMultiImageScene(sceneID) { return "width=220,height=180" }
    if c114IsAnonymousTextImageScene(sceneID) { return "width=220,height=60" }
    if ["anonymous-comment-continuation", "anonymous-runs-around-block",
        "anonymous-hidden-child-continuation",
        "whitespace-does-not-create-item"].contains(sceneID) {
        return "width=120,height=10"
    }
    if let expectation = c114NegativeTrackExpectation(sceneID) {
        return "width=\(expectation.rootWidth),height=\(expectation.rootHeight)"
    }
    if let implicitTrackFrame = c114ImplicitTextRootFrame(sceneID) {
        return implicitTrackFrame
    }
    if let autoFitMinimumSizeFrame = c114AutoFitMinimumSizeRootFrame(sceneID) {
        return autoFitMinimumSizeFrame
    }
    switch sceneID {
    case "text-span2-forward", "text-span2-reversed",
         "text-span2-before-span3", "text-span3-before-span2":
        return "width=220,height=60"
    case "mixed-text-image-span2-forward", "mixed-text-image-span2-reversed",
         "mixed-text-image-span2-before-span3", "mixed-text-image-span3-before-span2":
        return "width=220,height=180"
    case "image-span2-forward", "image-span2-reversed",
         "image-span2-before-span3", "image-span3-before-span2":
        return "width=40,height=60"
    case "column-auto-natural-ratio", "column-target-max-width-clamp",
         "column-opposite-min-over-max", "column-border-box-transfer",
         "column-content-box-transfer", "column-target-min-width-clamp":
        return "width=8,height=20"
    case "row-target-max-height-clamp":
        return "width=20,height=4"
    case "row-transfer-input-capped-by-fixed-column-span":
        return "width=8,height=4"
    case "column-transfer-input-capped-by-fixed-row-span":
        return "width=4,height=8"
    case "fixed-gutter-column-suggestion-clamp":
        return "width=12,height=6"
    case "fixed-gutter-row-suggestion-clamp":
        return "width=6,height=12"
    case "grid-gap-definite-percent", "grid-gap-definite-calc":
        return "width=100,height=40"
    case "grid-gap-cyclic-percent":
        return "width=100,height=20"
    case "grid-gap-cyclic-calc":
        return "width=105,height=25"
    case "grid-gap-cyclic-percent-min-basis", "grid-gap-cyclic-calc-min-basis":
        return "width=120,height=30"
    case "auto-fill-fixed-neighbors", "auto-fit-fixed-neighbors",
         "auto-fill-pattern-neighbors":
        return "width=200,height=60"
    case "auto-fit-span-neighbors",
         "auto-fill-prefix-neighbors", "auto-fill-suffix-neighbors":
        return "width=200,height=60"
    case "auto-fit-empty-neighbors", "auto-fill-max-size-neighbors",
         "auto-fill-min-size-neighbors":
        return ""
    case "auto-fill-indefinite-neighbors":
        return "width=86,height=60"
    case "auto-fill-overflow-neighbors":
        return "width=40,height=60"
    case "auto-fill-row-neighbors", "auto-fit-row-neighbors":
        return "width=50,height=200"
    case "both-fill-asymmetric":
        return "width=200,height=266"
    case "columns-fill-control", "rows-fill-control", "both-fill-square", "both-fit-empty",
         "both-fit-single", "both-fit-span-two", "both-fit-second-repeat",
         "columns-fit-rows-fill-empty", "columns-fill-rows-fit-empty",
         "columns-fit-rows-fill-second-repeat", "columns-fill-rows-fit-second-repeat":
        return "width=200,height=200"
    case "auto-fill-two-track-columns-both-190":
        return "width=190,height=60"
    case "auto-fill-two-track-columns-both-200":
        return "width=266,height=60"
    case "auto-fill-two-track-columns-both-342":
        return "width=342,height=60"
    case "auto-fill-two-track-columns-prefix-200":
        return "width=248,height=60"
    case "auto-fill-two-track-columns-suffix-200":
        return "width=238,height=60"
    case "auto-fill-two-track-rows-both-190":
        return "width=50,height=190"
    case "auto-fill-two-track-rows-both-200":
        return "width=50,height=266"
    case "auto-fill-two-track-rows-both-342":
        return "width=50,height=342"
    case "competing-span2-forward", "competing-span2-reversed",
         "span2-before-span3", "span3-before-span2":
        return "width=220,height=40"
    case "spanning-auto-tracks", "spanning-min-content-items",
         "spanning-auto-min-flex-tracks":
        return "width=100,height=30"
    case "fixed-small-track-overflow":
        return "width=12,height=40"
    case "min-content-row-image":
        return "width=100,height=80"
    case "fit-content-calc-limit":
        return "width=100,height=120"
    case "min-content-text-edge", "min-content-text-min-width", "min-content-text-max-width":
        return "width=120,height=50"
    case "min-content-text-normal", "auto-text-max-content", "max-content-text-normal",
         "fit-content-text-minimum", "min-content-text-break-all", "min-content-text-nowrap":
        return "width=120,height=40"
    default:
        return "width=100,height=40"
    }
}

private func c114ExpectedItemFrame(_ sceneID: String) -> String {
    if sceneID == "cyclic-fit-content-column" { return "width=25,height=20" }
    if sceneID == "cyclic-fit-content-row" || sceneID == "definite-fit-content-column" {
        return "width=20,height=20"
    }
    if sceneID == "cyclic-fit-content-calc-column" { return "width=29,height=20" }
    if sceneID == "cyclic-fit-content-calc-row" { return "width=20,height=22" }
    if sceneID == "definite-fit-content-calc-column" { return "width=24,height=20" }
    if sceneID == "definite-fit-content-calc-small-limit" { return "width=9,height=20" }
    if sceneID == "cyclic-minmax-calc-column" { return "width=29,height=20" }
    if sceneID == "cyclic-minmax-calc-row" { return "width=20,height=22" }
    if sceneID == "cyclic-minmax-calc-min-column"
        || sceneID == "definite-minmax-calc-min-column" { return "width=24,height=10" }
    if sceneID == "cyclic-minmax-calc-min-both-axes" { return "width=104,height=22" }
    if sceneID == "cyclic-minmax-calc-min-column-definite-row" {
        return "width=104,height=10"
    }
    if sceneID == "definite-column-cyclic-minmax-calc-min-row" {
        return "width=10,height=42"
    }
    if sceneID == "definite-minmax-calc-min-both-axes" { return "width=24,height=22" }
    if sceneID == "cyclic-minmax-calc-min-row"
        || sceneID == "definite-minmax-calc-min-row" { return "width=10,height=22" }
    if sceneID == "definite-minmax-calc-column" { return "width=24,height=20" }
    if sceneID == "definite-minmax-calc-small-limit" { return "width=9,height=20" }
    if sceneID == "cyclic-percent-column" || sceneID == "cyclic-percent-row"
        || sceneID == "definite-percent-column" || sceneID == "cyclic-minmax-column"
        || sceneID == "cyclic-minmax-row" || sceneID == "definite-minmax-column" {
        return "width=24,height=16"
    }
    if sceneID == "cyclic-percent-two-axis-multitrack"
        || sceneID == "cyclic-percent-columns-multitrack"
        || sceneID == "cyclic-percent-rows-multitrack"
        || sceneID == "definite-percent-two-axis-multitrack" {
        return "width=20,height=10"
    }
    if sceneID == "cyclic-calc-column" || sceneID == "cyclic-calc-row"
        || sceneID == "cyclic-calc-two-axis" || sceneID == "definite-calc-control" {
        return "width=24,height=16"
    }
    if ["nested-grid-span-two", "nested-grid-one-track",
        "nested-grid-definite-flex-control", "nested-grid-text-image-span-two",
        "nested-grid-text-image-one-track",
        "nested-grid-text-image-definite-flex-control",
        "nested-grid-text-image-row-span-two", "nested-grid-text-image-row-one-track",
        "nested-grid-text-image-row-definite-flex-control",
        "nested-grid-implicit-row-span-two", "nested-grid-implicit-row-one-row"]
        .contains(sceneID) { return "" }
    if sceneID.hasPrefix("nested-depth-one-") || sceneID.hasPrefix("nested-depth-two-")
        || sceneID.hasPrefix("nested-depth-three-")
        || sceneID.hasPrefix("nested-text-span")
        || c114IsNestedGridDepthAnonymousImageScene(sceneID) {
        return ""
    }
    if c114IsAnonymousTextScene(sceneID) || c114IsAnonymousTextImageScene(sceneID)
        || c114IsAnonymousTextMultiImageScene(sceneID) { return "" }
    if c114IsTextMultiSpanScene(sceneID) || c114IsImageMultiSpanScene(sceneID)
        || c114IsMixedMultiSpanScene(sceneID) { return "" }
    if c114AutoRepeatNegativeLineSceneIDs.contains(sceneID) { return "" }
    if c114IsImplicitTextScene(sceneID) { return "" }
    if let expectation = c114NegativeTrackExpectation(sceneID) {
        return "width=\(expectation.itemWidth),height=\(expectation.itemHeight)"
    }
    if let autoFitMinimumSizeItemFrame = c114AutoFitMinimumSizeItemFrame(sceneID) {
        return autoFitMinimumSizeItemFrame
    }
    switch sceneID {
    case "competing-span2-forward", "span2-before-span3":
        return "width=80,height=10"
    case "competing-span2-reversed":
        return "width=120,height=10"
    case "span3-before-span2":
        return "width=200,height=10"
    case "column-auto-natural-ratio":
        return "width=12,height=8"
    case "column-target-max-width-clamp":
        return "width=9,height=8"
    case "row-target-max-height-clamp":
        return "width=12,height=6"
    case "column-opposite-min-over-max":
        return "width=18,height=12"
    case "column-border-box-transfer":
        return "width=12,height=8"
    case "column-content-box-transfer":
        return "width=16,height=12"
    case "column-target-min-width-clamp":
        return "width=18,height=8"
    case "row-transfer-input-capped-by-fixed-column-span":
        return "width=18,height=12"
    case "column-transfer-input-capped-by-fixed-row-span":
        return "width=27,height=18"
    case "fixed-gutter-column-suggestion-clamp":
        return "width=36,height=24"
    case "fixed-gutter-row-suggestion-clamp":
        return "width=54,height=36"
    case "grid-gap-definite-percent", "grid-gap-cyclic-percent",
         "grid-gap-definite-calc", "grid-gap-cyclic-calc",
         "grid-gap-cyclic-percent-min-basis", "grid-gap-cyclic-calc-min-basis":
        return "width=40,height=10"
    case "spanning-auto-tracks":
        return "width=70,height=10"
    case "spanning-auto-min-flex-tracks":
        return "width=100,height=10"
    case "auto-fill-repeat":
        return "width=24,height=16"
    case "auto-fit-collapse":
        return "width=96,height=16"
    case "auto-fill-track-overflow":
        return "width=120,height=16"
    case "auto-fill-fixed-neighbors", "auto-fill-pattern-neighbors":
        return "width=10,height=40"
    case "auto-fit-fixed-neighbors":
        return "width=162,height=40"
    case "auto-fit-empty-neighbors", "auto-fill-max-size-neighbors",
         "auto-fill-min-size-neighbors", "auto-fill-two-track-columns-both-190",
         "auto-fill-two-track-columns-both-200", "auto-fill-two-track-columns-both-342",
         "auto-fill-two-track-columns-prefix-200", "auto-fill-two-track-columns-suffix-200",
         "auto-fill-two-track-rows-both-190", "auto-fill-two-track-rows-both-200",
         "auto-fill-two-track-rows-both-342", "both-fit-empty",
         "columns-fit-rows-fill-empty", "columns-fill-rows-fit-empty":
        return ""
    case "columns-fill-control", "rows-fill-control", "both-fill-square", "both-fit-single",
         "both-fit-span-two", "columns-fit-rows-fill-second-repeat",
         "columns-fill-rows-fit-second-repeat":
        return "width=12,height=12"
    case "both-fit-second-repeat":
        return "width=12,height=12"
    case "both-fill-asymmetric":
        return "width=12,height=12"
    case "auto-fit-span-neighbors", "auto-fill-prefix-neighbors",
         "auto-fill-suffix-neighbors", "auto-fill-indefinite-neighbors",
         "auto-fill-overflow-neighbors", "auto-fill-row-neighbors",
         "auto-fit-row-neighbors":
        return "width=12,height=12"
    case "min-content-text-normal", "fit-content-text-minimum":
        return "width=70,height=20"
    case "auto-text-max-content", "max-content-text-normal", "min-content-text-nowrap":
        return "width=100,height=10"
    case "fit-content-calc-limit":
        return "width=15,height=80"
    case "min-content-text-edge":
        return "width=80,height=20"
    case "min-content-text-min-width":
        return "width=90,height=20"
    case "min-content-text-max-width":
        return "width=40,height=20"
    case "min-content-text-break-all":
        return "width=10,height=80"
    default:
        return "width=24,height=16"
    }
}

private func c114ExpectedTextIntrinsicInputs(_ sceneID: String, report: String) -> Bool {
    let sceneID = c114BaseNestedGridSceneID(sceneID)
    if c114IsAnonymousTextMultiImageScene(sceneID) {
        return c114ExpectedAnonymousTextMultiImageInputs(sceneID, report: report)
    }
    if c114IsAnonymousTextImageScene(sceneID) {
        return c114ExpectedAnonymousTextImageInputs(sceneID, report: report)
    }
    if c114IsMixedMultiSpanScene(sceneID) {
        return c114ExpectedMixedIntrinsicInputs(sceneID, report: report)
    }
    if c114IsTextMultiSpanScene(sceneID) {
        return c114ExpectedTextMultiSpanIntrinsicInputs(sceneID, report: report)
    }
    if sceneID.hasPrefix("nested-text-span") {
        return c114ExpectedNestedTextMultiSpanIntrinsicInputs(sceneID, report: report)
    }
    let expected: (minContent: Int, minContentHeight: Int, maxContent: Int, maxContentHeight: Int)?
    var expectedGrid = 1
    var expectedNode = 2
    var expectedSourceOrder = 0
    switch sceneID {
    case "auto-text-max-content", "min-content-text-normal", "max-content-text-normal",
         "fit-content-text-minimum", "min-content-text-edge", "min-content-text-min-width",
         "min-content-text-max-width":
        expected = (70, 20, 100, 10)
    case "nested-depth-one-min-content", "nested-depth-two-min-content",
         "nested-depth-one-max-content", "nested-depth-two-max-content":
        expected = (70, 20, 100, 10)
        if sceneID.hasPrefix("nested-depth-one-") {
            expectedGrid = 2
            expectedNode = 3
        } else {
            expectedGrid = 3
            expectedNode = 4
        }
    case "nested-depth-three-min-content", "nested-depth-three-max-content":
        expected = (70, 20, 100, 10)
        expectedGrid = 4
        expectedNode = 5
    case "fit-content-calc-limit", "min-content-text-break-all":
        expected = (10, 80, 80, 10)
    case "min-content-text-nowrap":
        expected = (100, 10, 100, 10)
    case "spanning-auto-min-flex-tracks":
        expected = (200, 10, 200, 10)
    case "typed-mixed-cycle", "typed-auto", "typed-min-content", "typed-max-content",
         "typed-minmax-max-content", "typed-fit-content":
        expected = (48, 36, 108, 12)
    case "typed-auto-fit-occupied", "typed-auto-fit-gutter":
        expected = (48, 36, 108, 12)
        expectedNode = 3
        expectedSourceOrder = 1
    default:
        expected = nil
    }
    guard let expected else { return true }
    guard let marker = report.range(of: "grid_track_sizing_diagnostics=") else { return false }
    let jsonText = report[marker.upperBound...]
    var depth = 0
    var inString = false
    var escaped = false
    var endIndex: String.Index?
    for index in jsonText.indices {
        let character = jsonText[index]
        if inString {
            if escaped {
                escaped = false
            } else if character == "\\" {
                escaped = true
            } else if character == "\"" {
                inString = false
            }
            continue
        }
        if character == "\"" {
            inString = true
        } else if character == "{" {
            depth += 1
        } else if character == "}" {
            depth -= 1
            if depth == 0 {
                endIndex = jsonText.index(after: index)
                break
            }
        }
    }
    guard let endIndex,
          let data = String(jsonText[..<endIndex]).data(using: .utf8),
          let diagnostics = (try? JSONSerialization.jsonObject(with: data)) as? [String: Any],
          let inputs = diagnostics["gridItemIntrinsicInputs"] as? [[String: Any]],
          inputs.count == 1,
          let input = inputs.first,
          let grid = input["grid"] as? Int, grid == expectedGrid,
          let node = input["node"] as? Int, node == expectedNode,
          let sourceOrder = input["sourceOrder"] as? Int, sourceOrder == expectedSourceOrder,
          let source = input["source"] as? [String: Any],
          let kind = source["kind"] as? String, kind == "text",
          let fontRegistryRevision = source["fontRegistryRevision"] as? Int,
          fontRegistryRevision == 1,
          let minContent = source["minContent"] as? NSNumber,
          minContent.doubleValue == Double(expected.minContent),
          let minContentHeight = source["minContentHeight"] as? NSNumber,
          minContentHeight.doubleValue == Double(expected.minContentHeight),
          let maxContent = source["maxContent"] as? NSNumber,
          maxContent.doubleValue == Double(expected.maxContent),
          let maxContentHeight = source["maxContentHeight"] as? NSNumber,
          maxContentHeight.doubleValue == Double(expected.maxContentHeight) else {
        return false
    }
    return true
}

private func c114ExpectedImageIntrinsicInputs(_ sceneID: String, report: String) -> Bool {
    if c114IsAnonymousTextMultiImageScene(sceneID) {
        return c114ExpectedAnonymousTextMultiImageInputs(sceneID, report: report)
    }
    if c114IsAnonymousTextImageScene(sceneID) {
        return c114ExpectedAnonymousTextImageInputs(sceneID, report: report)
    }
    if c114IsMixedMultiSpanScene(sceneID) {
        return c114ExpectedMixedIntrinsicInputs(sceneID, report: report)
    }
    if c114IsImageMultiSpanScene(sceneID) {
        return c114ExpectedImageMultiSpanInputs(sceneID, report: report)
    }
    if sceneID.hasPrefix("nested-grid-implicit-row-") {
        return c114ExpectedNestedImplicitRowImageInputs(sceneID, report: report)
    }
    let imageScenes: Set<String> = [
        "auto-image-definite", "minmax-auto-image", "minmax-min-max-image",
        "fit-content-below-minimum", "fit-content-above-intrinsic",
        "fit-content-percent", "fixed-small-track-overflow",
        "spanning-min-content-items", "min-content-row-image",
        "column-auto-natural-ratio", "column-target-max-width-clamp",
        "row-target-max-height-clamp",
        "column-opposite-min-over-max", "column-border-box-transfer",
        "column-content-box-transfer", "column-target-min-width-clamp",
        "row-transfer-input-capped-by-fixed-column-span",
        "column-transfer-input-capped-by-fixed-row-span",
        "fixed-gutter-column-suggestion-clamp",
        "fixed-gutter-row-suggestion-clamp",
    ]
    guard imageScenes.contains(sceneID) else { return true }
    guard let marker = report.range(of: "grid_track_sizing_diagnostics=") else { return false }
    let jsonText = report[marker.upperBound...]
    var depth = 0
    var inString = false
    var escaped = false
    var endIndex: String.Index?
    for index in jsonText.indices {
        let character = jsonText[index]
        if inString {
            if escaped {
                escaped = false
            } else if character == "\\" {
                escaped = true
            } else if character == "\"" {
                inString = false
            }
            continue
        }
        if character == "\"" {
            inString = true
        } else if character == "{" {
            depth += 1
        } else if character == "}" {
            depth -= 1
            if depth == 0 {
                endIndex = jsonText.index(after: index)
                break
            }
        }
    }
    guard let endIndex,
          let data = String(jsonText[..<endIndex]).data(using: .utf8),
          let diagnostics = (try? JSONSerialization.jsonObject(with: data)) as? [String: Any],
          let revision = diagnostics["layoutInputRevision"] as? [String: Any],
          let intrinsicRevision = revision["intrinsicSize"] as? Int,
          intrinsicRevision == 1,
          let inputs = diagnostics["gridItemIntrinsicInputs"] as? [[String: Any]],
          inputs.count == 1,
          let input = inputs.first,
          let grid = input["grid"] as? Int, grid == 1,
          let node = input["node"] as? Int, node == 2,
          let sourceOrder = input["sourceOrder"] as? Int, sourceOrder == 0,
          let source = input["source"] as? [String: Any],
          let kind = source["kind"] as? String, kind == "replaced",
          let natural = source["natural"] as? [String: Any],
          let width = natural["width"] as? NSNumber, width.doubleValue == 24,
          let height = natural["height"] as? NSNumber, height.doubleValue == 16 else {
        return false
    }
    return c114ImageTrackSizingTraceMatches(sceneID, diagnostics: diagnostics)
}

private func c114ExpectedNestedImplicitRowImageInputs(_ sceneID: String, report: String) -> Bool {
    guard let marker = report.range(of: "grid_track_sizing_diagnostics=") else { return false }
    let jsonText = report[marker.upperBound...]
    var depth = 0
    var inString = false
    var escaped = false
    var endIndex: String.Index?
    for index in jsonText.indices {
        let character = jsonText[index]
        if inString {
            if escaped {
                escaped = false
            } else if character == "\\" {
                escaped = true
            } else if character == "\"" {
                inString = false
            }
            continue
        }
        if character == "\"" {
            inString = true
        } else if character == "{" {
            depth += 1
        } else if character == "}" {
            depth -= 1
            if depth == 0 {
                endIndex = jsonText.index(after: index)
                break
            }
        }
    }
    guard let endIndex,
          let data = String(jsonText[..<endIndex]).data(using: .utf8),
          let diagnostics = (try? JSONSerialization.jsonObject(with: data)) as? [String: Any],
          let revision = diagnostics["layoutInputRevision"] as? [String: Any],
          let intrinsicRevision = revision["intrinsicSize"] as? Int, intrinsicRevision == 1,
          let inputs = diagnostics["gridItemIntrinsicInputs"] as? [[String: Any]],
          inputs.count == 1,
          let input = inputs.first,
          let grid = input["grid"] as? Int, grid == 2,
          let node = input["node"] as? Int, node == 5,
          let sourceOrder = input["sourceOrder"] as? Int, sourceOrder == 2,
          let source = input["source"] as? [String: Any],
          let kind = source["kind"] as? String, kind == "replaced",
          let natural = source["natural"] as? [String: Any],
          let width = natural["width"] as? NSNumber, width.doubleValue == 24,
          let height = natural["height"] as? NSNumber, height.doubleValue == 16 else {
        return false
    }
    return c114NestedImplicitRowTrackSizingTraceMatches(sceneID, diagnostics: diagnostics)
}

private func c114NestedImplicitRowTrackSizingTraceMatches(
    _ sceneID: String, diagnostics: [String: Any]
) -> Bool {
    guard let traces = diagnostics["trackSizingTraces"] as? [[String: Any]], traces.count == 2 else {
        return false
    }
    var traceByGrid: [Int: [String: Any]] = [:]
    for trace in traces {
        guard let grid = trace["grid"] as? Int, [1, 2].contains(grid),
              traceByGrid[grid] == nil else { return false }
        traceByGrid[grid] = trace
    }
    let outerRows: [(size: Double, gutter: Bool)]
    let outerSpan: Int
    if sceneID == "nested-grid-implicit-row-span-two" {
        outerRows = [(18, false), (4, true), (18, false)]
        outerSpan = 2
    } else if sceneID == "nested-grid-implicit-row-one-row" {
        outerRows = [(40, false), (4, true), (0, false)]
        outerSpan = 1
    } else {
        return false
    }
    return c114NestedImplicitRowGridTraceMatches(
        traceByGrid[1], columns: [(24, false)], rows: outerRows,
        outerContribution: 40, outerSpan: outerSpan
    ) && c114NestedImplicitRowGridTraceMatches(
        traceByGrid[2], columns: [(24, false)],
        rows: [(8, false), (4, true), (8, false), (4, true), (16, false)],
        outerContribution: nil, outerSpan: 0
    )
}

private func c114NestedImplicitRowGridTraceMatches(
    _ trace: [String: Any]?,
    columns expectedColumns: [(size: Double, gutter: Bool)],
    rows expectedRows: [(size: Double, gutter: Bool)],
    outerContribution: Double?,
    outerSpan: Int
) -> Bool {
    guard let trace,
          let passes = trace["passes"] as? [[String: Any]], passes.count == 2,
          let columns = passes.first(where: { ($0["axis"] as? String) == "Columns" }),
          let rows = passes.first(where: { ($0["axis"] as? String) == "Rows" }) else {
        return false
    }
    return c114ImageTrackPassMatches(
        columns, axis: "Columns", expectedTracks: expectedColumns,
        minContentContribution: nil, span: 0, minContentContributionKind: "MinContent"
    ) && c114ImageTrackPassMatches(
        rows, axis: "Rows", expectedTracks: expectedRows,
        minContentContribution: outerContribution, span: outerSpan,
        minContentContributionKind: "MinContent"
    )
}

private func c114IsImageMultiSpanScene(_ sceneID: String) -> Bool {
    ["image-span2-forward", "image-span2-reversed",
     "image-span2-before-span3", "image-span3-before-span2"].contains(sceneID)
}

private func c114IsMixedMultiSpanScene(_ sceneID: String) -> Bool {
    ["mixed-text-image-span2-forward", "mixed-text-image-span2-reversed",
     "mixed-text-image-span2-before-span3", "mixed-text-image-span3-before-span2"]
        .contains(sceneID)
}

private func c114ExpectedMixedIntrinsicInputs(_ sceneID: String, report: String) -> Bool {
    let nodes: [Int]
    let kinds: [String]
    let textMetrics: [[Double]?]
    switch sceneID {
    case "mixed-text-image-span2-forward":
        nodes = [2, 4]
        kinds = ["text", "replaced"]
        textMetrics = [[50, 20, 80, 10], nil]
    case "mixed-text-image-span2-reversed":
        nodes = [2, 3]
        kinds = ["replaced", "text"]
        textMetrics = [nil, [50, 20, 80, 10]]
    case "mixed-text-image-span2-before-span3":
        nodes = [2, 4, 5]
        kinds = ["text", "replaced", "text"]
        textMetrics = [[50, 20, 80, 10], nil, [180, 10, 180, 10]]
    case "mixed-text-image-span3-before-span2":
        nodes = [2, 4, 5]
        kinds = ["text", "replaced", "text"]
        textMetrics = [[180, 10, 180, 10], nil, [50, 20, 80, 10]]
    default:
        return false
    }
    guard let diagnostics = c114GridTrackSizingDiagnostics(report),
          let revision = diagnostics["layoutInputRevision"] as? [String: Any],
          (revision["intrinsicSize"] as? Int) == 1,
          let inputs = diagnostics["gridItemIntrinsicInputs"] as? [[String: Any]],
          inputs.count == nodes.count else { return false }
    for (index, input) in inputs.enumerated() {
        guard (input["grid"] as? Int) == 1,
              (input["node"] as? Int) == nodes[index],
              (input["sourceOrder"] as? Int) == index,
              let source = input["source"] as? [String: Any],
              (source["kind"] as? String) == kinds[index] else { return false }
        if kinds[index] == "text" {
            guard let expected = textMetrics[index],
                  (source["fontRegistryRevision"] as? Int) == 1 else { return false }
            let values = ["minContent", "minContentHeight", "maxContent", "maxContentHeight"]
                .compactMap { (source[$0] as? NSNumber)?.doubleValue }
            guard values.count == expected.count,
                  zip(values, expected).allSatisfy({ abs($0.0 - $0.1) <= 0.0001 }) else {
                return false
            }
        } else {
            guard let natural = source["natural"] as? [String: Any],
                  (natural["width"] as? NSNumber)?.doubleValue == 24,
                  (natural["height"] as? NSNumber)?.doubleValue == 16,
                  natural.keys.contains("aspectRatio"), natural["aspectRatio"] is NSNull else {
                return false
            }
        }
    }
    return true
}

private func c114ExpectedAnonymousTextImageInputs(_ sceneID: String, report: String) -> Bool {
    let nodes: [Int]
    let kinds: [String]
    let textMetrics: [[Double]?]
    let textSources: [[(Int, String)]]
    var expectedGrid = 1
    switch sceneID {
    case "depth-three-min-content-text-before-image",
         "depth-three-max-content-text-before-image":
        expectedGrid = 4
        nodes = [6, 5]
        kinds = ["text", "replaced"]
        textMetrics = [[70, 20, 100, 10], nil]
        textSources = [[(6, "XX XXXXXXX")], []]
    case "depth-three-min-content-image-before-text",
         "depth-three-max-content-image-before-text":
        expectedGrid = 4
        nodes = [5, 6]
        kinds = ["replaced", "text"]
        textMetrics = [nil, [70, 20, 100, 10]]
        textSources = [[], [(6, "XX XXXXXXX")]]
    case "anonymous-text-image-text":
        nodes = [2, 3, 4]
        kinds = ["text", "replaced", "text"]
        textMetrics = [[20, 10, 20, 10], nil, [20, 10, 20, 10]]
        textSources = [[(2, "XX")], [], [(4, "XX")]]
    case "image-before-comment-group":
        nodes = [2, 3]
        kinds = ["replaced", "text"]
        textMetrics = [nil, [40, 10, 40, 10]]
        textSources = [[], [(3, "XX"), (4, "XX")]]
    case "comment-group-before-image":
        nodes = [2, 4]
        kinds = ["text", "replaced"]
        textMetrics = [[40, 10, 40, 10], nil]
        textSources = [[(2, "XX"), (3, "XX")], []]
    case "whitespace-around-image":
        nodes = [3, 4]
        kinds = ["replaced", "text"]
        textMetrics = [nil, [20, 10, 20, 10]]
        textSources = [[], [(4, "\n\tXX")]]
    case "hidden-child-around-image":
        nodes = [2, 5, 6]
        kinds = ["text", "replaced", "text"]
        textMetrics = [[20, 10, 20, 10], nil, [20, 10, 20, 10]]
        textSources = [[(2, "XX")], [], [(6, "XX")]]
    default:
        return false
    }
    guard let diagnostics = c114GridTrackSizingDiagnostics(report),
          let revision = diagnostics["layoutInputRevision"] as? [String: Any],
          (revision["intrinsicSize"] as? Int) == 1,
          let inputs = diagnostics["gridItemIntrinsicInputs"] as? [[String: Any]],
          inputs.count == nodes.count else { return false }
    for (index, input) in inputs.enumerated() {
        guard (input["grid"] as? Int) == expectedGrid,
              (input["node"] as? Int) == nodes[index],
              (input["sourceOrder"] as? Int) == index,
              let source = input["source"] as? [String: Any],
              (source["kind"] as? String) == kinds[index],
              let actualTextSources = input["textSources"] as? [[String: Any]],
              actualTextSources.count == textSources[index].count else { return false }
        for (sourceIndex, actual) in actualTextSources.enumerated() {
            guard (actual["node"] as? Int) == textSources[index][sourceIndex].0,
                  (actual["text"] as? String) == textSources[index][sourceIndex].1 else {
                return false
            }
        }
        if kinds[index] == "text" {
            guard let expected = textMetrics[index],
                  (source["fontRegistryRevision"] as? Int) == 1 else { return false }
            let values = ["minContent", "minContentHeight", "maxContent", "maxContentHeight"]
                .compactMap { (source[$0] as? NSNumber)?.doubleValue }
            guard values.count == expected.count,
                  zip(values, expected).allSatisfy({ abs($0.0 - $0.1) <= 0.0001 }) else {
                return false
            }
        } else {
            guard let natural = source["natural"] as? [String: Any],
                  (natural["width"] as? NSNumber)?.doubleValue == 24,
                  (natural["height"] as? NSNumber)?.doubleValue == 16,
                  natural.keys.contains("aspectRatio"), natural["aspectRatio"] is NSNull else {
                return false
            }
        }
    }
    return true
}

private func c114ExpectedAnonymousTextMultiImageInputs(
    _ sceneID: String, report: String
) -> Bool {
    let nodes: [Int]
    let kinds: [String]
    let spans: [Int]
    let imageWidths: [Double]
    let expectedFrames: [[Double]]
    let textIndices: [Int]
    switch sceneID {
    case "anonymous-text-two-images-forward":
        nodes = [2, 3, 4]
        kinds = ["text", "replaced", "replaced"]
        spans = [1, 2, 2]
        imageWidths = [0, 24, 48]
        expectedFrames = [[40, 0, 20, 10], [0, 0, 24, 16], [12, 60, 48, 32]]
        textIndices = [0]
    case "two-images-anonymous-text-reversed":
        nodes = [2, 3, 4]
        kinds = ["replaced", "text", "replaced"]
        spans = [2, 1, 2]
        imageWidths = [48, 0, 24]
        expectedFrames = [[12, 60, 48, 32], [40, 0, 20, 10], [0, 0, 24, 16]]
        textIndices = [1]
    case "anonymous-text-three-images-forward":
        nodes = [2, 3, 4, 5]
        kinds = ["text", "replaced", "replaced", "replaced"]
        spans = [1, 2, 2, 3]
        imageWidths = [0, 24, 48, 72]
        expectedFrames = [
            [52, 0, 20, 10], [0, 0, 24, 16], [18, 60, 48, 32], [0, 120, 72, 48],
        ]
        textIndices = [0]
    case "three-images-anonymous-text-reversed":
        nodes = [2, 3, 4, 5]
        kinds = ["replaced", "replaced", "text", "replaced"]
        spans = [3, 2, 1, 2]
        imageWidths = [72, 48, 0, 24]
        expectedFrames = [
            [0, 120, 72, 48], [18, 60, 48, 32], [52, 0, 20, 10], [0, 0, 24, 16],
        ]
        textIndices = [2]
    case "anonymous-runs-two-images-forward":
        nodes = [2, 3, 4, 5]
        kinds = ["text", "replaced", "text", "replaced"]
        spans = [1, 2, 1, 2]
        imageWidths = [0, 24, 0, 48]
        expectedFrames = [
            [42, 2, 20, 10], [2, 2, 24, 16], [62, 2, 20, 10], [14, 62, 48, 32],
        ]
        textIndices = [0, 2]
    case "two-images-anonymous-runs-reversed":
        nodes = [2, 3, 4, 5]
        kinds = ["replaced", "text", "replaced", "text"]
        spans = [2, 1, 2, 1]
        imageWidths = [48, 0, 24, 0]
        expectedFrames = [
            [14, 62, 48, 32], [42, 2, 20, 10], [2, 2, 24, 16], [62, 2, 20, 10],
        ]
        textIndices = [1, 3]
    case "two-anonymous-runs-three-images-forward":
        nodes = [2, 3, 4, 5, 6]
        kinds = ["text", "replaced", "text", "replaced", "replaced"]
        spans = [1, 2, 1, 2, 3]
        imageWidths = [0, 24, 0, 48, 72]
        expectedFrames = [
            [54, 2, 20, 10], [2, 2, 24, 16], [74, 2, 20, 10],
            [20, 62, 48, 32], [2, 122, 72, 48],
        ]
        textIndices = [0, 2]
    case "three-images-two-anonymous-runs-reversed":
        nodes = [2, 3, 4, 5, 6]
        kinds = ["replaced", "text", "replaced", "text", "replaced"]
        spans = [3, 1, 2, 1, 2]
        imageWidths = [72, 0, 48, 0, 24]
        expectedFrames = [
            [2, 122, 72, 48], [54, 2, 20, 10], [20, 62, 48, 32],
            [74, 2, 20, 10], [2, 2, 24, 16],
        ]
        textIndices = [1, 3]
    default:
        return false
    }
    guard let diagnostics = c114GridTrackSizingDiagnostics(report),
          let revision = diagnostics["layoutInputRevision"] as? [String: Any],
          (revision["intrinsicSize"] as? Int) == 1,
          let inputs = diagnostics["gridItemIntrinsicInputs"] as? [[String: Any]],
          inputs.count == nodes.count,
          let frames = diagnostics["gridItemFrames"] as? [[String: Any]],
          frames.count == nodes.count else { return false }
    for (index, input) in inputs.enumerated() {
        guard (input["grid"] as? Int) == 1,
              (input["node"] as? Int) == nodes[index],
              (input["sourceOrder"] as? Int) == index,
              let source = input["source"] as? [String: Any],
              (source["kind"] as? String) == kinds[index],
              let textSources = input["textSources"] as? [[String: Any]],
              frames.indices.contains(index) else { return false }
        let frame = frames[index]
        guard (frame["node"] as? Int) == nodes[index] else { return false }
        let actualFrame = ["x", "y", "width", "height"].compactMap {
            (frame[$0] as? NSNumber)?.doubleValue
        }
        guard actualFrame.count == expectedFrames[index].count,
              zip(actualFrame, expectedFrames[index]).allSatisfy({
                  abs($0.0 - $0.1) <= 1.0 / 64.0
              }) else { return false }
        if textIndices.contains(index) {
            guard textSources.count == 1,
                  (textSources[0]["node"] as? Int) == nodes[index],
                  (textSources[0]["text"] as? String) == "XX",
                  (source["fontRegistryRevision"] as? Int) == 1 else { return false }
            let values = ["minContent", "minContentHeight", "maxContent", "maxContentHeight"]
                .compactMap { (source[$0] as? NSNumber)?.doubleValue }
            guard values.count == 4,
                  zip(values, [20.0, 10.0, 20.0, 10.0]).allSatisfy({
                      abs($0.0 - $0.1) <= 0.0001
                  }) else { return false }
        } else {
            guard textSources.isEmpty,
                  let natural = source["natural"] as? [String: Any],
                  (natural["width"] as? NSNumber)?.doubleValue == 24,
                  (natural["height"] as? NSNumber)?.doubleValue == 16,
                  natural.keys.contains("aspectRatio"), natural["aspectRatio"] is NSNull else {
                return false
            }
        }
    }
    return c114AnonymousTextMultiImageTraceMatches(
        sceneID, nodes: nodes, spans: spans, imageWidths: imageWidths,
        textIndices: textIndices, diagnostics: diagnostics)
}

private func c114AnonymousTextMultiImageTraceMatches(
    _ sceneID: String, nodes: [Int], spans: [Int], imageWidths: [Double],
    textIndices: [Int], diagnostics: [String: Any]
) -> Bool {
    guard let traces = diagnostics["trackSizingTraces"] as? [[String: Any]],
          traces.count == 1,
          (traces[0]["grid"] as? Int) == 1,
          let passes = traces[0]["passes"] as? [[String: Any]],
          passes.count == 2,
          (passes[0]["axis"] as? String) == "Columns",
          (passes[1]["axis"] as? String) == "Rows",
          let stages = passes[0]["stages"] as? [[String: Any]],
          let contributions = passes[0]["contributions"] as? [[String: Any]] else { return false }
    let expectedColumns: [Double]
    if c114IsAnonymousGridBorderOverflowScene(sceneID) {
        expectedColumns = c114IsAnonymousGridBorderOverflowThreeImageScene(sceneID)
            ? [18, 34, 20, 20] : [12, 28, 20, 20]
    } else {
        expectedColumns = nodes.count == 3 ? [12, 28, 20, 0] : [18, 34, 20, 0]
    }
    guard c114ImageMultiSpanTracksMatch(passes[0], expected: expectedColumns),
          c114ImageMultiSpanTracksMatch(passes[1], expected: [60, 60, 60]) else { return false }
    let expectedSpans = c114IsAnonymousGridBorderOverflowScene(sceneID)
        ? (c114IsAnonymousGridBorderOverflowThreeImageScene(sceneID) ? [1, 2, 3] : [1, 2])
        : (nodes.count == 3 ? [1, 2] : [1, 2, 3])
    let observedSpans = stages.compactMap { stage -> Int? in
        guard let name = stage["stage"] as? String,
              name.hasPrefix("IntrinsicItemBatch { span: ") else { return nil }
        let suffix = name.dropFirst("IntrinsicItemBatch { span: ".count)
        guard let rawSpan = suffix.split(separator: ",").first else { return nil }
        return Int(rawSpan)
    }
    guard observedSpans == expectedSpans else { return false }
    var expectedContributions: [(node: Int, kind: String, span: Int, value: Double)] = []
    for span in expectedSpans {
        for index in nodes.indices where spans[index] == span && !textIndices.contains(index) {
            expectedContributions.append((nodes[index], "Minimum", span, 0))
        }
        if c114IsAnonymousGridBorderOverflowScene(sceneID) && span == 1 {
            for index in nodes.indices where spans[index] == span {
                for _ in 0..<2 {
                    expectedContributions.append((nodes[index], "MinContent", span, 20))
                }
            }
        } else {
            for _ in 0..<2 {
                for index in nodes.indices where spans[index] == span {
                    expectedContributions.append((
                        nodes[index], "MinContent", span,
                        textIndices.contains(index) ? 20 : imageWidths[index]))
                }
            }
        }
    }
    guard contributions.count == expectedContributions.count else { return false }
    for (index, contribution) in contributions.enumerated() {
        let expected = expectedContributions[index]
        guard (contribution["item"] as? Int) == expected.node,
              (contribution["kind"] as? String) == expected.kind,
              (contribution["span"] as? Int) == expected.span,
              let value = contribution["value"] as? NSNumber,
              abs(value.doubleValue - expected.value) <= 0.0001 else { return false }
    }
    return true
}

private func c114ExpectedImageMultiSpanInputs(_ sceneID: String, report: String) -> Bool {
    let spansAndWidths: [[Int]]
    switch sceneID {
    case "image-span2-forward": spansAndWidths = [[2, 24], [2, 48]]
    case "image-span2-reversed": spansAndWidths = [[2, 48], [2, 24]]
    case "image-span2-before-span3": spansAndWidths = [[2, 24], [2, 48], [3, 72]]
    case "image-span3-before-span2": spansAndWidths = [[3, 72], [2, 48], [2, 24]]
    default: return false
    }
    guard let diagnostics = c114GridTrackSizingDiagnostics(report),
          let revision = diagnostics["layoutInputRevision"] as? [String: Any],
          (revision["intrinsicSize"] as? Int) == 1,
          let inputs = diagnostics["gridItemIntrinsicInputs"] as? [[String: Any]],
          inputs.count == spansAndWidths.count else { return false }
    for (index, input) in inputs.enumerated() {
        guard (input["grid"] as? Int) == 1,
              (input["node"] as? Int) == index + 2,
              (input["sourceOrder"] as? Int) == index,
              let source = input["source"] as? [String: Any],
              (source["kind"] as? String) == "replaced",
              let natural = source["natural"] as? [String: Any],
              (natural["width"] as? NSNumber)?.doubleValue == 24,
              (natural["height"] as? NSNumber)?.doubleValue == 16,
              natural.keys.contains("aspectRatio"), natural["aspectRatio"] is NSNull else {
            return false
        }
    }
    return c114ImageMultiSpanTraceMatches(sceneID, spansAndWidths, diagnostics: diagnostics)
}

private func c114GridTrackSizingDiagnostics(_ report: String) -> [String: Any]? {
    guard let marker = report.range(of: "grid_track_sizing_diagnostics=") else { return nil }
    let jsonText = report[marker.upperBound...]
    var depth = 0
    var inString = false
    var escaped = false
    var endIndex: String.Index?
    for index in jsonText.indices {
        let character = jsonText[index]
        if inString {
            if escaped {
                escaped = false
            } else if character == "\\" {
                escaped = true
            } else if character == "\"" {
                inString = false
            }
            continue
        }
        if character == "\"" {
            inString = true
        } else if character == "{" {
            depth += 1
        } else if character == "}" {
            depth -= 1
            if depth == 0 {
                endIndex = jsonText.index(after: index)
                break
            }
        }
    }
    guard let endIndex,
          let data = String(jsonText[..<endIndex]).data(using: .utf8),
          let diagnostics = (try? JSONSerialization.jsonObject(with: data)) as? [String: Any] else {
        return nil
    }
    return diagnostics
}

private func c114ImageMultiSpanTraceMatches(
    _ sceneID: String, _ spansAndWidths: [[Int]], diagnostics: [String: Any]
) -> Bool {
    let expectedColumns = sceneID == "image-span2-before-span3"
        || sceneID == "image-span3-before-span2"
        ? [16.0, 28.0, 28.0, 0.0] : [12.0, 24.0, 24.0, 0.0]
    guard let traces = diagnostics["trackSizingTraces"] as? [[String: Any]],
          traces.count == 1,
          (traces[0]["grid"] as? Int) == 1,
          let passes = traces[0]["passes"] as? [[String: Any]],
          passes.count == 2,
          (passes[0]["axis"] as? String) == "Columns",
          (passes[1]["axis"] as? String) == "Rows",
          c114ImageMultiSpanTracksMatch(passes[0], expected: expectedColumns),
          c114ImageMultiSpanTracksMatch(passes[1], expected: [20, 20, 20]),
          let stages = passes[0]["stages"] as? [[String: Any]] else { return false }
    let observedSpans = stages.compactMap { stage -> Int? in
        guard let name = stage["stage"] as? String,
              name.hasPrefix("IntrinsicItemBatch { span: ") else { return nil }
        let suffix = name.dropFirst("IntrinsicItemBatch { span: ".count)
        guard let rawSpan = suffix.split(separator: ",").first else { return nil }
        return Int(rawSpan)
    }
    guard observedSpans == (spansAndWidths.count == 2 ? [2] : [2, 3]),
          let contributions = passes[0]["contributions"] as? [[String: Any]] else { return false }
    var expectedContributions: [(node: Int, kind: String, span: Int, value: Double)] = []
    for span in [2, 3] {
        for kind in ["Minimum", "MinContent"] {
            let repetitions = kind == "Minimum" ? 1 : 2
            for _ in 0..<repetitions {
                for (index, entry) in spansAndWidths.enumerated() where entry[0] == span {
                    expectedContributions.append((
                        node: index + 2, kind: kind, span: span,
                        value: kind == "Minimum" ? 0 : Double(entry[1])
                    ))
                }
            }
        }
    }
    guard contributions.count == expectedContributions.count else { return false }
    for (index, contribution) in contributions.enumerated() {
        let expected = expectedContributions[index]
        guard (contribution["item"] as? Int) == expected.node,
              (contribution["kind"] as? String) == expected.kind,
              (contribution["span"] as? Int) == expected.span,
              let value = contribution["value"] as? NSNumber,
              abs(value.doubleValue - expected.value) <= 0.0001 else { return false }
    }
    return true
}

private func c114ImageMultiSpanTracksMatch(
    _ pass: [String: Any], expected: [Double]
) -> Bool {
    guard let stages = pass["stages"] as? [[String: Any]],
          let finalStage = stages.last,
          let tracks = finalStage["tracks"] as? [[String: Any]] else { return false }
    let sizes = tracks.compactMap { track -> Double? in
        guard let gutter = track["gutter"] as? Bool, !gutter,
              let collapsed = track["collapsed"] as? Bool, !collapsed,
              let size = track["baseSize"] as? NSNumber else { return nil }
        return size.doubleValue
    }
    return sizes.count == expected.count
        && zip(sizes, expected).allSatisfy { abs($0.0 - $0.1) <= 0.0001 }
}

private func c114ImageTrackSizingTraceMatches(
    _ sceneID: String, diagnostics: [String: Any]
) -> Bool {
    if c114IsImageMultiSpanScene(sceneID) {
        let spansAndWidths: [[Int]]
        switch sceneID {
        case "image-span2-forward": spansAndWidths = [[2, 24], [2, 48]]
        case "image-span2-reversed": spansAndWidths = [[2, 48], [2, 24]]
        case "image-span2-before-span3": spansAndWidths = [[2, 24], [2, 48], [3, 72]]
        case "image-span3-before-span2": spansAndWidths = [[3, 72], [2, 48], [2, 24]]
        default: return false
        }
        return c114ImageMultiSpanTraceMatches(sceneID, spansAndWidths, diagnostics: diagnostics)
    }
    let expectedColumns: [(size: Double, gutter: Bool)]
    let expectedColumnContribution: Double?
    var expectedColumnContributionKind = "MinContent"
    var expectedRows: [(size: Double, gutter: Bool)] = [(16, false)]
    var expectedRowContribution: Double? = 16
    var expectedRowContributionKind = "MinContent"
    let expectedColumnSpan = sceneID == "spanning-min-content-items"
        || sceneID == "fixed-gutter-column-suggestion-clamp" ? 2 : 1
    let expectedRowSpan = sceneID == "fixed-gutter-row-suggestion-clamp" ? 2 : 1
    switch sceneID {
    case "fixed-gutter-column-suggestion-clamp":
        expectedColumns = [(3, false), (3, true), (4, false)]
        expectedColumnContribution = 10
        expectedColumnContributionKind = "Minimum"
        expectedRows = [(10.0 / 1.5, false)]
        expectedRowContribution = 10.0 / 1.5
        expectedRowContributionKind = "Minimum"
    case "fixed-gutter-row-suggestion-clamp":
        expectedColumns = [(15, false)]
        expectedColumnContribution = 15
        expectedColumnContributionKind = "Minimum"
        expectedRows = [(3, false), (3, true), (4, false)]
        expectedRowContribution = 10
        expectedRowContributionKind = "Minimum"
    case "row-transfer-input-capped-by-fixed-column-span":
        expectedColumns = [(18, false)]
        expectedColumnContribution = 18
        expectedColumnContributionKind = "Minimum"
        expectedRows = [(8.0 / 1.5, false)]
        expectedRowContribution = 8.0 / 1.5
        expectedRowContributionKind = "Minimum"
    case "column-transfer-input-capped-by-fixed-row-span":
        expectedColumns = [(12, false)]
        expectedColumnContribution = 12
        expectedColumnContributionKind = "Minimum"
        expectedRows = [(18, false)]
        expectedRowContribution = 18
        expectedRowContributionKind = "Minimum"
    case "column-auto-natural-ratio":
        expectedColumns = [(12, false)]
        expectedColumnContribution = 12
        expectedColumnContributionKind = "Minimum"
        expectedRows = [(20, false)]
        expectedRowContribution = nil
    case "column-target-max-width-clamp":
        expectedColumns = [(9, false)]
        expectedColumnContribution = 9
        expectedColumnContributionKind = "Minimum"
        expectedRows = [(20, false)]
        expectedRowContribution = nil
    case "column-opposite-min-over-max":
        expectedColumns = [(18, false)]
        expectedColumnContribution = 18
        expectedColumnContributionKind = "Minimum"
        expectedRows = [(20, false)]
        expectedRowContribution = nil
    case "column-border-box-transfer":
        expectedColumns = [(12, false)]
        expectedColumnContribution = 12
        expectedColumnContributionKind = "Minimum"
        expectedRows = [(20, false)]
        expectedRowContribution = nil
    case "column-content-box-transfer":
        expectedColumns = [(16, false)]
        expectedColumnContribution = 16
        expectedColumnContributionKind = "Minimum"
        expectedRows = [(20, false)]
        expectedRowContribution = nil
    case "column-target-min-width-clamp":
        expectedColumns = [(18, false)]
        expectedColumnContribution = 18
        expectedColumnContributionKind = "Minimum"
        expectedRows = [(20, false)]
        expectedRowContribution = nil
    case "row-target-max-height-clamp":
        expectedColumns = [(20, false)]
        expectedColumnContribution = nil
        expectedRows = [(6, false)]
        expectedRowContribution = 6
        expectedRowContributionKind = "Minimum"
    case "fixed-small-track-overflow":
        expectedColumns = [(12, false)]
        expectedColumnContribution = nil
    case "spanning-min-content-items":
        expectedColumns = [(10, false), (4, true), (10, false)]
        expectedColumnContribution = 24
    case "min-content-row-image":
        expectedColumns = [(100, false)]
        expectedColumnContribution = nil
    default:
        expectedColumns = [(24, false)]
        expectedColumnContribution = 24
    }
    let repeatedColumnSizing = [
        "column-target-min-width-clamp",
        "column-target-max-width-clamp",
        "column-opposite-min-over-max",
        "column-border-box-transfer",
        "column-content-box-transfer",
        "column-transfer-input-capped-by-fixed-row-span",
        "fixed-gutter-column-suggestion-clamp",
        "fixed-gutter-row-suggestion-clamp",
    ].contains(sceneID)
    let repeatedRowSizing = sceneID == "fixed-gutter-column-suggestion-clamp"
        || sceneID == "fixed-gutter-row-suggestion-clamp"
    guard let traces = diagnostics["trackSizingTraces"] as? [[String: Any]],
          traces.count == 1,
          let trace = traces.first,
          let grid = trace["grid"] as? Int, grid == 1,
          let passes = trace["passes"] as? [[String: Any]],
          passes.count == (repeatedRowSizing ? 4 : repeatedColumnSizing ? 3 : 2),
          c114ImageTrackPassMatches(passes[0], axis: "Columns",
                                    expectedTracks: expectedColumns,
                                    minContentContribution: expectedColumnContribution,
                                    span: expectedColumnSpan,
                                    minContentContributionKind: expectedColumnContributionKind),
          c114ImageTrackPassMatches(passes[1], axis: "Rows",
                                    expectedTracks: expectedRows,
                                    minContentContribution: expectedRowContribution,
                                    span: expectedRowSpan,
                                    minContentContributionKind: expectedRowContributionKind) else {
        return false
    }
    if repeatedColumnSizing && !c114ImageTrackPassMatches(
        passes[2], axis: "Columns", expectedTracks: expectedColumns,
        minContentContribution: expectedColumnContribution, span: expectedColumnSpan,
        minContentContributionKind: expectedColumnContributionKind
    ) {
        return false
    }
    if repeatedRowSizing && !c114ImageTrackPassMatches(
        passes[3], axis: "Rows", expectedTracks: expectedRows,
        minContentContribution: expectedRowContribution, span: expectedRowSpan,
        minContentContributionKind: expectedRowContributionKind
    ) {
        return false
    }
    return c114FixedGutterSuggestionTraceMatches(sceneID, passes: passes)
}

private func c114FixedGutterSuggestionTraceMatches(
    _ sceneID: String, passes: [[String: Any]]
) -> Bool {
    let passIndex: Int
    switch sceneID {
    case "fixed-gutter-column-suggestion-clamp": passIndex = 0
    case "fixed-gutter-row-suggestion-clamp": passIndex = 1
    default: return true
    }
    guard passes.indices.contains(passIndex),
          let contributions = passes[passIndex]["contributions"] as? [[String: Any]],
          let contribution = contributions.first(where: {
              ($0["item"] as? Int) == 2 && ($0["kind"] as? String) == "Minimum"
          }),
          let source = contribution["minimumSource"] as? String else {
        return false
    }
    return source.contains("PreferredSize")
        && source.contains("fixed_track_sum: Some(7.0)")
        && source.contains("fixed_gutter_sum: Some(3.0)")
        && source.contains("fixed_track_limit: Some(10.0)")
}

private func c114ImageTrackPassMatches(
    _ pass: [String: Any],
    axis: String,
    expectedTracks: [(size: Double, gutter: Bool)],
    minContentContribution: Double?,
    span: Int,
    minContentContributionKind: String
) -> Bool {
    guard let observedAxis = pass["axis"] as? String, observedAxis == axis,
          let stages = pass["stages"] as? [[String: Any]],
          let finalStage = stages.last,
          let tracks = finalStage["tracks"] as? [[String: Any]] else {
        return false
    }
    let activeTracks = tracks.filter { track in
        (track["collapsed"] as? Bool) == false
    }
    guard activeTracks.count == expectedTracks.count else { return false }
    for (index, expected) in expectedTracks.enumerated() {
        guard let baseSize = activeTracks[index]["baseSize"] as? NSNumber,
              abs(baseSize.doubleValue - expected.size) <= 0.0001,
              let gutter = activeTracks[index]["gutter"] as? Bool,
              gutter == expected.gutter else {
            return false
        }
    }
    guard let minContentContribution else { return true }
    guard let contributions = pass["contributions"] as? [[String: Any]] else { return false }
    return contributions.contains { contribution in
        guard let item = contribution["item"] as? Int, item == 2,
              let observedSpan = contribution["span"] as? Int, observedSpan == span,
              let kind = contribution["kind"] as? String,
              kind == minContentContributionKind,
              let value = contribution["value"] as? NSNumber else {
            return false
        }
        return abs(value.doubleValue - minContentContribution) <= 0.0001
    }
}

private func c114ExpectedNodeFrames(_ sceneID: String) -> String? {
    let sceneID = c114BaseNestedGridSceneID(sceneID)
    if sceneID.hasPrefix("nested-text-span") { return nil }
    if c114IsNestedGridDepthAnonymousImageScene(sceneID) {
        let minContent = sceneID.contains("-min-content-")
        let imageBeforeText = sceneID.contains("-image-before-text")
        let width = minContent ? 94 : 124
        let height = minContent ? 20 : 16
        let textWidth = minContent ? 70 : 100
        let imageX = imageBeforeText ? 0 : textWidth
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=\(width),height=\(height);"
            + "1:node=2,x=0,y=0,width=\(width),height=\(height);"
            + "2:node=3,x=0,y=0,width=\(width),height=\(height);"
            + "3:node=4,x=0,y=0,width=\(width),height=\(height);"
            + "4:node=5,x=\(imageX),y=0,width=24,height=16]"
    }
    switch sceneID {
    case "cyclic-minmax-calc-min-column", "definite-minmax-calc-min-column":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=14,height=14;"
            + "1:node=2,x=2,y=2,width=24,height=10]"
    case "cyclic-minmax-calc-min-both-axes":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=54,height=14;"
            + "1:node=2,x=2,y=2,width=104,height=22]"
    case "cyclic-minmax-calc-min-column-definite-row":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=54,height=14;"
            + "1:node=2,x=2,y=2,width=104,height=10]"
    case "definite-column-cyclic-minmax-calc-min-row":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=14,height=24;"
            + "1:node=2,x=2,y=2,width=10,height=42]"
    case "definite-minmax-calc-min-both-axes":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=14,height=14;"
            + "1:node=2,x=2,y=2,width=24,height=22]"
    case "cyclic-minmax-calc-min-row", "definite-minmax-calc-min-row":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=14,height=14;"
            + "1:node=2,x=2,y=2,width=10,height=22]"
    case "cyclic-calc-column", "cyclic-calc-row", "cyclic-calc-two-axis":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=16;"
            + "1:node=2,x=0,y=0,width=24,height=16]"
    case "definite-calc-control":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=20;"
            + "1:node=2,x=0,y=0,width=24,height=16]"
    case "cyclic-percent-two-axis-multitrack", "cyclic-percent-columns-multitrack",
         "cyclic-percent-rows-multitrack", "definite-percent-two-axis-multitrack":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=20;"
            + "1:node=2,x=0,y=0,width=20,height=10;"
            + "2:node=4,x=30,y=15,width=20,height=10]"
    case "cyclic-fit-content-column":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=50,height=20;"
            + "1:node=2,x=0,y=0,width=25,height=20]"
    case "cyclic-fit-content-row":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=20,height=40;"
            + "1:node=2,x=0,y=0,width=20,height=20]"
    case "definite-fit-content-column":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=20;"
            + "1:node=2,x=0,y=0,width=20,height=20]"
    case "cyclic-fit-content-calc-column":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=50,height=20;"
            + "1:node=2,x=0,y=0,width=29,height=20]"
    case "cyclic-fit-content-calc-row":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=20,height=40;"
            + "1:node=2,x=0,y=0,width=20,height=22]"
    case "definite-fit-content-calc-column":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=20;"
            + "1:node=2,x=0,y=0,width=24,height=20]"
    case "definite-fit-content-calc-small-limit":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=10,height=20;"
            + "1:node=2,x=0,y=0,width=9,height=20]"
    case "cyclic-minmax-calc-column":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=50,height=20;"
            + "1:node=2,x=0,y=0,width=29,height=20]"
    case "cyclic-minmax-calc-row":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=20,height=40;"
            + "1:node=2,x=0,y=0,width=20,height=22]"
    case "definite-minmax-calc-column":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=20;"
            + "1:node=2,x=0,y=0,width=24,height=20]"
    case "definite-minmax-calc-small-limit":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=10,height=20;"
            + "1:node=2,x=0,y=0,width=9,height=20]"
    case "cyclic-percent-column", "cyclic-percent-row",
         "cyclic-minmax-column", "cyclic-minmax-row":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=16;"
            + "1:node=2,x=0,y=0,width=24,height=16]"
    case "definite-percent-column", "definite-minmax-column":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=16;"
            + "1:node=2,x=0,y=0,width=24,height=16]"
    case "nested-depth-one-min-content":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=70,height=20;"
            + "1:node=2,x=0,y=0,width=70,height=20;"
            + "2:node=3,x=0,y=0,width=70,height=20]"
    case "nested-depth-two-min-content":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=70,height=20;"
            + "1:node=2,x=0,y=0,width=70,height=20;"
            + "2:node=3,x=0,y=0,width=70,height=20;"
            + "3:node=4,x=0,y=0,width=70,height=20]"
    case "nested-depth-three-min-content":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=70,height=20;"
            + "1:node=2,x=0,y=0,width=70,height=20;"
            + "2:node=3,x=0,y=0,width=70,height=20;"
            + "3:node=4,x=0,y=0,width=70,height=20;"
            + "4:node=5,x=0,y=0,width=70,height=20]"
    case "nested-depth-one-max-content":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=100,height=10;"
            + "1:node=2,x=0,y=0,width=100,height=10;"
            + "2:node=3,x=0,y=0,width=100,height=10]"
    case "nested-depth-two-max-content":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=100,height=10;"
            + "1:node=2,x=0,y=0,width=100,height=10;"
            + "2:node=3,x=0,y=0,width=100,height=10;"
            + "3:node=4,x=0,y=0,width=100,height=10]"
    case "nested-depth-three-max-content":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=100,height=10;"
            + "1:node=2,x=0,y=0,width=100,height=10;"
            + "2:node=3,x=0,y=0,width=100,height=10;"
            + "3:node=4,x=0,y=0,width=100,height=10;"
            + "4:node=5,x=0,y=0,width=100,height=10]"
    case "nested-grid-span-two":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=134,height=20;"
            + "1:node=2,x=0,y=0,width=134,height=20;"
            + "2:node=3,x=0,y=0,width=70,height=20;"
            + "3:node=5,x=74,y=0,width=60,height=20]"
    case "nested-grid-one-track":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=138,height=20;"
            + "1:node=2,x=0,y=0,width=134,height=20;"
            + "2:node=3,x=0,y=0,width=70,height=20;"
            + "3:node=5,x=74,y=0,width=60,height=20]"
    case "nested-grid-definite-flex-control":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=20;"
            + "1:node=2,x=0,y=0,width=120,height=20;"
            + "2:node=3,x=0,y=0,width=70,height=20;"
            + "3:node=5,x=74,y=0,width=60,height=20]"
    case "nested-grid-text-image-span-two":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=98,height=20;"
            + "1:node=2,x=0,y=0,width=98,height=20;"
            + "2:node=3,x=0,y=0,width=70,height=20;"
            + "3:node=5,x=74,y=0,width=24,height=16]"
    case "nested-grid-text-image-one-track":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=102,height=20;"
            + "1:node=2,x=0,y=0,width=98,height=20;"
            + "2:node=3,x=0,y=0,width=70,height=20;"
            + "3:node=5,x=74,y=0,width=24,height=16]"
    case "nested-grid-text-image-definite-flex-control":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=20;"
            + "1:node=2,x=0,y=0,width=120,height=20;"
            + "2:node=3,x=0,y=0,width=70,height=20;"
            + "3:node=5,x=74,y=0,width=24,height=16]"
    case "nested-grid-text-image-row-span-two":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=98,height=40;"
            + "1:node=2,x=0,y=0,width=98,height=40;"
            + "2:node=3,x=0,y=0,width=70,height=20;"
            + "3:node=5,x=74,y=24,width=24,height=16]"
    case "nested-grid-text-image-row-one-track":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=98,height=44;"
            + "1:node=2,x=0,y=0,width=98,height=40;"
            + "2:node=3,x=0,y=0,width=70,height=20;"
            + "3:node=5,x=74,y=24,width=24,height=16]"
    case "nested-grid-text-image-row-definite-flex-control":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=98,height=60;"
            + "1:node=2,x=0,y=0,width=98,height=60;"
            + "2:node=3,x=0,y=0,width=70,height=20;"
            + "3:node=5,x=74,y=24,width=24,height=16]"
    case "nested-grid-implicit-row-span-two":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=40;"
            + "1:node=2,x=0,y=0,width=24,height=40;"
            + "2:node=3,x=0,y=0,width=24,height=8;"
            + "3:node=4,x=0,y=12,width=4,height=8;"
            + "4:node=5,x=0,y=24,width=24,height=16]"
    case "nested-grid-implicit-row-one-row":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=44;"
            + "1:node=2,x=0,y=0,width=24,height=40;"
            + "2:node=3,x=0,y=0,width=24,height=8;"
            + "3:node=4,x=0,y=12,width=4,height=8;"
            + "4:node=5,x=0,y=24,width=24,height=16]"
    case "anonymous-runs-two-images-forward":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=184;"
            + "1:node=3,x=2,y=2,width=24,height=16;"
            + "2:node=5,x=14,y=62,width=48,height=32]"
    case "two-images-anonymous-runs-reversed":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=184;"
            + "1:node=2,x=14,y=62,width=48,height=32;"
            + "2:node=4,x=2,y=2,width=24,height=16]"
    case "two-anonymous-runs-three-images-forward":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=184;"
            + "1:node=3,x=2,y=2,width=24,height=16;"
            + "2:node=5,x=20,y=62,width=48,height=32;"
            + "3:node=6,x=2,y=122,width=72,height=48]"
    case "three-images-two-anonymous-runs-reversed":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=184;"
            + "1:node=2,x=2,y=122,width=72,height=48;"
            + "2:node=4,x=20,y=62,width=48,height=32;"
            + "3:node=6,x=2,y=2,width=24,height=16]"
    case "anonymous-text-two-images-forward":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;"
            + "1:node=3,x=0,y=0,width=24,height=16;"
            + "2:node=4,x=12,y=60,width=48,height=32]"
    case "two-images-anonymous-text-reversed":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;"
            + "1:node=2,x=12,y=60,width=48,height=32;"
            + "2:node=4,x=0,y=0,width=24,height=16]"
    case "anonymous-text-three-images-forward":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;"
            + "1:node=3,x=0,y=0,width=24,height=16;"
            + "2:node=4,x=18,y=60,width=48,height=32;"
            + "3:node=5,x=0,y=120,width=72,height=48]"
    case "three-images-anonymous-text-reversed":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;"
            + "1:node=2,x=0,y=120,width=72,height=48;"
            + "2:node=3,x=18,y=60,width=48,height=32;"
            + "3:node=5,x=0,y=0,width=24,height=16]"
    case "anonymous-text-image-text":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;"
            + "1:node=3,x=20,y=0,width=72,height=48]"
    case "image-before-comment-group":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;"
            + "1:node=2,x=0,y=0,width=72,height=48]"
    case "comment-group-before-image":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;"
            + "1:node=4,x=40,y=0,width=72,height=48]"
    case "whitespace-around-image":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;"
            + "1:node=3,x=0,y=0,width=72,height=48]"
    case "hidden-child-around-image":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;"
            + "1:node=5,x=20,y=0,width=72,height=48]"
    case "anonymous-min-content-wrap":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=20]"
    case "anonymous-comment-continuation", "anonymous-hidden-child-continuation":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=10]"
    case "anonymous-runs-around-block":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=10;"
            + "1:node=3,x=20,y=0,width=30,height=10]"
    case "whitespace-does-not-create-item":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=10;"
            + "1:node=2,x=0,y=0,width=10,height=10;"
            + "2:node=4,x=20,y=0,width=10,height=10]"
    default:
        break
    }
    if c114IsImplicitTextScene(sceneID) { return nil }
    if c114IsTextMultiSpanScene(sceneID) || c114IsImageMultiSpanScene(sceneID)
        || c114IsMixedMultiSpanScene(sceneID) { return nil }
    if c114NegativeTrackExpectation(sceneID) != nil {
        return nil
    }
    if let autoFitMinimumSizeNodeFrames = c114AutoFitMinimumSizeNodeFrames(sceneID) {
        return autoFitMinimumSizeNodeFrames
    }
    switch sceneID {
    case "spanning-auto-min-flex-tracks":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=100,height=30;"
            + "1:node=2,x=0,y=0,width=100,height=10]"
    case "competing-span2-forward":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=40;"
            + "1:node=2,x=0,y=0,width=80,height=10;"
            + "2:node=3,x=40,y=10,width=120,height=10]"
    case "competing-span2-reversed":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=40;"
            + "1:node=2,x=40,y=0,width=120,height=10;"
            + "2:node=3,x=0,y=10,width=80,height=10]"
    case "span2-before-span3":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=40;"
            + "1:node=2,x=0,y=0,width=80,height=10;"
            + "2:node=3,x=53.333332,y=10,width=120,height=10;"
            + "3:node=4,x=0,y=20,width=200,height=10]"
    case "span3-before-span2":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=40;"
            + "1:node=2,x=0,y=0,width=200,height=10;"
            + "2:node=3,x=53.333332,y=10,width=120,height=10;"
            + "3:node=4,x=0,y=20,width=80,height=10]"
    case "auto-fill-overflow-neighbors":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=60;"
            + "1:node=2,x=86,y=0,width=12,height=12]"
    case "both-fit-empty", "columns-fit-rows-fill-empty", "columns-fill-rows-fit-empty":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200]"
    case "columns-fill-control", "rows-fill-control", "both-fill-square", "both-fit-single",
         "both-fit-span-two":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;"
            + "1:node=2,x=28,y=28,width=12,height=12]"
    case "both-fit-second-repeat":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;"
            + "1:node=2,x=20,y=20,width=12,height=12]"
    case "both-fill-asymmetric":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=266;"
            + "1:node=2,x=28,y=28,width=12,height=12]"
    case "columns-fit-rows-fill-second-repeat":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;"
            + "1:node=2,x=20,y=109,width=12,height=12]"
    case "columns-fill-rows-fit-second-repeat":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;"
            + "1:node=2,x=109,y=20,width=12,height=12]"
    case "auto-fill-two-track-columns-both-190", "auto-fill-two-track-columns-both-200",
         "auto-fill-two-track-columns-both-342", "auto-fill-two-track-columns-prefix-200",
         "auto-fill-two-track-columns-suffix-200", "auto-fill-two-track-rows-both-190",
         "auto-fill-two-track-rows-both-200", "auto-fill-two-track-rows-both-342":
        return "node_frames_css_px=[0:node=1,x=0,y=0,\(c114ExpectedRootFrame(sceneID))]"
    case "auto-fill-fixed-neighbors", "auto-fill-pattern-neighbors":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;"
            + "1:node=2,x=190,y=0,width=10,height=40]"
    case "auto-fit-fixed-neighbors":
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;"
            + "1:node=2,x=28,y=0,width=162,height=40]"
    case "grid-gap-definite-percent":
        return c114GapNodeFrames("100", "40", "50", "20")
    case "grid-gap-cyclic-percent":
        return c114GapNodeFrames("100", "20", "50", "15")
    case "grid-gap-definite-calc":
        return c114GapNodeFrames("100", "40", "55", "25")
    case "grid-gap-cyclic-calc":
        return c114GapNodeFrames("105", "25", "55.5", "21.25")
    case "grid-gap-cyclic-percent-min-basis":
        return c114GapNodeFrames("120", "30", "52", "17.5")
    case "grid-gap-cyclic-calc-min-basis":
        return c114GapNodeFrames("120", "30", "57", "22.5")
    case "auto-text-max-content", "min-content-text-normal", "max-content-text-normal",
         "fit-content-text-minimum", "fit-content-calc-limit", "min-content-text-edge",
         "min-content-text-min-width", "min-content-text-max-width",
         "min-content-text-break-all", "min-content-text-nowrap",
         "auto-image-definite", "minmax-auto-image", "minmax-min-max-image",
         "fit-content-below-minimum", "fit-content-above-intrinsic",
         "fit-content-percent", "fixed-small-track-overflow",
         "spanning-min-content-items", "min-content-row-image",
         "column-auto-natural-ratio", "column-target-max-width-clamp",
         "row-target-max-height-clamp", "column-opposite-min-over-max",
         "column-border-box-transfer", "column-content-box-transfer",
         "column-target-min-width-clamp",
         "row-transfer-input-capped-by-fixed-column-span",
         "column-transfer-input-capped-by-fixed-row-span",
         "fixed-gutter-column-suggestion-clamp",
         "fixed-gutter-row-suggestion-clamp":
        let itemOrigin = sceneID.hasPrefix("fixed-gutter-") ? "x=1,y=1," : "x=0,y=0,"
        return "node_frames_css_px=[0:node=1,x=0,y=0,\(c114ExpectedRootFrame(sceneID));"
            + "1:node=2,\(itemOrigin)\(c114ExpectedItemFrame(sceneID))]"
    default:
        return nil
    }
}

private func c114IsTextMultiSpanScene(_ sceneID: String) -> Bool {
    ["text-span2-forward", "text-span2-reversed", "text-span2-before-span3",
     "text-span3-before-span2"].contains(sceneID)
}

private func c114ExpectedImageMultiSpanFramesMatch(_ sceneID: String, report: String) -> Bool {
    guard c114IsImageMultiSpanScene(sceneID) else { return true }
    let expected: [[Double]]
    switch sceneID {
    case "image-span2-forward":
        expected = [[0, 0, 40, 60], [1, 1, 24, 16], [13, 21, 48, 32]]
    case "image-span2-reversed":
        expected = [[0, 0, 40, 60], [13, 21, 48, 32], [1, 1, 24, 16]]
    case "image-span2-before-span3":
        expected = [[0, 0, 40, 60], [1, 1, 24, 16], [17, 21, 48, 32], [1, 41, 72, 48]]
    case "image-span3-before-span2":
        expected = [[0, 0, 40, 60], [1, 41, 72, 48], [17, 21, 48, 32], [1, 1, 24, 16]]
    default:
        return false
    }
    guard let marker = report.range(of: "node_frames_css_px=["),
          let end = report[marker.upperBound...].firstIndex(of: "]") else { return false }
    let frames = report[marker.upperBound..<end].split(separator: ";")
    guard frames.count == expected.count else { return false }
    for (index, frame) in frames.enumerated() {
        let fields = frame.split(separator: ",")
        guard fields.count == 5,
              let identity = fields[0].range(of: #"^\d+:node=\d+$"#, options: .regularExpression) else {
            return false
        }
        let identityParts = fields[0].split(separator: ":node=")
        guard identity.lowerBound == fields[0].startIndex,
              identityParts.count == 2,
              Int(identityParts[0]) == index,
              Int(identityParts[1]) == (index == 0 ? 1 : index + 1) else { return false }
        let prefixes = ["x=", "y=", "width=", "height="]
        for property in 0..<prefixes.count {
            guard fields[property + 1].hasPrefix(prefixes[property]),
                  let actual = Double(fields[property + 1].dropFirst(prefixes[property].count)),
                  actual.isFinite,
                  abs(actual - expected[index][property]) <= 1.0 / 64.0 else { return false }
        }
    }
    return true
}

private func c114ExpectedMixedMultiSpanFramesMatch(_ sceneID: String, report: String) -> Bool {
    guard c114IsMixedMultiSpanScene(sceneID) else { return true }
    let expected: [[Double]]
    let expectedNodes: [Int]
    switch sceneID {
    case "mixed-text-image-span2-forward":
        expected = [[0, 0, 220, 180], [0, 0, 61, 20], [25, 60, 72, 48]]
        expectedNodes = [1, 2, 4]
    case "mixed-text-image-span2-reversed":
        expected = [[0, 0, 220, 180], [25, 60, 72, 48], [0, 0, 61, 20]]
        expectedNodes = [1, 2, 3]
    case "mixed-text-image-span2-before-span3":
        expected = [
            [0, 0, 220, 180], [0, 0, 116.3333333333, 10],
            [52.6666666667, 60, 72, 48], [0, 120, 180, 10],
        ]
        expectedNodes = [1, 2, 4, 5]
    case "mixed-text-image-span3-before-span2":
        expected = [
            [0, 0, 220, 180], [0, 120, 180, 10],
            [52.6666666667, 60, 72, 48], [0, 0, 116.3333333333, 10],
        ]
        expectedNodes = [1, 2, 4, 5]
    default:
        return false
    }
    guard let marker = report.range(of: "node_frames_css_px=["),
          let end = report[marker.upperBound...].firstIndex(of: "]") else { return false }
    let frames = report[marker.upperBound..<end].split(separator: ";")
    guard frames.count == expected.count else { return false }
    for (index, frame) in frames.enumerated() {
        let fields = frame.split(separator: ",")
        guard fields.count == 5,
              let identity = fields[0].range(of: #"^\d+:node=\d+$"#, options: .regularExpression) else {
            return false
        }
        let identityParts = fields[0].split(separator: ":node=")
        guard identity.lowerBound == fields[0].startIndex,
              identityParts.count == 2,
              Int(identityParts[0]) == index,
              Int(identityParts[1]) == expectedNodes[index] else { return false }
        let prefixes = ["x=", "y=", "width=", "height="]
        for property in 0..<prefixes.count {
            guard fields[property + 1].hasPrefix(prefixes[property]),
                  let actual = Double(fields[property + 1].dropFirst(prefixes[property].count)),
                  actual.isFinite,
                  abs(actual - expected[index][property]) <= 1.0 / 64.0 else { return false }
        }
    }
    return true
}

private func c114ExpectedTextMultiSpanIntrinsicInputs(_ sceneID: String, report: String) -> Bool {
    let expected: [[Double]]
    switch sceneID {
    case "text-span2-forward":
        expected = [[50, 20, 80, 10], [70, 20, 120, 10]]
    case "text-span2-reversed":
        expected = [[70, 20, 120, 10], [50, 20, 80, 10]]
    case "text-span2-before-span3":
        expected = [[50, 20, 80, 10], [70, 20, 120, 10], [180, 10, 180, 10]]
    case "text-span3-before-span2":
        expected = [[180, 10, 180, 10], [70, 20, 120, 10], [50, 20, 80, 10]]
    default:
        return false
    }
    guard let marker = report.range(of: "grid_track_sizing_diagnostics=") else { return false }
    let jsonText = report[marker.upperBound...]
    var depth = 0
    var inString = false
    var escaped = false
    var endIndex: String.Index?
    for index in jsonText.indices {
        let character = jsonText[index]
        if inString {
            if escaped {
                escaped = false
            } else if character == "\\" {
                escaped = true
            } else if character == "\"" {
                inString = false
            }
            continue
        }
        if character == "\"" {
            inString = true
        } else if character == "{" {
            depth += 1
        } else if character == "}" {
            depth -= 1
            if depth == 0 {
                endIndex = jsonText.index(after: index)
                break
            }
        }
    }
    guard let endIndex,
          let data = String(jsonText[..<endIndex]).data(using: .utf8),
          let diagnostics = (try? JSONSerialization.jsonObject(with: data)) as? [String: Any],
          let inputs = diagnostics["gridItemIntrinsicInputs"] as? [[String: Any]],
          inputs.count == expected.count else { return false }
    for (index, input) in inputs.enumerated() {
        guard let grid = input["grid"] as? Int, grid == 1,
              let node = input["node"] as? Int, node == 2 + index * 2,
              let sourceOrder = input["sourceOrder"] as? Int, sourceOrder == index,
              let source = input["source"] as? [String: Any],
              let kind = source["kind"] as? String, kind == "text",
              let fontRegistryRevision = source["fontRegistryRevision"] as? Int,
              fontRegistryRevision == 1 else { return false }
        let values = ["minContent", "minContentHeight", "maxContent", "maxContentHeight"]
            .compactMap { (source[$0] as? NSNumber)?.doubleValue }
        guard values.count == 4,
              zip(values, expected[index]).allSatisfy({ abs($0.0 - $0.1) <= 0.0001 }) else {
            return false
        }
    }
    return true
}

private func c114ExpectedNestedTextMultiSpanIntrinsicInputs(
    _ sceneID: String, report: String
) -> Bool {
    let expected: [[Double]]
    switch sceneID {
    case "nested-text-span2-forward":
        expected = [[50, 20, 80, 10], [70, 20, 120, 10]]
    case "nested-text-span2-reversed":
        expected = [[70, 20, 120, 10], [50, 20, 80, 10]]
    case "nested-text-span2-before-span3":
        expected = [[50, 20, 80, 10], [70, 20, 120, 10], [180, 10, 180, 10]]
    case "nested-text-span3-before-span2":
        expected = [[180, 10, 180, 10], [70, 20, 120, 10], [50, 20, 80, 10]]
    default:
        return false
    }
    guard let marker = report.range(of: "grid_track_sizing_diagnostics=") else { return false }
    let jsonText = report[marker.upperBound...]
    var depth = 0
    var inString = false
    var escaped = false
    var endIndex: String.Index?
    for index in jsonText.indices {
        let character = jsonText[index]
        if inString {
            if escaped {
                escaped = false
            } else if character == "\\" {
                escaped = true
            } else if character == "\"" {
                inString = false
            }
            continue
        }
        if character == "\"" {
            inString = true
        } else if character == "{" {
            depth += 1
        } else if character == "}" {
            depth -= 1
            if depth == 0 {
                endIndex = jsonText.index(after: index)
                break
            }
        }
    }
    guard let endIndex,
          let data = String(jsonText[..<endIndex]).data(using: .utf8),
          let diagnostics = (try? JSONSerialization.jsonObject(with: data)) as? [String: Any],
          let inputs = diagnostics["gridItemIntrinsicInputs"] as? [[String: Any]],
          inputs.count == expected.count else { return false }
    for (index, input) in inputs.enumerated() {
        guard let grid = input["grid"] as? Int, grid == 4,
              let node = input["node"] as? Int, node == 5 + index * 2,
              let sourceOrder = input["sourceOrder"] as? Int, sourceOrder == index,
              let source = input["source"] as? [String: Any],
              let kind = source["kind"] as? String, kind == "text",
              let fontRegistryRevision = source["fontRegistryRevision"] as? Int,
              fontRegistryRevision == 1 else { return false }
        let values = ["minContent", "minContentHeight", "maxContent", "maxContentHeight"]
            .compactMap { (source[$0] as? NSNumber)?.doubleValue }
        guard values.count == 4,
              zip(values, expected[index]).allSatisfy({ abs($0.0 - $0.1) <= 0.0001 }) else {
            return false
        }
    }
    return true
}

private func c114ExpectedTextMultiSpanFramesMatch(_ sceneID: String, report: String) -> Bool {
    if sceneID.hasPrefix("nested-text-span") {
        return c114ExpectedNestedTextMultiSpanFramesMatch(sceneID, report: report)
    }
    guard c114IsTextMultiSpanScene(sceneID) else { return true }
    let expected: [[Double]]
    switch sceneID {
    case "text-span2-forward":
        expected = [[0, 0, 220, 60], [0, 0, 60, 20], [25, 20, 70, 20]]
    case "text-span2-reversed":
        expected = [[0, 0, 220, 60], [25, 20, 70, 20], [0, 0, 60, 20]]
    case "text-span2-before-span3":
        expected = [
            [0, 0, 220, 60], [0, 0, 116.6666666667, 10],
            [53.3333333333, 20, 126.6666666667, 10], [0, 40, 180, 10],
        ]
    case "text-span3-before-span2":
        expected = [
            [0, 0, 220, 60], [0, 40, 180, 10],
            [53.3333333333, 20, 126.6666666667, 10], [0, 0, 116.6666666667, 10],
        ]
    default:
        return false
    }
    guard let marker = report.range(of: "node_frames_css_px=["),
          let end = report[marker.upperBound...].firstIndex(of: "]") else { return false }
    let frames = report[marker.upperBound..<end].split(separator: ";")
    guard frames.count == expected.count else { return false }
    for (index, frame) in frames.enumerated() {
        let fields = frame.split(separator: ",")
        guard fields.count == 5,
              let identity = fields[0].range(of: #"^\d+:node=\d+$"#, options: .regularExpression) else {
            return false
        }
        let identityParts = fields[0].split(separator: ":node=")
        let expectedNode = index == 0 ? 1 : index * 2
        guard identity.lowerBound == fields[0].startIndex,
              identityParts.count == 2,
              Int(identityParts[0]) == index,
              Int(identityParts[1]) == expectedNode else { return false }
        let prefixes = ["x=", "y=", "width=", "height="]
        for property in 0..<prefixes.count {
            guard fields[property + 1].hasPrefix(prefixes[property]),
                  let actual = Double(fields[property + 1].dropFirst(prefixes[property].count)),
                  actual.isFinite,
                  abs(actual - expected[index][property]) <= 1.0 / 64.0 else { return false }
        }
    }
    return true
}

private func c114ExpectedNestedTextMultiSpanFramesMatch(
    _ sceneID: String, report: String
) -> Bool {
    let expected: [[Double]]
    switch sceneID {
    case "nested-text-span2-forward":
        expected = [
            [0, 0, 95, 60], [0, 0, 95, 60], [0, 0, 95, 60], [0, 0, 95, 60],
            [0, 0, 60, 20], [25, 20, 70, 20],
        ]
    case "nested-text-span2-reversed":
        expected = [
            [0, 0, 95, 60], [0, 0, 95, 60], [0, 0, 95, 60], [0, 0, 95, 60],
            [25, 20, 70, 20], [0, 0, 60, 20],
        ]
    case "nested-text-span2-before-span3":
        expected = [
            [0, 0, 180, 60], [0, 0, 180, 60], [0, 0, 180, 60], [0, 0, 180, 60],
            [0, 0, 116.6666666667, 10], [53.3333333333, 20, 126.6666666667, 10],
            [0, 40, 180, 10],
        ]
    case "nested-text-span3-before-span2":
        expected = [
            [0, 0, 180, 60], [0, 0, 180, 60], [0, 0, 180, 60], [0, 0, 180, 60],
            [0, 40, 180, 10], [53.3333333333, 20, 126.6666666667, 10],
            [0, 0, 116.6666666667, 10],
        ]
    default:
        return false
    }
    guard let marker = report.range(of: "node_frames_css_px=["),
          let end = report[marker.upperBound...].firstIndex(of: "]") else { return false }
    let frames = report[marker.upperBound..<end].split(separator: ";")
    guard frames.count == expected.count else { return false }
    for (index, frame) in frames.enumerated() {
        let fields = frame.split(separator: ",")
        guard fields.count == 5,
              let identity = fields[0].range(of: #"^\d+:node=\d+$"#, options: .regularExpression) else {
            return false
        }
        let identityParts = fields[0].split(separator: ":node=")
        let expectedNode = index < 4 ? index + 1 : 5 + (index - 4) * 2
        guard identity.lowerBound == fields[0].startIndex,
              identityParts.count == 2,
              Int(identityParts[0]) == index,
              Int(identityParts[1]) == expectedNode else { return false }
        let prefixes = ["x=", "y=", "width=", "height="]
        for property in 0..<prefixes.count {
            guard fields[property + 1].hasPrefix(prefixes[property]),
                  let actual = Double(fields[property + 1].dropFirst(prefixes[property].count)),
                  actual.isFinite,
                  abs(actual - expected[index][property]) <= 1.0 / 64.0 else { return false }
        }
    }
    return true
}

private func c114GapNodeFrames(
    _ gridWidth: String, _ gridHeight: String, _ secondColumnX: String, _ secondRowY: String
) -> String {
    "node_frames_css_px=[0:node=1,x=0,y=0,width=\(gridWidth),height=\(gridHeight);"
        + "1:node=2,x=0,y=0,width=40,height=10;"
        + "2:node=3,x=\(secondColumnX),y=0,width=60,height=10;"
        + "3:node=4,x=0,y=\(secondRowY),width=40,height=10;"
        + "4:node=5,x=\(secondColumnX),y=\(secondRowY),width=60,height=10]"
}

private func c114ExpectedBoxCount(_ sceneID: String) -> Int {
    let sceneID = c114BaseNestedGridSceneID(sceneID)
    if sceneID == "cyclic-percent-column" || sceneID == "cyclic-percent-row"
        || sceneID == "definite-percent-column" || sceneID == "cyclic-minmax-column"
        || sceneID == "cyclic-minmax-row" || sceneID == "definite-minmax-column"
        || sceneID == "cyclic-fit-content-column" || sceneID == "cyclic-fit-content-row"
        || sceneID == "definite-fit-content-column"
        || sceneID == "cyclic-fit-content-calc-column"
        || sceneID == "cyclic-fit-content-calc-row"
        || sceneID == "definite-fit-content-calc-column"
        || sceneID == "definite-fit-content-calc-small-limit"
        || sceneID == "cyclic-minmax-calc-column"
        || sceneID == "cyclic-minmax-calc-row"
        || sceneID == "cyclic-minmax-calc-min-column"
        || sceneID == "cyclic-minmax-calc-min-row"
        || sceneID == "definite-minmax-calc-min-column"
        || sceneID == "definite-minmax-calc-min-row"
        || sceneID == "cyclic-minmax-calc-min-both-axes"
        || sceneID == "cyclic-minmax-calc-min-column-definite-row"
        || sceneID == "definite-column-cyclic-minmax-calc-min-row"
        || sceneID == "definite-minmax-calc-min-both-axes"
        || sceneID == "definite-minmax-calc-column"
        || sceneID == "definite-minmax-calc-small-limit" {
        return 2
    }
    if sceneID == "cyclic-percent-two-axis-multitrack"
        || sceneID == "cyclic-percent-columns-multitrack"
        || sceneID == "cyclic-percent-rows-multitrack"
        || sceneID == "definite-percent-two-axis-multitrack" {
        return 3
    }
    if sceneID == "cyclic-calc-column" || sceneID == "cyclic-calc-row"
        || sceneID == "cyclic-calc-two-axis" || sceneID == "definite-calc-control" {
        return 2
    }
    if sceneID.hasPrefix("nested-text-span") {
        return sceneID.contains("before-span3") || sceneID.contains("span3-before-span2")
            ? 7 : 6
    }
    if c114IsNestedGridDepthAnonymousImageScene(sceneID) { return 5 }
    if sceneID == "nested-depth-one-min-content"
        || sceneID == "nested-depth-one-max-content" { return 3 }
    if sceneID == "nested-depth-two-min-content"
        || sceneID == "nested-depth-two-max-content" { return 4 }
    if sceneID == "nested-depth-three-min-content"
        || sceneID == "nested-depth-three-max-content" { return 5 }
    if ["nested-grid-span-two", "nested-grid-one-track",
        "nested-grid-definite-flex-control", "nested-grid-text-image-span-two",
        "nested-grid-text-image-one-track",
        "nested-grid-text-image-definite-flex-control",
        "nested-grid-text-image-row-span-two", "nested-grid-text-image-row-one-track",
        "nested-grid-text-image-row-definite-flex-control"].contains(sceneID) { return 4 }
    if ["nested-grid-implicit-row-span-two", "nested-grid-implicit-row-one-row"]
        .contains(sceneID) { return 5 }
    if c114IsAnonymousGridBorderOverflowThreeImageScene(sceneID) { return 4 }
    if c114IsAnonymousGridBorderOverflowScene(sceneID) { return 3 }
    if sceneID == "anonymous-text-two-images-forward"
        || sceneID == "two-images-anonymous-text-reversed" { return 3 }
    if sceneID == "anonymous-text-three-images-forward"
        || sceneID == "three-images-anonymous-text-reversed" { return 4 }
    if c114IsAnonymousTextImageScene(sceneID) { return 2 }
    if ["anonymous-min-content-wrap", "anonymous-comment-continuation",
        "anonymous-hidden-child-continuation"].contains(sceneID) {
        return 1
    }
    if sceneID == "anonymous-runs-around-block" { return 2 }
    if sceneID == "whitespace-does-not-create-item" { return 3 }
    if sceneID == "mixed-text-image-span2-forward"
        || sceneID == "mixed-text-image-span2-reversed" { return 3 }
    if sceneID == "mixed-text-image-span2-before-span3"
        || sceneID == "mixed-text-image-span3-before-span2" { return 4 }
    if sceneID == "image-span2-forward" || sceneID == "image-span2-reversed" { return 3 }
    if sceneID == "image-span2-before-span3" || sceneID == "image-span3-before-span2" {
        return 4
    }
    if sceneID == "competing-span2-forward" || sceneID == "competing-span2-reversed" {
        return 3
    }
    if sceneID == "span2-before-span3" || sceneID == "span3-before-span2" { return 4 }
    if sceneID == "text-span2-forward" || sceneID == "text-span2-reversed" { return 3 }
    if sceneID == "text-span2-before-span3" || sceneID == "text-span3-before-span2" { return 4 }
    if sceneID == "typed-fixed-cycle-three-leading-trailing" { return 4 }
    if ["typed-mixed-cycle", "typed-auto-fit-occupied", "typed-auto-fit-gutter"]
        .contains(sceneID) { return 3 }
    if sceneID == "columns-fit-negative-and-repeat-occupied"
        || sceneID == "columns-fill-negative-positive-cycle" { return 3 }
    if c114AutoRepeatNegativeLineSceneIDs.contains(sceneID) { return 2 }
    let implicitTrackBoxCount = c114ImplicitTextBoxCount(sceneID)
    if implicitTrackBoxCount != 0 { return implicitTrackBoxCount }
    if let expectation = c114NegativeTrackExpectation(sceneID) { return expectation.boxCount }
    if let autoFitMinimumSizeItemFrame = c114AutoFitMinimumSizeItemFrame(sceneID) {
        return autoFitMinimumSizeItemFrame.isEmpty ? 1 : 2
    }
    if sceneID == "auto-fit-span-neighbors" { return 3 }
    if ["auto-fit-empty-neighbors", "auto-fill-max-size-neighbors",
        "auto-fill-min-size-neighbors", "auto-fill-two-track-columns-both-190",
        "auto-fill-two-track-columns-both-200", "auto-fill-two-track-columns-both-342",
        "auto-fill-two-track-columns-prefix-200", "auto-fill-two-track-columns-suffix-200",
        "auto-fill-two-track-rows-both-190", "auto-fill-two-track-rows-both-200",
        "auto-fill-two-track-rows-both-342", "both-fit-empty",
        "columns-fit-rows-fill-empty", "columns-fill-rows-fit-empty"].contains(sceneID) { return 1 }
    return sceneID.hasPrefix("grid-gap-") ? 5 : 2
}

private func c114BaseNestedGridSceneID(_ sceneID: String) -> String {
    guard sceneID.hasPrefix("nested-depth-"), sceneID.hasSuffix("-two-axis") else {
        return sceneID
    }
    return String(sceneID.dropLast("-two-axis".count))
}

private func c114IsImplicitTextScene(_ sceneID: String) -> Bool {
    c114ImplicitTextRootFrame(sceneID) != nil
}

private func c114IsAnonymousTextScene(_ sceneID: String) -> Bool {
    ["anonymous-min-content-wrap", "anonymous-comment-continuation",
     "anonymous-runs-around-block", "anonymous-hidden-child-continuation",
     "whitespace-does-not-create-item"].contains(sceneID)
}

private func c114IsAnonymousTextImageScene(_ sceneID: String) -> Bool {
    c114IsNestedGridDepthAnonymousImageScene(sceneID)
        || ["anonymous-text-image-text", "image-before-comment-group",
     "comment-group-before-image", "whitespace-around-image",
     "hidden-child-around-image"].contains(sceneID)
}

private func c114IsNestedGridDepthAnonymousImageScene(_ sceneID: String) -> Bool {
    ["depth-three-min-content-text-before-image",
     "depth-three-min-content-image-before-text",
     "depth-three-max-content-text-before-image",
     "depth-three-max-content-image-before-text"].contains(sceneID)
}

private func c114IsAnonymousTextMultiImageScene(_ sceneID: String) -> Bool {
    ["anonymous-text-two-images-forward", "two-images-anonymous-text-reversed",
     "anonymous-text-three-images-forward", "three-images-anonymous-text-reversed"]
        .contains(sceneID)
        || c114IsAnonymousGridBorderOverflowScene(sceneID)
}

private func c114IsAnonymousGridBorderOverflowScene(_ sceneID: String) -> Bool {
    ["anonymous-runs-two-images-forward", "two-images-anonymous-runs-reversed"]
        .contains(sceneID)
        || c114IsAnonymousGridBorderOverflowThreeImageScene(sceneID)
}

private func c114IsAnonymousGridBorderOverflowThreeImageScene(_ sceneID: String) -> Bool {
    ["two-anonymous-runs-three-images-forward", "three-images-two-anonymous-runs-reversed"]
        .contains(sceneID)
}

private func c114ImplicitTextRootFrame(_ sceneID: String) -> String? {
    switch sceneID {
    case "row-min-content", "row-max-content", "row-auto-placement-min-content",
         "row-auto-placement-auto", "row-positive-implicit-locked-item":
        return "width=40,height=80"
    case "column-min-content", "column-max-content", "column-auto-placement-max-content",
         "column-auto-placement-auto", "implicit-percentage-column",
         "implicit-fraction-column", "implicit-minmax-column", "implicit-fit-content-column",
         "implicit-calc-column", "column-positive-implicit-locked-item":
        return "width=160,height=40"
    case "two-implicit-columns-min-content", "cycled-intrinsic-columns":
        return "width=240,height=40"
    case "cycled-fixed-max-rows":
        return "width=40,height=100"
    case "implicit-min-content-overflow-border":
        return "width=44,height=44"
    case "explicit-track-control":
        return "width=40,height=24"
    default:
        return nil
    }
}

private func c114ImplicitTextBoxCount(_ sceneID: String) -> Int {
    switch sceneID {
    case "row-min-content", "row-max-content", "column-min-content", "column-max-content",
         "explicit-track-control", "row-positive-implicit-locked-item",
         "column-positive-implicit-locked-item":
        return 2
    case "row-auto-placement-min-content", "column-auto-placement-max-content",
         "row-auto-placement-auto", "column-auto-placement-auto",
         "two-implicit-columns-min-content", "implicit-percentage-column",
         "implicit-fraction-column", "implicit-minmax-column", "implicit-fit-content-column",
         "implicit-min-content-overflow-border", "implicit-calc-column":
        return 3
    case "cycled-intrinsic-columns", "cycled-fixed-max-rows":
        return 5
    default:
        return 0
    }
}

private func c114AutoFitMinimumSizeRootFrame(_ sceneID: String) -> String? {
    guard sceneID.hasPrefix("auto-fit-min-"),
          let size = sceneID.split(separator: "-").last,
          sceneID.contains("-columns-") || sceneID.contains("-rows-") else {
        return nil
    }
    guard let minSize = Int(size) else { return nil }
    return sceneID.contains("-columns-")
        ? "width=\(minSize),height=60"
        : "width=50,height=\(minSize)"
}

private func c114AutoFitMinimumSizeItemFrame(_ sceneID: String) -> String? {
    guard sceneID.hasPrefix("auto-fit-min-") else { return nil }
    return sceneID.contains("-single-") || sceneID.contains("-span-two-")
        || sceneID.contains("-second-repeat-") ? "width=12,height=12" : ""
}

private func c114AutoFitMinimumSizeNodeFrames(_ sceneID: String) -> String? {
    guard let rootFrame = c114AutoFitMinimumSizeRootFrame(sceneID),
          let itemFrame = c114AutoFitMinimumSizeItemFrame(sceneID) else {
        return nil
    }
    guard !itemFrame.isEmpty else {
        return "node_frames_css_px=[0:node=1,x=0,y=0,\(rootFrame)]"
    }
    let position = sceneID.contains("-second-repeat-")
        ? sceneID.contains("-columns-") ? "x=20,y=0" : "x=0,y=20"
        : sceneID.contains("-columns-") ? "x=28,y=0" : "x=0,y=28"
    return "node_frames_css_px=[0:node=1,x=0,y=0,\(rootFrame);"
        + "1:node=2,\(position),width=12,height=12]"
}

private struct C124MetalPixelCase {
    let direction: String
    let blueOrder: Int
    let redOrder: Int
    let runID: String

    var caseID: String {
        "flex-\(direction)-paint-order-\(blueOrder)-\(redOrder)"
    }

    var cssSample: (x: Int, y: Int) {
        (70, direction.hasPrefix("column") && direction == "column-reverse" ? 175 : 155)
    }

    var expectedRGBA: String {
        let blueOnTop = (direction == "row-reverse" || direction == "column-reverse")
            ? blueOrder == 0 : blueOrder == 1
        return blueOnTop ? "#2866f6ff" : "#e5484dff"
    }

    static func parse(arguments: [String]) -> C124MetalPixelCase? {
        guard let raw = arguments.first(where: {
            $0.hasPrefix("--spinon-c124-metal-surface-readback=")
        }) else { return nil }
        let fields = String(raw.dropFirst("--spinon-c124-metal-surface-readback=".count))
            .split(separator: ",", omittingEmptySubsequences: false)
        guard fields.count == 3,
              let blueOrder = Int(fields[1]), let redOrder = Int(fields[2]),
              ["row", "row-reverse", "column", "column-reverse"].contains(String(fields[0])),
              (blueOrder == 0 || blueOrder == 1), redOrder == 0,
              let runArgument = arguments.first(where: {
                  $0.hasPrefix("--spinon-c124-metal-surface-run=")
              }) else { return nil }
        let runID = String(runArgument.dropFirst("--spinon-c124-metal-surface-run=".count))
        guard !runID.isEmpty,
              runID.allSatisfy({ $0.isASCII && ($0.isLetter || $0.isNumber || $0 == "-" || $0 == "_") })
        else { return nil }
        return C124MetalPixelCase(
            direction: String(fields[0]), blueOrder: blueOrder,
            redOrder: redOrder, runID: runID
        )
    }
}




private struct C111MetalPixelSample: Decodable {
    let id: String
    let nodeId: String
    let xCssPx: UInt32
    let yCssPx: UInt32
    let rgba: [UInt32]
}

private struct C132MetalPixelSample {
    let id: String
    let xCssPx: UInt32
    let yCssPx: UInt32
    let rgba: [UInt32]
}

private struct C132MetalPixelRun {
    let runID: String
    let samples: [C132MetalPixelSample]

    static let fixedSamples = [
        C132MetalPixelSample(id: "visible-escape", xCssPx: 150, yCssPx: 30,
                             rgba: [227, 84, 78, 255]),
        C132MetalPixelSample(id: "parent-gap", xCssPx: 136, yCssPx: 30,
                             rgba: [16, 24, 39, 255]),
        C132MetalPixelSample(id: "parent-interior", xCssPx: 20, yCssPx: 20,
                             rgba: [52, 66, 86, 255]),
        C132MetalPixelSample(id: "parent-border", xCssPx: 11, yCssPx: 11,
                             rgba: [240, 198, 116, 255]),
        C132MetalPixelSample(id: "root-background", xCssPx: 50, yCssPx: 110,
                             rgba: [16, 24, 39, 255]),
    ]

    static func parse(arguments: [String]) -> C132MetalPixelRun? {
        let prefix = "--spinon-c132-metal-surface-pixels="
        let values = arguments.filter { $0.hasPrefix(prefix) }
        guard values.count == 1 else { return nil }
        let runID = String(values[0].dropFirst(prefix.count))
        guard (8...80).contains(runID.count),
              runID.allSatisfy({ $0.isASCII && ($0.isLetter || $0.isNumber || $0 == "-" || $0 == "_") })
        else { return nil }
        return C132MetalPixelRun(runID: runID, samples: fixedSamples)
    }
}

private struct C111MetalPixelRun {
    let runID: String
    let sceneID: String
    let markerPrefix: String
    let samples: [C111MetalPixelSample]
    private static let c1121PixelSceneIDs: Set<String> = [
        "numeric-lines", "positive-spans", "reversed-lines",
        "shorthand-cascade", "overlap-source-order", "nested-placement-modes",
    ]
    private static let c1122PixelSceneIDs: Set<String> = [
        "definite-first-mixed", "auto-only-fill", "positive-implicit-row",
        "row-locked-auto-column", "column-locked-auto-row", "auto-column-span",
        "sparse-hole-not-backfilled", "implicit-track-size-and-gap",
        "hidden-child-excluded", "nested-independent-placement", "column-cursor-wrap",
    ]
    private static let c1123PixelSceneIDs: Set<String> = [
        "row-sparse-baseline", "row-dense-hole-fill", "row-dense-span-collision",
        "row-sparse-hole-control", "row-dense-fixed-column",
        "row-dense-positive-implicit-row", "column-sparse-explicit-fill",
        "column-sparse-positive-implicit-column", "column-sparse-fixed-row",
        "column-sparse-fixed-column", "column-dense-hole-fill",
        "column-sparse-hole-control", "column-dense-span-collision",
        "column-dense-positive-implicit-column", "definite-first-dense",
        "hidden-item-excluded", "nested-mixed-flow", "auto-track-no-implicit",
        "auto-span-at-track-limit",
    ]

    private static let c1131RepeatPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("grid-background", "c1131-multi-track-repeat-grid", 300, 120, [237, 241, 247, 255]),
        ("item-blue", "c1131-multi-track-repeat-a", 10, 10, [40, 102, 246, 255]),
        ("item-red", "c1131-multi-track-repeat-b", 40, 10, [229, 72, 77, 255]),
        ("column-gap", "c1131-multi-track-repeat-grid", 112, 10, [237, 241, 247, 255]),
        ("row-gap", "c1131-multi-track-repeat-grid", 10, 53, [237, 241, 247, 255]),
    ]

    private static let c1132NamedLinePixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("grid-background", "c1132-positive-grid", 20, 80, [237, 241, 247, 255]),
        ("item-blue", "c1132-positive-item", 100, 10, [40, 102, 246, 255]),
    ]
    private static let c1133TemplateAreaPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("grid-background", "c1133-main-grid", 150, 10, [16, 24, 32, 255]),
        ("item-blue", "c1133-main-item", 100, 50, [62, 99, 221, 255]),
    ]
    private static let c1133UnknownAreaOverflowPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("grid-interior", "fixed-implicit-grid", 150, 110, [16, 24, 32, 255]),
        ("known-area", "fixed-known-area", 10, 10, [48, 164, 108, 255]),
        ("unknown-area-overflow", "fixed-unknown-area", 182, 127, [229, 72, 77, 255]),
    ]
    private static let c114TextOverflowPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("item-inside-root", "c114-break-all-item", 5, 20, [229, 72, 77, 255]),
        ("item-overflow-beyond-root", "c114-break-all-item", 5, 45, [229, 72, 77, 255]),
        ("root-background-control", "c114-break-all-grid", 15, 20, [16, 24, 32, 255]),
    ]
    private static let c114ImageOverflowPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("item-inside-root", "c114-fixed-small-image-item", 5, 8, [229, 72, 77, 255]),
        ("item-overflow-beyond-root", "c114-fixed-small-image-item", 20, 8, [229, 72, 77, 255]),
        ("root-background-control", "c114-fixed-small-image-grid", 6, 25, [16, 24, 32, 255]),
    ]
    private static let c114ImageRatioTransferPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("item-inside-root", "c114-column-auto-natural-ratio-item", 5, 4, [229, 72, 77, 255]),
        ("item-overflow-beyond-root", "c114-column-auto-natural-ratio-item", 10, 4, [229, 72, 77, 255]),
        ("root-background-control", "c114-column-auto-natural-ratio-grid", 6, 15, [16, 24, 32, 255]),
    ]
    private static let c114AutoRepeatOverflowPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("overflow-root-edge-inside", "c114-auto-fill-overflow-neighbors-grid", 39, 8, [34, 197, 94, 255]),
        ("overflow-root-edge-outside", "c114-auto-fill-overflow-neighbors-grid", 40, 8, [0, 0, 0, 255]),
        ("overflow-item-outside-root", "c114-auto-fill-overflow-neighbors-item", 91, 8, [229, 72, 77, 255]),
        ("overflow-root-background", "c114-auto-fill-overflow-neighbors-grid", 10, 30, [34, 197, 94, 255]),
    ]
    private static let c114ImplicitTrackOverflowPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("overflow-root-border-inside", "c114-implicit-min-content-overflow-border-grid", 0, 8, [16, 24, 32, 255]),
        ("overflow-root-border-covered", "c114-implicit-min-content-overflow-border-grid", 43, 8, [229, 72, 77, 255]),
        ("overflow-root-border-outside", "c114-implicit-min-content-overflow-border-grid", 44, 8, [229, 72, 77, 255]),
        ("overflow-item-outside-root", "c114-implicit-min-content-overflow-border-item-2", 91, 8, [229, 72, 77, 255]),
        ("overflow-after-item", "c114-implicit-min-content-overflow-border-grid", 92, 8, [0, 0, 0, 255]),
    ]
    private static let c114RowTransferOverflowPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("inside-item", "c114-row-transfer-fixed-column-item", 4, 2, [229, 72, 77, 255]),
        ("right-of-grid", "c114-row-transfer-fixed-column-item", 12, 2, [229, 72, 77, 255]),
        ("below-grid", "c114-row-transfer-fixed-column-item", 4, 8, [229, 72, 77, 255]),
    ]
    private static let c114ColumnTransferOverflowPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("inside-item", "c114-column-transfer-fixed-row-item", 2, 2, [229, 72, 77, 255]),
        ("right-of-grid", "c114-column-transfer-fixed-row-item", 8, 2, [229, 72, 77, 255]),
        ("below-grid", "c114-column-transfer-fixed-row-item", 2, 12, [229, 72, 77, 255]),
    ]
    private static let c114FixedGutterColumnPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("grid-border", "c114-fixed-gutter-column-grid", 0, 2, [34, 197, 94, 255]),
        ("inside-item", "c114-fixed-gutter-column-item", 2, 2, [229, 72, 77, 255]),
        ("right-of-grid", "c114-fixed-gutter-column-item", 12, 2, [229, 72, 77, 255]),
        ("below-grid", "c114-fixed-gutter-column-item", 2, 6, [229, 72, 77, 255]),
    ]
    private static let c114FixedGutterRowPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("grid-border", "c114-fixed-gutter-row-grid", 0, 2, [34, 197, 94, 255]),
        ("inside-item", "c114-fixed-gutter-row-item", 2, 2, [229, 72, 77, 255]),
        ("right-of-grid", "c114-fixed-gutter-row-item", 6, 2, [229, 72, 77, 255]),
        ("below-grid", "c114-fixed-gutter-row-item", 2, 12, [229, 72, 77, 255]),
    ]
    private static let c114GridGapDefinitePercentPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("gap-background", "grid-gap-definite-percent-grid", 45, 5, [16, 24, 32, 255]),
        ("row-gap-background", "grid-gap-definite-percent-grid", 5, 12, [16, 24, 32, 255]),
        ("inside-item", "grid-gap-definite-percent-item-1", 5, 5, [229, 72, 77, 255]),
        ("right-of-grid", "grid-gap-definite-percent-item-2", 105, 5, [229, 72, 77, 255]),
    ]
    private static let c114GridGapCyclicPercentPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("gap-background", "grid-gap-cyclic-percent-grid", 45, 5, [16, 24, 32, 255]),
        ("row-gap-background", "grid-gap-cyclic-percent-grid", 5, 12, [16, 24, 32, 255]),
        ("inside-item", "grid-gap-cyclic-percent-item-1", 5, 5, [229, 72, 77, 255]),
        ("right-of-grid", "grid-gap-cyclic-percent-item-2", 105, 5, [229, 72, 77, 255]),
        ("below-grid", "grid-gap-cyclic-percent-item-4", 5, 22, [229, 72, 77, 255]),
    ]
    private static let c114GridGapDefiniteCalcPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("gap-background", "grid-gap-definite-calc-grid", 45, 5, [16, 24, 32, 255]),
        ("row-gap-background", "grid-gap-definite-calc-grid", 5, 12, [16, 24, 32, 255]),
        ("inside-item", "grid-gap-definite-calc-item-1", 5, 5, [229, 72, 77, 255]),
        ("right-of-grid", "grid-gap-definite-calc-item-2", 110, 5, [229, 72, 77, 255]),
    ]
    private static let c114GridGapCyclicCalcPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("gap-background", "grid-gap-cyclic-calc-grid", 45, 5, [16, 24, 32, 255]),
        ("row-gap-background", "grid-gap-cyclic-calc-grid", 5, 12, [16, 24, 32, 255]),
        ("inside-item", "grid-gap-cyclic-calc-item-1", 5, 5, [229, 72, 77, 255]),
        ("right-of-grid", "grid-gap-cyclic-calc-item-2", 111, 5, [229, 72, 77, 255]),
        ("below-grid", "grid-gap-cyclic-calc-item-4", 5, 28, [229, 72, 77, 255]),
    ]
    private static let c114GridGapPercentMinBasisPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("gap-background", "grid-gap-cyclic-percent-min-basis-grid", 45, 5, [16, 24, 32, 255]),
        ("row-gap-background", "grid-gap-cyclic-percent-min-basis-grid", 5, 12, [16, 24, 32, 255]),
        ("inside-item", "grid-gap-cyclic-percent-min-basis-item-1", 5, 5, [229, 72, 77, 255]),
        ("second-column", "grid-gap-cyclic-percent-min-basis-item-2", 60, 5, [229, 72, 77, 255]),
        ("second-row", "grid-gap-cyclic-percent-min-basis-item-3", 5, 20, [229, 72, 77, 255]),
    ]
    private static let c114GridGapCalcMinBasisPixelSamples: [(String, String, UInt32, UInt32, [UInt32])] = [
        ("gap-background", "grid-gap-cyclic-calc-min-basis-grid", 45, 5, [16, 24, 32, 255]),
        ("row-gap-background", "grid-gap-cyclic-calc-min-basis-grid", 5, 12, [16, 24, 32, 255]),
        ("inside-item", "grid-gap-cyclic-calc-min-basis-item-1", 5, 5, [229, 72, 77, 255]),
        ("second-column", "grid-gap-cyclic-calc-min-basis-item-2", 60, 5, [229, 72, 77, 255]),
        ("second-row", "grid-gap-cyclic-calc-min-basis-item-3", 5, 25, [229, 72, 77, 255]),
        ("below-grid", "grid-gap-cyclic-calc-min-basis-item-3", 5, 31, [229, 72, 77, 255]),
    ]

    private static func hasFixedC1131RepeatSamples(_ samples: [C111MetalPixelSample]) -> Bool {
        guard samples.count == c1131RepeatPixelSamples.count else { return false }
        return c1131RepeatPixelSamples.allSatisfy { expected in
            samples.contains { sample in
                sample.id == expected.0 && sample.nodeId == expected.1
                    && sample.xCssPx == expected.2 && sample.yCssPx == expected.3
                    && sample.rgba == expected.4
            }
        }
    }

    private static func hasFixedC1132NamedLineSamples(_ samples: [C111MetalPixelSample]) -> Bool {
        guard samples.count == c1132NamedLinePixelSamples.count else { return false }
        return c1132NamedLinePixelSamples.allSatisfy { expected in
            samples.contains { sample in
                sample.id == expected.0 && sample.nodeId == expected.1
                    && sample.xCssPx == expected.2 && sample.yCssPx == expected.3
                    && sample.rgba == expected.4
            }
        }
    }

    private static func hasFixedC1133TemplateAreaSamples(
        _ samples: [C111MetalPixelSample], sceneID: String
    ) -> Bool {
        let expectedSamples = sceneID == "basic-rectangles"
            ? c1133TemplateAreaPixelSamples
            : sceneID == "unknown-area-fixed-implicit-tracks"
                ? c1133UnknownAreaOverflowPixelSamples : []
        guard samples.count == expectedSamples.count else { return false }
        return expectedSamples.allSatisfy { expected in
            samples.contains { sample in
                sample.id == expected.0 && sample.nodeId == expected.1
                    && sample.xCssPx == expected.2 && sample.yCssPx == expected.3
                    && sample.rgba == expected.4
            }
        }
    }

    private static func hasFixedC114TrackSizingSamples(
        _ samples: [C111MetalPixelSample], sceneID: String
    ) -> Bool {
        let expectedSamples: [(String, String, UInt32, UInt32, [UInt32])]
        switch sceneID {
        case "cyclic-minmax-calc-min-column", "cyclic-minmax-calc-min-row",
             "definite-minmax-calc-min-column", "definite-minmax-calc-min-row":
            let rootID = "c114-\(sceneID)-grid"
            let itemID = "c114-\(sceneID)-item"
            let column = sceneID.hasSuffix("column")
            expectedSamples = [
                ("\(sceneID)-left-border", rootID, 1, 5, [240, 198, 116, 255]),
                ("\(sceneID)-item-inside", itemID, 5, 5, [229, 72, 77, 255]),
                ("\(sceneID)-\(column ? "right-border-overpaint" : "bottom-border-overpaint")",
                 itemID, column ? 13 : 5, column ? 5 : 13, [229, 72, 77, 255]),
                ("\(sceneID)-outside-root", itemID, column ? 16 : 5, column ? 5 : 16,
                 [229, 72, 77, 255]),
                ("\(sceneID)-past-item", rootID, column ? 27 : 5, column ? 5 : 25,
                 [0, 0, 0, 255]),
            ]
        case "cyclic-minmax-calc-min-both-axes",
             "cyclic-minmax-calc-min-column-definite-row",
             "definite-column-cyclic-minmax-calc-min-row",
             "definite-minmax-calc-min-both-axes":
            let rootID = "c114-\(sceneID)-grid"
            let itemID = "c114-\(sceneID)-item"
            switch sceneID {
            case "cyclic-minmax-calc-min-both-axes":
                expectedSamples = [
                    ("\(sceneID)-left-border", rootID, 1, 5, [240, 198, 116, 255]),
                    ("\(sceneID)-item-inside", itemID, 5, 5, [229, 72, 77, 255]),
                    ("\(sceneID)-right-border-overpaint", itemID, 53, 5,
                     [229, 72, 77, 255]),
                    ("\(sceneID)-outside-root", itemID, 60, 5, [229, 72, 77, 255]),
                    ("\(sceneID)-past-item-right", rootID, 106, 5, [0, 0, 0, 255]),
                    ("\(sceneID)-bottom-border-overpaint", itemID, 5, 13,
                     [229, 72, 77, 255]),
                    ("\(sceneID)-outside-root-bottom", itemID, 5, 16,
                     [229, 72, 77, 255]),
                    ("\(sceneID)-past-item-bottom", rootID, 5, 25, [0, 0, 0, 255]),
                ]
            case "cyclic-minmax-calc-min-column-definite-row":
                expectedSamples = [
                    ("\(sceneID)-left-border", rootID, 1, 5, [240, 198, 116, 255]),
                    ("\(sceneID)-item-inside", itemID, 5, 5, [229, 72, 77, 255]),
                    ("\(sceneID)-right-border-overpaint", itemID, 53, 5,
                     [229, 72, 77, 255]),
                    ("\(sceneID)-outside-root", itemID, 60, 5, [229, 72, 77, 255]),
                    ("\(sceneID)-last-item-column", itemID, 105, 5,
                     [229, 72, 77, 255]),
                    ("\(sceneID)-past-item-right", rootID, 106, 5, [0, 0, 0, 255]),
                ]
            case "definite-column-cyclic-minmax-calc-min-row":
                expectedSamples = [
                    ("\(sceneID)-left-border", rootID, 1, 5, [240, 198, 116, 255]),
                    ("\(sceneID)-item-inside", itemID, 5, 5, [229, 72, 77, 255]),
                    ("\(sceneID)-bottom-border-overpaint", itemID, 5, 23,
                     [229, 72, 77, 255]),
                    ("\(sceneID)-outside-root-bottom", itemID, 5, 25,
                     [229, 72, 77, 255]),
                    ("\(sceneID)-last-item-row", itemID, 5, 43, [229, 72, 77, 255]),
                    ("\(sceneID)-past-item-bottom", rootID, 5, 45, [0, 0, 0, 255]),
                ]
            default:
                expectedSamples = [
                    ("\(sceneID)-left-border", rootID, 1, 5, [240, 198, 116, 255]),
                    ("\(sceneID)-item-inside", itemID, 5, 5, [229, 72, 77, 255]),
                    ("\(sceneID)-right-border-overpaint", itemID, 13, 5,
                     [229, 72, 77, 255]),
                    ("\(sceneID)-outside-root", itemID, 16, 5, [229, 72, 77, 255]),
                    ("\(sceneID)-past-item-right", rootID, 27, 5, [0, 0, 0, 255]),
                    ("\(sceneID)-bottom-border-overpaint", itemID, 5, 13,
                     [229, 72, 77, 255]),
                    ("\(sceneID)-outside-root-bottom", itemID, 5, 16,
                     [229, 72, 77, 255]),
                    ("\(sceneID)-past-item-bottom", rootID, 5, 25, [0, 0, 0, 255]),
                ]
            }
        case "cyclic-calc-column", "cyclic-calc-row",
             "cyclic-calc-two-axis", "definite-calc-control":
            let itemID = "c114-\(sceneID)-item"
            switch sceneID {
            case "cyclic-calc-column":
                expectedSamples = [
                    ("cyclic-calc-column-inside-track", itemID, 15, 8, [229, 72, 77, 255]),
                    ("cyclic-calc-column-past-track", itemID, 16, 8, [229, 72, 77, 255]),
                ]
            case "cyclic-calc-row":
                expectedSamples = [
                    ("cyclic-calc-row-inside-track", itemID, 12, 9, [229, 72, 77, 255]),
                    ("cyclic-calc-row-past-track", itemID, 12, 10, [229, 72, 77, 255]),
                ]
            case "cyclic-calc-two-axis":
                expectedSamples = [
                    ("cyclic-calc-two-axis-inside-tracks", itemID, 15, 9, [229, 72, 77, 255]),
                    ("cyclic-calc-two-axis-past-both", itemID, 16, 10, [229, 72, 77, 255]),
                ]
            default:
                expectedSamples = [
                    ("definite-calc-control-inside-track", itemID, 23, 11, [229, 72, 77, 255]),
                    ("definite-calc-control-past-row", itemID, 5, 12, [229, 72, 77, 255]),
                ]
            }
        case "cyclic-percent-two-axis-multitrack",
             "cyclic-percent-columns-multitrack",
             "cyclic-percent-rows-multitrack",
             "definite-percent-two-axis-multitrack":
            let textID = "c114-\(sceneID)-text"
            let overflowID = "c114-\(sceneID)-overflow"
            expectedSamples = [
                ("\(sceneID)-text-red", textID, 10, 5, [229, 72, 77, 255]),
                ("\(sceneID)-overflow-inside-grid", overflowID, 35, 17,
                 [32, 191, 107, 255]),
                ("\(sceneID)-overflow-past-right", overflowID, 45, 18,
                 [32, 191, 107, 255]),
                ("\(sceneID)-overflow-past-bottom", overflowID, 35, 22,
                 [32, 191, 107, 255]),
                ("\(sceneID)-overflow-past-both", overflowID, 45, 22,
                 [32, 191, 107, 255]),
            ]
        case "cyclic-fit-content-column":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("cyclic-fit-content-column-inside-item", itemID, 24, 5, [229, 72, 77, 255]),
                ("cyclic-fit-content-column-past-item", itemID, 26, 5, [16, 24, 32, 255]),
            ]
        case "cyclic-fit-content-row":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("cyclic-fit-content-row-inside-item", itemID, 19, 19, [229, 72, 77, 255]),
                ("cyclic-fit-content-row-past-item-no-text", itemID, 19, 39, [16, 24, 32, 255]),
            ]
        case "definite-fit-content-column":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("definite-fit-content-column-inside-item", itemID, 19, 5, [229, 72, 77, 255]),
                ("definite-fit-content-column-past-item", itemID, 21, 5, [16, 24, 32, 255]),
            ]
        case "cyclic-fit-content-calc-column":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("cyclic-fit-content-calc-column-inside-item", itemID, 28, 5,
                 [229, 72, 77, 255]),
                ("cyclic-fit-content-calc-column-past-item", itemID, 30, 5,
                 [16, 24, 32, 255]),
            ]
        case "cyclic-fit-content-calc-row":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("cyclic-fit-content-calc-row-inside-item", itemID, 19, 21,
                 [229, 72, 77, 255]),
                ("cyclic-fit-content-calc-row-past-item", itemID, 19, 23,
                 [16, 24, 32, 255]),
            ]
        case "definite-fit-content-calc-column":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("definite-fit-content-calc-column-inside-item", itemID, 23, 5,
                 [229, 72, 77, 255]),
                ("definite-fit-content-calc-column-past-item", itemID, 25, 5,
                 [16, 24, 32, 255]),
            ]
        case "definite-fit-content-calc-small-limit":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("definite-fit-content-calc-small-limit-inside-item", itemID, 5, 5,
                 [229, 72, 77, 255]),
                ("definite-fit-content-calc-small-limit-text-overflow", itemID, 15, 5,
                 [229, 72, 77, 255]),
                ("definite-fit-content-calc-small-limit-past-text", itemID, 25, 5,
                 [0, 0, 0, 255]),
            ]
        case "cyclic-minmax-calc-column":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("cyclic-minmax-calc-column-inside-item", itemID, 28, 5,
                 [229, 72, 77, 255]),
                ("cyclic-minmax-calc-column-past-item", itemID, 30, 5,
                 [16, 24, 32, 255]),
            ]
        case "cyclic-minmax-calc-row":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("cyclic-minmax-calc-row-inside-item", itemID, 19, 21,
                 [229, 72, 77, 255]),
                ("cyclic-minmax-calc-row-past-item", itemID, 19, 23,
                 [16, 24, 32, 255]),
            ]
        case "definite-minmax-calc-column":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("definite-minmax-calc-column-inside-item", itemID, 23, 5,
                 [229, 72, 77, 255]),
                ("definite-minmax-calc-column-past-item", itemID, 25, 5,
                 [16, 24, 32, 255]),
            ]
        case "definite-minmax-calc-small-limit":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("definite-minmax-calc-small-limit-inside-item", itemID, 5, 5,
                 [229, 72, 77, 255]),
                ("definite-minmax-calc-small-limit-text-overflow", itemID, 15, 5,
                 [229, 72, 77, 255]),
                ("definite-minmax-calc-small-limit-past-text", itemID, 25, 5,
                 [0, 0, 0, 255]),
            ]
        case "cyclic-percent-column":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("cyclic-column-inside-track", itemID, 11, 8, [229, 72, 77, 255]),
                ("cyclic-column-past-track", itemID, 12, 8, [229, 72, 77, 255]),
            ]
        case "cyclic-minmax-column":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("cyclic-minmax-column-inside-track", itemID, 11, 8, [229, 72, 77, 255]),
                ("cyclic-minmax-column-past-track", itemID, 18, 8, [229, 72, 77, 255]),
            ]
        case "cyclic-percent-row":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("cyclic-row-inside-track", itemID, 12, 7, [229, 72, 77, 255]),
                ("cyclic-row-past-track", itemID, 12, 8, [229, 72, 77, 255]),
            ]
        case "cyclic-minmax-row":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("cyclic-minmax-row-inside-track", itemID, 12, 7, [229, 72, 77, 255]),
                ("cyclic-minmax-row-past-track", itemID, 12, 12, [229, 72, 77, 255]),
            ]
        case "definite-percent-column":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("definite-column-inside-track", itemID, 19, 8, [229, 72, 77, 255]),
                ("definite-column-past-track", itemID, 20, 8, [229, 72, 77, 255]),
            ]
        case "definite-minmax-column":
            let itemID = "c114-\(sceneID)-item"
            expectedSamples = [
                ("definite-minmax-column-inside-track", itemID, 19, 8, [229, 72, 77, 255]),
                ("definite-minmax-column-past-track", itemID, 22, 8, [229, 72, 77, 255]),
            ]
        case "nested-grid-definite-flex-control":
            let rootID = "c114-\(sceneID)-grid"
            let nestedID = "c114-\(sceneID)-nested"
            let itemBID = "c114-\(sceneID)-item-b"
            expectedSamples = [
                ("nested-grid-internal-gap", nestedID, 72, 10, [231, 246, 237, 255]),
                ("nested-grid-inside-outer-edge", itemBID, 119, 10, [229, 72, 77, 255]),
                ("nested-grid-outside-outer-edge", itemBID, 120, 10, [229, 72, 77, 255]),
                ("nested-grid-overflow-tail", itemBID, 133, 10, [229, 72, 77, 255]),
                ("nested-grid-after-overflow", rootID, 134, 10, [0, 0, 0, 255]),
            ]
        case "image-span2-forward", "image-span2-reversed",
             "image-span2-before-span3", "image-span3-before-span2":
            expectedSamples = c114ImageMultiSpanPixelSamples(sceneID)
        case "anonymous-runs-two-images-forward", "two-images-anonymous-runs-reversed",
             "two-anonymous-runs-three-images-forward", "three-images-two-anonymous-runs-reversed":
            let rootID = "c114-\(sceneID)-grid"
            let overflowSample = c114IsAnonymousGridBorderOverflowThreeImageScene(sceneID)
                ? ("image-c-overflow-green-png-texel", rootID, UInt32(62), UInt32(128))
                : ("image-overflow-png-quadrant", rootID, UInt32(51), UInt32(71))
            expectedSamples = [
                ("top-border-color", rootID, 10, 0, [240, 198, 116, 255]),
                (overflowSample.0, overflowSample.1, overflowSample.2, overflowSample.3,
                 [0, 255, 64, 255]),
            ]
        case "auto-fill-overflow-neighbors":
            expectedSamples = c114AutoRepeatOverflowPixelSamples
        case "implicit-min-content-overflow-border":
            expectedSamples = c114ImplicitTrackOverflowPixelSamples
        case "columns-fill-negative-positive-cycle",
            "columns-negative-fixed-root-overflow", "rows-negative-fixed-root-overflow":
            guard let negativeOverflowSamples = c114NegativeImplicitOverflowPixelSamples(
                sceneID: sceneID
            ) else { return false }
            expectedSamples = negativeOverflowSamples
        case "fixed-small-track-overflow":
            expectedSamples = c114ImageOverflowPixelSamples
        case "column-auto-natural-ratio":
            expectedSamples = c114ImageRatioTransferPixelSamples
        case "row-transfer-input-capped-by-fixed-column-span":
            expectedSamples = c114RowTransferOverflowPixelSamples
        case "column-transfer-input-capped-by-fixed-row-span":
            expectedSamples = c114ColumnTransferOverflowPixelSamples
        case "fixed-gutter-column-suggestion-clamp":
            expectedSamples = c114FixedGutterColumnPixelSamples
        case "fixed-gutter-row-suggestion-clamp":
            expectedSamples = c114FixedGutterRowPixelSamples
        case "grid-gap-definite-percent":
            expectedSamples = c114GridGapDefinitePercentPixelSamples
        case "grid-gap-cyclic-percent":
            expectedSamples = c114GridGapCyclicPercentPixelSamples
        case "grid-gap-definite-calc":
            expectedSamples = c114GridGapDefiniteCalcPixelSamples
        case "grid-gap-cyclic-calc":
            expectedSamples = c114GridGapCyclicCalcPixelSamples
        case "grid-gap-cyclic-percent-min-basis":
            expectedSamples = c114GridGapPercentMinBasisPixelSamples
        case "grid-gap-cyclic-calc-min-basis":
            expectedSamples = c114GridGapCalcMinBasisPixelSamples
        default:
            expectedSamples = c114TextOverflowPixelSamples
        }
        guard samples.count == expectedSamples.count else { return false }
        return expectedSamples.allSatisfy { expected in
            samples.contains { sample in
                sample.id == expected.0 && sample.nodeId == expected.1
                    && sample.xCssPx == expected.2 && sample.yCssPx == expected.3
                    && sample.rgba == expected.4
            }
        }
    }

    private static func c114ImageMultiSpanPixelSamples(
        _ sceneID: String
    ) -> [(String, String, UInt32, UInt32, [UInt32])] {
        let threeImages = sceneID == "image-span2-before-span3"
            || sceneID == "image-span3-before-span2"
        let rootID = "c114-\(sceneID)-grid"
        let overflowImageID = sceneID == "image-span2-forward" ? "image-b-forward"
            : sceneID == "image-span2-reversed" ? "image-b-reversed"
            : sceneID == "image-span3-before-span2" ? "image-r-c" : "image-c-c"
        var samples: [(String, String, UInt32, UInt32, [UInt32])] = [
            ("image-multispan-root-background", rootID, threeImages ? 35 : 30, 10,
             [16, 24, 32, 255]),
            ("image-multispan-outside-root", rootID, 40, 10, [0, 0, 0, 255]),
            ("image-multispan-item-overflow", overflowImageID,
             threeImages ? 65 : 45, threeImages ? 45 : 30, [229, 72, 77, 255]),
        ]
        if threeImages {
            samples.append(("image-multispan-item-below-root", overflowImageID,
                            10, 70, [229, 72, 77, 255]))
        }
        samples.append(("image-multispan-after-item", rootID,
                        threeImages ? 75 : 62, threeImages ? 70 : 30, [0, 0, 0, 255]))
        return samples
    }

    private static func c114NegativeImplicitOverflowPixelSamples(
        sceneID: String
    ) -> [(String, String, UInt32, UInt32, [UInt32])]? {
        let rootNodeID = "c114-\(sceneID)-grid"
        if sceneID == "columns-fill-negative-positive-cycle" {
            let overflowItemNodeID = "c114-\(sceneID)-item-2"
            return [
                ("negative-cycle-root-interior", rootNodeID, 100, 10, [226, 232, 240, 255]),
                ("negative-cycle-root-edge", rootNodeID, 199, 10, [226, 232, 240, 255]),
                ("negative-cycle-root-exterior", rootNodeID, 200, 10, [0, 0, 0, 255]),
                ("negative-cycle-overflow-item", overflowItemNodeID, 290, 10, [42, 157, 143, 255]),
                ("negative-cycle-overflow-tail", rootNodeID, 319, 10, [0, 0, 0, 255]),
            ]
        }
        let itemNodeID = "c114-\(sceneID)-item-5"
        if sceneID == "columns-negative-fixed-root-overflow" {
            return [
                ("negative-root-interior", rootNodeID, 33, 10, [226, 232, 240, 255]),
                ("negative-root-edge", rootNodeID, 0, 10, [226, 232, 240, 255]),
                ("negative-root-exterior", rootNodeID, 150, 10, [0, 0, 0, 255]),
                ("negative-overflow-item", itemNodeID, 250, 10, [141, 95, 211, 255]),
                ("negative-overflow-tail", rootNodeID, 273, 10, [0, 0, 0, 255]),
            ]
        }
        if sceneID == "rows-negative-fixed-root-overflow" {
            return [
                ("negative-root-interior", rootNodeID, 10, 19, [226, 232, 240, 255]),
                ("negative-root-edge", rootNodeID, 10, 0, [226, 232, 240, 255]),
                ("negative-root-exterior", rootNodeID, 10, 100, [0, 0, 0, 255]),
                ("negative-overflow-item", itemNodeID, 10, 195, [141, 95, 211, 255]),
                ("negative-overflow-tail", rootNodeID, 10, 209, [0, 0, 0, 255]),
            ]
        }
        return nil
    }

    static func parse(arguments: [String], sceneID: String?) -> C111MetalPixelRun? {
        guard let rawRun = arguments.first(where: {
            $0.hasPrefix("--spinon-c11-grid-metal-surface-pixels=")
        }), let rawSamples = arguments.first(where: {
            $0.hasPrefix("--spinon-c11-grid-metal-surface-samples=")
        }), let sceneID else { return nil }
        let runID = String(rawRun.dropFirst("--spinon-c11-grid-metal-surface-pixels=".count))
        let encoded = String(rawSamples.dropFirst("--spinon-c11-grid-metal-surface-samples=".count))
        let c1131Scene = arguments.first(where: {
            $0.hasPrefix("--spinon-c1131-grid-repeat-scene=")
        }).map { String($0.dropFirst("--spinon-c1131-grid-repeat-scene=".count)) }
        let isC1131Repeat = arguments.contains("--spinon-c1131-grid-repeat") || c1131Scene != nil
        if isC1131Repeat {
            let c1131RunID = arguments.first(where: {
                $0.hasPrefix("--spinon-c1131-grid-repeat-run-id=")
            }).map { String($0.dropFirst("--spinon-c1131-grid-repeat-run-id=".count)) }
            guard c1131Scene == "multi-track-repeat", c1131Scene == sceneID,
                  c1131RunID == runID,
                  !arguments.contains("--spinon-c1123-grid-auto-placement"),
                  !arguments.contains("--spinon-c1122-grid-auto-placement"),
                  !arguments.contains("--spinon-c1121-grid-placement"),
                  !arguments.contains(where: { $0.hasPrefix("--spinon-c1123-grid-scene=") }),
                  !arguments.contains(where: { $0.hasPrefix("--spinon-c1122-grid-scene=") }),
                  !arguments.contains(where: { $0.hasPrefix("--spinon-c1121-grid-scene=") }) else {
                return nil
            }
        }
        let isC1132NamedLine = arguments.contains("--spinon-c1132-grid-named-lines")
        if isC1132NamedLine {
            let c1132RunID = arguments.first(where: {
                $0.hasPrefix("--spinon-c1132-grid-run-id=")
            }).map { String($0.dropFirst("--spinon-c1132-grid-run-id=".count)) }
            guard sceneID == "positive-occurrence", c1132RunID == runID,
                  !isC1131Repeat,
                  !arguments.contains("--spinon-c1123-grid-auto-placement"),
                  !arguments.contains("--spinon-c1122-grid-auto-placement"),
                  !arguments.contains("--spinon-c1121-grid-placement") else {
                return nil
            }
        }
        let isC1133TemplateAreas = arguments.contains("--spinon-c1133-grid-template-areas")
        if isC1133TemplateAreas {
            let c1133RunID = arguments.first(where: {
                $0.hasPrefix("--spinon-c1133-grid-run-id=")
            }).map { String($0.dropFirst("--spinon-c1133-grid-run-id=".count)) }
            let c1133SceneID = arguments.first(where: {
                $0.hasPrefix("--spinon-c1133-grid-scene=")
            }).map { String($0.dropFirst("--spinon-c1133-grid-scene=".count)) }
                ?? "basic-rectangles"
            guard sceneID == c1133SceneID,
                  sceneID == "basic-rectangles" || sceneID == "unknown-area-fixed-implicit-tracks",
                  c1133GridTemplateAreasSceneIDs.contains(c1133SceneID), c1133RunID == runID,
                  !isC1131Repeat, !isC1132NamedLine,
                  !arguments.contains("--spinon-c1123-grid-auto-placement"),
                  !arguments.contains("--spinon-c1122-grid-auto-placement"),
                  !arguments.contains("--spinon-c1121-grid-placement") else {
                return nil
            }
        }
        let c114Scene = arguments.first(where: {
            $0.hasPrefix("--spinon-c114-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c114-grid-scene=".count)) }
        let isC114TrackSizing = arguments.contains("--spinon-c114-grid-track-sizing")
            || c114Scene != nil
        if isC114TrackSizing {
            let c114RunID = arguments.first(where: {
                $0.hasPrefix("--spinon-c114-grid-run-id=")
            }).map { String($0.dropFirst("--spinon-c114-grid-run-id=".count)) }
            guard let c114Scene,
                  c114GridTrackSizingSceneIDs.contains(c114Scene),
                  c114Scene == sceneID, c114RunID == runID,
                  arguments.contains("--spinon-c114-grid-track-sizing"),
                  !isC1131Repeat, !isC1132NamedLine, !isC1133TemplateAreas,
                  !arguments.contains("--spinon-c1123-grid-auto-placement"),
                  !arguments.contains("--spinon-c1122-grid-auto-placement"),
                  !arguments.contains("--spinon-c1121-grid-placement"),
                  !arguments.contains(where: { $0.hasPrefix("--spinon-c1131-grid-repeat-scene=") }),
                  !arguments.contains(where: { $0.hasPrefix("--spinon-c1132-grid-scene=") }),
                  !arguments.contains(where: { $0.hasPrefix("--spinon-c1133-grid-scene=") }),
                  !arguments.contains(where: { $0.hasPrefix("--spinon-c11-grid-case=") }) else {
                return nil
            }
        }
        let c1123Scene = arguments.first(where: {
            $0.hasPrefix("--spinon-c1123-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1123-grid-scene=".count)) }
        let isC1123Placement = arguments.contains("--spinon-c1123-grid-auto-placement")
            || c1123Scene != nil
        if isC1123Placement {
            let c1123RunID = arguments.first(where: {
                $0.hasPrefix("--spinon-c1123-grid-run-id=")
            }).map { String($0.dropFirst("--spinon-c1123-grid-run-id=".count)) }
            guard let c1123Scene, c1123PixelSceneIDs.contains(c1123Scene),
                  c1123Scene == sceneID, c1123RunID == runID else { return nil }
        }
        let c1122Scene = arguments.first(where: {
            $0.hasPrefix("--spinon-c1122-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1122-grid-scene=".count)) }
        let isC1122Placement = arguments.contains("--spinon-c1122-grid-auto-placement")
            || c1122Scene != nil
        if isC1123Placement && isC1122Placement { return nil }
        if isC1122Placement {
            guard let c1122Scene, c1122PixelSceneIDs.contains(c1122Scene),
                  c1122Scene == sceneID else { return nil }
        }
        let c1121Scene = arguments.first(where: {
            $0.hasPrefix("--spinon-c1121-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1121-grid-scene=".count)) }
        let isC1121Placement = arguments.contains("--spinon-c1121-grid-placement") || c1121Scene != nil
        if isC1121Placement && !isC1122Placement && !isC1123Placement {
            guard let c1121Scene, c1121PixelSceneIDs.contains(c1121Scene),
                  c1121Scene == sceneID else { return nil }
        }
        guard (8...128).contains(runID.count),
              runID.allSatisfy({ $0.isASCII && ($0.isLetter || $0.isNumber || $0 == "-" || $0 == "_") }),
              (1...16_384).contains(encoded.count),
              let sampleBytes = Data(base64Encoded: encoded),
              let samples = try? JSONDecoder().decode([C111MetalPixelSample].self, from: sampleBytes),
              (1...64).contains(samples.count),
              Set(samples.map(\.id)).count == samples.count,
              Set(samples.map { "\($0.xCssPx),\($0.yCssPx)" }).count == samples.count,
              samples.allSatisfy({ sample in
                  sample.id.range(of: "^[a-z0-9-]{1,80}$", options: .regularExpression) != nil
                      && sample.nodeId.range(of: "^[a-zA-Z0-9_-]{1,80}$", options: .regularExpression) != nil
                      && sample.xCssPx < 320 && sample.yCssPx < 240
                      && sample.rgba.count == 4 && sample.rgba.allSatisfy({ $0 <= 255 })
              }) else { return nil }
        if isC1131Repeat && !hasFixedC1131RepeatSamples(samples) { return nil }
        if isC1132NamedLine && !hasFixedC1132NamedLineSamples(samples) { return nil }
        if isC1133TemplateAreas
            && !hasFixedC1133TemplateAreaSamples(samples, sceneID: sceneID) { return nil }
        if isC114TrackSizing && !hasFixedC114TrackSizingSamples(samples, sceneID: sceneID) {
            return nil
        }
        return C111MetalPixelRun(runID: runID, sceneID: sceneID,
                                 markerPrefix: isC114TrackSizing ? "C114"
                                     : isC1133TemplateAreas ? "C1133"
                                     : isC1132NamedLine ? "C1132"
                                     : isC1131Repeat ? "C1131"
                                     : isC1123Placement ? "C1123"
                                     : isC1122Placement ? "C1122" : "C111",
                                 samples: samples)
    }
}

private func c11GridMetalPixelSceneID(arguments: [String]) -> String? {
    if arguments.contains("--spinon-c114-grid-track-sizing") {
        return arguments.first(where: {
            $0.hasPrefix("--spinon-c114-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c114-grid-scene=".count)) }
    }
    if arguments.contains("--spinon-c1133-grid-template-areas") {
        return arguments.first(where: {
            $0.hasPrefix("--spinon-c1133-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1133-grid-scene=".count)) }
            ?? "basic-rectangles"
    }
    if arguments.contains("--spinon-c1132-grid-named-lines") {
        return "positive-occurrence"
    }
    let prefixes = [
        "--spinon-c1131-grid-repeat-scene=",
        "--spinon-c1123-grid-scene=",
        "--spinon-c1122-grid-scene=",
        "--spinon-c11-grid-case=",
    ]
    for prefix in prefixes {
        if let argument = arguments.first(where: { $0.hasPrefix(prefix) }) {
            return String(argument.dropFirst(prefix.count))
        }
    }
    return nil
}

final class C0410RuntimeGpuDemoViewController: UIViewController {
    var defaultRootHtmlApplicationRequested = false
    private var rootHtmlApplicationScript: String?
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
    private struct C18MetalPixelSample {
        let id: String
        let xCssPx: Float
        let yCssPx: Float
        let rgba: [UInt32]
    }
#endif
#if SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST_1
    private let c181MetalPixelSamples = [
        C18MetalPixelSample(id: "visible-content-positive", xCssPx: 9, yCssPx: 9,
                             rgba: [255, 255, 64, 255]),
        C18MetalPixelSample(id: "visible-border-overpaint", xCssPx: 25, yCssPx: 10,
                             rgba: [0, 255, 64, 255]),
        C18MetalPixelSample(id: "visible-overflow-first", xCssPx: 26, yCssPx: 10,
                             rgba: [0, 255, 64, 255]),
        C18MetalPixelSample(id: "visible-overflow-middle", xCssPx: 29, yCssPx: 10,
                             rgba: [0, 255, 64, 255]),
        C18MetalPixelSample(id: "visible-overflow-last", xCssPx: 32, yCssPx: 10,
                             rgba: [0, 255, 64, 255]),
        C18MetalPixelSample(id: "visible-after-image", xCssPx: 33, yCssPx: 10,
                             rgba: [16, 24, 39, 255]),
        C18MetalPixelSample(id: "hidden-padding-positive", xCssPx: 60, yCssPx: 10,
                             rgba: [0, 255, 64, 255]),
        C18MetalPixelSample(id: "hidden-border-positive", xCssPx: 61, yCssPx: 10,
                             rgba: [240, 198, 116, 255]),
        C18MetalPixelSample(id: "hidden-outside", xCssPx: 62, yCssPx: 10,
                             rgba: [16, 24, 39, 255]),
    ]
#endif
#if SPINON_ENABLE_C18_2_ALPHA_IMAGE_FIXTURE_TEST_1
    private let c182MetalPixelSamples = [
        C18MetalPixelSample(id: "background-anchor", xCssPx: 2, yCssPx: 2,
                             rgba: [17, 34, 51, 255]),
        C18MetalPixelSample(id: "transparent-interior", xCssPx: 12, yCssPx: 20,
                             rgba: [17, 34, 51, 255]),
        C18MetalPixelSample(id: "transparent-right-edge", xCssPx: 15, yCssPx: 20,
                             rgba: [17, 34, 51, 255]),
        C18MetalPixelSample(id: "partial-left-edge", xCssPx: 16, yCssPx: 20,
                             rgba: [118, 37, 65, 255]),
        C18MetalPixelSample(id: "partial-interior", xCssPx: 20, yCssPx: 20,
                             rgba: [118, 37, 65, 255]),
        C18MetalPixelSample(id: "partial-right-edge", xCssPx: 23, yCssPx: 20,
                             rgba: [118, 37, 65, 255]),
        C18MetalPixelSample(id: "opaque-left-edge", xCssPx: 24, yCssPx: 20,
                             rgba: [32, 200, 64, 255]),
        C18MetalPixelSample(id: "opaque-interior", xCssPx: 28, yCssPx: 20,
                             rgba: [32, 200, 64, 255]),
        C18MetalPixelSample(id: "opaque-right-edge", xCssPx: 31, yCssPx: 20,
                             rgba: [32, 200, 64, 255]),
        C18MetalPixelSample(id: "after-image-anchor", xCssPx: 34, yCssPx: 20,
                             rgba: [17, 34, 51, 255]),
    ]
#endif
#if SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST_1
    private let c183MetalPixelSamples = [
        C18MetalPixelSample(id: "root-background-anchor", xCssPx: 2, yCssPx: 2,
                             rgba: [17, 34, 51, 255]),
        C18MetalPixelSample(id: "fill-top-left", xCssPx: 9, yCssPx: 9,
                             rgba: [24, 32, 232, 255]),
        C18MetalPixelSample(id: "fill-bottom-right", xCssPx: 30, yCssPx: 30,
                             rgba: [209, 214, 51, 255]),
        C18MetalPixelSample(id: "contain-background-top", xCssPx: 42, yCssPx: 9,
                             rgba: [210, 10, 180, 255]),
        C18MetalPixelSample(id: "contain-image-top", xCssPx: 42, yCssPx: 15,
                             rgba: [24, 32, 232, 255]),
        C18MetalPixelSample(id: "contain-image-bottom", xCssPx: 42, yCssPx: 25,
                             rgba: [24, 214, 206, 255]),
        C18MetalPixelSample(id: "contain-quarter-image", xCssPx: 75, yCssPx: 18,
                             rgba: [24, 32, 232, 255]),
        C18MetalPixelSample(id: "cover-center-left", xCssPx: 105, yCssPx: 12,
                             rgba: [61, 32, 201, 255]),
        C18MetalPixelSample(id: "cover-center-right", xCssPx: 126, yCssPx: 12,
                             rgba: [172, 32, 108, 255]),
        C18MetalPixelSample(id: "cover-start-left", xCssPx: 137, yCssPx: 12,
                             rgba: [24, 32, 232, 255]),
        C18MetalPixelSample(id: "cover-end-left", xCssPx: 169, yCssPx: 12,
                             rgba: [135, 32, 139, 255]),
        C18MetalPixelSample(id: "none-clipped-object", xCssPx: 10, yCssPx: 45,
                             rgba: [98, 32, 170, 255]),
        C18MetalPixelSample(id: "none-outside-content-box", xCssPx: 25, yCssPx: 45,
                             rgba: [17, 34, 51, 255]),
        C18MetalPixelSample(id: "scale-down-shrink-background", xCssPx: 42, yCssPx: 41,
                             rgba: [210, 10, 180, 255]),
        // Chrome 155 at the iPhone 17 Pro Simulator's drawable scale factor 3.
        C18MetalPixelSample(id: "scale-down-shrink-image", xCssPx: 45, yCssPx: 46,
                             rgba: [89, 55, 174, 255]),
        C18MetalPixelSample(id: "scale-down-no-upscale-image", xCssPx: 82, yCssPx: 47,
                             rgba: [61, 32, 201, 255]),
        C18MetalPixelSample(id: "initial-values-natural", xCssPx: 113, yCssPx: 41,
                             rgba: [24, 32, 232, 255]),
        C18MetalPixelSample(id: "contain-extended-position-image", xCssPx: 145, yCssPx: 60,
                             rgba: [24, 32, 232, 255]),
        C18MetalPixelSample(id: "cover-extended-position-image", xCssPx: 177, yCssPx: 44,
                             rgba: [172, 32, 108, 255]),
    ]
#endif
#if SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST_1
    private let c184MetalPixelSamples = [
        C18MetalPixelSample(id: "root-background", xCssPx: 2, yCssPx: 2,
                             rgba: [17, 34, 51, 255]),
        C18MetalPixelSample(id: "current-b", xCssPx: 9, yCssPx: 9,
                             rgba: [24, 64, 240, 255]),
        C18MetalPixelSample(id: "pending-c-background", xCssPx: 65, yCssPx: 9,
                             rgba: [210, 10, 180, 255]),
        C18MetalPixelSample(id: "c-success", xCssPx: 121, yCssPx: 9,
                             rgba: [32, 220, 80, 255]),
        C18MetalPixelSample(id: "decode-failure-background", xCssPx: 177, yCssPx: 9,
                             rgba: [210, 10, 180, 255]),
        C18MetalPixelSample(id: "recovery-d", xCssPx: 9, yCssPx: 49,
                             rgba: [200, 40, 220, 255]),
    ]
#endif
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
    private struct C133RuntimeEnvironment: Equatable {
        let width: Float
        let height: Float
        let scale: Float
        let dark: Bool
    }
#endif
    private let runtimeQueue = DispatchQueue(label: "spinon.c0410.runtime")
    private let renderQueue = DispatchQueue(label: "spinon.c0410.render")
    private let stateLock = NSLock()
    private let presentationUpdateLock = NSLock()
    private let hostLifetime = C0410RuntimeGpuHostLifetime()
    private let canvasView = C0410RuntimeGpuCanvasView()
    private let resizeButton = UIButton(type: .system)
    private let customPropertiesButton = UIButton(type: .system)
    private let statusLabel = UILabel()
    private var canvasWidthConstraint: NSLayoutConstraint?
    private var canvasHeightConstraint: NSLayoutConstraint?
    private var hostHandle: UInt64 = 0
    private var rendererReady = false
    private var rendererCreationPending = false
    private var surfaceConfigurationPending = false
    private var surfaceGeneration: UInt64 = 0
    private var rendererSurfaceGeneration: UInt64?
    private var rendererSurfaceFormat: String?
    private var lastDrawableSize = CGSize.zero
    private var c111MetalDrawableSize = CGSize.zero
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
    private var imagePaintScreenGeometryLogged = false
#endif
    private var viewportWidthCssPx: Float = 301
    private var viewportHeightCssPx: Float = 100
    private var presentationSequence: UInt64 = 0
    private var runtimeInitializationComplete = false
    private var runtimeInitializationEnqueued = false
    private var expandedSurface = false
    private var automaticResizeScheduled = false
    private let failureProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c0410-failure-probe")
    private let shutdownProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c0410-shutdown-probe")
    private let runtimeResultCacheProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c053-runtime-result-cache")
    private let incrementalRestyleProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c054-incremental-restyle")
    private let percentageDimensionsProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c061-percentage-dimensions")
    private let spacingPercentagesProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c062-spacing-percentages")
    private let absoluteLengthsProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c063-absolute-lengths")
    private let fontRelativeUnitsProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c064-font-relative-units")
    private let typedCssMathProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c065-typed-css-math")
    private let viewportUnitsProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c066-viewport-units")
    private let minMaxSizingProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c071-min-max-sizing")
    private let borderWidthProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c072-border-width")
    private let aspectRatioProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c073-aspect-ratio")
    private let blockPaintProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c08-block-paint")
    private let rootHtmlBlockPaintProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c0412-root-html-block-paint")
    private let rootHtmlTextPaintProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c0412-root-html-text-paint")
    private let rootHtmlDefaultRenderRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c0412-root-html-default-render")
    private var rootHtmlPageScaleArguments: [String] {
        ProcessInfo.processInfo.arguments.filter {
            $0.hasPrefix("--spinon-c0412-page-scale-case=")
        }
    }
    private var rootHtmlPageScaleCase: String? {
        guard let argument = rootHtmlPageScaleArguments.first else { return nil }
        return String(argument.dropFirst("--spinon-c0412-page-scale-case=".count))
    }
    private var rootHtmlTextPaintReadbackFinished = false
    private let marginCollapseProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c092-margin-collapse")
    private let flowRootProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c093-flow-root")
    private let flexWrapProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c101-flex-wrap")
    private let flexDistributionProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c102-flex-distribution")
    private let flexReverseProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1031-flex-reverse")
    private let flexOrderProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1032-flex-order")
    private let flexAlignmentProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1033-flex-alignment")
    private let flexBaselineProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1034-flex-baseline")
    private let c121PositioningProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c121-static-relative")
    private let c1121GridPlacementProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1121-grid-placement")
        || ProcessInfo.processInfo.arguments.contains(where: {
            $0.hasPrefix("--spinon-c1121-grid-scene=")
        })
    private var c1121GridSceneID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1121-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1121-grid-scene=".count)) } ?? "numeric-lines"
    }
    private var c1121GridRunID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1121-grid-run-id=")
        }).map { String($0.dropFirst("--spinon-c1121-grid-run-id=".count)) } ?? "manual"
    }
    private let c1122GridAutoPlacementSceneIDs: Set<String> = [
        "definite-first-mixed", "auto-only-fill", "positive-implicit-row",
        "row-locked-auto-column", "column-locked-auto-row", "auto-column-span",
        "sparse-hole-not-backfilled", "implicit-track-size-and-gap",
        "hidden-child-excluded", "nested-independent-placement", "reject-implicit-column",
        "reject-auto-sized-implicit-row", "reject-repeated-auto-row-pattern",
        "reject-dense-flow", "reject-column-flow", "column-cursor-wrap",
    ]
    private let c1122GridAutoPlacementProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1122-grid-auto-placement")
        || ProcessInfo.processInfo.arguments.contains(where: {
            $0.hasPrefix("--spinon-c1122-grid-scene=")
        })
    private var c1122GridSceneID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1122-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1122-grid-scene=".count)) } ?? ""
    }
    private var c1122GridRunID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1122-grid-run-id=")
        }).map { String($0.dropFirst("--spinon-c1122-grid-run-id=".count)) } ?? "manual"
    }
    private let c1123GridAutoPlacementSceneIDs: Set<String> = [
        "row-sparse-baseline", "row-dense-hole-fill", "row-dense-span-collision",
        "row-sparse-hole-control", "row-dense-fixed-row", "row-dense-fixed-column",
        "row-dense-positive-implicit-row", "column-sparse-explicit-fill",
        "column-sparse-positive-implicit-column", "column-sparse-fixed-row",
        "column-sparse-fixed-column", "column-dense-hole-fill", "column-sparse-hole-control",
        "column-dense-span-collision", "column-dense-positive-implicit-column",
        "definite-first-dense", "hidden-item-excluded", "nested-mixed-flow",
        "auto-track-no-implicit", "reject-repeated-auto-track", "reject-negative-implicit-line",
        "reject-nonzero-order", "reject-indefinite-axis", "auto-span-at-track-limit",
        "auto-span-over-track-limit",
    ]
    private let c1123GridAutoPlacementProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1123-grid-auto-placement")
        || ProcessInfo.processInfo.arguments.contains(where: {
            $0.hasPrefix("--spinon-c1123-grid-scene=")
        })
    private var c1123GridSceneID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1123-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1123-grid-scene=".count)) } ?? ""
    }
    private var c1123GridRunID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1123-grid-run-id=")
        }).map { String($0.dropFirst("--spinon-c1123-grid-run-id=".count)) } ?? "manual"
    }
    private let c1131GridRepeatSceneID = "multi-track-repeat"
    private let c1131GridRepeatProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1131-grid-repeat")
        || ProcessInfo.processInfo.arguments.contains(where: {
            $0.hasPrefix("--spinon-c1131-grid-repeat-scene=")
        })
    private var c1131GridRepeatSceneArgument: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1131-grid-repeat-scene=")
        }).map { String($0.dropFirst("--spinon-c1131-grid-repeat-scene=".count)) }
            ?? c1131GridRepeatSceneID
    }
    private var c1131GridRepeatRunID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1131-grid-repeat-run-id=")
        }).map { String($0.dropFirst("--spinon-c1131-grid-repeat-run-id=".count)) } ?? "manual"
    }
    private let c1132GridNamedLineSceneID = "positive-occurrence"
    private let c1132GridNamedLineRejectSceneID = "missing-name-implicit-line"
    private let c1132GridNamedLinesProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1132-grid-named-lines")
    private var c1132GridNamedLinesRunID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1132-grid-run-id=")
        }).map { String($0.dropFirst("--spinon-c1132-grid-run-id=".count)) } ?? "manual"
    }
    private var c1133GridTemplateAreasSceneID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1133-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c1133-grid-scene=".count)) }
            ?? "basic-rectangles"
    }
    private let c1133GridTemplateAreasRejectSceneID = "area-track-count-mismatch"
    private let c1133GridTemplateAreasProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1133-grid-template-areas")
    private var c1133GridTemplateAreasRunID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c1133-grid-run-id=")
        }).map { String($0.dropFirst("--spinon-c1133-grid-run-id=".count)) } ?? "manual"
    }
    private var c114GridTrackSizingSceneID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c114-grid-scene=")
        }).map { String($0.dropFirst("--spinon-c114-grid-scene=".count)) }
            ?? "auto-fixed-width-item"
    }
    private let c114GridTrackSizingProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c114-grid-track-sizing")
    private var c114GridTrackSizingRunID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c114-grid-run-id=")
        }).map { String($0.dropFirst("--spinon-c114-grid-run-id=".count)) } ?? "manual"
    }
    private var c11GridMetalPixelStatusName: String {
        c114GridTrackSizingProbeRequested ? "C11.4"
            : c1133GridTemplateAreasProbeRequested ? "C11.3.3"
            : c1132GridNamedLinesProbeRequested ? "C11.3.2"
            : c1131GridRepeatProbeRequested ? "C11.3.1"
            : c1123GridAutoPlacementProbeRequested ? "C11.2.3"
            : c1122GridAutoPlacementProbeRequested ? "C11.2.2"
            : c1121GridPlacementProbeRequested ? "C11.2.1" : "C11.1"
    }
    private let c11GridSceneIDs = [
        "fixed-tracks-gap", "percentage-tracks", "fractional-tracks-gap",
        "mixed-fixed-percent-fr", "row-auto-placement", "empty-grid",
        "single-item-grid", "grid-under-block", "grid-under-flex", "flex-under-grid",
    ]
    private var c11GridSceneArgument: String? {
        ProcessInfo.processInfo.arguments.first { $0.hasPrefix("--spinon-c11-grid-case=") }
            .map { String($0.dropFirst("--spinon-c11-grid-case=".count)) }
    }
    private var c11GridSceneIndex: Int? {
        if ProcessInfo.processInfo.arguments.contains("--spinon-c11-grid") { return 0 }
        guard let sceneId = c11GridSceneArgument else { return nil }
        return c11GridSceneIDs.firstIndex(of: sceneId)
    }
    private let c111MetalPixelRun = C111MetalPixelRun.parse(
        arguments: ProcessInfo.processInfo.arguments,
        sceneID: c11GridMetalPixelSceneID(arguments: ProcessInfo.processInfo.arguments)
    )
    private var c111MetalPixelArgumentRequested: Bool {
        ProcessInfo.processInfo.arguments.contains(where: {
            $0.hasPrefix("--spinon-c11-grid-metal-surface-pixels=")
        })
    }
    private var c111MetalPixelArgumentRunID: String {
        ProcessInfo.processInfo.arguments.first(where: {
            $0.hasPrefix("--spinon-c11-grid-metal-surface-pixels=")
        }).map { String($0.dropFirst("--spinon-c11-grid-metal-surface-pixels=".count)) }
            ?? "missing"
    }
    private var c111MetalPixelRequestIssued = false
    private var c111MetalPixelReadbackFinished = false
    private var c11GridFixtureRequested: Bool {
        c1121GridPlacementProbeRequested || c1122GridAutoPlacementProbeRequested
            || c1123GridAutoPlacementProbeRequested || c1131GridRepeatProbeRequested
            || c1132GridNamedLinesProbeRequested || c1133GridTemplateAreasProbeRequested
            || c114GridTrackSizingProbeRequested
            || ProcessInfo.processInfo.arguments.contains("--spinon-c11-grid")
            || c11GridSceneArgument != nil || c111MetalPixelRun != nil
    }
    private let c131CompactSurfaceMatrixProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c131-compact-surface-matrix")
    private let c131CompactHitIdentityGridProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c131-compact-hit-identity-grid")
    private let c131CompactMetalPixelCentersRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c131-compact-metal-pixel-centers")
    private let c131FullPhysicalMetalPixelsRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c131-full-physical-metal-pixels")
    private let c131StackingDescendantClipProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c131-stacking-descendant-clip")
    private let c131StackingDescendantClipStaleRevisionProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c131-stacking-descendant-clip-stale-revision")
    private static let c131StackingClipRoles = [
        "parent-only", "target-inside", "clip-edge-inside", "clip-edge-outside",
        "escape-before-overlay", "stacking-overlay",
    ]
    private static let c131StackingClipCases = ["visible", "clip", "hidden"]
    private static let c131StackingClipPixelCoordinates: [(Int, Int)] = [
        (44, 52), (122, 52), (139, 52), (140, 52), (144, 52), (156, 52),
        (214, 52), (292, 52), (309, 52), (310, 52), (314, 52), (326, 52),
        (384, 52), (462, 52), (479, 52), (480, 52), (484, 52), (496, 52),
    ]
    private static let c131StackingClipExpectedHitIDs = [
        "visible-parent", "visible-target", "visible-target", "visible-target",
        "visible-target", "visible-overlay", "clip-parent", "clip-target",
        "clip-target", "root", "root", "clip-overlay", "hidden-parent",
        "hidden-target", "hidden-target", "root", "root", "hidden-overlay",
    ]
    private static let c131StackingClipExpectedNodeIndices = [
        1, 3, 3, 3, 3, 4, 5, 7, 7, 0, 0, 8, 9, 11, 11, 0, 0, 12,
    ]
    private static let c131StackingClipExpectedRGBA: [[Int]] = [
        [240, 198, 116, 255], [87, 166, 217, 255], [87, 166, 217, 255],
        [87, 166, 217, 255], [87, 166, 217, 255], [228, 81, 79, 255],
        [240, 198, 116, 255], [87, 166, 217, 255], [87, 166, 217, 255],
        [240, 244, 247, 255], [240, 244, 247, 255], [228, 81, 79, 255],
        [240, 198, 116, 255], [87, 166, 217, 255], [87, 166, 217, 255],
        [240, 244, 247, 255], [240, 244, 247, 255], [228, 81, 79, 255],
    ]
    private var c131MetalPixelCaptureRequested: Bool {
        c131CompactMetalPixelCentersRequested || c131FullPhysicalMetalPixelsRequested
            || c131StackingDescendantClipProbeRequested
    }
    private var c131MetalCaptureRunID: String {
        c131StackingDescendantClipProbeRequested
            ? c131StackingDescendantClipRunID : c131CompactSurfaceMatrixRunID
    }
    private var c131CompactSurfaceMatrixRouteRequested: Bool {
        c131CompactSurfaceMatrixProbeRequested || c131CompactHitIdentityGridProbeRequested
    }
    private let c131CompactSurfaceMatrixRunID = UUID().uuidString
        .replacingOccurrences(of: "-", with: "").lowercased()
    private let c131StackingDescendantClipRunID = UUID().uuidString
        .replacingOccurrences(of: "-", with: "").lowercased()
    private var c131CompactHitIdentityGridStatusMessage: String?
    private var c131MetalPixelCapture: (any C131MetalSurfacePixelCaptureProtocol)?
    private var c131StackingClipSampleCoordinates: [(Int, Int)] = []
    private var c131StackingClipExpectedRGBA: [[Int]] = []
    private var c131StackingClipExpectedRevisionFields: [String: String] = [:]
    private var c131StackingClipStaleRevisionInjectionApplied = false
    private var c131MetalCaptureBaselineDrawReady = false
    private var c131MetalPixelBatchRequested = false
    private var c131MetalPixelCaptureComplete = false
    private var c131MetalPixelCaptureFailed = false
    private let c131OverflowClipProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c131-overflow-clip")
    private let c131BorderStripProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c131-overflow-clip-border")
    private let c153GlyphPaintArgumentRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c153-glyph-paint")
    private let c153HostDocumentTextPaintArgumentRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c153-hostdocument-text-paint")
    private func verifyC0412RootHtmlTextPaintReadback(
        result: String,
        generation: UInt64
    ) throws {
        let expectedSamples: [([UInt32], [Int])] = [
            ([1, 1], [9, 33, 112, 255]),
            ([16, 1], [9, 33, 112, 255]),
            ([1, 11], [9, 33, 112, 255]),
            ([16, 11], [247, 249, 252, 255]),
            ([100, 100], [247, 249, 252, 255]),
        ]
        func fail(_ reason: String) -> C0410LatestTaskLaneError {
            rootHtmlTextPaintReadbackFinished = true
            log("SPINON_C0412_TEXT_METAL_PIXEL status=error generation=\(generation) reason=\(reason) report=\(result)")
            postStatus("실패 · C04.12 iOS glyph Metal 표면 표본 \(reason)")
            return .operationFailed("C04.12 iOS Metal text readback \(reason)")
        }
        guard result.contains("runtime_text_count=2"),
              result.contains("glyph_batch_draw_count=2") else {
            throw fail("sample-report-incomplete")
        }
        let coordinatePairs = reportField(result, "root_html_text_sample_surface_xy=")
            .split(separator: ";")
        guard coordinatePairs.count == expectedSamples.count else {
            throw fail("sample-coordinates-missing")
        }
        var encodedCoordinates: [[UInt32]] = []
        for pair in coordinatePairs {
            let values = pair.split(separator: ",")
            guard values.count == 2,
                  let x = UInt32(values[0]), let y = UInt32(values[1]) else {
                throw fail("sample-coordinates-invalid")
            }
            encodedCoordinates.append([x, y])
        }
        guard let encodedSamples = reportField(result, "surface_pixel_samples_json=")
                .data(using: .utf8),
              let samples = try? JSONSerialization.jsonObject(with: encodedSamples)
                as? [[String: Any]], samples.count == expectedSamples.count else {
            throw fail("sample-pixels-missing")
        }
        stateLock.lock()
        let surface = c111MetalDrawableSize
        stateLock.unlock()
        guard isCurrentSurfaceGeneration(generation), surface.width > 0, surface.height > 0 else {
            throw fail("surface-generation-or-size")
        }
        var matched = 0
        for index in expectedSamples.indices {
            let sample = samples[index]
            guard let x = sample["x"] as? NSNumber,
                  let y = sample["y"] as? NSNumber,
                  let rgba = sample["rgba"] as? [NSNumber], rgba.count == 4,
                  index < encodedCoordinates.count else {
                throw fail("sample-value-invalid-\(index)")
            }
            let expectedPoint = encodedCoordinates[index]
            let actualPoint = [x.uint32Value, y.uint32Value]
            let coordinateMatch = actualPoint == expectedPoint
            let color = rgba.map(\.intValue)
            let colorMatch = color == expectedSamples[index].1
            if coordinateMatch && colorMatch { matched += 1 }
            log("SPINON_C0412_TEXT_METAL_PIXEL_SAMPLE generation=\(generation) index=\(index) xy=\(actualPoint[0]),\(actualPoint[1]) rgba=\(color) expected_xy=\(expectedPoint) expected_rgba=\(expectedSamples[index].1) coordinate_match=\(coordinateMatch) expected_match=\(colorMatch)")
        }
        rootHtmlTextPaintReadbackFinished = true
        guard matched == expectedSamples.count else { throw fail("pixel-mismatch-\(matched)-of-\(expectedSamples.count)") }
        log("SPINON_C0412_TEXT_METAL_PIXEL_SUMMARY status=matched generation=\(generation) samples=\(matched)/\(expectedSamples.count) drawable=\(surface.width)x\(surface.height) renderer=metal-surface")
        postStatus("완료 · C04.12 iOS glyph Metal surface pixel · 5/5")
    }

#if SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST_1
    private var c153GlyphPaintProbeRequested: Bool {
        c153GlyphPaintArgumentRequested || c153HostDocumentTextPaintArgumentRequested
    }
    private var c153GlyphPaintEvaluationReport = ""
    private var c153MetalPixelRequestIssued = false
    private var c153MetalPixelReadbackFinished = false
    private var c153MetalPixelRequestGeneration: UInt64?
    private var c153MetalPixelRequestDrawableSize = CGSize.zero
    private var c153MetalPixelRequestFormat = ""
    private var c153MetalSamplePointsCss: [(String, UInt32, UInt32)] = []
#else
    private let c153GlyphPaintProbeRequested = false
#endif
    private let c132ScrollFixtureRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c132-scroll")
    private let c192BoxPaintOpacityArgumentRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c192-box-paint-opacity")
#if SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST_1
    private var c192BoxPaintOpacityProbeRequested: Bool {
        c192BoxPaintOpacityArgumentRequested
    }
    private let c192BoxPaintOpacityRunID = UUID().uuidString
        .replacingOccurrences(of: "-", with: "").lowercased()
    private var c192MetalPixelCapture: C192MetalSurfacePixelCapture?
    private var c192MetalPixelRequestIssued = false
    private var c192MetalPixelReadbackFinished = false
    private var c192MetalPixelRequestGeneration: UInt64?
    private var c192MetalPixelRequestDrawableSize = CGSize.zero
    private var c192MetalPixelRequestFormat = ""
#else
    private let c192BoxPaintOpacityProbeRequested = false
#endif
#if SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST_1
    private let imageObjectFitPositionProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c183-object-fit-position")
#else
    private let imageObjectFitPositionProbeRequested = false
#endif
#if SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST_1
    private let imageAsyncReplacementProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c184-async-image")
#else
    private let imageAsyncReplacementProbeRequested = false
#endif
#if SPINON_ENABLE_C18_2_ALPHA_IMAGE_FIXTURE_TEST_1
    private let imageAlphaProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c182-alpha-image")
#else
    private let imageAlphaProbeRequested = false
#endif
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
    private let imagePaintProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c181-image-paint")
        || ProcessInfo.processInfo.arguments.contains("--spinon-c182-alpha-image")
        || ProcessInfo.processInfo.arguments.contains("--spinon-c183-object-fit-position")
        || ProcessInfo.processInfo.arguments.contains("--spinon-c184-async-image")
    private var imagePaintRouteArguments: [String] {
        imageAsyncReplacementProbeRequested ? ["--spinon-c184-async-image"]
            : imageObjectFitPositionProbeRequested ? ["--spinon-c183-object-fit-position"]
            : imageAlphaProbeRequested ? ["--spinon-c182-alpha-image"]
            : ["--spinon-c181-image-paint"]
    }
#else
    private let imagePaintProbeRequested = false
    private let imagePaintRouteArguments: [String] = []
#endif
    private let c133GestureFixtureRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c133-scroll-gesture")
    private let c133TapProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c133-tap-probe")
    private let c133WheelProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c133-wheel-probe")
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
    private let c133NativeWheelPreflightRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c133-native-wheel-preflight")
    private let c133NativeWheelAdapterArgumentRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c133-native-wheel-adapter")
    private let c133GesturePolicyScenarioArguments = ProcessInfo.processInfo.arguments.filter {
        $0.hasPrefix("--spinon-c133-policy-scenario-id=")
    }
    private var c133GesturePolicyScenarioID: Int32 {
        guard c133GesturePolicyScenarioArguments.count <= 1 else { return -1 }
        guard let argument = c133GesturePolicyScenarioArguments.first else { return 0 }
        let prefix = "--spinon-c133-policy-scenario-id="
        guard argument.hasPrefix(prefix),
              let scenarioID = Int32(argument.dropFirst(prefix.count)),
              scenarioID >= 0, scenarioID <= 10 else { return -1 }
        return scenarioID
    }
#endif
    private let c132MetalPixelRun = C132MetalPixelRun.parse(
        arguments: ProcessInfo.processInfo.arguments
    )
    private var c132MetalPixelArgumentRequested: Bool {
        ProcessInfo.processInfo.arguments.contains(where: {
            $0.hasPrefix("--spinon-c132-metal-surface-pixels=")
        })
    }
    private var c132ScrollAssertionsPassed = false
    private var c132MetalPixelRequestIssued = false
    private var c132MetalPixelReadbackFinished = false
    private var c132MetalPixelRequestGeneration: UInt64?
    private var c132MetalPixelRequestPresentationSequence: UInt64?
    private var c132MetalPixelRequestDrawableSize = CGSize.zero
    private var c132MetalPixelRequestFormat = ""
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
    private var imagePaintMetalPixelRequestIssued = false
    private var imagePaintMetalPixelReadbackFinished = false
#endif
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
    private var c133PanRecognizer: UIPanGestureRecognizer?
    private var c133TapRecognizer: UITapGestureRecognizer?
    private var c133NativeWheelDiscreteRecognizer: UIPanGestureRecognizer?
    private var c133NativeWheelContinuousRecognizer: UIPanGestureRecognizer?
    private var c133NativeWheelTranslations:
        [ObjectIdentifier: (translation: CGPoint, surfaceGeneration: UInt64)] = [:]
    private var c133ActiveTouchIDs = Set<ObjectIdentifier>()
    private var c133BackgroundObserver: NSObjectProtocol?
    private var c133DrainScheduled = false
    private var c133DrainAgain = false
    private var c133GestureEvaluationReady = false
    private var c133GestureFixtureReady = false
    private var c133WheelProbeStarted = false
    private var c133WheelProbeEnqueued = false
    private var c133WheelProbeProcessed = false
    private var c133WheelProbeCompleted = false
    private var c133WheelProbeFailed = false
    private var c133ConfiguredRuntimeEnvironment: C133RuntimeEnvironment?
    private var c133TouchActive = false
    private var c133TerminalQueued = false
    private var c133SuppressUntilAllPointersUp = false
    private var c133StyleMutationApplied = false
    private var c133StyleBarrierPending = false
    private var c133StyleDeferredMovePending = false
    private var c133StyleDeferredCumulativeCss = CGPoint.zero
    private var c133StyleDeferredTerminal: (action: Int, reason: Int)?
    private var c133NextGestureID: UInt64 = 0
    private var c133GestureID: UInt64 = 0
    private var c133PointerID: UInt64 = 0
    private var c133GestureSurfaceGeneration: UInt64 = 0
    private var c133CoordinateScale: Float = 1
    private var c133LastCumulativeCss = CGPoint.zero
    private var c133PanBegan = false
    private var c133ResolvedTouchAction = "unresolved"
    private var c133TapActivationCount = 0
#endif
    private let c122AbsoluteBlockProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c122-absolute-block")
    private let c123FixedPositionProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c123-fixed-position")
    private let c124StackingProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c124-stacking-context")
        || ProcessInfo.processInfo.arguments.contains("--spinon-c124-depth-safety")
        || ProcessInfo.processInfo.arguments.contains(where: {
            $0.hasPrefix("--spinon-c124-metal-surface-readback=")
        })
    private let c124MetalPixelCase = C124MetalPixelCase.parse(
        arguments: ProcessInfo.processInfo.arguments
    )
    private var c124MetalPixelReadbackFinished = false
    private let c124DepthSafetyProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c124-depth-safety")
    private let c1035PositionedFlexProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c1035-positioned-flex")
    private let blockFormattingProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c091-block-formatting")
        || ProcessInfo.processInfo.arguments.contains("--spinon-c092-margin-collapse")
        || ProcessInfo.processInfo.arguments.contains("--spinon-c093-flow-root")
    private let registeredPropertiesProbeRequested = ProcessInfo.processInfo.arguments
        .contains("--spinon-c052-registered-properties")
        || ProcessInfo.processInfo.arguments.contains("--spinon-c053-runtime-result-cache")
    private var authorStylesheetsProbeRequested: Bool {
        !registeredPropertiesProbeRequested && ProcessInfo.processInfo.arguments
            .contains("--spinon-c0411-runtime-author-stylesheets")
    }
    private var customPropertiesProbeRequested: Bool {
        !registeredPropertiesProbeRequested && !authorStylesheetsProbeRequested
            && ProcessInfo.processInfo.arguments.contains("--spinon-c051-custom-properties")
    }
    private var fixedSizeCssFixtureRequested: Bool {
        let selected = c153GlyphPaintProbeRequested || c192BoxPaintOpacityProbeRequested
            || c131CompactSurfaceMatrixRouteRequested || c131OverflowClipProbeRequested
            || c131StackingDescendantClipProbeRequested || c131BorderStripProbeRequested
            || c132ScrollFixtureRequested
            || c133GestureFixtureRequested || c11GridFixtureRequested
            || rootHtmlBlockPaintProbeRequested || rootHtmlTextPaintProbeRequested
            || rootHtmlDefaultRenderRequested
        var selectedWithImagePaint = selected
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
        selectedWithImagePaint = selectedWithImagePaint || imagePaintProbeRequested
#endif
        return selectedWithImagePaint
            || c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
            || c124StackingProbeRequested
            || c1035PositionedFlexProbeRequested
            || blockFormattingProbeRequested
            || flexWrapProbeRequested || flexDistributionProbeRequested
            || flexReverseProbeRequested || flexOrderProbeRequested || flexAlignmentProbeRequested
            || flexBaselineProbeRequested
    }
    private var failureProbeStarted = false
    private var shutdownProbeStarted = false
    private var closing = false
    private var c1121GridRuntimeSummaryEmitted = false
    private var c1122GridRuntimeSummaryEmitted = false
    private var c1123GridRuntimeSummaryEmitted = false
    private var displayScale: Float = 1
    private var darkMode = false
    private var runtimePresentationLane: C0410LatestTaskLane?
    private var renderDrawLane: C0410LatestTaskLane?

    override func viewDidLoad() {
        super.viewDidLoad()
        displayScale = Float(UIScreen.main.scale)
        darkMode = traitCollection.userInterfaceStyle == .dark
        view.backgroundColor = UIColor(red: 0.055, green: 0.075, blue: 0.12, alpha: 1)

        let title = UILabel()
        title.adjustsFontSizeToFitWidth = c153HostDocumentTextPaintArgumentRequested
            || rootHtmlDefaultRenderRequested
        title.minimumScaleFactor = 0.75
        title.text = rootHtmlDefaultRenderRequested
            ? "SPINON · C04.12 root HTML default runtime"
            : c153GlyphPaintProbeRequested
            ? c153HostDocumentTextPaintArgumentRequested
                ? "SPINON · C15.3 HostDocument text paint"
                : "SPINON · C15.3 WGPU glyph paint"
            : c192BoxPaintOpacityProbeRequested
            ? "SPINON · C19.2 box paint·opacity"
            : c131CompactHitIdentityGridProbeRequested
            ? "SPINON · C13.1 iOS hit identity grid"
            : c131StackingDescendantClipProbeRequested
            ? "SPINON · C13.1 nested stacking clip"
            : c131CompactSurfaceMatrixProbeRequested
            ? "SPINON · C13.1 compact surface"
            : c131BorderStripProbeRequested
            ? "SPINON · C13.1 paint/hit border"
            : c131OverflowClipProbeRequested
            ? "SPINON · C13.1 overflow: clip"
            : c132ScrollFixtureRequested
            ? "SPINON · C13.2 scroll state"
            : c133WheelProbeRequested
            ? "SPINON · C13.3 wheel fixture"
            : c133GestureFixtureRequested
            ? "SPINON · C13.3 iOS touch adapter"
            : c114GridTrackSizingProbeRequested
            ? "SPINON · C11.4 Grid track sizing"
            : c1133GridTemplateAreasProbeRequested
            ? "SPINON · C11.3.3 Grid template areas"
            : c1132GridNamedLinesProbeRequested
            ? "SPINON · C11.3.2 named Grid lines"
            : c1131GridRepeatProbeRequested
            ? "SPINON · C11.3.1 Grid repeat()"
            : c1123GridAutoPlacementProbeRequested
            ? "SPINON · C11.2.3 Grid"
            : c1122GridAutoPlacementProbeRequested
            ? "SPINON · C11.2.2 Grid"
            : c1121GridPlacementProbeRequested
            ? "SPINON · C11.2.1 Grid"
            : c11GridFixtureRequested
            ? "SPINON · C11.1 CSS Grid"
            : c1035PositionedFlexProbeRequested
            ? "SPINON · C10.3.5 positioned Flex"
            : c123FixedPositionProbeRequested
            ? "SPINON · C12.3 viewport fixed"
            : c124StackingProbeRequested
            ? "SPINON · C12.4 stacking context"
            : c122AbsoluteBlockProbeRequested
            ? "SPINON · C12.2 Block absolute"
            : c121PositioningProbeRequested
            ? "SPINON · C12.1 정적·상대 위치"
            : flexBaselineProbeRequested
            ? "SPINON · C10.3.4 Flex baseline"
            : flexAlignmentProbeRequested
            ? "SPINON · C10.3.3 Flex 정렬"
            : flexReverseProbeRequested
            ? "SPINON · C10.3.1 reverse Flex"
            : flexOrderProbeRequested
            ? "SPINON · C10.3.2 Flex order"
            : flexDistributionProbeRequested
            ? "SPINON · C10.2 flex distribution"
            : flexWrapProbeRequested
            ? "SPINON · C10.1 flex-wrap"
            : flowRootProbeRequested
            ? "SPINON · C09.3 flow-root"
            : marginCollapseProbeRequested
            ? "SPINON · C09.2 margin collapse"
            : blockFormattingProbeRequested
            ? "SPINON · C09.1 Block formatting"
            : viewportUnitsProbeRequested
            ? "SPINON · C06.6 viewport units"
            : rootHtmlTextPaintProbeRequested
            ? "SPINON · C04.12 root HTML text paint"
            : rootHtmlBlockPaintProbeRequested
            ? "SPINON · C04.12 root HTML Block paint"
            : blockPaintProbeRequested
            ? "SPINON · C08 Block 흐름"
            : aspectRatioProbeRequested
            ? "SPINON · C07.3 aspect ratio"
            : borderWidthProbeRequested
            ? "SPINON · C07.2 border width"
            : minMaxSizingProbeRequested
            ? "SPINON · C07.1 min/max sizing"
            : typedCssMathProbeRequested
            ? "SPINON · C06.5 typed CSS math"
            : fontRelativeUnitsProbeRequested
            ? "SPINON · C06.4 font-relative units"
            : absoluteLengthsProbeRequested
            ? "SPINON · C06.3 absolute lengths"
            : spacingPercentagesProbeRequested
            ? "SPINON · C06.2 spacing percentages"
            : percentageDimensionsProbeRequested
            ? "SPINON · C06.1 percentage dimensions"
            : incrementalRestyleProbeRequested
            ? "SPINON · C05.4 incremental restyle"
            : registeredPropertiesProbeRequested
            ? runtimeResultCacheProbeRequested ? "SPINON · C05.3 runtime cache"
                : "SPINON · C05.2 @property"
            : authorStylesheetsProbeRequested
                ? "SPINON · C04.11 CSS → WGPU" : "SPINON · C04.10 CSS → WGPU"
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
        if imagePaintProbeRequested {
            title.text = imageAsyncReplacementProbeRequested
                ? "SPINON · C18.4 async PNG replacement and recovery"
                : imageObjectFitPositionProbeRequested
                ? "SPINON · C18.3 object-fit/object-position"
                : imageAlphaProbeRequested
                ? "SPINON · C18.2 alpha image" : "SPINON · C18.1 image paint"
        }
#endif
        title.textColor = UIColor(red: 0.92, green: 0.95, blue: 0.99, alpha: 1)
        title.font = .systemFont(ofSize: 22, weight: .bold)
        title.translatesAutoresizingMaskIntoConstraints = false
        title.isHidden = c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
            || defaultRootHtmlApplicationRequested
        view.addSubview(title)

        let description = UILabel()
        description.text = c153GlyphPaintProbeRequested
            ? c153HostDocumentTextPaintArgumentRequested
                ? "HostDocument Text(XX) → Taffy text snapshot → Ahem glyph mask · 320×240 CSS px · DPR 1"
                : "고정 Noto Sans KR glyph coverage mask → WGPU Metal · 360×240 CSS px · DPR 1"
            : c192BoxPaintOpacityProbeRequested
            ? "alpha·중첩 opacity·clip·z-order·currentColor·OKLCH → WGPU Metal · 16표본 · 360×240 CSS px"
            : c131CompactHitIdentityGridProbeRequested
            ? "Chrome 155 integer hit identity 30,000점 · 52 sample · 300×100 CSS px"
            : c131StackingDescendantClipProbeRequested
            ? "visible/clip/hidden stacked descendant · 18 Chrome hit/color sample · 540×128 CSS px"
            : c131CompactSurfaceMatrixProbeRequested
            ? "13개 overflow clip case · 52 hit sample → WGPU Metal · 300×100 CSS px"
            : c131BorderStripProbeRequested
            ? "static/absolute clip border-strip hit oracle → WGPU Metal · 400×160 CSS px"
            : c131OverflowClipProbeRequested
            ? "V8 overflow: clip·clip margin·positioned escape → Stylo → Taffy → WGPU Metal · 320×240 CSS px"
            : c132ScrollFixtureRequested
            ? "V8 scroll owner·offset·paint/hit → Stylo → Taffy → WGPU Metal · 400×240 CSS px"
            : c133WheelProbeRequested
            ? "Synthetic CSS px delta → Rust queue → scroll router → WGPU Metal"
            : c133GestureFixtureRequested
            ? "UIPanGestureRecognizer · bounded Rust queue/session → WGPU Metal · 400×240 CSS px"
            : c114GridTrackSizingProbeRequested
            ? "Intrinsic Grid track sizing → Stylo → Taffy → WGPU Metal · 320×240 CSS px"
            : c1133GridTemplateAreasProbeRequested
            ? "Grid template area placement → Stylo → Taffy → WGPU Metal · 320×240 CSS px"
            : c1132GridNamedLinesProbeRequested
            ? "named Grid line occurrence·span → Stylo → Taffy → WGPU · 320×240 CSS px"
            : c1131GridRepeatProbeRequested
            ? "정수 Grid repeat() 확장·line-name metadata → Stylo → Taffy → WGPU · 320×240 CSS px"
            : c1123GridAutoPlacementProbeRequested
            ? "dense·column Grid 자동 배치 → Stylo → Taffy → WGPU Metal · 360×800 CSS px"
            : c1122GridAutoPlacementProbeRequested
            ? "V8 행 우선 sparse auto-placement·implicit row → Stylo → Taffy → WGPU · 360×800 CSS px"
            : c1121GridPlacementProbeRequested
            ? "V8 numeric Grid line/span → Stylo → Taffy → WGPU · 320×240 CSS px"
            : c11GridFixtureRequested
            ? "V8 DOM display:grid → Stylo → Taffy → Rust 장면 → WGPU · 320×240 CSS px"
            : c1035PositionedFlexProbeRequested
            ? "V8 직접 Flex 자식·wrapper·paint 순서 → Stylo → Taffy → WGPU · 320×240 CSS px"
            : c123FixedPositionProbeRequested
            ? "V8 fixed viewport owner·inset·resize → Stylo → Taffy → WGPU · 360×800 CSS px"
            : c122AbsoluteBlockProbeRequested
            ? "V8 absolute·containing block·inset → Stylo → Taffy → WGPU · 360×800 CSS px"
            : c121PositioningProbeRequested
            ? "V8 position·inset·flow/visual frame → Stylo → Taffy → WGPU"
            : flexBaselineProbeRequested
            ? "V8 first·last baseline·중첩 전파 → Stylo → Taffy → WGPU · 320×240 CSS px"
            : flexAlignmentProbeRequested
            ? "V8 align-items/self/content·place-* → Stylo → Taffy → WGPU · 320×240 CSS px"
            : flexReverseProbeRequested
            ? "V8 row-reverse·wrap-reverse → Stylo → Taffy → WGPU · 320×240 CSS px"
            : flexOrderProbeRequested
            ? "V8 Flex order·stable ties·overlap paint → Stylo → Taffy → WGPU · 320×240 CSS px"
            : flexDistributionProbeRequested
            ? "V8 flex basis·grow·shrink·min/max freeze → Stylo → Taffy → WGPU · 320×240 CSS px"
            : flexWrapProbeRequested
            ? "V8 CSS flex-wrap row·gap → Stylo → Taffy → WGPU · 320×240 CSS px"
            : flowRootProbeRequested
            ? "V8 DOM flow-root BFC 경계 → Stylo → Taffy → WGPU · 320×240 CSS px"
            : marginCollapseProbeRequested
            ? "V8 DOM signed vertical margins → Stylo → Taffy → WGPU · 320×240 CSS px"
            : blockFormattingProbeRequested
            ? "V8 DOM Block·auto margin → Stylo → Taffy → WGPU · 320×240 CSS px"
            : viewportUnitsProbeRequested
            ? "V8 vw/svw/lvw/dvw → Stylo CSS px → Taffy → WGPU"
            : rootHtmlTextPaintProbeRequested
            ? "V8 parsed direct text → Stylo → HostDocument → glyph layout → WGPU · 320×240 CSS px"
            : rootHtmlBlockPaintProbeRequested
            ? "V8 parsed root HTML → meta viewport/color-scheme → Stylo → Taffy → WGPU · 320×240 CSS px"
            : blockPaintProbeRequested
            ? "V8 DOM Block 흐름 → Stylo → Taffy → WGPU"
            : aspectRatioProbeRequested
            ? "V8 CSS aspect-ratio → Stylo → Taffy → WGPU"
            : borderWidthProbeRequested
            ? "V8 CSS border widths → Stylo → Taffy → WGPU · 테두리 페인트 제외"
            : minMaxSizingProbeRequested
            ? "V8 min/max size → Stylo typed values → Taffy → WGPU"
            : typedCssMathProbeRequested
            ? "V8 calc/min/max/clamp → Stylo typed values → Taffy → WGPU"
            : fontRelativeUnitsProbeRequested
            ? "V8 CSS em/rem → Stylo computed CSS px → Taffy → WGPU"
            : absoluteLengthsProbeRequested
            ? "V8 CSS absolute lengths → Stylo CSS px → Taffy → WGPU"
            : spacingPercentagesProbeRequested
            ? "V8 CSS spacing % → Stylo → Taffy → WGPU"
            : percentageDimensionsProbeRequested
            ? "V8 CSS % → typed dimensions → Taffy → WGPU"
            : incrementalRestyleProbeRequested
            ? "V8 inline style → dirty subtree → Stylo → Taffy → WGPU"
            : registeredPropertiesProbeRequested
            ? runtimeResultCacheProbeRequested
                ? "V8 detached DOM → worker cache → WGPU"
                : "V8 DOM <style> → Stylo → Taffy → WGPU"
            : authorStylesheetsProbeRequested
                ? "실제 V8 DOM <style> → Stylo → Taffy → wgpu surface"
            : "실제 V8 DOM → Stylo → Taffy → Rust 장면 → wgpu surface"
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
        if imagePaintProbeRequested {
            description.text = imageAsyncReplacementProbeRequested
                ? "Out-of-order PNG decode · pending/failure resource clear · recovery · 6 Metal samples · 224×88 CSS px"
                : imageObjectFitPositionProbeRequested
                ? "PNG 24×12 · 12 object-fit/position scenes · 19 Metal 표본 · 224×72 CSS px"
                : imageAlphaProbeRequested
                ? "RGBA PNG source-over alpha · 10 Metal 표본 · 80×40 CSS px"
                : "opaque PNG · visible/hidden 부모 경계 · 9 Metal 표본 · 80×40 CSS px"
        }
#endif
        description.textColor = UIColor(red: 0.78, green: 0.83, blue: 0.90, alpha: 1)
        description.font = .systemFont(ofSize: 14)
        description.numberOfLines = 2
        description.translatesAutoresizingMaskIntoConstraints = false
        description.isHidden = c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
            || defaultRootHtmlApplicationRequested
        view.addSubview(description)

        canvasView.translatesAutoresizingMaskIntoConstraints = false
        canvasView.isAccessibilityElement = true
        canvasView.accessibilityLabel = defaultRootHtmlApplicationRequested
            ? "Spinon 기본 root HTML 화면"
            : c153GlyphPaintProbeRequested
            ? c153HostDocumentTextPaintArgumentRequested
                ? "C15.3 HostDocument text layout Ahem overflow WGPU Metal fixture"
                : "C15.3 고정 Noto Sans KR glyph alpha mask WGPU Metal fixture"
            : c192BoxPaintOpacityProbeRequested
            ? "C19.2 box paint, opacity, clipping, z-order, currentColor, and wide-gamut WGPU Metal surface"
            : c131CompactHitIdentityGridProbeRequested
            ? "C13.1 iOS Simulator full 30,000-point hit identity grid WGPU Metal surface"
            : c131StackingDescendantClipProbeRequested
            ? "C13.1 nested stacking descendant clip Chrome oracle WGPU Metal surface"
            : c131CompactSurfaceMatrixProbeRequested
            ? "C13.1 compact 13-case overflow Chrome oracle WGPU Metal surface"
            : c131BorderStripProbeRequested
            ? "C13.1 paint/hit border-strip Chrome 경계 WGPU Metal 장면"
            : c131OverflowClipProbeRequested
            ? "C13.1 overflow clip과 positioned escape WGPU Metal 장면"
            : c133GestureFixtureRequested
            ? "C13.3 iOS UIPanGestureRecognizer nested scroll gesture WGPU Metal 장면"
            : c1123GridAutoPlacementProbeRequested
            ? "C11.2.3 dense·column Grid 자동 배치 WGPU 장면"
            : c1122GridAutoPlacementProbeRequested
            ? "C11.2.2 행 우선 Grid 자동 배치 WGPU 장면"
            : c1121GridPlacementProbeRequested
            ? "C11.2.1 numeric Grid line 배치 WGPU 장면"
            : c1035PositionedFlexProbeRequested
            ? "C10.3.5 positioned Flex 정렬·wrapper·paint 순서 WGPU 장면"
            : c11GridFixtureRequested
            ? "C11.1 고정 CSS Grid 트랙·gap 배치 WGPU 장면"
            : c123FixedPositionProbeRequested
            ? "C12.3 viewport fixed 배치 WGPU 장면"
            : c124StackingProbeRequested
            ? "C12.4 z-index 쌓임 맥락 WGPU 장면"
            : c122AbsoluteBlockProbeRequested
            ? "C12.2 Block absolute WGPU 장면"
            : c121PositioningProbeRequested
            ? "C12.1 정적·상대 위치 WGPU 장면"
            : flexBaselineProbeRequested
            ? "C10.3.4 Flex first·last baseline WGPU 장면"
            : flexAlignmentProbeRequested
            ? "C10.3.3 Flex Box Alignment WGPU 장면"
            : flexReverseProbeRequested
            ? "C10.3.1 row-reverse와 wrap-reverse WGPU 장면"
            : flexOrderProbeRequested
            ? "C10.3.2 Flex order와 겹친 색상 WGPU 장면"
            : flexDistributionProbeRequested
            ? "C10.2 flex grow·shrink·min/max freeze WGPU 장면"
            : flexWrapProbeRequested
            ? "C10.1 세 줄 flex-wrap·gap WGPU 장면"
            : flowRootProbeRequested
            ? "C09.3 flow-root 내부 margin 격리와 외부 collapse WGPU 장면"
            : marginCollapseProbeRequested
            ? "C09.2 signed vertical margin collapse WGPU 장면"
            : viewportUnitsProbeRequested
            ? "C06.6 viewport units WGPU 장면"
            : blockFormattingProbeRequested
            ? "C09.1 일반 Block 흐름·auto margin WGPU 장면"
            : rootHtmlTextPaintProbeRequested
            ? "C04.12 parsed root HTML direct text · Ahem glyphs · owner overflow clip WGPU scene"
            : rootHtmlBlockPaintProbeRequested
            ? "C04.12 실제 html/head/body Block 배경 페인트 WGPU 장면"
            : blockPaintProbeRequested
            ? "C08 Block 흐름 기본 페인트 WGPU 장면"
            : aspectRatioProbeRequested
            ? "C07.3 종횡비 WGPU 장면"
            : borderWidthProbeRequested
            ? "C07.2 테두리 폭 레이아웃 WGPU 장면"
            : minMaxSizingProbeRequested
            ? "C07.1 min/max sizing WGPU 장면"
            : typedCssMathProbeRequested
            ? "C06.5 calc min max clamp WGPU 장면"
            : fontRelativeUnitsProbeRequested
            ? "C06.4 em/rem font-relative units WGPU 장면"
            : absoluteLengthsProbeRequested
            ? "C06.3 absolute CSS length units WGPU 장면"
            : spacingPercentagesProbeRequested
            ? "C06.2 percentage margin, padding, gap WGPU 장면"
            : percentageDimensionsProbeRequested
            ? "C06.1 percentage width, height, flex-basis WGPU 장면"
            : incrementalRestyleProbeRequested
            ? "C05.4 inline style 하위 트리 재계산 검증 WGPU 장면"
            : registeredPropertiesProbeRequested
            ? runtimeResultCacheProbeRequested
                ? "C05.3 detached-node 결과 재사용 검증 WGPU 장면"
                : "C05.2 등록 사용자 지정 속성 Chromium fixture의 WGPU 장면"
            : authorStylesheetsProbeRequested
                ? "C04.11 Chromium stylesheet fixture의 WGPU 장면"
            : "C04.10 Chromium fixture의 WGPU 장면"
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
        if imagePaintProbeRequested {
            canvasView.accessibilityLabel =
                imageAsyncReplacementProbeRequested
                ? "C18.4 비동기 PNG 교체·실패·복구 WGPU Metal 장면"
                : imageObjectFitPositionProbeRequested
                ? "C18.3 PNG object-fit와 object-position WGPU Metal 장면"
                : imageAlphaProbeRequested
                ? "C18.2 PNG alpha source-over WGPU Metal 장면"
                : "C18.1 image paint overflow와 부모 border 경계 WGPU Metal 장면"
        }
#endif
        view.addSubview(canvasView)
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
        if c133GestureFixtureRequested {
            canvasView.isMultipleTouchEnabled = true
            canvasView.onActiveTouchCountChanged = { [weak self] count in
                self?.handleC133ActiveTouchCountChanged(count)
            }
            let recognizer = UIPanGestureRecognizer(
                target: self,
                action: #selector(handleC133Pan(_:))
            )
            recognizer.minimumNumberOfTouches = 1
            recognizer.maximumNumberOfTouches = 1
            recognizer.allowedScrollTypesMask = []
            recognizer.cancelsTouchesInView = false
            recognizer.delaysTouchesBegan = false
            canvasView.addGestureRecognizer(recognizer)
            c133PanRecognizer = recognizer
            if c133NativeWheelPreflightRequested || c133NativeWheelAdapterArgumentRequested {
                let discreteRecognizer = UIPanGestureRecognizer(
                    target: self,
                    action: #selector(handleC133NativeWheel(_:))
                )
                discreteRecognizer.allowedScrollTypesMask = .discrete
                discreteRecognizer.allowedTouchTypes = []
                discreteRecognizer.cancelsTouchesInView = false
                discreteRecognizer.delaysTouchesBegan = false
                canvasView.addGestureRecognizer(discreteRecognizer)
                c133NativeWheelDiscreteRecognizer = discreteRecognizer

                let continuousRecognizer = UIPanGestureRecognizer(
                    target: self,
                    action: #selector(handleC133NativeWheel(_:))
                )
                continuousRecognizer.allowedScrollTypesMask = .continuous
                continuousRecognizer.allowedTouchTypes = []
                continuousRecognizer.cancelsTouchesInView = false
                continuousRecognizer.delaysTouchesBegan = false
                canvasView.addGestureRecognizer(continuousRecognizer)
                c133NativeWheelContinuousRecognizer = continuousRecognizer
            }
            if c133TapProbeRequested {
                let tapRecognizer = UITapGestureRecognizer(
                    target: self,
                    action: #selector(handleC133Tap(_:))
                )
                tapRecognizer.numberOfTapsRequired = 1
                tapRecognizer.numberOfTouchesRequired = 1
                tapRecognizer.cancelsTouchesInView = false
                tapRecognizer.delaysTouchesBegan = false
                tapRecognizer.require(toFail: recognizer)
                canvasView.addGestureRecognizer(tapRecognizer)
                c133TapRecognizer = tapRecognizer
            }
            c133BackgroundObserver = NotificationCenter.default.addObserver(
                forName: UIApplication.didEnterBackgroundNotification,
                object: nil,
                queue: .main
            ) { [weak self] _ in
                self?.handleC133ApplicationDidEnterBackground()
            }
        }
#endif

        resizeButton.setTitle("표면 크기 전환 · 301×100 CSS px", for: .normal)
        resizeButton.addTarget(self, action: #selector(toggleSurfaceSize), for: .touchUpInside)
        resizeButton.translatesAutoresizingMaskIntoConstraints = false
        if fixedSizeCssFixtureRequested || defaultRootHtmlApplicationRequested {
            resizeButton.isHidden = true
        }
        view.addSubview(resizeButton)

        customPropertiesButton.setTitle(
            typedCssMathProbeRequested
                ? "C06.5 typed CSS math fixture 다시 실행"
                : fontRelativeUnitsProbeRequested
                ? "C06.4 em/rem fixture 실행 완료"
                : absoluteLengthsProbeRequested
                ? "C06.3 absolute length fixture 실행 완료"
                : spacingPercentagesProbeRequested
                ? "C06.2 spacing percentage fixture 실행 완료"
                : percentageDimensionsProbeRequested
                ? "C06.1 percentage fixture 실행 완료"
                : incrementalRestyleProbeRequested
                ? "C05.4 · 왼쪽 branch style 전환"
                : registeredPropertiesProbeRequested
                ? runtimeResultCacheProbeRequested
                    ? "C05.3 · detached / 연결 변경 실행"
                    : "C05.2 · 등록 사용자 지정 속성 다시 적용"
                : "C05 · 사용자 지정 속성 다시 적용",
            for: .normal
        )
        customPropertiesButton.addTarget(
            self,
            action: typedCssMathProbeRequested
                ? #selector(evaluateTypedCssMathFixture)
                : fontRelativeUnitsProbeRequested
                ? #selector(evaluateFontRelativeUnitsFixture)
                : absoluteLengthsProbeRequested
                ? #selector(evaluateAbsoluteLengthsFixture)
                : spacingPercentagesProbeRequested
                ? #selector(evaluateSpacingPercentagesFixture)
                : percentageDimensionsProbeRequested
                ? #selector(evaluatePercentageDimensionsFixture)
                : incrementalRestyleProbeRequested
                ? #selector(evaluateIncrementalRestyleFixture)
                : registeredPropertiesProbeRequested
                ? runtimeResultCacheProbeRequested
                    ? #selector(evaluateRuntimeResultCacheFixture)
                    : #selector(evaluateRegisteredPropertiesFixture)
                : #selector(evaluateCustomPropertiesFixture),
            for: .touchUpInside
        )
        customPropertiesButton.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(customPropertiesButton)
        if c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
            || c121PositioningProbeRequested
            || authorStylesheetsProbeRequested
            || fixedSizeCssFixtureRequested || blockPaintProbeRequested
            || minMaxSizingProbeRequested || borderWidthProbeRequested
            || aspectRatioProbeRequested
            || typedCssMathProbeRequested
            || fontRelativeUnitsProbeRequested
            || absoluteLengthsProbeRequested
            || percentageDimensionsProbeRequested
            || spacingPercentagesProbeRequested {
            customPropertiesButton.isHidden = true
        }
        if defaultRootHtmlApplicationRequested {
            customPropertiesButton.isHidden = true
        }

        statusLabel.text = "V8·CSS runtime 준비 중…"
        statusLabel.textColor = UIColor(red: 0.38, green: 0.73, blue: 1, alpha: 1)
        statusLabel.font = .monospacedSystemFont(ofSize: 11, weight: .regular)
        statusLabel.numberOfLines = 0
        statusLabel.translatesAutoresizingMaskIntoConstraints = false
        statusLabel.isHidden = c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
            || defaultRootHtmlApplicationRequested
        view.addSubview(statusLabel)

        let canvasWidth = canvasView.widthAnchor.constraint(
            equalToConstant: imagePaintProbeRequested
                ? imageAsyncReplacementProbeRequested || imageObjectFitPositionProbeRequested ? 224 : 80
                : c153HostDocumentTextPaintArgumentRequested ? 320
                : c153GlyphPaintProbeRequested || c192BoxPaintOpacityProbeRequested ? 360
                : c131StackingDescendantClipProbeRequested ? 540
                : c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
                ? 360 : c131CompactSurfaceMatrixRouteRequested ? 300
                : c131BorderStripProbeRequested || c132ScrollFixtureRequested
                    || c133GestureFixtureRequested
                ? 400 : fixedSizeCssFixtureRequested ? 320 : 301
        )
        let canvasHeight = canvasView.heightAnchor.constraint(
            equalToConstant: imagePaintProbeRequested
                ? imageAsyncReplacementProbeRequested ? 88
                : imageObjectFitPositionProbeRequested ? 72 : 40
                : c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
                ? 800 : c131CompactSurfaceMatrixRouteRequested ? 100
                : c131BorderStripProbeRequested ? 160
                : c131StackingDescendantClipProbeRequested ? 128
                : c153GlyphPaintProbeRequested || c192BoxPaintOpacityProbeRequested ? 240
                : fixedSizeCssFixtureRequested ? 240 : 100
        )
        var layoutConstraints: [NSLayoutConstraint] = [
            title.leadingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.leadingAnchor, constant: 22),
            title.trailingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.trailingAnchor, constant: -22),
            title.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 24),
            description.leadingAnchor.constraint(equalTo: title.leadingAnchor),
            description.trailingAnchor.constraint(equalTo: title.trailingAnchor),
            description.topAnchor.constraint(equalTo: title.bottomAnchor, constant: 8),
            resizeButton.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            resizeButton.topAnchor.constraint(equalTo: canvasView.bottomAnchor, constant: 12),
            customPropertiesButton.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            customPropertiesButton.topAnchor.constraint(equalTo: resizeButton.bottomAnchor, constant: 4),
            statusLabel.leadingAnchor.constraint(equalTo: title.leadingAnchor),
            statusLabel.trailingAnchor.constraint(equalTo: title.trailingAnchor),
            statusLabel.bottomAnchor.constraint(equalTo: view.safeAreaLayoutGuide.bottomAnchor, constant: -18),
        ]
        if defaultRootHtmlApplicationRequested {
            canvasWidthConstraint = nil
            canvasHeightConstraint = nil
            layoutConstraints.append(contentsOf: [
                canvasView.leadingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.leadingAnchor),
                canvasView.trailingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.trailingAnchor),
                canvasView.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor),
                canvasView.bottomAnchor.constraint(equalTo: view.safeAreaLayoutGuide.bottomAnchor),
            ])
        } else {
            canvasWidthConstraint = canvasWidth
            canvasHeightConstraint = canvasHeight
            layoutConstraints.append(contentsOf: [
                canvasView.centerXAnchor.constraint(equalTo: view.centerXAnchor),
                canvasView.centerYAnchor.constraint(equalTo: view.centerYAnchor),
                canvasWidth,
                canvasHeight,
            ])
        }
        NSLayoutConstraint.activate(layoutConstraints)
        if failureProbeRequested && shutdownProbeRequested {
            statusLabel.text = "draw 복구와 종료 검증은 나눠 실행해야 합니다."
            log("SPINON_C0410_PROBE_ARGUMENT_ERROR · failure와 shutdown 검증을 동시에 요청했습니다")
            return
        }
        runtimePresentationLane = C0410LatestTaskLane(
            queue: runtimeQueue,
            work: { [weak self] in
                guard let self else { return }
                try self.applyLatestEnvironment()
            },
            onFailure: { [weak self] message in self?.reportLaneFailure("runtime", message) }
        )
        renderDrawLane = C0410LatestTaskLane(
            queue: renderQueue,
            work: { [weak self] in
                guard let self else { return }
                try self.drawLatestIfReady()
            },
            onFailure: { [weak self] message in self?.reportLaneFailure("render", message) }
        )
        if !defaultRootHtmlApplicationRequested {
            runtimeInitializationEnqueued = true
            enqueueRuntime { [weak self] in self?.initializeRuntime() }
        }
    }

    override func viewDidDisappear(_ animated: Bool) {
        super.viewDidDisappear(animated)
        beginShutdown()
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        guard !automaticResizeScheduled,
              ProcessInfo.processInfo.arguments.contains("--spinon-c0410-auto-resize") else {
            return
        }
        automaticResizeScheduled = true
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) { [weak self] in
            self?.runAutomaticResizeProbe(step: 0)
        }
    }

    private func runAutomaticResizeProbe(step: Int) {
        guard step < 3, !isClosing else { return }
        toggleSurfaceSize()
        guard step + 1 < 3 else { return }
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.75) { [weak self] in
            self?.runAutomaticResizeProbe(step: step + 1)
        }
    }

    override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()
        let scale = view.window?.screen.scale ?? UIScreen.main.scale
        let viewportSize = c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
            ? CGSize(width: 360, height: 800) : canvasView.bounds.size
        guard viewportSize.width > 0, viewportSize.height > 0 else { return }
        stateLock.lock()
        let scaleChanged = displayScale != Float(scale)
        let viewportChanged = viewportWidthCssPx != Float(viewportSize.width)
            || viewportHeightCssPx != Float(viewportSize.height)
        displayScale = Float(scale)
        viewportWidthCssPx = Float(viewportSize.width)
        viewportHeightCssPx = Float(viewportSize.height)
        let shouldInitializeDefaultRootHtmlApplication =
            defaultRootHtmlApplicationRequested && !runtimeInitializationEnqueued
        if shouldInitializeDefaultRootHtmlApplication {
            runtimeInitializationEnqueued = true
        }
        stateLock.unlock()
        if shouldInitializeDefaultRootHtmlApplication {
            log("SPINON_ROOT_HTML_DEFAULT_APP_SURFACE_READY viewport_css_px=\(viewportSize.width)x\(viewportSize.height) scale=\(scale)")
            enqueueRuntime { [weak self] in self?.initializeRuntime() }
        }
        let size = CGSize(
            width: (canvasView.bounds.width * scale).rounded(),
            height: (canvasView.bounds.height * scale).rounded()
        )
        let sizeChanged = size != lastDrawableSize
        guard size.width > 0, size.height > 0 else { return }
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
        if imagePaintProbeRequested && !imagePaintScreenGeometryLogged,
           let window = canvasView.window,
           let screen = window.windowScene?.screen {
            let canvasFrame = canvasView.convert(canvasView.bounds, to: nil)
            let contentFrame = view.convert(view.bounds, to: nil)
            imagePaintScreenGeometryLogged = true
            let geometry = [
                imageAlphaProbeRequested
                    ? "SPINON_C182_SCREEN_GEOMETRY" : "SPINON_C181_SCREEN_GEOMETRY",
                "canvas_origin_ui_points=\(canvasFrame.minX),\(canvasFrame.minY)",
                "canvas_bounds=\(canvasFrame.width)x\(canvasFrame.height)",
                "content_origin_ui_points=\(contentFrame.minX),\(contentFrame.minY)",
                "content_bounds=\(contentFrame.width)x\(contentFrame.height)",
                "screen_bounds_ui_points=\(screen.bounds.width)x\(screen.bounds.height)",
                "screen_native_bounds_pixels=\(screen.nativeBounds.width)x\(screen.nativeBounds.height)",
                "screen_scale=\(screen.scale)",
                "drawable=\(Int(size.width))x\(Int(size.height))",
            ].joined(separator: " ")
            log(geometry)
        }
#endif
        stateLock.lock()
        c111MetalDrawableSize = size
        stateLock.unlock()
        if viewportChanged || sizeChanged {
            log("SPINON_C0410_VIEWPORT_CHANGED css=\(viewportSize.width)x\(viewportSize.height) scale=\(scale) drawable=\(Int(size.width))x\(Int(size.height))")
        }
        if scaleChanged || sizeChanged {
            if sizeChanged {
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
                if c133GestureFixtureRequested {
                    cancelC133Gesture(reason: 3, source: "surface-generation-change")
                }
#endif
                stateLock.lock()
                surfaceGeneration &+= 1
                let generation = surfaceGeneration
                stateLock.unlock()
                log("SPINON_C0410_SURFACE_CHANGED generation=\(generation) size=\(Int(size.width))x\(Int(size.height))")
            }
            _ = beginPresentationUpdate()
        }
        guard sizeChanged else {
            if scaleChanged || viewportChanged { refreshEnvironment() }
            return
        }
        lastDrawableSize = size
        (canvasView.layer as? CAMetalLayer)?.drawableSize = size
        if rendererIsReady {
            resizeRenderer(to: size)
        } else {
            ensureRenderer(width: Int(size.width), height: Int(size.height))
        }
    }

    override func traitCollectionDidChange(_ previousTraitCollection: UITraitCollection?) {
        super.traitCollectionDidChange(previousTraitCollection)
        guard previousTraitCollection?.userInterfaceStyle != traitCollection.userInterfaceStyle else {
            return
        }
        stateLock.lock()
        darkMode = traitCollection.userInterfaceStyle == .dark
        stateLock.unlock()
        refreshEnvironment()
    }

    private var rendererIsReady: Bool {
        stateLock.lock()
        defer { stateLock.unlock() }
        return rendererReady
    }

    private var isClosing: Bool {
        stateLock.lock()
        defer { stateLock.unlock() }
        return closing
    }

    private func currentEnvironment() -> (width: Float, height: Float, scale: Float, dark: Bool) {
        stateLock.lock()
        defer { stateLock.unlock() }
        return (
            c153HostDocumentTextPaintArgumentRequested ? 320
                : c153GlyphPaintProbeRequested ? 360 : viewportWidthCssPx,
            c153GlyphPaintProbeRequested ? 240 : viewportHeightCssPx,
            c153GlyphPaintProbeRequested ? 1 : displayScale,
            darkMode
        )
    }

    private func runtimeStatusSummary(_ report: String) -> String {
        let status = String(report.split(separator: " ", maxSplits: 1).first ?? "")
        guard report.contains(" layout=") else {
            return String(report.prefix(180))
        }
        return [
            status,
            "layout=\(reportField(report, "layout="))",
            "boxes=\(reportField(report, "boxes="))",
            "generation=\(reportField(report, "generation="))",
            "document_revision=\(reportField(report, "document_revision="))",
            "environment_revision=\(reportField(report, "environment_revision="))"
        ].joined(separator: " ")
    }

    private func logC114GridTrackSizingTraceChunks(_ report: String) -> Bool {
        let marker = "grid_track_sizing_diagnostics="
        guard let range = report.range(of: marker) else {
            log("SPINON_C114_GRID_TRACE_MISSING run_id=\(c114GridTrackSizingRunID)")
            return false
        }
        let trace = String(report[range.lowerBound...])
        let chunkSize = 800
        let chunkCount = (trace.utf8.count + chunkSize - 1) / chunkSize
        var start = trace.startIndex
        var chunkIndex = 0
        while start < trace.endIndex {
            let end = trace.index(start, offsetBy: chunkSize, limitedBy: trace.endIndex)
                ?? trace.endIndex
            chunkIndex += 1
            log("SPINON_C114_GRID_TRACE_CHUNK run_id=\(c114GridTrackSizingRunID) part=\(chunkIndex)/\(chunkCount) \(trace[start..<end])")
            start = end
        }
        return true
    }

    private func logC131FixtureReport(_ report: String) -> Bool {
        let expectedHitIDs = [
            "visible-target", "clip-inside", "clip-outside", "clip-margin-expanded",
            "clip-margin-outside", "nested-clip-inside", "nested-clip-outside",
            "nested-y-visible", "nested-y-clip-outside", "positioned-escape",
            "outer-clip-outside", "viewport-left-outside", "viewport-top-outside",
            "viewport-right-outside", "viewport-bottom-outside",
        ]
        guard report.hasPrefix("status=0 "),
              report.contains(" layout=ready "),
              reportField(report, "boxes=") == "13",
              let frameMarker = report.range(of: "node_frames_css_px=["),
              let frameEnd = report[frameMarker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C131_FIXTURE_SUMMARY status=error stage=runtime-report \(runtimeStatusSummary(report))")
            return false
        }
        let frames = report[frameMarker.upperBound..<frameEnd].split(separator: ";")
        for frame in frames {
            log("SPINON_C131_NODE_FRAME \(frame)")
        }
        log("SPINON_C131_NODE_FRAME_COUNT count=\(frames.count)")

        guard let sampleMarker = report.range(of: "hit_test_samples_json="),
              let sampleStart = report[sampleMarker.upperBound...].firstIndex(of: "["),
              let sampleEnd = report[report.index(after: sampleStart)...].firstIndex(of: "]") else {
            log("SPINON_C131_FIXTURE_SUMMARY status=error stage=hit-test-report")
            return false
        }
        let sampleJSON = String(report[sampleStart...sampleEnd])
        guard let sampleData = sampleJSON.data(using: .utf8),
              let samples = try? JSONSerialization.jsonObject(with: sampleData) as? [[String: Any]] else {
            log("SPINON_C131_FIXTURE_SUMMARY status=error stage=hit-test-json")
            return false
        }
        let sampleIDs = samples.compactMap { $0["id"] as? String }
        let matched = samples.filter { $0["status"] as? String == "matched" }
        for sample in samples {
            let id = sample["id"] as? String ?? "missing"
            let element = sample["elementId"] as? String ?? "null"
            let x = sample["xCssPx"] ?? "missing"
            let y = sample["yCssPx"] ?? "missing"
            let status = sample["status"] as? String ?? "missing"
            log("SPINON_C131_HIT_TEST id=\(id) x=\(x) y=\(y) element=\(element) status=\(status)")
        }
        let passed = frames.count == 13 && samples.count == 15
            && matched.count == 15 && sampleIDs == expectedHitIDs
        log("SPINON_C131_HIT_TEST_SUMMARY status=\(passed ? "ok" : "error") matched=\(matched.count) total=\(samples.count) ids=\(sampleIDs == expectedHitIDs ? "fixed" : "unexpected")")
        log("SPINON_C131_FIXTURE_SUMMARY status=\(passed ? "ok" : "error") boxes=13 frames=\(frames.count) hit_samples=\(samples.count) hit_matches=\(matched.count)")
        return passed
    }

    private func logC131StackingDescendantClipFixtureReport(_ report: String) -> Bool {
        guard report.hasPrefix("status=0 "), report.contains(" layout=ready "),
              reportField(report, "boxes=") == "13",
              reportField(report, "render_viewport_css_px=") == "540x128",
              let frameMarker = report.range(of: "node_frames_css_px=["),
              let frameEnd = report[frameMarker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C131_STACKING_CLIP_SUMMARY status=error stage=runtime-report \(runtimeStatusSummary(report))")
            return false
        }
        let frames = report[frameMarker.upperBound..<frameEnd].split(separator: ";")
        let expectedGeometry: [[Double]] = [
            [0, 0, 540, 128],
            [20, 28, 120, 72], [76, 40, 80, 48], [108, 40, 72, 48], [148, 40, 16, 48],
            [190, 28, 120, 72], [246, 40, 80, 48], [278, 40, 72, 48], [318, 40, 16, 48],
            [360, 28, 120, 72], [416, 40, 80, 48], [448, 40, 72, 48], [488, 40, 16, 48],
        ]
        var valid = frames.count == expectedGeometry.count
        var nodeIDs = Set<String>()
        for (index, frame) in frames.enumerated() {
            guard index < expectedGeometry.count else {
                valid = false
                continue
            }
            let fields = frame.split(separator: ",")
            guard fields.count == 5, fields[0].hasPrefix("\(index):node=") else {
                valid = false
                continue
            }
            let nodeID = String(fields[0].dropFirst("\(index):node=".count))
            valid = valid && !nodeID.isEmpty && nodeIDs.insert(nodeID).inserted
            let expectedKeys = ["x", "y", "width", "height"]
            for fieldIndex in 1..<fields.count {
                let pair = fields[fieldIndex].split(separator: "=", maxSplits: 1)
                guard pair.count == 2,
                      String(pair[0]) == expectedKeys[fieldIndex - 1],
                      let actual = Double(pair[1]), actual.isFinite,
                      abs(actual - expectedGeometry[index][fieldIndex - 1]) <= 0.5 else {
                    valid = false
                    continue
                }
            }
            log("SPINON_C131_STACKING_CLIP_NODE_FRAME \(frame)")
        }
        guard valid, nodeIDs.count == 13,
              let sampleMarker = report.range(of: "hit_test_samples_json="),
              let sampleStart = report[sampleMarker.upperBound...].firstIndex(of: "["),
              let sampleEnd = closingJSONArrayIndex(in: report, startingAt: sampleStart),
              let sampleData = String(report[sampleStart...sampleEnd]).data(using: .utf8),
              let samples = try? JSONSerialization.jsonObject(with: sampleData) as? [[String: Any]],
              samples.count == 18 else {
            log("SPINON_C131_STACKING_CLIP_SUMMARY status=error stage=frame-or-hit-report frames=\(frames.count)")
            return false
        }
        var seenIDs = Set<String>()
        var seenPixels = Set<String>()
        var matched = 0
        for (index, sample) in samples.enumerated() {
            let caseID = Self.c131StackingClipCases[index / Self.c131StackingClipRoles.count]
            let role = Self.c131StackingClipRoles[index % Self.c131StackingClipRoles.count]
            let id = "\(caseID):\(role)"
            let x = (sample["xCssPx"] as? NSNumber)?.doubleValue ?? .nan
            let y = (sample["yCssPx"] as? NSNumber)?.doubleValue ?? .nan
            let pixelX = sample["pixelX"] as? Int ?? -1
            let pixelY = sample["pixelY"] as? Int ?? -1
            let rgba = (sample["rgba"] as? [NSNumber])?.map(\.intValue) ?? []
            let pixel = "\(pixelX),\(pixelY)"
            let expectedCoordinate = Self.c131StackingClipPixelCoordinates[index]
            let sampleMatches = sample["id"] as? String == id
                && sample["caseId"] as? String == caseID
                && sample["role"] as? String == role
                && sample["elementId"] as? String == Self.c131StackingClipExpectedHitIDs[index]
                && sample["nodeIndex"] as? Int == Self.c131StackingClipExpectedNodeIndices[index]
                && sample["status"] as? String == "matched"
                && x == Double(expectedCoordinate.0) + 0.5
                && y == Double(expectedCoordinate.1) + 0.5
                && pixelX == expectedCoordinate.0 && pixelY == expectedCoordinate.1
                && rgba == Self.c131StackingClipExpectedRGBA[index]
                && seenIDs.insert(id).inserted && seenPixels.insert(pixel).inserted
            if sampleMatches { matched += 1 } else { valid = false }
            log("SPINON_C131_STACKING_CLIP_HIT_TEST id=\(id) x=\(x) y=\(y) element=\(sample["elementId"] ?? "missing") node_index=\(sample["nodeIndex"] ?? "missing") rgba=\(rgba) status=\(sampleMatches ? "matched" : "error")")
        }
        valid = valid && matched == 18 && seenIDs.count == 18 && seenPixels.count == 18
        guard valid, logC131StackingFullViewportHitIdentityGridSummary(report) else {
            log("SPINON_C131_STACKING_CLIP_SUMMARY status=error stage=full-viewport-grid frames=\(frames.count)")
            return false
        }
        var runtimeRevisionFields: [String: String] = [:]
        for field in ["generation", "document_revision", "render_tree_revision", "style_revision"] {
            let value = reportField(report, "\(field)=")
            guard UInt64(value) != nil else {
                log("SPINON_C131_STACKING_CLIP_FIXTURE_SUMMARY status=error stage=revision-identity field=\(field)")
                return false
            }
            runtimeRevisionFields[field] = value
        }
        c131StackingClipExpectedRevisionFields = runtimeRevisionFields
        log("SPINON_C131_STACKING_CLIP_RUNTIME_IDENTITY run_id=\(c131StackingDescendantClipRunID) source=evaluation \(c131StackingClipRuntimeRevisionSummary())")
        let generation = reportField(report, "generation=")
        log("SPINON_C131_STACKING_CLIP_HIT_TEST_SUMMARY status=ok run_id=\(c131StackingDescendantClipRunID) generation=\(generation) matched=\(matched)/18 unique_ids=\(seenIDs.count) unique_pixels=\(seenPixels.count)")
        log("SPINON_C131_STACKING_CLIP_FIXTURE_SUMMARY status=ok boxes=13 frames=\(frames.count) hit_samples=18 hit_matches=\(matched) hit_grid_points=69120 hit_grid_matches=69120 edge_controls=11/11 viewport=540x128")
        c131CompactHitIdentityGridStatusMessage = "완료 · C13.1 stacking clip Chrome hit identity 69,120/69,120 · 18/18"
        postStatus(c131CompactHitIdentityGridStatusMessage ?? "C13.1 stacking clip hit identity 검증 완료")
        return true
    }

    private func c131StackingClipRevisionMismatch(in report: String) -> String? {
        guard report.hasPrefix("status=0 "),
              reportField(report, "boxes=") == "13",
              reportField(report, "root_css_px=") == "540x128" else {
            return "draw-shape"
        }
        let fields = ["generation", "document_revision", "render_tree_revision", "style_revision"]
        guard c131StackingClipExpectedRevisionFields.count == fields.count else {
            return "missing-evaluation-revisions"
        }
        for field in fields {
            let actual = reportField(report, "\(field)=")
            guard UInt64(actual) != nil,
                  actual == c131StackingClipExpectedRevisionFields[field] else {
                return field
            }
        }
        return nil
    }

    private func c131StackingClipRuntimeRevisionSummary() -> String {
        ["generation", "document_revision", "render_tree_revision", "style_revision"]
            .map { field in "\(field)=\(c131StackingClipExpectedRevisionFields[field] ?? "missing")" }
            .joined(separator: " ")
    }

    private func closingJSONArrayIndex(in text: String, startingAt start: String.Index) -> String.Index? {
        var depth = 0
        var inString = false
        var escaped = false
        for index in text[start...].indices {
            let character = text[index]
            if inString {
                if escaped { escaped = false }
                else if character == "\\" { escaped = true }
                else if character == "\"" { inString = false }
                continue
            }
            if character == "\"" { inString = true }
            else if character == "[" { depth += 1 }
            else if character == "]" {
                depth -= 1
                if depth == 0 { return index }
            }
        }
        return nil
    }

    private func logC131CompactSurfaceMatrixFixtureReport(_ report: String, runID: String? = nil) -> Bool {
        let expectedCaseIDs = [
            "visible", "clip-x", "clip-y", "clip-both", "clip-margin-padding",
            "clip-margin-content", "clip-margin-border", "clip-margin-hidden",
            "mixed-visible-hidden", "mixed-clip-auto", "positioned-contained",
            "positioned-escape", "nested-clip",
        ]
        let expectedRoles = ["paint", "target-beyond-frame", "x-overflow", "y-overflow"]
        let expectedSampleIDs = expectedCaseIDs.flatMap { caseID in
            expectedRoles.map { "\(caseID):\($0)" }
        }
        guard report.hasPrefix("status=0 "), report.contains(" layout=ready "),
              reportField(report, "boxes=") == "41",
              reportField(report, "render_viewport_css_px=") == "300x100",
              let frameMarker = report.range(of: "node_frames_css_px=["),
              let frameEnd = report[frameMarker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C131_COMPACT_FIXTURE_SUMMARY status=error stage=runtime-report \(runtimeStatusSummary(report))")
            return false
        }
        let runtimeGeneration = reportField(report, "generation=")
        let runContext = runID.map {
            " run_id=\($0) runtime_generation=\(runtimeGeneration.isEmpty ? "missing" : runtimeGeneration)"
        } ?? ""
        let frames = report[frameMarker.upperBound..<frameEnd].split(separator: ";")
        guard frames.count == 41 else {
            log("SPINON_C131_COMPACT_GEOMETRY_SUMMARY status=error frames=\(frames.count) expected=41")
            return false
        }
        for frame in frames {
            log("SPINON_C131_COMPACT_NODE_FRAME \(frame)\(runContext)")
        }
        log("SPINON_C131_COMPACT_GEOMETRY_SUMMARY status=ok frames=41 viewport=300x100\(runContext)")

        guard let sampleMarker = report.range(of: "hit_test_samples_json="),
              let sampleStart = report[sampleMarker.upperBound...].firstIndex(of: "["),
              let sampleEnd = report[report.index(after: sampleStart)...].firstIndex(of: "]"),
              let sampleData = String(report[sampleStart...sampleEnd]).data(using: .utf8),
              let samples = try? JSONSerialization.jsonObject(with: sampleData) as? [[String: Any]] else {
            log("SPINON_C131_COMPACT_FIXTURE_SUMMARY status=error stage=hit-test-json")
            return false
        }
        var valid = samples.count == 52
        var seenIDs = Set<String>()
        var seenCoordinates = Set<String>()
        var matched = 0
        for (index, sample) in samples.enumerated() {
            let actualID = sample["id"] as? String ?? "missing"
            let expectedID = index < expectedSampleIDs.count ? expectedSampleIDs[index] : "unexpected"
            let expectedCase = index / expectedRoles.count < expectedCaseIDs.count
                ? expectedCaseIDs[index / expectedRoles.count] : "unexpected"
            let expectedRole = expectedRoles[index % expectedRoles.count]
            let caseID = sample["caseId"] as? String ?? "missing"
            let role = sample["role"] as? String ?? "missing"
            let elementID = sample["elementId"] as? String ?? ""
            let nodeIndex = sample["nodeIndex"] as? Int ?? -1
            let status = sample["status"] as? String ?? "missing"
            let xNumber = sample["xCssPx"] as? NSNumber
            let yNumber = sample["yCssPx"] as? NSNumber
            let x = xNumber?.doubleValue ?? .nan
            let y = yNumber?.doubleValue ?? .nan
            let rgbaText = sample["rgba"] as? String ?? ""
            let rgba = rgbaText.split(separator: ",", omittingEmptySubsequences: false)
            let validRGBA = rgba.count == 4 && rgba.allSatisfy {
                guard let value = Int($0) else { return false }
                return (0...255).contains(value)
            }
            let validCoordinate = x.isFinite && y.isFinite && x.rounded() == x && y.rounded() == y
                && (0..<300).contains(Int(x)) && (0..<100).contains(Int(y))
            let coordinate = "\(x),\(y)"
            let sampleMatches = actualID == expectedID && caseID == expectedCase
                && role == expectedRole && status == "matched" && !elementID.isEmpty
                && nodeIndex >= 0 && validCoordinate && validRGBA
                && seenIDs.insert(actualID).inserted && seenCoordinates.insert(coordinate).inserted
            if status == "matched" { matched += 1 }
            if !sampleMatches { valid = false }
            log("SPINON_C131_COMPACT_HIT_TEST id=\(actualID) x=\(x) y=\(y) element=\(elementID) node_index=\(nodeIndex) rgba=\(rgbaText) status=\(status)\(runContext)")
        }
        valid = valid && seenIDs.count == 52 && seenCoordinates.count == 52 && matched == 52
        log("SPINON_C131_COMPACT_HIT_TEST_SUMMARY status=\(valid ? "ok" : "error") matched=\(matched) total=\(samples.count) ids=\(seenIDs.count == 52 ? "fixed" : "unexpected")\(runContext)")
        log("SPINON_C131_COMPACT_FIXTURE_SUMMARY status=\(valid ? "ok" : "error") boxes=41 frames=\(frames.count) hit_samples=\(samples.count) hit_matches=\(matched)\(runContext)")
        return valid
    }

    private func logC131CompactHitIdentityGridSummary(_ report: String) -> Bool {
        do {
            let summary = try C131HitIdentityGridSummaryValidator.parse(report)
            let generation = reportField(report, "generation=")
            guard !generation.isEmpty else {
                c131CompactHitIdentityGridStatusMessage = "실패 · C13.1 Chrome hit identity 30,000점 검증 불일치"
                log("SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=error stage=missing-generation run_id=\(c131CompactSurfaceMatrixRunID)")
                return false
            }

            let status = summary.isMatched ? "matched" : "mismatch"
            log("SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=\(status) run_id=\(c131CompactSurfaceMatrixRunID) runtime_generation=\(generation) points=\(summary.pointCount) matched=\(summary.matchedCount) mismatches=\(summary.mismatchCount) preview=\(summary.mismatchPreview.count) expected_identity_sha256=\(summary.expectedIdentitySha256) identity_sha256=\(summary.actualIdentitySha256) oracle_sha256=\(summary.gridOracleSha256) observation_sha256=\(summary.chromeObservationSha256)")
            for (index, mismatch) in summary.mismatchPreview.enumerated() {
                log("SPINON_C131_COMPACT_HIT_IDENTITY_GRID_MISMATCH index=\(index) x=\(mismatch.xCssPx) y=\(mismatch.yCssPx) expected=\(mismatch.expectedNodeId) actual=\(mismatch.actualNodeId ?? "null")")
            }
            if summary.isMatched {
                c131CompactHitIdentityGridStatusMessage = "완료 · C13.1 Chrome hit identity 30,000/30,000 · 52/52"
                postStatus(c131CompactHitIdentityGridStatusMessage ?? "C13.1 hit identity 검증 완료")
            } else {
                c131CompactHitIdentityGridStatusMessage = "실패 · C13.1 Chrome hit identity 30,000점 불일치"
            }
            return summary.isMatched
        } catch let error as C131HitIdentityGridSummaryValidator.ValidationError {
            c131CompactHitIdentityGridStatusMessage = "실패 · C13.1 Chrome hit identity 30,000점 검증 불일치"
            log("SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=error run_id=\(c131CompactSurfaceMatrixRunID) stage=parser code=\(error)")
            return false
        } catch {
            c131CompactHitIdentityGridStatusMessage = "실패 · C13.1 Chrome hit identity 30,000점 검증 불일치"
            log("SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=error run_id=\(c131CompactSurfaceMatrixRunID) stage=parser code=unknown")
            return false
        }
    }

    private func logC131StackingFullViewportHitIdentityGridSummary(_ report: String) -> Bool {
        do {
            let summary = try C131StackingFullViewportHitGridSummaryValidator.parse(report)
            let generation = reportField(report, "generation=")
            guard UInt64(generation) != nil else {
                log("SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY status=error stage=missing-generation run_id=\(c131StackingDescendantClipRunID)")
                return false
            }
            log("SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY status=\(summary.isMatched ? "matched" : "mismatch") run_id=\(c131StackingDescendantClipRunID) runtime_generation=\(generation) points=\(summary.pointCount) matched=\(summary.matchedCount) mismatches=\(summary.mismatchCount) edge_controls=\(summary.edgeControlsMatched)/\(summary.edgeControlCount) preview=\(summary.mismatchPreview.count) expected_identity_sha256=\(summary.expectedIdentitySha256) identity_sha256=\(summary.actualIdentitySha256) oracle_sha256=\(summary.oracleSha256)")
            for (index, mismatch) in summary.mismatchPreview.enumerated() {
                log("SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_MISMATCH index=\(index) x=\(mismatch.xCssPx) y=\(mismatch.yCssPx) expected=\(mismatch.expectedNodeId) actual=\(mismatch.actualNodeId ?? "null")")
            }
            if summary.isMatched {
                return true
            }
            c131CompactHitIdentityGridStatusMessage = "실패 · C13.1 stacking Chrome hit identity 69,120점 불일치"
            return false
        } catch let error as C131StackingFullViewportHitGridSummaryValidator.ValidationError {
            c131CompactHitIdentityGridStatusMessage = "실패 · C13.1 stacking Chrome hit identity 69,120점 검증 불일치"
            log("SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY status=error run_id=\(c131StackingDescendantClipRunID) stage=parser code=\(error)")
            return false
        } catch {
            c131CompactHitIdentityGridStatusMessage = "실패 · C13.1 stacking Chrome hit identity 69,120점 검증 불일치"
            log("SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY status=error run_id=\(c131StackingDescendantClipRunID) stage=parser code=unknown")
            return false
        }
    }

    private func logC131BorderFixtureReport(_ report: String) -> Bool {
        let expectedSampleIDs = [
            "static-inside-clip", "static-border-strip", "static-outside-border",
            "positioned-inside-clip", "positioned-border-strip", "positioned-outside-border",
        ]
        let expectedTargets = [
            "static-target", "static-clip", "root", "positioned-target",
            "positioned-target", "root",
        ]
        let expectedPoints: [(Double, Double)] = [
            (125, 40), (128, 40), (131, 40), (315, 40), (318, 40), (321, 40),
        ]
        let expectedFrames: [(Double, Double, Double, Double)] = [
            (0, 0, 380, 120), (10, 10, 120, 96), (24, 24, 140, 120),
            (200, 10, 120, 96), (214, 24, 140, 120),
        ]
        guard report.hasPrefix("status=0 "),
              report.contains(" layout=ready "),
              reportField(report, "boxes=") == "5",
              let frameMarker = report.range(of: "node_frames_css_px=["),
              let frameEnd = report[frameMarker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C131_BORDER_FIXTURE_SUMMARY status=error stage=runtime-report \(runtimeStatusSummary(report))")
            return false
        }
        let frames = report[frameMarker.upperBound..<frameEnd].split(separator: ";")
        for frame in frames {
            log("SPINON_C131_BORDER_NODE_FRAME \(frame)")
        }
        let geometryMatches = frames.count == expectedFrames.count
            && frames.enumerated().allSatisfy { index, frame in
                var coordinates: [String: Double] = [:]
                for field in frame.split(separator: ",") {
                    let pair = field.split(separator: "=", maxSplits: 1)
                    guard pair.count == 2,
                          let value = Double(pair[1]),
                          ["x", "y", "width", "height"].contains(String(pair[0])) else {
                        continue
                    }
                    coordinates[String(pair[0])] = value
                }
                let expected = expectedFrames[index]
                return coordinates["x"] == expected.0
                    && coordinates["y"] == expected.1
                    && coordinates["width"] == expected.2
                    && coordinates["height"] == expected.3
            }
        log("SPINON_C131_BORDER_GEOMETRY_SUMMARY status=\(geometryMatches ? "ok" : "error") frames=\(frames.count)")

        guard let sampleMarker = report.range(of: "hit_test_samples_json="),
              let sampleStart = report[sampleMarker.upperBound...].firstIndex(of: "["),
              let sampleEnd = report[report.index(after: sampleStart)...].firstIndex(of: "]") else {
            log("SPINON_C131_BORDER_FIXTURE_SUMMARY status=error stage=hit-test-report")
            return false
        }
        let sampleJSON = String(report[sampleStart...sampleEnd])
        guard let sampleData = sampleJSON.data(using: .utf8),
              let samples = try? JSONSerialization.jsonObject(with: sampleData) as? [[String: Any]] else {
            log("SPINON_C131_BORDER_FIXTURE_SUMMARY status=error stage=hit-test-json")
            return false
        }
        let sampleIDs = samples.compactMap { $0["id"] as? String }
        let matched = samples.filter { $0["status"] as? String == "matched" }
        let samplesAgree = samples.count == expectedSampleIDs.count
            && samples.enumerated().allSatisfy { index, sample in
                (sample["id"] as? String) == expectedSampleIDs[index]
                    && (sample["elementId"] as? String) == expectedTargets[index]
                    && (sample["xCssPx"] as? Double) == expectedPoints[index].0
                    && (sample["yCssPx"] as? Double) == expectedPoints[index].1
                    && (sample["status"] as? String) == "matched"
            }
        for sample in samples {
            let id = sample["id"] as? String ?? "missing"
            let element = sample["elementId"] as? String ?? "null"
            let x = sample["xCssPx"] ?? "missing"
            let y = sample["yCssPx"] ?? "missing"
            let status = sample["status"] as? String ?? "missing"
            log("SPINON_C131_BORDER_HIT_TEST id=\(id) x=\(x) y=\(y) element=\(element) status=\(status)")
        }
        let passed = frames.count == 5 && geometryMatches && samplesAgree && matched.count == 6
            && sampleIDs == expectedSampleIDs
        log("SPINON_C131_BORDER_HIT_TEST_SUMMARY status=\(passed ? "ok" : "error") matched=\(matched.count) total=\(samples.count) ids=\(sampleIDs == expectedSampleIDs ? "fixed" : "unexpected")")
        log("SPINON_C131_BORDER_FIXTURE_SUMMARY status=\(passed ? "ok" : "error") boxes=5 frames=\(frames.count) hit_samples=\(samples.count) hit_matches=\(matched.count)")
        return passed
    }

    private func logC1031NodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1031_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1031_NODE_FRAME \(frame)")
        }
        log("SPINON_C1031_FRAME_SUMMARY frames=\(frames.count) marker=present")
    }

    private func logC121NodeFrames(_ state: String, _ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C121_FRAME_SUMMARY state=\(state) marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C121_NODE_FRAME state=\(state) \(frame)")
        }
        log("SPINON_C121_FRAME_SUMMARY state=\(state) frames=\(frames.count) marker=present")
    }

    private func logC122NodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C122_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C122_NODE_FRAME \(frame)")
        }
        log("SPINON_C122_FRAME_SUMMARY frames=\(frames.count) marker=present")
    }

    private func logC123NodeFrames(_ state: String, _ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C123_FRAME_SUMMARY state=\(state) marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C123_NODE_FRAME state=\(state) \(frame)")
        }
        log("SPINON_C123_FRAME_SUMMARY state=\(state) frames=\(frames.count) marker=present")
    }

    private func logC123FailureDetails(_ report: String) {
        let chunkSize = 600
        let characters = Array(report)
        let chunkCount = (characters.count + chunkSize - 1) / chunkSize
        for index in 0..<chunkCount {
            let start = index * chunkSize
            let end = min(characters.count, start + chunkSize)
            log("SPINON_C123_EVAL_DETAIL part=\(index + 1)/\(chunkCount) \(String(characters[start..<end]))")
        }
    }

    private func runC123EnvironmentMatrix(
        host: UInt64,
        dark: Bool,
        deviceScaleFactor: Float,
        initialEnvironmentRevision: UInt64
    ) -> Bool {
        let steps: [(String, Float, Float, Float)] = [
            ("matrix-360x800-dpr-2", 360, 800, 2),
            ("matrix-360x800-dpr-2.625", 360, 800, 2.625),
            ("matrix-360x800-dpr-3", 360, 800, 3),
            ("matrix-390x844-dpr-1", 390, 844, 1),
            ("matrix-390x844-dpr-2", 390, 844, 2),
            ("matrix-390x844-dpr-2.625", 390, 844, 2.625),
            ("matrix-390x844-dpr-3", 390, 844, 3),
            ("resize-start", 360, 800, 1),
            ("resize-outbound", 390, 844, 1),
            ("resize-noop", 390, 844, 1),
            ("resize-return", 360, 800, 1),
            ("device-final", 360, 800, deviceScaleFactor),
        ]
        var expectedRevision = initialEnvironmentRevision
        for (state, width, height, scale) in steps {
            if state != "resize-noop" { expectedRevision += 1 }
            let report = SpinonRunner.setRuntimeGpuEnvironment(
                host, width: width, height: height, scale: scale, dark: dark
            )
            guard let report, report.hasPrefix("status=0 ") else {
                log("SPINON_C123_MATRIX_FAILURE state=\(state) report=\(report ?? "missing")")
                return false
            }
            let actualRevision = reportField(report, "environment_revision=")
            guard actualRevision == String(expectedRevision) else {
                log("SPINON_C123_MATRIX_FAILURE state=\(state) expected_revision=\(expectedRevision) actual_revision=\(actualRevision)")
                return false
            }
            log("SPINON_C123_SUMMARY state=\(state) \(runtimeStatusSummary(report))")
            logC123NodeFrames(state, report)
        }
        return true
    }

    private func logC1035NodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1035_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1035_NODE_FRAME \(frame)")
        }
        log("SPINON_C1035_FRAME_SUMMARY frames=\(frames.count) marker=present")
    }

    private func logC111GridNodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C111_GRID_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C111_GRID_NODE_FRAME_SWIFT \(frame)")
        }
        log("SPINON_C111_GRID_FRAME_SUMMARY count=\(frames.count) marker=present")
    }

    private func logC1121GridNodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1121_GRID_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1121_GRID_NODE_FRAME_RUN run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) frame=\(frame)")
        }
        log("SPINON_C1121_GRID_FRAME_SUMMARY run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) count=\(frames.count) marker=present")
    }

    private func logC1132GridNamedLineFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1132_GRID_FRAME_SUMMARY run_id=\(c1132GridNamedLinesRunID) marker=missing")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1132_GRID_NODE_FRAME_RUN run_id=\(c1132GridNamedLinesRunID) scene_id=\(c1132GridNamedLineSceneID) frame=\(frame)")
        }
        log("SPINON_C1132_GRID_FRAME_SUMMARY run_id=\(c1132GridNamedLinesRunID) scene_id=\(c1132GridNamedLineSceneID) count=\(frames.count) marker=present")
    }

    private func logC1133GridTemplateAreaFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1133_GRID_FRAME_SUMMARY run_id=\(c1133GridTemplateAreasRunID) marker=missing")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1133_GRID_NODE_FRAME_RUN run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasSceneID) frame=\(frame)")
        }
        log("SPINON_C1133_GRID_FRAME_SUMMARY run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasSceneID) count=\(frames.count) marker=present")
        let paintMarker = "paint_node_order=["
        guard let paintStart = report.range(of: paintMarker),
              let paintEnd = report[paintStart.upperBound...].firstIndex(of: "]"),
              report.distance(from: paintStart.lowerBound, to: paintEnd) <= 512 else {
            log("SPINON_C1133_GRID_PAINT_ORDER_RUN run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasSceneID) marker=missing-or-too-large")
            return
        }
        log("SPINON_C1133_GRID_PAINT_ORDER_RUN run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasSceneID) \(report[paintStart.lowerBound...paintEnd])")
    }

    private func logC114GridNodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C114_GRID_FRAME_SUMMARY run_id=\(c114GridTrackSizingRunID) marker=missing")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C114_GRID_NODE_FRAME \(frame)")
        }
        log("SPINON_C114_GRID_FRAME_SUMMARY run_id=\(c114GridTrackSizingRunID) count=\(frames.count) marker=present")
    }

    private func logGridAutoPlacementReadySummaryIfNeeded(
        _ report: String,
        profile: String,
        runID: String,
        sceneID: String
    ) {
        guard report.hasPrefix("status=0 "), report.contains(" layout=ready ") else { return }
        stateLock.lock()
        let shouldLog: Bool
        if profile == "C1123" {
            shouldLog = !c1123GridRuntimeSummaryEmitted
            c1123GridRuntimeSummaryEmitted = true
        } else {
            shouldLog = !c1122GridRuntimeSummaryEmitted
            c1122GridRuntimeSummaryEmitted = true
        }
        stateLock.unlock()
        guard shouldLog else { return }
        log("SPINON_\(profile)_GRID_SUMMARY run_id=\(runID) scene_id=\(sceneID) \(runtimeStatusSummary(report))")
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_\(profile)_GRID_FRAME_SUMMARY run_id=\(runID) scene_id=\(sceneID) marker=missing")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_\(profile)_GRID_NODE_FRAME_RUN run_id=\(runID) scene_id=\(sceneID) frame=\(frame)")
        }
        log("SPINON_\(profile)_GRID_FRAME_SUMMARY run_id=\(runID) scene_id=\(sceneID) count=\(frames.count) marker=present")
        let paintMarker = "paint_node_order=["
        guard let start = report.range(of: paintMarker),
              let end = report[start.upperBound...].firstIndex(of: "]"),
              report.distance(from: start.lowerBound, to: end) <= 512 else { return }
        log("SPINON_\(profile)_GRID_PAINT_ORDER run_id=\(runID) scene_id=\(sceneID) \(report[start.lowerBound...end])")
    }

    private func logC1121GridPaintOrder(_ report: String) {
        let marker = "paint_node_order=["
        guard let start = report.range(of: marker),
              let end = report[start.upperBound...].firstIndex(of: "]"),
              report.distance(from: start.lowerBound, to: end) <= 512 else {
            log("SPINON_C1121_GRID_PAINT_ORDER run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) marker=missing-or-too-large")
            return
        }
        log("SPINON_C1121_GRID_PAINT_ORDER run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) \(report[start.lowerBound...end])")
    }

    private func logC1121GridReadySummaryIfNeeded(_ report: String) {
        guard report.hasPrefix("status=0 "), report.contains(" layout=ready ") else { return }
        stateLock.lock()
        let shouldLog = !c1121GridRuntimeSummaryEmitted
        c1121GridRuntimeSummaryEmitted = true
        stateLock.unlock()
        guard shouldLog else { return }
        log("SPINON_C1121_GRID_SUMMARY run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) \(runtimeStatusSummary(report))")
        logC1121GridNodeFrames(report)
        logC1121GridPaintOrder(report)
    }

    private func evalC121PositionState(
        host: UInt64,
        state: UInt32
    ) -> (report: String, attempts: Int) {
        var report = ""
        for attempt in 1...5 {
            report = SpinonRunner.evalRuntimeGpuC12_1PositionState(host, state: state)
            let superseded = report.hasPrefix("status=-12 ")
                && report.contains("op=eval status=0 ")
            if !superseded || attempt == 5 {
                return (report, attempt)
            }
            Thread.sleep(forTimeInterval: 0.025)
        }
        return (report, 5)
    }

    private func logC1032NodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1032_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1032_NODE_FRAME \(frame)")
        }
        log("SPINON_C1032_FRAME_SUMMARY frames=\(frames.count) marker=present")
    }

    private func logC1033NodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1033_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1033_NODE_FRAME \(frame)")
        }
        log("SPINON_C1033_FRAME_SUMMARY frames=\(frames.count) marker=present")
    }

    private func logC1033RuntimeSummary(_ report: String) {
        let fields = report.split(whereSeparator: \.isWhitespace)
        func field(_ name: String) -> String {
            fields.first(where: { $0.hasPrefix(name) }).map(String.init) ?? "\(name)missing"
        }
        log("SPINON_C1033_RUNTIME_SUMMARY \(field("status=")) \(field("layout=")) \(field("boxes=")) \(field("error="))")
    }

    private func logC1034NodeFrames(_ report: String) {
        guard let marker = report.range(of: "node_frames_css_px=["),
              let closing = report[marker.upperBound...].firstIndex(of: "]") else {
            log("SPINON_C1034_FRAME_SUMMARY marker=missing report_length=\(report.count)")
            return
        }
        let frames = report[marker.upperBound..<closing].split(separator: ";")
        for frame in frames {
            log("SPINON_C1034_NODE_FRAME \(frame)")
        }
        log("SPINON_C1034_FRAME_SUMMARY frames=\(frames.count) marker=present")
    }

    private func logC1034RuntimeSummary(_ report: String) {
        let fields = report.split(whereSeparator: \.isWhitespace)
        func field(_ name: String) -> String {
            fields.first(where: { $0.hasPrefix(name) }).map(String.init) ?? "\(name)missing"
        }
        log("SPINON_C1034_RUNTIME_SUMMARY \(field("status=")) \(field("layout=")) \(field("boxes=")) \(field("error="))")
    }

    @objc private func toggleSurfaceSize() {
        guard !isClosing else { return }
        expandedSurface.toggle()
        let width: CGFloat = expandedSurface ? 341 : 301
        let height: CGFloat = expandedSurface ? 128 : 100
        canvasWidthConstraint?.constant = width
        canvasHeightConstraint?.constant = height
        resizeButton.setTitle(
            "표면 크기 전환 · \(Int(width))×\(Int(height)) CSS px",
            for: .normal
        )
        log("SPINON_C0410_RESIZE_REQUEST css=\(Int(width))x\(Int(height))")
        view.setNeedsLayout()
        view.layoutIfNeeded()
    }

    private func createRootHtmlDefaultRuntimeHost() -> UInt64 {
        let pageScaleCase = rootHtmlPageScaleCase
        let resourceName = pageScaleCase == nil
            ? "root-html-default-rendering" : "root-html-page-scale-runtime"
        guard let url = Bundle.main.url(
            forResource: resourceName, withExtension: "html"
        ) else {
            log("SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_ASSET_ERROR missing=true")
            return 0
        }
        do {
            var source = try String(contentsOf: url, encoding: .utf8)
            if let pageScaleCase {
                let originalViewport = "content=\"width=device-width, initial-scale=1\""
                let replacementViewport: String
                switch pageScaleCase {
                case "half":
                    replacementViewport = "content=\"width=device-width, initial-scale=0.5\""
                case "double":
                    replacementViewport = "content=\"width=device-width, initial-scale=2\""
                case "fixed-width":
                    replacementViewport = "content=\"width=500, initial-scale=1\""
                default:
                    log("SPINON_C0412_ROOT_HTML_PAGE_SCALE_ARGUMENT_ERROR rejected=true case=\(pageScaleCase)")
                    return 0
                }
                guard source.components(separatedBy: originalViewport).count == 2 else {
                    log("SPINON_C0412_ROOT_HTML_PAGE_SCALE_ASSET_ERROR viewport_marker_count_invalid=true")
                    return 0
                }
                source = source.replacingOccurrences(
                    of: originalViewport, with: replacementViewport
                )
                log("SPINON_C0412_ROOT_HTML_PAGE_SCALE_SOURCE case=\(pageScaleCase) viewport=\(replacementViewport)")
            }
            let sourceDigest = SHA256.hash(data: Data(source.utf8))
                .map { String(format: "%02x", $0) }
                .joined()
            log("SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_SOURCE case=\(pageScaleCase ?? "default") bytes=\(source.utf8.count) sha256=\(sourceDigest)")
            return SpinonRunner.createRuntimeGpuHost(withRootHTML: source)
        } catch {
            log("SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_ASSET_ERROR reason=\(error)")
            return 0
        }
    }

    private func createRootHtmlApplicationHost() -> UInt64 {
        guard let htmlURL = Bundle.main.url(forResource: "index", withExtension: "html"),
              let appURL = Bundle.main.url(forResource: "root-app", withExtension: "js") else {
            log("SPINON_ROOT_HTML_DEFAULT_APP_ASSET_ERROR missing=index.html-or-root-app.js")
            return 0
        }
        do {
            let html = try String(contentsOf: htmlURL, encoding: .utf8)
            let appSource = try String(contentsOf: appURL, encoding: .utf8)
            guard !html.isEmpty, !appSource.isEmpty else {
                log("SPINON_ROOT_HTML_DEFAULT_APP_ASSET_ERROR empty=index.html-or-root-app.js")
                return 0
            }
            rootHtmlApplicationScript = appSource
            log("SPINON_ROOT_HTML_DEFAULT_APP_SOURCE html_bytes=\(html.utf8.count) script_bytes=\(appSource.utf8.count)")
            return SpinonRunner.createRuntimeGpuHost(withRootHTML: html)
        } catch {
            log("SPINON_ROOT_HTML_DEFAULT_APP_ASSET_ERROR reason=\(error)")
            return 0
        }
    }

    private func initializeRuntime() {
        guard !isClosing else { return }
        if rootHtmlDefaultRenderRequested {
            let pageScaleArguments = rootHtmlPageScaleArguments
            let validPageScaleCase = rootHtmlPageScaleCase.map {
                ["half", "double", "fixed-width"].contains($0)
            } ?? true
            if pageScaleArguments.count > 1 || !validPageScaleCase {
                log("SPINON_C0412_ROOT_HTML_PAGE_SCALE_ARGUMENT_ERROR rejected=true arguments=\(pageScaleArguments.joined(separator: ","))")
                postStatus("실패 · root HTML page scale case는 half, double, fixed-width 중 하나만 허용합니다")
                return
            }
            let conflicts = ProcessInfo.processInfo.arguments.filter {
                $0.hasPrefix("--spinon-c")
                    && $0 != "--spinon-c0412-root-html-default-render"
                    && $0 != "--spinon-c0410-runtime-gpu"
                    && !$0.hasPrefix("--spinon-c0412-page-scale-case=")
            }
            if !conflicts.isEmpty {
                log("SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_ARGUMENT_ERROR conflicting=\(conflicts.joined(separator: ","))")
                postStatus("실패 · root HTML 기본 렌더와 다른 CSS route를 함께 선택했습니다")
                return
            }
        } else if !rootHtmlPageScaleArguments.isEmpty {
            log("SPINON_C0412_ROOT_HTML_PAGE_SCALE_ARGUMENT_ERROR rejected=true reason=root-html-route-required")
            postStatus("실패 · page scale case에는 root HTML 기본 렌더 route가 필요합니다")
            return
        }
#if !SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST_1
        if c192BoxPaintOpacityArgumentRequested {
            log("SPINON_C192_ARGUMENT_ERROR rejected=true reason=fixture-feature-disabled")
            postStatus("실패 · C19.2 fixture-test feature를 켜서 빌드해야 합니다")
            return
        }
#endif
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
        if c133NativeWheelPreflightRequested && c133NativeWheelAdapterArgumentRequested {
            log("SPINON_C133_ARGUMENT_ERROR rejected=true reason=native-wheel-preflight-and-adapter-are-exclusive")
            postStatus("실패 · C13.3 native wheel preflight와 adapter route는 함께 사용할 수 없습니다")
            return
        }
        if (c133NativeWheelPreflightRequested || c133NativeWheelAdapterArgumentRequested)
            && !c133GestureFixtureRequested {
            log("SPINON_C133_ARGUMENT_ERROR rejected=true reason=native-wheel-route-requires-fixture-route")
            postStatus("실패 · C13.3 native wheel route에는 gesture fixture route가 필요합니다")
            return
        }
#endif
        if (c133TapProbeRequested || c133WheelProbeRequested)
            && !c133GestureFixtureRequested {
            log("SPINON_C133_ARGUMENT_ERROR rejected=true reason=probe-requires-fixture-route")
            postStatus("실패 · C13.3 probe에는 gesture fixture route가 필요합니다")
            return
        }
#if !SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
        if c133GestureFixtureRequested || c133TapProbeRequested || c133WheelProbeRequested {
            log("SPINON_C133_DISABLED rebuild with SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=1")
            postStatus("C13.3 gesture fixture-test 옵션을 켜서 빌드해야 합니다.")
            return
        }
#endif
        if c131CompactMetalPixelCentersRequested && c131FullPhysicalMetalPixelsRequested {
            log("SPINON_C131_METAL_PIXEL_ARGUMENT_ERROR reason=multiple-capture-modes")
            postStatus("실패 · C13.1 Metal pixel capture mode는 하나만 선택해야 합니다")
            return
        }
        if c131MetalPixelCaptureRequested
            && !(c131CompactHitIdentityGridProbeRequested
                || c131StackingDescendantClipProbeRequested) {
            log("SPINON_C131_METAL_PIXEL_ARGUMENT_ERROR reason=requires-fixed-hit-sample-route")
            postStatus("실패 · Metal pixel route에는 고정 C13.1 hit sample route가 필요합니다")
            return
        }
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
        if c133TapProbeRequested
            && (c133GesturePolicyScenarioArguments.count != 1
                || ![3, 4].contains(c133GesturePolicyScenarioID)) {
            log("SPINON_C133_ARGUMENT_ERROR rejected=true reason=tap-probe-requires-scenario-3-or-4")
            postStatus("실패 · C13.3 tap probe는 policy scenario 3 또는 4가 필요합니다")
            return
        }
        if c133WheelProbeRequested
            && (c133GesturePolicyScenarioArguments.count != 1
                || ![1, 2, 3].contains(c133GesturePolicyScenarioID)) {
            log("SPINON_C133_ARGUMENT_ERROR rejected=true reason=wheel-probe-requires-scenario-1-2-or-3")
            postStatus("실패 · C13.3 wheel probe는 policy scenario 1, 2 또는 3이 필요합니다")
            return
        }
        if (!c133GesturePolicyScenarioArguments.isEmpty && !c133GestureFixtureRequested)
            || (c133GestureFixtureRequested && c133GesturePolicyScenarioID < 0) {
            log("SPINON_C133_POLICY_SCENARIO_ARGUMENT_ERROR rejected=true")
            postStatus("실패 · C13.3 policy scenario ID는 fixture route에서 0~10으로 하나만 지정해야 합니다")
            return
        }
#endif
        if (c153GlyphPaintArgumentRequested || c153HostDocumentTextPaintArgumentRequested)
            && !c153GlyphPaintProbeRequested {
            log("SPINON_C153_ARGUMENT_ERROR rejected=true reason=fixture-feature-disabled")
            postStatus("실패 · C15.3 glyph paint fixture-test 옵션을 켜서 빌드해야 합니다")
            return
        }
        var selectedC131RouteFlags = [
            c153GlyphPaintProbeRequested,
            c192BoxPaintOpacityProbeRequested,
            c131OverflowClipProbeRequested,
            c131StackingDescendantClipProbeRequested,
            c131CompactSurfaceMatrixProbeRequested,
            c131CompactHitIdentityGridProbeRequested,
            c131BorderStripProbeRequested,
            c132ScrollFixtureRequested,
            c133GestureFixtureRequested,
        ]
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
        selectedC131RouteFlags.append(imagePaintProbeRequested)
#endif
        let selectedC131Routes = selectedC131RouteFlags.filter { $0 }.count
        if selectedC131Routes > 1 {
            log("SPINON_C0410_ARGUMENT_ERROR conflicting=multiple-fixture-routes")
            postStatus("실패 · CSS fixture route를 하나만 선택해야 합니다")
            return
        }
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
        let c133NativeWheelAdapterArguments: Set<String> = [
            "--spinon-c133-native-wheel-adapter",
        ]
#else
        let c133NativeWheelAdapterArguments: Set<String> = []
#endif
        if c131StackingDescendantClipStaleRevisionProbeRequested
            && !c131StackingDescendantClipProbeRequested {
            log("SPINON_C131_ARGUMENT_ERROR rejected=stale-revision-requires-stacking-route")
            postStatus("실패 · stale revision control에는 C13.1 stacking clip route가 필요합니다")
            return
        }
        if selectedC131Routes == 1,
           let conflictingArgument = ProcessInfo.processInfo.arguments.first(where: {
               $0.hasPrefix("--spinon-c")
                   && $0 != "--spinon-c131-overflow-clip"
                   && $0 != "--spinon-c131-stacking-descendant-clip"
                   && !(c131StackingDescendantClipProbeRequested
                       && $0 == "--spinon-c131-stacking-descendant-clip-stale-revision")
                   && $0 != "--spinon-c131-compact-surface-matrix"
                   && $0 != "--spinon-c131-compact-hit-identity-grid"
                   && $0 != "--spinon-c131-compact-metal-pixel-centers"
                   && $0 != "--spinon-c131-full-physical-metal-pixels"
                   && $0 != "--spinon-c131-overflow-clip-border"
                   && $0 != "--spinon-c132-scroll"
                   && $0 != "--spinon-c192-box-paint-opacity"
                   && $0 != "--spinon-c153-glyph-paint"
                   && $0 != "--spinon-c153-hostdocument-text-paint"
                   && !imagePaintRouteArguments.contains($0)
                   && $0 != "--spinon-c133-scroll-gesture"
                   && !(c133GestureFixtureRequested && $0 == "--spinon-c133-tap-probe")
                   && !(c133GestureFixtureRequested && $0 == "--spinon-c133-wheel-probe")
                   && !(c133GestureFixtureRequested
                       && $0 == "--spinon-c133-native-wheel-preflight")
                   && !(c133GestureFixtureRequested
                       && c133NativeWheelAdapterArguments.contains($0))
                   && !(c133GestureFixtureRequested
                       && $0.hasPrefix("--spinon-c133-policy-scenario-id="))
                   && $0 != "--spinon-c0410-runtime-gpu"
                   && !(c132ScrollFixtureRequested
                       && $0.hasPrefix("--spinon-c132-metal-surface-pixels="))
           }) {
            log("SPINON_C13_ARGUMENT_ERROR conflicting=\(conflictingArgument)")
            postStatus("실패 · C13 전용 route와 다른 CSS fixture를 함께 선택했습니다")
            return
        }
        if c1131GridRepeatProbeRequested
            && (c1131GridRepeatSceneArgument != c1131GridRepeatSceneID
                || c1131GridRepeatRunID.range(
                    of: "^[A-Za-z0-9_-]{1,80}$", options: .regularExpression) == nil
                || c1123GridAutoPlacementProbeRequested
                || c1122GridAutoPlacementProbeRequested
                || c1121GridPlacementProbeRequested) {
            log("SPINON_C1131_GRID_ARGUMENT_ERROR rejected=true scene_id=\(c1131GridRepeatSceneArgument)")
            postStatus("실패 · 알 수 없거나 중복 선택된 C11.3.1 Grid scene")
            return
        }
        if c1132GridNamedLinesProbeRequested
            && (c1132GridNamedLinesRunID.range(
                of: "^[A-Za-z0-9_-]{1,80}$", options: .regularExpression) == nil
                || c1131GridRepeatProbeRequested
                || c1123GridAutoPlacementProbeRequested
                || c1122GridAutoPlacementProbeRequested
                || c1121GridPlacementProbeRequested) {
            log("SPINON_C1132_GRID_ARGUMENT_ERROR rejected=true")
            postStatus("실패 · C11.3.2 Grid fixture 또는 profile 선택이 올바르지 않습니다")
            return
        }
        if c1133GridTemplateAreasProbeRequested
            && (c1133GridTemplateAreasRunID.range(
                of: "^[A-Za-z0-9_-]{1,80}$", options: .regularExpression) == nil
                || !c1133GridTemplateAreasSceneIDs.contains(c1133GridTemplateAreasSceneID)
                || c1131GridRepeatProbeRequested || c1132GridNamedLinesProbeRequested
                || c1123GridAutoPlacementProbeRequested
                || c1122GridAutoPlacementProbeRequested
                || c1121GridPlacementProbeRequested) {
            log("SPINON_C1133_GRID_ARGUMENT_ERROR rejected=true")
            postStatus("실패 · C11.3.3 Grid fixture 또는 profile 선택이 올바르지 않습니다")
            return
        }
        if c114GridTrackSizingProbeRequested
            && (c114GridTrackSizingRunID.range(
                of: "^[A-Za-z0-9_-]{1,80}$", options: .regularExpression) == nil
                || !c114GridTrackSizingSceneIDs.contains(c114GridTrackSizingSceneID)
                || c1131GridRepeatProbeRequested || c1132GridNamedLinesProbeRequested
                || c1133GridTemplateAreasProbeRequested || c1123GridAutoPlacementProbeRequested
                || c1122GridAutoPlacementProbeRequested || c1121GridPlacementProbeRequested
                || c11GridSceneArgument != nil) {
            log("SPINON_C114_GRID_ARGUMENT_ERROR rejected=true")
            postStatus("실패 · C11.4 Grid fixture 또는 profile 선택이 올바르지 않습니다")
            return
        }
        if c1123GridAutoPlacementProbeRequested
            && (!c1123GridAutoPlacementSceneIDs.contains(c1123GridSceneID)
                || c1123GridRunID.range(
                    of: "^[A-Za-z0-9_-]{1,80}$", options: .regularExpression) == nil
                || c1122GridAutoPlacementProbeRequested || c1121GridPlacementProbeRequested) {
            log("SPINON_C1123_GRID_ARGUMENT_ERROR rejected=true scene_id=\(c1123GridSceneID)")
            postStatus("실패 · 알 수 없거나 중복 선택된 C11.2.3 Grid scene")
            return
        }
        if c1122GridAutoPlacementProbeRequested
            && !c1122GridAutoPlacementSceneIDs.contains(c1122GridSceneID) {
            log("SPINON_C1122_GRID_ARGUMENT_ERROR rejected=true scene_id=\(c1122GridSceneID)")
            postStatus("실패 · 알 수 없는 C11.2.2 Grid scene")
            return
        }
        if c11GridFixtureRequested && !c1121GridPlacementProbeRequested
            && !c1122GridAutoPlacementProbeRequested
            && !c1123GridAutoPlacementProbeRequested
            && !c1131GridRepeatProbeRequested
            && !c1132GridNamedLinesProbeRequested
            && !c1133GridTemplateAreasProbeRequested
            && !c114GridTrackSizingProbeRequested
            && c11GridSceneIndex == nil {
            log("SPINON_C111_GRID_ARGUMENT_ERROR rejected=true")
            postStatus("실패 · 알 수 없는 C11.1 Grid scene")
            return
        }
        if c111MetalPixelArgumentRequested && c111MetalPixelRun == nil {
            let markerPrefix = c114GridTrackSizingProbeRequested ? "SPINON_C114"
                : c1133GridTemplateAreasProbeRequested ? "SPINON_C1133"
                : c1132GridNamedLinesProbeRequested ? "SPINON_C1132"
                : c1131GridRepeatProbeRequested ? "SPINON_C1131"
                : c1123GridAutoPlacementProbeRequested ? "SPINON_C1123"
                : c1122GridAutoPlacementProbeRequested ? "SPINON_C1122" : "SPINON_C111"
            let invalidSceneID = c114GridTrackSizingProbeRequested
                ? c114GridTrackSizingSceneID
                : c1133GridTemplateAreasProbeRequested
                ? c1133GridTemplateAreasSceneID
                : c1131GridRepeatProbeRequested
                ? c1131GridRepeatSceneArgument
                : c1132GridNamedLinesProbeRequested ? c1132GridNamedLineSceneID
                : c11GridSceneArgument ?? "unknown"
            log("\(markerPrefix)_METAL_PIXEL_ERROR run_id=\(c111MetalPixelArgumentRunID) scene_id=\(invalidSceneID) reason=invalid-run-scene-or-samples")
            postStatus("실패 · \(c11GridMetalPixelStatusName) Metal 표본 실행 인수·장면·표본이 올바르지 않습니다")
            return
        }
        if c132MetalPixelArgumentRequested
            && (c132MetalPixelRun == nil || !c132ScrollFixtureRequested
                || c131CompactSurfaceMatrixRouteRequested || c131BorderStripProbeRequested
                || c131OverflowClipProbeRequested || c111MetalPixelArgumentRequested
                || c124MetalPixelCase != nil) {
            log("SPINON_C132_METAL_PIXEL_ERROR rejected=true reason=invalid-run-or-fixture-selection")
            postStatus("실패 · C13.2 Metal 표면 표본 실행 인수 또는 fixture route가 올바르지 않습니다")
            return
        }
        if c124StackingProbeRequested && c124MetalPixelCase == nil
            && ProcessInfo.processInfo.arguments.contains(where: {
                $0.hasPrefix("--spinon-c124-metal-surface-readback=")
            }) {
            log("SPINON_C124_METAL_PIXEL status=error reason=invalid-case-arguments")
            postStatus("실패 · C12.4 Metal 표본 case 인수가 올바르지 않습니다")
            return
        }
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
        let c133GestureHostHandle = c133GestureFixtureRequested
            ? SpinonRunner.createRuntimeGpuHost(
                withC13_3GestureFixture: c133GesturePolicyScenarioID
            ) : 0
#else
        let c133GestureHostHandle: UInt64 = 0
#endif
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
        let imagePaintHost: UInt64?
        if !imagePaintProbeRequested {
            imagePaintHost = nil
        } else if imageAsyncReplacementProbeRequested {
#if SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST_1
            imagePaintHost = SpinonRunner.createRuntimeGpuHostWithC18_4AsyncImageFixture()
#else
            imagePaintHost = nil
#endif
        } else if imageObjectFitPositionProbeRequested {
#if SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST_1
            imagePaintHost = SpinonRunner.createRuntimeGpuHostWithC18_3ObjectFitPositionFixture()
#else
            imagePaintHost = nil
#endif
        } else {
#if SPINON_ENABLE_C18_2_ALPHA_IMAGE_FIXTURE_TEST_1
            if imageAlphaProbeRequested {
                imagePaintHost = SpinonRunner.createRuntimeGpuHostWithC18_2AlphaImageFixture()
            } else {
#if SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST_1
                imagePaintHost = SpinonRunner.createRuntimeGpuHostWithC18_1ImagePaintFixture()
#else
                imagePaintHost = nil
#endif
            }
#else
#if SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST_1
            imagePaintHost = SpinonRunner.createRuntimeGpuHostWithC18_1ImagePaintFixture()
#else
            imagePaintHost = nil
#endif
#endif
        }
#else
        let imagePaintHost: UInt64? = nil
#endif
        let handle: UInt64
#if SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST_1
        let c192Host: UInt64? = c192BoxPaintOpacityProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC19_2BoxPaintOpacityFixture() : nil
#else
        let c192Host: UInt64? = nil
#endif
#if SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST_1
        let c153Host: UInt64? = c153HostDocumentTextPaintArgumentRequested
            ? SpinonRunner.createRuntimeGpuHostWithC15_3HostDocumentTextPaintFixture()
            : c153GlyphPaintProbeRequested
                ? SpinonRunner.createRuntimeGpuHostWithC15_3GlyphPaintFixture() : nil
#else
        let c153Host: UInt64? = nil
#endif
        if let c153Host {
            handle = c153Host
        } else if let imagePaintHost {
            handle = imagePaintHost
        } else if let c192Host {
            handle = c192Host
        } else {
            handle = c131MetalPixelCaptureRequested
            ? c131StackingDescendantClipProbeRequested
                ? SpinonRunner.createRuntimeGpuHostWithC13_1StackingDescendantClipFixture()
                : SpinonRunner.createRuntimeGpuHostWithC13_1CompactSurfacePixelReadbackFixture()
            : c131StackingDescendantClipProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC13_1StackingDescendantClipFixture()
            : c131CompactSurfaceMatrixRouteRequested
            ? SpinonRunner.createRuntimeGpuHostWithC13_1CompactSurfaceMatrixFixture()
            : c131BorderStripProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC13_1OverflowClipBorderFixture()
            : c131OverflowClipProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC13_1OverflowClipFixture()
            : c132MetalPixelRun != nil
            ? SpinonRunner.createRuntimeGpuHostWithC13_2SurfacePixelReadbackFixture()
            : c133GestureFixtureRequested
            ? c133GestureHostHandle
            : c132ScrollFixtureRequested
            ? SpinonRunner.createRuntimeGpuHostWithC13_2ScrollFixture()
            : c114GridTrackSizingProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC11_4GridTrackSizingFixture()
            : c1133GridTemplateAreasProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC11_3_3GridTemplateAreasFixture()
            : c1132GridNamedLinesProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC11_3_2GridNamedLinesFixture()
            : c1131GridRepeatProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC11_3_1GridRepeatFixture()
            : c1123GridAutoPlacementProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC11_2_3GridAutoPlacementFixture()
            : c1122GridAutoPlacementProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC11_2_2GridAutoPlacementFixture()
            : c1121GridPlacementProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC11_2_1GridPlacementFixture()
            : c111MetalPixelRun != nil
            ? SpinonRunner.createRuntimeGpuHostWithC11_1GridSurfacePixelReadbackFixture()
            : c11GridFixtureRequested
            ? SpinonRunner.createRuntimeGpuHostWithC11_1GridFixture()
            : c124MetalPixelCase != nil
            ? SpinonRunner.createRuntimeGpuHostWithC12_4SurfaceReadbackFixture()
            : c1035PositionedFlexProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC10_3_5PositionedFlexFixture()
            : c123FixedPositionProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC12_3FixedFixture()
            : c124StackingProbeRequested || c124DepthSafetyProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC12_4StackingFixture()
            : c122AbsoluteBlockProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC12_2AbsoluteBlockFixture()
            : c121PositioningProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithC12_1PositionFixture()
            : blockFormattingProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithBlockFormattingFixture()
            : rootHtmlTextPaintProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithRootHtmlTextPaintFixture()
            : rootHtmlBlockPaintProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithRootHtmlBlockPaintFixture()
            : blockPaintProbeRequested
            ? SpinonRunner.createRuntimeGpuHostWithBlockPaintFixture()
            : defaultRootHtmlApplicationRequested
            ? createRootHtmlApplicationHost()
            : rootHtmlDefaultRenderRequested
            ? createRootHtmlDefaultRuntimeHost()
            : SpinonRunner.createRuntimeGpuHost(
                withRegisteredPropertiesFixture: registeredPropertiesProbeRequested
            )
        }
        guard handle != 0 else {
            postStatus("실패 · V8 runtime host를 만들지 못했습니다")
            return
        }
#if SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST_1
        if c192BoxPaintOpacityProbeRequested {
            do {
                c192MetalPixelCapture = try C192MetalSurfacePixelCapture(
                    runID: c192BoxPaintOpacityRunID
                )
                log("SPINON_C192_METAL_PIXEL_CAPTURE_BEGIN run_id=\(c192BoxPaintOpacityRunID) viewport=360x240 samples=16")
            } catch {
                SpinonRunner.freeRuntimeGpuHost(handle)
                log("SPINON_C192_METAL_PIXEL_CAPTURE_ERROR run_id=\(c192BoxPaintOpacityRunID) stage=artifact-init reason=\(error)")
                postStatus("실패 · C19.2 Metal pixel artifact를 준비하지 못했습니다")
                return
            }
        }
#endif
        if c131MetalPixelCaptureRequested {
            do {
                if c131StackingDescendantClipProbeRequested {
                    c131MetalPixelCapture = try C131MetalSurfacePixelCapture(
                        runID: c131StackingDescendantClipRunID,
                        viewportWidthCssPx: 540,
                        viewportHeightCssPx: 128,
                        sampleCoordinates: Self.c131StackingClipPixelCoordinates,
                        expectedRGBA: Self.c131StackingClipExpectedRGBA
                    )
                } else if c131FullPhysicalMetalPixelsRequested {
                    c131MetalPixelCapture = try C131MetalFullSurfacePixelCapture(
                        runID: c131MetalCaptureRunID
                    )
                } else {
                    c131MetalPixelCapture = try C131MetalSurfacePixelCapture(
                        runID: c131MetalCaptureRunID
                    )
                }
                if let capture = c131MetalPixelCapture {
                    let mode = c131StackingDescendantClipProbeRequested
                        ? "fixed-chrome-samples" : c131FullPhysicalMetalPixelsRequested
                        ? "full-physical-surface" : "css-pixel-centers"
                    let viewport = c131StackingDescendantClipProbeRequested
                        ? "540x128" : "300x100"
                    log("SPINON_C131_METAL_PIXEL_CAPTURE_BEGIN run_id=\(c131MetalCaptureRunID) mode=\(mode) viewport=\(viewport) samples=\(capture.expectedSampleCount) batch_size=64 batch_count=\(capture.expectedBatchCount)")
                }
            } catch {
                SpinonRunner.freeRuntimeGpuHost(handle)
                log("SPINON_C131_METAL_PIXEL_CAPTURE_ERROR run_id=\(c131MetalCaptureRunID) stage=artifact-init reason=\(error)")
                postStatus("실패 · C13.1 Metal pixel artifact를 준비하지 못했습니다")
                return
            }
        }
        log("SPINON_C0410_HOST_RETURNED handle=\(handle)")
        stateLock.lock()
        hostHandle = handle
        hostLifetime.store(handle)
        let shouldContinue = !closing
        stateLock.unlock()
        log("SPINON_C0410_HOST_STORED continue=\(shouldContinue)")
        guard shouldContinue else { return }

        let environmentInputs = currentEnvironment()
        let viewportWidth: Float = imagePaintProbeRequested
            ? imageAsyncReplacementProbeRequested || imageObjectFitPositionProbeRequested ? 224 : 80
            : c153HostDocumentTextPaintArgumentRequested ? 320
            : c153GlyphPaintProbeRequested || c192BoxPaintOpacityProbeRequested
                ? 360 : c131StackingDescendantClipProbeRequested
                ? 540 : c131CompactSurfaceMatrixRouteRequested
            ? 300 : c131BorderStripProbeRequested
            ? 400 : c132ScrollFixtureRequested || c133GestureFixtureRequested ? 400 : c131OverflowClipProbeRequested
            ? 320 : c122AbsoluteBlockProbeRequested
            || c123FixedPositionProbeRequested
            || c1122GridAutoPlacementProbeRequested
            || c1123GridAutoPlacementProbeRequested
            ? 360 : c114GridTrackSizingProbeRequested || c1131GridRepeatProbeRequested
                || fixedSizeCssFixtureRequested ? 320 : environmentInputs.width
        let viewportHeight: Float = imagePaintProbeRequested
            ? imageAsyncReplacementProbeRequested ? 88
            : imageObjectFitPositionProbeRequested ? 72 : 40
            : c153GlyphPaintProbeRequested || c192BoxPaintOpacityProbeRequested ? 240 : c131StackingDescendantClipProbeRequested
            ? 128 : c131CompactSurfaceMatrixRouteRequested
            ? 100 : c131BorderStripProbeRequested
            ? 160 : c132ScrollFixtureRequested || c133GestureFixtureRequested ? 240 : c131OverflowClipProbeRequested
            ? 240 : c122AbsoluteBlockProbeRequested
            || c123FixedPositionProbeRequested
            || c1122GridAutoPlacementProbeRequested
            || c1123GridAutoPlacementProbeRequested
            ? 800 : c114GridTrackSizingProbeRequested || c1133GridTemplateAreasProbeRequested
                || c1132GridNamedLinesProbeRequested || c1131GridRepeatProbeRequested
                || fixedSizeCssFixtureRequested ? 240 : environmentInputs.height
        log("SPINON_C0410_ENVIRONMENT_REQUEST width=\(viewportWidth) height=\(viewportHeight)")
        let environmentScale = rootHtmlTextPaintProbeRequested
            ? 1
            : rootHtmlBlockPaintProbeRequested || rootHtmlDefaultRenderRequested
                ? environmentInputs.scale
            : c153GlyphPaintProbeRequested || c192BoxPaintOpacityProbeRequested
                || c123FixedPositionProbeRequested || c1132GridNamedLinesProbeRequested
                || c1133GridTemplateAreasProbeRequested || c114GridTrackSizingProbeRequested
                || c132ScrollFixtureRequested
                || c133GestureFixtureRequested || imagePaintProbeRequested
                ? 1 : environmentInputs.scale
        let environment = SpinonRunner.setRuntimeGpuEnvironment(
            handle,
            width: defaultRootHtmlApplicationRequested || rootHtmlBlockPaintProbeRequested
                || rootHtmlTextPaintProbeRequested
                || rootHtmlDefaultRenderRequested
                ? viewportWidth * environmentScale : viewportWidth,
            height: defaultRootHtmlApplicationRequested || rootHtmlBlockPaintProbeRequested
                || rootHtmlTextPaintProbeRequested
                || rootHtmlDefaultRenderRequested
                ? viewportHeight * environmentScale : viewportHeight,
            scale: environmentScale,
            dark: environmentInputs.dark
        )
        guard environment?.hasPrefix("status=0 ") == true else {
            postStatus("실패 · \(environment ?? "환경 보고 없음")")
            return
        }
        log("SPINON_C0410_ENVIRONMENT \(environment ?? "")")

#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
        if c133GestureFixtureRequested {
            stateLock.lock()
            c133ConfiguredRuntimeEnvironment = C133RuntimeEnvironment(
                width: viewportWidth,
                height: viewportHeight,
                scale: 1,
                dark: environmentInputs.dark
            )
            stateLock.unlock()
        }
#endif
        var result: String? = nil
#if SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST_1
        if c153GlyphPaintProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC15_3GlyphPaintFixture(handle)
        }
#endif
#if SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST_1
        if c192BoxPaintOpacityProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC19_2BoxPaintOpacityFixture(handle)
        }
#endif
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
        if result == nil && imagePaintProbeRequested {
            if imageAsyncReplacementProbeRequested {
#if SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST_1
                result = SpinonRunner.evalRuntimeGpuC18_4AsyncImageFixture(handle)
#else
                result = "status=-90 C18.4 feature-disabled"
#endif
            } else if imageObjectFitPositionProbeRequested {
#if SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST_1
                result = SpinonRunner.evalRuntimeGpuC18_3ObjectFitPositionFixture(handle)
#else
                result = "status=-90 C18.3 feature-disabled"
#endif
            } else {
#if SPINON_ENABLE_C18_2_ALPHA_IMAGE_FIXTURE_TEST_1
                if imageAlphaProbeRequested {
                    result = SpinonRunner.evalRuntimeGpuC18_2AlphaImageFixture(handle)
                } else {
#if SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST_1
                    result = SpinonRunner.evalRuntimeGpuC18_1ImagePaintFixture(handle)
#else
                    result = "status=-90 C18.1 feature-disabled"
#endif
                }
#else
#if SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST_1
                result = SpinonRunner.evalRuntimeGpuC18_1ImagePaintFixture(handle)
#else
                result = "status=-90 C18.1 feature-disabled"
#endif
#endif
            }
        }
#endif
#if SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST_1
        if c153GlyphPaintProbeRequested {
            let report = result ?? "status=-1 missing-report"
            let hitTest = reportField(report, "hit_test=")
            log("SPINON_C153_EVAL_RUN \(runtimeStatusSummary(report))")
            log("SPINON_C153_HITTEST \(hitTest)")
            let hostDocumentPaintMatches =
                report.contains("c15_3_hostdocument_text_paint=attached")
                && report.contains("layout_source=fixture_owned_host_document")
                && report.contains("owner_frame_css_px=0,0,15,10")
                && report.contains("overflow_css_px=5")
                && !reportField(report, "hostdocument_text_samples_css_px=").isEmpty
            let glyphPaintMatches = report.contains("c15_3_text=attached")
                && report.contains("glyphs=")
                && hitTest == "visible_owner;hidden_inside_owner;hidden_outside_excludes_hidden_owner"
                && reportField(report, "scroll_sequences=") == "3>6>0;18>7>0;4+5>2+3>0+0"
                && reportField(report, "scroll_hit_test=") == "horizontal_owner;vertical_owner;nested_owner"
            guard report.hasPrefix("status=0 "),
                  c153HostDocumentTextPaintArgumentRequested
                    ? hostDocumentPaintMatches : glyphPaintMatches else {
                postStatus("실패 · C15.3 glyph paint command가 runtime snapshot에 제출되지 않았습니다")
                return
            }
            c153GlyphPaintEvaluationReport = report
        }
#endif
#if SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST_1
        if c192BoxPaintOpacityProbeRequested {
            let report = result ?? "status=-1 missing-report"
            log("SPINON_C192_EVAL_RUN run_id=\(c192BoxPaintOpacityRunID) \(runtimeStatusSummary(report))")
            guard report.hasPrefix("status=0 "),
                  report.contains("layout=ready"),
                  report.contains("viewport_css_px=360x240"),
                  reportField(report, "boxes=") == "19" else {
                postStatus("실패 · C19.2 runtime fixture·geometry 검증 결과 불일치")
                return
            }
        }
#endif
        if result == nil {
        if defaultRootHtmlApplicationRequested {
            guard let source = rootHtmlApplicationScript, !source.isEmpty else {
                log("SPINON_ROOT_HTML_DEFAULT_APP_EVAL_ERROR reason=script-missing")
                postStatus("실패 · 기본 root-app.js를 읽지 못했습니다")
                return
            }
            result = SpinonRunner.evalRuntimeGpuHostSource(handle, source: source)
            log("SPINON_ROOT_HTML_DEFAULT_APP_EVAL \(result ?? "native runtime returned no report")")
            guard result?.hasPrefix("status=0 ") == true,
                  result?.contains("layout=ready") == true else {
                log("SPINON_ROOT_HTML_DEFAULT_APP_BOOT_ERROR reason=script-or-layout-failed")
                postStatus("실패 · root-app.js 실행 또는 HTML 장면 계산에 실패했습니다")
                return
            }
            log("SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass root_app_marker=ready \(runtimeStatusSummary(result ?? ""))")
        } else if c131StackingDescendantClipProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC13_1StackingDescendantClipFixture(handle)
        } else if c131CompactSurfaceMatrixRouteRequested {
            result = SpinonRunner.evalRuntimeGpuC13_1CompactSurfaceMatrixFixture(handle)
        } else if c131BorderStripProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC13_1OverflowClipBorderFixture(handle)
        } else if c131OverflowClipProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC13_1OverflowClipFixture(handle)
        } else if c132ScrollFixtureRequested {
            result = SpinonRunner.evalRuntimeGpuC13_2ScrollFixture(handle)
        } else if c133GestureFixtureRequested {
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
            result = SpinonRunner.evalRuntimeGpuC13_3GestureFixture(handle)
#endif
        } else if c114GridTrackSizingProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC11_4GridTrackSizingFixture(
                handle, sceneID: c114GridTrackSizingSceneID,
                runID: c114GridTrackSizingRunID)
        } else if c1133GridTemplateAreasProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC11_3_3GridTemplateAreasFixture(
                handle, sceneID: c1133GridTemplateAreasSceneID,
                runID: c1133GridTemplateAreasRunID)
        } else if c1132GridNamedLinesProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC11_3_2GridNamedLinesFixture(
                handle, sceneID: c1132GridNamedLineSceneID, runID: c1132GridNamedLinesRunID)
        } else if c1131GridRepeatProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC11_3_1GridRepeatFixture(
                handle, sceneID: c1131GridRepeatSceneArgument, runID: c1131GridRepeatRunID)
        } else if c1123GridAutoPlacementProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC11_2_3GridAutoPlacementFixture(
                handle, sceneID: c1123GridSceneID, runID: c1123GridRunID)
        } else if c1122GridAutoPlacementProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC11_2_2GridAutoPlacementFixture(
                handle, sceneID: c1122GridSceneID, runID: c1122GridRunID)
        } else if c1121GridPlacementProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC11_2_1GridPlacementFixture(
                handle, sceneID: c1121GridSceneID, runID: c1121GridRunID)
        } else if c11GridFixtureRequested {
            result = SpinonRunner.evalRuntimeGpuC11_1GridSceneFixture(
                handle, sceneIndex: UInt32(c11GridSceneIndex ?? 0))
        } else if c1035PositionedFlexProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC10_3_5PositionedFlexFixture(handle)
        } else if c123FixedPositionProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC12_3FixedFixture(handle)
        } else if c124StackingProbeRequested || c124DepthSafetyProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC12_4StackingFixture(handle)
        } else if c122AbsoluteBlockProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC12_2AbsoluteBlockFixture(handle)
        } else if c121PositioningProbeRequested {
            result = SpinonRunner.evalRuntimeGpuC12_1PositionFixture(handle)
        } else if flexBaselineProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFlexBaselineFixture(handle)
        } else if flexAlignmentProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFlexAlignmentFixture(handle)
        } else if flexOrderProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFlexOrderFixture(handle)
        } else if flexReverseProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFlexReverseFixture(handle)
        } else if flexDistributionProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFlexDistributionFixture(handle)
        } else if flexWrapProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFlexWrapFixture(handle)
        } else if flowRootProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFlowRootFixture(handle)
        } else if marginCollapseProbeRequested {
            result = SpinonRunner.evalRuntimeGpuMarginCollapseFixture(handle)
        } else if viewportUnitsProbeRequested {
            result = SpinonRunner.evalRuntimeGpuViewportUnitsFixture(handle)
        } else if blockFormattingProbeRequested {
            result = SpinonRunner.evalRuntimeGpuBlockFormattingFixture(handle)
        } else if rootHtmlTextPaintProbeRequested {
            result = SpinonRunner.evalRuntimeGpuRootHtmlTextPaintFixture(handle)
        } else if rootHtmlBlockPaintProbeRequested {
            result = SpinonRunner.evalRuntimeGpuRootHtmlBlockPaintFixture(handle)
        } else if blockPaintProbeRequested {
            result = SpinonRunner.evalRuntimeGpuBlockPaintFixture(handle)
        } else if rootHtmlDefaultRenderRequested {
            result = "status=0 root_html_default_render=source"
        } else if aspectRatioProbeRequested {
            result = SpinonRunner.evalRuntimeGpuAspectRatioFixture(handle)
        } else if borderWidthProbeRequested {
            result = SpinonRunner.evalRuntimeGpuBorderWidthFixture(handle)
        } else if minMaxSizingProbeRequested {
            result = SpinonRunner.evalRuntimeGpuMinMaxSizingFixture(handle)
        } else if typedCssMathProbeRequested {
            result = SpinonRunner.evalRuntimeGpuTypedCssMathFixture(handle)
        } else if fontRelativeUnitsProbeRequested {
            result = SpinonRunner.evalRuntimeGpuFontRelativeUnitsFixture(handle)
        } else if absoluteLengthsProbeRequested {
            result = SpinonRunner.evalRuntimeGpuAbsoluteLengthsFixture(handle)
        } else if spacingPercentagesProbeRequested {
            result = SpinonRunner.evalRuntimeGpuSpacingPercentagesFixture(handle)
        } else if percentageDimensionsProbeRequested {
            result = SpinonRunner.evalRuntimeGpuPercentageDimensionsFixture(handle)
        } else if incrementalRestyleProbeRequested {
            result = SpinonRunner.evalRuntimeGpuIncrementalRestyleFixture(handle)
        } else if registeredPropertiesProbeRequested {
            result = SpinonRunner.evalRuntimeGpuRegisteredPropertiesFixture(handle)
        } else if authorStylesheetsProbeRequested {
            result = SpinonRunner.evalRuntimeGpuAuthorStylesheetsFixture(handle)
        } else {
            result = SpinonRunner.evalRuntimeGpuFixture(handle)
        }
        }
        if c1133GridTemplateAreasProbeRequested,
           c1133GridTemplateAreasSceneID == "basic-rectangles",
           result?.hasPrefix("status=0 ") == true {
            let rejectionHost = SpinonRunner.createRuntimeGpuHostWithC11_3_3GridTemplateAreasFixture()
            let rejection: String
            if rejectionHost == 0 {
                rejection = "status=-2 isolated rejection host creation failed"
            } else {
                let rejectionEnvironment = SpinonRunner.setRuntimeGpuEnvironment(
                    rejectionHost, width: 320, height: 240, scale: 1,
                    dark: environmentInputs.dark
                ) ?? "status=-1 missing-environment-report"
                rejection = rejectionEnvironment.hasPrefix("status=0 ")
                    ? SpinonRunner.evalRuntimeGpuC11_3_3GridTemplateAreasFixture(
                        rejectionHost,
                        sceneID: c1133GridTemplateAreasRejectSceneID,
                        runID: c1133GridTemplateAreasRunID
                    ) ?? "status=-1 missing-report"
                    : rejectionEnvironment
                SpinonRunner.freeRuntimeGpuHost(rejectionHost)
            }
            let expectedRejection = rejection.hasPrefix("status=-11 ")
                && rejection.contains("unsupported_computed_value")
                && rejection.contains("property=Some(\"grid-template-areas\")")
                && !rejection.contains("node_frames_css_px=[")
            log("SPINON_C1133_GRID_REJECTION run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasRejectSceneID) expected=\(expectedRejection) \(rejection)")
            let rejectionStatus = String(rejection.split(separator: " ", maxSplits: 1).first ?? "status=-1")
            let rejectionCode = rejection.contains("unsupported_computed_value")
                ? "unsupported_computed_value" : "missing"
            let rejectionProperty = rejection.contains("property=Some(\"grid-template-areas\")")
                ? "grid-template-areas" : "missing"
            let rejectionFrames = rejection.contains("node_frames_css_px=[") ? "present" : "absent"
            log("SPINON_C1133_GRID_REJECTION_SUMMARY run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasRejectSceneID) expected=\(expectedRejection) \(rejectionStatus) code=\(rejectionCode) property=\(rejectionProperty) frames=\(rejectionFrames)")
            guard expectedRejection else {
                postStatus("실패 · C11.3.3 area track 수 불일치 거부 계약 오류")
                return
            }
        }
        if c1132GridNamedLinesProbeRequested, result?.hasPrefix("status=0 ") == true {
            let rejectionHost = SpinonRunner.createRuntimeGpuHostWithC11_3_2GridNamedLinesFixture()
            let rejection: String
            if rejectionHost == 0 {
                rejection = "status=-2 isolated rejection host creation failed"
            } else {
                let rejectionEnvironment = SpinonRunner.setRuntimeGpuEnvironment(
                    rejectionHost, width: 320, height: 240, scale: 1,
                    dark: environmentInputs.dark
                ) ?? "status=-1 missing-environment-report"
                rejection = rejectionEnvironment.hasPrefix("status=0 ")
                    ? SpinonRunner.evalRuntimeGpuC11_3_2GridNamedLinesFixture(
                        rejectionHost,
                        sceneID: c1132GridNamedLineRejectSceneID,
                        runID: c1132GridNamedLinesRunID
                    ) ?? "status=-1 missing-report"
                    : rejectionEnvironment
                SpinonRunner.freeRuntimeGpuHost(rejectionHost)
            }
            let expectedRejection = rejection.hasPrefix("status=-11 ")
                && rejection.contains("unsupported_named_grid_line")
                && rejection.contains("property=Some(\"grid-column-start\")")
                && !rejection.contains("node_frames_css_px=[")
            log("SPINON_C1132_GRID_REJECTION run_id=\(c1132GridNamedLinesRunID) scene_id=\(c1132GridNamedLineRejectSceneID) expected=\(expectedRejection) \(rejection)")
            let rejectionStatus = String(rejection.split(separator: " ", maxSplits: 1).first ?? "status=-1")
            let rejectionCode = rejection.contains("unsupported_named_grid_line")
                ? "unsupported_named_grid_line" : "missing"
            let rejectionProperty = rejection.contains("property=Some(\"grid-column-start\")")
                ? "grid-column-start" : "missing"
            let rejectionFrames = rejection.contains("node_frames_css_px=[") ? "present" : "absent"
            log("SPINON_C1132_GRID_REJECTION_SUMMARY run_id=\(c1132GridNamedLinesRunID) scene_id=\(c1132GridNamedLineRejectSceneID) expected=\(expectedRejection) \(rejectionStatus) code=\(rejectionCode) property=\(rejectionProperty) frames=\(rejectionFrames)")
            guard expectedRejection else {
                postStatus("실패 · C11.3.2 implicit named line 거부 계약 불일치")
                return
            }
        }
        if c132ScrollFixtureRequested {
            let report = result ?? "status=-1 missing-report"
            log("SPINON_C132_SCROLL_EVAL_RUN \(runtimeStatusSummary(report))")
            let c132ScrollPassed = report.hasPrefix("status=0 ")
                && report.contains("c13_2_scroll_fixture=passed")
                && report.contains("border_escape_outside_parent=true")
                && report.contains("border_escape_hit=matched")
                && report.contains("c13_2_root_body_propagation=passed")
                && report.contains("c13_2_root_body_hidden=passed")
                && report.contains("c13_2_lifecycle=passed")
                && report.contains("c13_2_failure_controls=passed")
                && report.contains("reserve_failure=atomic")
                && report.contains("reserve_failure_marker=matched")
                && report.contains("stale_publish=rejected")
                && report.contains("recovery=same_host")
                && report.contains("growth_offset_retained=true")
                && report.contains("shrink_clamped_to_zero=true")
                && report.contains("detach_removed_owner=true")
                && report.contains("reparent_reset_offset=true")
            guard c132ScrollPassed else {
                postStatus("실패 · C13.2 scroll·paint·hit·lifecycle 결과 불일치")
                return
            }
            log("SPINON_C132_SCROLL_ASSERTIONS root_body_propagation=passed root_body_hidden=passed border_escape_outside_parent=true border_escape_hit=matched lifecycle=passed growth_offset_retained=true shrink_clamped_to_zero=true detach_removed_owner=true reparent_reset_offset=true reserve_failure=atomic reserve_failure_marker=matched stale_publish=rejected recovery=same_host")
            log("SPINON_C132_SCROLL_SUMMARY status=passed \(runtimeStatusSummary(report))")
            if let pixelRun = c132MetalPixelRun {
                stateLock.lock()
                c132ScrollAssertionsPassed = true
                stateLock.unlock()
                log("SPINON_C132_METAL_PIXEL_RUNTIME_READY run_id=\(pixelRun.runID) \(runtimeStatusSummary(report))")
            }
        } else if c133GestureFixtureRequested {
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
            let report = result ?? "status=-1 missing-report"
            let scenarioID = c133GesturePolicyScenarioID
            let scenario = scenarioID == 0 ? "auto-zero"
                : scenarioID == 1 ? "auto-boundary"
                : scenarioID == 2 ? "contain-x-auto-y-boundary"
                : scenarioID == 3 ? "touch-action-none"
                : scenarioID == 4 ? "touch-action-pan-x"
                : scenarioID == 5 ? "touch-action-pan-y"
                : scenarioID == 6 ? "touch-action-target-pan-x"
                : scenarioID == 7 ? "touch-action-intersection-none"
                : scenarioID == 8 ? "style-policy-invalidation"
                : scenarioID == 9 ? "geometry-invalidation"
                : scenarioID == 10 ? "paint-invalidation" : "invalid"
            let expectedPolicy = scenarioID == 2
                ? "policy_inner=contain/auto" : "policy_inner=auto/auto"
            let boundarySeed = scenarioID == 1 || scenarioID == 2
                || scenarioID == 8 || scenarioID == 9
            let expectedSeed = boundarySeed ? "fixture_seed=inner-max" : "fixture_seed=none"
            let expectedOffsets = boundarySeed
                ? "offsets=root:0,0;outer:0,0;inner:86,106"
                : "offsets=root:0,0;outer:0,0;inner:0,0"
            let expectedTouchAction = scenarioID == 3 || scenarioID == 7
                ? "touch_action_effective=none"
                : scenarioID == 4 || scenarioID == 6 || scenarioID == 8
                    ? "touch_action_effective=pan-x"
                : scenarioID == 5 ? "touch_action_effective=pan-y"
                : "touch_action_effective=auto"
            let expectedInnerTouchAction = scenarioID == 3 ? "none"
                : scenarioID == 4 ? "pan-x"
                : scenarioID == 8 ? "pan-x"
                : scenarioID == 5 || scenarioID == 7 ? "pan-y"
                : "auto"
            let expectedInnerContentTouchAction = scenarioID == 6 || scenarioID == 7
                ? "pan-x" : "auto"
            guard scenarioID >= 0 && scenarioID <= 10
                    && report.hasPrefix("status=0 ")
                    && report.contains("c13_3_gesture_fixture=ready")
                    && report.contains("scenario=\(scenario)")
                    && report.contains(expectedPolicy)
                    && report.contains(expectedTouchAction)
                    && report.contains("touch_action_inner=\(expectedInnerTouchAction)")
                    && report.contains("touch_action_inner_content=\(expectedInnerContentTouchAction)")
                    && report.contains(expectedSeed)
                    && report.contains(expectedOffsets)
                    && report.contains("viewport_css_px=400x240")
                    && report.contains("input_queue_capacity=64") else {
                postStatus("실패 · C13.3 policy scenario·boundary 상태가 불일치합니다")
                log("SPINON_C133_READY_ASSERTIONS failed=true \(report)")
                return
            }
            stateLock.lock()
            c133GestureEvaluationReady = true
            c133GestureFixtureReady = false
            stateLock.unlock()
            let actualTouchAction = reportField(report, "touch_action_effective=")
            let actualInnerTouchAction = reportField(report, "touch_action_inner=")
            let actualInnerContentTouchAction = reportField(report, "touch_action_inner_content=")
            c133ResolvedTouchAction = actualTouchAction
            let touchActionSummary = "touch_action_effective=\(actualTouchAction) touch_action_inner=\(actualInnerTouchAction) touch_action_inner_content=\(actualInnerContentTouchAction)"
            log("SPINON_C133_EVAL_READY input=UIPanGestureRecognizer native_slop=system scenario=\(scenario) \(touchActionSummary) \(runtimeStatusSummary(report))")
            postStatus("준비 중 · C13.3 boundary snapshot의 첫 Metal draw를 확인합니다")
            if c133TapProbeRequested {
                DispatchQueue.main.async { [weak self] in
                    guard let self else { return }
                    let panEnabled = actualTouchAction != "none"
                    self.c133PanRecognizer?.isEnabled = panEnabled
                    self.log("SPINON_C133_TAP_IOS arbitration_ready=true effective=\(actualTouchAction) native_pan_enabled=\(panEnabled) tap_requires_pan_failure=true")
                    self.requestDraw()
                }
            } else {
                requestDraw()
            }
#endif
        } else if c131StackingDescendantClipProbeRequested {
            let report = result ?? "status=-1 missing-report"
            log("SPINON_C131_STACKING_CLIP_EVAL_RUN run_id=\(c131StackingDescendantClipRunID) \(runtimeStatusSummary(report))")
            guard logC131StackingDescendantClipFixtureReport(report) else {
                c131CompactHitIdentityGridStatusMessage = "실패 · C13.1 stacking clip frame·hit-test 검증 결과 불일치"
                postStatus("실패 · C13.1 stacking clip frame·hit-test 검증 결과 불일치")
                return
            }
        } else if c131CompactSurfaceMatrixRouteRequested {
            let report = result ?? "status=-1 missing-report"
            if c131CompactHitIdentityGridProbeRequested {
                log("SPINON_C131_COMPACT_EVAL_RUN run_id=\(c131CompactSurfaceMatrixRunID) \(runtimeStatusSummary(report))")
            } else {
                log("SPINON_C131_COMPACT_EVAL_RUN \(runtimeStatusSummary(report))")
            }
            let runID = c131CompactHitIdentityGridProbeRequested ? c131CompactSurfaceMatrixRunID : nil
            guard logC131CompactSurfaceMatrixFixtureReport(report, runID: runID) else {
                if c131CompactHitIdentityGridProbeRequested {
                    c131CompactHitIdentityGridStatusMessage = "실패 · C13.1 compact frame·hit-test 검증 결과 불일치"
                }
                postStatus("실패 · C13.1 compact frame·hit-test 검증 결과 불일치")
                return
            }
            if c131CompactHitIdentityGridProbeRequested,
               !logC131CompactHitIdentityGridSummary(report) {
                postStatus("실패 · C13.1 Chrome hit identity 30,000점 검증 불일치")
                return
            }
        } else if c131BorderStripProbeRequested {
            let report = result ?? "status=-1 missing-report"
            log("SPINON_C131_BORDER_EVAL_RUN \(runtimeStatusSummary(report))")
            guard logC131BorderFixtureReport(report) else {
                postStatus("실패 · C13.1 border-strip frame·hit-test 검증 결과 불일치")
                return
            }
        } else if c131OverflowClipProbeRequested {
            let report = result ?? "status=-1 missing-report"
            log("SPINON_C131_EVAL_RUN \(runtimeStatusSummary(report))")
            guard logC131FixtureReport(report) else {
                postStatus("실패 · C13.1 frame·hit-test 검증 결과 불일치")
                return
            }
        } else if c114GridTrackSizingProbeRequested {
            let report = result ?? "status=-1 missing-report"
            if !report.hasPrefix("status=0 ") {
                log("SPINON_C114_GRID_EVAL_FAILURE_DETAIL run_id=\(c114GridTrackSizingRunID) \(report.suffix(700))")
            }
            let rootFrame = c114ExpectedRootFrame(c114GridTrackSizingSceneID)
            let itemFrame = c114ExpectedItemFrame(c114GridTrackSizingSceneID)
            let expectedNodeFrames = c114ExpectedNodeFrames(c114GridTrackSizingSceneID)
            let expectedTextInputs = c114ExpectedTextIntrinsicInputs(
                c114GridTrackSizingSceneID, report: report)
            let expectedImageInputs = c114ExpectedImageIntrinsicInputs(
                c114GridTrackSizingSceneID, report: report)
            let expectedNodeFramesMatch = expectedNodeFrames.map(report.contains) ?? true
            let expectedTextMultiSpanFramesMatch = c114ExpectedTextMultiSpanFramesMatch(
                c114GridTrackSizingSceneID, report: report)
            let expectedImageMultiSpanFramesMatch = c114ExpectedImageMultiSpanFramesMatch(
                c114GridTrackSizingSceneID, report: report)
            let expectedMixedMultiSpanFramesMatch = c114ExpectedMixedMultiSpanFramesMatch(
                c114GridTrackSizingSceneID, report: report)
            let expectedRootFrameMatches = rootFrame.isEmpty || report.contains(rootFrame)
            let expectedItemFrameMatches = itemFrame.isEmpty || report.contains(itemFrame)
            log("SPINON_C114_GRID_EVAL_RUN run_id=\(c114GridTrackSizingRunID) scene_id=\(c114GridTrackSizingSceneID) \(runtimeStatusSummary(report))")
            let gridTrackSizingTraceAvailable = logC114GridTrackSizingTraceChunks(report)
            guard report.hasPrefix("status=0 "),
                  report.contains("layout=ready boxes=\(c114ExpectedBoxCount(c114GridTrackSizingSceneID))"),
                  expectedRootFrameMatches, expectedItemFrameMatches, expectedTextInputs,
                  expectedImageInputs, expectedNodeFramesMatch,
                  expectedTextMultiSpanFramesMatch, expectedImageMultiSpanFramesMatch,
                  expectedMixedMultiSpanFramesMatch else {
                log("SPINON_C114_GRID_ASSERTION_ERROR run_id=\(c114GridTrackSizingRunID) scene_id=\(c114GridTrackSizingSceneID) expected_root_frame=\(rootFrame) expected_item_frame=\(itemFrame) expected_node_frames=\(String(describing: expectedNodeFrames)) report=\(report)")
                postStatus("실패 · C11.4 track-sizing frame 검증 결과 불일치")
                return
            }
            guard gridTrackSizingTraceAvailable else {
                postStatus("실패 · C11.4 track-sizing trace가 없습니다")
                return
            }
            log("SPINON_C114_GRID_SUMMARY run_id=\(c114GridTrackSizingRunID) scene_id=\(c114GridTrackSizingSceneID) expected_root_frame=\(rootFrame) expected_item_frame=\(itemFrame) \(runtimeStatusSummary(report))")
            logC114GridNodeFrames(report)
        } else if c1133GridTemplateAreasProbeRequested {
            log("SPINON_C1133_GRID_EVAL_RUN run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasSceneID) \(runtimeStatusSummary(result ?? "status=-1 missing-report"))")
            if let result, result.hasPrefix("status=0 ") {
                log("SPINON_C1133_GRID_SUMMARY run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasSceneID) \(runtimeStatusSummary(result))")
                logC1133GridTemplateAreaFrames(result)
            } else if let result {
                let code = result.contains("unsupported_computed_value")
                    ? "unsupported_computed_value"
                    : result.contains("unsupported_grid_auto_rows")
                        ? "unsupported_grid_auto_rows"
                    : result.contains("unsupported_named_grid_line")
                        ? "unsupported_named_grid_line" : "missing"
                let property = result.contains("property=Some(\"grid-template-areas\")")
                    ? "grid-template-areas"
                    : result.contains("property=Some(\"grid-auto-rows\")")
                        ? "grid-auto-rows"
                    : result.contains("property=Some(\"grid-row-start\")")
                        ? "grid-row-start" : "missing"
                let nodeID = result.range(of: #"node=Some\(\d+\)"#, options: .regularExpression).map {
                    String(result[$0].dropFirst("node=Some(".count).dropLast())
                } ?? "missing"
                let expectedNodeID = c1133GridTemplateAreasSceneID == "area-track-count-mismatch"
                    || c1133GridTemplateAreasSceneID == "unknown-area-implicit-tracks"
                    ? "1" : "missing"
                let status = String(result.split(separator: " ", maxSplits: 1).first ?? "status=-1")
                let expected = status == "status=-11"
                    && ((c1133GridTemplateAreasSceneID == "area-track-count-mismatch"
                        && code == "unsupported_computed_value" && property == "grid-template-areas"
                        && nodeID == expectedNodeID)
                    || (c1133GridTemplateAreasSceneID == "unknown-area-implicit-tracks"
                        && code == "unsupported_grid_auto_rows" && property == "grid-auto-rows"
                        && nodeID == expectedNodeID))
                let frames = result.contains("node_frames_css_px=[") ? "present" : "absent"
                log("SPINON_C1133_GRID_REJECTION_SUMMARY run_id=\(c1133GridTemplateAreasRunID) scene_id=\(c1133GridTemplateAreasSceneID) expected=\(expected) \(status) code=\(code) property=\(property) node_id=\(nodeID) frames=\(frames)")
            }
        } else if c1132GridNamedLinesProbeRequested {
            log("SPINON_C1132_GRID_EVAL_RUN run_id=\(c1132GridNamedLinesRunID) scene_id=\(c1132GridNamedLineSceneID) \(runtimeStatusSummary(result ?? "status=-1 missing-report"))")
            if let result, result.hasPrefix("status=0 ") {
                log("SPINON_C1132_GRID_SUMMARY run_id=\(c1132GridNamedLinesRunID) scene_id=\(c1132GridNamedLineSceneID) \(runtimeStatusSummary(result))")
                logC1132GridNamedLineFrames(result)
            }
        } else if c1131GridRepeatProbeRequested {
            log("SPINON_C1131_GRID_EVAL_RUN run_id=\(c1131GridRepeatRunID) scene_id=\(c1131GridRepeatSceneArgument) \(runtimeStatusSummary(result ?? "status=-1 missing-report"))")
        } else if c1123GridAutoPlacementProbeRequested || c1122GridAutoPlacementProbeRequested {
            let profile = c1123GridAutoPlacementProbeRequested ? "C1123" : "C1122"
            let runID = c1123GridAutoPlacementProbeRequested ? c1123GridRunID : c1122GridRunID
            let sceneID = c1123GridAutoPlacementProbeRequested ? c1123GridSceneID : c1122GridSceneID
            log("SPINON_\(profile)_GRID_EVAL_RUN run_id=\(runID) scene_id=\(sceneID) \(runtimeStatusSummary(result ?? "status=-1 missing-report"))")
            if let result, result.hasPrefix("status=0 ") {
                logGridAutoPlacementReadySummaryIfNeeded(
                    result, profile: profile, runID: runID, sceneID: sceneID)
            } else if let result,
                      result.hasPrefix("status=-12 "),
                      result.contains("op=eval status=0 ") {
                log("SPINON_\(profile)_GRID_SUPERSEDED run_id=\(runID) scene_id=\(sceneID) \(runtimeStatusSummary(result))")
            } else if let result {
                log("SPINON_\(profile)_GRID_SUMMARY run_id=\(runID) scene_id=\(sceneID) \(runtimeStatusSummary(result))")
                let marker = "CSS runtime 장면 생성 실패:"
                let diagnostic = result.range(of: marker).map { String(result[$0.lowerBound...]) }
                    ?? "diagnostic-marker-missing"
                log("SPINON_\(profile)_GRID_DIAGNOSTIC run_id=\(runID) scene_id=\(sceneID) \(diagnostic.prefix(768))")
            } else {
                log("SPINON_\(profile)_GRID_SUMMARY run_id=\(runID) scene_id=\(sceneID) status=-1 missing-report")
            }
        } else if c1121GridPlacementProbeRequested {
            log("SPINON_C1121_GRID_EVAL_RUN run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) \(runtimeStatusSummary(result ?? "status=-1 missing-report"))")
            if let result, result.hasPrefix("status=0 ") {
                logC1121GridReadySummaryIfNeeded(result)
            } else if let result,
                      result.hasPrefix("status=-12 "),
                      result.contains("op=eval status=0 ") {
                log("SPINON_C1121_GRID_SUPERSEDED run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) \(runtimeStatusSummary(result))")
            } else if let result {
                log("SPINON_C1121_GRID_SUMMARY run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) \(runtimeStatusSummary(result))")
                let marker = "CSS runtime 장면 생성 실패:"
                let diagnostic = result.range(of: marker).map { String(result[$0.lowerBound...]) }
                    ?? "diagnostic-marker-missing"
                log("SPINON_C1121_GRID_DIAGNOSTIC run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) \(diagnostic.prefix(768))")
            } else {
                log("SPINON_C1121_GRID_SUMMARY run_id=\(c1121GridRunID) scene_id=\(c1121GridSceneID) status=-1 missing-report")
            }
        } else if c11GridFixtureRequested {
            let sceneId = c11GridSceneIndex.map { c11GridSceneIDs[$0] } ?? "invalid"
            log("SPINON_C111_GRID_SUMMARY scene_id=\(sceneId) \(runtimeStatusSummary(result ?? "status=-1 missing-report"))")
            if let result, result.hasPrefix("status=0 ") {
                logC111GridNodeFrames(result)
            }
        }
        let sceneWasSupersededAfterCommit = result?.hasPrefix("status=-12 ") == true
            && result?.contains("op=eval status=0 ") == true
        if c123FixedPositionProbeRequested,
           let result,
           !result.hasPrefix("status=0 ") {
            logC123FailureDetails(result)
        }
        if c123FixedPositionProbeRequested && sceneWasSupersededAfterCommit {
            postStatus("실패 · C12.3 초기 장면이 environment matrix 전에 교체됐습니다")
            log("SPINON_C123_MATRIX_FAILURE reason=initial-scene-superseded report=\(result ?? "missing")")
            return
        }
        guard result?.hasPrefix("status=0 ") == true || sceneWasSupersededAfterCommit else {
            postStatus("실패 · \(result ?? "JavaScript 보고 없음")")
            return
        }
        if let pixelCase = c124MetalPixelCase {
            let mutation = SpinonRunner.evalRuntimeGpuC12_4Source(
                handle, source: c124MetalPixelMutationSource(for: pixelCase)
            ) ?? "status=-1 missing-report"
            guard mutation.hasPrefix("status=0 ") else {
                log("SPINON_C124_METAL_PIXEL status=error run_id=\(pixelCase.runID) case=\(pixelCase.caseID) stage=fixture-mutation report=\(mutation)")
                postStatus("실패 · C12.4 reverse Flex fixture 변경 · \(mutation)")
                return
            }
            result = mutation
            log("SPINON_C124_METAL_FIXTURE run_id=\(pixelCase.runID) case=\(pixelCase.caseID) direction=\(pixelCase.direction) blue_order=\(pixelCase.blueOrder) red_order=\(pixelCase.redOrder) expected=\(pixelCase.expectedRGBA) \(runtimeStatusSummary(mutation))")
        }
        if c124DepthSafetyProbeRequested {
            guard let fixtureNodeCount = Int(reportField(result ?? "", "document_nodes=")) else {
                postStatus("실패 · C12.4 fixture 기본 노드 수를 읽지 못했습니다")
                return
            }
            let depthProbe = runC124DepthSafetyProbe(
                host: handle, fixtureNodeCount: fixtureNodeCount
            )
            guard depthProbe.passed else {
                postStatus("실패 · C12.4 깊이 검증 · \(depthProbe.failure)")
                return
            }
        }
        if flexReverseProbeRequested, result?.hasPrefix("status=0 ") == true {
            logC1031NodeFrames(result ?? "")
        }
        if flexAlignmentProbeRequested, result?.hasPrefix("status=0 ") == true {
            logC1033RuntimeSummary(result ?? "")
            logC1033NodeFrames(result ?? "")
        }
        if flexBaselineProbeRequested, result?.hasPrefix("status=0 ") == true {
            logC1034RuntimeSummary(result ?? "")
            logC1034NodeFrames(result ?? "")
        }
        if flexOrderProbeRequested, result?.hasPrefix("status=0 ") == true {
            logC1032NodeFrames(result ?? "")
        }
        if c121PositioningProbeRequested, result?.hasPrefix("status=0 ") == true {
            logC121NodeFrames("initial", result ?? "")
        }
        if c122AbsoluteBlockProbeRequested, result?.hasPrefix("status=0 ") == true {
            log("SPINON_C122_SUMMARY \(runtimeStatusSummary(result ?? ""))")
            logC122NodeFrames(result ?? "")
        }
        if c123FixedPositionProbeRequested, result?.hasPrefix("status=0 ") == true {
            guard let initialEnvironmentRevision = UInt64(
                reportField(environment ?? "", "environment_revision=")
            ) else {
                log("SPINON_C123_MATRIX_FAILURE state=initial missing_environment_revision")
                postStatus("실패 · C12.3 초기 환경 revision이 없습니다")
                return
            }
            let initialEvalRevision = reportField(result ?? "", "environment_revision=")
            guard initialEvalRevision == String(initialEnvironmentRevision) else {
                log("SPINON_C123_MATRIX_FAILURE state=initial expected_revision=\(initialEnvironmentRevision) actual_revision=\(initialEvalRevision)")
                postStatus("실패 · C12.3 초기 environment revision 불일치")
                return
            }
            log("SPINON_C123_SUMMARY state=matrix-360x800-dpr-1 \(runtimeStatusSummary(result ?? ""))")
            logC123NodeFrames("matrix-360x800-dpr-1", result ?? "")
            guard runC123EnvironmentMatrix(
                host: handle,
                dark: environmentInputs.dark,
                deviceScaleFactor: environmentInputs.scale,
                initialEnvironmentRevision: initialEnvironmentRevision
            ) else {
                postStatus("실패 · C12.3 environment matrix")
                return
            }
        }
        if c1035PositionedFlexProbeRequested, result?.hasPrefix("status=0 ") == true {
            log("SPINON_C1035_SUMMARY \(runtimeStatusSummary(result ?? ""))")
            logC1035NodeFrames(result ?? "")
        }
        if sceneWasSupersededAfterCommit {
            let scope = c1133GridTemplateAreasProbeRequested
                ? "SPINON_C1133_GRID"
                : c1132GridNamedLinesProbeRequested
                ? "SPINON_C1132_GRID"
                : c1131GridRepeatProbeRequested
                ? "SPINON_C1131_GRID"
                : c1121GridPlacementProbeRequested
                ? "SPINON_C1121_GRID"
                : c11GridFixtureRequested
                ? "SPINON_C111_GRID"
                : c1035PositionedFlexProbeRequested
                ? "SPINON_C1035"
                : c122AbsoluteBlockProbeRequested
                ? "SPINON_C122"
                : c124StackingProbeRequested
                ? "SPINON_C124"
                : c121PositioningProbeRequested
                ? "SPINON_C121"
                : flexAlignmentProbeRequested
                ? "SPINON_C1033"
                : flexBaselineProbeRequested
                ? "SPINON_C1034"
                : flexOrderProbeRequested
                ? "SPINON_C1032"
                : flexReverseProbeRequested
                ? "SPINON_C1031"
                : flexDistributionProbeRequested
                ? "SPINON_C102"
                : flexWrapProbeRequested
                ? "SPINON_C101"
                : marginCollapseProbeRequested
                ? "SPINON_C092"
                : viewportUnitsProbeRequested
                ? "SPINON_C066A"
                : blockFormattingProbeRequested
                ? "SPINON_C091"
                : blockPaintProbeRequested
                ? "SPINON_C08"
                : aspectRatioProbeRequested
                ? "SPINON_C073"
                : borderWidthProbeRequested
                ? "SPINON_C072"
                : minMaxSizingProbeRequested
                ? "SPINON_C071"
                : typedCssMathProbeRequested
                ? "SPINON_C065"
                : fontRelativeUnitsProbeRequested
                ? "SPINON_C064"
                : absoluteLengthsProbeRequested
                ? "SPINON_C063"
                : spacingPercentagesProbeRequested
                ? "SPINON_C062"
                : percentageDimensionsProbeRequested
                ? "SPINON_C061"
                : incrementalRestyleProbeRequested
                ? "SPINON_C054"
                : registeredPropertiesProbeRequested
                ? runtimeResultCacheProbeRequested ? "SPINON_C053" : "SPINON_C052"
                : authorStylesheetsProbeRequested ? "SPINON_C0411" : "SPINON_C0410"
            log("\(scope)_EVAL_SCENE_SUPERSEDED \(result ?? "")")
            postStatus("JavaScript 적용 완료 · 최신 CSS 장면 다시 계산 중")
        } else {
            let scope = c1133GridTemplateAreasProbeRequested
                ? "SPINON_C1133_GRID"
                : c1132GridNamedLinesProbeRequested
                ? "SPINON_C1132_GRID"
                : c1131GridRepeatProbeRequested
                ? "SPINON_C1131_GRID"
                : c1121GridPlacementProbeRequested
                ? "SPINON_C1121_GRID"
                : c11GridFixtureRequested
                ? "SPINON_C111_GRID"
                : c1035PositionedFlexProbeRequested
                ? "SPINON_C1035"
                : c122AbsoluteBlockProbeRequested
                ? "SPINON_C122"
                : c124StackingProbeRequested
                ? "SPINON_C124"
                : c121PositioningProbeRequested
                ? "SPINON_C121"
                : flexAlignmentProbeRequested
                ? "SPINON_C1033"
                : flexBaselineProbeRequested
                ? "SPINON_C1034"
                : flexOrderProbeRequested
                ? "SPINON_C1032"
                : flexReverseProbeRequested
                ? "SPINON_C1031"
                : flexDistributionProbeRequested
                ? "SPINON_C102"
                : flexWrapProbeRequested
                ? "SPINON_C101"
                : marginCollapseProbeRequested
                ? "SPINON_C092"
                : viewportUnitsProbeRequested
                ? "SPINON_C066A"
                : blockFormattingProbeRequested
                ? "SPINON_C091"
                : blockPaintProbeRequested
                ? "SPINON_C08"
                : aspectRatioProbeRequested
                ? "SPINON_C073"
                : borderWidthProbeRequested
                ? "SPINON_C072"
                : minMaxSizingProbeRequested
                ? "SPINON_C071"
                : typedCssMathProbeRequested
                ? "SPINON_C065"
                : fontRelativeUnitsProbeRequested
                ? "SPINON_C064"
                : absoluteLengthsProbeRequested
                ? "SPINON_C063"
                : spacingPercentagesProbeRequested
                ? "SPINON_C062"
                : percentageDimensionsProbeRequested
                ? "SPINON_C061"
                : incrementalRestyleProbeRequested
                ? "SPINON_C054"
                : registeredPropertiesProbeRequested
                ? runtimeResultCacheProbeRequested ? "SPINON_C053" : "SPINON_C052"
                : authorStylesheetsProbeRequested ? "SPINON_C0411" : "SPINON_C0410"
            log("\(scope)_EVAL \(result ?? "")")
            if c131CompactHitIdentityGridProbeRequested {
                postStatus(c131CompactHitIdentityGridStatusMessage
                    ?? "완료 · C13.1 Chrome hit identity 30,000/30,000 · 52/52")
            } else {
                postStatus(runtimeStatusSummary(result ?? "runtime scene 준비 완료"))
            }
            if c121PositioningProbeRequested {
                log("SPINON_C121_SUMMARY state=initial \(runtimeStatusSummary(result ?? ""))")
            } else if aspectRatioProbeRequested {
                log("SPINON_C073_SUMMARY \(runtimeStatusSummary(result ?? ""))")
            } else if borderWidthProbeRequested {
                log("SPINON_C072_SUMMARY \(runtimeStatusSummary(result ?? ""))")
            }
        }
        if c121PositioningProbeRequested, !sceneWasSupersededAfterCommit {
            for state in UInt32(1)...UInt32(2) {
                let stateName = state == 1 ? "target-relative" : "ancestor-relative"
                let stateRun = evalC121PositionState(host: handle, state: state)
                let stateReport = stateRun.report
                guard stateReport.hasPrefix("status=0 ") else {
                    postStatus("실패 · C12.1 \(stateName) · \(stateReport)")
                    return
                }
                log("SPINON_C121_SUMMARY state=\(stateName) attempts=\(stateRun.attempts) \(runtimeStatusSummary(stateReport))")
                logC121NodeFrames(stateName, stateReport)
                result = stateReport
            }
            postStatus(runtimeStatusSummary(result ?? "C12.1 mutation 보고 없음"))
        }
        if customPropertiesProbeRequested && !authorStylesheetsProbeRequested {
            result = SpinonRunner.evalRuntimeGpuCustomPropertiesFixture(handle)
            guard result?.hasPrefix("status=0 ") == true else {
                postStatus("실패 · C05 사용자 지정 속성 · \(result ?? "보고 없음")")
                return
            }
            log("SPINON_C051_EVAL \(result ?? "")")
            postStatus(runtimeStatusSummary(result ?? "C05 장면 준비 완료"))
        }
        if c124DepthSafetyProbeRequested {
            postStatus("C12.4 깊이 검증 통과 · 1024 통과, 1025 거부, 얕은 revision 복구")
        }
        stateLock.lock()
        runtimeInitializationComplete = !closing
        stateLock.unlock()
        DispatchQueue.main.async { [weak self] in
            guard let self else { return }
            let size = self.lastDrawableSize
            if size.width > 0 && size.height > 0 {
                self.ensureRenderer(width: Int(size.width), height: Int(size.height))
            }
            self.refreshEnvironment()
        }
    }

    private func runC124DepthSafetyProbe(
        host: UInt64,
        fixtureNodeCount: Int
    ) -> (passed: Bool, failure: String) {
        let maximumStarted = ProcessInfo.processInfo.systemUptime
        let maximum = SpinonRunner.evalRuntimeGpuC12_4Source(
            host, source: c124DepthBuildSource(maximumDepth: 1024)
        ) ?? "status=-1 missing-report"
        let maximumMillis = Int((ProcessInfo.processInfo.systemUptime - maximumStarted) * 1000)
        let maximumPassed = maximum.hasPrefix("status=0 ")
            && reportField(maximum, "layout=") == "ready"
            && reportField(maximum, "boxes=") == String(fixtureNodeCount + 1023)
            && reportField(maximum, "cascadeRecomputedStyleElements=")
                == String(fixtureNodeCount + 1023)
        log("SPINON_C124_DEPTH_STAGE stage=maximum expected=1024 passed=\(maximumPassed) elapsed_ms=\(maximumMillis) report_length=\(maximum.utf8.count)")
        guard maximumPassed else {
            log("SPINON_C124_DEPTH_SUMMARY status=failed stage=maximum report=\(maximum)")
            return (false, "1024 깊이를 계산하지 못했습니다")
        }

        let overflowStarted = ProcessInfo.processInfo.systemUptime
        let overflow = SpinonRunner.evalRuntimeGpuC12_4Source(
            host, source: c124DepthOverflowSource()
        ) ?? "status=-1 missing-report"
        let overflowMillis = Int((ProcessInfo.processInfo.systemUptime - overflowStarted) * 1000)
        let overflowPassed = !overflow.hasPrefix("status=0 ")
            && overflow.contains("tree_depth_exceeded")
            && overflow.contains("1025") && overflow.contains("1024")
        log("SPINON_C124_DEPTH_STAGE stage=overflow expected=reject-1025 passed=\(overflowPassed) elapsed_ms=\(overflowMillis) report=\(overflow)")
        guard overflowPassed else {
            log("SPINON_C124_DEPTH_SUMMARY status=failed stage=overflow")
            return (false, "1025 초과 요청이 지정된 오류로 거부되지 않았습니다")
        }

        let recoveryStarted = ProcessInfo.processInfo.systemUptime
        let recovery = SpinonRunner.evalRuntimeGpuC12_4Source(
            host, source: c124DepthRecoverySource()
        ) ?? "status=-1 missing-report"
        let recoveryMillis = Int((ProcessInfo.processInfo.systemUptime - recoveryStarted) * 1000)
        let recoveryPassed = recovery.hasPrefix("status=0 ")
            && reportField(recovery, "layout=") == "ready"
            && reportField(recovery, "boxes=") == String(fixtureNodeCount)
        log("SPINON_C124_DEPTH_STAGE stage=recovery expected=shallow-ready passed=\(recoveryPassed) elapsed_ms=\(recoveryMillis) report_length=\(recovery.utf8.count)")
        let passed = maximumPassed && overflowPassed && recoveryPassed
        log("SPINON_C124_DEPTH_SUMMARY status=\(passed ? "passed" : "failed") maximum=1024 overflow=1025 recovery=\(recoveryPassed) platform=ios-simulator")
        return (passed, recoveryPassed ? "" : "초과 거부 후 얕은 revision 계산이 복구되지 않았습니다")
    }

    private func c124DepthBuildSource(maximumDepth: Int) -> String {
        """
        (() => {
          var parent = globalThis.spinonC124NodeRefs['c124-root'];
          var root = null;
          for (let index = 0; index < \(maximumDepth - 1); index += 1) {
            const node = document.createElement('div');
            node.setAttribute('style', 'position:relative;z-index:0;width:1px;height:1px');
            parent.appendChild(node);
            if (root === null) root = node;
            parent = node;
          }
          globalThis.__spinonC124DepthRoot = root;
          globalThis.__spinonC124DepthLeaf = parent;
          return 'SPINON_C124_DEPTH_MAX=\(maximumDepth)';
        })()
        """
    }

    private func c124MetalPixelMutationSource(for pixelCase: C124MetalPixelCase) -> String {
        let column = pixelCase.direction.hasPrefix("column")
        let reverse = pixelCase.direction.hasSuffix("reverse")
        let itemSize = column
            ? "flex:0 0 60px;width:100px;height:60px"
            : "flex:0 0 80px;width:80px;height:60px"
        let overlapMargin: String
        switch (column, reverse) {
        case (false, false): overlapMargin = "margin-right:-40px"
        case (false, true): overlapMargin = "margin-left:-40px"
        case (true, false): overlapMargin = "margin-bottom:-40px"
        case (true, true): overlapMargin = "margin-top:-40px"
        }
        let containerStyle = "display:flex;position:absolute;left:20px;top:125px;width:100px;height:100px;flex-direction:\(pixelCase.direction);align-items:flex-start;justify-content:flex-start"
        let blueStyle = "position:static;\(itemSize);\(overlapMargin);order:\(pixelCase.blueOrder);background-color:#2866f6"
        let redStyle = "position:static;\(itemSize);\(overlapMargin);order:\(pixelCase.redOrder);background-color:#e5484d"
        return """
        (() => {
          const nodes = globalThis.spinonC124NodeRefs;
          if (!nodes) throw new Error('C12.4 surface readback fixture node map is missing');
          const targets = [
            ['c124-static-flex-container', '\(containerStyle)'],
            ['c124-static-flex-z-one', '\(blueStyle)'],
            ['c124-static-flex-z-two', '\(redStyle)'],
          ];
          for (const [id, style] of targets) {
            const element = nodes[id];
            if (!element) throw new Error('C12.4 surface readback target missing: ' + id);
            element.setAttribute('style', style);
          }
          return { direction: '\(pixelCase.direction)', blueOrder: \(pixelCase.blueOrder), redOrder: \(pixelCase.redOrder) };
        })()
        """
    }

    private func c124DepthOverflowSource() -> String {
        """
        (() => {
          const node = document.createElement('div');
          node.setAttribute('style', 'position:relative;z-index:0;width:1px;height:1px');
          globalThis.__spinonC124DepthLeaf.appendChild(node);
          globalThis.__spinonC124DepthOverflow = node;
          return 'SPINON_C124_DEPTH_OVERFLOW=1025';
        })()
        """
    }

    private func c124DepthRecoverySource() -> String {
        """
        (() => {
          globalThis.__spinonC124DepthLeaf.removeChild(globalThis.__spinonC124DepthOverflow);
          globalThis.spinonC124NodeRefs['c124-root'].removeChild(
            globalThis.__spinonC124DepthRoot
          );
          delete globalThis.__spinonC124DepthOverflow;
          delete globalThis.__spinonC124DepthRoot;
          delete globalThis.__spinonC124DepthLeaf;
          return 'SPINON_C124_DEPTH_RECOVERED';
        })()
        """
    }

    @objc private func evaluateCustomPropertiesFixture() {
        enqueueRuntime { [weak self] in
            guard let self else { return }
            stateLock.lock()
            let handle = closing ? 0 : hostHandle
            stateLock.unlock()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuCustomPropertiesFixture(handle)
            log("SPINON_C051_EVAL \(report ?? "보고 없음")")
            postStatus(runtimeStatusSummary(report ?? "사용자 지정 속성 실행 결과가 없습니다"))
            guard report?.hasPrefix("status=0 ") == true else { return }
            requestDraw()
        }
    }

    @objc private func evaluateRegisteredPropertiesFixture() {
        enqueueRuntime { [weak self] in
            guard let self else { return }
            stateLock.lock()
            let handle = closing ? 0 : hostHandle
            stateLock.unlock()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuRegisteredPropertiesFixture(handle)
            log("SPINON_C052_EVAL \(report ?? "보고 없음")")
            postStatus(runtimeStatusSummary(report ?? "등록 사용자 지정 속성 실행 결과가 없습니다"))
            guard report?.hasPrefix("status=0 ") == true else { return }
            requestDraw()
        }
    }

    @objc private func evaluateRuntimeResultCacheFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuRuntimeResultCacheFixture(handle)
            self.log("SPINON_C053_EVAL \(report ?? "runtime cache fixture 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "runtime cache fixture 보고 없음"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    @objc private func evaluateIncrementalRestyleFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuIncrementalRestyleFixture(handle)
            self.log("SPINON_C054_EVAL \(report ?? "incremental restyle fixture 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "incremental restyle 결과가 없습니다"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    @objc private func evaluatePercentageDimensionsFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuPercentageDimensionsFixture(handle)
            self.log("SPINON_C061_EVAL \(report ?? "percentage fixture 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "percentage fixture 보고 없음"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    @objc private func evaluateSpacingPercentagesFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuSpacingPercentagesFixture(handle)
            self.log("SPINON_C062_EVAL \(report ?? "spacing percentage 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "spacing percentage 보고 없음"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    @objc private func evaluateAbsoluteLengthsFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuAbsoluteLengthsFixture(handle)
            self.log("SPINON_C063_EVAL \(report ?? "absolute length fixture 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "absolute length fixture 보고 없음"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    @objc private func evaluateFontRelativeUnitsFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuFontRelativeUnitsFixture(handle)
            self.log("SPINON_C064_EVAL \(report ?? "font-relative fixture 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "font-relative fixture 보고 없음"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    @objc private func evaluateTypedCssMathFixture() {
        enqueueRuntime { [weak self] in
            guard let self, !self.isClosing else { return }
            let handle = self.hostLifetime.load()
            guard handle != 0 else { return }
            let report = SpinonRunner.evalRuntimeGpuTypedCssMathFixture(handle)
            self.log("SPINON_C065_EVAL \(report ?? "typed CSS math fixture 보고 없음")")
            self.postStatus(self.runtimeStatusSummary(report ?? "typed CSS math fixture 보고 없음"))
            if report?.hasPrefix("status=0 ") == true {
                self.canvasView.setNeedsDisplay()
                self.renderDrawLane?.request()
            }
        }
    }

    private func ensureRenderer(width: Int, height: Int) {
        dispatchPrecondition(condition: .onQueue(.main))
        guard width > 0, height > 0 else { return }
        stateLock.lock()
        guard hostHandle != 0, !rendererReady, !rendererCreationPending, !closing else {
            stateLock.unlock()
            return
        }
        rendererCreationPending = true
        let handle = hostHandle
        let generation = surfaceGeneration
        let viewPointer = Unmanaged.passUnretained(canvasView).toOpaque()
        stateLock.unlock()

        let surfaceReport = SpinonRunner.prepareRuntimeGpuWgpuSurface(
            handle, view: viewPointer
        )
        log("SPINON_C0410_SURFACE_PREPARE \(surfaceReport ?? "")")
        guard surfaceReport?.hasPrefix("status=0 ") == true else {
            stateLock.lock()
            rendererCreationPending = false
            stateLock.unlock()
            postStatus(surfaceReport ?? "UIKit surface 준비 보고 없음")
            return
        }

        enqueueRender { [weak self] in
            guard let self else { return }
            guard self.isCurrentSurfaceGeneration(generation) else {
                SpinonRunner.destroyRuntimeGpuRenderer(handle)
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
                self.retryRendererCreation()
                return
            }
            let report = SpinonRunner.createRuntimeGpuWgpu(
                handle, width: UInt32(width), height: UInt32(height)
            )
            self.log("SPINON_C0410_RENDERER \(report ?? "")")
            guard report?.hasPrefix("status=0 ") == true else {
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
                self.postStatus(report ?? "WGPU renderer 보고 없음")
                return
            }
            guard self.isCurrentSurfaceGeneration(generation) else {
                SpinonRunner.destroyRuntimeGpuRenderer(handle)
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
                self.retryRendererCreation()
                return
            }
            DispatchQueue.main.async { [weak self] in
                self?.configureRendererOnMain(
                    handle: handle, generation: generation, rendererReport: report ?? ""
                )
            }
        }
    }

    private func configureRendererOnMain(
        handle: UInt64, generation: UInt64, rendererReport: String
    ) {
        dispatchPrecondition(condition: .onQueue(.main))
        guard isCurrentSurfaceGeneration(generation) else {
            enqueueRender { [weak self] in
                SpinonRunner.destroyRuntimeGpuRenderer(handle)
                guard let self else { return }
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
                self.retryRendererCreation()
            }
            return
        }

        let configureReport = SpinonRunner.configureRuntimeGpuWgpuSurface(handle)
        let configureThread = Thread.isMainThread ? "main" : "other"
        log("SPINON_C0410_SURFACE_CONFIGURE thread=\(configureThread) \(configureReport ?? "")")
        let ready = configureReport?.hasPrefix("status=0 ") == true
        let surfaceFormat = reportField(rendererReport, "format=")
        stateLock.lock()
        let stale = closing || surfaceGeneration != generation
        rendererReady = ready && !stale
        rendererSurfaceGeneration = ready && !stale ? generation : nil
        rendererSurfaceFormat = ready && !stale && !surfaceFormat.isEmpty ? surfaceFormat : nil
        if ready && !stale { rendererCreationPending = false }
        stateLock.unlock()
        if ready && stale {
            enqueueRender { [weak self] in
                SpinonRunner.destroyRuntimeGpuRenderer(handle)
                guard let self else { return }
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
                self.retryRendererCreation()
            }
            return
        }
        guard ready else {
            postStatus(configureReport ?? "UIKit surface 구성 보고 없음")
            enqueueRender { [weak self] in
                SpinonRunner.destroyRuntimeGpuRenderer(handle)
                guard let self else { return }
                self.stateLock.lock()
                self.rendererCreationPending = false
                self.stateLock.unlock()
            }
            return
        }
        postStatus("\(rendererReport) · \(configureReport ?? "surface configured")")
        refreshEnvironment()
    }

    private func resizeRenderer(to size: CGSize) {
        dispatchPrecondition(condition: .onQueue(.main))
        guard size.width > 0, size.height > 0 else { return }
        stateLock.lock()
        guard rendererReady, !surfaceConfigurationPending, !closing else {
            stateLock.unlock()
            return
        }
        surfaceConfigurationPending = true
        rendererSurfaceGeneration = nil
        let generation = surfaceGeneration
        stateLock.unlock()
        enqueueRender { [weak self] in
            guard let self else { return }
            DispatchQueue.main.async { [weak self] in
                self?.configureResizeOnMain(size: size, generation: generation)
            }
        }
    }

    private func configureResizeOnMain(size: CGSize, generation: UInt64) {
        dispatchPrecondition(condition: .onQueue(.main))
        stateLock.lock()
        let current = !closing && surfaceGeneration == generation
        let handle = hostHandle
        stateLock.unlock()
        guard current, handle != 0 else {
            finishSurfaceResize(generation: generation, success: false)
            return
        }

        let result = SpinonRunner.resizeRuntimeGpuWgpu(
            handle, width: UInt32(size.width), height: UInt32(size.height)
        )
        log("SPINON_C0410_RESIZE status=\(result) thread=main")
        guard result == 0 else {
            finishSurfaceResize(generation: generation, success: false)
            postStatus("실패 · WGPU 표면 크기 변경 status=\(result)")
            return
        }
        finishSurfaceResize(generation: generation, success: true)
    }

    private func finishSurfaceResize(generation: UInt64, success: Bool) {
        stateLock.lock()
        let current = !closing && surfaceGeneration == generation
        surfaceConfigurationPending = false
        if success && current { rendererSurfaceGeneration = generation }
        let retrySize = lastDrawableSize
        stateLock.unlock()
        guard !isClosing else { return }
        guard current else {
            if rendererIsReady, retrySize.width > 0, retrySize.height > 0 {
                resizeRenderer(to: retrySize)
            }
            return
        }
        guard success else { return }
        scheduleLatestPresentation()
    }

    private func refreshEnvironment() {
        guard beginPresentationUpdate() != 0 else { return }
        scheduleLatestPresentation()
    }

    private func scheduleLatestPresentation() {
        guard let runtimePresentationLane else { return }
        if case .closed = runtimePresentationLane.request() {
            postStatus("실패 · 닫힌 runtime presentation lane에 요청했습니다")
        }
    }

    private func applyLatestEnvironment() throws {
        let generation: UInt64
        let sequence: UInt64
        let handle: UInt64
        let environmentInputs: (width: Float, height: Float, scale: Float, dark: Bool)
        stateLock.lock()
        guard !closing, hostHandle != 0 else {
            stateLock.unlock()
            return
        }
        generation = surfaceGeneration
        sequence = presentationSequence
        handle = hostHandle
        let fixedCssScaleRoute = c153GlyphPaintProbeRequested
            || c123FixedPositionProbeRequested
            || c1132GridNamedLinesProbeRequested || c1133GridTemplateAreasProbeRequested
            || c132ScrollFixtureRequested || c133GestureFixtureRequested
        environmentInputs = (
            rootHtmlBlockPaintProbeRequested || rootHtmlTextPaintProbeRequested
                || rootHtmlDefaultRenderRequested ? 320
                : c153HostDocumentTextPaintArgumentRequested ? 320
                : c153GlyphPaintProbeRequested ? 360 : viewportWidthCssPx,
            rootHtmlBlockPaintProbeRequested || rootHtmlTextPaintProbeRequested
                || rootHtmlDefaultRenderRequested
                || c153GlyphPaintProbeRequested
                ? 240 : viewportHeightCssPx,
            rootHtmlTextPaintProbeRequested ? 1
                : rootHtmlBlockPaintProbeRequested ? displayScale
                : fixedCssScaleRoute ? 1 : displayScale, darkMode
        )
        stateLock.unlock()
        guard isCurrentPresentation(generation: generation, sequence: sequence) else { return }
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
        if c133GestureFixtureRequested {
            let requestedEnvironment = C133RuntimeEnvironment(
                width: environmentInputs.width,
                height: environmentInputs.height,
                scale: environmentInputs.scale,
                dark: environmentInputs.dark
            )
            stateLock.lock()
            let environmentAlreadyConfigured =
                c133ConfiguredRuntimeEnvironment == requestedEnvironment
            stateLock.unlock()
            if environmentAlreadyConfigured {
                log("SPINON_C0410_ENVIRONMENT skipped=unchanged viewport=\(environmentInputs.width)x\(environmentInputs.height) scale=\(environmentInputs.scale) dark=\(environmentInputs.dark) sequence=\(sequence)")
                requestDraw()
                return
            }
        }
#endif
        let report = SpinonRunner.setRuntimeGpuEnvironment(
            handle,
            width: defaultRootHtmlApplicationRequested || rootHtmlBlockPaintProbeRequested
                || rootHtmlTextPaintProbeRequested
                || rootHtmlDefaultRenderRequested
                ? environmentInputs.width * environmentInputs.scale : environmentInputs.width,
            height: defaultRootHtmlApplicationRequested || rootHtmlBlockPaintProbeRequested
                || rootHtmlTextPaintProbeRequested
                || rootHtmlDefaultRenderRequested
                ? environmentInputs.height * environmentInputs.scale : environmentInputs.height,
            scale: environmentInputs.scale, dark: environmentInputs.dark
        )
        log("SPINON_C0410_ENVIRONMENT viewport=\(environmentInputs.width)x\(environmentInputs.height) scale=\(environmentInputs.scale) dark=\(environmentInputs.dark) sequence=\(sequence) \(report ?? "")")
        if c1131GridRepeatProbeRequested, let report {
            log("SPINON_C1131_GRID_ENVIRONMENT_RESULT run_id=\(c1131GridRepeatRunID) scene_id=\(c1131GridRepeatSceneArgument) \(runtimeStatusSummary(report))")
        } else if c1123GridAutoPlacementProbeRequested, let report {
            logGridAutoPlacementReadySummaryIfNeeded(
                report, profile: "C1123", runID: c1123GridRunID, sceneID: c1123GridSceneID)
        } else if c1122GridAutoPlacementProbeRequested, let report {
            logGridAutoPlacementReadySummaryIfNeeded(
                report, profile: "C1122", runID: c1122GridRunID, sceneID: c1122GridSceneID)
        } else if c1121GridPlacementProbeRequested, let report {
            logC1121GridReadySummaryIfNeeded(report)
        }
        if c131CompactHitIdentityGridProbeRequested,
           report?.hasPrefix("status=0 ") == true,
           let statusMessage = c131CompactHitIdentityGridStatusMessage {
            postStatus(statusMessage)
        } else {
            postStatus(runtimeStatusSummary(report ?? "환경 보고 없음"))
        }
        guard report?.hasPrefix("status=0 ") == true else {
            throw C0410LatestTaskLaneError.operationFailed(report ?? "환경 갱신 보고 없음")
        }
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
        if c133GestureFixtureRequested {
            stateLock.lock()
            c133ConfiguredRuntimeEnvironment = C133RuntimeEnvironment(
                width: environmentInputs.width,
                height: environmentInputs.height,
                scale: environmentInputs.scale,
                dark: environmentInputs.dark
            )
            stateLock.unlock()
        }
#endif
        if report?.hasPrefix("status=0 ") == true
            && isCurrentPresentation(generation: generation, sequence: sequence) {
            requestDraw()
        }
    }

    private func requestDraw() {
        stateLock.lock()
        guard !closing, rendererReady, rendererSurfaceGeneration == surfaceGeneration else {
            stateLock.unlock()
            return
        }
        stateLock.unlock()
        guard let renderDrawLane else { return }
        if case .closed = renderDrawLane.request() {
            postStatus("실패 · 닫힌 render draw lane에 요청했습니다")
        }
    }

#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
    @objc private func handleC133Tap(_ recognizer: UITapGestureRecognizer) {
        guard c133TapProbeRequested else { return }
        let point = recognizer.location(in: canvasView)
        guard recognizer.state == .recognized else {
            if recognizer.state == .failed {
                log("SPINON_C133_TAP_IOS activation=false reason=recognizer-failed count=\(c133TapActivationCount)")
            }
            return
        }
        stateLock.lock()
        let ready = c133GestureFixtureReady && !closing && hostHandle != 0
        let widthCss = viewportWidthCssPx
        let heightCss = viewportHeightCssPx
        let drawable = c111MetalDrawableSize
        stateLock.unlock()
        guard ready, canvasView.bounds.width > 0, canvasView.bounds.height > 0,
              widthCss > 0, heightCss > 0,
              c133ResolvedTouchAction == "none" || c133ResolvedTouchAction == "pan-x" else {
            log("SPINON_C133_TAP_IOS activation_rejected=true reason=fixture-not-ready-or-action-unresolved")
            return
        }
        let xCss = point.x * CGFloat(widthCss) / canvasView.bounds.width
        let yCss = point.y * CGFloat(heightCss) / canvasView.bounds.height
        let surfaceScale = drawable.width / CGFloat(widthCss)
        c133TapActivationCount += 1
        log("SPINON_C133_TAP_IOS activation=true count=\(c133TapActivationCount) scenario=\(c133GesturePolicyScenarioID) effective=\(c133ResolvedTouchAction) point_css_px=\(xCss),\(yCss) point_ui_points=\(point.x),\(point.y) surface_scale=\(surfaceScale) pan_state=\(c133PanRecognizer?.state.rawValue ?? -1) pan_enabled=\(c133PanRecognizer?.isEnabled ?? false)")
        requestDraw()
    }

    @objc private func handleC133Pan(_ recognizer: UIPanGestureRecognizer) {
        guard c133GestureFixtureRequested else { return }
        let started = DispatchTime.now().uptimeNanoseconds
        switch recognizer.state {
        case .began:
            beginC133Pan(recognizer, started: started)
        case .changed:
            moveC133Pan(recognizer, started: started)
        case .ended:
            endC133Pan(recognizer, started: started)
        case .cancelled, .failed:
            cancelC133Gesture(reason: 4, source: "native-recognizer")
            if c133ActiveTouchIDs.isEmpty && !c133StyleBarrierPending {
                resetC133GestureState(clearSuppression: true)
            }
        default:
            break
        }
    }

    @objc private func handleC133NativeWheel(_ recognizer: UIPanGestureRecognizer) {
        guard c133NativeWheelPreflightRequested || c133NativeWheelAdapterArgumentRequested else {
            return
        }
        let identifier = ObjectIdentifier(recognizer)
        let translation = recognizer.translation(in: canvasView)
        stateLock.lock()
        let ready = c133GestureFixtureReady && !closing && hostHandle != 0
        let handle = hostHandle
        let generation = surfaceGeneration
        let widthCss = viewportWidthCssPx
        let heightCss = viewportHeightCssPx
        stateLock.unlock()

        let previous = c133NativeWheelTranslations[identifier]
        let generationChanged = previous.map {
            $0.surfaceGeneration != generation
        } ?? false
        let previousTranslation = previous?.translation ?? .zero
        let delta = CGPoint(
            x: translation.x - previousTranslation.x,
            y: translation.y - previousTranslation.y
        )
        c133NativeWheelTranslations[identifier] = (
            translation: translation,
            surfaceGeneration: generation
        )
        let isTerminal = recognizer.state == .ended || recognizer.state == .cancelled
            || recognizer.state == .failed
        defer {
            if isTerminal {
                c133NativeWheelTranslations.removeValue(forKey: identifier)
            }
        }

        let source: Int32
        let kind: String
        if recognizer === c133NativeWheelDiscreteRecognizer {
            source = 0
            kind = "discrete-mouse"
        } else if recognizer === c133NativeWheelContinuousRecognizer {
            source = 1
            kind = "continuous-trackpad"
        } else {
            log("SPINON_C133_NATIVE_WHEEL_REJECT reason=unknown-recognizer")
            return
        }
        let location = recognizer.location(in: canvasView)
        let modifiers = recognizer.modifierFlags
        if c133NativeWheelPreflightRequested {
            log("SPINON_C133_NATIVE_WHEEL_PREFLIGHT source=\(kind) state=\(recognizer.state.rawValue) translation_points=\(translation.x),\(translation.y) delta_points=\(delta.x),\(delta.y) location_points=\(location.x),\(location.y) modifiers=\(modifiers.rawValue) control=\(modifiers.contains(.control)) touch_types=empty")
            return
        }

        guard translation.x.isFinite, translation.y.isFinite,
              delta.x.isFinite, delta.y.isFinite else {
            c133NativeWheelTranslations.removeValue(forKey: identifier)
            log("SPINON_C133_NATIVE_WHEEL_REJECT reason=nonfinite-translation source=\(kind) state=\(recognizer.state.rawValue)")
            return
        }
        guard !generationChanged else {
            log("SPINON_C133_NATIVE_WHEEL_REJECT reason=surface-generation-changed source=\(kind) generation=\(generation)")
            return
        }
        switch recognizer.state {
        case .began, .changed, .ended:
            break
        case .cancelled, .failed:
            log("SPINON_C133_NATIVE_WHEEL_RESET source=\(kind) state=\(recognizer.state.rawValue) generation=\(generation)")
            return
        default:
            return
        }

        var normalizedModifiers: UInt32 = 0
        if modifiers.contains(.shift) { normalizedModifiers |= 1 }
        if modifiers.contains(.control) { normalizedModifiers |= 2 }
        if modifiers.contains(.alternate) { normalizedModifiers |= 4 }
        if modifiers.contains(.command) { normalizedModifiers |= 8 }
        guard (normalizedModifiers & 2) == 0 else {
            log("SPINON_C133_NATIVE_WHEEL_REJECT reason=control-wheel source=\(kind) raw_delta_points=\(delta.x),\(delta.y) modifiers=\(normalizedModifiers) native_modifiers=\(modifiers.rawValue) generation=\(generation)")
            return
        }

        guard ready, widthCss > 0, heightCss > 0,
              canvasView.bounds.width > 0, canvasView.bounds.height > 0 else {
            log("SPINON_C133_NATIVE_WHEEL_REJECT reason=surface-or-runtime-not-ready source=\(kind) generation=\(generation) ready=\(ready) viewport_css=\(widthCss)x\(heightCss) bounds_points=\(canvasView.bounds.width)x\(canvasView.bounds.height)")
            return
        }
        let bounds = canvasView.bounds
        let pointsPerCssX = bounds.width / CGFloat(widthCss)
        let pointsPerCssY = bounds.height / CGFloat(heightCss)
        guard pointsPerCssX.isFinite, pointsPerCssY.isFinite,
              pointsPerCssX > 0, pointsPerCssY > 0,
              abs(pointsPerCssX - pointsPerCssY) <= 0.01 else {
            log("SPINON_C133_NATIVE_WHEEL_REJECT reason=invalid-coordinate-scale source=\(kind) points_per_css_px=\(pointsPerCssX),\(pointsPerCssY) generation=\(generation)")
            return
        }
        guard location.x.isFinite, location.y.isFinite,
              location.x >= bounds.minX, location.y >= bounds.minY,
              location.x < bounds.maxX, location.y < bounds.maxY else {
            log("SPINON_C133_NATIVE_WHEEL_REJECT reason=point-outside-surface source=\(kind) location_points=\(location.x),\(location.y) bounds_points=\(bounds.minX),\(bounds.minY),\(bounds.maxX),\(bounds.maxY) generation=\(generation)")
            return
        }

        let pointXCss = Float((location.x - bounds.minX) / pointsPerCssX)
        let pointYCss = Float((location.y - bounds.minY) / pointsPerCssY)
        let rawDeltaX = Float(delta.x)
        let rawDeltaY = Float(delta.y)
        let deltaXCss = Float(-delta.x / pointsPerCssX)
        let deltaYCss = Float(-delta.y / pointsPerCssY)
        let coordinateScale = Float(pointsPerCssX)
        guard pointXCss.isFinite, pointYCss.isFinite,
              rawDeltaX.isFinite, rawDeltaY.isFinite,
              deltaXCss.isFinite, deltaYCss.isFinite,
              coordinateScale.isFinite, coordinateScale > 0 else {
            log("SPINON_C133_NATIVE_WHEEL_REJECT reason=nonfinite-normalized-sample source=\(kind) generation=\(generation)")
            return
        }
        guard deltaXCss != 0 || deltaYCss != 0 else {
            log("SPINON_C133_NATIVE_WHEEL_REJECT reason=zero-delta source=\(kind) raw_delta_points=\(rawDeltaX),\(rawDeltaY) generation=\(generation)")
            return
        }

        let report = SpinonRunner.enqueueRuntimeGpuC13_3Wheel(
            handle,
            intent: 0,
            surfaceGeneration: generation,
            pointXCssPx: pointXCss,
            pointYCssPx: pointYCss,
            deltaXCssPx: deltaXCss,
            deltaYCssPx: deltaYCss,
            rawDeltaX: rawDeltaX,
            rawDeltaY: rawDeltaY,
            sourceUnit: 2,
            coordinateScale: coordinateScale,
            source: source,
            momentumPhase: 0,
            modifierFlags: normalizedModifiers
        ) ?? "status=-1 empty wheel enqueue report"
        log("SPINON_C133_NATIVE_WHEEL source=\(kind) state=\(recognizer.state.rawValue) generation=\(generation) location_points=\(location.x),\(location.y) point_css_px=\(pointXCss),\(pointYCss) raw_delta_points=\(rawDeltaX),\(rawDeltaY) coordinate_scale_points_per_css_px=\(coordinateScale) delta_css_px=\(deltaXCss),\(deltaYCss) native_modifiers=\(modifiers.rawValue) modifiers=\(normalizedModifiers) unit=UIKitPoints momentum_phase=None \(report)")
        if report.hasPrefix("status=0 ") && report.contains("admitted=true") {
            scheduleC133Drain()
        } else {
            log("SPINON_C133_NATIVE_WHEEL_REJECT reason=queue-rejected source=\(kind) generation=\(generation) report=\(report)")
        }
    }

    private func beginC133Pan(_ recognizer: UIPanGestureRecognizer, started: UInt64) {
        if c133SuppressUntilAllPointersUp || c133StyleBarrierPending { return }
        stateLock.lock()
        let ready = c133GestureFixtureReady && !closing && hostHandle != 0
        let generation = surfaceGeneration
        let widthCss = viewportWidthCssPx
        let heightCss = viewportHeightCssPx
        let drawable = c111MetalDrawableSize
        stateLock.unlock()
        guard ready, canvasView.bounds.width > 0, canvasView.bounds.height > 0,
              widthCss > 0, heightCss > 0, drawable.width > 0, drawable.height > 0 else {
            log("SPINON_C133_TOUCH_REJECT reason=surface-not-ready")
            return
        }

        let cssPerPointX = CGFloat(widthCss) / canvasView.bounds.width
        let cssPerPointY = CGFloat(heightCss) / canvasView.bounds.height
        let scaleX = Float(drawable.width / CGFloat(widthCss))
        let scaleY = Float(drawable.height / CGFloat(heightCss))
        guard scaleX.isFinite, scaleY.isFinite, scaleX > 0, scaleY > 0,
              abs(scaleX - scaleY) <= 0.01,
              abs(cssPerPointX - cssPerPointY) <= 0.01 else {
            log("SPINON_C133_TOUCH_REJECT reason=nonuniform-surface-scale scale_x=\(scaleX) scale_y=\(scaleY)")
            return
        }

        let current = recognizer.location(in: canvasView)
        let translation = recognizer.translation(in: canvasView)
        let origin = CGPoint(x: current.x - translation.x, y: current.y - translation.y)
        let originCss = CGPoint(x: origin.x * cssPerPointX, y: origin.y * cssPerPointY)
        let cumulativeCss = CGPoint(
            x: translation.x * cssPerPointX,
            y: translation.y * cssPerPointY
        )
        c133NextGestureID &+= 1
        if c133NextGestureID == 0 { c133NextGestureID = 1 }
        c133GestureID = c133NextGestureID
        c133PointerID = c133GestureID
        c133GestureSurfaceGeneration = generation
        c133CoordinateScale = scaleX
        c133LastCumulativeCss = .zero
        c133TerminalQueued = false
        c133PanBegan = false

        let beginReport = enqueueC133Gesture(
            action: 0,
            x: Float(originCss.x),
            y: Float(originCss.y),
            reason: 0
        )
        guard beginReport.hasPrefix("status=0 ") else {
            log("SPINON_C133_TOUCH_BEGIN_REJECTED \(beginReport)")
            return
        }
        c133TouchActive = true
        let panReport: String
        if cumulativeCss.x == 0, cumulativeCss.y == 0 {
            panReport = "pending-first-nonzero-translation"
        } else {
            if isC133StyleInvalidationScenario && !c133StyleMutationApplied {
                c133StyleBarrierPending = true
            }
            panReport = enqueueC133Gesture(
                action: 1,
                x: Float(cumulativeCss.x),
                y: Float(cumulativeCss.y),
                reason: 0
            )
            if !panReport.hasPrefix("status=0 ") {
                c133StyleBarrierPending = false
                log("SPINON_C133_TOUCH_PAN_BEGIN_REJECTED \(panReport)")
                cancelC133Gesture(reason: 2, source: "pan-begin-rejected")
                return
            }
            c133PanBegan = true
        }
        c133LastCumulativeCss = cumulativeCss
        if panReport.contains("GestureCancelledForCapacity") {
            c133TerminalQueued = true
            c133SuppressUntilAllPointersUp = true
        }
        scheduleC133Drain()
        logC133Callback(
            c133PanBegan ? "pan-began" : "pan-awaiting-translation",
            recognizer: recognizer,
            cumulative: cumulativeCss,
            report: "begin={\(beginReport)} pan={\(panReport)}",
            started: started
        )
    }

    private func moveC133Pan(_ recognizer: UIPanGestureRecognizer, started: UInt64) {
        if c133SuppressUntilAllPointersUp {
            logC133Callback("suppressed-move", recognizer: recognizer,
                            cumulative: .zero, report: "accepted=false", started: started)
            return
        }
        guard c133TouchActive, !c133TerminalQueued else { return }
        let translation = recognizer.translation(in: canvasView)
        let widthCss = CGFloat(viewportWidthCssPx)
        let heightCss = CGFloat(viewportHeightCssPx)
        guard canvasView.bounds.width > 0, canvasView.bounds.height > 0 else {
            cancelC133Gesture(reason: 3, source: "empty-surface")
            return
        }
        let cumulative = CGPoint(
            x: translation.x * widthCss / canvasView.bounds.width,
            y: translation.y * heightCss / canvasView.bounds.height
        )
        if !c133PanBegan, cumulative.x == 0, cumulative.y == 0 {
            logC133Callback("pan-awaiting-translation", recognizer: recognizer,
                            cumulative: cumulative, report: "accepted=true", started: started)
            return
        }
        if c133StyleBarrierPending {
            c133LastCumulativeCss = cumulative
            c133StyleDeferredMovePending = true
            c133StyleDeferredCumulativeCss = cumulative
            logC133Callback("style-barrier-deferred-move", recognizer: recognizer,
                            cumulative: cumulative, report: "accepted=deferred", started: started)
            return
        }
        let startsPan = !c133PanBegan
        if startsPan && isC133StyleInvalidationScenario && !c133StyleMutationApplied {
            c133StyleBarrierPending = true
        }
        let report = enqueueC133Gesture(
            action: startsPan ? 1 : 2,
            x: Float(cumulative.x),
            y: Float(cumulative.y),
            reason: 0
        )
        if report.hasPrefix("status=0 ") {
            c133LastCumulativeCss = cumulative
            if startsPan { c133PanBegan = true }
            if report.contains("GestureCancelledForCapacity") {
                c133TerminalQueued = true
                c133SuppressUntilAllPointersUp = true
            }
            scheduleC133Drain()
        } else {
            if startsPan { c133StyleBarrierPending = false }
            cancelC133Gesture(reason: 2, source: "move-rejected")
        }
        logC133Callback(startsPan ? "pan-began" : "move", recognizer: recognizer,
                        cumulative: cumulative, report: report, started: started)
    }

    private func endC133Pan(_ recognizer: UIPanGestureRecognizer, started: UInt64) {
        if c133StyleBarrierPending {
            if c133StyleDeferredTerminal == nil {
                c133StyleDeferredTerminal = (action: 3, reason: 0)
            }
            c133TouchActive = false
            c133TerminalQueued = true
            log("SPINON_C133_STYLE_BARRIER terminal=deferred action=\(c133StyleDeferredTerminal?.action ?? -1) gesture_id=\(c133GestureID)")
            return
        }
        guard c133TouchActive, !c133TerminalQueued else {
            if c133ActiveTouchIDs.isEmpty {
                resetC133GestureState(clearSuppression: true)
            }
            return
        }
        let report = enqueueC133Gesture(action: 3, x: 0, y: 0, reason: 0)
        c133TerminalQueued = report.hasPrefix("status=0 ")
        if c133TerminalQueued { scheduleC133Drain() }
        logC133Callback("up", recognizer: recognizer,
                        cumulative: c133LastCumulativeCss, report: report, started: started)
        c133TouchActive = false
        if c133ActiveTouchIDs.isEmpty {
            resetC133GestureState(clearSuppression: true)
        }
    }

    private func handleC133ActiveTouchCountChanged(_ count: Int) {
        guard c133GestureFixtureRequested else { return }
        if count > 1 {
            c133SuppressUntilAllPointersUp = true
            cancelC133Gesture(reason: 1, source: "second-pointer")
            log("SPINON_C133_TOUCH second-pointer active_contacts=\(count) suppressed=true")
        } else if count == 0, c133SuppressUntilAllPointersUp, !c133StyleBarrierPending {
            resetC133GestureState(clearSuppression: true)
        }
    }

    private func handleC133ApplicationDidEnterBackground() {
        for recognizer in [
            c133NativeWheelDiscreteRecognizer,
            c133NativeWheelContinuousRecognizer,
        ].compactMap({ $0 }) {
            recognizer.isEnabled = false
            recognizer.isEnabled = true
        }
        c133NativeWheelTranslations.removeAll()
        cancelC133Gesture(reason: 3, source: "application-background")
        c133ActiveTouchIDs.removeAll()
        // Keep the gesture identity until the in-flight style barrier submits its deferred cancel.
        if !c133StyleBarrierPending {
            resetC133GestureState(clearSuppression: true)
        }
    }

    private func cancelC133Gesture(reason: Int, source: String) {
        guard c133TouchActive, !c133TerminalQueued else { return }
        if c133StyleBarrierPending {
            c133StyleDeferredTerminal = (action: 4, reason: reason)
            c133TerminalQueued = true
            c133TouchActive = false
            log("SPINON_C133_STYLE_BARRIER terminal=deferred action=4 reason=\(reason) source=\(source)")
            return
        }
        let report = enqueueC133Gesture(action: 4, x: 0, y: 0, reason: reason)
        c133TerminalQueued = report.hasPrefix("status=0 ")
        if c133TerminalQueued { scheduleC133Drain() }
        log("SPINON_C133_TOUCH_CANCEL source=\(source) generation=\(c133GestureSurfaceGeneration) \(report)")
    }

    private func resetC133GestureState(clearSuppression: Bool) {
        c133TouchActive = false
        c133TerminalQueued = false
        c133GestureID = 0
        c133PointerID = 0
        c133LastCumulativeCss = .zero
        c133PanBegan = false
        if clearSuppression { c133SuppressUntilAllPointersUp = false }
    }

    private var isC133StyleInvalidationScenario: Bool {
        (8...10).contains(c133GesturePolicyScenarioID)
    }

    private func completeC133StyleBarrier(updateApplied: Bool) {
        guard c133StyleBarrierPending else { return }
        let terminal = c133StyleDeferredTerminal
        var queued = false
        if !updateApplied {
            let report = enqueueC133Gesture(action: 4, x: 0, y: 0, reason: 4)
            queued = report.hasPrefix("status=0 ")
            log("SPINON_C133_STYLE_BARRIER update_failed=true cancel=\(report)")
        } else {
            if c133StyleDeferredMovePending && terminal?.action != 4 {
                let report = enqueueC133Gesture(
                    action: 2,
                    x: Float(c133StyleDeferredCumulativeCss.x),
                    y: Float(c133StyleDeferredCumulativeCss.y),
                    reason: 0
                )
                queued = report.hasPrefix("status=0 ") || queued
                log("SPINON_C133_STYLE_BARRIER deferred_move=\(report)")
            }
            if let terminal {
                let report = enqueueC133Gesture(
                    action: terminal.action,
                    x: 0,
                    y: 0,
                    reason: terminal.reason
                )
                queued = report.hasPrefix("status=0 ") || queued
                log("SPINON_C133_STYLE_BARRIER deferred_terminal=\(report)")
            }
        }
        c133StyleBarrierPending = false
        c133StyleDeferredMovePending = false
        c133StyleDeferredCumulativeCss = .zero
        c133StyleDeferredTerminal = nil
        if queued { scheduleC133Drain() }
        if terminal != nil || !updateApplied {
            resetC133GestureState(clearSuppression: true)
        }
    }

    private func enqueueC133Gesture(action: Int, x: Float, y: Float, reason: Int) -> String {
        let handle = hostLifetime.load()
        guard handle != 0 else { return "status=-1 C13.3 host is closing" }
        return SpinonRunner.enqueueRuntimeGpuC13_3Gesture(
            handle,
            action: Int32(action),
            surfaceGeneration: c133GestureSurfaceGeneration,
            gestureID: c133GestureID,
            pointerID: c133PointerID,
            pointXCssPx: x,
            pointYCssPx: y,
            coordinateScale: c133CoordinateScale,
            reason: Int32(reason)
        ) ?? "status=-1 empty enqueue report"
    }

    private func runC133WheelProbe() {
        stateLock.lock()
        guard c133WheelProbeRequested, !c133WheelProbeStarted,
              c133GestureFixtureReady, !closing, hostHandle != 0 else {
            stateLock.unlock()
            return
        }
        c133WheelProbeStarted = true
        let handle = hostHandle
        let generation = surfaceGeneration
        let scenario = c133GesturePolicyScenarioID
        stateLock.unlock()

        let delta = scenario == 3 ? CGPoint(x: 0, y: 40) : CGPoint(x: 40, y: 40)
        let report = SpinonRunner.enqueueRuntimeGpuC13_3Wheel(
            handle,
            intent: 0,
            surfaceGeneration: generation,
            pointXCssPx: 132,
            pointYCssPx: 112,
            deltaXCssPx: Float(delta.x),
            deltaYCssPx: Float(delta.y),
            rawDeltaX: Float(delta.x),
            rawDeltaY: Float(delta.y),
            sourceUnit: 0,
            coordinateScale: 1,
            source: 1,
            momentumPhase: 2,
            modifierFlags: 0
        ) ?? "status=-1 empty wheel enqueue report"
        guard report.hasPrefix("status=0 "), report.contains("admitted=true") else {
            stateLock.lock()
            c133WheelProbeFailed = true
            stateLock.unlock()
            log("SPINON_C133_WHEEL_PROBE failed=true stage=enqueue \(report)")
            postStatus("실패 · C13.3 normalized wheel fixture enqueue · \(report)")
            return
        }
        stateLock.lock()
        c133WheelProbeEnqueued = true
        stateLock.unlock()
        log("SPINON_C133_WHEEL_PROBE_BEGIN synthetic_normalized_fixture_input=true scenario=\(scenario) point_css_px=132,112 delta_css_px=\(delta.x),\(delta.y) source=TrackpadScroll unit=CssPixels phase=Change scale=1 \(report)")
        scheduleC133Drain()
    }

    private func c133WheelProbeExpectedRuntimeOffsets() -> String {
        switch c133GesturePolicyScenarioID {
        case 1: return "offsets=root:0,0;outer:40,40;inner:86,106"
        case 2: return "offsets=root:0,0;outer:0,40;inner:86,106"
        case 3: return "offsets=root:0,0;outer:0,0;inner:0,40"
        default: return "offsets=invalid"
        }
    }

    private func c133WheelProbeExpectedDrawOffsets() -> String {
        switch c133GesturePolicyScenarioID {
        case 1: return "scroll_offsets=[0,0;40,40;86,106]"
        case 2: return "scroll_offsets=[0,0;0,40;86,106]"
        case 3: return "scroll_offsets=[0,0;0,0;0,40]"
        default: return "scroll_offsets=invalid"
        }
    }

    private func verifyC133WheelProcessReport(_ report: String) {
        guard report.contains("event=wheel") else { return }
        stateLock.lock()
        guard c133WheelProbeRequested, c133WheelProbeEnqueued,
              !c133WheelProbeProcessed, !c133WheelProbeFailed else {
            stateLock.unlock()
            return
        }
        let expected = c133WheelProbeExpectedRuntimeOffsets()
        if report.contains(expected) {
            c133WheelProbeProcessed = true
            stateLock.unlock()
            log("SPINON_C133_WHEEL_PROBE_PROCESSED synthetic_normalized_fixture_input=true expected=\(expected) report=\(report)")
        } else {
            c133WheelProbeFailed = true
            stateLock.unlock()
            log("SPINON_C133_WHEEL_PROBE failed=true stage=queue expected=\(expected) report=\(report)")
            postStatus("실패 · C13.3 wheel queue 결과가 oracle과 다릅니다")
        }
    }

    private func verifyC133WheelDraw(_ report: String) {
        stateLock.lock()
        guard c133WheelProbeRequested, c133WheelProbeProcessed,
              !c133WheelProbeCompleted, !c133WheelProbeFailed else {
            stateLock.unlock()
            return
        }
        let expectedOffsets = c133WheelProbeExpectedDrawOffsets()
        let expectedRevision = c133GesturePolicyScenarioID == 3
            ? "scroll_state_revision=1" : "scroll_state_revision=2"
        guard report.contains(expectedOffsets), report.contains(expectedRevision) else {
            c133WheelProbeFailed = true
            stateLock.unlock()
            log("SPINON_C133_WHEEL_PROBE failed=true stage=draw expected=\(expectedOffsets) \(expectedRevision) report=\(report)")
            postStatus("실패 · C13.3 wheel 뒤 Metal draw가 기대 offset과 다릅니다")
            return
        }
        c133WheelProbeCompleted = true
        stateLock.unlock()
        log("SPINON_C133_WHEEL_PROBE_COMPLETE synthetic_normalized_fixture_input=true expected=\(expectedOffsets) \(expectedRevision) report=\(report)")
        postStatus("완료 · synthetic normalized wheel fixture 결과를 Metal draw에서 확인했습니다")
    }

    private func scheduleC133Drain() {
        stateLock.lock()
        guard !closing, hostHandle != 0 else {
            stateLock.unlock()
            return
        }
        c133DrainAgain = true
        guard !c133DrainScheduled else {
            stateLock.unlock()
            return
        }
        c133DrainScheduled = true
        stateLock.unlock()

        runtimeQueue.async { [weak self] in
            guard let self else { return }
            while true {
                self.stateLock.lock()
                guard !self.closing, self.hostHandle != 0 else {
                    self.c133DrainScheduled = false
                    self.stateLock.unlock()
                    return
                }
                self.c133DrainAgain = false
                let handle = self.hostHandle
                self.stateLock.unlock()

                let report = SpinonRunner.processRuntimeGpuC13_3GestureQueue(handle)
                    ?? "status=-1 empty queue report"
                self.log("SPINON_C133_PROCESS \(report)")
                if report.hasPrefix("status=0 ") {
                    self.verifyC133WheelProcessReport(report)
                    if self.isC133StyleInvalidationScenario
                        && !self.c133StyleMutationApplied
                        && report.contains("result=pan_started") {
                        let styleReport = SpinonRunner.applyRuntimeGpuC13_3StyleInvalidation(handle)
                            ?? "status=-1 empty style update report"
                        let updateApplied = styleReport.hasPrefix("status=0 ")
                            && styleReport.contains("c13_3_style_invalidation=applied")
                        self.log("SPINON_C133_STYLE_UPDATE \(styleReport)")
                        if updateApplied { self.c133StyleMutationApplied = true }
                        DispatchQueue.main.async { [weak self] in
                            self?.completeC133StyleBarrier(updateApplied: updateApplied)
                        }
                    }
                    self.requestDraw()
                } else {
                    self.postStatus("실패 · C13.3 gesture queue · \(report)")
                }

                self.stateLock.lock()
                let shouldDrainAgain = self.c133DrainAgain
                if !shouldDrainAgain { self.c133DrainScheduled = false }
                self.stateLock.unlock()
                if !shouldDrainAgain { return }
            }
        }
    }

    private func logC133Callback(
        _ action: String,
        recognizer: UIPanGestureRecognizer,
        cumulative: CGPoint,
        report: String,
        started: UInt64
    ) {
        let elapsed = (DispatchTime.now().uptimeNanoseconds - started) / 1_000
        let location = recognizer.location(in: canvasView)
        log("SPINON_C133_TOUCH source=UIPanGestureRecognizer action=\(action) pointer_id=\(c133PointerID) point=\(location.x),\(location.y) cumulative_css_px=\(cumulative.x),\(cumulative.y) coordinate_scale=\(c133CoordinateScale) native_slop=system callback_us=\(elapsed) waits_for_worker=false \(report)")
    }
#endif

    private func failC131MetalPixelCapture(
        stage: String,
        error: Error
    ) -> C0410LatestTaskLaneError {
        c131MetalPixelCapture?.abort()
        c131MetalPixelBatchRequested = false
        c131MetalPixelCaptureFailed = true
        let reason = String(describing: error)
        log("SPINON_C131_METAL_PIXEL_CAPTURE_ERROR run_id=\(c131MetalCaptureRunID) stage=\(stage) reason=\(reason)")
        postStatus("실패 · C13.1 Metal pixel 표면 대조 \(stage)")
        return .operationFailed("C13.1 Metal pixel capture \(stage): \(reason)")
    }

    private func failC132MetalPixelReadback(
        stage: String,
        detail: String
    ) -> C0410LatestTaskLaneError {
        c132MetalPixelReadbackFinished = true
        let runID = c132MetalPixelRun?.runID ?? "missing"
        log("SPINON_C132_METAL_PIXEL_ERROR run_id=\(runID) stage=\(stage) detail=\(detail)")
        postStatus("실패 · C13.2 Metal surface 표본 \(stage)")
        return .operationFailed("C13.2 Metal surface readback \(stage): \(detail)")
    }

#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
    private func failC181MetalPixelReadback(
        stage: String,
        detail: String
    ) -> C0410LatestTaskLaneError {
        imagePaintMetalPixelReadbackFinished = true
        let label = imageAsyncReplacementProbeRequested ? "C18.4"
            : imageObjectFitPositionProbeRequested ? "C18.3"
            : imageAlphaProbeRequested ? "C18.2" : "C18.1"
        let prefix = imageAsyncReplacementProbeRequested ? "C184"
            : imageObjectFitPositionProbeRequested ? "C183"
            : imageAlphaProbeRequested ? "C182" : "C181"
        log("SPINON_\(prefix)_METAL_PIXEL_ERROR stage=\(stage) detail=\(detail)")
        postStatus("실패 · \(label) Metal surface 표본 \(stage)")
        return .operationFailed("\(label) Metal surface readback \(stage): \(detail)")
    }

    private func requestC181MetalPixelReadback(
        handle: UInt64,
        generation: UInt64
    ) throws {
        stateLock.lock()
        let drawableSize = c111MetalDrawableSize
        let viewportWidth = viewportWidthCssPx
        let viewportHeight = viewportHeightCssPx
        let format = rendererSurfaceFormat ?? ""
        stateLock.unlock()
        let expectedViewportWidth: Float = imageAsyncReplacementProbeRequested
            || imageObjectFitPositionProbeRequested ? 224 : 80
        let expectedViewportHeight: Float = imageAsyncReplacementProbeRequested ? 88
            : imageObjectFitPositionProbeRequested ? 72 : 40
        guard viewportWidth == expectedViewportWidth, viewportHeight == expectedViewportHeight,
              drawableSize.width > 0, drawableSize.height > 0,
              let width = Int(exactly: drawableSize.width),
              let height = Int(exactly: drawableSize.height) else {
            throw failC181MetalPixelReadback(
                stage: "surface-size",
                detail: "viewport 또는 Metal drawable 크기가 고정 이미지 fixture에 맞지 않습니다"
            )
        }
        let scaleX = Float(width) / viewportWidth
        let scaleY = Float(height) / viewportHeight
        guard scaleX.isFinite, scaleY.isFinite, scaleX > 0, scaleY > 0,
              abs(scaleX - scaleY) < 0.000001 else {
            throw failC181MetalPixelReadback(
                stage: "surface-scale",
                detail: "가로·세로 drawable scale이 다릅니다"
            )
        }
        var coordinates: [UInt32] = []
        guard let samples = imagePaintPixelSamples() else {
            throw failC181MetalPixelReadback(
                stage: "sample-config",
                detail: "활성 C18 image fixture의 Metal 표본 설정이 없습니다"
            )
        }
        coordinates.reserveCapacity(samples.count * 2)
        for sample in samples {
            let x = floor((sample.xCssPx + 0.5) * scaleX)
            let y = floor((sample.yCssPx + 0.5) * scaleY)
            guard x >= 0, y >= 0, x < Float(width), y < Float(height),
                  let sampleX = UInt32(exactly: x), let sampleY = UInt32(exactly: y) else {
                throw failC181MetalPixelReadback(
                    stage: "sample-bounds",
                    detail: "Metal drawable sample 범위를 벗어났습니다: \(sample.id)"
                )
            }
            coordinates.append(sampleX)
            coordinates.append(sampleY)
        }
        let request = coordinates.withUnsafeBufferPointer { buffer -> String? in
            if imageAsyncReplacementProbeRequested {
#if SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST_1
                return SpinonRunner.requestRuntimeGpuC18_4SurfacePixels(
                    forTest: handle, sampleXY: buffer.baseAddress,
                    sampleCount: UInt(coordinates.count / 2))
#else
                return "status=-90 C18.4 feature-disabled"
#endif
            }
            if imageObjectFitPositionProbeRequested {
#if SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST_1
                return SpinonRunner.requestRuntimeGpuC18_3SurfacePixels(
                    forTest: handle, sampleXY: buffer.baseAddress,
                    sampleCount: UInt(coordinates.count / 2))
#else
                return "status=-90 C18.3 feature-disabled"
#endif
            }
            if imageAlphaProbeRequested {
#if SPINON_ENABLE_C18_2_ALPHA_IMAGE_FIXTURE_TEST_1
                return SpinonRunner.requestRuntimeGpuC18_2SurfacePixels(
                    forTest: handle, sampleXY: buffer.baseAddress,
                    sampleCount: UInt(coordinates.count / 2))
#else
                return "status=-90 C18.2 feature-disabled"
#endif
            }
#if SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST_1
            return SpinonRunner.requestRuntimeGpuC18_1SurfacePixels(
                forTest: handle,
                sampleXY: buffer.baseAddress,
                sampleCount: UInt(coordinates.count / 2))
#else
            return "status=-90 C18.1 feature-disabled"
#endif
        } ?? "status=-1 missing-request-report"
        guard request.hasPrefix("status=0 ") else {
            throw failC181MetalPixelReadback(stage: "request", detail: request)
        }
        imagePaintMetalPixelRequestIssued = true
        let prefix = imageAsyncReplacementProbeRequested ? "C184"
            : imageObjectFitPositionProbeRequested ? "C183"
            : imageAlphaProbeRequested ? "C182" : "C181"
        log("SPINON_\(prefix)_METAL_PIXEL_REQUESTED generation=\(generation) viewport=\(Int(expectedViewportWidth))x\(Int(expectedViewportHeight)) drawable=\(width)x\(height) scale_x=\(scaleX) scale_y=\(scaleY) format=\(format) samples=\(samples.count)")
    }

    private func verifyC181MetalPixelReadback(
        result: String,
        generation: UInt64
    ) throws {
        func failure(_ stage: String, _ detail: String) -> C0410LatestTaskLaneError {
            failC181MetalPixelReadback(stage: stage, detail: detail)
        }
        guard let marker = result.range(of: "surface_pixel_samples_json=") else {
            throw failure("sample-missing", "draw report에 Metal 표본 JSON이 없습니다")
        }
        let tail = String(result[marker.upperBound...])
        guard let arrayStart = tail.firstIndex(of: "[") else {
            throw failure("sample-json", "표본 JSON 시작 괄호가 없습니다")
        }
        let nsTail = tail as NSString
        let tailUTF16 = tail.utf16
        guard let arrayStartUTF16 = arrayStart.samePosition(in: tailUTF16) else {
            throw failure("sample-json", "표본 JSON 시작 위치가 유효하지 않습니다")
        }
        let utf16 = Array(tailUTF16)
        let startOffset = tailUTF16.distance(from: tailUTF16.startIndex, to: arrayStartUTF16)
        var depth = 0
        var quoted = false
        var escaped = false
        var endOffset: Int?
        for offset in startOffset..<utf16.count {
            let unit = utf16[offset]
            if quoted {
                if escaped { escaped = false }
                else if unit == 92 { escaped = true }
                else if unit == 34 { quoted = false }
            } else if unit == 34 {
                quoted = true
            } else if unit == 91 {
                depth += 1
            } else if unit == 93 {
                depth -= 1
                if depth == 0 {
                    endOffset = offset
                    break
                }
            }
        }
        guard let endOffset else {
            throw failure("sample-json", "표본 JSON 닫는 괄호가 없습니다")
        }
        let json = nsTail.substring(with: NSRange(location: startOffset, length: endOffset - startOffset + 1))
        guard let samples = imagePaintPixelSamples() else {
            throw failure("sample-config", "활성 C18 image fixture의 Metal 표본 설정이 없습니다")
        }
        guard let data = json.data(using: String.Encoding.utf8),
              let values = try? JSONSerialization.jsonObject(with: data) as? [[String: Any]],
              values.count == samples.count else {
            throw failure("sample-shape", "표본 JSON이 고정 C18 image 표본 수와 다릅니다")
        }
        stateLock.lock()
        let drawableSize = c111MetalDrawableSize
        let format = rendererSurfaceFormat ?? ""
        stateLock.unlock()
        let expectedViewportWidth: Float = imageAsyncReplacementProbeRequested
            || imageObjectFitPositionProbeRequested ? 224 : 80
        let expectedViewportHeight: Float = imageAsyncReplacementProbeRequested ? 88
            : imageObjectFitPositionProbeRequested ? 72 : 40
        let scaleX = Float(drawableSize.width) / expectedViewportWidth
        let scaleY = Float(drawableSize.height) / expectedViewportHeight
        guard drawableSize.width > 0, drawableSize.height > 0,
              abs(scaleX - scaleY) < 0.000001 else {
            throw failure("surface-size", "표본 검증 시 drawable scale이 균일하지 않습니다")
        }
        var matched = 0
        for (index, sample) in samples.enumerated() {
            guard let xNumber = values[index]["x"] as? NSNumber,
                  let yNumber = values[index]["y"] as? NSNumber,
                  let channels = values[index]["rgba"] as? [NSNumber], channels.count == 4 else {
                throw failure("sample-shape", "Metal 표본 JSON 항목 형태가 올바르지 않습니다")
            }
            let x = xNumber.uint32Value
            let y = yNumber.uint32Value
            let rgba = channels.map { $0.uint32Value }
            let expectedX = UInt32(floor((sample.xCssPx + 0.5) * scaleX))
            let expectedY = UInt32(floor((sample.yCssPx + 0.5) * scaleY))
            let exact = x == expectedX && y == expectedY && rgba == sample.rgba
            if exact { matched += 1 }
            let prefix = imageAsyncReplacementProbeRequested ? "C184"
                : imageObjectFitPositionProbeRequested ? "C183"
                : imageAlphaProbeRequested ? "C182" : "C181"
            log("SPINON_\(prefix)_METAL_PIXEL_SAMPLE generation=\(generation) id=\(sample.id) xy=\(x),\(y) rgba=\(rgba.map(String.init).joined(separator: ",")) expected=\(sample.rgba.map(String.init).joined(separator: ",")) exact=\(exact)")
        }
        imagePaintMetalPixelReadbackFinished = true
        let passed = matched == samples.count
        let prefix = imageAsyncReplacementProbeRequested ? "C184"
            : imageObjectFitPositionProbeRequested ? "C183"
            : imageAlphaProbeRequested ? "C182" : "C181"
        log("SPINON_\(prefix)_METAL_PIXEL_SUMMARY status=\(passed ? "matched" : "mismatch") generation=\(generation) viewport=\(Int(expectedViewportWidth))x\(Int(expectedViewportHeight)) drawable=\(Int(drawableSize.width))x\(Int(drawableSize.height)) format=\(format) expected=\(samples.count) observed=\(values.count) matched=\(matched) renderer=metal-drawable-before-compositor")
        guard passed else {
            throw failure("pixel-mismatch", "Metal drawable RGBA가 Chrome 155 기준과 다릅니다")
        }
        postStatus(imageAsyncReplacementProbeRequested
            ? "완료 · C18.4 Metal drawable 표본 6/6"
            : imageObjectFitPositionProbeRequested
            ? "완료 · C18.3 Metal drawable 표본 19/19"
            : imageAlphaProbeRequested
            ? "완료 · C18.2 Metal drawable 표본 10/10"
            : "완료 · C18.1 Metal drawable 표본 9/9")
    }

    private func imagePaintPixelSamples() -> [C18MetalPixelSample]? {
        if imageAsyncReplacementProbeRequested {
#if SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST_1
            return c184MetalPixelSamples
#else
            return nil
#endif
        }
        if imageObjectFitPositionProbeRequested {
#if SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST_1
            return c183MetalPixelSamples
#else
            return nil
#endif
        }
        if imageAlphaProbeRequested {
#if SPINON_ENABLE_C18_2_ALPHA_IMAGE_FIXTURE_TEST_1
            return c182MetalPixelSamples
#else
            return nil
#endif
        }
#if SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST_1
        return c181MetalPixelSamples
#else
        return nil
#endif
    }
#endif

    private func drawLatestIfReady() throws {
        stateLock.lock()
        guard !closing, rendererReady, rendererSurfaceGeneration == surfaceGeneration,
              hostHandle != 0, let generation = rendererSurfaceGeneration else {
            stateLock.unlock()
            return
        }
        let handle = hostHandle
        stateLock.unlock()
        if shutdownProbeRequested && !shutdownProbeStarted {
            shutdownProbeStarted = true
            runShutdownProbeGate()
        }
        guard canUseSurfaceGeneration(generation) else { return }
        if let capture = c131MetalPixelCapture,
           !c131MetalPixelCaptureComplete,
           !c131MetalPixelCaptureFailed,
           c131MetalCaptureBaselineDrawReady,
           c131CompactHitIdentityGridStatusMessage?.hasPrefix("완료") == true,
           !c131MetalPixelBatchRequested {
            stateLock.lock()
            let drawableSize = c111MetalDrawableSize
            let viewportWidth = viewportWidthCssPx
            let viewportHeight = viewportHeightCssPx
            let format = rendererSurfaceFormat ?? ""
            stateLock.unlock()
            do {
                let expectedViewportWidth = c131StackingDescendantClipProbeRequested ? 540 : 300
                let expectedViewportHeight = c131StackingDescendantClipProbeRequested ? 128 : 100
                guard viewportWidth == Float(expectedViewportWidth),
                      viewportHeight == Float(expectedViewportHeight),
                      drawableSize.width > 0, drawableSize.height > 0,
                      let width = Int(exactly: drawableSize.width),
                      let height = Int(exactly: drawableSize.height) else {
                    throw C131MetalSurfacePixelCapture.CaptureError.invalidSurfaceSize
                }
                guard let batch = try capture.nextBatch(
                    drawableWidth: width,
                    drawableHeight: height,
                    generation: generation,
                    format: format
                ), !batch.isEmpty else {
                    throw C131MetalSurfacePixelCapture.CaptureError.batchMissing
                }
                var sampleXY: [UInt32] = []
                sampleXY.reserveCapacity(batch.count * 2)
                for sample in batch {
                    guard let x = UInt32(exactly: sample.drawableX),
                          let y = UInt32(exactly: sample.drawableY) else {
                        throw C131MetalSurfacePixelCapture.CaptureError.invalidSurfaceSize
                    }
                    sampleXY.append(x)
                    sampleXY.append(y)
                }
                let request = sampleXY.withUnsafeBufferPointer { buffer in
                    SpinonRunner.requestRuntimeGpuC13_1SurfacePixels(
                        forTest: handle,
                        sampleXY: buffer.baseAddress,
                        sampleCount: UInt(sampleXY.count / 2)
                    )
                } ?? "status=-1 missing-request-report"
                guard request.hasPrefix("status=0 ") else {
                    throw C0410LatestTaskLaneError.operationFailed(request)
                }
                c131MetalPixelBatchRequested = true
                log("SPINON_C131_METAL_PIXEL_BATCH_REQUESTED run_id=\(c131MetalCaptureRunID) batch=\(batch[0].index / 64) count=\(batch.count) generation=\(generation) drawable=\(width)x\(height)")
            } catch {
                throw failC131MetalPixelCapture(stage: "request", error: error)
            }
        }
        if let pixelRun = c111MetalPixelRun, !c111MetalPixelRequestIssued {
            let pixelMarker = "SPINON_\(pixelRun.markerPrefix)_METAL_PIXEL"
            stateLock.lock()
            let drawableSize = c111MetalDrawableSize
            let viewportWidth = viewportWidthCssPx
            let viewportHeight = viewportHeightCssPx
            stateLock.unlock()
            guard viewportWidth == 320, viewportHeight == 240,
                  drawableSize.width > 0, drawableSize.height > 0 else {
                let detail = "C11 Metal pixel viewport/drawable이 준비되지 않았습니다"
                log("\(pixelMarker)_ERROR run_id=\(pixelRun.runID) stage=surface-size")
                c111MetalPixelReadbackFinished = true
                throw C0410LatestTaskLaneError.operationFailed(detail)
            }
            let scaleX = Float(drawableSize.width) / viewportWidth
            let scaleY = Float(drawableSize.height) / viewportHeight
            guard scaleX.isFinite, scaleY.isFinite, scaleX > 0,
                  abs(scaleX - scaleY) < 0.000001 else {
                let detail = "C11 Metal pixel drawable scale의 가로·세로 비율이 다릅니다"
                log("\(pixelMarker)_ERROR run_id=\(pixelRun.runID) stage=surface-scale")
                c111MetalPixelReadbackFinished = true
                throw C0410LatestTaskLaneError.operationFailed(detail)
            }
            var coordinates: [UInt32] = []
            coordinates.reserveCapacity(pixelRun.samples.count * 2)
            for sample in pixelRun.samples {
                let x = floor((Float(sample.xCssPx) + 0.5) * scaleX)
                let y = floor((Float(sample.yCssPx) + 0.5) * scaleY)
                guard x >= 0, y >= 0, x < Float(drawableSize.width),
                      y < Float(drawableSize.height) else {
                    let detail = "C11 Metal pixel 표본이 drawable 범위를 벗어났습니다: \(sample.id)"
                    log("\(pixelMarker)_ERROR run_id=\(pixelRun.runID) stage=sample-bounds sample=\(sample.id)")
                    c111MetalPixelReadbackFinished = true
                    throw C0410LatestTaskLaneError.operationFailed(detail)
                }
                coordinates.append(UInt32(x))
                coordinates.append(UInt32(y))
            }
            let request = coordinates.withUnsafeBufferPointer { buffer in
                if pixelRun.markerPrefix == "C114" {
                    SpinonRunner.requestRuntimeGpuC11_4GridSurfacePixels(
                        forTest: handle,
                        sampleXY: buffer.baseAddress,
                        sampleCount: UInt(coordinates.count / 2)
                    )
                } else if pixelRun.markerPrefix == "C1133" {
                    SpinonRunner.requestRuntimeGpuC11_3_3GridSurfacePixels(
                        forTest: handle,
                        sampleXY: buffer.baseAddress,
                        sampleCount: UInt(coordinates.count / 2)
                    )
                } else if pixelRun.markerPrefix == "C1132" {
                    SpinonRunner.requestRuntimeGpuC11_3_2GridSurfacePixels(
                        forTest: handle,
                        sampleXY: buffer.baseAddress,
                        sampleCount: UInt(coordinates.count / 2)
                    )
                } else if pixelRun.markerPrefix == "C1131" {
                    SpinonRunner.requestRuntimeGpuC11_3_1GridSurfacePixels(
                        forTest: handle,
                        sampleXY: buffer.baseAddress,
                        sampleCount: UInt(coordinates.count / 2)
                    )
                } else if pixelRun.markerPrefix == "C1123" {
                    SpinonRunner.requestRuntimeGpuC11_2_3GridSurfacePixels(
                        forTest: handle,
                        sampleXY: buffer.baseAddress,
                        sampleCount: UInt(coordinates.count / 2)
                    )
                } else if pixelRun.markerPrefix == "C1122" {
                    SpinonRunner.requestRuntimeGpuC11_2_2GridSurfacePixels(
                        forTest: handle,
                        sampleXY: buffer.baseAddress,
                        sampleCount: UInt(coordinates.count / 2)
                    )
                } else {
                    SpinonRunner.requestRuntimeGpuC11_1GridSurfacePixels(
                        forTest: handle,
                        sampleXY: buffer.baseAddress,
                        sampleCount: UInt(coordinates.count / 2)
                    )
                }
            } ?? "status=-1 missing-request-report"
            guard request.hasPrefix("status=0 ") else {
                log("\(pixelMarker)_ERROR run_id=\(pixelRun.runID) stage=request report=\(request)")
                c111MetalPixelReadbackFinished = true
                throw C0410LatestTaskLaneError.operationFailed(request)
            }
            c111MetalPixelRequestIssued = true
            log("\(pixelMarker)_REQUESTED run_id=\(pixelRun.runID) scene=\(pixelRun.sceneID) generation=\(generation) drawable=\(Int(drawableSize.width))x\(Int(drawableSize.height)) scale_x=\(scaleX) scale_y=\(scaleY) samples=\(pixelRun.samples.count)")
        }
        if let pixelCase = c124MetalPixelCase, !c124MetalPixelReadbackFinished {
            stateLock.lock()
            let scale = displayScale
            stateLock.unlock()
            let pixelX = UInt32(((Float(pixelCase.cssSample.x) + 0.5) * scale).rounded(.down))
            let pixelY = UInt32(((Float(pixelCase.cssSample.y) + 0.5) * scale).rounded(.down))
            let request = SpinonRunner.requestRuntimeGpuSurfacePixelReadback(
                forTest: handle,
                x: pixelX, y: pixelY
            ) ?? "status=-1 missing-request-report"
            guard request.hasPrefix("status=0 ") else {
                log("SPINON_C124_METAL_PIXEL status=error run_id=\(pixelCase.runID) case=\(pixelCase.caseID) stage=request report=\(request)")
                c124MetalPixelReadbackFinished = true
                throw C0410LatestTaskLaneError.operationFailed(request)
            }
            log("SPINON_C124_METAL_PIXEL_REQUESTED run_id=\(pixelCase.runID) case=\(pixelCase.caseID) css_x=\(pixelCase.cssSample.x) css_y=\(pixelCase.cssSample.y) scale=\(scale) pixel_x=\(pixelX) pixel_y=\(pixelY)")
        }
        if let pixelRun = c132MetalPixelRun,
           !c132MetalPixelRequestIssued,
           !c132MetalPixelReadbackFinished {
            try requestC132MetalPixelReadback(
                run: pixelRun,
                handle: handle,
                surfaceGeneration: generation
            )
        }
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
        if imagePaintProbeRequested && !imagePaintMetalPixelRequestIssued
            && !imagePaintMetalPixelReadbackFinished {
            try requestC181MetalPixelReadback(handle: handle, generation: generation)
        }
#endif
#if SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST_1
        if c192BoxPaintOpacityProbeRequested && !c192MetalPixelRequestIssued
            && !c192MetalPixelReadbackFinished {
            try requestC192MetalPixelReadback(handle: handle, generation: generation)
        }
#endif
#if SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST_1
        if c153GlyphPaintProbeRequested && !c153MetalPixelRequestIssued
            && !c153MetalPixelReadbackFinished {
            try requestC153MetalPixelReadback(handle: handle, generation: generation)
        }
#endif
        let result = SpinonRunner.drawRuntimeGpuWgpu(handle)
        if c133TapProbeRequested {
            log("SPINON_C133_TAP_IOS_DRAW \(result ?? "missing-report")")
        }
        if c131MetalPixelCapture != nil {
            log("SPINON_C0410_DRAW generation=\(generation) status=\(result?.hasPrefix("status=0 ") == true ? "0" : "error") boxes=\(reportField(result ?? "", "boxes=")) surface_texture=\(reportField(result ?? "", "surface_texture="))")
        } else {
            log("SPINON_C0410_DRAW generation=\(generation) \(result ?? "")")
        }
        if result?.hasPrefix("status=-12 ") == true {
#if SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST_1
            if c153MetalPixelRequestIssued && !c153MetalPixelReadbackFinished {
                c153MetalPixelReadbackFinished = true
                log("SPINON_C153_METAL_PIXEL_ERROR stage=draw-superseded generation=\(generation)")
                throw C0410LatestTaskLaneError.operationFailed(
                    "C15.3 Metal 표본 요청 뒤 draw report가 stale 처리됐습니다"
                )
            }
#endif
#if SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST_1
            if c192MetalPixelRequestIssued && !c192MetalPixelReadbackFinished {
                c192MetalPixelReadbackFinished = true
                log("SPINON_C192_METAL_PIXEL_ERROR run_id=\(c192BoxPaintOpacityRunID) stage=draw-superseded")
                throw C0410LatestTaskLaneError.operationFailed(
                    "C19.2 Metal 표본 요청 뒤 draw report가 stale 처리됐습니다"
                )
            }
#endif
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
            if imagePaintMetalPixelRequestIssued && !imagePaintMetalPixelReadbackFinished {
                throw failC181MetalPixelReadback(
                    stage: "draw-superseded",
                    detail: "표본 요청 뒤 draw report가 stale 처리되었습니다"
                )
            }
#endif
            if c132MetalPixelRequestIssued && !c132MetalPixelReadbackFinished {
                throw failC132MetalPixelReadback(
                    stage: "draw-superseded",
                    detail: "표본 요청 뒤 draw report가 새 presentation에 의해 폐기됐습니다"
                )
            }
            if c131MetalPixelCapture != nil,
               c131MetalPixelBatchRequested,
               !c131MetalPixelCaptureComplete,
               !c131MetalPixelCaptureFailed {
                throw failC131MetalPixelCapture(
                    stage: "draw-superseded",
                    error: C0410LatestTaskLaneError.operationFailed("draw report가 새 presentation에 의해 폐기됐습니다")
                )
            }
            log("SPINON_C0410_DRAW_SUPERSEDED generation=\(generation)")
            requestDraw()
            return
        }
        if result?.hasPrefix("status=0 ") != true {
            postStatus("실패 · \(result ?? "WGPU draw 보고 없음")")
#if SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST_1
            if c153MetalPixelRequestIssued && !c153MetalPixelReadbackFinished {
                c153MetalPixelReadbackFinished = true
                log("SPINON_C153_METAL_PIXEL_ERROR stage=draw generation=\(generation) report=\(result ?? "missing")")
                throw C0410LatestTaskLaneError.operationFailed(
                    result ?? "C15.3 Metal 표본을 포함한 draw report가 없습니다"
                )
            }
#endif
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
            if imagePaintMetalPixelRequestIssued && !imagePaintMetalPixelReadbackFinished {
                throw failC181MetalPixelReadback(
                    stage: "draw",
                    detail: result ?? "WGPU draw 보고 없음"
                )
            }
#endif
#if SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST_1
            if c192MetalPixelRequestIssued && !c192MetalPixelReadbackFinished {
                c192MetalPixelReadbackFinished = true
                log("SPINON_C192_METAL_PIXEL_ERROR run_id=\(c192BoxPaintOpacityRunID) stage=draw report=\(result ?? "missing")")
                throw C0410LatestTaskLaneError.operationFailed(
                    result ?? "C19.2 Metal 표본을 포함한 draw report가 없습니다"
                )
            }
#endif
            if c132MetalPixelRequestIssued && !c132MetalPixelReadbackFinished {
                throw failC132MetalPixelReadback(
                    stage: "draw",
                    detail: result ?? "WGPU draw 보고 없음"
                )
            }
            if c131MetalPixelCapture != nil,
               !c131MetalPixelCaptureComplete,
               !c131MetalPixelCaptureFailed {
                throw failC131MetalPixelCapture(
                    stage: "draw",
                    error: C0410LatestTaskLaneError.operationFailed(result ?? "WGPU draw 보고 없음")
                )
            }
            throw C0410LatestTaskLaneError.operationFailed(result ?? "WGPU draw 실패 보고 없음")
        }
        if c131StackingDescendantClipProbeRequested,
           c131StackingDescendantClipStaleRevisionProbeRequested,
           !c131StackingClipStaleRevisionInjectionApplied,
           let documentRevision = UInt64(c131StackingClipExpectedRevisionFields["document_revision"] ?? ""),
           documentRevision < UInt64.max {
            c131StackingClipExpectedRevisionFields["document_revision"] = "\(documentRevision + 1)"
            c131StackingClipStaleRevisionInjectionApplied = true
            log("SPINON_C131_STACKING_CLIP_NEGATIVE_CONTROL name=stale-document-revision injected=true")
        }
        if c131StackingDescendantClipProbeRequested,
           c131StackingClipExpectedRevisionFields.count == 4,
           let staleField = c131StackingClipRevisionMismatch(in: result ?? "") {
            if c131StackingDescendantClipStaleRevisionProbeRequested {
                log("SPINON_C131_STACKING_CLIP_NEGATIVE_CONTROL name=stale-document-revision status=\(staleField == "document_revision" ? "rejected" : "error") field=\(staleField) surface_readback=not-requested")
            }
            throw failC131MetalPixelCapture(
                stage: "stale-revision",
                error: C131MetalSurfacePixelCapture.CaptureError.invalidReportField(staleField)
            )
        }
        if c131StackingDescendantClipProbeRequested,
           c131StackingClipExpectedRevisionFields.count == 4 {
            log("SPINON_C131_STACKING_CLIP_DRAW_IDENTITY run_id=\(c131StackingDescendantClipRunID) surface_generation=\(generation) \(c131StackingClipRuntimeRevisionSummary())")
        }
        if c153GlyphPaintProbeRequested
            && (c153HostDocumentTextPaintArgumentRequested
                ? (result?.contains("runtime_text_count=1") != true
                    || result?.contains("glyph_batch_draw_count=1") != true)
                : (result?.contains("runtime_text_count=6") != true
                    || result?.contains("glyph_batch_draw_count=6") != true)) {
            log("SPINON_C153_DRAW_ASSERTIONS failed=true \(result ?? "missing-report")")
            postStatus(c153HostDocumentTextPaintArgumentRequested
                ? "실패 · C15.3 HostDocument Ahem glyph batch가 제출되지 않았습니다"
                : "실패 · C15.3 Noto glyph batch 6개가 제출되지 않았습니다")
            throw C0410LatestTaskLaneError.operationFailed(
                "C15.3 WGPU draw report에 text batch가 없습니다"
            )
        }
#if SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST_1
        if c153GlyphPaintProbeRequested && c153MetalPixelRequestIssued
            && !c153MetalPixelReadbackFinished {
            try verifyC153MetalPixelReadback(
                result: result ?? "",
                generation: generation
            )
        }
#endif
        if rootHtmlTextPaintProbeRequested && !rootHtmlTextPaintReadbackFinished {
            try verifyC0412RootHtmlTextPaintReadback(result: result ?? "", generation: generation)
        }
#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1
        if imagePaintProbeRequested && imagePaintMetalPixelRequestIssued
            && !imagePaintMetalPixelReadbackFinished {
            try verifyC181MetalPixelReadback(result: result ?? "", generation: generation)
        }
#endif
#if SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST_1
        if c192BoxPaintOpacityProbeRequested && c192MetalPixelRequestIssued
            && !c192MetalPixelReadbackFinished {
            try verifyC192MetalPixelReadback(result: result ?? "", generation: generation)
        }
#endif
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
        stateLock.lock()
        let shouldVerifyC133InitialDraw = c133GestureFixtureRequested
            && c133GestureEvaluationReady && !c133GestureFixtureReady
        stateLock.unlock()
        if shouldVerifyC133InitialDraw {
            if result?.contains("presented empty scene while runtime calculation is pending") == true {
                log("SPINON_C133_DRAW_PENDING reason=runtime-snapshot-not-published")
                return
            }
            let scenarioID = c133GesturePolicyScenarioID
            let boundarySeed = scenarioID == 1 || scenarioID == 2
                || scenarioID == 8 || scenarioID == 9
            let expectedRevision = boundarySeed
                ? "scroll_state_revision=1" : "scroll_state_revision=0"
            let expectedOffsets = boundarySeed
                ? "scroll_offsets=[0,0;0,0;86,106]"
                : "scroll_offsets=[0,0;0,0;0,0]"
            guard result?.contains(expectedRevision) == true,
                  result?.contains(expectedOffsets) == true else {
                log("SPINON_C133_DRAW_ASSERTIONS failed=true \(result ?? "missing-report")")
                postStatus("실패 · C13.3 첫 Metal draw가 boundary seed와 다릅니다")
                throw C0410LatestTaskLaneError.operationFailed(
                    "C13.3 첫 draw snapshot이 gesture ready report와 일치하지 않습니다"
                )
            }
            stateLock.lock()
            c133GestureFixtureReady = true
            stateLock.unlock()
            let canvasOrigin = canvasView.convert(CGPoint.zero, to: nil)
            let screen = canvasView.window?.windowScene?.screen
            let screenBounds = screen?.bounds ?? CGRect.zero
            let screenScale = screen?.scale ?? 1
            log("SPINON_C133_READY input=UIPanGestureRecognizer native_slop=system canvas_origin_ui_points=\(canvasOrigin.x),\(canvasOrigin.y) canvas_bounds=\(canvasView.bounds.width)x\(canvasView.bounds.height) screen_bounds_ui_points=\(screenBounds.width)x\(screenBounds.height) screen_scale=\(screenScale) \(result ?? "")")
            let scenario = scenarioID == 0 ? "auto-zero"
                : scenarioID == 1 ? "auto-boundary"
                : scenarioID == 2 ? "contain-x-auto-y-boundary"
                : scenarioID == 3 ? "touch-action-none"
                : scenarioID == 4 ? "touch-action-pan-x"
                : scenarioID == 5 ? "touch-action-pan-y"
                : scenarioID == 6 ? "touch-action-target-pan-x"
                : scenarioID == 8 ? "style-policy-invalidation"
                : scenarioID == 9 ? "geometry-invalidation"
                : scenarioID == 10 ? "paint-invalidation"
                : "touch-action-intersection-none"
            postStatus("준비 · \(scenario) 상태에서 실제 iOS swipe 입력을 확인하세요")
            if c133WheelProbeRequested {
                DispatchQueue.main.async { [weak self] in self?.runC133WheelProbe() }
            }
        }
        if c133WheelProbeRequested { verifyC133WheelDraw(result ?? "") }
#endif
        if c131MetalPixelCapture != nil,
           !c131MetalCaptureBaselineDrawReady,
           !c131MetalPixelCaptureComplete,
           !c131MetalPixelCaptureFailed,
           c131CompactHitIdentityGridStatusMessage?.hasPrefix("완료") == true {
            c131MetalCaptureBaselineDrawReady = true
            log("SPINON_C131_METAL_PIXEL_BASELINE_DRAW_READY run_id=\(c131MetalCaptureRunID) generation=\(generation) boxes=\(reportField(result ?? "", "boxes=")) surface_texture=\(reportField(result ?? "", "surface_texture="))")
            requestDraw()
        }
        if let capture = c131MetalPixelCapture,
           c131MetalPixelBatchRequested,
           !c131MetalPixelCaptureComplete,
           !c131MetalPixelCaptureFailed {
            guard isCurrentSurfaceGeneration(generation) else {
                throw failC131MetalPixelCapture(
                    stage: "surface-generation",
                    error: C131MetalSurfacePixelCapture.CaptureError.generationChanged
                )
            }
            stateLock.lock()
            let drawableSize = c111MetalDrawableSize
            let format = rendererSurfaceFormat ?? ""
            stateLock.unlock()
            guard let width = Int(exactly: drawableSize.width),
                  let height = Int(exactly: drawableSize.height) else {
                throw failC131MetalPixelCapture(
                    stage: "surface-size",
                    error: C131MetalSurfacePixelCapture.CaptureError.invalidSurfaceSize
                )
            }
            do {
                let progress = try capture.acceptDrawReport(
                    result ?? "",
                    generation: generation,
                    drawableWidth: width,
                    drawableHeight: height,
                    format: format
                )
                c131MetalPixelBatchRequested = false
                if progress.complete {
                    c131MetalPixelCaptureComplete = true
                    log("SPINON_C131_METAL_PIXEL_CAPTURE_COMPLETE run_id=\(c131MetalCaptureRunID) generation=\(generation) samples=\(progress.capturedSamples) batches=\(progress.capturedBatches) sample_file=\(progress.sampleFileName ?? "missing") manifest_file=\(progress.manifestFileName ?? "missing")")
                    let mode = c131StackingDescendantClipProbeRequested
                        ? "고정 Chrome pixel 표본" : c131FullPhysicalMetalPixelsRequested
                        ? "전체 물리 pixel" : "CSS pixel center"
                    postStatus("완료 · C13.1 iOS Metal \(mode) \(progress.capturedSamples)/\(progress.totalSamples)")
                } else {
                    log("SPINON_C131_METAL_PIXEL_BATCH_COMPLETE run_id=\(c131MetalCaptureRunID) generation=\(generation) samples=\(progress.capturedSamples)/\(progress.totalSamples) batches=\(progress.capturedBatches)/\(progress.totalBatches)")
                    requestDraw()
                }
            } catch {
                throw failC131MetalPixelCapture(stage: "sample-validation", error: error)
            }
        }
        if let pixelRun = c132MetalPixelRun,
           c132MetalPixelRequestIssued,
           !c132MetalPixelReadbackFinished {
            try verifyC132MetalPixelReadback(
                result: result ?? "",
                run: pixelRun,
                surfaceGeneration: generation
            )
        }
        if let pixelRun = c111MetalPixelRun, !c111MetalPixelReadbackFinished {
            try verifyC111MetalPixelReadback(
                result: result ?? "",
                run: pixelRun,
                generation: generation
            )
        }
        if let pixelCase = c124MetalPixelCase, !c124MetalPixelReadbackFinished {
            let sampleXValue = reportField(result ?? "", "surface_pixel_x=")
            let sampleYValue = reportField(result ?? "", "surface_pixel_y=")
            let rgba = reportField(result ?? "", "surface_pixel_rgba=")
            let surfaceSize = reportField(result ?? "", "surface_texture=")
            guard let sampleX = UInt32(sampleXValue),
                  let sampleY = UInt32(sampleYValue),
                  !rgba.isEmpty, !surfaceSize.isEmpty else {
                log("SPINON_C124_METAL_PIXEL status=error run_id=\(pixelCase.runID) case=\(pixelCase.caseID) stage=missing-sample report=\(result ?? "missing")")
                c124MetalPixelReadbackFinished = true
                throw C0410LatestTaskLaneError.operationFailed("Metal surface pixel 표본이 draw 보고에 없습니다")
            }
            stateLock.lock()
            let scale = displayScale
            stateLock.unlock()
            let passed = rgba == pixelCase.expectedRGBA
                && sampleX == UInt32(((Float(pixelCase.cssSample.x) + 0.5) * scale).rounded(.down))
                && sampleY == UInt32(((Float(pixelCase.cssSample.y) + 0.5) * scale).rounded(.down))
            c124MetalPixelReadbackFinished = true
            let status = passed ? "ok" : "error"
            log("SPINON_C124_METAL_PIXEL status=\(status) run_id=\(pixelCase.runID) case=\(pixelCase.caseID) direction=\(pixelCase.direction) blue_order=\(pixelCase.blueOrder) red_order=\(pixelCase.redOrder) css_x=\(pixelCase.cssSample.x) css_y=\(pixelCase.cssSample.y) scale=\(scale) pixel_x=\(sampleX) pixel_y=\(sampleY) rgba=\(rgba) expected=\(pixelCase.expectedRGBA) surface=\(surfaceSize) renderer=metal-surface")
            guard passed else {
                postStatus("실패 · C12.4 Metal surface 색상·좌표 비교")
                throw C0410LatestTaskLaneError.operationFailed("Metal surface sample이 고정 Chrome 표본과 다릅니다")
            }
            postStatus("C12.4 Metal surface 표본 통과 · \(pixelCase.caseID)")
        }
        if failureProbeRequested && !failureProbeStarted {
            failureProbeStarted = true
            runFailureRecoveryProbe(handle: handle, baseline: result ?? "")
        }
    }

#if SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST_1
    private func c153OverflowSamplePoints() throws -> [(String, UInt32, UInt32)] {
        if c153HostDocumentTextPaintArgumentRequested {
            return try c153HostDocumentTextSamplePoints()
        }
        let encodings = [
            reportField(c153GlyphPaintEvaluationReport, "overflow_samples_css_px="),
            reportField(c153GlyphPaintEvaluationReport, "scroll_samples_css_px=")
        ]
        var points: [String: (UInt32, UInt32)] = [:]
        let acceptedNames: Set<String> = [
            "visible", "hidden_inside", "hidden_outside",
            "horizontal", "vertical", "nested"
        ]
        for encoded in encodings {
            for entry in encoded.split(separator: ";") {
                let pair = entry.split(separator: ":", maxSplits: 1)
                guard pair.count == 2 else {
                    throw C0410LatestTaskLaneError.operationFailed(
                        "C15.3 overflow/scroll 표본 항목이 올바르지 않습니다"
                    )
                }
                let coordinates = pair[1].split(separator: ",", maxSplits: 1)
                guard coordinates.count == 2,
                      let x = UInt32(coordinates[0]), let y = UInt32(coordinates[1]),
                      x < 360, y < 240 else {
                    throw C0410LatestTaskLaneError.operationFailed(
                        "C15.3 overflow/scroll CSS 표본 좌표가 올바르지 않습니다"
                    )
                }
                let name = String(pair[0])
                guard acceptedNames.contains(name), points[name] == nil else {
                    throw C0410LatestTaskLaneError.operationFailed(
                        "C15.3 overflow/scroll 표본 이름이 중복되거나 알 수 없습니다"
                    )
                }
                points[name] = (x, y)
            }
        }
        guard let visible = points["visible"],
              let hiddenInside = points["hidden_inside"],
              let hiddenOutside = points["hidden_outside"],
              let horizontal = points["horizontal"],
              let vertical = points["vertical"],
              let nested = points["nested"],
              points.count == 6,
              visible.0 == hiddenOutside.0,
              visible.1 + 40 == hiddenOutside.1,
              visible.0 >= 29, visible.1 >= 81, visible.1 < 91,
              hiddenInside.0 >= 13, hiddenInside.0 < 28,
              hiddenInside.1 >= 121, hiddenInside.1 < 131,
              hiddenOutside.0 >= 29,
              hiddenOutside.1 >= 121, hiddenOutside.1 < 131,
              horizontal.0 >= 41, horizontal.0 < 54,
              horizontal.1 >= 161, horizontal.1 < 181,
              vertical.0 >= 101, vertical.0 < 129,
              vertical.1 >= 161, vertical.1 < 174,
              nested.0 >= 257, nested.0 < 276,
              nested.1 >= 162, nested.1 < 181 else {
            throw C0410LatestTaskLaneError.operationFailed(
                "C15.3 overflow/scroll 표본이 paired clip·scrollport 경계를 증명하지 않습니다"
            )
        }
        return [
            ("visible-outside-parent", visible.0, visible.1),
            ("hidden-inside-clip", hiddenInside.0, hiddenInside.1),
            ("hidden-outside-parent", hiddenOutside.0, hiddenOutside.1),
            ("horizontal-scroll-glyph", horizontal.0, horizontal.1),
            ("vertical-scroll-glyph", vertical.0, vertical.1),
            ("nested-scroll-glyph", nested.0, nested.1)
        ]
    }

    private func c153HostDocumentTextSamplePoints() throws -> [(String, UInt32, UInt32)] {
        let encoded = reportField(
            c153GlyphPaintEvaluationReport,
            "hostdocument_text_samples_css_px="
        )
        let acceptedNames: Set<String> = [
            "first_ink_1", "first_ink_2", "overflow_ink_1", "overflow_ink_2",
            "overflow_ink_3", "overflow_ink_4"
        ]
        var points: [String: (UInt32, UInt32)] = [:]
        for entry in encoded.split(separator: ";") {
            let pair = entry.split(separator: ":", maxSplits: 1)
            guard pair.count == 2 else {
                throw C0410LatestTaskLaneError.operationFailed(
                    "C15.3 HostDocument 표본 항목이 올바르지 않습니다"
                )
            }
            let name = String(pair[0])
            let coordinates = pair[1].split(separator: ",", maxSplits: 1)
            guard acceptedNames.contains(name), coordinates.count == 2,
                  let x = UInt32(coordinates[0]), let y = UInt32(coordinates[1]),
                  x < 320, y < 240, points[name] == nil else {
                throw C0410LatestTaskLaneError.operationFailed(
                    "C15.3 HostDocument 표본 이름 또는 좌표가 올바르지 않습니다"
                )
            }
            points[name] = (x, y)
        }
        guard points.count == 6,
              let first1 = points["first_ink_1"], let first2 = points["first_ink_2"],
              let overflow1 = points["overflow_ink_1"],
              let overflow2 = points["overflow_ink_2"],
              let overflow3 = points["overflow_ink_3"],
              let overflow4 = points["overflow_ink_4"],
              Set(points.values.map { "\($0.0):\($0.1)" }).count == 6,
              first1.0 < 15, first2.0 < 15,
              overflow1.0 >= 15, overflow2.0 >= 15,
              overflow3.0 >= 15, overflow4.0 >= 15 else {
            throw C0410LatestTaskLaneError.operationFailed(
                "C15.3 HostDocument 표본이 15px owner 경계 양쪽을 증명하지 않습니다"
            )
        }
        return [
            ("first-glyph-ink-1", first1.0, first1.1),
            ("first-glyph-ink-2", first2.0, first2.1),
            ("overflow-glyph-ink-1", overflow1.0, overflow1.1),
            ("overflow-glyph-ink-2", overflow2.0, overflow2.1),
            ("overflow-glyph-ink-3", overflow3.0, overflow3.1),
            ("overflow-glyph-ink-4", overflow4.0, overflow4.1)
        ]
    }

    private func requestC153MetalPixelReadback(
        handle: UInt64,
        generation: UInt64
    ) throws {
        stateLock.lock()
        let drawableSize = c111MetalDrawableSize
        let viewportWidth = viewportWidthCssPx
        let viewportHeight = viewportHeightCssPx
        let format = rendererSurfaceFormat ?? ""
        stateLock.unlock()
        let expectedViewportWidth: Float = c153HostDocumentTextPaintArgumentRequested ? 320 : 360
        guard viewportWidth == expectedViewportWidth, viewportHeight == 240,
              drawableSize.width > 0, drawableSize.height > 0,
              let width = Int(exactly: drawableSize.width),
              let height = Int(exactly: drawableSize.height), !format.isEmpty,
              isCurrentSurfaceGeneration(generation) else {
            c153MetalPixelReadbackFinished = true
            throw C0410LatestTaskLaneError.operationFailed(
                "C15.3 Metal viewport 또는 drawable 크기가 올바르지 않습니다"
            )
        }
        do {
            let samples = try c153OverflowSamplePoints()
            let scaleX = Double(width) / Double(expectedViewportWidth)
            let scaleY = Double(height) / 240.0
            let coordinates = samples.flatMap { _, x, y in
                [
                    UInt32(floor((Double(x) + 0.5) * scaleX)),
                    UInt32(floor((Double(y) + 0.5) * scaleY))
                ]
            }
            guard coordinates.count == 12,
                  Set(stride(from: 0, to: coordinates.count, by: 2).map {
                      "\(coordinates[$0]):\(coordinates[$0 + 1])"
                  }).count == 6,
                  stride(from: 0, to: coordinates.count, by: 2).allSatisfy({
                      coordinates[$0] < UInt32(width) && coordinates[$0 + 1] < UInt32(height)
                  }) else {
                throw C0410LatestTaskLaneError.operationFailed(
                    "C15.3 Metal 표본 좌표가 중복되거나 drawable 밖입니다"
                )
            }
            let request = coordinates.withUnsafeBufferPointer { buffer in
                SpinonRunner.requestRuntimeGpuC15_3SurfacePixels(
                    forTest: handle,
                    sampleXY: buffer.baseAddress,
                    sampleCount: UInt(samples.count)
                )
            } ?? "status=-1 missing-request-report"
            guard request.hasPrefix("status=0 ") else {
                throw C0410LatestTaskLaneError.operationFailed(request)
            }
            c153MetalSamplePointsCss = samples
            c153MetalPixelRequestIssued = true
            c153MetalPixelRequestGeneration = generation
            c153MetalPixelRequestDrawableSize = drawableSize
            c153MetalPixelRequestFormat = format
            log("SPINON_C153_METAL_PIXEL_REQUESTED generation=\(generation) viewport=\(Int(expectedViewportWidth))x240 drawable=\(width)x\(height) scale_x=\(scaleX) scale_y=\(scaleY) format=\(format) samples=\(samples.count) css=\(samples.map { "\($0.0):\($0.1),\($0.2)" }.joined(separator: ";"))")
        } catch {
            c153MetalPixelReadbackFinished = true
            log("SPINON_C153_METAL_PIXEL_ERROR stage=request reason=\(error)")
            throw C0410LatestTaskLaneError.operationFailed(
                "C15.3 Metal 표본 요청 실패: \(error)"
            )
        }
    }

    private func verifyC153MetalPixelReadback(
        result: String,
        generation: UInt64
    ) throws {
        stateLock.lock()
        let drawableSize = c111MetalDrawableSize
        let format = rendererSurfaceFormat ?? ""
        stateLock.unlock()
        guard c153MetalPixelRequestGeneration == generation,
              c153MetalPixelRequestDrawableSize == drawableSize,
              c153MetalPixelRequestFormat == format,
              isCurrentSurfaceGeneration(generation),
              let width = Int(exactly: drawableSize.width),
              let height = Int(exactly: drawableSize.height),
              let encoded = reportField(result, "surface_pixel_samples_json=")
                .data(using: .utf8),
              let values = try? JSONSerialization.jsonObject(with: encoded)
                as? [[String: Any]], values.count == c153MetalSamplePointsCss.count,
              values.count == 6 else {
            c153MetalPixelReadbackFinished = true
            throw C0410LatestTaskLaneError.operationFailed(
                "C15.3 Metal drawable report가 요청된 표본 세 개를 돌려주지 않았습니다"
            )
        }
        let expectedViewportWidth: Double = c153HostDocumentTextPaintArgumentRequested ? 320 : 360
        let scaleX = Double(width) / expectedViewportWidth
        let scaleY = Double(height) / 240.0
        var visibleInk = false
        var hiddenInsideInk = false
        var hiddenOutsideBackground = false
        var horizontalScrollInk = false
        var verticalScrollInk = false
        var nestedScrollInk = false
        var firstGlyphInkSamples = 0
        var overflowGlyphInkSamples = 0
        for (index, sample) in c153MetalSamplePointsCss.enumerated() {
            let value = values[index]
            guard let xNumber = value["x"] as? NSNumber,
                  let yNumber = value["y"] as? NSNumber,
                  let channels = value["rgba"] as? [NSNumber], channels.count == 4 else {
                c153MetalPixelReadbackFinished = true
                throw C0410LatestTaskLaneError.operationFailed(
                    "C15.3 Metal 표본 JSON 형태가 올바르지 않습니다"
                )
            }
            let x = xNumber.uint32Value
            let y = yNumber.uint32Value
            let rgba = channels.map(\.uint32Value)
            let expectedX = UInt32(floor((Double(sample.1) + 0.5) * scaleX))
            let expectedY = UInt32(floor((Double(sample.2) + 0.5) * scaleY))
            let background = rgba == [247, 249, 252, 255]
            let expectedInk = rgba == [9, 33, 112, 255]
            if c153HostDocumentTextPaintArgumentRequested {
                if sample.0.hasPrefix("first-glyph-ink-") {
                    if expectedInk { firstGlyphInkSamples += 1 }
                    visibleInk = visibleInk || expectedInk && sample.1 < 15
                } else if sample.0.hasPrefix("overflow-glyph-ink-") {
                    if expectedInk { overflowGlyphInkSamples += 1 }
                    hiddenInsideInk = hiddenInsideInk || expectedInk && sample.1 >= 15
                }
            } else {
                if sample.0 == "visible-outside-parent" { visibleInk = expectedInk }
                else if sample.0 == "hidden-inside-clip" { hiddenInsideInk = expectedInk }
                else if sample.0 == "hidden-outside-parent" {
                    hiddenOutsideBackground = background
                } else if sample.0 == "horizontal-scroll-glyph" {
                    horizontalScrollInk = expectedInk
                } else if sample.0 == "vertical-scroll-glyph" {
                    verticalScrollInk = expectedInk
                } else if sample.0 == "nested-scroll-glyph" {
                    nestedScrollInk = expectedInk
                }
            }
            let exactCoordinates = x == expectedX && y == expectedY
            log("SPINON_C153_METAL_PIXEL_SAMPLE sample=\(sample.0) x=\(x) y=\(y) rgba=\(rgba.map(String.init).joined(separator: ",")) expected_x=\(expectedX) expected_y=\(expectedY) stage_background=\(background) expected_ink=\(expectedInk) coordinates_match=\(exactCoordinates)")
            guard exactCoordinates else {
                c153MetalPixelReadbackFinished = true
                throw C0410LatestTaskLaneError.operationFailed(
                    "C15.3 Metal drawable sample 좌표가 CSS pixel 중심 변환과 다릅니다"
                )
            }
        }
        c153MetalPixelReadbackFinished = true
        let passed = c153HostDocumentTextPaintArgumentRequested
            ? firstGlyphInkSamples == 2 && overflowGlyphInkSamples == 4
                && visibleInk && hiddenInsideInk
            : visibleInk && hiddenInsideInk && hiddenOutsideBackground
                && horizontalScrollInk && verticalScrollInk && nestedScrollInk
        if c153HostDocumentTextPaintArgumentRequested {
            log("SPINON_C153_HOSTDOCUMENT_METAL_PIXEL_SUMMARY status=\(passed ? "matched" : "mismatch") generation=\(generation) viewport=320x240 owner_width=15 text_advance=20 drawable=\(width)x\(height) format=\(format) samples=6 first_glyph_ink=\(firstGlyphInkSamples)/2 overflow_glyph_ink=\(overflowGlyphInkSamples)/4 owner_overflow_css_px=5 renderer=metal-drawable-before-compositor")
        } else {
            log("SPINON_C153_METAL_PIXEL_SUMMARY status=\(passed ? "matched" : "mismatch") generation=\(generation) viewport=360x240 drawable=\(width)x\(height) format=\(format) samples=6 visible_outside_ink=\(visibleInk) hidden_inside_ink=\(hiddenInsideInk) hidden_outside_background=\(hiddenOutsideBackground) horizontal_scroll_ink=\(horizontalScrollInk) vertical_scroll_ink=\(verticalScrollInk) nested_scroll_ink=\(nestedScrollInk) renderer=metal-drawable-before-compositor")
        }
        guard passed else {
            throw C0410LatestTaskLaneError.operationFailed(
                "실제 Metal drawable overflow clip 표본이 CSS 기대값과 다릅니다"
            )
        }
        postStatus(c153HostDocumentTextPaintArgumentRequested
            ? "C15.3 HostDocument text overflow Metal 표본 통과 · 6/6"
            : "C15.3 Metal drawable overflow/scroll 표본 통과 · 6/6")
    }
#endif

#if SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST_1
    private func requestC192MetalPixelReadback(
        handle: UInt64,
        generation: UInt64
    ) throws {
        guard let capture = c192MetalPixelCapture else {
            throw C0410LatestTaskLaneError.operationFailed(
                "C19.2 Metal pixel capture artifact가 초기화되지 않았습니다"
            )
        }
        stateLock.lock()
        let drawableSize = c111MetalDrawableSize
        let viewportWidth = viewportWidthCssPx
        let viewportHeight = viewportHeightCssPx
        let format = rendererSurfaceFormat ?? ""
        stateLock.unlock()
        guard viewportWidth == Float(C192MetalSurfacePixelCapture.viewportWidth),
              viewportHeight == Float(C192MetalSurfacePixelCapture.viewportHeight),
              let width = Int(exactly: drawableSize.width),
              let height = Int(exactly: drawableSize.height), !format.isEmpty else {
            c192MetalPixelReadbackFinished = true
            log("SPINON_C192_METAL_PIXEL_ERROR run_id=\(c192BoxPaintOpacityRunID) stage=surface-size viewport=\(viewportWidth)x\(viewportHeight) drawable=\(drawableSize.width)x\(drawableSize.height)")
            throw C0410LatestTaskLaneError.operationFailed(
                "C19.2 viewport 또는 Metal drawable 크기가 올바르지 않습니다"
            )
        }
        do {
            let coordinates = try capture.coordinates(
                drawableWidth: width,
                drawableHeight: height
            )
            let request = coordinates.withUnsafeBufferPointer { buffer in
                SpinonRunner.requestRuntimeGpuC19_2SurfacePixels(
                    forTest: handle,
                    sampleXY: buffer.baseAddress,
                    sampleCount: UInt(coordinates.count / 2)
                )
            } ?? "status=-1 missing-request-report"
            guard request.hasPrefix("status=0 ") else {
                throw C0410LatestTaskLaneError.operationFailed(request)
            }
            c192MetalPixelRequestIssued = true
            c192MetalPixelRequestGeneration = generation
            c192MetalPixelRequestDrawableSize = drawableSize
            c192MetalPixelRequestFormat = format
            log("SPINON_C192_METAL_PIXEL_REQUESTED run_id=\(c192BoxPaintOpacityRunID) generation=\(generation) viewport=360x240 drawable=\(width)x\(height) scale_x=\(Double(width) / 360.0) scale_y=\(Double(height) / 240.0) format=\(format) samples=16")
        } catch {
            c192MetalPixelReadbackFinished = true
            log("SPINON_C192_METAL_PIXEL_ERROR run_id=\(c192BoxPaintOpacityRunID) stage=request reason=\(error)")
            throw C0410LatestTaskLaneError.operationFailed(
                "C19.2 Metal 표본 요청 실패: \(error)"
            )
        }
    }

    private func verifyC192MetalPixelReadback(
        result: String,
        generation: UInt64
    ) throws {
        guard let capture = c192MetalPixelCapture else {
            c192MetalPixelReadbackFinished = true
            throw C0410LatestTaskLaneError.operationFailed(
                "C19.2 Metal pixel capture artifact가 없습니다"
            )
        }
        stateLock.lock()
        let drawableSize = c111MetalDrawableSize
        let format = rendererSurfaceFormat ?? ""
        let viewportWidth = viewportWidthCssPx
        let viewportHeight = viewportHeightCssPx
        stateLock.unlock()
        guard c192MetalPixelRequestGeneration == generation,
              c192MetalPixelRequestDrawableSize == drawableSize,
              c192MetalPixelRequestFormat == format,
              viewportWidth == Float(C192MetalSurfacePixelCapture.viewportWidth),
              viewportHeight == Float(C192MetalSurfacePixelCapture.viewportHeight),
              isCurrentSurfaceGeneration(generation),
              let width = Int(exactly: drawableSize.width),
              let height = Int(exactly: drawableSize.height) else {
            c192MetalPixelReadbackFinished = true
            log("SPINON_C192_METAL_PIXEL_ERROR run_id=\(c192BoxPaintOpacityRunID) stage=surface-identity generation=\(generation)")
            throw C0410LatestTaskLaneError.operationFailed(
                "C19.2 표본 요청 중 surface identity가 바뀌었습니다"
            )
        }
        do {
            let summary = try capture.accept(
                report: result,
                generation: generation,
                drawableWidth: width,
                drawableHeight: height,
                surfaceFormat: format
            )
            c192MetalPixelReadbackFinished = true
            log("SPINON_C192_METAL_PIXEL_SUMMARY status=\(summary.productPassed ? "product-matched" : "mismatch") run_id=\(c192BoxPaintOpacityRunID) generation=\(generation) viewport=360x240 drawable=\(width)x\(height) format=\(format) product_matched=\(summary.productMatched)/\(summary.sampleCount) chrome_exact=\(summary.chromeExact)/\(summary.sampleCount) known_chrome_delta=wideGamutSrgb-blue:-4 sample_file=\(summary.sampleFileName) manifest_file=\(summary.manifestFileName) sha256=\(summary.sampleSha256) renderer=metal-drawable-before-compositor")
            guard summary.productPassed else {
                postStatus("실패 · C19.2 iOS Simulator Metal product 표본 불일치")
                throw C0410LatestTaskLaneError.operationFailed(
                    "C19.2 Metal drawable이 Local MINDE product oracle과 다릅니다"
                )
            }
            postStatus("완료 · C19.2 iOS Simulator Metal product 16/16 · Chrome exact \(summary.chromeExact)/16")
        } catch {
            c192MetalPixelReadbackFinished = true
            log("SPINON_C192_METAL_PIXEL_ERROR run_id=\(c192BoxPaintOpacityRunID) stage=sample-validation reason=\(error)")
            postStatus("실패 · C19.2 iOS Simulator Metal 표본 검증 오류")
            if let laneError = error as? C0410LatestTaskLaneError { throw laneError }
            throw C0410LatestTaskLaneError.operationFailed(
                "C19.2 Metal 표본 검증 실패: \(error)"
            )
        }
    }
#endif

    private func requestC132MetalPixelReadback(
        run: C132MetalPixelRun,
        handle: UInt64,
        surfaceGeneration: UInt64
    ) throws {
        stateLock.lock()
        let assertionsPassed = c132ScrollAssertionsPassed
        let sequence = presentationSequence
        let viewportWidth = viewportWidthCssPx
        let viewportHeight = viewportHeightCssPx
        let drawableSize = c111MetalDrawableSize
        let format = rendererSurfaceFormat ?? ""
        stateLock.unlock()
        guard assertionsPassed else { return }
        guard isCurrentPresentation(generation: surfaceGeneration, sequence: sequence),
              viewportWidth == 400, viewportHeight == 240,
              drawableSize.width > 0, drawableSize.height > 0,
              let width = UInt32(exactly: drawableSize.width),
              let height = UInt32(exactly: drawableSize.height),
              !format.isEmpty else {
            throw failC132MetalPixelReadback(
                stage: "surface-identity",
                detail: "C13.2 viewport·drawable·format·presentation이 준비되지 않았습니다"
            )
        }
        let scaleX = Float(width) / Float(viewportWidth)
        let scaleY = Float(height) / Float(viewportHeight)
        guard scaleX.isFinite, scaleY.isFinite, scaleX > 0,
              abs(scaleX - scaleY) < 0.000001 else {
            throw failC132MetalPixelReadback(
                stage: "surface-scale",
                detail: "Metal drawable의 가로·세로 scale이 일치하지 않습니다"
            )
        }
        var coordinates: [UInt32] = []
        coordinates.reserveCapacity(run.samples.count * 2)
        for sample in run.samples {
            let x = floor((Float(sample.xCssPx) + 0.5) * scaleX)
            let y = floor((Float(sample.yCssPx) + 0.5) * scaleY)
            guard x >= 0, y >= 0, x < Float(width), y < Float(height) else {
                throw failC132MetalPixelReadback(
                    stage: "sample-bounds",
                    detail: "고정 표본이 Metal drawable 밖입니다: \(sample.id)"
                )
            }
            coordinates.append(UInt32(x))
            coordinates.append(UInt32(y))
        }
        let request = coordinates.withUnsafeBufferPointer { buffer in
            SpinonRunner.requestRuntimeGpuC13_2SurfacePixels(
                handle,
                sampleXY: buffer.baseAddress,
                sampleCount: UInt(run.samples.count)
            )
        } ?? "status=-1 missing-request-report"
        guard request.hasPrefix("status=0 ") else {
            throw failC132MetalPixelReadback(
                stage: "request",
                detail: request
            )
        }
        c132MetalPixelRequestIssued = true
        c132MetalPixelRequestGeneration = surfaceGeneration
        c132MetalPixelRequestPresentationSequence = sequence
        c132MetalPixelRequestDrawableSize = drawableSize
        c132MetalPixelRequestFormat = format
        log("SPINON_C132_METAL_PIXEL_REQUESTED run_id=\(run.runID) pid=\(ProcessInfo.processInfo.processIdentifier) surface_generation=\(surfaceGeneration) presentation_sequence=\(sequence) viewport=400x240 drawable=\(width)x\(height) format=\(format) scale_x=\(scaleX) scale_y=\(scaleY) samples=\(run.samples.count)")
    }

    private func verifyC132MetalPixelReadback(
        result: String,
        run: C132MetalPixelRun,
        surfaceGeneration: UInt64
    ) throws {
        let pid = ProcessInfo.processInfo.processIdentifier
        func failure(_ stage: String, _ detail: String) -> C0410LatestTaskLaneError {
            failC132MetalPixelReadback(stage: stage, detail: detail)
        }

        guard c132MetalPixelRequestGeneration == surfaceGeneration,
              let sequence = c132MetalPixelRequestPresentationSequence,
              isCurrentPresentation(generation: surfaceGeneration, sequence: sequence) else {
            throw failure("stale-presentation", "표면 세대 또는 presentation sequence가 바뀌었습니다")
        }
        let surface = reportField(result, "surface_texture=")
            .split(separator: "x").compactMap { UInt32($0) }
        stateLock.lock()
        let currentFormat = rendererSurfaceFormat ?? ""
        stateLock.unlock()
        guard let requestedWidth = Int(exactly: c132MetalPixelRequestDrawableSize.width),
              let requestedHeight = Int(exactly: c132MetalPixelRequestDrawableSize.height),
              surface.count == 2, surface[0] > 0, surface[1] > 0,
              surface[0] == UInt32(requestedWidth),
              surface[1] == UInt32(requestedHeight),
              c132MetalPixelRequestFormat == currentFormat,
              reportField(result, "boxes=") == "26",
              reportField(result, "viewport_css_px=") == "400x240" else {
            throw failure("surface-metadata", "Metal draw report의 boxes·viewport·drawable·format이 달라졌습니다")
        }
        let layoutGeneration = reportField(result, "generation=")
        let documentRevision = reportField(result, "document_revision=")
        let renderTreeRevision = reportField(result, "render_tree_revision=")
        let styleRevision = reportField(result, "style_revision=")
        let environmentRevision = reportField(result, "environment_revision=")
        let scrollRevision = reportField(result, "scroll_state_revision=")
        guard UInt64(layoutGeneration) != nil, UInt64(documentRevision) != nil,
              UInt64(renderTreeRevision) != nil, UInt64(styleRevision) != nil,
              UInt64(environmentRevision) != nil, UInt64(scrollRevision) != nil else {
            throw failure("revision", "draw report에 C13.2 layout·document·tree·style·environment·scroll revision이 없습니다")
        }
        guard let encoded = reportField(result, "surface_pixel_samples_json=").data(using: .utf8),
              let values = try? JSONSerialization.jsonObject(with: encoded) as? [[String: Any]],
              values.count == run.samples.count else {
            throw failure("sample-count", "draw report의 Metal 표본 수가 5개가 아닙니다")
        }
        let width = Float(surface[0])
        let height = Float(surface[1])
        let scaleX = width / 400.0
        let scaleY = height / 240.0
        let drawableText = "\(surface[0])x\(surface[1])"
        log("SPINON_C132_METAL_PIXEL_BEGIN run_id=\(run.runID) pid=\(pid) surface_generation=\(surfaceGeneration) presentation_sequence=\(sequence) layout_generation=\(layoutGeneration) document_revision=\(documentRevision) render_tree_revision=\(renderTreeRevision) style_revision=\(styleRevision) environment_revision=\(environmentRevision) scroll_state_revision=\(scrollRevision) boxes=26 viewport=400x240 drawable=\(drawableText) format=\(c132MetalPixelRequestFormat)")

        var matched = 0
        var escapeOutsideParent = false
        for (index, sample) in run.samples.enumerated() {
            guard let value = values[index] as? [String: Any],
                  let xNumber = value["x"] as? NSNumber,
                  let yNumber = value["y"] as? NSNumber,
                  let channels = value["rgba"] as? [NSNumber], channels.count == 4 else {
                throw failure("sample-shape", "Metal 표본 JSON 형태가 올바르지 않습니다: \(sample.id)")
            }
            let x = xNumber.uint32Value
            let y = yNumber.uint32Value
            let rgba = channels.map(\.uint32Value)
            let expectedX = UInt32(floor((Float(sample.xCssPx) + 0.5) * scaleX))
            let expectedY = UInt32(floor((Float(sample.yCssPx) + 0.5) * scaleY))
            let exact = x == expectedX && y == expectedY && rgba == sample.rgba
            if exact { matched += 1 }
            if sample.id == "visible-escape" {
                escapeOutsideParent = Float(x) + 0.5 > 130.0 * scaleX
            }
            log("SPINON_C132_METAL_PIXEL_SAMPLE run_id=\(run.runID) pid=\(pid) surface_generation=\(surfaceGeneration) sample=\(sample.id) x=\(x) y=\(y) rgba=\(rgba.map(String.init).joined(separator: ",")) expected=\(sample.rgba.map(String.init).joined(separator: ",")) exact=\(exact)")
        }
        c132MetalPixelReadbackFinished = true
        let passed = matched == run.samples.count && escapeOutsideParent
        log("SPINON_C132_METAL_PIXEL_SUMMARY status=\(passed ? "matched" : "mismatch") run_id=\(run.runID) pid=\(pid) surface_generation=\(surfaceGeneration) presentation_sequence=\(sequence) layout_generation=\(layoutGeneration) document_revision=\(documentRevision) render_tree_revision=\(renderTreeRevision) style_revision=\(styleRevision) environment_revision=\(environmentRevision) scroll_state_revision=\(scrollRevision) boxes=26 viewport=400x240 drawable=\(drawableText) format=\(c132MetalPixelRequestFormat) scale_x=\(scaleX) scale_y=\(scaleY) expected_samples=\(run.samples.count) observed_samples=\(values.count) matched_samples=\(matched) escape_outside_parent=\(escapeOutsideParent) renderer=metal-drawable-before-compositor")
        guard passed else {
            throw failure("pixel-mismatch", "실제 Metal drawable 표본이 고정 Chrome RGBA와 다릅니다")
        }
        postStatus("C13.2 Metal surface 표본 통과 · 5/5")
    }

    private func verifyC111MetalPixelReadback(
        result: String,
        run: C111MetalPixelRun,
        generation: UInt64
    ) throws {
        let pixelMarker = "SPINON_\(run.markerPrefix)_METAL_PIXEL"
        func failure(_ stage: String, _ detail: String) -> C0410LatestTaskLaneError {
            c111MetalPixelReadbackFinished = true
            log("\(pixelMarker)_ERROR run_id=\(run.runID) stage=\(stage) detail=\(detail)")
            postStatus("실패 · \(c11GridMetalPixelStatusName) Metal surface 표본 \(stage)")
            return .operationFailed(detail)
        }

        let surface = reportField(result, "surface_texture=")
            .split(separator: "x").compactMap { UInt32($0) }
        guard surface.count == 2, surface[0] > 0, surface[1] > 0 else {
            throw failure("surface-size", "WGPU draw report에 실제 surface 크기가 없습니다")
        }
        let viewport = (width: UInt32(320), height: UInt32(240))
        stateLock.lock()
        let drawableSize = c111MetalDrawableSize
        let viewportWidth = UInt32(viewportWidthCssPx)
        let viewportHeight = UInt32(viewportHeightCssPx)
        stateLock.unlock()
        guard viewportWidth == viewport.width, viewportHeight == viewport.height,
              drawableSize.width == CGFloat(surface[0]),
              drawableSize.height == CGFloat(surface[1]) else {
            throw failure("surface-generation", "viewport 또는 surface drawable 크기가 요청과 달라졌습니다")
        }
        let scaleX = Float(surface[0]) / Float(viewportWidth)
        let scaleY = Float(surface[1]) / Float(viewportHeight)
        guard scaleX.isFinite, scaleY.isFinite, scaleX > 0,
              abs(scaleX - scaleY) < 0.000001 else {
            throw failure("surface-scale", "drawable 가로·세로 pixel scale이 일치하지 않습니다")
        }
        let documentRevision = reportField(result, "document_revision=")
        let renderTreeRevision = reportField(result, "render_tree_revision=")
        let environmentRevision = reportField(result, "environment_revision=")
        let layoutGeneration = reportField(result, "generation=")
        guard UInt64(documentRevision) != nil, UInt64(renderTreeRevision) != nil,
              UInt64(environmentRevision) != nil, UInt64(layoutGeneration) != nil else {
            throw failure("revision", "draw report에 문서·트리·환경 revision이 없습니다")
        }
        guard let encodedSamples = reportField(result, "surface_pixel_samples_json=").data(using: .utf8),
              let values = try? JSONSerialization.jsonObject(with: encodedSamples) as? [[String: Any]],
              values.count == run.samples.count else {
            throw failure("sample-count", "draw report의 surface sample 개수가 실행 fixture와 다릅니다")
        }

        let pid = ProcessInfo.processInfo.processIdentifier
        let drawableText = "\(surface[0])x\(surface[1])"
        log("\(pixelMarker)_BEGIN run_id=\(run.runID) pid=\(pid) surface_generation=\(generation) layout_generation=\(layoutGeneration) document_revision=\(documentRevision) render_tree_revision=\(renderTreeRevision) environment_revision=\(environmentRevision) scene=\(run.sceneID) viewport=\(viewport.width)x\(viewport.height) drawable=\(drawableText)")

        var matchedCount = 0
        var records: [(String, UInt32, UInt32, [UInt32], [UInt32], Bool)] = []
        for (index, expected) in run.samples.enumerated() {
            guard let value = values[index] as? [String: Any],
                  let xNumber = value["x"] as? NSNumber,
                  let yNumber = value["y"] as? NSNumber,
                  let channels = value["rgba"] as? [NSNumber], channels.count == 4 else {
                throw failure("sample-shape", "surface 표본 JSON 형태가 올바르지 않습니다: \(expected.id)")
            }
            let actualX = xNumber.uint32Value
            let actualY = yNumber.uint32Value
            let actualRGBA = channels.map(\.uint32Value)
            let expectedX = UInt32(floor((Float(expected.xCssPx) + 0.5) * scaleX))
            let expectedY = UInt32(floor((Float(expected.yCssPx) + 0.5) * scaleY))
            let exact = actualX == expectedX && actualY == expectedY
                && actualRGBA == expected.rgba
            if exact { matchedCount += 1 }
            records.append((expected.id, actualX, actualY, actualRGBA, expected.rgba, exact))
        }

        for (id, x, y, actual, expected, exact) in records {
            let actualText = actual.map(String.init).joined(separator: ",")
            let expectedText = expected.map(String.init).joined(separator: ",")
            log("\(pixelMarker)_SAMPLE run_id=\(run.runID) pid=\(pid) surface_generation=\(generation) sample=\(id) x=\(x) y=\(y) rgba=\(actualText) expected=\(expectedText) exact=\(exact)")
        }
        let status = matchedCount == run.samples.count ? "matched" : "mismatch"
        log("\(pixelMarker)_SUMMARY status=\(status) run_id=\(run.runID) pid=\(pid) surface_generation=\(generation) document_revision=\(documentRevision) render_tree_revision=\(renderTreeRevision) environment_revision=\(environmentRevision) scene=\(run.sceneID) drawable=\(drawableText) scale_x=\(scaleX) scale_y=\(scaleY) expected_samples=\(run.samples.count) observed_samples=\(values.count) matched_samples=\(matchedCount)")
        c111MetalPixelReadbackFinished = true
        guard matchedCount == run.samples.count else {
            throw failure("pixel-mismatch", "실제 Metal surface 표본이 고정 Chrome RGBA와 다릅니다")
        }
        postStatus("\(c11GridMetalPixelStatusName) Metal surface 표본 통과 · \(matchedCount)/\(run.samples.count)")
    }

    private func runShutdownProbeGate() {
        let release = DispatchSemaphore(value: 0)
        for _ in 0..<10_000 { _ = renderDrawLane?.request() }
        let snapshot = renderDrawLane?.snapshot()
        log("SPINON_C0410_SHUTDOWN_PROBE_PENDING requests=10000 "
            + "scheduled=\(snapshot?.scheduled ?? false) "
            + "dirty=\(snapshot?.dirty ?? false) pending=\(snapshot?.pendingDrains ?? -1)")
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) { [weak self] in
            self?.beginShutdown()
        }
        DispatchQueue.global(qos: .utility).asyncAfter(deadline: .now() + 0.35) {
            release.signal()
        }
        release.wait()
        log("SPINON_C0410_SHUTDOWN_PROBE_GATE_RELEASED")
    }

    private func runFailureRecoveryProbe(handle: UInt64, baseline: String) {
#if SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE_1
        let armed = SpinonRunner.injectNextRuntimeGpuDrawFailure(handle)
        log("SPINON_C0410_FAILURE_PROBE_ARM \(armed ?? "")")
        guard armed?.hasPrefix("status=0 ") == true else {
            log("SPINON_C0410_FAILURE_PROBE_FAILED stage=arm")
            postStatus("실패 · draw 오류 주입을 설정하지 못했습니다")
            return
        }

        let expectedFailure = SpinonRunner.drawRuntimeGpuWgpu(handle)
        log("SPINON_C0410_FAILURE_PROBE_EXPECTED_ERROR \(expectedFailure ?? "")")
        guard expectedFailure?.hasPrefix("status=0 ") != true,
              expectedFailure?.contains("시험용으로 다음 draw를 실패") == true else {
            log("SPINON_C0410_FAILURE_PROBE_FAILED stage=expected-error")
            postStatus("실패 · 시험용 draw 오류가 예상대로 전달되지 않았습니다")
            return
        }

        let recovered = SpinonRunner.drawRuntimeGpuWgpu(handle)
        let baselineRevision = reportField(baseline, "environment_revision=")
        let recoveredRevision = reportField(recovered ?? "", "environment_revision=")
        let sameScene = !baselineRevision.isEmpty && baselineRevision == recoveredRevision
        let passed = recovered?.hasPrefix("status=0 ") == true && sameScene
        log("SPINON_C0410_FAILURE_PROBE_RECOVERY passed=\(passed) "
            + "baseline_revision=\(baselineRevision) recovered_revision=\(recoveredRevision) "
            + "\(recovered ?? "")")
        if passed {
            postStatus("draw 오류가 장면 revision을 바꾸지 않았고 다음 draw가 회복됐습니다")
        } else {
            log("SPINON_C0410_FAILURE_PROBE_FAILED stage=recovery")
            postStatus("실패 · draw 오류 뒤 정상 장면 복구가 확인되지 않았습니다")
        }
#else
        log("SPINON_C0410_FAILURE_PROBE_DISABLED · 내부 실패 fixture 빌드가 아닙니다")
        postStatus("실패 · draw 실패 검증용 빌드 옵션이 꺼져 있습니다")
#endif
    }

    private func reportField(_ report: String, _ field: String) -> String {
        guard let range = report.range(of: field) else { return "" }
        let value = report[range.upperBound...]
        return String(value.prefix { !$0.isWhitespace })
    }

    private func parseSurfaceDimensions(_ value: String) -> (width: UInt32, height: UInt32)? {
        let fields = value.split(separator: "x", omittingEmptySubsequences: false)
        guard fields.count == 2,
              let width = UInt32(fields[0]), let height = UInt32(fields[1]),
              width > 0, height > 0 else { return nil }
        return (width, height)
    }

    private func isCurrentSurfaceGeneration(_ generation: UInt64) -> Bool {
        stateLock.lock()
        defer { stateLock.unlock() }
        return !closing && surfaceGeneration == generation
    }

    private func canUseSurfaceGeneration(_ generation: UInt64) -> Bool {
        stateLock.lock()
        defer { stateLock.unlock() }
        return !closing && rendererReady && surfaceGeneration == generation
            && rendererSurfaceGeneration == generation
    }

    private func isCurrentPresentation(generation: UInt64, sequence: UInt64) -> Bool {
        stateLock.lock()
        defer { stateLock.unlock() }
        return !closing && surfaceGeneration == generation && presentationSequence == sequence
    }

    private func retryRendererCreation() {
        DispatchQueue.main.async { [weak self] in
            guard let self, !self.isClosing else { return }
            let size = self.lastDrawableSize
            guard size.width > 0, size.height > 0 else { return }
            self.ensureRenderer(width: Int(size.width), height: Int(size.height))
        }
    }

    private func postStatus(_ value: String) {
        DispatchQueue.main.async { [weak self] in
            guard let self, !self.isClosing else { return }
            self.statusLabel.text = value
        }
    }

    private func log(_ value: String) {
        NSLog("%@", value)
    }

    private func reportLaneFailure(_ lane: String, _ message: String) {
        log("SPINON_C0410_QUEUE_FAILURE lane=\(lane) message=\(message)")
        postStatus("실패 · \(lane) queue: \(message)")
    }

    private func beginPresentationUpdate() -> UInt64 {
        presentationUpdateLock.lock()
        defer { presentationUpdateLock.unlock() }
        stateLock.lock()
        guard !closing, hostHandle != 0,
            !(c131CompactSurfaceMatrixRouteRequested || c132ScrollFixtureRequested
                || c133GestureFixtureRequested)
                  || runtimeInitializationComplete else {
            stateLock.unlock()
            return 0
        }
        let handle = hostHandle
        stateLock.unlock()

        let sequence = SpinonRunner.beginRuntimeGpuPresentationUpdate(handle)
        stateLock.lock()
        if !closing && hostHandle == handle && sequence != 0 {
            presentationSequence = sequence
        }
        stateLock.unlock()
        return sequence
    }

    private func enqueueRuntime(_ work: @escaping () -> Void) {
        stateLock.lock()
        guard !closing else {
            stateLock.unlock()
            return
        }
        runtimeQueue.async(execute: work)
        stateLock.unlock()
    }

    private func enqueueRender(_ work: @escaping () -> Void) {
        stateLock.lock()
        guard !closing else {
            stateLock.unlock()
            return
        }
        renderQueue.async(execute: work)
        stateLock.unlock()
    }

    private func beginShutdown() {
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
        c133NativeWheelTranslations.removeAll()
        cancelC133Gesture(reason: 3, source: "view-disappear")
        if let observer = c133BackgroundObserver {
            NotificationCenter.default.removeObserver(observer)
            c133BackgroundObserver = nil
        }
#endif
        stateLock.lock()
        guard !closing else {
            stateLock.unlock()
            return
        }
        stateLock.unlock()
        _ = beginPresentationUpdate()
        stateLock.lock()
        guard !closing else {
            stateLock.unlock()
            return
        }
        surfaceGeneration &+= 1
        let destroyedGeneration = surfaceGeneration
        closing = true
        let runtimeQueue = self.runtimeQueue
        let renderQueue = self.renderQueue
        let hostLifetime = self.hostLifetime
        let retainedSurfaceView = canvasView
        stateLock.unlock()
        runtimePresentationLane?.close()
        renderDrawLane?.close()
        log("SPINON_C0410_SURFACE_DESTROYED generation=\(destroyedGeneration)")

        runtimeQueue.async {
            renderQueue.async {
                let handle = hostLifetime.take()
                if handle != 0 {
                    SpinonRunner.destroyRuntimeGpuRenderer(handle)
                    SpinonRunner.freeRuntimeGpuHost(handle)
                }
                _ = retainedSurfaceView
                NSLog("SPINON_C0410_DRAINED generation=%llu", destroyedGeneration)
            }
        }
    }

    deinit {
        beginShutdown()
    }
}

private enum C0410LatestTaskLaneError: Error, CustomStringConvertible {
    case operationFailed(String)

    var description: String {
        switch self {
        case let .operationFailed(message): message
        }
    }
}

private final class C0410RuntimeGpuHostLifetime {
    private let lock = NSLock()
    private var handle: UInt64 = 0

    func store(_ value: UInt64) {
        lock.lock()
        handle = value
        lock.unlock()
    }

    func load() -> UInt64 {
        lock.lock()
        defer { lock.unlock() }
        return handle
    }

    func take() -> UInt64 {
        lock.lock()
        defer { lock.unlock() }
        let value = handle
        handle = 0
        return value
    }
}

private final class C0410RuntimeGpuCanvasView: UIView {
#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
    var onActiveTouchCountChanged: ((Int) -> Void)?
    private var activeTouchIDs = Set<ObjectIdentifier>()
#endif

    override class var layerClass: AnyClass { CAMetalLayer.self }

#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        super.touchesBegan(touches, with: event)
        for touch in touches { activeTouchIDs.insert(ObjectIdentifier(touch)) }
        onActiveTouchCountChanged?(activeTouchIDs.count)
    }

    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        super.touchesEnded(touches, with: event)
        for touch in touches { activeTouchIDs.remove(ObjectIdentifier(touch)) }
        onActiveTouchCountChanged?(activeTouchIDs.count)
    }

    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        super.touchesCancelled(touches, with: event)
        for touch in touches { activeTouchIDs.remove(ObjectIdentifier(touch)) }
        onActiveTouchCountChanged?(activeTouchIDs.count)
    }
#endif
}
