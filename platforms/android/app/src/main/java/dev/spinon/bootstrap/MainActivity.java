package dev.spinon.bootstrap;

import android.app.Activity;
import android.graphics.Color;
import android.graphics.Typeface;
import android.os.Bundle;
import android.os.Looper;
import android.util.DisplayMetrics;
import android.util.Log;
import android.view.Gravity;
import android.widget.Button;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.RejectedExecutionException;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.ThreadPoolExecutor;
import java.util.concurrent.TimeUnit;
import java.util.function.BiConsumer;

public final class MainActivity extends Activity {
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
    private static native byte[] nativeSessionShutdownProbe();
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
    private boolean gpuWasPaused;
    private volatile long runtimeSession;
    private volatile boolean activityClosing;
    private TextView runtimeLog;
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
    private int pendingDispatchesFromLongEvaluation;
    private int tapCount;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        if (getIntent().getBooleanExtra("spinon_priority_probe", false)) {
            showPriorityProbe();
            return;
        }
        if (getIntent().getBooleanExtra("spinon_shutdown_probe", false)) {
            showShutdownProbe();
            return;
        }
        boolean runR13 = getIntent().getBooleanExtra("spinon_r13", false);
        boolean runS04 = getIntent().getBooleanExtra("spinon_s04", false);
        if (runS04 || runR13 || getIntent().getBooleanExtra("spinon_r08", false)) {
            int backend = getIntent().getIntExtra("spinon_r08_backend", 1);
            boolean useWgpu = runS04 || runR13
                    || !getIntent().getBooleanExtra("spinon_r08_native", false);
            int failureInjection = getIntent().getIntExtra("spinon_r13_failure", 0);
            int recoveryFailureInjection = getIntent().getIntExtra("spinon_r13_recovery_failure", 0);
            R08WgpuSurface surface = R08GpuDemo.show(
                    this, useWgpu, backend, runR13, runS04, failureInjection,
                    recoveryFailureInjection);
            hostGpuSurface = runS04 || runR13 ? surface : null;
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
        description.setText("개발 전용 · 실제 V8의 세 우선순위 선택과 등급별 FIFO를 확인합니다");
        description.setTextColor(Color.rgb(170, 184, 207));
        description.setTextSize(13);
        description.setPadding(0, Math.round(8 * density), 0, Math.round(12 * density));
        root.addView(description);

        TextView status = new TextView(this);
        status.setText("실제 V8 우선순위 검증 중…");
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
            String result = decode(nativeSessionPriorityProbe());
            Log.i(TAG, "SPINON_PRIORITY_PROBE " + result);
            runOnUiThread(() -> {
                boolean passed = result.contains("status=0 priority_probe=PASS");
                status.setText(passed ? "실제 V8 우선순위 검증 통과" : "실제 V8 우선순위 검증 실패");
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
                : "개발용 실험 · 호출 스레드와 V8 소유 스레드, 큐 대기·실행 시간 확인");
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
        root.addView(dispatchButton);
        root.addView(loopButton);
        root.addView(cancelButton);
        root.addView(delayedButton);
        root.addView(lifecycleButton);

        ScrollView scroll = new ScrollView(this);
        runtimeLog = new TextView(this);
        runtimeLog.setTextColor(Color.rgb(218, 226, 240));
        runtimeLog.setTextSize(12);
        runtimeLog.setTypeface(Typeface.MONOSPACE);
        runtimeLog.setPadding(0, Math.round(12 * density), 0, Math.round(20 * density));
        scroll.addView(runtimeLog);
        root.addView(scroll, new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT, 0, 1));
        setContentView(root);

        dispatchButton.setOnClickListener(view -> {
            int nodeId = ++tapCount;
            runtimeStatus.setText("UI 탭 " + nodeId + "회 · UI는 계속 입력을 받습니다");
            boolean submittedDuringLongEvaluation = longEvaluationRunning;
            if (submittedDuringLongEvaluation) {
                longEvaluationHadDispatch = true;
                pendingDispatchesFromLongEvaluation++;
            }
            boolean accepted = submitRuntimeCallWithCompletion(
                    () -> nativeSessionDispatch(runtimeSession, nodeId),
                    "dispatch", (result, error) -> {
                        if (!submittedDuringLongEvaluation) return;
                        boolean dispatchSucceeded = error == null
                                && decode(result).startsWith("status=0 ");
                        pendingDispatchesFromLongEvaluation--;
                        if (!dispatchSucceeded) pendingDispatchFailed = true;
                        updateLongEvaluationButtons();
                        if (!longEvaluationRunning && pendingDispatchesFromLongEvaluation == 0
                                && !activityClosing) {
                            String eventResult = pendingDispatchFailed
                                    ? "대기 중이던 JS 이벤트 처리 실패"
                                    : "대기 중이던 JS 이벤트 처리 완료";
                            runtimeStatus.setText(lastLongEvaluationCancelled
                                    ? "취소 완료 · " + eventResult
                                    : "준비됨 · " + eventResult);
                        }
                    });
            if (!accepted && submittedDuringLongEvaluation) {
                pendingDispatchesFromLongEvaluation--;
                pendingDispatchFailed = true;
                updateLongEvaluationButtons();
            }
        });
        loopButton.setOnClickListener(view -> {
            if (longEvaluationRunning || pendingDispatchesFromLongEvaluation != 0
                    || runtimeSession == 0 || activityClosing) return;
            lastLongEvaluationCancelled = false;
            longEvaluationHadDispatch = false;
            pendingDispatchFailed = false;
            setLongEvaluationRunning(true);
            appendRuntimeLog("무한 JS 평가를 시작했습니다. 화면은 계속 탭할 수 있어야 합니다.");
            runtimeStatus.setText("긴 JavaScript 실행 중 · UI 입력은 가능하고 JS 이벤트는 대기합니다");
            boolean accepted = submitRuntimeCallWithCompletion(() -> nativeSessionEval(runtimeSession,
                    "while (true) { /* 취소 경로 검증 */ }".getBytes(StandardCharsets.UTF_8)),
                    "long-eval", (result, error) -> {
                        String report = decode(result);
                        lastLongEvaluationCancelled = error == null
                                && report.startsWith("status=-8 ");
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
                    });
            if (!accepted) {
                setLongEvaluationRunning(false);
                runtimeStatus.setText("실행 대기열이 가득 차 긴 JavaScript를 시작하지 못했습니다");
            }
        });
        cancelButton.setOnClickListener(view -> {
            if (!longEvaluationRunning || cancelRequestPending || activityClosing) return;
            long handle = runtimeSession;
            if (handle == 0) return;
            cancelRequestPending = true;
            updateLongEvaluationButtons();
            runtimeStatus.setText("JavaScript 취소 요청 중…");
            try {
                runtimeControl.execute(() -> {
                    int result = nativeSessionCancel(handle);
                    appendRuntimeLog("취소 요청 status=" + result
                            + " (0=실행 중 취소 요청, 1=실행 중인 JS 없음)");
                    runOnUiThread(() -> {
                        if (activityClosing || !longEvaluationRunning || !cancelRequestPending) return;
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
                    });
                });
            } catch (RejectedExecutionException error) {
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
                submitRuntimeCall(() -> nativeSessionEval(runtimeSession,
                        "spinon.setText('delayed-host-response')".getBytes(StandardCharsets.UTF_8)),
                        "delayed-host-response", () -> delayedButton.setEnabled(!activityClosing));
            }, 500, TimeUnit.MILLISECONDS);
        });
        lifecycleButton.setOnClickListener(view -> runLifecycleCollectionProbe());

