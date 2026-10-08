package dev.spinon.bootstrap;

import android.app.Activity;
import android.graphics.Color;
import android.os.Handler;
import android.os.Looper;
import android.os.SystemClock;
import android.util.Log;
import android.view.Gravity;
import android.view.InputDevice;
import android.view.MotionEvent;
import android.view.SurfaceHolder;
import android.view.View;
import android.view.ViewGroup;
import android.widget.FrameLayout;
import android.widget.TextView;

import java.util.concurrent.Callable;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicReference;
import java.util.function.BooleanSupplier;

/** 실제 R08 SurfaceView transaction callback의 표면 수명주기를 확인하는 debug fixture입니다. */
final class R05AppliedTransactionLifecycleFixture {
    private static final String TAG = "SpinonBootstrap";
    private static final String RENDERER = "opengl_es";
    private static final long WAIT_MS = 4_000;
    private static final long CALLBACK_QUEUE_WAIT_MS = 1_200;
    private static final Handler MAIN_HANDLER = new Handler(Looper.getMainLooper());

    private final Activity activity;
    private final TextView restoreView;
    private Harness harness;
    private boolean baselineRecorded;
    private boolean staleRecorded;
    private boolean recoveryRecorded;
    private boolean passed = true;

    private R05AppliedTransactionLifecycleFixture(Activity activity, TextView restoreView) {
        this.activity = activity;
        this.restoreView = restoreView;
    }

    static boolean run(Activity activity, TextView restoreView) {
        return new R05AppliedTransactionLifecycleFixture(activity, restoreView).run();
    }

    private boolean run() {
        try {
            if (!R05PresentFenceProbe.failureFixtureIdle()) {
                throw new IllegalStateException("callback_executor_not_idle");
            }
            harness = onMain(this::createHarness);
            if (!await(harness.lifecycle.nextCreated, WAIT_MS)
                    || !awaitSurfaceReady()) {
                throw new IllegalStateException("initial_surface_not_ready");
            }

            R05PresentFenceProbe.FailureFixtureRequest baselineRequest =
                    runPositiveCallback("actual_surface_callback_baseline");
            baselineRecorded = true;
            if (baselineRequest == null) {
                passed = false;
            }

            if (baselineRequest != null) {
                passed &= runCancelledCallbackAfterSurfaceRecreation();
                staleRecorded = true;
                passed &= runPositiveCallback("actual_surface_callback_recovery") != null;
                recoveryRecorded = true;
            }
        } catch (Exception | LinkageError | AssertionError error) {
            passed = false;
            Log.e(TAG, "SPINON_R05_APPLIED_TRANSACTION fixture=setup outcome=FAIL"
                    + " error=" + error.getClass().getSimpleName()
                    + " detail=" + String.valueOf(error.getMessage()));
        } finally {
            cleanupHarness();
        }

        if (!baselineRecorded) {
            record("actual_surface_callback_baseline", false, "not_run_after_setup_failure");
        }
        if (!staleRecorded) {
            record("actual_surface_callback_after_recreation", false,
                    "not_run_after_baseline_failure");
        }
        if (!recoveryRecorded) {
            record("actual_surface_callback_recovery", false,
                    "not_run_after_lifecycle_failure");
        }
        return passed;
    }

    private Harness createHarness() {
        FrameLayout root = new FrameLayout(activity);
        root.setBackgroundColor(Color.rgb(14, 19, 31));

        R08GpuSurface surface = new R08GpuSurface(activity, true, false, true);
        FrameLayout.LayoutParams surfaceParams = new FrameLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.MATCH_PARENT);
        root.addView(surface, surfaceParams);

