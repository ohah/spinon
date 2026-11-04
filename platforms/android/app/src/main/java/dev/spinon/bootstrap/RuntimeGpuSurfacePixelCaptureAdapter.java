package dev.spinon.bootstrap;

import android.app.Activity;
import android.os.Bundle;
import android.os.Handler;
import android.view.SurfaceView;

import java.util.concurrent.Executor;
import java.util.function.BooleanSupplier;
import java.util.function.Consumer;

/** Opt-in extension point for runtime surface capture adapters. */
public interface RuntimeGpuSurfacePixelCaptureAdapter {
    boolean isEnabled(Bundle extras);

    boolean acceptsIntentExtra(Bundle extras, String key);

    void request(
            Activity activity, SurfaceView surfaceView, int width, int height, float density,
            long generation, String drawReport, Handler mainHandler, Executor runtimeQueue,
            BooleanSupplier sourceIsCurrent, Consumer<String> statusCallback);
}
