# C11.4 cyclic row percentage auto-repeat runtime evidence

**상태:** 구현 PR [#632](https://github.com/ohah/spinon/pull/632)이 2026-10-25T14:56:09Z Rebase and merge로 병합됐다(merge SHA 64739104e4be3e06f95c60b94a5eb1f87a4e78b0). 이 두-scene 제한 slice는 전체 Grid track sizing이나 일반 CSS 지원 완료를 뜻하지 않는다. C11.4 checkbox와 공식 완료 수 72/213은 유지한다.

## 판정 기준과 관측값

두 scene의 기대값은 [구현 계획](../../../../plan/c11-4-cyclic-percentage-auto-repeat-row-axis.md)과 [Chrome 156 사전 비교 자료](../c11-4-cyclic-row-percentage-auto-repeat-precomparison-2026-10-25/README.md)에 고정되어 있다. 시뮬레이터 로그는 독립 기대값과 별도로 보존한다.

| Scene | Root frame | 반복 수 / percentage basis | 최종 Rows | 관측 |
|---|---|---|---|---|
| `cyclic-max-content-auto-fill-rows` | 24×16px | `1 → 1`, `null → 16px` | `[8px]` | 통과. 24×16px item이 8px row track을 8px 넘는다. |
| `definite-40px-auto-fill-control-rows` | 24×40px | `2`, `40px` | `[20px, 20px]` | 통과. 일반 definite-axis 경로를 사용한다. |

양 플랫폼에서 root/item frame, row track trace, draw/present 완료가 확인됐다. Target row pass는 `IntrinsicMaxContent → FinalDefinite` 순서이며 count는 재확장되지 않았다. Control은 단일 `FinalDefinite` row pass다.

이 두 scene은 capture script에 `pixelSamples`가 없어 PixelCopy/Metal surface pixel capture를 실행하지 않았다. 아래 스크린샷은 scene 배치 확인용이며 pixel equality 근거가 아니다.

## 시뮬레이터 캡처

각 행을 눌러 원본 스크린샷을 볼 수 있다.

| 장면 | Android API 37 ARM64 16 KB AVD | iPhone 17 Pro, iOS 26.2 Simulator |
|---|---|---|
| Target | [화면](./android-avd-c114-cyclic-percent-tracks-android-avd-20261025T142848934Z-bf999679/cyclic-max-content-auto-fill-rows.png) | [화면](./ios-simulator-c114-cyclic-percent-tracks-ios-simulator-20261025T142848934Z-d81cce94/cyclic-max-content-auto-fill-rows.png) |
| Definite control | [화면](./android-avd-c114-cyclic-percent-tracks-android-avd-20261025T142848934Z-bf999679/definite-40px-auto-fill-control-rows.png) | [화면](./ios-simulator-c114-cyclic-percent-tracks-ios-simulator-20261025T142848934Z-d81cce94/definite-40px-auto-fill-control-rows.png) |

실행 원본과 관측 결과:

- [Android simulator metadata](./android-avd-c114-cyclic-percent-tracks-android-avd-20261025T142848934Z-bf999679/simulator.json), [scene 결과](./android-avd-c114-cyclic-percent-tracks-android-avd-20261025T142848934Z-bf999679/result.json), [runtime logs](./android-avd-c114-cyclic-percent-tracks-android-avd-20261025T142848934Z-bf999679/)
- [iOS simulator metadata](./ios-simulator-c114-cyclic-percent-tracks-ios-simulator-20261025T142848934Z-d81cce94/simulator.json), [scene 결과](./ios-simulator-c114-cyclic-percent-tracks-ios-simulator-20261025T142848934Z-d81cce94/result.json), [runtime logs](./ios-simulator-c114-cyclic-percent-tracks-ios-simulator-20261025T142848934Z-d81cce94/)
- [iOS 설치 앱 bundle hash](./verification/ios-installed-app.json): simulator 설치 실행 파일과 Info.plist가 빌드 결과와 일치한다.
- [소스·빌드 연결 manifest](./source-revision.json), [증거 SHA-256 목록](./SHA256SUMS.txt)

## 재현 명령

빌드는 `c04-runtime-gpu`와 `c04-runtime-gpu-fixture-test`만 활성화했다.

```sh
mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bun run build:android
mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bun run build:ios-sim
node tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs android-avd emulator-5560 --scene-id=cyclic-max-content-auto-fill-rows,definite-40px-auto-fill-control-rows --evidence-group=c11-4-cyclic-row-percentage-auto-repeat-runtime
node tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs ios-simulator ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D --scene-id=cyclic-max-content-auto-fill-rows,definite-40px-auto-fill-control-rows --evidence-group=c11-4-cyclic-row-percentage-auto-repeat-runtime
```

수행한 검증과 원본 출력은 [`verification/`](./verification/)에 있다. 포함 항목: Rust layout/runtime/FFI focused tests, feature-off runtime tests, Clippy, rustfmt, fixture-module boundary, Android/iOS build logs, 그리고 fixture 기본 산출물 제외와 opt-in positive control 검사.

## 범위 한계

Android 근거는 API 37 ARM64 16 KB 가상 기기 `emulator-5560`이고 iOS 근거는 iPhone 17 Pro / iOS 26.2 Simulator다. 실기기 GPU, compositor 제출 이후 픽셀, WPT 전체 적합성은 확인하지 않았다. Failed capture 재시도 폴더는 보존되어 있으며, 최신 성공 run만 위에 연결했다. C11.4 공식 완료 상태와 완료 수 72/213은 이 제한 slice로 승격하지 않는다.
