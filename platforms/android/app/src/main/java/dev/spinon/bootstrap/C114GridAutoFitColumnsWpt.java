package dev.spinon.bootstrap;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;
import org.json.JSONTokener;
import java.util.ArrayList;
import java.util.List;
import java.util.Set;
import java.util.TreeSet;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

/** WPT-authored expectations and strict runtime checks for the fixed C11.4 slice. */
final class C114GridAutoFitColumnsWpt {
    private static final String EXPECTED_MARKER =
            "c11_4_grid_auto_fit_columns_wpt_expected=";
    private static final String TRACE_MARKER = "grid_track_sizing_diagnostics=";
    private static final Pattern USED_TRACK_TOKEN = Pattern.compile(
            "\\[([^\\]]*)\\]|((?:\\d+(?:\\.\\d+)?|\\.\\d+)px)");
    private static final double EPSILON = 0.02;

    private C114GridAutoFitColumnsWpt() {}

    static int caseIndex(String sceneId) {
        if (sceneId == null || !sceneId.startsWith("grid-auto-fit-columns-")) return -1;
        String suffix = sceneId.substring("grid-auto-fit-columns-".length());
        if (suffix.length() != 2) return -1;
        try {
            int index = Integer.parseInt(suffix);
            return index >= 1 && index <= 35 && String.format("%02d", index).equals(suffix)
                    ? index : -1;
        } catch (NumberFormatException ignored) {
            return -1;
        }
    }

    static boolean isScene(String sceneId) {
        return caseIndex(sceneId) > 0;
    }

    static int expectedBoxCount(String sceneId) {
        int index = caseIndex(sceneId);
        if (index < 1) return 0;
        return index == 32 || index == 35 ? 3 : 2;
    }

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

    static List<SurfacePixelExpectation> surfacePixelSamples(String sceneId) {
        int index = caseIndex(sceneId);
        final int itemX;
        final int itemWidth;
        final int itemSampleX;
        switch (index) {
            case 7: itemX = 0; itemWidth = 357; itemSampleX = 230; break;
            case 8: itemX = 0; itemWidth = 257; itemSampleX = 230; break;
            case 11: itemX = 50; itemWidth = 297; itemSampleX = 250; break;
            case 14: itemX = 267; itemWidth = 157; itemSampleX = 300; break;
            case 19: itemX = 80; itemWidth = 222; itemSampleX = 250; break;
            case 31: itemX = 40; itemWidth = 180; itemSampleX = 210; break;
            default: return null;
        }
        String suffix = String.format("%02d", index);
        String rootId = "c114-grid-auto-fit-wpt-" + suffix + "-root";
        String itemId = "c114-grid-auto-fit-wpt-" + suffix + "-item-1";
        String samplePrefix = "grid-auto-fit-columns-" + suffix;
        List<SurfacePixelExpectation> samples = new ArrayList<>(4);
        samples.add(new SurfacePixelExpectation(samplePrefix + "-border", rootId,
                100, 28, new int[] {255, 0, 255, 255}));
        samples.add(new SurfacePixelExpectation(samplePrefix + "-overflow-item", itemId,
                itemSampleX, 10, new int[] {0, 255, 255, 255}));
        samples.add(new SurfacePixelExpectation(samplePrefix + "-item-end-background", rootId,
                itemX + 2 + itemWidth + 1, 10, new int[] {0, 0, 0, 255}));
        samples.add(new SurfacePixelExpectation(samplePrefix + "-control-background", rootId,
                470, 200, new int[] {0, 0, 0, 255}));
        return samples;
    }

