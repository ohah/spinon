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

public final class ImagePaintFixtureCaptureAdapter {
    private static final String TAG = "SpinonC0410";
    private static final String[] SAMPLE_IDS = {
            "visible-content-positive", "visible-border-overpaint", "visible-overflow-first",
            "visible-overflow-middle", "visible-overflow-last", "visible-after-image",
            "hidden-padding-positive", "hidden-border-positive", "hidden-outside",
    };
    private static final int[][] SAMPLE_CSS = {
            {9, 9}, {25, 10}, {26, 10}, {29, 10}, {32, 10},
            {33, 10}, {60, 10}, {61, 10}, {62, 10},
    };

    private ImagePaintFixtureCaptureAdapter() {}

    public static void request(View contentView, SurfaceView surfaceView, Activity activity,
            int width, int height, long generation, Handler mainHandler, Executor runtimeQueue,
            BooleanSupplier sourceIsCurrent, Consumer<String> status) {
        String runId = Long.toHexString(android.os.SystemClock.elapsedRealtimeNanos());
        List<SurfaceViewPixelCapture.Sample> samples = new ArrayList<>(SAMPLE_IDS.length);
        for (int index = 0; index < SAMPLE_IDS.length; index++) {
            samples.add(new SurfaceViewPixelCapture.Sample(
                    SAMPLE_IDS[index], SAMPLE_CSS[index][0], SAMPLE_CSS[index][1]));
        }
        File output = new File(new File(activity.getCacheDir(), "c181-image-paint-captures"),
                "image-overflow-" + runId + ".png");
        Log.i(TAG, "SPINON_C181_PIXELCOPY_BEGIN run_id=" + runId + " generation=" + generation
                + " surface=" + width + "x" + height + " viewport=80x40 samples="
                + samples.size());
        logScreenGeometry(contentView, surfaceView, activity, mainHandler, sourceIsCurrent,
                runId, width, height, "before_pixelcopy", 0);
        SurfaceViewPixelCapture.requestForCssViewport(surfaceView, width, height, 80, 40,
                0, 0, "c181-image-overflow", runId, output, samples,
                mainHandler, runtimeQueue, sourceIsCurrent, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C181_PIXELCOPY status=error run_id=" + runId
                                + " reason=" + result.error);
                        status.accept("실패 · C18.1 PixelCopy · " + result.error);
                        return;
                    }
                    int matched = 0;
                    for (SurfaceViewPixelCapture.Pixel pixel : result.pixels) {
                        int[] expected = expectedRgba(pixel.id);
                        boolean exact = pixel.red == expected[0] && pixel.green == expected[1]
                                && pixel.blue == expected[2] && pixel.alpha == expected[3];
                        if (exact) matched++;
                        Log.i(TAG, "SPINON_C181_PIXELCOPY_SAMPLE id=" + pixel.id
                                + " xy=" + pixel.x + "," + pixel.y + " rgba="
                                + pixel.red + "," + pixel.green + "," + pixel.blue + ","
                                + pixel.alpha + " exact=" + exact);
                    }
                    boolean passed = result.pixels.size() == SAMPLE_IDS.length
                            && matched == SAMPLE_IDS.length;
                    Log.i(TAG, "SPINON_C181_PIXELCOPY_SUMMARY status="
                            + (passed ? "ok" : "error") + " matched=" + matched
                            + " total=" + result.pixels.size() + " path=" + output.getAbsolutePath()
                            + " sha256=" + result.sha256);
                    if (passed) {
                        status.accept("완료 · C18.1 drawable 9/9 · PixelCopy 9/9");
                        logScreenGeometry(contentView, surfaceView, activity, mainHandler,
                                sourceIsCurrent, runId, width, height,
                                "post_pixelcopy_status", 150);
                    } else {
                        status.accept("실패 · C18.1 PixelCopy 표본 불일치");
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
            Log.i(TAG, "SPINON_C181_SCREEN_GEOMETRY run_id=" + runId + " phase=" + phase
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
        if ("visible-content-positive".equals(sampleId)) return new int[] {255, 255, 64, 255};
        if ("visible-border-overpaint".equals(sampleId)
                || "visible-overflow-first".equals(sampleId)
                || "visible-overflow-middle".equals(sampleId)
                || "visible-overflow-last".equals(sampleId)
                || "hidden-padding-positive".equals(sampleId)) {
            return new int[] {0, 255, 64, 255};
        }
        if ("hidden-border-positive".equals(sampleId)) return new int[] {240, 198, 116, 255};
        return new int[] {16, 24, 39, 255};
    }
}
