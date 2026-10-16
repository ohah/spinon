package dev.spinon.bootstrap;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

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

public final class BoxPaintOpacityFixtureCaptureAdapter {
    private static final String TAG = "SpinonC0410";
    private static final int[][] CSS_POINTS = {
        {340, 232}, {26, 26}, {64, 26}, {80, 26}, {128, 24}, {168, 24}, {208, 24},
        {16, 72}, {84, 90}, {68, 68}, {124, 68}, {202, 76}, {212, 76}, {220, 76},
        {234, 58}, {256, 80},
    };
    private static final String[] SAMPLE_IDS = {
        "rootBackdrop", "sourceAlpha", "groupSingle", "groupOverlap", "nestedOpacity",
        "alphaTimesGroupOpacity", "wideGamutSrgb", "opacityZero", "opacityHalfSingle",
        "opacityHalfOverlap", "opacityOneOverlap", "clippedZOverlap", "clippedRed",
        "clippedAway", "currentColorBorder", "borderInterior",
    };
    private static final int[][] PRODUCT_RGBA = {
        {32, 49, 75, 255}, {143, 25, 38, 255}, {16, 89, 165, 255}, {143, 121, 38, 255},
        {24, 101, 56, 255}, {88, 37, 56, 255}, {0, 142, 196, 255}, {32, 49, 75, 255},
        {143, 25, 38, 255}, {16, 25, 165, 255}, {0, 0, 255, 255}, {16, 25, 165, 255},
        {143, 25, 38, 255}, {32, 49, 75, 255}, {28, 121, 102, 255}, {32, 49, 75, 255},
    };
    private static final int[][] CHROME_RGBA = {
        {32, 49, 75, 255}, {143, 25, 38, 255}, {16, 89, 165, 255}, {143, 121, 38, 255},
        {24, 101, 56, 255}, {88, 37, 56, 255}, {0, 142, 200, 255}, {32, 49, 75, 255},
        {143, 25, 38, 255}, {16, 25, 165, 255}, {0, 0, 255, 255}, {16, 25, 165, 255},
        {143, 25, 38, 255}, {32, 49, 75, 255}, {28, 121, 102, 255}, {32, 49, 75, 255},
    };

    static int[] surfaceSampleCoordinates(int width, int height) {
        double scaleX = width / 360.0;
        double scaleY = height / 240.0;
        if (width <= 0 || height <= 0 || scaleX < 1.0 || scaleX > 8.0
                || Math.abs(scaleX - scaleY) > 0.000001) {
            throw new IllegalArgumentException("drawable scale must be uniform and between 1 and 8");
        }
        int[] coordinates = new int[CSS_POINTS.length * 2];
        for (int index = 0; index < CSS_POINTS.length; index++) {
            coordinates[index * 2] = (int) Math.floor((CSS_POINTS[index][0] + 0.5) * scaleX);
            coordinates[index * 2 + 1] = (int) Math.floor((CSS_POINTS[index][1] + 0.5) * scaleY);
        }
        return coordinates;
    }

    static void logPixelSamples(String report, String label) {
        String marker = "surface_pixel_samples_json=";
        int start = report.indexOf(marker);
        if (start < 0) return;
        String json = report.substring(start + marker.length()).split("\\s", 2)[0];
        try {
            JSONArray samples = new JSONArray(json);
            if (samples.length() != SAMPLE_IDS.length) {
                Log.e(TAG, "SPINON_C192_" + label + "_SUMMARY status=sample-count-mismatch expected="
                        + SAMPLE_IDS.length + " actual=" + samples.length());
                return;
            }
            int productMatched = 0;
            int chromeMatched = 0;
            for (int index = 0; index < samples.length(); index++) {
                JSONObject sample = samples.getJSONObject(index);
                JSONArray channels = sample.getJSONArray("rgba");
                boolean productExact = matches(channels, PRODUCT_RGBA[index]);
                boolean chromeExact = matches(channels, CHROME_RGBA[index]);
                if (productExact) productMatched++;
                if (chromeExact) chromeMatched++;
                Log.i(TAG, "SPINON_C192_" + label + "_PIXEL sample=" + SAMPLE_IDS[index]
                        + " xy=" + sample.getInt("x") + "," + sample.getInt("y")
                        + " rgba=" + channels.getInt(0) + "," + channels.getInt(1) + ","
                        + channels.getInt(2) + "," + channels.getInt(3)
                        + " product_exact=" + productExact + " chrome_exact=" + chromeExact);
            }
            Log.i(TAG, "SPINON_C192_" + label + "_SUMMARY status="
                    + (productMatched == SAMPLE_IDS.length ? "product-matched" : "mismatch")
                    + " samples=" + SAMPLE_IDS.length + " product_matched=" + productMatched
                    + " chrome_exact=" + chromeMatched + " readback_path=" + label.toLowerCase());
        } catch (JSONException error) {
            Log.e(TAG, "SPINON_C192_" + label + "_SUMMARY status=parse-error reason="
                    + error.getClass().getSimpleName());
        }
    }

