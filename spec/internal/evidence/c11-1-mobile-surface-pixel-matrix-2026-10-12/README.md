# C11.1 10 scene 모바일 표면 픽셀 검증 · 2026-10-12

## 결과

고정 Chromium 154 DPR 1 screenshot에서 선택한 C11.1 scene 10개·표면 표본 36개를 Android API 37 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 각각 비교했다. 두 환경 모두 10/10 scene·36/36 RGBA exact, 최대 channel error 0이었다.

- [Android API 37 AVD · PixelCopy 10/10·36/36](./android-avd-c11-pixels-20261012T014701830Z-e435929f/README.md)
- [iPhone 17 Pro / iOS 26.2 Simulator · Metal 10/10·36/36](./ios-simulator-c11-pixels-20261012T014858759Z-38b0e689/README.md)
- [구현 후 실패 관점 검토 20개](./implementation-review-2026-10-12.md)
- [fixture·비교기·workspace 최종 재검사](./validation-current-source/final-test.log)
- [Clippy](./validation-current-source/clippy-final.log) · [rustfmt](./validation-current-source/rustfmt-final.log)

## 판정 범위

- Android 캡처는 API 37 AVD의 SurfaceView PixelCopy다. renderer는 ANGLE/SwiftShader였다. 하드웨어 GPU와 Android 실기기 결과로 일반화하지 않는다.
- iOS 캡처는 iOS 26.2 Simulator의 WGPU Metal drawable texture이며 compositor 제출 전이다. iPhone 실기기나 최종 display의 color-management 결과는 측정하지 않았다.
- 두 실행은 현재 ADB에서 연결된 단일 Android AVD와 고정 iPhone 17 Pro Simulator를 각각 별도 process로 사용했다.
- 이 fixture는 C11.1의 고정 장면·표본만 검증한다. Spinon WPT runner, 전체 Grid/CSS 적합성 및 공개 CSS 지원은 판정하지 않는다.

## 전체 회귀 실행

`bun run test` 결과: JavaScript 2/2, CSS reference 302/302, Android touch benchmark 29/29, Rust workspace 596 passed·3 ignored. `cargo clippy --locked --workspace --all-targets -- -D warnings`와 `cargo fmt --all -- --check`도 통과했다. 전체 로그는 `validation-current-source/`에 보존했다.
