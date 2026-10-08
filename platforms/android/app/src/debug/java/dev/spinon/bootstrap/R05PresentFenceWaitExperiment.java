package dev.spinon.bootstrap;

import android.os.Build;
import android.os.Process;
import android.util.Log;

import java.time.Duration;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.RejectedExecutionException;
import java.util.concurrent.ThreadFactory;
import java.util.concurrent.ThreadPoolExecutor;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;

/** debug 전용으로 TransactionStats present fence의 callback 이후 signal을 관찰합니다. */
final class R05PresentFenceWaitExperiment {
    private static final String TAG = "SpinonBootstrap";
    private static final long WAIT_TIMEOUT_MS = 250;
    private static final int WAIT_THREADS = 2;
    private static final int WAIT_QUEUE_CAPACITY = 64;
    private static final AtomicBoolean ENABLED = new AtomicBoolean();
    private static final AtomicInteger PENDING = new AtomicInteger();
    private static final AtomicInteger ACTIVE = new AtomicInteger();
    private static final AtomicLong COMPLETED = new AtomicLong();

    private R05PresentFenceWaitExperiment() {}

    static boolean enable() {
        if (!BuildConfig.DEBUG) {
            Log.e(TAG, "SPINON_R05_FENCE_WAIT status=UNAVAILABLE reason=debug_only");
            return false;
        }
        if (Build.VERSION.SDK_INT < 35) {
            Log.e(TAG, "SPINON_R05_FENCE_WAIT status=UNAVAILABLE reason=api_below_35"
                    + " api=" + Build.VERSION.SDK_INT);
            return false;
        }
        ENABLED.set(true);
        Log.i(TAG, "SPINON_R05_FENCE_WAIT_START api=" + Build.VERSION.SDK_INT
                + " timeout_ms=" + WAIT_TIMEOUT_MS
                + " workers=" + WAIT_THREADS
                + " queue_capacity=" + WAIT_QUEUE_CAPACITY);
        return true;
    }

    static boolean isEnabled() {
        return ENABLED.get();
    }

    static void disable() {
        ENABLED.set(false);
    }

    static void observe(Object fence, String renderer, long requestId, long inputSequence,
                        long revision, long generation, long currentGeneration,
                        boolean currentSurface, long targetVsyncId, long eventTimeNanos,
                        long latchTimeNanos, String initialFenceState,
                        long initialSignalTimeNanos, boolean callbackInlineOverflow) {
        if (!ENABLED.get()) return;
        if (Build.VERSION.SDK_INT < 35) {
            logSkip(renderer, requestId, inputSequence, revision, generation,
                    "api_below_35");
            return;
        }
        if (!currentSurface || currentGeneration != generation) {
            logSkip(renderer, requestId, inputSequence, revision, generation,
                    "stale_surface_at_callback");
            return;
        }
        if (callbackInlineOverflow) {
            logSkip(renderer, requestId, inputSequence, revision, generation,
                    "callback_inline_overflow");
            return;
        }

        Snapshot snapshot = new Snapshot(renderer, requestId, inputSequence, revision,
                generation, currentGeneration, currentSurface, targetVsyncId,
                eventTimeNanos, latchTimeNanos, initialFenceState,
                initialSignalTimeNanos, callbackInlineOverflow);
        Api35.copyAndQueue(fence, snapshot);
    }

    private static void logSkip(String renderer, long requestId, long inputSequence,
                                long revision, long generation, String reason) {
        Log.w(TAG, "SPINON_R05_FENCE_WAIT renderer=" + renderer
                + " request_id=" + requestId
                + " input_seq=" + inputSequence
                + " revision=" + revision
                + " generation=" + generation
                + " outcome=skipped reason=" + reason);
    }

    private static ThreadPoolExecutor waitPool() {
        return WaitPoolHolder.INSTANCE;
    }

    private static final class WaitPoolHolder {
        static final ThreadPoolExecutor INSTANCE = new ThreadPoolExecutor(
                WAIT_THREADS, WAIT_THREADS, 0L, TimeUnit.MILLISECONDS,
                new ArrayBlockingQueue<>(WAIT_QUEUE_CAPACITY), new WaitThreadFactory(),
                new ThreadPoolExecutor.AbortPolicy());
    }

