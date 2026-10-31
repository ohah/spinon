# C11.3.3 positive named-line auto implicit tracks · 구현·runtime 근거

Chrome 156 DPR 1의 고정 세 scene을 C11.3.3 전용 FFI source로 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에 실행했다. 앱 frame은 세 scene 모두 expected frame과 exact하게 일치했고, 각 scene에서 root 경계 바깥 item의 표면 sample 하나를 확인했다.

| scene | expected columns (CSS px) | root-relative item frame (CSS px) | Android PixelCopy | iOS Metal drawable |
| --- | --- | --- | --- | --- |
| `named-line-auto-columns` | `[40, 40, 0, 30]` | `(80, 0, 30, 10)` | 1/1 exact | 1/1 exact |
| `named-line-fixed-columns-control` | `[40, 40, 30, 30]` | `(110, 0, 30, 10)` | 1/1 exact | 1/1 exact |
| `named-line-auto-rows` | `[20, 0, 25]` | `(0, 20, 20, 25)` | 1/1 exact | 1/1 exact |

## 모드 경계 수정 후 재실행

회귀 수정에서 no-area named-line `auto` 크기는 모든 in-flow item이 고정 크기이고, 한 축의 named start line만 빠져 end가 `auto`이며 다른 축에는 definite start line이 있는 경우에만 전용 mode에서 허용한다. fixed-only implicit mode의 no-area `auto`는 계속 거부하고, 양 끝 이름이 모두 implicit인 배치, auto-size item, 부적격 implicit 배치를 섞은 컨테이너도 거부한다. 수정된 checkout으로 2026-10-31에 여섯 장면을 다시 실행했다.

| 플랫폼 | scene | 실행 | frame / 표면 표본 |
| --- | --- | --- | --- |
| Android API 37 ARM64 AVD / SwiftShader | `named-line-auto-columns` | [결과](../c11-3-3-grid-template-areas-runtime-2026-10-31/android-avd-c1133-android-avd-20261031T143645249Z-50b7de14/README.md) | exact / 1 of 1 exact |
| Android API 37 ARM64 AVD / SwiftShader | `named-line-fixed-columns-control` | [결과](../c11-3-3-grid-template-areas-runtime-2026-10-31/android-avd-c1133-android-avd-20261031T143711610Z-011fff85/README.md) | exact / 1 of 1 exact |
| Android API 37 ARM64 AVD / SwiftShader | `named-line-auto-rows` | [결과](../c11-3-3-grid-template-areas-runtime-2026-10-31/android-avd-c1133-android-avd-20261031T143721106Z-d9aa16d7/README.md) | exact / 1 of 1 exact |
| iPhone 17 Pro / iOS 26.2 Simulator | `named-line-auto-columns` | [결과](../c11-3-3-grid-template-areas-runtime-2026-10-31/ios-simulator-c1133-ios-simulator-20261031T143731498Z-ae43a911/README.md) | exact / 1 of 1 exact |
| iPhone 17 Pro / iOS 26.2 Simulator | `named-line-fixed-columns-control` | [결과](../c11-3-3-grid-template-areas-runtime-2026-10-31/ios-simulator-c1133-ios-simulator-20261031T143817539Z-9499092d/README.md) | exact / 1 of 1 exact |
| iPhone 17 Pro / iOS 26.2 Simulator | `named-line-auto-rows` | [결과](../c11-3-3-grid-template-areas-runtime-2026-10-31/ios-simulator-c1133-ios-simulator-20261031T143854035Z-606166e8/README.md) | exact / 1 of 1 exact |

## 실행 근거와 한계

최종 실행의 세부 결과와 JSON은 표의 링크에 있다. Android renderer는 ANGLE/Vulkan/SwiftShader이며, 표면 픽셀은 Android API 37 AVD SurfaceView의 PixelCopy로 읽었다. iOS 표본은 OS compositor 제출 전 Metal drawable에서 읽었다. 각 실행은 잘못된 sample 좌표를 readback 전에 거부했고, Android는 알 수 없는 scene도 runtime draw 전에 거부했다.

Track sizing trace를 포함한 직접 layout test는 columns auto `[40, 40, 0, 30]`, fixed control `[40, 40, 30, 30]`, rows auto `[20, 0, 25]`를 비교한다. 세 앱 fixture는 fixed-size item의 frame/overflow와 표면 색을 검증한다. 기본/fixture-test static archive 경계는 [artifact 검사](./artifact-boundary/README.md)에서 기본 symbol 0개/payload 없음과 opt-in symbol 2개/payload 포함을 확인했다. [구현 실패 관점 20개](./implementation-review-20.md)와 [PR diff 검토 30개](./pr-diff-review-30.md)는 통과했다. 이 evidence는 C11.3.3/C11 완료나 WPT 적합성을 뜻하지 않는다.

fixed-only `auto` validation guard 복원 뒤의 전체 Grid/style/Node regression, workspace와 fixture Clippy, archive 재검사는 [후속 최종 기록](./post-validator-guard-20261031/README.md)에 보존했다. [PR diff 적대 검토 30개](./pr-diff-review-30.md)는 이 최종 변경을 검토하며 CI/merge는 PR 이후 확인한다.
