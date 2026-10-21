package dev.spinon.bootstrap;

import android.app.Activity;
import android.graphics.Color;
import android.graphics.Point;
import android.graphics.Typeface;
import android.os.Build;
import android.os.Bundle;
import android.os.Debug;
import android.os.Handler;
import android.os.Looper;
import android.os.SystemClock;
import android.util.Log;
import android.view.Gravity;
import android.view.InputDevice;
import android.view.KeyEvent;
import android.view.MotionEvent;
import android.view.Surface;
import android.view.SurfaceHolder;
import android.view.SurfaceView;
import android.view.View;
import android.view.ViewGroup;
import android.view.ViewConfiguration;
import android.widget.Button;
import android.widget.LinearLayout;
import android.widget.TextView;

import org.json.JSONArray;
import org.json.JSONObject;

import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicBoolean;

final class C0410RuntimeGpuDemo extends LinearLayout implements SurfaceHolder.Callback {
    private static final String TAG = "SpinonBootstrap";
    private static final int C133_WHEEL_INTENT_SCROLL = 0;
    private static final int C133_WHEEL_UNIT_ANDROID_PHYSICAL_PIXELS = 1;
    private static final int C133_WHEEL_SOURCE_MOUSE = 0;
    private static final int C133_WHEEL_SOURCE_TRACKPAD = 1;
    private static final int C133_WHEEL_MOMENTUM_NONE = 0;
    private static final int C133_MODIFIER_SHIFT = 1;
    private static final int C133_MODIFIER_CONTROL = 2;
    private static final int C133_MODIFIER_ALT = 4;
    private static final int C133_MODIFIER_META = 8;
    private static final String[] C11_GRID_SCENE_IDS = {
            "fixed-tracks-gap", "percentage-tracks", "fractional-tracks-gap",
            "mixed-fixed-percent-fr", "row-auto-placement", "empty-grid",
            "single-item-grid", "grid-under-block", "grid-under-flex", "flex-under-grid",
    };
    private static final String[] C1121_GRID_PIXEL_SCENE_IDS = {
            "numeric-lines", "positive-spans", "reversed-lines",
            "shorthand-cascade", "overlap-source-order", "nested-placement-modes",
    };
    private static final String[] C1122_GRID_AUTO_PLACEMENT_SCENE_IDS = {
            "definite-first-mixed", "auto-only-fill", "positive-implicit-row",
            "row-locked-auto-column", "column-locked-auto-row", "auto-column-span",
            "sparse-hole-not-backfilled", "implicit-track-size-and-gap",
            "hidden-child-excluded", "nested-independent-placement", "reject-implicit-column",
            "reject-auto-sized-implicit-row", "reject-repeated-auto-row-pattern",
            "reject-dense-flow", "reject-column-flow", "column-cursor-wrap",
    };
    private static final String[] C1123_GRID_AUTO_PLACEMENT_SCENE_IDS = {
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
    };
    private static final String[] C1122_GRID_PIXEL_SCENE_IDS = {
            "definite-first-mixed", "auto-only-fill", "positive-implicit-row",
            "row-locked-auto-column", "column-locked-auto-row", "auto-column-span",
            "sparse-hole-not-backfilled", "implicit-track-size-and-gap",
            "hidden-child-excluded", "nested-independent-placement", "column-cursor-wrap",
    };
    private static final String[] C1123_GRID_PIXEL_SCENE_IDS = {
            "row-sparse-baseline", "row-dense-hole-fill", "row-dense-span-collision",
            "row-sparse-hole-control", "row-dense-fixed-column",
            "row-dense-positive-implicit-row", "column-sparse-explicit-fill",
            "column-sparse-positive-implicit-column", "column-sparse-fixed-row",
            "column-sparse-fixed-column", "column-dense-hole-fill",
            "column-sparse-hole-control", "column-dense-span-collision",
            "column-dense-positive-implicit-column", "definite-first-dense",
            "hidden-item-excluded", "nested-mixed-flow", "auto-track-no-implicit",
            "auto-span-at-track-limit",
    };
    private static final String C1131_GRID_REPEAT_SCENE_ID = "multi-track-repeat";
    private static final String C1132_GRID_NAMED_LINE_SCENE_ID = "positive-occurrence";
    private static final String C1132_GRID_NAMED_LINE_REJECT_SCENE_ID =
            "missing-name-implicit-line";
    private static final String C1133_GRID_TEMPLATE_AREAS_DEFAULT_SCENE_ID = "basic-rectangles";
    private static final String[] C1133_GRID_TEMPLATE_AREAS_SCENE_IDS = {
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
    };
    private static final String C1133_GRID_TEMPLATE_AREAS_REJECT_SCENE_ID =
            "area-track-count-mismatch";
    private static final String[] C114_GRID_TRACK_SIZING_SCENE_IDS = {
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
            "whitespace-does-not-create-item",
            "anonymous-text-image-text", "image-before-comment-group",
            "comment-group-before-image", "whitespace-around-image",
            "hidden-child-around-image",
            "nested-grid-text-image-row-span-two",
            "nested-grid-text-image-row-one-track",
            "nested-grid-text-image-row-definite-flex-control",
            "nested-grid-implicit-row-span-two",
            "nested-grid-implicit-row-one-row",
            "nested-depth-one-min-content", "nested-depth-two-min-content",
            "nested-depth-one-max-content", "nested-depth-two-max-content",
            "nested-depth-one-min-content-two-axis", "nested-depth-two-min-content-two-axis",
            "nested-depth-one-max-content-two-axis", "nested-depth-two-max-content-two-axis",
            "column-auto-natural-ratio", "column-target-max-width-clamp",
            "row-target-max-height-clamp",
            "column-opposite-min-over-max", "column-border-box-transfer",
            "column-content-box-transfer", "column-target-min-width-clamp",
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
            "anonymous-text-two-images-forward", "two-images-anonymous-text-reversed",
            "anonymous-text-three-images-forward", "three-images-anonymous-text-reversed",
            "anonymous-runs-two-images-forward", "two-images-anonymous-runs-reversed",
            "two-anonymous-runs-three-images-forward",
            "three-images-two-anonymous-runs-reversed",
            "nested-grid-span-two", "nested-grid-one-track",
            "nested-grid-definite-flex-control",
            "nested-grid-text-image-span-two", "nested-grid-text-image-one-track",
            "nested-grid-text-image-definite-flex-control",
    };
    private static final String[] C114_AUTO_REPEAT_NEGATIVE_LINE_SCENE_IDS = {
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
    };
    private static final String[][] C114_NEGATIVE_TRACK_EXPECTATIONS = {
            {"columns-one-negative", "189", "22", "47", "20", "4"},
            {"columns-two-negative", "223", "22", "31", "20", "5"},
            {"columns-three-negative", "273", "22", "47", "20", "6"},
            {"columns-four-negative", "273", "22", "31", "20", "6"},
            {"columns-negative-and-positive-cycle", "273", "22", "31", "20", "6"},
            {"columns-negative-fixed-single", "185", "22", "37", "20", "4"},
            {"columns-zero-track-cycle", "161", "22", "0", "20", "3"},
            {"columns-explicit-boundary-span", "155", "45", "90", "20", "4"},
            {"columns-reversed-lines-normalize", "223", "22", "81", "20", "4"},
            {"columns-negative-line-span", "223", "22", "81", "20", "4"},
            {"columns-no-explicit-tracks", "167", "22", "31", "20", "5"},
            {"columns-negative-minmax-track", "230", "22", "72", "20", "4"},
            {"columns-negative-min-content-track", "218", "22", "60", "20", "4"},
            {"columns-negative-max-content-track", "254", "22", "96", "20", "4"},
            {"rows-one-negative", "22", "157", "20", "29", "4"},
            {"rows-two-negative", "22", "177", "20", "17", "5"},
            {"rows-three-negative", "22", "209", "20", "29", "6"},
            {"rows-negative-and-positive-cycle", "22", "209", "20", "17", "6"},
            {"both-axes-negative-independent-patterns", "189", "137", "47", "29", "4"},
            {"hidden-negative-does-not-expand", "139", "22", "40", "20", "3"},
            {"negative-column-auto-row-placement", "189", "22", "47", "20", "4"},
            {"columns-negative-grid-auto-items", "189", "45", "31", "20", "6"},
            {"rows-negative-grid-auto-items", "22", "177", "20", "17", "6"},
            {"columns-negative-row-dense-auto-items", "189", "45", "31", "20", "6"},
            {"columns-negative-row-sparse-cursor", "155", "45", "47", "20", "5"},
            {"columns-negative-row-dense-cursor", "155", "45", "47", "20", "5"},
            {"columns-negative-column-flow-auto-items", "189", "45", "31", "20", "6"},
            {"columns-negative-column-dense-auto-items", "189", "45", "31", "20", "6"},
            {"rows-negative-row-dense-auto-items", "22", "177", "20", "17", "6"},
            {"rows-negative-column-flow-auto-items", "45", "157", "20", "17", "6"},
            {"rows-negative-column-dense-auto-items", "45", "157", "20", "17", "6"},
            {"columns-negative-fixed-root-overflow", "100", "22", "31", "20", "6"},
            {"rows-negative-fixed-root-overflow", "22", "100", "20", "17", "6"},
    };
    private static final String[] C131_COMPACT_SURFACE_CASE_IDS = {
            "visible", "clip-x", "clip-y", "clip-both", "clip-margin-padding",
            "clip-margin-content", "clip-margin-border", "clip-margin-hidden",
            "mixed-visible-hidden", "mixed-clip-auto", "positioned-contained",
            "positioned-escape", "nested-clip",
    };
    private static final String[] C131_COMPACT_SURFACE_SAMPLE_ROLES = {
            "paint", "target-beyond-frame", "x-overflow", "y-overflow",
    };
    private static final String[] C131_STACKING_CLIP_SAMPLE_ROLES = {
            "parent-only", "target-inside", "clip-edge-inside", "clip-edge-outside",
            "escape-before-overlay", "stacking-overlay",
    };
    private static final String[] C131_STACKING_CLIP_CASE_IDS = {"visible", "clip", "hidden"};
    private static final double[] C131_STACKING_CLIP_SAMPLE_X = {
            44.5, 122.5, 139.5, 140.5, 144.5, 156.5,
            214.5, 292.5, 309.5, 310.5, 314.5, 326.5,
            384.5, 462.5, 479.5, 480.5, 484.5, 496.5,
    };
    private static final String[] C131_STACKING_CLIP_HIT_IDS = {
            "visible-parent", "visible-target", "visible-target", "visible-target",
            "visible-target", "visible-overlay", "clip-parent", "clip-target",
            "clip-target", "root", "root", "clip-overlay", "hidden-parent",
            "hidden-target", "hidden-target", "root", "root", "hidden-overlay",
    };
    private static final int[] C131_STACKING_CLIP_NODE_INDEX = {
            1, 3, 3, 3, 3, 4, 5, 7, 7, 0, 0, 8, 9, 11, 11, 0, 0, 12,
    };
    private static final String C131_COMPACT_HIT_IDENTITY_SHA256 =
            "93cccd042e82d8cd30cb7a8ab34a5154073d4a9eccbc4038294d3ca5b9dfce87";
    private static final String C131_COMPACT_GRID_ORACLE_SHA256 =
            "35e88e57edc16e526e389317b0d18ef227e35d1356a2353b92476c75ef6414f6";
    private static final String C131_COMPACT_CHROME_OBSERVATION_SHA256 =
            "515fc95f811399fda9e336a13e7583f2d22555fce4727f1fe7f734981abd0299";
    private static final String C131_STACKING_FULL_VIEWPORT_IDENTITY_SHA256 =
            "866073e0c6f092d4b2459fa3b5cfd2ca38dfb8935f582ee3c9614dba3bfc7483";
    private static final String C131_STACKING_FULL_VIEWPORT_ORACLE_SHA256 =
            "279364905336ef53d225c781bae68ec26d3fe4d50cfc7d0cfce689ab5354eee2";
    private static final String[] C131_STACKING_FULL_VIEWPORT_NODE_IDS = {
            "root", "visible-parent", "visible-layer", "visible-target", "visible-overlay",
            "clip-parent", "clip-layer", "clip-target", "clip-overlay", "hidden-parent",
            "hidden-layer", "hidden-target", "hidden-overlay",
    };
    private static boolean isC1132GridNamedLinePixelSamples(
            List<SurfaceViewPixelCapture.Sample> samples
    ) {
        if (samples.size() != 2) return false;
        boolean background = false;
        boolean blue = false;
        for (SurfaceViewPixelCapture.Sample sample : samples) {
            switch (sample.id) {
                case "grid-background":
                    background = sample.xCssPx == 20 && sample.yCssPx == 80;
                    if (!background) return false;
                    break;
                case "item-blue":
                    blue = sample.xCssPx == 100 && sample.yCssPx == 10;
                    if (!blue) return false;
                    break;
                default:
                    return false;
            }
        }
        return background && blue;
    }
    private static boolean isC1133GridTemplateAreasScene(String sceneId) {
        for (String candidate : C1133_GRID_TEMPLATE_AREAS_SCENE_IDS) {
            if (candidate.equals(sceneId)) return true;
        }
        return "revision-reuse".equals(sceneId);
    }
    static boolean isC114GridTrackSizingScene(String sceneId) {
        for (String candidate : C114_GRID_TRACK_SIZING_SCENE_IDS) {
            if (candidate.equals(sceneId)) return true;
        }
        return false;
    }

    private static String[] c114NegativeTrackExpectation(String sceneId) {
        for (String[] expectation : C114_NEGATIVE_TRACK_EXPECTATIONS) {
            if (expectation[0].equals(sceneId)) return expectation;
        }
        return null;
    }

    private static String c114NegativeTrackRootFrame(String sceneId) {
        String[] expectation = c114NegativeTrackExpectation(sceneId);
        return expectation == null ? null
                : "width=" + expectation[1] + ",height=" + expectation[2];
    }

    private static String c114NegativeTrackItemFrame(String sceneId) {
        String[] expectation = c114NegativeTrackExpectation(sceneId);
        return expectation == null ? null
                : "width=" + expectation[3] + ",height=" + expectation[4];
    }

    private static String c114NegativeTrackNodeFrames(String sceneId) {
        return null;
    }

    private static String c114ExpectedRootFrame(String sceneId) {
        sceneId = c114BaseNestedGridSceneId(sceneId);
        if (sceneId.equals("nested-depth-one-min-content")
                || sceneId.equals("nested-depth-two-min-content")) {
            return "width=70,height=20";
        }
        if (sceneId.equals("nested-depth-one-max-content")
                || sceneId.equals("nested-depth-two-max-content")) {
            return "width=100,height=10";
        }
        if (sceneId.equals("nested-grid-implicit-row-span-two")) {
            return "width=24,height=40";
        }
        if (sceneId.equals("nested-grid-implicit-row-one-row")) {
            return "width=24,height=44";
        }
        if (sceneId.equals("nested-grid-text-image-span-two")) return "width=98,height=20";
        if (sceneId.equals("nested-grid-text-image-one-track")) return "width=102,height=20";
        if (sceneId.equals("nested-grid-text-image-definite-flex-control")) {
            return "width=120,height=20";
        }
        if (sceneId.equals("nested-grid-text-image-row-span-two")) {
            return "width=98,height=40";
        }
        if (sceneId.equals("nested-grid-text-image-row-one-track")) {
            return "width=98,height=44";
        }
        if (sceneId.equals("nested-grid-text-image-row-definite-flex-control")) {
            return "width=98,height=60";
        }
        if (sceneId.equals("nested-grid-span-two")) return "width=134,height=20";
        if (sceneId.equals("nested-grid-one-track")) return "width=138,height=20";
        if (sceneId.equals("nested-grid-definite-flex-control")) {
            return "width=120,height=20";
        }
        if (c114IsAutoRepeatNegativeLineScene(sceneId)) return "";
        if (sceneId.equals("anonymous-min-content-wrap")) return "width=120,height=20";
        if (c114IsAnonymousGridBorderOverflowScene(sceneId)) return "width=40,height=184";
        if (c114IsAnonymousTextMultiImageScene(sceneId)) return "width=220,height=180";
        if (c114IsAnonymousTextImageScene(sceneId)) return "width=220,height=60";
        if (sceneId.equals("anonymous-comment-continuation")
                || sceneId.equals("anonymous-runs-around-block")
                || sceneId.equals("anonymous-hidden-child-continuation")
                || sceneId.equals("whitespace-does-not-create-item")) {
            return "width=120,height=10";
        }
        String implicitTrackFrame = c114ImplicitTextRootFrame(sceneId);
        if (implicitTrackFrame != null) return implicitTrackFrame;
        String negativeTrackFrame = c114NegativeTrackRootFrame(sceneId);
        if (negativeTrackFrame != null) return negativeTrackFrame;
        String autoFitMinimumSizeFrame = c114AutoFitMinimumSizeRootFrame(sceneId);
        if (autoFitMinimumSizeFrame != null) return autoFitMinimumSizeFrame;
        switch (sceneId) {
            case "column-auto-natural-ratio":
            case "column-target-max-width-clamp":
            case "column-opposite-min-over-max":
            case "column-border-box-transfer":
            case "column-content-box-transfer":
            case "column-target-min-width-clamp":
                return "width=8,height=20";
            case "row-target-max-height-clamp":
                return "width=20,height=4";
            case "row-transfer-input-capped-by-fixed-column-span":
                return "width=8,height=4";
            case "column-transfer-input-capped-by-fixed-row-span":
                return "width=4,height=8";
            case "fixed-gutter-column-suggestion-clamp":
                return "width=12,height=6";
            case "fixed-gutter-row-suggestion-clamp":
                return "width=6,height=12";
            case "text-span2-forward":
            case "text-span2-reversed":
            case "text-span2-before-span3":
            case "text-span3-before-span2":
                return "width=220,height=60";
            case "mixed-text-image-span2-forward":
            case "mixed-text-image-span2-reversed":
            case "mixed-text-image-span2-before-span3":
            case "mixed-text-image-span3-before-span2":
                return "width=220,height=180";
            case "image-span2-forward":
            case "image-span2-reversed":
            case "image-span2-before-span3":
            case "image-span3-before-span2":
                return "width=40,height=60";
            case "grid-gap-definite-percent":
            case "grid-gap-definite-calc":
                return "width=100,height=40";
            case "grid-gap-cyclic-percent":
                return "width=100,height=20";
            case "grid-gap-cyclic-calc":
                return "width=105,height=25";
            case "grid-gap-cyclic-percent-min-basis":
            case "grid-gap-cyclic-calc-min-basis":
                return "width=120,height=30";
            case "auto-fill-fixed-neighbors":
            case "auto-fit-fixed-neighbors":
            case "auto-fill-pattern-neighbors":
                return "width=200,height=60";
            case "auto-fit-span-neighbors":
            case "auto-fill-prefix-neighbors":
            case "auto-fill-suffix-neighbors":
                return "width=200,height=60";
            case "auto-fit-empty-neighbors":
            case "auto-fill-max-size-neighbors":
            case "auto-fill-min-size-neighbors":
                return "";
            case "auto-fill-indefinite-neighbors":
                return "width=86,height=60";
            case "auto-fill-overflow-neighbors":
                return "width=40,height=60";
            case "auto-fill-row-neighbors":
            case "auto-fit-row-neighbors":
                return "width=50,height=200";
            case "both-fill-asymmetric":
                return "width=200,height=266";
            case "columns-fill-control":
            case "rows-fill-control":
            case "both-fill-square":
            case "both-fit-empty":
            case "both-fit-single":
            case "both-fit-span-two":
            case "both-fit-second-repeat":
            case "columns-fit-rows-fill-empty":
            case "columns-fill-rows-fit-empty":
            case "columns-fit-rows-fill-second-repeat":
            case "columns-fill-rows-fit-second-repeat":
                return "width=200,height=200";
            case "auto-fill-two-track-columns-both-190":
                return "width=190,height=60";
            case "auto-fill-two-track-columns-both-200":
                return "width=266,height=60";
            case "auto-fill-two-track-columns-both-342":
                return "width=342,height=60";
            case "auto-fill-two-track-columns-prefix-200":
                return "width=248,height=60";
            case "auto-fill-two-track-columns-suffix-200":
                return "width=238,height=60";
            case "auto-fill-two-track-rows-both-190":
                return "width=50,height=190";
            case "auto-fill-two-track-rows-both-200":
                return "width=50,height=266";
            case "auto-fill-two-track-rows-both-342":
                return "width=50,height=342";
            case "spanning-auto-tracks":
            case "spanning-auto-min-flex-tracks":
                return "width=100,height=30";
            case "spanning-min-content-items":
                return "width=100,height=30";
            case "competing-span2-forward":
            case "competing-span2-reversed":
            case "span2-before-span3":
            case "span3-before-span2":
                return "width=220,height=40";
            case "fit-content-calc-limit":
                return "width=100,height=120";
            case "fixed-small-track-overflow":
                return "width=12,height=40";
            case "min-content-row-image":
                return "width=100,height=80";
            case "min-content-text-edge":
            case "min-content-text-min-width":
            case "min-content-text-max-width":
                return "width=120,height=50";
            case "min-content-text-normal":
            case "auto-text-max-content":
            case "max-content-text-normal":
            case "fit-content-text-minimum":
            case "min-content-text-break-all":
            case "min-content-text-nowrap":
                return "width=120,height=40";
            default:
                return "width=100,height=40";
        }
    }

    private static String c114ExpectedItemFrame(String sceneId) {
        if (sceneId.equals("nested-grid-span-two")
                || sceneId.equals("nested-grid-one-track")
                || sceneId.equals("nested-grid-definite-flex-control")
                || sceneId.equals("nested-grid-text-image-span-two")
                || sceneId.equals("nested-grid-text-image-one-track")
                || sceneId.equals("nested-grid-text-image-definite-flex-control")
                || sceneId.equals("nested-grid-text-image-row-span-two")
                || sceneId.equals("nested-grid-text-image-row-one-track")
                || sceneId.equals("nested-grid-text-image-row-definite-flex-control")
                || sceneId.equals("nested-grid-implicit-row-span-two")
                || sceneId.equals("nested-grid-implicit-row-one-row")
                || sceneId.startsWith("nested-depth-one-")
                || sceneId.startsWith("nested-depth-two-")) return "";
        if (c114IsAnonymousTextScene(sceneId) || c114IsAnonymousTextImageScene(sceneId)
                || c114IsAnonymousTextMultiImageScene(sceneId)) return "";
        if (c114IsImageMultiSpanScene(sceneId) || c114IsTextMultiSpanScene(sceneId)
                || c114IsMixedMultiSpanScene(sceneId)
                || sceneId.startsWith("competing-span2-")
                || sceneId.startsWith("span2-before-")
                || sceneId.startsWith("span3-before-")) return "";
        if (c114IsAutoRepeatNegativeLineScene(sceneId)) return "";
        if (c114IsImplicitTextScene(sceneId)) return "";
        String negativeTrackFrame = c114NegativeTrackItemFrame(sceneId);
        if (negativeTrackFrame != null) return negativeTrackFrame;
        String autoFitMinimumSizeItemFrame = c114AutoFitMinimumSizeItemFrame(sceneId);
        if (autoFitMinimumSizeItemFrame != null) return autoFitMinimumSizeItemFrame;
        switch (sceneId) {
            case "column-auto-natural-ratio":
                return "width=12,height=8";
            case "column-target-max-width-clamp":
                return "width=9,height=8";
            case "row-target-max-height-clamp":
                return "width=12,height=6";
            case "column-opposite-min-over-max":
                return "width=18,height=12";
            case "column-border-box-transfer":
                return "width=12,height=8";
            case "column-content-box-transfer":
                return "width=16,height=12";
            case "column-target-min-width-clamp":
                return "width=18,height=8";
            case "row-transfer-input-capped-by-fixed-column-span":
                return "width=18,height=12";
            case "column-transfer-input-capped-by-fixed-row-span":
                return "width=27,height=18";
            case "fixed-gutter-column-suggestion-clamp":
                return "width=36,height=24";
            case "fixed-gutter-row-suggestion-clamp":
                return "width=54,height=36";
            case "grid-gap-definite-percent":
            case "grid-gap-cyclic-percent":
            case "grid-gap-definite-calc":
            case "grid-gap-cyclic-calc":
            case "grid-gap-cyclic-percent-min-basis":
            case "grid-gap-cyclic-calc-min-basis":
                return "width=40,height=10";
            case "spanning-auto-tracks":
                return "width=70,height=10";
            case "spanning-auto-min-flex-tracks":
                return "width=100,height=10";
            case "auto-fill-repeat":
                return "width=24,height=16";
            case "auto-fit-collapse":
                return "width=96,height=16";
            case "auto-fill-track-overflow":
                return "width=120,height=16";
            case "auto-fill-fixed-neighbors":
            case "auto-fill-pattern-neighbors":
                return "width=10,height=40";
            case "auto-fit-fixed-neighbors":
                return "width=162,height=40";
            case "auto-fit-empty-neighbors":
            case "auto-fill-max-size-neighbors":
            case "auto-fill-min-size-neighbors":
            case "auto-fill-two-track-columns-both-190":
            case "auto-fill-two-track-columns-both-200":
            case "auto-fill-two-track-columns-both-342":
            case "auto-fill-two-track-columns-prefix-200":
            case "auto-fill-two-track-columns-suffix-200":
            case "auto-fill-two-track-rows-both-190":
            case "auto-fill-two-track-rows-both-200":
            case "auto-fill-two-track-rows-both-342":
            case "both-fit-empty":
            case "columns-fit-rows-fill-empty":
            case "columns-fill-rows-fit-empty":
                return "";
            case "columns-fill-control":
            case "rows-fill-control":
            case "both-fill-square":
            case "both-fit-single":
            case "both-fit-span-two":
            case "both-fit-second-repeat":
            case "columns-fit-rows-fill-second-repeat":
            case "columns-fill-rows-fit-second-repeat":
            case "both-fill-asymmetric":
                return "width=12,height=12";
            case "auto-fit-span-neighbors":
            case "auto-fill-prefix-neighbors":
            case "auto-fill-suffix-neighbors":
            case "auto-fill-indefinite-neighbors":
            case "auto-fill-overflow-neighbors":
            case "auto-fill-row-neighbors":
            case "auto-fit-row-neighbors":
                return "width=12,height=12";
            case "min-content-text-normal":
            case "fit-content-text-minimum":
                return "width=70,height=20";
            case "auto-text-max-content":
            case "max-content-text-normal":
                return "width=100,height=10";
            case "fit-content-calc-limit":
                return "width=15,height=80";
            case "min-content-text-edge":
                return "width=80,height=20";
            case "min-content-text-min-width":
                return "width=90,height=20";
            case "min-content-text-max-width":
                return "width=40,height=20";
            case "min-content-text-break-all":
                return "width=10,height=80";
            case "min-content-text-nowrap":
                return "width=100,height=10";
            default:
                return "width=24,height=16";
        }
    }

    private static boolean c114ExpectedTextIntrinsicInputs(String sceneId, String report) {
        sceneId = c114BaseNestedGridSceneId(sceneId);
        if (c114IsAnonymousTextMultiImageScene(sceneId)) {
            return c114ExpectedAnonymousTextMultiImageInputs(sceneId, report);
        }
        if (c114IsAnonymousTextImageScene(sceneId)) {
            return c114ExpectedAnonymousTextImageInputs(sceneId, report);
        }
        if (c114IsMixedMultiSpanScene(sceneId)) {
            return c114ExpectedMixedIntrinsicInputs(sceneId, report);
        }
        if (c114IsTextMultiSpanScene(sceneId)) {
            return c114ExpectedTextMultiSpanIntrinsicInputs(sceneId, report);
        }
        int minContent;
        int minContentHeight;
        int maxContent;
        int maxContentHeight;
        int expectedGrid = 1;
        int expectedNode = 2;
        int expectedSourceOrder = 0;
        switch (sceneId) {
            case "auto-text-max-content":
            case "min-content-text-normal":
            case "max-content-text-normal":
            case "fit-content-text-minimum":
            case "min-content-text-edge":
            case "min-content-text-min-width":
            case "min-content-text-max-width":
            case "nested-depth-one-min-content":
            case "nested-depth-two-min-content":
                minContent = 70;
                minContentHeight = 20;
                maxContent = 100;
                maxContentHeight = 10;
                if (sceneId.equals("nested-depth-one-min-content")) {
                    expectedGrid = 2;
                    expectedNode = 3;
                } else if (sceneId.equals("nested-depth-two-min-content")) {
                    expectedGrid = 3;
                    expectedNode = 4;
                }
                break;
            case "nested-depth-one-max-content":
            case "nested-depth-two-max-content":
                minContent = 70;
                minContentHeight = 20;
                maxContent = 100;
                maxContentHeight = 10;
                if (sceneId.equals("nested-depth-one-max-content")) {
                    expectedGrid = 2;
                    expectedNode = 3;
                } else {
                    expectedGrid = 3;
                    expectedNode = 4;
                }
                break;
            case "fit-content-calc-limit":
            case "min-content-text-break-all":
                minContent = 10;
                minContentHeight = 80;
                maxContent = 80;
                maxContentHeight = 10;
                break;
            case "min-content-text-nowrap":
                minContent = 100;
                minContentHeight = 10;
                maxContent = 100;
                maxContentHeight = 10;
                break;
            case "spanning-auto-min-flex-tracks":
                minContent = 200;
                minContentHeight = 10;
                maxContent = 200;
                maxContentHeight = 10;
                break;
            case "typed-mixed-cycle":
            case "typed-auto":
            case "typed-min-content":
            case "typed-max-content":
            case "typed-minmax-max-content":
            case "typed-fit-content":
                minContent = 48;
                minContentHeight = 36;
                maxContent = 108;
                maxContentHeight = 12;
                break;
            case "typed-auto-fit-occupied":
            case "typed-auto-fit-gutter":
                minContent = 48;
                minContentHeight = 36;
                maxContent = 108;
                maxContentHeight = 12;
                expectedNode = 3;
                expectedSourceOrder = 1;
                break;
            default:
                return true;
        }
        int diagnosticsMarker = report.indexOf("grid_track_sizing_diagnostics=");
        if (diagnosticsMarker < 0) return false;
        String diagnosticsJson = report.substring(
                diagnosticsMarker + "grid_track_sizing_diagnostics=".length());
        try {
            Object value = new org.json.JSONTokener(diagnosticsJson).nextValue();
            if (!(value instanceof org.json.JSONObject)) return false;
            org.json.JSONObject diagnostics = (org.json.JSONObject) value;
            org.json.JSONArray inputs = diagnostics.optJSONArray("gridItemIntrinsicInputs");
            if (inputs == null || inputs.length() != 1) return false;
            org.json.JSONObject input = inputs.optJSONObject(0);
            if (input == null || input.optInt("grid", -1) != expectedGrid
                    || input.optInt("node", -1) != expectedNode
                    || input.optInt("sourceOrder", -1) != expectedSourceOrder) {
                return false;
            }
            org.json.JSONObject source = input.optJSONObject("source");
            return source != null && "text".equals(source.optString("kind"))
                    && source.optInt("fontRegistryRevision", -1) == 1
                    && source.optDouble("minContent", Double.NaN) == minContent
                    && source.optDouble("minContentHeight", Double.NaN) == minContentHeight
                    && source.optDouble("maxContent", Double.NaN) == maxContent
                    && source.optDouble("maxContentHeight", Double.NaN) == maxContentHeight;
        } catch (org.json.JSONException | ClassCastException error) {
            return false;
        }
    }

    private static boolean c114ExpectedTextMultiSpanIntrinsicInputs(String sceneId, String report) {
        int[][] expected;
        switch (sceneId) {
            case "text-span2-forward":
                expected = new int[][] {{50, 20, 80, 10}, {70, 20, 120, 10}};
                break;
            case "text-span2-reversed":
                expected = new int[][] {{70, 20, 120, 10}, {50, 20, 80, 10}};
                break;
            case "text-span2-before-span3":
                expected = new int[][] {
                    {50, 20, 80, 10}, {70, 20, 120, 10}, {180, 10, 180, 10},
                };
                break;
            case "text-span3-before-span2":
                expected = new int[][] {
                    {180, 10, 180, 10}, {70, 20, 120, 10}, {50, 20, 80, 10},
                };
                break;
            default:
                return false;
        }
        int diagnosticsMarker = report.indexOf("grid_track_sizing_diagnostics=");
        if (diagnosticsMarker < 0) return false;
        String diagnosticsJson = report.substring(
                diagnosticsMarker + "grid_track_sizing_diagnostics=".length());
        try {
            Object value = new org.json.JSONTokener(diagnosticsJson).nextValue();
            if (!(value instanceof org.json.JSONObject)) return false;
            org.json.JSONArray inputs = ((org.json.JSONObject) value)
                    .optJSONArray("gridItemIntrinsicInputs");
            if (inputs == null || inputs.length() != expected.length) return false;
            for (int index = 0; index < expected.length; index++) {
                org.json.JSONObject input = inputs.optJSONObject(index);
                if (input == null || input.optInt("grid", -1) != 1
                        || input.optInt("node", -1) != 2 + index * 2
                        || input.optInt("sourceOrder", -1) != index) return false;
                org.json.JSONObject source = input.optJSONObject("source");
                int[] sizes = expected[index];
                if (source == null || !"text".equals(source.optString("kind"))
                        || source.optInt("fontRegistryRevision", -1) != 1
                        || source.optDouble("minContent", Double.NaN) != sizes[0]
                        || source.optDouble("minContentHeight", Double.NaN) != sizes[1]
                        || source.optDouble("maxContent", Double.NaN) != sizes[2]
                        || source.optDouble("maxContentHeight", Double.NaN) != sizes[3]) {
                    return false;
                }
            }
            return true;
        } catch (org.json.JSONException | ClassCastException error) {
            return false;
        }
    }
    private static boolean c114ExpectedImageIntrinsicInputs(String sceneId, String report) {
        if (c114IsAnonymousTextMultiImageScene(sceneId)) {
            return c114ExpectedAnonymousTextMultiImageInputs(sceneId, report);
        }
        if (c114IsAnonymousTextImageScene(sceneId)) {
            return c114ExpectedAnonymousTextImageInputs(sceneId, report);
        }
        if (c114IsMixedMultiSpanScene(sceneId)) {
            return c114ExpectedMixedIntrinsicInputs(sceneId, report);
        }
        if (c114IsImageMultiSpanScene(sceneId)) {
            return c114ExpectedImageMultiSpanInputs(sceneId, report);
        }
        if (sceneId.startsWith("nested-grid-implicit-row-")) {
            return c114ExpectedNestedImplicitRowImageInputs(sceneId, report);
        }
        switch (sceneId) {
            case "auto-image-definite":
            case "minmax-auto-image":
            case "minmax-min-max-image":
            case "fit-content-below-minimum":
            case "fit-content-above-intrinsic":
            case "fit-content-percent":
            case "fixed-small-track-overflow":
            case "spanning-min-content-items":
            case "min-content-row-image":
            case "column-auto-natural-ratio":
            case "column-target-max-width-clamp":
            case "row-target-max-height-clamp":
            case "column-opposite-min-over-max":
            case "column-border-box-transfer":
            case "column-content-box-transfer":
            case "column-target-min-width-clamp":
            case "row-transfer-input-capped-by-fixed-column-span":
            case "column-transfer-input-capped-by-fixed-row-span":
            case "fixed-gutter-column-suggestion-clamp":
            case "fixed-gutter-row-suggestion-clamp":
                break;
            default:
                return true;
        }
        int diagnosticsMarker = report.indexOf("grid_track_sizing_diagnostics=");
        if (diagnosticsMarker < 0) return false;
        String diagnosticsJson = report.substring(
                diagnosticsMarker + "grid_track_sizing_diagnostics=".length());
        try {
            Object value = new org.json.JSONTokener(diagnosticsJson).nextValue();
            if (!(value instanceof org.json.JSONObject)) return false;
            org.json.JSONObject diagnostics = (org.json.JSONObject) value;
            org.json.JSONObject revision = diagnostics.optJSONObject("layoutInputRevision");
            org.json.JSONArray inputs = diagnostics.optJSONArray("gridItemIntrinsicInputs");
            if (revision == null || revision.optInt("intrinsicSize", 0) != 1
                    || inputs == null || inputs.length() != 1) return false;
            org.json.JSONObject input = inputs.optJSONObject(0);
            if (input == null || input.optInt("grid", -1) != 1
                    || input.optInt("node", -1) != 2 || input.optInt("sourceOrder", -1) != 0) {
                return false;
            }
            org.json.JSONObject source = input.optJSONObject("source");
            org.json.JSONObject natural = source == null ? null : source.optJSONObject("natural");
            return source != null && "replaced".equals(source.optString("kind"))
                    && natural != null && natural.optDouble("width", Double.NaN) == 24.0
                    && natural.optDouble("height", Double.NaN) == 16.0
                    && c114ImageTrackSizingTraceMatches(sceneId, diagnostics);
        } catch (org.json.JSONException | ClassCastException error) {
            return false;
        }
    }

    private static boolean c114ExpectedNestedImplicitRowImageInputs(String sceneId, String report) {
        int diagnosticsMarker = report.indexOf("grid_track_sizing_diagnostics=");
        if (diagnosticsMarker < 0) return false;
        String diagnosticsJson = report.substring(
                diagnosticsMarker + "grid_track_sizing_diagnostics=".length());
        try {
            Object value = new org.json.JSONTokener(diagnosticsJson).nextValue();
            if (!(value instanceof org.json.JSONObject)) return false;
            org.json.JSONObject diagnostics = (org.json.JSONObject) value;
            org.json.JSONObject revision = diagnostics.optJSONObject("layoutInputRevision");
            org.json.JSONArray inputs = diagnostics.optJSONArray("gridItemIntrinsicInputs");
            if (revision == null || revision.optInt("intrinsicSize", 0) != 1
                    || inputs == null || inputs.length() != 1) return false;
            org.json.JSONObject input = inputs.optJSONObject(0);
            if (input == null || input.optInt("grid", -1) != 2
                    || input.optInt("node", -1) != 5 || input.optInt("sourceOrder", -1) != 2) {
                return false;
            }
            org.json.JSONObject source = input.optJSONObject("source");
            org.json.JSONObject natural = source == null ? null : source.optJSONObject("natural");
            return source != null && "replaced".equals(source.optString("kind"))
                    && natural != null && natural.optDouble("width", Double.NaN) == 24.0
                    && natural.optDouble("height", Double.NaN) == 16.0
                    && c114NestedImplicitRowTrackSizingTraceMatches(sceneId, diagnostics);
        } catch (org.json.JSONException | ClassCastException error) {
            return false;
        }
    }

    private static boolean c114NestedImplicitRowTrackSizingTraceMatches(
            String sceneId, org.json.JSONObject diagnostics
    ) throws org.json.JSONException {
        org.json.JSONArray traces = diagnostics.optJSONArray("trackSizingTraces");
        if (traces == null || traces.length() != 2) return false;
        org.json.JSONObject outer = null;
        org.json.JSONObject nested = null;
        for (int index = 0; index < traces.length(); index++) {
            org.json.JSONObject trace = traces.optJSONObject(index);
            if (trace == null) return false;
            int grid = trace.optInt("grid", -1);
            if (grid == 1 && outer == null) outer = trace;
            else if (grid == 2 && nested == null) nested = trace;
            else return false;
        }
        double[] outerRows = "nested-grid-implicit-row-span-two".equals(sceneId)
                ? new double[] {18.0, 4.0, 18.0} : new double[] {40.0, 4.0, 0.0};
        int outerSpan = "nested-grid-implicit-row-span-two".equals(sceneId) ? 2 : 1;
        return c114NestedImplicitRowGridTraceMatches(outer, new double[] {24.0}, outerRows,
                        40.0, outerSpan)
                && c114NestedImplicitRowGridTraceMatches(
                        nested, new double[] {24.0}, new double[] {8.0, 4.0, 8.0, 4.0, 16.0},
                        Double.NaN, 0);
    }

    private static boolean c114NestedImplicitRowGridTraceMatches(
            org.json.JSONObject trace,
            double[] expectedColumns,
            double[] expectedRows,
            double expectedOuterContribution,
            int expectedOuterSpan
    ) throws org.json.JSONException {
        org.json.JSONArray passes = trace == null ? null : trace.optJSONArray("passes");
        if (passes == null || passes.length() != 2) return false;
        org.json.JSONObject columns = null;
        org.json.JSONObject rows = null;
        for (int index = 0; index < passes.length(); index++) {
            org.json.JSONObject pass = passes.optJSONObject(index);
            if (pass == null) return false;
            String axis = pass.optString("axis");
            if ("Columns".equals(axis) && columns == null) columns = pass;
            else if ("Rows".equals(axis) && rows == null) rows = pass;
            else return false;
        }
        boolean[] columnGutters = new boolean[expectedColumns.length];
        boolean[] rowGutters = new boolean[expectedRows.length];
        for (int index = 1; index < rowGutters.length; index += 2) rowGutters[index] = true;
        return c114ImageTrackPassMatches(columns, "Columns", expectedColumns,
                        columnGutters, Double.NaN, 0, "MinContent")
                && c114ImageTrackPassMatches(rows, "Rows", expectedRows,
                        rowGutters, expectedOuterContribution, expectedOuterSpan, "MinContent");
    }

    private static boolean c114ExpectedImageMultiSpanInputs(String sceneId, String report) {
        int expectedCount = sceneId.endsWith("before-span3")
                || sceneId.startsWith("image-span3-") ? 3 : 2;
        int[][] spansAndWidths;
        switch (sceneId) {
            case "image-span2-forward":
                spansAndWidths = new int[][] {{2, 24}, {2, 48}};
                break;
            case "image-span2-reversed":
                spansAndWidths = new int[][] {{2, 48}, {2, 24}};
                break;
            case "image-span2-before-span3":
                spansAndWidths = new int[][] {{2, 24}, {2, 48}, {3, 72}};
                break;
            case "image-span3-before-span2":
                spansAndWidths = new int[][] {{3, 72}, {2, 48}, {2, 24}};
                break;
            default:
                return false;
        }
        if (spansAndWidths.length != expectedCount) return false;
        int diagnosticsMarker = report.indexOf("grid_track_sizing_diagnostics=");
        if (diagnosticsMarker < 0) return false;
        String diagnosticsJson = report.substring(
                diagnosticsMarker + "grid_track_sizing_diagnostics=".length());
        try {
            Object value = new org.json.JSONTokener(diagnosticsJson).nextValue();
            if (!(value instanceof org.json.JSONObject)) return false;
            org.json.JSONObject diagnostics = (org.json.JSONObject) value;
            org.json.JSONObject revision = diagnostics.optJSONObject("layoutInputRevision");
            org.json.JSONArray inputs = diagnostics.optJSONArray("gridItemIntrinsicInputs");
            if (revision == null || revision.optInt("intrinsicSize", 0) != 1
                    || inputs == null || inputs.length() != expectedCount) return false;
            for (int index = 0; index < expectedCount; index++) {
                org.json.JSONObject input = inputs.optJSONObject(index);
                if (input == null || input.optInt("grid", -1) != 1
                        || input.optInt("node", -1) != index + 2
                        || input.optInt("sourceOrder", -1) != index) return false;
                org.json.JSONObject source = input.optJSONObject("source");
                org.json.JSONObject natural = source == null ? null : source.optJSONObject("natural");
                if (source == null || !"replaced".equals(source.optString("kind"))
                        || natural == null || natural.optDouble("width", Double.NaN) != 24.0
                        || natural.optDouble("height", Double.NaN) != 16.0
                        || !natural.has("aspectRatio") || !natural.isNull("aspectRatio")) {
                    return false;
                }
            }
            return c114ImageMultiSpanTraceMatches(sceneId, spansAndWidths, diagnostics);
        } catch (org.json.JSONException | ClassCastException error) {
            return false;
        }
    }

    private static boolean c114ImageMultiSpanTraceMatches(
            String sceneId, int[][] spansAndWidths, org.json.JSONObject diagnostics
    ) throws org.json.JSONException {
        double[] expectedColumns = sceneId.endsWith("before-span3")
                || sceneId.startsWith("image-span3-")
                ? new double[] {16.0, 28.0, 28.0, 0.0}
                : new double[] {12.0, 24.0, 24.0, 0.0};
        double[][] expectedContributions = c114ExpectedImageMultiSpanContributions(
                spansAndWidths);
        org.json.JSONArray traces = diagnostics.optJSONArray("trackSizingTraces");
        if (traces == null || traces.length() != 1) return false;
        org.json.JSONObject trace = traces.optJSONObject(0);
        org.json.JSONArray passes = trace == null ? null : trace.optJSONArray("passes");
        if (trace == null || trace.optInt("grid", -1) != 1
                || passes == null || passes.length() != 2) return false;
        org.json.JSONObject columns = passes.optJSONObject(0);
        org.json.JSONObject rows = passes.optJSONObject(1);
        if (columns == null || rows == null
                || !"Columns".equals(columns.optString("axis"))
                || !"Rows".equals(rows.optString("axis"))) return false;
        if (!c114ImageMultiSpanTracksMatch(columns, expectedColumns)
                || !c114ImageMultiSpanTracksMatch(rows, new double[] {20.0, 20.0, 20.0})) {
            return false;
        }
        org.json.JSONArray stages = columns.optJSONArray("stages");
        if (stages == null) return false;
        java.util.ArrayList<Integer> observedSpans = new java.util.ArrayList<>();
        for (int index = 0; index < stages.length(); index++) {
            org.json.JSONObject stage = stages.optJSONObject(index);
            if (stage == null) return false;
            java.util.regex.Matcher matcher = java.util.regex.Pattern
                    .compile("^IntrinsicItemBatch \\{ span: (\\d+),")
                    .matcher(stage.optString("stage"));
            if (matcher.find()) observedSpans.add(Integer.parseInt(matcher.group(1)));
        }
        java.util.List<Integer> expectedSpans = spansAndWidths.length == 2
                ? java.util.List.of(2) : java.util.List.of(2, 3);
        if (!observedSpans.equals(expectedSpans)) return false;
        org.json.JSONArray contributions = columns.optJSONArray("contributions");
        if (contributions == null || contributions.length() != expectedContributions.length) {
            return false;
        }
        for (int index = 0; index < expectedContributions.length; index++) {
            org.json.JSONObject contribution = contributions.optJSONObject(index);
            double[] expected = expectedContributions[index];
            if (contribution == null || contribution.optInt("item", -1) != (int) expected[0]
                    || !((expected[1] == 0.0 && "Minimum".equals(contribution.optString("kind")))
                            || (expected[1] == 1.0
                                    && "MinContent".equals(contribution.optString("kind"))))
                    || contribution.optInt("span", -1) != (int) expected[2]
                    || Math.abs(contribution.optDouble("value", Double.NaN) - expected[3]) > 0.0001) {
                return false;
            }
        }
        return true;
    }

    private static double[][] c114ExpectedImageMultiSpanContributions(int[][] spansAndWidths) {
        java.util.ArrayList<double[]> expected = new java.util.ArrayList<>();
        for (int span : new int[] {2, 3}) {
            for (int kind = 0; kind < 2; kind++) {
                int repetitions = kind == 0 ? 1 : 2;
                for (int repetition = 0; repetition < repetitions; repetition++) {
                    for (int index = 0; index < spansAndWidths.length; index++) {
                        if (spansAndWidths[index][0] == span) {
                            expected.add(new double[] {
                                index + 2, kind, span, kind == 0 ? 0 : spansAndWidths[index][1],
                            });
                        }
                    }
                }
            }
        }
        return expected.toArray(new double[0][]);
    }

    private static boolean c114ImageMultiSpanTracksMatch(
            org.json.JSONObject pass, double[] expectedSizes
    ) throws org.json.JSONException {
        org.json.JSONArray stages = pass.optJSONArray("stages");
        if (stages == null || stages.length() == 0) return false;
        org.json.JSONObject finalStage = stages.optJSONObject(stages.length() - 1);
        org.json.JSONArray tracks = finalStage == null ? null : finalStage.optJSONArray("tracks");
        if (tracks == null) return false;
        int observedCount = 0;
        for (int index = 0; index < tracks.length(); index++) {
            org.json.JSONObject track = tracks.optJSONObject(index);
            if (track == null) return false;
            if (!track.has("gutter") || !track.has("collapsed") || !track.has("baseSize")) {
                return false;
            }
            if (track.optBoolean("gutter", false) || track.optBoolean("collapsed", false)) continue;
            double baseSize = track.optDouble("baseSize", Double.NaN);
            if (observedCount >= expectedSizes.length
                    || !Double.isFinite(baseSize)
                    || Math.abs(baseSize - expectedSizes[observedCount]) > 0.0001) return false;
            observedCount++;
        }
        return observedCount == expectedSizes.length;
    }
    private static boolean c114ImageTrackSizingTraceMatches(
            String sceneId, org.json.JSONObject diagnostics
    ) throws org.json.JSONException {
        double[] expectedColumns;
        boolean[] expectedColumnGutters;
        double expectedColumnContribution;
        double[] expectedRows = new double[] {16.0};
        boolean[] expectedRowGutters = new boolean[] {false};
        double expectedRowContribution = 16.0;
        String expectedColumnContributionKind = "MinContent";
        String expectedRowContributionKind = "MinContent";
        int expectedColumnSpan = "spanning-min-content-items".equals(sceneId)
                || "fixed-gutter-column-suggestion-clamp".equals(sceneId) ? 2 : 1;
        int expectedRowSpan = "fixed-gutter-row-suggestion-clamp".equals(sceneId) ? 2 : 1;
        if ("fixed-gutter-column-suggestion-clamp".equals(sceneId)) {
            expectedColumns = new double[] {3.0, 3.0, 4.0};
            expectedColumnGutters = new boolean[] {false, true, false};
            expectedColumnContribution = 10.0;
            expectedColumnContributionKind = "Minimum";
            expectedRows = new double[] {10.0 / 1.5};
            expectedRowContribution = 10.0 / 1.5;
            expectedRowContributionKind = "Minimum";
        } else if ("fixed-gutter-row-suggestion-clamp".equals(sceneId)) {
            expectedColumns = new double[] {15.0};
            expectedColumnGutters = new boolean[] {false};
            expectedColumnContribution = 15.0;
            expectedColumnContributionKind = "Minimum";
            expectedRows = new double[] {3.0, 3.0, 4.0};
            expectedRowGutters = new boolean[] {false, true, false};
            expectedRowContribution = 10.0;
            expectedRowContributionKind = "Minimum";
        } else if ("row-transfer-input-capped-by-fixed-column-span".equals(sceneId)) {
            expectedColumns = new double[] {18.0};
            expectedColumnGutters = new boolean[] {false};
            expectedColumnContribution = 18.0;
            expectedColumnContributionKind = "Minimum";
            expectedRows = new double[] {8.0 / 1.5};
            expectedRowContribution = 8.0 / 1.5;
            expectedRowContributionKind = "Minimum";
        } else if ("column-transfer-input-capped-by-fixed-row-span".equals(sceneId)) {
            expectedColumns = new double[] {12.0};
            expectedColumnGutters = new boolean[] {false};
            expectedColumnContribution = 12.0;
            expectedColumnContributionKind = "Minimum";
            expectedRows = new double[] {18.0};
            expectedRowContribution = 18.0;
            expectedRowContributionKind = "Minimum";
        } else if ("column-auto-natural-ratio".equals(sceneId)) {
            expectedColumns = new double[] {12.0};
            expectedColumnGutters = new boolean[] {false};
            expectedColumnContribution = 12.0;
            expectedColumnContributionKind = "Minimum";
            expectedRows = new double[] {20.0};
            expectedRowContribution = Double.NaN;
        } else if ("column-target-max-width-clamp".equals(sceneId)) {
            expectedColumns = new double[] {9.0};
            expectedColumnGutters = new boolean[] {false};
            expectedColumnContribution = 9.0;
            expectedColumnContributionKind = "Minimum";
            expectedRows = new double[] {20.0};
            expectedRowContribution = Double.NaN;
        } else if ("column-opposite-min-over-max".equals(sceneId)) {
            expectedColumns = new double[] {18.0};
            expectedColumnGutters = new boolean[] {false};
            expectedColumnContribution = 18.0;
            expectedColumnContributionKind = "Minimum";
            expectedRows = new double[] {20.0};
            expectedRowContribution = Double.NaN;
        } else if ("column-border-box-transfer".equals(sceneId)) {
            expectedColumns = new double[] {12.0};
            expectedColumnGutters = new boolean[] {false};
            expectedColumnContribution = 12.0;
            expectedColumnContributionKind = "Minimum";
            expectedRows = new double[] {20.0};
            expectedRowContribution = Double.NaN;
        } else if ("column-content-box-transfer".equals(sceneId)) {
            expectedColumns = new double[] {16.0};
            expectedColumnGutters = new boolean[] {false};
            expectedColumnContribution = 16.0;
            expectedColumnContributionKind = "Minimum";
            expectedRows = new double[] {20.0};
            expectedRowContribution = Double.NaN;
        } else if ("column-target-min-width-clamp".equals(sceneId)) {
            expectedColumns = new double[] {18.0};
            expectedColumnGutters = new boolean[] {false};
            expectedColumnContribution = 18.0;
            expectedColumnContributionKind = "Minimum";
            expectedRows = new double[] {20.0};
            expectedRowContribution = Double.NaN;
        } else if ("row-target-max-height-clamp".equals(sceneId)) {
            expectedColumns = new double[] {20.0};
            expectedColumnGutters = new boolean[] {false};
            expectedColumnContribution = Double.NaN;
            expectedRows = new double[] {6.0};
            expectedRowContribution = 6.0;
            expectedRowContributionKind = "Minimum";
        } else if ("fixed-small-track-overflow".equals(sceneId)) {
            expectedColumns = new double[] {12.0};
            expectedColumnGutters = new boolean[] {false};
            expectedColumnContribution = Double.NaN;
        } else if ("spanning-min-content-items".equals(sceneId)) {
            expectedColumns = new double[] {10.0, 4.0, 10.0};
            expectedColumnGutters = new boolean[] {false, true, false};
            expectedColumnContribution = 24.0;
        } else if ("min-content-row-image".equals(sceneId)) {
            expectedColumns = new double[] {100.0};
            expectedColumnGutters = new boolean[] {false};
            expectedColumnContribution = Double.NaN;
        } else {
            expectedColumns = new double[] {24.0};
            expectedColumnGutters = new boolean[] {false};
            expectedColumnContribution = 24.0;
        }
        org.json.JSONArray traces = diagnostics.optJSONArray("trackSizingTraces");
        if (traces == null || traces.length() != 1) return false;
        org.json.JSONObject trace = traces.optJSONObject(0);
        if (trace == null || trace.optInt("grid", -1) != 1) return false;
        org.json.JSONArray passes = trace.optJSONArray("passes");
        boolean repeatedColumnSizing = "column-target-min-width-clamp".equals(sceneId)
                || "column-target-max-width-clamp".equals(sceneId)
                || "column-opposite-min-over-max".equals(sceneId)
                || "column-border-box-transfer".equals(sceneId)
                || "column-content-box-transfer".equals(sceneId)
                || "column-transfer-input-capped-by-fixed-row-span".equals(sceneId)
                || "fixed-gutter-column-suggestion-clamp".equals(sceneId)
                || "fixed-gutter-row-suggestion-clamp".equals(sceneId);
        boolean repeatedRowSizing = "fixed-gutter-column-suggestion-clamp".equals(sceneId)
                || "fixed-gutter-row-suggestion-clamp".equals(sceneId);
        int expectedPassCount = repeatedRowSizing ? 4 : repeatedColumnSizing ? 3 : 2;
        if (passes == null || passes.length() != expectedPassCount) return false;
        boolean firstColumnAndRowsMatch = c114ImageTrackPassMatches(passes.optJSONObject(0), "Columns",
                        expectedColumns, expectedColumnGutters, expectedColumnContribution,
                        expectedColumnSpan, expectedColumnContributionKind)
                && c114ImageTrackPassMatches(passes.optJSONObject(1), "Rows",
                        expectedRows, expectedRowGutters, expectedRowContribution, expectedRowSpan,
                        expectedRowContributionKind);
        boolean passesMatch = firstColumnAndRowsMatch && (!repeatedColumnSizing
                || c114ImageTrackPassMatches(passes.optJSONObject(2), "Columns",
                        expectedColumns, expectedColumnGutters, expectedColumnContribution,
                        expectedColumnSpan, expectedColumnContributionKind));
        if (repeatedRowSizing) {
            passesMatch = passesMatch && c114ImageTrackPassMatches(passes.optJSONObject(3), "Rows",
                    expectedRows, expectedRowGutters, expectedRowContribution, expectedRowSpan,
                    expectedRowContributionKind);
        }
        return passesMatch && c114FixedGutterSuggestionTraceMatches(sceneId, passes);
    }

    private static boolean c114FixedGutterSuggestionTraceMatches(
            String sceneId, org.json.JSONArray passes
    ) throws org.json.JSONException {
        int passIndex;
        if ("fixed-gutter-column-suggestion-clamp".equals(sceneId)) {
            passIndex = 0;
        } else if ("fixed-gutter-row-suggestion-clamp".equals(sceneId)) {
            passIndex = 1;
        } else {
            return true;
        }
        org.json.JSONObject pass = passes.optJSONObject(passIndex);
        org.json.JSONArray contributions = pass == null ? null : pass.optJSONArray("contributions");
        if (contributions == null) return false;
        for (int index = 0; index < contributions.length(); index++) {
            org.json.JSONObject contribution = contributions.optJSONObject(index);
            if (contribution == null || contribution.optInt("item", -1) != 2
                    || !"Minimum".equals(contribution.optString("kind"))) continue;
            String source = contribution.optString("minimumSource");
            return source.contains("PreferredSize")
                    && source.contains("fixed_track_sum: Some(7.0)")
                    && source.contains("fixed_gutter_sum: Some(3.0)")
                    && source.contains("fixed_track_limit: Some(10.0)");
        }
        return false;
    }
    private static boolean c114ImageTrackPassMatches(
            org.json.JSONObject pass,
            String expectedAxis,
            double[] expectedSizes,
            boolean[] expectedGutters,
            double expectedMinContentContribution,
            int expectedSpan,
            String expectedContributionKind
    ) throws org.json.JSONException {
        if (pass == null || !expectedAxis.equals(pass.optString("axis"))) return false;
        org.json.JSONArray stages = pass.optJSONArray("stages");
        if (stages == null || stages.length() == 0) return false;
        org.json.JSONObject finalStage = stages.optJSONObject(stages.length() - 1);
        org.json.JSONArray tracks = finalStage == null ? null : finalStage.optJSONArray("tracks");
        if (tracks == null) return false;
        int observedCount = 0;
        for (int i = 0; i < tracks.length(); i++) {
            org.json.JSONObject track = tracks.optJSONObject(i);
            if (track == null) return false;
            if (track.optBoolean("collapsed", true)) continue;
            if (observedCount >= expectedSizes.length
                    || track.optBoolean("gutter", false) != expectedGutters[observedCount]
                    || Math.abs(track.optDouble("baseSize", Double.NaN)
                            - expectedSizes[observedCount]) > 0.0001) {
                return false;
            }
            observedCount++;
        }
        if (observedCount != expectedSizes.length) return false;
        if (Double.isNaN(expectedMinContentContribution)) return true;
        org.json.JSONArray contributions = pass.optJSONArray("contributions");
        if (contributions == null) return false;
        for (int i = 0; i < contributions.length(); i++) {
            org.json.JSONObject contribution = contributions.optJSONObject(i);
            if (contribution != null && contribution.optInt("item", -1) == 2
                    && contribution.optInt("span", -1) == expectedSpan
                    && expectedContributionKind.equals(contribution.optString("kind"))
                    && Math.abs(contribution.optDouble("value", Double.NaN)
                            - expectedMinContentContribution) <= 0.0001) {
                return true;
            }
        }
        return false;
    }
    private static String c114ExpectedNodeFrames(String sceneId) {
        sceneId = c114BaseNestedGridSceneId(sceneId);
        switch (sceneId) {
            case "nested-depth-one-min-content":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=70,height=20;"
                        + "1:node=2,x=0,y=0,width=70,height=20;"
                        + "2:node=3,x=0,y=0,width=70,height=20]";
            case "nested-depth-two-min-content":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=70,height=20;"
                        + "1:node=2,x=0,y=0,width=70,height=20;"
                        + "2:node=3,x=0,y=0,width=70,height=20;"
                        + "3:node=4,x=0,y=0,width=70,height=20]";
            case "nested-depth-one-max-content":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=100,height=10;"
                        + "1:node=2,x=0,y=0,width=100,height=10;"
                        + "2:node=3,x=0,y=0,width=100,height=10]";
            case "nested-depth-two-max-content":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=100,height=10;"
                        + "1:node=2,x=0,y=0,width=100,height=10;"
                        + "2:node=3,x=0,y=0,width=100,height=10;"
                        + "3:node=4,x=0,y=0,width=100,height=10]";
            case "nested-grid-span-two":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=134,height=20;"
                        + "1:node=2,x=0,y=0,width=134,height=20;"
                        + "2:node=3,x=0,y=0,width=70,height=20;"
                        + "3:node=5,x=74,y=0,width=60,height=20]";
            case "nested-grid-one-track":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=138,height=20;"
                        + "1:node=2,x=0,y=0,width=134,height=20;"
                        + "2:node=3,x=0,y=0,width=70,height=20;"
                        + "3:node=5,x=74,y=0,width=60,height=20]";
            case "nested-grid-definite-flex-control":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=20;"
                        + "1:node=2,x=0,y=0,width=120,height=20;"
                        + "2:node=3,x=0,y=0,width=70,height=20;"
                        + "3:node=5,x=74,y=0,width=60,height=20]";
            case "nested-grid-text-image-span-two":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=98,height=20;"
                        + "1:node=2,x=0,y=0,width=98,height=20;"
                        + "2:node=3,x=0,y=0,width=70,height=20;"
                        + "3:node=5,x=74,y=0,width=24,height=16]";
            case "nested-grid-text-image-one-track":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=102,height=20;"
                        + "1:node=2,x=0,y=0,width=98,height=20;"
                        + "2:node=3,x=0,y=0,width=70,height=20;"
                        + "3:node=5,x=74,y=0,width=24,height=16]";
            case "nested-grid-text-image-definite-flex-control":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=20;"
                        + "1:node=2,x=0,y=0,width=120,height=20;"
                        + "2:node=3,x=0,y=0,width=70,height=20;"
                        + "3:node=5,x=74,y=0,width=24,height=16]";
            case "nested-grid-text-image-row-span-two":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=98,height=40;"
                        + "1:node=2,x=0,y=0,width=98,height=40;"
                        + "2:node=3,x=0,y=0,width=70,height=20;"
                        + "3:node=5,x=74,y=24,width=24,height=16]";
            case "nested-grid-text-image-row-one-track":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=98,height=44;"
                        + "1:node=2,x=0,y=0,width=98,height=40;"
                        + "2:node=3,x=0,y=0,width=70,height=20;"
                        + "3:node=5,x=74,y=24,width=24,height=16]";
            case "nested-grid-text-image-row-definite-flex-control":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=98,height=60;"
                        + "1:node=2,x=0,y=0,width=98,height=60;"
                        + "2:node=3,x=0,y=0,width=70,height=20;"
                        + "3:node=5,x=74,y=24,width=24,height=16]";
            case "nested-grid-implicit-row-span-two":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=40;"
                        + "1:node=2,x=0,y=0,width=24,height=40;"
                        + "2:node=3,x=0,y=0,width=24,height=8;"
                        + "3:node=4,x=0,y=12,width=4,height=8;"
                        + "4:node=5,x=0,y=24,width=24,height=16]";
            case "nested-grid-implicit-row-one-row":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=24,height=44;"
                        + "1:node=2,x=0,y=0,width=24,height=40;"
                        + "2:node=3,x=0,y=0,width=24,height=8;"
                        + "3:node=4,x=0,y=12,width=4,height=8;"
                        + "4:node=5,x=0,y=24,width=24,height=16]";
            case "anonymous-runs-two-images-forward":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=184;"
                        + "1:node=3,x=2,y=2,width=24,height=16;"
                        + "2:node=5,x=14,y=62,width=48,height=32]";
            case "two-images-anonymous-runs-reversed":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=184;"
                        + "1:node=2,x=14,y=62,width=48,height=32;"
                        + "2:node=4,x=2,y=2,width=24,height=16]";
            case "two-anonymous-runs-three-images-forward":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=184;"
                        + "1:node=3,x=2,y=2,width=24,height=16;"
                        + "2:node=5,x=20,y=62,width=48,height=32;"
                        + "3:node=6,x=2,y=122,width=72,height=48]";
            case "three-images-two-anonymous-runs-reversed":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=184;"
                        + "1:node=2,x=2,y=122,width=72,height=48;"
                        + "2:node=4,x=20,y=62,width=48,height=32;"
                        + "3:node=6,x=2,y=2,width=24,height=16]";
            case "anonymous-text-two-images-forward":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;"
                        + "1:node=3,x=0,y=0,width=24,height=16;"
                        + "2:node=4,x=12,y=60,width=48,height=32]";
            case "two-images-anonymous-text-reversed":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;"
                        + "1:node=2,x=12,y=60,width=48,height=32;"
                        + "2:node=4,x=0,y=0,width=24,height=16]";
            case "anonymous-text-three-images-forward":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;"
                        + "1:node=3,x=0,y=0,width=24,height=16;"
                        + "2:node=4,x=18,y=60,width=48,height=32;"
                        + "3:node=5,x=0,y=120,width=72,height=48]";
            case "three-images-anonymous-text-reversed":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=180;"
                        + "1:node=2,x=0,y=120,width=72,height=48;"
                        + "2:node=3,x=18,y=60,width=48,height=32;"
                        + "3:node=5,x=0,y=0,width=24,height=16]";
            case "anonymous-text-image-text":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;"
                        + "1:node=3,x=20,y=0,width=72,height=48]";
            case "image-before-comment-group":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;"
                        + "1:node=2,x=0,y=0,width=72,height=48]";
            case "comment-group-before-image":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;"
                        + "1:node=4,x=40,y=0,width=72,height=48]";
            case "whitespace-around-image":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;"
                        + "1:node=3,x=0,y=0,width=72,height=48]";
            case "hidden-child-around-image":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=220,height=60;"
                        + "1:node=5,x=20,y=0,width=72,height=48]";
            case "anonymous-min-content-wrap":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=20]";
            case "anonymous-comment-continuation":
            case "anonymous-hidden-child-continuation":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=10]";
            case "anonymous-runs-around-block":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=10;"
                        + "1:node=3,x=20,y=0,width=30,height=10]";
            case "whitespace-does-not-create-item":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=120,height=10;"
                        + "1:node=2,x=0,y=0,width=10,height=10;"
                        + "2:node=4,x=20,y=0,width=10,height=10]";
            default:
                break;
        }
        if (c114IsImplicitTextScene(sceneId)) return null;
        if (c114IsMultiSpanOrderingScene(sceneId) || c114IsTextMultiSpanScene(sceneId)) return null;
        String negativeTrackFrames = c114NegativeTrackNodeFrames(sceneId);
        if (negativeTrackFrames != null) return negativeTrackFrames;
        String autoFitMinimumSizeNodeFrames = c114AutoFitMinimumSizeNodeFrames(sceneId);
        if (autoFitMinimumSizeNodeFrames != null) return autoFitMinimumSizeNodeFrames;
        switch (sceneId) {
            case "spanning-auto-min-flex-tracks":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=100,height=30;"
                        + "1:node=2,x=0,y=0,width=100,height=10]";
            case "auto-fill-overflow-neighbors":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=40,height=60;"
                        + "1:node=2,x=86,y=0,width=12,height=12]";
            case "both-fit-empty":
            case "columns-fit-rows-fill-empty":
            case "columns-fill-rows-fit-empty":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200]";
            case "columns-fill-control":
            case "rows-fill-control":
            case "both-fill-square":
            case "both-fit-single":
            case "both-fit-span-two":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;"
                        + "1:node=2,x=28,y=28,width=12,height=12]";
            case "both-fit-second-repeat":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;"
                        + "1:node=2,x=20,y=20,width=12,height=12]";
            case "both-fill-asymmetric":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=266;"
                        + "1:node=2,x=28,y=28,width=12,height=12]";
            case "columns-fit-rows-fill-second-repeat":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;"
                        + "1:node=2,x=20,y=109,width=12,height=12]";
            case "columns-fill-rows-fit-second-repeat":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=200;"
                        + "1:node=2,x=109,y=20,width=12,height=12]";
            case "auto-fill-two-track-columns-both-190":
            case "auto-fill-two-track-columns-both-200":
            case "auto-fill-two-track-columns-both-342":
            case "auto-fill-two-track-columns-prefix-200":
            case "auto-fill-two-track-columns-suffix-200":
            case "auto-fill-two-track-rows-both-190":
            case "auto-fill-two-track-rows-both-200":
            case "auto-fill-two-track-rows-both-342":
                return "node_frames_css_px=[0:node=1,x=0,y=0,"
                        + c114ExpectedRootFrame(sceneId) + "]";
            case "auto-fill-fixed-neighbors":
            case "auto-fill-pattern-neighbors":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;"
                        + "1:node=2,x=190,y=0,width=10,height=40]";
            case "auto-fit-fixed-neighbors":
                return "node_frames_css_px=[0:node=1,x=0,y=0,width=200,height=60;"
                        + "1:node=2,x=28,y=0,width=162,height=40]";
            case "grid-gap-definite-percent":
                return c114GapNodeFrames("100", "40", "50", "20");
            case "grid-gap-cyclic-percent":
                return c114GapNodeFrames("100", "20", "50", "15");
            case "grid-gap-definite-calc":
                return c114GapNodeFrames("100", "40", "55", "25");
            case "grid-gap-cyclic-calc":
                return c114GapNodeFrames("105", "25", "55.5", "21.25");
            case "grid-gap-cyclic-percent-min-basis":
                return c114GapNodeFrames("120", "30", "52", "17.5");
            case "grid-gap-cyclic-calc-min-basis":
                return c114GapNodeFrames("120", "30", "57", "22.5");
            case "auto-text-max-content":
            case "min-content-text-normal":
            case "max-content-text-normal":
            case "fit-content-text-minimum":
            case "fit-content-calc-limit":
            case "min-content-text-edge":
            case "min-content-text-min-width":
            case "min-content-text-max-width":
            case "min-content-text-break-all":
            case "min-content-text-nowrap":
            case "auto-image-definite":
            case "minmax-auto-image":
            case "minmax-min-max-image":
            case "fit-content-below-minimum":
            case "fit-content-above-intrinsic":
            case "fit-content-percent":
            case "fixed-small-track-overflow":
            case "spanning-min-content-items":
            case "min-content-row-image":
            case "column-auto-natural-ratio":
            case "column-target-max-width-clamp":
            case "row-target-max-height-clamp":
            case "column-opposite-min-over-max":
            case "column-border-box-transfer":
            case "column-content-box-transfer":
            case "column-target-min-width-clamp":
            case "row-transfer-input-capped-by-fixed-column-span":
            case "column-transfer-input-capped-by-fixed-row-span":
            case "fixed-gutter-column-suggestion-clamp":
            case "fixed-gutter-row-suggestion-clamp":
                String imageXy = sceneId.startsWith("fixed-gutter-") ? "x=1,y=1," : "x=0,y=0,";
                return "node_frames_css_px=[0:node=1,x=0,y=0," + c114ExpectedRootFrame(sceneId)
                        + ";1:node=2," + imageXy + c114ExpectedItemFrame(sceneId) + "]";
            default:
                return null;
        }
    }

    private static boolean c114IsMultiSpanOrderingScene(String sceneId) {
        return "competing-span2-forward".equals(sceneId)
                || "competing-span2-reversed".equals(sceneId)
                || "span2-before-span3".equals(sceneId)
                || "span3-before-span2".equals(sceneId);
    }

    private static boolean c114IsTextMultiSpanScene(String sceneId) {
        return "text-span2-forward".equals(sceneId)
                || "text-span2-reversed".equals(sceneId)
                || "text-span2-before-span3".equals(sceneId)
                || "text-span3-before-span2".equals(sceneId);
    }

    private static boolean c114IsImageMultiSpanScene(String sceneId) {
        return "image-span2-forward".equals(sceneId)
                || "image-span2-reversed".equals(sceneId)
                || "image-span2-before-span3".equals(sceneId)
                || "image-span3-before-span2".equals(sceneId);
    }

    private static boolean c114IsMixedMultiSpanScene(String sceneId) {
        return "mixed-text-image-span2-forward".equals(sceneId)
                || "mixed-text-image-span2-reversed".equals(sceneId)
                || "mixed-text-image-span2-before-span3".equals(sceneId)
                || "mixed-text-image-span3-before-span2".equals(sceneId);
    }

    private static boolean c114ExpectedMixedIntrinsicInputs(String sceneId, String report) {
        int[] nodes;
        String[] kinds;
        int[][] textMetrics;
        switch (sceneId) {
            case "mixed-text-image-span2-forward":
                nodes = new int[] {2, 4};
                kinds = new String[] {"text", "replaced"};
                textMetrics = new int[][] {{50, 20, 80, 10}, null};
                break;
            case "mixed-text-image-span2-reversed":
                nodes = new int[] {2, 3};
                kinds = new String[] {"replaced", "text"};
                textMetrics = new int[][] {null, {50, 20, 80, 10}};
                break;
            case "mixed-text-image-span2-before-span3":
                nodes = new int[] {2, 4, 5};
                kinds = new String[] {"text", "replaced", "text"};
                textMetrics = new int[][] {{50, 20, 80, 10}, null, {180, 10, 180, 10}};
                break;
            case "mixed-text-image-span3-before-span2":
                nodes = new int[] {2, 4, 5};
                kinds = new String[] {"text", "replaced", "text"};
                textMetrics = new int[][] {{180, 10, 180, 10}, null, {50, 20, 80, 10}};
                break;
            default:
                return false;
        }
        int diagnosticsMarker = report.indexOf("grid_track_sizing_diagnostics=");
        if (diagnosticsMarker < 0) return false;
        String diagnosticsJson = report.substring(
                diagnosticsMarker + "grid_track_sizing_diagnostics=".length());
        try {
            Object value = new org.json.JSONTokener(diagnosticsJson).nextValue();
            if (!(value instanceof org.json.JSONObject)) return false;
            org.json.JSONObject diagnostics = (org.json.JSONObject) value;
            org.json.JSONObject revision = diagnostics.optJSONObject("layoutInputRevision");
            org.json.JSONArray inputs = diagnostics.optJSONArray("gridItemIntrinsicInputs");
            if (revision == null || revision.optInt("intrinsicSize", 0) != 1
                    || inputs == null || inputs.length() != nodes.length) return false;
            for (int index = 0; index < nodes.length; index++) {
                org.json.JSONObject input = inputs.optJSONObject(index);
                org.json.JSONObject source = input == null ? null : input.optJSONObject("source");
                if (input == null || input.optInt("grid", -1) != 1
                        || input.optInt("node", -1) != nodes[index]
                        || input.optInt("sourceOrder", -1) != index || source == null
                        || !kinds[index].equals(source.optString("kind"))) return false;
                if ("text".equals(kinds[index])) {
                    int[] metrics = textMetrics[index];
                    if (source.optInt("fontRegistryRevision", -1) != 1
                            || source.optDouble("minContent", Double.NaN) != metrics[0]
                            || source.optDouble("minContentHeight", Double.NaN) != metrics[1]
                            || source.optDouble("maxContent", Double.NaN) != metrics[2]
                            || source.optDouble("maxContentHeight", Double.NaN) != metrics[3]) {
                        return false;
                    }
                } else {
                    org.json.JSONObject natural = source.optJSONObject("natural");
                    if (natural == null || natural.optDouble("width", Double.NaN) != 24.0
                            || natural.optDouble("height", Double.NaN) != 16.0
                            || !natural.has("aspectRatio") || !natural.isNull("aspectRatio")) {
                        return false;
                    }
                }
            }
            return true;
        } catch (org.json.JSONException | ClassCastException error) {
            return false;
        }
    }

    private static boolean c114ExpectedAnonymousTextMultiImageInputs(
            String sceneId, String report
    ) {
        int[] nodes;
        String[] kinds;
        int[] spans;
        double[] imageWidths;
        double[][] expectedFrames;
        int[] textIndices;
        switch (sceneId) {
            case "anonymous-text-two-images-forward":
                nodes = new int[] {2, 3, 4};
                kinds = new String[] {"text", "replaced", "replaced"};
                spans = new int[] {1, 2, 2};
                imageWidths = new double[] {0, 24, 48};
                expectedFrames = new double[][] {
                    {2, 40, 0, 20, 10}, {3, 0, 0, 24, 16}, {4, 12, 60, 48, 32},
                };
                textIndices = new int[] {0};
                break;
            case "two-images-anonymous-text-reversed":
                nodes = new int[] {2, 3, 4};
                kinds = new String[] {"replaced", "text", "replaced"};
                spans = new int[] {2, 1, 2};
                imageWidths = new double[] {48, 0, 24};
                expectedFrames = new double[][] {
                    {2, 12, 60, 48, 32}, {3, 40, 0, 20, 10}, {4, 0, 0, 24, 16},
                };
                textIndices = new int[] {1};
                break;
            case "anonymous-text-three-images-forward":
                nodes = new int[] {2, 3, 4, 5};
                kinds = new String[] {"text", "replaced", "replaced", "replaced"};
                spans = new int[] {1, 2, 2, 3};
                imageWidths = new double[] {0, 24, 48, 72};
                expectedFrames = new double[][] {
                    {2, 52, 0, 20, 10}, {3, 0, 0, 24, 16},
                    {4, 18, 60, 48, 32}, {5, 0, 120, 72, 48},
                };
                textIndices = new int[] {0};
                break;
            case "three-images-anonymous-text-reversed":
                nodes = new int[] {2, 3, 4, 5};
                kinds = new String[] {"replaced", "replaced", "text", "replaced"};
                spans = new int[] {3, 2, 1, 2};
                imageWidths = new double[] {72, 48, 0, 24};
                expectedFrames = new double[][] {
                    {2, 0, 120, 72, 48}, {3, 18, 60, 48, 32},
                    {4, 52, 0, 20, 10}, {5, 0, 0, 24, 16},
                };
                textIndices = new int[] {2};
                break;
            case "anonymous-runs-two-images-forward":
                nodes = new int[] {2, 3, 4, 5};
                kinds = new String[] {"text", "replaced", "text", "replaced"};
                spans = new int[] {1, 2, 1, 2};
                imageWidths = new double[] {0, 24, 0, 48};
                expectedFrames = new double[][] {
                    {2, 42, 2, 20, 10}, {3, 2, 2, 24, 16},
                    {4, 62, 2, 20, 10}, {5, 14, 62, 48, 32},
                };
                textIndices = new int[] {0, 2};
                break;
            case "two-images-anonymous-runs-reversed":
                nodes = new int[] {2, 3, 4, 5};
                kinds = new String[] {"replaced", "text", "replaced", "text"};
                spans = new int[] {2, 1, 2, 1};
                imageWidths = new double[] {48, 0, 24, 0};
                expectedFrames = new double[][] {
                    {2, 14, 62, 48, 32}, {3, 42, 2, 20, 10},
                    {4, 2, 2, 24, 16}, {5, 62, 2, 20, 10},
                };
                textIndices = new int[] {1, 3};
                break;
            case "two-anonymous-runs-three-images-forward":
                nodes = new int[] {2, 3, 4, 5, 6};
                kinds = new String[] {"text", "replaced", "text", "replaced", "replaced"};
                spans = new int[] {1, 2, 1, 2, 3};
                imageWidths = new double[] {0, 24, 0, 48, 72};
                expectedFrames = new double[][] {
                    {2, 54, 2, 20, 10}, {3, 2, 2, 24, 16}, {4, 74, 2, 20, 10},
                    {5, 20, 62, 48, 32}, {6, 2, 122, 72, 48},
                };
                textIndices = new int[] {0, 2};
                break;
            case "three-images-two-anonymous-runs-reversed":
                nodes = new int[] {2, 3, 4, 5, 6};
                kinds = new String[] {"replaced", "text", "replaced", "text", "replaced"};
                spans = new int[] {3, 1, 2, 1, 2};
                imageWidths = new double[] {72, 0, 48, 0, 24};
                expectedFrames = new double[][] {
                    {2, 2, 122, 72, 48}, {3, 54, 2, 20, 10}, {4, 20, 62, 48, 32},
                    {5, 74, 2, 20, 10}, {6, 2, 2, 24, 16},
                };
                textIndices = new int[] {1, 3};
                break;
            default:
                return false;
        }
        int diagnosticsMarker = report.indexOf("grid_track_sizing_diagnostics=");
        if (diagnosticsMarker < 0) return false;
        String diagnosticsJson = report.substring(
                diagnosticsMarker + "grid_track_sizing_diagnostics=".length());
        try {
            Object value = new org.json.JSONTokener(diagnosticsJson).nextValue();
            if (!(value instanceof org.json.JSONObject)) return false;
            org.json.JSONObject diagnostics = (org.json.JSONObject) value;
            org.json.JSONObject revision = diagnostics.optJSONObject("layoutInputRevision");
            org.json.JSONArray inputs = diagnostics.optJSONArray("gridItemIntrinsicInputs");
            org.json.JSONArray itemFrames = diagnostics.optJSONArray("gridItemFrames");
            if (revision == null || revision.optInt("intrinsicSize", 0) != 1
                    || inputs == null || inputs.length() != nodes.length
                    || itemFrames == null || itemFrames.length() != nodes.length) return false;
            for (int index = 0; index < nodes.length; index++) {
                org.json.JSONObject input = inputs.optJSONObject(index);
                org.json.JSONObject source = input == null ? null : input.optJSONObject("source");
                org.json.JSONArray textSources = input == null ? null : input.optJSONArray("textSources");
                org.json.JSONObject frame = itemFrames.optJSONObject(index);
                if (input == null || input.optInt("grid", -1) != 1
                        || input.optInt("node", -1) != nodes[index]
                        || input.optInt("sourceOrder", -1) != index || source == null
                        || !kinds[index].equals(source.optString("kind")) || textSources == null
                        || frame == null || frame.optInt("node", -1) != nodes[index]) return false;
                for (int component = 0; component < 4; component++) {
                    String[] keys = {"x", "y", "width", "height"};
                    double actual = frame.optDouble(keys[component], Double.NaN);
                    if (!Double.isFinite(actual)
                            || Math.abs(actual - expectedFrames[index][component + 1])
                                    > 1.0 / 64.0) return false;
                }
                boolean textItem = c114IsTextInput(textIndices, index);
                if (textItem) {
                    org.json.JSONObject textSource = textSources.optJSONObject(0);
                    if (textSources.length() != 1 || textSource == null
                            || textSource.optInt("node", -1) != nodes[index]
                            || !"XX".equals(textSource.optString("text"))
                            || source.optInt("fontRegistryRevision", -1) != 1
                            || source.optDouble("minContent", Double.NaN) != 20.0
                            || source.optDouble("minContentHeight", Double.NaN) != 10.0
                            || source.optDouble("maxContent", Double.NaN) != 20.0
                            || source.optDouble("maxContentHeight", Double.NaN) != 10.0) return false;
                } else {
                    org.json.JSONObject natural = source.optJSONObject("natural");
                    if (textSources.length() != 0 || natural == null
                            || natural.optDouble("width", Double.NaN) != 24.0
                            || natural.optDouble("height", Double.NaN) != 16.0
                            || !natural.has("aspectRatio") || !natural.isNull("aspectRatio")) {
                        return false;
                    }
                }
            }
            org.json.JSONArray traces = diagnostics.optJSONArray("trackSizingTraces");
            if (traces == null || traces.length() != 1) return false;
            org.json.JSONObject trace = traces.optJSONObject(0);
            org.json.JSONArray passes = trace == null ? null : trace.optJSONArray("passes");
            if (trace == null || trace.optInt("grid", -1) != 1
                    || passes == null || passes.length() != 2) return false;
            org.json.JSONObject columns = passes.optJSONObject(0);
            org.json.JSONObject rows = passes.optJSONObject(1);
            if (columns == null || rows == null
                    || !"Columns".equals(columns.optString("axis"))
                    || !"Rows".equals(rows.optString("axis"))) return false;
            double[] expectedColumns = c114IsAnonymousGridBorderOverflowScene(sceneId)
                    ? c114IsAnonymousGridBorderOverflowThreeImageScene(sceneId)
                            ? new double[] {18, 34, 20, 20}
                            : new double[] {12, 28, 20, 20}
                    : nodes.length == 3 ? new double[] {12, 28, 20, 0}
                            : new double[] {18, 34, 20, 0};
            if (!c114ImageMultiSpanTracksMatch(columns, expectedColumns)
                    || !c114ImageMultiSpanTracksMatch(rows, new double[] {60, 60, 60})) {
                return false;
            }
            org.json.JSONArray stages = columns.optJSONArray("stages");
            if (stages == null) return false;
            java.util.ArrayList<Integer> observedSpans = new java.util.ArrayList<>();
            for (int index = 0; index < stages.length(); index++) {
                org.json.JSONObject stage = stages.optJSONObject(index);
                String name = stage == null ? "" : stage.optString("stage");
                java.util.regex.Matcher matcher = java.util.regex.Pattern
                        .compile("^IntrinsicItemBatch \\{ span: (\\d+),")
                        .matcher(name);
                if (matcher.find()) observedSpans.add(Integer.parseInt(matcher.group(1)));
            }
            int[] expectedSpans = c114IsAnonymousGridBorderOverflowScene(sceneId)
                    ? c114IsAnonymousGridBorderOverflowThreeImageScene(sceneId)
                            ? new int[] {1, 2, 3}
                            : new int[] {1, 2}
                    : nodes.length == 3 ? new int[] {1, 2} : new int[] {1, 2, 3};
            if (observedSpans.size() != expectedSpans.length) return false;
            for (int index = 0; index < expectedSpans.length; index++) {
                if (observedSpans.get(index) != expectedSpans[index]) return false;
            }
            org.json.JSONArray contributions = columns.optJSONArray("contributions");
            if (contributions == null) return false;
            java.util.ArrayList<String> expectedContributions = new java.util.ArrayList<>();
            for (int span : expectedSpans) {
                for (int index = 0; index < nodes.length; index++) {
                    boolean textItem = c114IsTextInput(textIndices, index);
                    if (spans[index] == span && !textItem) {
                        expectedContributions.add(nodes[index] + ":Minimum:" + span + ":0.0");
                    }
                }
                if (c114IsAnonymousGridBorderOverflowScene(sceneId) && span == 1) {
                    for (int index = 0; index < nodes.length; index++) {
                        if (spans[index] != span) continue;
                        for (int repetition = 0; repetition < 2; repetition++) {
                            expectedContributions.add(nodes[index] + ":MinContent:" + span
                                    + ":20.0");
                        }
                    }
                } else {
                    for (int repetition = 0; repetition < 2; repetition++) {
                        for (int index = 0; index < nodes.length; index++) {
                            if (spans[index] != span) continue;
                            boolean textItem = c114IsTextInput(textIndices, index);
                            expectedContributions.add(nodes[index] + ":MinContent:" + span + ":"
                                    + (textItem ? 20.0 : imageWidths[index]));
                        }
                    }
                }
            }
            if (contributions.length() != expectedContributions.size()) return false;
            for (int index = 0; index < contributions.length(); index++) {
                org.json.JSONObject contribution = contributions.optJSONObject(index);
                String expected = expectedContributions.get(index);
                if (contribution == null || !expected.equals(
                        contribution.optInt("item", -1) + ":"
                                + contribution.optString("kind") + ":"
                                + contribution.optInt("span", -1) + ":"
                                + contribution.optDouble("value", Double.NaN))) return false;
            }
            return true;
        } catch (org.json.JSONException | ClassCastException | NumberFormatException error) {
            return false;
        }
    }

    private static boolean c114IsTextInput(int[] textIndices, int index) {
        for (int textIndex : textIndices) {
            if (textIndex == index) return true;
        }
        return false;
    }

    private static boolean c114ExpectedMultiSpanFramesMatch(String sceneId, String report) {
        double[][] expected;
        switch (sceneId) {
            case "competing-span2-forward":
                expected = new double[][] {
                    {0, 0, 220, 40}, {0, 0, 80, 10}, {40, 10, 120, 10},
                };
                break;
            case "competing-span2-reversed":
                expected = new double[][] {
                    {0, 0, 220, 40}, {40, 0, 120, 10}, {0, 10, 80, 10},
                };
                break;
            case "span2-before-span3":
                expected = new double[][] {
                    {0, 0, 220, 40}, {0, 0, 80, 10}, {53.328125, 10, 120, 10},
                    {0, 20, 200, 10},
                };
                break;
            case "span3-before-span2":
                expected = new double[][] {
                    {0, 0, 220, 40}, {0, 0, 200, 10}, {53.328125, 10, 120, 10},
                    {0, 20, 80, 10},
                };
                break;
            case "text-span2-forward":
                expected = new double[][] {
                    {0, 0, 220, 60}, {0, 0, 60, 20}, {25, 20, 70, 20},
                };
                break;
            case "text-span2-reversed":
                expected = new double[][] {
                    {0, 0, 220, 60}, {25, 20, 70, 20}, {0, 0, 60, 20},
                };
                break;
            case "text-span2-before-span3":
                expected = new double[][] {
                    {0, 0, 220, 60}, {0, 0, 116.6666666667, 10},
                    {53.3333333333, 20, 126.6666666667, 10}, {0, 40, 180, 10},
                };
                break;
            case "text-span3-before-span2":
                expected = new double[][] {
                    {0, 0, 220, 60}, {0, 40, 180, 10},
                    {53.3333333333, 20, 126.6666666667, 10}, {0, 0, 116.6666666667, 10},
                };
                break;
            case "image-span2-forward":
                expected = new double[][] {
                    {0, 0, 40, 60}, {1, 1, 24, 16}, {13, 21, 48, 32},
                };
                break;
            case "image-span2-reversed":
                expected = new double[][] {
                    {0, 0, 40, 60}, {13, 21, 48, 32}, {1, 1, 24, 16},
                };
                break;
            case "image-span2-before-span3":
                expected = new double[][] {
                    {0, 0, 40, 60}, {1, 1, 24, 16}, {17, 21, 48, 32}, {1, 41, 72, 48},
                };
                break;
            case "image-span3-before-span2":
                expected = new double[][] {
                    {0, 0, 40, 60}, {1, 41, 72, 48}, {17, 21, 48, 32}, {1, 1, 24, 16},
                };
                break;
            case "mixed-text-image-span2-forward":
                expected = new double[][] {
                    {0, 0, 220, 180}, {0, 0, 61, 20}, {25, 60, 72, 48},
                };
                break;
            case "mixed-text-image-span2-reversed":
                expected = new double[][] {
                    {0, 0, 220, 180}, {25, 60, 72, 48}, {0, 0, 61, 20},
                };
                break;
            case "mixed-text-image-span2-before-span3":
                expected = new double[][] {
                    {0, 0, 220, 180}, {0, 0, 116.3333333333, 10},
                    {52.6666666667, 60, 72, 48}, {0, 120, 180, 10},
                };
                break;
            case "mixed-text-image-span3-before-span2":
                expected = new double[][] {
                    {0, 0, 220, 180}, {0, 120, 180, 10},
                    {52.6666666667, 60, 72, 48}, {0, 0, 116.3333333333, 10},
                };
                break;
            default:
                return true;
        }
        String marker = "node_frames_css_px=[";
        int start = report.indexOf(marker);
        if (start < 0) return false;
        int end = report.indexOf(']', start + marker.length());
        if (end < 0) return false;
        String[] frames = report.substring(start + marker.length(), end).split(";");
        if (frames.length != expected.length) return false;
        String[] propertyPrefixes = {"x=", "y=", "width=", "height="};
        double tolerance = 1.0 / 64.0;
        for (int index = 0; index < frames.length; index++) {
            String[] fields = frames[index].split(",");
            if (fields.length != 5) return false;
            String[] identity = fields[0].split(":node=");
            if (identity.length != 2) return false;
            try {
                int expectedNodeId = c114ExpectedMultiSpanNodeId(sceneId, index);
                if (Integer.parseInt(identity[0]) != index
                        || Integer.parseInt(identity[1]) != expectedNodeId) return false;
                for (int property = 0; property < propertyPrefixes.length; property++) {
                    String prefix = propertyPrefixes[property];
                    if (!fields[property + 1].startsWith(prefix)) return false;
                    double actual = Double.parseDouble(fields[property + 1].substring(prefix.length()));
                    if (Double.isNaN(actual) || Double.isInfinite(actual)
                            || Math.abs(actual - expected[index][property]) > tolerance) {
                        return false;
                    }
                }
            } catch (NumberFormatException | IndexOutOfBoundsException error) {
                return false;
            }
        }
        return true;
    }

    private static boolean c114ExpectedAnonymousTextImageInputs(String sceneId, String report) {
        int[] nodes;
        String[] kinds;
        int[][] textMetrics;
        int[][] sourceNodes;
        String[][] sourceText;
        switch (sceneId) {
            case "anonymous-text-image-text":
                nodes = new int[] {2, 3, 4};
                kinds = new String[] {"text", "replaced", "text"};
                textMetrics = new int[][] {{20, 10, 20, 10}, null, {20, 10, 20, 10}};
                sourceNodes = new int[][] {{2}, {}, {4}};
                sourceText = new String[][] {{"XX"}, {}, {"XX"}};
                break;
            case "image-before-comment-group":
                nodes = new int[] {2, 3};
                kinds = new String[] {"replaced", "text"};
                textMetrics = new int[][] {null, {40, 10, 40, 10}};
                sourceNodes = new int[][] {{}, {3, 4}};
                sourceText = new String[][] {{}, {"XX", "XX"}};
                break;
            case "comment-group-before-image":
                nodes = new int[] {2, 4};
                kinds = new String[] {"text", "replaced"};
                textMetrics = new int[][] {{40, 10, 40, 10}, null};
                sourceNodes = new int[][] {{2, 3}, {}};
                sourceText = new String[][] {{"XX", "XX"}, {}};
                break;
            case "whitespace-around-image":
                nodes = new int[] {3, 4};
                kinds = new String[] {"replaced", "text"};
                textMetrics = new int[][] {null, {20, 10, 20, 10}};
                sourceNodes = new int[][] {{}, {4}};
                sourceText = new String[][] {{}, {"\n\tXX"}};
                break;
            case "hidden-child-around-image":
                nodes = new int[] {2, 5, 6};
                kinds = new String[] {"text", "replaced", "text"};
                textMetrics = new int[][] {{20, 10, 20, 10}, null, {20, 10, 20, 10}};
                sourceNodes = new int[][] {{2}, {}, {6}};
                sourceText = new String[][] {{"XX"}, {}, {"XX"}};
                break;
            default:
                return false;
        }
        int diagnosticsMarker = report.indexOf("grid_track_sizing_diagnostics=");
        if (diagnosticsMarker < 0) return false;
        String diagnosticsJson = report.substring(
                diagnosticsMarker + "grid_track_sizing_diagnostics=".length());
        try {
            Object value = new org.json.JSONTokener(diagnosticsJson).nextValue();
            if (!(value instanceof org.json.JSONObject)) return false;
            org.json.JSONObject diagnostics = (org.json.JSONObject) value;
            org.json.JSONObject revision = diagnostics.optJSONObject("layoutInputRevision");
            org.json.JSONArray inputs = diagnostics.optJSONArray("gridItemIntrinsicInputs");
            if (revision == null || revision.optInt("intrinsicSize", 0) != 1
                    || inputs == null || inputs.length() != nodes.length) return false;
            for (int index = 0; index < nodes.length; index++) {
                org.json.JSONObject input = inputs.optJSONObject(index);
                org.json.JSONObject source = input == null ? null : input.optJSONObject("source");
                org.json.JSONArray textSources = input == null ? null : input.optJSONArray("textSources");
                if (input == null || input.optInt("grid", -1) != 1
                        || input.optInt("node", -1) != nodes[index]
                        || input.optInt("sourceOrder", -1) != index || source == null
                        || !kinds[index].equals(source.optString("kind"))
                        || textSources == null || textSources.length() != sourceNodes[index].length) {
                    return false;
                }
                for (int sourceIndex = 0; sourceIndex < sourceNodes[index].length; sourceIndex++) {
                    org.json.JSONObject textSource = textSources.optJSONObject(sourceIndex);
                    if (textSource == null || textSource.optInt("node", -1)
                                    != sourceNodes[index][sourceIndex]
                            || !sourceText[index][sourceIndex].equals(textSource.optString("text"))) {
                        return false;
                    }
                }
                if ("text".equals(kinds[index])) {
                    int[] metrics = textMetrics[index];
                    if (source.optInt("fontRegistryRevision", -1) != 1
                            || source.optDouble("minContent", Double.NaN) != metrics[0]
                            || source.optDouble("minContentHeight", Double.NaN) != metrics[1]
                            || source.optDouble("maxContent", Double.NaN) != metrics[2]
                            || source.optDouble("maxContentHeight", Double.NaN) != metrics[3]) {
                        return false;
                    }
                } else {
                    org.json.JSONObject natural = source.optJSONObject("natural");
                    if (natural == null || natural.optDouble("width", Double.NaN) != 24.0
                            || natural.optDouble("height", Double.NaN) != 16.0
                            || !natural.has("aspectRatio") || !natural.isNull("aspectRatio")) {
                        return false;
                    }
                }
            }
            return true;
        } catch (org.json.JSONException | ClassCastException error) {
            return false;
        }
    }

    private static int c114ExpectedMultiSpanNodeId(String sceneId, int index) {
        if (c114IsTextMultiSpanScene(sceneId)) return index == 0 ? 1 : index * 2;
        if (c114IsMixedMultiSpanScene(sceneId)) {
            if ("mixed-text-image-span2-reversed".equals(sceneId)) {
                return new int[] {1, 2, 3}[index];
            }
            return new int[] {1, 2, 4, 5}[index];
        }
        return index + 1;
    }

    private static String c114GapNodeFrames(
            String gridWidth, String gridHeight, String secondColumnX, String secondRowY
    ) {
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=" + gridWidth + ",height="
                + gridHeight + ";1:node=2,x=0,y=0,width=40,height=10;2:node=3,x="
                + secondColumnX + ",y=0,width=60,height=10;3:node=4,x=0,y=" + secondRowY
                + ",width=40,height=10;4:node=5,x=" + secondColumnX + ",y=" + secondRowY
                + ",width=60,height=10]";
    }

    private static int c114ExpectedBoxCount(String sceneId) {
        sceneId = c114BaseNestedGridSceneId(sceneId);
        if (sceneId.equals("nested-depth-one-min-content")
                || sceneId.equals("nested-depth-one-max-content")) return 3;
        if (sceneId.equals("nested-depth-two-min-content")
                || sceneId.equals("nested-depth-two-max-content")) return 4;
        if (sceneId.equals("nested-grid-span-two")
                || sceneId.equals("nested-grid-one-track")
                || sceneId.equals("nested-grid-definite-flex-control")
                || sceneId.equals("nested-grid-text-image-span-two")
                || sceneId.equals("nested-grid-text-image-one-track")
                || sceneId.equals("nested-grid-text-image-definite-flex-control")
                || sceneId.equals("nested-grid-text-image-row-span-two")
                || sceneId.equals("nested-grid-text-image-row-one-track")
                || sceneId.equals("nested-grid-text-image-row-definite-flex-control")) return 4;
        if (sceneId.equals("nested-grid-implicit-row-span-two")
                || sceneId.equals("nested-grid-implicit-row-one-row")) return 5;
        if (c114IsAnonymousGridBorderOverflowThreeImageScene(sceneId)) return 4;
        if (c114IsAnonymousGridBorderOverflowScene(sceneId)) return 3;
        if (sceneId.equals("anonymous-text-two-images-forward")
                || sceneId.equals("two-images-anonymous-text-reversed")) return 3;
        if (sceneId.equals("anonymous-text-three-images-forward")
                || sceneId.equals("three-images-anonymous-text-reversed")) return 4;
        if (c114IsAnonymousTextImageScene(sceneId)) return 2;
        if (sceneId.equals("anonymous-min-content-wrap")
                || sceneId.equals("anonymous-comment-continuation")
                || sceneId.equals("anonymous-hidden-child-continuation")) return 1;
        if (sceneId.equals("anonymous-runs-around-block")) return 2;
        if (sceneId.equals("whitespace-does-not-create-item")) return 3;
        if (sceneId.equals("image-span2-forward")
                || sceneId.equals("image-span2-reversed")) return 3;
        if (sceneId.equals("image-span2-before-span3")
                || sceneId.equals("image-span3-before-span2")) return 4;
        if (sceneId.equals("mixed-text-image-span2-forward")
                || sceneId.equals("mixed-text-image-span2-reversed")) return 3;
        if (sceneId.equals("mixed-text-image-span2-before-span3")
                || sceneId.equals("mixed-text-image-span3-before-span2")) return 4;
        if (sceneId.equals("competing-span2-forward")
                || sceneId.equals("competing-span2-reversed")) return 3;
        if (sceneId.equals("span2-before-span3")
                || sceneId.equals("span3-before-span2")) return 4;
        if (sceneId.equals("text-span2-forward")
                || sceneId.equals("text-span2-reversed")) return 3;
        if (sceneId.equals("text-span2-before-span3")
                || sceneId.equals("text-span3-before-span2")) return 4;
        if (sceneId.equals("typed-fixed-cycle-three-leading-trailing")) return 4;
        if (sceneId.equals("typed-mixed-cycle")
                || sceneId.equals("typed-auto-fit-occupied")
                || sceneId.equals("typed-auto-fit-gutter")) return 3;
        if (sceneId.equals("columns-fit-negative-and-repeat-occupied")
                || sceneId.equals("columns-fill-negative-positive-cycle")) return 3;
        if (c114IsAutoRepeatNegativeLineScene(sceneId)) return 2;
        int implicitTrackBoxCount = c114ImplicitTextBoxCount(sceneId);
        if (implicitTrackBoxCount != 0) return implicitTrackBoxCount;
        String[] negativeTrackExpectation = c114NegativeTrackExpectation(sceneId);
        if (negativeTrackExpectation != null) return Integer.parseInt(negativeTrackExpectation[5]);
        String autoFitMinimumSizeItemFrame = c114AutoFitMinimumSizeItemFrame(sceneId);
        if (autoFitMinimumSizeItemFrame != null) return autoFitMinimumSizeItemFrame.isEmpty() ? 1 : 2;
        if (sceneId.equals("auto-fit-span-neighbors")) return 3;
        if (sceneId.equals("auto-fit-empty-neighbors")
                || sceneId.equals("auto-fill-max-size-neighbors")
                || sceneId.equals("auto-fill-min-size-neighbors")
                || sceneId.equals("both-fit-empty")
                || sceneId.equals("columns-fit-rows-fill-empty")
                || sceneId.equals("columns-fill-rows-fit-empty")
                || sceneId.startsWith("auto-fill-two-track-")) return 1;
        return sceneId.startsWith("grid-gap-") ? 5 : 2;
    }

    private static String c114BaseNestedGridSceneId(String sceneId) {
        return sceneId.endsWith("-two-axis")
                ? sceneId.substring(0, sceneId.length() - "-two-axis".length()) : sceneId;
    }

    private static boolean c114IsImplicitTextScene(String sceneId) {
        return c114ImplicitTextRootFrame(sceneId) != null;
    }

    private static boolean c114IsAnonymousTextScene(String sceneId) {
        return sceneId.equals("anonymous-min-content-wrap")
                || sceneId.equals("anonymous-comment-continuation")
                || sceneId.equals("anonymous-runs-around-block")
                || sceneId.equals("anonymous-hidden-child-continuation")
                || sceneId.equals("whitespace-does-not-create-item");
    }

    private static boolean c114IsAnonymousTextImageScene(String sceneId) {
        return sceneId.equals("anonymous-text-image-text")
                || sceneId.equals("image-before-comment-group")
                || sceneId.equals("comment-group-before-image")
                || sceneId.equals("whitespace-around-image")
                || sceneId.equals("hidden-child-around-image");
    }

    private static boolean c114IsAnonymousTextMultiImageScene(String sceneId) {
        return sceneId.equals("anonymous-text-two-images-forward")
                || sceneId.equals("two-images-anonymous-text-reversed")
                || sceneId.equals("anonymous-text-three-images-forward")
                || sceneId.equals("three-images-anonymous-text-reversed")
                || c114IsAnonymousGridBorderOverflowScene(sceneId);
    }

    private static boolean c114IsAnonymousGridBorderOverflowScene(String sceneId) {
        return sceneId.equals("anonymous-runs-two-images-forward")
                || sceneId.equals("two-images-anonymous-runs-reversed")
                || c114IsAnonymousGridBorderOverflowThreeImageScene(sceneId);
    }

    private static boolean c114IsAnonymousGridBorderOverflowThreeImageScene(String sceneId) {
        return sceneId.equals("two-anonymous-runs-three-images-forward")
                || sceneId.equals("three-images-two-anonymous-runs-reversed");
    }

    private static boolean c114IsAutoRepeatNegativeLineScene(String sceneId) {
        for (String candidate : C114_AUTO_REPEAT_NEGATIVE_LINE_SCENE_IDS) {
            if (candidate.equals(sceneId)) return true;
        }
        return false;
    }

    private static String c114ImplicitTextRootFrame(String sceneId) {
        switch (sceneId) {
            case "row-min-content":
            case "row-max-content":
            case "row-auto-placement-min-content":
            case "row-auto-placement-auto":
            case "row-positive-implicit-locked-item":
                return "width=40,height=80";
            case "column-min-content":
            case "column-max-content":
            case "column-auto-placement-max-content":
            case "column-auto-placement-auto":
            case "implicit-percentage-column":
            case "implicit-fraction-column":
            case "implicit-minmax-column":
            case "implicit-fit-content-column":
            case "implicit-calc-column":
            case "column-positive-implicit-locked-item":
                return "width=160,height=40";
            case "two-implicit-columns-min-content":
            case "cycled-intrinsic-columns":
                return "width=240,height=40";
            case "cycled-fixed-max-rows":
                return "width=40,height=100";
            case "implicit-min-content-overflow-border":
                return "width=44,height=44";
            case "explicit-track-control":
                return "width=40,height=24";
            default:
                return null;
        }
    }

    private static int c114ImplicitTextBoxCount(String sceneId) {
        switch (sceneId) {
            case "row-min-content":
            case "row-max-content":
            case "column-min-content":
            case "column-max-content":
            case "row-positive-implicit-locked-item":
            case "column-positive-implicit-locked-item":
            case "explicit-track-control":
                return 2;
            case "row-auto-placement-min-content":
            case "column-auto-placement-max-content":
            case "row-auto-placement-auto":
            case "column-auto-placement-auto":
            case "two-implicit-columns-min-content":
            case "implicit-percentage-column":
            case "implicit-fraction-column":
            case "implicit-minmax-column":
            case "implicit-fit-content-column":
            case "implicit-min-content-overflow-border":
            case "implicit-calc-column":
                return 3;
            case "cycled-intrinsic-columns":
            case "cycled-fixed-max-rows":
                return 5;
            default:
                return 0;
        }
    }

    private static boolean c114IsAutoFitMinimumSizeScene(String sceneId) {
        return sceneId.startsWith("auto-fit-min-");
    }

    private static String c114AutoFitMinimumSizeRootFrame(String sceneId) {
        if (!c114IsAutoFitMinimumSizeScene(sceneId)) return null;
        String[] parts = sceneId.split("-");
        int size = Integer.parseInt(parts[parts.length - 1]);
        return sceneId.contains("-columns-")
                ? "width=" + size + ",height=60"
                : "width=50,height=" + size;
    }

    private static String c114AutoFitMinimumSizeItemFrame(String sceneId) {
        if (!c114IsAutoFitMinimumSizeScene(sceneId)) return null;
        return sceneId.contains("-single-") || sceneId.contains("-span-two-")
                || sceneId.contains("-second-repeat-") ? "width=12,height=12" : "";
    }

    private static String c114AutoFitMinimumSizeNodeFrames(String sceneId) {
        String rootFrame = c114AutoFitMinimumSizeRootFrame(sceneId);
        if (rootFrame == null) return null;
        String itemFrame = c114AutoFitMinimumSizeItemFrame(sceneId);
        if (itemFrame.isEmpty()) {
            return "node_frames_css_px=[0:node=1,x=0,y=0," + rootFrame + "]";
        }
        String position = sceneId.contains("-second-repeat-")
                ? sceneId.contains("-columns-") ? "x=20,y=0" : "x=0,y=20"
                : sceneId.contains("-columns-") ? "x=28,y=0" : "x=0,y=28";
        return "node_frames_css_px=[0:node=1,x=0,y=0," + rootFrame
                + ";1:node=2," + position + ",width=12,height=12]";
    }

    private static boolean isC114GridTrackSizingPixelSamples(
            String sceneId,
            List<SurfaceViewPixelCapture.Sample> samples
    ) {
        if (sceneId.equals("nested-grid-definite-flex-control")) {
            if (samples.size() != 5) return false;
            String rootId = "c114-" + sceneId + "-grid";
            String nestedId = "c114-" + sceneId + "-nested";
            String itemBId = "c114-" + sceneId + "-item-b";
            boolean internalGap = false;
            boolean insideOuterEdge = false;
            boolean outsideOuterEdge = false;
            boolean overflowTail = false;
            boolean afterOverflow = false;
            for (SurfaceViewPixelCapture.Sample sample : samples) {
                switch (sample.id) {
                    case "nested-grid-internal-gap":
                        internalGap = sample.targetNodeId.equals(nestedId)
                                && sample.xCssPx == 72 && sample.yCssPx == 10;
                        if (!internalGap) return false;
                        break;
                    case "nested-grid-inside-outer-edge":
                        insideOuterEdge = sample.targetNodeId.equals(itemBId)
                                && sample.xCssPx == 119 && sample.yCssPx == 10;
                        if (!insideOuterEdge) return false;
                        break;
                    case "nested-grid-outside-outer-edge":
                        outsideOuterEdge = sample.targetNodeId.equals(itemBId)
                                && sample.xCssPx == 120 && sample.yCssPx == 10;
                        if (!outsideOuterEdge) return false;
                        break;
                    case "nested-grid-overflow-tail":
                        overflowTail = sample.targetNodeId.equals(itemBId)
                                && sample.xCssPx == 133 && sample.yCssPx == 10;
                        if (!overflowTail) return false;
                        break;
                    case "nested-grid-after-overflow":
                        afterOverflow = sample.targetNodeId.equals(rootId)
                                && sample.xCssPx == 134 && sample.yCssPx == 10;
                        if (!afterOverflow) return false;
                        break;
                    default:
                        return false;
                }
            }
            return internalGap && insideOuterEdge && outsideOuterEdge
                    && overflowTail && afterOverflow;
        }
        if (c114IsAnonymousGridBorderOverflowScene(sceneId)) {
            if (samples.size() != 2) return false;
            String rootId = "c114-" + sceneId + "-grid";
            boolean border = false;
            boolean overflowImage = false;
            for (SurfaceViewPixelCapture.Sample sample : samples) {
                switch (sample.id) {
                    case "top-border-color":
                        border = sample.targetNodeId.equals(rootId)
                                && sample.xCssPx == 10 && sample.yCssPx == 0;
                        if (!border) return false;
                        break;
                    case "image-overflow-png-quadrant":
                        overflowImage = !c114IsAnonymousGridBorderOverflowThreeImageScene(sceneId)
                                && sample.targetNodeId.equals(rootId)
                                && sample.xCssPx == 51 && sample.yCssPx == 71;
                        if (!overflowImage) return false;
                        break;
                    case "image-c-overflow-green-png-texel":
                        overflowImage = c114IsAnonymousGridBorderOverflowThreeImageScene(sceneId)
                                && sample.targetNodeId.equals(rootId)
                                && sample.xCssPx == 62 && sample.yCssPx == 128;
                        if (!overflowImage) return false;
                        break;
                    default:
                        return false;
                }
            }
            return border && overflowImage;
        }
        if (c114IsImageMultiSpanScene(sceneId)) {
            return c114ImageMultiSpanPixelSamplesMatch(sceneId, samples);
        }
        if ("columns-fill-negative-positive-cycle".equals(sceneId)) {
            if (samples.size() != 5) return false;
            String rootId = "c114-" + sceneId + "-grid";
            String overflowItemId = "c114-" + sceneId + "-item-2";
            boolean interior = false;
            boolean rootEdge = false;
            boolean exterior = false;
            boolean overflowItem = false;
            boolean overflowTail = false;
            for (SurfaceViewPixelCapture.Sample sample : samples) {
                switch (sample.id) {
                    case "negative-cycle-root-interior":
                        interior = sample.targetNodeId.equals(rootId)
                                && sample.xCssPx == 100 && sample.yCssPx == 10;
                        if (!interior) return false;
                        break;
                    case "negative-cycle-root-edge":
                        rootEdge = sample.targetNodeId.equals(rootId)
                                && sample.xCssPx == 199 && sample.yCssPx == 10;
                        if (!rootEdge) return false;
                        break;
                    case "negative-cycle-root-exterior":
                        exterior = sample.targetNodeId.equals(rootId)
                                && sample.xCssPx == 200 && sample.yCssPx == 10;
                        if (!exterior) return false;
                        break;
                    case "negative-cycle-overflow-item":
                        overflowItem = sample.targetNodeId.equals(overflowItemId)
                                && sample.xCssPx == 290 && sample.yCssPx == 10;
                        if (!overflowItem) return false;
                        break;
                    case "negative-cycle-overflow-tail":
                        overflowTail = sample.targetNodeId.equals(rootId)
                                && sample.xCssPx == 319 && sample.yCssPx == 10;
                        if (!overflowTail) return false;
                        break;
                    default:
                        return false;
                }
            }
            return interior && rootEdge && exterior && overflowItem && overflowTail;
        }
        if ("implicit-min-content-overflow-border".equals(sceneId)) {
            if (samples.size() != 5) return false;
            boolean edgeInside = false;
            boolean borderCovered = false;
            boolean edgeOutside = false;
            boolean overflow = false;
            boolean afterItem = false;
            for (SurfaceViewPixelCapture.Sample sample : samples) {
                switch (sample.id) {
                    case "overflow-root-border-inside":
                        edgeInside = sample.targetNodeId.equals("c114-" + sceneId + "-grid")
                                && sample.xCssPx == 0 && sample.yCssPx == 8;
                        if (!edgeInside) return false;
                        break;
                    case "overflow-root-border-covered":
                        borderCovered = sample.targetNodeId.equals("c114-" + sceneId + "-grid")
                                && sample.xCssPx == 43 && sample.yCssPx == 8;
                        if (!borderCovered) return false;
                        break;
                    case "overflow-root-border-outside":
                        edgeOutside = sample.targetNodeId.equals("c114-" + sceneId + "-grid")
                                && sample.xCssPx == 44 && sample.yCssPx == 8;
                        if (!edgeOutside) return false;
                        break;
                    case "overflow-item-outside-root":
                        overflow = sample.targetNodeId.equals("c114-" + sceneId + "-item-2")
                                && sample.xCssPx == 91 && sample.yCssPx == 8;
                        if (!overflow) return false;
                        break;
                    case "overflow-after-item":
                        afterItem = sample.targetNodeId.equals("c114-" + sceneId + "-grid")
                                && sample.xCssPx == 92 && sample.yCssPx == 8;
                        if (!afterItem) return false;
                        break;
                    default:
                        return false;
                }
            }
            return edgeInside && borderCovered && edgeOutside && overflow && afterItem;
        }
        if ("columns-negative-fixed-root-overflow".equals(sceneId)
                || "rows-negative-fixed-root-overflow".equals(sceneId)) {
            if (samples.size() != 5) return false;
            boolean columns = sceneId.startsWith("columns-");
            String rootId = "c114-" + sceneId + "-grid";
            String itemId = "c114-" + sceneId + "-item-5";
            boolean interior = false;
            boolean rootEdge = false;
            boolean exterior = false;
            boolean overflowItem = false;
            boolean overflowTail = false;
            for (SurfaceViewPixelCapture.Sample sample : samples) {
                switch (sample.id) {
                    case "negative-root-interior":
                        interior = sample.targetNodeId.equals(rootId)
                                && sample.xCssPx == (columns ? 33 : 10)
                                && sample.yCssPx == (columns ? 10 : 19);
                        if (!interior) return false;
                        break;
                    case "negative-root-edge":
                        rootEdge = sample.targetNodeId.equals(rootId)
                                && sample.xCssPx == (columns ? 0 : 10)
                                && sample.yCssPx == (columns ? 10 : 0);
                        if (!rootEdge) return false;
                        break;
                    case "negative-root-exterior":
                        exterior = sample.targetNodeId.equals(rootId)
                                && sample.xCssPx == (columns ? 150 : 10)
                                && sample.yCssPx == (columns ? 10 : 100);
                        if (!exterior) return false;
                        break;
                    case "negative-overflow-item":
                        overflowItem = sample.targetNodeId.equals(itemId)
                                && sample.xCssPx == (columns ? 250 : 10)
                                && sample.yCssPx == (columns ? 10 : 195);
                        if (!overflowItem) return false;
                        break;
                    case "negative-overflow-tail":
                        overflowTail = sample.targetNodeId.equals(rootId)
                                && sample.xCssPx == (columns ? 273 : 10)
                                && sample.yCssPx == (columns ? 10 : 209);
                        if (!overflowTail) return false;
                        break;
                    default:
                        return false;
                }
            }
            return interior && rootEdge && exterior && overflowItem && overflowTail;
        }
        if ("auto-fill-overflow-neighbors".equals(sceneId)) {
            if (samples.size() != 4) return false;
            boolean edgeInside = false;
            boolean edgeOutside = false;
            boolean overflow = false;
            boolean background = false;
            for (SurfaceViewPixelCapture.Sample sample : samples) {
                switch (sample.id) {
                    case "overflow-root-edge-inside":
                        edgeInside = sample.targetNodeId.equals("c114-" + sceneId + "-grid")
                                && sample.xCssPx == 39 && sample.yCssPx == 8;
                        if (!edgeInside) return false;
                        break;
                    case "overflow-root-edge-outside":
                        edgeOutside = sample.targetNodeId.equals("c114-" + sceneId + "-grid")
                                && sample.xCssPx == 40 && sample.yCssPx == 8;
                        if (!edgeOutside) return false;
                        break;
                    case "overflow-item-outside-root":
                        overflow = sample.targetNodeId.equals("c114-" + sceneId + "-item")
                                && sample.xCssPx == 91 && sample.yCssPx == 8;
                        if (!overflow) return false;
                        break;
                    case "overflow-root-background":
                        background = sample.targetNodeId.equals("c114-" + sceneId + "-grid")
                                && sample.xCssPx == 10 && sample.yCssPx == 30;
                        if (!background) return false;
                        break;
                    default:
                        return false;
                }
            }
            return edgeInside && edgeOutside && overflow && background;
        }
        if (sceneId.endsWith("-min-basis")) {
            boolean calc = sceneId.equals("grid-gap-cyclic-calc-min-basis");
            if (samples.size() != (calc ? 6 : 5)) return false;
            boolean gapBackground = false;
            boolean rowGapBackground = false;
            boolean insideItem = false;
            boolean secondColumn = false;
            boolean secondRow = false;
            boolean belowGrid = !calc;
            for (SurfaceViewPixelCapture.Sample sample : samples) {
                switch (sample.id) {
                    case "gap-background":
                        gapBackground = sample.targetNodeId.equals(sceneId + "-grid")
                                && sample.xCssPx == 45 && sample.yCssPx == 5;
                        if (!gapBackground) return false;
                        break;
                    case "row-gap-background":
                        rowGapBackground = sample.targetNodeId.equals(sceneId + "-grid")
                                && sample.xCssPx == 5 && sample.yCssPx == 12;
                        if (!rowGapBackground) return false;
                        break;
                    case "inside-item":
                        insideItem = sample.targetNodeId.equals(sceneId + "-item-1")
                                && sample.xCssPx == 5 && sample.yCssPx == 5;
                        if (!insideItem) return false;
                        break;
                    case "second-column":
                        secondColumn = sample.targetNodeId.equals(sceneId + "-item-2")
                                && sample.xCssPx == 60 && sample.yCssPx == 5;
                        if (!secondColumn) return false;
                        break;
                    case "second-row":
                        secondRow = sample.targetNodeId.equals(sceneId + "-item-3")
                                && sample.xCssPx == 5 && sample.yCssPx == (calc ? 25 : 20);
                        if (!secondRow) return false;
                        break;
                    case "below-grid":
                        belowGrid = calc && sample.targetNodeId.equals(sceneId + "-item-3")
                                && sample.xCssPx == 5 && sample.yCssPx == 31;
                        if (!belowGrid) return false;
                        break;
                    default:
                        return false;
                }
            }
            return gapBackground && rowGapBackground && insideItem && secondColumn
                    && secondRow && belowGrid;
        }
        if (sceneId.startsWith("grid-gap-")) {
            boolean cyclic = sceneId.equals("grid-gap-cyclic-percent")
                    || sceneId.equals("grid-gap-cyclic-calc");
            boolean calc = sceneId.endsWith("-calc");
            int rightX = calc ? (cyclic ? 111 : 110) : 105;
            int belowY = sceneId.equals("grid-gap-cyclic-calc") ? 28 : 22;
            if (samples.size() != (cyclic ? 5 : 4)) return false;
            boolean gapBackground = false;
            boolean rowGapBackground = false;
            boolean insideItem = false;
            boolean rightOfGrid = false;
            boolean belowGrid = !cyclic;
            for (SurfaceViewPixelCapture.Sample sample : samples) {
                switch (sample.id) {
                    case "gap-background":
                        gapBackground = sample.targetNodeId.equals(sceneId + "-grid")
                                && sample.xCssPx == 45 && sample.yCssPx == 5;
                        if (!gapBackground) return false;
                        break;
                    case "row-gap-background":
                        rowGapBackground = sample.targetNodeId.equals(sceneId + "-grid")
                                && sample.xCssPx == 5 && sample.yCssPx == 12;
                        if (!rowGapBackground) return false;
                        break;
                    case "inside-item":
                        insideItem = sample.targetNodeId.equals(sceneId + "-item-1")
                                && sample.xCssPx == 5 && sample.yCssPx == 5;
                        if (!insideItem) return false;
                        break;
                    case "right-of-grid":
                        rightOfGrid = sample.targetNodeId.equals(sceneId + "-item-2")
                                && sample.xCssPx == rightX && sample.yCssPx == 5;
                        if (!rightOfGrid) return false;
                        break;
                    case "below-grid":
                        belowGrid = cyclic && sample.targetNodeId.equals(sceneId + "-item-4")
                                && sample.xCssPx == 5 && sample.yCssPx == belowY;
                        if (!belowGrid) return false;
                        break;
                    default:
                        return false;
                }
            }
            return gapBackground && rowGapBackground && insideItem && rightOfGrid && belowGrid;
        }
        boolean fixedGutterColumn = "fixed-gutter-column-suggestion-clamp".equals(sceneId);
        boolean fixedGutterRow = "fixed-gutter-row-suggestion-clamp".equals(sceneId);
        if (fixedGutterColumn || fixedGutterRow) {
            if (samples.size() != 4) return false;
            boolean border = false;
            boolean inside = false;
            boolean right = false;
            boolean below = false;
            for (SurfaceViewPixelCapture.Sample sample : samples) {
                switch (sample.id) {
                    case "grid-border":
                        border = sample.xCssPx == 0 && sample.yCssPx == 2;
                        if (!border) return false;
                        break;
                    case "inside-item":
                        inside = sample.xCssPx == 2 && sample.yCssPx == 2;
                        if (!inside) return false;
                        break;
                    case "right-of-grid":
                        right = sample.xCssPx == (fixedGutterColumn ? 12 : 6)
                                && sample.yCssPx == 2;
                        if (!right) return false;
                        break;
                    case "below-grid":
                        below = sample.xCssPx == 2
                                && sample.yCssPx == (fixedGutterColumn ? 6 : 12);
                        if (!below) return false;
                        break;
                    default:
                        return false;
                }
            }
            return border && inside && right && below;
        }
        if (samples.size() != 3) return false;
        boolean itemInsideRoot = false;
        boolean itemOutsideRoot = false;
        boolean rootBackground = false;
        boolean fixedSpanRightOfGrid = false;
        boolean fixedSpanBelowGrid = false;
        boolean imageOverflow = "fixed-small-track-overflow".equals(sceneId);
        boolean ratioOverflow = "column-auto-natural-ratio".equals(sceneId);
        boolean rowTransferOverflow =
                "row-transfer-input-capped-by-fixed-column-span".equals(sceneId);
        boolean columnTransferOverflow =
                "column-transfer-input-capped-by-fixed-row-span".equals(sceneId);
        for (SurfaceViewPixelCapture.Sample sample : samples) {
            switch (sample.id) {
                case "inside-item":
                    itemInsideRoot = rowTransferOverflow
                            ? sample.xCssPx == 4 && sample.yCssPx == 2
                            : columnTransferOverflow
                            ? sample.xCssPx == 2 && sample.yCssPx == 2 : false;
                    if (!itemInsideRoot) return false;
                    break;
                case "right-of-grid":
                    fixedSpanRightOfGrid = rowTransferOverflow
                            ? sample.xCssPx == 12 && sample.yCssPx == 2
                            : columnTransferOverflow
                            ? sample.xCssPx == 8 && sample.yCssPx == 2 : false;
                    if (!fixedSpanRightOfGrid) return false;
                    break;
                case "below-grid":
                    fixedSpanBelowGrid = rowTransferOverflow
                            ? sample.xCssPx == 4 && sample.yCssPx == 8
                            : columnTransferOverflow
                            ? sample.xCssPx == 2 && sample.yCssPx == 12 : false;
                    if (!fixedSpanBelowGrid) return false;
                    break;
                case "item-inside-root":
                    itemInsideRoot = imageOverflow
                            ? sample.xCssPx == 5 && sample.yCssPx == 8
                            : ratioOverflow
                            ? sample.xCssPx == 5 && sample.yCssPx == 4
                            : sample.xCssPx == 5 && sample.yCssPx == 20;
                    if (!itemInsideRoot) return false;
                    break;
                case "item-overflow-beyond-root":
                    itemOutsideRoot = imageOverflow
                            ? sample.xCssPx == 20 && sample.yCssPx == 8
                            : ratioOverflow
                            ? sample.xCssPx == 10 && sample.yCssPx == 4
                            : sample.xCssPx == 5 && sample.yCssPx == 45;
                    if (!itemOutsideRoot) return false;
                    break;
                case "root-background-control":
                    rootBackground = imageOverflow
                            ? sample.xCssPx == 6 && sample.yCssPx == 25
                            : ratioOverflow
                            ? sample.xCssPx == 6 && sample.yCssPx == 15
                            : sample.xCssPx == 15 && sample.yCssPx == 20;
                    if (!rootBackground) return false;
                    break;
                default:
                    return false;
            }
        }
        return rowTransferOverflow || columnTransferOverflow
                ? itemInsideRoot && fixedSpanRightOfGrid && fixedSpanBelowGrid
                : itemInsideRoot && itemOutsideRoot && rootBackground;
    }

    private static boolean c114ImageMultiSpanPixelSamplesMatch(
            String sceneId, List<SurfaceViewPixelCapture.Sample> samples
    ) {
        boolean threeImages = sceneId.endsWith("before-span3")
                || sceneId.startsWith("image-span3-");
        if (samples.size() != (threeImages ? 5 : 4)) return false;
        String rootId = "c114-" + sceneId + "-grid";
        String overflowImageId = sceneId.equals("image-span2-reversed")
                ? "image-b-reversed" : sceneId.equals("image-span2-forward")
                ? "image-b-forward" : sceneId.equals("image-span3-before-span2")
                ? "image-r-c" : "image-c-c";
        boolean background = false;
        boolean outside = false;
        boolean overflow = false;
        boolean below = !threeImages;
        boolean after = false;
        for (SurfaceViewPixelCapture.Sample sample : samples) {
            switch (sample.id) {
                case "image-multispan-root-background":
                    background = sample.targetNodeId.equals(rootId)
                            && sample.xCssPx == (threeImages ? 35 : 30)
                            && sample.yCssPx == 10;
                    if (!background) return false;
                    break;
                case "image-multispan-outside-root":
                    outside = sample.targetNodeId.equals(rootId)
                            && sample.xCssPx == 40 && sample.yCssPx == 10;
                    if (!outside) return false;
                    break;
                case "image-multispan-item-overflow":
                    overflow = sample.targetNodeId.equals(overflowImageId)
                            && sample.xCssPx == (threeImages ? 65 : 45)
                            && sample.yCssPx == (threeImages ? 45 : 30);
                    if (!overflow) return false;
                    break;
                case "image-multispan-item-below-root":
                    below = threeImages && sample.targetNodeId.equals(overflowImageId)
                            && sample.xCssPx == 10 && sample.yCssPx == 70;
                    if (!below) return false;
                    break;
                case "image-multispan-after-item":
                    after = sample.targetNodeId.equals(rootId)
                            && sample.xCssPx == (threeImages ? 75 : 62)
                            && sample.yCssPx == (threeImages ? 70 : 30);
                    if (!after) return false;
                    break;
                default:
                    return false;
            }
        }
        return background && outside && overflow && below && after;
    }

    private static boolean isC1133GridTemplateAreasPixelSamples(
            String sceneId, List<SurfaceViewPixelCapture.Sample> samples
    ) {
        if (C1133_GRID_TEMPLATE_AREAS_DEFAULT_SCENE_ID.equals(sceneId)) {
            if (samples.size() != 2) return false;
            boolean background = false;
            boolean blue = false;
            for (SurfaceViewPixelCapture.Sample sample : samples) {
                switch (sample.id) {
                    case "grid-background":
                        background = "c1133-main-grid".equals(sample.targetNodeId)
                                && sample.xCssPx == 150 && sample.yCssPx == 10;
                        if (!background) return false;
                        break;
                    case "item-blue":
                        blue = "c1133-main-item".equals(sample.targetNodeId)
                                && sample.xCssPx == 100 && sample.yCssPx == 50;
                        if (!blue) return false;
                        break;
                    default:
                        return false;
                }
            }
            return background && blue;
        }
        if (!"unknown-area-fixed-implicit-tracks".equals(sceneId) || samples.size() != 3) {
            return false;
        }
        boolean background = false;
        boolean knownArea = false;
        boolean overflow = false;
        for (SurfaceViewPixelCapture.Sample sample : samples) {
            switch (sample.id) {
                case "grid-interior":
                    background = "fixed-implicit-grid".equals(sample.targetNodeId)
                            && sample.xCssPx == 150 && sample.yCssPx == 110;
                    if (!background) return false;
                    break;
                case "known-area":
                    knownArea = "fixed-known-area".equals(sample.targetNodeId)
                            && sample.xCssPx == 10 && sample.yCssPx == 10;
                    if (!knownArea) return false;
                    break;
                case "unknown-area-overflow":
                    overflow = "fixed-unknown-area".equals(sample.targetNodeId)
                            && sample.xCssPx == 182 && sample.yCssPx == 127;
                    if (!overflow) return false;
                    break;
                default:
                    return false;
            }
        }
        return background && knownArea && overflow;
    }
    private static boolean isC1121PixelScene(String sceneId) {
        for (String candidate : C1121_GRID_PIXEL_SCENE_IDS) {
            if (candidate.equals(sceneId)) return true;
        }
        return false;
    }
    private static boolean isC1122GridAutoPlacementScene(String sceneId) {
        for (String candidate : C1122_GRID_AUTO_PLACEMENT_SCENE_IDS) {
            if (candidate.equals(sceneId)) return true;
        }
        return false;
    }
    private static boolean isC1123GridAutoPlacementScene(String sceneId) {
        for (String candidate : C1123_GRID_AUTO_PLACEMENT_SCENE_IDS) {
            if (candidate.equals(sceneId)) return true;
        }
        return false;
    }
    private static boolean isC1122GridPixelScene(String sceneId) {
        for (String candidate : C1122_GRID_PIXEL_SCENE_IDS) {
            if (candidate.equals(sceneId)) return true;
        }
        return false;
    }
    private static boolean isC1123GridPixelScene(String sceneId) {
        for (String candidate : C1123_GRID_PIXEL_SCENE_IDS) {
            if (candidate.equals(sceneId)) return true;
        }
        return false;
    }
    private static boolean isC1131GridRepeatPixelSamples(
            List<SurfaceViewPixelCapture.Sample> samples
    ) {
        if (samples.size() != 5) return false;
        boolean background = false;
        boolean blue = false;
        boolean red = false;
        boolean columnGap = false;
        boolean rowGap = false;
        for (SurfaceViewPixelCapture.Sample sample : samples) {
            switch (sample.id) {
                case "grid-background":
                    background = sample.xCssPx == 300 && sample.yCssPx == 120;
                    if (!background) return false;
                    break;
                case "item-blue":
                    blue = sample.xCssPx == 10 && sample.yCssPx == 10;
                    if (!blue) return false;
                    break;
                case "item-red":
                    red = sample.xCssPx == 40 && sample.yCssPx == 10;
                    if (!red) return false;
                    break;
                case "column-gap":
                    columnGap = sample.xCssPx == 112 && sample.yCssPx == 10;
                    if (!columnGap) return false;
                    break;
                case "row-gap":
                    rowGap = sample.xCssPx == 10 && sample.yCssPx == 53;
                    if (!rowGap) return false;
                    break;
                default:
                    return false;
            }
        }
        return background && blue && red && columnGap && rowGap;
    }
    private static native long nativeCreateHost(boolean registeredPropertiesFixture);
    private static native long nativeCreateHostWithRootHtml(byte[] source);
    private static native long nativeCreateBlockPaintHost();
    private static native long nativeCreateRootHtmlBlockPaintHost();
    private static native long nativeCreateRootHtmlTextPaintHost();
    private static native long nativeCreateBlockFormattingHost();
    private static native long nativeCreateC121PositionHost();
    private static native long nativeCreateC122AbsoluteBlockHost();
    private static native long nativeCreateC123FixedHost();
    private static native long nativeCreateC124StackingHost();
    private static native long nativeCreateC131OverflowClipHost();
    private static native long nativeCreateC131CompactSurfaceMatrixHost();
    private static native long nativeCreateC131OverflowClipBorderHost();
    private static native long nativeCreateC131StackingDescendantClipHost();
    private static native long nativeCreateC132ScrollHost();
    private static native long nativeCreateC133GestureHost(int scenarioId);
    private static native long nativeCreateC11GridHost();
    private static native long nativeCreateC1121GridPlacementHost();
    private static native long nativeCreateC1122GridAutoPlacementHost();
    private static native long nativeCreateC1123GridAutoPlacementHost();
    private static native long nativeCreateC1131GridRepeatHost();
    private static native long nativeCreateC1132GridNamedLinesHost();
    private static native long nativeCreateC1133GridTemplateAreasHost();
    private static native long nativeCreateC114GridTrackSizingHost();
    private static native byte[] nativeC124WorkerStackResidencySample(long host);
    private static native long nativeCreateC1035PositionedFlexHost();
    private static native long nativeBeginPresentationUpdate(long host);
    private static native byte[] nativeSetEnvironment(
            long host, float widthCssPx, float heightCssPx, float scale, boolean dark);
    private static native byte[] nativeEvalFixture(long host);
    private static native byte[] nativeEvalRootHtmlApplicationScript(long host, String source);
    private static native byte[] nativeEvalAuthorStylesheetsFixture(long host);
    private static native byte[] nativeEvalCustomPropertiesFixture(long host);
    private static native byte[] nativeEvalRegisteredPropertiesFixture(long host);
    private static native byte[] nativeEvalRuntimeResultCacheFixture(long host);
    private static native byte[] nativeEvalIncrementalRestyleFixture(long host);
    private static native byte[] nativeEvalPercentageDimensionsFixture(long host);
    private static native byte[] nativeEvalSpacingPercentagesFixture(long host);
    private static native byte[] nativeEvalAbsoluteLengthsFixture(long host);
    private static native byte[] nativeEvalFontRelativeUnitsFixture(long host);
    private static native byte[] nativeEvalTypedCssMathFixture(long host);
    private static native byte[] nativeEvalViewportUnitsFixture(long host);
    private static native byte[] nativeEvalMinMaxSizingFixture(long host);
    private static native byte[] nativeEvalBorderWidthFixture(long host);
    private static native byte[] nativeEvalAspectRatioFixture(long host);
    private static native byte[] nativeEvalBlockPaintFixture(long host);
    private static native byte[] nativeEvalRootHtmlBlockPaintFixture(long host);
    private static native byte[] nativeEvalRootHtmlTextPaintFixture(long host);
    private static native byte[] nativeEvalBlockFormattingFixture(long host);
    private static native byte[] nativeEvalC121PositionFixture(long host);
    private static native byte[] nativeEvalC122AbsoluteBlockFixture(long host);
    private static native byte[] nativeEvalC123FixedFixture(long host);
    private static native byte[] nativeEvalC124StackingFixture(long host);
    private static native byte[] nativeEvalC131OverflowClipFixture(long host);
    private static native byte[] nativeEvalC131CompactSurfaceMatrixFixture(long host);
    private static native byte[] nativeEvalC131OverflowClipBorderFixture(long host);
    private static native byte[] nativeEvalC131StackingDescendantClipFixture(long host);
    private static native byte[] nativeEvalC132ScrollFixture(long host);
    private static native byte[] nativeEvalC133GestureFixture(long host);
    private static native byte[] nativeApplyC133StyleInvalidation(long host);
    private static native byte[] nativeEnqueueC133Gesture(
            long host, int action, long surfaceGeneration, long gestureId, long pointerId,
            float xOrCumulativeXCssPx, float yOrCumulativeYCssPx,
            float coordinateScale, int reason);
    private static native byte[] nativeEnqueueC133Wheel(
            long host, int intent, long surfaceGeneration,
            float pointXCssPx, float pointYCssPx,
            float deltaXCssPx, float deltaYCssPx,
            float rawDeltaX, float rawDeltaY,
            int sourceUnit, float coordinateScale, int source,
            int momentumPhase, int modifierFlags);
    private static native byte[] nativeProcessC133GestureQueue(long host);

    private static Object invokeFixtureNative(String adapterClass, String methodName,
            Class<?>[] parameterTypes, Object... arguments) {
        try {
            return Class.forName(adapterClass).getMethod(methodName, parameterTypes)
                    .invoke(null, arguments);
        } catch (ReflectiveOperationException exception) {
            throw new IllegalStateException("fixture native adapter invocation failed", exception);
        }
    }
    private static native byte[] nativeEvalC11GridFixture(long host);
    private static native byte[] nativeEvalC11GridSceneFixture(long host, int sceneIndex);
    private static native byte[] nativeEvalC1121GridPlacementFixture(
            long host, String sceneId, String runId);
    private static native byte[] nativeEvalC1122GridAutoPlacementFixture(
            long host, String sceneId, String runId);
    private static native byte[] nativeEvalC1123GridAutoPlacementFixture(
            long host, String sceneId, String runId);
    private static native byte[] nativeEvalC1131GridRepeatFixture(
            long host, String sceneId, String runId);
    private static native byte[] nativeEvalC1132GridNamedLinesFixture(
            long host, String sceneId, String runId);
    private static native byte[] nativeEvalC1133GridTemplateAreasFixture(
            long host, String sceneId, String runId);
    private static native byte[] nativeEvalC114GridTrackSizingFixture(
            long host, String sceneId, String runId);
    private static native byte[] nativeEvalC124Source(long host, String source);
    private static native byte[] nativeEvalC1035PositionedFlexFixture(long host);
    private static native byte[] nativeEvalC121PositionState(long host, int state);
    private static native byte[] nativeEvalMarginCollapseFixture(long host);
    private static native byte[] nativeEvalFlowRootFixture(long host);
    private static native byte[] nativeEvalFlexWrapFixture(long host);
    private static native byte[] nativeEvalFlexDistributionFixture(long host);
    private static native byte[] nativeEvalFlexReverseFixture(long host);
    private static native byte[] nativeEvalFlexOrderFixture(long host);
    private static native byte[] nativeEvalFlexAlignmentFixture(long host);
    private static native byte[] nativeEvalFlexBaselineFixture(long host);
    private static native long nativeCreateSurface(
            long host, Surface surface, int width, int height, int backend);
    private static native int nativeResizeSurface(long renderer, int width, int height);
    private static native byte[] nativeDrawSurface(long renderer);
    private static native byte[] nativeRequestC192SurfacePixels(long renderer, int[] sampleXY);
    private static native byte[] nativeReadbackC192Offscreen(long renderer);
    private static native byte[] nativeInjectNextDrawFailure(long renderer);
    private static native void nativeDestroySurface(long renderer);
    private static native void nativeFreeHost(long host);

    private static void updateC124LifecycleStatus(Activity activity, TextView statusView,
            String status) {
        activity.runOnUiThread(() -> statusView.setText(status));
    }

    static void runC124RepeatedSessionLifecycleProbe(Activity activity, String runId,
            TextView statusView) {
        updateC124LifecycleStatus(activity, statusView, "C12.4 session lifecycle · starting");
        Thread probe = new Thread(() -> {
            int completedCycles = 0;
            int totalCreated = 0;
            boolean failed = false;
            Log.i(TAG, "SPINON_C124_LIFECYCLE status=started run_id=" + runId
                    + " cycles=16 sessions_per_cycle=8");
            updateC124LifecycleStatus(activity, statusView,
                    "C12.4 session lifecycle · cycle 0/16");
            SystemClock.sleep(3_000);
            for (int cycle = 1; cycle <= 16; cycle++) {
                List<Long> hosts = new ArrayList<>(8);
                int released = 0;
                Log.i(TAG, "SPINON_C124_LIFECYCLE status=cycle_begin run_id=" + runId
                        + " cycle=" + cycle);
                updateC124LifecycleStatus(activity, statusView,
                        "C12.4 session lifecycle · cycle " + cycle + "/16 running");
                try {
                    while (hosts.size() < 8) {
                        long host = nativeCreateC124StackingHost();
                        if (host == 0) {
                            Log.e(TAG, "SPINON_C124_LIFECYCLE status=create_failed run_id="
                                    + runId + " cycle=" + cycle + " created=" + hosts.size());
                            failed = true;
                            break;
                        }
                        hosts.add(host);
                        totalCreated++;
                    }
                    if (!failed) {
                        for (int index = 0; index < hosts.size(); index++) {
                            String report = new String(
                                    nativeC124WorkerStackResidencySample(hosts.get(index)),
                                    StandardCharsets.UTF_8);
                            Log.i(TAG, "SPINON_C124_LIFECYCLE_WORKER run_id=" + runId
                                    + " cycle=" + cycle + " session_index=" + index
                                    + " report=" + report);
                        }
                        Log.i(TAG, "SPINON_C124_LIFECYCLE status=ready run_id=" + runId
                                + " cycle=" + cycle + " session_count=" + hosts.size());
                        SystemClock.sleep(cycle == 1 || cycle == 8 || cycle == 16
                                ? 5_000 : 1_000);
                    }
                } catch (RuntimeException error) {
                    failed = true;
                    Log.e(TAG, "SPINON_C124_LIFECYCLE status=probe_failed run_id=" + runId
                            + " cycle=" + cycle + " message=" + error.getMessage(), error);
                } finally {
                    for (int index = hosts.size() - 1; index >= 0; index--) {
                        try {
                            nativeFreeHost(hosts.get(index));
                            released++;
                        } catch (RuntimeException error) {
                            failed = true;
                            Log.e(TAG, "SPINON_C124_LIFECYCLE status=free_failed run_id="
                                    + runId + " cycle=" + cycle + " session_index=" + index,
                                    error);
                        }
                    }
                }
                Log.i(TAG, "SPINON_C124_LIFECYCLE status=freed run_id=" + runId
                        + " cycle=" + cycle + " released=" + released
                        + " created=" + hosts.size());
                if (failed || released != hosts.size()) {
                    Log.e(TAG, "SPINON_C124_LIFECYCLE status=cycle_failed run_id=" + runId
                            + " cycle=" + cycle);
                    failed = true;
                    break;
                }
                SystemClock.sleep(1_500);
                completedCycles++;
                updateC124LifecycleStatus(activity, statusView,
                        "C12.4 session lifecycle · cycle " + completedCycles + "/16 complete");
                Log.i(TAG, "SPINON_C124_LIFECYCLE status=cycle_complete run_id=" + runId
                        + " cycle=" + cycle + " created=" + hosts.size()
                        + " released=" + released);
            }
            updateC124LifecycleStatus(activity, statusView,
                    "C12.4 session lifecycle · " + (failed ? "FAIL" : "PASS")
                            + " · " + completedCycles + "/16 cycles");
            Log.i(TAG, "SPINON_C124_LIFECYCLE status=finished run_id=" + runId
                    + " result=" + (failed ? "failed" : "passed")
                    + " cycles_completed=" + completedCycles
                    + " sessions_created=" + totalCreated);
        }, "spinon-c124-lifecycle-probe");
        probe.start();
    }

    static void runC124SessionStackCostProbe(Activity activity) {
        Thread probe = new Thread(() -> {
            List<Long> hosts = new ArrayList<>();
            boolean failed = false;
            for (int target : new int[] {0, 1, 2, 4, 8}) {
                while (hosts.size() < target) {
                    long host = nativeCreateC124StackingHost();
                    if (host == 0) {
                        Log.e(TAG, "SPINON_C124_SESSION_STACK status=create_failed target="
                                + target + " created=" + hosts.size());
                        failed = true;
                        break;
                    }
                    hosts.add(host);
                }
                if (failed) break;
                long reservedBytes = (long) hosts.size() * 8L * 1024L * 1024L;
                Log.i(TAG, "SPINON_C124_SESSION_STACK status=ready session_count="
                        + hosts.size() + " css_stack_reserved_bytes=" + reservedBytes
                        + " hold_seconds=8");
                if (BuildConfig.SPINON_C124_WORKER_STACK_RESIDENCY_TEST) {
                    for (int sample = 1; sample <= 3; sample++) {
                        for (int index = 0; index < hosts.size(); index++) {
                            String report = new String(
                                    nativeC124WorkerStackResidencySample(hosts.get(index)),
                                    StandardCharsets.UTF_8);
                            Log.i(TAG, "SPINON_C124_WORKER_STACK_RESIDENCY stage="
                                    + hosts.size() + " sample=" + sample
                                    + " session_index=" + index + " report=" + report);
                        }
                        SystemClock.sleep(100);
                    }
                }
                SystemClock.sleep(8_000);
            }
            for (int index = hosts.size() - 1; index >= 0; index--) {
                nativeFreeHost(hosts.get(index));
            }
            Log.i(TAG, "SPINON_C124_SESSION_STACK status=freed session_count=0 released_handles="
                    + hosts.size());
            SystemClock.sleep(10_000);
            Log.i(TAG, "SPINON_C124_SESSION_STACK status=finished failed=" + failed);
        }, "spinon-c124-stack-cost");
        probe.start();
    }


    private final Activity activity;
    private final Handler mainHandler = new Handler(Looper.getMainLooper());
    private final ExecutorService runtimeQueue = Executors.newSingleThreadExecutor(
            runnable -> new Thread(runnable, "spinon-c0410-runtime"));
    private final ExecutorService renderQueue = Executors.newSingleThreadExecutor(
            runnable -> new Thread(runnable, "spinon-c0410-render"));
    private final Object stateLock = new Object();
    private final Object presentationUpdateLock = new Object();
    private final SurfaceView surfaceView;
    private final TextView status;
    private final C0410LatestTaskLane runtimePresentationLane;
    private final C0410LatestTaskLane renderLane;
    private final int backend;
    private final boolean failureProbeRequested;
    private final boolean shutdownProbeRequested;
    private final boolean customPropertiesProbeRequested;
    private final boolean runtimeResultCacheProbeRequested;
    private final boolean incrementalRestyleProbeRequested;
    private final boolean percentageDimensionsProbeRequested;
    private final boolean spacingPercentagesProbeRequested;
    private final boolean absoluteLengthsProbeRequested;
    private final boolean fontRelativeUnitsProbeRequested;
    private final boolean typedCssMathProbeRequested;
    private final boolean viewportUnitsProbeRequested;
    private final boolean minMaxSizingProbeRequested;
    private final boolean borderWidthProbeRequested;
    private final boolean aspectRatioProbeRequested;
    private final boolean blockPaintProbeRequested;
    private final boolean rootHtmlBlockPaintProbeRequested;
    private final boolean rootHtmlTextPaintProbeRequested;
    private final boolean rootHtmlDefaultRenderRequested;
    private final String rootHtmlPageScaleCase;
    private final boolean rootHtmlApplicationRequested;
    private String rootHtmlApplicationSource;
    private String rootHtmlApplicationScript;
    private final boolean blockFormattingProbeRequested;
    private final boolean c121PositioningProbeRequested;
    private final boolean c122AbsoluteBlockProbeRequested;
    private final boolean c123FixedPositionProbeRequested;
    private final boolean c124StackingProbeRequested;
    private final boolean c11GridFixtureRequested;
    private final boolean c1121GridPlacementRequested;
    private final boolean c1122GridAutoPlacementRequested;
    private final boolean c1123GridAutoPlacementRequested;
    private final boolean c1131GridRepeatRequested;
    private final boolean c1132GridNamedLinesRequested;
    private final boolean c1133GridTemplateAreasRequested;
    private final String c1133GridTemplateAreasSceneId;
    private final boolean c114GridTrackSizingRequested;
    private final String c114GridTrackSizingSceneId;
    private final String c114GridTrackSizingRunId;
    private final String c1121GridSceneId;
    private final String c1121GridRunId;
    private final String c1122GridSceneId;
    private final String c1122GridRunId;
    private final String c1123GridSceneId;
    private final String c1123GridRunId;
    private final String c1131GridRepeatRunId;
    private final String c1132GridNamedLinesRunId;
    private final String c1133GridTemplateAreasRunId;
    private final int c11GridSceneIndex;
    private final String c11GridPixelMatrixRunId;
    private final String c11GridPixelMatrixSceneId;
    private final List<SurfaceViewPixelCapture.Sample> c11GridPixelMatrixSamples;
    private final String c11GridPixelMatrixConfigError;
    private final boolean c124RuntimeMatrixProbeRequested;
    private final boolean c131OverflowClipProbeRequested;
    private final boolean c131CompactSurfaceMatrixProbeRequested;
    private final String c131CompactSurfaceMatrixRunId;
    private final boolean c131StackingDescendantClipProbeRequested;
    private final boolean c131StackingDescendantClipStaleRevisionProbeRequested;
    private final String c131StackingDescendantClipRunId;
    private boolean c131StackingDescendantClipCaptureStarted;
    private final boolean c131BorderStripProbeRequested;
    private final boolean c192BoxPaintOpacityProbeRequested;
    private boolean rootHtmlBlockPaintSurfaceCaptureStarted;
    private boolean rootHtmlTextPaintSurfaceCaptureStarted;
    private final boolean c153GlyphPaintProbeRequested;
    private final boolean c153HostDocumentTextPaintProbeRequested;
    private volatile String c153GlyphPaintEvaluationReport = "";
    private boolean c192SurfaceCaptureStarted;
    private final boolean imageObjectFitPositionProbeRequested;
    private final boolean imageAsyncReplacementProbeRequested;
    private final boolean imageAlphaProbeRequested;
    private final boolean imagePaintProbeRequested;
    private boolean imagePaintSurfaceCaptureStarted;
    private final boolean c132ScrollFixtureRequested;
    private final String c132SurfacePixelRunId;
    private final boolean c133GestureFixtureRequested;
    private final boolean c133TapProbeRequested;
    private final boolean c133WheelProbeRequested;
    private final int c133GesturePolicyScenarioID;
    private final int c133WorkerDelayMillis;
    private final boolean c133SyntheticSecondPointerProbeRequested;
    private final boolean c124DepthSafetyProbeRequested;
    private final boolean c124SurfaceCaptureProbeRequested;
    private final String c124SurfaceCaseRequested;
    private final String c124SurfaceRunId;
    private final boolean c1035PositionedFlexProbeRequested;
    private final boolean marginCollapseProbeRequested;
    private final boolean flowRootProbeRequested;
    private final boolean flexWrapProbeRequested;
    private final boolean flexDistributionProbeRequested;
    private final boolean flexReverseProbeRequested;
    private final boolean flexOrderProbeRequested;
    private final boolean flexAlignmentProbeRequested;
    private final boolean flexBaselineProbeRequested;
    private final boolean fixedSizeCssFixtureRequested;
    private final boolean registeredPropertiesProbeRequested;
    private final boolean authorStylesheetsProbeRequested;
    private final float density;
    private volatile boolean darkMode;
    private volatile long hostHandle;
    private volatile long rendererHandle;
    private int c124FixtureNodeCount;
    private volatile int surfaceWidth;
    private volatile int surfaceHeight;
    private int surfaceFormat = -1;
    private volatile long surfaceGeneration;
    private long rendererGeneration = -1;
    private volatile boolean surfaceAvailable;
    private volatile boolean closing;
    private boolean runtimeInitializationQueued;
    private long hostPresentationSequence;
    private boolean expandedSurface;
    private boolean pendingDraw;
    private boolean failureProbeStarted;
    private boolean shutdownProbeStarted;
    private boolean c1121GridRuntimeSummaryEmitted;
    private boolean c124SurfaceCaptureStarted;
    private boolean c131OverflowClipSurfaceCaptureStarted;
    private boolean c131CompactSurfaceMatrixCaptureStarted;
    private boolean c131BorderStripSurfaceCaptureStarted;
    private boolean c153SurfaceCaptureStarted;
    private boolean c132SurfacePixelCaptureStarted;
    private volatile boolean c132ScrollAssertionsPassed;
    private volatile boolean c133GestureFixtureReady;
    private volatile boolean c133GestureInitialDrawReady;
    private volatile boolean c133WheelProbeStarted;
    private volatile boolean c133WheelProbeEnqueued;
    private volatile boolean c133WheelProbeProcessed;
    private volatile boolean c133WheelProbeCompleted;
    private volatile boolean c133WheelProbeFailed;
    private final AtomicBoolean c133DrainScheduled = new AtomicBoolean();
    private final AtomicBoolean c133DrainAgain = new AtomicBoolean();
    private final int c133TouchSlopPx;
    private long c133NextGestureId;
    private long c133GestureId;
    private long c133GestureSurfaceGeneration;
    private int c133ActivePointerId = MotionEvent.INVALID_POINTER_ID;
    private float c133DownXPhysicalPx;
    private float c133DownYPhysicalPx;
    private float c133CoordinateScale;
    private float c133LastCumulativeXCssPx;
    private float c133LastCumulativeYCssPx;
    private boolean c133TouchActive;
    private boolean c133PanStarted;
    private boolean c133TerminalQueued;
    private boolean c133SuppressUntilAllPointersUp;
    private volatile boolean c133StyleMutationApplied;
    private boolean c133StyleBarrierPending;
    private boolean c133StyleDeferredMovePending;
    private float c133StyleDeferredCumulativeX;
    private float c133StyleDeferredCumulativeY;
    private int c133StyleDeferredTerminalAction = -1;
    private int c133StyleDeferredTerminalReason;
    private boolean c133TapDownAccepted;
    private boolean c133TapForwardingGesture;
    private boolean c133TapClickCandidate;
    private int c133TapActivationCount;
    private boolean c133WorkerDelayUsed;
    private boolean c11GridSurfaceCaptureStarted;
    private List<C124SurfacePixelCapture.Sample> c124SurfaceSamples = List.of();
    private int c124SurfaceOriginXCssPx;
    private int c124SurfaceOriginYCssPx;
    private String c124SurfaceInventorySha256 = "";
    private volatile List<SurfaceViewPixelCapture.Sample> c131CompactSurfaceSamples = List.of();
    private volatile Map<String, String> c131CompactSurfaceExpectedRgba = Collections.emptyMap();
    private volatile List<SurfaceViewPixelCapture.Sample> c131StackingDescendantClipSamples = List.of();
    private volatile Map<String, String> c131StackingDescendantClipExpectedRgba = Collections.emptyMap();
    private volatile Map<String, String> c131StackingDescendantClipExpectedRevisions = Collections.emptyMap();
    private CountDownLatch surfaceDestroyBarrier;

    C0410RuntimeGpuDemo(
            Activity activity,
            int backend,
            boolean failureProbeRequested,
            boolean shutdownProbeRequested,
            int c11GridSceneIndex,
            boolean rootHtmlApplicationRequested
    ) {
        super(activity);
        this.activity = activity;
        this.backend = backend;
        this.failureProbeRequested = failureProbeRequested;
        this.shutdownProbeRequested = shutdownProbeRequested;
        this.c11GridSceneIndex = c11GridSceneIndex;
        runtimeResultCacheProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c053_runtime_result_cache", false);
        incrementalRestyleProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c054_incremental_restyle", false);
        percentageDimensionsProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c061_percentage_dimensions", false);
        spacingPercentagesProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c062_spacing_percentages", false);
        absoluteLengthsProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c063_absolute_lengths", false);
        fontRelativeUnitsProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c064_font_relative_units", false);
        typedCssMathProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c065_typed_css_math", false);
        viewportUnitsProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c066_viewport_units", false);
        minMaxSizingProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c071_min_max_sizing", false);
        borderWidthProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c072_border_width", false);
        aspectRatioProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c073_aspect_ratio", false);
        blockPaintProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c08_block_paint", false);
        rootHtmlBlockPaintProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c0412_root_html_block_paint", false);
        rootHtmlTextPaintProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c0412_root_html_text_paint", false);
        rootHtmlDefaultRenderRequested = activity.getIntent()
                .getBooleanExtra("spinon_c0412_root_html_default_render", false);
        rootHtmlPageScaleCase = activity.getIntent()
                .getStringExtra("spinon_c0412_root_html_page_scale_case");
        this.rootHtmlApplicationRequested = rootHtmlApplicationRequested;
        marginCollapseProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c092_margin_collapse", false);
        flowRootProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c093_flow_root", false);
        flexWrapProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c101_flex_wrap", false);
        flexDistributionProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c102_flex_distribution", false);
        flexReverseProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c1031_flex_reverse", false);
        flexOrderProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c1032_flex_order", false);
        flexAlignmentProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c1033_flex_alignment", false);
        flexBaselineProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c1034_flex_baseline", false);
        c121PositioningProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c121_static_relative", false);
        c122AbsoluteBlockProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c122_absolute_block", false);
        c123FixedPositionProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c123_fixed_position", false);
        c124RuntimeMatrixProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c124_stacking_matrix", false);
        c131OverflowClipProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c131_overflow_clip", false);
        c131CompactSurfaceMatrixProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c131_compact_surface_matrix", false);
        c131CompactSurfaceMatrixRunId = c131CompactSurfaceMatrixProbeRequested
                ? Long.toHexString(SystemClock.elapsedRealtimeNanos()) : "";
        c131StackingDescendantClipProbeRequested = BuildConfig.SPINON_C131_OVERFLOW_CLIP_FIXTURE_TEST
                && activity.getIntent().getBooleanExtra(
                        "spinon_c131_stacking_descendant_clip", false);
        c131StackingDescendantClipStaleRevisionProbeRequested =
                c131StackingDescendantClipProbeRequested
                && activity.getIntent().getBooleanExtra(
                        "spinon_c131_stacking_descendant_clip_stale_revision", false);
        c131StackingDescendantClipRunId = c131StackingDescendantClipProbeRequested
                ? Long.toHexString(SystemClock.elapsedRealtimeNanos()) : "";
        c131BorderStripProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c131_overflow_clip_border", false);
        c192BoxPaintOpacityProbeRequested = BuildConfig.SPINON_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST
                && activity.getIntent().getBooleanExtra("spinon_c192_box_paint_opacity", false);
        c153HostDocumentTextPaintProbeRequested =
                BuildConfig.SPINON_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST
                && activity.getIntent().getBooleanExtra(
                        "spinon_c153_hostdocument_text_paint", false);
        c153GlyphPaintProbeRequested = BuildConfig.SPINON_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST
                && (activity.getIntent().getBooleanExtra("spinon_c153_glyph_paint", false)
                        || c153HostDocumentTextPaintProbeRequested);
        imageObjectFitPositionProbeRequested =
                BuildConfig.SPINON_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST
                && activity.getIntent().getBooleanExtra("spinon_c183_object_fit_position", false);
        imageAsyncReplacementProbeRequested =
                BuildConfig.SPINON_C18_4_ASYNC_IMAGE_FIXTURE_TEST
                && activity.getIntent().getBooleanExtra("spinon_c184_async_image", false);
        imageAlphaProbeRequested = BuildConfig.SPINON_C18_2_ALPHA_IMAGE_FIXTURE_TEST
                && activity.getIntent().getBooleanExtra("spinon_c182_alpha_image", false);
        imagePaintProbeRequested = imageAsyncReplacementProbeRequested
                || imageObjectFitPositionProbeRequested || imageAlphaProbeRequested
                || BuildConfig.SPINON_C18_1_IMAGE_PAINT_FIXTURE_TEST
                && activity.getIntent().getBooleanExtra("spinon_c181_image_paint", false);
        c132ScrollFixtureRequested = activity.getIntent()
                .getBooleanExtra("spinon_c132_scroll_fixture", false);
        String requestedC132RunId = activity.getIntent()
                .getStringExtra("spinon_c132_surface_pixel_run_id");
        c132SurfacePixelRunId = c132ScrollFixtureRequested
                ? requestedC132RunId == null || requestedC132RunId.isBlank()
                    ? Long.toHexString(SystemClock.elapsedRealtimeNanos())
                    : requestedC132RunId
                : "";
        c133GestureFixtureRequested = activity.getIntent()
                .getBooleanExtra("spinon_c133_gesture_fixture", false);
        c133TapProbeRequested = BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST
                && activity.getIntent().getBooleanExtra("spinon_c133_tap_probe", false);
        c133WheelProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c133_wheel_probe", false);
        int requestedC133PolicyScenarioID = 0;
        if (BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST
                && c133GestureFixtureRequested
                && activity.getIntent().hasExtra("spinon_c133_policy_scenario_id")) {
            Bundle extras = activity.getIntent().getExtras();
            Object scenarioID = extras == null ? null
                    : extras.get("spinon_c133_policy_scenario_id");
            requestedC133PolicyScenarioID = scenarioID instanceof Integer
                    ? (Integer) scenarioID : -1;
        }
        c133GesturePolicyScenarioID = requestedC133PolicyScenarioID;
        c133WorkerDelayMillis = activity.getIntent()
                .getIntExtra("spinon_c133_worker_delay_ms", 0);
        c133SyntheticSecondPointerProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c133_synthetic_second_pointer", false);
        c133TouchSlopPx = ViewConfiguration.get(activity).getScaledTouchSlop();
        c124DepthSafetyProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c124_depth_safety", false);
        c124SurfaceCaseRequested = activity.getIntent()
                .getStringExtra("spinon_c124_surface_case");
        c124SurfaceRunId = activity.getIntent()
                .getStringExtra("spinon_c124_surface_run_id");
        c124SurfaceCaptureProbeRequested = c124SurfaceCaseRequested != null;
        c124StackingProbeRequested = c124SurfaceCaptureProbeRequested
                || c124RuntimeMatrixProbeRequested
                || c124DepthSafetyProbeRequested || activity.getIntent()
                .getBooleanExtra("spinon_c124_stacking_context", false);
        String c1121SceneId = activity.getIntent().getStringExtra("spinon_c1121_grid_scene");
        c1121GridPlacementRequested = activity.getIntent()
                .getBooleanExtra("spinon_c1121_grid_placement", false) || c1121SceneId != null;
        c1121GridSceneId = c1121SceneId == null ? "numeric-lines" : c1121SceneId;
        String c1121RunId = activity.getIntent().getStringExtra("spinon_c1121_grid_run_id");
        c1121GridRunId = c1121RunId == null ? "manual" : c1121RunId;
        String c1122SceneId = activity.getIntent()
                .getStringExtra("spinon_c1122_grid_auto_placement_scene");
        c1122GridAutoPlacementRequested = activity.getIntent()
                .getBooleanExtra("spinon_c1122_grid_auto_placement", false)
                || c1122SceneId != null;
        c1122GridSceneId = c1122SceneId == null ? "" : c1122SceneId;
        String c1122RunId = activity.getIntent()
                .getStringExtra("spinon_c1122_grid_auto_placement_run_id");
        c1122GridRunId = c1122RunId == null ? "manual" : c1122RunId;
        if (c1122GridAutoPlacementRequested
                && !isC1122GridAutoPlacementScene(c1122GridSceneId)) {
            throw new IllegalArgumentException("알 수 없는 C11.2.2 Grid scene입니다");
        }
        String c1123SceneId = activity.getIntent()
                .getStringExtra("spinon_c1123_grid_auto_placement_scene");
        c1123GridAutoPlacementRequested = activity.getIntent()
                .getBooleanExtra("spinon_c1123_grid_auto_placement", false)
                || c1123SceneId != null;
        c1123GridSceneId = c1123SceneId == null ? "" : c1123SceneId;
        String c1123RunId = activity.getIntent()
                .getStringExtra("spinon_c1123_grid_auto_placement_run_id");
        c1123GridRunId = c1123RunId == null ? "manual" : c1123RunId;
        if (c1123GridAutoPlacementRequested
                && (!isC1123GridAutoPlacementScene(c1123GridSceneId)
                        || !c1123GridRunId.matches("[A-Za-z0-9_-]{1,80}")
                        || c1122GridAutoPlacementRequested || c1121GridPlacementRequested)) {
            throw new IllegalArgumentException("C11.2.3 Grid scene 또는 profile 선택이 올바르지 않습니다");
        }
        String c1131SceneId = activity.getIntent()
                .getStringExtra("spinon_c1131_grid_repeat_scene");
        c1131GridRepeatRequested = activity.getIntent()
                .getBooleanExtra("spinon_c1131_grid_repeat", false) || c1131SceneId != null;
        String c1131RunId = activity.getIntent()
                .getStringExtra("spinon_c1131_grid_repeat_run_id");
        c1131GridRepeatRunId = c1131RunId == null ? "manual" : c1131RunId;
        if (c1131GridRepeatRequested
                && (!C1131_GRID_REPEAT_SCENE_ID.equals(
                        c1131SceneId == null ? C1131_GRID_REPEAT_SCENE_ID : c1131SceneId)
                        || !c1131GridRepeatRunId.matches("[A-Za-z0-9_-]{1,80}")
                        || c1123GridAutoPlacementRequested || c1122GridAutoPlacementRequested
                        || c1121GridPlacementRequested)) {
            throw new IllegalArgumentException("C11.3.1 Grid scene 또는 profile 선택이 올바르지 않습니다");
        }
        c1132GridNamedLinesRequested = activity.getIntent()
                .getBooleanExtra("spinon_c1132_grid_named_lines", false);
        String c1132RunId = activity.getIntent()
                .getStringExtra("spinon_c1132_grid_named_lines_run_id");
        c1132GridNamedLinesRunId = c1132RunId == null ? "manual" : c1132RunId;
        if (c1132GridNamedLinesRequested
                && (!c1132GridNamedLinesRunId.matches("[A-Za-z0-9_-]{1,80}")
                        || c1131GridRepeatRequested || c1123GridAutoPlacementRequested
                        || c1122GridAutoPlacementRequested || c1121GridPlacementRequested)) {
            throw new IllegalArgumentException("C11.3.2 Grid fixture 또는 profile 선택이 올바르지 않습니다");
        }
        c1133GridTemplateAreasRequested = activity.getIntent()
                .getBooleanExtra("spinon_c1133_grid_template_areas", false);
        String c1133SceneId = activity.getIntent().getStringExtra("spinon_c11_grid_case");
        c1133GridTemplateAreasSceneId = c1133SceneId == null
                ? C1133_GRID_TEMPLATE_AREAS_DEFAULT_SCENE_ID : c1133SceneId;
        String c1133RunId = activity.getIntent()
                .getStringExtra("spinon_c1133_grid_template_areas_run_id");
        c1133GridTemplateAreasRunId = c1133RunId == null ? "manual" : c1133RunId;
        if (c1133GridTemplateAreasRequested
                && (!c1133GridTemplateAreasRunId.matches("[A-Za-z0-9_-]{1,80}")
                        || !isC1133GridTemplateAreasScene(c1133GridTemplateAreasSceneId)
                        || c1131GridRepeatRequested || c1132GridNamedLinesRequested
                        || c1123GridAutoPlacementRequested || c1122GridAutoPlacementRequested
                        || c1121GridPlacementRequested)) {
            throw new IllegalArgumentException("C11.3.3 Grid fixture 또는 profile 선택이 올바르지 않습니다");
        }
        c114GridTrackSizingRequested = activity.getIntent()
                .getBooleanExtra("spinon_c114_grid_track_sizing", false);
        String c114SceneId = activity.getIntent().getStringExtra("spinon_c114_grid_scene");
        c114GridTrackSizingSceneId = c114SceneId == null
                ? C114_GRID_TRACK_SIZING_SCENE_IDS[0] : c114SceneId;
        String c114RunId = activity.getIntent().getStringExtra("spinon_c114_grid_run_id");
        c114GridTrackSizingRunId = c114RunId == null ? "manual" : c114RunId;
        if (c114GridTrackSizingRequested
                && (!c114GridTrackSizingRunId.matches("[A-Za-z0-9_-]{1,80}")
                        || !java.util.Arrays.asList(C114_GRID_TRACK_SIZING_SCENE_IDS)
                                .contains(c114GridTrackSizingSceneId)
                        || c1131GridRepeatRequested || c1132GridNamedLinesRequested
                        || c1133GridTemplateAreasRequested || c1123GridAutoPlacementRequested
                        || c1122GridAutoPlacementRequested || c1121GridPlacementRequested
                        || activity.getIntent().hasExtra("spinon_c11_grid_case"))) {
            throw new IllegalArgumentException("C11.4 Grid fixture 또는 profile 선택이 올바르지 않습니다");
        }
        c11GridFixtureRequested = c1121GridPlacementRequested || c1122GridAutoPlacementRequested
                || c1123GridAutoPlacementRequested || c1131GridRepeatRequested
                || c1132GridNamedLinesRequested || c1133GridTemplateAreasRequested
                || c114GridTrackSizingRequested
                || activity.getIntent()
                .getBooleanExtra("spinon_c11_grid_fixture", false)
                || activity.getIntent().getStringExtra("spinon_c11_grid_case") != null;
        String pixelMatrixRunId = activity.getIntent()
                .getStringExtra("spinon_c11_grid_pixel_matrix_run_id");
        String pixelMatrixSceneId = c114GridTrackSizingRequested
                ? c114GridTrackSizingSceneId
                : activity.getIntent().getStringExtra("spinon_c11_grid_case");
        String pixelMatrixEncodedSamples = activity.getIntent()
                .getStringExtra("spinon_c11_grid_pixel_matrix_samples");
        boolean pixelMatrixRequested = pixelMatrixRunId != null || pixelMatrixEncodedSamples != null;
        boolean pixelMatrixSceneMatches = c114GridTrackSizingRequested
                ? ("auto-fill-overflow-neighbors".equals(pixelMatrixSceneId)
                        || "columns-fill-negative-positive-cycle".equals(pixelMatrixSceneId)
                        || "implicit-min-content-overflow-border".equals(pixelMatrixSceneId)
                        || "columns-negative-fixed-root-overflow".equals(pixelMatrixSceneId)
                        || "rows-negative-fixed-root-overflow".equals(pixelMatrixSceneId)
                        || "min-content-text-break-all".equals(pixelMatrixSceneId)
                        || "fixed-small-track-overflow".equals(pixelMatrixSceneId)
                        || "column-auto-natural-ratio".equals(pixelMatrixSceneId)
                        || "row-transfer-input-capped-by-fixed-column-span".equals(pixelMatrixSceneId)
                        || "column-transfer-input-capped-by-fixed-row-span".equals(pixelMatrixSceneId)
                        || "fixed-gutter-column-suggestion-clamp".equals(pixelMatrixSceneId)
                        || "fixed-gutter-row-suggestion-clamp".equals(pixelMatrixSceneId)
                        || c114IsImageMultiSpanScene(pixelMatrixSceneId)
                        || c114IsAnonymousGridBorderOverflowScene(pixelMatrixSceneId)
                        || "grid-gap-definite-percent".equals(pixelMatrixSceneId)
                        || "grid-gap-cyclic-percent".equals(pixelMatrixSceneId)
                        || "grid-gap-definite-calc".equals(pixelMatrixSceneId)
                        || "grid-gap-cyclic-calc".equals(pixelMatrixSceneId)
                        || "grid-gap-cyclic-percent-min-basis".equals(pixelMatrixSceneId)
                        || "grid-gap-cyclic-calc-min-basis".equals(pixelMatrixSceneId)
                        || "nested-grid-definite-flex-control".equals(pixelMatrixSceneId))
                        && c114GridTrackSizingRunId.equals(pixelMatrixRunId)
                : c1133GridTemplateAreasRequested
                ? (C1133_GRID_TEMPLATE_AREAS_DEFAULT_SCENE_ID.equals(pixelMatrixSceneId)
                        || "unknown-area-fixed-implicit-tracks".equals(pixelMatrixSceneId))
                : c1131GridRepeatRequested
                ? pixelMatrixSceneId != null
                        && C1131_GRID_REPEAT_SCENE_ID.equals(pixelMatrixSceneId)
                : c1132GridNamedLinesRequested
                ? C1132_GRID_NAMED_LINE_SCENE_ID.equals(pixelMatrixSceneId)
                : c1123GridAutoPlacementRequested
                ? pixelMatrixSceneId != null && pixelMatrixSceneId.equals(c1123GridSceneId)
                        && isC1123GridPixelScene(pixelMatrixSceneId)
                : c1121GridPlacementRequested
                ? pixelMatrixSceneId != null && pixelMatrixSceneId.equals(c1121GridSceneId)
                        && isC1121PixelScene(pixelMatrixSceneId)
                : c1122GridAutoPlacementRequested
                ? pixelMatrixSceneId != null && pixelMatrixSceneId.equals(c1122GridSceneId)
                        && isC1122GridPixelScene(pixelMatrixSceneId)
                : pixelMatrixSceneId != null && c11GridSceneIndex >= 0
                        && c11GridSceneIndex < C11_GRID_SCENE_IDS.length
                        && pixelMatrixSceneId.equals(C11_GRID_SCENE_IDS[c11GridSceneIndex]);
        List<SurfaceViewPixelCapture.Sample> pixelMatrixSamples = List.of();
        String pixelMatrixConfigError = "";
        if (pixelMatrixRequested) {
            if (pixelMatrixRunId == null || !pixelMatrixRunId.matches("[A-Za-z0-9_-]{8,80}")) {
                pixelMatrixConfigError = "invalid-run-id";
            } else if (!pixelMatrixSceneMatches) {
                pixelMatrixConfigError = "scene-index-mismatch";
            } else if (pixelMatrixEncodedSamples == null
                    || pixelMatrixEncodedSamples.length() > 16_384) {
                pixelMatrixConfigError = "invalid-sample-payload";
            } else {
                try {
                    byte[] decoded = android.util.Base64.decode(pixelMatrixEncodedSamples,
                            android.util.Base64.URL_SAFE | android.util.Base64.NO_WRAP);
                    pixelMatrixSamples = parseC11GridPixelMatrixSamples(
                            new String(decoded, StandardCharsets.UTF_8));
                    if (pixelMatrixSamples.isEmpty()) pixelMatrixConfigError = "empty-samples";
                    else if (c1131GridRepeatRequested
                            && !isC1131GridRepeatPixelSamples(pixelMatrixSamples)) {
                        pixelMatrixSamples = List.of();
                        pixelMatrixConfigError = "invalid-c1131-sample-fixture";
                    } else if (c1132GridNamedLinesRequested
                            && !isC1132GridNamedLinePixelSamples(pixelMatrixSamples)) {
                        pixelMatrixSamples = List.of();
                        pixelMatrixConfigError = "invalid-c1132-sample-fixture";
                    } else if (c1133GridTemplateAreasRequested
                            && !isC1133GridTemplateAreasPixelSamples(
                                    pixelMatrixSceneId, pixelMatrixSamples)) {
                        pixelMatrixSamples = List.of();
                        pixelMatrixConfigError = "invalid-c1133-sample-fixture";
                    } else if (c114GridTrackSizingRequested
                            && !isC114GridTrackSizingPixelSamples(
                                    c114GridTrackSizingSceneId, pixelMatrixSamples)) {
                        pixelMatrixSamples = List.of();
                        pixelMatrixConfigError = "invalid-c114-sample-fixture";
                    }
                } catch (Exception error) {
                    pixelMatrixConfigError = "invalid-sample-payload";
                }
            }
        }
        c11GridPixelMatrixRunId = pixelMatrixRunId == null ? "" : pixelMatrixRunId;
        c11GridPixelMatrixSceneId = pixelMatrixSceneId == null ? "" : pixelMatrixSceneId;
        c11GridPixelMatrixSamples = pixelMatrixSamples;
        c11GridPixelMatrixConfigError = pixelMatrixConfigError;
        if (!c11GridPixelMatrixConfigError.isEmpty()) {
            String marker = c114GridTrackSizingRequested ? "SPINON_C114_GRID_PIXEL_MATRIX"
                    : c1133GridTemplateAreasRequested ? "SPINON_C1133_GRID_PIXEL_MATRIX"
                    : c1131GridRepeatRequested ? "SPINON_C1131_GRID_PIXEL_MATRIX"
                    : c1132GridNamedLinesRequested ? "SPINON_C1132_GRID_PIXEL_MATRIX"
                    : c1123GridAutoPlacementRequested ? "SPINON_C1123_GRID_PIXEL_MATRIX"
                    : c1122GridAutoPlacementRequested
                            ? "SPINON_C1122_GRID_PIXEL_MATRIX"
                            : "SPINON_C111_GRID_PIXEL_MATRIX";
            Log.e(TAG, marker + " status=error run_id=" + c11GridPixelMatrixRunId
                    + " scene=" + c11GridPixelMatrixSceneId + " reason="
                    + c11GridPixelMatrixConfigError);
        }
        c1035PositionedFlexProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c1035_positioned_flex", false);
        blockFormattingProbeRequested = flowRootProbeRequested || marginCollapseProbeRequested
                || activity.getIntent().getBooleanExtra("spinon_c091_block_formatting", false);
        fixedSizeCssFixtureRequested = c122AbsoluteBlockProbeRequested
                || rootHtmlBlockPaintProbeRequested
                || rootHtmlTextPaintProbeRequested
                || rootHtmlDefaultRenderRequested
                || BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST && imagePaintProbeRequested
                || c131CompactSurfaceMatrixProbeRequested
                || c131BorderStripProbeRequested
                || c192BoxPaintOpacityProbeRequested
                || c153GlyphPaintProbeRequested
                || c132ScrollFixtureRequested
                || c133GestureFixtureRequested
                || c123FixedPositionProbeRequested
                || c11GridFixtureRequested
                || c124StackingProbeRequested
                || blockFormattingProbeRequested || flexWrapProbeRequested
                || flexDistributionProbeRequested || flexReverseProbeRequested
                || flexOrderProbeRequested || flexAlignmentProbeRequested
                || flexBaselineProbeRequested || c1035PositionedFlexProbeRequested;
        if (!c1131GridRepeatRequested && !c1132GridNamedLinesRequested
                && !c1133GridTemplateAreasRequested && !c114GridTrackSizingRequested
                && (c11GridSceneIndex < 0 || c11GridSceneIndex >= 10)) {
            throw new IllegalArgumentException("C11.1 scene index가 허용 범위 밖입니다");
        }
        registeredPropertiesProbeRequested = runtimeResultCacheProbeRequested
                || activity.getIntent().getBooleanExtra("spinon_c052_registered_properties", false);
        authorStylesheetsProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c0411_author_stylesheets", false)
                && !registeredPropertiesProbeRequested;
        customPropertiesProbeRequested = !authorStylesheetsProbeRequested
                && !registeredPropertiesProbeRequested && activity.getIntent()
                .getBooleanExtra("spinon_c051_custom_properties", false);
        density = activity.getResources().getDisplayMetrics().density;
        darkMode = (activity.getResources().getConfiguration().uiMode
                & android.content.res.Configuration.UI_MODE_NIGHT_MASK)
                == android.content.res.Configuration.UI_MODE_NIGHT_YES;
        setOrientation(VERTICAL);
        int horizontalPadding = rootHtmlApplicationRequested ? 0 : dp(22, density);
        int topPadding = rootHtmlApplicationRequested ? 0 : dp(44, density);
        int bottomPadding = rootHtmlApplicationRequested ? 0 : dp(24, density);
        setPadding(horizontalPadding, topPadding, horizontalPadding, bottomPadding);
        setOnApplyWindowInsetsListener((view, insets) -> {
            view.setPadding(
                    horizontalPadding + insets.getSystemWindowInsetLeft(),
                    topPadding + insets.getSystemWindowInsetTop(),
                    horizontalPadding + insets.getSystemWindowInsetRight(),
                    bottomPadding + insets.getSystemWindowInsetBottom());
            return insets;
        });
        setBackgroundColor(Color.rgb(14, 19, 31));

        TextView title = new TextView(activity);
        title.setText(c153GlyphPaintProbeRequested
                ? c153HostDocumentTextPaintProbeRequested
                        ? "SPINON · C15.3 HostDocument text paint"
                        : "SPINON · C15.3 WGPU glyph paint"
                : c192BoxPaintOpacityProbeRequested
                ? "SPINON · C19.2 box paint·opacity"
                : c114GridTrackSizingRequested
                ? "SPINON · C11.4 Grid track sizing"
                : c1133GridTemplateAreasRequested
                ? "SPINON · C11.3.3 Grid template areas"
                : c1132GridNamedLinesRequested
                ? "SPINON · C11.3.2 named Grid lines"
                : c1131GridRepeatRequested
                ? "SPINON · C11.3.1 Grid repeat()"
                : c1123GridAutoPlacementRequested
                ? "SPINON · C11.2.3 Grid"
                : c1122GridAutoPlacementRequested
                ? "SPINON · C11.2.2 Grid"
                : c1121GridPlacementRequested
                ? "SPINON · C11.2.1 Grid"
                : c132ScrollFixtureRequested
                ? "SPINON · C13.2 scroll state"
                : BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST && imagePaintProbeRequested
                ? imageAsyncReplacementProbeRequested
                        ? "SPINON · C18.4 async PNG 교체·실패 복구"
                        : imageObjectFitPositionProbeRequested
                        ? "SPINON · C18.3 object-fit/object-position"
                        : imageAlphaProbeRequested
                        ? "SPINON · C18.2 alpha image composite"
                        : "SPINON · C18.1 image overflow paint"
                : c133WheelProbeRequested
                ? "SPINON · C13.3 wheel fixture"
                : c133GestureFixtureRequested
                ? "SPINON · C13.3 Android native input"
                : c131StackingDescendantClipProbeRequested
                ? "SPINON · C13.1 nested stacking clip"
                : c131BorderStripProbeRequested
                ? "SPINON · C13.1 paint/hit border"
                : c131OverflowClipProbeRequested
                ? "SPINON · C13.1 overflow clip"
                : c11GridFixtureRequested
                ? "SPINON · C11.1 CSS Grid"
                : c131CompactSurfaceMatrixProbeRequested
                ? "SPINON · C13.1 compact surface"
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
                : rootHtmlDefaultRenderRequested
                        ? "SPINON · C04.12 root HTML → WGPU"
                : authorStylesheetsProbeRequested
                        ? "SPINON · C04.11 CSS → WGPU" : "SPINON · C04.10 CSS → WGPU");
        title.setTextColor(Color.rgb(235, 241, 250));
        title.setTextSize(22);
        title.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        if (rootHtmlApplicationRequested) title.setVisibility(GONE);
        addView(title, new LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.WRAP_CONTENT));

        TextView description = new TextView(activity);
        description.setText(c153GlyphPaintProbeRequested
                ? c153HostDocumentTextPaintProbeRequested
                        ? "HostDocument Text(XX) → Taffy text snapshot → Ahem glyph mask · 320×240 CSS px · DPR 1"
                        : "고정 Noto Sans KR glyph coverage mask · WGPU · 360×240 CSS px · DPR 1"
                : c192BoxPaintOpacityProbeRequested
                ? "alpha·중첩 opacity·clip·z-order·currentColor·OKLCH · 16 PixelCopy 표본 · 360×240 CSS px"
                : c131CompactSurfaceMatrixProbeRequested
                ? "13개 overflow clip case · 30,000 hit identity + 52 PixelCopy 표본 · WGPU · 300×100 CSS px"
                : c131StackingDescendantClipProbeRequested
                ? "visible/clip/hidden stacking descendant · 18 Chrome hit/color 표본 · WGPU · 540×128 CSS px"
                : c132ScrollFixtureRequested
                ? "V8 scroll owner·offset·paint/hit → WGPU · 5 PixelCopy 표본 · 400×240 CSS px"
                : BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST && imagePaintProbeRequested
                ? imageAsyncReplacementProbeRequested
                        ? "비동기 PNG 요청 정체성 · pending/failure resource 제거 · D source 복구 · 6 PixelCopy 표본 · 224×88 CSS px"
                        : imageObjectFitPositionProbeRequested
                        ? "PNG 24×12 · 12 object-fit/position 장면 · 19 PixelCopy 표본 · 224×72 CSS px"
                        : imageAlphaProbeRequested
                        ? "RGBA PNG · source-over alpha 표본 10개 · 80×40 CSS px"
                        : "고정 PNG · visible/hidden 부모 경계 · 9 PixelCopy 표본 · 80×40 CSS px"
                : c133WheelProbeRequested
                ? "Synthetic CSS px delta → Rust queue → scroll router → WGPU · 400×240 CSS px"
                : c133GestureFixtureRequested
                ? "MotionEvent touch/wheel · bounded Rust queue/session → WGPU · 400×240 CSS px"
                : c131BorderStripProbeRequested
                ? "static/absolute clip border-strip oracle → WGPU · 400×160 CSS px"
                : c131OverflowClipProbeRequested
                ? "overflow clip·clip margin·positioned escape → WGPU"
                : c114GridTrackSizingRequested
                ? "V8 intrinsic Grid track sizing → Stylo → Taffy → WGPU · 320×240 CSS px"
                : c1133GridTemplateAreasRequested
                ? "V8 Grid template area placement → Stylo → Taffy → WGPU · 320×240 CSS px"
                : c1132GridNamedLinesRequested
                ? "V8 named Grid line occurrence → Stylo → Taffy → WGPU · 320×240 CSS px"
                : c1131GridRepeatRequested
                ? "V8 정수 Grid repeat() → Stylo → Taffy → WGPU · 320×240 CSS px"
                : c1123GridAutoPlacementRequested
                ? "C11.2.3 dense·column Grid 자동 배치 → Stylo → Taffy → WGPU · 360×800 CSS px"
                : c1122GridAutoPlacementRequested
                ? "C11.2.2 행 우선 Grid 자동 배치 → Stylo → Taffy → WGPU · 360×800 CSS px"
                : c1121GridPlacementRequested
                ? "V8 numeric Grid line/span → Stylo → Taffy → WGPU · 320×240 CSS px"
                : c11GridFixtureRequested
                ? "V8 DOM display:grid → Stylo → Taffy → Rust 장면 → WGPU · 320×240 CSS px"
                : c1035PositionedFlexProbeRequested
                ? "V8 직접 Flex 자식·wrapper·paint 순서 → Stylo → Taffy → WGPU · 320×240 CSS px"
                : c123FixedPositionProbeRequested
                ? "V8 fixed viewport owner·inset·resize → Stylo → Taffy → WGPU · 360×800 CSS px"
                : c124StackingProbeRequested
                ? "V8 z-index·visibility·Flex order → Stylo → Taffy → WGPU · 320×240 CSS px"
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
                : "실제 V8 DOM → Stylo → Taffy → Rust 장면 → wgpu surface");
        description.setTextColor(Color.rgb(200, 211, 228));
        description.setTextSize(14);
        if (rootHtmlApplicationRequested) description.setVisibility(GONE);
        LayoutParams descriptionParams =
                new LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.WRAP_CONTENT);
        descriptionParams.topMargin = dp(8, density);
        addView(description, descriptionParams);

        LinearLayout stage = new LinearLayout(activity);
        stage.setGravity(Gravity.CENTER);
        LayoutParams stageParams = new LayoutParams(LayoutParams.MATCH_PARENT, 0, 1.0f);
        addView(stage, stageParams);

        surfaceView = new SurfaceView(activity);
        surfaceView.getHolder().addCallback(this);
        if (BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST && c133GestureFixtureRequested) {
            surfaceView.setClickable(true);
            surfaceView.setOnTouchListener((view, event) -> handleC133GestureTouch(event));
            if (c133TapProbeRequested) {
                surfaceView.setOnClickListener(view -> onC133TapProbeClick());
            }
        }
        surfaceView.setContentDescription(c153GlyphPaintProbeRequested
                ? c153HostDocumentTextPaintProbeRequested
                        ? "C15.3 HostDocument text layout Ahem overflow glyph를 WGPU surface에 그리는 fixture"
                        : "C15.3 고정 Noto Sans KR glyph alpha mask를 WGPU surface에 그리는 fixture"
                : c192BoxPaintOpacityProbeRequested
                ? "C19.2 box paint, opacity, clipping, z-order, and wide-gamut WGPU surface"
                : c131CompactSurfaceMatrixProbeRequested
                ? "C13.1 compact 13-case overflow Chrome oracle WGPU surface"
                : c131StackingDescendantClipProbeRequested
                ? "C13.1 nested stacking descendant clip Chrome oracle WGPU surface"
                : c131BorderStripProbeRequested
                ? "C13.1 paint/hit border-strip Chrome oracle WGPU 장면"
                : c132ScrollFixtureRequested
                ? "C13.2 scroll owner·offset·paint/hit WGPU 장면"
                : BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST && imagePaintProbeRequested
                ? imageAsyncReplacementProbeRequested
                        ? "C18.4 비동기 PNG 교체·decode 실패·복구 WGPU 장면"
                        : imageAlphaProbeRequested
                        ? "C18.2 PNG alpha source-over WGPU 장면"
                        : "C18.1 이미지와 overflow 부모 경계 WGPU 장면"
                : c133GestureFixtureRequested
                ? "C13.3 실제 Android MotionEvent nested scroll gesture fixture"
                : c1123GridAutoPlacementRequested
                ? "C11.2.3 dense·column Grid 자동 배치 결과"
                : c1122GridAutoPlacementRequested
                ? "C11.2.2 행 우선 Grid 자동 배치 결과"
                : c1121GridPlacementRequested
                ? "C11.2.1 numeric Grid line 배치 WGPU 장면"
                : c11GridFixtureRequested
                ? "C11.1 고정 CSS Grid 트랙·gap 배치 WGPU 장면"
                : c1035PositionedFlexProbeRequested
                ? "C10.3.5 positioned Flex 정렬·wrapper·paint 순서 WGPU 장면"
                : c123FixedPositionProbeRequested
                ? "C12.3 viewport fixed 배치 WGPU 장면"
                : c124StackingProbeRequested
                ? "C12.4 z-index 쌓임 맥락 WGPU 장면"
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
                : blockFormattingProbeRequested
                ? "C09.1 일반 Block 흐름·auto margin WGPU 장면"
                : viewportUnitsProbeRequested
                ? "C06.6 viewport units WGPU 장면"
                : rootHtmlTextPaintProbeRequested
                ? "C04.12 parsed root HTML direct text · Ahem glyphs · owner overflow clip WGPU scene"
                : rootHtmlDefaultRenderRequested
                ? "C04.12 일반 root HTML 기본 렌더 WGPU 장면"
                : rootHtmlBlockPaintProbeRequested
                ? "C04.12 실제 html/head/body Block 배경 페인트 WGPU 장면"
                : blockPaintProbeRequested
                ? "C08 Block 흐름 기본 페인트 WGPU 장면"
                : aspectRatioProbeRequested
                ? "C07.3 종횡비 WGPU 장면"
                : borderWidthProbeRequested
                ? "C07.2 border width layout WGPU 장면"
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
                : rootHtmlDefaultRenderRequested
                        ? "C04.12 일반 root HTML 기본 렌더 WGPU 장면"
                        : "C04.10 Chromium fixture의 WGPU 장면");
        int initialSurfaceWidth = BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST
                && imagePaintProbeRequested ? imageAsyncReplacementProbeRequested
                        || imageObjectFitPositionProbeRequested ? 224 : 80
                : c131StackingDescendantClipProbeRequested ? 540
                : c131CompactSurfaceMatrixProbeRequested ? 300
                : c131BorderStripProbeRequested ? 400
                : c153HostDocumentTextPaintProbeRequested ? 320
                : c153GlyphPaintProbeRequested || c192BoxPaintOpacityProbeRequested ? 360
                : c132ScrollFixtureRequested ? 400
                : c133GestureFixtureRequested ? 400
                : fixedSizeCssFixtureRequested ? 320 : 301;
        int initialSurfaceHeight = BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST
                && imagePaintProbeRequested ? imageAsyncReplacementProbeRequested ? 88
                        : imageObjectFitPositionProbeRequested ? 72 : 40
                : c131StackingDescendantClipProbeRequested ? 128
                : c131CompactSurfaceMatrixProbeRequested ? 100
                : c131BorderStripProbeRequested ? 160
                : c153GlyphPaintProbeRequested || c192BoxPaintOpacityProbeRequested ? 240
                : fixedSizeCssFixtureRequested ? 240 : 100;
        LayoutParams surfaceParams = rootHtmlApplicationRequested
                ? new LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.MATCH_PARENT)
                : new LayoutParams(dp(initialSurfaceWidth, density), dp(initialSurfaceHeight, density));
        if (BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST
                && c133GestureFixtureRequested
                && activity.getIntent().getBooleanExtra(
                        "spinon_c133_touchpad_origin_target", false)) {
            try {
                Class<?> fixture = Class.forName(
                        "dev.spinon.bootstrap.C133TouchpadOriginTargetFixture");
                fixture.getDeclaredMethod(
                        "install", ViewGroup.class, ViewGroup.class,
                        SurfaceView.class, ViewGroup.LayoutParams.class)
                        .invoke(null, this, stage, surfaceView, surfaceParams);
            } catch (ReflectiveOperationException exception) {
                throw new IllegalStateException(
                        "C13.3 touchpad fixture helper is unavailable", exception);
            }
        } else {
            stage.addView(surfaceView, surfaceParams);
        }

        Button resizeButton = new Button(activity);
        resizeButton.setText("표면 크기 전환 · 301×100 CSS px");
        resizeButton.setOnClickListener(view -> toggleSurfaceSize(resizeButton));
        if (fixedSizeCssFixtureRequested || rootHtmlApplicationRequested) {
            resizeButton.setVisibility(GONE);
        }
        LayoutParams resizeButtonParams =
                new LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.WRAP_CONTENT);
        resizeButtonParams.topMargin = dp(8, density);
        addView(resizeButton, resizeButtonParams);

        Button customPropertiesButton = new Button(activity);
        customPropertiesButton.setText(viewportUnitsProbeRequested
                ? "C06.6 viewport unit fixture 다시 실행"
                : aspectRatioProbeRequested
                ? "C07.3 aspect-ratio fixture 실행 완료"
                : typedCssMathProbeRequested
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
                : "C05 · 사용자 지정 속성 다시 적용");
        customPropertiesButton.setOnClickListener(view -> {
            if (viewportUnitsProbeRequested) evaluateViewportUnitsFixture();
            else if (typedCssMathProbeRequested) evaluateTypedCssMathFixture();
            else if (fontRelativeUnitsProbeRequested) evaluateFontRelativeUnitsFixture();
            else if (absoluteLengthsProbeRequested) evaluateAbsoluteLengthsFixture();
            else if (spacingPercentagesProbeRequested) evaluateSpacingPercentagesFixture();
            else if (percentageDimensionsProbeRequested) evaluatePercentageDimensionsFixture();
            else if (incrementalRestyleProbeRequested) evaluateIncrementalRestyleFixture();
            else if (runtimeResultCacheProbeRequested) evaluateRuntimeResultCacheFixture();
            else if (registeredPropertiesProbeRequested) evaluateRegisteredPropertiesFixture();
            else evaluateCustomPropertiesFixture();
        });
        if (c122AbsoluteBlockProbeRequested || c121PositioningProbeRequested || authorStylesheetsProbeRequested || viewportUnitsProbeRequested || fixedSizeCssFixtureRequested || blockPaintProbeRequested || aspectRatioProbeRequested
                || minMaxSizingProbeRequested
                || borderWidthProbeRequested
                || typedCssMathProbeRequested
                || fontRelativeUnitsProbeRequested
                || absoluteLengthsProbeRequested
                || percentageDimensionsProbeRequested
                || spacingPercentagesProbeRequested) {
            customPropertiesButton.setVisibility(GONE);
        }
        if (rootHtmlApplicationRequested) customPropertiesButton.setVisibility(GONE);
        LayoutParams customPropertiesButtonParams =
                new LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.WRAP_CONTENT);
        customPropertiesButtonParams.topMargin = dp(4, density);
        addView(customPropertiesButton, customPropertiesButtonParams);

        status = new TextView(activity);
        status.setText("V8·CSS runtime 준비 중…");
        status.setTextColor(Color.rgb(97, 185, 255));
        status.setTextSize(13);
        status.setTypeface(Typeface.MONOSPACE);
        status.setMinLines(c133GestureFixtureRequested ? 2 : 1);
        if (rootHtmlApplicationRequested) status.setVisibility(GONE);
        addView(status, new LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.WRAP_CONTENT));

        runtimePresentationLane = new C0410LatestTaskLane(
                runtimeQueue,
                this::applyLatestEnvironment,
                error -> reportLaneFailure("runtime", error));
        renderLane = new C0410LatestTaskLane(
                renderQueue,
                this::reconcileRenderState,
                error -> reportLaneFailure("render", error));
        if (!rootHtmlApplicationRequested
                && !c121PositioningProbeRequested && !c122AbsoluteBlockProbeRequested
                && !c123FixedPositionProbeRequested && !c124StackingProbeRequested
                && !c1133GridTemplateAreasRequested && !c132ScrollFixtureRequested
                && !c133GestureFixtureRequested) {
            enqueueRuntime(this::initializeRuntime);
        }
    }

    private static int dp(int value, float density) {
        return Math.round(value * density);
    }

    private static void awaitTerminationUninterruptibly(ExecutorService executor) {
        boolean interrupted = false;
        while (!executor.isTerminated()) {
            try {
                executor.awaitTermination(Long.MAX_VALUE, TimeUnit.NANOSECONDS);
            } catch (InterruptedException error) {
                interrupted = true;
            }
        }
        if (interrupted) Thread.currentThread().interrupt();
    }

    private static void awaitUninterruptibly(CountDownLatch latch) {
        boolean interrupted = false;
        while (true) {
            try {
                latch.await();
                break;
            } catch (InterruptedException error) {
                interrupted = true;
            }
        }
        if (interrupted) Thread.currentThread().interrupt();
    }

    private boolean validateC131FixtureRoute() {
        if (activity.getIntent().getBooleanExtra(
                    "spinon_c131_stacking_descendant_clip", false)
                && !BuildConfig.SPINON_C131_OVERFLOW_CLIP_FIXTURE_TEST) {
            Log.e(TAG, "SPINON_C131_STACKING_CLIP_ARGUMENT_ERROR rejected=true reason=fixture-feature-disabled");
            postStatus("실패 · C13.1 stacking descendant clip fixture-test feature가 꺼져 있습니다");
            return false;
        }
        if (!BuildConfig.SPINON_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST
                && activity.getIntent().getBooleanExtra("spinon_c192_box_paint_opacity", false)) {
            Log.e(TAG, "SPINON_C192_ARGUMENT_ERROR rejected=true reason=fixture-feature-disabled");
            postStatus("실패 · C19.2 fixture-test feature가 꺼져 있습니다");
            return false;
        }
        if (!BuildConfig.SPINON_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST
                && activity.getIntent().getBooleanExtra("spinon_c153_glyph_paint", false)) {
            Log.e(TAG, "SPINON_C153_ARGUMENT_ERROR rejected=true reason=fixture-feature-disabled");
            postStatus("실패 · C15.3 glyph paint fixture-test 옵션이 꺼져 있습니다");
            return false;
        }
        if (BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST
                && activity.getIntent().getBooleanExtra("spinon_c133_tap_probe", false)
                && !c133GestureFixtureRequested) {
            Log.e(TAG, "SPINON_C133_ARGUMENT_ERROR rejected=true reason=tap-probe-requires-fixture-route");
            postStatus("실패 · C13.3 tap probe에는 gesture fixture route가 필요합니다");
            return false;
        }
        if (c133WheelProbeRequested && !c133GestureFixtureRequested) {
            Log.e(TAG, "SPINON_C133_ARGUMENT_ERROR rejected=true reason=wheel-probe-requires-fixture-route");
            postStatus("실패 · C13.3 wheel probe에는 gesture fixture route가 필요합니다");
            return false;
        }
        if (c133TapProbeRequested
                && (!activity.getIntent().hasExtra("spinon_c133_policy_scenario_id")
                        || (c133GesturePolicyScenarioID != 3
                                && c133GesturePolicyScenarioID != 4))) {
            Log.e(TAG, "SPINON_C133_ARGUMENT_ERROR rejected=true reason=tap-probe-requires-scenario-3-or-4");
            postStatus("실패 · C13.3 tap probe는 policy scenario 3 또는 4가 필요합니다");
            return false;
        }
        if (c133WheelProbeRequested
                && (!activity.getIntent().hasExtra("spinon_c133_policy_scenario_id")
                        || (c133GesturePolicyScenarioID != 1
                                && c133GesturePolicyScenarioID != 2
                                && c133GesturePolicyScenarioID != 3))) {
            Log.e(TAG, "SPINON_C133_ARGUMENT_ERROR rejected=true reason=wheel-probe-requires-scenario-1-2-or-3");
            postStatus("실패 · C13.3 wheel probe는 policy scenario 1, 2 또는 3이 필요합니다");
            return false;
        }
        if (BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST
                && !c133GestureFixtureRequested
                && activity.getIntent().hasExtra("spinon_c133_policy_scenario_id")) {
            Log.e(TAG, "SPINON_C133_ARGUMENT_ERROR rejected=true reason=scenario-requires-fixture-route");
            postStatus("실패 · C13.3 policy scenario ID에는 gesture fixture route가 필요합니다");
            return false;
        }
        int selectedRoutes = (c131OverflowClipProbeRequested ? 1 : 0)
                + (c131CompactSurfaceMatrixProbeRequested ? 1 : 0)
                + (c131StackingDescendantClipProbeRequested ? 1 : 0)
                + (c131BorderStripProbeRequested ? 1 : 0)
                + (c192BoxPaintOpacityProbeRequested ? 1 : 0)
                + (c153GlyphPaintProbeRequested ? 1 : 0)
                + (c132ScrollFixtureRequested ? 1 : 0)
                + (c133GestureFixtureRequested ? 1 : 0)
                + (BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST && imagePaintProbeRequested ? 1 : 0);
        if (selectedRoutes > 1) {
            Log.e(TAG, "SPINON_C13_ARGUMENT_ERROR conflicting=multiple-fixture-routes");
            postStatus("실패 · C13 route를 하나만 선택해야 합니다");
            return false;
        }
        if (activity.getIntent().hasExtra("spinon_c131_stacking_descendant_clip_stale_revision")
                && !c131StackingDescendantClipProbeRequested) {
            Log.e(TAG, "SPINON_C131_ARGUMENT_ERROR rejected=stale-revision-requires-stacking-route");
            postStatus("실패 · stale revision control에는 C13.1 stacking clip route가 필요합니다");
            return false;
        }
        if (selectedRoutes == 0) return true;

        String selectedExtra = c133GestureFixtureRequested
                ? "spinon_c133_gesture_fixture"
                : c153HostDocumentTextPaintProbeRequested
                ? "spinon_c153_hostdocument_text_paint"
                : c153GlyphPaintProbeRequested
                ? "spinon_c153_glyph_paint"
                : c192BoxPaintOpacityProbeRequested
                ? "spinon_c192_box_paint_opacity"
                : BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST && imagePaintProbeRequested
                ? imageAsyncReplacementProbeRequested ? "spinon_c184_async_image"
                        : imageObjectFitPositionProbeRequested ? "spinon_c183_object_fit_position"
                        : imageAlphaProbeRequested ? "spinon_c182_alpha_image" : "spinon_c181_image_paint"
                : c132ScrollFixtureRequested
                ? "spinon_c132_scroll_fixture"
                : c131StackingDescendantClipProbeRequested
                ? "spinon_c131_stacking_descendant_clip"
                : c131CompactSurfaceMatrixProbeRequested
                ? "spinon_c131_compact_surface_matrix"
                : c131BorderStripProbeRequested
                ? "spinon_c131_overflow_clip_border" : "spinon_c131_overflow_clip";
        if (c132ScrollFixtureRequested
                && !c132SurfacePixelRunId.matches("[A-Za-z0-9_-]{8,80}")) {
            Log.e(TAG, "SPINON_C132_ARGUMENT_ERROR rejected=true reason=invalid-run-id");
            postStatus("실패 · C13.2 surface pixel run ID가 올바르지 않습니다");
            return false;
        }
        if (c133WorkerDelayMillis < 0 || c133WorkerDelayMillis > 3000) {
            Log.e(TAG, "SPINON_C133_ARGUMENT_ERROR rejected=true reason=invalid-worker-delay");
            postStatus("실패 · C13.3 worker 지연 값은 0~3000ms여야 합니다");
            return false;
        }
        if (BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST
                && c133GestureFixtureRequested
                && (c133GesturePolicyScenarioID < 0 || c133GesturePolicyScenarioID > 10)) {
            Log.e(TAG, "SPINON_C133_ARGUMENT_ERROR rejected=true reason=invalid-policy-scenario-id");
            postStatus("실패 · C13.3 policy scenario ID는 0~10이어야 합니다");
            return false;
        }
        Bundle extras = activity.getIntent().getExtras();
        if (extras == null) return true;
        for (String key : extras.keySet()) {
            if (!key.startsWith("spinon_c") || key.equals(selectedExtra)
                    || key.equals("spinon_c0410_runtime_gpu")
                    || (c132ScrollFixtureRequested
                            && key.equals("spinon_c132_surface_pixel_run_id"))
                    || (c131StackingDescendantClipProbeRequested
                            && key.equals("spinon_c131_stacking_descendant_clip_stale_revision"))
                    || (c133GestureFixtureRequested
                            && (key.equals("spinon_c133_worker_delay_ms")
                                    || key.equals("spinon_c133_synthetic_second_pointer")
                                    || key.equals("spinon_c133_policy_scenario_id")
                                    || key.equals("spinon_c133_tap_probe")
                                    || key.equals("spinon_c133_wheel_probe")
                                    || (BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST
                                            && key.equals(
                                                    "spinon_c133_touchpad_origin_target"))))) {
                continue;
            }
            Object value = extras.get(key);
            if (Boolean.FALSE.equals(value)) continue;
            Log.e(TAG, "SPINON_C13_ARGUMENT_ERROR conflicting=" + key);
            postStatus("실패 · C13 전용 route와 다른 CSS fixture를 함께 선택했습니다");
            return false;
        }
        return true;
    }

    private void initializeRuntime() {
        if (rootHtmlPageScaleCase != null
                && (!rootHtmlDefaultRenderRequested
                        || !(rootHtmlPageScaleCase.equals("half")
                                || rootHtmlPageScaleCase.equals("double")
                                || rootHtmlPageScaleCase.equals("fixed-width")))) {
            Log.e(TAG, "SPINON_C0412_ROOT_HTML_PAGE_SCALE_ARGUMENT_ERROR rejected=true case="
                    + rootHtmlPageScaleCase + " root_html_default="
                    + rootHtmlDefaultRenderRequested);
            postStatus("실패 · page scale case는 root HTML 기본 렌더 route에서만 사용할 수 있습니다");
            return;
        }
        if (rootHtmlDefaultRenderRequested) {
            android.os.Bundle extras = activity.getIntent().getExtras();
            if (extras != null) {
                for (String key : extras.keySet()) {
                    if (!key.startsWith("spinon_c")
                            || key.equals("spinon_c0412_root_html_default_render")
                            || key.equals("spinon_c0412_root_html_page_scale_case")
                            || key.equals("spinon_c0410_runtime_gpu")) continue;
                    Object value = extras.get(key);
                    if (Boolean.FALSE.equals(value)) continue;
                    Log.e(TAG, "SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_ARGUMENT_ERROR conflicting="
                            + key);
                    postStatus("실패 · root HTML 기본 렌더와 다른 CSS route를 함께 선택했습니다");
                    return;
                }
            }
        }
        if (!validateC131FixtureRoute()) {
            return;
        }
        if (!c11GridPixelMatrixConfigError.isEmpty()) {
            Log.e(TAG, (c114GridTrackSizingRequested ? "SPINON_C114_GRID_PIXEL_MATRIX"
                    : c1133GridTemplateAreasRequested ? "SPINON_C1133_GRID_PIXEL_MATRIX"
                    : c1132GridNamedLinesRequested ? "SPINON_C1132_GRID_PIXEL_MATRIX"
                    : c1131GridRepeatRequested ? "SPINON_C1131_GRID_PIXEL_MATRIX"
                    : c1123GridAutoPlacementRequested ? "SPINON_C1123_GRID_PIXEL_MATRIX"
                    : c1122GridAutoPlacementRequested
                            ? "SPINON_C1122_GRID_PIXEL_MATRIX"
                            : "SPINON_C111_GRID_PIXEL_MATRIX")
                    + " status=error reason=" + c11GridPixelMatrixConfigError);
            postStatus("실패 · C11 Grid 표면 표본 인수가 올바르지 않습니다");
            return;
        }
        if (rootHtmlBlockPaintProbeRequested
                && !BuildConfig.SPINON_C04_12_ROOT_HTML_BLOCK_PAINT_FIXTURE_TEST) {
            postStatus("C04.12 root HTML Block paint fixture-test 옵션을 켜서 빌드해야 합니다.");
            return;
        }
        if (rootHtmlTextPaintProbeRequested
                && !BuildConfig.SPINON_C04_12_ROOT_HTML_TEXT_PAINT_FIXTURE_TEST) {
            postStatus("C04.12 root HTML text paint fixture-test 옵션을 켜서 빌드해야 합니다.");
            return;
        }
        long host = c114GridTrackSizingRequested
                ? nativeCreateC114GridTrackSizingHost()
                : c1133GridTemplateAreasRequested
                ? nativeCreateC1133GridTemplateAreasHost()
                : c1132GridNamedLinesRequested
                ? nativeCreateC1132GridNamedLinesHost()
                : c1131GridRepeatRequested
                ? nativeCreateC1131GridRepeatHost()
                : c1123GridAutoPlacementRequested
                ? nativeCreateC1123GridAutoPlacementHost()
                : c1122GridAutoPlacementRequested
                ? nativeCreateC1122GridAutoPlacementHost()
                : c1121GridPlacementRequested
                ? nativeCreateC1121GridPlacementHost()
                : c11GridFixtureRequested
                ? nativeCreateC11GridHost()
                : c153HostDocumentTextPaintProbeRequested
                ? (Long) invokeFixtureNative(
                        "dev.spinon.bootstrap.GlyphPaintFixtureNativeAdapter",
                        "createHostDocumentTextPaintHost", new Class<?>[0])
                : BuildConfig.SPINON_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST
                        && c153GlyphPaintProbeRequested
                ? (Long) invokeFixtureNative(
                        "dev.spinon.bootstrap.GlyphPaintFixtureNativeAdapter",
                        "createHost", new Class<?>[0])
                : BuildConfig.SPINON_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST
                        && c192BoxPaintOpacityProbeRequested
                ? (Long) invokeFixtureNative(
                        "dev.spinon.bootstrap.BoxPaintOpacityFixtureNativeAdapter",
                        "createHost", new Class<?>[0])
                : c133GestureFixtureRequested
                ? nativeCreateC133GestureHost(c133GesturePolicyScenarioID)
                : BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST && imagePaintProbeRequested
                ? (Long) invokeFixtureNative(
                        imageAsyncReplacementProbeRequested
                                ? "dev.spinon.bootstrap.AsyncImageFixtureNativeAdapter"
                                : imageObjectFitPositionProbeRequested
                                ? "dev.spinon.bootstrap.ObjectFitPositionFixtureNativeAdapter"
                                : imageAlphaProbeRequested
                                ? "dev.spinon.bootstrap.AlphaImageFixtureNativeAdapter"
                                : "dev.spinon.bootstrap.ImagePaintFixtureNativeAdapter",
                        "createHost",
                        new Class<?>[0])
                : c132ScrollFixtureRequested
                ? nativeCreateC132ScrollHost()
                : c131StackingDescendantClipProbeRequested
                ? nativeCreateC131StackingDescendantClipHost()
                : c131CompactSurfaceMatrixProbeRequested
                ? nativeCreateC131CompactSurfaceMatrixHost()
                : c131BorderStripProbeRequested
                ? nativeCreateC131OverflowClipBorderHost()
                : c131OverflowClipProbeRequested
                ? nativeCreateC131OverflowClipHost()
                : c124StackingProbeRequested
                ? nativeCreateC124StackingHost()
                : c123FixedPositionProbeRequested
                ? nativeCreateC123FixedHost()
                : c1035PositionedFlexProbeRequested
                ? nativeCreateC1035PositionedFlexHost()
                : c122AbsoluteBlockProbeRequested
                ? nativeCreateC122AbsoluteBlockHost()
                : c121PositioningProbeRequested
                ? nativeCreateC121PositionHost()
                : blockFormattingProbeRequested
                ? nativeCreateBlockFormattingHost()
                : rootHtmlTextPaintProbeRequested
                ? nativeCreateRootHtmlTextPaintHost()
                : rootHtmlBlockPaintProbeRequested
                ? nativeCreateRootHtmlBlockPaintHost()
                : blockPaintProbeRequested
                ? nativeCreateBlockPaintHost()
                : rootHtmlApplicationRequested
                ? nativeCreateRootHtmlApplicationHost()
                : rootHtmlDefaultRenderRequested
                ? nativeCreateRootHtmlDefaultRenderHost()
                : nativeCreateHost(registeredPropertiesProbeRequested);
        if (host == 0) {
            postStatus("실패 · V8 runtime host를 만들지 못했습니다");
            return;
        }
        synchronized (stateLock) {
            hostHandle = host;
        }
        final float widthCssPx;
        final float heightCssPx;
        final boolean dark;
        synchronized (stateLock) {
            widthCssPx = BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST
                    && imagePaintProbeRequested ? imageAsyncReplacementProbeRequested
                            || imageObjectFitPositionProbeRequested ? 224 : 80
                    : c153HostDocumentTextPaintProbeRequested ? 320
                    : c153GlyphPaintProbeRequested || c192BoxPaintOpacityProbeRequested ? 360
                    : c131StackingDescendantClipProbeRequested ? 540
                    : c131CompactSurfaceMatrixProbeRequested ? 300
                    : c131BorderStripProbeRequested ? 400
                    : c132ScrollFixtureRequested || c133GestureFixtureRequested ? 400
                    : c114GridTrackSizingRequested || c1133GridTemplateAreasRequested
                    || c1132GridNamedLinesRequested || c1131GridRepeatRequested ? 320
                    : c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
                    || c1122GridAutoPlacementRequested || c1123GridAutoPlacementRequested
                    ? 360 : fixedSizeCssFixtureRequested ? 320 : cssViewportWidthLocked();
            heightCssPx = BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST
                    && imagePaintProbeRequested ? imageAsyncReplacementProbeRequested ? 88
                            : imageObjectFitPositionProbeRequested ? 72 : 40
                    : c153GlyphPaintProbeRequested || c192BoxPaintOpacityProbeRequested ? 240
                    : c131StackingDescendantClipProbeRequested ? 128
                    : c131CompactSurfaceMatrixProbeRequested ? 100
                    : c131BorderStripProbeRequested ? 160
                    : c132ScrollFixtureRequested || c133GestureFixtureRequested ? 240
                    : c1133GridTemplateAreasRequested
                    || c1132GridNamedLinesRequested || c1131GridRepeatRequested ? 240
                    : c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
                    || c1122GridAutoPlacementRequested || c1123GridAutoPlacementRequested
                    ? 800 : fixedSizeCssFixtureRequested ? 240 : cssViewportHeightLocked();
            dark = darkMode;
        }
        float environmentScale = c153GlyphPaintProbeRequested || c192BoxPaintOpacityProbeRequested
                || rootHtmlTextPaintProbeRequested
                || c123FixedPositionProbeRequested || c1131GridRepeatRequested
                || c1132GridNamedLinesRequested || c1133GridTemplateAreasRequested
                || c114GridTrackSizingRequested
                ? 1.0f : density;
        String environment = decode(nativeSetEnvironment(
                host,
                (rootHtmlApplicationRequested || rootHtmlBlockPaintProbeRequested
                        || rootHtmlTextPaintProbeRequested
                        || rootHtmlDefaultRenderRequested)
                        ? widthCssPx * environmentScale : widthCssPx,
                (rootHtmlApplicationRequested || rootHtmlBlockPaintProbeRequested
                        || rootHtmlTextPaintProbeRequested
                        || rootHtmlDefaultRenderRequested)
                        ? heightCssPx * environmentScale : heightCssPx,
                environmentScale, dark));
        if (!environment.startsWith("status=0 ")) {
            postStatus("실패 · " + environment);
            return;
        }
        Log.i(TAG, "SPINON_C0410_ENVIRONMENT viewport=" + widthCssPx + "x" + heightCssPx
                + " scale=" + environmentScale + " dark=" + dark + " " + environment);
        String result;
        if (c1133GridTemplateAreasRequested
                && "revision-reuse".equals(c1133GridTemplateAreasSceneId)) {
            String firstRevisionResult = decode(nativeEvalC1133GridTemplateAreasFixture(
                    host, c1133GridTemplateAreasSceneId, c1133GridTemplateAreasRunId));
            String secondRevisionResult = decode(nativeEvalC1133GridTemplateAreasFixture(
                    host, c1133GridTemplateAreasSceneId, c1133GridTemplateAreasRunId));
            boolean revisionReusePassed = c1133GridRevisionReusePassed(
                    host, c1133GridTemplateAreasRunId, firstRevisionResult, secondRevisionResult);
            result = revisionReusePassed ? secondRevisionResult
                    : !firstRevisionResult.startsWith("status=0 ") ? firstRevisionResult
                    : !secondRevisionResult.startsWith("status=0 ") ? secondRevisionResult
                    : "status=-11 C11.3.3 same-host revision reuse assertion failed";
        } else {
            result = c114GridTrackSizingRequested
                ? decode(nativeEvalC114GridTrackSizingFixture(
                        host, c114GridTrackSizingSceneId, c114GridTrackSizingRunId))
                : c1133GridTemplateAreasRequested
                ? decode(nativeEvalC1133GridTemplateAreasFixture(
                        host, c1133GridTemplateAreasSceneId,
                        c1133GridTemplateAreasRunId))
                : c1132GridNamedLinesRequested
                ? decode(nativeEvalC1132GridNamedLinesFixture(
                        host, C1132_GRID_NAMED_LINE_SCENE_ID, c1132GridNamedLinesRunId))
                : c1131GridRepeatRequested
                ? decode(nativeEvalC1131GridRepeatFixture(
                        host, C1131_GRID_REPEAT_SCENE_ID, c1131GridRepeatRunId))
                : c1123GridAutoPlacementRequested
                ? decode(nativeEvalC1123GridAutoPlacementFixture(
                        host, c1123GridSceneId, c1123GridRunId))
                : c1122GridAutoPlacementRequested
                ? decode(nativeEvalC1122GridAutoPlacementFixture(
                        host, c1122GridSceneId, c1122GridRunId))
                : c1121GridPlacementRequested
                ? decode(nativeEvalC1121GridPlacementFixture(
                        host, c1121GridSceneId, c1121GridRunId))
                : c11GridFixtureRequested
                ? decode(nativeEvalC11GridSceneFixture(host, c11GridSceneIndex))
                : BuildConfig.SPINON_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST
                        && c153GlyphPaintProbeRequested
                ? decode((byte[]) invokeFixtureNative(
                        "dev.spinon.bootstrap.GlyphPaintFixtureNativeAdapter",
                        "evalFixture", new Class<?>[] {long.class}, host))
                : BuildConfig.SPINON_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST
                        && c192BoxPaintOpacityProbeRequested
                ? decode((byte[]) invokeFixtureNative(
                        "dev.spinon.bootstrap.BoxPaintOpacityFixtureNativeAdapter",
                        "evalFixture", new Class<?>[] {long.class}, host))
                : c133GestureFixtureRequested
                ? decode(nativeEvalC133GestureFixture(host))
                : BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST && imagePaintProbeRequested
                ? decode((byte[]) invokeFixtureNative(
                        imageAsyncReplacementProbeRequested
                                ? "dev.spinon.bootstrap.AsyncImageFixtureNativeAdapter"
                                : imageObjectFitPositionProbeRequested
                                ? "dev.spinon.bootstrap.ObjectFitPositionFixtureNativeAdapter"
                                : imageAlphaProbeRequested
                                ? "dev.spinon.bootstrap.AlphaImageFixtureNativeAdapter"
                                : "dev.spinon.bootstrap.ImagePaintFixtureNativeAdapter",
                        "evalFixture",
                        new Class<?>[] {long.class}, host))
                : c132ScrollFixtureRequested
                ? decode(nativeEvalC132ScrollFixture(host))
                : c131StackingDescendantClipProbeRequested
                ? decode(nativeEvalC131StackingDescendantClipFixture(host))
                : c131CompactSurfaceMatrixProbeRequested
                ? decode(nativeEvalC131CompactSurfaceMatrixFixture(host))
                : c131BorderStripProbeRequested
                ? decode(nativeEvalC131OverflowClipBorderFixture(host))
                : c131OverflowClipProbeRequested
                ? decode(nativeEvalC131OverflowClipFixture(host))
                : c124SurfaceCaptureProbeRequested
                ? evaluateC124SurfaceCase(host)
                : c124StackingProbeRequested
                ? decode(nativeEvalC124StackingFixture(host))
                : c123FixedPositionProbeRequested
                ? decode(nativeEvalC123FixedFixture(host))
                : c1035PositionedFlexProbeRequested
                ? decode(nativeEvalC1035PositionedFlexFixture(host))
                : c122AbsoluteBlockProbeRequested
                ? decode(nativeEvalC122AbsoluteBlockFixture(host))
                : c121PositioningProbeRequested
                ? decode(nativeEvalC121PositionFixture(host))
                : flexBaselineProbeRequested
                ? decode(nativeEvalFlexBaselineFixture(host))
                : flexAlignmentProbeRequested
                ? decode(nativeEvalFlexAlignmentFixture(host))
                : flexOrderProbeRequested
                ? decode(nativeEvalFlexOrderFixture(host))
                : flexReverseProbeRequested
                ? decode(nativeEvalFlexReverseFixture(host))
                : flexDistributionProbeRequested
                ? decode(nativeEvalFlexDistributionFixture(host))
                : flexWrapProbeRequested
                ? decode(nativeEvalFlexWrapFixture(host))
                : flowRootProbeRequested
                ? decode(nativeEvalFlowRootFixture(host))
                : viewportUnitsProbeRequested
                ? decode(nativeEvalViewportUnitsFixture(host))
                : marginCollapseProbeRequested
                ? decode(nativeEvalMarginCollapseFixture(host))
                : blockFormattingProbeRequested
                ? decode(nativeEvalBlockFormattingFixture(host))
                : rootHtmlApplicationRequested
                ? decode(nativeEvalRootHtmlApplicationScript(host, rootHtmlApplicationScript))
                : rootHtmlDefaultRenderRequested
                ? "status=0 root_html_default_render=source"
                : rootHtmlTextPaintProbeRequested
                ? decode(nativeEvalRootHtmlTextPaintFixture(host))
                : rootHtmlBlockPaintProbeRequested
                ? decode(nativeEvalRootHtmlBlockPaintFixture(host))
                : blockPaintProbeRequested
                ? decode(nativeEvalBlockPaintFixture(host))
                : aspectRatioProbeRequested
                ? decode(nativeEvalAspectRatioFixture(host))
                : borderWidthProbeRequested
                ? decode(nativeEvalBorderWidthFixture(host))
                : minMaxSizingProbeRequested
                ? decode(nativeEvalMinMaxSizingFixture(host))
                : typedCssMathProbeRequested
                ? decode(nativeEvalTypedCssMathFixture(host))
                : fontRelativeUnitsProbeRequested
                ? decode(nativeEvalFontRelativeUnitsFixture(host))
                : absoluteLengthsProbeRequested
                ? decode(nativeEvalAbsoluteLengthsFixture(host))
                : spacingPercentagesProbeRequested
                ? decode(nativeEvalSpacingPercentagesFixture(host))
                : percentageDimensionsProbeRequested
                ? decode(nativeEvalPercentageDimensionsFixture(host))
                : incrementalRestyleProbeRequested
                ? decode(nativeEvalIncrementalRestyleFixture(host))
                : registeredPropertiesProbeRequested
                ? decode(nativeEvalRegisteredPropertiesFixture(host))
                : authorStylesheetsProbeRequested
                        ? decode(nativeEvalAuthorStylesheetsFixture(host))
                        : decode(nativeEvalFixture(host));
        }
        if (c153HostDocumentTextPaintProbeRequested
                ? (!result.startsWith("status=0 ")
                        || !result.contains("c15_3_hostdocument_text_paint=attached")
                        || !result.contains("layout_source=fixture_owned_host_document")
                        || !result.contains("owner_frame_css_px=0,0,15,10")
                        || !result.contains("overflow_css_px=5")
                        || reportField(result, "hostdocument_text_samples_css_px=").isEmpty())
                : c153GlyphPaintProbeRequested
                && (!result.startsWith("status=0 ")
                        || !result.contains("c15_3_text=attached")
                        || !result.contains("glyphs=")
                        || !"3>6>0;18>7>0;4+5>2+3>0+0".equals(
                                reportField(result, "scroll_sequences="))
                        || !"horizontal_owner;vertical_owner;nested_owner".equals(
                                reportField(result, "scroll_hit_test=")))) {
            Log.e(TAG, "SPINON_C153_EVAL_ERROR rejected=true reason=text-command-not-published report="
                    + result);
            postStatus("실패 · C15.3 glyph paint command가 runtime snapshot에 제출되지 않았습니다");
            return;
        }
        if (c153GlyphPaintProbeRequested) c153GlyphPaintEvaluationReport = result;
        if (c1133GridTemplateAreasRequested
                && C1133_GRID_TEMPLATE_AREAS_DEFAULT_SCENE_ID.equals(c1133GridTemplateAreasSceneId)
                && result.startsWith("status=0 ")) {
            long rejectionHost = nativeCreateC1133GridTemplateAreasHost();
            String rejection;
            if (rejectionHost == 0) {
                rejection = "status=-2 isolated rejection host creation failed";
            } else {
                try {
                    String rejectionEnvironment = decode(nativeSetEnvironment(
                            rejectionHost, 320.0f, 240.0f, 1.0f, dark));
                    rejection = rejectionEnvironment.startsWith("status=0 ")
                            ? decode(nativeEvalC1133GridTemplateAreasFixture(
                                    rejectionHost, C1133_GRID_TEMPLATE_AREAS_REJECT_SCENE_ID,
                                    c1133GridTemplateAreasRunId))
                            : rejectionEnvironment;
                } finally {
                    nativeFreeHost(rejectionHost);
                }
            }
            boolean expectedRejection = rejection.startsWith("status=-11 ")
                    && rejection.contains("unsupported_computed_value")
                    && rejection.contains("property=Some(\"grid-template-areas\")")
                    && !rejection.contains("node_frames_css_px=[");
            Log.i(TAG, "SPINON_C1133_GRID_REJECTION run_id=" + c1133GridTemplateAreasRunId
                    + " scene_id=" + C1133_GRID_TEMPLATE_AREAS_REJECT_SCENE_ID
                    + " expected=" + expectedRejection + " " + rejection);
            if (!expectedRejection) {
                postStatus("실패 · C11.3.3 area track 수 불일치 거부 계약 오류");
                return;
            }
            Log.i(TAG, "SPINON_C1133_GRID_REJECTION_SUMMARY run_id="
                    + c1133GridTemplateAreasRunId + " scene_id="
                    + C1133_GRID_TEMPLATE_AREAS_REJECT_SCENE_ID
                    + " expected=true status=-11 code=unsupported_computed_value"
                    + " property=grid-template-areas frames=absent");
        }
        if (c1132GridNamedLinesRequested && result.startsWith("status=0 ")) {
            long rejectionHost = nativeCreateC1132GridNamedLinesHost();
            String rejection;
            if (rejectionHost == 0) {
                rejection = "status=-2 isolated rejection host creation failed";
            } else {
                try {
                    String rejectionEnvironment = decode(nativeSetEnvironment(
                            rejectionHost, 320.0f, 240.0f, 1.0f, dark));
                    rejection = rejectionEnvironment.startsWith("status=0 ")
                            ? decode(nativeEvalC1132GridNamedLinesFixture(
                                    rejectionHost, C1132_GRID_NAMED_LINE_REJECT_SCENE_ID,
                                    c1132GridNamedLinesRunId))
                            : rejectionEnvironment;
                } finally {
                    nativeFreeHost(rejectionHost);
                }
            }
            boolean expectedRejection = rejection.startsWith("status=-11 ")
                    && rejection.contains("unsupported_named_grid_line")
                    && rejection.contains("property=Some(\"grid-column-start\")")
                    && !rejection.contains("node_frames_css_px=[");
            Log.i(TAG, "SPINON_C1132_GRID_REJECTION run_id=" + c1132GridNamedLinesRunId
                    + " scene_id=" + C1132_GRID_NAMED_LINE_REJECT_SCENE_ID
                    + " expected=" + expectedRejection + " " + rejection);
            if (!expectedRejection) {
                postStatus("실패 · C11.3.2 implicit named line 거부 계약 불일치");
                return;
            }
        }
        if (c114GridTrackSizingRequested) {
            String expectedRootFrame = c114ExpectedRootFrame(c114GridTrackSizingSceneId);
            String expectedItemFrame = c114ExpectedItemFrame(c114GridTrackSizingSceneId);
            String expectedNodeFrames = c114ExpectedNodeFrames(c114GridTrackSizingSceneId);
            boolean c114GridTrackSizingTraceAvailable =
                    logC114GridTrackSizingTraceChunks(c114GridTrackSizingRunId, result);
            boolean c114MultiSpanFramesMatch = c114ExpectedMultiSpanFramesMatch(
                    c114GridTrackSizingSceneId, result);
            boolean c114TextIntrinsicInputsMatch = c114ExpectedTextIntrinsicInputs(
                    c114GridTrackSizingSceneId, result);
            boolean c114ImageIntrinsicInputsMatch = c114ExpectedImageIntrinsicInputs(
                    c114GridTrackSizingSceneId, result);
            if (!result.startsWith("status=0 ") || !result.contains(
                    "layout=ready boxes=" + c114ExpectedBoxCount(c114GridTrackSizingSceneId))
                    || (!expectedRootFrame.isEmpty() && !result.contains(expectedRootFrame))
                    || (!expectedItemFrame.isEmpty() && !result.contains(expectedItemFrame))
                    || (expectedNodeFrames != null && !result.contains(expectedNodeFrames))
                    || !c114MultiSpanFramesMatch
                    || !c114TextIntrinsicInputsMatch
                    || !c114ImageIntrinsicInputsMatch) {
                Log.e(TAG, "SPINON_C114_GRID_ASSERTION_ERROR run_id="
                        + c114GridTrackSizingRunId + " scene_id=" + c114GridTrackSizingSceneId
                        + " expected_root_frame=" + expectedRootFrame
                        + " expected_item_frame=" + expectedItemFrame
                        + " expected_node_frames=" + expectedNodeFrames
                        + " multi_span_frames_match=" + c114MultiSpanFramesMatch
                        + " text_intrinsic_inputs_match=" + c114TextIntrinsicInputsMatch
                        + " image_intrinsic_inputs_match=" + c114ImageIntrinsicInputsMatch
                        + " report=" + result);
                postStatus("실패 · C11.4 track-sizing frame 검증 결과 불일치");
                return;
            }
            if (!c114GridTrackSizingTraceAvailable) {
                postStatus("실패 · C11.4 track-sizing trace가 없습니다");
                return;
            }
            Log.i(TAG, "SPINON_C114_GRID_SUMMARY run_id=" + c114GridTrackSizingRunId
                    + " scene_id=" + c114GridTrackSizingSceneId + " "
                    + "expected_root_frame=" + expectedRootFrame + " "
                    + "expected_item_frame=" + expectedItemFrame + " "
                    + runtimeStatusSummary(result));
            logNodeFrames("C114", c114GridTrackSizingSceneId, result);
        } else if (c1133GridTemplateAreasRequested) {
            Log.i(TAG, "SPINON_C1133_GRID_EVAL_RUN run_id=" + c1133GridTemplateAreasRunId
                    + " scene_id=" + c1133GridTemplateAreasSceneId + " "
                    + runtimeStatusSummary(result));
        } else if (c1132GridNamedLinesRequested) {
            Log.i(TAG, "SPINON_C1132_GRID_EVAL_RUN run_id=" + c1132GridNamedLinesRunId
                    + " scene_id=" + C1132_GRID_NAMED_LINE_SCENE_ID + " "
                    + runtimeStatusSummary(result));
        }
        if (c1122GridAutoPlacementRequested || c1123GridAutoPlacementRequested) {
            String marker = c1123GridAutoPlacementRequested ? "C1123" : "C1122";
            String runId = c1123GridAutoPlacementRequested ? c1123GridRunId : c1122GridRunId;
            String sceneId = c1123GridAutoPlacementRequested ? c1123GridSceneId : c1122GridSceneId;
            Log.i(TAG, "SPINON_" + marker + "_GRID_EVAL_RUN run_id=" + runId
                    + " scene_id=" + sceneId + " " + runtimeStatusSummary(result));
        } else if (c1121GridPlacementRequested) {
            Log.i(TAG, "SPINON_C1121_GRID_EVAL_RUN run_id=" + c1121GridRunId
                    + " scene_id=" + c1121GridSceneId + " " + runtimeStatusSummary(result));
        }
        if (!result.startsWith("status=0 ")) {
            if (c1133GridTemplateAreasRequested) {
                String failureMarker = "CSS runtime 장면 생성 실패:";
                int failureStart = result.indexOf(failureMarker);
                String diagnostic = failureStart >= 0
                        ? result.substring(failureStart) : "diagnostic-marker-missing";
                Log.e(TAG, "SPINON_C1133_GRID_DIAGNOSTIC run_id="
                        + c1133GridTemplateAreasRunId + " scene_id="
                        + c1133GridTemplateAreasSceneId + " "
                        + (diagnostic.length() <= 768
                                ? diagnostic : diagnostic.substring(0, 768) + " [truncated]"));
                String code = result.contains("unsupported_computed_value")
                        ? "unsupported_computed_value"
                        : result.contains("unsupported_grid_auto_rows")
                                ? "unsupported_grid_auto_rows"
                        : result.contains("unsupported_named_grid_line")
                                ? "unsupported_named_grid_line" : "missing";
                String property = result.contains("property=Some(\"grid-template-areas\")")
                        ? "grid-template-areas"
                        : result.contains("property=Some(\"grid-auto-rows\")")
                                ? "grid-auto-rows"
                        : result.contains("property=Some(\"grid-column-start\")")
                                ? "grid-column-start"
                                : result.contains("property=Some(\"grid-row-start\")")
                                        ? "grid-row-start" : "missing";
                java.util.regex.Matcher nodeMatcher = java.util.regex.Pattern
                        .compile("node=Some\\((\\d+)\\)").matcher(result);
                String nodeId = nodeMatcher.find() ? nodeMatcher.group(1) : "missing";
                String expectedNodeId = "area-track-count-mismatch".equals(c1133GridTemplateAreasSceneId)
                        || "unknown-area-implicit-tracks".equals(c1133GridTemplateAreasSceneId)
                        ? "1" : "missing";
                boolean expected = result.startsWith("status=-11 ")
                        && (("area-track-count-mismatch".equals(c1133GridTemplateAreasSceneId)
                                && "unsupported_computed_value".equals(code)
                                && "grid-template-areas".equals(property)
                                && expectedNodeId.equals(nodeId))
                        || ("unknown-area-implicit-tracks".equals(c1133GridTemplateAreasSceneId)
                                && "unsupported_grid_auto_rows".equals(code)
                                && "grid-auto-rows".equals(property)
                                && expectedNodeId.equals(nodeId)));
                String rejectionStatus = result.split(" ", 2)[0];
                Log.i(TAG, "SPINON_C1133_GRID_REJECTION_SUMMARY run_id="
                        + c1133GridTemplateAreasRunId + " scene_id="
                        + c1133GridTemplateAreasSceneId + " expected=" + expected
                        + " " + rejectionStatus + " code=" + code + " property=" + property
                        + " node_id=" + nodeId
                        + " frames=" + (result.contains("node_frames_css_px=[") ? "present" : "absent"));
            }
            if (c1122GridAutoPlacementRequested || c1123GridAutoPlacementRequested) {
                String marker = c1123GridAutoPlacementRequested ? "C1123" : "C1122";
                String runId = c1123GridAutoPlacementRequested ? c1123GridRunId : c1122GridRunId;
                String sceneId = c1123GridAutoPlacementRequested ? c1123GridSceneId : c1122GridSceneId;
                Log.e(TAG, "SPINON_" + marker + "_GRID_SUMMARY run_id=" + runId
                        + " scene_id=" + sceneId + " " + result);
                String failureMarker = "CSS runtime 장면 생성 실패:";
                int failureStart = result.indexOf(failureMarker);
                String diagnostic = failureStart >= 0
                        ? result.substring(failureStart) : "diagnostic-marker-missing";
                Log.e(TAG, "SPINON_" + marker + "_GRID_DIAGNOSTIC run_id=" + runId
                        + " scene_id=" + sceneId + " "
                        + (diagnostic.length() <= 768
                                ? diagnostic : diagnostic.substring(0, 768) + " [truncated]"));
            } else if (c1121GridPlacementRequested) {
                boolean superseded = result.startsWith("status=-12 ")
                        && result.contains("op=eval status=0 ");
                if (superseded) {
                    Log.i(TAG, "SPINON_C1121_GRID_SUPERSEDED run_id=" + c1121GridRunId
                            + " scene_id=" + c1121GridSceneId + " "
                            + runtimeStatusSummary(result));
                    return;
                }
                Log.e(TAG, "SPINON_C1121_GRID_SUMMARY run_id=" + c1121GridRunId
                        + " scene_id=" + c1121GridSceneId + " " + result);
                String failureMarker = "CSS runtime 장면 생성 실패:";
                int failureStart = result.indexOf(failureMarker);
                String diagnostic = failureStart >= 0
                        ? result.substring(failureStart) : "diagnostic-marker-missing";
                Log.e(TAG, "SPINON_C1121_GRID_DIAGNOSTIC run_id=" + c1121GridRunId
                        + " scene_id=" + c1121GridSceneId + " "
                        + (diagnostic.length() <= 768
                                ? diagnostic : diagnostic.substring(0, 768) + " [truncated]"));
            }
            if (c123FixedPositionProbeRequested) logC123FailureDetails(result);
            if (c122AbsoluteBlockProbeRequested) logC122FailureDetails(result);
            postStatus("실패 · " + result);
            return;
        }
        if (rootHtmlApplicationRequested) {
            if (!result.contains("layout=ready")) {
                Log.e(TAG, "SPINON_ROOT_HTML_DEFAULT_APP_BOOT_ERROR reason=layout-not-ready "
                        + result);
                postStatus("실패 · root-app.js 평가 뒤 HTML 장면이 준비되지 않았습니다");
                return;
            }
            Log.i(TAG, "SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass root_app_marker=ready "
                    + runtimeStatusSummary(result));
        }
        if (c132ScrollFixtureRequested) {
            boolean c132ScrollPassed = result.contains("c13_2_scroll_fixture=passed")
                    && result.contains("border_escape_outside_parent=true")
                    && result.contains("border_escape_hit=matched")
                    && result.contains("c13_2_root_body_propagation=passed")
                    && result.contains("c13_2_root_body_hidden=passed")
                    && result.contains("c13_2_lifecycle=passed")
                    && result.contains("c13_2_failure_controls=passed")
                    && result.contains("reserve_failure=atomic")
                    && result.contains("reserve_failure_marker=matched")
                    && result.contains("stale_publish=rejected")
                    && result.contains("recovery=same_host")
                    && result.contains("growth_offset_retained=true")
                    && result.contains("shrink_clamped_to_zero=true")
                    && result.contains("detach_removed_owner=true")
                    && result.contains("reparent_reset_offset=true");
            if (!c132ScrollPassed) {
                postStatus("실패 · C13.2 scroll·paint·hit·lifecycle 결과 불일치");
                return;
            }
            Log.i(TAG, "SPINON_C132_SCROLL_ASSERTIONS root_body_propagation=passed"
                    + " root_body_hidden=passed border_escape_outside_parent=true"
                    + " border_escape_hit=matched lifecycle=passed"
                    + " growth_offset_retained=true shrink_clamped_to_zero=true"
                    + " detach_removed_owner=true reparent_reset_offset=true"
                    + " reserve_failure=atomic reserve_failure_marker=matched"
                    + " stale_publish=rejected recovery=same_host");
            c132ScrollAssertionsPassed = true;
        }
        if (BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST && c133GestureFixtureRequested) {
            boolean knownScenario = c133GesturePolicyScenarioID >= 0
                    && c133GesturePolicyScenarioID <= 10;
            String scenario = c133GesturePolicyScenarioID == 0 ? "auto-zero"
                    : c133GesturePolicyScenarioID == 1 ? "auto-boundary"
                    : c133GesturePolicyScenarioID == 2 ? "contain-x-auto-y-boundary"
                    : c133GesturePolicyScenarioID == 3 ? "touch-action-none"
                    : c133GesturePolicyScenarioID == 4 ? "touch-action-pan-x"
                    : c133GesturePolicyScenarioID == 5 ? "touch-action-pan-y"
                    : c133GesturePolicyScenarioID == 6 ? "touch-action-target-pan-x"
                    : c133GesturePolicyScenarioID == 7
                            ? "touch-action-intersection-none"
                    : c133GesturePolicyScenarioID == 8 ? "style-policy-invalidation"
                    : c133GesturePolicyScenarioID == 9 ? "geometry-invalidation"
                    : c133GesturePolicyScenarioID == 10 ? "paint-invalidation" : "invalid";
            String expectedPolicy = c133GesturePolicyScenarioID == 2
                    ? "policy_inner=contain/auto" : "policy_inner=auto/auto";
            boolean boundarySeed = c133GesturePolicyScenarioID == 1
                    || c133GesturePolicyScenarioID == 2
                    || c133GesturePolicyScenarioID == 8
                    || c133GesturePolicyScenarioID == 9;
            String expectedSeed = boundarySeed ? "fixture_seed=inner-max" : "fixture_seed=none";
            String expectedOffsets = boundarySeed
                    ? "offsets=root:0,0;outer:0,0;inner:86,106"
                    : "offsets=root:0,0;outer:0,0;inner:0,0";
            String expectedTouchAction = c133GesturePolicyScenarioID == 3
                    || c133GesturePolicyScenarioID == 7 ? "touch_action_effective=none"
                    : c133GesturePolicyScenarioID == 4 || c133GesturePolicyScenarioID == 6
                            || c133GesturePolicyScenarioID == 8
                            ? "touch_action_effective=pan-x"
                    : c133GesturePolicyScenarioID == 5 ? "touch_action_effective=pan-y"
                    : "touch_action_effective=auto";
            c133GestureFixtureReady = result.contains("c13_3_gesture_fixture=ready")
                    && result.contains("viewport_css_px=400x240")
                    && result.contains("owners=root:")
                    && result.contains("input_queue_capacity=64")
                    && knownScenario
                    && result.contains("scenario=" + scenario)
                    && result.contains(expectedPolicy)
                    && result.contains(expectedTouchAction)
                    && result.contains(expectedSeed)
                    && result.contains(expectedOffsets);
            if (!c133GestureFixtureReady) {
                postStatus("실패 · C13.3 입력 장면·owner 초기화 결과가 불일치합니다");
                Log.e(TAG, "SPINON_C133_READY_ASSERTIONS failed=true " + result);
                return;
            }
            c133GestureInitialDrawReady = false;
            postStatus("준비 중 · C13.3 boundary snapshot의 첫 draw를 확인합니다");
            Log.i(TAG, "SPINON_C133_EVAL_READY input=MotionEvent native_slop_px="
                    + c133TouchSlopPx + " worker_delay_ms=" + c133WorkerDelayMillis
                    + " " + result);
            requestDraw();
            if (c133SyntheticSecondPointerProbeRequested) {
                surfaceView.post(this::runC133SyntheticSecondPointerProbe);
            }
        }
        if (c131CompactSurfaceMatrixProbeRequested
                && !logC131CompactSurfaceMatrixFixtureReport(result)) {
            postStatus("실패 · C13.1 compact frame·hit-test 결과 불일치");
            return;
        }
        if (c131StackingDescendantClipProbeRequested
                && !logC131StackingDescendantClipFixtureReport(result)) {
            postStatus("실패 · C13.1 nested stacking frame·hit-test 결과 불일치");
            return;
        }
        if (c131BorderStripProbeRequested && !logC131BorderFixtureReport(result)) {
            postStatus("실패 · C13.1 border-strip frame·hit-test 결과 불일치");
            return;
        }
        if (c124DepthSafetyProbeRequested) {
            try {
                c124FixtureNodeCount = Integer.parseInt(reportField(result, "document_nodes="));
            } catch (NumberFormatException error) {
                postStatus("실패 · C12.4 fixture 기본 노드 수를 읽지 못했습니다");
                return;
            }
        }
        Log.i(TAG, (BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST && imagePaintProbeRequested
                ? imageAsyncReplacementProbeRequested ? "SPINON_C184_ASYNC_IMAGE_EVAL "
                        : imageObjectFitPositionProbeRequested ? "SPINON_C183_OBJECT_FIT_POSITION_EVAL "
                        : imageAlphaProbeRequested ? "SPINON_C182_ALPHA_IMAGE_EVAL "
                        : "SPINON_C181_IMAGE_PAINT_EVAL " : c133GestureFixtureRequested
                ? "SPINON_C133_EVAL " : c132ScrollFixtureRequested
                ? "SPINON_C132_SCROLL_EVAL " : c131StackingDescendantClipProbeRequested
                ? "SPINON_C131_STACKING_CLIP_EVAL_RUN run_id="
                        + c131StackingDescendantClipRunId + " "
                : c131CompactSurfaceMatrixProbeRequested
                ? "SPINON_C131_COMPACT_EVAL_RUN run_id=" + c131CompactSurfaceMatrixRunId + " "
                : c131BorderStripProbeRequested
                ? "SPINON_C131_BORDER_EVAL_RUN " : c131OverflowClipProbeRequested
                ? "SPINON_C131_EVAL_RUN " : c114GridTrackSizingRequested
                ? "SPINON_C114_GRID_EVAL run_id=" + c114GridTrackSizingRunId
                        + " scene_id=" + c114GridTrackSizingSceneId + " "
                : c1133GridTemplateAreasRequested
                ? "SPINON_C1133_GRID_EVAL run_id=" + c1133GridTemplateAreasRunId
                        + " scene_id=" + c1133GridTemplateAreasSceneId + " "
                : c1132GridNamedLinesRequested
                ? "SPINON_C1132_GRID_EVAL run_id=" + c1132GridNamedLinesRunId
                        + " scene_id=" + C1132_GRID_NAMED_LINE_SCENE_ID + " "
                : c1131GridRepeatRequested
                ? "SPINON_C1131_GRID_EVAL run_id=" + c1131GridRepeatRunId
                        + " scene_id=" + C1131_GRID_REPEAT_SCENE_ID + " "
                : (c1122GridAutoPlacementRequested || c1123GridAutoPlacementRequested)
                ? "SPINON_" + (c1123GridAutoPlacementRequested ? "C1123" : "C1122")
                        + "_GRID_EVAL run_id="
                        + (c1123GridAutoPlacementRequested ? c1123GridRunId : c1122GridRunId)
                        + " scene_id="
                        + (c1123GridAutoPlacementRequested ? c1123GridSceneId : c1122GridSceneId) + " "
                : c1121GridPlacementRequested
                ? "SPINON_C1121_GRID_EVAL run_id=" + c1121GridRunId
                        + " scene_id=" + c1121GridSceneId + " "
                : c11GridFixtureRequested
                ? "SPINON_C111_GRID_EVAL " : c1035PositionedFlexProbeRequested
                ? "SPINON_C1035_EVAL " : c123FixedPositionProbeRequested
                ? "SPINON_C123_EVAL " : c122AbsoluteBlockProbeRequested
                ? "SPINON_C122_INITIAL " : c121PositioningProbeRequested
                ? "SPINON_C121_INITIAL " : flexBaselineProbeRequested
                ? "SPINON_C1034_EVAL " : flexAlignmentProbeRequested
                ? "SPINON_C1033_EVAL " : flexOrderProbeRequested
                ? "SPINON_C1032_EVAL " : flexReverseProbeRequested
                ? "SPINON_C1031_EVAL " : flexDistributionProbeRequested
                ? "SPINON_C102_EVAL " : flexWrapProbeRequested
                ? "SPINON_C101_EVAL " : marginCollapseProbeRequested
                ? "SPINON_C092_EVAL " : blockFormattingProbeRequested
                ? "SPINON_C091_EVAL " : viewportUnitsProbeRequested
                ? "SPINON_C066A_EVAL " : rootHtmlTextPaintProbeRequested
                ? "SPINON_C0412_ROOT_HTML_TEXT_PAINT_EVAL " : rootHtmlBlockPaintProbeRequested
                ? "SPINON_C0412_ROOT_HTML_BLOCK_PAINT_EVAL " : blockPaintProbeRequested
                ? "SPINON_C08_EVAL " : aspectRatioProbeRequested
                ? "SPINON_C073_EVAL " : borderWidthProbeRequested
                ? "SPINON_C072_EVAL " : minMaxSizingProbeRequested
                ? "SPINON_C071_EVAL " : typedCssMathProbeRequested
                ? "SPINON_C065_EVAL " : fontRelativeUnitsProbeRequested
                ? "SPINON_C064_EVAL " : absoluteLengthsProbeRequested
                ? "SPINON_C063_EVAL " : spacingPercentagesProbeRequested
                ? "SPINON_C062_EVAL " : percentageDimensionsProbeRequested
                ? "SPINON_C061_EVAL " : incrementalRestyleProbeRequested
                ? "SPINON_C054_INIT "
                : registeredPropertiesProbeRequested
                ? runtimeResultCacheProbeRequested ? "SPINON_C053_INIT " : "SPINON_C052_EVAL "
                : authorStylesheetsProbeRequested
                        ? "SPINON_C0411_EVAL " : "SPINON_C0410_EVAL ") + result);
        if (c1133GridTemplateAreasRequested) {
            Log.i(TAG, "SPINON_C1133_GRID_SUMMARY run_id=" + c1133GridTemplateAreasRunId
                    + " scene_id=" + c1133GridTemplateAreasSceneId + " "
                    + runtimeStatusSummary(result));
            logNodeFrames("C1133", c1133GridTemplateAreasSceneId, result);
        } else if (c1132GridNamedLinesRequested) {
            Log.i(TAG, "SPINON_C1132_GRID_SUMMARY run_id=" + c1132GridNamedLinesRunId
                    + " scene_id=" + C1132_GRID_NAMED_LINE_SCENE_ID + " "
                    + runtimeStatusSummary(result));
            logNodeFrames("C1132", C1132_GRID_NAMED_LINE_SCENE_ID, result);
        } else if (c1131GridRepeatRequested) {
            Log.i(TAG, "SPINON_C1131_GRID_SUMMARY run_id=" + c1131GridRepeatRunId
                    + " scene_id=" + C1131_GRID_REPEAT_SCENE_ID + " "
                    + runtimeStatusSummary(result));
            logNodeFrames("C1131", "multi-track-repeat", result);
        } else if (c1122GridAutoPlacementRequested || c1123GridAutoPlacementRequested) {
            logGridAutoPlacementReadySummaryIfNeeded(result);
        } else if (c1121GridPlacementRequested) {
            logC1121GridReadySummaryIfNeeded(result);
        }
        if (c123FixedPositionProbeRequested) {
            Log.i(TAG, "SPINON_C123_SUMMARY state=matrix-360x800-dpr-1 "
                    + runtimeStatusSummary(result));
            logNodeFrames("C123", "matrix-360x800-dpr-1", result);
            String initialEnvironmentRevisionValue = reportField(
                    environment, "environment_revision=");
            String initialEvalRevision = reportField(result, "environment_revision=");
            final long initialEnvironmentRevision;
            try {
                initialEnvironmentRevision = Long.parseLong(initialEnvironmentRevisionValue);
            } catch (NumberFormatException error) {
                Log.e(TAG, "SPINON_C123_MATRIX_FAILURE state=initial "
                        + "missing_environment_revision=" + initialEnvironmentRevisionValue);
                postStatus("실패 · C12.3 초기 환경 revision이 없습니다");
                return;
            }
            if (!initialEnvironmentRevisionValue.equals(initialEvalRevision)) {
                Log.e(TAG, "SPINON_C123_MATRIX_FAILURE state=initial expected_revision="
                        + initialEnvironmentRevisionValue + " actual_revision="
                        + initialEvalRevision);
                postStatus("실패 · C12.3 초기 environment revision 불일치");
                return;
            }
            result = runC123EnvironmentMatrix(
                    host, dark, initialEnvironmentRevision);
            if (result == null) return;
        }
        if (c122AbsoluteBlockProbeRequested) {
            Log.i(TAG, "SPINON_C122_SUMMARY " + runtimeStatusSummary(result));
            logNodeFrames("C122", "initial", result);
        }
        if (c1035PositionedFlexProbeRequested) {
            Log.i(TAG, "SPINON_C1035_SUMMARY " + runtimeStatusSummary(result));
            logNodeFrames("C1035", "initial", result);
        }
        if (c121PositioningProbeRequested) {
            Log.i(TAG, "SPINON_C121_SUMMARY state=initial " + runtimeStatusSummary(result));
            logC121NodeFrames("initial", result);
            for (int state = 1; state <= 2; state++) {
                String stateName = state == 1 ? "target-relative" : "ancestor-relative";
                String stateResult = "";
                int attempts = 0;
                do {
                    attempts++;
                    stateResult = decode(nativeEvalC121PositionState(host, state));
                    boolean superseded = stateResult.startsWith("status=-12 ")
                            && stateResult.contains("op=eval status=0 ");
                    if (!superseded || attempts == 5) break;
                    Log.i(TAG, "SPINON_C121_STATE_RETRY name=" + stateName
                            + " attempt=" + attempts + " reason=scene-superseded");
                    try {
                        Thread.sleep(25);
                    } catch (InterruptedException error) {
                        Thread.currentThread().interrupt();
                        postStatus("실패 · C12.1 " + stateName + " 재시도가 중단됐습니다");
                        return;
                    }
                } while (attempts < 5);
                if (!stateResult.startsWith("status=0 ")) {
                    postStatus("실패 · C12.1 " + stateName + " · " + stateResult);
                    return;
                }
                Log.i(TAG, "SPINON_C121_STATE name=" + stateName + " attempts=" + attempts + " "
                        + runtimeStatusSummary(stateResult));
                logC121NodeFrames(stateName, stateResult);
                result = stateResult;
            }
        }
        if (customPropertiesProbeRequested) {
            result = decode(nativeEvalCustomPropertiesFixture(host));
            if (!result.startsWith("status=0 ")) {
                postStatus("실패 · C05 사용자 지정 속성 · " + result);
                return;
            }
            Log.i(TAG, "SPINON_C051_EVAL " + result);
        }
        postStatus(runtimeStatusSummary(result));
        requestDraw();
        renderLane.request();
        if (c124RuntimeMatrixProbeRequested) enqueueRuntime(this::runC124RuntimeMatrix);
        if (c124DepthSafetyProbeRequested) enqueueRuntime(this::runC124DepthSafetyProbe);
    }

    private void runC124DepthSafetyProbe() {
        final long host;
        synchronized (stateLock) {
            host = closing ? 0 : hostHandle;
        }
        if (host == 0) {
            Log.e(TAG, "SPINON_C124_DEPTH_SUMMARY status=failed reason=host-unavailable");
            postStatus("실패 · C12.4 깊이 검증 runtime host가 없습니다");
            return;
        }


        long started = SystemClock.elapsedRealtime();
        String maximum = decode(nativeEvalC124Source(host, c124DepthBuildSource(1024)));
        long maximumMillis = SystemClock.elapsedRealtime() - started;
        boolean maximumPassed = maximum.startsWith("status=0 ")
                && "ready".equals(reportField(maximum, "layout="))
                && String.valueOf(c124FixtureNodeCount + 1023)
                        .equals(reportField(maximum, "boxes="))
                && String.valueOf(c124FixtureNodeCount + 1023)
                        .equals(reportField(maximum, "cascadeRecomputedStyleElements="));
        Log.i(TAG, "SPINON_C124_DEPTH_STAGE stage=maximum expected=1024 passed="
                + maximumPassed + " elapsed_ms=" + maximumMillis
                + " report_length=" + maximum.length()
                + (maximumPassed ? "" : " report=" + maximum));
        if (!maximumPassed) {
            Log.e(TAG, "SPINON_C124_DEPTH_SUMMARY status=failed stage=maximum");
            postStatus("실패 · C12.4 최대 깊이 1024 계산을 확인하지 못했습니다");
            return;
        }

        started = SystemClock.elapsedRealtime();
        String overflow = decode(nativeEvalC124Source(host, c124DepthOverflowSource()));
        long overflowMillis = SystemClock.elapsedRealtime() - started;
        boolean overflowPassed = !overflow.startsWith("status=0 ")
                && overflow.contains("tree_depth_exceeded")
                && overflow.contains("1025") && overflow.contains("1024");
        Log.i(TAG, "SPINON_C124_DEPTH_STAGE stage=overflow expected=reject-1025 passed="
                + overflowPassed + " elapsed_ms=" + overflowMillis
                + " report=" + overflow);
        if (!overflowPassed) {
            Log.e(TAG, "SPINON_C124_DEPTH_SUMMARY status=failed stage=overflow");
            postStatus("실패 · C12.4 깊이 1025 거부 사유가 다릅니다");
            return;
        }

        started = SystemClock.elapsedRealtime();
        String recovery = decode(nativeEvalC124Source(host, c124DepthRecoverySource()));
        long recoveryMillis = SystemClock.elapsedRealtime() - started;
        boolean recoveryPassed = recovery.startsWith("status=0 ")
                && "ready".equals(reportField(recovery, "layout="))
                && String.valueOf(c124FixtureNodeCount)
                        .equals(reportField(recovery, "boxes="));
        Log.i(TAG, "SPINON_C124_DEPTH_STAGE stage=recovery expected=ready passed="
                + recoveryPassed + " elapsed_ms=" + recoveryMillis
                + " report_length=" + recovery.length()
                + (recoveryPassed ? "" : " report=" + recovery));
        boolean passed = maximumPassed && overflowPassed && recoveryPassed;
        Log.i(TAG, "SPINON_C124_DEPTH_SUMMARY status=" + (passed ? "passed" : "failed")
                + " maximum=1024 overflow=1025 recovery=" + recoveryPassed
                + " platform=android model=" + Build.MODEL
                + " api=" + Build.VERSION.SDK_INT);
        postStatus(passed ? "C12.4 깊이 검증 통과 · 1024 통과, 1025 거부, 복구 통과"
                : "실패 · C12.4 깊이 초과 후 복구를 확인하지 못했습니다");
        if (passed) requestDraw();
    }


    private static String c124DepthBuildSource(int maximumDepth) {
        return "(() => { let parent = globalThis.spinonC124NodeRefs['c124-root']; "
                + "let root = null; "
                + "for (let index = 0; index < " + (maximumDepth - 1) + "; index += 1) { "
                + "const node = document.createElement('div'); "
                + "node.setAttribute('style', 'position:relative;z-index:0;width:1px;height:1px'); "
                + "parent.appendChild(node); if (root === null) root = node; parent = node; } "
                + "globalThis.__spinonC124DepthRoot = root; "
                + "globalThis.__spinonC124DepthLeaf = parent; "
                + "return 'SPINON_C124_DEPTH_MAX=" + maximumDepth + "'; })()";
    }

    private static String c124DepthOverflowSource() {
        return "(() => { const node = document.createElement('div'); "
                + "node.setAttribute('style', 'position:relative;z-index:0;width:1px;height:1px'); "
                + "globalThis.__spinonC124DepthLeaf.appendChild(node); "
                + "globalThis.__spinonC124DepthOverflow = node; "
                + "return 'SPINON_C124_DEPTH_OVERFLOW=1025'; })()";
    }

    private static String c124DepthRecoverySource() {
        return "(() => { globalThis.__spinonC124DepthLeaf.removeChild("
                + "globalThis.__spinonC124DepthOverflow); "
                + "globalThis.spinonC124NodeRefs['c124-root'].removeChild("
                + "globalThis.__spinonC124DepthRoot); "
                + "delete globalThis.__spinonC124DepthOverflow; "
                + "delete globalThis.__spinonC124DepthRoot; "
                + "delete globalThis.__spinonC124DepthLeaf; "
                + "return 'SPINON_C124_DEPTH_RECOVERED'; })()";
    }

    private void runC124RuntimeMatrix() {
        int caseCount = 0;
        int expectedReady = 0;
        int expectedRejected = 0;
        int matched = 0;
        try {
            JSONObject matrix = new JSONObject(readAsset("c12-4-runtime-matrix.json"));
            JSONArray cases = matrix.getJSONArray("cases");
            for (int index = 0; index < cases.length(); index++) {
                JSONObject fixtureCase = cases.getJSONObject(index);
                String caseId = fixtureCase.getString("id");
                JSONObject expected = fixtureCase.getJSONObject("expected");
                boolean shouldBeSupported = "supported".equals(expected.getString("kind"));
                if (shouldBeSupported) expectedReady++;
                else expectedRejected++;

                long host = nativeCreateC124StackingHost();
                if (host == 0) {
                    Log.e(TAG, "SPINON_C124_MATRIX_CASE id=" + caseId
                            + " expected=" + (shouldBeSupported ? "ready" : "reject")
                            + " actual=host-create-failed reason_matched=false matched=false");
                    caseCount++;
                    continue;
                }
                String report;
                try {
                    String environment = decode(nativeSetEnvironment(
                            host, 320, 240, density, darkMode));
                    report = environment.startsWith("status=0 ")
                            ? decode(nativeEvalC124Source(host, fixtureCase.getString("source")))
                            : environment;
                } finally {
                    nativeFreeHost(host);
                }

                boolean ready = report.startsWith("status=0 ")
                        && report.contains("layout=ready");
                boolean rejected = report.contains(" op=eval ")
                        && !report.startsWith("status=0 ");
                boolean reasonMatched = shouldBeSupported
                        || matchesC124ExpectedDiagnostic(expected, report);
                boolean caseMatched = shouldBeSupported ? ready : rejected && reasonMatched;
                if (caseMatched) matched++;
                String expectation = shouldBeSupported ? "ready" : "reject";
                String actual = ready ? "ready" : rejected
                        ? reasonMatched ? "reject" : "reject-wrong-diagnostic"
                        : "invalid-layout";
                Log.i(TAG, "SPINON_C124_MATRIX_CASE index=" + index + " id=" + caseId
                        + " expected=" + expectation + " actual=" + actual
                        + " reason_matched=" + reasonMatched + " matched=" + caseMatched
                        + " " + report);
                caseCount++;
            }
            boolean complete = caseCount == cases.length() && matched == caseCount;
            Log.i(TAG, "SPINON_C124_MATRIX_SUMMARY status=" + (complete ? "matched" : "mismatch")
                    + " cases=" + caseCount + " nodes=" + matrix.getJSONObject("summary").getInt("nodes")
                    + " expected_ready=" + expectedReady
                    + " expected_rejected=" + expectedRejected + " matched=" + matched);
            postStatus("C12.4 행렬 " + matched + "/" + caseCount + " 장면 확인");
        } catch (Exception error) {
            Log.e(TAG, "SPINON_C124_MATRIX_SUMMARY status=error cases=" + caseCount
                    + " matched=" + matched, error);
            postStatus("실패 · C12.4 행렬 " + error.getMessage());
        }
    }

    private boolean matchesC124ExpectedDiagnostic(JSONObject expected, String report) {
        String kind = expected.optString("kind", "");
        String property = expected.optString("property", "");
        String api = expected.optString("api", "");
        if (!property.isEmpty() && "pseudo-element".equals(property)) {
            return report.contains("cascade_error=")
                    && report.contains("지원하지 않는 CSS 선언 content");
        }
        if (!property.isEmpty()) {
            String expectedProperty = "property=Some(\"" + property + "\")";
            return report.contains("node=Some(") && report.contains(expectedProperty);
        }
        return "api-rejection".equals(kind)
                && "showModal".equals(api)
                && report.contains("TypeError:")
                && report.contains("showModal is not a function");
    }

    void setRootHtmlApplicationSources(String htmlSource, String appScript) {
        if (!rootHtmlApplicationRequested || hostHandle != 0) {
            throw new IllegalStateException("기본 root HTML source는 runtime 초기화 전에만 설정할 수 있습니다");
        }
        if (htmlSource == null || htmlSource.isBlank()
                || appScript == null || appScript.isBlank()) {
            throw new IllegalArgumentException("index.html과 root-app.js가 비어 있습니다");
        }
        rootHtmlApplicationSource = htmlSource;
        rootHtmlApplicationScript = appScript;
        Log.i(TAG, "SPINON_ROOT_HTML_DEFAULT_APP_ASSET html_bytes="
                + htmlSource.getBytes(StandardCharsets.UTF_8).length
                + " script_bytes=" + appScript.getBytes(StandardCharsets.UTF_8).length);
        scheduleRuntimeInitializationForReadySurface();
    }

    private long nativeCreateRootHtmlApplicationHost() {
        if (rootHtmlApplicationSource == null) {
            Log.e(TAG, "SPINON_ROOT_HTML_DEFAULT_APP_ASSET_ERROR source=missing");
            return 0;
        }
        byte[] source = rootHtmlApplicationSource.getBytes(StandardCharsets.UTF_8);
        Log.i(TAG, "SPINON_ROOT_HTML_DEFAULT_APP_SOURCE bytes=" + source.length);
        return nativeCreateHostWithRootHtml(source);
    }

    private long nativeCreateRootHtmlDefaultRenderHost() {
        try {
            String html = rootHtmlPageScaleCase == null
                    ? readAsset("root-html-default-rendering.html")
                    : readAsset("root-html-page-scale-runtime.html");
            if (rootHtmlPageScaleCase != null) {
                String originalViewport = "content=\"width=device-width, initial-scale=1\"";
                String replacementViewport = rootHtmlPageScaleCase.equals("half")
                        ? "content=\"width=device-width, initial-scale=0.5\""
                        : rootHtmlPageScaleCase.equals("double")
                        ? "content=\"width=device-width, initial-scale=2\""
                        : "content=\"width=500, initial-scale=1\"";
                int marker = html.indexOf(originalViewport);
                if (marker < 0 || html.indexOf(originalViewport,
                        marker + originalViewport.length()) >= 0) {
                    throw new IOException("viewport meta marker count must equal one");
                }
                html = html.substring(0, marker) + replacementViewport
                        + html.substring(marker + originalViewport.length());
                Log.i(TAG, "SPINON_C0412_ROOT_HTML_PAGE_SCALE_SOURCE case="
                        + rootHtmlPageScaleCase + " viewport=" + replacementViewport);
            }
            byte[] source = html.getBytes(StandardCharsets.UTF_8);
            Log.i(TAG, "SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_SOURCE case="
                    + (rootHtmlPageScaleCase == null ? "default" : rootHtmlPageScaleCase)
                    + " bytes=" + source.length + " sha256=" + sha256Hex(source));
            return nativeCreateHostWithRootHtml(source);
        } catch (IOException error) {
            Log.e(TAG, "SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_ASSET_ERROR", error);
            return 0;
        }
    }

    private String readAsset(String name) throws IOException {
        try (InputStream input = activity.getAssets().open(name);
                ByteArrayOutputStream output = new ByteArrayOutputStream()) {
            byte[] buffer = new byte[8192];
            int count;
            while ((count = input.read(buffer)) != -1) output.write(buffer, 0, count);
            return output.toString(StandardCharsets.UTF_8.name());
        }
    }

    private static String sha256Hex(byte[] value) {
        try {
            byte[] digest = MessageDigest.getInstance("SHA-256").digest(value);
            StringBuilder result = new StringBuilder(digest.length * 2);
            for (byte octet : digest) result.append(String.format("%02x", octet & 0xff));
            return result.toString();
        } catch (NoSuchAlgorithmException error) {
            throw new IllegalStateException("SHA-256 is unavailable", error);
        }
    }

    private String evaluateC124SurfaceCase(long host) {
        if (!c124SurfaceCaseRequested.matches("[a-z0-9-]{1,80}")
                || c124SurfaceRunId == null
                || !c124SurfaceRunId.matches("[A-Za-z0-9_-]{1,80}")) {
            return "status=-70 C12.4 surface case/run ID가 올바르지 않습니다";
        }
        try {
            JSONObject matrix = new JSONObject(readAsset("c12-4-runtime-matrix.json"));
            JSONObject stage = matrix.getJSONObject("stage");
            JSONArray cases = matrix.getJSONArray("cases");
            for (int caseIndex = 0; caseIndex < cases.length(); caseIndex++) {
                JSONObject fixtureCase = cases.getJSONObject(caseIndex);
                if (!c124SurfaceCaseRequested.equals(fixtureCase.getString("id"))) continue;
                if (!"supported".equals(fixtureCase.getJSONObject("expected").getString("kind"))) {
                    return "status=-71 C12.4 surface case는 지원 장면이어야 합니다";
                }
                JSONArray sampleJson = fixtureCase.getJSONArray("samples");
                List<C124SurfacePixelCapture.Sample> samples = new ArrayList<>(sampleJson.length());
                for (int sampleIndex = 0; sampleIndex < sampleJson.length(); sampleIndex++) {
                    JSONObject sample = sampleJson.getJSONObject(sampleIndex);
                    samples.add(new C124SurfacePixelCapture.Sample(
                            sample.getString("id"),
                            sample.getDouble("xCssPx"),
                            sample.getDouble("yCssPx")));
                }
                if (samples.isEmpty()) return "status=-72 C12.4 surface 표본이 없습니다";
                c124SurfaceSamples = samples;
                c124SurfaceOriginXCssPx = stage.getInt("x");
                c124SurfaceOriginYCssPx = stage.getInt("y");
                c124SurfaceInventorySha256 = matrix.getJSONObject("inventory").getString("sha256");
                Log.i(TAG, "SPINON_C124_SURFACE_CASE status=selected id="
                        + c124SurfaceCaseRequested + " run_id=" + c124SurfaceRunId
                        + " sample_count=" + samples.size()
                        + " inventory_sha256=" + c124SurfaceInventorySha256);
                return decode(nativeEvalC124Source(host, fixtureCase.getString("source")));
            }
            return "status=-73 C12.4 surface case가 matrix에 없습니다";
        } catch (Exception error) {
            return "status=-74 C12.4 surface case asset 오류: "
                    + error.getClass().getSimpleName();
        }
    }

    private void requestC124SurfaceCapture(long renderer, long generation) {
        final int width;
        final int height;
        synchronized (stateLock) {
            if (closing || !c124SurfaceCaptureProbeRequested || c124SurfaceCaptureStarted
                    || !surfaceAvailable || surfaceGeneration != generation
                    || rendererHandle != renderer || rendererGeneration != generation) {
                return;
            }
            c124SurfaceCaptureStarted = true;
            width = surfaceWidth;
            height = surfaceHeight;
        }
        File captureDirectory = new File(activity.getCacheDir(), "c124-surface-captures");
        File output = new File(captureDirectory,
                c124SurfaceCaseRequested + "-" + c124SurfaceRunId + ".png");
        C124SurfacePixelCapture.request(surfaceView, width, height, density,
                c124SurfaceOriginXCssPx, c124SurfaceOriginYCssPx,
                c124SurfaceCaseRequested, c124SurfaceRunId, output,
                c124SurfaceSamples, mainHandler, runtimeQueue,
                () -> {
                    synchronized (stateLock) {
                        return !closing && surfaceAvailable && surfaceGeneration == generation
                                && rendererHandle == renderer && rendererGeneration == generation
                                && surfaceWidth == width && surfaceHeight == height;
                    }
                }, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C124_SURFACE_CAPTURE status=error run_id="
                                + c124SurfaceRunId + " case=" + c124SurfaceCaseRequested
                                + " attempts=" + result.attempts + " reason=" + result.error);
                        postStatus("실패 · C12.4 PixelCopy · " + result.error);
                        return;
                    }
                    Log.i(TAG, "SPINON_C124_SURFACE_CAPTURE status=ok run_id="
                            + c124SurfaceRunId + " case=" + c124SurfaceCaseRequested
                            + " width=" + result.width + " height=" + result.height
                            + " density=" + density + " attempts=" + result.attempts
                            + " inventory_sha256=" + c124SurfaceInventorySha256
                            + " sha256=" + result.sha256);
                    for (C124SurfacePixelCapture.Pixel pixel : result.pixels) {
                        Log.i(TAG, "SPINON_C124_SURFACE_PIXEL run_id=" + c124SurfaceRunId
                                + " case=" + c124SurfaceCaseRequested
                                + " sample=" + pixel.id + " x=" + pixel.x + " y=" + pixel.y
                                + " rgba=" + pixel.red + "," + pixel.green + ","
                                + pixel.blue + "," + pixel.alpha);
                    }
                    postStatus("C12.4 surface pixel " + result.pixels.size() + "개 캡처 완료");
                });
    }

    private boolean logC131StackingDescendantClipFixtureReport(String report) {
        if (!report.startsWith("status=0 ") || !report.contains(" layout=ready ")
                || !"13".equals(reportField(report, "boxes="))
                || !"540x128".equals(reportField(report, "render_viewport_css_px="))) {
            Log.e(TAG, "SPINON_C131_STACKING_CLIP_SUMMARY status=error stage=runtime-report "
                    + runtimeStatusSummary(report));
            return false;
        }
        int frameMarker = report.indexOf("node_frames_css_px=[");
        int frameStart = frameMarker < 0 ? -1 : report.indexOf('[', frameMarker);
        int frameEnd = frameStart < 0 ? -1 : report.indexOf(']', frameStart + 1);
        if (frameStart < 0 || frameEnd < 0) {
            Log.e(TAG, "SPINON_C131_STACKING_CLIP_SUMMARY status=error stage=frame-report");
            return false;
        }
        String[] frames = report.substring(frameStart + 1, frameEnd).split(";");
        double[] expectedFrameGeometry = {
                0, 0, 540, 128,
                20, 28, 120, 72, 76, 40, 80, 48, 108, 40, 72, 48, 148, 40, 16, 48,
                190, 28, 120, 72, 246, 40, 80, 48, 278, 40, 72, 48, 318, 40, 16, 48,
                360, 28, 120, 72, 416, 40, 80, 48, 448, 40, 72, 48, 488, 40, 16, 48,
        };
        Set<String> nodeIds = new HashSet<>();
        boolean geometryValid = frames.length == 13;
        for (int index = 0; index < frames.length; index++) {
            if (index >= 13) {
                geometryValid = false;
                continue;
            }
            String[] fields = frames[index].split(",");
            if (fields.length != 5 || !fields[0].startsWith(index + ":node=")) {
                geometryValid = false;
                continue;
            }
            String nodeId = fields[0].substring((index + ":node=").length());
            geometryValid &= !nodeId.isEmpty() && nodeIds.add(nodeId);
            for (int fieldIndex = 1; fieldIndex < fields.length; fieldIndex++) {
                String[] pair = fields[fieldIndex].split("=", -1);
                if (pair.length != 2 || !Double.isFinite(parseDoubleOrNaN(pair[1]))) {
                    geometryValid = false;
                    continue;
                }
                double actual = parseDoubleOrNaN(pair[1]);
                double expected = expectedFrameGeometry[index * 4 + fieldIndex - 1];
                geometryValid &= Math.abs(actual - expected) <= 0.5;
            }
            Log.i(TAG, "SPINON_C131_STACKING_CLIP_NODE_FRAME " + frames[index]);
        }
        if (!geometryValid || nodeIds.size() != 13) {
            Log.e(TAG, "SPINON_C131_STACKING_CLIP_GEOMETRY_SUMMARY status=error frames="
                    + frames.length + " expected=13 unique_nodes=" + nodeIds.size());
            return false;
        }

        String marker = "hit_test_samples_json=";
        int markerIndex = report.indexOf(marker);
        int sampleStart = markerIndex < 0 ? -1 : report.indexOf('[', markerIndex + marker.length());
        int sampleEnd = sampleStart < 0 ? -1 : findJsonArrayEnd(report, sampleStart);
        if (sampleStart < 0 || sampleEnd < 0) {
            Log.e(TAG, "SPINON_C131_STACKING_CLIP_SUMMARY status=error stage=hit-report");
            return false;
        }
        final JSONArray observations;
        try {
            observations = new JSONArray(report.substring(sampleStart, sampleEnd));
        } catch (Exception error) {
            Log.e(TAG, "SPINON_C131_STACKING_CLIP_SUMMARY status=error stage=hit-json");
            return false;
        }
        List<SurfaceViewPixelCapture.Sample> samples = new ArrayList<>(18);
        Map<String, String> expectedRgba = new HashMap<>(18);
        Set<String> uniqueIds = new HashSet<>(18);
        Set<String> uniquePixels = new HashSet<>(18);
        int matched = 0;
        boolean valid = observations.length() == 18;
        for (int index = 0; index < observations.length(); index++) {
            JSONObject sample = observations.optJSONObject(index);
            if (sample == null || index >= C131_STACKING_CLIP_SAMPLE_X.length) {
                valid = false;
                continue;
            }
            int caseIndex = index / C131_STACKING_CLIP_SAMPLE_ROLES.length;
            int roleIndex = index % C131_STACKING_CLIP_SAMPLE_ROLES.length;
            String caseId = C131_STACKING_CLIP_CASE_IDS[caseIndex];
            String role = C131_STACKING_CLIP_SAMPLE_ROLES[roleIndex];
            String id = caseId + ":" + role;
            String elementId = C131_STACKING_CLIP_HIT_IDS[index];
            double x = sample.optDouble("xCssPx", Double.NaN);
            double y = sample.optDouble("yCssPx", Double.NaN);
            int pixelX = sample.optInt("pixelX", -1);
            int pixelY = sample.optInt("pixelY", -1);
            int expectedPixelX = (int) Math.floor(C131_STACKING_CLIP_SAMPLE_X[index]);
            boolean validCoordinates = Double.isFinite(x) && Double.isFinite(y)
                    && x == C131_STACKING_CLIP_SAMPLE_X[index] && y == 52.5
                    && pixelX == expectedPixelX && pixelY == 52;
            String pixelKey = pixelX + "," + pixelY;
            String actualRgba = rgbaString(sample.optJSONArray("rgba"));
            String expectedColor = c131StackingDescendantClipExpectedRgba(elementId);
            boolean sampleMatches = id.equals(sample.optString("id", "missing"))
                    && caseId.equals(sample.optString("caseId", "missing"))
                    && role.equals(sample.optString("role", "missing"))
                    && "matched".equals(sample.optString("status", "missing"))
                    && elementId.equals(sample.optString("elementId", "missing"))
                    && sample.optInt("nodeIndex", -1) == C131_STACKING_CLIP_NODE_INDEX[index]
                    && validCoordinates && expectedColor.equals(actualRgba)
                    && uniqueIds.add(id) && uniquePixels.add(pixelKey);
            if (!sampleMatches) valid = false;
            if ("matched".equals(sample.optString("status", ""))) matched++;
            if (validCoordinates && expectedColor.equals(actualRgba)) {
                samples.add(new SurfaceViewPixelCapture.Sample(id, pixelX, pixelY));
                expectedRgba.put(id, expectedColor);
            }
            Log.i(TAG, "SPINON_C131_STACKING_CLIP_HIT_TEST id=" + id + " x=" + x
                    + " y=" + y + " element=" + sample.optString("elementId", "missing")
                    + " node_index=" + sample.optInt("nodeIndex", -1)
                    + " rgba=" + actualRgba + " status="
                    + (sampleMatches ? "matched" : "error"));
        }
        valid &= observations.length() == 18 && samples.size() == 18
                && expectedRgba.size() == 18 && uniqueIds.size() == 18
                && uniquePixels.size() == 18 && matched == 18;
        Log.i(TAG, "SPINON_C131_STACKING_CLIP_HIT_TEST_SUMMARY status="
                + (valid ? "ok" : "error") + " matched=" + matched
                + " total=" + observations.length() + " unique_ids=" + uniqueIds.size()
                + " unique_pixels=" + uniquePixels.size());
        if (!valid) {
            Log.e(TAG, "SPINON_C131_STACKING_CLIP_SUMMARY status=error frames="
                    + frames.length + " hit_samples=" + observations.length());
            return false;
        }
        if (!logC131StackingFullViewportHitIdentityGridReport(report)) {
            Log.e(TAG, "SPINON_C131_STACKING_CLIP_SUMMARY status=error stage=full-viewport-grid");
            return false;
        }
        Map<String, String> runtimeRevisions = new HashMap<>(4);
        for (String field : List.of("generation", "document_revision",
                "render_tree_revision", "style_revision")) {
            String value = reportField(report, field + "=");
            if (!isUnsignedInteger(value)) {
                Log.e(TAG, "SPINON_C131_STACKING_CLIP_SUMMARY status=error stage=revision-identity field="
                        + field + " value=" + value);
                return false;
            }
            runtimeRevisions.put(field, value);
        }
        c131StackingDescendantClipSamples = Collections.unmodifiableList(samples);
        c131StackingDescendantClipExpectedRgba = Collections.unmodifiableMap(expectedRgba);
        c131StackingDescendantClipExpectedRevisions = Collections.unmodifiableMap(runtimeRevisions);
        Log.i(TAG, "SPINON_C131_STACKING_CLIP_SUMMARY status=ok boxes=13 frames=13 "
                + "hit_samples=18 hit_matches=18 hit_grid_points=69120 hit_grid_matches=69120 "
                + "edge_controls=11/11 viewport=540x128");
        return true;
    }

    private boolean logC131StackingFullViewportHitIdentityGridReport(String report) {
        String marker = "hit_identity_grid_summary_json=";
        int markerIndex = report.indexOf(marker);
        int objectStart = markerIndex < 0 ? -1 : report.indexOf('{', markerIndex + marker.length());
        int objectEnd = objectStart < 0 ? -1 : findJsonObjectEnd(report, objectStart);
        if (markerIndex < 0 || report.indexOf(marker, markerIndex + marker.length()) >= 0
                || objectStart < 0 || objectEnd < 0) {
            Log.e(TAG, "SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY "
                    + "status=error stage=missing-or-duplicate-json");
            return false;
        }
        final JSONObject summary;
        try {
            summary = new JSONObject(report.substring(objectStart, objectEnd));
        } catch (Exception error) {
            Log.e(TAG, "SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY "
                    + "status=error stage=json");
            return false;
        }
        Set<String> expectedKeys = Set.of(
                "status", "point_count", "matched_count", "mismatch_count",
                "mismatch_preview_limit", "mismatch_preview", "edge_control_count",
                "edge_controls_matched", "expected_identity_sha256", "actual_identity_sha256",
                "oracle_sha256");
        Set<String> actualKeys = new HashSet<>();
        Iterator<String> keyIterator = summary.keys();
        while (keyIterator.hasNext()) actualKeys.add(keyIterator.next());
        String status = summary.optString("status", "missing");
        long pointCount = jsonInteger(summary, "point_count", -1);
        long matchedCount = jsonInteger(summary, "matched_count", -1);
        long mismatchCount = jsonInteger(summary, "mismatch_count", -1);
        long previewLimit = jsonInteger(summary, "mismatch_preview_limit", -1);
        long edgeControlCount = jsonInteger(summary, "edge_control_count", -1);
        long edgeControlsMatched = jsonInteger(summary, "edge_controls_matched", -1);
        String expectedSha = summary.optString("expected_identity_sha256", "");
        String actualSha = summary.optString("actual_identity_sha256", "");
        String oracleSha = summary.optString("oracle_sha256", "");
        JSONArray previews = summary.optJSONArray("mismatch_preview");
        int previewCount = previews == null ? -1 : previews.length();
        boolean countsValid = pointCount == 69120 && matchedCount >= 0
                && matchedCount <= pointCount && mismatchCount >= 0
                && mismatchCount <= pointCount && matchedCount + mismatchCount == pointCount
                && edgeControlCount == 11 && edgeControlsMatched == 11;
        boolean previewValid = previewLimit == 16 && previews != null
                && previewCount == Math.min(mismatchCount, previewLimit);
        boolean previewsValid = previewValid;
        Set<String> previewCoordinates = new HashSet<>();
        if (previews != null) {
            for (int index = 0; index < previews.length(); index++) {
                JSONObject mismatch = previews.optJSONObject(index);
                long x = mismatch == null ? -1 : jsonInteger(mismatch, "xCssPx", -1);
                long y = mismatch == null ? -1 : jsonInteger(mismatch, "yCssPx", -1);
                Object expectedNodeId = mismatch == null ? null : mismatch.opt("expectedNodeId");
                Object actualNodeId = mismatch == null ? null : mismatch.opt("actualNodeId");
                Set<String> previewKeys = new HashSet<>();
                if (mismatch != null) {
                    Iterator<String> previewKeyIterator = mismatch.keys();
                    while (previewKeyIterator.hasNext()) previewKeys.add(previewKeyIterator.next());
                }
                String coordinate = x + "," + y;
                boolean previewEntryValid = mismatch != null
                        && previewKeys.equals(Set.of("xCssPx", "yCssPx", "expectedNodeId", "actualNodeId"))
                        && x >= 0 && x < 540 && y >= 0 && y < 128
                        && expectedNodeId instanceof String
                        && List.of(C131_STACKING_FULL_VIEWPORT_NODE_IDS).contains(expectedNodeId)
                        && (actualNodeId == JSONObject.NULL
                                || (actualNodeId instanceof String && !((String) actualNodeId).isEmpty()))
                        && previewCoordinates.add(coordinate);
                if (!previewEntryValid) {
                    previewsValid = false;
                    break;
                }
            }
        }
        boolean statusValid = (mismatchCount == 0 && "matched".equals(status))
                || (mismatchCount > 0 && "mismatch".equals(status));
        boolean digestValid = C131_STACKING_FULL_VIEWPORT_IDENTITY_SHA256.equals(expectedSha)
                && isSha256Hex(actualSha)
                && (mismatchCount == 0
                        ? C131_STACKING_FULL_VIEWPORT_IDENTITY_SHA256.equals(actualSha)
                        : !C131_STACKING_FULL_VIEWPORT_IDENTITY_SHA256.equals(actualSha))
                && C131_STACKING_FULL_VIEWPORT_ORACLE_SHA256.equals(oracleSha);
        boolean valid = actualKeys.equals(expectedKeys) && countsValid && previewValid
                && previewsValid && statusValid && digestValid;
        String generation = reportField(report, "generation=");
        String markerLine = "SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_SUMMARY status="
                + (valid && mismatchCount == 0 ? "matched" : "mismatch")
                + " run_id=" + c131StackingDescendantClipRunId
                + " runtime_generation=" + generation + " points=" + pointCount
                + " matched=" + matchedCount + " mismatches=" + mismatchCount
                + " edge_controls=" + edgeControlsMatched + "/" + edgeControlCount
                + " preview=" + Math.max(0, previewCount)
                + " identity_sha256=" + actualSha + " oracle_sha256=" + oracleSha;
        if (valid && mismatchCount == 0) Log.i(TAG, markerLine);
        else Log.e(TAG, markerLine + " schema_valid=" + actualKeys.equals(expectedKeys)
                + " counts_valid=" + countsValid + " preview_valid=" + previewsValid
                + " digest_valid=" + digestValid);
        if (previews != null) {
            for (int index = 0; index < Math.min(previews.length(), 16); index++) {
                JSONObject mismatch = previews.optJSONObject(index);
                if (mismatch == null) continue;
                Object actualNodeId = mismatch.opt("actualNodeId");
                Log.e(TAG, "SPINON_C131_STACKING_FULL_VIEWPORT_HIT_IDENTITY_GRID_MISMATCH index="
                        + index + " x=" + mismatch.opt("xCssPx") + " y=" + mismatch.opt("yCssPx")
                        + " expected=" + mismatch.optString("expectedNodeId", "missing")
                        + " actual=" + (actualNodeId == JSONObject.NULL ? "null" : actualNodeId));
            }
        }
        return valid && mismatchCount == 0;
    }

    private static long jsonInteger(JSONObject object, String key, long fallback) {
        Object value = object.opt(key);
        if (!(value instanceof Integer || value instanceof Long || value instanceof Short)) return fallback;
        double asDouble = ((Number) value).doubleValue();
        long asLong = ((Number) value).longValue();
        return Double.isFinite(asDouble) && asDouble == (double) asLong ? asLong : fallback;
    }

    private static boolean isSha256Hex(String value) {
        return value != null && value.matches("[0-9a-f]{64}");
    }

    private static String c131StackingDescendantClipExpectedRgba(String elementId) {
        if ("root".equals(elementId)) return "240,244,247,255";
        if (elementId.endsWith("-parent")) return "240,198,116,255";
        if (elementId.endsWith("-target")) return "87,166,217,255";
        if (elementId.endsWith("-overlay")) return "228,81,79,255";
        return "invalid";
    }

    private static String rgbaString(JSONArray rgba) {
        if (rgba == null || rgba.length() != 4) return "invalid";
        StringBuilder result = new StringBuilder();
        for (int index = 0; index < 4; index++) {
            int channel = rgba.optInt(index, -1);
            if (channel < 0 || channel > 255) return "invalid";
            if (index > 0) result.append(',');
            result.append(channel);
        }
        return result.toString();
    }

    private static double parseDoubleOrNaN(String value) {
        try {
            double parsed = Double.parseDouble(value);
            return Double.isFinite(parsed) ? parsed : Double.NaN;
        } catch (NumberFormatException error) {
            return Double.NaN;
        }
    }

    private static int findJsonArrayEnd(String text, int start) {
        int depth = 0;
        boolean inString = false;
        boolean escaped = false;
        for (int index = start; index < text.length(); index++) {
            char current = text.charAt(index);
            if (inString) {
                if (escaped) escaped = false;
                else if (current == '\\') escaped = true;
                else if (current == '"') inString = false;
                continue;
            }
            if (current == '"') inString = true;
            else if (current == '[') depth++;
            else if (current == ']' && --depth == 0) return index + 1;
        }
        return -1;
    }

    private boolean logC131CompactSurfaceMatrixFixtureReport(String report) {
        if (!report.startsWith("status=0 ") || !report.contains(" layout=ready ")
                || !"41".equals(reportField(report, "boxes="))
                || !"300x100".equals(reportField(report, "render_viewport_css_px="))) {
            Log.e(TAG, "SPINON_C131_COMPACT_FIXTURE_SUMMARY status=error stage=runtime-report "
                    + runtimeStatusSummary(report));
            return false;
        }
        int frameMarker = report.indexOf("node_frames_css_px=[");
        int frameStart = frameMarker < 0 ? -1 : report.indexOf('[', frameMarker);
        int frameEnd = frameStart < 0 ? -1 : report.indexOf(']', frameStart + 1);
        if (frameStart < 0 || frameEnd < 0) {
            Log.e(TAG, "SPINON_C131_COMPACT_FIXTURE_SUMMARY status=error stage=frame-report");
            return false;
        }
        String[] frames = report.substring(frameStart + 1, frameEnd).split(";");
        if (frames.length != 41) {
            Log.e(TAG, "SPINON_C131_COMPACT_GEOMETRY_SUMMARY status=error frames="
                    + frames.length + " expected=41");
            return false;
        }
        for (String frame : frames) Log.i(TAG, "SPINON_C131_COMPACT_NODE_FRAME " + frame);
        Log.i(TAG, "SPINON_C131_COMPACT_GEOMETRY_SUMMARY status=ok frames=41 viewport=300x100");

        int sampleMarker = report.indexOf("hit_test_samples_json=");
        int sampleStart = sampleMarker < 0 ? -1 : report.indexOf('[', sampleMarker);
        int sampleEnd = sampleStart < 0 ? -1 : report.indexOf(']', sampleStart + 1);
        if (sampleStart < 0 || sampleEnd < 0) {
            Log.e(TAG, "SPINON_C131_COMPACT_FIXTURE_SUMMARY status=error stage=hit-report");
            return false;
        }
        final JSONArray observations;
        try {
            observations = new JSONArray(report.substring(sampleStart, sampleEnd + 1));
        } catch (Exception error) {
            Log.e(TAG, "SPINON_C131_COMPACT_FIXTURE_SUMMARY status=error stage=hit-json");
            return false;
        }
        List<SurfaceViewPixelCapture.Sample> samples = new ArrayList<>(52);
        Map<String, String> expectedRgba = new HashMap<>(52);
        Set<String> uniqueIds = new HashSet<>(52);
        Set<String> uniqueCoordinates = new HashSet<>(52);
        int matched = 0;
        boolean valid = observations.length() == 52;
        for (int index = 0; index < observations.length(); index++) {
            JSONObject observation = observations.optJSONObject(index);
            if (observation == null) {
                valid = false;
                continue;
            }
            int caseIndex = index / C131_COMPACT_SURFACE_SAMPLE_ROLES.length;
            int roleIndex = index % C131_COMPACT_SURFACE_SAMPLE_ROLES.length;
            String caseId = caseIndex < C131_COMPACT_SURFACE_CASE_IDS.length
                    ? C131_COMPACT_SURFACE_CASE_IDS[caseIndex] : "unexpected";
            String role = C131_COMPACT_SURFACE_SAMPLE_ROLES[roleIndex];
            String id = caseId + ":" + role;
            String actualId = observation.optString("id", "missing");
            String actualCaseId = observation.optString("caseId", "missing");
            String actualRole = observation.optString("role", "missing");
            String status = observation.optString("status", "missing");
            String elementId = observation.optString("elementId", "");
            int nodeIndex = observation.optInt("nodeIndex", -1);
            double x = observation.optDouble("xCssPx", Double.NaN);
            double y = observation.optDouble("yCssPx", Double.NaN);
            String rgba = observation.optString("rgba", "");
            String[] channels = rgba.split(",", -1);
            boolean validRgba = channels.length == 4;
            for (String channel : channels) {
                try {
                    int value = Integer.parseInt(channel);
                    if (value < 0 || value > 255) validRgba = false;
                } catch (NumberFormatException error) {
                    validRgba = false;
                }
            }
            boolean validCoordinate = Double.isFinite(x) && Double.isFinite(y)
                    && x == Math.rint(x) && y == Math.rint(y)
                    && x >= 0 && x < 300 && y >= 0 && y < 100;
            String coordinate = x + "," + y;
            boolean sampleMatches = id.equals(actualId) && caseId.equals(actualCaseId)
                    && role.equals(actualRole) && "matched".equals(status)
                    && !elementId.isEmpty() && nodeIndex >= 0 && validCoordinate && validRgba
                    && uniqueIds.add(actualId) && uniqueCoordinates.add(coordinate);
            if ("matched".equals(status)) matched++;
            if (!sampleMatches) valid = false;
            if (validCoordinate && validRgba) {
                samples.add(new SurfaceViewPixelCapture.Sample(actualId, x, y));
                expectedRgba.put(actualId, rgba);
            }
            Log.i(TAG, "SPINON_C131_COMPACT_HIT_TEST id=" + actualId + " x=" + x
                    + " y=" + y + " element=" + elementId + " node_index=" + nodeIndex
                    + " rgba=" + rgba + " status=" + status);
        }
        valid &= samples.size() == 52 && expectedRgba.size() == 52
                && uniqueIds.size() == 52 && uniqueCoordinates.size() == 52 && matched == 52;
        Log.i(TAG, "SPINON_C131_COMPACT_HIT_TEST_SUMMARY status="
                + (valid ? "ok" : "error") + " matched=" + matched
                + " total=" + observations.length() + " ids="
                + (uniqueIds.size() == 52 ? "fixed" : "unexpected"));
        if (!valid) {
            Log.e(TAG, "SPINON_C131_COMPACT_FIXTURE_SUMMARY status=error boxes=41 "
                    + "frames=" + frames.length + " hit_samples=" + observations.length());
            return false;
        }
        if (!logC131CompactHitIdentityGridReport(report)) {
            Log.e(TAG, "SPINON_C131_COMPACT_FIXTURE_SUMMARY status=error boxes=41 "
                    + "frames=41 hit_grid=failed");
            return false;
        }
        c131CompactSurfaceSamples = Collections.unmodifiableList(samples);
        c131CompactSurfaceExpectedRgba = Collections.unmodifiableMap(expectedRgba);
        Log.i(TAG, "SPINON_C131_COMPACT_FIXTURE_SUMMARY status=ok boxes=41 frames=41 "
                + "hit_samples=52 hit_matches=52 hit_grid_points=30000 hit_grid_matches=30000");
        return true;
    }

    private boolean logC131CompactHitIdentityGridReport(String report) {
        String marker = "hit_identity_grid_summary_json=";
        int markerIndex = report.indexOf(marker);
        int objectStart = markerIndex < 0 ? -1 : report.indexOf('{', markerIndex + marker.length());
        int objectEnd = objectStart < 0 ? -1 : findJsonObjectEnd(report, objectStart);
        if (objectStart < 0 || objectEnd < 0) {
            Log.e(TAG, "SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=error stage=missing-json");
            return false;
        }
        final JSONObject summary;
        try {
            summary = new JSONObject(report.substring(objectStart, objectEnd));
        } catch (Exception error) {
            Log.e(TAG, "SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status=error stage=json");
            return false;
        }
        String status = summary.optString("status", "missing");
        int pointCount = summary.optInt("point_count", -1);
        int matchedCount = summary.optInt("matched_count", -1);
        int mismatchCount = summary.optInt("mismatch_count", -1);
        int previewLimit = summary.optInt("mismatch_preview_limit", -1);
        String expectedSha = summary.optString("expected_identity_sha256", "");
        String actualSha = summary.optString("actual_identity_sha256", "");
        String gridOracleSha = summary.optString("grid_oracle_sha256", "");
        String chromeObservationSha = summary.optString("chrome_observation_sha256", "");
        JSONArray previews = summary.optJSONArray("mismatch_preview");
        int previewCount = previews == null ? -1 : previews.length();
        boolean countsValid = pointCount == 30000 && matchedCount >= 0
                && matchedCount <= pointCount && mismatchCount >= 0
                && mismatchCount <= pointCount && matchedCount + mismatchCount == pointCount;
        boolean previewEntriesValid = previews != null;
        if (previewEntriesValid) {
            for (int index = 0; index < previews.length(); index++) {
                JSONObject mismatch = previews.optJSONObject(index);
                Object expectedNodeId = mismatch == null ? null : mismatch.opt("expectedNodeId");
                Object actualNodeId = mismatch == null ? null : mismatch.opt("actualNodeId");
                int x = mismatch == null ? -1 : mismatch.optInt("xCssPx", -1);
                int y = mismatch == null ? -1 : mismatch.optInt("yCssPx", -1);
                if (mismatch == null || x < 0 || x >= 300 || y < 0 || y >= 100
                        || !(expectedNodeId instanceof String)
                        || ((String) expectedNodeId).isEmpty()
                        || !(actualNodeId == JSONObject.NULL || actualNodeId instanceof String)) {
                    previewEntriesValid = false;
                    break;
                }
            }
        }
        boolean previewValid = previewLimit == 16 && previewCount >= 0
                && previewCount == Math.min(mismatchCount, previewLimit) && previewEntriesValid;
        boolean statusValid = (mismatchCount == 0 && "matched".equals(status))
                || (mismatchCount > 0 && "mismatch".equals(status));
        boolean identityValid = C131_COMPACT_HIT_IDENTITY_SHA256.equals(expectedSha)
                && (mismatchCount == 0
                        ? C131_COMPACT_HIT_IDENTITY_SHA256.equals(actualSha)
                        : !C131_COMPACT_HIT_IDENTITY_SHA256.equals(actualSha))
                && C131_COMPACT_GRID_ORACLE_SHA256.equals(gridOracleSha)
                && C131_COMPACT_CHROME_OBSERVATION_SHA256.equals(chromeObservationSha);
        boolean valid = countsValid && previewValid && statusValid && identityValid;
        String generation = reportField(report, "generation=");
        String markerLine = "SPINON_C131_COMPACT_HIT_IDENTITY_GRID_SUMMARY status="
                + (valid && mismatchCount == 0 ? "matched" : "mismatch")
                + " run_id=" + c131CompactSurfaceMatrixRunId
                + " runtime_generation=" + generation + " points=" + pointCount
                + " matched=" + matchedCount + " mismatches=" + mismatchCount
                + " preview=" + Math.max(0, previewCount)
                + " identity_sha256=" + actualSha
                + " oracle_sha256=" + gridOracleSha;
        if (valid && mismatchCount == 0) Log.i(TAG, markerLine);
        else Log.e(TAG, markerLine);
        if (previews != null) {
            for (int index = 0; index < Math.min(previews.length(), 16); index++) {
                JSONObject mismatch = previews.optJSONObject(index);
                if (mismatch == null) {
                    valid = false;
                    continue;
                }
                Log.e(TAG, "SPINON_C131_COMPACT_HIT_IDENTITY_GRID_MISMATCH index=" + index
                        + " x=" + mismatch.optInt("xCssPx", -1)
                        + " y=" + mismatch.optInt("yCssPx", -1)
                        + " expected=" + mismatch.optString("expectedNodeId", "missing")
                        + " actual=" + mismatch.optString("actualNodeId", "null"));
            }
        }
        return valid && mismatchCount == 0;
    }

    private static int findJsonObjectEnd(String text, int start) {
        int depth = 0;
        boolean inString = false;
        boolean escaped = false;
        for (int index = start; index < text.length(); index++) {
            char current = text.charAt(index);
            if (inString) {
                if (escaped) escaped = false;
                else if (current == '\\') escaped = true;
                else if (current == '"') inString = false;
                continue;
            }
            if (current == '"') inString = true;
            else if (current == '{') depth++;
            else if (current == '}' && --depth == 0) return index + 1;
        }
        return -1;
    }

    private boolean logC131BorderFixtureReport(String report) {
        String[] expectedSampleIds = {
                "static-inside-clip", "static-border-strip", "static-outside-border",
                "positioned-inside-clip", "positioned-border-strip", "positioned-outside-border",
        };
        String[] expectedTargets = {
                "static-target", "static-clip", "root", "positioned-target",
                "positioned-target", "root",
        };
        int[] expectedX = {125, 128, 131, 315, 318, 321};
        int[] expectedY = {40, 40, 40, 40, 40, 40};
        int[] expectedNodeIndices = {2, 1, 0, 4, 4, 0};
        double[][] expectedFrames = {
                {0, 0, 380, 120}, {10, 10, 120, 96}, {24, 24, 140, 120},
                {200, 10, 120, 96}, {214, 24, 140, 120},
        };
        if (!report.startsWith("status=0 ") || !report.contains(" layout=ready ")
                || !"5".equals(reportField(report, "boxes="))) {
            Log.e(TAG, "SPINON_C131_BORDER_FIXTURE_SUMMARY status=error stage=runtime-report "
                    + runtimeStatusSummary(report));
            return false;
        }
        int frameMarker = report.indexOf("node_frames_css_px=[");
        int frameStart = frameMarker < 0 ? -1 : report.indexOf('[', frameMarker);
        int frameEnd = frameStart < 0 ? -1 : report.indexOf(']', frameStart + 1);
        if (frameStart < 0 || frameEnd < 0) {
            Log.e(TAG, "SPINON_C131_BORDER_FIXTURE_SUMMARY status=error stage=frame-report");
            return false;
        }
        String[] frames = report.substring(frameStart + 1, frameEnd).split(";");
        boolean geometryMatches = frames.length == expectedFrames.length;
        if (geometryMatches) {
            for (int index = 0; index < frames.length; index++) {
                double[] actual = new double[4];
                boolean[] found = new boolean[4];
                for (String field : frames[index].split(",")) {
                    String[] pair = field.split("=", 2);
                    if (pair.length != 2) continue;
                    int coordinateIndex = "x".equals(pair[0]) ? 0
                            : "y".equals(pair[0]) ? 1
                            : "width".equals(pair[0]) ? 2
                            : "height".equals(pair[0]) ? 3 : -1;
                    if (coordinateIndex < 0) continue;
                    try {
                        actual[coordinateIndex] = Double.parseDouble(pair[1]);
                        found[coordinateIndex] = Double.isFinite(actual[coordinateIndex]);
                    } catch (NumberFormatException error) {
                        found[coordinateIndex] = false;
                    }
                }
                for (int coordinate = 0; coordinate < actual.length; coordinate++) {
                    if (!found[coordinate]
                            || actual[coordinate] != expectedFrames[index][coordinate]) {
                        geometryMatches = false;
                    }
                }
                Log.i(TAG, "SPINON_C131_BORDER_NODE_FRAME " + frames[index]);
            }
        }
        Log.i(TAG, "SPINON_C131_BORDER_GEOMETRY_SUMMARY status="
                + (geometryMatches ? "ok" : "error") + " frames=" + frames.length);

        int sampleMarker = report.indexOf("hit_test_samples_json=");
        int sampleStart = sampleMarker < 0 ? -1 : report.indexOf('[', sampleMarker);
        int sampleEnd = sampleStart < 0 ? -1 : report.indexOf(']', sampleStart + 1);
        if (sampleStart < 0 || sampleEnd < 0) {
            Log.e(TAG, "SPINON_C131_BORDER_FIXTURE_SUMMARY status=error stage=hit-report");
            return false;
        }
        final JSONArray samples;
        try {
            samples = new JSONArray(report.substring(sampleStart, sampleEnd + 1));
        } catch (Exception error) {
            Log.e(TAG, "SPINON_C131_BORDER_FIXTURE_SUMMARY status=error stage=hit-json");
            return false;
        }
        boolean samplesAgree = samples.length() == expectedSampleIds.length;
        int matched = 0;
        for (int index = 0; index < samples.length(); index++) {
            JSONObject sample = samples.optJSONObject(index);
            if (sample == null) {
                samplesAgree = false;
                continue;
            }
            String id = sample.optString("id", "missing");
            String target = sample.optString("elementId", "missing");
            double x = sample.optDouble("xCssPx", Double.NaN);
            double y = sample.optDouble("yCssPx", Double.NaN);
            int nodeIndex = sample.optInt("nodeIndex", -1);
            String status = sample.optString("status", "missing");
            boolean sampleMatches = index < expectedSampleIds.length
                    && expectedSampleIds[index].equals(id)
                    && expectedTargets[index].equals(target)
                    && x == expectedX[index] && y == expectedY[index]
                    && nodeIndex == expectedNodeIndices[index]
                    && "matched".equals(status);
            if ("matched".equals(status)) matched++;
            if (!sampleMatches) samplesAgree = false;
            Log.i(TAG, "SPINON_C131_BORDER_HIT_TEST id=" + id + " x=" + x + " y=" + y
                    + " element=" + target + " node_index=" + nodeIndex + " status=" + status);
        }
        boolean passed = geometryMatches && samplesAgree && matched == expectedSampleIds.length;
        Log.i(TAG, "SPINON_C131_BORDER_HIT_TEST_SUMMARY status="
                + (passed ? "ok" : "error") + " matched=" + matched
                + " total=" + samples.length() + " ids="
                + (samplesAgree ? "fixed" : "unexpected"));
        Log.i(TAG, "SPINON_C131_BORDER_FIXTURE_SUMMARY status="
                + (passed ? "ok" : "error") + " boxes=5 frames=" + frames.length
                + " hit_samples=" + samples.length() + " hit_matches=" + matched);
        return passed;
    }

    private void requestC131BorderStripSurfaceCapture(long renderer, long generation) {
        final int width;
        final int height;
        synchronized (stateLock) {
            if (closing || c131BorderStripSurfaceCaptureStarted || !surfaceAvailable
                    || surfaceGeneration != generation || rendererHandle != renderer
                    || rendererGeneration != generation) {
                return;
            }
            c131BorderStripSurfaceCaptureStarted = true;
            width = surfaceWidth;
            height = surfaceHeight;
        }
        String runId = Long.toHexString(SystemClock.elapsedRealtimeNanos());
        List<SurfaceViewPixelCapture.Sample> samples = List.of(
                new SurfaceViewPixelCapture.Sample("static-inside-clip", 125, 40),
                new SurfaceViewPixelCapture.Sample("static-border-strip", 128, 40),
                new SurfaceViewPixelCapture.Sample("static-outside-border", 131, 40),
                new SurfaceViewPixelCapture.Sample("positioned-inside-clip", 315, 40),
                new SurfaceViewPixelCapture.Sample("positioned-border-strip", 318, 40),
                new SurfaceViewPixelCapture.Sample("positioned-outside-border", 321, 40));
        File output = new File(
                new File(activity.getCacheDir(), "c131-overflow-clip-border-surface-captures"),
                "runtime-overflow-clip-border-" + runId + ".png");
        Log.i(TAG, "SPINON_C131_BORDER_PIXELCOPY_BEGIN run_id=" + runId
                + " pid=" + android.os.Process.myPid() + " generation=" + generation
                + " width=" + width + " height=" + height + " density=" + density
                + " viewport_width=400 viewport_height=160 samples=" + samples.size());
        SurfaceViewPixelCapture.requestForCssViewport(surfaceView, width, height,
                400, 160, 0, 0, "runtime-overflow-clip-border", runId, output, samples,
                mainHandler, runtimeQueue,
                () -> {
                    synchronized (stateLock) {
                        return !closing && surfaceAvailable && surfaceGeneration == generation
                                && rendererHandle == renderer && rendererGeneration == generation
                                && surfaceWidth == width && surfaceHeight == height;
                    }
                }, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C131_BORDER_PIXELCOPY status=error run_id=" + runId
                                + " generation=" + generation + " attempts=" + result.attempts
                                + " reason=" + result.error);
                        postStatus("실패 · C13.1 border PixelCopy · " + result.error);
                        return;
                    }
                    int matched = 0;
                    for (SurfaceViewPixelCapture.Pixel pixel : result.pixels) {
                        int[] expected = c131BorderStripExpectedRgba(pixel.id);
                        boolean exact = pixel.red == expected[0] && pixel.green == expected[1]
                                && pixel.blue == expected[2] && pixel.alpha == expected[3];
                        if (exact) matched++;
                        Log.i(TAG, "SPINON_C131_BORDER_PIXEL run_id=" + runId
                                + " sample=" + pixel.id + " x=" + pixel.x + " y=" + pixel.y
                                + " rgba=" + pixel.red + "," + pixel.green + ","
                                + pixel.blue + "," + pixel.alpha + " expected="
                                + expected[0] + "," + expected[1] + "," + expected[2]
                                + "," + expected[3] + " exact=" + exact);
                    }
                    boolean passed = result.pixels.size() == samples.size()
                            && matched == samples.size();
                    Log.i(TAG, "SPINON_C131_BORDER_PIXELCOPY_SUMMARY status="
                            + (passed ? "matched" : "mismatch") + " run_id=" + runId
                            + " expected_samples=" + samples.size()
                            + " observed_samples=" + result.pixels.size()
                            + " matched_samples=" + matched + " sha256=" + result.sha256
                            + " png=" + output.getAbsolutePath());
                    if (!passed) postStatus("실패 · C13.1 border PixelCopy 표본 불일치");
                });
    }

    private static int[] c131BorderStripExpectedRgba(String sampleId) {
        if ("static-inside-clip".equals(sampleId)
                || "positioned-inside-clip".equals(sampleId)) {
            return new int[] {87, 166, 217, 255};
        }
        if ("static-border-strip".equals(sampleId)
                || "positioned-border-strip".equals(sampleId)) {
            return new int[] {240, 198, 116, 255};
        }
        return new int[] {16, 24, 39, 255};
    }

    private void requestC131StackingDescendantClipSurfaceCapture(
            long renderer, long generation, String drawReport
    ) {
        Map<String, String> expectedRevisions = c131StackingDescendantClipExpectedRevisions;
        Map<String, String> revisionExpectations = new HashMap<>(expectedRevisions);
        if (c131StackingDescendantClipStaleRevisionProbeRequested) {
            String documentRevision = revisionExpectations.get("document_revision");
            if (isUnsignedInteger(documentRevision)) {
                revisionExpectations.put("document_revision",
                        Long.toString(Long.parseLong(documentRevision) + 1));
                Log.i(TAG, "SPINON_C131_STACKING_CLIP_NEGATIVE_CONTROL"
                        + " name=stale-document-revision injected=true");
            }
        }
        Map<String, String> drawRevisions = new HashMap<>(4);
        boolean revisionIdentityMatches = drawReport.startsWith("status=0 ")
                && "13".equals(reportField(drawReport, "boxes="))
                && "540x128".equals(reportField(drawReport, "root_css_px="));
        String mismatchField = "";
        for (String field : List.of("generation", "document_revision",
                "render_tree_revision", "style_revision")) {
            String value = reportField(drawReport, field + "=");
            drawRevisions.put(field, value);
            boolean fieldMatches = isUnsignedInteger(value)
                    && value.equals(revisionExpectations.get(field));
            revisionIdentityMatches &= fieldMatches;
            if (!fieldMatches && mismatchField.isEmpty()) mismatchField = field;
        }
        if (!revisionIdentityMatches) {
            if (c131StackingDescendantClipStaleRevisionProbeRequested) {
                Log.i(TAG, "SPINON_C131_STACKING_CLIP_NEGATIVE_CONTROL"
                        + " name=stale-document-revision status="
                        + ("document_revision".equals(mismatchField) ? "rejected" : "error")
                        + " field=" + mismatchField + " surface_readback=not-requested");
            }
            Log.e(TAG, "SPINON_C131_STACKING_CLIP_PIXELCOPY status=error stage=stale-revision"
                    + " run_id=" + c131StackingDescendantClipRunId
                    + " expected=" + revisionExpectations + " actual=" + drawRevisions);
            postStatus("실패 · C13.1 PixelCopy runtime revision이 검증 snapshot과 다릅니다");
            return;
        }

        final int width;
        final int height;
        final long presentationSequence;
        synchronized (stateLock) {
            if (closing || c131StackingDescendantClipCaptureStarted || !surfaceAvailable
                    || surfaceGeneration != generation || rendererHandle != renderer
                    || rendererGeneration != generation) {
                return;
            }
            if (c131StackingDescendantClipSamples.size() != 18
                    || c131StackingDescendantClipExpectedRgba.size() != 18
                    || expectedRevisions.size() != 4) {
                Log.e(TAG, "SPINON_C131_STACKING_CLIP_PIXELCOPY status=error stage=sample-state");
                return;
            }
            c131StackingDescendantClipCaptureStarted = true;
            width = surfaceWidth;
            height = surfaceHeight;
            presentationSequence = hostPresentationSequence;
        }
        String runId = c131StackingDescendantClipRunId;
        List<SurfaceViewPixelCapture.Sample> samples = c131StackingDescendantClipSamples;
        Map<String, String> expectedRgba = c131StackingDescendantClipExpectedRgba;
        File output = new File(
                new File(activity.getCacheDir(), "c131-stacking-descendant-clip-captures"),
                "stacking-descendant-clip-" + runId + ".png");
        Log.i(TAG, "SPINON_C131_STACKING_CLIP_PIXELCOPY_BEGIN run_id=" + runId
                + " pid=" + android.os.Process.myPid() + " surface_generation=" + generation
                + " presentation_sequence=" + presentationSequence
                + " draw_generation=" + drawRevisions.get("generation")
                + " document_revision=" + drawRevisions.get("document_revision")
                + " render_tree_revision=" + drawRevisions.get("render_tree_revision")
                + " style_revision=" + drawRevisions.get("style_revision")
                + " width=" + width + " height=" + height + " viewport=540x128"
                + " samples=" + samples.size());
        SurfaceViewPixelCapture.requestForCssViewport(surfaceView, width, height,
                540, 128, 0, 0, "runtime-stacking-descendant-clip", runId, output,
                samples, mainHandler, runtimeQueue,
                () -> {
                    synchronized (stateLock) {
                        return !closing && surfaceAvailable && surfaceGeneration == generation
                                && rendererHandle == renderer && rendererGeneration == generation
                                && surfaceWidth == width && surfaceHeight == height
                                && hostPresentationSequence == presentationSequence;
                    }
                }, result -> {
                    synchronized (stateLock) {
                        if (closing || !surfaceAvailable || surfaceGeneration != generation
                                || rendererHandle != renderer || rendererGeneration != generation
                                || surfaceWidth != width || surfaceHeight != height
                                || hostPresentationSequence != presentationSequence) {
                            boolean artifactDeleted = result.error.isEmpty() && output.delete();
                            Log.e(TAG, "SPINON_C131_STACKING_CLIP_PIXELCOPY status=error"
                                    + " run_id=" + runId + " stage=stale-callback"
                                    + " surface_generation=" + generation
                                    + " presentation_sequence=" + presentationSequence
                                    + " artifact_deleted=" + artifactDeleted);
                            return;
                        }
                    }
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C131_STACKING_CLIP_PIXELCOPY status=error run_id="
                                + runId + " surface_generation=" + generation + " attempts="
                                + result.attempts + " reason=" + result.error);
                        postStatus("실패 · C13.1 stacking clip PixelCopy · " + result.error);
                        return;
                    }
                    int matched = 0;
                    for (SurfaceViewPixelCapture.Pixel pixel : result.pixels) {
                        String expected = expectedRgba.get(pixel.id);
                        String actual = pixel.red + "," + pixel.green + ","
                                + pixel.blue + "," + pixel.alpha;
                        boolean exact = actual.equals(expected);
                        if (exact) matched++;
                        Log.i(TAG, "SPINON_C131_STACKING_CLIP_PIXEL run_id=" + runId
                                + " sample=" + pixel.id + " x=" + pixel.x + " y=" + pixel.y
                                + " rgba=" + actual + " expected=" + expected
                                + " exact=" + exact);
                    }
                    boolean valid = result.pixels.size() == 18 && matched == 18;
                    Log.i(TAG, "SPINON_C131_STACKING_CLIP_PIXELCOPY_SUMMARY status="
                            + (valid ? "matched" : "mismatch") + " run_id=" + runId
                            + " pid=" + android.os.Process.myPid()
                            + " surface_generation=" + generation
                            + " presentation_sequence=" + presentationSequence
                            + " draw_generation=" + drawRevisions.get("generation")
                            + " document_revision=" + drawRevisions.get("document_revision")
                            + " render_tree_revision=" + drawRevisions.get("render_tree_revision")
                            + " style_revision=" + drawRevisions.get("style_revision")
                            + " samples=" + matched + "/18" + " png="
                            + output.getAbsolutePath() + " sha256=" + result.sha256);
                    if (valid) postStatus("완료 · C13.1 stacking clip PixelCopy 18/18");
                    else postStatus("실패 · C13.1 stacking clip PixelCopy 표본 불일치");
                });
    }

    private void requestC131CompactSurfaceMatrixCapture(long renderer, long generation) {
        final int width;
        final int height;
        synchronized (stateLock) {
            if (closing || c131CompactSurfaceMatrixCaptureStarted || !surfaceAvailable
                    || surfaceGeneration != generation || rendererHandle != renderer
                    || rendererGeneration != generation) {
                return;
            }
            if (c131CompactSurfaceSamples.size() != 52
                    || c131CompactSurfaceExpectedRgba.size() != 52) {
                Log.e(TAG, "SPINON_C131_COMPACT_PIXELCOPY status=error stage=sample-state");
                postStatus("실패 · C13.1 compact 표본이 준비되지 않았습니다");
                return;
            }
            c131CompactSurfaceMatrixCaptureStarted = true;
            width = surfaceWidth;
            height = surfaceHeight;
        }
        double scaleX = width / 300.0;
        double scaleY = height / 100.0;
        if (width <= 0 || height <= 0 || Math.abs(scaleX - scaleY) > 0.01) {
            Log.e(TAG, "SPINON_C131_COMPACT_PIXELCOPY status=error width=" + width
                    + " height=" + height + " scale_x=" + scaleX + " scale_y=" + scaleY);
            postStatus("실패 · C13.1 compact 표면 비율이 300×100과 다릅니다");
            return;
        }
        String runId = c131CompactSurfaceMatrixRunId;
        List<SurfaceViewPixelCapture.Sample> samples = c131CompactSurfaceSamples;
        Map<String, String> expectedRgba = c131CompactSurfaceExpectedRgba;
        File output = new File(
                new File(activity.getCacheDir(), "c131-compact-surface-matrix-captures"),
                "runtime-compact-matrix-" + runId + ".png");
        Log.i(TAG, "SPINON_C131_COMPACT_PIXELCOPY_BEGIN run_id=" + runId
                + " pid=" + android.os.Process.myPid() + " generation=" + generation
                + " width=" + width + " height=" + height + " css_width=300 css_height=100"
                + " scale_x=" + scaleX + " scale_y=" + scaleY + " samples=" + samples.size());
        SurfaceViewPixelCapture.requestForCssViewport(surfaceView, width, height, 300, 100,
                0, 0, "runtime-c13-1-compact-surface", runId, output, samples,
                mainHandler, runtimeQueue,
                () -> {
                    synchronized (stateLock) {
                        return !closing && surfaceAvailable && surfaceGeneration == generation
                                && rendererHandle == renderer && rendererGeneration == generation
                                && surfaceWidth == width && surfaceHeight == height;
                    }
                }, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C131_COMPACT_PIXELCOPY status=error run_id=" + runId
                                + " generation=" + generation + " attempts=" + result.attempts
                                + " reason=" + result.error);
                        postStatus("실패 · C13.1 compact PixelCopy · " + result.error);
                        return;
                    }
                    int matched = 0;
                    for (SurfaceViewPixelCapture.Pixel pixel : result.pixels) {
                        String expectedText = expectedRgba.get(pixel.id);
                        int[] expected = parseC131CompactRgba(expectedText);
                        boolean exact = expected != null && pixel.red == expected[0]
                                && pixel.green == expected[1] && pixel.blue == expected[2]
                                && pixel.alpha == expected[3];
                        if (exact) matched++;
                        Log.i(TAG, "SPINON_C131_COMPACT_PIXEL run_id=" + runId
                                + " sample=" + pixel.id + " x_px=" + pixel.x + " y_px=" + pixel.y
                                + " rgba=" + pixel.red + "," + pixel.green + ","
                                + pixel.blue + "," + pixel.alpha + " expected="
                                + (expectedText == null ? "missing" : expectedText)
                                + " exact=" + exact);
                    }
                    boolean passed = result.width / 300.0 / (result.height / 100.0) > 0.99
                            && result.width / 300.0 / (result.height / 100.0) < 1.01
                            && result.pixels.size() == 52 && matched == 52;
                    Log.i(TAG, "SPINON_C131_COMPACT_PIXELCOPY_SUMMARY status="
                            + (passed ? "matched" : "mismatch") + " run_id=" + runId
                            + " width=" + result.width + " height=" + result.height
                            + " scale_x=" + result.width / 300.0
                            + " scale_y=" + result.height / 100.0
                            + " expected_samples=52 observed_samples=" + result.pixels.size()
                            + " matched_samples=" + matched + " attempts=" + result.attempts
                            + " sha256=" + result.sha256 + " png=" + output.getAbsolutePath());
                    if (!passed) postStatus("실패 · C13.1 compact PixelCopy 표본 불일치");
                });
    }

    private static int[] parseC131CompactRgba(String rgba) {
        if (rgba == null) return null;
        String[] channels = rgba.split(",", -1);
        if (channels.length != 4) return null;
        int[] values = new int[4];
        try {
            for (int index = 0; index < values.length; index++) {
                values[index] = Integer.parseInt(channels[index]);
                if (values[index] < 0 || values[index] > 255) return null;
            }
        } catch (NumberFormatException error) {
            return null;
        }
        return values;
    }

    private void requestC132VisibleOverflowSurfaceCapture(
            long renderer,
            long generation,
            String drawReport
    ) {
        final int width;
        final int height;
        final long presentationSequence;
        synchronized (stateLock) {
            if (closing || !c132ScrollAssertionsPassed || c132SurfacePixelCaptureStarted
                    || !surfaceAvailable || surfaceGeneration != generation
                    || rendererHandle != renderer || rendererGeneration != generation) {
                return;
            }
            c132SurfacePixelCaptureStarted = true;
            width = surfaceWidth;
            height = surfaceHeight;
            presentationSequence = hostPresentationSequence;
        }
        String drawGeneration = reportField(drawReport, "generation=");
        String documentRevision = reportField(drawReport, "document_revision=");
        String renderTreeRevision = reportField(drawReport, "render_tree_revision=");
        String styleRevision = reportField(drawReport, "style_revision=");
        String environmentRevision = reportField(drawReport, "environment_revision=");
        String scrollRevision = reportField(drawReport, "scroll_state_revision=");
        String reportedSurface = reportField(drawReport, "surface_texture=");
        double scaleX = width / 400.0;
        double scaleY = height / 240.0;
        if (!"26".equals(reportField(drawReport, "boxes="))
                || !"400x240".equals(reportField(drawReport, "viewport_css_px="))
                || !reportedSurface.equals(width + "x" + height)
                || !isUnsignedInteger(drawGeneration)
                || !isUnsignedInteger(documentRevision)
                || !isUnsignedInteger(renderTreeRevision)
                || !isUnsignedInteger(styleRevision)
                || !isUnsignedInteger(environmentRevision)
                || !isUnsignedInteger(scrollRevision)
                || width <= 0 || height <= 0 || Math.abs(scaleX - scaleY) > 0.01) {
            Log.e(TAG, "SPINON_C132_PIXELCOPY status=error run_id=" + c132SurfacePixelRunId
                    + " stage=draw-identity boxes=" + reportField(drawReport, "boxes=")
                    + " surface=" + reportedSurface + " expected_surface=" + width + "x" + height
                    + " viewport=400x240");
            postStatus("실패 · C13.2 PixelCopy draw identity/표면 크기 불일치");
            return;
        }
        List<SurfaceViewPixelCapture.Sample> samples = List.of(
                new SurfaceViewPixelCapture.Sample("visible-escape", 150, 30),
                new SurfaceViewPixelCapture.Sample("parent-gap", 136, 30),
                new SurfaceViewPixelCapture.Sample("parent-interior", 20, 20),
                new SurfaceViewPixelCapture.Sample("parent-border", 11, 11),
                new SurfaceViewPixelCapture.Sample("root-background", 50, 110));
        File output = new File(
                new File(activity.getCacheDir(), "c132-visible-overflow-surface-pixels"),
                "surface-" + c132SurfacePixelRunId + ".png");
        int processId = android.os.Process.myPid();
        Log.i(TAG, "SPINON_C132_PIXELCOPY_BEGIN run_id=" + c132SurfacePixelRunId
                + " pid=" + processId + " surface_generation=" + generation
                + " presentation_sequence=" + presentationSequence
                + " draw_generation=" + drawGeneration + " boxes=26"
                + " document_revision=" + documentRevision
                + " render_tree_revision=" + renderTreeRevision
                + " style_revision=" + styleRevision
                + " environment_revision=" + environmentRevision
                + " scroll_state_revision=" + scrollRevision
                + " viewport=400x240 surface=" + width + "x" + height
                + " scale_x=" + scaleX + " scale_y=" + scaleY + " samples=" + samples.size());
        SurfaceViewPixelCapture.requestForCssViewport(surfaceView, width, height, 400, 240,
                0, 0, "runtime-c13-2-visible-overflow", c132SurfacePixelRunId, output,
                samples, mainHandler, runtimeQueue,
                () -> {
                    synchronized (stateLock) {
                        return !closing && surfaceAvailable && surfaceGeneration == generation
                                && rendererHandle == renderer && rendererGeneration == generation
                                && surfaceWidth == width && surfaceHeight == height
                                && hostPresentationSequence == presentationSequence;
                    }
                }, result -> {
                    synchronized (stateLock) {
                        if (closing || !surfaceAvailable || surfaceGeneration != generation
                                || rendererHandle != renderer || rendererGeneration != generation
                                || surfaceWidth != width || surfaceHeight != height
                                || hostPresentationSequence != presentationSequence) {
                            boolean artifactDeleted = result.error.isEmpty() && output.delete();
                            Log.e(TAG, "SPINON_C132_PIXELCOPY status=error run_id="
                                    + c132SurfacePixelRunId + " stage=stale-callback"
                                    + " surface_generation=" + generation
                                    + " artifact_deleted=" + artifactDeleted);
                            return;
                        }
                    }
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C132_PIXELCOPY status=error run_id="
                                + c132SurfacePixelRunId + " pid=" + processId
                                + " surface_generation=" + generation
                                + " attempts=" + result.attempts + " reason=" + result.error);
                        postStatus("실패 · C13.2 PixelCopy · " + result.error);
                        return;
                    }
                    int matched = 0;
                    boolean escapeOutsideParent = false;
                    for (SurfaceViewPixelCapture.Pixel pixel : result.pixels) {
                        int[] expected = c132SurfacePixelExpectedRgba(pixel.id);
                        boolean coordinateExact = pixel.x == (int) Math.floor(
                                (c132SurfacePixelCssX(pixel.id) + 0.5) * scaleX)
                                && pixel.y == (int) Math.floor(
                                        (c132SurfacePixelCssY(pixel.id) + 0.5) * scaleY);
                        boolean exact = expected != null && coordinateExact
                                && pixel.red == expected[0] && pixel.green == expected[1]
                                && pixel.blue == expected[2] && pixel.alpha == expected[3];
                        if (exact) matched++;
                        if ("visible-escape".equals(pixel.id)) {
                            escapeOutsideParent = pixel.x + 0.5 > 130.0 * scaleX;
                        }
                        Log.i(TAG, "SPINON_C132_PIXEL run_id=" + c132SurfacePixelRunId
                                + " pid=" + processId + " surface_generation=" + generation
                                + " sample=" + pixel.id + " x_px=" + pixel.x
                                + " y_px=" + pixel.y + " rgba=" + pixel.red + ","
                                + pixel.green + "," + pixel.blue + "," + pixel.alpha
                                + " expected=" + rgbaText(expected)
                                + " coordinate_exact=" + coordinateExact + " exact=" + exact);
                    }
                    boolean passed = result.width == width && result.height == height
                            && result.pixels.size() == samples.size()
                            && matched == samples.size() && escapeOutsideParent;
                    Log.i(TAG, "SPINON_C132_PIXELCOPY_SUMMARY status="
                            + (passed ? "matched" : "mismatch") + " run_id="
                            + c132SurfacePixelRunId + " pid=" + processId
                            + " surface_generation=" + generation
                            + " presentation_sequence=" + presentationSequence
                            + " draw_generation="
                            + drawGeneration + " boxes=26 document_revision=" + documentRevision
                            + " render_tree_revision=" + renderTreeRevision + " style_revision="
                            + styleRevision + " environment_revision=" + environmentRevision
                            + " scroll_state_revision=" + scrollRevision + " viewport=400x240"
                            + " surface=" + result.width + "x" + result.height
                            + " scale_x=" + scaleX + " scale_y=" + scaleY
                            + " expected_samples=" + samples.size()
                            + " observed_samples=" + result.pixels.size()
                            + " matched_samples=" + matched
                            + " escape_outside_parent=" + escapeOutsideParent
                            + " attempts=" + result.attempts + " sha256=" + result.sha256
                            + " png=" + output.getAbsolutePath());
                    if (passed) postStatus("C13.2 PixelCopy 표면 표본 통과 · 5/5");
                    else postStatus("실패 · C13.2 PixelCopy 표면 표본 불일치");
                });
    }

    private static int c132SurfacePixelCssX(String id) {
        switch (id) {
            case "visible-escape": return 150;
            case "parent-gap": return 136;
            case "parent-interior": return 20;
            case "parent-border": return 11;
            case "root-background": return 50;
            default: return -1;
        }
    }

    private static int c132SurfacePixelCssY(String id) {
        switch (id) {
            case "visible-escape":
            case "parent-gap": return 30;
            case "parent-interior": return 20;
            case "parent-border": return 11;
            case "root-background": return 110;
            default: return -1;
        }
    }

    private static int[] c132SurfacePixelExpectedRgba(String id) {
        switch (id) {
            case "visible-escape": return new int[] {227, 84, 78, 255};
            case "parent-gap":
            case "root-background": return new int[] {16, 24, 39, 255};
            case "parent-interior": return new int[] {52, 66, 86, 255};
            case "parent-border": return new int[] {240, 198, 116, 255};
            default: return null;
        }
    }

    private static boolean isUnsignedInteger(String value) {
        if (value.isEmpty()) return false;
        for (int index = 0; index < value.length(); index++) {
            if (!Character.isDigit(value.charAt(index))) return false;
        }
        return true;
    }

    private static String rgbaText(int[] rgba) {
        return rgba == null ? "missing"
                : rgba[0] + "," + rgba[1] + "," + rgba[2] + "," + rgba[3];
    }

    private void requestC131OverflowClipSurfaceCapture(long renderer, long generation) {
        final int width;
        final int height;
        synchronized (stateLock) {
            if (closing || c131OverflowClipSurfaceCaptureStarted || !surfaceAvailable
                    || surfaceGeneration != generation || rendererHandle != renderer
                    || rendererGeneration != generation) {
                return;
            }
            c131OverflowClipSurfaceCaptureStarted = true;
            width = surfaceWidth;
            height = surfaceHeight;
        }
        String runId = Long.toHexString(SystemClock.elapsedRealtimeNanos());
        List<SurfaceViewPixelCapture.Sample> samples = List.of(
                new SurfaceViewPixelCapture.Sample("visible-target", 102, 20),
                new SurfaceViewPixelCapture.Sample("clip-inside", 180, 20),
                new SurfaceViewPixelCapture.Sample("clip-outside", 198, 20),
                new SurfaceViewPixelCapture.Sample("clip-margin-expanded", 291, 20),
                new SurfaceViewPixelCapture.Sample("clip-margin-outside", 297, 20),
                new SurfaceViewPixelCapture.Sample("nested-clip-inside", 84, 76),
                new SurfaceViewPixelCapture.Sample("nested-clip-outside", 95, 76),
                new SurfaceViewPixelCapture.Sample("nested-y-visible", 45, 92),
                new SurfaceViewPixelCapture.Sample("nested-y-clip-outside", 45, 97),
                new SurfaceViewPixelCapture.Sample("positioned-escape", 230, 76),
                new SurfaceViewPixelCapture.Sample("outer-clip-outside", 295, 76));
        File output = new File(
                new File(activity.getCacheDir(), "c131-overflow-clip-surface-captures"),
                "runtime-overflow-clip-" + runId + ".png");
        Log.i(TAG, "SPINON_C131_PIXELCOPY_BEGIN run_id=" + runId
                + " pid=" + android.os.Process.myPid()
                + " generation=" + generation + " width=" + width + " height=" + height
                + " density=" + density + " samples=" + samples.size());
        SurfaceViewPixelCapture.request(surfaceView, width, height, density,
                0, 0, "runtime-overflow-clip", runId, output, samples,
                mainHandler, runtimeQueue,
                () -> {
                    synchronized (stateLock) {
                        return !closing && surfaceAvailable && surfaceGeneration == generation
                                && rendererHandle == renderer && rendererGeneration == generation
                                && surfaceWidth == width && surfaceHeight == height;
                    }
                }, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C131_PIXELCOPY status=error run_id=" + runId
                                + " generation=" + generation + " attempts=" + result.attempts
                                + " reason=" + result.error);
                        postStatus("실패 · C13.1 PixelCopy · " + result.error);
                        return;
                    }
                    Log.i(TAG, "SPINON_C131_PIXELCOPY status=ok run_id=" + runId
                            + " generation=" + generation + " width=" + result.width
                            + " height=" + result.height + " density=" + density
                            + " attempts=" + result.attempts + " png=" + output.getAbsolutePath()
                            + " sha256=" + result.sha256);
                    int matchedSamples = 0;
                    for (SurfaceViewPixelCapture.Pixel pixel : result.pixels) {
                        int[] expected = c131OverflowClipExpectedRgba(pixel.id);
                        boolean exact = pixel.red == expected[0] && pixel.green == expected[1]
                                && pixel.blue == expected[2] && pixel.alpha == expected[3];
                        if (exact) matchedSamples++;
                        Log.i(TAG, "SPINON_C131_PIXEL run_id=" + runId
                                + " sample=" + pixel.id + " x=" + pixel.x + " y=" + pixel.y
                                + " rgba=" + pixel.red + "," + pixel.green + ","
                                + pixel.blue + "," + pixel.alpha
                                + " expected=" + expected[0] + "," + expected[1] + ","
                                + expected[2] + "," + expected[3] + " exact=" + exact);
                    }
                    boolean matched = result.pixels.size() == samples.size()
                            && matchedSamples == samples.size();
                    Log.i(TAG, "SPINON_C131_PIXELCOPY_SUMMARY status="
                            + (matched ? "matched" : "mismatch") + " run_id=" + runId
                            + " expected_samples=" + samples.size()
                            + " observed_samples=" + result.pixels.size()
                            + " matched_samples=" + matchedSamples);
                    if (!matched) postStatus("실패 · C13.1 PixelCopy 표본 불일치");
                });
    }

    private static int[] c131OverflowClipExpectedRgba(String sampleId) {
        if ("visible-target".equals(sampleId) || "clip-inside".equals(sampleId)
                || "clip-margin-expanded".equals(sampleId)) {
            return new int[] {87, 166, 217, 255};
        }
        if ("nested-clip-inside".equals(sampleId) || "nested-y-visible".equals(sampleId)
                || "positioned-escape".equals(sampleId)) {
            return new int[] {227, 84, 78, 255};
        }
        if ("nested-y-clip-outside".equals(sampleId)) {
            return new int[] {16, 24, 39, 255};
        }
        if ("nested-clip-outside".equals(sampleId)) {
            return new int[] {52, 66, 86, 255};
        }
        return new int[] {16, 24, 39, 255};
    }

    private static List<SurfaceViewPixelCapture.Sample> parseC11GridPixelMatrixSamples(
            String json
    ) throws Exception {
        JSONArray records = new JSONArray(json);
        if (records.length() < 1 || records.length() > 64) return List.of();
        List<SurfaceViewPixelCapture.Sample> samples = new ArrayList<>(records.length());
        Set<String> ids = new HashSet<>();
        Set<String> coordinates = new HashSet<>();
        for (int index = 0; index < records.length(); index++) {
            JSONObject record = records.getJSONObject(index);
            String id = record.getString("id");
            String targetNodeId = "";
            if (record.has("nodeId")) {
                Object nodeIdValue = record.get("nodeId");
                if (!(nodeIdValue instanceof String)) return List.of();
                targetNodeId = (String) nodeIdValue;
                if (!targetNodeId.matches("[a-z0-9-]{1,80}")) return List.of();
            }
            Object xValue = record.get("xCssPx");
            Object yValue = record.get("yCssPx");
            if (!(xValue instanceof Number) || !(yValue instanceof Number)) {
                return List.of();
            }
            Number xNumber = (Number) xValue;
            Number yNumber = (Number) yValue;
            double x = xNumber.doubleValue();
            double y = yNumber.doubleValue();
            if (!id.matches("[a-z0-9-]{1,80}") || !Double.isFinite(x) || !Double.isFinite(y)
                    || x != Math.rint(x) || y != Math.rint(y)
                    || x < 0 || x >= 320 || y < 0 || y >= 240
                    || !ids.add(id) || !coordinates.add((int) x + "," + (int) y)) {
                return List.of();
            }
            samples.add(new SurfaceViewPixelCapture.Sample(id, targetNodeId, x, y));
        }
        return samples;
    }

    private void requestC11GridSurfaceCapture(long renderer, long generation) {
        final int width;
        final int height;
        synchronized (stateLock) {
            if (closing || c11GridSurfaceCaptureStarted || !surfaceAvailable
                    || surfaceGeneration != generation || rendererHandle != renderer
                    || rendererGeneration != generation) {
                return;
            }
            c11GridSurfaceCaptureStarted = true;
            width = surfaceWidth;
            height = surfaceHeight;
        }
        List<SurfaceViewPixelCapture.Sample> samples = List.of(
                new SurfaceViewPixelCapture.Sample("grid-background", 300, 120),
                new SurfaceViewPixelCapture.Sample("item-blue", 20, 10),
                new SurfaceViewPixelCapture.Sample("item-red", 113, 12),
                new SurfaceViewPixelCapture.Sample("item-green", 226, 13),
                new SurfaceViewPixelCapture.Sample("item-yellow", 15, 65),
                new SurfaceViewPixelCapture.Sample("column-gap", 84, 10),
                new SurfaceViewPixelCapture.Sample("row-gap", 10, 53));
        String runId = Long.toHexString(SystemClock.elapsedRealtimeNanos());
        File output = new File(
                new File(activity.getCacheDir(), "c11-grid-surface-captures"),
                "fixed-tracks-gap-" + runId + ".png");
        SurfaceViewPixelCapture.request(surfaceView, width, height, density,
                0, 0, "fixed-tracks-gap", runId, output, samples,
                mainHandler, runtimeQueue,
                () -> {
                    synchronized (stateLock) {
                        return !closing && surfaceAvailable && surfaceGeneration == generation
                                && rendererHandle == renderer && rendererGeneration == generation
                                && surfaceWidth == width && surfaceHeight == height;
                    }
                }, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C111_GRID_SURFACE_CAPTURE status=error run_id="
                                + runId + " reason=" + result.error);
                        return;
                    }
                    Log.i(TAG, "SPINON_C111_GRID_SURFACE_CAPTURE status=ok run_id=" + runId
                            + " width=" + result.width + " height=" + result.height
                            + " density=" + density + " png=" + output.getAbsolutePath()
                            + " sha256=" + result.sha256);
                    int matchedSamples = 0;
                    for (SurfaceViewPixelCapture.Pixel pixel : result.pixels) {
                        int[] expected = c11GridExpectedRgb(pixel.id);
                        boolean exact = pixel.red == expected[0] && pixel.green == expected[1]
                                && pixel.blue == expected[2] && pixel.alpha == 255;
                        if (exact) matchedSamples++;
                        Log.i(TAG, "SPINON_C111_GRID_SURFACE_PIXEL run_id=" + runId
                                + " sample=" + pixel.id + " x=" + pixel.x + " y=" + pixel.y
                                + " rgba=" + pixel.red + "," + pixel.green + ","
                                + pixel.blue + "," + pixel.alpha + " expected="
                                + expected[0] + "," + expected[1] + "," + expected[2]
                                + ",255 exact=" + exact);
                    }
                    boolean matched = result.pixels.size() == samples.size()
                            && matchedSamples == samples.size();
                    Log.i(TAG, "SPINON_C111_GRID_SURFACE_SUMMARY status="
                            + (matched ? "matched" : "mismatch") + " run_id=" + runId
                            + " expected_samples=" + samples.size()
                            + " observed_samples=" + result.pixels.size()
                            + " matched_samples=" + matchedSamples);
                    if (!matched) {
                        postStatus("실패 · C11 Grid surface pixel 표본 불일치");
                    }
                });
    }

    private void requestC11GridPixelMatrixCapture(long renderer, long generation) {
        final int width;
        final int height;
        synchronized (stateLock) {
            if (closing || c11GridSurfaceCaptureStarted || !surfaceAvailable
                    || surfaceGeneration != generation || rendererHandle != renderer
                    || rendererGeneration != generation) {
                return;
            }
            c11GridSurfaceCaptureStarted = true;
            width = surfaceWidth;
            height = surfaceHeight;
        }
        String runId = c11GridPixelMatrixRunId;
        String sceneId = c11GridPixelMatrixSceneId;
        List<SurfaceViewPixelCapture.Sample> samples = c11GridPixelMatrixSamples;
        String marker = c114GridTrackSizingRequested ? "SPINON_C114_GRID_PIXEL_MATRIX"
                : c1133GridTemplateAreasRequested ? "SPINON_C1133_GRID_PIXEL_MATRIX"
                : c1132GridNamedLinesRequested ? "SPINON_C1132_GRID_PIXEL_MATRIX"
                : c1131GridRepeatRequested ? "SPINON_C1131_GRID_PIXEL_MATRIX"
                : c1123GridAutoPlacementRequested ? "SPINON_C1123_GRID_PIXEL_MATRIX"
                : c1122GridAutoPlacementRequested
                ? "SPINON_C1122_GRID_PIXEL_MATRIX" : "SPINON_C111_GRID_PIXEL_MATRIX";
        if (!c11GridPixelMatrixConfigError.isEmpty() || c11GridPixelMatrixSamples.isEmpty()) {
            Log.e(TAG, marker + " status=error run_id="
                    + runId + " scene=" + sceneId
                    + " reason=invalid-sample-fixture");
            return;
        }
        String cacheDirectory = c114GridTrackSizingRequested
                ? "c114-grid-surface-pixel-matrix"
                : c1133GridTemplateAreasRequested
                ? "c1133-grid-surface-pixel-matrix"
                : c1132GridNamedLinesRequested
                ? "c1132-grid-surface-pixel-matrix"
                : c1131GridRepeatRequested
                ? "c1131-grid-surface-pixel-matrix"
                : c1123GridAutoPlacementRequested ? "c1123-grid-surface-pixel-matrix"
                : c1122GridAutoPlacementRequested
                ? "c1122-grid-surface-pixel-matrix" : "c11-grid-surface-pixel-matrix";
        final int viewportWidthCssPx = 320;
        final int viewportHeightCssPx = 240;
        double scaleX = width / (double) viewportWidthCssPx;
        double scaleY = height / (double) viewportHeightCssPx;
        Log.i(TAG, marker + "_BEGIN run_id=" + runId
                + " pid=" + android.os.Process.myPid() + " scene=" + sceneId
                + " generation=" + generation + " width=" + width + " height=" + height
                + " viewport_width=" + viewportWidthCssPx
                + " viewport_height=" + viewportHeightCssPx
                + " scale_x=" + scaleX + " scale_y=" + scaleY
                + " density=" + density + " samples=" + samples.size());
        File output = new File(
                new File(activity.getCacheDir(), cacheDirectory),
                sceneId + "-" + runId + ".png");
        SurfaceViewPixelCapture.requestForCssViewport(surfaceView, width, height,
                viewportWidthCssPx, viewportHeightCssPx, 0, 0,
                sceneId, runId, output, samples,
                mainHandler, runtimeQueue,
                () -> {
                    synchronized (stateLock) {
                        return !closing && surfaceAvailable && surfaceGeneration == generation
                                && rendererHandle == renderer && rendererGeneration == generation
                                && surfaceWidth == width && surfaceHeight == height;
                    }
                }, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, marker + "_CAPTURE status=error run_id="
                                + runId + " scene=" + sceneId + " generation=" + generation
                                + " attempts=" + result.attempts + " reason=" + result.error);
                        postStatus("실패 · C11 Grid PixelCopy · " + result.error);
                        return;
                    }
                    Log.i(TAG, marker + "_CAPTURE status=ok run_id="
                            + runId + " scene=" + sceneId + " generation=" + generation
                            + " width=" + result.width + " height=" + result.height
                            + " viewport_width=" + viewportWidthCssPx
                            + " viewport_height=" + viewportHeightCssPx
                            + " scale_x=" + (result.width / (double) viewportWidthCssPx)
                            + " scale_y=" + (result.height / (double) viewportHeightCssPx)
                            + " density=" + density + " attempts=" + result.attempts
                            + " png=" + output.getAbsolutePath() + " sha256=" + result.sha256);
                    int observed = 0;
                    int matchedSamples = 0;
                    for (SurfaceViewPixelCapture.Pixel pixel : result.pixels) {
                        SurfaceViewPixelCapture.Sample sample = null;
                        for (SurfaceViewPixelCapture.Sample candidate : samples) {
                            if (candidate.id.equals(pixel.id)) {
                                sample = candidate;
                                break;
                            }
                        }
                        if (sample == null) {
                            Log.e(TAG, marker + "_SAMPLE status=error run_id="
                                    + runId + " scene=" + sceneId + " sample=" + pixel.id
                                    + " reason=unknown-sample");
                            continue;
                        }
                        observed++;
                        boolean verifyExactColor = c1131GridRepeatRequested
                                || c1132GridNamedLinesRequested || c1133GridTemplateAreasRequested
                                || c114GridTrackSizingRequested;
                        int[] expected = !verifyExactColor ? null
                                : c114GridTrackSizingRequested
                                ? c114GridExpectedRgb(pixel.id)
                                : c11GridExpectedRgb(pixel.id, c1133GridTemplateAreasRequested);
                        boolean exact = !verifyExactColor
                                || (pixel.red == expected[0] && pixel.green == expected[1]
                                        && pixel.blue == expected[2] && pixel.alpha == 255);
                        if (exact) matchedSamples++;
                        Log.i(TAG, marker + "_SAMPLE run_id=" + runId
                                + " pid=" + android.os.Process.myPid() + " scene=" + sceneId
                                + " generation=" + generation + " sample=" + pixel.id
                                + " css_x=" + sample.xCssPx + " css_y=" + sample.yCssPx
                                + (sample.targetNodeId.isEmpty() ? ""
                                        : " target_node_id=" + sample.targetNodeId)
                                + " x=" + pixel.x + " y=" + pixel.y
                                + " rgba=" + pixel.red + "," + pixel.green + ","
                                + pixel.blue + "," + pixel.alpha
                                + (verifyExactColor
                                        ? " expected=" + expected[0] + "," + expected[1]
                                                + "," + expected[2] + ",255 exact=" + exact
                                        : ""));
                    }
                    boolean complete = observed == samples.size()
                            && result.pixels.size() == samples.size();
                    boolean matched = complete
                            && (!(c1131GridRepeatRequested || c1132GridNamedLinesRequested
                                    || c1133GridTemplateAreasRequested || c114GridTrackSizingRequested)
                                    || matchedSamples == samples.size());
                    String status = !complete ? "error"
                            : c1131GridRepeatRequested || c1132GridNamedLinesRequested
                                    || c1133GridTemplateAreasRequested || c114GridTrackSizingRequested
                            ? (matched ? "matched" : "mismatch") : "captured";
                    Log.i(TAG, marker + "_SUMMARY status="
                            + status + " run_id=" + runId
                            + " scene=" + sceneId + " generation=" + generation
                            + " expected_samples=" + samples.size()
                            + " observed_samples=" + result.pixels.size()
                            + (c1131GridRepeatRequested || c1132GridNamedLinesRequested
                                    || c1133GridTemplateAreasRequested || c114GridTrackSizingRequested
                                    ? " matched_samples=" + matchedSamples : ""));
                    if (!matched) postStatus(c114GridTrackSizingRequested
                            ? "실패 · C11.4 text overflow Grid PixelCopy 표본 불일치"
                            : c1133GridTemplateAreasRequested
                            ? "실패 · C11.3.3 Grid PixelCopy 표본 불일치"
                            : c1132GridNamedLinesRequested
                            ? "실패 · C11.3.2 Grid PixelCopy 표본 불일치"
                            : c1131GridRepeatRequested
                            ? "실패 · C11.3.1 Grid PixelCopy 표본 불일치"
                            : "실패 · C11 Grid PixelCopy 표본 누락");
                });
    }

    private static int[] c114GridExpectedRgb(String sampleId) {
        switch (sampleId) {
            case "nested-grid-internal-gap":
                return new int[] {231, 246, 237};
            case "nested-grid-inside-outer-edge":
            case "nested-grid-outside-outer-edge":
            case "nested-grid-overflow-tail":
                return new int[] {229, 72, 77};
            case "nested-grid-after-overflow":
                return new int[] {0, 0, 0};
            case "top-border-color":
                return new int[] {240, 198, 116};
            case "image-overflow-png-quadrant":
            case "image-c-overflow-green-png-texel":
                return new int[] {0, 255, 64};
            case "overflow-root-border-inside":
                return new int[] {16, 24, 32};
            case "overflow-root-border-covered":
            case "overflow-root-border-outside":
                return new int[] {229, 72, 77};
            case "overflow-after-item":
                return new int[] {0, 0, 0};
            case "overflow-root-edge-inside":
            case "grid-border":
                return new int[] {34, 197, 94};
            case "overflow-item-outside-root":
            case "item-inside-root":
            case "item-overflow-beyond-root":
            case "inside-item":
            case "right-of-grid":
            case "below-grid":
            case "second-column":
            case "second-row":
                return new int[] {229, 72, 77};
            case "overflow-root-edge-outside":
                return new int[] {0, 0, 0};
            case "gap-background":
            case "row-gap-background":
            case "root-background-control":
            case "image-multispan-root-background":
                return new int[] {16, 24, 32};
            case "image-multispan-item-overflow":
            case "image-multispan-item-below-root":
                return new int[] {229, 72, 77};
            case "image-multispan-outside-root":
            case "image-multispan-after-item":
                return new int[] {0, 0, 0};
            case "overflow-root-background":
                return new int[] {34, 197, 94};
            case "negative-root-interior":
                return new int[] {226, 232, 240};
            case "negative-root-edge":
                return new int[] {226, 232, 240};
            case "negative-cycle-root-interior":
            case "negative-cycle-root-edge":
                return new int[] {226, 232, 240};
            case "negative-root-exterior":
            case "negative-overflow-tail":
            case "negative-cycle-root-exterior":
            case "negative-cycle-overflow-tail":
                return new int[] {0, 0, 0};
            case "negative-overflow-item":
                return new int[] {141, 95, 211};
            case "negative-cycle-overflow-item":
                return new int[] {42, 157, 143};
            default:
                return new int[] {-1, -1, -1};
        }
    }

    private static int[] c11GridExpectedRgb(String sampleId, boolean c1133TemplateAreas) {
        if (c1133TemplateAreas) {
            switch (sampleId) {
                case "grid-interior":
                    return new int[] {16, 24, 32};
                case "known-area":
                    return new int[] {48, 164, 108};
                case "unknown-area-overflow":
                    return new int[] {229, 72, 77};
                default:
                    break;
            }
        }
        if (c1133TemplateAreas && "grid-background".equals(sampleId)) {
            return new int[] {16, 24, 32};
        }
        if (c1133TemplateAreas && "item-blue".equals(sampleId)) {
            return new int[] {62, 99, 221};
        }
        if ("item-blue".equals(sampleId)) return new int[] {40, 102, 246};
        if ("item-red".equals(sampleId)) return new int[] {229, 72, 77};
        if ("item-green".equals(sampleId)) return new int[] {25, 169, 116};
        if ("item-yellow".equals(sampleId)) return new int[] {255, 200, 87};
        return new int[] {237, 241, 247};
    }

    private static int[] c11GridExpectedRgb(String sampleId) {
        return c11GridExpectedRgb(sampleId, false);
    }

    private void scheduleRuntimeInitializationForReadySurface() {
        if (!rootHtmlApplicationRequested
                && !c121PositioningProbeRequested && !c122AbsoluteBlockProbeRequested
                && !c123FixedPositionProbeRequested && !c124StackingProbeRequested
                && !c1133GridTemplateAreasRequested && !c132ScrollFixtureRequested
                && !c133GestureFixtureRequested) return;
        synchronized (stateLock) {
            if (closing || hostHandle != 0 || runtimeInitializationQueued
                    || !surfaceAvailable || surfaceWidth <= 0 || surfaceHeight <= 0) {
                return;
            }
            if (rootHtmlApplicationRequested
                    && (rootHtmlApplicationSource == null || rootHtmlApplicationScript == null)) {
                return;
            }
            runtimeInitializationQueued = true;
        }
        enqueueRuntime(() -> {
            synchronized (stateLock) {
                if (closing || hostHandle != 0 || !surfaceAvailable
                        || surfaceWidth <= 0 || surfaceHeight <= 0) {
                    runtimeInitializationQueued = false;
                    return;
                }
            }
            initializeRuntime();
        });
    }

    private void evaluateCustomPropertiesFixture() {
        enqueueRuntime(() -> {
            final long host;
            synchronized (stateLock) {
                if (closing || hostHandle == 0) return;
                host = hostHandle;
            }
            String result = decode(nativeEvalCustomPropertiesFixture(host));
            Log.i(TAG, "SPINON_C051_EVAL " + result);
            postStatus(runtimeStatusSummary(result));
            if (result.startsWith("status=0 ")) requestDraw();
        });
    }

    private void evaluateRegisteredPropertiesFixture() {
        enqueueRuntime(() -> {
            final long host;
            synchronized (stateLock) {
                if (closing || hostHandle == 0) return;
                host = hostHandle;
            }
            String result = decode(nativeEvalRegisteredPropertiesFixture(host));
            Log.i(TAG, "SPINON_C052_EVAL " + result);
            postStatus(runtimeStatusSummary(result));
            if (result.startsWith("status=0 ")) requestDraw();
        });
    }

    private void evaluateRuntimeResultCacheFixture() {
        enqueueRuntime(() -> {
            final long host;
            synchronized (stateLock) {
                if (closing || hostHandle == 0) return;
                host = hostHandle;
            }
            String result = decode(nativeEvalRuntimeResultCacheFixture(host));
            Log.i(TAG, "SPINON_C053_EVAL " + result);
            postStatus(runtimeStatusSummary(result));
            if (result.startsWith("status=0 ")) requestDraw();
        });
    }

    private void evaluateIncrementalRestyleFixture() {
        enqueueRuntime(() -> {
            final long host;
            synchronized (stateLock) {
                if (closing || hostHandle == 0) return;
                host = hostHandle;
            }
            String result = decode(nativeEvalIncrementalRestyleFixture(host));
            Log.i(TAG, "SPINON_C054_EVAL " + result);
            postStatus(runtimeStatusSummary(result));
            if (result.startsWith("status=0 ")) requestDraw();
        });
    }

    private void evaluatePercentageDimensionsFixture() {
        enqueueRuntime(() -> {
            final long host;
            synchronized (stateLock) {
                if (closing || hostHandle == 0) return;
                host = hostHandle;
            }
            String result = decode(nativeEvalPercentageDimensionsFixture(host));
            Log.i(TAG, "SPINON_C061_EVAL " + result);
            postStatus(runtimeStatusSummary(result));
            if (result.startsWith("status=0 ")) requestDraw();
        });
    }

    private void evaluateSpacingPercentagesFixture() {
        enqueueRuntime(() -> {
            final long host;
            synchronized (stateLock) {
                if (closing || hostHandle == 0) return;
                host = hostHandle;
            }
            String result = decode(nativeEvalSpacingPercentagesFixture(host));
            Log.i(TAG, "SPINON_C062_EVAL " + result);
            postStatus(runtimeStatusSummary(result));
            if (result.startsWith("status=0 ")) requestDraw();
        });
    }

    private void evaluateAbsoluteLengthsFixture() {
        enqueueRuntime(() -> {
            final long host;
            synchronized (stateLock) {
                if (closing || hostHandle == 0) return;
                host = hostHandle;
            }
            String result = decode(nativeEvalAbsoluteLengthsFixture(host));
            Log.i(TAG, "SPINON_C063_EVAL " + result);
            postStatus(runtimeStatusSummary(result));
            if (result.startsWith("status=0 ")) requestDraw();
        });
    }

    private void evaluateFontRelativeUnitsFixture() {
        enqueueRuntime(() -> {
            final long host;
            synchronized (stateLock) {
                if (closing || hostHandle == 0) return;
                host = hostHandle;
            }
            String result = decode(nativeEvalFontRelativeUnitsFixture(host));
            Log.i(TAG, "SPINON_C064_EVAL " + result);
            postStatus(runtimeStatusSummary(result));
            if (result.startsWith("status=0 ")) requestDraw();
        });
    }

    private void evaluateTypedCssMathFixture() {
        enqueueRuntime(() -> {
            final long host;
            synchronized (stateLock) {
                if (closing || hostHandle == 0) return;
                host = hostHandle;
            }
            String result = decode(nativeEvalTypedCssMathFixture(host));
            Log.i(TAG, "SPINON_C065_EVAL " + result);
            postStatus(runtimeStatusSummary(result));
            if (result.startsWith("status=0 ")) requestDraw();
        });
    }

    private void evaluateViewportUnitsFixture() {
        enqueueRuntime(() -> {
            final long host;
            synchronized (stateLock) {
                if (closing || hostHandle == 0) return;
                host = hostHandle;
            }
            String result = decode(nativeEvalViewportUnitsFixture(host));
            Log.i(TAG, "SPINON_C066A_EVAL " + result);
            postStatus(runtimeStatusSummary(result));
            if (result.startsWith("status=0 ")) requestDraw();
        });
    }

    private void evaluateMinMaxSizingFixture() {
        enqueueRuntime(() -> {
            final long host;
            synchronized (stateLock) {
                if (closing || hostHandle == 0) return;
                host = hostHandle;
            }
            String result = decode(nativeEvalMinMaxSizingFixture(host));
            Log.i(TAG, "SPINON_C071_EVAL " + result);
            postStatus(runtimeStatusSummary(result));
            if (result.startsWith("status=0 ")) requestDraw();
        });
    }

    private void evaluateBorderWidthFixture() {
        enqueueRuntime(() -> {
            final long host;
            synchronized (stateLock) {
                if (closing || hostHandle == 0) return;
                host = hostHandle;
            }
            String result = decode(nativeEvalBorderWidthFixture(host));
            Log.i(TAG, "SPINON_C072_EVAL " + result);
            postStatus(runtimeStatusSummary(result));
            if (result.startsWith("status=0 ")) requestDraw();
        });
    }

    private static String runtimeStatusSummary(String report) {
        int statusEnd = report.indexOf(' ');
        if (statusEnd >= 0 && report.contains(" layout=")) {
            return report.substring(0, statusEnd)
                    + " layout=" + reportField(report, "layout=")
                    + " boxes=" + reportField(report, "boxes=")
                    + " generation=" + reportField(report, "generation=")
                    + " document_revision=" + reportField(report, "document_revision=")
                    + " environment_revision=" + reportField(report, "environment_revision=");
        }
        return report.length() <= 180 ? report : report.substring(0, 180) + "…";
    }

    private static boolean logC114GridTrackSizingTraceChunks(String runId, String report) {
        String marker = "grid_track_sizing_diagnostics=";
        int traceStart = report.indexOf(marker);
        if (traceStart < 0) {
            Log.e(TAG, "SPINON_C114_GRID_TRACE_MISSING run_id=" + runId);
            return false;
        }
        String trace = report.substring(traceStart);
        int chunkSize = 1600;
        int chunkCount = (trace.length() + chunkSize - 1) / chunkSize;
        for (int chunkIndex = 0; chunkIndex < chunkCount; chunkIndex++) {
            int start = chunkIndex * chunkSize;
            int end = Math.min(trace.length(), start + chunkSize);
            Log.i(TAG, "SPINON_C114_GRID_TRACE_CHUNK run_id=" + runId
                    + " part=" + (chunkIndex + 1) + "/" + chunkCount + " "
                    + trace.substring(start, end));
        }
        return true;
    }

    private static boolean c1133GridRevisionReusePassed(
            long host, String runId, String first, String second
    ) {
        String[] firstFrames = c1133GridNodeFrames(first);
        String[] secondFrames = c1133GridNodeFrames(second);
        boolean expected = false;
        long firstDocumentRevision = -1;
        long secondDocumentRevision = -1;
        long firstRenderRevision = -1;
        long secondRenderRevision = -1;
        long firstItemNode = -1;
        long secondItemNode = -1;
        String firstItemFrame = firstFrames.length > 1 ? firstFrames[1] : "missing";
        String secondItemFrame = secondFrames.length > 1 ? secondFrames[1] : "missing";
        try {
            firstDocumentRevision = Long.parseLong(reportField(first, "document_revision="));
            secondDocumentRevision = Long.parseLong(reportField(second, "document_revision="));
            firstRenderRevision = Long.parseLong(reportField(first, "render_tree_revision="));
            secondRenderRevision = Long.parseLong(reportField(second, "render_tree_revision="));
            boolean bothLayoutsReady = first.startsWith("status=0 ")
                    && first.contains(" layout=ready ")
                    && second.startsWith("status=0 ")
                    && second.contains(" layout=ready ");
            if (bothLayoutsReady && firstFrames.length == 2 && secondFrames.length == 2) {
                firstItemNode = c1133FrameNodeId(firstItemFrame);
                secondItemNode = c1133FrameNodeId(secondItemFrame);
                expected = firstFrames.length == 2 && secondFrames.length == 2
                    && secondDocumentRevision > firstDocumentRevision
                    && secondRenderRevision > firstRenderRevision
                    && firstItemNode == secondItemNode
                    && c1133FrameValue(firstItemFrame, "x") == 0.0f
                    && c1133FrameValue(firstItemFrame, "y") == 0.0f
                    && c1133FrameValue(firstItemFrame, "width") == 80.0f
                    && c1133FrameValue(firstItemFrame, "height") == 40.0f
                    && c1133FrameValue(secondItemFrame, "x") == 80.0f
                    && c1133FrameValue(secondItemFrame, "y") == 0.0f
                    && c1133FrameValue(secondItemFrame, "width") == 80.0f
                    && c1133FrameValue(secondItemFrame, "height") == 40.0f;
            }
        } catch (RuntimeException error) {
            Log.e(TAG, "SPINON_C1133_GRID_REVISION_PARSE_ERROR run_id=" + runId, error);
        }
        Log.i(TAG, "SPINON_C1133_GRID_REVISION_NODE_FRAME run_id=" + runId
                + " phase=before frame=" + firstItemFrame);
        Log.i(TAG, "SPINON_C1133_GRID_REVISION_NODE_FRAME run_id=" + runId
                + " phase=after frame=" + secondItemFrame);
        Log.i(TAG, "SPINON_C1133_GRID_REVISION_SUMMARY run_id=" + runId
                + " scene_id=revision-reuse host_id=" + Long.toHexString(host)
                + " same_host=true first_status=" + reportField(first, "status=")
                + " second_status=" + reportField(second, "status=")
                + " first_document_revision=" + firstDocumentRevision
                + " second_document_revision=" + secondDocumentRevision
                + " first_render_tree_revision=" + firstRenderRevision
                + " second_render_tree_revision=" + secondRenderRevision
                + " first_item_node=" + firstItemNode + " second_item_node=" + secondItemNode
                + " expected=" + expected);
        return expected;
    }

    private static String[] c1133GridNodeFrames(String report) {
        String marker = "node_frames_css_px=[";
        int start = report.indexOf(marker);
        int end = start < 0 ? -1 : report.indexOf(']', start + marker.length());
        if (start < 0 || end < 0) return new String[0];
        String value = report.substring(start + marker.length(), end);
        return value.isEmpty() ? new String[0] : value.split(";", -1);
    }

    private static long c1133FrameNodeId(String frame) {
        String marker = "node=";
        int start = frame.indexOf(marker);
        int end = frame.indexOf(',', start + marker.length());
        if (start < 0 || end < 0) throw new IllegalArgumentException("node frame ID가 없습니다");
        return Long.parseLong(frame.substring(start + marker.length(), end));
    }

    private static float c1133FrameValue(String frame, String field) {
        String marker = field + "=";
        int start = frame.indexOf(marker);
        int end = frame.indexOf(',', start + marker.length());
        if (start < 0) throw new IllegalArgumentException("node frame field가 없습니다: " + field);
        if (end < 0) end = frame.length();
        return Float.parseFloat(frame.substring(start + marker.length(), end));
    }

    private void logC121NodeFrames(String state, String report) {
        logNodeFrames("C121", state, report);
    }

    private void logGridAutoPlacementReadySummaryIfNeeded(String report) {
        if (!report.startsWith("status=0 ") || !report.contains(" layout=ready ")) return;
        String markerPrefix = c1123GridAutoPlacementRequested ? "C1123" : "C1122";
        String runId = c1123GridAutoPlacementRequested ? c1123GridRunId : c1122GridRunId;
        String sceneId = c1123GridAutoPlacementRequested ? c1123GridSceneId : c1122GridSceneId;
        Log.i(TAG, "SPINON_" + markerPrefix + "_GRID_SUMMARY run_id=" + runId
                + " scene_id=" + sceneId + " " + runtimeStatusSummary(report));
        String frameMarker = "node_frames_css_px=[";
        int start = report.indexOf(frameMarker);
        int end = start < 0 ? -1 : report.indexOf(']', start + frameMarker.length());
        if (start < 0 || end < 0) {
            Log.e(TAG, "SPINON_" + markerPrefix + "_GRID_FRAME_SUMMARY run_id=" + runId
                    + " scene_id=" + sceneId + " marker=missing");
            return;
        }
        String[] frames = report.substring(start + frameMarker.length(), end).split(";", -1);
        int count = 0;
        for (String frame : frames) {
            if (frame.isEmpty()) continue;
            Log.i(TAG, "SPINON_" + markerPrefix + "_GRID_NODE_FRAME_RUN run_id=" + runId
                    + " scene_id=" + sceneId + " frame=" + frame);
            count++;
        }
        Log.i(TAG, "SPINON_" + markerPrefix + "_GRID_FRAME_SUMMARY run_id=" + runId
                + " scene_id=" + sceneId + " count=" + count + " marker=present");
        int paintStart = report.indexOf("paint_node_order=[");
        int paintEnd = paintStart < 0 ? -1 : report.indexOf(']', paintStart);
        if (paintStart >= 0 && paintEnd > paintStart && paintEnd - paintStart <= 512) {
            Log.i(TAG, "SPINON_" + markerPrefix + "_GRID_PAINT_ORDER run_id=" + runId
                    + " scene_id=" + sceneId + " "
                    + report.substring(paintStart, paintEnd + 1));
        }
    }

    private void logC1121GridNodeFrames(String report) {
        String marker = "node_frames_css_px=[";
        int start = report.indexOf(marker);
        int end = start < 0 ? -1 : report.indexOf(']', start + marker.length());
        if (start < 0 || end < 0) {
            Log.e(TAG, "SPINON_C1121_GRID_FRAME_SUMMARY run_id=" + c1121GridRunId
                    + " scene_id=" + c1121GridSceneId + " marker=missing");
            return;
        }
        String[] frames = report.substring(start + marker.length(), end).split(";", -1);
        int count = 0;
        for (String frame : frames) {
            if (frame.isEmpty()) continue;
            Log.i(TAG, "SPINON_C1121_GRID_NODE_FRAME_RUN run_id=" + c1121GridRunId
                    + " scene_id=" + c1121GridSceneId + " frame=" + frame);
            count++;
        }
        Log.i(TAG, "SPINON_C1121_GRID_FRAME_SUMMARY run_id=" + c1121GridRunId
                + " scene_id=" + c1121GridSceneId + " count=" + count + " marker=present");
    }

    private void logC1121GridPaintOrder(String report) {
        String marker = "paint_node_order=[";
        int start = report.indexOf(marker);
        int end = start < 0 ? -1 : report.indexOf(']', start + marker.length());
        if (start < 0 || end < 0 || end - start > 512) {
            Log.e(TAG, "SPINON_C1121_GRID_PAINT_ORDER run_id=" + c1121GridRunId
                    + " scene_id=" + c1121GridSceneId + " marker=missing-or-too-large");
            return;
        }
        Log.i(TAG, "SPINON_C1121_GRID_PAINT_ORDER run_id=" + c1121GridRunId
                + " scene_id=" + c1121GridSceneId + " "
                + report.substring(start, end + 1));
    }

    private void logC1121GridReadySummaryIfNeeded(String report) {
        if (!report.startsWith("status=0 ") || !report.contains(" layout=ready ")) return;
        synchronized (stateLock) {
            if (c1121GridRuntimeSummaryEmitted) return;
            c1121GridRuntimeSummaryEmitted = true;
        }
        Log.i(TAG, "SPINON_C1121_GRID_SUMMARY run_id=" + c1121GridRunId
                + " scene_id=" + c1121GridSceneId + " " + runtimeStatusSummary(report));
        logC1121GridNodeFrames(report);
        logC1121GridPaintOrder(report);
    }

    private void logNodeFrames(String prefix, String state, String report) {
        String marker = "node_frames_css_px=[";
        int start = report.indexOf(marker);
        int end = start < 0 ? -1 : report.indexOf(']', start + marker.length());
        if (start < 0 || end < 0) {
            Log.w(TAG, "SPINON_" + prefix + "_FRAME_SUMMARY state=" + state
                    + " marker=missing report_length=" + report.length());
            return;
        }
        String[] frames = report.substring(start + marker.length(), end).split(";", -1);
        int loggedFrames = 0;
        for (String frame : frames) {
            if (!frame.isEmpty()) {
                Log.i(TAG, "SPINON_" + prefix + "_NODE_FRAME state=" + state + " " + frame);
                loggedFrames++;
            }
        }
        Log.i(TAG, "SPINON_" + prefix + "_FRAME_SUMMARY state=" + state
                + " frames=" + loggedFrames + " marker=present");
    }

    private String runC123EnvironmentMatrix(
            long host, boolean dark, long initialEnvironmentRevision) {
        String[][] steps = {
                {"matrix-360x800-dpr-2", "360", "800", "2"},
                {"matrix-360x800-dpr-2.625", "360", "800", "2.625"},
                {"matrix-360x800-dpr-3", "360", "800", "3"},
                {"matrix-390x844-dpr-1", "390", "844", "1"},
                {"matrix-390x844-dpr-2", "390", "844", "2"},
                {"matrix-390x844-dpr-2.625", "390", "844", "2.625"},
                {"matrix-390x844-dpr-3", "390", "844", "3"},
                {"resize-start", "360", "800", "1"},
                {"resize-outbound", "390", "844", "1"},
                {"resize-noop", "390", "844", "1"},
                {"resize-return", "360", "800", "1"},
                {"device-final", "360", "800", Float.toString(density)},
        };
        long expectedRevision = initialEnvironmentRevision;
        String lastReport = "";
        for (String[] step : steps) {
            String state = step[0];
            if (!state.equals("resize-noop")) expectedRevision++;
            float width = Float.parseFloat(step[1]);
            float height = Float.parseFloat(step[2]);
            float scale = Float.parseFloat(step[3]);
            String report = decode(nativeSetEnvironment(host, width, height, scale, dark));
            if (!report.startsWith("status=0 ")) {
                Log.e(TAG, "SPINON_C123_MATRIX_FAILURE state=" + state + " report=" + report);
                postStatus("실패 · C12.3 " + state + " · " + report);
                return null;
            }
            String actualRevision = reportField(report, "environment_revision=");
            if (!Long.toString(expectedRevision).equals(actualRevision)) {
                Log.e(TAG, "SPINON_C123_MATRIX_FAILURE state=" + state
                        + " expected_revision=" + expectedRevision
                        + " actual_revision=" + actualRevision);
                postStatus("실패 · C12.3 environment revision " + state);
                return null;
            }
            Log.i(TAG, "SPINON_C123_SUMMARY state=" + state + " viewport="
                    + width + "x" + height + " scale=" + scale + " "
                    + runtimeStatusSummary(report));
            logNodeFrames("C123", state, report);
            lastReport = report;
        }
        return lastReport;
    }

    private void logC122FailureDetails(String report) {
        final int chunkSize = 700;
        final int chunkCount = (report.length() + chunkSize - 1) / chunkSize;
        for (int index = 0; index < chunkCount; index++) {
            int start = index * chunkSize;
            int end = Math.min(report.length(), start + chunkSize);
            Log.e(TAG, "SPINON_C122_EVAL_DETAIL part=" + (index + 1) + "/"
                    + chunkCount + " " + report.substring(start, end));
        }
    }

    private void logC123FailureDetails(String report) {
        final int chunkSize = 700;
        final int chunkCount = (report.length() + chunkSize - 1) / chunkSize;
        for (int index = 0; index < chunkCount; index++) {
            int start = index * chunkSize;
            int end = Math.min(report.length(), start + chunkSize);
            Log.e(TAG, "SPINON_C123_EVAL_DETAIL part=" + (index + 1) + "/"
                    + chunkCount + " " + report.substring(start, end));
        }
    }

    private void refreshEnvironment() {
        if (beginPresentationUpdate() == 0) return;
        C0410LatestTaskLane.Admission admission = runtimePresentationLane.request();
        if (admission == C0410LatestTaskLane.Admission.REJECTED) {
            postStatus("실패 · runtime presentation queue가 요청을 받지 못했습니다");
        }
    }

    private void applyLatestEnvironment() {
        final long host;
        final long generation;
        final long sequence;
        final float widthCssPx;
        final float heightCssPx;
        final boolean dark;
        final float environmentScale;
        synchronized (stateLock) {
            if (closing || !surfaceAvailable || hostHandle == 0) return;
            host = hostHandle;
            generation = surfaceGeneration;
            sequence = hostPresentationSequence;
            widthCssPx = cssViewportWidthLocked();
            heightCssPx = cssViewportHeightLocked();
            dark = darkMode;
            environmentScale = c153GlyphPaintProbeRequested || c192BoxPaintOpacityProbeRequested
                    || rootHtmlTextPaintProbeRequested
                    ? 1.0f : density;
        }
        if (!isCurrentPresentation(generation, sequence)) return;
        String result = decode(nativeSetEnvironment(
                host,
                (rootHtmlApplicationRequested || rootHtmlBlockPaintProbeRequested
                        || rootHtmlTextPaintProbeRequested
                        || rootHtmlDefaultRenderRequested)
                        ? widthCssPx * environmentScale : widthCssPx,
                (rootHtmlApplicationRequested || rootHtmlBlockPaintProbeRequested
                        || rootHtmlTextPaintProbeRequested
                        || rootHtmlDefaultRenderRequested)
                        ? heightCssPx * environmentScale : heightCssPx,
                environmentScale, dark));
        Log.i(TAG, "SPINON_C0410_ENVIRONMENT viewport=" + widthCssPx + "x" + heightCssPx
                + " scale=" + environmentScale + " dark=" + dark + " sequence=" + sequence
                + " " + result);
        if (c1131GridRepeatRequested && result.startsWith("status=0 ")) {
            Log.i(TAG, "SPINON_C1131_GRID_ENVIRONMENT_RESULT run_id=" + c1131GridRepeatRunId
                    + " scene_id=" + C1131_GRID_REPEAT_SCENE_ID + " "
                    + runtimeStatusSummary(result));
            logNodeFrames("C1131", "environment", result);
        }
        postStatus(runtimeStatusSummary(result));
        if (result.startsWith("status=0 ") && isCurrentPresentation(generation, sequence)) {
            if (c1121GridPlacementRequested) {
                logC1121GridReadySummaryIfNeeded(result);
            }
            requestDraw();
        }
    }

    private float cssViewportWidthLocked() {
        if (c153HostDocumentTextPaintProbeRequested) return 320.0f;
        if (c153GlyphPaintProbeRequested || c192BoxPaintOpacityProbeRequested) return 360.0f;
        if (BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST && imagePaintProbeRequested) {
            return imageAsyncReplacementProbeRequested || imageObjectFitPositionProbeRequested
                    ? 224.0f : 80.0f;
        }
        if (c131CompactSurfaceMatrixProbeRequested) return 300.0f;
        if (c131BorderStripProbeRequested) return 400.0f;
        if (c132ScrollFixtureRequested || c133GestureFixtureRequested) return 400.0f;
        if (fixedSizeCssFixtureRequested) return 320.0f;
        return surfaceWidth > 0 ? (float) surfaceWidth / density : 301.0f;
    }

    private float cssViewportHeightLocked() {
        if (c153GlyphPaintProbeRequested || c192BoxPaintOpacityProbeRequested) return 240.0f;
        if (BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST && imagePaintProbeRequested) {
            return imageAsyncReplacementProbeRequested ? 88.0f
                    : imageObjectFitPositionProbeRequested ? 72.0f : 40.0f;
        }
        if (c131CompactSurfaceMatrixProbeRequested) return 100.0f;
        if (c131BorderStripProbeRequested) return 160.0f;
        if (c132ScrollFixtureRequested || c133GestureFixtureRequested) return 240.0f;
        if (fixedSizeCssFixtureRequested) return 240.0f;
        return surfaceHeight > 0 ? (float) surfaceHeight / density : 100.0f;
    }

    private boolean isCurrentPresentation(long generation, long sequence) {
        synchronized (stateLock) {
            return !closing && surfaceAvailable && surfaceGeneration == generation
                    && hostPresentationSequence == sequence;
        }
    }

    private void toggleSurfaceSize(Button button) {
        if (closing) return;
        expandedSurface = !expandedSurface;
        int widthCssPx = expandedSurface ? 341 : 301;
        int heightCssPx = expandedSurface ? 128 : 100;
        LayoutParams params = (LayoutParams) surfaceView.getLayoutParams();
        params.width = dp(widthCssPx, density);
        params.height = dp(heightCssPx, density);
        surfaceView.setLayoutParams(params);
        button.setText("표면 크기 전환 · " + widthCssPx + "×" + heightCssPx + " CSS px");
        Log.i(TAG, "SPINON_C0410_RESIZE_REQUEST viewport=" + widthCssPx + "x" + heightCssPx);
    }

    void updateColorScheme(android.content.res.Configuration configuration) {
        boolean updatedDarkMode = (configuration.uiMode
                & android.content.res.Configuration.UI_MODE_NIGHT_MASK)
                == android.content.res.Configuration.UI_MODE_NIGHT_YES;
        synchronized (stateLock) {
            if (closing || darkMode == updatedDarkMode) return;
            darkMode = updatedDarkMode;
        }
        refreshEnvironment();
    }

    private void requestDraw() {
        synchronized (stateLock) {
            if (closing) return;
            pendingDraw = true;
        }
        if (renderLane.request() == C0410LatestTaskLane.Admission.REJECTED) {
            postStatus("실패 · render queue가 draw 요청을 받지 못했습니다");
        }
    }

    private void reconcileRenderState() {
        final boolean available;
        final long generation;
        final long host;
        final long previousRenderer;
        final int width;
        final int height;
        final Surface surface;
        final CountDownLatch destroyBarrier;
        synchronized (stateLock) {
            if (closing) return;
            available = surfaceAvailable;
            generation = surfaceGeneration;
            host = hostHandle;
            width = surfaceWidth;
            height = surfaceHeight;
            previousRenderer = !available || rendererGeneration != generation
                    ? rendererHandle : 0;
            if (previousRenderer != 0) {
                rendererHandle = 0;
                rendererGeneration = -1;
            }
            if (!available) {
                pendingDraw = false;
                destroyBarrier = surfaceDestroyBarrier;
                surfaceDestroyBarrier = null;
            } else {
                destroyBarrier = null;
            }
        }

        if (!available) {
            try {
                if (previousRenderer != 0) nativeDestroySurface(previousRenderer);
                Log.i(TAG, "SPINON_C0410_SURFACE_RELEASED generation=" + generation);
            } finally {
                if (destroyBarrier != null) destroyBarrier.countDown();
            }
            return;
        }

        if (previousRenderer != 0) nativeDestroySurface(previousRenderer);
        if (host == 0 || width <= 0 || height <= 0) return;

        long renderer;
        boolean rendererIsCurrent;
        synchronized (stateLock) {
            renderer = rendererHandle;
            rendererIsCurrent = renderer != 0 && rendererGeneration == generation;
        }
        if (rendererIsCurrent) {
            drawCurrentRenderer(renderer, generation);
            return;
        }

        Surface currentSurface = surfaceView.getHolder().getSurface();
        if (!currentSurface.isValid()) {
            synchronized (stateLock) {
                if (!closing && surfaceAvailable && surfaceGeneration == generation) {
                    Log.w(TAG, "SPINON_C0410_SURFACE_RECONCILE_INVALID generation=" + generation);
                }
            }
            return;
        }
        long created = nativeCreateSurface(host, currentSurface, width, height, backend);
        boolean stale;
        boolean adopted = false;
        synchronized (stateLock) {
            stale = closing || !surfaceAvailable || surfaceGeneration != generation
                    || hostHandle != host;
            if (!stale && created != 0) {
                rendererHandle = created;
                rendererGeneration = generation;
                adopted = true;
            }
        }
        if (!adopted) {
            if (created != 0) nativeDestroySurface(created);
            if (stale) renderLane.request();
            else postStatus("실패 · WGPU 표면 생성 실패");
            return;
        }
        Log.i(TAG, "SPINON_C0410_RENDERER generation=" + generation
                + " size=" + width + "x" + height);
        drawCurrentRenderer(created, generation);
    }

    private void drawCurrentRenderer(long renderer, long generation) {
        if (shutdownProbeRequested && !shutdownProbeStarted) {
            shutdownProbeStarted = true;
            runShutdownProbeGate();
        }
        final boolean shouldDraw;
        synchronized (stateLock) {
            shouldDraw = !closing && surfaceAvailable && surfaceGeneration == generation
                    && rendererHandle == renderer && rendererGeneration == generation
                    && pendingDraw;
            if (shouldDraw) pendingDraw = false;
        }
        if (!shouldDraw) {
            if (shutdownProbeRequested) {
                Log.i(TAG, "SPINON_C0410_SHUTDOWN_PROBE_DRAW_DROPPED generation=" + generation);
            }
            return;
        }
        String result = decode(nativeDrawSurface(renderer));
        Log.i(TAG, "SPINON_C0410_DRAW generation=" + generation + " " + result);
        if (c192BoxPaintOpacityProbeRequested
                && result.contains("surface_pixel_samples_json=")) {
            invokeC192CaptureAdapter("logPixelSamples", result, "SURFACE");
        }
        if (result.startsWith("status=-12 ")) {
            Log.i(TAG, "SPINON_C0410_DRAW_SUPERSEDED generation=" + generation);
            requestDraw();
            return;
        }
        if (!result.startsWith("status=0 ")) {
            postStatus("실패 · " + result);
            return;
        }
        if (c153GlyphPaintProbeRequested
                && (c153HostDocumentTextPaintProbeRequested
                        ? (!result.contains("runtime_text_count=1")
                                || !result.contains("glyph_batch_draw_count=1"))
                        : (!result.contains("runtime_text_count=6")
                                || !result.contains("glyph_batch_draw_count=6")))) {
            Log.e(TAG, "SPINON_C153_DRAW_ASSERTIONS failed=true " + result);
            postStatus(c153HostDocumentTextPaintProbeRequested
                    ? "실패 · C15.3 HostDocument Ahem glyph batch가 제출되지 않았습니다"
                    : "실패 · C15.3 Noto glyph batch 6개가 제출되지 않았습니다");
            return;
        }
        if (c153GlyphPaintProbeRequested) {
            requestC153SurfaceCapture(renderer, generation);
        }
        if (BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST && imagePaintProbeRequested
                && !imagePaintSurfaceCaptureStarted) {
            requestImagePaintSurfaceCapture(renderer, generation);
        }
        if (BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST
                && c133GestureFixtureRequested
                && c133GestureFixtureReady && !c133GestureInitialDrawReady) {
            boolean boundarySeed = c133GesturePolicyScenarioID == 1
                    || c133GesturePolicyScenarioID == 2
                    || c133GesturePolicyScenarioID == 8
                    || c133GesturePolicyScenarioID == 9;
            String expectedRevision = boundarySeed
                    ? "scroll_state_revision=1" : "scroll_state_revision=0";
            String expectedOffsets = boundarySeed
                    ? "scroll_offsets=[0,0;0,0;86,106]"
                    : "scroll_offsets=[0,0;0,0;0,0]";
            if (!result.contains(expectedRevision) || !result.contains(expectedOffsets)) {
                c133GestureFixtureReady = false;
                Log.e(TAG, "SPINON_C133_DRAW_ASSERTIONS failed=true " + result);
                postStatus("실패 · C13.3 첫 draw snapshot이 boundary seed와 다릅니다");
                return;
            }
            c133GestureInitialDrawReady = true;
            Log.i(TAG, "SPINON_C133_READY input=MotionEvent " + result);
            postStatus("준비 · Android swipe 또는 wheel/trackpad 입력을 기다립니다");
            logC133SurfaceBoundsAfterLayout("ready");
            if (c133WheelProbeRequested) {
                surfaceView.post(this::runC133WheelProbe);
            }
        }
        if (c133WheelProbeRequested) verifyC133WheelDraw(result);
        if (c11GridFixtureRequested && !c11GridPixelMatrixRunId.isEmpty()) {
            requestC11GridPixelMatrixCapture(renderer, generation);
        } else if (c11GridFixtureRequested && !c1121GridPlacementRequested
                && !c1122GridAutoPlacementRequested
                && !c1123GridAutoPlacementRequested
                && !c1131GridRepeatRequested
                && !c1132GridNamedLinesRequested
                && !c1133GridTemplateAreasRequested
                && !c114GridTrackSizingRequested
                && c11GridSceneIndex == 0) {
            requestC11GridSurfaceCapture(renderer, generation);
        }
        if (c124SurfaceCaptureProbeRequested) {
            requestC124SurfaceCapture(renderer, generation);
        }
        if (BuildConfig.SPINON_C04_12_ROOT_HTML_BLOCK_PAINT_FIXTURE_TEST
                && rootHtmlBlockPaintProbeRequested
                && !rootHtmlBlockPaintSurfaceCaptureStarted) {
            requestRootHtmlBlockPaintSurfaceCapture(renderer, generation, result);
        }
        if (BuildConfig.SPINON_C04_12_ROOT_HTML_TEXT_PAINT_FIXTURE_TEST
                && rootHtmlTextPaintProbeRequested
                && !rootHtmlTextPaintSurfaceCaptureStarted) {
            requestRootHtmlTextPaintSurfaceCapture(renderer, generation, result);
        }
        if (c192BoxPaintOpacityProbeRequested) {
            requestC192SurfaceCapture(renderer, generation);
        } else if (c131StackingDescendantClipProbeRequested) {
            requestC131StackingDescendantClipSurfaceCapture(renderer, generation, result);
        } else if (c131CompactSurfaceMatrixProbeRequested) {
            requestC131CompactSurfaceMatrixCapture(renderer, generation);
        } else if (c131BorderStripProbeRequested) {
            requestC131BorderStripSurfaceCapture(renderer, generation);
        } else if (c131OverflowClipProbeRequested) {
            requestC131OverflowClipSurfaceCapture(renderer, generation);
        } else if (c132ScrollFixtureRequested) {
            requestC132VisibleOverflowSurfaceCapture(renderer, generation, result);
        }
        if (failureProbeRequested && !failureProbeStarted) {
            failureProbeStarted = true;
            runFailureRecoveryProbe(renderer, result);
        }
    }

    private void requestC192SurfaceCapture(long renderer, long generation) {
        final int width;
        final int height;
        synchronized (stateLock) {
            if (closing || c192SurfaceCaptureStarted || !surfaceAvailable
                    || surfaceGeneration != generation || rendererHandle != renderer
                    || rendererGeneration != generation) return;
            c192SurfaceCaptureStarted = true;
            width = surfaceWidth;
            height = surfaceHeight;
        }
        try {
            int[] sampleCoordinates = (int[]) invokeC192CaptureAdapter(
                    "surfaceSampleCoordinates", width, height);
            String request = decode(nativeRequestC192SurfacePixels(renderer, sampleCoordinates));
            Log.i(TAG, "SPINON_C192_DIRECT_REQUEST " + request);
            if (request.startsWith("status=0 ")) requestDraw();
            String offscreen = decode(nativeReadbackC192Offscreen(renderer));
            Log.i(TAG, "SPINON_C192_OFFSCREEN_REQUEST " + offscreen);
            invokeC192CaptureAdapter("logPixelSamples", offscreen, "OFFSCREEN");
        } catch (IllegalArgumentException error) {
            Log.e(TAG, "SPINON_C192_DIRECT_REQUEST status=error reason=" + error.getMessage());
        }
        java.util.function.BooleanSupplier sourceIsCurrent = () -> {
            synchronized (stateLock) {
                return !closing && surfaceAvailable && surfaceGeneration == generation
                        && rendererHandle == renderer && rendererGeneration == generation
                        && surfaceWidth == width && surfaceHeight == height;
            }
        };
        java.util.function.Consumer<String> status = this::postStatus;
        invokeC192CaptureAdapter("request", surfaceView, activity, width, height,
                generation, mainHandler, runtimeQueue, sourceIsCurrent, status);
    }

    private void requestRootHtmlBlockPaintSurfaceCapture(
            long renderer, long generation, String drawReport) {
        final int width;
        final int height;
        final boolean dark;
        synchronized (stateLock) {
            if (closing || rootHtmlBlockPaintSurfaceCaptureStarted || !surfaceAvailable
                    || surfaceGeneration != generation || rendererHandle != renderer
                    || rendererGeneration != generation) return;
            rootHtmlBlockPaintSurfaceCaptureStarted = true;
            width = surfaceWidth;
            height = surfaceHeight;
            dark = darkMode;
        }
        java.util.function.BooleanSupplier sourceIsCurrent = () -> {
            synchronized (stateLock) {
                return !closing && surfaceAvailable && surfaceGeneration == generation
                        && rendererHandle == renderer && rendererGeneration == generation
                        && surfaceWidth == width && surfaceHeight == height && darkMode == dark;
            }
        };
        java.util.function.Consumer<String> statusCallback = this::postStatus;
        try {
            invokeRootHtmlBlockPaintCaptureAdapter("request", surfaceView, activity, width,
                    height, generation, dark, drawReport, mainHandler, runtimeQueue,
                    sourceIsCurrent, statusCallback);
        } catch (IllegalStateException error) {
            Log.e(TAG, "SPINON_C0412_PIXELCOPY status=adapter-error generation=" + generation
                    + " reason=" + error.getClass().getSimpleName());
            postStatus("실패 · C04.12 Android PixelCopy 어댑터 오류");
        }
    }

    private void requestRootHtmlTextPaintSurfaceCapture(
            long renderer, long generation, String drawReport) {
        final int width;
        final int height;
        synchronized (stateLock) {
            if (closing || rootHtmlTextPaintSurfaceCaptureStarted || !surfaceAvailable
                    || surfaceGeneration != generation || rendererHandle != renderer
                    || rendererGeneration != generation) return;
            rootHtmlTextPaintSurfaceCaptureStarted = true;
            width = surfaceWidth;
            height = surfaceHeight;
        }
        java.util.function.BooleanSupplier sourceIsCurrent = () -> {
            synchronized (stateLock) {
                return !closing && surfaceAvailable && surfaceGeneration == generation
                        && rendererHandle == renderer && rendererGeneration == generation
                        && surfaceWidth == width && surfaceHeight == height;
            }
        };
        java.util.function.Consumer<String> statusCallback = this::postStatus;
        try {
            invokeRootHtmlTextPaintCaptureAdapter("request", surfaceView, activity, width,
                    height, generation, drawReport, mainHandler, runtimeQueue,
                    sourceIsCurrent, statusCallback);
        } catch (IllegalStateException error) {
            Log.e(TAG, "SPINON_C0412_TEXT_PIXELCOPY status=adapter-error generation="
                    + generation + " reason=" + error.getClass().getSimpleName());
            postStatus("실패 · C04.12 Android text PixelCopy 어댑터 오류");
        }
    }

    private void requestC153SurfaceCapture(long renderer, long generation) {
        final int width;
        final int height;
        synchronized (stateLock) {
            if (closing || c153SurfaceCaptureStarted || !surfaceAvailable
                    || surfaceGeneration != generation || rendererHandle != renderer
                    || rendererGeneration != generation) return;
            c153SurfaceCaptureStarted = true;
            width = surfaceWidth;
            height = surfaceHeight;
        }
        List<SurfaceViewPixelCapture.Sample> samples = parseC153SurfaceSamples(
                c153GlyphPaintEvaluationReport);
        if (samples == null) {
            Log.e(TAG, "SPINON_C153_PIXELCOPY status=error stage=sample-report");
            postStatus("실패 · C15.3 glyph overflow 표본 좌표를 읽지 못했습니다");
            return;
        }
        String runId = Long.toHexString(SystemClock.elapsedRealtimeNanos());
        int viewportWidth = c153HostDocumentTextPaintProbeRequested ? 320 : 360;
        File output = new File(
                new File(activity.getCacheDir(), c153HostDocumentTextPaintProbeRequested
                        ? "c153-hostdocument-text-paint-captures"
                        : "c153-glyph-overflow-surface-captures"),
                (c153HostDocumentTextPaintProbeRequested
                        ? "runtime-c15-3-hostdocument-text-paint-"
                        : "runtime-c15-3-glyph-overflow-") + runId + ".png");
        Log.i(TAG, "SPINON_C153_PIXELCOPY_BEGIN run_id=" + runId
                + " pid=" + android.os.Process.myPid() + " generation=" + generation
                + " width=" + width + " height=" + height + " viewport="
                + viewportWidth + "x240"
                + " samples=" + samples.size());
        SurfaceViewPixelCapture.requestForCssViewport(surfaceView, width, height,
                viewportWidth, 240, 0, 0,
                c153HostDocumentTextPaintProbeRequested
                        ? "c153-hostdocument-text-paint" : "c153-glyph-overflow",
                runId, output, samples,
                mainHandler, runtimeQueue,
                () -> {
                    synchronized (stateLock) {
                        return !closing && surfaceAvailable && surfaceGeneration == generation
                                && rendererHandle == renderer && rendererGeneration == generation
                                && surfaceWidth == width && surfaceHeight == height;
                    }
                }, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C153_PIXELCOPY status=error run_id=" + runId
                                + " generation=" + generation + " attempts=" + result.attempts
                                + " reason=" + result.error);
                        postStatus("실패 · C15.3 PixelCopy · " + result.error);
                        return;
                    }
                    if (c153HostDocumentTextPaintProbeRequested) {
                        verifyC153HostDocumentSurfaceCapture(
                                runId, generation, samples, output, result);
                        return;
                    }
                    boolean visibleInk = false;
                    boolean hiddenInsideInk = false;
                    boolean hiddenOutsideBackground = false;
                    boolean horizontalScrollInk = false;
                    boolean verticalScrollInk = false;
                    boolean nestedScrollInk = false;
                    int observed = 0;
                    for (SurfaceViewPixelCapture.Pixel pixel : result.pixels) {
                        boolean background = pixel.red == 247 && pixel.green == 249
                                && pixel.blue == 252 && pixel.alpha == 255;
                        boolean expectedInk = pixel.red == 9 && pixel.green == 33
                                && pixel.blue == 112 && pixel.alpha == 255;
                        if ("visible-outside-parent".equals(pixel.id)) {
                            visibleInk = expectedInk;
                        } else if ("hidden-inside-clip".equals(pixel.id)) {
                            hiddenInsideInk = expectedInk;
                        } else if ("hidden-outside-parent".equals(pixel.id)) {
                            hiddenOutsideBackground = background;
                        } else if ("horizontal-scroll-glyph".equals(pixel.id)) {
                            horizontalScrollInk = expectedInk;
                        } else if ("vertical-scroll-glyph".equals(pixel.id)) {
                            verticalScrollInk = expectedInk;
                        } else if ("nested-scroll-glyph".equals(pixel.id)) {
                            nestedScrollInk = expectedInk;
                        }
                        observed++;
                        Log.i(TAG, "SPINON_C153_PIXELCOPY_SAMPLE run_id=" + runId
                                + " sample=" + pixel.id + " x=" + pixel.x + " y=" + pixel.y
                                + " rgba=" + pixel.red + "," + pixel.green + ","
                                + pixel.blue + "," + pixel.alpha + " stage_background="
                                + background + " expected_ink=" + expectedInk);
                    }
                    boolean outsideParent = samples.get(2).xCssPx >= 29.0;
                    boolean passed = observed == 6 && visibleInk && hiddenInsideInk
                            && hiddenOutsideBackground && horizontalScrollInk
                            && verticalScrollInk && nestedScrollInk && outsideParent;
                    Log.i(TAG, "SPINON_C153_PIXELCOPY_SUMMARY status="
                            + (passed ? "matched" : "mismatch") + " run_id=" + runId
                            + " pid=" + android.os.Process.myPid() + " generation=" + generation
                            + " viewport=360x240 expected_samples=6 observed_samples=" + observed
                            + " visible_outside_ink=" + visibleInk
                            + " hidden_inside_ink=" + hiddenInsideInk
                            + " hidden_outside_background=" + hiddenOutsideBackground
                            + " horizontal_scroll_ink=" + horizontalScrollInk
                            + " vertical_scroll_ink=" + verticalScrollInk
                            + " nested_scroll_ink=" + nestedScrollInk
                            + " outside_parent=" + outsideParent + " png="
                            + output.getAbsolutePath() + " sha256=" + result.sha256);
                    if (!passed) postStatus("실패 · C15.3 실제 surface overflow clip 표본 불일치");
                });
    }

    private static List<SurfaceViewPixelCapture.Sample> parseC153SurfaceSamples(String report) {
        if (report.contains("c15_3_hostdocument_text_paint=attached")) {
            return parseC153HostDocumentSurfaceSamples(report);
        }
        Map<String, int[]> points = new HashMap<>();
        Set<String> coordinatesSeen = new HashSet<>();
        String[] encodedFields = new String[] {
                reportField(report, "overflow_samples_css_px="),
                reportField(report, "scroll_samples_css_px=")
        };
        Set<String> acceptedNames = Set.of(
                "visible", "hidden_inside", "hidden_outside",
                "horizontal", "vertical", "nested");
        for (String encoded : encodedFields) {
            if (encoded.isEmpty()) return null;
            for (String entry : encoded.split(";")) {
                String[] pair = entry.split(":", 2);
                if (pair.length != 2 || !acceptedNames.contains(pair[0])) return null;
                String[] coordinates = pair[1].split(",", 2);
                if (coordinates.length != 2) return null;
                final int x;
                final int y;
                try {
                    x = Integer.parseInt(coordinates[0]);
                    y = Integer.parseInt(coordinates[1]);
                } catch (NumberFormatException error) {
                    return null;
                }
                String coordinate = x + ":" + y;
                if (x < 0 || x >= 360 || y < 0 || y >= 240
                        || !coordinatesSeen.add(coordinate)
                        || points.put(pair[0], new int[] {x, y}) != null) return null;
            }
        }
        int[] visible = points.get("visible");
        int[] hiddenInside = points.get("hidden_inside");
        int[] hiddenOutside = points.get("hidden_outside");
        int[] horizontal = points.get("horizontal");
        int[] vertical = points.get("vertical");
        int[] nested = points.get("nested");
        if (visible == null || hiddenInside == null || hiddenOutside == null
                || horizontal == null || vertical == null || nested == null
                || points.size() != 6
                || visible[0] != hiddenOutside[0] || visible[1] + 40 != hiddenOutside[1]
                || visible[0] < 29 || visible[1] < 81 || visible[1] >= 91
                || hiddenInside[0] < 13 || hiddenInside[0] >= 28
                || hiddenInside[1] < 121 || hiddenInside[1] >= 131
                || hiddenOutside[0] < 29 || hiddenOutside[1] < 121
                || hiddenOutside[1] >= 131
                || horizontal[0] < 41 || horizontal[0] >= 54
                || horizontal[1] < 161 || horizontal[1] >= 181
                || vertical[0] < 101 || vertical[0] >= 129
                || vertical[1] < 161 || vertical[1] >= 174
                || nested[0] < 257 || nested[0] >= 276
                || nested[1] < 162 || nested[1] >= 181) return null;
        return List.of(
                new SurfaceViewPixelCapture.Sample(
                        "visible-outside-parent", visible[0], visible[1]),
                new SurfaceViewPixelCapture.Sample(
                        "hidden-inside-clip", hiddenInside[0], hiddenInside[1]),
                new SurfaceViewPixelCapture.Sample(
                        "hidden-outside-parent", hiddenOutside[0], hiddenOutside[1]),
                new SurfaceViewPixelCapture.Sample(
                        "horizontal-scroll-glyph", horizontal[0], horizontal[1]),
                new SurfaceViewPixelCapture.Sample(
                        "vertical-scroll-glyph", vertical[0], vertical[1]),
                new SurfaceViewPixelCapture.Sample(
                        "nested-scroll-glyph", nested[0], nested[1]));
    }

    private static List<SurfaceViewPixelCapture.Sample> parseC153HostDocumentSurfaceSamples(
            String report) {
        String encoded = reportField(report, "hostdocument_text_samples_css_px=");
        Map<String, int[]> points = new HashMap<>();
        Set<String> coordinatesSeen = new HashSet<>();
        Set<String> acceptedNames = Set.of(
                "first_ink_1", "first_ink_2", "overflow_ink_1", "overflow_ink_2",
                "overflow_ink_3", "overflow_ink_4");
        if (encoded.isEmpty()) return null;
        for (String entry : encoded.split(";")) {
            String[] pair = entry.split(":", 2);
            if (pair.length != 2 || !acceptedNames.contains(pair[0])) return null;
            String[] coordinates = pair[1].split(",", 2);
            if (coordinates.length != 2) return null;
            final int x;
            final int y;
            try {
                x = Integer.parseInt(coordinates[0]);
                y = Integer.parseInt(coordinates[1]);
            } catch (NumberFormatException error) {
                return null;
            }
            String coordinate = x + ":" + y;
            if (x < 0 || x >= 320 || y < 0 || y >= 240
                    || !coordinatesSeen.add(coordinate)
                    || points.put(pair[0], new int[] {x, y}) != null) return null;
        }
        int[] first1 = points.get("first_ink_1");
        int[] first2 = points.get("first_ink_2");
        int[] overflow1 = points.get("overflow_ink_1");
        int[] overflow2 = points.get("overflow_ink_2");
        int[] overflow3 = points.get("overflow_ink_3");
        int[] overflow4 = points.get("overflow_ink_4");
        if (points.size() != 6 || first1 == null || first2 == null || overflow1 == null
                || overflow2 == null || overflow3 == null || overflow4 == null
                || first1[0] >= 15 || first2[0] >= 15
                || overflow1[0] < 15 || overflow2[0] < 15
                || overflow3[0] < 15 || overflow4[0] < 15) return null;
        return List.of(
                new SurfaceViewPixelCapture.Sample("first-glyph-ink-1", first1[0], first1[1]),
                new SurfaceViewPixelCapture.Sample("first-glyph-ink-2", first2[0], first2[1]),
                new SurfaceViewPixelCapture.Sample("overflow-glyph-ink-1", overflow1[0], overflow1[1]),
                new SurfaceViewPixelCapture.Sample("overflow-glyph-ink-2", overflow2[0], overflow2[1]),
                new SurfaceViewPixelCapture.Sample("overflow-glyph-ink-3", overflow3[0], overflow3[1]),
                new SurfaceViewPixelCapture.Sample("overflow-glyph-ink-4", overflow4[0], overflow4[1]));
    }

    private void verifyC153HostDocumentSurfaceCapture(
            String runId,
            long generation,
            List<SurfaceViewPixelCapture.Sample> samples,
            File output,
            SurfaceViewPixelCapture.Result result) {
        int firstGlyphInk = 0;
        int overflowGlyphInk = 0;
        boolean passed = samples.size() == 6 && result.pixels.size() == 6;
        for (SurfaceViewPixelCapture.Pixel pixel : result.pixels) {
            boolean expectedInk = pixel.red == 9 && pixel.green == 33
                    && pixel.blue == 112 && pixel.alpha == 255;
            boolean firstGlyph = pixel.id.startsWith("first-glyph-ink-");
            boolean overflowGlyph = pixel.id.startsWith("overflow-glyph-ink-");
            passed &= expectedInk
                    && (firstGlyph && pixel.x < 15 || overflowGlyph && pixel.x >= 15);
            if (firstGlyph && expectedInk) firstGlyphInk++;
            if (overflowGlyph && expectedInk) overflowGlyphInk++;
            Log.i(TAG, "SPINON_C153_HOSTDOCUMENT_PIXELCOPY_SAMPLE run_id=" + runId
                    + " sample=" + pixel.id + " x=" + pixel.x + " y=" + pixel.y
                    + " rgba=" + pixel.red + "," + pixel.green + ","
                    + pixel.blue + "," + pixel.alpha + " expected_ink=" + expectedInk);
        }
        passed &= firstGlyphInk == 2 && overflowGlyphInk == 4;
        Log.i(TAG, "SPINON_C153_HOSTDOCUMENT_PIXELCOPY_SUMMARY status="
                + (passed ? "matched" : "mismatch") + " run_id=" + runId
                + " pid=" + android.os.Process.myPid() + " generation=" + generation
                + " viewport=320x240 owner_width=15 text_advance=20 first_glyph_ink="
                + firstGlyphInk + "/2 overflow_glyph_ink=" + overflowGlyphInk
                + "/4 owner_overflow_css_px=5 observed_samples=" + result.pixels.size()
                + " png=" + output.getAbsolutePath() + " sha256=" + result.sha256);
        if (!passed) postStatus("실패 · C15.3 HostDocument text overflow PixelCopy 표본 불일치");
        else postStatus("통과 · C15.3 HostDocument text overflow PixelCopy · 6/6");
    }

    // C19.2 캡처 어댑터는 fixture-test source set에만 있으므로 기본 앱에서 직접 참조하지 않는다.
    private static Object invokeC192CaptureAdapter(String methodName, Object... arguments) {
        try {
            Class<?> adapter = Class.forName(
                    "dev.spinon.bootstrap.BoxPaintOpacityFixtureCaptureAdapter");
            for (java.lang.reflect.Method method : adapter.getDeclaredMethods()) {
                if (!method.getName().equals(methodName)
                        || method.getParameterCount() != arguments.length) continue;
                method.setAccessible(true);
                return method.invoke(null, arguments);
            }
            throw new IllegalStateException("C19.2 adapter method not found: " + methodName);
        } catch (ReflectiveOperationException | SecurityException error) {
            throw new IllegalStateException("C19.2 adapter invocation failed: " + methodName,
                    error);
        }
    }

    private static Object invokeRootHtmlBlockPaintCaptureAdapter(
            String methodName, Object... arguments) {
        try {
            Class<?> adapter = Class.forName(
                    "dev.spinon.bootstrap.RootHtmlBlockPaintFixtureCaptureAdapter");
            for (java.lang.reflect.Method method : adapter.getDeclaredMethods()) {
                if (!method.getName().equals(methodName)
                        || method.getParameterCount() != arguments.length) continue;
                method.setAccessible(true);
                return method.invoke(null, arguments);
            }
            throw new IllegalStateException("C04.12 root HTML Block paint adapter method not found: "
                    + methodName);
        } catch (ReflectiveOperationException | RuntimeException error) {
            throw new IllegalStateException("C04.12 root HTML Block paint adapter invocation failed: "
                    + methodName, error);
        }
    }

    private static Object invokeRootHtmlTextPaintCaptureAdapter(
            String methodName, Object... arguments) {
        try {
            Class<?> adapter = Class.forName(
                    "dev.spinon.bootstrap.RootHtmlTextPaintFixtureCaptureAdapter");
            for (java.lang.reflect.Method method : adapter.getDeclaredMethods()) {
                if (!method.getName().equals(methodName)
                        || method.getParameterCount() != arguments.length) continue;
                method.setAccessible(true);
                return method.invoke(null, arguments);
            }
            throw new IllegalStateException("C04.12 root HTML text paint adapter method not found: "
                    + methodName);
        } catch (ReflectiveOperationException | RuntimeException error) {
            throw new IllegalStateException("C04.12 root HTML text paint adapter invocation failed: "
                    + methodName, error);
        }
    }

    private void requestImagePaintSurfaceCapture(long renderer, long generation) {
        if (BuildConfig.SPINON_IMAGE_PAINT_FIXTURE_TEST) {
            final int width;
            final int height;
            synchronized (stateLock) {
                if (closing || imagePaintSurfaceCaptureStarted || !surfaceAvailable
                        || surfaceGeneration != generation || rendererHandle != renderer
                        || rendererGeneration != generation) return;
                imagePaintSurfaceCaptureStarted = true;
                width = surfaceWidth;
                height = surfaceHeight;
            }
            java.util.function.BooleanSupplier sourceIsCurrent = () -> {
                synchronized (stateLock) {
                    return !closing && surfaceAvailable && surfaceGeneration == generation
                            && rendererHandle == renderer && rendererGeneration == generation
                            && surfaceWidth == width && surfaceHeight == height;
                }
            };
            java.util.function.Consumer<String> status = this::postStatus;
            invokeFixtureNative(
                    imageAsyncReplacementProbeRequested
                            ? "dev.spinon.bootstrap.AsyncImageFixtureCaptureAdapter"
                            : imageObjectFitPositionProbeRequested
                            ? "dev.spinon.bootstrap.ObjectFitPositionFixtureCaptureAdapter"
                            : imageAlphaProbeRequested
                            ? "dev.spinon.bootstrap.AlphaImageFixtureCaptureAdapter"
                            : "dev.spinon.bootstrap.ImagePaintFixtureCaptureAdapter",
                    "request",
                    new Class<?>[] {View.class, SurfaceView.class, Activity.class,
                            int.class, int.class, long.class, Handler.class,
                            java.util.concurrent.Executor.class,
                            java.util.function.BooleanSupplier.class,
                            java.util.function.Consumer.class},
                    this, surfaceView, activity, width, height, generation,
                    mainHandler, runtimeQueue, sourceIsCurrent, status);
        }
    }

    private void runShutdownProbeGate() {
        CountDownLatch release = new CountDownLatch(1);
        for (int request = 0; request < 10_000; request++) requestDraw();
        C0410LatestTaskLane.Snapshot snapshot = renderLane.snapshot();
        Log.i(TAG, "SPINON_C0410_SHUTDOWN_PROBE_PENDING requests=10000"
                + " scheduled=" + snapshot.scheduled() + " dirty=" + snapshot.dirty());
        mainHandler.postDelayed(() -> {
            dispose();
            activity.finish();
        }, 100);
        Thread releaser = new Thread(() -> {
            try {
                Thread.sleep(350);
            } catch (InterruptedException error) {
                Thread.currentThread().interrupt();
            } finally {
                release.countDown();
            }
        }, "spinon-c0410-shutdown-release");
        releaser.setDaemon(true);
        releaser.start();
        awaitUninterruptibly(release);
        Log.i(TAG, "SPINON_C0410_SHUTDOWN_PROBE_GATE_RELEASED");
    }

    private void runFailureRecoveryProbe(long renderer, String baseline) {
        String armed = decode(nativeInjectNextDrawFailure(renderer));
        Log.i(TAG, "SPINON_C0410_FAILURE_PROBE_ARM " + armed);
        if (!armed.startsWith("status=0 ")) {
            Log.e(TAG, "SPINON_C0410_FAILURE_PROBE_FAILED stage=arm " + armed);
            postStatus("실패 · draw 오류 주입을 설정하지 못했습니다");
            return;
        }
        String expectedFailure = decode(nativeDrawSurface(renderer));
        Log.i(TAG, "SPINON_C0410_FAILURE_PROBE_EXPECTED_ERROR " + expectedFailure);
        if (expectedFailure.startsWith("status=0 ")
                || !expectedFailure.contains("시험용으로 다음 draw를 실패")) {
            Log.e(TAG, "SPINON_C0410_FAILURE_PROBE_FAILED stage=expected-error "
                    + expectedFailure);
            postStatus("실패 · 시험용 draw 오류가 예상대로 전달되지 않았습니다");
            return;
        }
        String recovered = decode(nativeDrawSurface(renderer));
        String baselineRevision = reportField(baseline, "environment_revision=");
        String recoveredRevision = reportField(recovered, "environment_revision=");
        boolean sameScene = !baselineRevision.isEmpty()
                && baselineRevision.equals(recoveredRevision);
        boolean passed = recovered.startsWith("status=0 ") && sameScene;
        Log.i(TAG, "SPINON_C0410_FAILURE_PROBE_RECOVERY passed=" + passed
                + " baseline_revision=" + baselineRevision
                + " recovered_revision=" + recoveredRevision + " " + recovered);
        if (passed) {
            postStatus("draw 오류가 장면 revision을 바꾸지 않았고 다음 draw가 회복됐습니다");
        } else {
            Log.e(TAG, "SPINON_C0410_FAILURE_PROBE_FAILED stage=recovery");
            postStatus("실패 · draw 오류 뒤 정상 장면 복구가 확인되지 않았습니다");
        }
    }

    private static String reportField(String report, String field) {
        int start = report.indexOf(field);
        if (start < 0) return "";
        start += field.length();
        int end = report.indexOf(' ', start);
        return end < 0 ? report.substring(start) : report.substring(start, end);
    }

    private void reportLaneFailure(String lane, RuntimeException error) {
        Log.e(TAG, "SPINON_C0410_QUEUE_FAILURE lane=" + lane, error);
        postStatus("실패 · " + lane + " queue: " + error.getClass().getSimpleName());
    }

    private void postStatus(String value) {
        mainHandler.post(() -> {
            if (!closing) status.setText(value);
        });
    }

    private void logC133SurfaceBoundsAfterLayout(String phase) {
        if (!BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST || !c133GestureFixtureRequested) return;
        surfaceView.postDelayed(() -> {
            int[] screenOrigin = new int[2];
            surfaceView.getLocationOnScreen(screenOrigin);
            String marker = new StringBuilder("SPINON_C133_SURFACE")
                    .append("_BOUNDS").toString();
            Log.i(TAG, marker + " phase=" + phase
                    + " screen_px=" + screenOrigin[0] + "," + screenOrigin[1]
                    + " size_px=" + surfaceView.getWidth() + "x" + surfaceView.getHeight());
        }, 100);
    }

    private static String decode(byte[] value) {
        return value == null ? "status=-1 native bridge returned no report"
                : new String(value, StandardCharsets.UTF_8);
    }

    void dispose() {
        final long generation;
        synchronized (stateLock) {
            if (closing) return;
        }
        if (c133GestureFixtureRequested) {
            cancelC133Gesture(3, "activity-destroy");
            resetC133TouchState(true);
        }
        beginPresentationUpdate();
        synchronized (stateLock) {
            if (closing) return;
            surfaceAvailable = false;
            surfaceWidth = 0;
            surfaceHeight = 0;
            surfaceFormat = -1;
            pendingDraw = false;
            generation = ++surfaceGeneration;
            closing = true;
        }
        runtimePresentationLane.close();
        renderLane.close();
        runtimeQueue.execute(() -> {
            renderQueue.execute(() -> {
                final long renderer;
                final long host;
                final CountDownLatch barrier;
                synchronized (stateLock) {
                    renderer = rendererHandle;
                    host = hostHandle;
                    rendererHandle = 0;
                    rendererGeneration = -1;
                    hostHandle = 0;
                    barrier = surfaceDestroyBarrier;
                    surfaceDestroyBarrier = null;
                }
                try {
                    if (renderer != 0) nativeDestroySurface(renderer);
                    if (host != 0) nativeFreeHost(host);
                } finally {
                    if (barrier != null) barrier.countDown();
                }
            });
            renderQueue.shutdown();
        });
        runtimeQueue.shutdown();
        awaitTerminationUninterruptibly(runtimeQueue);
        awaitTerminationUninterruptibly(renderQueue);
        surfaceView.getHolder().removeCallback(this);
        Log.i(TAG, "SPINON_C0410_DISPOSE_DRAINED generation=" + generation);
    }

    @Override
    public void surfaceCreated(SurfaceHolder holder) {
        final long generation;
        synchronized (stateLock) {
            if (closing) return;
            generation = ++surfaceGeneration;
            surfaceAvailable = true;
            pendingDraw = false;
        }
        Log.i(TAG, "SPINON_C0410_SURFACE_CREATED generation=" + generation);
        refreshEnvironment();
        renderLane.request();
    }

    @Override
    public void surfaceChanged(SurfaceHolder holder, int format, int width, int height) {
        if (width <= 0 || height <= 0) {
            Log.w(TAG, "SPINON_C0410_SURFACE_CHANGED_IGNORED size=" + width + "x" + height);
            return;
        }
        if (c133GestureFixtureRequested) {
            cancelC133Gesture(3, "surface-generation-change");
            resetC133TouchState(true);
        }
        final long generation;
        final boolean changed;
        synchronized (stateLock) {
            if (closing) return;
            changed = !surfaceAvailable || surfaceFormat != format
                    || surfaceWidth != width || surfaceHeight != height;
            surfaceAvailable = true;
            if (changed) {
                surfaceGeneration++;
                pendingDraw = false;
            }
            surfaceWidth = width;
            surfaceHeight = height;
            surfaceFormat = format;
            generation = surfaceGeneration;
        }
        Log.i(TAG, "SPINON_C0410_SURFACE_CHANGED generation=" + generation
                + " size=" + width + "x" + height + " changed=" + changed);
        if (changed) refreshEnvironment();
        scheduleRuntimeInitializationForReadySurface();
        renderLane.request();
    }

    @Override
    public void surfaceDestroyed(SurfaceHolder holder) {
        if (c133GestureFixtureRequested) {
            cancelC133Gesture(3, "surface-destroy");
            resetC133TouchState(true);
        }
        final long generation;
        final CountDownLatch renderQueueDrained = new CountDownLatch(1);
        synchronized (stateLock) {
            if (closing) return;
            surfaceAvailable = false;
            surfaceGeneration++;
            surfaceWidth = 0;
            surfaceHeight = 0;
            surfaceFormat = -1;
            pendingDraw = false;
            generation = surfaceGeneration;
            surfaceDestroyBarrier = renderQueueDrained;
        }
        beginPresentationUpdate();
        C0410LatestTaskLane.Admission admission = renderLane.request();
        if (admission == C0410LatestTaskLane.Admission.CLOSED) {
            Log.i(TAG, "SPINON_C0410_SURFACE_DESTROYED_WAITING_FOR_ACTIVITY_DRAIN");
        }
        // SurfaceHolder 계약상 콜백이 반환되기 전에 렌더 큐가 surface 사용을 끝내야 합니다.
        long waitStarted = SystemClock.elapsedRealtimeNanos();
        awaitUninterruptibly(renderQueueDrained);
        long waitMillis = TimeUnit.NANOSECONDS.toMillis(
                SystemClock.elapsedRealtimeNanos() - waitStarted);
        Log.i(TAG, "SPINON_C0410_SURFACE_DESTROYED_DRAINED generation=" + generation
                + " wait_ms=" + waitMillis);
    }

    void onHostPaused() {
        if (c133GestureFixtureRequested) {
            cancelC133Gesture(3, "activity-background");
            resetC133TouchState(true);
        }
    }

    private boolean handleC133GestureTouch(MotionEvent event) {
        boolean wasForwardingTapGesture = c133TapForwardingGesture;
        boolean panWasStarted = c133PanStarted;
        boolean handled = handleC133GestureTouchInternal(event);
        int action = event.getActionMasked();
        if (action == MotionEvent.ACTION_DOWN) {
            c133TapForwardingGesture = c133TapProbeRequested && c133TapDownAccepted;
        } else if (action == MotionEvent.ACTION_MOVE
                && wasForwardingTapGesture && !panWasStarted && c133PanStarted) {
            surfaceView.setPressed(false);
            surfaceView.cancelLongPress();
            c133TapForwardingGesture = false;
            return true;
        } else if (action == MotionEvent.ACTION_UP || action == MotionEvent.ACTION_CANCEL) {
            c133TapForwardingGesture = false;
        }
        return wasForwardingTapGesture || c133TapForwardingGesture ? false : handled;
    }

    private boolean handleC133GestureTouchInternal(MotionEvent event) {
        if (!c133GestureFixtureRequested) return false;
        if (event.getPointerCount() > 0
                && event.getToolType(0) != MotionEvent.TOOL_TYPE_FINGER) return false;
        long callbackStarted = SystemClock.elapsedRealtimeNanos();
        int action = event.getActionMasked();
        switch (action) {
            case MotionEvent.ACTION_DOWN: {
                c133TapDownAccepted = false;
                c133TapClickCandidate = false;
                if (!c133GestureFixtureReady || !c133GestureInitialDrawReady
                        || c133SuppressUntilAllPointersUp || c133StyleBarrierPending) return true;
                synchronized (stateLock) {
                    if (closing || hostHandle == 0 || surfaceWidth <= 0 || surfaceHeight <= 0) {
                        return false;
                    }
                    c133GestureSurfaceGeneration = surfaceGeneration;
                    c133CoordinateScale = (float) surfaceWidth / 400.0f;
                    float scaleY = (float) surfaceHeight / 240.0f;
                    if (!Float.isFinite(c133CoordinateScale) || c133CoordinateScale <= 0.0f
                            || Math.abs(c133CoordinateScale - scaleY) > 0.01f) {
                        Log.e(TAG, "SPINON_C133_TOUCH_REJECT reason=nonuniform-surface-scale"
                                + " scale_x=" + c133CoordinateScale + " scale_y=" + scaleY);
                        return false;
                    }
                }
                c133GestureId = ++c133NextGestureId;
                if (c133GestureId == 0) c133GestureId = ++c133NextGestureId;
                c133ActivePointerId = event.getPointerId(0);
                c133DownXPhysicalPx = event.getX(0);
                c133DownYPhysicalPx = event.getY(0);
                c133LastCumulativeXCssPx = 0.0f;
                c133LastCumulativeYCssPx = 0.0f;
                c133PanStarted = false;
                c133TerminalQueued = false;
                String result = enqueueC133Gesture(0,
                        c133DownXPhysicalPx / c133CoordinateScale,
                        c133DownYPhysicalPx / c133CoordinateScale, 0);
                c133TouchActive = result.startsWith("status=0 ");
                c133TapDownAccepted = c133TouchActive;
                if (!c133TouchActive) {
                    Log.e(TAG, "SPINON_C133_TOUCH_BEGIN_REJECTED " + result);
                    return true;
                }
                scheduleC133Drain();
                logC133Callback("down", event, 0.0f, 0.0f, 0.0f, 0.0f,
                        result, callbackStarted);
                return true;
            }
            case MotionEvent.ACTION_POINTER_DOWN:
                if (c133TouchActive && !c133TerminalQueued) {
                    cancelC133Gesture(1, "second-pointer");
                    c133SuppressUntilAllPointersUp = true;
                }
                logC133Callback("second-pointer", event, 0.0f, 0.0f, 0.0f, 0.0f,
                        "cancelled=true", callbackStarted);
                return true;
            case MotionEvent.ACTION_MOVE: {
                if (c133SuppressUntilAllPointersUp) {
                    logC133Callback("suppressed-move", event, 0.0f, 0.0f, 0.0f, 0.0f,
                            "accepted=false", callbackStarted);
                    return true;
                }
                if (!c133TouchActive || c133TerminalQueued) return true;
                int pointerIndex = event.findPointerIndex(c133ActivePointerId);
                if (pointerIndex < 0) {
                    cancelC133Gesture(4, "active-pointer-missing");
                    c133SuppressUntilAllPointersUp = true;
                    return true;
                }
                float rawX = event.getX(pointerIndex) - c133DownXPhysicalPx;
                float rawY = event.getY(pointerIndex) - c133DownYPhysicalPx;
                float cumulativeX = rawX / c133CoordinateScale;
                float cumulativeY = rawY / c133CoordinateScale;
                float deltaX = cumulativeX - c133LastCumulativeXCssPx;
                float deltaY = cumulativeY - c133LastCumulativeYCssPx;
                if (!c133PanStarted && Math.hypot(rawX, rawY) < c133TouchSlopPx) {
                    logC133Callback("below-slop", event, cumulativeX, cumulativeY,
                            deltaX, deltaY, "accepted=false", callbackStarted);
                    return true;
                }
                boolean began = !c133PanStarted;
                if (c133StyleBarrierPending) {
                    c133LastCumulativeXCssPx = cumulativeX;
                    c133LastCumulativeYCssPx = cumulativeY;
                    c133StyleDeferredMovePending = true;
                    c133StyleDeferredCumulativeX = cumulativeX;
                    c133StyleDeferredCumulativeY = cumulativeY;
                    logC133Callback("style-barrier-deferred-move", event, cumulativeX,
                            cumulativeY, deltaX, deltaY, "accepted=deferred", callbackStarted);
                    return true;
                }
                if (began && c133StyleInvalidationScenario() && !c133StyleMutationApplied) {
                    c133StyleBarrierPending = true;
                }
                String result = enqueueC133Gesture(began ? 1 : 2,
                        cumulativeX, cumulativeY, 0);
                if (result.startsWith("status=0 ")) {
                    c133PanStarted = true;
                    c133LastCumulativeXCssPx = cumulativeX;
                    c133LastCumulativeYCssPx = cumulativeY;
                    if (result.contains("GestureCancelledForCapacity")) {
                        c133TerminalQueued = true;
                        c133SuppressUntilAllPointersUp = true;
                    }
                    scheduleC133Drain();
                } else {
                    if (began) c133StyleBarrierPending = false;
                    Log.e(TAG, "SPINON_C133_TOUCH_MOTION_REJECTED " + result);
                }
                logC133Callback(began ? "pan-began" : "move", event,
                        cumulativeX, cumulativeY, deltaX, deltaY, result, callbackStarted);
                return true;
            }
            case MotionEvent.ACTION_POINTER_UP:
                if (c133SuppressUntilAllPointersUp) return true;
                return c133TouchActive;
            case MotionEvent.ACTION_UP:
                if (c133StyleBarrierPending) {
                    if (c133StyleDeferredTerminalAction < 0) {
                        c133StyleDeferredTerminalAction = 3;
                    }
                    c133TouchActive = false;
                    c133TerminalQueued = true;
                    Log.i(TAG, "SPINON_C133_STYLE_BARRIER terminal=deferred action="
                            + c133StyleDeferredTerminalAction + " gesture_id=" + c133GestureId);
                    return true;
                }
                boolean tapCandidate = c133TapProbeRequested && c133TapDownAccepted
                        && c133TouchActive && !c133PanStarted && !c133TerminalQueued
                        && !c133SuppressUntilAllPointersUp;
                boolean tapEndAccepted = false;
                if (c133TouchActive && !c133TerminalQueued) {
                    String result = enqueueC133Gesture(3, 0.0f, 0.0f, 0);
                    c133TerminalQueued = result.startsWith("status=0 ");
                    tapEndAccepted = c133TerminalQueued;
                    if (c133TerminalQueued) scheduleC133Drain();
                    logC133Callback("up", event, c133LastCumulativeXCssPx,
                            c133LastCumulativeYCssPx, 0.0f, 0.0f, result, callbackStarted);
                }
                resetC133TouchState(true);
                c133TapClickCandidate = tapCandidate && tapEndAccepted;
                if (c133TapProbeRequested) {
                    Log.i(TAG, "SPINON_C133_TAP_ANDROID up_candidate=" + c133TapClickCandidate
                            + " activation_count=" + c133TapActivationCount
                            + " gesture_id=" + c133GestureId);
                }
                return true;
            case MotionEvent.ACTION_CANCEL:
                cancelC133Gesture(0, "motion-event-cancel");
                if (!c133StyleBarrierPending) resetC133TouchState(true);
                return true;
            default:
                return c133TouchActive || c133SuppressUntilAllPointersUp;
        }
    }

    private String enqueueC133Gesture(int action, float xOrCumulativeX,
            float yOrCumulativeY, int reason) {
        final long host;
        synchronized (stateLock) {
            if (closing || hostHandle == 0) return "status=-1 C13.3 host is closing";
            host = hostHandle;
        }
        return decode(nativeEnqueueC133Gesture(host, action, c133GestureSurfaceGeneration,
                c133GestureId, Integer.toUnsignedLong(c133ActivePointerId),
                xOrCumulativeX, yOrCumulativeY, c133CoordinateScale, reason));
    }

    boolean handleC133WheelMotionEvent(MotionEvent event) {
        if (!BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST || !c133GestureFixtureRequested
                || event.getActionMasked() != MotionEvent.ACTION_SCROLL) return false;

        long callbackStarted = SystemClock.elapsedRealtimeNanos();
        InputDevice device = event.getDevice();
        int pointerCount = event.getPointerCount();
        int pointerToolType = pointerCount > 0
                ? event.getToolType(0) : MotionEvent.TOOL_TYPE_UNKNOWN;
        boolean eventTouchpadSource = event.isFromSource(InputDevice.SOURCE_TOUCHPAD);
        boolean eventMouseSource = event.isFromSource(InputDevice.SOURCE_MOUSE);
        int source = WheelSourceClassifier.classify(eventTouchpadSource, eventMouseSource,
                pointerCount > 0, pointerToolType == MotionEvent.TOOL_TYPE_FINGER,
                pointerToolType == MotionEvent.TOOL_TYPE_MOUSE);
        String sourceName = source == C133_WHEEL_SOURCE_TRACKPAD ? "TrackpadScroll"
                : source == C133_WHEEL_SOURCE_MOUSE ? "MouseWheel" : "Unknown";
        // Supported sources are diagnostic only; classify the event from its own source and tool.
        int deviceSupportedSourceBits = device == null ? 0 : device.getSources();
        String sourceDiagnostics = " event_source_bits=0x"
                + Integer.toHexString(event.getSource())
                + " pointer_count=" + pointerCount
                + " pointer_tool=" + c133WheelPointerToolName(pointerToolType)
                + " pointer_tool_type=" + pointerToolType
                + " device_supported_source_bits=0x"
                + Integer.toHexString(deviceSupportedSourceBits)
                + " device=" + event.getDeviceId();
        int modifiers = normalizedC133Modifiers(event.getMetaState());
        float hscroll = event.getAxisValue(MotionEvent.AXIS_HSCROLL);
        float vscroll = event.getAxisValue(MotionEvent.AXIS_VSCROLL);
        if (source < 0) {
            Log.i(TAG, "SPINON_C133_NATIVE_WHEEL_REJECT reason=unknown-source action="
                    + event.getActionMasked() + " source_bits=0x"
                    + Integer.toHexString(event.getSource()) + sourceDiagnostics);
            return true;
        }
        if ((modifiers & C133_MODIFIER_CONTROL) != 0) {
            Log.i(TAG, "SPINON_C133_NATIVE_WHEEL_REJECT reason=control-wheel source="
                    + sourceName + " hscroll=" + hscroll + " vscroll=" + vscroll
                    + " meta=" + event.getMetaState() + " modifiers=" + modifiers
                    + sourceDiagnostics);
            return true;
        }
        if (!Float.isFinite(hscroll) || !Float.isFinite(vscroll)
                || (hscroll == 0.0f && vscroll == 0.0f)) {
            Log.i(TAG, "SPINON_C133_NATIVE_WHEEL_REJECT reason=zero-or-nonfinite-axis source="
                    + sourceName + " hscroll=" + hscroll + " vscroll=" + vscroll
                    + sourceDiagnostics);
            return true;
        }

        final long host;
        final long generation;
        final int width;
        final int height;
        synchronized (stateLock) {
            if (closing || hostHandle == 0 || !surfaceAvailable
                    || !c133GestureFixtureReady || !c133GestureInitialDrawReady
                    || surfaceWidth <= 0 || surfaceHeight <= 0) {
                Log.i(TAG, "SPINON_C133_NATIVE_WHEEL_REJECT reason=surface-or-runtime-not-ready"
                        + " source=" + sourceName + " closing=" + closing
                        + " surface_available=" + surfaceAvailable
                        + " generation=" + surfaceGeneration + sourceDiagnostics);
                return true;
            }
            host = hostHandle;
            generation = surfaceGeneration;
            width = surfaceWidth;
            height = surfaceHeight;
        }

        float scaleX = (float) width / 400.0f;
        float scaleY = (float) height / 240.0f;
        if (!Float.isFinite(scaleX) || !Float.isFinite(scaleY)
                || scaleX <= 0.0f || scaleY <= 0.0f
                || Math.abs(scaleX - scaleY) > 0.01f) {
            Log.e(TAG, "SPINON_C133_NATIVE_WHEEL_REJECT reason=invalid-surface-scale"
                    + " scale_x=" + scaleX + " scale_y=" + scaleY
                    + " surface_px=" + width + "x" + height + sourceDiagnostics);
            return true;
        }

        ViewConfiguration configuration = ViewConfiguration.get(activity);
        float horizontalFactorPx = configuration.getScaledHorizontalScrollFactor();
        float verticalFactorPx = configuration.getScaledVerticalScrollFactor();
        if (!Float.isFinite(horizontalFactorPx) || !Float.isFinite(verticalFactorPx)
                || horizontalFactorPx <= 0.0f || verticalFactorPx <= 0.0f) {
            Log.e(TAG, "SPINON_C133_NATIVE_WHEEL_REJECT reason=invalid-scroll-factor"
                    + " hfactor_px=" + horizontalFactorPx + " vfactor_px=" + verticalFactorPx
                    + sourceDiagnostics);
            return true;
        }

        float rawDeltaXPhysicalPx = hscroll * horizontalFactorPx;
        float rawDeltaYPhysicalPx = -vscroll * verticalFactorPx;
        float deltaXCssPx = rawDeltaXPhysicalPx / scaleX;
        float deltaYCssPx = rawDeltaYPhysicalPx / scaleY;
        if (!Float.isFinite(rawDeltaXPhysicalPx) || !Float.isFinite(rawDeltaYPhysicalPx)
                || !Float.isFinite(deltaXCssPx) || !Float.isFinite(deltaYCssPx)) {
            Log.i(TAG, "SPINON_C133_NATIVE_WHEEL_REJECT reason=nonfinite-normalized-delta"
                    + " source=" + sourceName + " hscroll=" + hscroll + " vscroll=" + vscroll
                    + " hfactor_px=" + horizontalFactorPx + " vfactor_px=" + verticalFactorPx
                    + " scale_x=" + scaleX + " scale_y=" + scaleY + sourceDiagnostics);
            return true;
        }

        int[] surfaceOriginOnScreen = new int[2];
        surfaceView.getLocationOnScreen(surfaceOriginOnScreen);
        float pointSurfaceX = event.getRawX() - surfaceOriginOnScreen[0];
        float pointSurfaceY = event.getRawY() - surfaceOriginOnScreen[1];
        int viewWidth = surfaceView.getWidth();
        int viewHeight = surfaceView.getHeight();
        if (!Float.isFinite(pointSurfaceX) || !Float.isFinite(pointSurfaceY)
                || viewWidth <= 0 || viewHeight <= 0
                || pointSurfaceX < 0.0f || pointSurfaceY < 0.0f
                || pointSurfaceX >= viewWidth || pointSurfaceY >= viewHeight) {
            Log.i(TAG, "SPINON_C133_NATIVE_WHEEL_REJECT reason=point-outside-surface"
                    + " source=" + sourceName + " point_screen_px=" + event.getRawX() + ","
                    + event.getRawY() + " surface_origin_screen_px=" + surfaceOriginOnScreen[0]
                    + "," + surfaceOriginOnScreen[1] + " point_surface_px=" + pointSurfaceX
                    + "," + pointSurfaceY + " surface_view_px=" + viewWidth + "x" + viewHeight
                    + sourceDiagnostics);
            return true;
        }
        float pointXCssPx = pointSurfaceX / scaleX;
        float pointYCssPx = pointSurfaceY / scaleY;
        if (!Float.isFinite(pointXCssPx) || !Float.isFinite(pointYCssPx)) {
            Log.i(TAG, "SPINON_C133_NATIVE_WHEEL_REJECT reason=nonfinite-point source="
                    + sourceName + " point_surface_px=" + pointSurfaceX + "," + pointSurfaceY
                    + sourceDiagnostics);
            return true;
        }

        String report = decode(nativeEnqueueC133Wheel(host, C133_WHEEL_INTENT_SCROLL,
                generation, pointXCssPx, pointYCssPx, deltaXCssPx, deltaYCssPx,
                rawDeltaXPhysicalPx, rawDeltaYPhysicalPx,
                C133_WHEEL_UNIT_ANDROID_PHYSICAL_PIXELS, scaleX, source,
                C133_WHEEL_MOMENTUM_NONE, modifiers));
        long callbackMicros = (SystemClock.elapsedRealtimeNanos() - callbackStarted) / 1000;
        Log.i(TAG, "SPINON_C133_NATIVE_WHEEL source=" + sourceName
                + " action=" + event.getActionMasked() + " source_bits=0x"
                + Integer.toHexString(event.getSource()) + sourceDiagnostics
                + " hscroll=" + hscroll + " vscroll=" + vscroll
                + " hfactor_px=" + horizontalFactorPx + " vfactor_px=" + verticalFactorPx
                + " raw_delta_physical_px=" + rawDeltaXPhysicalPx + ","
                + rawDeltaYPhysicalPx + " scale_px_per_css_px=" + scaleX
                + " point_screen_px=" + event.getRawX() + "," + event.getRawY()
                + " surface_origin_screen_px=" + surfaceOriginOnScreen[0] + ","
                + surfaceOriginOnScreen[1] + " point_css_px=" + pointXCssPx + "," + pointYCssPx
                + " delta_css_px=" + deltaXCssPx + "," + deltaYCssPx
                + " unit=AndroidPhysicalPixels phase=None modifiers=" + modifiers
                + " generation=" + generation + " callback_us=" + callbackMicros
                + " waits_for_worker=false " + report);
        if (report.startsWith("status=0 ") && report.contains("admitted=true")) {
            postStatus("Wheel · " + sourceName + " · Δ " + deltaXCssPx + ","
                    + deltaYCssPx + " CSS px");
            logC133SurfaceBoundsAfterLayout("wheel");
            scheduleC133Drain();
        }
        return true;
    }

    private static String c133WheelPointerToolName(int pointerToolType) {
        switch (pointerToolType) {
            case MotionEvent.TOOL_TYPE_FINGER:
                return "finger";
            case MotionEvent.TOOL_TYPE_MOUSE:
                return "mouse";
            case MotionEvent.TOOL_TYPE_STYLUS:
                return "stylus";
            case MotionEvent.TOOL_TYPE_ERASER:
                return "eraser";
            default:
                return "unknown";
        }
    }

    private static int normalizedC133Modifiers(int metaState) {
        int normalizedMetaState = KeyEvent.normalizeMetaState(metaState);
        int modifiers = 0;
        if ((normalizedMetaState & KeyEvent.META_SHIFT_ON) != 0) modifiers |= C133_MODIFIER_SHIFT;
        if ((normalizedMetaState & KeyEvent.META_CTRL_ON) != 0) modifiers |= C133_MODIFIER_CONTROL;
        if ((normalizedMetaState & KeyEvent.META_ALT_ON) != 0) modifiers |= C133_MODIFIER_ALT;
        if ((normalizedMetaState & KeyEvent.META_META_ON) != 0) modifiers |= C133_MODIFIER_META;
        return modifiers;
    }

    private void runC133WheelProbe() {
        final long host;
        final long generation;
        synchronized (stateLock) {
            if (!c133WheelProbeRequested || c133WheelProbeStarted
                    || !c133GestureFixtureReady || closing || hostHandle == 0) return;
            c133WheelProbeStarted = true;
            host = hostHandle;
            generation = surfaceGeneration;
        }
        float deltaX = c133GesturePolicyScenarioID == 3 ? 0.0f : 40.0f;
        float deltaY = 40.0f;
        String report = decode(nativeEnqueueC133Wheel(host, 0, generation,
                132.0f, 112.0f, deltaX, deltaY, deltaX, deltaY,
                0, 1.0f, 1, 2, 0));
        if (!report.startsWith("status=0 ") || !report.contains("admitted=true")) {
            c133WheelProbeFailed = true;
            Log.e(TAG, "SPINON_C133_WHEEL_PROBE failed=true stage=enqueue " + report);
            postStatus("실패 · C13.3 normalized wheel fixture enqueue · " + report);
            return;
        }
        c133WheelProbeEnqueued = true;
        Log.i(TAG, "SPINON_C133_WHEEL_PROBE_BEGIN synthetic_normalized_fixture_input=true"
                + " scenario=" + c133GesturePolicyScenarioID
                + " point_css_px=132,112 delta_css_px=" + deltaX + "," + deltaY
                + " source=TrackpadScroll unit=CssPixels phase=Change scale=1 " + report);
        scheduleC133Drain();
    }

    private String c133WheelProbeExpectedRuntimeOffsets() {
        switch (c133GesturePolicyScenarioID) {
            case 1: return "offsets=root:0,0;outer:40,40;inner:86,106";
            case 2: return "offsets=root:0,0;outer:0,40;inner:86,106";
            case 3: return "offsets=root:0,0;outer:0,0;inner:0,40";
            default: return "offsets=invalid";
        }
    }

    private String c133WheelProbeExpectedDrawOffsets() {
        switch (c133GesturePolicyScenarioID) {
            case 1: return "scroll_offsets=[0,0;40,40;86,106]";
            case 2: return "scroll_offsets=[0,0;0,40;86,106]";
            case 3: return "scroll_offsets=[0,0;0,0;0,40]";
            default: return "scroll_offsets=invalid";
        }
    }

    private void verifyC133WheelProcessReport(String report) {
        if (!c133WheelProbeRequested || !c133WheelProbeEnqueued || c133WheelProbeProcessed
                || c133WheelProbeFailed || !report.contains("event=wheel")) return;
        String expected = c133WheelProbeExpectedRuntimeOffsets();
        if (report.contains(expected)) {
            c133WheelProbeProcessed = true;
            Log.i(TAG, "SPINON_C133_WHEEL_PROBE_PROCESSED"
                    + " synthetic_normalized_fixture_input=true expected=" + expected
                    + " report=" + report);
        } else {
            c133WheelProbeFailed = true;
            Log.e(TAG, "SPINON_C133_WHEEL_PROBE failed=true stage=queue"
                    + " expected=" + expected + " report=" + report);
            postStatus("실패 · C13.3 wheel queue 결과가 oracle과 다릅니다");
        }
    }

    private void verifyC133WheelDraw(String report) {
        if (!c133WheelProbeRequested || !c133WheelProbeProcessed || c133WheelProbeCompleted
                || c133WheelProbeFailed) return;
        String expectedOffsets = c133WheelProbeExpectedDrawOffsets();
        String expectedRevision = c133GesturePolicyScenarioID == 3
                ? "scroll_state_revision=1" : "scroll_state_revision=2";
        if (!report.contains(expectedOffsets) || !report.contains(expectedRevision)) {
            c133WheelProbeFailed = true;
            Log.e(TAG, "SPINON_C133_WHEEL_PROBE failed=true stage=draw expected="
                    + expectedOffsets + " " + expectedRevision + " report=" + report);
            postStatus("실패 · C13.3 wheel 뒤 Android draw가 기대 offset과 다릅니다");
            return;
        }
        c133WheelProbeCompleted = true;
        Log.i(TAG, "SPINON_C133_WHEEL_PROBE_COMPLETE"
                + " synthetic_normalized_fixture_input=true expected=" + expectedOffsets
                + " " + expectedRevision + " report=" + report);
        postStatus("완료 · synthetic normalized wheel fixture 결과를 Android draw에서 확인했습니다");
    }

    private boolean c133StyleInvalidationScenario() {
        return c133GesturePolicyScenarioID >= 8 && c133GesturePolicyScenarioID <= 10;
    }

    private static String c133StyleUpdateMarker() {
        return new StringBuilder(30)
                .append("SPINON_C133_")
                .append("STYLE_UPDATE ")
                .toString();
    }

    private void completeC133StyleBarrier(boolean updateApplied) {
        if (!c133StyleBarrierPending) return;
        int terminalAction = c133StyleDeferredTerminalAction;
        int terminalReason = c133StyleDeferredTerminalReason;
        boolean queued = false;
        if (!updateApplied) {
            String result = enqueueC133Gesture(4, 0.0f, 0.0f, 4);
            queued = result.startsWith("status=0 ");
            Log.e(TAG, "SPINON_C133_STYLE_BARRIER update_failed=true cancel=" + result);
        } else {
            if (c133StyleDeferredMovePending && terminalAction != 4) {
                String result = enqueueC133Gesture(2,
                        c133StyleDeferredCumulativeX, c133StyleDeferredCumulativeY, 0);
                queued |= result.startsWith("status=0 ");
                Log.i(TAG, "SPINON_C133_STYLE_BARRIER deferred_move=" + result);
            }
            if (terminalAction >= 0) {
                String result = enqueueC133Gesture(terminalAction, 0.0f, 0.0f, terminalReason);
                queued |= result.startsWith("status=0 ");
                Log.i(TAG, "SPINON_C133_STYLE_BARRIER deferred_terminal=" + result);
            }
        }
        c133StyleBarrierPending = false;
        c133StyleDeferredMovePending = false;
        c133StyleDeferredTerminalAction = -1;
        c133StyleDeferredTerminalReason = 0;
        if (queued) scheduleC133Drain();
        if (terminalAction >= 0 || !updateApplied) resetC133TouchState(true);
    }

    private void cancelC133Gesture(int reason, String source) {
        if (!c133TouchActive || c133TerminalQueued) return;
        if (c133StyleBarrierPending) {
            c133StyleDeferredTerminalAction = 4;
            c133StyleDeferredTerminalReason = reason;
            c133TerminalQueued = true;
            c133TouchActive = false;
            Log.i(TAG, "SPINON_C133_STYLE_BARRIER terminal=deferred action=4 reason="
                    + reason + " source=" + source);
            return;
        }
        String result = enqueueC133Gesture(4, 0.0f, 0.0f, reason);
        c133TerminalQueued = result.startsWith("status=0 ");
        if (c133TerminalQueued) scheduleC133Drain();
        Log.i(TAG, "SPINON_C133_TOUCH_CANCEL source=" + source
                + " generation=" + c133GestureSurfaceGeneration + " " + result);
    }

    private void resetC133TouchState(boolean clearSuppression) {
        c133TouchActive = false;
        c133PanStarted = false;
        c133TerminalQueued = false;
        c133ActivePointerId = MotionEvent.INVALID_POINTER_ID;
        c133TapDownAccepted = false;
        c133TapForwardingGesture = false;
        c133TapClickCandidate = false;
        if (clearSuppression) c133SuppressUntilAllPointersUp = false;
    }

    private void onC133TapProbeClick() {
        if (!c133TapClickCandidate) {
            Log.e(TAG, "SPINON_C133_TAP_ANDROID unexpected_click=true activation_count="
                    + c133TapActivationCount + " gesture_id=" + c133GestureId);
            return;
        }
        c133TapClickCandidate = false;
        c133TapActivationCount++;
        Log.i(TAG, "SPINON_C133_TAP_ANDROID activation=true count="
                + c133TapActivationCount + " gesture_id=" + c133GestureId
                + " tap_css_px=" + c133DownXPhysicalPx / c133CoordinateScale + ","
                + c133DownYPhysicalPx / c133CoordinateScale
                + " tap_physical_px=" + c133DownXPhysicalPx + "," + c133DownYPhysicalPx
                + " scale=" + c133CoordinateScale);
    }

    private void logC133Callback(String action, MotionEvent event, float cumulativeX,
            float cumulativeY, float deltaX, float deltaY, String result, long started) {
        long callbackMicros = (SystemClock.elapsedRealtimeNanos() - started) / 1000;
        int[] surfaceOriginOnScreen = new int[2];
        surfaceView.getLocationOnScreen(surfaceOriginOnScreen);
        String eventDiagnostics = "";
        if (BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST) {
            InputDevice device = event.getDevice();
            int pointerCount = event.getPointerCount();
            int pointerToolType = pointerCount > 0
                    ? event.getToolType(0) : MotionEvent.TOOL_TYPE_UNKNOWN;
            eventDiagnostics = " event_source_bits=0x"
                    + Integer.toHexString(event.getSource())
                    + " pointer_count=" + pointerCount
                    + " pointer_tool_type=" + pointerToolType
                    + " device_supported_source_bits=0x"
                    + Integer.toHexString(device == null ? 0 : device.getSources())
                    + " device=" + event.getDeviceId();
        }
        Log.i(TAG, "SPINON_C133_TOUCH source=MotionEvent action=" + action
                + " event_action=" + event.getActionMasked() + eventDiagnostics
                + " pointer_id=" + c133ActivePointerId
                + " surface_local_physical_px=" + event.getX() + "," + event.getY()
                + " screen_physical_px=" + event.getRawX() + "," + event.getRawY()
                + " surface_origin_screen_px=" + surfaceOriginOnScreen[0] + ","
                + surfaceOriginOnScreen[1]
                + " surface_view_size_px=" + surfaceView.getWidth() + "x"
                + surfaceView.getHeight()
                + " cumulative_css_px=" + cumulativeX + "," + cumulativeY
                + " delta_css_px=" + deltaX + "," + deltaY
                + " scale=" + c133CoordinateScale
                + " slop_source=ViewConfiguration.scaledTouchSlop slop_px=" + c133TouchSlopPx
                + " generation=" + c133GestureSurfaceGeneration
                + " callback_us=" + callbackMicros + " waits_for_worker=false " + result);
    }

    private void runC133SyntheticSecondPointerProbe() {
        if (!c133GestureFixtureReady || !c133GestureInitialDrawReady || closing) return;
        final float scale;
        synchronized (stateLock) {
            if (surfaceWidth <= 0 || surfaceHeight <= 0) return;
            scale = (float) surfaceWidth / 400.0f;
        }
        float x = 148.0f * scale;
        float y = 132.0f * scale;
        long downTime = SystemClock.uptimeMillis();
        dispatchC133SyntheticSingle(downTime, MotionEvent.ACTION_DOWN, x, y);
        dispatchC133SyntheticSingle(downTime, MotionEvent.ACTION_MOVE, x, y - 30.0f);

        MotionEvent.PointerProperties first = new MotionEvent.PointerProperties();
        first.id = 0;
        first.toolType = MotionEvent.TOOL_TYPE_FINGER;
        MotionEvent.PointerProperties second = new MotionEvent.PointerProperties();
        second.id = 1;
        second.toolType = MotionEvent.TOOL_TYPE_FINGER;
        MotionEvent.PointerCoords firstCoords = c133SyntheticPointerCoords(x, y - 30.0f);
        MotionEvent.PointerCoords secondCoords = c133SyntheticPointerCoords(x + 24.0f, y - 30.0f);
        MotionEvent.PointerProperties[] properties = {first, second};
        MotionEvent.PointerCoords[] coordinates = {firstCoords, secondCoords};
        int pointerAction = MotionEvent.ACTION_POINTER_DOWN
                | (1 << MotionEvent.ACTION_POINTER_INDEX_SHIFT);
        dispatchC133SyntheticMulti(downTime, pointerAction, properties, coordinates);
        MotionEvent.PointerCoords[] movedCoordinates = {
                c133SyntheticPointerCoords(x, y - 54.0f),
                c133SyntheticPointerCoords(x + 24.0f, y - 54.0f),
        };
        dispatchC133SyntheticMulti(downTime, MotionEvent.ACTION_MOVE,
                properties, movedCoordinates);
        dispatchC133SyntheticMulti(downTime,
                MotionEvent.ACTION_POINTER_UP | (1 << MotionEvent.ACTION_POINTER_INDEX_SHIFT),
                properties, movedCoordinates);
        dispatchC133SyntheticSingle(downTime, MotionEvent.ACTION_UP, x, y - 54.0f);
        Log.i(TAG, "SPINON_C133_SECOND_POINTER_PROBE synthetic_motion_event=true dispatched=true");
    }

    private void dispatchC133SyntheticSingle(long downTime, int action, float x, float y) {
        MotionEvent.PointerProperties pointer = new MotionEvent.PointerProperties();
        pointer.id = 0;
        pointer.toolType = MotionEvent.TOOL_TYPE_FINGER;
        dispatchC133SyntheticMulti(downTime, action,
                new MotionEvent.PointerProperties[] {pointer},
                new MotionEvent.PointerCoords[] {c133SyntheticPointerCoords(x, y)});
    }

    private void dispatchC133SyntheticMulti(long downTime, int action,
            MotionEvent.PointerProperties[] properties, MotionEvent.PointerCoords[] coordinates) {
        MotionEvent event = MotionEvent.obtain(
                downTime, SystemClock.uptimeMillis(), action, properties.length,
                properties, coordinates, 0, 0, 1.0f, 1.0f, 0, 0,
                InputDevice.SOURCE_TOUCHSCREEN, 0);
        try {
            surfaceView.dispatchTouchEvent(event);
        } finally {
            event.recycle();
        }
    }

    private static MotionEvent.PointerCoords c133SyntheticPointerCoords(float x, float y) {
        MotionEvent.PointerCoords coordinates = new MotionEvent.PointerCoords();
        coordinates.x = x;
        coordinates.y = y;
        coordinates.pressure = 1.0f;
        coordinates.size = 1.0f;
        return coordinates;
    }

    private void scheduleC133Drain() {
        c133DrainAgain.set(true);
        if (!c133DrainScheduled.compareAndSet(false, true)) return;
        synchronized (stateLock) {
            if (closing || hostHandle == 0) {
                c133DrainScheduled.set(false);
                return;
            }
            runtimeQueue.execute(() -> {
                try {
                    while (c133DrainAgain.getAndSet(false)) {
                        final long host;
                        synchronized (stateLock) {
                            host = hostHandle;
                        }
                        if (host == 0) break;
                        if (c133WorkerDelayMillis > 0 && !c133WorkerDelayUsed) {
                            c133WorkerDelayUsed = true;
                            Log.i(TAG, "SPINON_C133_WORKER_DELAY_BEGIN millis="
                                    + c133WorkerDelayMillis);
                            SystemClock.sleep(c133WorkerDelayMillis);
                            Log.i(TAG, "SPINON_C133_WORKER_DELAY_END millis="
                                    + c133WorkerDelayMillis);
                        }
                        String report = decode(nativeProcessC133GestureQueue(host));
                        Log.i(TAG, "SPINON_C133_INPUT_BATCH " + report);
                        if (!report.startsWith("status=0 ")) {
                            Log.e(TAG, "SPINON_C133_INPUT_BATCH_ERROR " + report);
                            continue;
                        }
                        verifyC133WheelProcessReport(report);
                        if (c133StyleInvalidationScenario() && !c133StyleMutationApplied
                                && report.contains("result=pan_started")) {
                            String styleReport = decode(nativeApplyC133StyleInvalidation(host));
                            boolean updateApplied = styleReport.startsWith("status=0 ");
                            Log.i(TAG, c133StyleUpdateMarker() + styleReport);
                            if (updateApplied) c133StyleMutationApplied = true;
                            surfaceView.post(() -> completeC133StyleBarrier(updateApplied));
                            if (updateApplied) requestDraw();
                        }
                        requestDraw();
                    }
                } finally {
                    c133DrainScheduled.set(false);
                    if (c133DrainAgain.get()) scheduleC133Drain();
                }
            });
        }
    }

    private long beginPresentationUpdate() {
        synchronized (presentationUpdateLock) {
            final long host;
            synchronized (stateLock) {
                if (closing || hostHandle == 0) return 0;
                host = hostHandle;
                pendingDraw = false;
            }
            long sequence = nativeBeginPresentationUpdate(host);
            synchronized (stateLock) {
                if (!closing && hostHandle == host && sequence != 0) {
                    hostPresentationSequence = sequence;
                }
            }
            return sequence;
        }
    }

    private void enqueueRuntime(Runnable work) {
        synchronized (stateLock) {
            if (closing) return;
            runtimeQueue.execute(work);
        }
    }

}
