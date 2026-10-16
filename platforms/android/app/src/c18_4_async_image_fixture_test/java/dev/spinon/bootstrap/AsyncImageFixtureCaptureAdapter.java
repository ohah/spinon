package dev.spinon.bootstrap;

import android.app.Activity;
import android.os.Handler;
import android.util.Log;
import android.view.SurfaceView;
import android.view.View;

import java.io.File;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.Executor;
import java.util.function.BooleanSupplier;
import java.util.function.Consumer;

public final class AsyncImageFixtureCaptureAdapter {
    private static final String TAG = "SpinonC0410";
    private static final String[] SAMPLE_IDS = {
            "root-background", "current-b", "pending-c-background",
            "c-success", "decode-failure-background", "recovery-d",
    };
    private static final int[][] SAMPLE_CSS = {
            {2, 2}, {9, 9}, {65, 9}, {121, 9}, {177, 9}, {9, 49},
    };
    private static final int[][] EXPECTED_RGBA = {
            {17, 34, 51, 255}, {24, 64, 240, 255}, {210, 10, 180, 255},
            {32, 220, 80, 255}, {210, 10, 180, 255}, {200, 40, 220, 255},
    };

    private AsyncImageFixtureCaptureAdapter() {}

    public static void request(View contentView, SurfaceView surfaceView, Activity activity,
            int width, int height, long generation, Handler mainHandler, Executor runtimeQueue,
            BooleanSupplier sourceIsCurrent, Consumer<String> status) {
        String runId = Long.toHexString(android.os.SystemClock.elapsedRealtimeNanos());
        List<SurfaceViewPixelCapture.Sample> samples = new ArrayList<>(SAMPLE_IDS.length);
        for (int index = 0; index < SAMPLE_IDS.length; index++) {
            samples.add(new SurfaceViewPixelCapture.Sample(
                    SAMPLE_IDS[index], SAMPLE_CSS[index][0], SAMPLE_CSS[index][1]));
        }
        File output = new File(new File(activity.getCacheDir(), "c184-async-image-captures"),
                "async-image-replacement-" + runId + ".png");
        Log.i(TAG, "SPINON_C184_PIXELCOPY_BEGIN run_id=" + runId + " generation=" + generation
                + " surface=" + width + "x" + height + " viewport=224x88 samples="
                + samples.size());
        SurfaceViewPixelCapture.requestForCssViewport(surfaceView, width, height, 224, 88,
                0, 0, "c184-async-image-replacement", runId, output, samples,
                mainHandler, runtimeQueue, sourceIsCurrent, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C184_PIXELCOPY status=error run_id=" + runId
                                + " reason=" + result.error);
                        status.accept("실패 · C18.4 PixelCopy · " + result.error);
                        return;
                    }
                    int matched = 0;
                    for (int index = 0; index < result.pixels.size(); index++) {
                        SurfaceViewPixelCapture.Pixel pixel = result.pixels.get(index);
                        int[] expected = EXPECTED_RGBA[index];
                        int expectedX = (int) Math.floor(
                                (SAMPLE_CSS[index][0] + 0.5) * width / 224.0);
                        int expectedY = (int) Math.floor(
                                (SAMPLE_CSS[index][1] + 0.5) * height / 88.0);
                        boolean exact = pixel.id.equals(SAMPLE_IDS[index])
                                && pixel.x == expectedX && pixel.y == expectedY
                                && pixel.red == expected[0] && pixel.green == expected[1]
                                && pixel.blue == expected[2] && pixel.alpha == expected[3];
                        if (exact) matched++;
                        Log.i(TAG, "SPINON_C184_PIXELCOPY_SAMPLE id=" + pixel.id
                                + " xy=" + pixel.x + "," + pixel.y + " rgba="
                                + pixel.red + "," + pixel.green + "," + pixel.blue + ","
                                + pixel.alpha + " expected=" + expected[0] + "," + expected[1]
                                + "," + expected[2] + "," + expected[3] + " exact=" + exact);
                    }
                    boolean passed = result.pixels.size() == SAMPLE_IDS.length
                            && matched == SAMPLE_IDS.length;
                    Log.i(TAG, "SPINON_C184_PIXELCOPY_SUMMARY status="
                            + (passed ? "ok" : "error") + " matched=" + matched
                            + " total=" + result.pixels.size() + " path=" + output.getAbsolutePath()
                            + " sha256=" + result.sha256);
                    status.accept(passed
                            ? "완료 · C18.4 async replacement · PixelCopy 6/6"
                            : "실패 · C18.4 PixelCopy 표본 불일치");
                });
    }
}
