package dev.spinon.bootstrap;

import android.os.Build;
import android.os.Handler;
import android.os.Looper;
import android.os.Process;
import android.os.SystemClock;
import android.util.Log;
import android.view.SurfaceControl;
import android.view.SurfaceView;

import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.Executor;
import java.util.concurrent.RejectedExecutionHandler;
import java.util.concurrent.ThreadFactory;
import java.util.concurrent.ThreadPoolExecutor;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;
import java.util.function.BooleanSupplier;
import java.util.function.LongSupplier;

/** Android API 35 이상 SurfaceView transaction 표시 fence를 조사합니다. */
final class R05PresentFenceProbe {
    private static final String TAG = "SpinonBootstrap";
    private static final int MIN_API = 35;
    private static final int MAX_PENDING = 64;
    private static final long CALLBACK_TIMEOUT_MS = 2_000;
    private static final AtomicLong NEXT_REQUEST_ID = new AtomicLong(1);
    private static final AtomicLong INLINE_CALLBACK_COUNT = new AtomicLong();
    private static final AtomicLong CALLBACK_EVENT_TOTAL = new AtomicLong();
    private static final AtomicLong TIMEOUT_EVENT_TOTAL = new AtomicLong();
    private static final Object PENDING_LOCK = new Object();
    private static final LinkedHashMap<Long, Request> PENDING_REQUESTS = new LinkedHashMap<>();
    private static final Handler TIMEOUT_HANDLER = new Handler(Looper.getMainLooper());
    private static final ThreadLocal<Boolean> CALLBACK_RAN_INLINE =
            ThreadLocal.withInitial(() -> false);
    private static final ThreadPoolExecutor CALLBACK_POOL = new ThreadPoolExecutor(
            1, 1, 0L, TimeUnit.MILLISECONDS,
            new ArrayBlockingQueue<>(MAX_PENDING), new ProbeThreadFactory(),
            new InlineOnRejectHandler());
    private static final Executor CALLBACK_EXECUTOR = command ->
            CALLBACK_POOL.execute(new CallbackTask(command));

    private R05PresentFenceProbe() {}

    static final class FailureFixtureRequest {
        private final Request request;
        private final SurfaceControl.Transaction transaction;
        private final AtomicBoolean transactionClosed = new AtomicBoolean();

        FailureFixtureRequest(Request request, SurfaceControl.Transaction transaction) {
            this.request = request;
            this.transaction = transaction;
        }

        long requestId() {
            return request.requestId;
        }

        long generation() {
            return request.generation;
        }

        long inputSequence() {
            return request.inputSequence;
        }
    }

    static final class CallbackObservation {
        final String outcome;
        final String fenceState;
        final boolean currentSurface;
        final boolean transactionStatsAvailable;
        final boolean fenceSignalUsable;
        final boolean callbackInlineOverflow;

        CallbackObservation(String outcome, String fenceState, boolean currentSurface,
                            boolean transactionStatsAvailable, boolean fenceSignalUsable,
                            boolean callbackInlineOverflow) {
            this.outcome = outcome;
            this.fenceState = fenceState;
            this.currentSurface = currentSurface;
            this.transactionStatsAvailable = transactionStatsAvailable;
            this.fenceSignalUsable = fenceSignalUsable;
            this.callbackInlineOverflow = callbackInlineOverflow;
        }
    }

    static boolean failureFixtureIdle() {
        synchronized (PENDING_LOCK) {
            return PENDING_REQUESTS.isEmpty()
                    && CALLBACK_POOL.getActiveCount() == 0
                    && CALLBACK_POOL.getQueue().isEmpty();
        }
    }

