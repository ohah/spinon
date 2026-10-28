package dev.spinon.bootstrap;

import java.util.ArrayList;
import java.util.List;
import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;
import org.json.JSONTokener;

/** Pinned C11.4 WPT implicit-column oracle for the fixture-test runtime route. */
final class C114FitContentImplicitColumnWpt {
    static final String SCENE_ID = "fit-content-implicit-column-wpt";
    private static final String ROOT_ID = "c114-fit-content-implicit-column-grid";
    private static final String ITEM_ID = "c114-fit-content-implicit-column-item";
    private static final String CHILD_ID = "c114-fit-content-implicit-column-child";
    private static final String TRACE_MARKER = "grid_track_sizing_diagnostics=";
    private static final double EPSILON = 0.000001;

    private C114FitContentImplicitColumnWpt() {}

    static boolean isScene(String sceneId) {
        return SCENE_ID.equals(sceneId);
    }

    static String expectedRootFrame(String sceneId) {
        return isScene(sceneId) ? "width=300,height=300" : "";
    }

    static String expectedItemFrame(String sceneId) {
        return isScene(sceneId) ? "width=40,height=10" : "";
    }

    static int expectedBoxCount(String sceneId) {
        return isScene(sceneId) ? 3 : 0;
    }

    static String expectedNodeFrames(String sceneId) {
        if (!isScene(sceneId)) return "";
        return "node_frames_css_px=[0:node=1,x=0,y=0,width=300,height=300;"
                + "1:node=2,x=60,y=0,width=40,height=10;"
                + "2:node=3,x=60,y=0,width=400,height=10]";
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
        if (!isScene(sceneId)) return null;
        List<SurfacePixelExpectation> samples = new ArrayList<>(4);
        samples.add(new SurfacePixelExpectation("implicit-column-root-background", ROOT_ID,
                30, 5, new int[] {23, 33, 43, 255}));
        samples.add(new SurfacePixelExpectation("implicit-column-child-inside-item", CHILD_ID,
                80, 5, new int[] {255, 209, 102, 255}));
        samples.add(new SurfacePixelExpectation("implicit-column-child-outside-root", CHILD_ID,
                350, 5, new int[] {255, 209, 102, 255}));
        samples.add(new SurfacePixelExpectation("implicit-column-surface-clear", ROOT_ID,
                470, 5, new int[] {0, 0, 0, 255}));
        return samples;
    }

