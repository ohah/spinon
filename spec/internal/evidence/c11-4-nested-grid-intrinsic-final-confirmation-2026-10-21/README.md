# C11.4 nested Grid PR diff 검토 후 최종 Simulator 확인

PR diff 검토에서 fixture text validator의 typography 및 item box edge 누락을 발견해 수정한 뒤, 변경된 runtime source를 포함해 Android와 iOS 앱을 다시 빌드하고 세 장면을 재실행했다. 두 simulator에서 frame·track·trace·draw/present 3/3, definite flex overflow 표본 5/5 exact다.

| 장면 | outer/nested width | outer columns | inner columns | outer contribution |
| --- | --- | --- | --- | --- |
| `nested-grid-span-two` | `134/134px` | `65/65px` | `70/60px` | `MinContent`, span 2, `134px` |
| `nested-grid-one-track` | `138/134px` | `134/0px` | `70/60px` | `MinContent`, span 1, `134px` |
| `nested-grid-definite-flex-control` | `120/120px` | `58/58px` | `70/60px` | `Minimum`, span 2, `0px` |

## Android API 37 ARM64 AVD

serial `emulator-5558`; 세 장면 3/3, draw/present 통과. flex control PixelCopy 표본은 x=72, 119, 120, 133, 134에서 5/5 exact다. 로컬 APK와 설치된 `base.apk` SHA-256은 모두 `0b8971bc3cf57d6b0b86f73fbab81d0522d30fa21bb3d2aaf384046f1f0a3433`이다.

- [실행 JSON](./android-avd-c114-nested-grid-intrinsic-final-android-avd-20261021T105814807Z-efe16ffa/result.json) · [시뮬레이터·APK hash](./android-avd-c114-nested-grid-intrinsic-final-android-avd-20261021T105814807Z-efe16ffa/simulator.json) · [설치 APK 일치](./android-avd-c114-nested-grid-intrinsic-final-android-avd-20261021T105814807Z-efe16ffa/installed-apk-verification.json)
- [flex control runtime trace와 PixelCopy](./android-avd-c114-nested-grid-intrinsic-final-android-avd-20261021T105814807Z-efe16ffa/runtime-nested-grid-definite-flex-control.log) · [화면](./android-avd-c114-nested-grid-intrinsic-final-android-avd-20261021T105814807Z-efe16ffa/nested-grid-definite-flex-control.png) · SHA-256 `54c86ea115be66bde518a59e9d5871147c0c5eb09a92cb0a2cc39e773625dfe3`
- [Android fixture build log](./android-fixture-build.log)

## iPhone 17 Pro / iOS 26.2 Simulator

세 장면 3/3, draw/present 통과. flex control Metal drawable 표본은 동일 다섯 좌표에서 5/5 exact이며 OS compositor 제출 전 값이다.

- [실행 JSON](./ios-simulator-c114-nested-grid-intrinsic-final-ios-simulator-20261021T105913166Z-1ad61a9c/result.json) · [simulator와 app hash](./ios-simulator-c114-nested-grid-intrinsic-final-ios-simulator-20261021T105913166Z-1ad61a9c/simulator.json)
- [flex control runtime trace와 Metal sample](./ios-simulator-c114-nested-grid-intrinsic-final-ios-simulator-20261021T105913166Z-1ad61a9c/runtime-nested-grid-definite-flex-control.log) · [화면](./ios-simulator-c114-nested-grid-intrinsic-final-ios-simulator-20261021T105913166Z-1ad61a9c/nested-grid-definite-flex-control.png) · SHA-256 `d49d9ab6b6333e264829d6a23b3b95ba5ae20882f6fed3a2ead684ddfb68dab0`
- [iOS Simulator fixture build log](./ios-simulator-fixture-build.log)

## 변경 후 검사

- `cargo fmt --all -- --check`
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test c114_nested_grid_items_keep_inner_text_sources_and_parent_contributions -- --nocapture` — 1 passed
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test text_profile_requires_all_intrinsic_sizing_properties -- --nocapture` — 1 passed
- `cargo clippy --locked -p spinon-runtime --all-targets --features c11-4-grid-track-sizing-fixture-test -- -D warnings` — passed
- 명령 출력은 [runtime validation log](./follow-up-runtime-validation.log)에 보존했다.

## 한계

이 최종 재실행은 고정된 nested Grid 한 단계와 direct Ahem text 두 개만 검증한다. arbitrary nesting, subgrid, 일반 CSS 지원, WPT, Samsung 실기기, Android hardware GPU와 iOS compositor 이후 화면은 포함하지 않는다.
