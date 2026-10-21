# C11.4 anonymous text + multiple spanning image 실행 근거

2026-10-21에 고정한 네 장면을 같은 Grid document revision에서 실행했다. Android 실기기 대신 사용자가 선택한 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 사용했다.

## 결과

| 장면 | 예상/관측 Columns | 익명 text frame | Android AVD | iOS Simulator |
| --- | --- | --- | --- | --- |
| `anonymous-text-two-images-forward` | `[12, 28, 20, 0]` | `(40, 0) 20×10` | 통과 · [화면](android-avd-c114-anonymous-text-multi-image-android-avd-20261021T055635581Z-d858eeca/anonymous-text-two-images-forward.png) | 통과 · [화면](ios-simulator-c114-anonymous-text-multi-image-ios-simulator-20261021T055343765Z-c2f1a5a4/anonymous-text-two-images-forward.png) |
| `two-images-anonymous-text-reversed` | `[12, 28, 20, 0]` | `(40, 0) 20×10` | 통과 · [화면](android-avd-c114-anonymous-text-multi-image-android-avd-20261021T055635581Z-d858eeca/two-images-anonymous-text-reversed.png) | 통과 · [화면](ios-simulator-c114-anonymous-text-multi-image-ios-simulator-20261021T055343765Z-c2f1a5a4/two-images-anonymous-text-reversed.png) |
| `anonymous-text-three-images-forward` | `[18, 34, 20, 0]` | `(52, 0) 20×10` | 통과 · [화면](android-avd-c114-anonymous-text-multi-image-android-avd-20261021T055635581Z-d858eeca/anonymous-text-three-images-forward.png) | 통과 · [화면](ios-simulator-c114-anonymous-text-multi-image-ios-simulator-20261021T055343765Z-c2f1a5a4/anonymous-text-three-images-forward.png) |
| `three-images-anonymous-text-reversed` | `[18, 34, 20, 0]` | `(52, 0) 20×10` | 통과 · [화면](android-avd-c114-anonymous-text-multi-image-android-avd-20261021T055635581Z-d858eeca/three-images-anonymous-text-reversed.png) | 통과 · [화면](ios-simulator-c114-anonymous-text-multi-image-ios-simulator-20261021T055343765Z-c2f1a5a4/three-images-anonymous-text-reversed.png) |

모든 실행에서 root frame은 `220×180 CSS px`, 행 track은 `[60, 60, 60]`, intrinsic-size revision은 `1`, `drawPresented=true`였다. 각 결과 JSON과 scene별 원본 런타임 로그는 플랫폼별 디렉터리에 있다. Android 최종 설치 APK의 SHA-256은 로컬 APK와 일치한다.

## 환경과 검증

- Android: `emulator-5558`, API 37 / Android 17, `sdk_gphone64_arm64`, `arm64-v8a`. APK SHA-256과 설치된 `base.apk` SHA-256은 모두 `90f0019dfccbddc6b7a756adf516d7b96de5a6357b22bc90cf917a34a7349874`다.
- iOS: iPhone 17 Pro Simulator, iOS 26.2, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`. 실행 파일 SHA-256은 `b2fe90449bf5d6e7ed534e69eb2e6484ce48b77c35149ee20bee98bcda63d451`다.
- Android와 iOS fixture 앱 빌드는 `SPINON_ENABLE_C04_RUNTIME_GPU=1` 및 `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`로 성공했다.
- Runtime C11.4 관련 테스트 8/8, FFI scene matrix 테스트 1/1, `cargo fmt --all -- --check`, workspace 전체 Clippy가 통과했다.
- 기본 `spinon-ffi` static archive에서 새 scene ID가 빠지고, `c04-runtime-gpu-fixture-test` positive control archive에는 포함되는 것을 확인했다. 이 결과는 Rust static archive 경계만 확인하며 최종 배포 APK 크기나 릴리스 산출물 검증으로 확대하지 않는다.

## 호환성 차이와 범위

세 image scene의 독립 CSS Grid 계산값 `[18, 34, 20, 0]`은 Chrome 155 관측값 `[16, 32, 24, 0]`과 다르다. 두 결과와 규범 계산 과정은 [계획](../../../../plan/c11-4-anonymous-text-multispan-images.md) 및 [사전 비교 근거](../c11-4-anonymous-text-multimage-precomparison-2026-10-21/README.md)에 보존했다. 원인은 확정하지 않았다.

화면과 로그는 제한 fixture의 layout trace·frame과 host surface present 경로를 보여준다. PNG는 intrinsic 크기를 위해 decode했지만 texel readback/pixel equality는 검사하지 않았다. 장면에 CSS border가 없고 모든 지정 item frame은 root 안에 있으므로, border stroke나 border 바깥 overflow는 이 검증 범위가 아니다. OS compositor 이후 표시 픽셀, WPT, 일반 CSS Grid 적합성도 주장하지 않는다.

첫 Android 캡처 시도는 캡처 검증기가 같은 span의 contribution을 item별로 묶어 기대한 반면 실제 trace는 sizing 단계별로 기록해 중단됐다. 검증기 기대 순서를 고친 뒤 위의 Android 4/4 최종 실행과 iOS 4/4 실행을 완료했다. 초기 중단 로그는 [보존된 실패 실행](android-avd-c114-anonymous-text-multi-image-android-avd-20261021T055203346Z-4cf81b95/runtime-anonymous-text-two-images-forward.log)에 있다.
