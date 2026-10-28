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
        "column-freeze", "column-no-freeze", "column-equal-boundary", "column-freeze-reversed",
        "row-freeze", "row-no-freeze", "row-equal-boundary", "row-freeze-reversed",
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
        if (isMixedScene(sceneId)) return 3;
        return isScene(sceneId) ? 2 : 0;
    }

    static String expectedRootFrame(String sceneId) {
        if (!isScene(sceneId)) return "";
        if (isMixedScene(sceneId)) {
            return isRowScene(sceneId) ? "width=120,height=300" : "width=300,height=120";
        }
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
                        || Integer.parseInt(frame.group(2)) != expectedFrameNode(sceneId, index)) return false;
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
                    && flexibleAxisMatches(sceneId, target)
                    && (!isMixedScene(sceneId) || intrinsicInputsMatch(sceneId, diagnostics));
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
        if (expected == null || tracks == null || tracks.length() != expected.length * 2 + 1) {
            return false;
        }
        int cssTrackIndex = 0;
        for (int index = 0; index < tracks.length(); index++) {
            JSONObject track = tracks.optJSONObject(index);
            if (track == null || !integerEquals(track, "internalIndex", index)) return false;
            boolean gutter = index % 2 == 0;
            boolean boundary = index == 0 || index == tracks.length() - 1;
            double baseSize = gutter
                    ? (boundary ? 0.0 : 10.0)
                    : expected[cssTrackIndex++];
            if (!booleanEquals(track, "gutter", gutter)
                    || !booleanEquals(track, "collapsed", boundary)
                    || !numberNear(track, "baseSize", baseSize)) return false;
        }
        return cssTrackIndex == expected.length;
    }

    private static double[][] expectedNodeFrames(String sceneId) {
        switch (sceneId) {
            case "column-freeze":
                return new double[][] {
                    {0, 0, 300, 120}, {0, 0, 100, 120}, {110, 0, 24, 16}, {160, 0, 140, 120},
                };
            case "column-no-freeze":
            case "column-equal-boundary":
                return new double[][] {
                    {0, 0, 300, 120}, {0, 0, 80, 120}, {90, 0, 24, 16}, {140, 0, 160, 120},
                };
            case "column-freeze-reversed":
                return new double[][] {
                    {0, 0, 300, 120}, {160, 0, 140, 120}, {110, 0, 24, 16}, {0, 0, 100, 120},
                };
            case "row-freeze":
                return new double[][] {
                    {0, 0, 120, 300}, {0, 0, 120, 100}, {0, 110, 24, 16}, {0, 160, 120, 140},
                };
            case "row-no-freeze":
            case "row-equal-boundary":
                return new double[][] {
                    {0, 0, 120, 300}, {0, 0, 120, 80}, {0, 90, 24, 16}, {0, 140, 120, 160},
                };
            case "row-freeze-reversed":
                return new double[][] {
                    {0, 0, 120, 300}, {0, 160, 120, 140}, {0, 110, 24, 16}, {0, 0, 120, 100},
                };
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
            case "column-freeze":
            case "column-freeze-reversed":
            case "row-freeze":
            case "row-freeze-reversed": return new double[] {100, 40, 140};
            case "column-no-freeze":
            case "column-equal-boundary":
            case "row-no-freeze":
            case "row-equal-boundary": return new double[] {80, 40, 160};
            case "weighted-columns": return new double[] {190.0 / 3.0, 380.0 / 3.0};
            case "frozen-row": return new double[] {80, 110};
            case "subunit-columns": return new double[] {85, 20};
            case "zero-fr-row": return new double[] {24, 166};
            default: return null;
        }
    }

    private static boolean isRowScene(String sceneId) {
        return "frozen-row".equals(sceneId) || "zero-fr-row".equals(sceneId)
                || sceneId.startsWith("row-");
    }

    private static boolean isMixedScene(String sceneId) {
        return sceneId.startsWith("column-") || sceneId.startsWith("row-");
    }

    private static boolean intrinsicInputsMatch(String sceneId, JSONObject diagnostics) {
        JSONArray inputs = diagnostics.optJSONArray("gridItemIntrinsicInputs");
        if (inputs == null || inputs.length() != 3) return false;
        boolean reversed = sceneId.endsWith("reversed");
        double aMinContent = sceneId.contains("equal-boundary") ? 80
                : sceneId.contains("no-freeze") ? 40 : 100;
        for (int index = 0; index < inputs.length(); index++) {
            JSONObject input = inputs.optJSONObject(index);
            if (input == null || !integerEquals(input, "sourceOrder", index)
                    || !integerEquals(input, "node", expectedIntrinsicNode(index))) return false;
            JSONObject source = input.optJSONObject("source");
            if (source == null) return false;
            if (index == 1) {
                JSONObject natural = source.optJSONObject("natural");
                if (!stringEquals(source, "kind", "replaced") || natural == null
                        || !numberNear(natural, "width", 24)
                        || !numberNear(natural, "height", 16)) return false;
                continue;
            }
            if (!stringEquals(source, "kind", isRowScene(sceneId) ? "orthogonalText" : "text")) {
                return false;
            }
            boolean itemA = reversed ? index == 2 : index == 0;
            double expected = itemA ? aMinContent : 40;
            JSONObject contribution = input.optJSONObject("contributionInput");
            JSONObject axisSize = contribution == null ? null
                    : contribution.optJSONObject(isRowScene(sceneId) ? "height" : "width");
            if (axisSize == null || !numberNear(axisSize, "minContent", expected)) return false;
        }
        return true;
    }

    private static int expectedFrameNode(String sceneId, int frameIndex) {
        if (!isMixedScene(sceneId)) return frameIndex + 1;
        switch (frameIndex) {
            case 0: return 1;
            case 1: return 2;
            case 2: return 4;
            case 3: return 5;
            default: return -1;
        }
    }

    private static int expectedIntrinsicNode(int inputIndex) {
        switch (inputIndex) {
            case 0: return 2;
            case 1: return 4;
            case 2: return 5;
            default: return -1;
        }
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
