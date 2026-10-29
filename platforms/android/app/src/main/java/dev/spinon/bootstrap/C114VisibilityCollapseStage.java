package dev.spinon.bootstrap;

import java.util.List;

/** C11.4 visibility:collapse Grid fixture의 고정 장면 기대값입니다. */
final class C114VisibilityCollapseStage {
    private static final String[] SCENE_IDS = {
        "collapse-column", "hidden-column", "display-none-column",
        "collapse-visible-descendant-column", "collapse-row", "hidden-row",
        "display-none-row", "collapse-visible-descendant-row", "visible-column", "visible-row",
    };

    private C114VisibilityCollapseStage() {}

    static boolean isScene(String sceneId) {
        if (sceneId == null) return false;
        for (String candidate : SCENE_IDS) {
            if (candidate.equals(sceneId)) return true;
        }
        return false;
    }

    static String expectedRootFrame(String sceneId) {
        return isColumnScene(sceneId) ? "width=80,height=20" : "width=20,height=80";
    }

    static String expectedItemFrame(String sceneId) {
        if (sceneId.startsWith("display-none-")) return "width=0,height=0";
        return isColumnScene(sceneId) ? "width=100,height=20" : "width=20,height=100";
    }

    static int expectedBoxCount(String sceneId) {
        if (sceneId.startsWith("collapse-visible-descendant-")) return 2;
        if (sceneId.startsWith("visible-")) return 3;
        return 1;
    }

    static String rootId(String sceneId) {
        return "c114-grid-visibility-collapse-" + sceneId;
    }

    static boolean surfaceSamplesMatch(
            String sceneId,
            List<SurfaceViewPixelCapture.Sample> samples
    ) {
        SurfaceSample[] expected = surfaceSamples(sceneId);
        if (expected == null || samples.size() != expected.length) return false;
        boolean[] found = new boolean[expected.length];
        String rootId = rootId(sceneId);
        for (SurfaceViewPixelCapture.Sample sample : samples) {
            int index = -1;
            for (int candidate = 0; candidate < expected.length; candidate++) {
                if (expected[candidate].id.equals(sample.id)) {
                    index = candidate;
                    break;
                }
            }
            if (index < 0 || found[index] || !rootId.equals(sample.targetNodeId)
                    || sample.xCssPx != expected[index].x
                    || sample.yCssPx != expected[index].y) return false;
            found[index] = true;
        }
        for (boolean sampleFound : found) if (!sampleFound) return false;
        return true;
    }

    static int[] expectedSurfacePixelRgb(String sceneId, String sampleId) {
        SurfaceSample[] expected = surfaceSamples(sceneId);
        if (expected == null) return null;
        for (SurfaceSample sample : expected) {
            if (sample.id.equals(sampleId)) return sample.rgb.clone();
        }
        return null;
    }

    private static SurfaceSample[] surfaceSamples(String sceneId) {
        if (!isScene(sceneId)) return null;
        if (sceneId.startsWith("collapse-visible-descendant-")) {
            return new SurfaceSample[] {
                new SurfaceSample(sceneId + "-visible-child", 4, 4, 0, 0, 255),
                new SurfaceSample(sceneId + "-outside-root", 81, 10, 255, 255, 255),
            };
        }
        if (sceneId.equals("collapse-column") || sceneId.equals("hidden-column")) {
            return new SurfaceSample[] {
                new SurfaceSample(sceneId + "-inside-root", 40, 10, 240, 240, 240),
                new SurfaceSample(sceneId + "-outside-root", 81, 10, 255, 255, 255),
            };
        }
        if (sceneId.equals("collapse-row") || sceneId.equals("hidden-row")) {
            return new SurfaceSample[] {
                new SurfaceSample(sceneId + "-inside-root", 10, 40, 240, 240, 240),
                new SurfaceSample(sceneId + "-outside-root", 10, 81, 255, 255, 255),
            };
        }
        if (sceneId.equals("visible-column")) {
            return new SurfaceSample[] {
                new SurfaceSample(sceneId + "-visible-overflow", 81, 10, 255, 0, 0),
            };
        }
        if (sceneId.equals("visible-row")) {
            return new SurfaceSample[] {
                new SurfaceSample(sceneId + "-visible-overflow", 10, 81, 255, 0, 0),
            };
        }
        return null;
    }

    private static boolean isColumnScene(String sceneId) {
        return sceneId.endsWith("-column");
    }

    private static final class SurfaceSample {
        final String id;
        final int x;
        final int y;
        final int[] rgb;

        SurfaceSample(String id, int x, int y, int red, int green, int blue) {
            this.id = id;
            this.x = x;
            this.y = y;
            this.rgb = new int[] {red, green, blue};
        }
    }
}
