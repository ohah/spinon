package dev.spinon.bootstrap;

import java.util.regex.Matcher;
import java.util.regex.Pattern;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;
import org.json.JSONTokener;

/** C11.4 §11.7 fixed flexible-track runtime fixture matcher. */
final class C114FlexStage {
    private static final String[] SCENE_IDS = {
        "weighted-columns", "frozen-row", "subunit-columns", "zero-fr-row",
    };
    private static final double TOLERANCE = 1.0e-4;
    private static final String DIAGNOSTICS_MARKER = "grid_track_sizing_diagnostics=";
    private static final Pattern NODE_FRAMES = Pattern.compile("node_frames_css_px=\\[([^\\]]+)\\]");
    private static final Pattern NODE_FRAME = Pattern.compile(
            "^(\\d+):node=(\\d+),x=([^,]+),y=([^,]+),width=([^,]+),height=([^,]+)$");

    private C114FlexStage() {}

    static boolean isScene(String sceneId) {
        for (String candidate : SCENE_IDS) {
            if (candidate.equals(sceneId)) return true;
        }
        return false;
    }

    static int itemCount(String sceneId) {
        return isScene(sceneId) ? 2 : 0;
    }

    static String expectedRootFrame(String sceneId) {
        if (!isScene(sceneId)) return "";
        return isRowScene(sceneId) ? "width=120,height=200" : "width=200,height=120";
    }

    static boolean nodeFramesMatch(String sceneId, String report) {
        double[][] expected = expectedNodeFrames(sceneId);
        if (expected == null) return false;
        Matcher matcher = NODE_FRAMES.matcher(report);
        if (!matcher.find()) return false;
        String encodedFrames = matcher.group(1);
        if (matcher.find()) return false;
        String[] frames = encodedFrames.split(";", -1);
        if (frames.length != expected.length) return false;
        for (int index = 0; index < frames.length; index++) {
            Matcher frame = NODE_FRAME.matcher(frames[index].trim());
            if (!frame.matches()) return false;
            try {
                if (Integer.parseInt(frame.group(1)) != index
                        || Integer.parseInt(frame.group(2)) != index + 1) return false;
                for (int field = 0; field < 4; field++) {
                    double actual = Double.parseDouble(frame.group(field + 3));
                    if (!Double.isFinite(actual)
                            || Math.abs(actual - expected[index][field]) > TOLERANCE) {
                        return false;
                    }
                }
            } catch (NumberFormatException error) {
                return false;
            }
        }
        return true;
    }

    static boolean traceMatches(String sceneId, String report) {
        if (!isScene(sceneId)) return false;
        int marker = report.indexOf(DIAGNOSTICS_MARKER);
        if (marker < 0) return false;
        try {
            Object value = new JSONTokener(report.substring(marker + DIAGNOSTICS_MARKER.length()))
                    .nextValue();
            if (!(value instanceof JSONObject)) return false;
            JSONObject diagnostics = (JSONObject) value;
            JSONArray traces = diagnostics.optJSONArray("trackSizingTraces");
            if (traces == null || traces.length() != 1) return false;
            JSONObject trace = traces.optJSONObject(0);
            JSONArray passes = trace == null ? null : trace.optJSONArray("passes");
            if (trace == null || !integerEquals(trace, "grid", 1)
                    || passes == null || passes.length() != 2) return false;

            JSONObject columns = passes.optJSONObject(0);
            JSONObject rows = passes.optJSONObject(1);
            if (columns == null || rows == null
                    || !integerEquals(columns, "sequence", 0)
                    || !integerEquals(rows, "sequence", 1)
                    || !stringEquals(columns, "axis", "Columns")
                    || !stringEquals(rows, "axis", "Rows")) return false;

            JSONObject target = isRowScene(sceneId) ? rows : columns;
            JSONObject fixed = isRowScene(sceneId) ? columns : rows;
            return fixedAxisMatches(fixed)
                    && flexibleAxisMatches(sceneId, target);
        } catch (JSONException | ClassCastException | NumberFormatException error) {
            return false;
        }
    }

    private static boolean fixedAxisMatches(JSONObject pass) {
        JSONArray stages = pass.optJSONArray("stages");
        JSONArray contributions = pass.optJSONArray("contributions");
        if (stages == null || stages.length() != 2
                || contributions == null || contributions.length() != 0) return false;
        JSONObject initialize = stages.optJSONObject(0);
        JSONObject earlyReturn = stages.optJSONObject(1);
        return initialize != null && earlyReturn != null
                && stringEquals(initialize, "stage", "Initialize")
                && stringEquals(earlyReturn, "stage", "FixedTrackEarlyReturn")
                && fixedTracksMatch(initialize.optJSONArray("tracks"))
                && fixedTracksMatch(earlyReturn.optJSONArray("tracks"));
    }

