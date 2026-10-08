package dev.spinon.bootstrap;

import android.os.Build;
import android.os.Handler;
import android.os.Looper;
import android.os.Process;
import android.os.SystemClock;
import android.util.Log;
import android.view.SurfaceControl;
import android.view.SurfaceView;

import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.Executor;
import java.util.concurrent.ThreadFactory;
import java.util.concurrent.ThreadPoolExecutor;
import java.util.concurrent.TimeUnit;
import java.util.LinkedHashMap;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicLong;
import java.util.function.BooleanSupplier;
import java.util.function.LongSupplier;

/** Android 17 QPR2 SurfaceView 표시 신호를 원시값으로 기록하는 내부 probe. */
final class R05PresentTimingProbe {
    private static final String TAG = "SpinonBootstrap";
    static final long INVALID_SURFACE_GENERATION = -1L;
    private static final AtomicLong NEXT_SURFACE_GENERATION = new AtomicLong();
    private static final Handler MAIN_HANDLER = new Handler(Looper.getMainLooper());
    private static final int MAX_FRAME_TIMELINE_TARGETS = 64;
    private static final Object FRAME_TIMELINE_TARGET_LOCK = new Object();
    private static final LinkedHashMap<Long, FrameTimelineTarget> FRAME_TIMELINE_TARGETS =
            new LinkedHashMap<>();

    private R05PresentTimingProbe() {}

    static long nextSurfaceGeneration() {
        while (true) {
            long current = NEXT_SURFACE_GENERATION.get();
            if (current == Long.MAX_VALUE) {
                return INVALID_SURFACE_GENERATION;
            }
            long next = current + 1;
            if (NEXT_SURFACE_GENERATION.compareAndSet(current, next)) return next;
        }
    }

    static boolean isFrameTimelineJoinAvailable() {
        if (Build.VERSION.SDK_INT < 37) return false;
        return ApiVersion37_2.isAvailable(ApiVersion36.sdkIntFull());
    }

    static boolean registerFrameTimelineTarget(long vsyncId, String renderer,
                                               long generation, long inputSequence,
                                               long revision, long requestId) {
        synchronized (FRAME_TIMELINE_TARGET_LOCK) {
            if (FRAME_TIMELINE_TARGETS.size() >= MAX_FRAME_TIMELINE_TARGETS
                    || FRAME_TIMELINE_TARGETS.containsKey(vsyncId)) {
                return false;
            }
            FRAME_TIMELINE_TARGETS.put(vsyncId, new FrameTimelineTarget(
                    vsyncId, renderer, generation, inputSequence, revision, requestId));
            return true;
        }
    }

    static void removeFrameTimelineTarget(long vsyncId, long requestId) {
        synchronized (FRAME_TIMELINE_TARGET_LOCK) {
            FrameTimelineTarget target = FRAME_TIMELINE_TARGETS.get(vsyncId);
            if (target != null && target.requestId == requestId) {
                FRAME_TIMELINE_TARGETS.remove(vsyncId);
            }
        }
    }

    private static FrameTimelineMatch matchFrameTimeline(long vsyncId, String renderer,
                                                          long generation, boolean currentSurface) {
        synchronized (FRAME_TIMELINE_TARGET_LOCK) {
            FrameTimelineTarget target = FRAME_TIMELINE_TARGETS.get(vsyncId);
            if (target == null) return FrameTimelineMatch.unmatched();
            int recordNumber = ++target.recordCount;
            String status;
            if (recordNumber > 1) {
                status = "duplicate";
            } else if (!target.renderer.equals(renderer)) {
                status = "wrong_renderer";
            } else if (target.generation != generation || !currentSurface) {
                status = "stale_surface";
            } else {
                status = "exact_match";
            }
            return new FrameTimelineMatch(status, target.inputSequence, target.revision,
                    target.requestId, target.vsyncId);
        }
    }

