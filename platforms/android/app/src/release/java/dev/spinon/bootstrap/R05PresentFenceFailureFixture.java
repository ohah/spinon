package dev.spinon.bootstrap;

import android.app.Activity;
import android.util.Log;

/** release variant에서는 callback fault fixture를 실행하지 않습니다. */
final class R05PresentFenceFailureFixture {
    private R05PresentFenceFailureFixture() {}

    static void start(Activity activity) {
        Log.e("SpinonBootstrap", "SPINON_R05_CALLBACK_FAULT_SUMMARY"
                + " status=UNAVAILABLE reason=debug_only");
        activity.finish();
    }
}
