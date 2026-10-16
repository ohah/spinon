# C18.2 기본·opt-in artifact 경계 — 2026-10-16

C18.2 Cargo feature, Android APK native library와 iOS Simulator executable을 기본/opt-in 빌드로 대조했다. 검사 대상은 C18.2 C ABI symbol 세 개, fixture JS의 원본 1,020 bytes, PNG의 원본 99 bytes, source URI marker다. 기본 artifact에는 대상이 없고 opt-in positive control에는 모두 있다.

## Cargo staticlib

- 기본: `cargo build --locked -p spinon-ffi --lib`
- opt-in: `cargo build --locked -p spinon-ffi --lib --features c18-2-alpha-image-compositing-fixture-test`
- 기본 staticlib SHA-256 `e5772d0ac9166f19f1fc8f26d5d484428c531d93360c12b51feabcca91425254`, opt-in `052f7eb7b6c711398d2c521da6d62cafa7ee87584086facd06a2a2e612e481c3`.
- [symbol·payload scan 결과](./scan-results.txt).

Apple `nm`은 Rust 1.96.1/LLVM 22 object 속성 일부를 읽지 못했다. 최종 경계 확인은 archive 안의 ABI 이름과 source URI 문자열, JS 파일 전체 bytes, PNG 전체 bytes를 직접 대조했다.

## Android APK

- default: `SPINON_ENABLE_C04_RUNTIME_GPU=1`, C18.1/C18.2 fixture flags `0`.
- opt-in: C18.1/C18.2 fixture flags `1`.
- APK SHA-256: default `9f2dea4a0686acf8b5d6d65d4abf04bb505fa6b75e27a4f2c2086e9281e80601`, opt-in `90dec4de82788547fa9e8750d0448239aca9261755349352fba8ff691f9c175a`.
- arm64 native library 및 각 PNG/JS payload 대조는 [android-app-boundary.txt](./android-app-boundary.txt).

## iOS Simulator app

- default/opt-in 모두 C04 runtime GPU를 켰다. default는 C18.1/C18.2 flags `0`; opt-in은 C18.2를 켜 C18.1 공용 fixture build flag도 활성화했다.
- app executable SHA-256: default `c21569cc7effccbec206d21207f19b15c520e778cbfb0e23dc013661ac746f06`, opt-in `f117783216247c8d73d8906ade6e35279fa859e1952c9bd012432f3405c1d5e4`.
- [app symbol·payload scan 결과](../app-artifact-boundary.txt).

이 검사는 opt-in fixture module이 기본 앱으로 노출되지 않는 경계만 확인한다. 공개 C ABI/API 지원, 일반 image loader, Android 실기기 또는 제품 배포 artifact의 보증은 아니다.
