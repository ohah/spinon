package dev.spinon.bootstrap;

import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;
import org.json.JSONTokener;

/** C11.4의 고정 행 축 overlapping-span runtime fixture 검증기입니다. */
final class C114RowAxisSpanStage {
    private static final String[] SCENE_IDS = {
        "row-axis-span2-forward",
        "row-axis-span2-reversed",
        "row-axis-span2-before-span3",
        "row-axis-span3-before-span2",
    };
    private static final double FRAME_TOLERANCE = 1.0 / 64.0;
    private static final double TRACE_TOLERANCE = 1.0e-4;
    private static final String DIAGNOSTICS_MARKER = "grid_track_sizing_diagnostics=";
    private static final Pattern NODE_FRAMES = Pattern.compile("node_frames_css_px=\\[([^\\]]+)\\]");
    private static final Pattern NODE_FRAME = Pattern.compile(
            "^(\\d+):node=(\\d+),x=([^,]+),y=([^,]+),width=([^,]+),height=([^,]+)$");

    private C114RowAxisSpanStage() {}

    static boolean isScene(String sceneId) {
        for (String candidate : SCENE_IDS) {
            if (candidate.equals(sceneId)) return true;
        }
        return false;
    }

    static int itemCount(String sceneId) {
        if (!isScene(sceneId)) return 0;
        return sceneId.endsWith("before-span3") || sceneId.endsWith("before-span2") ? 3 : 2;
    }

    static int firstItemHeight(String sceneId) {
        switch (sceneId) {
            case "row-axis-span2-reversed": return 120;
            case "row-axis-span2-before-span3": return 80;
            case "row-axis-span3-before-span2": return 200;
            case "row-axis-span2-forward": return 80;
            default: return 0;
        }
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
                        || Integer.parseInt(frame.group(2)) != (int) expected[index][0]) {
                    return false;
                }
                for (int field = 0; field < 4; field++) {
                    double actual = Double.parseDouble(frame.group(field + 3));
                    if (!Double.isFinite(actual)
                            || Math.abs(actual - expected[index][field + 1]) > FRAME_TOLERANCE) {
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
            if (trace == null || !jsonIntegerEquals(trace, "grid", 1)
                    || passes == null || passes.length() != 2) return false;
            JSONObject columns = passes.optJSONObject(0);
            JSONObject rows = passes.optJSONObject(1);
            return columns != null && rows != null
                    && jsonIntegerEquals(columns, "sequence", 0)
                    && jsonIntegerEquals(rows, "sequence", 1)
                    && jsonStringEquals(columns, "axis", "Columns")
                    && jsonStringEquals(rows, "axis", "Rows")
                    && fixedColumnsMatch(columns)
                    && rowsMatch(sceneId, rows);
        } catch (JSONException | ClassCastException | NumberFormatException error) {
            return false;
        }
    }

    private static boolean fixedColumnsMatch(JSONObject pass) throws JSONException {
        JSONArray stages = pass.optJSONArray("stages");
        if (stages == null || stages.length() != 2) return false;
        JSONObject initialize = stages.optJSONObject(0);
        JSONObject earlyReturn = stages.optJSONObject(1);
        if (initialize == null || earlyReturn == null
                || !jsonStringEquals(initialize, "stage", "Initialize")
                || !jsonStringEquals(earlyReturn, "stage", "FixedTrackEarlyReturn")) {
            return false;
        }
        JSONArray contributions = pass.optJSONArray("contributions");
        if (contributions == null || contributions.length() != 0) return false;
        for (int index = 0; index < stages.length(); index++) {
            JSONObject stage = stages.optJSONObject(index);
            if (stage == null || !fixedColumnSnapshotMatch(stage.optJSONArray("tracks"))) {
                return false;
            }
        }
        return true;
    }

    private static boolean fixedColumnSnapshotMatch(JSONArray tracks) {
        if (tracks == null || tracks.length() != 3) return false;
        for (int index = 0; index < tracks.length(); index++) {
            JSONObject track = tracks.optJSONObject(index);
            if (track == null || !track.has("internalIndex")
                    || !track.has("gutter") || !track.has("collapsed")
                    || !track.has("baseSize") || !track.has("growthLimit")
                    || !jsonIntegerEquals(track, "internalIndex", index)) {
                return false;
            }
            boolean gutter = index % 2 == 0;
            boolean boundary = index == 0 || index == 2;
            if (!jsonBooleanEquals(track, "gutter", gutter)
                    || !jsonBooleanEquals(track, "collapsed", boundary)) return false;
            double expected = gutter ? 0.0 : 220.0;
            if (!jsonNumberNear(track, "baseSize", expected, TRACE_TOLERANCE)
                    || !jsonNumberNear(track, "growthLimit", expected, TRACE_TOLERANCE)) {
                return false;
            }
        }
        return true;
    }