    private static boolean fixedTracksMatch(JSONArray tracks) {
        if (tracks == null || tracks.length() != 3) return false;
        for (int index = 0; index < tracks.length(); index++) {
            JSONObject track = tracks.optJSONObject(index);
            if (track == null || !integerEquals(track, "internalIndex", index)) return false;
            boolean gutter = index % 2 == 0;
            boolean boundary = index == 0 || index == 2;
            double size = gutter ? 0.0 : 120.0;
            if (!booleanEquals(track, "gutter", gutter)
                    || !booleanEquals(track, "collapsed", boundary)
                    || !numberNear(track, "baseSize", size)
                    || !numberNear(track, "growthLimit", size)) return false;
        }
        return true;
    }

    private static boolean flexibleAxisMatches(String sceneId, JSONObject pass)
            throws JSONException {
        JSONArray stages = pass.optJSONArray("stages");
        if (stages == null || !stringEquals(pass, "axis", isRowScene(sceneId) ? "Rows" : "Columns")) {
            return false;
        }
        int initialize = -1;
        int maximize = -1;
        int expand = -1;
        int stretch = -1;
        int initializeCount = 0;
        int maximizeCount = 0;
        int expandCount = 0;
        int stretchCount = 0;
        for (int index = 0; index < stages.length(); index++) {
            JSONObject stage = stages.optJSONObject(index);
            if (stage == null) return false;
            String name = stage.optString("stage");
            if ("Initialize".equals(name)) {
                initialize = index;
                initializeCount++;
            }
            if ("MaximizeTracks".equals(name)) {
                maximize = index;
                maximizeCount++;
            }
            if ("ExpandFlexibleTracks".equals(name)) {
                expand = index;
                expandCount++;
            }
            if ("StretchAutoTracks { applied: false }".equals(name)) {
                stretch = index;
                stretchCount++;
            }
        }
        return initializeCount == 1 && maximizeCount == 1 && expandCount == 1
                && stretchCount == 1 && initialize < maximize && maximize < expand
                && expand < stretch
                && stages.optJSONObject(expand) != null
                && flexibleTracksMatch(
                        sceneId, stages.optJSONObject(expand).optJSONArray("tracks"));
    }

    private static boolean flexibleTracksMatch(String sceneId, JSONArray tracks) {
        double[] expected = expectedTrackBases(sceneId);
        if (expected == null || tracks == null || tracks.length() != 5) return false;
        int cssTrackIndex = 0;
        for (int index = 0; index < tracks.length(); index++) {
            JSONObject track = tracks.optJSONObject(index);
            if (track == null || !integerEquals(track, "internalIndex", index)) return false;
            boolean gutter = index % 2 == 0;
            boolean boundary = index == 0 || index == 4;
            double baseSize = gutter ? (index == 2 ? 10.0 : 0.0) : expected[cssTrackIndex++];
            if (!booleanEquals(track, "gutter", gutter)
                    || !booleanEquals(track, "collapsed", boundary)
                    || !numberNear(track, "baseSize", baseSize)) return false;
        }
        return cssTrackIndex == expected.length;
    }

    private static double[][] expectedNodeFrames(String sceneId) {
        switch (sceneId) {
            case "weighted-columns":
                return new double[][] {
                    {0, 0, 200, 120}, {0, 0, 190.0 / 3.0, 120}, {220.0 / 3.0, 0, 380.0 / 3.0, 120},
                };
            case "frozen-row":
                return new double[][] {
                    {0, 0, 120, 200}, {0, 0, 120, 80}, {0, 90, 120, 110},
                };
            case "subunit-columns":
                return new double[][] {
                    {0, 0, 200, 120}, {0, 0, 85, 120}, {95, 0, 20, 120},
                };
            case "zero-fr-row":
                return new double[][] {
                    {0, 0, 120, 200}, {0, 0, 120, 24}, {0, 34, 120, 166},
                };
            default:
                return null;
        }
    }

    private static double[] expectedTrackBases(String sceneId) {
        switch (sceneId) {
            case "weighted-columns": return new double[] {190.0 / 3.0, 380.0 / 3.0};
            case "frozen-row": return new double[] {80, 110};
            case "subunit-columns": return new double[] {85, 20};
            case "zero-fr-row": return new double[] {24, 166};
            default: return null;
        }
    }

    private static boolean isRowScene(String sceneId) {
        return "frozen-row".equals(sceneId) || "zero-fr-row".equals(sceneId);
    }

    private static boolean integerEquals(JSONObject object, String key, int expected) {
        Object value = object.opt(key);
        if (!(value instanceof Number)) return false;
        double number = ((Number) value).doubleValue();
        return Double.isFinite(number) && number == expected;
    }

    private static boolean numberNear(JSONObject object, String key, double expected) {
        Object value = object.opt(key);
        if (!(value instanceof Number)) return false;
        double number = ((Number) value).doubleValue();
        return Double.isFinite(number) && Math.abs(number - expected) <= TOLERANCE;
    }

    private static boolean booleanEquals(JSONObject object, String key, boolean expected) {
        Object value = object.opt(key);
        return value instanceof Boolean && ((Boolean) value) == expected;
    }

    private static boolean stringEquals(JSONObject object, String key, String expected) {
        Object value = object.opt(key);
        return value instanceof String && expected.equals(value);
    }
}
