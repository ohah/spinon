# C11.4 percentage neighbor auto-fit gutter 보정 후 Android 회귀 · 2026-10-24

CSS Grid §11.1의 collapsed gutter 중첩 해석에 맞춰 Taffy의 빈 `auto-fit` track 경계 처리를 바꾼 뒤, 기존 definite percentage neighbor의 Android matrix를 다시 실행했다.

- 대상: Android API 37 ARM64 AVD `emulator-5558` (`sdk_gphone64_arm64`). 앱과 source revision/diff hash는 아래 [`simulator.json`](./android-avd-c114-percentage-neighbors-android-avd-20261023T224222319Z-12753e41/simulator.json)에 있다.
- 결과: 13개 기존 percentage neighbor scene 전부에서 axis track, node frame, trace chunk와 `drawPresented`를 확인했다. 13/13 scene이 통과하고 superseded retry는 0이다. [`result.json`](./android-avd-c114-percentage-neighbors-android-avd-20261023T224222319Z-12753e41/result.json)
- 이는 Android AVD 결과다. 이 matrix에서 PixelCopy 표본은 실행하지 않았다. 기존 13-scene matrix를 gutter 보정 뒤 iOS Simulator에서 다시 돌렸다는 뜻도 아니다. 신규 `calc()` 후보의 별도 14-scene Android/iOS evidence는 [projection pass03](../c11-4-auto-repeat-calc-neighbors-projection-pass03-2026-10-24/README.md)에 있다.
- 화면 PNG와 장면별 runtime/launch log는 이 디렉터리에 보존했다. 이전 PR #580 실행 로그는 보정 전 source 결과로 유지한다.
