# Android C12.5 content-edge PixelCopy gate · 2026-10-26

## Build and runtime identity

- AVD: `spinon_api37_2_presenttime_probe_20261018`, attached as `emulator-5560`; Android 17 / API 37, ARM64, 16 KiB pages. The physical Samsung remained untouched.
- Build: `SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C12_5_STICKY_FIXTURE_TEST=1 mise exec -- bun run build:android` — passed. The build log records C04 runtime and fixture gates enabled and C12.5 enabled; unrelated fixture gates are disabled.
- Capture-time whole-source fingerprint: 35 changed source/config/fixture files, SHA-256 `5dbd898cddc5c496dea7b729b532dafd7af94b72132ccc08b3a244f253191e3d`.
- Current whole-source fingerprint after a later iOS-only warning cleanup: 35 files, SHA-256 `c31f9f5351f0431db1f75f746faeb411afc4378d4cfe433ffc2be815bf75a308`. The Android target subset is still 29 files, SHA-256 `d72a396378a7460ba274150d4d90c7418e5d0353c94779b7f0dbbed2fd21c7a9`; the sole source change after this Android capture removed a redundant conditional cast from the iOS Metal reader. No Android, shared Rust, or fixture source changed, so the installed APK/native evidence remains bound to the unchanged Android target subset.
- APK SHA-256: `c25bdb06ce6ad6cbed9b9634ade38c830187a3e8939593cf8cea3d4dc5786cf3`. The installed `base.apk` was pulled from the AVD and had the same hash.
- Native input SHA-256: `build/spinon/android/jniLibs/arm64-v8a/libspinon_bootstrap.so` → `363d4bf86a3abed11a538b8c96ac8dc1335210cec1ee3760af8ba5df8244e60d`. The stripped ARM64 library embedded in the APK hashes to `8d67dec0ac15c0017f7249685a215ad3d2e496b890015cc53b4651809b7437b3`.
- The pinned Chrome oracle screenshot SHA-256 is `16a6e256b15a68bf8dce745c0243b312bd295c78bd585ae2440cabfb9c2c4f61`.

## Result

The opt-in route launched successfully as run `96932dd07`. Native runtime assertions reported section border bottom `82`, content bottom `68`, sticky top `40`, and the expected `section` / `section-after` hit targets. The Vulkan device was Goldfish GFXStream with SwiftShader software rendering.

PixelCopy read the final 945×683 app surface from a 360×260 CSS viewport at scroll revision 5, offsets `[0,0,0,212]`. Both CSS pixel centers matched the pinned Chrome colors exactly:

| CSS point | PixelCopy point | Hit target | Chrome / Android RGBA |
| --- | --- | --- | --- |
| `(200,78)` | `(526,206)` | `section` | `(245,245,245,255)` |
| `(240,100)` | `(631,264)` | `section-after` | `(204,221,238,255)` |

Result: **2/2 exact**. The PNG is [`pixelcopy.png`](./pixelcopy.png), SHA-256 `193a35a2e3c20c032c922b3bf7c3add4d4a101fb209e6fae3e7b27e257307381`. Run details and artifact hashes are in [`manifest.json`](./manifest.json); filtered log and build log are alongside it.

This proves the source-bound PixelCopy surface samples on this software-rendered AVD. It does not claim physical-device GPU or post-compositor scanout behavior.
