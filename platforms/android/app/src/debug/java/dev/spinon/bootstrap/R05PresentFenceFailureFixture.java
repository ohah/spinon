package dev.spinon.bootstrap;

import android.app.Activity;
import android.graphics.Color;
import android.os.Build;
import android.os.Handler;
import android.os.Looper;
import android.os.SystemClock;
import android.util.Log;
import android.view.Gravity;
import android.widget.TextView;

import java.util.concurrent.CountDownLatch;
import java.util.concurrent.CyclicBarrier;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;
import java.util.concurrent.atomic.AtomicReference;

/** debug 전용 Android R05 transaction callback 실패 주입 fixture입니다. */
final class R05PresentFenceFailureFixture {
    private static final String TAG = "SpinonBootstrap";
    private static final int RACE_ITERATIONS = 100;
    private static final int IDLE_SAMPLES = 60;
    private static final long IDLE_INTERVAL_MS = 1_000;
    private static final long SCENARIO_TIMEOUT_SECONDS = 10;
    private static final Handler MAIN_HANDLER = new Handler(Looper.getMainLooper());

    private final Activity activity;
    private final TextView statusView;
    private final AtomicInteger checks = new AtomicInteger();
    private final AtomicInteger failures = new AtomicInteger();
    private volatile R05PresentFenceProbe.FailureFixtureRequest actualTimeoutRequest;
    private volatile R05PresentFenceProbe.FailureFixtureRequest actualCancelRequest;
    private long idleCallbackBaseline;
    private long idleTimeoutBaseline;
    private int idleSampleCount;
    private String firstIdleFailure = "none";

    private R05PresentFenceFailureFixture(Activity activity) {
        this.activity = activity;
        statusView = new TextView(activity);
        statusView.setTextColor(Color.WHITE);
        statusView.setTextSize(18);
        statusView.setGravity(Gravity.CENTER);
        statusView.setBackgroundColor(Color.rgb(14, 19, 31));
    }

    static void start(Activity activity) {
        if (!BuildConfig.DEBUG) {
            Log.e(TAG, "SPINON_R05_CALLBACK_FAULT_SUMMARY status=FAIL reason=debug_only");
            activity.finish();
            return;
        }
        if (Build.VERSION.SDK_INT < 35) {
            Log.e(TAG, "SPINON_R05_CALLBACK_FAULT_SUMMARY status=UNAVAILABLE api="
                    + Build.VERSION.SDK_INT + " minimum_api=35");
            activity.finish();
            return;
        }
        if (Looper.myLooper() != Looper.getMainLooper()) {
            Log.e(TAG, "SPINON_R05_CALLBACK_FAULT_SUMMARY status=FAIL reason=not_main_thread");
            activity.finish();
            return;
        }
        if (!R05PresentFenceProbe.failureFixtureIdle()) {
            Log.e(TAG, "SPINON_R05_CALLBACK_FAULT_SUMMARY status=FAIL reason=precondition_busy"
                    + " pending=" + R05PresentFenceProbe.failurePendingCount()
                    + " queue=" + R05PresentFenceProbe.failureCallbackQueueDepth()
                    + " active=" + R05PresentFenceProbe.failureCallbackActiveCount());
            activity.finish();
            return;
        }
        new R05PresentFenceFailureFixture(activity).begin();
    }

    private void begin() {
        statusView.setText("R05 표시 callback 실패 주입\n결정적 실패 경로를 확인하는 중…");
        activity.setContentView(statusView);
        Log.i(TAG, "SPINON_R05_CALLBACK_FAULT_START api=" + Build.VERSION.SDK_INT);
        Thread worker = new Thread(this::runSyntheticScenarios,
                "SpinonR05CallbackFaultFixture");
        worker.setDaemon(true);
        worker.start();
    }

    private void runSyntheticScenarios() {
        runTimeoutThenLateCallback();
        runCancelOwnerAndLateCallback();
        runSurfaceGenerationCancellation();
        runDuplicateAndMissingStats();
        runStaleGeneration();
        runTimeoutCallbackRace();
        runExecutorOverflow();
        boolean appliedTransactionPassed =
                R05AppliedTransactionLifecycleFixture.run(activity, statusView);
        record("applied_transaction_surface_lifecycle", appliedTransactionPassed,
                "renderer=opengl_es input_source=synthetic_fixture");
        MAIN_HANDLER.post(this::beginActualTimeouts);
    }

