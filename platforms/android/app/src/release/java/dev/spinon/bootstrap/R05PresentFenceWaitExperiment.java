package dev.spinon.bootstrap;

import android.util.Log;

/** release variant에서는 Android R05 async fence wait 실험을 실행하지 않습니다. */
final class R05PresentFenceWaitExperiment {
    private R05PresentFenceWaitExperiment() {}

    static boolean enable() {
        Log.e("SpinonBootstrap", "SPINON_R05_FENCE_WAIT status=UNAVAILABLE reason=debug_only");
        return false;
    }

    static boolean isEnabled() {
        return false;
    }

    static void disable() {}

    static void observe(Object fence, String renderer, long requestId, long inputSequence,
                        long revision, long generation, long currentGeneration,
                        boolean currentSurface, long targetVsyncId, long eventTimeNanos,
                        long latchTimeNanos, String initialFenceState,
                        long initialSignalTimeNanos, boolean callbackInlineOverflow) {}
}
