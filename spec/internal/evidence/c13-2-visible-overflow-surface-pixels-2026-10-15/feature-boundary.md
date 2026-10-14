# C13.2 surface pixel FFI feature boundary · 2026-10-15

## 빌드

동일한 `spinon-ffi` 소스와 Rust 1.96.1 toolchain으로 feature-off/default 및 opt-in positive control을 각각 독립 target directory에 빌드했다.

| 빌드 | 명령 feature | `libspinon_ffi.a` SHA-256 |
| --- | --- | --- |
| default | 없음 | `4fb7ebd858f39caff0d920120d45f02f647d776d8aed2a12f64e5185e15d5466` |
| positive control | `c13-2-scroll-fixture-test` | `9673bfc3d77a057d88d0dd93edf0e7f242f9ac4262913f5f8e60a415123f836f` |

재현 명령:

```sh
CARGO_TARGET_DIR=/tmp/spinon-c132-default-target SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 cargo build --locked -p spinon-ffi --lib
CARGO_TARGET_DIR=/tmp/spinon-c132-feature-target SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 cargo build --locked -p spinon-ffi --lib --features c13-2-scroll-fixture-test
```

## 심볼과 문자열

Rust toolchain `llvm-nm -g`에서 default archive에는 아래 두 외부 심볼이 없고, positive control에는 각각 `T`로 나타났다.

- `_spinon_runtime_gpu_host_new_c13_2_scroll_surface_pixel_readback_fixture`
- `_spinon_runtime_gpu_host_request_c13_2_surface_pixels_for_test`

`strings`에서 전용 문구 `C13.2 surface-pixel host`는 default에서 검색되지 않았고 positive control에서만 검색됐다. Apple Xcode `nm`은 Rust LLVM 22 object attribute를 해석하지 못해 결과에 사용하지 않았으며, Rust 1.96.1과 함께 제공되는 `llvm-nm`으로 archive를 확인했다.

## 소스 gate

- Cargo fixture module과 host API는 `c13-2-scroll-fixture-test`로 묶였다. C header, Android JNI, iOS Objective-C++ wrapper 선언·구현은 `SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST` 전처리 조건 아래 있다.
- Android Gradle은 `SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST`를 `0|1`로 제한하고 C04 runtime GPU fixture와 함께만 켠다. 빌드 script가 동일 값을 native define과 Cargo feature에 전달한다.
- iOS Xcode build setting은 같은 `SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST` 값을 native define에 넘긴다. run5는 stale draw 취소 수정까지 포함한 이 opt-in build의 iPhone 17 Pro / iOS 26.2 Simulator 실행이다.

이 검사는 Rust static archive의 FFI symbol/string 경계를 증명한다. 최종 Android APK/iOS app bundle의 크기, stripped product archive, release packaging은 검사하지 않았다.
