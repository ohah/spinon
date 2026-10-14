package dev.spinon.bootstrap;

import android.app.Activity;
import android.graphics.Color;
import android.graphics.Typeface;
import android.os.Build;
import android.os.Debug;
import android.os.Handler;
import android.os.Looper;
import android.os.SystemClock;
import android.util.Log;
import android.view.Gravity;
import android.view.Surface;
import android.view.SurfaceHolder;
import android.view.SurfaceView;
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
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

final class C0410RuntimeGpuDemo extends LinearLayout implements SurfaceHolder.Callback {
    private static final String TAG = "SpinonBootstrap";
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
    };
    private static final String C1133_GRID_TEMPLATE_AREAS_REJECT_SCENE_ID =
            "area-track-count-mismatch";
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
        return false;
    }
    private static boolean isC1133GridTemplateAreasPixelSamples(
            List<SurfaceViewPixelCapture.Sample> samples
    ) {
        if (samples.size() != 2) return false;
        boolean background = false;
        boolean blue = false;
        for (SurfaceViewPixelCapture.Sample sample : samples) {
            switch (sample.id) {
                case "grid-background":
                    background = sample.xCssPx == 150 && sample.yCssPx == 10;
                    if (!background) return false;
                    break;
                case "item-blue":
                    blue = sample.xCssPx == 100 && sample.yCssPx == 50;
                    if (!blue) return false;
                    break;
                default:
                    return false;
            }
        }
        return background && blue;
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
    private static native long nativeCreateBlockPaintHost();
    private static native long nativeCreateBlockFormattingHost();
    private static native long nativeCreateC121PositionHost();
    private static native long nativeCreateC122AbsoluteBlockHost();
    private static native long nativeCreateC123FixedHost();
    private static native long nativeCreateC124StackingHost();
    private static native long nativeCreateC11GridHost();
    private static native long nativeCreateC1121GridPlacementHost();
    private static native long nativeCreateC1122GridAutoPlacementHost();
    private static native long nativeCreateC1123GridAutoPlacementHost();
    private static native long nativeCreateC1131GridRepeatHost();
    private static native long nativeCreateC1132GridNamedLinesHost();
    private static native long nativeCreateC1133GridTemplateAreasHost();
    private static native byte[] nativeC124WorkerStackResidencySample(long host);
    private static native long nativeCreateC1035PositionedFlexHost();
    private static native long nativeBeginPresentationUpdate(long host);
    private static native byte[] nativeSetEnvironment(
            long host, float widthCssPx, float heightCssPx, float scale, boolean dark);
    private static native byte[] nativeEvalFixture(long host);
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
    private static native byte[] nativeEvalBlockFormattingFixture(long host);
    private static native byte[] nativeEvalC121PositionFixture(long host);
    private static native byte[] nativeEvalC122AbsoluteBlockFixture(long host);
    private static native byte[] nativeEvalC123FixedFixture(long host);
    private static native byte[] nativeEvalC124StackingFixture(long host);
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
    private static native byte[] nativeInjectNextDrawFailure(long renderer);
    private static native void nativeDestroySurface(long renderer);
    private static native void nativeFreeHost(long host);

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
    private boolean c11GridSurfaceCaptureStarted;
    private List<C124SurfacePixelCapture.Sample> c124SurfaceSamples = List.of();
    private int c124SurfaceOriginXCssPx;
    private int c124SurfaceOriginYCssPx;
    private String c124SurfaceInventorySha256 = "";
    private CountDownLatch surfaceDestroyBarrier;

    C0410RuntimeGpuDemo(
            Activity activity,
            int backend,
            boolean failureProbeRequested,
            boolean shutdownProbeRequested,
            int c11GridSceneIndex
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
        c11GridFixtureRequested = c1121GridPlacementRequested || c1122GridAutoPlacementRequested
                || c1123GridAutoPlacementRequested || c1131GridRepeatRequested
                || c1132GridNamedLinesRequested || c1133GridTemplateAreasRequested
                || activity.getIntent()
                .getBooleanExtra("spinon_c11_grid_fixture", false)
                || activity.getIntent().getStringExtra("spinon_c11_grid_case") != null;
        String pixelMatrixRunId = activity.getIntent()
                .getStringExtra("spinon_c11_grid_pixel_matrix_run_id");
        String pixelMatrixSceneId = activity.getIntent().getStringExtra("spinon_c11_grid_case");
        String pixelMatrixEncodedSamples = activity.getIntent()
                .getStringExtra("spinon_c11_grid_pixel_matrix_samples");
        boolean pixelMatrixRequested = pixelMatrixRunId != null || pixelMatrixEncodedSamples != null;
        boolean pixelMatrixSceneMatches = c1133GridTemplateAreasRequested
                ? C1133_GRID_TEMPLATE_AREAS_DEFAULT_SCENE_ID.equals(pixelMatrixSceneId)
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
                            && !isC1133GridTemplateAreasPixelSamples(pixelMatrixSamples)) {
                        pixelMatrixSamples = List.of();
                        pixelMatrixConfigError = "invalid-c1133-sample-fixture";
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
            String marker = c1133GridTemplateAreasRequested ? "SPINON_C1133_GRID_PIXEL_MATRIX"
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
                || c123FixedPositionProbeRequested
                || c11GridFixtureRequested
                || c124StackingProbeRequested
                || blockFormattingProbeRequested || flexWrapProbeRequested
                || flexDistributionProbeRequested || flexReverseProbeRequested
                || flexOrderProbeRequested || flexAlignmentProbeRequested
                || flexBaselineProbeRequested || c1035PositionedFlexProbeRequested;
        if (!c1131GridRepeatRequested && !c1132GridNamedLinesRequested
                && !c1133GridTemplateAreasRequested
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
        int horizontalPadding = dp(22, density);
        int topPadding = dp(44, density);
        int bottomPadding = dp(24, density);
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
        title.setText(c1133GridTemplateAreasRequested
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
                        ? "SPINON · C04.11 CSS → WGPU" : "SPINON · C04.10 CSS → WGPU");
        title.setTextColor(Color.rgb(235, 241, 250));
        title.setTextSize(22);
        title.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        addView(title, new LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.WRAP_CONTENT));

        TextView description = new TextView(activity);
        description.setText(c1133GridTemplateAreasRequested
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
        surfaceView.setContentDescription(c1123GridAutoPlacementRequested
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
                : "C04.10 Chromium fixture의 WGPU 장면");
        int initialSurfaceWidth = fixedSizeCssFixtureRequested ? 320 : 301;
        int initialSurfaceHeight = fixedSizeCssFixtureRequested ? 240 : 100;
        LayoutParams surfaceParams = new LayoutParams(
                dp(initialSurfaceWidth, density), dp(initialSurfaceHeight, density));
        stage.addView(surfaceView, surfaceParams);

        Button resizeButton = new Button(activity);
        resizeButton.setText("표면 크기 전환 · 301×100 CSS px");
        resizeButton.setOnClickListener(view -> toggleSurfaceSize(resizeButton));
        if (fixedSizeCssFixtureRequested) resizeButton.setVisibility(GONE);
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
        LayoutParams customPropertiesButtonParams =
                new LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.WRAP_CONTENT);
        customPropertiesButtonParams.topMargin = dp(4, density);
        addView(customPropertiesButton, customPropertiesButtonParams);

        status = new TextView(activity);
        status.setText("V8·CSS runtime 준비 중…");
        status.setTextColor(Color.rgb(97, 185, 255));
        status.setTextSize(13);
        status.setTypeface(Typeface.MONOSPACE);
        addView(status, new LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.WRAP_CONTENT));

        runtimePresentationLane = new C0410LatestTaskLane(
                runtimeQueue,
                this::applyLatestEnvironment,
                error -> reportLaneFailure("runtime", error));
        renderLane = new C0410LatestTaskLane(
                renderQueue,
                this::reconcileRenderState,
                error -> reportLaneFailure("render", error));
        if (!c121PositioningProbeRequested && !c122AbsoluteBlockProbeRequested
                && !c123FixedPositionProbeRequested && !c124StackingProbeRequested
                && !c1133GridTemplateAreasRequested) {
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

    private void initializeRuntime() {
        if (!c11GridPixelMatrixConfigError.isEmpty()) {
            Log.e(TAG, (c1133GridTemplateAreasRequested ? "SPINON_C1133_GRID_PIXEL_MATRIX"
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
        long host = c1133GridTemplateAreasRequested
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
                : blockPaintProbeRequested
                ? nativeCreateBlockPaintHost() : nativeCreateHost(registeredPropertiesProbeRequested);
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
            widthCssPx = c1133GridTemplateAreasRequested
                    || c1132GridNamedLinesRequested || c1131GridRepeatRequested ? 320
                    : c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
                    || c1122GridAutoPlacementRequested || c1123GridAutoPlacementRequested
                    ? 360 : fixedSizeCssFixtureRequested ? 320 : cssViewportWidthLocked();
            heightCssPx = c1133GridTemplateAreasRequested
                    || c1132GridNamedLinesRequested || c1131GridRepeatRequested ? 240
                    : c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
                    || c1122GridAutoPlacementRequested || c1123GridAutoPlacementRequested
                    ? 800 : fixedSizeCssFixtureRequested ? 240 : cssViewportHeightLocked();
            dark = darkMode;
        }
        float environmentScale = c123FixedPositionProbeRequested || c1131GridRepeatRequested
                || c1132GridNamedLinesRequested || c1133GridTemplateAreasRequested
                ? 1.0f : density;
        String environment = decode(nativeSetEnvironment(
                host, widthCssPx, heightCssPx, environmentScale, dark));
        if (!environment.startsWith("status=0 ")) {
            postStatus("실패 · " + environment);
            return;
        }
        Log.i(TAG, "SPINON_C0410_ENVIRONMENT viewport=" + widthCssPx + "x" + heightCssPx
                + " scale=" + environmentScale + " dark=" + dark + " " + environment);
        String result = c1133GridTemplateAreasRequested
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
        if (c1133GridTemplateAreasRequested) {
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
                        : result.contains("unsupported_named_grid_line")
                                ? "unsupported_named_grid_line" : "missing";
                String property = result.contains("property=Some(\"grid-template-areas\")")
                        ? "grid-template-areas"
                        : result.contains("property=Some(\"grid-column-start\")")
                                ? "grid-column-start"
                                : result.contains("property=Some(\"grid-row-start\")")
                                        ? "grid-row-start" : "missing";
                java.util.regex.Matcher nodeMatcher = java.util.regex.Pattern
                        .compile("node=Some\\((\\d+)\\)").matcher(result);
                String nodeId = nodeMatcher.find() ? nodeMatcher.group(1) : "missing";
                String expectedNodeId = "area-track-count-mismatch".equals(c1133GridTemplateAreasSceneId)
                        ? "1" : "unknown-area-implicit-tracks".equals(c1133GridTemplateAreasSceneId)
                                ? "2" : "missing";
                boolean expected = result.startsWith("status=-11 ")
                        && (("area-track-count-mismatch".equals(c1133GridTemplateAreasSceneId)
                                && "unsupported_computed_value".equals(code)
                                && "grid-template-areas".equals(property)
                                && expectedNodeId.equals(nodeId))
                        || ("unknown-area-implicit-tracks".equals(c1133GridTemplateAreasSceneId)
                                && "unsupported_named_grid_line".equals(code)
                                && "grid-row-start".equals(property)
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
        if (c124DepthSafetyProbeRequested) {
            try {
                c124FixtureNodeCount = Integer.parseInt(reportField(result, "document_nodes="));
            } catch (NumberFormatException error) {
                postStatus("실패 · C12.4 fixture 기본 노드 수를 읽지 못했습니다");
                return;
            }
        }
        Log.i(TAG, (c1133GridTemplateAreasRequested
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
                ? "SPINON_C066A_EVAL " : blockPaintProbeRequested
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

    private String readAsset(String name) throws IOException {
        try (InputStream input = activity.getAssets().open(name);
                ByteArrayOutputStream output = new ByteArrayOutputStream()) {
            byte[] buffer = new byte[8192];
            int count;
            while ((count = input.read(buffer)) != -1) output.write(buffer, 0, count);
            return output.toString(StandardCharsets.UTF_8.name());
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
            samples.add(new SurfaceViewPixelCapture.Sample(id, x, y));
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
        String marker = c1133GridTemplateAreasRequested ? "SPINON_C1133_GRID_PIXEL_MATRIX"
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
        String cacheDirectory = c1133GridTemplateAreasRequested
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
                                || c1132GridNamedLinesRequested || c1133GridTemplateAreasRequested;
                        int[] expected = verifyExactColor
                                ? c11GridExpectedRgb(pixel.id, c1133GridTemplateAreasRequested) : null;
                        boolean exact = !verifyExactColor
                                || (pixel.red == expected[0] && pixel.green == expected[1]
                                        && pixel.blue == expected[2] && pixel.alpha == 255);
                        if (exact) matchedSamples++;
                        Log.i(TAG, marker + "_SAMPLE run_id=" + runId
                                + " pid=" + android.os.Process.myPid() + " scene=" + sceneId
                                + " generation=" + generation + " sample=" + pixel.id
                                + " css_x=" + sample.xCssPx + " css_y=" + sample.yCssPx
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
                                    || c1133GridTemplateAreasRequested)
                                    || matchedSamples == samples.size());
                    String status = !complete ? "error"
                            : c1131GridRepeatRequested || c1132GridNamedLinesRequested
                                    || c1133GridTemplateAreasRequested
                            ? (matched ? "matched" : "mismatch") : "captured";
                    Log.i(TAG, marker + "_SUMMARY status="
                            + status + " run_id=" + runId
                            + " scene=" + sceneId + " generation=" + generation
                            + " expected_samples=" + samples.size()
                            + " observed_samples=" + result.pixels.size()
                            + (c1131GridRepeatRequested || c1132GridNamedLinesRequested
                                    || c1133GridTemplateAreasRequested
                                    ? " matched_samples=" + matchedSamples : ""));
                    if (!matched) postStatus(c1133GridTemplateAreasRequested
                            ? "실패 · C11.3.3 Grid PixelCopy 표본 불일치"
                            : c1132GridNamedLinesRequested
                            ? "실패 · C11.3.2 Grid PixelCopy 표본 불일치"
                            : c1131GridRepeatRequested
                            ? "실패 · C11.3.1 Grid PixelCopy 표본 불일치"
                            : "실패 · C11 Grid PixelCopy 표본 누락");
                });
    }

    private static int[] c11GridExpectedRgb(String sampleId, boolean c1133TemplateAreas) {
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
        if (!c121PositioningProbeRequested && !c122AbsoluteBlockProbeRequested
                && !c123FixedPositionProbeRequested && !c124StackingProbeRequested
                && !c1133GridTemplateAreasRequested) return;
        synchronized (stateLock) {
            if (closing || hostHandle != 0 || runtimeInitializationQueued
                    || !surfaceAvailable || surfaceWidth <= 0 || surfaceHeight <= 0) {
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
        synchronized (stateLock) {
            if (closing || !surfaceAvailable || hostHandle == 0) return;
            host = hostHandle;
            generation = surfaceGeneration;
            sequence = hostPresentationSequence;
            widthCssPx = cssViewportWidthLocked();
            heightCssPx = cssViewportHeightLocked();
            dark = darkMode;
        }
        if (!isCurrentPresentation(generation, sequence)) return;
        String result = decode(nativeSetEnvironment(
                host, widthCssPx, heightCssPx, density, dark));
        Log.i(TAG, "SPINON_C0410_ENVIRONMENT viewport=" + widthCssPx + "x" + heightCssPx
                + " scale=" + density + " dark=" + dark + " sequence=" + sequence
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
        if (fixedSizeCssFixtureRequested) return 320.0f;
        return surfaceWidth > 0 ? (float) surfaceWidth / density : 301.0f;
    }

    private float cssViewportHeightLocked() {
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
        if (result.startsWith("status=-12 ")) {
            Log.i(TAG, "SPINON_C0410_DRAW_SUPERSEDED generation=" + generation);
            requestDraw();
            return;
        }
        if (!result.startsWith("status=0 ")) {
            postStatus("실패 · " + result);
            return;
        }
        if (c11GridFixtureRequested && !c11GridPixelMatrixRunId.isEmpty()) {
            requestC11GridPixelMatrixCapture(renderer, generation);
        } else if (c11GridFixtureRequested && !c1121GridPlacementRequested
                && !c1122GridAutoPlacementRequested
                && !c1123GridAutoPlacementRequested
                && !c1131GridRepeatRequested
                && c11GridSceneIndex == 0) {
            requestC11GridSurfaceCapture(renderer, generation);
        }
        if (c124SurfaceCaptureProbeRequested) {
            requestC124SurfaceCapture(renderer, generation);
        }
        if (failureProbeRequested && !failureProbeStarted) {
            failureProbeStarted = true;
            runFailureRecoveryProbe(renderer, result);
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

    private static String decode(byte[] value) {
        return value == null ? "status=-1 native bridge returned no report"
                : new String(value, StandardCharsets.UTF_8);
    }

    void dispose() {
        final long generation;
        synchronized (stateLock) {
            if (closing) return;
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
