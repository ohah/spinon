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

import java.nio.charset.StandardCharsets;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

final class C0410RuntimeGpuDemo extends LinearLayout implements SurfaceHolder.Callback {
    private static final String TAG = "SpinonBootstrap";
    private static native long nativeCreateHost(boolean registeredPropertiesFixture);
    private static native long nativeCreateBlockPaintHost();
    private static native long nativeCreateBlockFormattingHost();
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
    private static native long nativeCreateSurface(
            long host, Surface surface, int width, int height, int backend);
    private static native int nativeResizeSurface(long renderer, int width, int height);
    private static native byte[] nativeDrawSurface(long renderer);
    private static native byte[] nativeInjectNextDrawFailure(long renderer);
    private static native void nativeDestroySurface(long renderer);
    private static native void nativeFreeHost(long host);

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
    private final boolean registeredPropertiesProbeRequested;
    private final boolean authorStylesheetsProbeRequested;
    private final float density;
    private volatile boolean darkMode;
    private volatile long hostHandle;
    private volatile long rendererHandle;
    private volatile int surfaceWidth;
    private volatile int surfaceHeight;
    private int surfaceFormat = -1;
    private volatile long surfaceGeneration;
    private long rendererGeneration = -1;
    private volatile boolean surfaceAvailable;
    private volatile boolean closing;
    private long hostPresentationSequence;
    private boolean expandedSurface;
    private boolean pendingDraw;
    private boolean failureProbeStarted;
    private boolean shutdownProbeStarted;
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
        blockFormattingProbeRequested = activity.getIntent()
                .getBooleanExtra("spinon_c091_block_formatting", false);
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
        setPadding(dp(22, density), dp(44, density), dp(22, density), dp(24, density));
        setBackgroundColor(Color.rgb(14, 19, 31));

        TextView title = new TextView(activity);
        title.setText(blockFormattingProbeRequested
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
        description.setText(blockFormattingProbeRequested
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
        surfaceView.setContentDescription(blockFormattingProbeRequested
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
        int initialSurfaceWidth = blockFormattingProbeRequested ? 320 : 301;
        int initialSurfaceHeight = blockFormattingProbeRequested ? 240 : 100;
        LayoutParams surfaceParams = new LayoutParams(
                dp(initialSurfaceWidth, density), dp(initialSurfaceHeight, density));
        stage.addView(surfaceView, surfaceParams);

        Button resizeButton = new Button(activity);
        resizeButton.setText("표면 크기 전환 · 301×100 CSS px");
        resizeButton.setOnClickListener(view -> toggleSurfaceSize(resizeButton));
        if (blockFormattingProbeRequested) resizeButton.setVisibility(GONE);
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
        if (authorStylesheetsProbeRequested || viewportUnitsProbeRequested || blockFormattingProbeRequested || blockPaintProbeRequested || aspectRatioProbeRequested
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
        enqueueRuntime(this::initializeRuntime);
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
        long host = blockFormattingProbeRequested
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
            widthCssPx = blockFormattingProbeRequested ? 320 : cssViewportWidthLocked();
            heightCssPx = blockFormattingProbeRequested ? 240 : cssViewportHeightLocked();
            dark = darkMode;
        }
        String environment = decode(nativeSetEnvironment(host, widthCssPx, heightCssPx, density, dark));
        if (!environment.startsWith("status=0 ")) {
            postStatus("실패 · " + environment);
            return;
        }
        Log.i(TAG, "SPINON_C0410_ENVIRONMENT viewport=" + widthCssPx + "x" + heightCssPx
                + " scale=" + density + " dark=" + dark + " " + environment);
        String result = viewportUnitsProbeRequested
                ? decode(nativeEvalViewportUnitsFixture(host))
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
            postStatus("실패 · " + result);
            return;
        }
        Log.i(TAG, (blockFormattingProbeRequested
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
        int layoutStart = report.indexOf(" layout=");
        if (statusEnd >= 0 && layoutStart >= 0) {
            return report.substring(0, statusEnd) + " " + report.substring(layoutStart + 1);
        }
        return report.length() <= 180 ? report : report.substring(0, 180) + "…";
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
        postStatus(result);
        if (result.startsWith("status=0 ") && isCurrentPresentation(generation, sequence)) {
            requestDraw();
        }
    }

    private float cssViewportWidthLocked() {
        return surfaceWidth > 0 ? (float) surfaceWidth / density : 301.0f;
    }

    private float cssViewportHeightLocked() {
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
        if (!result.startsWith("status=0 ")) {
            postStatus("실패 · " + result);
            return;
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
