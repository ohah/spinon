package dev.spinon.bootstrap;

import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.ArrayList;
import java.util.List;
import java.util.TreeSet;

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
        "column-hidden", "row-hidden",
        "column-visibility-hidden", "column-visibility-visible-control",
        "row-visibility-hidden", "row-visibility-visible-control",
        "column-visibility-hidden-border-overflow",
        "column-visibility-visible-border-overflow-control",
        "row-visibility-hidden-border-overflow",
        "row-visibility-visible-border-overflow-control",
    };
    private static final double TOLERANCE = 1.0e-4;
    private static final String DIAGNOSTICS_MARKER = "grid_track_sizing_diagnostics=";
    private static final Pattern NODE_FRAMES = Pattern.compile("node_frames_css_px=\\[([^\\]]+)\\]");
    private static final Pattern NODE_FRAME = Pattern.compile(
            "^(\\d+):node=(\\d+),x=([^,]+),y=([^,]+),width=([^,]+),height=([^,]+)$");

    static final class SurfacePixelExpectation {
        final String id;
        final String nodeId;
        final int x;
        final int y;
        final int[] rgba;

        SurfacePixelExpectation(String id, String nodeId, int x, int y, int[] rgba) {
            this.id = id;
            this.nodeId = nodeId;
            this.x = x;
            this.y = y;
            this.rgba = rgba;
        }
    }

    private C114FlexStage() {}

    static boolean isScene(String sceneId) {
        for (String candidate : SCENE_IDS) {
            if (candidate.equals(sceneId)) return true;
        }
        return false;
    }

    static int itemCount(String sceneId) {
        if (isBorderOverflowScene(sceneId)) return isVisibilityHiddenScene(sceneId) ? 2 : 3;
        if (isVisibilityScene(sceneId)) return isVisibilityHiddenScene(sceneId) ? 3 : 4;
        if (isMixedScene(sceneId)) return 3;
        return isScene(sceneId) ? 2 : 0;
    }

    static String expectedRootFrame(String sceneId) {
        if (!isScene(sceneId)) return "";
        if (isBorderOverflowScene(sceneId)) {
            return isRowScene(sceneId) ? "width=120,height=100" : "width=100,height=120";
        }
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

    static List<SurfacePixelExpectation> surfacePixelSamples(String sceneId) {
        if (!isVisibilityScene(sceneId)) return null;
        String rootId = "novel-mixed-intrinsic-flex-fixed-" + sceneId + "-root";
        String spanId = "novel-mixed-intrinsic-flex-fixed-" + sceneId + "-visibility";
        boolean rows = isRowScene(sceneId);
        boolean hidden = isVisibilityHiddenScene(sceneId);
        String overlapId = sceneId + "-overlap";
        String gapId = sceneId + "-fixed-track-gap";
        if (isBorderOverflowScene(sceneId)) {
            String aId = "novel-mixed-intrinsic-flex-fixed-" + sceneId + "-a";
            String overflowOwner = hidden ? aId : spanId;
            return List.of(
                    new SurfacePixelExpectation(sceneId + "-border", rootId,
                            rows ? 60 : 1, rows ? 1 : 60,
                            new int[] {240, 198, 116, 255}),
                    new SurfacePixelExpectation(sceneId + "-overflow", overflowOwner,
                            rows ? 60 : 120, rows ? 120 : 60,
                            hidden ? new int[] {40, 100, 220, 255}
                                    : new int[] {142, 68, 173, 255}));
        }
        return List.of(
                new SurfacePixelExpectation(overlapId, spanId,
                        rows ? 60 : 120, rows ? 125 : 60,
                        hidden ? new int[] {40, 100, 220, 255}
                                : new int[] {142, 68, 173, 255}),
                new SurfacePixelExpectation(gapId, rootId,
                        rows ? 60 : 155, rows ? 155 : 60,
                        new int[] {232, 237, 245, 255}));
    }

    static boolean surfacePixelSamplesMatch(
            String sceneId, List<SurfaceViewPixelCapture.Sample> actual
    ) {
        List<SurfacePixelExpectation> expected = surfacePixelSamples(sceneId);
        if (expected == null || actual.size() != expected.size()) return false;
        TreeSet<String> found = new TreeSet<>();
        for (SurfaceViewPixelCapture.Sample sample : actual) {
            SurfacePixelExpectation match = null;
            for (SurfacePixelExpectation candidate : expected) {
                if (candidate.id.equals(sample.id)) match = candidate;
            }
            if (match == null || !found.add(sample.id)
                    || !match.nodeId.equals(sample.targetNodeId)
                    || sample.xCssPx != match.x || sample.yCssPx != match.y) return false;
        }
        return found.size() == expected.size();
    }

    static int[] expectedSurfacePixelRgb(String sceneId, String sampleId) {
        List<SurfacePixelExpectation> expected = surfacePixelSamples(sceneId);
        if (expected == null) return null;
        for (SurfacePixelExpectation sample : expected) {
            if (sample.id.equals(sampleId)) {
                return new int[] {sample.rgba[0], sample.rgba[1], sample.rgba[2]};
            }
        }
        return null;
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
            return fixedAxisMatches(sceneId, fixed)
                    && flexibleAxisMatches(sceneId, target)
                    && (!isMixedScene(sceneId) || intrinsicInputsMatch(sceneId, diagnostics));
        } catch (JSONException | ClassCastException | NumberFormatException error) {
            return false;
        }
    }

    private static boolean fixedAxisMatches(String sceneId, JSONObject pass) {
        JSONArray stages = pass.optJSONArray("stages");
        JSONArray contributions = pass.optJSONArray("contributions");
        if (stages == null || stages.length() != 2
                || contributions == null || contributions.length() != 0) return false;
        JSONObject initialize = stages.optJSONObject(0);
        JSONObject earlyReturn = stages.optJSONObject(1);
        return initialize != null && earlyReturn != null
                && stringEquals(initialize, "stage", "Initialize")
                && stringEquals(earlyReturn, "stage", "FixedTrackEarlyReturn")
                && fixedTracksMatch(sceneId, initialize.optJSONArray("tracks"))
                && fixedTracksMatch(sceneId, earlyReturn.optJSONArray("tracks"));
    }

    private static boolean fixedTracksMatch(String sceneId, JSONArray tracks) {
        if (tracks == null || tracks.length() != 3) return false;
        for (int index = 0; index < tracks.length(); index++) {
            JSONObject track = tracks.optJSONObject(index);
            if (track == null || !integerEquals(track, "internalIndex", index)) return false;
            boolean gutter = index % 2 == 0;
            boolean boundary = index == 0 || index == 2;
            double size = gutter ? 0.0 : isBorderOverflowScene(sceneId) ? 116.0 : 120.0;
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
                && visibilityContributionsMatch(sceneId, pass)
                && stages.optJSONObject(expand) != null
                && flexibleTracksMatch(
                        sceneId, stages.optJSONObject(expand).optJSONArray("tracks"));
    }

    private static boolean visibilityContributionsMatch(String sceneId, JSONObject pass)
            throws JSONException {
        if (!isVisibilityScene(sceneId)) return true;
        JSONArray contributions = pass.optJSONArray("contributions");
        if (contributions == null || contributions.length() != 4) return false;
        String automaticMinimum = "AutomaticMinimum { spans_auto_min_track: false, spans_multiple_tracks: false, spans_flexible_max_track: true, content_based: false, is_compressible_replaced: false, fixed_track_sum: None, fixed_gutter_sum: None, fixed_track_limit: None }";
        String[] keys = {"2:Minimum", "2:MinContent", "4:Minimum", "4:MinContent"};
        double[] values = {0, 100, 0, 150};
        boolean[] found = new boolean[keys.length];
        for (int index = 0; index < contributions.length(); index++) {
            JSONObject contribution = contributions.optJSONObject(index);
            if (contribution == null) return false;
            String kind = contribution.optString("kind");
            String key = contribution.optInt("item", -1) + ":" + kind;
            int expectedIndex = -1;
            for (int candidate = 0; candidate < keys.length; candidate++) {
                if (keys[candidate].equals(key)) expectedIndex = candidate;
            }
            if (expectedIndex < 0 || found[expectedIndex]
                    || !integerEquals(contribution, "span", 1)
                    || !booleanEquals(contribution, "crossesFlexibleTrack", true)
                    || !numberNear(contribution, "value", values[expectedIndex])) return false;
            found[expectedIndex] = true;
            if ("Minimum".equals(kind)) {
                if (!debugValueEquals(contribution, "minimumSource", automaticMinimum)) return false;
            } else if (!contribution.has("minimumSource")
                    || !contribution.isNull("minimumSource")) {
                return false;
            }
        }
        for (boolean matched : found) if (!matched) return false;
        return true;
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
            case "column-hidden":
                return new double[][] {
                    {0, 0, 300, 120}, {0, 0, 0, 0}, {0, 0, 100, 120},
                    {110, 0, 24, 16}, {160, 0, 140, 120},
                };
            case "row-hidden":
                return new double[][] {
                    {0, 0, 120, 300}, {0, 0, 0, 0}, {0, 0, 120, 100},
                    {0, 110, 24, 16}, {0, 160, 120, 140},
                };
            case "column-visibility-hidden":
            case "column-visibility-visible-control":
                return new double[][] {
                    {0, 0, 300, 120}, {0, 0, 150, 120}, {0, 0, 150, 120},
                    {160, 0, 24, 16}, {210, 0, 90, 120},
                };
            case "row-visibility-hidden":
            case "row-visibility-visible-control":
                return new double[][] {
                    {0, 0, 120, 300}, {0, 0, 120, 150}, {0, 0, 120, 150},
                    {0, 160, 24, 16}, {0, 210, 120, 90},
                };
            case "column-visibility-hidden-border-overflow":
            case "column-visibility-visible-border-overflow-control":
                return new double[][] {
                    {0, 0, 100, 120}, {2, 2, 150, 116}, {2, 2, 150, 116},
                    {162, 2, 24, 16}, {212, 2, 0, 116},
                };
            case "row-visibility-hidden-border-overflow":
            case "row-visibility-visible-border-overflow-control":
                return new double[][] {
                    {0, 0, 120, 100}, {2, 2, 116, 150}, {2, 2, 116, 150},
                    {2, 162, 24, 16}, {2, 212, 116, 0},
                };
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
            case "column-hidden":
            case "row-hidden": return new double[] {100, 40, 140};
            case "column-visibility-hidden":
            case "column-visibility-visible-control":
            case "row-visibility-hidden":
            case "row-visibility-visible-control": return new double[] {150, 40, 90};
            case "column-visibility-hidden-border-overflow":
            case "column-visibility-visible-border-overflow-control":
            case "row-visibility-hidden-border-overflow":
            case "row-visibility-visible-border-overflow-control": return new double[] {150, 40, 0};
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

    private static boolean isHiddenScene(String sceneId) {
        return "column-hidden".equals(sceneId) || "row-hidden".equals(sceneId);
    }

    private static boolean isVisibilityScene(String sceneId) {
        return sceneId.contains("-visibility-hidden")
                || sceneId.contains("-visibility-visible");
    }

    private static boolean isVisibilityHiddenScene(String sceneId) {
        return sceneId.contains("-visibility-hidden");
    }

    private static boolean isBorderOverflowScene(String sceneId) {
        return sceneId.contains("-border-overflow");
    }

    private static boolean intrinsicInputsMatch(String sceneId, JSONObject diagnostics) {
        JSONArray inputs = diagnostics.optJSONArray("gridItemIntrinsicInputs");
        int expectedInputCount = isVisibilityScene(sceneId) ? 4 : 3;
        if (inputs == null || inputs.length() != expectedInputCount) return false;
        boolean reversed = sceneId.endsWith("reversed");
        double aMinContent = sceneId.contains("equal-boundary") ? 80
                : sceneId.contains("no-freeze") ? 40 : 100;
        for (int index = 0; index < inputs.length(); index++) {
            JSONObject input = inputs.optJSONObject(index);
            int expectedSourceOrder = isHiddenScene(sceneId) ? index + 1 : index;
            if (input == null || !integerEquals(input, "sourceOrder", expectedSourceOrder)
                    || !integerEquals(input, "node", expectedIntrinsicNode(sceneId, index))) return false;
            JSONObject source = input.optJSONObject("source");
            if (source == null) return false;
            if (isVisibilityScene(sceneId) && index == 2 || !isVisibilityScene(sceneId) && index == 1) {
                JSONObject natural = source.optJSONObject("natural");
                if (!stringEquals(source, "kind", "replaced") || natural == null
                        || !numberNear(natural, "width", 24)
                        || !numberNear(natural, "height", 16)) return false;
                continue;
            }
            if (!stringEquals(source, "kind", isRowScene(sceneId) ? "orthogonalText" : "text")) {
                return false;
            }
            boolean visibilitySpan = isVisibilityScene(sceneId) && index == 1;
            boolean itemA = isVisibilityScene(sceneId)
                    ? index == 0 : reversed ? index == 2 : index == 0;
            double expected = visibilitySpan ? 150 : itemA ? aMinContent : 40;
            JSONObject contribution = input.optJSONObject("contributionInput");
            JSONObject axisSize = contribution == null ? null
                    : contribution.optJSONObject(isRowScene(sceneId) ? "height" : "width");
            if (axisSize == null || !numberNear(axisSize, "minContent", expected)) return false;
        }
        return true;
    }

    private static int expectedFrameNode(String sceneId, int frameIndex) {
        if (!isMixedScene(sceneId)) return frameIndex + 1;
        if (isVisibilityScene(sceneId)) {
            switch (frameIndex) {
                case 0: return 1;
                case 1: return 2;
                case 2: return 4;
                case 3: return 6;
                case 4: return 7;
                default: return -1;
            }
        }
        if (isHiddenScene(sceneId)) {
            switch (frameIndex) {
                case 0: return 1;
                case 1: return 2;
                case 2: return 4;
                case 3: return 6;
                case 4: return 7;
                default: return -1;
            }
        }
        switch (frameIndex) {
            case 0: return 1;
            case 1: return 2;
            case 2: return 4;
            case 3: return 5;
            default: return -1;
        }
    }

    private static int expectedIntrinsicNode(String sceneId, int inputIndex) {
        if (isVisibilityScene(sceneId)) {
            switch (inputIndex) {
                case 0: return 2;
                case 1: return 4;
                case 2: return 6;
                case 3: return 7;
                default: return -1;
            }
        }
        if (isHiddenScene(sceneId)) {
            switch (inputIndex) {
                case 0: return 4;
                case 1: return 6;
                case 2: return 7;
                default: return -1;
            }
        }
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

    private static boolean debugValueEquals(JSONObject object, String key, String expected) {
        Object value = object.opt(key);
        return value instanceof String
                && ((String) value).replaceAll("\\s+", "")
                        .equals(expected.replaceAll("\\s+", ""));
    }
}
