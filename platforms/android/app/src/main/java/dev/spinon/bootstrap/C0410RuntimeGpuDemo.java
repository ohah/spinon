package dev.spinon.bootstrap;

import android.app.Activity;
import android.graphics.Color;
import android.graphics.Typeface;
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
import java.util.List;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

final class C0410RuntimeGpuDemo extends LinearLayout implements SurfaceHolder.Callback {
    private static final String TAG = "SpinonBootstrap";
    private static native long nativeCreateHost(boolean registeredPropertiesFixture);
    private static native long nativeCreateBlockPaintHost();
    private static native long nativeCreateBlockFormattingHost();
    private static native long nativeCreateC121PositionHost();
    private static native long nativeCreateC122AbsoluteBlockHost();
    private static native long nativeCreateC123FixedHost();
    private static native long nativeCreateC124StackingHost();
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
    private boolean c124SurfaceCaptureStarted;
    private List<C124SurfacePixelCapture.Sample> c124SurfaceSamples = List.of();
    private int c124SurfaceOriginXCssPx;
    private int c124SurfaceOriginYCssPx;
    private String c124SurfaceInventorySha256 = "";
    private CountDownLatch surfaceDestroyBarrier;

    C0410RuntimeGpuDemo(
            Activity activity,
            int backend,
            boolean failureProbeRequested,
            boolean shutdownProbeRequested
    ) {
        super(activity);
        this.activity = activity;
        this.backend = backend;
        this.failureProbeRequested = failureProbeRequested;
        this.shutdownProbeRequested = shutdownProbeRequested;
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
        c1035PositionedFlexProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c1035_positioned_flex", false);
        blockFormattingProbeRequested = flowRootProbeRequested || marginCollapseProbeRequested
                || activity.getIntent().getBooleanExtra("spinon_c091_block_formatting", false);
        fixedSizeCssFixtureRequested = c122AbsoluteBlockProbeRequested
                || c123FixedPositionProbeRequested
                || c124StackingProbeRequested
                || blockFormattingProbeRequested || flexWrapProbeRequested
                || flexDistributionProbeRequested || flexReverseProbeRequested
                || flexOrderProbeRequested || flexAlignmentProbeRequested
                || flexBaselineProbeRequested || c1035PositionedFlexProbeRequested;
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
        title.setText(c1035PositionedFlexProbeRequested
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
        description.setText(c1035PositionedFlexProbeRequested
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
        surfaceView.setContentDescription(c1035PositionedFlexProbeRequested
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
                && !c123FixedPositionProbeRequested && !c124StackingProbeRequested) {
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
        long host = c124StackingProbeRequested
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
            widthCssPx = c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
                    ? 360 : fixedSizeCssFixtureRequested ? 320 : cssViewportWidthLocked();
            heightCssPx = c122AbsoluteBlockProbeRequested || c123FixedPositionProbeRequested
                    ? 800 : fixedSizeCssFixtureRequested ? 240 : cssViewportHeightLocked();
            dark = darkMode;
        }
        float environmentScale = c123FixedPositionProbeRequested ? 1.0f : density;
        String environment = decode(nativeSetEnvironment(
                host, widthCssPx, heightCssPx, environmentScale, dark));
        if (!environment.startsWith("status=0 ")) {
            postStatus("실패 · " + environment);
            return;
        }
        Log.i(TAG, "SPINON_C0410_ENVIRONMENT viewport=" + widthCssPx + "x" + heightCssPx
                + " scale=" + environmentScale + " dark=" + dark + " " + environment);
        String result = c124SurfaceCaptureProbeRequested
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
        if (!result.startsWith("status=0 ")) {
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
        Log.i(TAG, (c1035PositionedFlexProbeRequested
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
                + " platform=android-avd");
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

    private void schedulePositioningInitializationForReadySurface() {
        if (!c121PositioningProbeRequested && !c122AbsoluteBlockProbeRequested
                && !c123FixedPositionProbeRequested && !c124StackingProbeRequested) return;
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
        postStatus(runtimeStatusSummary(result));
        if (result.startsWith("status=0 ") && isCurrentPresentation(generation, sequence)) {
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
        schedulePositioningInitializationForReadySurface();
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
