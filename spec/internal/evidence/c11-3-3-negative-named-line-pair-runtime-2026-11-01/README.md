# C11.3.3 음수 named-line pair leading auto implicit tracks

## 범위와 결과

작업 commit [`0096e39d`](https://github.com/ohah/spinon/commit/0096e39d59988abb60768129a5cdada0ec077497)은 no-area C11.3.3 opt-in fixture에서만, 고정 크기 direct item 하나가 한 축의 동일한 negative named-line occurrence pair로 leading implicit tracks를 만들 때 `grid-auto-*: auto` 또는 fixed-px 목록을 허용한다. 반대 축은 explicit definite placement여야 한다. authored name 우선순위, 이름 불일치, 추가 participating child, 양 축 implicit placement, auto-sized item, unsupported track kind, 비유한·음수 px와 10,000-track 경계를 fail-closed로 유지한다.

Chrome 156 Agent Browser의 10개 사전 비교와 independent expected values는 [사전 비교 근거](../c11-3-3-negative-named-line-auto-precomparison-2026-11-01/README.md)에 있다. 양 플랫폼 앱의 전용 matrix는 각각 17개 element frame과 8개 표면 표본을 비교했다. 기존 C11.3.3 regression matrix는 각각 39 scene·126 node이며 126 frame의 최대 geometry 오차는 0 CSS px다.

## 최종 실행 (commit `0096e39d`)

- [Android API 37 ARM64 AVD 전체 행렬](../c11-3-3-grid-template-areas-runtime-2026-11-01/android-avd-c1133-matrix-20261101T043044448Z-cd4aa025/README.md): 39 scene·126 node, 성공 34·cascade fallback 5·기대 거부 0, visible frame 126개 최대 오차 0 CSS px. SwiftShader SurfaceView PixelCopy 회귀 표본도 exact다.
- [Android 전용 음수 pair scene](./android-avd-c1133-negative-pair-android-avd-20261101T043156396Z-dfdf6879/README.md): 17 frame, PixelCopy 8/8 exact, 최대 channel 오차 0. 좌표 변조는 readback 전에, 등록되지 않은 scene은 runtime draw 전에 거부했다. APK SHA-256 `ce82bd8b32a78514ba2230422e577772b530ea9c03002239d8d38d7266928f2d`.
- [iPhone 17 Pro / iOS 26.2 Simulator 전체 행렬](../c11-3-3-grid-template-areas-runtime-2026-11-01/ios-simulator-c1133-ios-matrix-20261101T043323357Z-0ca80ab6/README.md): 39 scene·126 node, 성공 34·cascade fallback 5·기대 거부 0, visible frame 126개 최대 오차 0 CSS px. 기존 Metal 표본도 exact다.
- [iOS 전용 음수 pair scene](./ios-simulator-c1133-negative-pair-ios-simulator-20261101T043205958Z-dc6eff5e/README.md): 17 frame, Metal drawable 8/8 exact, 최대 channel 오차 0. 표본은 OS compositor 제출 전에 읽었다. 앱 binary SHA-256 `6055e923f82f34777028bac6751ac95163ba18b26d726641710723fd4deee437`.
- [Android 기본/fixture-test artifact 경계](./artifact-boundary/README.md): 기본 linked ELF에서 fixture export와 payload가 없고 opt-in positive control에는 있음을 각각 확인했다.
- [구현/runtime 실패 관점 20개](./implementation-review-20.md).

Android AVD는 `sdk_gphone64_arm64`, Android 17/API 37, ANGLE/Vulkan/SwiftShader software renderer다. iOS는 Simulator Metal drawable 결과다. 실기기, Android hardware GPU, iOS compositor 이후 픽셀, Spinon WPT 적합성 또는 일반 CSS author admission을 검증하지 않았다.

## 보존한 Android 실패 기록

첫 전체 Android 실행은 [`c1133-matrix-20261101T040026850Z-107de2aa`](../c11-3-3-grid-template-areas-runtime-2026-11-01/android-avd-c1133-matrix-20261101T040026850Z-107de2aa/)의 scene `area-auto-tracks-short-col`에서 timeout됐다. 해당 scene의 CSS layout 평가는 `status=0`, `layout=ready`와 5개 기대 frame을 반환했지만 renderer draw marker는 남기지 못했다. 이후 AVD의 RenderThread가 CPU를 계속 사용했고 재부팅 뒤 Android system service가 복구되지 않았다. scene의 정확한 hang trigger는 특정하지 못했다.

이 실패 artifact는 남겨 두었다. 해당 API 37.1/compat16k AVD 환경의 hang을 제품 결함으로 단정하지 않는다. healthy한 Android 17/API 37 ARM64 `spinon_api37_0_applied_txn` AVD에서 같은 scene 격리 실행, 이후 전체 행렬, 최신 source의 전용 negative pair를 다시 실행해 각각 통과했다. 대체 AVD 결과도 hardware/physical-device 근거는 아니다.

## 상태

이 제한 slice는 C11.3.3·C11의 전체 완료가 아니다. 공식 완료 집계 `72/213`과 `spec/STATUS.md`의 미완료 checkbox는 그대로 둔다. 내부 계약 `0059` 숫자 버전도 `0.1.0`으로 유지한다.
