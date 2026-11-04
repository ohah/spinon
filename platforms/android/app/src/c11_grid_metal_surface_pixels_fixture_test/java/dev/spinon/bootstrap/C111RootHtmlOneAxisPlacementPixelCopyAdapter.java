package dev.spinon.bootstrap;

import android.app.Activity;
import android.os.Bundle;
import android.os.Handler;
import android.util.Log;
import android.view.SurfaceView;

import java.io.File;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.Executor;
import java.util.function.BooleanSupplier;
import java.util.function.Consumer;

/** C11.1 Root HTML 혼합 명시·한 축 배치의 실제 SurfaceView를 PixelCopy로 확인합니다. */
public final class C111RootHtmlOneAxisPlacementPixelCopyAdapter
        implements RuntimeGpuSurfacePixelCaptureAdapter {
    private static final String TAG = "SpinonBootstrap";
    private static final String ONE_AXIS_RUN_ID = "c11oap-20261104";
    private static final String MIXED_RUN_ID = "c11mix19-20261104";
    private static final String ONE_AXIS_SCENE_ID = "root-html-grid-one-axis-placement";
    private static final String MIXED_SCENE_ID = "root-html-grid-mixed-explicit-one-axis";
    private static final String CAPTURE_EXTRA = "spinon_root_html_one_axis_placement_pixelcopy";
    private static final String RUN_ID_EXTRA =
            "spinon_root_html_one_axis_placement_pixelcopy_run_id";
    private static final String[] SAMPLE_IDS = {
            "column-first", "visibility-hidden-occupancy", "column-last-after-hidden",
            "row-first", "row-second", "outside-grid",
    };
    private static final int[][] CSS_POINTS = {
            {50, 10}, {50, 30}, {50, 60}, {10, 100}, {50, 100}, {125, 100},
    };
    private static final int[][] EXPECTED_RGBA = {
            {229, 72, 77, 255}, {255, 255, 255, 255}, {53, 120, 199, 255},
            {249, 115, 22, 255}, {143, 63, 113, 255}, {255, 255, 255, 255},
    };
    private static final String[] MIXED_SAMPLE_IDS = {
            "column-first", "visibility-hidden-full-occupancy", "column-last-after-hidden",
            "row-first", "row-full-blocker", "row-second", "row-span-blocker", "row-span-partial",
            "column-full-blocker", "column-partial-after-blocker",
            "column-span-blocker", "column-span-partial", "display-none-partial",
            "display-none-visible-full", "outside-grid", "all-auto-first",
            "all-auto-second",
            "fully-definite-item", "fully-definite-empty",
    };
    private static final int[][] MIXED_CSS_POINTS = {
            {50, 10}, {50, 30}, {50, 60}, {10, 100}, {50, 100}, {100, 100}, {10, 190}, {50, 200},
            {50, 280}, {50, 300}, {50, 370}, {50, 400}, {10, 460}, {100, 520},
            {125, 100}, {10, 550}, {50, 550}, {50, 600}, {10, 600},
    };
    private static final int[][] MIXED_EXPECTED_RGBA = {
            {229, 72, 77, 255}, {255, 255, 255, 255}, {53, 120, 199, 255},
            {249, 115, 22, 255}, {58, 143, 91, 255}, {143, 63, 113, 255}, {58, 143, 91, 255},
            {132, 94, 247, 255}, {249, 115, 22, 255}, {53, 120, 199, 255},
            {249, 115, 22, 255}, {132, 94, 247, 255}, {229, 72, 77, 255},
            {58, 143, 91, 255}, {255, 255, 255, 255},
            {229, 72, 77, 255}, {53, 120, 199, 255},
            {132, 94, 247, 255}, {255, 255, 255, 255},
    };

    private boolean captureStarted;

    public C111RootHtmlOneAxisPlacementPixelCopyAdapter() {}

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
            long generation, String drawReport, Handler mainHandler,
            Executor runtimeQueue, BooleanSupplier sourceIsCurrent, Consumer<String> status) {
        synchronized (this) {
            if (captureStarted) return;
            captureStarted = true;
        }
        String requestedRunId = activity.getIntent().getStringExtra(RUN_ID_EXTRA);
        final String runId = requestedRunId == null ? "" : requestedRunId;
        boolean mixedPlacement = MIXED_RUN_ID.equals(runId);
        if (!ONE_AXIS_RUN_ID.equals(runId) && !mixedPlacement) {
            fail("invalid-run-id", runId, status);
            return;
        }
        String sceneId = mixedPlacement ? MIXED_SCENE_ID : ONE_AXIS_SCENE_ID;
        String marker = mixedPlacement ? "SPINON_C111MIX_PIXELCOPY" : "SPINON_C111OAP_PIXELCOPY";
        String[] sampleIds = mixedPlacement ? MIXED_SAMPLE_IDS : SAMPLE_IDS;
        int[][] cssPoints = mixedPlacement ? MIXED_CSS_POINTS : CSS_POINTS;
        int[][] expectedRgba = mixedPlacement ? MIXED_EXPECTED_RGBA : EXPECTED_RGBA;
        double[] viewport = parsePair(drawReport, "viewport_css_px=");
        int[] reportedSurface = parseIntegerPair(drawReport, "surface_texture=");
        if (!drawReport.contains("status=0") || !drawReport.contains("presented")
                || viewport == null || reportedSurface == null
                || reportedSurface[0] != width || reportedSurface[1] != height
                || !scaleMatches(width, viewport[0], density)
                || !scaleMatches(height, viewport[1], density)) {
            Log.e(TAG, marker + " status=error run_id=" + runId
                    + " generation=" + generation + " reason=draw-surface-identity-mismatch"
                    + " report=" + drawReport);
            status.accept("실패 · C11.1 한 축 배치 PixelCopy draw/surface identity 불일치");
            return;
        }

        List<SurfaceViewPixelCapture.Sample> samples = new ArrayList<>(sampleIds.length);
        for (int index = 0; index < sampleIds.length; index++) {
            samples.add(new SurfaceViewPixelCapture.Sample(
                    sampleIds[index], cssPoints[index][0], cssPoints[index][1]));
        }
        File output = new File(new File(activity.getCacheDir(),
                mixedPlacement ? "c11-root-html-mixed-placement-pixelcopy"
                        : "c11-root-html-one-axis-placement-pixelcopy"),
                "surface-" + runId + ".png");
        Log.i(TAG, marker + "_BEGIN run_id=" + runId + " scene=" + sceneId
                + " pid=" + android.os.Process.myPid() + " generation=" + generation
                + " surface=" + width + "x" + height + " viewport="
                + viewport[0] + "x" + viewport[1] + " density=" + density
                + " samples=" + samples.size());
        SurfaceViewPixelCapture.request(surfaceView, width, height, density, 0, 0,
                sceneId, runId, output, samples, mainHandler, runtimeQueue,
                sourceIsCurrent, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, marker + " status=error run_id=" + runId
                                + " generation=" + generation + " attempts=" + result.attempts
                                + " reason=" + result.error);
                        status.accept("실패 · C11.1 placement PixelCopy · " + result.error);
                        return;
                    }
                    if (result.width != width || result.height != height
                            || result.pixels.size() != sampleIds.length) {
                        Log.e(TAG, marker + " status=error run_id=" + runId
                                + " reason=surface-or-sample-count width=" + result.width
                                + " height=" + result.height + " samples=" + result.pixels.size());
                        status.accept("실패 · C11.1 placement PixelCopy surface/sample 개수 불일치");
                        return;
                    }
                    int matched = 0;
                    for (int index = 0; index < result.pixels.size(); index++) {
                        SurfaceViewPixelCapture.Pixel pixel = result.pixels.get(index);
                        int expectedX = (int) Math.floor((cssPoints[index][0] + 0.5) * density);
                        int expectedY = (int) Math.floor((cssPoints[index][1] + 0.5) * density);
                        int[] expected = expectedRgba[index];
                        boolean coordinateMatch = pixel.x == expectedX && pixel.y == expectedY;
                        boolean colorMatch = pixel.red == expected[0] && pixel.green == expected[1]
                                && pixel.blue == expected[2] && pixel.alpha == expected[3];
                        boolean exact = coordinateMatch && colorMatch;
                        if (exact) matched++;
                        Log.i(TAG, marker + "_SAMPLE run_id=" + runId
                                + " sample=" + pixel.id + " x=" + pixel.x + " y=" + pixel.y
                                + " rgba=" + pixel.red + "," + pixel.green + "," + pixel.blue
                                + "," + pixel.alpha + " coordinate_match=" + coordinateMatch
                                + " color_match=" + colorMatch + " exact=" + exact);
                    }
                    boolean passed = matched == sampleIds.length;
                    Log.i(TAG, marker + "_SUMMARY status="
                            + (passed ? "matched" : "mismatch") + " run_id=" + runId
                            + " scene=" + sceneId + " pid=" + android.os.Process.myPid()
                            + " generation=" + generation
                            + " expected_samples=" + sampleIds.length + " observed_samples="
                            + result.pixels.size() + " matched_samples=" + matched
                            + " surface=" + result.width + "x" + result.height
                            + " viewport=" + viewport[0] + "x" + viewport[1]
                            + " png=" + output.getAbsolutePath() + " sha256=" + result.sha256);
                    status.accept(passed
                            ? "완료 · C11.1 placement SurfaceView PixelCopy · "
                                    + sampleIds.length + "/" + sampleIds.length
                            : "실패 · C11.1 placement PixelCopy 표본 불일치");
                });
    }

    private static boolean scaleMatches(int surfacePixels, double viewportCssPx, float density) {
        return viewportCssPx > 0 && Double.isFinite(viewportCssPx)
                && Math.abs(surfacePixels / viewportCssPx - density) <= 0.002;
    }

    private static double[] parsePair(String report, String marker) {
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
        Log.e(TAG, "SPINON_C111_PLACEMENT_PIXELCOPY status=error run_id=" + runId
                + " reason=" + reason);
        status.accept("실패 · C11.1 한 축 배치 PixelCopy · " + reason);
    }
}
