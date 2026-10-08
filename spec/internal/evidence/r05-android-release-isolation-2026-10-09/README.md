# Android debug 기본 경로와 R08 probe-off 대조

- **실행:** 2026-10-09 01:36–01:39 KST
- **기기:** Samsung SM-S731N · Android 16 / API 36 / SDK_INT_FULL 36.1 · ARM64 · 1080×2340 · density 450
- **화면:** 시작·종료 active mode 2 = 60 Hz · 밝기 248 · timeout 30,000 ms · stay-awake 3
- **앱:** 설치 APK는 PR #79의 고정 debug artifact와 SHA-256 일치. 새 process별 PID log를 수집했다.
- **복구:** Spinon을 force-stop하고 Chrome foreground로 복귀했다. 화면 설정은 유지됐다.

## Debug 기본 bootstrap · R05/R08 flag 없음

`MainActivity`를 intent extra 없이 새 process(PID 28058)로 실행했다. `SPINON_BOOTSTRAP_EXECUTION is_main_thread=false`와 `SPINON_BOOTSTRAP_RESULT=nodes=2 ... document_nodes=10`을 관측했고 `SPINON_R05_` marker는 0건이었다.

이 경로는 bootstrap DOM fixture만 실행하고 render report를 화면에 표시하지 않는다. 흰 화면은 이 앱 경로의 현재 동작이며 GPU visual pass가 아니다. 잘못된 초기 계획에서 blank decor에 ADB tap을 한 번 보냈지만 GPU 도형이 없으므로 해당 동작은 집계·시각 검증에서 제외했다. [시작 화면](debug-default/launch.png) · [비점수 화면](debug-default/after-one-synthetic-tap.png) · [PID log](debug-default/logcat.txt)

## R08 WGPU probe-off visual control

`spinon_r08=true`만 전달하고 R05 extra는 모두 생략해 새 process(PID 28292)로 실행했다. `SPINON_R08_WGPU=ready backend=Vulkan device=IntegratedGpu name=Samsung Xclipse 940`과 surface 1080×2340을 확인했다. 한 번의 `adb shell input tap 540 1170` 뒤 `SPINON_R08_TOUCH count=1`이 기록됐고 화면 도형이 파란색에서 주황색으로 바뀌었다. PID log의 `SPINON_R05_` marker는 0건이며 fatal/ANR marker도 없었다. 입력은 synthetic이고 이번 control은 물리 touch/성능 측정이 아니다. [시작 화면](r08-probe-off/launch.png) · [한 번 탭한 화면](r08-probe-off/after-one-synthetic-tap.png) · [PID log](r08-probe-off/logcat.txt)

## 첫 Release build 시도 차단

첫 `assembleRelease` 실행은 Gradle에 Android SDK 경로가 없어 중단됐다. `ANDROID_HOME`·`ANDROID_SDK_ROOT`를 설치된 SDK 경로로 명시한 재실행은 `:app:prepareSpinonBootstrap`에서 멈췄다. repository의 고정 V8 source tree가 이 worktree에 없어서 `tools/build-android.sh`가 종료했으며 release APK·release runtime은 만들지 않았다. 필요한 V8 revision은 `7b50b62cb18f28617959e8452e2cd18195b38bcf`다. 외부 source/dependency를 가져오는 checkout 절차는 자동 실행하지 않았다.

- [SDK 경로 미설정 build log](build-release.log)
- [SDK 경로 설정 후 build log](gradle-release-sdk-configured.log)
- [실기기 설치 APK SHA-256](installed-apk.sha256)
- [실행 환경](environment.txt) · [serial을 제거한 기기·emulator 확인](connection.txt)

## 검증 경계

두 debug 조건은 서로 다른 목적이다. 첫 조건은 default bootstrap worker 경로, 둘째는 R08 GPU surface가 R05 probe 없이 작동하는지 확인한다. 이 문서의 build 차단은 이 실행의 결과다. 이후 고정 V8 revision을 준비한 별도 run에서 Release build·기본 cold launch·debug-only 거부를 확인했다([후속 실기기·Release smoke](../r05-android-physical-touch-positive-2026-10-09/README.md)). 이 초기 결과만으로 release 격리·제품 default visual UI·실제 손가락 입력·event-to-present·VSync·optical scanout·성능을 주장하지 않는다.

실행 로그·캡처·기기 설정·build failure 관점은 [runtime-review.md](runtime-review.md)에 20개 실패 관점으로 대조했다. 계획 전제 수정 및 재검토는 [비동기 fence 계획](../../../../plan/r05-android-async-present-fence.md)에서 관리한다.