    static FailureFixtureRequest attachUnappliedForFailureFixture(String renderer,
                                                                   long inputSequence) {
        if (Build.VERSION.SDK_INT < MIN_API) return null;
        long requestId = nextRequestId();
        if (requestId < 0) return null;

        SurfaceControl.Transaction transaction;
        try {
            transaction = new SurfaceControl.Transaction();
        } catch (RuntimeException | LinkageError error) {
            Log.e(TAG, "SPINON_R05_CALLBACK_FAULT scenario=unapplied_timeout"
                    + " outcome=transaction_create_failed"
                    + " error=" + error.getClass().getSimpleName());
            return null;
        }

        long generation = inputSequence;
        long monotonicBeforeNanos = System.nanoTime();
        long uptimeAnchorNanos = SystemClock.uptimeNanos();
        long monotonicAfterNanos = System.nanoTime();
        boolean attached = Api35.attach(transaction, requestId, renderer, generation,
                () -> generation, () -> true, inputSequence, inputSequence,
                uptimeAnchorNanos, uptimeAnchorNanos,
                monotonicBeforeNanos, monotonicAfterNanos, -1L);
        Request request;
        synchronized (PENDING_LOCK) {
            request = PENDING_REQUESTS.get(requestId);
        }
        if (!attached || request == null) {
            Api35.closeTransaction(transaction, renderer, generation, requestId);
            return null;
        }
        Log.i(TAG, "SPINON_R05_CALLBACK_FAULT scenario=unapplied_timeout"
                + " request_id=" + requestId + " source=real_transaction"
                + " transaction_applied=false");
        return new FailureFixtureRequest(request, transaction);
    }

    static FailureFixtureRequest createSyntheticFailureRequest(String renderer,
                                                                long generation,
                                                                LongSupplier currentGeneration,
                                                                BooleanSupplier surfaceAvailable,
                                                                long inputSequence) {
        long requestId = nextRequestId();
        if (requestId < 0) return null;
        long monotonicBeforeNanos = System.nanoTime();
        long uptimeAnchorNanos = SystemClock.uptimeNanos();
        long monotonicAfterNanos = System.nanoTime();
        Request request = new Request(requestId, renderer, generation,
                currentGeneration, surfaceAvailable, inputSequence, inputSequence,
                uptimeAnchorNanos, uptimeAnchorNanos,
                monotonicBeforeNanos, monotonicAfterNanos, -1L);
        synchronized (PENDING_LOCK) {
            if (PENDING_REQUESTS.size() >= MAX_PENDING
                    || PENDING_REQUESTS.containsKey(requestId)) {
                Log.e(TAG, "SPINON_R05_CALLBACK_FAULT scenario=synthetic_request"
                        + " outcome=" + (PENDING_REQUESTS.containsKey(requestId)
                        ? "duplicate_request_id" : "pending_limit"));
                return null;
            }
            PENDING_REQUESTS.put(requestId, request);
        }
        return new FailureFixtureRequest(request, null);
    }

    static void timeoutForFailureFixture(FailureFixtureRequest fixtureRequest) {
        Api35.timeout(fixtureRequest.request);
    }

    static CallbackObservation callbackForFailureFixture(FailureFixtureRequest fixtureRequest) {
        return Api35.handleCallback(fixtureRequest.request, null);
    }

    static void cancelForFailureFixture(FailureFixtureRequest fixtureRequest,
                                        String renderer, long generation, String reason) {
        Api35.cancel(fixtureRequest.request.requestId, renderer, generation, reason);
    }

    static void cancelSurfaceForFailureFixture(String renderer, long generation,
                                               String reason) {
        Api35.cancelForSurface(renderer, generation, reason);
    }

    static String failureState(FailureFixtureRequest fixtureRequest) {
        int state = fixtureRequest.request.state.get();
        if (state == Api35.PENDING) return "pending";
        if (state == Api35.CALLBACK_RECEIVED) return "callback_received";
        if (state == Api35.TIMED_OUT) return "timed_out";
        if (state == Api35.CANCELLED) return "cancelled";
        return "unknown";
    }

    static boolean failureRequestPending(FailureFixtureRequest fixtureRequest) {
        synchronized (PENDING_LOCK) {
            return PENDING_REQUESTS.get(fixtureRequest.request.requestId)
                    == fixtureRequest.request;
        }
    }

    static boolean failureTimeoutScheduled(FailureFixtureRequest fixtureRequest) {
        Runnable timeout = fixtureRequest.request.timeout;
        return timeout != null && TIMEOUT_HANDLER.hasCallbacks(timeout);
    }

    static int failurePendingCount() {
        synchronized (PENDING_LOCK) {
            return PENDING_REQUESTS.size();
        }
    }

    static FailureFixtureRequest findPendingForFailureFixture(String renderer,
                                                               long generation,
                                                               long inputSequence) {
        synchronized (PENDING_LOCK) {
            for (Request request : PENDING_REQUESTS.values()) {
                if (request.renderer.equals(renderer)
                        && request.generation == generation
                        && request.inputSequence == inputSequence) {
                    return new FailureFixtureRequest(request, null);
                }
            }
        }
        return null;
    }

