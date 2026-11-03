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

/** 표시된 SurfaceView에서 C11.5 Root HTML item 정렬 색상 표본을 읽습니다. */
public final class C115RootHtmlGridItemSelfAlignmentPixelCaptureAdapter {
    private static final String TAG = "SpinonBootstrap";
    private static final int VIEWPORT_WIDTH_CSS_PX = 980;
    private static final int VIEWPORT_HEIGHT_CSS_PX = 735;
    private static final String SCENE_ID = "root-html-grid-item-self-alignment";
    private static final String[] SAMPLE_IDS = {
            "centered-default-item", "right-edge-item-interior",
            "cell-right-boundary-outside", "safe-overflow-outside-cell",
            "unsafe-overflow-outside-cell", "viewport-bottom-clip-visible-last-row",
            "viewport-bottom-clip-outside-x",
    };
    private static final int[][] CSS_POINTS = {
            {44, 51}, {63, 151}, {64, 151}, {69, 556}, {20, 596}, {49, 734}, {90, 734},
    };
    private static final int[][] EXPECTED_RGBA = {
            {249, 115, 22, 255}, {249, 115, 22, 255}, {255, 255, 255, 255},
            {249, 115, 22, 255}, {249, 115, 22, 255}, {249, 115, 22, 255},
            {255, 255, 255, 255},
    };
    private static final String AUTO_STRETCH_SCENE_ID = "root-html-grid-item-auto-stretch";
    private static final String[] AUTO_STRETCH_SAMPLE_IDS = {
            "auto-parent-normal", "normal-self", "stretch-self", "position-self-start",
            "auto-parent-start", "fixed-stretch", "fixed-stretch-outside-x",
            "fixed-width-auto-height",
            "auto-width-fixed-height", "align-stretch-only", "justify-stretch-only",
    };
    private static final int[][] AUTO_STRETCH_CSS_POINTS = {
            {25, 25}, {25, 75}, {25, 125}, {25, 175}, {25, 225},
            {25, 275}, {40, 275}, {25, 345}, {55, 375}, {25, 425}, {55, 475},
    };
    private static final int[][] AUTO_STRETCH_EXPECTED_RGBA = {
            {249, 115, 22, 255}, {249, 115, 22, 255}, {249, 115, 22, 255},
            {219, 234, 254, 255}, {219, 234, 254, 255}, {249, 115, 22, 255},
            {219, 234, 254, 255}, {249, 115, 22, 255}, {249, 115, 22, 255},
            {219, 234, 254, 255}, {219, 234, 254, 255},
    };

    private C115RootHtmlGridItemSelfAlignmentPixelCaptureAdapter() {}