    private void runTimeoutThenLateCallback() {
        R05PresentFenceProbe.FailureFixtureRequest request = synthetic("wgpu", 101,
                () -> 101L, () -> true);
        if (request == null) {
            record("synthetic_timeout_late", false, "request_create_failed");
            return;
        }
        R05PresentFenceProbe.timeoutForFailureFixture(request);
        boolean timedOut = "timed_out".equals(R05PresentFenceProbe.failureState(request))
                && R05PresentFenceProbe.failureTimeoutCount(request) == 1
                && !R05PresentFenceProbe.failureRequestPending(request);
        R05PresentFenceProbe.CallbackObservation late =
                R05PresentFenceProbe.callbackForFailureFixture(request);
        record("synthetic_timeout_late", timedOut
                        && "late_after_timeout".equals(late.outcome)
                        && !late.fenceSignalUsable
                        && "timed_out".equals(R05PresentFenceProbe.failureState(request))
                        && !R05PresentFenceProbe.failureRequestPending(request),
                "outcome=" + late.outcome + " timeout_count="
                        + R05PresentFenceProbe.failureTimeoutCount(request));
    }

    private void runCancelOwnerAndLateCallback() {
        R05PresentFenceProbe.FailureFixtureRequest request = synthetic("wgpu", 102,
                () -> 102L, () -> true);
        if (request == null) {
            record("cancel_owner_late", false, "request_create_failed");
            return;
        }
        R05PresentFenceProbe.cancelForFailureFixture(request, "opengl_es", 102,
                "fixture_wrong_owner");
        boolean ownerMismatchPreserved = R05PresentFenceProbe.failureRequestPending(request)
                && "pending".equals(R05PresentFenceProbe.failureState(request));
        R05PresentFenceProbe.cancelForFailureFixture(request, "wgpu", 102,
                "fixture_cancel");
        boolean cancelled = "cancelled".equals(R05PresentFenceProbe.failureState(request))
                && !R05PresentFenceProbe.failureRequestPending(request)
                && !R05PresentFenceProbe.failureTimeoutScheduled(request);
        R05PresentFenceProbe.CallbackObservation late =
                R05PresentFenceProbe.callbackForFailureFixture(request);
        record("cancel_owner_late", ownerMismatchPreserved && cancelled
                        && "late_after_cancel".equals(late.outcome)
                        && !late.fenceSignalUsable
                        && "cancelled".equals(R05PresentFenceProbe.failureState(request)),
                "owner_mismatch_preserved=" + ownerMismatchPreserved
                        + " late_outcome=" + late.outcome);
    }

    private void runSurfaceGenerationCancellation() {
        R05PresentFenceProbe.FailureFixtureRequest wgpu = synthetic("wgpu", 103,
                () -> 103L, () -> true);
        R05PresentFenceProbe.FailureFixtureRequest gles = synthetic("opengl_es", 103,
                () -> 103L, () -> true);
        if (wgpu == null || gles == null) {
            if (wgpu != null) cancel(wgpu, "wgpu", 103);
            if (gles != null) cancel(gles, "opengl_es", 103);
            record("surface_generation_cancel", false, "request_create_failed");
            return;
        }
        R05PresentFenceProbe.cancelSurfaceForFailureFixture("wgpu", 103,
                "fixture_surface_replaced");
        boolean wgpuCancelled = "cancelled".equals(R05PresentFenceProbe.failureState(wgpu))
                && !R05PresentFenceProbe.failureRequestPending(wgpu);
        boolean glesPreserved = "pending".equals(R05PresentFenceProbe.failureState(gles))
                && R05PresentFenceProbe.failureRequestPending(gles);
        cancel(gles, "opengl_es", 103);
        R05PresentFenceProbe.CallbackObservation late =
                R05PresentFenceProbe.callbackForFailureFixture(wgpu);
        record("surface_generation_cancel", wgpuCancelled && glesPreserved
                        && "late_after_cancel".equals(late.outcome)
                        && !late.fenceSignalUsable,
                "wgpu_cancelled=" + wgpuCancelled + " gles_preserved=" + glesPreserved);
    }