    static int failureCallbackQueueDepth() {
        return CALLBACK_POOL.getQueue().size();
    }

    static int failureCallbackActiveCount() {
        return CALLBACK_POOL.getActiveCount();
    }

    static long failureInlineCallbackTotal() {
        return INLINE_CALLBACK_COUNT.get();
    }

    static long failureCallbackEventTotal() {
        return CALLBACK_EVENT_TOTAL.get();
    }

    static long failureTimeoutEventTotal() {
        return TIMEOUT_EVENT_TOTAL.get();
    }

    static int failureCallbackCount(FailureFixtureRequest fixtureRequest) {
        return fixtureRequest == null ? -1 : fixtureRequest.request.callbackCount.get();
    }

    static int failureTimeoutCount(FailureFixtureRequest fixtureRequest) {
        return fixtureRequest == null ? -1 : fixtureRequest.request.timeoutCount.get();
    }

    static CallbackObservation failureLastCallbackObservation(
            FailureFixtureRequest fixtureRequest) {
        return fixtureRequest == null ? null : fixtureRequest.request.lastCallbackObservation;
    }

    static boolean closeFailureTransaction(FailureFixtureRequest fixtureRequest) {
        if (fixtureRequest.transaction == null
                || !fixtureRequest.transactionClosed.compareAndSet(false, true)) return false;
        return Api35.closeTransaction(fixtureRequest.transaction,
                fixtureRequest.request.renderer, fixtureRequest.request.generation,
                fixtureRequest.request.requestId);
    }

    static void executeCallbackForFailureFixture(Runnable command) {
        CALLBACK_EXECUTOR.execute(command);
    }

    static void submitNextFrame(SurfaceView surface, String renderer, long generation,
                                LongSupplier currentGeneration,
                                BooleanSupplier surfaceAvailable, long inputSequence,
                                long revision, long eventTimeNanos,
                                long inputUptimeAnchorNanos,
                                long inputMonotonicBeforeNanos,
                                long inputMonotonicAfterNanos, Runnable submit) {
        if (Build.VERSION.SDK_INT < MIN_API) {
            logUnavailable(renderer, generation, inputSequence, revision, "api_below_35");
            submit.run();
            return;
        }
        if (Looper.myLooper() != Looper.getMainLooper()) {
            logUnavailable(renderer, generation, inputSequence, revision, "not_main_thread");
            submit.run();
            return;
        }
        if (!isCurrentSurface(surface, generation, currentGeneration, surfaceAvailable)) {
            logUnavailable(renderer, generation, inputSequence, revision,
                    "stale_surface_before_apply");
            submit.run();
            return;
        }
        Api35.submitNextFrame(surface, renderer, generation, currentGeneration,
                surfaceAvailable, inputSequence, revision, eventTimeNanos,
                inputUptimeAnchorNanos, inputMonotonicBeforeNanos,
                inputMonotonicAfterNanos, submit);
    }

    static boolean attach(SurfaceControl.Transaction transaction, long requestId,
                          String renderer, long generation,
                          LongSupplier currentGeneration,
                          BooleanSupplier surfaceAvailable, long inputSequence,
                          long revision, long eventTimeNanos,
                          long inputUptimeAnchorNanos,
                          long inputMonotonicBeforeNanos,
                          long inputMonotonicAfterNanos, long targetVsyncId) {
        if (Build.VERSION.SDK_INT < MIN_API) return false;
        return Api35.attach(transaction, requestId, renderer, generation,
                currentGeneration, surfaceAvailable, inputSequence, revision, eventTimeNanos,
                inputUptimeAnchorNanos, inputMonotonicBeforeNanos,
                inputMonotonicAfterNanos, targetVsyncId);
    }

    static void cancel(long requestId, String renderer, long generation, String reason) {
        if (Build.VERSION.SDK_INT < MIN_API) return;
        Api35.cancel(requestId, renderer, generation, reason);
    }

    static void cancelForSurface(String renderer, long generation, String reason) {
        if (Build.VERSION.SDK_INT < MIN_API) return;
        Api35.cancelForSurface(renderer, generation, reason);
    }

