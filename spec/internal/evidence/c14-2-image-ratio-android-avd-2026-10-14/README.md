# C14.2 Android API 37 AVD Rust test · 2026-10-14

## 대상과 ADB 복구 확인

- `adb devices -l` inventory는 `emulator-5554` 한 대였다. 실기기는 연결하지 않았다. 저장된 목록은 [adb-devices.txt](adb-devices.txt)다.
- AVD 이름 `spinon_api37_1_compat16k`, product/model `sdk_gphone16k_arm64`, API 37, ABI `arm64-v8a`, `ro.kernel.qemu=1`, page size `16384`, `sys.boot_completed=1`이었다. 전체 [getprop snapshot](getprop.txt)과 [AVD name](avd-name.txt)을 보관했다.
- ADB client `36.0.2-14143358`로 emulator transport를 사용했다. 이전 device-discovery 실패는 ADB daemon 자체가 아니라 없는 `spinon_api37_2.avd`를 가리키며 `arm` architecture를 쓰던 불완전한 AVD 정의에서 발생했다. 현재 실행은 기존의 정상 `spinon_api37_1_compat16k` arm64 profile을 사용했다. 사용자 AVD 파일은 수정하지 않았다. 원인 조사와 복구 기록은 [C14.1 AVD 근거](../c14-1-android-avd-bootstrap-2026-10-14/README.md)에 있다.

## 빌드와 실행

- Rust target `aarch64-linux-android`; installed NDK `29/AndroidNDK14206865`의 `aarch64-linux-android35-clang` linker로 `cargo test --locked -p spinon-layout --target aarch64-linux-android --no-run`을 실행했다. 출력은 [build log](cargo-test-build.log)다.
- 수정 뒤 재빌드한 ARM64 executable의 SHA-256은 [test-binary-sha256.txt](test-binary-sha256.txt)다. API 37 AVD에서 `intrinsic_size` filter 18/18과 CSS 2.2 min/max table test 1/1을 실행했다. 이번 실행에는 Flex sizing 두 경계와 positioned image의 flow/visual pass 동등성 회귀가 포함된다. 원본은 [intrinsic tests](android-intrinsic-tests.log), [min/max table test](android-css22-table-test.log), [빌드 log](cargo-test-build.log)다.
- 테스트 후 `/data/local/tmp/spinon-layout-c14-2-tests`를 지우고 부재를 확인했다. 마지막 ADB inventory에도 `emulator-5554`만 있었다.

## 판정 경계

Android AVD의 ARM64 Rust unit-test 실행만 확인했다. HostDocument→Stylo→Taffy oracle test는 macOS `spinon-style-to-layout` test executable에서 실행했다. 이 AVD 실행은 APK, V8, 앱 runtime adapter, Android 화면, WGPU/GPU pixels, 성능 또는 실기기 동작을 증명하지 않는다.
