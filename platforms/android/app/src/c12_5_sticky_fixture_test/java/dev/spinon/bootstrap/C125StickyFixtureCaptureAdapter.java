package dev.spinon.bootstrap;

import android.app.Activity;
import android.os.Handler;
import android.util.Log;
import android.view.SurfaceView;

import java.io.File;
import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.function.BooleanSupplier;
import java.util.function.Consumer;

/** C12.5 고정 sticky 색상 표본을 Android SurfaceView PixelCopy로 검사합니다. */
final class C125StickyFixtureCaptureAdapter {
    private static final String TAG = "SpinonBootstrap";
    private static final int VIEWPORT_WIDTH = 360;
    private static final int VIEWPORT_HEIGHT = 260;
    private static final List<SurfaceViewPixelCapture.Sample> SAMPLES = List.of(
            new SurfaceViewPixelCapture.Sample("section-at-content-edge", 200, 78),
            new SurfaceViewPixelCapture.Sample("section-after", 240, 100));

    private C125StickyFixtureCaptureAdapter() {}

    static void request(
            SurfaceView surfaceView,
            Activity activity,
            int width,
            int height,
            String runId,
            String drawReport,
            long generation,
            long presentationSequence,
            Handler mainHandler,
            ExecutorService runtimeQueue,
            BooleanSupplier sourceIsCurrent,
            Consumer<String> status
    ) {
        String drawGeneration = field(drawReport, "generation=");
        String scrollRevision = field(drawReport, "scroll_state_revision=");
        String reportedSurface = field(drawReport, "surface_texture=");
        double scaleX = width / (double) VIEWPORT_WIDTH;
        double scaleY = height / (double) VIEWPORT_HEIGHT;
        if (!"13".equals(field(drawReport, "boxes="))
                || !(VIEWPORT_WIDTH + "x" + VIEWPORT_HEIGHT)
                        .equals(field(drawReport, "viewport_css_px="))
                || !"5".equals(scrollRevision)
                || !drawReport.contains("scroll_offsets=[0,0;0,0;0,0;0,212]")
                || !reportedSurface.equals(width + "x" + height)
                || !isUnsignedInteger(drawGeneration) || width <= 0 || height <= 0
                || Math.abs(scaleX - scaleY) > 0.01) {
            Log.e(TAG, "SPINON_C125_PIXELCOPY status=error run_id=" + runId
                    + " stage=draw-identity boxes=" + field(drawReport, "boxes=")
                    + " surface=" + reportedSurface + " expected_surface=" + width + "x" + height
                    + " viewport=360x260 scroll_revision=" + scrollRevision);
            status.accept("실패 · C12.5 PixelCopy draw snapshot identity 불일치");
            return;
        }
        File output = new File(
                new File(activity.getCacheDir(), "c125-sticky-surface-pixels"),
                "surface-" + runId + ".png");
        Log.i(TAG, "SPINON_C125_PIXELCOPY_BEGIN run_id=" + runId
                + " pid=" + android.os.Process.myPid() + " surface_generation=" + generation
                + " presentation_sequence=" + presentationSequence + " draw_generation="
                + drawGeneration + " boxes=13 scroll_state_revision=" + scrollRevision
                + " viewport=360x260 surface=" + width + "x" + height
                + " scale_x=" + scaleX + " scale_y=" + scaleY + " samples=" + SAMPLES.size());
        SurfaceViewPixelCapture.requestForCssViewport(surfaceView, width, height,
                VIEWPORT_WIDTH, VIEWPORT_HEIGHT, 0, 0, "runtime-c12-5-sticky", runId,
                output, SAMPLES, mainHandler, runtimeQueue, sourceIsCurrent,
                result -> {
                    if (!sourceIsCurrent.getAsBoolean()) {
                        boolean artifactDeleted = result.error.isEmpty() && output.delete();
                        Log.e(TAG, "SPINON_C125_PIXELCOPY status=error run_id=" + runId
                                + " stage=stale-callback artifact_deleted=" + artifactDeleted);
                        return;
                    }
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C125_PIXELCOPY status=error run_id=" + runId
                                + " attempts=" + result.attempts + " reason=" + result.error);
                        status.accept("실패 · C12.5 PixelCopy · " + result.error);
                        return;
                    }
                    int matched = 0;
                    for (SurfaceViewPixelCapture.Pixel pixel : result.pixels) {
                        int[] expected = expectedRgba(pixel.id);
                        int cssX = "section-at-content-edge".equals(pixel.id) ? 200 : 240;
                        int cssY = "section-at-content-edge".equals(pixel.id) ? 78 : 100;
                        int expectedX = (int) Math.floor((cssX + 0.5) * scaleX);
                        int expectedY = (int) Math.floor((cssY + 0.5) * scaleY);
                        boolean coordinateExact = pixel.x == expectedX && pixel.y == expectedY;
                        boolean exact = expected != null && coordinateExact
                                && pixel.red == expected[0] && pixel.green == expected[1]
                                && pixel.blue == expected[2] && pixel.alpha == expected[3];
                        if (exact) matched++;
                        Log.i(TAG, "SPINON_C125_PIXEL run_id=" + runId + " sample=" + pixel.id
                                + " x_px=" + pixel.x + " y_px=" + pixel.y + " rgba="
                                + pixel.red + "," + pixel.green + "," + pixel.blue + ","
                                + pixel.alpha + " expected=" + (expected == null ? "missing"
                                        : expected[0] + "," + expected[1] + "," + expected[2]
                                                + "," + expected[3])
                                + " coordinate_exact=" + coordinateExact + " exact=" + exact);
                    }
                    boolean passed = result.width == width && result.height == height
                            && result.pixels.size() == SAMPLES.size() && matched == SAMPLES.size();
                    Log.i(TAG, "SPINON_C125_PIXELCOPY_SUMMARY status="
                            + (passed ? "matched" : "mismatch") + " run_id=" + runId
                            + " pid=" + android.os.Process.myPid() + " surface_generation="
                            + generation + " presentation_sequence=" + presentationSequence
                            + " draw_generation=" + drawGeneration + " scroll_state_revision="
                            + scrollRevision + " viewport=360x260 surface=" + result.width + "x"
                            + result.height + " scale_x=" + result.width / 360.0 + " scale_y="
                            + result.height / 260.0 + " expected_samples=" + SAMPLES.size()
                            + " observed_samples=" + result.pixels.size() + " matched_samples="
                            + matched + " attempts=" + result.attempts + " sha256="
                            + result.sha256 + " png=" + output.getAbsolutePath());
                    status.accept(passed ? "C12.5 Android PixelCopy 표본 통과 · 2/2"
                            : "실패 · C12.5 Android PixelCopy 표본 불일치");
                });
    }

    private static int[] expectedRgba(String id) {
        if ("section-at-content-edge".equals(id)) return new int[]{245, 245, 245, 255};
        if ("section-after".equals(id)) return new int[]{204, 221, 238, 255};
        return null;
    }

    private static String field(String report, String prefix) {
        int start = report.indexOf(prefix);
        if (start < 0) return "";
        start += prefix.length();
        int end = report.indexOf(' ', start);
        return report.substring(start, end < 0 ? report.length() : end);
    }

    private static boolean isUnsignedInteger(String value) {
        if (value.isEmpty()) return false;
        for (int index = 0; index < value.length(); index++) {
            if (value.charAt(index) < '0' || value.charAt(index) > '9') return false;
        }
        return true;
    }
}
