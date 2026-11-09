# API 37.0 Android Emulator follow-up

- A disposable `spinon_c114_tmp_20261109_v370` Pixel 7 AVD used the installed `android-37.0;google_apis;arm64-v8a` image. It had no preexisting app data and the application APK was not installed.
- Android Emulator 37.2.12 reported `hvf is not enabled on this aarch64 host` and repeated `qemu_mprotect__osdep: mprotect failed: Permission denied`. ADB remained `emulator-5554 offline` during startup.
- After about 1m55s, QEMU PID 87687 used 99.5% CPU. The emulator reported the QEMU main loop and CPU0–CPU3 threads hanging, then exited with status 139. The transient ADB entry disappeared with the QEMU process; subsequent `adb shell` calls returned `device not found`.
- The host reports `kern.hv_support=1`, and `emulator -accel-check` reports `Hypervisor.Framework OS X Version 26.5`. A second run with `-accel on` still logged `hvf is not enabled`, remained offline for about 1m25s, reported the same QEMU thread hangs, and exited with status 139. This narrows the failure to Emulator 37.2.12's actual AVD/QEMU startup path; it does not show that macOS lacks Hypervisor.framework.
- Two debug APK builds succeeded but were not installed because neither AVD completed boot. The last generated APK SHA-256 was `f8f39c3d2c629893bc850cb96ce6fd2e6b03164fdfb57c34a78dfa0a1d1129d8`.
- This reproduces the startup failure on a non-16K API 37 image before app installation. It does not establish the cause of the separate PackageManager parse failure on the reused AVD.
- The exact temporary AVD was deleted after the failed attempt. No Android simulator is running.
