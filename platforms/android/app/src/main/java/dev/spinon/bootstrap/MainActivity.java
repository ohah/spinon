package dev.spinon.bootstrap;

import android.app.Activity;
import android.content.Intent;
import android.content.res.Configuration;
import android.content.res.ColorStateList;
import android.graphics.Color;
import android.graphics.Typeface;
import android.graphics.drawable.GradientDrawable;
import android.graphics.drawable.StateListDrawable;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.os.Trace;
import android.util.DisplayMetrics;
import android.util.Log;
import android.view.Gravity;
import android.view.View;
import android.widget.Button;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.RejectedExecutionException;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.ThreadPoolExecutor;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.BiConsumer;

public final class MainActivity extends Activity {
    private static final int MAX_RUNTIME_LOG_CHARS = 20_000;
    private static final int RUNTIME_LOG_TRIM_TARGET_CHARS = 16_000;
    private static final String RUNTIME_LOG_TRUNCATION_MARKER = "… 이전 로그 생략 · 최근 기록만 표시 …\n\n";
    private static final String TAG = "SpinonBootstrap";
    private static final int RUNTIME_QUEUE_CAPACITY = 64;
    private static final String LIFECYCLE_FIXTURE_ASSET = "s03-lifecycle-probe.js";
    private static final String LIFECYCLE_FIXTURE_GLOBAL = "__spinonS03LifecycleProbeV1";
    private static final int LIFECYCLE_STRESS_ROUNDS = 6;
    private static final int LIFECYCLE_STRESS_CYCLES_PER_ROUND = 32;
    private static final int LIFECYCLE_STRESS_NODES_PER_ROUND =
            LIFECYCLE_STRESS_CYCLES_PER_ROUND * 2;
    private static final int LIFECYCLE_LARGE_REGISTRY_NODES = 16_385;
    private static final int LIFECYCLE_LARGE_REGISTRY_SCAN_SAMPLES = 10;

    static {
        System.loadLibrary("spinon_bootstrap");
    }

    private static native byte[] nativeRun(byte[] sourceUtf8, float width, float height,
                                           float density, boolean runR10);
    private static native long nativeSessionCreate();
    private static native byte[] nativeSessionEval(long session, byte[] sourceUtf8);
    private static native byte[] nativeSessionDispatch(long session, int nodeId);
    private static native byte[] nativeSessionPriorityProbe();
    private static native byte[] nativeSessionPriorityFairnessProbe();
    private static native byte[] nativeSessionShutdownProbe();
    private static native byte[] nativeSessionUaCascadeProbe();
    private static native int nativeSessionCancel(long session);
    private static native int nativeSessionMemoryPressure(long session, int level);
    private static native void nativeSessionFree(long session);

    private final ThreadPoolExecutor runtimeCalls = new ThreadPoolExecutor(
            4, 4, 0L, TimeUnit.MILLISECONDS,
            new ArrayBlockingQueue<>(RUNTIME_QUEUE_CAPACITY),
            runnable -> new Thread(runnable, "spinon-platform-call"),
            new ThreadPoolExecutor.AbortPolicy());
    private final ExecutorService runtimeControl = Executors.newSingleThreadExecutor(
            runnable -> new Thread(runnable, "spinon-runtime-control"));
    private final ExecutorService bootstrapExecutor = Executors.newSingleThreadExecutor(
            runnable -> new Thread(runnable, "spinon-bootstrap"));
    private final ScheduledExecutorService delayedHost = Executors.newSingleThreadScheduledExecutor(
            runnable -> new Thread(runnable, "spinon-delayed-host"));

