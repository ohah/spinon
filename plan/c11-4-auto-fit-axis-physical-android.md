# C11.4 definite minimum-size `auto-fit` 축별 실기기 후속 계획

**상위 계획:** [축별 definite minimum-size `auto-fit` 계획](./c11-4-auto-fit-min-size-axis.md) · [C11.4 상태](../spec/STATUS.md#c11-css-grid)
**요청:** 2026-10-21 사용자가 Android 실기기 연결을 알리고 Android 실기기 검증을 요청했다.
**범위:** 기존 53-scene 기대값을 Samsung SM-S731N에서 다시 실행하는 보충 검증이다. C11.4 구현 완료 판정이나 공개 지원 범위를 넓히지 않는다.

**실행 결과:** 계획 실패 관점 20개를 기록했고, SM-S731N Android 16/API 36에서 53/53 scene을 통과했다. API 37 ARM64 AVD 대비 used track과 관측 node frame의 최대 차이는 모두 0 CSS px, overflow SurfaceView 표본은 4/4 exact다. [기기·로그·화면과 한계](../spec/internal/evidence/c11-4-auto-fit-min-size-axis-physical-android-2026-10-21/README.md).

계획 사전 검토: [서로 다른 실패 관점 20개](../spec/internal/evidence/c11-4-auto-fit-min-size-axis-physical-android-2026-10-21/plan-review-20.md).

## oracle과 입력

- 규범 oracle은 CSS Grid Level 1 §7.2.3.2와 상위 계획의 고정 계산표다. Chrome 155 관찰은 비교 자료이며 기준값으로 사용하지 않는다.
- 입력은 기존 `auto-fit-min-size-axis` 53-scene 행렬이다. 장면 ID와 기대 geometry는 [기존 Android AVD 결과](../spec/internal/evidence/c11-4-auto-fit-min-size-axis-2026-10-20/android-avd-c114-autofit-minsize-android-avd-20261020T010237848Z-6e767f6f/result.json)에서 고정하며, 실행 harness는 `tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs`의 같은 장면 정의와 수치 assertion을 사용한다. 선택한 ID 집합은 AVD 결과와 정확히 같아야 한다.
- 환경은 ADB serial `R5KYB06SMPE`, Samsung SM-S731N, Android 16/API 36, `arm64-v8a`, `ro.kernel.qemu=0`, 1080×2340, density 450이다. GPU renderer와 빌드된 APK SHA-256을 실행 산출물에 기록한다.
- 현재 소스 revision에서 `SPINON_ENABLE_C04_RUNTIME_GPU=1`과 `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`로 Android APK를 새로 빌드한다. `pm path`로 설치된 base.apk를 찾고 기기 내 `sha256sum`과 로컬 APK SHA-256이 같아야 한다.

## 관찰값과 사전 판정 기준

각 scene에서 run ID가 일치하는 layout summary, 모든 trace chunk, 관측된 root/item frame, 최종 열·행 track, collapse 상태, gutter 값, `SPINON_C0410_DRAW ... presented`를 수집한다. 관측 frame을 scene result에 보존해 Android API 37 AVD 결과와 실제 수치 비교를 한다. fixture에 surface pixel 표본이 지정된 장면은 PixelCopy 결과도 수집한다.

- 53/53 scene이 완료되고, 각 scene의 track 수·순서·collapse·gutter·frame이 harness 기대값과 일치해야 한다. 허용 오차는 기존 harness와 같이 1/64 CSS px다.
- trace chunk가 빠짐없이 복원되고 draw/present marker가 scene의 box 수와 일치해야 한다.
- overflow 표면 표본은 fixture에 적힌 RGBA와 정확히 일치해야 한다. 이 검사는 앱 SurfaceView 표면까지만 증명한다.
- Android API 37 AVD의 저장 결과와 같은 scene별 geometry를 비교한다. 비교 허용치는 1/64 CSS px다. 불일치 시 기대값을 덮어쓰지 않고 원본 log와 화면을 보존하고 실패로 보고한다.

## 실행 절차와 한계

1. 계획을 서로 다른 20개 실패 관점으로 검토하고 이 문서에 기록한다.
2. 기존 capture harness에 `android-device` 모드를 추가한다. 해당 모드는 지정 serial만 사용하고 `ro.kernel.qemu=0`, SM-S731N, API 36 이상, ARM64를 검증하며 AVD 모드의 API 37 고정 검증은 유지한다. 결과 디렉터리와 run ID에는 `auto-fit-min-size-axis` label을 기록한다.
3. 현재 source revision에서 fixture APK를 빌드하고 AVD 결과의 53 scene 전체를 실기기에서 실행한다. 각 장면의 실패·재시도도 숨기지 않고 증거에 보존한다.
4. 새 harness 구현을 별도의 20개 실패 관점으로 검토하고, 결과와 한계를 검증한다. 최종 PR diff는 별도 30개 관점으로 검토한다.
5. 상태 대장에는 추가된 실기기 근거만 반영한다. C11.4 체크와 공식 완료 수는 유지한다.

이번 실행은 단일 Samsung 기기·API 36·Samsung Xclipse 940/Vulkan 환경의 보충 결과다. WPT 적합성, 다른 Android 기기, 전체 CSS Grid, Android compositor 이후 픽셀을 증명하지 않는다.
