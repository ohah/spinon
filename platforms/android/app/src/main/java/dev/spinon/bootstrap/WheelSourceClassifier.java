package dev.spinon.bootstrap;

/** Classifies one MotionEvent without consulting the input device's supported-source union. */
final class WheelSourceClassifier {
    static final int UNKNOWN = -1;
    static final int MOUSE = 0;
    static final int TRACKPAD = 1;

    private WheelSourceClassifier() {}

    static int classify(boolean eventFromTouchpad, boolean eventFromMouse,
            boolean hasPointer, boolean pointerIsFinger, boolean pointerIsMouse) {
        if (eventFromTouchpad) return TRACKPAD;
        if (!eventFromMouse || !hasPointer) return UNKNOWN;
        if (pointerIsFinger) return TRACKPAD;
        if (pointerIsMouse) return MOUSE;
        return UNKNOWN;
    }
}