    private static boolean isCurrentSurface(SurfaceView surface, long generation,
                                            LongSupplier currentGeneration,
                                            BooleanSupplier surfaceAvailable) {
        try {
            return surface != null && surface.isAttachedToWindow()
                    && surface.getHolder().getSurface().isValid()
                    && surfaceAvailable.getAsBoolean()
                    && currentGeneration.getAsLong() == generation;
        } catch (RuntimeException | LinkageError ignored) {
            return false;
        }
    }

    private static long nextRequestId() {
        while (true) {
            long current = NEXT_REQUEST_ID.get();
            if (current <= 0 || current == Long.MAX_VALUE) return -1;
            if (NEXT_REQUEST_ID.compareAndSet(current, current + 1)) return current;
        }
    }

    private static void logUnavailable(String renderer, long generation, long inputSequence,
                                       long revision, String reason) {
        Log.w(TAG, "SPINON_R05_PRESENT_FENCE renderer=" + renderer
                + " input_seq=" + inputSequence + " revision=" + revision
                + " generation=" + generation + " outcome=unavailable reason=" + reason);
    }

    /** API 35 transaction stats와 SyncFence 참조를 runtime API 경계 안에 둡니다. */
    private static final class Api35 {
        private static final int PENDING = 0;
        private static final int CALLBACK_RECEIVED = 1;
        private static final int TIMED_OUT = 2;
        private static final int CANCELLED = 3;

        private Api35() {}

        static void submitNextFrame(SurfaceView surface, String renderer, long generation,
                                   LongSupplier currentGeneration,
                                   BooleanSupplier surfaceAvailable, long inputSequence,
                                   long revision, long eventTimeNanos,
                                   long inputUptimeAnchorNanos,
                                   long inputMonotonicBeforeNanos,
                                   long inputMonotonicAfterNanos, Runnable submit) {
            long requestId = nextRequestId();
            if (requestId < 0) {
                logUnavailable(renderer, generation, inputSequence, revision,
                        "request_id_exhausted");
                submit.run();
                return;
            }

            SurfaceControl.Transaction transaction;
            try {
                transaction = new SurfaceControl.Transaction();
            } catch (RuntimeException | LinkageError error) {
                logUnavailable(renderer, generation, inputSequence, revision,
                        "transaction_create_failed_" + error.getClass().getSimpleName());
                submit.run();
                return;
            }

            boolean listenerAttached = attach(transaction, requestId, renderer, generation,
                    currentGeneration, surfaceAvailable, inputSequence, revision,
                    eventTimeNanos, inputUptimeAnchorNanos, inputMonotonicBeforeNanos,
                    inputMonotonicAfterNanos, -1L);
            boolean transactionHandedToSurfaceView = false;
            try {
                surface.applyTransactionToFrame(transaction);
                transactionHandedToSurfaceView = true;
                Log.i(TAG, "SPINON_R05_PRESENT_FENCE_APPLY renderer=" + renderer
                        + " request_id=" + requestId + " input_seq=" + inputSequence
                        + " revision=" + revision + " generation=" + generation
                        + " target_vsync_id=unavailable transaction=queued_for_next_surface_frame"
                        + " listener_attached=" + listenerAttached);
            } catch (RuntimeException | LinkageError error) {
                if (listenerAttached) {
                    cancel(requestId, renderer, generation,
                            "apply_failed_" + error.getClass().getSimpleName());
                }
                if (!transactionHandedToSurfaceView) closeTransaction(transaction, renderer,
                        generation, requestId);
                logUnavailable(renderer, generation, inputSequence, revision,
                        "apply_failed_" + error.getClass().getSimpleName());
            }
            submit.run();
        }