    private static final class Api35 {
        static void copyAndQueue(Object sourceObject, Snapshot snapshot) {
            if (!(sourceObject instanceof android.hardware.SyncFence)) {
                logSkip(snapshot.renderer, snapshot.requestId, snapshot.inputSequence,
                        snapshot.revision, snapshot.generation, "unexpected_fence_type");
                return;
            }
            android.hardware.SyncFence source = (android.hardware.SyncFence) sourceObject;
            android.hardware.SyncFence copy = null;
            FenceWaitTask task = null;
            boolean pendingRegistered = false;
            boolean submitted = false;
            try {
                if (!source.isValid()) {
                    logResult(snapshot, "invalid_source_fence", false, false,
                            false, -1, -1, -1, -1, "none", -1, -1);
                    return;
                }
                copy = new android.hardware.SyncFence(source);
                if (!copy.isValid()) {
                    String closeError = close(copy);
                    copy = null;
                    logResult(snapshot, "invalid_copied_fence", false, false,
                            true, -1, -1, -1, -1, closeError, -1, -1);
                    return;
                }
                task = new FenceWaitTask(copy, snapshot);
                PENDING.incrementAndGet();
                pendingRegistered = true;
                waitPool().execute(task);
                submitted = true;
                copy = null;
                task = null;
            } catch (Throwable error) {
                if (!submitted) {
                    if (pendingRegistered) PENDING.decrementAndGet();
                    String closeError = task != null
                            ? task.closeCopy() : copy == null ? "none" : close(copy);
                    String outcome = error instanceof RejectedExecutionException
                            ? "queue_rejected" : "copy_or_enqueue_failed";
                    logResult(snapshot, outcome + "_" + error.getClass().getSimpleName(),
                            false, false, false, -1, -1, -1, -1, closeError,
                            PENDING.get(), queueDepth());
                }
                return;
            }
            if (submitted) {
                try {
                    Log.i(TAG, "SPINON_R05_FENCE_WAIT_QUEUED renderer=" + snapshot.renderer
                            + " request_id=" + snapshot.requestId
                            + " input_seq=" + snapshot.inputSequence
                            + " revision=" + snapshot.revision
                            + " generation=" + snapshot.generation
                            + " pending=" + PENDING.get()
                            + " active=" + ACTIVE.get()
                            + " queue_depth=" + queueDepth());
                } catch (Throwable ignored) {
                    // 진단 로그 실패가 worker의 fence 소유권을 바꾸지 않습니다.
                }
            }
        }

        private static final class FenceWaitTask implements Runnable {
            private final android.hardware.SyncFence fence;
            private final Snapshot snapshot;
            private final AtomicBoolean closed = new AtomicBoolean();

            FenceWaitTask(android.hardware.SyncFence fence, Snapshot snapshot) {
                this.fence = fence;
                this.snapshot = snapshot;
            }

            @Override
            public void run() {
                ACTIVE.incrementAndGet();
                long waitStartNanos = System.nanoTime();
                long waitReturnNanos = -1;
                long signalTimeNanos = android.hardware.SyncFence.SIGNAL_TIME_INVALID;
                long signalObservedMonotonicNanos = -1;
                boolean validBefore = false;
                boolean awaitReturned = false;
                boolean validAfter = false;
                String outcome = "wait_failed";
                String waitError = "none";
                try {
                    validBefore = fence.isValid();
                    if (!validBefore) {
                        outcome = "invalid_before_wait";
                    } else {
                        awaitReturned = fence.await(Duration.ofMillis(WAIT_TIMEOUT_MS));
                        waitReturnNanos = System.nanoTime();
                        validAfter = fence.isValid();
                        signalTimeNanos = fence.getSignalTime();
                        signalObservedMonotonicNanos = System.nanoTime();
                        outcome = classify(validAfter, awaitReturned, waitReturnNanos,
                                signalTimeNanos, signalObservedMonotonicNanos,
                                snapshot.latchTimeNanos);
                    }
                } catch (Throwable error) {
                    waitError = error.getClass().getSimpleName();
                    outcome = "wait_exception";
                    if (waitReturnNanos < 0) waitReturnNanos = System.nanoTime();
                } finally {
                    String closeError = closeCopy();
                    int pending = PENDING.decrementAndGet();
                    int active = ACTIVE.decrementAndGet();
                    long completed = COMPLETED.incrementAndGet();
                    boolean usable = "signaled".equals(outcome) && validBefore && validAfter
                            && snapshot.currentSurface && !snapshot.callbackInlineOverflow
                            && "none".equals(waitError) && "none".equals(closeError);
                    logResult(snapshot, outcome, validBefore, awaitReturned, validAfter,
                            waitStartNanos, waitReturnNanos, signalTimeNanos,
                            signalObservedMonotonicNanos, closeError, pending,
                            queueDepth(), active, completed, waitError, usable);
                }
            }

            String closeCopy() {
                if (!closed.compareAndSet(false, true)) return "already_closed";
                return close(fence);
            }
        }

        private static String classify(boolean validAfter, boolean awaitReturned,
                                       long waitReturnNanos, long signalTimeNanos,
                                       long signalObservedMonotonicNanos,
                                       long latchTimeNanos) {
            if (!validAfter) return "invalid_after_wait";
            if (signalTimeNanos == android.hardware.SyncFence.SIGNAL_TIME_INVALID) {
                return "invalid_signal_time";
            }
            if (signalTimeNanos == android.hardware.SyncFence.SIGNAL_TIME_PENDING) {
                return awaitReturned ? "await_true_but_pending" : "timed_out_pending";
            }
            if (signalTimeNanos <= 0) return "non_positive_signal_time";
            if (signalTimeNanos > signalObservedMonotonicNanos) return "future_signal_time";
            if (latchTimeNanos > 0 && latchTimeNanos > signalTimeNanos) {
                return "latch_after_signal";
            }
            if (!awaitReturned) {
                return signalTimeNanos > waitReturnNanos
                        ? "signaled_after_timeout" : "timeout_with_predeadline_signal";
            }
            return signalTimeNanos <= waitReturnNanos
                    ? "signaled" : "await_true_without_signal";
        }