    static boolean surfacePixelSamplesMatch(
            String sceneId, List<SurfaceViewPixelCapture.Sample> actual
    ) {
        List<SurfacePixelExpectation> expected = surfacePixelSamples(sceneId);
        if (expected == null || actual.size() != expected.size()) return false;
        boolean[] found = new boolean[expected.size()];
        for (SurfaceViewPixelCapture.Sample sample : actual) {
            int index = -1;
            for (int candidate = 0; candidate < expected.size(); candidate++) {
                if (expected.get(candidate).id.equals(sample.id)) index = candidate;
            }
            if (index < 0 || found[index]) return false;
            SurfacePixelExpectation item = expected.get(index);
            if (!item.nodeId.equals(sample.targetNodeId)
                    || sample.xCssPx != item.x || sample.yCssPx != item.y) return false;
            found[index] = true;
        }
        for (boolean sampleFound : found) if (!sampleFound) return false;
        return true;
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

    static boolean nodeFramesMatch(String sceneId, String report) {
        return !isScene(sceneId) || report.contains(expectedNodeFrames(sceneId));
    }

    static boolean traceMatches(String sceneId, String report) {
        if (!isScene(sceneId)) return true;
        int marker = report.indexOf(TRACE_MARKER);
        if (marker < 0) return false;
        try {
            Object value = new JSONTokener(report.substring(marker + TRACE_MARKER.length()))
                    .nextValue();
            if (!(value instanceof JSONObject)) return false;
            JSONObject diagnostics = (JSONObject) value;
            JSONObject observation = diagnostics.optJSONObject("fitContentImplicitColumn");
            JSONObject style = observation == null ? null : observation.optJSONObject("styleSnapshot");
            JSONObject computed = style == null ? null : style.optJSONObject("computedProperties");
            JSONObject input = observation == null ? null
                    : observation.optJSONObject("layoutInputGridContainer");
            JSONObject ids = observation == null ? null : observation.optJSONObject("nodeIds");
            JSONObject frames = observation == null ? null : observation.optJSONObject("framesCssPx");
            JSONObject revision = diagnostics.optJSONObject("layoutInputRevision");
            JSONObject overflow = observation == null ? null : observation.optJSONObject("overflowCssPx");
            if (observation == null || style == null || computed == null || input == null
                    || ids == null || frames == null || revision == null || overflow == null
                    || !SCENE_ID.equals(observation.optString("sceneId"))
                    || !"pre-render LayoutOutput CSS px".equals(observation.optString("frameSource"))
                    || !"RuntimeBlockFlexGridTrackSizingV1".equals(style.optString("profile"))
                    || !"grid".equals(computed.optString("display"))
                    || !"LengthPx(300.0)".equals(computed.optString("width"))
                    || !"LengthPx(300.0)".equals(computed.optString("height"))
                    || !"60px".equals(computed.optString("gridTemplateColumns"))
                    || !"none".equals(computed.optString("gridTemplateRows"))
                    || !"fit-content(40px)".equals(computed.optString("gridAutoColumns"))
                    || !"fit-content(40px)".equals(computed.optString("gridAutoRows"))
                    || !"normal".equals(computed.optString("alignContent"))
                    || !computed.optString("itemColumnStart").contains("Line(2)")
                    || !"LengthPx(0.0)".equals(computed.optString("itemMinWidth"))
                    || !"[LengthPx(60.0)]".equals(input.optString("columns"))
                    || !"[]".equals(input.optString("explicitRows"))
                    || !"TrackList([FitContent(LengthPx(40.0))])".equals(
                            input.optString("autoColumns"))
                    || !"TrackList([FitContent(LengthPx(40.0))])".equals(
                            input.optString("autoRows"))
                    || !"Row".equals(input.optString("autoFlow"))
                    || !"TrackSizing".equals(input.optString("placementMode"))
                    || ids.optLong("root", -1) != 1 || ids.optLong("item", -1) != 2
                    || ids.optLong("child", -1) != 3
                    || style.optLong("rootNode", -1) != 1 || input.optLong("rootNode", -1) != 1
                    || !frameMatches(frames.optJSONObject("root"), 0, 0, 300, 300)
                    || !frameMatches(frames.optJSONObject("item"), 60, 0, 40, 10)
                    || !frameMatches(frames.optJSONObject("child"), 60, 0, 400, 10)
                    || !intArrayMatches(observation.optJSONArray("expectedColumnsCssPx"), 60, 40)
                    || !intArrayMatches(observation.optJSONArray("expectedRowsCssPx"), 10)
                    || observation.optInt("expectedItemMinimumContributionCssPx", -1) != 0
                    || observation.optInt("expectedItemMaxContentContributionCssPx", -1) != 400
                    || overflow.optInt("childPastItem", -1) != 360
                    || overflow.optInt("childPastRoot", -1) != 160
                    || !stagesMatch(observation.optJSONArray("columnSizingStages"),
                            new double[][] {{60, 0}, {60, 0}, {60, 0}, {60, 40}, {60, 40}, {60, 40}},
                            false)
                    || !stagesMatch(observation.optJSONArray("rowSizingStages"),
                            new double[][] {{0}, {10}, {10}, {10}, {10}, {10}}, true)
                    || !contributionsMatch(observation.optJSONArray("columnContributions"),
                            new double[] {0, 400, 400}, "SpecifiedMinimum")
                    || !contributionsMatch(observation.optJSONArray("rowContributions"),
                            new double[] {10, 10, 10},
                            "AutomaticMinimum { spans_auto_min_track: true, spans_multiple_tracks: false, spans_flexible_max_track: false, content_based: true, is_compressible_replaced: false, fixed_track_sum: None, fixed_gutter_sum: None, fixed_track_limit: None }")) {
                return false;
            }
            long generation = style.optLong("generation", -1);
            long document = style.optLong("documentRevision", -1);
            long renderTree = style.optLong("renderTreeRevision", -1);
            long styleRevision = style.optLong("styleRevision", -1);
            long environment = style.optLong("environmentRevision", -1);
            String source = revision.optString("source");
            return generation > 0 && document > 0 && renderTree > 0 && styleRevision >= 0
                    && environment >= 0 && revision.optLong("style", -1) == styleRevision
                    && revision.optLong("environment", -1) == environment
                    && source.startsWith("HostDocument {")
                    && source.contains("DocumentGeneration(" + generation + ")")
                    && source.contains("DocumentRevision(" + document + ")")
                    && source.contains("RenderTreeRevision(" + renderTree + ")");
        } catch (JSONException | ClassCastException error) {
            return false;
        }
    }

    static String presentationIdentity(String report, boolean drawReport) {
        String rootFrameField = drawReport ? "root_css_px=" : "root_frame_css_px=";
        String viewportField = drawReport ? "viewport_css_px=" : "render_viewport_css_px=";
        if (!report.startsWith("status=0 ")
                || !"3".equals(reportField(report, "boxes="))
                || !"300x300".equals(reportField(report, rootFrameField))
                || !"480x320".equals(reportField(report, viewportField))
                || (!drawReport && !"ready".equals(reportField(report, "layout=")))
                || (drawReport && (!report.contains("presented boxes=3 ")
                        || !reportField(report, "surface_texture=").matches("[0-9]+x[0-9]+")))) {
            return "";
        }
        String[] fields = {
            "generation=", "document_revision=", "render_tree_revision=",
            "style_revision=", "environment_revision="
        };
        StringBuilder identity = new StringBuilder("300x300/480x320");
        for (String field : fields) {
            String value = reportField(report, field);
            if (!value.matches("[0-9]+")) return "";
            identity.append('/').append(value);
        }
        return identity.toString();
    }

    private static String reportField(String report, String field) {
        int start = report.indexOf(field);
        if (start < 0) return "";
        start += field.length();
        int end = report.indexOf(' ', start);
        if (end < 0) end = report.indexOf('\n', start);
        return end < 0 ? report.substring(start) : report.substring(start, end);
    }

    private static boolean frameMatches(JSONObject frame, double x, double y,
            double width, double height) {
        return frame != null && near(frame.optDouble("x", Double.NaN), x)
                && near(frame.optDouble("y", Double.NaN), y)
                && near(frame.optDouble("width", Double.NaN), width)
                && near(frame.optDouble("height", Double.NaN), height);
    }

    private static boolean intArrayMatches(JSONArray values, int... expected) {
        if (values == null || values.length() != expected.length) return false;
        for (int index = 0; index < expected.length; index++) {
            if (values.optInt(index, Integer.MIN_VALUE) != expected[index]) return false;
        }
        return true;
    }

    private static boolean stagesMatch(JSONArray stages, double[][] expectedBases,
            boolean stretchApplied) {
        String[] names = {
            "Initialize", "IntrinsicItemBatch { span: 1, crosses_flexible_track: false }",
            "IntrinsicSizingComplete", "MaximizeTracks", "ExpandFlexibleTracks",
            "StretchAutoTracks { applied: " + stretchApplied + " }"
        };
        if (stages == null || stages.length() != names.length) return false;
        for (int index = 0; index < names.length; index++) {
            JSONObject stage = stages.optJSONObject(index);
            JSONArray bases = stage == null ? null : stage.optJSONArray("baseSizesCssPx");
            if (stage == null || bases == null || bases.length() != expectedBases[index].length
                    || !names[index].equals(stage.optString("stage"))) return false;
            for (int track = 0; track < expectedBases[index].length; track++) {
                if (!near(bases.optDouble(track, Double.NaN), expectedBases[index][track])) {
                    return false;
                }
            }
        }
        return true;
    }

    private static boolean contributionsMatch(
            JSONArray values, double[] sizes, String expectedMinimumSource
    ) {
        if (values == null || values.length() != 3) return false;
        String[] kinds = {"Minimum", "MinContent", "MaxContent"};
        for (int index = 0; index < kinds.length; index++) {
            JSONObject contribution = values.optJSONObject(index);
            if (contribution == null || contribution.optLong("item", -1) != 2
                    || !kinds[index].equals(contribution.optString("kind"))
                    || contribution.optInt("span", -1) != 1
                    || contribution.optBoolean("crossesFlexibleTrack", true)
                    || !near(contribution.optDouble("valueCssPx", Double.NaN), sizes[index])) {
                return false;
            }
            if (index > 0 && !contribution.isNull("minimumSource")) return false;
        }
        return expectedMinimumSource.equals(values.optJSONObject(0).optString("minimumSource"));
    }

    private static boolean near(double actual, double expected) {
        return Double.isFinite(actual) && Math.abs(actual - expected) <= EPSILON;
    }
}
