# C11.4 PR #515 선행 회귀·표면 oracle 재검증

2026-10-22, PR #515의 세-item mixed-span scene을 포함해 C11.4 고정 runtime 회귀 행렬을 Android와 iOS simulator에서 다시 실행했다. 이 evidence는 기존의 부분 Android 실행 결과를 갱신한다. C11.4는 제한 fixture 구현이며 공식 checkbox와 완료 수 `77/213`을 유지한다.

## 발견한 결함과 수정

- Android에서 `anonymous-min-content-wrap`이 `unknown anonymous text/image scene`으로 실패했다. C11.4 text/image fixture matcher의 `anonymous-` prefix가 순수 anonymous text fixture까지 가로챘다. matcher를 `anonymous-text-image-`로 좁혔고, 기존 text/image family prefix는 유지했다. [수정 전 실패 로그](./android-anonymous-routing-failure-before-fix.log)와 [수정 후 로그](./android-anonymous-routing-after-fix.log)를 대조했다.
- 기존 C11.4 PixelCopy/Metal 기대 표본 두 곳이 실제 root border와 맞지 않았다. `overflow-root-border-inside`는 root 배경이 아니라 green border `[17,204,136]`이고, `negative-root-edge`는 root interior가 아니라 dark border `[17,24,39]`다. Android/iOS validators와 canonical scene data의 기대값을 같이 수정했다. border가 없는 `negative-cycle-root-edge` 기대값은 그대로 두었다.
- 첫 Android API 37 ARM64 AVD run의 `columns-two-negative`는 layout trace 뒤 renderer/draw에 도달하지 못했다. ADB transport 자체가 끊긴 것은 아니며, SwiftShader AVD의 WindowManager/system_server watchdog 기록에 surface capture 동기화 정지가 남았다. 해당 로그와 재실행 조건은 [watchdog context](./android-swiftshader-watchdog-context.log)에 있다. 같은 AVD를 host GPU로 다시 시작한 전체 행렬은 통과했다.

## 최종 runtime 결과

| 대상 | 실행 환경 | 장면·표면 결과 | 앱 hash |
| --- | --- | --- | --- |
| Android | API 37 ARM64 AVD, `emulator-5558`, Android Emulator OpenGL ES Translator (Apple M4 Max host GPU) | C11.4 전체 263/263 draw/present. PixelCopy 대상 63 scene의 표본 219/219 exact. PR #515의 네 scene 4/4 draw/present, definite overflow·clear 표본 4/4 exact. | APK와 설치 APK SHA-256 동일: `83c9938fed977eeed47d51729eedd05f5f498e4c22717fc7b13b5a2bd1360664` |
| iOS | iPhone 17 Pro, iOS 26.2 Simulator | C11.4 전체 263/263 draw/present. Metal drawable 대상 63 scene의 표본 219/219 exact. PR #515의 네 scene 4/4 draw/present, definite overflow·clear 표본 4/4 exact. | app SHA-256 `235d0bdda061c2cbe46530c052ee86308a6b6a7c5a068b201308a9e02ee680b7` |

PR #515의 네 scene은 `cyclic-minmax-cross-axis-mixed-span-groups-intrinsic-forward/reverse`와 `definite-minmax-cross-axis-mixed-span-groups-forward/reverse`다. Intrinsic track float는 `[15.666666]*3`으로 표현되며 독립 quantized 기대값 `[15.671875,15.671875,15.65625]`과 각각 1/64 CSS px 이내다. Rows는 `[9,9]`, root는 `55×24px`다. Definite scene은 Columns `[12,12,12]`, Rows `[19,19]`, root `44×44px`다.

두 definite scene에서 item B는 root border를 넘어 그려졌다. 표면 표본은 B overflow color와 B 뒤 clear를 검사해 Android PixelCopy 및 iOS Metal drawable에서 각각 `4/4` exact였다. iOS 표면 readback은 compositor 제출 전이다. Android 결과는 host GPU를 쓰는 AVD simulator 결과이지 Android 실기기나 device hardware GPU 결과가 아니다. 이 captures는 fixed box scene을 보이며 glyph raster equality를 입증하지 않는다.

## 재현 자료

- 전체 263 scene의 Android/iOS observation과 track/frame 결과: [Android matrix JSON](./android-matrix-result.json), [iOS matrix JSON](./ios-matrix-result.json), [계산된 matrix 요약·sample 수·hash](./matrix-summary.json)
- 실행 대상 및 binary hash: [Android simulator metadata](./android-simulator.json), [iOS simulator metadata](./ios-simulator.json)
- PR #515 representative runtime logs: [Android intrinsic](./android-intrinsic-forward.log), [Android definite](./android-definite-forward.log), [iOS intrinsic](./ios-intrinsic-forward.log), [iOS definite](./ios-definite-forward.log)
- [Android definite forward 화면](./android-definite-forward.png) (`1080×2400`) · [iOS definite forward 화면](./ios-definite-forward.png) (`1206×2622`)
- [최초 SwiftShader AVD watchdog 기록](./android-swiftshader-watchdog-context.log)

전체 회귀 출력의 runtime log와 scene별 캡처도 local evidence run directory에 보존했다. 이 commit에는 재현에 필요한 두 matrix JSON, simulator manifest, representative logs, watchdog 근거와 PR 화면만 포함한다.

## 범위와 남은 검증

이 검증은 fixture allowlist 입력과 두 simulator에서 관찰한 고정 동작이다. 일반 author CSS admission, WPT, 임의 span/topology, Android 실기기 또는 device hardware GPU, iOS compositor 이후 pixel, glyph paint, 전체 CSS Grid/C11.4 적합성은 확인하지 않았다. 공식 C11.4 checkbox와 `77/213`은 변경하지 않는다.
