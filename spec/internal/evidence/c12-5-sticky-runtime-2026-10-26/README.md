# C12.5 runtime evidence · 2026-10-26

This evidence covers a feature-gated top-only sticky profile and the parent content-edge boundary case. The current whole-source fingerprint is `c31f9f5351f0431db1f75f746faeb411afc4378d4cfe433ffc2be815bf75a308` (35 changed source/config/fixture files). The Android PixelCopy run was captured at the prior whole-source fingerprint `5dbd898c…`; only an iOS C125 reader cleanup changed afterward, and the Android target subset remains unchanged at 29 files / `d72a3963…`. The final iOS Metal run uses the current `c31f9f53…` fingerprint. The independent 20-view implementation review was refreshed at the current fingerprint with no new source blocker. C12.5 remains incomplete: PR diff review 30 and PR creation are pending, the profile is not general author CSS admission, and C12/C13 parent status and official `72/213` count remain unchanged.

## Chrome boundary oracle

The standalone parent padding/border fixture was observed through Agent Browser with HeadlessChrome 156. At `inner.scrollTop=212`, section border bottom is `82`, content bottom is `68`, and sticky border box is `[40,68]`. `(200,78)` resolves to section and screenshot RGBA `(245,245,245,255)`; `(240,100)` resolves to section-after and `(204,221,238,255)`. Chrome screenshot SHA-256: `16a6e256b15a68bf8dce745c0243b312bd295c78bd585ae2440cabfb9c2c4f61`. The prior border-box clamp predicted top `54`, 14px below the observed top `40`. Fixture, screenshot and observation details are in the [Chrome precomparison evidence](../c12-5-sticky-precomparison-2026-10-26/README.md) and [repro record](./parent-content-edge-repro.json).

## Android API 37 ARM64 AVD

The opt-in route on `emulator-5560` passed the runtime geometry/clip/hit assertions and Android PixelCopy color gate. The AVD uses SwiftShader software rendering. Both pixel centers match Chrome exactly: `(200,78)` → `(526,206)` is section `(245,245,245,255)`; `(240,100)` → `(631,264)` is section-after `(204,221,238,255)`. The installed `base.apk` hash matched the local APK; the manifest binds the capture to that APK, its embedded ARM64 native library, capture-time whole-source fingerprint, unchanged Android target-source fingerprint, run ID and readback PNG hash.

- [Android run details, logs, PNG and manifest](./android-content-edge-pixelgate-20261026/README.md)
- [Runtime log](./android-content-edge-pixelgate-20261026/android-runtime-pixelcopy.log)
- [PixelCopy PNG](./android-content-edge-pixelgate-20261026/pixelcopy.png)

After capture the AVD was stopped. `adb devices -l` showed no emulator; the already attached physical Samsung was left untouched. Agent Browser sessions and Agent Browser/Chrome automation processes were absent. The final device/process state is recorded in [`process-state-final-2026-10-26.txt`](./process-state-final-2026-10-26.txt).

## iPhone 17 Pro Simulator Metal drawable

The designated iPhone 17 Pro Simulator (`ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`, iOS 26.2) rebuilt the current source and passed the native runtime assertions plus Metal drawable readback. Both samples match Chrome exactly: `(200,78)` → `(601,235)` is section `(245,245,245,255)`; `(240,100)` → `(721,301)` is section-after `(204,221,238,255)`. Drawable is `1080×780`, `Bgra8Unorm`; the renderer reads it before compositor submission.

- [Final-source iOS build, app/Chrome hashes, screenshot and log](./ios-content-edge-metal-2026-10-26.md)
- [Final-source hash manifest](./ios-content-edge-metal-manifest-2026-10-26.json)
- [Final-source Simulator screenshot](./ios-content-edge-metal-final-2026-10-26.png)
- [Final-source runtime log](./ios-content-edge-metal-final-2026-10-26.log)
- [Final-source build log](./ios-content-edge-metal-build-final-2026-10-26.log)

The built and installed app executable hashes match: `4b77055e541db8adbaa9d75004150a91946469e18dbf68e839656a4bc25b181b`. The earlier failed route attempt log and screenshot remain preserved and explicitly excluded from C12.5 evidence. The iOS Simulator result does not establish hardware GPU, compositor-after-submit, or scanout output.

## Other evidence

- [Final-source implementation review · 20 perspectives](./final-source-implementation-review-20-2026-10-26.md) (fingerprint `c31f9f53…`; no new blocker)
- [Prior post-fix review · 20 perspectives](./post-fix-implementation-review-20-2026-10-26.md) (fingerprint `5dbd898c…`; retained as historical)
- [Default-versus-opt-in static archive boundary](./artifact-boundary.json)
- [Failed first iOS route attempt](./ios-first-launch-route-miss.log) (preserved, excluded from success evidence)
