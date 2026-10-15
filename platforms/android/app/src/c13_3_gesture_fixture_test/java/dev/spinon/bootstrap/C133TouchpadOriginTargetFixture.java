package dev.spinon.bootstrap;

import android.util.Log;
import android.view.SurfaceView;
import android.view.ViewGroup;

final class C133TouchpadOriginTargetFixture {
    private static final String TAG = "SpinonBootstrap";

    private C133TouchpadOriginTargetFixture() {}

    static void install(ViewGroup owner, ViewGroup stage, SurfaceView surfaceView,
            ViewGroup.LayoutParams surfaceParams) {
        stage.setClipChildren(false);
        stage.setClipToPadding(false);
        owner.setClipChildren(false);
        owner.setClipToPadding(false);

        Runnable positionOriginTarget = () -> {
            int width = surfaceView.getWidth();
            int height = surfaceView.getHeight();
            if (width <= 0 || height <= 0) {
                Log.e(TAG, "SPINON_C133_TOUCHPAD_ORIGIN_TARGET failed=surface-not-laid-out"
                        + " size=" + width + "x" + height);
                return;
            }
            float scaleX = width / 400.0f;
            float scaleY = height / 240.0f;
            if (!Float.isFinite(scaleX) || !Float.isFinite(scaleY)
                    || scaleX <= 0.0f || scaleY <= 0.0f
                    || Math.abs(scaleX - scaleY) > 0.01f) {
                Log.e(TAG, "SPINON_C133_TOUCHPAD_ORIGIN_TARGET failed=invalid-scale"
                        + " scale=" + scaleX + "," + scaleY
                        + " size=" + width + "x" + height);
                return;
            }
            int[] currentOrigin = new int[2];
            surfaceView.getLocationOnScreen(currentOrigin);
            float targetOriginX = -132.0f * scaleX;
            float targetOriginY = -112.0f * scaleY;
            surfaceView.setTranslationX(surfaceView.getTranslationX()
                    + targetOriginX - currentOrigin[0]);
            surfaceView.setTranslationY(surfaceView.getTranslationY()
                    + targetOriginY - currentOrigin[1]);
            int[] translatedOrigin = new int[2];
            surfaceView.getLocationOnScreen(translatedOrigin);
            Log.i(TAG, "SPINON_C133_TOUCHPAD_ORIGIN_TARGET enabled=true"
                    + " target_css_px=132,112 raw_screen_px=0,0"
                    + " surface_origin_screen_px=" + translatedOrigin[0] + ","
                    + translatedOrigin[1] + " surface_size_px=" + width + "x" + height
                    + " scale=" + scaleX + "," + scaleY);
        };
        surfaceView.addOnLayoutChangeListener((view, left, top, right, bottom,
                oldLeft, oldTop, oldRight, oldBottom) -> positionOriginTarget.run());
        stage.addView(surfaceView, surfaceParams);
        surfaceView.post(positionOriginTarget);
    }
}