    private void runDuplicateAndMissingStats() {
        R05PresentFenceProbe.FailureFixtureRequest request = synthetic("wgpu", 104,
                () -> 104L, () -> true);
        if (request == null) {
            record("duplicate_missing_stats", false, "request_create_failed");
            return;
        }
        R05PresentFenceProbe.CallbackObservation first =
                R05PresentFenceProbe.callbackForFailureFixture(request);
        R05PresentFenceProbe.CallbackObservation duplicate =
                R05PresentFenceProbe.callbackForFailureFixture(request);
        record("duplicate_missing_stats",
                "callback_received".equals(first.outcome)
                        && "duplicate_callback".equals(duplicate.outcome)
                        && "null_transaction_stats".equals(first.fenceState)
                        && first.currentSurface && !first.fenceSignalUsable
                        && !duplicate.fenceSignalUsable
                        && "callback_received".equals(R05PresentFenceProbe.failureState(request))
                        && !R05PresentFenceProbe.failureRequestPending(request),
                "first=" + first.outcome + " duplicate=" + duplicate.outcome
                        + " fence=" + first.fenceState);
    }

    private void runStaleGeneration() {
        R05PresentFenceProbe.FailureFixtureRequest request = synthetic("wgpu", 105,
                () -> 106L, () -> true);
        if (request == null) {
            record("stale_generation", false, "request_create_failed");
            return;
        }
        R05PresentFenceProbe.CallbackObservation observation =
                R05PresentFenceProbe.callbackForFailureFixture(request);
        record("stale_generation", "callback_received".equals(observation.outcome)
                        && !observation.currentSurface && !observation.fenceSignalUsable
                        && !R05PresentFenceProbe.failureRequestPending(request),
                "current_surface=" + observation.currentSurface
                        + " fence_usable=" + observation.fenceSignalUsable);
    }

    private void runTimeoutCallbackRace() {
        ExecutorService raceWorkers = Executors.newFixedThreadPool(2);
        int callbackWins = 0;
        int timeoutWins = 0;
        boolean passed = true;
        try {
            for (int iteration = 0; iteration < RACE_ITERATIONS; iteration++) {
                long generation = 200L + iteration;
                R05PresentFenceProbe.FailureFixtureRequest request = synthetic(
                        "wgpu", generation, () -> generation, () -> true);
                if (request == null) {
                    passed = false;
                    break;
                }
                CyclicBarrier start = new CyclicBarrier(3);
                R05PresentFenceProbe.FailureFixtureRequest current = request;
                Future<?> timeout = raceWorkers.submit(() -> {
                    start.await(SCENARIO_TIMEOUT_SECONDS, TimeUnit.SECONDS);
                    R05PresentFenceProbe.timeoutForFailureFixture(current);
                    return null;
                });
                Future<R05PresentFenceProbe.CallbackObservation> callback = raceWorkers.submit(() -> {
                    start.await(SCENARIO_TIMEOUT_SECONDS, TimeUnit.SECONDS);
                    return R05PresentFenceProbe.callbackForFailureFixture(current);
                });
                start.await(SCENARIO_TIMEOUT_SECONDS, TimeUnit.SECONDS);
                timeout.get(SCENARIO_TIMEOUT_SECONDS, TimeUnit.SECONDS);
                R05PresentFenceProbe.CallbackObservation observation =
                        callback.get(SCENARIO_TIMEOUT_SECONDS, TimeUnit.SECONDS);
                String state = R05PresentFenceProbe.failureState(request);
                if ("callback_received".equals(state)) callbackWins++;
                else if ("timed_out".equals(state)) timeoutWins++;
                else passed = false;
                passed &= !R05PresentFenceProbe.failureRequestPending(request)
                        && !observation.fenceSignalUsable
                        && ("callback_received".equals(observation.outcome)
                        || "late_after_timeout".equals(observation.outcome));
                if (!passed) break;
            }
        } catch (Exception error) {
            passed = false;
            Log.e(TAG, "SPINON_R05_CALLBACK_FAULT scenario=timeout_callback_race"
                    + " outcome=exception error=" + error.getClass().getSimpleName());
        } finally {
            raceWorkers.shutdownNow();
            try {
                passed &= raceWorkers.awaitTermination(3, TimeUnit.SECONDS);
            } catch (InterruptedException error) {
                Thread.currentThread().interrupt();
                passed = false;
            }
        }
        record("timeout_callback_race", passed
                        && callbackWins + timeoutWins == RACE_ITERATIONS
                        && R05PresentFenceProbe.failurePendingCount() == 0,
                "iterations=" + (callbackWins + timeoutWins)
                        + " callback_wins=" + callbackWins + " timeout_wins=" + timeoutWins
                        + " pending=" + R05PresentFenceProbe.failurePendingCount());
    }

