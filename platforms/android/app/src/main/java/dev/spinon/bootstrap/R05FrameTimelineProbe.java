package dev.spinon.bootstrap;

import android.os.Build;
import android.os.Looper;
import android.util.Log;
import android.view.SurfaceView;

import java.util.function.BooleanSupplier;
import java.util.function.LongSupplier;

/** 입력 revision에 VSync token을 다음 idle SurfaceView frame에 붙이는 내부 실험. */
final class R05FrameTimelineProbe {
    private static final String TAG = "SpinonBootstrap";
    private static final int MIN_API = 35;

    private R05FrameTimelineProbe() {}

    static void submitNextFrame(SurfaceView surface, String renderer, long generation,
                                LongSupplier currentGeneration,
                                BooleanSupplier surfaceAvailable, long inputSequence,
                                long revision, Runnable submit) {
        submitNextFrame(surface, renderer, generation, currentGeneration, surfaceAvailable,
                inputSequence, revision, false, submit);
    }

    static void submitNextFrame(SurfaceView surface, String renderer, long generation,
                                LongSupplier currentGeneration,
                                BooleanSupplier surfaceAvailable, long inputSequence,
                                long revision, boolean capturePresentFence, Runnable submit) {
        submitNextFrame(surface, renderer, generation, currentGeneration, surfaceAvailable,
                inputSequence, revision, 0, 0, 0, 0, capturePresentFence, submit);
    }