    private R08WgpuSurface hostGpuSurface;
    private R08GpuSurface hostGlesSurface;
    private C0410RuntimeGpuDemo runtimeGpuDemo;
    private boolean gpuWasPaused;
    private volatile long runtimeSession;
    private volatile boolean activityClosing;
    private TextView runtimeLog;
    private ScrollView runtimeScroll;
    private final StringBuilder runtimeLogBuffer = new StringBuilder();
    private final Object runtimeLogPendingLock = new Object();
    private final StringBuilder pendingRuntimeLogEntries = new StringBuilder();
    private boolean runtimeLogFlushScheduled;
    private boolean runtimeLogScrollPending;
    private TextView runtimeStatus;
    private Button dispatchButton;
    private Button loopButton;
    private Button cancelButton;
    private Button delayedButton;
    private Button lifecycleButton;
    private boolean lifecycleGcFixtureRequested;
    private boolean runLifecycleGcAutomatically;
    private volatile boolean lifecycleGcPassed;
    private boolean longEvaluationRunning;
    private boolean cancelRequestPending;
    private boolean lastLongEvaluationCancelled;
    private boolean longEvaluationHadDispatch;
    private boolean pendingDispatchFailed;
    private boolean longEvaluationEvalFinished;
    private boolean longEvaluationSummaryWritten;
    private boolean longEvaluationDispatchThreadsMatch;
    private boolean longEvaluationTimeoutTriggered;
    private int longEvaluationCancelStatus = -1;
    private int longEvaluationManualCancelCount;
    private int longEvaluationAutomaticCancelCount;
    private int pendingDispatchesFromLongEvaluation;
    private int runtimeHeartbeatCount;
    private int longEvaluationHeartbeatStart;
    private boolean spinonUiOnly;
    private boolean frameAttributionMode;
    private boolean asynchronousMainHandoff;
    private final ConcurrentLinkedQueue<byte[]> frameAttributionReports = new ConcurrentLinkedQueue<>();
    private final AtomicInteger runtimeMainThreadHandoffSequence = new AtomicInteger(1);
    private int runtimeDispatchAcceptedCount;
    private long longEvaluationOwnerThread = -1;
    private long longEvaluationDispatchOwnerThread = -1;
    private long longEvaluationCallbackThread = -1;
    private int tapCount;
    private final Handler mainHandler = new Handler(Looper.getMainLooper());
    private final Handler asynchronousMainHandler =
            Handler.createAsync(Looper.getMainLooper());
    private final Runnable longEvaluationTimeout = () -> {
        if (activityClosing || !longEvaluationRunning || cancelRequestPending) return;
        longEvaluationTimeoutTriggered = true;
        longEvaluationAutomaticCancelCount++;
        Trace.setCounter("SpinonR05AutomaticCancelCount", longEvaluationAutomaticCancelCount);
        appendRuntimeLog("시간 초과 · 안전을 위해 V8 취소를 요청합니다");
        if (cancelButton != null) cancelButton.performClick();
    };
    private final Runnable runtimeHeartbeat = new Runnable() {
        @Override
        public void run() {
            if (activityClosing || !longEvaluationRunning) return;
            runtimeHeartbeatCount++;
            Trace.setCounter("SpinonR05MainThreadHeartbeatCount", runtimeHeartbeatCount);
            mainHandler.postDelayed(this, 50);
        }
    };

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        String frameBenchmarkMode = getIntent().getStringExtra(FrameAttributionActivity.EXTRA_MODE);
        if (frameBenchmarkMode != null) {
            startActivity(new Intent(this, FrameAttributionActivity.class)
                    .putExtra(FrameAttributionActivity.EXTRA_MODE, frameBenchmarkMode));
            finish();
            return;
        }
        spinonUiOnly = getIntent().getBooleanExtra("spinon_ui_only", false);
        frameAttributionMode = getIntent().getBooleanExtra("spinon_frame_attribution", false);
        asynchronousMainHandoff = getIntent().getBooleanExtra("spinon_async_main_handoff", false);
        if (frameAttributionMode) {
            Trace.setCounter("SpinonR05RuntimeDispatchCount", 0);
            long reportDelayMillis = (long) getIntent()
                    .getIntExtra("spinon_frame_duration_seconds", 20) * 1000L + 10_000L;
            mainHandler.postDelayed(this::dumpFrameAttributionReports, reportDelayMillis);
        }
        if (spinonUiOnly) {
            Log.i(TAG, "SPINON_R05_UI_ONLY · 같은 화면에서 런타임 dispatch 생략");
        }
        if (frameAttributionMode) {
            Log.i(TAG, "SPINON_R05_FRAME_ATTRIBUTION · 이벤트 상세 로그 UI 생략");
        }
        if (getIntent().getBooleanExtra("spinon_priority_probe", false)) {
            showPriorityProbe();
            return;
        }
        if (getIntent().getBooleanExtra("spinon_shutdown_probe", false)) {
            showShutdownProbe();
            return;
        }
        if (getIntent().getBooleanExtra("spinon_c0410_runtime_gpu", false)
                || getIntent().getBooleanExtra("spinon_c0411_author_stylesheets", false)
                || getIntent().getBooleanExtra("spinon_c052_registered_properties", false)
                || getIntent().getBooleanExtra("spinon_c053_runtime_result_cache", false)
                || getIntent().getBooleanExtra("spinon_c054_incremental_restyle", false)
                || getIntent().getBooleanExtra("spinon_c061_percentage_dimensions", false)
                || getIntent().getBooleanExtra("spinon_c062_spacing_percentages", false)
                || getIntent().getBooleanExtra("spinon_c063_absolute_lengths", false)
                || getIntent().getBooleanExtra("spinon_c064_font_relative_units", false)
                || getIntent().getBooleanExtra("spinon_c065_typed_css_math", false)
                || getIntent().getBooleanExtra("spinon_c066_viewport_units", false)
                || getIntent().getBooleanExtra("spinon_c071_min_max_sizing", false)
                || getIntent().getBooleanExtra("spinon_c072_border_width", false)
                || getIntent().getBooleanExtra("spinon_c073_aspect_ratio", false)
                || getIntent().getBooleanExtra("spinon_c08_block_paint", false)
                || getIntent().getBooleanExtra("spinon_c091_block_formatting", false)
                || getIntent().getBooleanExtra("spinon_c092_margin_collapse", false)
                || getIntent().getBooleanExtra("spinon_c093_flow_root", false)
                || getIntent().getBooleanExtra("spinon_c101_flex_wrap", false)
                || getIntent().getBooleanExtra("spinon_c102_flex_distribution", false)
                || getIntent().getBooleanExtra("spinon_c1031_flex_reverse", false)
                || getIntent().getBooleanExtra("spinon_c1032_flex_order", false)
                || getIntent().getBooleanExtra("spinon_c1033_flex_alignment", false)
                || getIntent().getBooleanExtra("spinon_c1034_flex_baseline", false)
                || getIntent().getBooleanExtra("spinon_c1035_positioned_flex", false)
                || getIntent().getBooleanExtra("spinon_c122_absolute_block", false)
                || getIntent().getBooleanExtra("spinon_c123_fixed_position", false)
                || getIntent().getBooleanExtra("spinon_c121_static_relative", false)) {
            if (!BuildConfig.SPINON_C04_RUNTIME_GPU) {
                TextView unavailable = new TextView(this);
                unavailable.setText("C04.10 GPU 실험을 켜서 빌드해야 합니다.");
                unavailable.setTextColor(Color.WHITE);
                unavailable.setGravity(Gravity.CENTER);
                setContentView(unavailable);
                Log.e(TAG, "SPINON_C0410_DISABLED · SPINON_ENABLE_C04_RUNTIME_GPU=1로 빌드하세요");
                return;
            }
            boolean failureProbe = getIntent().getBooleanExtra(
                    "spinon_c0410_failure_probe", false);
            boolean shutdownProbe = getIntent().getBooleanExtra(
                    "spinon_c0410_shutdown_probe", false);
            if (failureProbe && shutdownProbe) {
                TextView unavailable = new TextView(this);
                unavailable.setText("draw 복구와 종료 검증은 나눠 실행해야 합니다.");
                unavailable.setTextColor(Color.WHITE);
                unavailable.setGravity(Gravity.CENTER);
                setContentView(unavailable);
                Log.e(TAG, "SPINON_C0410_PROBE_ARGUMENT_ERROR · "
                        + "failure와 shutdown 검증을 동시에 요청했습니다");
                return;
            }
            if ((failureProbe || shutdownProbe)
                    && !BuildConfig.SPINON_C04_RUNTIME_GPU_FAILURE_FIXTURE) {
                TextView unavailable = new TextView(this);
                unavailable.setText("draw 실패 검증용 빌드 옵션을 켜야 합니다.");
                unavailable.setTextColor(Color.WHITE);
                unavailable.setGravity(Gravity.CENTER);
                setContentView(unavailable);
                Log.e(TAG, "SPINON_C0410_FAILURE_PROBE_DISABLED · "
                        + "SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE=1로 빌드하세요");
                return;
            }
            int backend = getIntent().getIntExtra("spinon_r08_backend", 0);
            runtimeGpuDemo = new C0410RuntimeGpuDemo(
                    this, backend, failureProbe, shutdownProbe);
            setContentView(runtimeGpuDemo);
            return;
        }
        if (getIntent().getBooleanExtra("spinon_r05_callback_faults", false)) {
            R05PresentFenceFailureFixture.start(this);
            return;
        }
        if (getIntent().getBooleanExtra("spinon_r05_queue_saturation", false)) {
            TextView status = new TextView(this);
            status.setText("R05 표시 신호 callback 대기열 포화 검증 중…");
            status.setTextColor(Color.WHITE);
            status.setTextSize(18);
            status.setGravity(Gravity.CENTER);
            status.setBackgroundColor(Color.rgb(14, 19, 31));
            setContentView(status);
            R05PresentTimingProbe.runQueueSaturationProbe("wgpu");
            return;
        }
        boolean runR13 = getIntent().getBooleanExtra("spinon_r13", false);
        boolean runS04 = getIntent().getBooleanExtra("spinon_s04", false);
        boolean runR05PresentationProbe =
                getIntent().getBooleanExtra("spinon_r05_presentation", false);
        boolean runR05AsyncFenceWait =
                getIntent().getBooleanExtra("spinon_r05_async_fence_wait", false);
        if (runR05AsyncFenceWait && !R05PresentFenceWaitExperiment.enable()) {
            finish();
            return;
        }
        boolean runR05PresentFenceProbe =
                getIntent().getBooleanExtra("spinon_r05_present_fence", false)
                        || runR05AsyncFenceWait;
        boolean runR05FrameTimelineJoin =
                getIntent().getBooleanExtra("spinon_r05_frame_timeline_join", false)
                        || (runR05PresentFenceProbe
                        && R05PresentTimingProbe.isFrameTimelineJoinAvailable());
        boolean runR05GlesControl =
                getIntent().getBooleanExtra("spinon_r05_gles_control", false);
        if ((runR05GlesControl || runR05PresentFenceProbe) && (runS04 || runR13)) {
            Log.e(TAG, "SPINON_R05_MODE_ERROR renderer=opengl_es reason=conflicting_mode_flags");
            finish();
            return;
        }
        runR05PresentationProbe |= runR05GlesControl || runR05FrameTimelineJoin
                || runR05PresentFenceProbe;
        if (runS04 || runR13 || runR05PresentationProbe
                || getIntent().getBooleanExtra("spinon_r08", false)) {
            int backend = getIntent().getIntExtra("spinon_r08_backend", 1);
            boolean useWgpu = !runR05GlesControl && (runS04 || runR13
                    || runR05PresentationProbe
                    || !getIntent().getBooleanExtra("spinon_r08_native", false));
            int failureInjection = getIntent().getIntExtra("spinon_r13_failure", 0);
            int recoveryFailureInjection = getIntent().getIntExtra("spinon_r13_recovery_failure", 0);
            View surface = R08GpuDemo.show(
                    this, useWgpu, backend, runR13, runS04, failureInjection,
                    recoveryFailureInjection, runR05PresentationProbe,
                    runR05FrameTimelineJoin, runR05PresentFenceProbe);
            hostGpuSurface = surface instanceof R08WgpuSurface
                    && (runS04 || runR13 || runR05PresentationProbe)
                    ? (R08WgpuSurface) surface : null;
            hostGlesSurface = surface instanceof R08GpuSurface
                    ? (R08GpuSurface) surface : null;
            return;
        }
        if (getIntent().getBooleanExtra("spinon_c048_ua_cascade", false)) {
            showUaCascadeProbe();
            return;
        }

