package dev.spinon.bootstrap;

final class WheelSourceClassifierJvmTest {
    private static int assertions;

    private WheelSourceClassifierJvmTest() {}

    private static void expect(String name, int actual, int expected) {
        assertions++;
        if (actual != expected) {
            throw new AssertionError(name + ": expected=" + expected + " actual=" + actual);
        }
        System.out.println("case=" + name + " result=" + actual);
    }

    public static void main(String[] args) {
        expect("explicit-touchpad-source", WheelSourceClassifier.classify(
                true, false, true, false, false), WheelSourceClassifier.TRACKPAD);
        expect("mouse-source-finger-tool", WheelSourceClassifier.classify(
                false, true, true, true, false), WheelSourceClassifier.TRACKPAD);
        expect("mouse-source-mouse-tool", WheelSourceClassifier.classify(
                false, true, true, false, true), WheelSourceClassifier.MOUSE);
        expect("mouse-source-unknown-tool", WheelSourceClassifier.classify(
                false, true, true, false, false), WheelSourceClassifier.UNKNOWN);
        expect("unknown-source-mouse-tool", WheelSourceClassifier.classify(
                false, false, true, false, true), WheelSourceClassifier.UNKNOWN);
        expect("mouse-source-no-pointer", WheelSourceClassifier.classify(
                false, true, false, false, false), WheelSourceClassifier.UNKNOWN);

        boolean compositeDeviceSupportsTouchpad = true;
        int legacySource = compositeDeviceSupportsTouchpad
                ? WheelSourceClassifier.TRACKPAD
                : WheelSourceClassifier.MOUSE;
        int eventSourceResult = WheelSourceClassifier.classify(
                false, true, true, false, true);
        expect("composite-device-mouse-negative-control", eventSourceResult,
                WheelSourceClassifier.MOUSE);
        assertions++;
        if (legacySource == eventSourceResult) {
            throw new AssertionError("legacy device-source union unexpectedly matched event source");
        }
        System.out.println("case=legacy-device-source-union result=" + legacySource
                + " expected_mismatch=true");
        System.out.println("assertions=" + assertions + " status=passed");
    }
}
