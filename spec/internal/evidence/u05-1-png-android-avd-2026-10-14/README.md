# U05.1 PNG decoder Android AVD 실행 · 2026-10-14

## 대상

- Android AVD serial: `emulator-5554`.
- AVD product/model: `sdk_gphone16k_arm64`.
- Android API: 37.
- ABI: `arm64-v8a`.
- AVD profile: `spinon_api37_1_compat16k`.
- attached-device inventory에서 확인한 대상은 이 AVD 한 대였다. 실기기 serial에는 명령을 보내지 않았다([ADB inventory](./adb-devices.txt)).
- compiler target: Rust `aarch64-linux-android`.
- NDK target linker: `aarch64-linux-android35-clang`; AVD보다 낮은 API baseline으로 빌드해 API 37 AVD에서 실행했다.

## 실행

```sh
cargo check --locked -p spinon-image --target aarch64-linux-android
CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=/opt/homebrew/Caskroom/android-ndk/29/AndroidNDK14206865.app/Contents/NDK/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android35-clang cargo test --locked -p spinon-image --target aarch64-linux-android --no-run
adb -s emulator-5554 push target/aarch64-linux-android/debug/deps/spinon_image-032333ec6b2c7771 /data/local/tmp/spinon-image-u05-1-tests
adb -s emulator-5554 shell chmod 700 /data/local/tmp/spinon-image-u05-1-tests
adb -s emulator-5554 shell /data/local/tmp/spinon-image-u05-1-tests --nocapture
```

현재 코드로 Android ARM64 test executable을 다시 빌드해 실행했다. `cargo check`와 `--no-run` 빌드 로그는 [`cargo-check.log`](./cargo-check.log), [`cargo-test-build.log`](./cargo-test-build.log)에 있고, binary SHA-256은 [`test-binary-sha256.txt`](./test-binary-sha256.txt)에 있다. test output은 [`android-test.log`](./android-test.log)에 보존했다. decoder·request·snapshot 경계 10개가 Android에서 `10 passed, 0 failed`로 끝났다. 실행 파일은 테스트 후 `/data/local/tmp/spinon-image-u05-1-tests`에서 제거했으며 `adb shell test ! -e`로 부재를 확인했다.

## ADB 실패 원인

ADB client `36.0.2`와 daemon/transport는 동작했다. 최초 ADB inventory가 비어 있었던 원인은 선택된 `spinon_api37_2.ini`가 존재하지 않는 `/Users/yoonhb/.android/avd/spinon_api37_2.avd`를 가리키는 잘못된 AVD 정의였다. 기존 로그에는 그 profile의 지원되지 않는 `arm` architecture 오류도 남아 있다. 실제 디코더 실행은 유효한 `spinon_api37_1_compat16k` arm64 AVD를 사용해 `emulator-5554`로 연결했고, 현재 inventory에서도 이 AVD 하나만 확인했다. 별도 bootstrap 재현과 복구 기록은 [C14.1 AVD 근거](../c14-1-android-avd-bootstrap-2026-10-14/README.md)에 있다. 잘못된 profile은 현재 실행 경로에 사용하지 않았고, ADB executable이나 사용자 AVD 파일은 수정하지 않았다.

이전 첫 `--no-run` 시도는 없는 `darwin-arm64/.../aarch64-linux-android37-clang` 경로를 사용해 linker not found로 실패했다. 설치된 NDK는 `darwin-x86_64` prebuilt와 API 35 target clang wrapper를 제공했고, 이 linker로 재빌드·AVD 실행에 성공했다. 이번 소스 갱신 중에는 namespaced-src test의 namespace 인자 타입 오류가 첫 cross-test build에서 드러났다([실패 로그](./cargo-test-build-before-test-fix.log)). 소유 `String`으로 고친 뒤 최종 build와 AVD 실행이 통과했다. 두 실패 모두 ADB daemon/transport 원인이 아니었다.

## 한계

Rust unit-test executable을 AVD OS에서 직접 실행한 결과다. APK/V8/runtime route, `ImageResourceSnapshot`의 Stylo/Taffy 통합, 화면 표시, GPU pixels는 실행하지 않았다. C14.1 HostDocument→Stylo→Taffy adapter는 macOS focused test로 별도 확인했다. 따라서 Android 앱 기능·UI·GPU 통합 근거로 해석하지 않는다.
