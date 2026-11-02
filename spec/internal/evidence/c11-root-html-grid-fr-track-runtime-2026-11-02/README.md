# C11.1 Root HTML `fr` runtime evidence

The shared mobile runtime source is [root-html-grid-fr-runtime.html](../../../../tests/fixtures/css/c11/root-html-grid-fr-runtime.html), SHA-256 `a18f14f5f34f5a8d51b2b996b8093ec600ae21b4a25eb524aeac577af73784fc`. Rust tests exercise this exact source through `RuntimeSession::new_runtime_gpu_with_root_html()` and compare the root and item frames. The smaller fixed-size items leave the `fr` tracks to determine the next item origins instead of setting their own base size to the expected track width.

Agent Browser Chrome 156 opened the same source. It observed the root frame `(0,0,130,40)`, used columns `40px 80px`, and marker item frames `(0,0,30,40)` and `(50,0,30,40)`. The browser observation is recorded in [chrome-marker-observation.json](./chrome-marker-observation.json), and the [Agent Browser screenshot](./chrome-marker-source.png) is supplemental visual evidence. This verifies the comparison input; it does not verify either native simulator.

## Native simulator runs

The Android ARM64 emulator (API 37, `sdk_gphone64_arm64`, OpenGL ES through ANGLE/SwiftShader) and iPhone 17 Pro simulator (iOS 26.2, Metal) ran the shared source through the existing parsed Root HTML default-render route. Both packaged source assets had SHA-256 `a18f14f5f34f5a8d51b2b996b8093ec600ae21b4a25eb524aeac577af73784fc`. The repository's C04.12 platform assets were restored after each build and remain unchanged at SHA-256 `38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85`.

| Platform | Build artifact | Runtime observation | Screenshot and raw log |
|---|---|---|---|
| Android | Simulator smoke APK SHA-256 `c99833324bb469a8492197fa684508d73ce35ddb9b5bd1cd481558c60c621c6b` | Source hash matched; `status=0`, `layout=ready`, 5 boxes; draw presented on Android Emulator OpenGL ES (ANGLE/SwiftShader). | [Screenshot](./android/simulator.png) · [Runtime log](./android/runtime.log) · screenshot SHA-256 `0f808a7a85a7bf096281203f13b3918b798011757a55970dca800a6d902bc7cd` |
| iOS | `SpinonBootstrap` binary SHA-256 `373099f7e6cc3fc391d40831ae0919db66b1b863af1a09b1efa11714808f1c30` | Source hash matched; `status=0`, `layout=ready`, 5 boxes; draw presented on iOS Simulator GPU/Metal. | [Screenshot](./ios/simulator.png) · [Runtime log](./ios/runtime.log) · [full simulator log](./ios/simulator.log) · screenshot SHA-256 `c36fc8bf38aa43d9423742adf904d29f841f395a6eebd9cc225da0b302d8396d` |

Sequential Rust and tooling logs: [Root HTML regressions 10/10](./rust/root-html-runtime.log), [Grid projection 47/47](./rust/grid-projection.log), [Taffy library 176/176](./rust/taffy-library.log), [scoped Clippy](./rust/scoped-clippy.log), [workspace rustfmt](./rust/rustfmt-workspace.log), [Taffy rustfmt](./rust/rustfmt-taffy.log), and [diff check](./rust/diff-check.log). The vendored Taffy test build emitted six existing unused-variable/mut warnings from untouched `compute/grid/mod.rs`; no test failed.

The [implementation/runtime failure review](./implementation-runtime-review-20.md) and [separate implementation PR diff review](./implementation-pr-diff-review-30.md) record the bounded checks and remaining verification limits.

The screenshots show the two colored fixed-size markers inside the existing C04.12 demo surface. That demo's native heading and surface scaling are visible; screenshots are visual smoke evidence only, not CSS-pixel geometry or post-compositor pixel-parity proof. Exact root/item geometry and typed track assertions come from the Rust runtime matrix. PixelCopy and Metal drawable readback were not run. The Android and iOS build logs showed the existing runtime-GPU path enabled and their fixture-test build flags disabled; no task-specific fixture ABI, FFI entry, or committed platform asset was added.

A separate default app build was run after restoring both platform assets. The packaged Android and iOS asset hashes equal the original platform HTML hash `38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85`, not the C11 test source hash. The default APK SHA-256 is `5660cd8eea6b7d44302a5a0804d998ae243a1edd698c1cfed96c01f795ee73d7`; the iOS app binary SHA-256 is `373099f7e6cc3fc391d40831ae0919db66b1b863af1a09b1efa11714808f1c30`. Build logs and package asset hashes: [Android/iOS default app artifacts](./product-artifact/).


PR [#1015](https://github.com/ohah/spinon/pull/1015)의 이미지 URL은 Agent Browser에서 두 이미지 모두 `complete=true`와 원본 크기(Android 1080×2400, iOS 1206×2622)로 로드됨을 확인했다. 관측값은 [검증 JSON](./agent-browser-pr-image-check.json)에 저장했다. Agent Browser 세션을 닫고 프로세스 종료를 확인했다. PR check rollup은 비어 있다.