    private static boolean rowsMatch(String sceneId, JSONObject pass) throws JSONException {
        JSONArray stages = pass.optJSONArray("stages");
        if (stages == null) return false;
        boolean twoBatches = itemCount(sceneId) == 3;
        List<String> expectedNames = new ArrayList<>();
        expectedNames.add("Initialize");
        expectedNames.add("IntrinsicItemBatch { span: 2, crosses_flexible_track: false }");
        if (twoBatches) {
            expectedNames.add("IntrinsicItemBatch { span: 3, crosses_flexible_track: false }");
        }
        expectedNames.add("IntrinsicSizingComplete");
        expectedNames.add("MaximizeTracks");
        expectedNames.add("ExpandFlexibleTracks");
        expectedNames.add("StretchAutoTracks { applied: false }");
        if (stages.length() != expectedNames.size()) return false;

        double[] zero = {0, 0, 0, 0};
        Double[] unbounded = {null, null, null, null};
        double[] span2Limits = {40, 60, 60, 0};
        double[] span23Limits = {160.0 / 3.0, 220.0 / 3.0, 220.0 / 3.0, 0};
        double[] batch2Limits = {40, 60, 60, Double.NaN};
        double[] batch3Limits = {
            160.0 / 3.0, 220.0 / 3.0, 220.0 / 3.0, Double.NaN,
        };
        double[] finalSizes = twoBatches
                ? new double[] {160.0 / 3.0, 220.0 / 3.0, 220.0 / 3.0, 0}
                : new double[] {40, 60, 60, 0};

        for (int index = 0; index < stages.length(); index++) {
            JSONObject stage = stages.optJSONObject(index);
            if (stage == null || !jsonStringEquals(stage, "stage", expectedNames.get(index))) {
                return false;
            }
            double[] expectedBase;
            Double[] expectedGrowth;
            if (index == 0) {
                expectedBase = zero;
                expectedGrowth = unbounded;
            } else if (index == 1) {
                expectedBase = zero;
                expectedGrowth = boxed(batch2Limits);
            } else if (twoBatches && index == 2) {
                expectedBase = zero;
                expectedGrowth = boxed(batch3Limits);
            } else if ((twoBatches && index == 3) || (!twoBatches && index == 2)) {
                expectedBase = zero;
                expectedGrowth = twoBatches ? boxed(span23Limits) : boxed(span2Limits);
            } else {
                expectedBase = finalSizes;
                expectedGrowth = twoBatches ? boxed(span23Limits) : boxed(span2Limits);
            }
            if (!rowStageTracksMatch(stage.optJSONArray("tracks"), expectedBase, expectedGrowth)) {
                return false;
            }
        }
        return contributionsMatch(sceneId, pass);
    }

    private static Double[] boxed(double[] values) {
        Double[] boxed = new Double[values.length];
        for (int index = 0; index < values.length; index++) {
            boxed[index] = Double.isNaN(values[index]) ? null : values[index];
        }
        return boxed;
    }