    private void runExecutorOverflow() {
        if (R05PresentFenceProbe.failureCallbackActiveCount() != 0
                || R05PresentFenceProbe.failureCallbackQueueDepth() != 0
                || R05PresentFenceProbe.failurePendingCount() != 0) {
            record("callback_executor_overflow", false, "precondition_busy");
            return;
        }

        CountDownLatch workerEntered = new CountDownLatch(1);
        CountDownLatch releaseWorker = new CountDownLatch(1);
        CountDownLatch lastQueuedTaskDone = new CountDownLatch(1);
        AtomicInteger queuedTasksExecuted = new AtomicInteger();
        AtomicLong overflowThreadId = new AtomicLong(-1L);
        AtomicReference<R05PresentFenceProbe.CallbackObservation> overflowObservation =
                new AtomicReference<>();
        AtomicReference<R05PresentFenceProbe.FailureFixtureRequest> overflowRequest =
                new AtomicReference<>();
        long inlineBefore = R05PresentFenceProbe.failureInlineCallbackTotal();
        long callerThreadId = Thread.currentThread().getId();
        boolean submitted = false;
        boolean entered = false;
        boolean queueFilled = false;
        boolean queueDrained = false;
        try {
            R05PresentFenceProbe.executeCallbackForFailureFixture(() -> {
                workerEntered.countDown();
                try {
                    if (!releaseWorker.await(SCENARIO_TIMEOUT_SECONDS, TimeUnit.SECONDS)) {
                        throw new IllegalStateException("worker_release_timeout");
                    }
                } catch (InterruptedException error) {
                    Thread.currentThread().interrupt();
                    throw new IllegalStateException("worker_interrupted", error);
                }
            });
            entered = workerEntered.await(2, TimeUnit.SECONDS);
            if (!entered) throw new IllegalStateException("callback_worker_not_started");

            for (int index = 0; index < 64; index++) {
                boolean lastTask = index == 63;
                R05PresentFenceProbe.executeCallbackForFailureFixture(() -> {
                    queuedTasksExecuted.incrementAndGet();
                    if (lastTask) lastQueuedTaskDone.countDown();
                });
            }
            queueFilled = R05PresentFenceProbe.failureCallbackQueueDepth() == 64;
            if (!queueFilled) throw new IllegalStateException("queue_capacity_not_reached");

            R05PresentFenceProbe.FailureFixtureRequest request = synthetic("wgpu", 306,
                    () -> 306L, () -> true);
            overflowRequest.set(request);
            if (request == null) throw new IllegalStateException("overflow_request_failed");
            R05PresentFenceProbe.executeCallbackForFailureFixture(() -> {
                overflowThreadId.set(Thread.currentThread().getId());
                overflowObservation.set(
                        R05PresentFenceProbe.callbackForFailureFixture(request));
            });
            submitted = true;
        } catch (Exception error) {
            Log.e(TAG, "SPINON_R05_CALLBACK_FAULT scenario=callback_executor_overflow"
                    + " outcome=exception error=" + error.getClass().getSimpleName());
        } finally {
            releaseWorker.countDown();
            try {
                queueDrained = lastQueuedTaskDone.await(5, TimeUnit.SECONDS)
                        && awaitExecutorIdle(5_000);
            } catch (InterruptedException error) {
                Thread.currentThread().interrupt();
            }
            R05PresentFenceProbe.FailureFixtureRequest request = overflowRequest.get();
            if (request != null && R05PresentFenceProbe.failureRequestPending(request)) {
                cancel(request, "wgpu", 306);
            }
        }

        R05PresentFenceProbe.CallbackObservation observation = overflowObservation.get();
        boolean passed = entered && queueFilled && submitted && queueDrained
                && queuedTasksExecuted.get() == 64
                && overflowThreadId.get() == callerThreadId
                && R05PresentFenceProbe.failureInlineCallbackTotal() - inlineBefore == 1
                && observation != null && observation.callbackInlineOverflow
                && !observation.fenceSignalUsable
                && "null_transaction_stats".equals(observation.fenceState)
                && R05PresentFenceProbe.failurePendingCount() == 0
                && R05PresentFenceProbe.failureCallbackQueueDepth() == 0
                && R05PresentFenceProbe.failureCallbackActiveCount() == 0;
        record("callback_executor_overflow", passed,
                "queued=" + queuedTasksExecuted.get() + " queue_drained=" + queueDrained
                        + " caller_runs=" + (overflowThreadId.get() == callerThreadId)
                        + " inline_delta="
                        + (R05PresentFenceProbe.failureInlineCallbackTotal() - inlineBefore)
                        + " fence_usable="
                        + (observation != null && observation.fenceSignalUsable));
    }

