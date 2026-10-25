# iOS C12.5 content-edge Metal gate · 2026-10-26

## Build and runtime identity

- Simulator: designated iPhone 17 Pro `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`, iOS 26.2 (23C54). The other booted iPhone 17 Pro simulator (`9DCCE836-7659-4914-B6C1-13F54BFFA9DB`) was not targeted.
- Build: `xcodebuild -project platforms/ios/SpinonBootstrap.xcodeproj -scheme SpinonBootstrap -configuration Debug -destination 'platform=iOS Simulator,id=ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D' -derivedDataPath DerivedData SPINON_V8_ROOT=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 SPINON_ENABLE_C12_5_STICKY_FIXTURE_TEST=1 CODE_SIGNING_ALLOWED=NO build` — passed. The log records some existing duplicate-symbol link warnings and redundant conditional-cast warnings; no build errors occurred.
- The final source was rebuilt after removing one new redundant conditional cast from the C125 Metal sample reader. Build passed; that warning is absent from the final build log. Existing duplicate-symbol warnings, a deprecated UIKit callback, unrelated redundant casts, and V8 debug-map warnings remain.
- Source fingerprint: 35 changed source/config/fixture files, SHA-256 `c31f9f5351f0431db1f75f746faeb411afc4378d4cfe433ffc2be815bf75a308`.
- Built app executable SHA-256: `4b77055e541db8adbaa9d75004150a91946469e18dbf68e839656a4bc25b181b`. The installed app executable was read from this Simulator's app container and had the same hash.
- V8 revision: `7b50b62cb18f28617959e8452e2cd18195b38bcf`.
- The pinned Chrome oracle screenshot SHA-256 is `16a6e256b15a68bf8dce745c0243b312bd295c78bd585ae2440cabfb9c2c4f61`.

## Result

The final-source opt-in `--spinon-c125-sticky` route launched as PID `43715`. Runtime assertions passed with section border bottom `82`, content bottom `68`, sticky top `40`, and the expected `section` / `section-after` hit targets. The readback came from the `1080×780` `Bgra8Unorm` Metal drawable at surface generation 1, presentation sequence 7, scroll revision 5 and offsets `[0,0,0,212]`.

| CSS point | Metal drawable point | Hit target | Chrome / iOS RGBA |
| --- | --- | --- | --- |
| `(200,78)` | `(601,235)` | `section` | `(245,245,245,255)` |
| `(240,100)` | `(721,301)` | `section-after` | `(204,221,238,255)` |

Result: **2/2 exact** against the pinned Chrome pixels. The Metal readback is before compositor submission; it does not prove hardware GPU or scanout output. The final simulator screenshot, runtime log, build log, launch output and source/app/Chrome hashes are linked from the [final manifest](./ios-content-edge-metal-manifest-2026-10-26.json). The earlier successful run at the previous source hash is retained in [`ios-content-edge-metal-pre-cast-cleanup-manifest-2026-10-26.json`](./ios-content-edge-metal-pre-cast-cleanup-manifest-2026-10-26.json) and superseded by this final-source capture.

The earlier first-launch route failure remains preserved in `ios-first-launch-route-miss.*` and is explicitly excluded from this evidence. After final capture the app was terminated and only the designated simulator was shut down; the other booted simulator remains unchanged. Release state is in [`ios-simulator-release-final-2026-10-26.txt`](./ios-simulator-release-final-2026-10-26.txt).