        submitRuntimeCall(() -> {
            long handle = nativeSessionCreate();
            runtimeSession = handle;
            if (handle == 0) throw new IllegalStateException("V8 세션 생성 실패");
            return nativeSessionEval(handle, source.getBytes(StandardCharsets.UTF_8));
        }, "session-create", () -> {
            if (runtimeSession == 0) {
                runtimeStatus.setText("세션 생성 실패 · logcat 확인");
                return;
            }
            runtimeStatus.setText("준비됨 · Android 실행기 스레드에서 V8을 소유합니다");
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
        button.setGravity(Gravity.CENTER);
        return button;
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
                    result = call.call();
                    appendRuntimeLog(label + " " + decode(result));
                } catch (Exception error) {
                    failure = error;
                    appendRuntimeLog(label + " 오류: " + error.getMessage());
                } finally {
                    if (onComplete != null && !activityClosing) {
                        byte[] completedResult = result;
                        Exception completedFailure = failure;
                        runOnUiThread(() -> onComplete.accept(completedResult, completedFailure));
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
        if (!running) cancelRequestPending = false;
        updateLongEvaluationButtons();
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

    private void appendRuntimeLog(String line) {
        Log.i(TAG, "SPINON_RUNTIME_UI " + line);
        runOnUiThread(() -> {
            if (activityClosing || runtimeLog == null) return;
            runtimeLog.append(line);
            runtimeLog.append("\n\n");
        });
    }

    @Override
    protected void onPause() {
        if (hostGpuSurface != null) {
            hostGpuSurface.onHostPaused();
            gpuWasPaused = true;
        }
        super.onPause();
    }

    @Override
    protected void onResume() {
        super.onResume();
        if (hostGpuSurface != null && gpuWasPaused) {
            gpuWasPaused = false;
            hostGpuSurface.onHostResumed();
        }
    }

    @Override
    protected void onDestroy() {
        activityClosing = true;
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
