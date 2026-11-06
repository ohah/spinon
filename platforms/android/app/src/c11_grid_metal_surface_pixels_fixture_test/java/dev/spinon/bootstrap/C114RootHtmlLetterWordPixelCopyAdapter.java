package dev.spinon.bootstrap;

import android.app.Activity;
import android.os.Bundle;
import android.os.Handler;
import android.util.Log;
import android.view.SurfaceView;

import java.io.File;
import java.util.List;
import java.util.concurrent.Executor;
import java.util.function.BooleanSupplier;
import java.util.function.Consumer;

/** C11.4 ASCII 단어가 Grid 바깥으로 그려지는 표본을 PixelCopy로 확인합니다. */
public final class C114RootHtmlLetterWordPixelCopyAdapter
        implements RuntimeGpuSurfacePixelCaptureAdapter {
    private static final String TAG = "SpinonBootstrap";
    private static final String RUN_ID = "c11word-20261106";
    private static final String CAPTURE_EXTRA = "spinon_root_html_letter_word_pixelcopy";
    private static final String RUN_ID_EXTRA = "spinon_root_html_letter_word_pixelcopy_run_id";
    private static final String CACHE_DIRECTORY = "c11-root-html-grid-letter-word-pixelcopy";
    private static final int CSS_X = 42;
    private static final int CSS_Y = 19;
    private static final int[] EXPECTED_RGBA = {91, 168, 245, 255};

    private boolean captureStarted;

    public C114RootHtmlLetterWordPixelCopyAdapter() {}

    @Override
    public boolean isEnabled(Bundle extras) {
        return extras != null && extras.getBoolean(CAPTURE_EXTRA, false);
    }

    @Override
    public boolean acceptsIntentExtra(Bundle extras, String key) {
        return isEnabled(extras) && (CAPTURE_EXTRA.equals(key) || RUN_ID_EXTRA.equals(key));
    }

    @Override
    public void request(
            Activity activity, SurfaceView surfaceView, int width, int height, float density,
            long generation, String drawReport, Handler mainHandler, Executor runtimeQueue,
            BooleanSupplier sourceIsCurrent, Consumer<String> status) {
        synchronized (this) {
            if (captureStarted) return;
            captureStarted = true;
        }
        String requestedRunId = activity.getIntent().getStringExtra(RUN_ID_EXTRA);
        String runId = requestedRunId == null ? "" : requestedRunId;
        if (!RUN_ID.equals(runId)) {
            fail("invalid-run-id", runId, status);
            return;
        }
        double[] viewport = parsePair(drawReport, "viewport_css_px=");
        int[] reportedSurface = parseIntegerPair(drawReport, "surface_texture=");
        if (!drawReport.contains("status=0") || !drawReport.contains("presented")
                || viewport == null || reportedSurface == null
                || reportedSurface[0] != width || reportedSurface[1] != height
                || !scaleMatches(width, viewport[0], density)
                || !scaleMatches(height, viewport[1], density)) {
            Log.e(TAG, "SPINON_C111WORD_PIXELCOPY status=error run_id=" + runId
                    + " generation=" + generation + " reason=draw-surface-identity-mismatch"
                    + " report=" + drawReport);
            status.accept("실패 · C11.4 단어 overflow PixelCopy draw/surface identity 불일치");
            return;
        }

        List<SurfaceViewPixelCapture.Sample> samples = List.of(
                new SurfaceViewPixelCapture.Sample("word-overflow-outside-grid", CSS_X, CSS_Y));
        File output = new File(
                new File(activity.getCacheDir(), CACHE_DIRECTORY), "surface-" + runId + ".png");
        Log.i(TAG, "SPINON_C111WORD_PIXELCOPY_BEGIN run_id=" + runId
                + " scene=root-html-grid-letter-word-auto-minimum"
                + " pid=" + android.os.Process.myPid() + " generation=" + generation
                + " surface=" + width + "x" + height + " viewport="
                + viewport[0] + "x" + viewport[1] + " density=" + density
                + " css_sample=" + CSS_X + "," + CSS_Y + " samples=" + samples.size());
        SurfaceViewPixelCapture.request(surfaceView, width, height, density, 0, 0,
                "root-html-grid-letter-word-auto-minimum", runId, output, samples,
                mainHandler, runtimeQueue, sourceIsCurrent, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C111WORD_PIXELCOPY status=error run_id=" + runId
                                + " generation=" + generation + " attempts=" + result.attempts
                                + " reason=" + result.error);
                        status.accept("실패 · C11.4 단어 overflow PixelCopy · " + result.error);
                        return;
                    }
                    if (result.width != width || result.height != height
                            || result.pixels.size() != samples.size()) {
                        fail("surface-or-sample-count", runId, status);
                        return;
                    }
                    int expectedX = (int) Math.floor((CSS_X + 0.5) * density);
                    int expectedY = (int) Math.floor((CSS_Y + 0.5) * density);
                    SurfaceViewPixelCapture.Pixel pixel = result.pixels.get(0);
                    boolean coordinateMatch = pixel.x == expectedX && pixel.y == expectedY;
                    boolean colorMatch = pixel.red == EXPECTED_RGBA[0]
                            && pixel.green == EXPECTED_RGBA[1]
                            && pixel.blue == EXPECTED_RGBA[2]
                            && pixel.alpha == EXPECTED_RGBA[3];
                    boolean exact = coordinateMatch && colorMatch;
                    Log.i(TAG, "SPINON_C111WORD_PIXELCOPY_SAMPLE run_id=" + runId
                            + " sample=" + pixel.id + " css_x=" + CSS_X + " css_y=" + CSS_Y
                            + " x=" + pixel.x + " y=" + pixel.y
                            + " rgba=" + pixel.red + "," + pixel.green + ","
                            + pixel.blue + "," + pixel.alpha
                            + " coordinate_match=" + coordinateMatch
                            + " color_match=" + colorMatch + " exact=" + exact);
                    Log.i(TAG, "SPINON_C111WORD_PIXELCOPY_SUMMARY status="
                            + (exact ? "matched" : "mismatch") + " run_id=" + runId
                            + " scene=root-html-grid-letter-word-auto-minimum"
                            + " pid=" + android.os.Process.myPid() + " generation=" + generation
                            + " expected_samples=1 observed_samples=" + result.pixels.size()
                            + " matched_samples=" + (exact ? 1 : 0)
                            + " surface=" + result.width + "x" + result.height
                            + " viewport=" + viewport[0] + "x" + viewport[1]
                            + " png=" + output.getAbsolutePath() + " sha256=" + result.sha256);
                    status.accept(exact
                            ? "완료 · C11.4 단어 overflow SurfaceView PixelCopy · 1/1"
                            : "실패 · C11.4 단어 overflow PixelCopy 표본 불일치");
                });
    }

    private static boolean scaleMatches(int surfacePixels, double viewportCssPx, float density) {
        return viewportCssPx > 0 && Double.isFinite(viewportCssPx)
                && Math.abs(surfacePixels / viewportCssPx - density) <= 0.002;
    }

    private static double[] parsePair(String report, String marker) {
        if (report == null) return null;
        int start = report.indexOf(marker);
        if (start < 0) return null;
        String value = report.substring(start + marker.length()).split("\\s", 2)[0];
        String[] parts = value.split("x", -1);
        if (parts.length != 2) return null;
        try {
            return new double[] {Double.parseDouble(parts[0]), Double.parseDouble(parts[1])};
        } catch (NumberFormatException error) {
            return null;
        }
    }

    private static int[] parseIntegerPair(String report, String marker) {
        double[] values = parsePair(report, marker);
        if (values == null || values[0] != Math.rint(values[0])
                || values[1] != Math.rint(values[1])) return null;
        return new int[] {(int) values[0], (int) values[1]};
    }

    private static void fail(String reason, String runId, Consumer<String> status) {
        Log.e(TAG, "SPINON_C111WORD_PIXELCOPY status=error run_id=" + runId
                + " reason=" + reason);
        status.accept("실패 · C11.4 단어 overflow PixelCopy · " + reason);
    }
}