    private static boolean matches(JSONArray channels, int[] rgba) throws JSONException {
        return channels.length() == 4 && channels.getInt(0) == rgba[0]
                && channels.getInt(1) == rgba[1] && channels.getInt(2) == rgba[2]
                && channels.getInt(3) == rgba[3];
    }

    private BoxPaintOpacityFixtureCaptureAdapter() {}

    public static void request(SurfaceView surfaceView, Activity activity, int width, int height,
            long generation, Handler mainHandler, Executor runtimeQueue,
            BooleanSupplier sourceIsCurrent, Consumer<String> status) {
        String runId = Long.toHexString(android.os.SystemClock.elapsedRealtimeNanos());
        List<SurfaceViewPixelCapture.Sample> samples = new ArrayList<>(SAMPLE_IDS.length);
        for (int index = 0; index < SAMPLE_IDS.length; index++) {
            samples.add(new SurfaceViewPixelCapture.Sample(
                    SAMPLE_IDS[index], CSS_POINTS[index][0], CSS_POINTS[index][1]));
        }
        File output = new File(new File(activity.getCacheDir(), "c192-box-paint-opacity-captures"),
                "box-paint-opacity-" + runId + ".png");
        Log.i(TAG, "SPINON_C192_PIXELCOPY_BEGIN run_id=" + runId + " pid="
                + android.os.Process.myPid() + " generation=" + generation
                + " surface=" + width + "x" + height + " viewport=360x240 samples="
                + samples.size());
        SurfaceViewPixelCapture.requestForCssViewport(surfaceView, width, height, 360, 240,
                0, 0, "c192-box-paint-opacity", runId, output, samples, mainHandler,
                runtimeQueue, sourceIsCurrent, result -> {
                    if (!result.error.isEmpty()) {
                        Log.e(TAG, "SPINON_C192_PIXELCOPY status=error run_id=" + runId
                                + " generation=" + generation + " reason=" + result.error);
                        status.accept("실패 · C19.2 Android Simulator PixelCopy · " + result.error);
                        return;
                    }
                    if (result.pixels.size() != SAMPLE_IDS.length) {
                        Log.e(TAG, "SPINON_C192_PIXELCOPY status=sample-count-mismatch run_id="
                                + runId + " expected=" + SAMPLE_IDS.length + " actual="
                                + result.pixels.size());
                        status.accept("실패 · C19.2 Android Simulator PixelCopy 표본 개수 불일치");
                        return;
                    }
                    int productMatched = 0;
                    int chromeMatched = 0;
                    for (int index = 0; index < result.pixels.size(); index++) {
                        SurfaceViewPixelCapture.Pixel pixel = result.pixels.get(index);
                        boolean productExact = matches(pixel, PRODUCT_RGBA[index]);
                        boolean chromeExact = matches(pixel, CHROME_RGBA[index]);
                        if (productExact) productMatched++;
                        if (chromeExact) chromeMatched++;
                        Log.i(TAG, "SPINON_C192_PIXEL run_id=" + runId + " sample=" + pixel.id
                                + " xy=" + pixel.x + "," + pixel.y + " rgba=" + rgba(pixel)
                                + " product_exact=" + productExact + " chrome_exact=" + chromeExact);
                    }
                    boolean passed = result.pixels.size() == SAMPLE_IDS.length
                            && productMatched == SAMPLE_IDS.length;
                    Log.i(TAG, "SPINON_C192_PIXELCOPY_SUMMARY status="
                            + (passed ? "ok" : "mismatch") + " run_id=" + runId
                            + " viewport=360x240 surface=" + result.width + "x" + result.height
                            + " samples=" + result.pixels.size() + " product_matched="
                            + productMatched + " chrome_exact=" + chromeMatched
                            + " known_gamut_delta=wideGamutSrgb:-4-blue sha256=" + result.sha256
                            + " png=" + output.getAbsolutePath());
                    status.accept(passed
                            ? "완료 · C19.2 Android Simulator product 16/16 · Chrome exact "
                                    + chromeMatched + "/16"
                            : "실패 · C19.2 Android Simulator PixelCopy 표본 불일치");
                });
    }

    private static boolean matches(SurfaceViewPixelCapture.Pixel pixel, int[] rgba) {
        return pixel.red == rgba[0] && pixel.green == rgba[1]
                && pixel.blue == rgba[2] && pixel.alpha == rgba[3];
    }

    private static String rgba(SurfaceViewPixelCapture.Pixel pixel) {
        return pixel.red + "," + pixel.green + "," + pixel.blue + "," + pixel.alpha;
    }
}