        private static String close(android.hardware.SyncFence fence) {
            try {
                fence.close();
                return "none";
            } catch (Throwable error) {
                return error.getClass().getSimpleName();
            }
        }
    }

    private static int queueDepth() {
        try {
            return waitPool().getQueue().size();
        } catch (Throwable ignored) {
            return -1;
        }
    }

    private static void logResult(Snapshot snapshot, String outcome,
                                  boolean validBefore, boolean awaitReturned,
                                  boolean validAfter, long waitStartNanos,
                                  long waitReturnNanos, long signalTimeNanos,
                                  long signalObservedMonotonicNanos, String closeError,
                                  int pending, int queueDepth) {
        logResult(snapshot, outcome, validBefore, awaitReturned, validAfter,
                waitStartNanos, waitReturnNanos, signalTimeNanos,
                signalObservedMonotonicNanos, closeError, pending, queueDepth,
                ACTIVE.get(), COMPLETED.get(), "none", false);
    }

    private static void logResult(Snapshot snapshot, String outcome,
                                  boolean validBefore, boolean awaitReturned,
                                  boolean validAfter, long waitStartNanos,
                                  long waitReturnNanos, long signalTimeNanos,
                                  long signalObservedMonotonicNanos, String closeError,
                                  int pending, int queueDepth, int active, long completed,
                                  String waitError, boolean usable) {
        long waitElapsedNanos = waitStartNanos >= 0 && waitReturnNanos >= waitStartNanos
                ? waitReturnNanos - waitStartNanos : -1;
        Log.i(TAG, "SPINON_R05_FENCE_WAIT renderer=" + snapshot.renderer
                + " request_id=" + snapshot.requestId
                + " input_seq=" + snapshot.inputSequence
                + " revision=" + snapshot.revision
                + " generation=" + snapshot.generation
                + " current_generation=" + snapshot.currentGeneration
                + " current_surface_at_callback=" + snapshot.currentSurface
                + " target_vsync_id=" + snapshot.targetVsyncId
                + " event_time_ns=" + snapshot.eventTimeNanos
                + " latch_time_ns=" + snapshot.latchTimeNanos
                + " initial_fence_state=" + snapshot.initialFenceState
                + " initial_signal_time_ns=" + snapshot.initialSignalTimeNanos
                + " outcome=" + outcome
                + " fence_valid_before=" + validBefore
                + " await_returned=" + awaitReturned
                + " fence_valid_after=" + validAfter
                + " signal_time_ns=" + signalTimeNanos
                + " signal_time_observed_monotonic_ns=" + signalObservedMonotonicNanos
                + " wait_start_monotonic_ns=" + waitStartNanos
                + " wait_return_monotonic_ns=" + waitReturnNanos
                + " wait_elapsed_ns=" + waitElapsedNanos
                + " callback_inline_overflow=" + snapshot.callbackInlineOverflow
                + " fence_signal_usable=" + usable
                + " wait_error=" + waitError
                + " fence_close_error=" + closeError
                + " pending=" + pending
                + " active=" + active
                + " queue_depth=" + queueDepth
                + " completed=" + completed
                + " waiter_os_tid=" + Process.myTid());
    }

    private static final class Snapshot {
        final String renderer;
        final long requestId;
        final long inputSequence;
        final long revision;
        final long generation;
        final long currentGeneration;
        final boolean currentSurface;
        final long targetVsyncId;
        final long eventTimeNanos;
        final long latchTimeNanos;
        final String initialFenceState;
        final long initialSignalTimeNanos;
        final boolean callbackInlineOverflow;

        Snapshot(String renderer, long requestId, long inputSequence, long revision,
                 long generation, long currentGeneration, boolean currentSurface,
                 long targetVsyncId, long eventTimeNanos, long latchTimeNanos,
                 String initialFenceState, long initialSignalTimeNanos,
                 boolean callbackInlineOverflow) {
            this.renderer = renderer;
            this.requestId = requestId;
            this.inputSequence = inputSequence;
            this.revision = revision;
            this.generation = generation;
            this.currentGeneration = currentGeneration;
            this.currentSurface = currentSurface;
            this.targetVsyncId = targetVsyncId;
            this.eventTimeNanos = eventTimeNanos;
            this.latchTimeNanos = latchTimeNanos;
            this.initialFenceState = initialFenceState;
            this.initialSignalTimeNanos = initialSignalTimeNanos;
            this.callbackInlineOverflow = callbackInlineOverflow;
        }
    }

    private static final class WaitThreadFactory implements ThreadFactory {
        private final AtomicInteger nextId = new AtomicInteger(1);

        @Override
        public Thread newThread(Runnable runnable) {
            Thread thread = new Thread(runnable,
                    "SpinonR05FenceWait-" + nextId.getAndIncrement());
            thread.setDaemon(true);
            return thread;
        }
    }
}