        try {
            String source = readAsset("app.js");
            DisplayMetrics metrics = getResources().getDisplayMetrics();
            lifecycleGcFixtureRequested = getIntent().getBooleanExtra("spinon_dom_gc", false);
            runLifecycleGcAutomatically = lifecycleGcFixtureRequested
                    && BuildConfig.SPINON_S03_DOM_GC_FIXTURE;
            if (lifecycleGcFixtureRequested && !BuildConfig.SPINON_S03_DOM_GC_FIXTURE) {
                Log.e(TAG, "SPINON_DOM_GC_FIXTURE_DISABLED · 검증 전용 빌드로 다시 빌드하세요");
            }
            if (getIntent().getBooleanExtra("spinon_runtime_threads", false)
                    || lifecycleGcFixtureRequested) {
                showRuntimeThreadExperiment(source);
                return;
            }
            boolean runR10 = getIntent().getBooleanExtra("spinon_r10", false);
            float width = metrics.widthPixels / metrics.density;
            float height = metrics.heightPixels / metrics.density;
            float density = metrics.density;
            bootstrapExecutor.execute(() -> {
                boolean isMainThread = Thread.currentThread() == Looper.getMainLooper().getThread();
                Log.i(TAG, "SPINON_BOOTSTRAP_EXECUTION is_main_thread=" + isMainThread);
                byte[] outputUtf8 = nativeRun(source.getBytes(StandardCharsets.UTF_8),
                        width, height, density, runR10);
                String output = outputUtf8 == null
                        ? "native bridge returned no result"
                        : new String(outputUtf8, StandardCharsets.UTF_8);
                Log.i(TAG, "SPINON_BOOTSTRAP_RESULT=" + output);
                if (runR10 && !activityClosing) {
                    runOnUiThread(() -> {
                        if (!activityClosing) showR10Report(r10Report(output), density);
                    });
                }
            });
        } catch (IOException error) {
            Log.e(TAG, "SPINON_BOOTSTRAP_ASSET_ERROR", error);
        } catch (RuntimeException error) {
            Log.e(TAG, "SPINON_BOOTSTRAP_RUNTIME_ERROR", error);
        }
    }

    private void showPriorityProbe() {
        float density = getResources().getDisplayMetrics().density;
        int inset = Math.round(24 * density);
        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setPadding(inset, inset, inset, inset);
        root.setBackgroundColor(Color.rgb(14, 19, 31));
        root.setOnApplyWindowInsetsListener((view, windowInsets) -> {
            view.setPadding(
                    inset + windowInsets.getSystemWindowInsetLeft(),
                    inset + windowInsets.getSystemWindowInsetTop(),
                    inset + windowInsets.getSystemWindowInsetRight(),
                    inset + windowInsets.getSystemWindowInsetBottom());
            return windowInsets;
        });

        TextView title = new TextView(this);
        title.setText("SPINON · Android R06 우선순위 검증");
        title.setTextColor(Color.rgb(230, 237, 248));
        title.setTextSize(20);
        title.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        root.addView(title);

        TextView description = new TextView(this);
        description.setText("개발 전용 · 실제 V8 우선순위, FIFO, 큐 포화·거부·복구를 확인합니다");
        description.setTextColor(Color.rgb(170, 184, 207));
        description.setTextSize(13);
        description.setPadding(0, Math.round(8 * density), 0, Math.round(12 * density));
        root.addView(description);

        TextView status = new TextView(this);
        status.setText("실제 V8 우선순위·큐 포화 검증 중…");
        status.setTextColor(Color.rgb(97, 185, 255));
        status.setTextSize(15);
        root.addView(status);

        TextView report = new TextView(this);
        report.setTextColor(Color.rgb(230, 237, 248));
        report.setTypeface(Typeface.MONOSPACE);
        report.setTextSize(12);
        report.setPadding(0, Math.round(12 * density), 0, Math.round(16 * density));
        ScrollView scroll = new ScrollView(this);
        scroll.addView(report);
        root.addView(scroll, new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT, 0, 1));
        setContentView(root);

        bootstrapExecutor.execute(() -> {
            String priority = decode(nativeSessionPriorityProbe());
            String fairness = decode(nativeSessionPriorityFairnessProbe());
            String result = priority + " " + fairness;
            Log.i(TAG, "SPINON_PRIORITY_PROBE " + result);
            runOnUiThread(() -> {
                boolean passed = result.contains("status=0 priority_probe=PASS")
                        && result.contains("status=0 priority_stream_probe=PASS")
                        && result.contains("queue_saturation_probe=PASS");
                status.setText(passed ? "실제 V8 우선순위·큐 포화 복구 검증 통과" : "실제 V8 우선순위·큐 포화 복구 검증 실패");
                report.setText(result
                        .replace(" priority_probe=", "\npriority_probe=")
                        .replace(" blocker_status=", "\n차단 작업 status=")
                        .replace(" cancel_status=", "\n취소 status=")
                        .replace(" order=[", "\n실행 순서\n  ")
                        .replace(",", "\n  ")
                        .replace(" owner_tid=", "\n소유 스레드="));
            });
        });
    }

    private void showUaCascadeProbe() {
        float density = getResources().getDisplayMetrics().density;
        int inset = Math.round(24 * density);
        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setPadding(inset, inset, inset, inset);
        root.setBackgroundColor(Color.rgb(14, 19, 31));

        TextView title = new TextView(this);
        title.setText("SPINON · C04 Runtime CSS→Taffy");
        title.setTextColor(Color.rgb(230, 237, 248));
        title.setTextSize(20);
        title.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        root.addView(title);

        TextView description = new TextView(this);
        description.setText("개발 전용 · 실제 V8 DOM 변경부터 Stylo cascade와 Taffy 프레임 JSON까지 확인합니다");
        description.setTextColor(Color.rgb(170, 184, 207));
        description.setTextSize(13);
        description.setPadding(0, Math.round(8 * density), 0, Math.round(12 * density));
        root.addView(description);

        TextView status = new TextView(this);
        status.setText("실제 V8 CSS→Taffy 검증 중…");
        status.setTextColor(Color.rgb(97, 185, 255));
        status.setTextSize(15);
        root.addView(status);

        TextView report = new TextView(this);
        report.setTextColor(Color.rgb(230, 237, 248));
        report.setTypeface(Typeface.MONOSPACE);
        report.setTextSize(12);
        ScrollView scroll = new ScrollView(this);
        scroll.addView(report);
        root.addView(scroll, new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT, 0, 1));
        setContentView(root);

        bootstrapExecutor.execute(() -> {
            String result = decode(nativeSessionUaCascadeProbe());
            Log.i(TAG, "SPINON_C04_RUNTIME_LAYOUT_PROBE=" + result);
            runOnUiThread(() -> {
                boolean passed = result.startsWith("status=0 ua_cascade_probe=PASS")
                        && result.contains("runtime_layout=PASS");
                status.setText(passed ? "실제 V8 CSS→Taffy 검증 통과" : "실제 V8 CSS→Taffy 검증 실패");
                report.setText(result.replace(" roots=", "\nroots=")
                        .replace(" document_revision=", "\ndocument_revision=")
                        .replace(" ua_values=", "\nua_values=")
                        .replace(" result=", "\nresult=\n"));
            });
        });
    }

    private void showShutdownProbe() {
        float density = getResources().getDisplayMetrics().density;
        int inset = Math.round(24 * density);
        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setPadding(inset, inset, inset, inset);
        root.setBackgroundColor(Color.rgb(14, 19, 31));

        TextView title = new TextView(this);
        title.setText("SPINON · Android V8 세션 종료 검증");
        title.setTextColor(Color.rgb(230, 237, 248));
        title.setTextSize(20);
        title.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        root.addView(title);

        TextView description = new TextView(this);
        description.setText("개발 전용 · 활성 평가 취소, 대기 명령 거부, 종료 후 호출 거부를 확인합니다");
        description.setTextColor(Color.rgb(170, 184, 207));
        description.setTextSize(13);
        description.setPadding(0, Math.round(8 * density), 0, Math.round(12 * density));
        root.addView(description);

        TextView status = new TextView(this);
        status.setText("실제 V8 세션 종료 검증 중…");
        status.setTextColor(Color.rgb(97, 185, 255));
        status.setTextSize(15);
        root.addView(status);

        TextView report = new TextView(this);
        report.setTextColor(Color.rgb(230, 237, 248));
        report.setTypeface(Typeface.MONOSPACE);
        report.setTextSize(12);
        report.setPadding(0, Math.round(12 * density), 0, Math.round(16 * density));
        ScrollView scroll = new ScrollView(this);
        scroll.addView(report);
        root.addView(scroll, new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT, 0, 1));
        setContentView(root);

        bootstrapExecutor.execute(() -> {
            String result = decode(nativeSessionShutdownProbe());
            Log.i(TAG, "SPINON_SHUTDOWN_PROBE " + result);
            runOnUiThread(() -> {
                boolean passed = result.contains("status=0 shutdown_probe=PASS");
                status.setText(passed ? "V8 세션 종료 검증 통과" : "V8 세션 종료 검증 실패");
                report.setText(result
                        .replace(" shutdown_probe=", "\nshutdown_probe=")
                        .replace(" active_status=", "\n활성 평가 status=")
                        .replace(" queued_statuses=", "\n대기 명령 status=")
                        .replace(" post_eval_status=", "\n종료 뒤 eval status=")
                        .replace(" post_dispatch_status=", "\n종료 뒤 dispatch status=")
                        .replace(" close_ms=", "\n종료 소요 ms=")
                        .replace(" owner_tid=", "\nV8 소유 스레드=")
                        .replace(" shutdown_tid=", "\n종료 요청 스레드="));
            });
        });
    }

    private void showRuntimeThreadExperiment(String source) {
        float density = getResources().getDisplayMetrics().density;
        int inset = Math.round(18 * density);
        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setPadding(inset, Math.round(20 * density), inset, inset);
        root.setOnApplyWindowInsetsListener((view, windowInsets) -> {
            view.setPadding(
                    inset + windowInsets.getSystemWindowInsetLeft(),
                    Math.round(20 * density) + windowInsets.getSystemWindowInsetTop(),
                    inset + windowInsets.getSystemWindowInsetRight(),
                    inset + windowInsets.getSystemWindowInsetBottom());
            return windowInsets;
        });
        root.setBackgroundColor(Color.rgb(14, 19, 31));

        TextView title = new TextView(this);
        title.setText(runLifecycleGcAutomatically
                ? "SPINON · DOM wrapper 수명 검증"
                : "SPINON · V8 실행 스레드 실험");
        title.setTextColor(Color.rgb(230, 237, 248));
        title.setTextSize(20);
        title.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        root.addView(title);

        TextView description = new TextView(this);
        description.setText(runLifecycleGcAutomatically
                ? "검증용 빌드 · 약한 wrapper GC와 Rust HostDocument 회수 확인"
                : "개발 전용 · 긴 JavaScript 실행 중에도 화면 입력과 JS 이벤트 대기·취소를 확인합니다");
        description.setTextColor(Color.rgb(170, 184, 207));
        description.setTextSize(13);
        description.setPadding(0, Math.round(8 * density), 0, Math.round(12 * density));
        root.addView(description);

        runtimeStatus = new TextView(this);
        runtimeStatus.setText("V8 세션 초기화 중…");
        runtimeStatus.setTextColor(Color.rgb(97, 185, 255));
        runtimeStatus.setTextSize(14);
        root.addView(runtimeStatus);

        dispatchButton = runtimeButton("터치 이벤트 보내기");
        loopButton = runtimeButton("긴 JavaScript 실행 시작");
        cancelButton = runtimeButton("실행 취소");
        delayedButton = runtimeButton("지연 호스트 응답 모의 (0.5초)");
        lifecycleButton = runtimeButton("V8 약한 wrapper 회수 검증");
        lifecycleButton.setVisibility(BuildConfig.SPINON_S03_DOM_GC_FIXTURE
                ? android.view.View.VISIBLE : android.view.View.GONE);
        dispatchButton.setEnabled(false);
        loopButton.setEnabled(false);
        cancelButton.setEnabled(false);
        delayedButton.setEnabled(false);
        lifecycleButton.setEnabled(false);
        addRuntimeButton(root, dispatchButton, density);
        addRuntimeButton(root, loopButton, density);
        addRuntimeButton(root, cancelButton, density);
        addRuntimeButton(root, delayedButton, density);
        root.addView(lifecycleButton);

        ScrollView scroll = new ScrollView(this);
        runtimeScroll = scroll;
        scroll.setPadding(0, Math.round(16 * density), 0, 0);
        runtimeLogBuffer.setLength(0);
        synchronized (runtimeLogPendingLock) {
            pendingRuntimeLogEntries.setLength(0);
            runtimeLogFlushScheduled = false;
        }
        runtimeLogScrollPending = false;
        runtimeLog = new TextView(this);
        runtimeLog.setTextColor(Color.rgb(218, 226, 240));
        runtimeLog.setTextSize(12);
        runtimeLog.setTypeface(Typeface.MONOSPACE);
        runtimeLog.setBackgroundColor(Color.rgb(8, 11, 17));
        runtimeLog.setPadding(Math.round(6 * density), Math.round(12 * density),
                Math.round(6 * density), Math.round(20 * density));
        scroll.addView(runtimeLog);
        root.addView(scroll, new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT, 0, 1));
        setContentView(root);

        dispatchButton.setOnClickListener(view -> {
            int nodeId = ++tapCount;
            Trace.setCounter("SpinonR05SpinonInputCount", nodeId);
            Trace.beginSection("SpinonR05:spinon-event-submit");
            try {
                if (!frameAttributionMode) {
                    Trace.beginSection("SpinonR05:spinon-event-status-update");
                    try {
                        runtimeStatus.setText("UI 탭 " + nodeId + "회 · 이벤트 입력 처리");
                    } finally {
                        Trace.endSection();
                    }
                }
                if (!frameAttributionMode) {
                    appendRuntimeLog("UI 타깃 액션 " + nodeId + " 입력");
                }
                if (!spinonUiOnly) {
                    Trace.beginAsyncSection("SpinonR05:runtime-dispatch", nodeId);
                }
            } finally {
                Trace.endSection();
            }
            if (spinonUiOnly) {
                Trace.setCounter("SpinonR05RuntimeDispatchCount", 0);
                return;
            }
            boolean submittedDuringLongEvaluation = longEvaluationRunning;
            if (submittedDuringLongEvaluation) {
                longEvaluationHadDispatch = true;
                pendingDispatchesFromLongEvaluation++;
            }
            boolean accepted = submitRuntimeCallWithCompletion(
                    () -> nativeSessionDispatch(runtimeSession, nodeId),
                    "이벤트 " + nodeId + " ·", (result, error) -> {
                        Trace.endAsyncSection("SpinonR05:runtime-dispatch", nodeId);
                        if (!submittedDuringLongEvaluation) return;
                        String report = error == null ? decode(result) : "";
                        boolean dispatchSucceeded = error == null
                                && report.startsWith("status=0 ");
                        pendingDispatchesFromLongEvaluation--;
                        if (!dispatchSucceeded) pendingDispatchFailed = true;
                        long ownerThread = reportLongField(report, "owner_tid");
                        long callbackThread = reportLongField(report, "callback_tid");
                        if (longEvaluationDispatchOwnerThread < 0) {
                            longEvaluationDispatchOwnerThread = ownerThread;
                            longEvaluationCallbackThread = callbackThread;
                        } else if (longEvaluationDispatchOwnerThread != ownerThread
                                || longEvaluationCallbackThread != callbackThread) {
                            longEvaluationDispatchThreadsMatch = false;
                        }
                        if (ownerThread < 0 || callbackThread < 0
                                || ownerThread != callbackThread) {
                            longEvaluationDispatchThreadsMatch = false;
                        }
                        updateLongEvaluationButtons();
                        finishLongEvaluationIfReady();
                    });
            if (accepted) {
                Trace.setCounter(
                        "SpinonR05RuntimeDispatchCount", ++runtimeDispatchAcceptedCount);
            }
            if (!accepted) {
                Trace.endAsyncSection("SpinonR05:runtime-dispatch", nodeId);
                if (submittedDuringLongEvaluation) {
                    pendingDispatchesFromLongEvaluation--;
                    pendingDispatchFailed = true;
                    updateLongEvaluationButtons();
                    finishLongEvaluationIfReady();
                }
            }
        });
        loopButton.setOnClickListener(view -> {
            if (longEvaluationRunning || pendingDispatchesFromLongEvaluation != 0
                    || runtimeSession == 0 || activityClosing) return;
            lastLongEvaluationCancelled = false;
            longEvaluationHadDispatch = false;
            pendingDispatchFailed = false;
            longEvaluationEvalFinished = false;
            longEvaluationSummaryWritten = false;
            longEvaluationDispatchThreadsMatch = true;
            longEvaluationTimeoutTriggered = false;
            longEvaluationCancelStatus = -1;
            longEvaluationManualCancelCount = 0;
            longEvaluationAutomaticCancelCount = 0;
            Trace.setCounter("SpinonR05ManualCancelCount", 0);
            Trace.setCounter("SpinonR05AutomaticCancelCount", 0);
            pendingDispatchesFromLongEvaluation = 0;
            longEvaluationOwnerThread = -1;
            longEvaluationDispatchOwnerThread = -1;
            longEvaluationCallbackThread = -1;
            longEvaluationHeartbeatStart = runtimeHeartbeatCount;
            mainHandler.removeCallbacks(runtimeHeartbeat);
            mainHandler.postDelayed(runtimeHeartbeat, 50);
            Trace.beginAsyncSection("SpinonR05:long-eval", 1);
            Trace.beginSection("SpinonR05:long-eval-submit");
            try {
                Trace.beginSection("SpinonR05:long-eval-button-state");
                try {
                    setLongEvaluationRunning(true);
                } finally {
                    Trace.endSection();
                }
                Trace.beginSection("SpinonR05:long-eval-timeout-schedule");
                try {
                    mainHandler.postDelayed(longEvaluationTimeout, 12_000);
                } finally {
                    Trace.endSection();
                }
                Trace.beginSection("SpinonR05:long-eval-log-enqueue");
                try {
                    appendRuntimeLog("무한 JavaScript 평가 제출 · UI 메인 스레드는 대기하지 않음");
                } finally {
                    Trace.endSection();
                }
                Trace.beginSection("SpinonR05:long-eval-status-update");
                try {
                    runtimeStatus.setText("긴 JavaScript 실행 중 · 화면 입력 가능 · JS 이벤트 대기 중");
                } finally {
                    Trace.endSection();
                }
            } finally {
                Trace.endSection();
            }
            boolean accepted = submitRuntimeCallWithCompletion(() -> {
                Trace.beginSection("SpinonR05:long-eval-worker-call");
                try {
                    return nativeSessionEval(runtimeSession,
                            "while (true) { /* 취소 경로 검증 */ }"
                                    .getBytes(StandardCharsets.UTF_8));
                } finally {
                    Trace.endSection();
                }
            },
                    "긴 평가 반환 ·", (result, error) -> {
                        Trace.endAsyncSection("SpinonR05:long-eval", 1);
                        String report = decode(result);
                        lastLongEvaluationCancelled = error == null
                                && report.startsWith("status=-8 ");
                        longEvaluationEvalFinished = true;
                        longEvaluationOwnerThread = reportLongField(report, "owner_tid");
                        setLongEvaluationRunning(false);
                        if (!activityClosing) {
                            if (error != null) {
                                runtimeStatus.setText("긴 JavaScript 실행 오류 · " + error.getMessage());
                            } else if (pendingDispatchesFromLongEvaluation != 0) {
                                runtimeStatus.setText(lastLongEvaluationCancelled
                                        ? "취소 완료 · 대기 중인 JS 이벤트 처리 중"
                                        : "긴 JavaScript 종료 · 대기 중인 JS 이벤트 처리 중");
                            } else if (longEvaluationHadDispatch) {
                                String eventResult = pendingDispatchFailed
                                        ? "대기 중이던 JS 이벤트 처리 실패"
                                        : "대기 중이던 JS 이벤트 처리 완료";
                                runtimeStatus.setText(lastLongEvaluationCancelled
                                        ? "취소 완료 · " + eventResult
                                        : "준비됨 · " + eventResult);
                            } else if (lastLongEvaluationCancelled) {
                                runtimeStatus.setText("취소 완료 · JavaScript가 종료되었습니다");
                            } else if (report.startsWith("status=0 ")) {
                                runtimeStatus.setText("준비됨 · 긴 JavaScript 실행 완료");
                            } else {
                                runtimeStatus.setText("긴 JavaScript 실행 실패 · " + report);
                            }
                        }
                        finishLongEvaluationIfReady();
                    });
            if (!accepted) {
                Trace.endAsyncSection("SpinonR05:long-eval", 1);
                setLongEvaluationRunning(false);
                mainHandler.removeCallbacks(runtimeHeartbeat);
                runtimeStatus.setText("실행 대기열이 가득 차 긴 JavaScript를 시작하지 못했습니다");
            }
        });
        cancelButton.setOnClickListener(view -> {
            if (!longEvaluationRunning || cancelRequestPending || activityClosing) return;
            long handle = runtimeSession;
            if (handle == 0) return;
            if (longEvaluationTimeoutTriggered) {
                Trace.setCounter("SpinonR05AutomaticCancelCount",
                        longEvaluationAutomaticCancelCount);
            } else {
                longEvaluationManualCancelCount++;
                Trace.setCounter("SpinonR05ManualCancelCount",
                        longEvaluationManualCancelCount);
            }
            Trace.beginSection("SpinonR05:long-eval-cancel-submit");
            try {
                cancelRequestPending = true;
                updateLongEvaluationButtons();
                runtimeStatus.setText("JavaScript 취소 요청 중…");
            } finally {
                Trace.endSection();
            }
            try {
                runtimeControl.execute(() -> {
                    Trace.beginSection("SpinonR05:long-eval-cancel-call");
                    int result;
                    try {
                        result = nativeSessionCancel(handle);
                    } finally {
                        Trace.endSection();
                    }
                    appendRuntimeLog("취소 요청 · status=" + result);
                    runOnUiThread(() -> {
                        if (activityClosing) return;
                        longEvaluationCancelStatus = result;
                        if (!longEvaluationRunning || !cancelRequestPending) {
                            finishLongEvaluationIfReady();
                            return;
                        }
                        if (result == 0) {
                            runtimeStatus.setText("취소 요청 접수 · JavaScript 종료 대기 중");
                        } else if (result == 1) {
                            cancelRequestPending = false;
                            updateLongEvaluationButtons();
                            runtimeStatus.setText("실행 중인 JavaScript가 없습니다 · 시작 중이면 다시 취소하세요");
                        } else {
                            cancelRequestPending = false;
                            updateLongEvaluationButtons();
                            runtimeStatus.setText("취소 요청 실패 · status=" + result);
                        }
                        finishLongEvaluationIfReady();
                    });
                });
            } catch (RejectedExecutionException error) {
                longEvaluationCancelStatus = -5;
                cancelRequestPending = false;
                updateLongEvaluationButtons();
                runtimeStatus.setText("취소 요청을 제출하지 못했습니다");
                appendRuntimeLog("취소 요청이 거부되었습니다: " + error.getMessage());
            }
        });
        delayedButton.setOnClickListener(view -> {
            delayedButton.setEnabled(false);
            appendRuntimeLog("호스트가 0.5초 뒤 JS 콜백을 큐에 넣도록 예약했습니다.");
            delayedHost.schedule(() -> {
                boolean accepted = submitRuntimeCall(() -> nativeSessionEval(runtimeSession,
                        "spinon.setText('delayed-host-response')".getBytes(StandardCharsets.UTF_8)),
                        "지연 호스트 응답 ·", () -> delayedButton.setEnabled(!activityClosing));
                if (!accepted) {
                    runOnUiThread(() -> {
                        if (activityClosing) return;
                        delayedButton.setEnabled(true);
                        runtimeStatus.setText("실행 대기열이 가득 차 호스트 응답을 제출하지 못했습니다");
                    });
                }
            }, 500, TimeUnit.MILLISECONDS);
        });
        lifecycleButton.setOnClickListener(view -> runLifecycleCollectionProbe());

        submitRuntimeCall(() -> {
            long handle = nativeSessionCreate();
            runtimeSession = handle;
            if (handle == 0) throw new IllegalStateException("V8 세션 생성 실패");
            return nativeSessionEval(handle, source.getBytes(StandardCharsets.UTF_8));
        }, "초기 JavaScript ·", () -> {
            if (runtimeSession == 0) {
                runtimeStatus.setText("세션 생성 실패 · logcat 확인");
                return;
            }
            runtimeStatus.setText("준비됨 · V8 세션 대기 중");
            dispatchButton.setEnabled(true);
            updateLongEvaluationButtons();
            delayedButton.setEnabled(true);
            lifecycleButton.setEnabled(true);
            if (runLifecycleGcAutomatically) runLifecycleCollectionProbe();
            else if (lifecycleGcFixtureRequested) {
                runtimeStatus.setText("DOM GC 검증 fixture 비활성 · 검증 전용 빌드로 다시 빌드하세요");
            }
        });
    }

    private void runLifecycleCollectionProbe() {
        long handle = runtimeSession;
        if (handle == 0 || activityClosing) return;
        lifecycleButton.setEnabled(false);
        runtimeStatus.setText("V8 callback root와 반복 DOM 수명 회수 검증 중…");
        submitRuntimeCall(() -> {
            String fixtureSource;
            try {
                fixtureSource = readAsset(LIFECYCLE_FIXTURE_ASSET);
            } catch (IOException error) {
                lifecycleGcPassed = false;
                return ("status=-1 dom_gc=FAIL fixture_asset=missing")
                        .getBytes(StandardCharsets.UTF_8);
            }

            String loaded = decode(nativeSessionEval(handle,
                    fixtureSource.getBytes(StandardCharsets.UTF_8)));
            if (!loaded.startsWith("status=0 ")) {
                lifecycleGcPassed = false;
                return ("status=-1 dom_gc=FAIL fixture_load=FAIL report={" + loaded + "}")
                        .getBytes(StandardCharsets.UTF_8);
            }

            String reset = callLifecycleFixture(handle, "reset");
            String baselineFirst = decode(nativeSessionEval(handle, new byte[0]));
            String baseline = decode(nativeSessionEval(handle, new byte[0]));
            long baselineNodes = reportLongField(baseline, "document_nodes");
            long baselineStringUnits = reportLongField(baseline, "document_string_units");
            long baselineWrapperHandles = reportLongField(
                    baseline, "document_collection_scanned_handles");
            boolean baselineReady = reset.startsWith("status=0 ")
                    && reportHasHealthyCollection(reset)
                    && reportHasHealthyCollection(baselineFirst)
                    && reportHasHealthyCollection(baseline)
                    && baselineNodes >= 0 && baselineStringUnits >= 0
                    && reportLongField(baseline, "document_collection_empty_handles") == 0
                    && reportLongField(baseline, "document_collection_live_handles")
                            == baselineWrapperHandles;
            if (!baselineReady) {
                lifecycleGcPassed = false;
                return ("status=-1 dom_gc=FAIL baseline=FAIL nodes=" + baselineNodes
                        + " string_units=" + baselineStringUnits
                        + " wrapper_handles=" + baselineWrapperHandles
                        + " reset={" + reset + "} baseline={" + baseline + "}")
                        .getBytes(StandardCharsets.UTF_8);
            }

            String rootSetup = callLifecycleFixture(handle, "setupRootCases");
            String rootVerify = callLifecycleFixture(handle, "verifyRootCases");
            long rootSetupNodes = reportLongField(rootSetup, "document_nodes");
            boolean rootCasesPassed = rootSetup.startsWith("status=0 ")
                    && rootVerify.startsWith("status=0 ")
                    && rootSetupNodes == baselineNodes + 4
                    && reportHasHealthyCollection(rootSetup)
                    && reportHasHealthyCollection(rootVerify)
                    && reportScanCountsConsistent(rootVerify);
            String rootCleanup = callLifecycleFixture(handle, "cleanupRootCases");
            String rootCleanupStable = decode(nativeSessionEval(handle, new byte[0]));
            boolean initialBaselineReturned = rootCleanup.startsWith("status=0 ")
                    && reportHasHealthyCollection(rootCleanup)
                    && reportMatchesResourceBaseline(rootCleanupStable, baselineNodes,
                            baselineStringUnits, baselineWrapperHandles);

            String callbackSetup = callLifecycleFixture(handle, "setupCallbackClosureRoot");
            long callbackNodes = reportLongField(callbackSetup, "document_nodes");
            long callbackStringUnits = reportLongField(callbackSetup, "document_string_units");
            long expectedCallbackStringUnits = baselineStringUnits
                    + "http://www.w3.org/1999/xhtml".length()
                    + "aside".length() + "callback-root".length();
            boolean callbackClosureRootPassed = callbackSetup.startsWith("status=0 ")
                    && callbackNodes == baselineNodes + 2
                    && callbackStringUnits == expectedCallbackStringUnits
                    && reportHasHealthyCollection(callbackSetup);
            String callbackDispatch = decode(nativeSessionDispatch(handle, 1));
            String callbackRootVerify = callLifecycleFixture(handle, "verifyCallbackClosureRoot");
            callbackClosureRootPassed = callbackClosureRootPassed
                    && callbackDispatch.startsWith("status=0 ")
                    && callbackRootVerify.startsWith("status=0 ")
                    && reportHasHealthyCollection(callbackDispatch)
                    && reportHasHealthyCollection(callbackRootVerify);

            String callbackRelease = callLifecycleFixture(handle, "replaceCallbackClosureRoot");
            String callbackReleaseVerify = callLifecycleFixture(
                    handle, "verifyCallbackClosureReleased");
            String callbackReleaseStable = decode(nativeSessionEval(handle, new byte[0]));
            boolean callbackClosureReleased = callbackRelease.startsWith("status=0 ")
                    && callbackReleaseVerify.startsWith("status=0 ")
                    && reportMatchesResourceBaseline(callbackReleaseStable, baselineNodes,
                            baselineStringUnits, baselineWrapperHandles);

            String largeSetup = callLifecycleFixture(handle, "setupLargeRegistry");
            String largeVerify = callLifecycleFixture(handle, "verifyLargeRegistry");
            long expectedLargeNodes = baselineNodes + LIFECYCLE_LARGE_REGISTRY_NODES;
            long expectedLargeWrappers = baselineWrapperHandles
                    + LIFECYCLE_LARGE_REGISTRY_NODES;
            StringBuilder largeRegistryScanSamplesUs = new StringBuilder();
            boolean largeRegistryRepeatedScansPassed = true;
            for (int sample = 0; sample < LIFECYCLE_LARGE_REGISTRY_SCAN_SAMPLES; sample++) {
                String scanReport = decode(nativeSessionEval(handle, new byte[0]));
                long scanUs = reportLongField(scanReport, "document_collection_last_scan_us");
                boolean samplePassed = scanReport.startsWith("status=0 ")
                        && reportHasHealthyCollection(scanReport)
                        && reportLongField(scanReport, "document_nodes") == expectedLargeNodes
                        && reportLongField(scanReport, "document_collection_scanned_handles")
                                == expectedLargeWrappers
                        && reportLongField(scanReport, "document_collection_live_handles")
                                == expectedLargeWrappers
                        && reportLongField(scanReport, "document_collection_empty_handles") == 0
                        && scanUs >= 0;
                if (!samplePassed) {
                    largeRegistryRepeatedScansPassed = false;
                    break;
                }
                if (largeRegistryScanSamplesUs.length() > 0) {
                    largeRegistryScanSamplesUs.append(',');
                }
                largeRegistryScanSamplesUs.append(scanUs);
            }
            long largeSetupScanUs = reportLongField(
                    largeSetup, "document_collection_last_scan_us");
            long largeSetupRootBufferBytes = reportLongField(
                    largeSetup, "document_collection_wrapper_root_buffer_bytes");
            boolean largeRegistryRetained = largeSetup.startsWith("status=0 ")
                    && largeVerify.startsWith("status=0 ")
                    && reportHasHealthyCollection(largeSetup)
                    && reportHasHealthyCollection(largeVerify)
                    && reportLongField(largeSetup, "document_nodes") == expectedLargeNodes
                    && reportLongField(largeSetup, "document_collection_scanned_handles")
                            == expectedLargeWrappers
                    && reportLongField(largeSetup, "document_collection_live_handles")
                            == expectedLargeWrappers
                    && reportLongField(largeSetup, "document_collection_empty_handles") == 0
                    && largeSetupRootBufferBytes >= expectedLargeWrappers * Integer.BYTES
                    && largeSetupScanUs >= 0
                    && largeRegistryRepeatedScansPassed
                    && largeRegistryScanSamplesUs.toString().split(",", -1).length
                            == LIFECYCLE_LARGE_REGISTRY_SCAN_SAMPLES;
            String largeRelease = callLifecycleFixture(handle, "releaseLargeRegistry");
            long largeEmptyWrappers = reportLongField(
                    largeRelease, "document_collection_empty_handles");
            long largeReclaimedNodeBufferBytes = reportLongField(
                    largeRelease, "document_collection_reclaimed_node_buffer_bytes");
            String largeReleaseVerify = callLifecycleFixture(
                    handle, "verifyLargeRegistryReleased");
            String largeReleaseStable = decode(nativeSessionEval(handle, new byte[0]));
            boolean largeRegistryReleased = largeRelease.startsWith("status=0 ")
                    && largeReleaseVerify.startsWith("status=0 ")
                    && reportHasHealthyCollection(largeRelease)
                    && reportHasHealthyCollection(largeReleaseVerify)
                    && largeEmptyWrappers == LIFECYCLE_LARGE_REGISTRY_NODES
                    && reportLongField(largeRelease, "document_collection_scanned_handles")
                            == expectedLargeWrappers
                    && reportLongField(largeRelease, "document_collection_live_handles")
                            == baselineWrapperHandles
                    && largeReclaimedNodeBufferBytes
                            >= LIFECYCLE_LARGE_REGISTRY_NODES * Integer.BYTES
                    && reportMatchesResourceBaseline(largeReleaseStable, baselineNodes,
                            baselineStringUnits, baselineWrapperHandles);

            int baselineReturnRounds = 0;
            long maximumEmptyWrappers = 0;
            for (int round = 0; round < LIFECYCLE_STRESS_ROUNDS; round++) {
                String stress = callLifecycleFixture(handle, "stressRound");
                String stable = decode(nativeSessionEval(handle, new byte[0]));
                long emptyWrappers = reportLongField(
                        stress, "document_collection_empty_handles");
                maximumEmptyWrappers = Math.max(maximumEmptyWrappers, emptyWrappers);
                boolean roundReturned = stress.startsWith("status=0 ")
                        && reportHasHealthyCollection(stress)
                        && reportScanCountsConsistent(stress)
                        && reportLongField(stress, "document_nodes") == baselineNodes
                        && reportLongField(stress, "document_string_units") == baselineStringUnits
                        && emptyWrappers >= LIFECYCLE_STRESS_NODES_PER_ROUND
                        && reportMatchesResourceBaseline(stable, baselineNodes,
                                baselineStringUnits, baselineWrapperHandles);
                if (!roundReturned) break;
                baselineReturnRounds += 1;
            }

            String finalReport = (baselineReturnRounds > 0)
                    ? decode(nativeSessionEval(handle, new byte[0])) : callbackReleaseStable;
            long finalNodes = reportLongField(finalReport, "document_nodes");
            long finalStringUnits = reportLongField(finalReport, "document_string_units");
            boolean finalBaselineReturned = reportMatchesResourceBaseline(finalReport,
                    baselineNodes, baselineStringUnits, baselineWrapperHandles);
            boolean repeatedBaseline = baselineReturnRounds == LIFECYCLE_STRESS_ROUNDS
                    && finalBaselineReturned;
            boolean scanStatsPassed = reportScanCountsConsistent(reset)
                    && reportScanCountsConsistent(baselineFirst)
                    && reportScanCountsConsistent(baseline)
                    && reportScanCountsConsistent(rootSetup)
                    && reportScanCountsConsistent(rootVerify)
                    && reportScanCountsConsistent(rootCleanup)
                    && reportScanCountsConsistent(rootCleanupStable)
                    && reportScanCountsConsistent(callbackSetup)
                    && reportScanCountsConsistent(callbackDispatch)
                    && reportScanCountsConsistent(callbackRootVerify)
                    && reportScanCountsConsistent(callbackRelease)
                    && reportScanCountsConsistent(callbackReleaseVerify)
                    && reportScanCountsConsistent(callbackReleaseStable)
                    && reportScanCountsConsistent(largeSetup)
                    && reportScanCountsConsistent(largeVerify)
                    && reportScanCountsConsistent(largeRelease)
                    && reportScanCountsConsistent(largeReleaseVerify)
                    && reportScanCountsConsistent(largeReleaseStable)
                    && baselineReturnRounds == LIFECYCLE_STRESS_ROUNDS
                    && reportScanCountsConsistent(finalReport);
            boolean passed = rootCasesPassed && initialBaselineReturned
                    && callbackClosureRootPassed && callbackClosureReleased
                    && largeRegistryRetained && largeRegistryRepeatedScansPassed
                    && largeRegistryReleased
                    && repeatedBaseline && scanStatsPassed;
            lifecycleGcPassed = passed;
            return ("status=" + (passed ? "0" : "-1")
                    + " dom_gc=" + (passed ? "PASS" : "FAIL")
                    + " attached_tree_and_live_wrapper=" + (rootCasesPassed ? "PASS" : "FAIL")
                    + " attached_wrapper_recreated=" + (rootCasesPassed ? "PASS" : "FAIL")
                    + " orphan_weakref_cleared=" + (rootCasesPassed ? "PASS" : "FAIL")
                    + " callback_closure_root=" + (callbackClosureRootPassed ? "PASS" : "FAIL")
                    + " callback_closure_release=" + (callbackClosureReleased ? "PASS" : "FAIL")
                    + " large_registry_retained=" + (largeRegistryRetained ? "PASS" : "FAIL")
                    + " large_registry_released=" + (largeRegistryReleased ? "PASS" : "FAIL")
                    + " large_registry_nodes=" + LIFECYCLE_LARGE_REGISTRY_NODES
                    + " large_registry_empty_wrappers=" + largeEmptyWrappers
                    + " large_registry_scan_us=" + largeSetupScanUs
                    + " large_registry_scan_samples=" + LIFECYCLE_LARGE_REGISTRY_SCAN_SAMPLES
                    + " large_registry_scan_samples_us=" + largeRegistryScanSamplesUs
                    + " large_registry_root_buffer_bytes=" + largeSetupRootBufferBytes
                    + " large_registry_reclaimed_buffer_bytes=" + largeReclaimedNodeBufferBytes
                    + " initial_baseline_return=" + (initialBaselineReturned ? "PASS" : "FAIL")
                    + " repeated_baseline=" + (repeatedBaseline ? "PASS" : "FAIL")
                    + " baseline_return_rounds=" + baselineReturnRounds + "/"
                    + LIFECYCLE_STRESS_ROUNDS
                    + " cycles_per_round=" + LIFECYCLE_STRESS_CYCLES_PER_ROUND
                    + " reclaimed_nodes_per_round=" + LIFECYCLE_STRESS_NODES_PER_ROUND
                    + " baseline_nodes=" + baselineNodes + " final_nodes=" + finalNodes
                    + " baseline_string_units=" + baselineStringUnits
                    + " final_string_units=" + finalStringUnits
                    + " baseline_wrapper_handles=" + baselineWrapperHandles
                    + " max_empty_wrappers=" + maximumEmptyWrappers
                    + " collector_scan_stats=" + (scanStatsPassed ? "PASS" : "FAIL")
                    + " collector_succeeded=" + (passed ? "PASS" : "FAIL")
                    + " initial_root_cleanup={" + rootCleanup + "}")
                    .getBytes(StandardCharsets.UTF_8);
        }, "DOM-GC 검증", () -> {
            lifecycleButton.setEnabled(!activityClosing);
            runtimeStatus.setText(lifecycleGcPassed ? "V8 반복 수명 회수 검증 통과"
                    : "V8 반복 수명 회수 검증 실패 · 로그 확인");
        });
    }

    private String callLifecycleFixture(long handle, String method) {
        String source = "globalThis." + LIFECYCLE_FIXTURE_GLOBAL + "." + method + "();";
        return decode(nativeSessionEval(handle, source.getBytes(StandardCharsets.UTF_8)));
    }

    private boolean reportHasHealthyCollection(String report) {
        return report.startsWith("status=0 ")
                && "none".equals(reportField(report, "document_collection_error"))
                && "0".equals(reportField(report, "document_collection_poisoned"))
                && "0".equals(reportField(report, "document_collection_deferred"))
                && reportScanCountsConsistent(report);
    }

    private boolean reportScanCountsConsistent(String report) {
        long scanned = reportLongField(report, "document_collection_scanned_handles");
        long live = reportLongField(report, "document_collection_live_handles");
        long empty = reportLongField(report, "document_collection_empty_handles");
        return scanned >= 0 && live >= 0 && empty >= 0 && scanned == live + empty;
    }

    private boolean reportMatchesResourceBaseline(
            String report, long nodes, long stringUnits, long wrapperHandles) {
        return reportHasHealthyCollection(report)
                && reportLongField(report, "document_nodes") == nodes
                && reportLongField(report, "document_string_units") == stringUnits
                && reportLongField(report, "document_collection_scanned_handles") == wrapperHandles
                && reportLongField(report, "document_collection_live_handles") == wrapperHandles
                && reportLongField(report, "document_collection_empty_handles") == 0;
    }

    private long reportLongField(String report, String name) {
        for (String field : report.split(" ")) {
            if (field.startsWith(name + "=")) {
                try {
                    return Long.parseLong(field.substring(name.length() + 1));
                } catch (NumberFormatException ignored) {
                    return -1;
                }
            }
        }
        return -1;
    }

    private String reportField(String report, String name) {
        for (String field : report.split(" ")) {
            if (field.startsWith(name + "=")) {
                return field.substring(name.length() + 1);
            }
        }
        return "";
    }

    private Button runtimeButton(String label) {
        Button button = new Button(this);
        button.setText(label);
        button.setAllCaps(false);
        button.setTextSize(16);
        button.setTypeface(Typeface.create("sans-serif", Typeface.NORMAL));
        button.setGravity(Gravity.CENTER_VERTICAL | Gravity.START);
        float density = getResources().getDisplayMetrics().density;
        button.setPadding(Math.round(12 * density), 0, Math.round(12 * density), 0);
        button.setTextColor(new ColorStateList(
                new int[][] {{-android.R.attr.state_enabled}, {}},
                new int[] {Color.rgb(109, 114, 128), Color.WHITE}));
        StateListDrawable backgrounds = new StateListDrawable();
        backgrounds.addState(new int[] {android.R.attr.state_pressed},
                roundedButtonBackground(Color.rgb(20, 63, 116), density));
        backgrounds.addState(new int[] {-android.R.attr.state_enabled},
                roundedButtonBackground(Color.rgb(38, 41, 51), density));
        backgrounds.addState(new int[0], roundedButtonBackground(Color.rgb(26, 79, 148), density));
        button.setBackground(backgrounds);
        return button;
    }

    private GradientDrawable roundedButtonBackground(int color, float density) {
        GradientDrawable background = new GradientDrawable();
        background.setColor(color);
        background.setCornerRadius(8 * density);
        return background;
    }

    private void addRuntimeButton(LinearLayout root, Button button, float density) {
        LinearLayout.LayoutParams params = new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT,
                LinearLayout.LayoutParams.WRAP_CONTENT);
        params.topMargin = Math.round(8 * density);
        root.addView(button, params);
    }

    private boolean submitRuntimeCall(java.util.concurrent.Callable<byte[]> call, String label) {
        return submitRuntimeCall(call, label, null);
    }

    private boolean submitRuntimeCall(java.util.concurrent.Callable<byte[]> call, String label,
                                      Runnable onComplete) {
        return submitRuntimeCallWithCompletion(call, label, (result, error) -> {
            if (onComplete != null) onComplete.run();
        });
    }

    private boolean submitRuntimeCallWithCompletion(
            java.util.concurrent.Callable<byte[]> call, String label,
            BiConsumer<byte[], Exception> onComplete) {
        if (activityClosing) return false;
        try {
            runtimeCalls.execute(() -> {
                byte[] result = null;
                Exception failure = null;
                try {
                    Trace.beginSection("SpinonR05:runtime-worker-call");
                    try {
                        result = call.call();
                    } finally {
                        Trace.endSection();
                    }
                    Trace.beginSection("SpinonR05:runtime-result-processing");
                    try {
                        if (label.startsWith("이벤트 ") && result != null) {
                            if (frameAttributionMode) {
                                frameAttributionReports.add(result);
                            } else {
                                Trace.beginSection("SpinonR05:dispatch-report-logcat");
                                try {
                                    Log.i(TAG, "SPINON_RUNTIME_DISPATCH=" + decode(result));
                                } finally {
                                    Trace.endSection();
                                }
                            }
                        }
                    } finally {
                        Trace.endSection();
                    }
                    if (!frameAttributionMode) appendRuntimeLog(label + " " + decode(result));
                } catch (Exception error) {
                    failure = error;
                    if (!frameAttributionMode) appendRuntimeLog(label + " 오류: " + error.getMessage());
                } finally {
                    if (onComplete != null && !activityClosing) {
                        byte[] completedResult = result;
                        Exception completedFailure = failure;
                        boolean traceHandoff = frameAttributionMode;
                        int handoffId = traceHandoff
                                ? runtimeMainThreadHandoffSequence.getAndIncrement() : 0;
                        if (traceHandoff) {
                            Trace.beginAsyncSection("SpinonR05:runtime-main-thread-handoff", handoffId);
                            Trace.beginSection("SpinonR05:runtime-main-thread-post");
                        }
                        boolean posted = false;
                        try {
                            Runnable mainCompletion = () -> {
                                if (traceHandoff) {
                                    Trace.endAsyncSection(
                                            "SpinonR05:runtime-main-thread-handoff", handoffId);
                                    Trace.beginSection("SpinonR05:runtime-main-thread-completion");
                                }
                                try {
                                    onComplete.accept(completedResult, completedFailure);
                                } finally {
                                    if (traceHandoff) Trace.endSection();
                                }
                            };
                            if (asynchronousMainHandoff) {
                                posted = asynchronousMainHandler.post(mainCompletion);
                            } else {
                                runOnUiThread(mainCompletion);
                                posted = true;
                            }
                        } finally {
                            if (traceHandoff) {
                                Trace.endSection();
                                if (!posted) {
                                    Trace.endAsyncSection(
                                            "SpinonR05:runtime-main-thread-handoff", handoffId);
                                }
                            }
                        }
                    }
                }
            });
            return true;
        } catch (RejectedExecutionException error) {
            appendRuntimeLog("호출 대기열이 가득 차거나 닫혀 작업을 거부했습니다 · " + label);
            return false;
        }
    }

    private void setLongEvaluationRunning(boolean running) {
        longEvaluationRunning = running;
        Trace.setCounter("SpinonR05LongEvalRunning", running ? 1 : 0);
        if (!running) {
            cancelRequestPending = false;
            mainHandler.removeCallbacks(runtimeHeartbeat);
            mainHandler.removeCallbacks(longEvaluationTimeout);
        }
        updateLongEvaluationButtons();
    }

    private void finishLongEvaluationIfReady() {
        if (activityClosing || longEvaluationSummaryWritten || !longEvaluationEvalFinished
                || longEvaluationCancelStatus < 0 || pendingDispatchesFromLongEvaluation != 0) {
            return;
        }
        longEvaluationSummaryWritten = true;
        int heartbeatDelta = runtimeHeartbeatCount - longEvaluationHeartbeatStart;
        boolean cancellationPassed = lastLongEvaluationCancelled;
        boolean cancelRequestPassed = longEvaluationCancelStatus == 0;
        boolean dispatchSucceeded = longEvaluationHadDispatch && !pendingDispatchFailed;
        boolean heartbeatPassed = heartbeatDelta >= 5;
        boolean isolateOwnerMatches = longEvaluationOwnerThread >= 0
                && longEvaluationOwnerThread == longEvaluationDispatchOwnerThread;
        boolean callbackOwnerMatches = longEvaluationDispatchOwnerThread >= 0
                && longEvaluationDispatchOwnerThread == longEvaluationCallbackThread
                && longEvaluationDispatchThreadsMatch;

        appendRuntimeLog((cancellationPassed ? "통과" : "실패") + " · 취소된 평가");
        appendRuntimeLog((cancelRequestPassed ? "통과" : "실패") + " · 실행 중 취소 요청");
        if (longEvaluationHadDispatch) {
            appendRuntimeLog((heartbeatPassed ? "통과" : "실패") + " · 메인 UI heartbeat");
            appendRuntimeLog((dispatchSucceeded ? "통과" : "실패") + " · 대기 이벤트 처리");
            appendRuntimeLog((isolateOwnerMatches ? "통과" : "실패") + " · Isolate 소유 스레드");
            appendRuntimeLog((callbackOwnerMatches ? "통과" : "실패") + " · JS 콜백 소유 스레드");
        } else {
            appendRuntimeLog("생략 · UI heartbeat·대기 이벤트 검증 · 이벤트 탭 없음");
        }
        appendRuntimeLog("heartbeat 증가량=" + heartbeatDelta + " · owner_tid="
                + (longEvaluationOwnerThread < 0 ? "없음" : longEvaluationOwnerThread));

        boolean passed = cancellationPassed && cancelRequestPassed
                && (!longEvaluationHadDispatch || (heartbeatPassed && dispatchSucceeded
                        && isolateOwnerMatches && callbackOwnerMatches));
        if (!activityClosing) {
            runtimeStatus.setText(passed
                    ? (dispatchSucceeded
                            ? "취소 완료 · 대기 중이던 JS 이벤트 처리 완료"
                            : "취소 완료 · JavaScript가 종료되었습니다")
                    : "취소 검증 실패 · 상세 결과는 아래 기록을 확인하세요");
        }
    }

    private void updateLongEvaluationButtons() {
        boolean sessionReady = runtimeSession != 0 && !activityClosing;
        if (loopButton != null) {
            loopButton.setEnabled(sessionReady && !longEvaluationRunning
                    && pendingDispatchesFromLongEvaluation == 0);
        }
        if (cancelButton != null) {
            cancelButton.setEnabled(sessionReady && longEvaluationRunning && !cancelRequestPending);
        }
    }

    private String decode(byte[] value) {
        return value == null ? "native result missing" : new String(value, StandardCharsets.UTF_8);
    }

    private void dumpFrameAttributionReports() {
        if (frameAttributionReports.isEmpty()) return;
        new Thread(() -> {
            byte[] report;
            while ((report = frameAttributionReports.poll()) != null) {
                Log.i(TAG, "SPINON_RUNTIME_DISPATCH=" + decode(report));
            }
        }, "spinon-r05-report-dump").start();
    }

    private void appendRuntimeLog(String line) {
        Trace.beginSection("SpinonR05:runtime-log-enqueue");
        try {
            enqueueRuntimeLog(line);
        } finally {
            Trace.endSection();
        }
    }

    private void enqueueRuntimeLog(String line) {
        Log.i(TAG, "SPINON_RUNTIME_UI " + line);
        synchronized (runtimeLogPendingLock) {
            pendingRuntimeLogEntries.append(line).append("\n\n");
            if (runtimeLogFlushScheduled) return;
            runtimeLogFlushScheduled = true;
        }
        if (!mainHandler.post(this::flushRuntimeLog)) {
            synchronized (runtimeLogPendingLock) {
                runtimeLogFlushScheduled = false;
            }
        }
    }

    private void flushRuntimeLog() {
        Trace.beginSection("SpinonR05:runtime-log-flush");
        try {
            flushRuntimeLogContents();
        } finally {
            Trace.endSection();
        }
    }

    private void flushRuntimeLogContents() {
        String entries;
        synchronized (runtimeLogPendingLock) {
            entries = pendingRuntimeLogEntries.toString();
            pendingRuntimeLogEntries.setLength(0);
            runtimeLogFlushScheduled = false;
        }
        if (activityClosing || runtimeLog == null || entries.isEmpty()) return;

        runtimeLogBuffer.append(entries);
        if (runtimeLogBuffer.length() > MAX_RUNTIME_LOG_CHARS) {
            int trimStart = runtimeLogBuffer.length() - RUNTIME_LOG_TRIM_TARGET_CHARS;
            int nextLine = runtimeLogBuffer.indexOf("\n", trimStart);
            runtimeLogBuffer.delete(0, nextLine >= 0 ? nextLine + 1 : trimStart);
            runtimeLog.setText(RUNTIME_LOG_TRUNCATION_MARKER + runtimeLogBuffer);
        } else {
            runtimeLog.append(entries);
        }

        ScrollView scroll = runtimeScroll;
        if (scroll != null && !runtimeLogScrollPending) {
            runtimeLogScrollPending = true;
            boolean posted = scroll.post(() -> {
                runtimeLogScrollPending = false;
                if (!activityClosing && runtimeScroll == scroll) {
                    Trace.beginSection("SpinonR05:runtime-log-scroll");
                    try {
                        scroll.fullScroll(View.FOCUS_DOWN);
                    } finally {
                        Trace.endSection();
                    }
                }
            });
            if (!posted) runtimeLogScrollPending = false;
        }
    }

    @Override
    protected void onPause() {
        if (hostGlesSurface != null) hostGlesSurface.onHostPaused();
        if (hostGpuSurface != null) {
            hostGpuSurface.onHostPaused();
            gpuWasPaused = true;
        }
        super.onPause();
    }

    @Override
    protected void onResume() {
        super.onResume();
        if (hostGlesSurface != null) hostGlesSurface.onHostResumed();
        if (hostGpuSurface != null && gpuWasPaused) {
            gpuWasPaused = false;
            hostGpuSurface.onHostResumed();
        }
    }

    @Override
    public void onConfigurationChanged(Configuration newConfig) {
        super.onConfigurationChanged(newConfig);
        if (runtimeGpuDemo != null) runtimeGpuDemo.updateColorScheme(newConfig);
    }

    @Override
    protected void onDestroy() {
        R05PresentFenceWaitExperiment.disable();
        activityClosing = true;
        if (runtimeGpuDemo != null) runtimeGpuDemo.dispose();
        mainHandler.removeCallbacks(runtimeHeartbeat);
        mainHandler.removeCallbacks(longEvaluationTimeout);
        bootstrapExecutor.shutdownNow();
        delayedHost.shutdownNow();
        runtimeCalls.shutdown();
        try {
            runtimeControl.execute(() -> {
                long handle = runtimeSession;
                if (handle != 0) nativeSessionCancel(handle);
                boolean interrupted = false;
                while (!runtimeCalls.isTerminated()) {
                    try {
                        runtimeCalls.awaitTermination(Long.MAX_VALUE, TimeUnit.NANOSECONDS);
                    } catch (InterruptedException error) {
                        interrupted = true;
                    }
                }
                handle = runtimeSession;
                if (handle != 0) {
                    nativeSessionFree(handle);
                    runtimeSession = 0;
                }
                if (interrupted) Thread.currentThread().interrupt();
            });
        } catch (RejectedExecutionException error) {
            Log.w(TAG, "R06 종료 정리 작업을 큐에 넣지 못했습니다", error);
        }
        runtimeControl.shutdown();
        super.onDestroy();
    }

    private String readAsset(String name) throws IOException {
        try (InputStream input = getAssets().open(name);
             ByteArrayOutputStream output = new ByteArrayOutputStream()) {
            byte[] buffer = new byte[4096];
            int count;
            while ((count = input.read(buffer)) != -1) {
                output.write(buffer, 0, count);
            }
            return new String(output.toByteArray(), StandardCharsets.UTF_8);
        }
    }

    private void showR10Report(String report, float density) {
        TextView text = new TextView(this);
        text.setText("SPINON · R10 TAFFY 실험\n\nAndroid 에뮬레이터 · 개발 전용\n\n" + report);
        text.setTextColor(Color.rgb(230, 237, 248));
        text.setTextSize(12);
        text.setTypeface(Typeface.MONOSPACE);
        int inset = Math.round(18 * density);
        text.setPadding(inset, Math.round(24 * density), inset, Math.round(24 * density));

        ScrollView scroll = new ScrollView(this);
        scroll.setBackgroundColor(Color.rgb(14, 19, 31));
        scroll.addView(text);
        setContentView(scroll);
        scroll.post(() -> scroll.scrollTo(0, 0));
    }

    private String r10Report(String output) {
        String[] markers = {"SPINON_TAFFY_R10_RESULT=", "SPINON_TAFFY_R10_ERROR="};
        for (String marker : markers) {
            int start = output.indexOf(marker);
            if (start >= 0) {
                return formatR10Report(output.substring(start + marker.length()).trim());
            }
        }
        return formatR10Report(output);
    }

    private String formatR10Report(String report) {
        return report
                .replace(" nodes=", "\nnodes=")
                .replace(" text-id=", "\ntext-id=")
                .replace(" measured=", "\nmeasured=")
                .replace(" rtl=", "\nrtl=")
                .replace(" ltr-button-offset=", "\nltr-button-offset=")
                .replace(" rtl-text-offset=", "\nrtl-text-offset=")
                .replace(" update=equivalent", "\nupdate=equivalent")
                .replace(" rounding=[", "\nrounding:\n  ")
                .replace(",physical-pixel=", "\n  physical-pixel=")
                .replace(",float=", "\n  float=")
                .replace("] update-us-p50=", "\nupdate-us: p50=")
                .replace(" update-us-p95=", " p95=")
                .replace(" rebuild-us-p50=", "\nrebuild-us: p50=")
                .replace(" rebuild-us-p95=", " p95=")
                .replace(" iterations=", "\niterations=");
    }
}