    static Object register(SurfaceView surface, String renderer, long generation,
                           LongSupplier currentGeneration,
                           BooleanSupplier surfaceAvailable) {
        if (Build.VERSION.SDK_INT < 36) {
            logCapability(renderer, false, "api_below_36", Build.VERSION.SDK_INT, 0);
            return null;
        }

        int sdkIntFull = ApiVersion36.sdkIntFull();
        if (Build.VERSION.SDK_INT < 37) {
            logCapability(renderer, false, "api_below_37", Build.VERSION.SDK_INT, sdkIntFull);
            return null;
        }
        if (!ApiVersion37_2.isAvailable(sdkIntFull)) {
            logCapability(renderer, false, "api_below_37_2", Build.VERSION.SDK_INT, sdkIntFull);
            return null;
        }

        try {
            Object registration = Api37_2.register(
                    surface, renderer, generation, currentGeneration, surfaceAvailable);
            logCapability(renderer, registration != null,
                    registration != null ? "registered" : "registration_returned_null",
                    Build.VERSION.SDK_INT, sdkIntFull);
            return registration;
        } catch (RuntimeException | LinkageError error) {
            Log.w(TAG, "SPINON_R05_SIGNAL_CAPABILITY renderer=" + renderer
                    + " signal=SurfaceView_JankData available=false reason=registration_failed"
                    + " api=" + Build.VERSION.SDK_INT + " api_full=" + sdkIntFull
                    + " error=" + error.getClass().getSimpleName());
            return null;
        }
    }

    static void unregister(Object registration, String renderer, long generation) {
        if (registration == null) return;
        try {
            Api37_2.unregister(registration, renderer, generation);
        } catch (RuntimeException | LinkageError error) {
            Log.w(TAG, "SPINON_R05_SIGNAL_LISTENER outcome=remove_failed"
                    + " renderer=" + renderer + " generation=" + generation
                    + " error=" + error.getClass().getSimpleName());
        }
    }

    static void flush(Object registration, String renderer, long generation,
                      LongSupplier currentGeneration, BooleanSupplier surfaceAvailable) {
        if (registration == null) return;
        try {
            boolean queued = Api37_2.flush(
                    registration, renderer, generation, currentGeneration, surfaceAvailable);
            Log.i(TAG, "SPINON_R05_PRESENT_FLUSH renderer=" + renderer + " queued=" + queued
                    + (queued ? "" : " reason=queue_full")
                    + " generation=" + generation);
        } catch (RuntimeException | LinkageError error) {
            Log.w(TAG, "SPINON_R05_PRESENT_FLUSH requested=false"
                    + " renderer=" + renderer + " generation=" + generation
                    + " error=" + error.getClass().getSimpleName());
        }
    }

    static void flushAfterPresentation(Object registration, String renderer, long generation,
                                       LongSupplier currentGeneration,
                                       BooleanSupplier surfaceAvailable) {
        boolean queued = MAIN_HANDLER.postDelayed(() -> flush(registration, renderer, generation,
                currentGeneration, surfaceAvailable), 250);
        Log.i(TAG, "SPINON_R05_PRESENT_DRAIN renderer=" + renderer
                + " generation=" + generation + " queued=" + queued + " delay_ms=250");
    }

    static void runQueueSaturationProbe(String renderer) {
        if (Build.VERSION.SDK_INT < 37
                || !ApiVersion37_2.isAvailable(ApiVersion36.sdkIntFull())) {
            Log.i(TAG, "SPINON_R05_QUEUE_PROBE=UNSUPPORTED renderer=" + renderer
                    + " api=" + Build.VERSION.SDK_INT);
            return;
        }
        Api37_2.runQueueSaturationProbe(renderer);
    }

    private static void logCapability(String renderer, boolean available, String reason,
                                      int api, int apiFull) {
        Log.i(TAG, "SPINON_R05_SIGNAL_CAPABILITY renderer=" + renderer
                + " signal=SurfaceView_JankData available=" + available
                + " reason=" + reason + " api=" + api + " api_full=" + apiFull);
    }

    /** API 36에서 추가된 full SDK 값을 구버전 API에서 읽지 않습니다. */
    private static final class ApiVersion36 {
        private ApiVersion36() {}

        static int sdkIntFull() {
            return Build.VERSION.SDK_INT_FULL;
        }
    }

    /** API 37.2 상수 비교만 수행합니다. */
    private static final class ApiVersion37_2 {
        private ApiVersion37_2() {}

        static boolean isAvailable(int sdkIntFull) {
            return sdkIntFull >= Build.VERSION_CODES_FULL.CINNAMON_BUN_2;
        }
    }

    /** 37.2 전용 framework 참조와 callback executor를 구버전에서 분리합니다. */
    private static final class Api37_2 {
        private static final int QUEUE_CAPACITY = 64;
        private static final AtomicLong BATCH_SEQUENCE = new AtomicLong();
        private static final AtomicLong REJECTED_CALLBACK_BATCHES = new AtomicLong();
        private static final AtomicLong REJECTED_FLUSH_TASKS = new AtomicLong();
        private static final ThreadPoolExecutor CALLBACK_EXECUTOR = new ThreadPoolExecutor(
                1, 1, 0L, TimeUnit.MILLISECONDS,
                new ArrayBlockingQueue<>(QUEUE_CAPACITY), new ProbeThreadFactory(),
                (task, executor) -> {
                    if (task instanceof CallbackBatchTask) {
                        ((CallbackBatchTask) task).rejected = true;
                        REJECTED_CALLBACK_BATCHES.incrementAndGet();
                    } else if (task instanceof FlushTask) {
                        ((FlushTask) task).rejected = true;
                        REJECTED_FLUSH_TASKS.incrementAndGet();
                    }
                });

