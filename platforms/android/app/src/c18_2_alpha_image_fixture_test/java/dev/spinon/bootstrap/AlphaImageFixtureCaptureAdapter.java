package dev.spinon.bootstrap;

import android.app.Activity;
import android.graphics.Point;
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

public final class AlphaImageFixtureCaptureAdapter {
    private static final String TAG = "SpinonC0410";
    private static final String[] SAMPLE_IDS = {
            "background-anchor", "transparent-interior", "transparent-right-edge",
            "partial-left-edge", "partial-interior", "partial-right-edge",
            "opaque-left-edge", "opaque-interior", "opaque-right-edge", "after-image-anchor",
    };
    private static final int[][] SAMPLE_CSS = {
            {2, 2}, {12, 20}, {15, 20}, {16, 20}, {20, 20},
            {23, 20}, {24, 20}, {28, 20}, {31, 20}, {34, 20},
    };

    private AlphaImageFixtureCaptureAdapter() {}

    public static void request(View contentView, SurfaceView surfaceView, Activity activity,
            int width, int height, long generation, Handler mainHandler, Executor runtimeQueue,
            BooleanSupplier sourceIsCurrent, Consumer<String> status) {
        String runId = Long.toHexString(android.os.SystemClock.elapsedRealtimeNanos());
        List<SurfaceViewPixelCapture.Sample> samples = new ArrayList<>(SAMPLE_IDS.length);
        for (int index = 0; index < SAMPLE_IDS.length; index++) {
            samples.add(new SurfaceViewPixelCapture.Sample(
                    SAMPLE_IDS[index], SAMPLE_CSS[index][0], SAMPLE_CSS[index][1]));
        }
        File output = new File(new File(activity.getCacheDir(), "c182-alpha-image-captures"),
                "alpha-source-over-" + runId + ".png");
        Log.i(TAG, "SPINON_C182_PIXELCOPY_BEGIN run_id=" + runId + " generation=" + generation
                + " surface=" + width + "x" + height + " viewport=80x40 samples="
                + samples.size());
        logScreenGeometry(contentView, surfaceView, activity, mainHandler, sourceIsCurrent,
                runId, width, height, "before_pixelcopy", 0);
        SurfaceViewPixelCapture.requestForCssViewport(surfaceView, width, height, 80, 40,
                0, 0, "c182-alpha-source-over", runId, output, samples,
                mainHandler, runtimeQueue, sourceIsCurrent, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C182_PIXELCOPY status=error run_id=" + runId
                                + " reason=" + result.error);
                        status.accept("실패 · C18.2 PixelCopy · " + result.error);
                        return;
                    }
                    int matched = 0;
                    for (SurfaceViewPixelCapture.Pixel pixel : result.pixels) {
                        int[] expected = expectedRgba(pixel.id);
                        boolean exact = pixel.red == expected[0] && pixel.green == expected[1]
                                && pixel.blue == expected[2] && pixel.alpha == expected[3];
                        if (exact) matched++;
                        Log.i(TAG, "SPINON_C182_PIXELCOPY_SAMPLE id=" + pixel.id
                                + " xy=" + pixel.x + "," + pixel.y + " rgba="
                                + pixel.red + "," + pixel.green + "," + pixel.blue + ","
                                + pixel.alpha + " exact=" + exact);
                    }
                    boolean passed = result.pixels.size() == SAMPLE_IDS.length
                            && matched == SAMPLE_IDS.length;
                    Log.i(TAG, "SPINON_C182_PIXELCOPY_SUMMARY status="
                            + (passed ? "ok" : "error") + " matched=" + matched
                            + " total=" + result.pixels.size() + " path=" + output.getAbsolutePath()
                            + " sha256=" + result.sha256);
                    if (passed) {
                        status.accept("완료 · C18.2 drawable 10/10 · PixelCopy 10/10");
                        logScreenGeometry(contentView, surfaceView, activity, mainHandler,
                                sourceIsCurrent, runId, width, height,
                                "post_pixelcopy_status", 150);
                    } else {
                        status.accept("실패 · C18.2 PixelCopy 표본 불일치");
                    }
                });
    }

    private static void logScreenGeometry(View contentView, SurfaceView surfaceView,
            Activity activity, Handler mainHandler, BooleanSupplier sourceIsCurrent,
            String runId, int width, int height, String phase, long delayMillis) {
        mainHandler.postDelayed(() -> {
            if (!sourceIsCurrent.getAsBoolean()) return;
            int[] contentOrigin = new int[2];
            int[] surfaceOrigin = new int[2];
            Point displaySize = new Point();
            activity.getWindowManager().getDefaultDisplay().getRealSize(displaySize);
            contentView.getLocationOnScreen(contentOrigin);
            surfaceView.getLocationOnScreen(surfaceOrigin);
            Log.i(TAG, "SPINON_C182_SCREEN_GEOMETRY run_id=" + runId + " phase=" + phase
                    + " screenshot_px=" + displaySize.x + "x" + displaySize.y
                    + " screenshot_scale=1.0"
                    + " content_view_origin_screen_px=" + contentOrigin[0] + "," + contentOrigin[1]
                    + " content_view_size_px=" + contentView.getWidth() + "x"
                    + contentView.getHeight()
                    + " surface_view_origin_screen_px=" + surfaceOrigin[0] + "," + surfaceOrigin[1]
                    + " surface_view_size_px=" + surfaceView.getWidth() + "x"
                    + surfaceView.getHeight()
                    + " surface_texture_px=" + width + "x" + height
                    + " viewport_css_px=80x40 surface_scale=" + ((double) width / 80.0)
                    + "," + ((double) height / 40.0));
        }, delayMillis);
    }

    private static int[] expectedRgba(String sampleId) {
        if ("transparent-interior".equals(sampleId)
                || "transparent-right-edge".equals(sampleId)
                || "background-anchor".equals(sampleId)
                || "after-image-anchor".equals(sampleId)) {
            return new int[] {17, 34, 51, 255};
        }
        if ("partial-left-edge".equals(sampleId)
                || "partial-interior".equals(sampleId)
                || "partial-right-edge".equals(sampleId)) {
            return new int[] {118, 37, 65, 255};
        }
        return new int[] {32, 200, 64, 255};
    }
}