    static void submitNextFrame(SurfaceView surface, String renderer, long generation,
                                LongSupplier currentGeneration,
                                BooleanSupplier surfaceAvailable, long inputSequence,
                                long revision, long eventTimeNanos,
                                long inputUptimeAnchorNanos,
                                long inputMonotonicBeforeNanos,
                                long inputMonotonicAfterNanos,
                                boolean capturePresentFence, Runnable submit) {
        if (Build.VERSION.SDK_INT < MIN_API) {
            logUnavailable(renderer, generation, inputSequence, revision, "api_below_35");
            submit.run();
            return;
        }
        if (!R05PresentTimingProbe.isFrameTimelineJoinAvailable()) {
            if (capturePresentFence) {
                R05PresentFenceProbe.submitNextFrame(surface, renderer, generation,
                        currentGeneration, surfaceAvailable, inputSequence, revision,
                        eventTimeNanos, inputUptimeAnchorNanos,
                        inputMonotonicBeforeNanos, inputMonotonicAfterNanos, submit);
                return;
            }
            logUnavailable(renderer, generation, inputSequence, revision,
                    "present_signal_api_unavailable");
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
                    "stale_surface_before_vsync");
            submit.run();
            return;
        }
        Api35.submit(surface, renderer, generation, currentGeneration, surfaceAvailable,
                inputSequence, revision, eventTimeNanos, inputUptimeAnchorNanos,
                inputMonotonicBeforeNanos, inputMonotonicAfterNanos,
                capturePresentFence, submit);
    }

    static void cancelForSurface(String renderer, long generation, String reason) {
        if (Build.VERSION.SDK_INT < MIN_API
                || !R05PresentTimingProbe.isFrameTimelineJoinAvailable()) return;
        if (Looper.myLooper() != Looper.getMainLooper()) {
            Log.w(TAG, "SPINON_R05_FRAME_TIMELINE_CANCEL renderer=" + renderer
                    + " generation=" + generation + " outcome=wrong_thread");
            return;
        }
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

    private static void logUnavailable(String renderer, long generation, long inputSequence,
                                       long revision, String reason) {
        Log.w(TAG, "SPINON_R05_FRAME_TIMELINE renderer=" + renderer
                + " input_seq=" + inputSequence + " revision=" + revision
                + " generation=" + generation + " outcome=unmatched reason=" + reason);
    }

    /** API 33/35 이상 유형 참조를 구버전 class verifier에서 분리합니다. */
    private static final class Api35 {
        private static final int MAX_PENDING_REQUESTS = 64;
        private static final Object PENDING_LOCK = new Object();
        private static final java.util.LinkedHashMap<Long, PendingRequest> PENDING_REQUESTS =
                new java.util.LinkedHashMap<>();
        private static long nextRequestId = 1;

        private Api35() {}

        static void submit(SurfaceView surface, String renderer, long generation,
                           LongSupplier currentGeneration, BooleanSupplier surfaceAvailable,
                           long inputSequence, long revision, long eventTimeNanos,
                           long inputUptimeAnchorNanos,
                           long inputMonotonicBeforeNanos, long inputMonotonicAfterNanos,
                           boolean capturePresentFence, Runnable submit) {
            long requestId;
            synchronized (PENDING_LOCK) {
                if (PENDING_REQUESTS.size() >= MAX_PENDING_REQUESTS
                        || nextRequestId <= 0 || nextRequestId == Long.MAX_VALUE) {
                    requestId = -1;
                } else {
                    requestId = nextRequestId++;
                }
            }
            if (requestId < 0) {
                logUnavailable(renderer, generation, inputSequence, revision,
                        "pending_limit_or_request_id_exhausted");
                submit.run();
                return;
            }

            PendingRequest request = new PendingRequest(requestId, surface, renderer, generation,
                    currentGeneration, surfaceAvailable, inputSequence, revision,
                    eventTimeNanos, inputUptimeAnchorNanos, inputMonotonicBeforeNanos,
                    inputMonotonicAfterNanos, capturePresentFence, submit);
            request.callback = frameData -> onVsync(request, frameData);
            synchronized (PENDING_LOCK) {
                PENDING_REQUESTS.put(requestId, request);
            }
            try {
                android.view.Choreographer.getInstance().postVsyncCallback(request.callback);
                Log.i(TAG, "SPINON_R05_FRAME_TIMELINE_REQUEST renderer=" + renderer
                        + " request_id=" + requestId + " input_seq=" + inputSequence
                        + " revision=" + revision + " generation=" + generation
                        + " state=posted");
            } catch (RuntimeException | LinkageError error) {
                removePending(request);
                logUnavailable(renderer, generation, inputSequence, revision,
                        "post_vsync_failed_" + error.getClass().getSimpleName());
                submit.run();
            }
        }

        static void cancelForSurface(String renderer, long generation, String reason) {
            java.util.ArrayList<PendingRequest> cancelled = new java.util.ArrayList<>();
            synchronized (PENDING_LOCK) {
                PENDING_REQUESTS.values().removeIf(request -> {
                    boolean matches = request.renderer.equals(renderer)
                            && request.generation == generation;
                    if (matches) cancelled.add(request);
                    return matches;
                });
            }
            android.view.Choreographer choreographer =
                    android.view.Choreographer.getInstance();
            for (PendingRequest request : cancelled) {
                try {
                    choreographer.removeVsyncCallback(request.callback);
                    Log.i(TAG, "SPINON_R05_FRAME_TIMELINE_CANCEL renderer=" + renderer
                            + " request_id=" + request.requestId
                            + " input_seq=" + request.inputSequence + " revision=" + request.revision
                            + " generation=" + generation + " reason=" + reason);
                } catch (RuntimeException | LinkageError error) {
                    Log.w(TAG, "SPINON_R05_FRAME_TIMELINE_CANCEL renderer=" + renderer
                            + " request_id=" + request.requestId + " generation=" + generation
                            + " outcome=remove_failed error=" + error.getClass().getSimpleName());
                }
            }
        }

        private static void onVsync(PendingRequest request,
                                    android.view.Choreographer.FrameData frameData) {
            removePending(request);
            if (!isCurrentSurface(request.surface, request.generation,
                    request.currentGeneration, request.surfaceAvailable)) {
                logUnavailable(request.renderer, request.generation, request.inputSequence,
                        request.revision, "stale_surface_at_vsync");
                return;
            }
            if (frameData == null) {
                logUnavailable(request.renderer, request.generation, request.inputSequence,
                        request.revision, "null_frame_data");
                request.submit.run();
                return;
            }

            long vsyncId;
            long deadlineNanos;
            long expectedPresentationNanos;
            android.view.Choreographer.FrameTimeline timeline;
            try {
                timeline = frameData.getPreferredFrameTimeline();
            } catch (RuntimeException | LinkageError error) {
                logUnavailable(request.renderer, request.generation, request.inputSequence,
                        request.revision, "timeline_read_failed_" + error.getClass().getSimpleName());
                request.submit.run();
                return;
            }
            if (timeline == null) {
                logUnavailable(request.renderer, request.generation, request.inputSequence,
                        request.revision, "missing_preferred_timeline");
                request.submit.run();
                return;
            }
            try {
                vsyncId = timeline.getVsyncId();
                deadlineNanos = timeline.getDeadlineNanos();
                expectedPresentationNanos = timeline.getExpectedPresentationTimeNanos();
            } catch (RuntimeException | LinkageError error) {
                logUnavailable(request.renderer, request.generation, request.inputSequence,
                        request.revision, "timeline_read_failed_" + error.getClass().getSimpleName());
                request.submit.run();
                return;
            }
            if (vsyncId == -1L) {
                logUnavailable(request.renderer, request.generation, request.inputSequence,
                        request.revision, "invalid_vsync_id_sentinel");
                request.submit.run();
                return;
            }
            if (!R05PresentTimingProbe.registerFrameTimelineTarget(vsyncId,
                    request.renderer, request.generation, request.inputSequence,
                    request.revision, request.requestId)) {
                logUnavailable(request.renderer, request.generation, request.inputSequence,
                        request.revision, "target_registry_rejected");
                request.submit.run();
                return;
            }

            Log.i(TAG, "SPINON_R05_FRAME_TIMELINE_TARGET renderer=" + request.renderer
                    + " request_id=" + request.requestId + " input_seq=" + request.inputSequence
                    + " revision=" + request.revision + " generation=" + request.generation
                    + " vsync_id=" + vsyncId + " deadline_ns=" + deadlineNanos
                    + " expected_presentation_ns=" + expectedPresentationNanos
                    + " render_mode=on_demand");

            android.view.SurfaceControl.Transaction transaction;
            try {
                transaction = new android.view.SurfaceControl.Transaction();
            } catch (RuntimeException | LinkageError error) {
                R05PresentTimingProbe.removeFrameTimelineTarget(vsyncId, request.requestId);
                logUnavailable(request.renderer, request.generation, request.inputSequence,
                        request.revision, "transaction_create_failed_"
                                + error.getClass().getSimpleName());
                request.submit.run();
                return;
            }
            boolean transactionHandedToSurfaceView = false;
            try {
                if (request.capturePresentFence) {
                    R05PresentFenceProbe.attach(transaction, request.requestId,
                            request.renderer, request.generation, request.currentGeneration,
                            request.surfaceAvailable, request.inputSequence, request.revision,
                            request.eventTimeNanos, request.inputUptimeAnchorNanos,
                            request.inputMonotonicBeforeNanos,
                            request.inputMonotonicAfterNanos, vsyncId);
                }
                transaction.setFrameTimeline(vsyncId);
                request.surface.applyTransactionToFrame(transaction);
                transactionHandedToSurfaceView = true;
                Log.i(TAG, "SPINON_R05_FRAME_TIMELINE_APPLY renderer=" + request.renderer
                        + " request_id=" + request.requestId + " input_seq=" + request.inputSequence
                        + " revision=" + request.revision + " generation=" + request.generation
                        + " target_vsync_id=" + vsyncId
                        + " transaction=queued_for_next_surface_frame");
            } catch (RuntimeException | LinkageError error) {
                if (!transactionHandedToSurfaceView) {
                    try {
                        transaction.close();
                    } catch (RuntimeException | LinkageError closeError) {
                        Log.w(TAG, "SPINON_R05_FRAME_TIMELINE_TRANSACTION_CLOSE outcome=failed"
                                + " error=" + closeError.getClass().getSimpleName());
                    }
                }
                if (request.capturePresentFence) {
                    R05PresentFenceProbe.cancel(request.requestId, request.renderer,
                            request.generation, "frame_timeline_apply_failed");
                }
                R05PresentTimingProbe.removeFrameTimelineTarget(vsyncId, request.requestId);
                logUnavailable(request.renderer, request.generation, request.inputSequence,
                        request.revision, "apply_failed_" + error.getClass().getSimpleName());
            }
            request.submit.run();
        }

        private static void removePending(PendingRequest request) {
            synchronized (PENDING_LOCK) {
                PENDING_REQUESTS.remove(request.requestId, request);
            }
        }

        private static final class PendingRequest {
            final long requestId;
            final SurfaceView surface;
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
            final boolean capturePresentFence;
            final Runnable submit;
            android.view.Choreographer.VsyncCallback callback;

            PendingRequest(long requestId, SurfaceView surface, String renderer, long generation,
                           LongSupplier currentGeneration, BooleanSupplier surfaceAvailable,
                           long inputSequence, long revision, long eventTimeNanos,
                           long inputUptimeAnchorNanos, long inputMonotonicBeforeNanos,
                           long inputMonotonicAfterNanos, boolean capturePresentFence,
                           Runnable submit) {
                this.requestId = requestId;
                this.surface = surface;
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
                this.capturePresentFence = capturePresentFence;
                this.submit = submit;
            }
        }
    }
}