    private static boolean rowStageTracksMatch(
            JSONArray tracks, double[] expectedBase, Double[] expectedGrowth
    ) throws JSONException {
        if (tracks == null || tracks.length() != 9
                || expectedBase.length != 4 || expectedGrowth.length != 4) return false;
        for (int index = 0; index < tracks.length(); index++) {
            JSONObject track = tracks.optJSONObject(index);
            if (track == null || !track.has("internalIndex") || !track.has("gutter")
                    || !track.has("collapsed") || !track.has("baseSize")
                    || !track.has("growthLimit")
                    || !jsonIntegerEquals(track, "internalIndex", index)) {
                return false;
            }
            boolean gutter = index % 2 == 0;
            boolean boundary = index == 0 || index == 8;
            if (!jsonBooleanEquals(track, "gutter", gutter)
                    || !jsonBooleanEquals(track, "collapsed", boundary)) return false;
            double expected = gutter ? 0.0 : expectedBase[(index - 1) / 2];
            if (!jsonNumberNear(track, "baseSize", expected, TRACE_TOLERANCE)) return false;
            if (gutter) {
                if (!jsonNumberNear(track, "growthLimit", 0.0, TRACE_TOLERANCE)) return false;
            } else {
                Double limit = expectedGrowth[(index - 1) / 2];
                if (limit == null) {
                    if (!track.isNull("growthLimit")) return false;
                } else {
                    if (!jsonNumberNear(track, "growthLimit", limit, TRACE_TOLERANCE)) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    private static boolean contributionsMatch(String sceneId, JSONObject pass)
            throws JSONException {
        JSONArray actual = pass.optJSONArray("contributions");
        if (actual == null) return false;
        int[][] items = itemSpecs(sceneId);
        List<int[]> expected = new ArrayList<>();
        for (int span : new int[] {2, 3}) {
            if (span == 3 && itemCount(sceneId) == 2) continue;
            for (String kind : new String[] {"Minimum", "MinContent", "MaxContent"}) {
                for (int index = 0; index < items.length; index++) {
                    int[] item = items[index];
                    if (item[1] - item[0] == span) {
                        expected.add(new int[] {index + 2, span, item[2]});
                    }
                }
            }
        }
        if (actual.length() != expected.size()) return false;
        int position = 0;
        for (int span : new int[] {2, 3}) {
            if (span == 3 && itemCount(sceneId) == 2) continue;
            for (String kind : new String[] {"Minimum", "MinContent", "MaxContent"}) {
                for (int index = 0; index < items.length; index++) {
                    int[] item = items[index];
                    if (item[1] - item[0] != span) continue;
                    JSONObject contribution = actual.optJSONObject(position++);
                    if (contribution == null
                            || !jsonIntegerEquals(contribution, "item", index + 2)
                            || !jsonStringEquals(contribution, "kind", kind)
                            || !jsonIntegerEquals(contribution, "span", span)
                            || !jsonBooleanEquals(contribution, "crossesFlexibleTrack", false)
                            || !jsonNumberNear(
                                    contribution, "value", item[2], TRACE_TOLERANCE)) {
                        return false;
                    }
                }
            }
        }
        return position == actual.length();
    }

    private static int[][] itemSpecs(String sceneId) {
        if ("row-axis-span2-reversed".equals(sceneId)) {
            return new int[][] {{2, 4, 120}, {1, 3, 80}};
        }
        if ("row-axis-span2-before-span3".equals(sceneId)) {
            return new int[][] {{1, 3, 80}, {2, 4, 120}, {1, 4, 200}};
        }
        if ("row-axis-span3-before-span2".equals(sceneId)) {
            return new int[][] {{1, 4, 200}, {2, 4, 120}, {1, 3, 80}};
        }
        if ("row-axis-span2-forward".equals(sceneId)) {
            return new int[][] {{1, 3, 80}, {2, 4, 120}};
        }
        return new int[0][0];
    }

    private static double[][] expectedNodeFrames(String sceneId) {
        if (!isScene(sceneId)) return null;
        List<double[]> frames = new ArrayList<>();
        frames.add(new double[] {1, 0, 0, 220, 220});
        int[][] items = itemSpecs(sceneId);
        for (int index = 0; index < items.length; index++) {
            int[] item = items[index];
            double y;
            if (item[2] == 120) {
                y = items.length == 2 ? 40.0 : 160.0 / 3.0;
            } else {
                y = 0;
            }
            frames.add(new double[] {index + 2, 0, y, 220, item[2]});
        }
        return frames.toArray(new double[0][]);
    }

    private static boolean jsonIntegerEquals(JSONObject object, String key, int expected) {
        Object value = object.opt(key);
        if (!(value instanceof Number)) return false;
        double number = ((Number) value).doubleValue();
        return Double.isFinite(number) && number == expected;
    }

    private static boolean jsonNumberNear(
            JSONObject object, String key, double expected, double tolerance
    ) {
        Object value = object.opt(key);
        if (!(value instanceof Number)) return false;
        double number = ((Number) value).doubleValue();
        return Double.isFinite(number) && Math.abs(number - expected) <= tolerance;
    }

    private static boolean jsonBooleanEquals(JSONObject object, String key, boolean expected) {
        Object value = object.opt(key);
        return value instanceof Boolean && ((Boolean) value) == expected;
    }

    private static boolean jsonStringEquals(JSONObject object, String key, String expected) {
        Object value = object.opt(key);
        return value instanceof String && expected.equals(value);
    }
}