        private Api37_2() {}

        static Object register(SurfaceView surface, String renderer, long generation,
                               LongSupplier currentGeneration,
                               BooleanSupplier surfaceAvailable) {
            long registeringThreadOsTid = Process.myTid();
            Executor callbackExecutor = command -> CALLBACK_EXECUTOR.execute(
                    new CallbackBatchTask(command));
            return surface.registerOnJankDataListener(callbackExecutor, data -> {
                long batchSequence = BATCH_SEQUENCE.incrementAndGet();
                long callbackUptimeNanos = SystemClock.uptimeNanos();
                int batchSize = data == null ? 0 : data.size();
                Log.i(TAG, "SPINON_R05_PRESENT_BATCH renderer=" + renderer
                        + " generation=" + generation + " batch_seq=" + batchSequence
                        + " record_count=" + batchSize
                        + " callback_os_tid=" + Process.myTid()
                        + " callback_thread_id=" + Thread.currentThread().getId()
                        + " registering_thread_os_tid=" + registeringThreadOsTid
                        + " queue_depth=" + CALLBACK_EXECUTOR.getQueue().size()
                        + " rejected_callback_batches=" + REJECTED_CALLBACK_BATCHES.get()
                        + " rejected_flush_tasks=" + REJECTED_FLUSH_TASKS.get()
                        + " callback_uptime_ns=" + callbackUptimeNanos
                        + " attribution=unmatched");
                if (data == null) return;
                int index = 0;
                for (SurfaceControl.JankData record : data) {
                    if (record != null) {
                        logRecord(renderer, record, generation, currentGeneration.getAsLong(),
                                surfaceAvailable.getAsBoolean(), callbackUptimeNanos,
                                batchSequence, index, registeringThreadOsTid);
                    } else {
                        Log.w(TAG, "SPINON_R05_PRESENT_RECORD renderer=" + renderer
                                + " generation=" + generation + " batch_seq=" + batchSequence
                                + " record_index=" + index + " outcome=null_record");
                    }
                    index++;
                }
            });
        }

        static void unregister(Object registration, String renderer, long generation) {
            ((SurfaceControl.OnJankDataListenerRegistration) registration).removeAfter(0);
            Log.i(TAG, "SPINON_R05_SIGNAL_LISTENER outcome=remove_requested"
                    + " renderer=" + renderer + " generation=" + generation
                    + " rejected_callback_batches=" + REJECTED_CALLBACK_BATCHES.get()
                    + " rejected_flush_tasks=" + REJECTED_FLUSH_TASKS.get());
        }

        static boolean flush(Object registration, String renderer, long generation,
                             LongSupplier currentGeneration,
                             BooleanSupplier surfaceAvailable) {
            FlushTask task = new FlushTask(registration, renderer, generation,
                    currentGeneration, surfaceAvailable);
            CALLBACK_EXECUTOR.execute(task);
            return !task.rejected;
        }

        static void runQueueSaturationProbe(String renderer) {
            Thread probe = new Thread(() -> runQueueSaturationProbeOnWorker(renderer),
                    "SpinonR05QueueProbe");
            probe.setDaemon(true);
            probe.start();
        }