        static boolean attach(SurfaceControl.Transaction transaction, long requestId,
                              String renderer, long generation,
                              LongSupplier currentGeneration,
                              BooleanSupplier surfaceAvailable, long inputSequence,
                              long revision, long eventTimeNanos,
                              long inputUptimeAnchorNanos,
                              long inputMonotonicBeforeNanos,
                              long inputMonotonicAfterNanos, long targetVsyncId) {
            if (transaction == null || requestId <= 0) {
                logUnavailable(renderer, generation, inputSequence, revision,
                        "invalid_transaction_or_request_id");
                return false;
            }
            Request request = new Request(requestId, renderer, generation, currentGeneration,
                    surfaceAvailable, inputSequence, revision, eventTimeNanos,
                    inputUptimeAnchorNanos, inputMonotonicBeforeNanos,
                    inputMonotonicAfterNanos, targetVsyncId);
            synchronized (PENDING_LOCK) {
                if (PENDING_REQUESTS.size() >= MAX_PENDING
                        || PENDING_REQUESTS.containsKey(requestId)) {
                    logUnavailable(renderer, generation, inputSequence, revision,
                            PENDING_REQUESTS.containsKey(requestId)
                                    ? "duplicate_request_id" : "pending_limit");
                    return false;
                }
                PENDING_REQUESTS.put(requestId, request);
            }

            request.timeout = () -> timeout(request);
            if (!TIMEOUT_HANDLER.postDelayed(request.timeout, CALLBACK_TIMEOUT_MS)) {
                request.state.set(CANCELLED);
                removePending(request);
                Log.w(TAG, "SPINON_R05_PRESENT_FENCE renderer=" + renderer
                        + " request_id=" + requestId + " input_seq=" + inputSequence
                        + " revision=" + revision + " generation=" + generation
                        + " outcome=timeout_schedule_failed");
                return false;
            }

            try {
                transaction.addTransactionCompletedListener(CALLBACK_EXECUTOR,
                        stats -> safelyHandleCallback(request, stats));
            } catch (RuntimeException | LinkageError error) {
                removePending(request);
                request.state.set(CANCELLED);
                TIMEOUT_HANDLER.removeCallbacks(request.timeout);
                Log.w(TAG, "SPINON_R05_PRESENT_FENCE renderer=" + renderer
                        + " request_id=" + requestId + " input_seq=" + inputSequence
                        + " revision=" + revision + " generation=" + generation
                        + " target_vsync_id=" + targetVsyncId
                        + " outcome=listener_registration_failed"
                        + " error=" + error.getClass().getSimpleName());
                return false;
            }

            Log.i(TAG, "SPINON_R05_PRESENT_FENCE_LISTENER renderer=" + renderer
                    + " request_id=" + requestId + " input_seq=" + inputSequence
                    + " revision=" + revision + " generation=" + generation
                    + " target_vsync_id=" + targetVsyncId + " state=registered"
                    + " timeout_ms=" + CALLBACK_TIMEOUT_MS);
            return true;
        }

        static void cancel(long requestId, String renderer, long generation, String reason) {
            Request request;
            synchronized (PENDING_LOCK) {
                request = PENDING_REQUESTS.get(requestId);
            }
            if (request == null) return;
            if (!request.renderer.equals(renderer) || request.generation != generation) {
                Log.w(TAG, "SPINON_R05_PRESENT_FENCE renderer=" + renderer
                        + " request_id=" + requestId + " generation=" + generation
                        + " outcome=cancel_owner_mismatch");
                return;
            }
            if (!request.state.compareAndSet(PENDING, CANCELLED)) return;
            removePending(request);
            if (request.timeout != null) TIMEOUT_HANDLER.removeCallbacks(request.timeout);
            Log.i(TAG, "SPINON_R05_PRESENT_FENCE renderer=" + renderer
                    + " request_id=" + requestId + " input_seq=" + request.inputSequence
                    + " revision=" + request.revision + " generation=" + generation
                    + " outcome=cancelled reason=" + reason);
        }

        static void cancelForSurface(String renderer, long generation, String reason) {
            ArrayList<Request> cancelled = new ArrayList<>();
            synchronized (PENDING_LOCK) {
                for (Request request : PENDING_REQUESTS.values()) {
                    if (request.renderer.equals(renderer) && request.generation == generation
                            && request.state.compareAndSet(PENDING, CANCELLED)) {
                        cancelled.add(request);
                    }
                }
                for (Request request : cancelled) {
                    PENDING_REQUESTS.remove(request.requestId);
                }
            }
            for (Request request : cancelled) {
                if (request.timeout != null) TIMEOUT_HANDLER.removeCallbacks(request.timeout);
                Log.i(TAG, "SPINON_R05_PRESENT_FENCE renderer=" + renderer
                        + " request_id=" + request.requestId
                        + " input_seq=" + request.inputSequence
                        + " revision=" + request.revision + " generation=" + generation
                        + " outcome=cancelled reason=" + reason);
            }
        }