    private void beginActualTimeouts() {
        if (!R05PresentFenceProbe.failureFixtureIdle()
                || R05PresentFenceProbe.failurePendingCount() != 0) {
            record("actual_timeout_precondition", false, "fixture_not_idle");
        }
        actualCancelRequest = R05PresentFenceProbe.attachUnappliedForFailureFixture("wgpu", 901);
        boolean cancelAttached = actualCancelRequest != null
                && R05PresentFenceProbe.failureTimeoutScheduled(actualCancelRequest);
        if (actualCancelRequest != null) {
            R05PresentFenceProbe.cancelForFailureFixture(actualCancelRequest, "wgpu", 901,
                    "fixture_actual_cancel");
        }
        boolean cancelRemoved = actualCancelRequest != null
                && "cancelled".equals(R05PresentFenceProbe.failureState(actualCancelRequest))
                && !R05PresentFenceProbe.failureRequestPending(actualCancelRequest)
                && !R05PresentFenceProbe.failureTimeoutScheduled(actualCancelRequest)
                && R05PresentFenceProbe.closeFailureTransaction(actualCancelRequest);
        record("actual_cancel_removes_timeout", cancelAttached && cancelRemoved,
                "attached=" + cancelAttached + " timer_removed=" + cancelRemoved);

        actualTimeoutRequest = R05PresentFenceProbe.attachUnappliedForFailureFixture("wgpu", 902);
        boolean timeoutAttached = actualTimeoutRequest != null
                && R05PresentFenceProbe.failureRequestPending(actualTimeoutRequest)
                && R05PresentFenceProbe.failureTimeoutScheduled(actualTimeoutRequest);
        record("actual_unapplied_transaction_registered", timeoutAttached,
                "listener_attached=" + timeoutAttached);
        MAIN_HANDLER.postDelayed(this::verifyActualTimeout, 2_500);
    }

    private void verifyActualTimeout() {
        boolean timedOut = actualTimeoutRequest != null
                && "timed_out".equals(R05PresentFenceProbe.failureState(actualTimeoutRequest))
                && R05PresentFenceProbe.failureTimeoutCount(actualTimeoutRequest) == 1
                && R05PresentFenceProbe.failureCallbackCount(actualTimeoutRequest) == 0
                && !R05PresentFenceProbe.failureRequestPending(actualTimeoutRequest)
                && !R05PresentFenceProbe.failureTimeoutScheduled(actualTimeoutRequest);
        boolean transactionClosed = actualTimeoutRequest != null
                && R05PresentFenceProbe.closeFailureTransaction(actualTimeoutRequest);
        record("actual_main_handler_timeout", timedOut && transactionClosed,
                "state=" + (actualTimeoutRequest == null ? "missing"
                        : R05PresentFenceProbe.failureState(actualTimeoutRequest))
                        + " timeout_count=" + (actualTimeoutRequest == null ? -1
                        : R05PresentFenceProbe.failureTimeoutCount(actualTimeoutRequest))
                        + " callback_count=" + (actualTimeoutRequest == null ? -1
                        : R05PresentFenceProbe.failureCallbackCount(actualTimeoutRequest))
                        + " transaction_closed=" + transactionClosed);
        beginIdleObservation();
    }

