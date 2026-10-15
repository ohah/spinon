# C13.3 fixture artifact 경계 · 2026-10-15

기본 경로와 fixture opt-in 경로를 각각 현재 source에서 다시 빌드했다. 검사 문자열은 다음 C13.3 전용 FFI symbol과 runtime payload다.

- `spinon_runtime_gpu_host_new_c13_3_gesture_fixture`
- `c13_3_gesture_fixture=ready`
- `c13_3_input_batch=processed`

## Host static library

| 빌드 | SHA-256 | 위 세 문자열의 개수 | 판정 |
| --- | --- | --- | --- |
| `cargo build --locked -p spinon-ffi --lib` | `6b0d24f2092a2cdac553068a01129f590dabf4e69619262fec3536d93f4c37af` | `0 / 0 / 0` | 기본 산출물에서 모두 없음 |
| `cargo build --locked -p spinon-ffi --lib --features c13-3-gesture-fixture-test` | `4bc5a6cac3a855c18c3759f9d97949d55a05ebb31746991e19a59d9ae3e66535` | `3 / 1 / 1` | opt-in 양성 대조 |

원본 build log 파일은 `host-ffi-build-default.log`, `host-ffi-build-optin.log`다.

## Android API 37 ARM64 AVD APK

두 빌드 모두 `SPINON_ENABLE_C04_RUNTIME_GPU=1`, 고정 V8 source, 같은 worktree를 사용했다. 기본 빌드는 C13.3 flag를 unset했고 opt-in 빌드는 `SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=1`로 실행했다.

| 빌드 | APK SHA-256 | 모든 DEX + native `.so`의 symbol / ready / batch 개수 | 판정 |
| --- | --- | --- | --- |
| 기본 | `3fe1c1e0bcdd4f051861797f29946033ed20d7f7f1f5544387126fc80c2afe6d` | `0 / 0 / 0` | 기본 artifact 깨끗함 |
| opt-in | `6c73e9e73a2564450b0762ba964fa086b33ec2b9339a24ca9a4c822d4aa213ab` | `1 / 2 / 1` | opt-in 양성 대조 |

기본 APK는 Java `BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST` guard가 ready marker와 touch listener 양쪽에 적용된 뒤 다시 빌드했다. 이 guard 전 검사에서 `classes2.dex`에 ready marker가 남았고, 그 문제를 이 변경에서 수정했다. 검사는 APK의 `classes.dex`, `classes2.dex`, `lib/arm64-v8a/libspinon_bootstrap.so` 전체를 대상으로 했다. 원본 log는 `android-api37-avd-build-default.log`, `android-api37-avd-build-optin.log`다.

## iOS Simulator archive와 app executable

두 빌드 모두 C04.10 GPU fixture는 켜고, 기본은 `c13_3_gesture_fixture=0`, opt-in은 `=1`로 컴파일했다.

| 빌드 | FFI archive SHA-256 | App executable SHA-256 | 전용 문자열 검사 |
| --- | --- | --- | --- |
| 기본 | `4a77134f9ac63f5b4e3276d0dcf50e4fa09826bb73a8968f54f6219c5fabf56b` | `9cfbf8dc8f21b827c57abb6b2c860d285f9739f58624ddc4123808613b55b83e` | symbol·marker·payload 모두 없음 |
| opt-in | `a3b9436433659ed6a93941ba91325374fc8cd36815ec661a21b08ebfc6edcc03` | `dc59209a82cf0c3d0dbf332c32358e36a86bb95a55a9d8792850190b5e7e17c1` | symbol / marker / payload `1 / 2 / 1` |

원본 build log 파일은 `ios-iphone17pro-build-default.log`, `ios-iphone17pro-build-optin.log`다. 기존 Apple LLVM 도구는 Rust LLVM 22 archive 속성을 읽지 못해 `nm` 출력 대신 archive와 executable의 정확한 byte string 검사를 썼다.

## 한계

이 검사는 지정 문자열이 기본 산출물에서 빠지고 opt-in에 들어가는지 확인한다. 공개 CSS 지원, WPT 적합성, C13.3 전체 완료를 뜻하지 않는다. Android AVD와 iOS Simulator 산출물이며 실기기 빌드 결과가 아니다.
