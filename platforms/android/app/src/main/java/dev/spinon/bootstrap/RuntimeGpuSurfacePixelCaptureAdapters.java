package dev.spinon.bootstrap;

import android.os.Bundle;

import java.util.ArrayList;
import java.util.List;
import java.util.ServiceLoader;

final class RuntimeGpuSurfacePixelCaptureAdapters {
    private static final List<RuntimeGpuSurfacePixelCaptureAdapter> ADAPTERS = loadAdapters();

    private RuntimeGpuSurfacePixelCaptureAdapters() {}

    static boolean acceptsIntentExtra(Bundle extras, String key) {
        List<RuntimeGpuSurfacePixelCaptureAdapter> enabled = enabledAdapters(extras);
        return enabled.size() == 1 && enabled.get(0).acceptsIntentExtra(extras, key);
    }

    static RuntimeGpuSurfacePixelCaptureAdapter singleEnabledAdapter(Bundle extras) {
        List<RuntimeGpuSurfacePixelCaptureAdapter> enabled = enabledAdapters(extras);
        return enabled.size() == 1 ? enabled.get(0) : null;
    }

    private static List<RuntimeGpuSurfacePixelCaptureAdapter> enabledAdapters(Bundle extras) {
        if (extras == null) return List.of();
        List<RuntimeGpuSurfacePixelCaptureAdapter> enabled = new ArrayList<>();
        for (RuntimeGpuSurfacePixelCaptureAdapter adapter : ADAPTERS) {
            if (adapter.isEnabled(extras)) enabled.add(adapter);
        }
        return enabled;
    }

    private static List<RuntimeGpuSurfacePixelCaptureAdapter> loadAdapters() {
        List<RuntimeGpuSurfacePixelCaptureAdapter> adapters = new ArrayList<>();
        for (RuntimeGpuSurfacePixelCaptureAdapter adapter :
                ServiceLoader.load(RuntimeGpuSurfacePixelCaptureAdapter.class)) {
            adapters.add(adapter);
        }
        return List.copyOf(adapters);
    }
}
