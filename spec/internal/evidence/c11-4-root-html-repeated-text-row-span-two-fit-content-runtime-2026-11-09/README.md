# Repeated text row-span with two `fit-content()` tracks

## RuntimeSession and Rust checks

- Chrome 157 observations: 3,840/3,840 scenes matched in the parsed Root HTML `RuntimeSession` matrix. The same run included the min-content-floor control and the three-track atomic-rejection/same-session-recovery test (3 tests total, 0 failures; 171.42s).
- Previous single-fit row-span regressions passed: authored positions 1–3 (1,920 scenes), positions 4–6 (4,320 scenes), and the existing 960-scene row-span matrix. The associated rejection and recovery tests passed (4, 3, and 3 tests respectively across the focused commands).
- Layout profile tests passed for all 15 two-position pairs and the fixed tail orders. The five `blink_` distribution/alignment tests passed, including fractional remainder assignment and the used-prefix with no auto track.
- Feature-enabled Clippy passed for `spinon-layout`, `spinon-style-to-layout`, and `spinon-runtime` with `-D warnings -A clippy::too_many_arguments`. The allowance is for two preexisting fixture-test helpers in unrelated files. `cargo fmt --all -- --check` and `git diff --check` passed.
- Chrome 157 control `chrome157-min-content-floor-control.json` has two `fit-content(0.5px)` rows whose min-content floors resolve above their caps. The paired screenshot is observational evidence; RuntimeSession independently checks its row trace, item frames, paint boxes, and text descriptors.

## iOS Simulator

- A disposable iPhone 17 Pro / iOS 26.2 Simulator ran the default Root HTML route. The recorded runtime reached `layout=ready boxes=12`, a `402×220` CSS-pixel root, Metal renderer, and `SPINON_C0410_DRAW ... presented` with four text payloads.
- [Screenshot](ios/screenshot.png) is OS visual smoke only; there is no Metal drawable readback or pixel-parity claim. The runtime markers are in [ios/runtime-markers.log](ios/runtime-markers.log).
- The disposable simulator was shut down and deleted. The preexisting iPhone 17 Pro and Pro Max profiles were left intact and shut down. The iOS 26.2 inventory had no `S03` profile.

## Android Emulator

- The API 37.2 `google_apis_ps16k` disposable AVD stayed offline for 5m39s before it was stopped and deleted; no app was installed in that AVD.
- A follow-up API 37.0 `google_apis` AVD reproduced the failure before app installation. The Mac reports `kern.hv_support=1` and `emulator -accel-check` reports Hypervisor.framework, but Emulator 37.2.12 still said `hvf is not enabled` even with `-accel on`; it repeated `mprotect failed: Permission denied`, stayed offline, then reported QEMU thread hangs and exited with status 139. This narrows the issue to the emulator's AVD/QEMU startup path, not the application package. Details are in [android/api370-emulator-hang.md](android/api370-emulator-hang.md).
- The Android debug APK built successfully but was not installed because QEMU never completed boot. Its exact generated APK was removed after the failed run. Therefore there is no Android app layout/draw or screenshot result for this slice.
- The earlier PackageManager parse failure on the reused AVD is a separate failure and remains un-attributed; its app data was preserved and the original APK restore command returned `Success`.

## Scope and cleanup

- This is a bounded implementation candidate, not general CSS Grid support, WPT conformance, hardware-device parity, or completion of C11.4/C11. The official completion count remains `78/214`.
- No iOS Simulator, Android AVD, or browser session from this verification remains open. The preexisting simulator profiles and unrelated workspace processes were preserved.