        private static void timeout(Request request) {
            if (!request.state.compareAndSet(PENDING, TIMED_OUT)) return;
            request.timeoutCount.incrementAndGet();
            TIMEOUT_EVENT_TOTAL.incrementAndGet();
            removePending(request);
            Log.w(TAG, "SPINON_R05_PRESENT_FENCE renderer=" + request.renderer
                    + " request_id=" + request.requestId
                    + " input_seq=" + request.inputSequence
                    + " revision=" + request.revision
                    + " generation=" + request.generation
                    + " target_vsync_id=" + request.targetVsyncId
                    + " outcome=callback_timeout timeout_ms=" + CALLBACK_TIMEOUT_MS);
        }

        private static void safelyHandleCallback(Request request,
                                                SurfaceControl.TransactionStats stats) {
            // Framework가 listener 정상 반환 뒤 TransactionStats.close()를 호출하므로,
            // 이 경계에서는 어떤 handler 예외도 밖으로 전파하지 않습니다.
            try {
                handleCallback(request, stats);
            } catch (Throwable error) {
                try {
                    Log.e(TAG, "SPINON_R05_PRESENT_FENCE renderer=" + request.renderer
                            + " request_id=" + request.requestId
                            + " input_seq=" + request.inputSequence
                            + " generation=" + request.generation
                            + " outcome=callback_handler_error"
                            + " error=" + error.getClass().getSimpleName());
                } catch (Throwable ignored) {
                    // 진단 로그 실패가 framework 자원 정리를 막지 않게 합니다.
                }
            }
        }