    static boolean surfacePixelSamplesMatch(
            String sceneId, List<SurfaceViewPixelCapture.Sample> actual
    ) {
        List<SurfacePixelExpectation> expected = surfacePixelSamples(sceneId);
        if (expected == null || actual.size() != expected.size()) return false;
        Set<String> found = new TreeSet<>();
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

    static String expectedRootFrame(String sceneId) {
        int index = caseIndex(sceneId);
        if (index < 1) return "";
        int width = index == 2 ? 230 : 204;
        int height = index == 32 ? 54 : 29;
        return "width=" + width + ",height=" + height;
    }

    static boolean nodeFramesMatch(String sceneId, String report) {
        if (!isScene(sceneId)) return true;
        try {
            JSONObject expected = expected(sceneId, report);
            JSONObject root = expected.getJSONObject("expectedRootFrame");
            JSONArray items = expected.getJSONArray("items");
            StringBuilder frames = new StringBuilder("node_frames_css_px=[")
                    .append("0:node=1,x=").append(root.getInt("x"))
                    .append(",y=").append(root.getInt("y"))
                    .append(",width=").append(root.getInt("width"))
                    .append(",height=").append(root.getInt("height"));
            for (int index = 0; index < items.length(); index++) {
                JSONObject frame = items.getJSONObject(index).getJSONObject("expected");
                frames.append(';').append(index + 1).append(":node=").append(index + 2)
                        .append(",x=").append(2 + frame.getInt("x"))
                        .append(",y=").append(2 + frame.getInt("y"))
                        .append(",width=").append(frame.getInt("width"))
                        .append(",height=").append(frame.getInt("height"));
            }
            return items.length() + 1 == expectedBoxCount(sceneId)
                    && report.contains(frames.append(']').toString());
        } catch (JSONException | ClassCastException error) {
            return false;
        }
    }

    static boolean traceMatches(String sceneId, String report) {
        if (!isScene(sceneId)) return true;
        try {
            JSONObject expected = expected(sceneId, report);
            JSONObject repeat = expected.getJSONObject("repeat");
            JSONObject observation = expected.getJSONObject("usedTrackObservation");
            int repeatCount = repeat.getInt("expectedCount");
            int explicitTracks = repeat.getInt("prefixTrackCount")
                    + repeat.getInt("patternTrackCount") * repeatCount
                    + repeat.getInt("suffixTrackCount");
            JSONObject diagnostics = jsonAfter(TRACE_MARKER, report);
            JSONArray traces = diagnostics.getJSONArray("trackSizingTraces");
            if (traces.length() != 1) return false;
            JSONObject trace = traces.getJSONObject(0);
            if (!trace.has("usedColumns") || !trace.has("usedRows")) return false;
            JSONArray passes = trace.getJSONArray("passes");
            if (trace.getInt("grid") != 1 || passes.length() != 2) return false;
            JSONObject columns = passes.getJSONObject(0);
            JSONObject rows = passes.getJSONObject(1);
            if (!"Columns".equals(columns.getString("axis"))
                    || !"Rows".equals(rows.getString("axis"))
                    || columns.getInt("sequence") != 0 || rows.getInt("sequence") != 1) {
                return false;
            }
            Object observedCount = columns.opt("autoRepeatCount");
            if (!(observedCount instanceof Number)
                    || ((Number) observedCount).intValue() != repeatCount) return false;
            if (rows.opt("autoRepeatCount") != null && !rows.isNull("autoRepeatCount")) {
                return false;
            }
            Object percentageBasis = columns.opt("percentageBasis");
            boolean authoredPercentage = repeat.getString("authoredTrackList").contains("%");
            if (percentageBasis instanceof Number
                    && !close(((Number) percentageBasis).doubleValue(),
                    Double.parseDouble(repeat.getString("definiteContentBasisPx")))) return false;
            if (authoredPercentage && !(percentageBasis instanceof Number)) return false;
            JSONArray columnContributions = columns.getJSONArray("contributions");
            JSONArray rowContributions = rows.getJSONArray("contributions");
            if (!contributionsMatch(columnContributions, sceneId)
                    || rowContributions.length() != 0) return false;
            return resolvedAxisMatches(trace.getJSONObject("usedColumns"),
                    observation, "columns", explicitTracks, repeat.getString("mode"))
                    && resolvedAxisMatches(trace.getJSONObject("usedRows"),
                    observation, "rows", 0, "none")
                    && passMatches(columns, observation.getString("gridTemplateColumns"),
                    observation.getString("columnGap"), repeat.getString("mode"))
                    && passMatches(rows, observation.getString("gridTemplateRows"),
                    observation.getString("rowGap"), "none");
        } catch (JSONException | ClassCastException error) {
            return false;
        }
    }

    private static boolean contributionsMatch(JSONArray contributions, String sceneId)
            throws JSONException {
        int expectedSpan = 0;
        if ("grid-auto-fit-columns-15".equals(sceneId)) {
            expectedSpan = 4;
        } else if ("grid-auto-fit-columns-18".equals(sceneId)) {
            expectedSpan = 3;
        } else if ("grid-auto-fit-columns-19".equals(sceneId)) {
            expectedSpan = 2;
        }
        if (expectedSpan == 0) return contributions.length() == 0;
        if (contributions.length() != 3) return false;
        Set<String> kinds = new TreeSet<>();
        for (int index = 0; index < contributions.length(); index++) {
            JSONObject contribution = contributions.getJSONObject(index);
            String kind = contribution.getString("kind");
            if (!("Minimum".equals(kind) || "MinContent".equals(kind)
                    || "MaxContent".equals(kind))
                    || !kinds.add(kind)
                    || contribution.getInt("item") != 2
                    || contribution.getInt("span") != expectedSpan
                    || contribution.getBoolean("crossesFlexibleTrack")
                    || !Double.isFinite(contribution.getDouble("value"))
                    || !close(contribution.getDouble("value"), 0)) return false;
            Object source = contribution.opt("minimumSource");
            if ("Minimum".equals(kind)) {
                if (!(source instanceof String)) return false;
            } else if (source != null && source != JSONObject.NULL) {
                return false;
            }
        }
        return kinds.size() == 3 && kinds.contains("Minimum")
                && kinds.contains("MinContent") && kinds.contains("MaxContent");
    }

    private static boolean resolvedAxisMatches(JSONObject actual, JSONObject observation,
            String axis, int explicitTrackCount, String repeatMode) throws JSONException {
        String expectedList = observation.getString(axis.equals("columns")
                ? "gridTemplateColumns" : "gridTemplateRows");
        ParsedTrackList expected = parseTrackList(expectedList);
        ParsedTrackList resolved = parseTrackList(actual.getString("usedTrackList"));
        if (expected == null || resolved == null || !trackListsMatch(resolved, expected)) {
            return false;
        }
        int expectedExplicit = observation.getInt(axis.equals("columns")
                ? "explicitColumnTrackCount" : "explicitRowTrackCount");
        int expectedNegative = observation.getInt(axis.equals("columns")
                ? "negativeImplicitColumnTrackCount" : "negativeImplicitRowTrackCount");
        int expectedPositive = observation.getInt(axis.equals("columns")
                ? "positiveImplicitColumnTrackCount" : "positiveImplicitRowTrackCount");
        if (expectedExplicit != explicitTrackCount
                || actual.getInt("explicitTrackCount") != expectedExplicit
                || actual.getInt("negativeImplicitTrackCount") != expectedNegative
                || actual.getInt("positiveImplicitTrackCount") != expectedPositive
                || expectedNegative + expectedExplicit + expectedPositive != expected.tracks.size()) {
            return false;
        }
        JSONArray collapsed = actual.getJSONArray("collapsedTrackIndices");
        List<Integer> expectedCollapsed = new ArrayList<>();
        if ("auto-fit".equals(repeatMode)) {
            for (int index = 0; index < expected.tracks.size(); index++) {
                if (close(expected.tracks.get(index).size, 0)) expectedCollapsed.add(index);
            }
        }
        if (collapsed.length() != expectedCollapsed.size()) return false;
        for (int index = 0; index < collapsed.length(); index++) {
            if (collapsed.getInt(index) != expectedCollapsed.get(index)) return false;
        }
        return true;
    }

    private static boolean passMatches(JSONObject pass, String expectedList, String gapValue,
            String repeatMode) throws JSONException {
        ParsedTrackList expected = parseTrackList(expectedList);
        JSONArray stages = pass.getJSONArray("stages");
        if (expected == null || stages.length() < 2) return false;
        JSONArray finalTracks = stages.getJSONObject(stages.length() - 1).getJSONArray("tracks");
        int expectedInternalCount = expected.tracks.size() * 2 + 1;
        if (finalTracks.length() != expectedInternalCount) return false;
        double gap = parseGap(gapValue);
        List<Boolean> collapsed = new ArrayList<>();
        for (Track track : expected.tracks) {
            collapsed.add("auto-fit".equals(repeatMode) && close(track.size, 0));
        }
        int trailingCollapsedStart = collapsed.size();
        while (trailingCollapsedStart > 0 && collapsed.get(trailingCollapsedStart - 1)) {
            trailingCollapsedStart--;
        }
        for (int stageIndex = 0; stageIndex < stages.length(); stageIndex++) {
            JSONObject stage = stages.getJSONObject(stageIndex);
            JSONArray tracks = stage.getJSONArray("tracks");
            if (tracks.length() != expectedInternalCount) return false;
            boolean finalStage = stageIndex == stages.length() - 1;
            for (int index = 0; index < tracks.length(); index++) {
                JSONObject track = tracks.getJSONObject(index);
                if (track.getInt("internalIndex") != index
                        || track.getBoolean("gutter") != (index % 2 == 0)) return false;
                double base = track.getDouble("baseSize");
                if (!Double.isFinite(base) || base < -EPSILON) return false;
                Object growthLimit = track.opt("growthLimit");
                if (growthLimit instanceof Number) {
                    double value = ((Number) growthLimit).doubleValue();
                    if (!Double.isFinite(value) || value < -EPSILON) return false;
                } else if (growthLimit != JSONObject.NULL && growthLimit != null) {
                    return false;
                }
                if (index % 2 == 1) {
                    int trackIndex = index / 2;
                    boolean expectedCollapsed = collapsed.get(trackIndex);
                    if (track.getBoolean("collapsed") != expectedCollapsed
                            || (finalStage
                            && !close(base, expected.tracks.get(trackIndex).size))) return false;
                    if (expectedCollapsed && (!close(base, 0)
                            || !(growthLimit instanceof Number)
                            || !close(((Number) growthLimit).doubleValue(), 0))) return false;
                } else {
                    int lineIndex = index / 2;
                    boolean edgeGutter = lineIndex == 0 || lineIndex == expected.tracks.size();
                    boolean previousTrackCollapsed = !edgeGutter && collapsed.get(lineIndex - 1);
                    boolean startsTrailingCollapsedRun = lineIndex == trailingCollapsedStart
                            && trailingCollapsedStart < expected.tracks.size();
                    boolean expectedCollapsed = edgeGutter || previousTrackCollapsed
                            || startsTrailingCollapsedRun;
                    double expectedGap = expectedCollapsed ? 0 : gap;
                    if (track.getBoolean("collapsed") != expectedCollapsed
                            || !close(base, expectedGap)) return false;
                }
            }
        }
        return "Initialize".equals(stages.getJSONObject(0).getString("stage"));
    }

    private static double parseGap(String value) {
        if ("normal".equals(value)) return 0;
        if (value == null || !value.endsWith("px")) return Double.NaN;
        try {
            return Double.parseDouble(value.substring(0, value.length() - 2));
        } catch (NumberFormatException ignored) {
            return Double.NaN;
        }
    }

    private static boolean trackListsMatch(ParsedTrackList actual, ParsedTrackList expected) {
        if (actual.tracks.size() != expected.tracks.size()
                || !actual.trailingNames.equals(expected.trailingNames)) return false;
        for (int index = 0; index < actual.tracks.size(); index++) {
            Track left = actual.tracks.get(index);
            Track right = expected.tracks.get(index);
            if (!left.names.equals(right.names) || !close(left.size, right.size)) return false;
        }
        return true;
    }

    private static ParsedTrackList parseTrackList(String value) {
        if (value == null || value.isEmpty() || "none".equals(value)) return null;
        Matcher matcher = USED_TRACK_TOKEN.matcher(value);
        List<Track> tracks = new ArrayList<>();
        Set<String> pendingNames = new TreeSet<>();
        int cursor = 0;
        while (matcher.find()) {
            if (!value.substring(cursor, matcher.start()).trim().isEmpty()) return null;
            if (matcher.group(1) != null) {
                for (String name : matcher.group(1).trim().split("\\s+")) {
                    if (!name.isEmpty()) pendingNames.add(name);
                }
            } else {
                String sizeToken = matcher.group(2);
                double size;
                try {
                    size = Double.parseDouble(sizeToken.substring(0, sizeToken.length() - 2));
                } catch (NumberFormatException error) {
                    return null;
                }
                tracks.add(new Track(new TreeSet<>(pendingNames), size));
                pendingNames.clear();
            }
            cursor = matcher.end();
        }
        if (!value.substring(cursor).trim().isEmpty() || tracks.isEmpty()) return null;
        return new ParsedTrackList(tracks, pendingNames);
    }

    private static boolean close(double left, double right) {
        return Double.isFinite(left) && Double.isFinite(right) && Math.abs(left - right) <= EPSILON;
    }

    private static final class ParsedTrackList {
        final List<Track> tracks;
        final Set<String> trailingNames;
        ParsedTrackList(List<Track> tracks, Set<String> trailingNames) {
            this.tracks = tracks;
            this.trailingNames = new TreeSet<>(trailingNames);
        }
    }

    private static final class Track {
        final Set<String> names;
        final double size;
        Track(Set<String> names, double size) {
            this.names = new TreeSet<>(names);
            this.size = size;
        }
    }

    private static JSONObject expected(String sceneId, String report) throws JSONException {
        JSONObject value = jsonAfter(EXPECTED_MARKER, report);
        if (!sceneId.equals(value.getString("sceneId"))
                || value.getInt("wptIndex") != caseIndex(sceneId)) {
            throw new JSONException("WPT expectation identity mismatch");
        }
        return value;
    }

    private static JSONObject jsonAfter(String marker, String report) throws JSONException {
        int start = report.indexOf(marker);
        if (start < 0) throw new JSONException("expected report marker is missing");
        Object value = new JSONTokener(report.substring(start + marker.length())).nextValue();
        if (!(value instanceof JSONObject)) throw new JSONException("report value is not an object");
        return (JSONObject) value;
    }
}
