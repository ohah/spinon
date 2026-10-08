# Synthetic input provenance control

- APK: 0.1.0-bootstrap debug, SHA-256 608fbdf9279ddded605c7129fb0123ce32a65d7452f6ff1b36702b9eed7b198e (installed package pulled from the device).
- Device: Samsung SM-S731N, Android 16 / SDK 36 / SDK_INT_FULL=36.1.
- App: fresh process PID 25537, launched with spinon_r05_async_fence_wait=true.
- Action: one adb shell input tap 540 1170 synthetic input.
- App log: input_source=unknown; input sequence 1, revision 1, submit 1, TransactionStats callback 1, usable async signal 1. target_vsync_id=-1.
- Concurrent raw capture: current sec_touchscreen direct-input device /dev/input/event6 recorded zero lines. The synthetic tap reached the app without a raw touchscreen contact.
- This control was excluded from physical-input and latency samples. adb shell input uses Android InputManager injection (AOSP InputShellCommand: https://android.googlesource.com/platform/frameworks/base/%2B/master/services/core/java/com/android/server/input/InputShellCommand.java); MotionEvent source/device metadata alone is not the provenance check.