    private void beginIdleObservation() {
        idleCallbackBaseline = R05PresentFenceProbe.failureCallbackEventTotal();
        idleTimeoutBaseline = R05PresentFenceProbe.failureTimeoutEventTotal();
        idleSampleCount = 0;
        firstIdleFailure = "none";
        sampleIdleState();
    }

    private void sampleIdleState() {
        idleSampleCount++;
        boolean clean = R05PresentFenceProbe.failurePendingCount() == 0
                && R05PresentFenceProbe.failureCallbackQueueDepth() == 0
                && R05PresentFenceProbe.failureCallbackActiveCount() == 0
                && R05PresentFenceProbe.failureCallbackEventTotal() == idleCallbackBaseline
                && R05PresentFenceProbe.failureTimeoutEventTotal() == idleTimeoutBaseline
                && R05PresentFenceProbe.failureCallbackCount(actualTimeoutRequest) == 0
                && R05PresentFenceProbe.failureCallbackCount(actualCancelRequest) == 0;
        if (!clean && "none".equals(firstIdleFailure)) {
            firstIdleFailure = "sample_" + idleSampleCount;
        }
        if (idleSampleCount < IDLE_SAMPLES) {
            MAIN_HANDLER.postDelayed(this::sampleIdleState, IDLE_INTERVAL_MS);
            return;
        }
        record("idle_60s", "none".equals(firstIdleFailure),
                "samples=" + idleSampleCount + " first_bad=" + firstIdleFailure
                        + " pending=" + R05PresentFenceProbe.failurePendingCount()
                        + " queue=" + R05PresentFenceProbe.failureCallbackQueueDepth()
                        + " active=" + R05PresentFenceProbe.failureCallbackActiveCount());
        finish();
    }

    private R05PresentFenceProbe.FailureFixtureRequest synthetic(
            String renderer, long generation, java.util.function.LongSupplier currentGeneration,
            java.util.function.BooleanSupplier surfaceAvailable) {
        return R05PresentFenceProbe.createSyntheticFailureRequest(renderer, generation,
                currentGeneration, surfaceAvailable, generation);
    }

    private void cancel(R05PresentFenceProbe.FailureFixtureRequest request,
                        String renderer, long generation) {
        R05PresentFenceProbe.cancelForFailureFixture(request, renderer, generation,
                "fixture_cleanup");
    }

    private boolean awaitExecutorIdle(long timeoutMs) throws InterruptedException {
        long deadline = SystemClock.elapsedRealtime() + timeoutMs;
        while (SystemClock.elapsedRealtime() < deadline) {
            if (R05PresentFenceProbe.failureCallbackQueueDepth() == 0
                    && R05PresentFenceProbe.failureCallbackActiveCount() == 0) return true;
            Thread.sleep(5);
        }
        return R05PresentFenceProbe.failureCallbackQueueDepth() == 0
                && R05PresentFenceProbe.failureCallbackActiveCount() == 0;
    }

    private void record(String scenario, boolean passed, String detail) {
        checks.incrementAndGet();
        if (!passed) failures.incrementAndGet();
        Log.i(TAG, "SPINON_R05_CALLBACK_FAULT scenario=" + scenario
                + " outcome=" + (passed ? "PASS" : "FAIL") + " " + detail);
    }

    private void finish() {
        String outcome = failures.get() == 0 ? "PASS" : "FAIL";
        String message = "R05 callback 실패 주입: " + outcome
                + "\n검증 항목: " + checks.get()
                + " · 경합: " + RACE_ITERATIONS + "회"
                + " · idle 관찰: " + idleSampleCount + "초";
        statusView.setText(message);
        Log.i(TAG, "SPINON_R05_CALLBACK_FAULT_SUMMARY status=" + outcome
                + " checks=" + checks.get() + " failures=" + failures.get()
                + " race_iterations=" + RACE_ITERATIONS
                + " idle_samples=" + idleSampleCount
                + " pending=" + R05PresentFenceProbe.failurePendingCount()
                + " queue=" + R05PresentFenceProbe.failureCallbackQueueDepth()
                + " active=" + R05PresentFenceProbe.failureCallbackActiveCount());
    }
}