        private static CallbackObservation handleCallback(
                Request request, SurfaceControl.TransactionStats stats) {
            request.callbackCount.incrementAndGet();
            CALLBACK_EVENT_TOTAL.incrementAndGet();
            boolean timely = request.state.compareAndSet(PENDING, CALLBACK_RECEIVED);
            if (timely) {
                removePending(request);
                if (request.timeout != null) TIMEOUT_HANDLER.removeCallbacks(request.timeout);
            }

            android.hardware.SyncFence fence = null;
            long latchTimeNanos = -1;
            long signalTimeNanos = android.hardware.SyncFence.SIGNAL_TIME_INVALID;
            long monotonicBeforeNanos = System.nanoTime();
            long uptimeAnchorNanos = SystemClock.uptimeNanos();
            long monotonicAfterNanos = System.nanoTime();
            long signalObservedMonotonicNanos = -1;
            String fenceState = "stats_unavailable";
            String processingError = "none";
            String closeError = "none";
            boolean fenceValid = false;
            try {
                if (stats == null) {
                    fenceState = "null_transaction_stats";
                } else {
                    latchTimeNanos = stats.getLatchTimeNanos();
                    fence = stats.getPresentFence();
                    fenceValid = fence.isValid();
                    if (!fenceValid) {
                        fenceState = "empty_or_invalid";
                    } else {
                        signalTimeNanos = fence.getSignalTime();
                        signalObservedMonotonicNanos = System.nanoTime();
                        if (signalTimeNanos == android.hardware.SyncFence.SIGNAL_TIME_INVALID) {
                            fenceState = "invalid_signal_time";
                        } else if (signalTimeNanos == android.hardware.SyncFence.SIGNAL_TIME_PENDING) {
                            fenceState = "pending";
                        } else if (signalTimeNanos > 0) {
                            if (signalTimeNanos > signalObservedMonotonicNanos) {
                                fenceState = "future_signal_time";
                            } else if (latchTimeNanos > signalTimeNanos) {
                                fenceState = "latch_after_signal";
                            } else {
                                fenceState = "signaled";
                            }
                        } else {
                            fenceState = "non_positive_signal_time";
                        }
                        if (timely && request.callbackCount.get() == 1
                                && R05PresentFenceWaitExperiment.isEnabled()) {
                            long waitGeneration = safeCurrentGeneration(request);
                            boolean waitSurface = safeSurfaceAvailable(request)
                                    && waitGeneration == request.generation;
                            try {
                                R05PresentFenceWaitExperiment.observe(fence,
                                        request.renderer, request.requestId,
                                        request.inputSequence, request.revision,
                                        request.generation, waitGeneration, waitSurface,
                                        request.targetVsyncId, request.eventTimeNanos,
                                        latchTimeNanos, fenceState, signalTimeNanos,
                                        CALLBACK_RAN_INLINE.get());
                            } catch (Throwable error) {
                                Log.e(TAG, "SPINON_R05_FENCE_WAIT renderer="
                                        + request.renderer
                                        + " request_id=" + request.requestId
                                        + " input_seq=" + request.inputSequence
                                        + " outcome=observer_error error="
                                        + error.getClass().getSimpleName());
                            }
                        }
                    }
                }
            } catch (Throwable error) {
                processingError = error.getClass().getSimpleName();
                fenceState = "read_failed";
            } finally {
                if (fence != null) {
                    try {
                        fence.close();
                    } catch (Throwable error) {
                        closeError = error.getClass().getSimpleName();
                    }
                }
            }
            long currentGeneration = safeCurrentGeneration(request);
            boolean currentSurface = safeSurfaceAvailable(request)
                    && currentGeneration == request.generation;
            boolean inlineOverflow = CALLBACK_RAN_INLINE.get();
            boolean fenceSignalUsable = timely && currentSurface && fenceValid
                    && "signaled".equals(fenceState) && !inlineOverflow
                    && "none".equals(processingError) && "none".equals(closeError);
            String outcome = timely ? "callback_received" : stateName(request.state.get());
            long inputOffsetMinNanos = request.inputUptimeAnchorNanos
                    - request.inputMonotonicAfterNanos;
            long inputOffsetMaxNanos = request.inputUptimeAnchorNanos
                    - request.inputMonotonicBeforeNanos;
            long callbackOffsetMinNanos = uptimeAnchorNanos - monotonicAfterNanos;
            long callbackOffsetMaxNanos = uptimeAnchorNanos - monotonicBeforeNanos;
            boolean clockOffsetIntervalsOverlap = inputOffsetMinNanos <= callbackOffsetMaxNanos
                    && callbackOffsetMinNanos <= inputOffsetMaxNanos;

            Log.i(TAG, "SPINON_R05_PRESENT_FENCE renderer=" + request.renderer
                    + " request_id=" + request.requestId
                    + " input_seq=" + request.inputSequence
                    + " revision=" + request.revision
                    + " event_time_ns=" + request.eventTimeNanos
                    + " generation=" + request.generation
                    + " current_generation=" + currentGeneration
                    + " current_surface=" + currentSurface
                    + " target_vsync_id=" + request.targetVsyncId
                    + " outcome=" + outcome
                    + " fence_valid=" + fenceValid
                    + " fence_state=" + fenceState
                    + " signal_time_ns=" + signalTimeNanos
                    + " latch_time_ns=" + latchTimeNanos
                    + " fence_signal_usable=" + fenceSignalUsable
                    + " callback_inline_overflow=" + inlineOverflow
                    + " inline_callback_total=" + INLINE_CALLBACK_COUNT.get()
                    + " callback_os_tid=" + Process.myTid()
                    + " callback_thread_id=" + Thread.currentThread().getId()
                    + " queue_depth=" + CALLBACK_POOL.getQueue().size()
                    + " input_uptime_anchor_ns=" + request.inputUptimeAnchorNanos
                    + " input_monotonic_before_ns=" + request.inputMonotonicBeforeNanos
                    + " input_monotonic_after_ns=" + request.inputMonotonicAfterNanos
                    + " callback_uptime_anchor_ns=" + uptimeAnchorNanos
                    + " callback_monotonic_before_ns=" + monotonicBeforeNanos
                    + " callback_monotonic_after_ns=" + monotonicAfterNanos
                    + " signal_observed_monotonic_ns=" + signalObservedMonotonicNanos
                    + " input_offset_min_ns=" + inputOffsetMinNanos
                    + " input_offset_max_ns=" + inputOffsetMaxNanos
                    + " callback_offset_min_ns=" + callbackOffsetMinNanos
                    + " callback_offset_max_ns=" + callbackOffsetMaxNanos
                    + " clock_offset_intervals_overlap=" + clockOffsetIntervalsOverlap
                    + " processing_error=" + processingError
                    + " fence_close_error=" + closeError);
            CallbackObservation observation = new CallbackObservation(outcome, fenceState,
                    currentSurface, stats != null, fenceSignalUsable, inlineOverflow);
            request.lastCallbackObservation = observation;
            return observation;
        }

