# Android Emulator 37.2.13 follow-up

- SDK Manager offered Emulator 37.2.13 over installed 37.2.12; the package update completed. The new emulator reports build `16740729`. `emulator -accel-check` reports `Hypervisor.Framework OS X Version 26.5` and the host reports `kern.hv_support=1`.
- A disposable Pixel 7 AVD used `android-37.2;google_apis_ps16k;arm64-v8a` and launched with `-accel on`. The startup log still said `hvf is not enabled on this aarch64 host` and repeated `qemu_mprotect__osdep: mprotect failed: Permission denied`.
- The Android console answered `avd name` for `spinon_c114_tmp_20261109_v37213`, so the emulator process and console were responsive. ADB remained `emulator-5554 offline` for 5m08s. `adb reconnect offline` and restarting the ADB server did not make `sys.boot_completed` reachable. QEMU stayed at low CPU and did not complete guest boot.
- The debug APK build succeeded at SHA-256 `f8f39c3d2c629893bc850cb96ce6fd2e6b03164fdfb57c34a78dfa0a1d1129d8` but was not installed. The exact temporary AVD was stopped and deleted; the generated APK was removed afterward.
- The package update did not resolve this AVD boot failure. Evidence points to a QEMU/HVF guest-startup problem; it does not identify why the separate reused-AVD PackageManager scan rejected a prior APK.
