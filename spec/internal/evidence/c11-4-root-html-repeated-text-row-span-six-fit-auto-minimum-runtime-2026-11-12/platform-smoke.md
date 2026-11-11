# Platform Smoke

Date: 2026-11-12

These checks validate the candidate app build and the default root HTML route on simulators. They do not exercise the six-fit row-span fixture or prove pixel parity for the feature.

## Android

- Emulator: `spinon_api37_0_applied_txn`, API 37, `arm64-v8a`.
- Candidate APK built and installed; default `index.html` route reported ready, three boxes, and presented draw in light and dark appearance.
- Final rebuild SHA-256: `b6b58ffcc773734bc8ba29f3cafeb23845ac7c2e70ae4f501f72d23f1b5abedb`; it matches the APK used for the screenshots.
- Screenshots: `android/android-light.png`, `android/android-dark.png`.
- The pre-existing APK was restored after smoke. Its SHA-256 before and after was `710bc94b5a83daed068959a531c4b5c3f67e2bd7ea43cc140d54d719ae5ba9b6`.
- Original appearance (`Night mode: no`) was restored. The emulator was shut down and the ADB server started for this run was stopped.
- A later cold-start check used `-gpu swiftshader_indirect`, which changed the renderer and caused the AVD's `default_boot` snapshot to be rejected. That cold boot had no `dev.spinon.bootstrap` package installed; no candidate was installed during this check, and the AVD was shut down with no booted process. The earlier APK restoration had been hash-verified before this snapshot fallback.

## iOS

- Simulator: iPhone 17 Pro, iOS 26.2.
- Candidate app opened the default `index.html` route with layout ready, three boxes, and a presented draw in light and dark appearance. Dark-mode logs report the Metal backend.
- Final rebuild SHA-256: `7794724df96dfef0e0f1d3fb273be47c6e0fe28467305de03f904367cdfc5b33`; it matches the executable used for the screenshots.
- Screenshots: `ios/ios-light.png`, `ios/ios-dark.png`.
- The pre-existing app executable was restored after smoke. Its SHA-256 before and after was `fe80d12f74ec1927be468d31ccdfaf945c1b62a349e6e04d60ec365de4134212`.
- App data was restored from the pre-smoke copy and then compared with it; the comparison was empty. Original light appearance was restored, and the simulator was shut down.

## Scope limit

These smoke checks only verify that each candidate app build starts, reads the default root HTML, observes the requested color scheme, lays out the default route, and presents a draw. The six-fit row-span geometry and cross-platform pixel output are covered by the separate Chrome-oracle RuntimeSession matrix; no device-side fixture geometry claim is made here.
