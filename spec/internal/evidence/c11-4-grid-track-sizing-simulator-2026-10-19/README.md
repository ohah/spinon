# C11.4 제한 fixture 앱의 Android·iOS Simulator 실행

**판정:** 제한 runtime profile의 총 7개 장면이 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 모두 `status=0`, `layout=ready` 및 기대 root/item frame을 반환했다. 추가 auto-repeat 세 장면은 item 색 영역이 Grid background 경계를 넘는 범위도 screenshot에서 측정했다. 이 smoke는 C11.4 전체 알고리즘, Chrome oracle 비교 전체 또는 CSS Grid 지원 완료를 뜻하지 않는다.

2026-10-19 후속에서 app fixture를 7개 scene으로 늘렸다. 추가 auto-repeat 세 scene은 Android AVD와 iOS Simulator에서 각각 세 frame 모두 확인했고, item의 정확한 RGB 영역이 Grid background 끝을 Android 58px·iOS 66px 넘어가는 것을 screenshot에서 측정했다. [auto-fill·auto-fit·Grid overflow 후속](auto-repeat/README.md). border stroke의 색 표본은 이 후속에서도 판정하지 않았다.

## 환경과 실행

- Android: ARM64 AVD `sdk_gphone16k_arm64`, Android 17 / API 37 (`emulator-5554`). 실기기에서 실행하지 않았다.
- iOS: iPhone 17 Pro Simulator, iOS 26.2 (`ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`). 별도로 부팅되어 있던 iPad Simulator에서는 실행하지 않았다.
- 두 빌드는 기존 고정 V8 checkout `/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8`을 사용했다.
- Android 빌드: `mise exec -- env SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bash tools/build-android-app.sh` — [build.log](android-avd/build.log), `BUILD SUCCESSFUL`.
- iOS 빌드: `mise exec -- env SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=1 bun run build:ios-sim` — [build.log](ios-simulator/build.log), `BUILD SUCCEEDED`. 이 build 조합은 기존 Objective-C++ wrapper가 참조하는 C11.2.3 gated declaration도 포함한다.
- Android 장면은 고정 allowlist Intent로, iOS 장면은 `--spinon-c114-grid-track-sizing` 및 고정 scene/run ID 인수로 각각 실행했다. 각 앱 로그와 화면 캡처는 아래에 연결했다.

## 관찰한 frame

모든 좌표와 크기는 CSS px이며 로그의 runtime frame 보고를 옮겼다.

| 장면 | root | item | Android AVD | iOS Simulator |
| --- | --- | --- | --- | --- |
| `auto-fixed-width-item` | 100×40 | 24×16 | 통과 | 통과 |
| `minmax-auto-fixed-width-item` | 100×40 | 24×16 | 통과 | 통과 |
| `fit-content-below-item-size` | 100×40 | 24×16 | 통과¹ | 통과 |
| `spanning-auto-tracks` | 100×30 | 70×10 | 통과 | 통과 |

Android는 네 장면 각각 `SPINON_C114_GRID_SUMMARY`를, iOS는 `SPINON_C114_GRID_SUMMARY`와 eval 결과를 남겼다. C11.4 실행에 C11.1 고정-scene PixelCopy 검사가 잘못 끼어드는 Android 경로를 제외한 뒤 캡처했다. 최종 Android 로그에는 C11.1 surface capture 결과가 없다. iOS `AppDelegate`의 runtime-GPU 인수 allowlist에도 C11.4를 연결해 기본 root HTML 화면 대신 C11.4 demo가 열리는 것을 확인했다.

¹ 빠르게 장면을 연속 실행한 첫 pass에서는 `fit-content-below-item-size`가 `-12`(새 화면 요청으로 이전 장면 취소)로 끝났다. 같은 장면을 단독 재실행해 성공했고, 초기 취소 로그와 캡처도 아래 `initial-transient` 파일로 보존했다. 성공 pass의 frame 값은 취소 전후 모두 같은 기대값이다.

## 원본 로그와 캡처

- Android AVD: [auto](android-avd/auto-fixed-width-item.log) · [minmax(auto)](android-avd/minmax-auto-fixed-width-item.log) · [fit-content](android-avd/fit-content-below-item-size.log) · [spanning](android-avd/spanning-auto-tracks.log) · [초기 취소 로그](android-avd/fit-content-below-item-size-initial-transient.log)
- iOS Simulator: [auto](ios-simulator/auto-fixed-width-item.log) · [minmax(auto)](ios-simulator/minmax-auto-fixed-width-item.log) · [fit-content](ios-simulator/fit-content-below-item-size.log) · [spanning](ios-simulator/spanning-auto-tracks.log)
- Android 캡처: [auto](android-avd/auto-fixed-width-item.png) · [minmax(auto)](android-avd/minmax-auto-fixed-width-item.png) · [fit-content](android-avd/fit-content-below-item-size.png) · [초기 취소 화면](android-avd/fit-content-below-item-size-initial-transient.png) · [spanning](android-avd/spanning-auto-tracks.png)
- iOS 캡처: [auto](ios-simulator/auto-fixed-width-item.png) · [minmax(auto)](ios-simulator/minmax-auto-fixed-width-item.png) · [fit-content](ios-simulator/fit-content-below-item-size.png) · [spanning](ios-simulator/spanning-auto-tracks.png)

## 범위와 다음 관문

초기 4개 장면은 작은 박스의 root/item frame만 검증했다. auto-repeat 후속 장면은 auto-fill·auto-fit layout과 100px Grid 밖으로 나오는 120px item의 screen screenshot 영역을 별도로 확인했다. text/image intrinsic contribution, min/max-content, cyclic percentage의 앱 실행, 실제 auto-track stage 값, named-line/area 교차, PixelCopy/Metal drawable 좌표 표본은 검증하지 않았다. 고정 Chrome 155 26-case 전체 Rust matrix는 별도 oracle 근거이며 앱 smoke와 구분한다. 독립 normative stage 검토와 C11.4/C11 상위는 미완료다.
