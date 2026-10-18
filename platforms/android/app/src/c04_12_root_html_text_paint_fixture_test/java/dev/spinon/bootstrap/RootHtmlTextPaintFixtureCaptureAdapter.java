package dev.spinon.bootstrap;

import android.app.Activity;
import android.os.Handler;
import android.util.Log;
import android.view.SurfaceView;

import java.io.File;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.Executor;
import java.util.function.BooleanSupplier;
import java.util.function.Consumer;

/** C04.12 parsed root HTML glyph output is sampled from the Android SurfaceView. */
public final class RootHtmlTextPaintFixtureCaptureAdapter {
    private static final String TAG = "SpinonC0410";
    private static final int[][] CSS_POINTS = {
            {1, 1}, {16, 1}, {1, 11}, {16, 11}, {100, 100},
    };
    private static final String[] SAMPLE_IDS = {
            "visible-first-glyph", "visible-overflow-glyph", "hidden-owner-first-glyph",
            "outside-owner-clip", "root-background",
    };
    private static final int[][] EXPECTED_RGBA = {
            {9, 33, 112, 255}, {9, 33, 112, 255}, {9, 33, 112, 255},
            {247, 249, 252, 255}, {247, 249, 252, 255},
    };

    private RootHtmlTextPaintFixtureCaptureAdapter() {}

    public static void request(SurfaceView surfaceView, Activity activity, int width, int height,
            long generation, String drawReport, Handler mainHandler, Executor runtimeQueue,
            BooleanSupplier sourceIsCurrent, Consumer<String> status) {
        String runId = Long.toHexString(android.os.SystemClock.elapsedRealtimeNanos());
        List<SurfaceViewPixelCapture.Sample> samples = new ArrayList<>(SAMPLE_IDS.length);
        for (int index = 0; index < SAMPLE_IDS.length; index++) {
            samples.add(new SurfaceViewPixelCapture.Sample(
                    SAMPLE_IDS[index], CSS_POINTS[index][0], CSS_POINTS[index][1]));
        }
        File output = new File(new File(activity.getCacheDir(),
                "c0412-root-html-text-paint-captures"), "root-html-text-paint-" + runId + ".png");
        Log.i(TAG, "SPINON_C0412_TEXT_PIXELCOPY_BEGIN run_id=" + runId + " pid="
                + android.os.Process.myPid() + " generation=" + generation
                + " surface=" + width + "x" + height + " viewport=320x240 samples="
                + samples.size());
        SurfaceViewPixelCapture.requestForCssViewport(surfaceView, width, height, 320, 240,
                0, 0, "c0412-root-html-text-paint", runId, output, samples,
                mainHandler, runtimeQueue, sourceIsCurrent, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C0412_TEXT_PIXELCOPY status=error run_id=" + runId
                                + " generation=" + generation + " reason=" + result.error);
                        status.accept("실패 · C04.12 Android text PixelCopy · " + result.error);
                        return;
                    }
                    if (result.pixels.size() != SAMPLE_IDS.length) {
                        Log.e(TAG, "SPINON_C0412_TEXT_PIXELCOPY status=sample-count-mismatch run_id="
                                + runId + " expected=" + SAMPLE_IDS.length + " actual="
                                + result.pixels.size());
                        status.accept("실패 · C04.12 Android text PixelCopy 표본 개수 불일치");
                        return;
                    }
                    int[] reportPoints = parseRuntimeReportPoints(drawReport);
                    int matched = 0;
                    for (int index = 0; index < result.pixels.size(); index++) {
                        SurfaceViewPixelCapture.Pixel pixel = result.pixels.get(index);
                        boolean coordinateMatch = reportPoints != null
                                && reportPoints[index * 2] == pixel.x
                                && reportPoints[index * 2 + 1] == pixel.y;
                        boolean colorMatch = pixel.red == EXPECTED_RGBA[index][0]
                                && pixel.green == EXPECTED_RGBA[index][1]
                                && pixel.blue == EXPECTED_RGBA[index][2]
                                && pixel.alpha == EXPECTED_RGBA[index][3];
                        if (coordinateMatch && colorMatch) matched++;
                        Log.i(TAG, "SPINON_C0412_TEXT_PIXELCOPY_SAMPLE run_id=" + runId
                                + " id=" + pixel.id + " xy=" + pixel.x + "," + pixel.y
                                + " rgba=" + pixel.red + "," + pixel.green + ","
                                + pixel.blue + "," + pixel.alpha + " coordinate_match="
                                + coordinateMatch + " expected_match=" + colorMatch);
                    }
                    boolean passed = reportPoints != null && matched == SAMPLE_IDS.length;
                    Log.i(TAG, "SPINON_C0412_TEXT_PIXELCOPY_SUMMARY status="
                            + (passed ? "matched" : "mismatch") + " run_id=" + runId
                            + " pid=" + android.os.Process.myPid() + " generation=" + generation
                            + " samples=" + matched + "/" + SAMPLE_IDS.length + " surface="
                            + result.width + "x" + result.height + " viewport=320x240 png="
                            + output.getAbsolutePath() + " sha256=" + result.sha256);
                    status.accept(passed
                            ? "완료 · C04.12 Android glyph SurfaceView PixelCopy · 5/5"
                            : "실패 · C04.12 Android glyph PixelCopy 표본 불일치");
                });
    }

    private static int[] parseRuntimeReportPoints(String report) {
        String marker = "root_html_text_sample_surface_xy=";
        int start = report.indexOf(marker);
        if (start < 0) return null;
        String encoded = report.substring(start + marker.length()).split("\\s", 2)[0];
        String[] pairs = encoded.split(";", -1);
        if (pairs.length != SAMPLE_IDS.length) return null;
        int[] points = new int[pairs.length * 2];
        try {
            for (int index = 0; index < pairs.length; index++) {
                String[] coordinates = pairs[index].split(",", -1);
                if (coordinates.length != 2) return null;
                points[index * 2] = Integer.parseInt(coordinates[0]);
                points[index * 2 + 1] = Integer.parseInt(coordinates[1]);
            }
        } catch (NumberFormatException error) {
            return null;
        }
        return points;
    }
}