        private static long safeCurrentGeneration(Request request) {
            try {
                return request.currentGeneration.getAsLong();
            } catch (RuntimeException | LinkageError ignored) {
                return -1;
            }
        }

        private static boolean safeSurfaceAvailable(Request request) {
            try {
                return request.surfaceAvailable.getAsBoolean();
            } catch (RuntimeException | LinkageError ignored) {
                return false;
            }
        }

        private static String stateName(int state) {
            if (state == TIMED_OUT) return "late_after_timeout";
            if (state == CANCELLED) return "late_after_cancel";
            return "duplicate_callback";
        }

        private static void removePending(Request request) {
            synchronized (PENDING_LOCK) {
                PENDING_REQUESTS.remove(request.requestId, request);
            }
        }

        private static boolean closeTransaction(SurfaceControl.Transaction transaction,
                                                String renderer, long generation,
                                                long requestId) {
            try {
                transaction.close();
                return true;
            } catch (RuntimeException | LinkageError error) {
                Log.w(TAG, "SPINON_R05_PRESENT_FENCE_TRANSACTION_CLOSE renderer=" + renderer
                        + " request_id=" + requestId + " generation=" + generation
                        + " outcome=failed error=" + error.getClass().getSimpleName());
                return false;
            }
        }
    }

    private static final class Request {
        final long requestId;
        final String renderer;
        final long generation;
        final LongSupplier currentGeneration;
        final BooleanSupplier surfaceAvailable;
        final long inputSequence;
        final long revision;
        final long eventTimeNanos;
        final long inputUptimeAnchorNanos;
        final long inputMonotonicBeforeNanos;
        final long inputMonotonicAfterNanos;
        final long targetVsyncId;
        final AtomicInteger state = new AtomicInteger(Api35.PENDING);
        final AtomicInteger callbackCount = new AtomicInteger();
        final AtomicInteger timeoutCount = new AtomicInteger();
        volatile CallbackObservation lastCallbackObservation;
        Runnable timeout;

        Request(long requestId, String renderer, long generation,
                LongSupplier currentGeneration, BooleanSupplier surfaceAvailable,
                long inputSequence, long revision, long eventTimeNanos,
                long inputUptimeAnchorNanos, long inputMonotonicBeforeNanos,
                long inputMonotonicAfterNanos, long targetVsyncId) {
            this.requestId = requestId;
            this.renderer = renderer;
            this.generation = generation;
            this.currentGeneration = currentGeneration;
            this.surfaceAvailable = surfaceAvailable;
            this.inputSequence = inputSequence;
            this.revision = revision;
            this.eventTimeNanos = eventTimeNanos;
            this.inputUptimeAnchorNanos = inputUptimeAnchorNanos;
            this.inputMonotonicBeforeNanos = inputMonotonicBeforeNanos;
            this.inputMonotonicAfterNanos = inputMonotonicAfterNanos;
            this.targetVsyncId = targetVsyncId;
        }
    }

    private static final class CallbackTask implements Runnable {
        private final Runnable command;
        volatile boolean inlineOverflow;

        CallbackTask(Runnable command) {
            this.command = command;
        }

        @Override
        public void run() {
            boolean previous = CALLBACK_RAN_INLINE.get();
            if (inlineOverflow) CALLBACK_RAN_INLINE.set(true);
            try {
                command.run();
            } catch (Throwable error) {
                try {
                    Log.e(TAG, "SPINON_R05_PRESENT_FENCE_EXECUTOR outcome=callback_command_error"
                            + " error=" + error.getClass().getSimpleName());
                } catch (Throwable ignored) {
                    // framework callback command의 종료 예외를 다시 전파하지 않습니다.
                }
            } finally {
                CALLBACK_RAN_INLINE.set(previous);
            }
        }
    }

    private static final class InlineOnRejectHandler implements RejectedExecutionHandler {
        @Override
        public void rejectedExecution(Runnable task, ThreadPoolExecutor executor) {
            if (task instanceof CallbackTask) {
                ((CallbackTask) task).inlineOverflow = true;
                INLINE_CALLBACK_COUNT.incrementAndGet();
            }
            task.run();
        }
    }

    private static final class ProbeThreadFactory implements ThreadFactory {
        @Override
        public Thread newThread(Runnable task) {
            Thread thread = new Thread(task, "SpinonR05PresentFence");
            thread.setDaemon(true);
            return thread;
        }
    }
}