    public static void request(
            SurfaceView surfaceView, Activity activity, int width, int height,
            long generation, boolean autoStretch, String runId, String drawReport, Handler mainHandler,
            Executor runtimeQueue, BooleanSupplier sourceIsCurrent, Consumer<String> status) {
        int[] reportedViewport = parsePair(drawReport, "viewport_css_px=");
        int[] reportedSurface = parsePair(drawReport, "surface_texture=");
        if (!drawReport.contains("status=0") || !drawReport.contains("presented")
                || reportedViewport == null || reportedSurface == null
                || reportedViewport[0] != VIEWPORT_WIDTH_CSS_PX
                || reportedViewport[1] != VIEWPORT_HEIGHT_CSS_PX
                || reportedSurface[0] != width || reportedSurface[1] != height) {
            Log.e(TAG, "SPINON_C115_PIXELCOPY status=error run_id=" + runId
                    + " reason=draw-viewport-or-surface-mismatch report=" + drawReport);
            status.accept("실패 · C11.5 Android PixelCopy draw/viewport identity 불일치");
            return;
        }

        String sceneId = autoStretch ? AUTO_STRETCH_SCENE_ID : SCENE_ID;
        String[] sampleIds = autoStretch ? AUTO_STRETCH_SAMPLE_IDS : SAMPLE_IDS;
        int[][] cssPoints = autoStretch ? AUTO_STRETCH_CSS_POINTS : CSS_POINTS;
        int[][] expectedRgba = autoStretch ? AUTO_STRETCH_EXPECTED_RGBA : EXPECTED_RGBA;
        List<SurfaceViewPixelCapture.Sample> samples = new ArrayList<>(sampleIds.length);
        for (int index = 0; index < sampleIds.length; index++) {
            samples.add(new SurfaceViewPixelCapture.Sample(
                    sampleIds[index], cssPoints[index][0], cssPoints[index][1]));
        }
        File output = new File(new File(activity.getCacheDir(),
                autoStretch ? "c115-root-html-grid-item-auto-stretch"
                        : "c115-root-html-grid-item-self-alignment"),
                "surface-" + runId + ".png");
        Log.i(TAG, "SPINON_C115_PIXELCOPY_BEGIN run_id=" + runId
                + " pid=" + android.os.Process.myPid() + " generation=" + generation
                + " surface=" + width + "x" + height + " viewport="
                + VIEWPORT_WIDTH_CSS_PX + "x" + VIEWPORT_HEIGHT_CSS_PX
                + " samples=" + samples.size());
        SurfaceViewPixelCapture.requestForCssViewport(
                surfaceView, width, height,
                VIEWPORT_WIDTH_CSS_PX, VIEWPORT_HEIGHT_CSS_PX,
                0, 0, sceneId, runId, output, samples,
                mainHandler, runtimeQueue, sourceIsCurrent, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C115_PIXELCOPY status=error run_id=" + runId
                                + " generation=" + generation + " reason=" + result.error);
                        status.accept("실패 · C11.5 Android PixelCopy · " + result.error);
                        return;
                    }
                    if (result.width != width || result.height != height
                            || result.pixels.size() != sampleIds.length) {
                        Log.e(TAG, "SPINON_C115_PIXELCOPY status=error run_id=" + runId
                                + " reason=surface-or-sample-count width=" + result.width
                                + " height=" + result.height + " samples=" + result.pixels.size());
                        status.accept("실패 · C11.5 Android PixelCopy surface/sample 개수 불일치");
                        return;
                    }
                    double scaleX = result.width / (double) VIEWPORT_WIDTH_CSS_PX;
                    double scaleY = result.height / (double) VIEWPORT_HEIGHT_CSS_PX;
                    int matched = 0;
                    for (int index = 0; index < result.pixels.size(); index++) {
                        SurfaceViewPixelCapture.Pixel pixel = result.pixels.get(index);
                        boolean coordinateMatch = pixel.x == (int) Math.floor(
                                (cssPoints[index][0] + 0.5) * scaleX)
                                && pixel.y == (int) Math.floor(
                                        (cssPoints[index][1] + 0.5) * scaleY);
                        boolean colorMatch = pixel.red == expectedRgba[index][0]
                                && pixel.green == expectedRgba[index][1]
                                && pixel.blue == expectedRgba[index][2]
                                && pixel.alpha == expectedRgba[index][3];
                        boolean exact = coordinateMatch && colorMatch;
                        if (exact) matched++;
                        Log.i(TAG, "SPINON_C115_PIXELCOPY_SAMPLE run_id=" + runId
                                + " sample=" + pixel.id + " xy=" + pixel.x + "," + pixel.y
                                + " rgba=" + pixel.red + "," + pixel.green + ","
                                + pixel.blue + "," + pixel.alpha + " coordinate_match="
                                + coordinateMatch + " expected_match=" + colorMatch
                                + " exact=" + exact);
                    }
                    boolean passed = matched == sampleIds.length;
                    Log.i(TAG, "SPINON_C115_PIXELCOPY_SUMMARY status="
                            + (passed ? "matched" : "mismatch") + " run_id=" + runId
                            + " pid=" + android.os.Process.myPid() + " generation=" + generation
                            + " expected_samples=" + sampleIds.length + " observed_samples="
                            + result.pixels.size() + " matched_samples=" + matched
                            + " surface=" + result.width + "x" + result.height
                            + " viewport=" + VIEWPORT_WIDTH_CSS_PX + "x"
                            + VIEWPORT_HEIGHT_CSS_PX + " png=" + output.getAbsolutePath()
                            + " sha256=" + result.sha256);
                    status.accept(passed
                            ? "완료 · C11.5 Android SurfaceView PixelCopy · "
                                    + sampleIds.length + "/" + sampleIds.length
                            : "실패 · C11.5 Android PixelCopy 표본 불일치");
                });
    }

    private static int[] parsePair(String report, String marker) {
        int start = report.indexOf(marker);
        if (start < 0) return null;
        String value = report.substring(start + marker.length()).split("\\s", 2)[0];
        String[] parts = value.split("x", -1);
        if (parts.length != 2) return null;
        try {
            return new int[] {Integer.parseInt(parts[0]), Integer.parseInt(parts[1])};
        } catch (NumberFormatException error) {
            return null;
        }
    }
}
