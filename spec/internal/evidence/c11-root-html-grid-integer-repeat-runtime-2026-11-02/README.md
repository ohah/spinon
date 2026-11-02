# C11.1 parsed Root HTML integer `repeat()` runtime 근거

이 실행은 [구현 계획](../../../../plan/c11-root-html-grid-integer-repeat.md)의 같은 7-grid HTML 입력으로 default parsed Root HTML runtime, Android AVD, iOS Simulator를 확인한다. Chrome baseline은 [사전 비교 기록](../c11-root-html-grid-integer-repeat-precomparison-2026-11-02/README.md)에 있다.

## 고정 입력과 산출물 식별

- 입력: `tests/fixtures/css/c11/root-html-grid-integer-repeat-precomparison.html`
- source SHA-256: `f2d0d35e88ce89d64ebaac5d37365ad9621735f54f6399e90a1959a7412ccd5d`
- Android APK `platforms/android/app/build/outputs/apk/debug/app-debug.apk`, SHA-256 `f335d7a0b76f4c903f62b2e2c824cd28dbd83e3eaefde40f2a42058a963cf12f`
- Android APK `assets/index.html` SHA-256은 fixture와 같은 `f2d0d35e88ce89d64ebaac5d37365ad9621735f54f6399e90a1959a7412ccd5d`다.
- iOS executable `build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app/SpinonBootstrap`, SHA-256 `9b7817c23189d711102a12714288603eedffa92ff24058d17430c5a976488fe2`
- iOS bundle `index.html` SHA-256도 같은 fixture hash다.
- 양 app에 임시 번들한 marker `root-app.js` SHA-256은 `c3e817bc8ddbeab5734e353136b94894c0e04d41d0d51ec01242c798e8b88ce5`다. marker는 7개 parsed top-level grid를 확인한 뒤 ready 상태를 보고한다.

## Rust 검증

- [Root HTML runtime grid regression](./rust/root-html-grid-regression-final.log): 30 passed. Positive 7-scene matrix는 computed typed tracks, seven root frames, 28 item frames, 37 total boxes, transparent html/body wrapper, cascade/layout key 일치를 확인한다.
- [Style Root HTML tests](./rust/style-root-html-final.log): 13 passed. 정확히 10,000개 integer-repeat track, allocation failure 복구를 포함한다.
- [Style-to-layout Root HTML tests](./rust/style-to-layout-root-html-final.log): 15 passed.
- [Scoped Clippy](./rust/scoped-clippy-final.log): `spinon-runtime`, `spinon-style-to-layout`, `spinon-style` all targets, `-D warnings` 통과.
- `cargo fmt --all -- --check`와 `git diff --check`도 최종 diff 점검에서 통과했다.

## Android API 37 ARM64 AVD

- emulator `emulator-5556`, Android API 37 / release 17. Build은 `mise exec -- bun run build:android`로 성공했다.
- [Build](./android-avd/build.log), [install](./android-avd/install.log), [launch](./android-avd/launch.log), [runtime](./android-avd/runtime.log).
- Build feature flags에서 `c04_runtime_gpu=1`; C04 Root HTML 및 다른 task fixture flags는 모두 `0`이다.
- Runtime 결과는 `layout=ready`, 37 boxes, root `411.42856×220 CSS px`; `SPINON_ROOT_HTML_DEFAULT_APP_BOOT ... marker=ready`; renderer는 ANGLE/Vulkan SwiftShader software path; draw log는 `presented boxes=37`이다.
- [Simulator screenshot](./android-avd/simulator.png), 1080×2400 pixel, SHA-256 `790c31e6241f6ddf03263f44a3104612a7c460d1e88bc88c46d5a550ce2d62d2`.

## iOS Simulator

- iPhone 17 Pro, iOS 26.2, simulator UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`.
- `bash tools/build-ios-sim.sh`로 build했다. [Build](./ios-simulator/build.log), [install](./ios-simulator/install.log), [launch](./ios-simulator/launch.log), [runtime](./ios-simulator/runtime.log).
- Build feature flags에서 `c04_runtime_gpu=1`; Root HTML 및 다른 task fixture flags는 모두 `0`이다. Build log에는 기존 duplicate-symbol 및 AppIntents metadata warning이 남는다.
- Runtime 결과는 `layout=ready`, 37 boxes, root `402×220 CSS px`; marker가 ready를 보고하고 renderer는 simulator Metal GPU; draw log는 `presented boxes=37`이다.
- [Simulator screenshot](./ios-simulator/simulator.png), 1206×2622 pixel, SHA-256 `ad3abfe60ba8f9aec8aa6d67e117208a5c08bd9984bdf00b68d3434629f7fe47`.

## 복구와 증거 한계

- Simulator 실행에만 `examples/bootstrap/index.html`과 `root-app.js`를 잠시 fixture/marker 파일로 대체했다. 실행 뒤 [원본 백업](./original-platform-assets/)과 비교해 repo의 두 파일을 byte-for-byte 복원했다. `index.html` SHA-256은 `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`, `root-app.js`는 `b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`다.
- Android와 iOS app 프로세스는 종료 상태를 확인했다. 이전 Chrome 사전 비교 Agent Browser 세션도 종료했고 이 증거를 위해 browser를 다시 시작하지 않았다.
- 제출한 build/test log 사본은 diff 검사를 위해 trailing whitespace와 EOF의 중복 빈 줄만 정리했다. command output의 실질 내용은 바꾸지 않았으며, 정리 전 원본 log는 로컬 `build/spinon/artifacts/c11-root-html-grid-integer-repeat/raw-logs/`에 보존했다.
- 화면 screenshot과 draw/present는 simulator smoke evidence다. Simulator에서 item별 surface pixel readback을 하지 않았으며, Android 결과는 hardware GPU가 아니라 SwiftShader다. iOS screenshot은 compositor 이후 image지만 Chrome과의 pixel equality 측정은 아니다.
- 공식 `spec/STATUS.md`의 C11/C11.1 완료 checkbox와 `72/213` 집계는 이 제한 slice만으로 변경하지 않는다.
