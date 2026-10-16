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

public final class ObjectFitPositionFixtureCaptureAdapter {
    private static final String TAG = "SpinonC0410";
    private static final String[] SAMPLE_IDS = {
            "root-background-anchor", "fill-top-left", "fill-bottom-right",
            "contain-background-top", "contain-image-top", "contain-image-bottom",
            "contain-quarter-image", "cover-center-left", "cover-center-right",
            "cover-start-left", "cover-end-left", "none-clipped-object",
            "none-outside-content-box", "scale-down-shrink-background",
            "scale-down-shrink-image", "scale-down-no-upscale-image",
            "initial-values-natural", "contain-extended-position-image",
            "cover-extended-position-image",
    };
    private static final int[][] SAMPLE_CSS = {
            {2, 2}, {9, 9}, {30, 30}, {42, 9}, {42, 15}, {42, 25},
            {75, 18}, {105, 12}, {126, 12}, {137, 12}, {169, 12},
            {10, 45}, {25, 45}, {42, 41}, {45, 46}, {82, 47},
            {113, 41}, {145, 60}, {177, 44},
    };
    // Chrome 155 samples at the API 37 ARM64 AVD's device-scale factor 2.625.
    private static final int[][] EXPECTED_RGBA = {
            {17, 34, 51, 255}, {24, 32, 232, 255}, {209, 214, 51, 255},
            {210, 10, 180, 255}, {24, 32, 232, 255}, {24, 214, 206, 255},
            {28, 32, 228, 255}, {61, 32, 201, 255}, {172, 32, 108, 255},
            {24, 32, 232, 255}, {135, 32, 139, 255}, {98, 32, 170, 255},
            {17, 34, 51, 255}, {210, 10, 180, 255}, {90, 78, 170, 255},
            {61, 32, 201, 255}, {24, 32, 232, 255}, {24, 32, 232, 255},
            {172, 32, 108, 255},
    };

    private ObjectFitPositionFixtureCaptureAdapter() {}

    public static void request(View contentView, SurfaceView surfaceView, Activity activity,
            int width, int height, long generation, Handler mainHandler, Executor runtimeQueue,
            BooleanSupplier sourceIsCurrent, Consumer<String> status) {
        String runId = Long.toHexString(android.os.SystemClock.elapsedRealtimeNanos());
        List<SurfaceViewPixelCapture.Sample> samples = new ArrayList<>(SAMPLE_IDS.length);
        for (int index = 0; index < SAMPLE_IDS.length; index++) {
            samples.add(new SurfaceViewPixelCapture.Sample(
                    SAMPLE_IDS[index], SAMPLE_CSS[index][0], SAMPLE_CSS[index][1]));
        }
        File output = new File(new File(activity.getCacheDir(), "c183-object-fit-captures"),
                "object-fit-position-" + runId + ".png");
        Log.i(TAG, "SPINON_C183_PIXELCOPY_BEGIN run_id=" + runId + " generation=" + generation
                + " surface=" + width + "x" + height + " viewport=224x72 samples="
                + samples.size());
        SurfaceViewPixelCapture.requestForCssViewport(surfaceView, width, height, 224, 72,
                0, 0, "c183-object-fit-position", runId, output, samples,
                mainHandler, runtimeQueue, sourceIsCurrent, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C183_PIXELCOPY status=error run_id=" + runId
                                + " reason=" + result.error);
                        status.accept("실패 · C18.3 PixelCopy · " + result.error);
                        return;
                    }
                    int matched = 0;
                    for (int index = 0; index < result.pixels.size(); index++) {
                        SurfaceViewPixelCapture.Pixel pixel = result.pixels.get(index);
                        int[] expected = EXPECTED_RGBA[index];
                        int expectedX = (int) Math.floor(
                                (SAMPLE_CSS[index][0] + 0.5) * width / 224.0);
                        int expectedY = (int) Math.floor(
                                (SAMPLE_CSS[index][1] + 0.5) * height / 72.0);
                        boolean exact = pixel.id.equals(SAMPLE_IDS[index])
                                && pixel.x == expectedX
                                && pixel.y == expectedY
                                && pixel.red == expected[0] && pixel.green == expected[1]
                                && pixel.blue == expected[2] && pixel.alpha == expected[3];
                        if (exact) matched++;
                        Log.i(TAG, "SPINON_C183_PIXELCOPY_SAMPLE id=" + pixel.id
                                + " xy=" + pixel.x + "," + pixel.y + " rgba="
                                + pixel.red + "," + pixel.green + "," + pixel.blue + ","
                                + pixel.alpha + " expected=" + expected[0] + "," + expected[1]
                                + "," + expected[2] + "," + expected[3] + " exact=" + exact);
                    }
                    boolean passed = result.pixels.size() == SAMPLE_IDS.length
                            && matched == SAMPLE_IDS.length;
                    Log.i(TAG, "SPINON_C183_PIXELCOPY_SUMMARY status="
                            + (passed ? "ok" : "error") + " matched=" + matched
                            + " total=" + result.pixels.size() + " path=" + output.getAbsolutePath()
                            + " sha256=" + result.sha256);
                    status.accept(passed
                            ? "완료 · C18.3 drawable 19/19 · PixelCopy 19/19"
                            : "실패 · C18.3 PixelCopy 표본 불일치");
                });
    }
}