        TextView status = new TextView(activity);
        status.setText("R05 실제 SurfaceView transaction callback fixture");
        status.setTextColor(Color.WHITE);
        status.setGravity(Gravity.CENTER_HORIZONTAL | Gravity.TOP);
        status.setPadding(16, 24, 16, 0);
        status.setClickable(false);
        root.addView(status, new FrameLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.WRAP_CONTENT,
                Gravity.TOP));

        LifecycleObserver lifecycle = new LifecycleObserver();
        surface.getHolder().addCallback(lifecycle);
        int[] activationCount = {0};
        surface.setOnClickListener(view -> surface.setActivationCount(++activationCount[0]));
        activity.setContentView(root);

        Log.i(TAG, "SPINON_R05_APPLIED_TRANSACTION fixture=created"
                + " view_identity=" + System.identityHashCode(surface)
                + " renderer=" + RENDERER + " input_source=synthetic_fixture");
        return new Harness(root, surface, surfaceParams, lifecycle);
    }

    private R05PresentFenceProbe.FailureFixtureRequest runPositiveCallback(String scenario) {
        BlockedSubmission submission = null;
        boolean callbackPassed = false;
        boolean recorded = false;
        try {
            submission = beginBlockedTap();
            if (!submission.queued) {
                record(scenario, false, submission.queueDetail);
                recorded = true;
                return null;
            }

            submission.release.countDown();
            callbackPassed = awaitCallback(submission.request, true,
                    submission.generation, true);
            boolean drained = awaitExecutorIdle(WAIT_MS);
            callbackPassed &= drained;
            record(scenario, callbackPassed, callbackDetail(submission.request)
                    + " executor_idle=" + drained);
            recorded = true;
            return callbackPassed ? submission.request : null;
        } catch (Exception | LinkageError | AssertionError error) {
            Log.e(TAG, "SPINON_R05_APPLIED_TRANSACTION scenario=" + scenario
                    + " outcome=FAIL error=" + error.getClass().getSimpleName());
            if (!recorded) {
                record(scenario, false, "error=" + error.getClass().getSimpleName());
            }
            return null;
        } finally {
            if (submission != null) submission.release.countDown();
            if (!callbackPassed && submission != null && submission.request != null) {
                R05PresentFenceProbe.cancelForFailureFixture(submission.request,
                        RENDERER, submission.generation, "fixture_positive_cleanup");
            }
        }
    }

    private boolean runCancelledCallbackAfterSurfaceRecreation() {
        BlockedSubmission submission = null;
        boolean scenarioPassed = false;
        boolean recorded = false;
        try {
            submission = beginBlockedTap();
            if (submission == null || !submission.queued) {
                record("actual_surface_callback_after_recreation", false,
                        submission == null ? "request_not_created" : submission.queueDetail);
                recorded = true;
                return false;
            }

            long oldGeneration = submission.generation;
            BlockedSubmission blocked = submission;
            long oldViewIdentity = System.identityHashCode(harness.surface);
            harness.lifecycle.armDestroyed();
            onMain(() -> {
                ViewGroup parent = (ViewGroup) harness.surface.getParent();
                if (parent == null) throw new IllegalStateException("surface_parent_missing");
                parent.removeView(harness.surface);
                return null;
            });

            boolean destroyed = await(harness.lifecycle.nextDestroyed, WAIT_MS);
            boolean cancelled = await(() ->
                    "cancelled".equals(R05PresentFenceProbe.failureState(blocked.request))
                            && !R05PresentFenceProbe.failureRequestPending(blocked.request)
                            && !R05PresentFenceProbe.failureTimeoutScheduled(blocked.request),
                    CALLBACK_QUEUE_WAIT_MS);
            boolean noTimeoutWon = R05PresentFenceProbe.failureTimeoutCount(blocked.request) == 0;

            harness.lifecycle.armCreated();
            onMain(() -> {
                ViewGroup parent = harness.root;
                if (harness.surface.getParent() != null) {
                    throw new IllegalStateException("surface_still_attached_before_recreate");
                }
                parent.addView(harness.surface, harness.surfaceParams);
                return null;
            });

            boolean recreated = await(harness.lifecycle.nextCreated, WAIT_MS)
                    && awaitSurfaceReady();
            long newGeneration = harness.surface.r05SurfaceGenerationForFailureFixture();
            boolean sameView = oldViewIdentity == System.identityHashCode(harness.surface)
                    && onMain(() -> harness.surface.getParent() == harness.root
                    && harness.root.indexOfChild(harness.surface) >= 0);
            boolean generationAdvanced = newGeneration > oldGeneration;

            submission.release.countDown();
            boolean lateCallback = await(() ->
                    R05PresentFenceProbe.failureCallbackCount(blocked.request) == 1
                            && R05PresentFenceProbe.failureLastCallbackObservation(
                                    blocked.request) != null,
                    WAIT_MS);
            boolean drained = awaitExecutorIdle(WAIT_MS);
            R05PresentFenceProbe.CallbackObservation observation =
                    R05PresentFenceProbe.failureLastCallbackObservation(blocked.request);
            boolean correctlyStale = observation != null
                    && "late_after_cancel".equals(observation.outcome)
                    && observation.transactionStatsAvailable
                    && !observation.currentSurface
                    && !observation.fenceSignalUsable
                    && !observation.callbackInlineOverflow
                    && "cancelled".equals(R05PresentFenceProbe.failureState(blocked.request))
                    && !R05PresentFenceProbe.failureRequestPending(blocked.request)
                    && !R05PresentFenceProbe.failureTimeoutScheduled(blocked.request)
                    && R05PresentFenceProbe.failureTimeoutCount(blocked.request) == 0;
            scenarioPassed = destroyed && cancelled && noTimeoutWon && recreated
                    && sameView && generationAdvanced && lateCallback && drained
                    && correctlyStale && R05PresentFenceProbe.failureFixtureIdle();
            record("actual_surface_callback_after_recreation", scenarioPassed,
                    "request_id=" + submission.request.requestId()
                            + " input_seq=" + submission.inputSequence
                            + " old_generation=" + oldGeneration
                            + " new_generation=" + newGeneration
                            + " surface_destroyed=" + destroyed
                            + " cancelled_before_timeout=" + (cancelled && noTimeoutWon)
                            + " same_view=" + sameView
                            + " late_callback=" + lateCallback
                            + " " + observationDetail(observation)
                            + " executor_idle=" + drained);
            recorded = true;
        } catch (Exception | LinkageError | AssertionError error) {
            Log.e(TAG, "SPINON_R05_APPLIED_TRANSACTION"
                    + " scenario=actual_surface_callback_after_recreation outcome=FAIL"
                    + " error=" + error.getClass().getSimpleName()
                    + " detail=" + String.valueOf(error.getMessage()));
            if (!recorded) {
                record("actual_surface_callback_after_recreation", false,
                        "error=" + error.getClass().getSimpleName());
            }
            scenarioPassed = false;
        } finally {
            if (submission != null) {
                submission.release.countDown();
                if (submission.request != null) {
                    R05PresentFenceProbe.cancelForFailureFixture(submission.request,
                            RENDERER, submission.generation,
                            "fixture_surface_recreation_cleanup");
                }
            }
        }
        return scenarioPassed;
    }

    private BlockedSubmission beginBlockedTap() throws Exception {
        if (!R05PresentFenceProbe.failureFixtureIdle()) {
            throw new IllegalStateException("callback_executor_not_idle_before_tap");
        }
        CountDownLatch blockerStarted = new CountDownLatch(1);
        CountDownLatch release = new CountDownLatch(1);
        R05PresentFenceProbe.executeCallbackForFailureFixture(() -> {
            blockerStarted.countDown();
            try {
                release.await(WAIT_MS, TimeUnit.MILLISECONDS);
            } catch (InterruptedException error) {
                Thread.currentThread().interrupt();
            }
        });
        if (!await(blockerStarted, WAIT_MS)) {
            release.countDown();
            throw new IllegalStateException("callback_executor_blocker_not_started");
        }

        try {
            long generation = harness.surface.r05SurfaceGenerationForFailureFixture();
            long callbackBaseline = R05PresentFenceProbe.failureCallbackEventTotal();
            long inputSequence = onMain(() -> dispatchSyntheticTap(harness.surface));
            R05PresentFenceProbe.FailureFixtureRequest request = awaitPendingRequest(
                    generation, inputSequence, CALLBACK_QUEUE_WAIT_MS);
            boolean queued = request != null
                    && R05PresentFenceProbe.failureCallbackQueueDepth() == 1
                    && R05PresentFenceProbe.failureCallbackActiveCount() == 1
                    && R05PresentFenceProbe.failureCallbackCount(request) == 0
                    && R05PresentFenceProbe.failureRequestPending(request)
                    && R05PresentFenceProbe.failureTimeoutScheduled(request)
                    && "pending".equals(R05PresentFenceProbe.failureState(request))
                    && R05PresentFenceProbe.failureCallbackEventTotal() == callbackBaseline;
            String detail = "input_source=synthetic_fixture input_seq=" + inputSequence
                    + " generation=" + generation + " queue="
                    + R05PresentFenceProbe.failureCallbackQueueDepth()
                    + " active=" + R05PresentFenceProbe.failureCallbackActiveCount()
                    + " callback_count=" + R05PresentFenceProbe.failureCallbackCount(request)
                    + " request_pending=" + (request != null
                    && R05PresentFenceProbe.failureRequestPending(request));
            Log.i(TAG, "SPINON_R05_APPLIED_TRANSACTION scenario=callback_queued"
                    + " outcome=" + (queued ? "PASS" : "FAIL") + " " + detail);
            return new BlockedSubmission(request, release, generation, inputSequence,
                    queued, detail);
        } catch (Exception | LinkageError | AssertionError error) {
            release.countDown();
            throw error;
        }
    }

    private long dispatchSyntheticTap(R08GpuSurface surface) {
        if (Looper.myLooper() != Looper.getMainLooper()) {
            throw new IllegalStateException("synthetic_tap_not_on_main_thread");
        }
        if (!surface.isAttachedToWindow() || surface.getWidth() <= 0 || surface.getHeight() <= 0
                || !surface.r05SurfaceAvailableForFailureFixture()) {
            throw new IllegalStateException("surface_not_dispatchable");
        }
        long upTime = SystemClock.uptimeMillis();
        long downTime = Math.max(0, upTime - 8);
        float x = surface.getWidth() / 2.0f;
        float y = surface.getHeight() / 2.0f;
        MotionEvent down = null;
        MotionEvent up = null;
        try {
            down = MotionEvent.obtain(downTime, downTime,
                    MotionEvent.ACTION_DOWN, x, y, 0);
            down.setSource(InputDevice.SOURCE_TOUCHSCREEN);
            up = MotionEvent.obtain(downTime, upTime,
                    MotionEvent.ACTION_UP, x, y, 0);
            up.setSource(InputDevice.SOURCE_TOUCHSCREEN);
            surface.dispatchTouchEvent(down);
            surface.dispatchTouchEvent(up);
        } finally {
            if (down != null) down.recycle();
            if (up != null) up.recycle();
        }
        return surface.r05InputSequenceForFailureFixture();
    }

    private R05PresentFenceProbe.FailureFixtureRequest awaitPendingRequest(
            long generation, long inputSequence, long timeoutMs) throws InterruptedException {
        final R05PresentFenceProbe.FailureFixtureRequest[] result = {null};
        await(() -> {
            result[0] = R05PresentFenceProbe.findPendingForFailureFixture(
                    RENDERER, generation, inputSequence);
            return result[0] != null
                    && R05PresentFenceProbe.failureCallbackQueueDepth() == 1;
        }, timeoutMs);
        return result[0];
    }

    private boolean awaitCallback(R05PresentFenceProbe.FailureFixtureRequest request,
                                  boolean expectedCurrentSurface,
                                  long generation, boolean expectedStatsAvailable)
            throws InterruptedException {
        boolean callback = await(() ->
                R05PresentFenceProbe.failureCallbackCount(request) == 1
                        && R05PresentFenceProbe.failureLastCallbackObservation(request) != null,
                WAIT_MS);
        R05PresentFenceProbe.CallbackObservation observation =
                R05PresentFenceProbe.failureLastCallbackObservation(request);
        return callback && observation != null
                && "callback_received".equals(observation.outcome)
                && observation.currentSurface == expectedCurrentSurface
                && observation.transactionStatsAvailable == expectedStatsAvailable
                && !observation.callbackInlineOverflow
                && R05PresentFenceProbe.failureState(request).equals("callback_received")
                && !R05PresentFenceProbe.failureRequestPending(request)
                && !R05PresentFenceProbe.failureTimeoutScheduled(request)
                && R05PresentFenceProbe.failureTimeoutCount(request) == 0
                && request.generation() == generation;
    }

    private boolean awaitSurfaceReady() throws InterruptedException {
        return await(() -> harness.surface.isAttachedToWindow()
                        && harness.surface.getWidth() > 0
                        && harness.surface.getHeight() > 0
                        && harness.surface.r05SurfaceAvailableForFailureFixture()
                        && harness.surface.getHolder().getSurface().isValid()
                        && harness.surface.r05SurfaceGenerationForFailureFixture() > 0,
                WAIT_MS);
    }

    private boolean awaitExecutorIdle(long timeoutMs) throws InterruptedException {
        return await(R05PresentFenceProbe::failureFixtureIdle, timeoutMs);
    }

    private static boolean await(CountDownLatch latch, long timeoutMs) throws InterruptedException {
        return latch.await(timeoutMs, TimeUnit.MILLISECONDS);
    }

    private static boolean await(BooleanSupplier condition, long timeoutMs)
            throws InterruptedException {
        long deadline = SystemClock.elapsedRealtime() + timeoutMs;
        while (SystemClock.elapsedRealtime() < deadline) {
            if (condition.getAsBoolean()) return true;
            Thread.sleep(5);
        }
        return condition.getAsBoolean();
    }

    private <T> T onMain(Callable<T> action) throws Exception {
        if (Looper.myLooper() == Looper.getMainLooper()) return action.call();
        AtomicReference<T> result = new AtomicReference<>();
        AtomicReference<Throwable> failure = new AtomicReference<>();
        CountDownLatch complete = new CountDownLatch(1);
        if (!MAIN_HANDLER.post(() -> {
            try {
                result.set(action.call());
            } catch (Throwable error) {
                failure.set(error);
            } finally {
                complete.countDown();
            }
        })) {
            throw new IllegalStateException("main_handler_rejected_fixture_action");
        }
        if (!complete.await(WAIT_MS, TimeUnit.MILLISECONDS)) {
            throw new IllegalStateException("main_thread_fixture_action_timeout");
        }
        Throwable error = failure.get();
        if (error instanceof Exception) throw (Exception) error;
        if (error != null) throw new IllegalStateException("main_thread_fixture_action_failed", error);
        return result.get();
    }

    private void cleanupHarness() {
        if (harness == null) return;
        try {
            onMain(() -> {
                R05PresentFenceProbe.cancelSurfaceForFailureFixture(RENDERER,
                        harness.surface.r05SurfaceGenerationForFailureFixture(),
                        "fixture_cleanup");
                harness.surface.getHolder().removeCallback(harness.lifecycle);
                activity.setContentView(restoreView);
                return null;
            });
            if (!awaitExecutorIdle(WAIT_MS)) {
                passed = false;
                Log.e(TAG, "SPINON_R05_APPLIED_TRANSACTION fixture=cleanup outcome=FAIL"
                        + " reason=callback_executor_not_drained");
            }
        } catch (Exception | LinkageError | AssertionError error) {
            passed = false;
            Log.e(TAG, "SPINON_R05_APPLIED_TRANSACTION fixture=cleanup outcome=FAIL"
                    + " error=" + error.getClass().getSimpleName());
        }
    }

    private void record(String scenario, boolean scenarioPassed, String detail) {
        if (!scenarioPassed) passed = false;
        Log.i(TAG, "SPINON_R05_APPLIED_TRANSACTION scenario=" + scenario
                + " outcome=" + (scenarioPassed ? "PASS" : "FAIL") + " " + detail);
    }

    private static String callbackDetail(
            R05PresentFenceProbe.FailureFixtureRequest request) {
        return "request_id=" + request.requestId()
                + " input_seq=" + request.inputSequence()
                + " generation=" + request.generation()
                + " callback_count=" + R05PresentFenceProbe.failureCallbackCount(request)
                + " " + observationDetail(
                        R05PresentFenceProbe.failureLastCallbackObservation(request));
    }

    private static String observationDetail(R05PresentFenceProbe.CallbackObservation observation) {
        if (observation == null) return "observation=missing";
        return "callback_outcome=" + observation.outcome
                + " stats_available=" + observation.transactionStatsAvailable
                + " current_surface=" + observation.currentSurface
                + " fence_signal_usable=" + observation.fenceSignalUsable
                + " callback_inline_overflow=" + observation.callbackInlineOverflow
                + " fence_state=" + observation.fenceState;
    }

    private static final class Harness {
        final FrameLayout root;
        final R08GpuSurface surface;
        final FrameLayout.LayoutParams surfaceParams;
        final LifecycleObserver lifecycle;

        Harness(FrameLayout root, R08GpuSurface surface,
                FrameLayout.LayoutParams surfaceParams, LifecycleObserver lifecycle) {
            this.root = root;
            this.surface = surface;
            this.surfaceParams = surfaceParams;
            this.lifecycle = lifecycle;
        }
    }

    private static final class BlockedSubmission {
        final R05PresentFenceProbe.FailureFixtureRequest request;
        final CountDownLatch release;
        final long generation;
        final long inputSequence;
        final boolean queued;
        final String queueDetail;

        BlockedSubmission(R05PresentFenceProbe.FailureFixtureRequest request,
                          CountDownLatch release, long generation, long inputSequence,
                          boolean queued, String queueDetail) {
            this.request = request;
            this.release = release;
            this.generation = generation;
            this.inputSequence = inputSequence;
            this.queued = queued;
            this.queueDetail = queueDetail;
        }
    }

    private static final class LifecycleObserver implements SurfaceHolder.Callback {
        volatile CountDownLatch nextCreated = new CountDownLatch(1);
        volatile CountDownLatch nextDestroyed = new CountDownLatch(0);

        void armCreated() {
            nextCreated = new CountDownLatch(1);
        }

        void armDestroyed() {
            nextDestroyed = new CountDownLatch(1);
        }

        @Override
        public void surfaceCreated(SurfaceHolder holder) {
            Log.i(TAG, "SPINON_R05_APPLIED_TRANSACTION surface=created");
            nextCreated.countDown();
        }

        @Override
        public void surfaceChanged(SurfaceHolder holder, int format, int width, int height) {}

        @Override
        public void surfaceDestroyed(SurfaceHolder holder) {
            Log.i(TAG, "SPINON_R05_APPLIED_TRANSACTION surface=destroyed");
            nextDestroyed.countDown();
        }
    }
}