        private static void runQueueSaturationProbeOnWorker(String renderer) {
            CountDownLatch workerEntered = new CountDownLatch(1);
            CountDownLatch releaseWorker = new CountDownLatch(1);
            CountDownLatch queuedTasksFinished = new CountDownLatch(QUEUE_CAPACITY);
            long callbackRejectsBefore = REJECTED_CALLBACK_BATCHES.get();
            long flushRejectsBefore = REJECTED_FLUSH_TASKS.get();
            boolean blockerAccepted = false;
            int callbackTasksAccepted = 0;
            boolean overflowCallbackRejected = false;
            AtomicBoolean overflowCallbackRan = new AtomicBoolean();
            boolean overflowFlushRejected = false;
            boolean workerRecovered = false;
            try {
                CALLBACK_EXECUTOR.execute(() -> {
                    workerEntered.countDown();
                    try {
                        releaseWorker.await(5, TimeUnit.SECONDS);
                    } catch (InterruptedException interrupted) {
                        Thread.currentThread().interrupt();
                    }
                });
                blockerAccepted = workerEntered.await(5, TimeUnit.SECONDS);
                if (blockerAccepted) {
                    for (int i = 0; i < QUEUE_CAPACITY; i++) {
                        CallbackBatchTask task = new CallbackBatchTask(queuedTasksFinished::countDown);
                        CALLBACK_EXECUTOR.execute(task);
                        if (task.rejected) break;
                        callbackTasksAccepted++;
                    }
                    CallbackBatchTask overflowCallback = new CallbackBatchTask(
                            () -> overflowCallbackRan.set(true));
                    CALLBACK_EXECUTOR.execute(overflowCallback);
                    overflowCallbackRejected = overflowCallback.rejected;

                    FlushTask overflowFlush = new FlushTask(null, renderer, -1, () -> -2, () -> false);
                    CALLBACK_EXECUTOR.execute(overflowFlush);
                    overflowFlushRejected = overflowFlush.rejected;
                }
            } catch (InterruptedException interrupted) {
                Thread.currentThread().interrupt();
                Log.e(TAG, "SPINON_R05_QUEUE_PROBE=INTERRUPTED renderer=" + renderer);
            } catch (RuntimeException failure) {
                Log.e(TAG, "SPINON_R05_QUEUE_PROBE=ERROR renderer=" + renderer
                        + " error=" + failure.getClass().getSimpleName());
            } finally {
                releaseWorker.countDown();
            }

            try {
                workerRecovered = callbackTasksAccepted == QUEUE_CAPACITY
                        && queuedTasksFinished.await(10, TimeUnit.SECONDS)
                        && CALLBACK_EXECUTOR.getQueue().isEmpty();
                CountDownLatch sentinelFinished = new CountDownLatch(1);
                CALLBACK_EXECUTOR.execute(sentinelFinished::countDown);
                workerRecovered &= sentinelFinished.await(5, TimeUnit.SECONDS);
            } catch (InterruptedException interrupted) {
                Thread.currentThread().interrupt();
            } catch (RuntimeException failure) {
                Log.e(TAG, "SPINON_R05_QUEUE_PROBE=RECOVERY_ERROR renderer=" + renderer
                        + " error=" + failure.getClass().getSimpleName());
            }

            long callbackRejects = REJECTED_CALLBACK_BATCHES.get() - callbackRejectsBefore;
            long flushRejects = REJECTED_FLUSH_TASKS.get() - flushRejectsBefore;
            boolean passed = blockerAccepted && callbackTasksAccepted == QUEUE_CAPACITY
                    && overflowCallbackRejected && overflowFlushRejected
                    && !overflowCallbackRan.get()
                    && callbackRejects == 1 && flushRejects == 1 && workerRecovered;
            Log.i(TAG, "SPINON_R05_QUEUE_PROBE=" + (passed ? "PASS" : "FAIL")
                    + " renderer=" + renderer + " capacity=" + QUEUE_CAPACITY
                    + " blocker_accepted=" + blockerAccepted
                    + " callback_tasks_accepted=" + callbackTasksAccepted
                    + " overflow_callback_rejected=" + overflowCallbackRejected
                    + " overflow_callback_ran=" + overflowCallbackRan.get()
                    + " overflow_flush_rejected=" + overflowFlushRejected
                    + " callback_reject_delta=" + callbackRejects
                    + " flush_reject_delta=" + flushRejects
                    + " worker_recovered=" + workerRecovered
                    + " final_queue_depth=" + CALLBACK_EXECUTOR.getQueue().size());
        }

