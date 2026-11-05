# C11 Root HTML intrinsic auto-row text: simulator runtime

This run used the default Root HTML application route with the fixture as the bundled `index.html`. The fixture source SHA-256 was `f0759d21c51af46497327686a854b52cd5e004134c35fd562e3e0170c8df49e6` in both installed app bundles. A temporary, DOM-independent `root-app.js` smoke marker was used because the repository's normal smoke script requires the unrelated `#app` document. Both tracked bootstrap files were restored to their original Git contents after the builds.

## Results

- Android API 37 ARM64 AVD (`emulator-5554`): default app route, `layout=ready`, `boxes=7`, root-app evaluation passed, and the renderer initialized on the Android Emulator / SwiftShader backend. The screen capture is `android/screenshot.png` (1080×2400).
- iPhone 17 Pro / iOS 26.2 Simulator (`9DCCE836-7659-4914-B6C1-13F54BFFA9DB`): default app route, `layout=ready`, `boxes=7`, root-app evaluation passed, and the renderer initialized on the Apple iOS simulator GPU / Metal backend. The screen capture is `ios/screenshot.png` (1206×2622).
- Both screenshots show the red 10px explicit row, blue 20px text item, and green 34px item overflowing below the 40px Grid container. Android wraps the six `A` glyphs as four plus two; iOS wraps them as three plus three. Both platforms measure two lines, so the bounded min-content contribution remains 20 CSS px. Font metrics and line breaks are platform observations, not a shared browser constant.
- Rust style/layout trace and runtime tests verify the exact item frames `[0,0,40,10]`, `[0,10,40,20]`, `[0,30,40,34]`, min-content contribution `20`, paint ordering, and visible overflow to y=64px.

## Evidence boundary

The app logs and screenshots establish default-route startup, `layout=ready`, renderer initialization, and visible drawing on each simulator. The screenshots are simulator display captures. No Android PixelCopy or iOS Metal drawable readback was collected, so this evidence does not claim final compositor scanout parity or pixel-level surface readback. The builds used `c04_runtime_gpu=1`; all CSS fixture-test build flags shown in the build logs were disabled.

`android/app.log.gz`, `android/build.log.gz`, `ios/app.log.gz`, and `ios/build.log.gz` preserve the complete raw logs byte-for-byte. `SHA256SUMS` covers those logs, install/launch records, asset hashes, environment record, and screenshots.