        private static void logRecord(String renderer, SurfaceControl.JankData record,
                                      long generation, long currentGeneration,
                                      boolean surfaceAvailable, long callbackUptimeNanos,
                                      long batchSequence, int recordIndex,
                                      long registeringThreadOsTid) {
            long presentTimeNanos = record.getPresentTimeNanos();
            String presentState;
            if (presentTimeNanos == SurfaceControl.JankData.PRESENTATION_TIME_UNKNOWN) {
                presentState = "unknown";
            } else if (presentTimeNanos == SurfaceControl.JankData.PRESENTATION_TIME_UNSET) {
                presentState = "unset_or_not_presented";
            } else if (presentTimeNanos > 0) {
                presentState = "presented";
            } else {
                presentState = "invalid_negative";
            }
            boolean currentSurface = surfaceAvailable && generation == currentGeneration;
            FrameTimelineMatch match = matchFrameTimeline(
                    record.getVsyncId(), renderer, generation, currentSurface);
            Log.i(TAG, "SPINON_R05_PRESENT renderer=" + renderer
                    + " signal=SurfaceView_JankData generation=" + generation
                    + " current_generation=" + currentGeneration
                    + " current_surface=" + currentSurface
                    + " batch_seq=" + batchSequence + " record_index=" + recordIndex
                    + " callback_os_tid=" + Process.myTid()
                    + " callback_thread_id=" + Thread.currentThread().getId()
                    + " registering_thread_os_tid=" + registeringThreadOsTid
                    + " queue_depth=" + CALLBACK_EXECUTOR.getQueue().size()
                    + " vsync_id=" + record.getVsyncId()
                    + " present_time_ns=" + presentTimeNanos
                    + " present_state=" + presentState
                    + " jank_type=" + record.getJankType()
                    + " actual_app_frame_ns=" + record.getActualAppFrameTimeNanos()
                    + " scheduled_app_frame_ns=" + record.getScheduledAppFrameTimeNanos()
                    + " callback_uptime_ns=" + callbackUptimeNanos
                    + " frame_id=unavailable join_status=" + match.status
                    + " input_seq=" + match.inputSequence + " revision=" + match.revision
                    + " request_id=" + match.requestId
                    + " target_vsync_id=" + match.targetVsyncId);
        }
    }

    private static final class FrameTimelineTarget {
        final long vsyncId;
        final String renderer;
        final long generation;
        final long inputSequence;
        final long revision;
        final long requestId;
        int recordCount;

        FrameTimelineTarget(long vsyncId, String renderer, long generation, long inputSequence,
                            long revision, long requestId) {
            this.vsyncId = vsyncId;
            this.renderer = renderer;
            this.generation = generation;
            this.inputSequence = inputSequence;
            this.revision = revision;
            this.requestId = requestId;
        }
    }

    private static final class FrameTimelineMatch {
        final String status;
        final long inputSequence;
        final long revision;
        final long requestId;
        final long targetVsyncId;

        FrameTimelineMatch(String status, long inputSequence, long revision,
                           long requestId, long targetVsyncId) {
            this.status = status;
            this.inputSequence = inputSequence;
            this.revision = revision;
            this.requestId = requestId;
            this.targetVsyncId = targetVsyncId;
        }

        static FrameTimelineMatch unmatched() {
            return new FrameTimelineMatch("unmatched", -1, -1, -1, -1);
        }
    }

    private static final class CallbackBatchTask implements Runnable {
        private final Runnable command;
        private volatile boolean rejected;

        CallbackBatchTask(Runnable command) {
            this.command = command;
        }

        @Override
        public void run() {
            command.run();
        }
    }

    private static final class FlushTask implements Runnable {
        private final Object registration;
        private final String renderer;
        private final long generation;
        private final LongSupplier currentGeneration;
        private final BooleanSupplier surfaceAvailable;
        private volatile boolean rejected;

        FlushTask(Object registration, String renderer, long generation,
                  LongSupplier currentGeneration, BooleanSupplier surfaceAvailable) {
            this.registration = registration;
            this.renderer = renderer;
            this.generation = generation;
            this.currentGeneration = currentGeneration;
            this.surfaceAvailable = surfaceAvailable;
        }

        @Override
        public void run() {
            long current = currentGeneration.getAsLong();
            if (registration == null || !surfaceAvailable.getAsBoolean()
                    || generation != current) {
                Log.i(TAG, "SPINON_R05_PRESENT_FLUSH skipped=stale_surface"
                        + " renderer=" + renderer + " generation=" + generation
                        + " current_generation=" + current);
                return;
            }
            try {
                ((SurfaceControl.OnJankDataListenerRegistration) registration).flush();
                Log.i(TAG, "SPINON_R05_PRESENT_FLUSH requested=true"
                        + " renderer=" + renderer + " generation=" + generation);
            } catch (RuntimeException | LinkageError error) {
                Log.w(TAG, "SPINON_R05_PRESENT_FLUSH requested=false"
                        + " renderer=" + renderer + " generation=" + generation
                        + " error=" + error.getClass().getSimpleName());
            }
        }
    }

    private static final class ProbeThreadFactory implements ThreadFactory {
        @Override
        public Thread newThread(Runnable task) {
            Thread thread = new Thread(task, "SpinonR05PresentSignal");
            thread.setDaemon(true);
            return thread;
        }
    }
}
