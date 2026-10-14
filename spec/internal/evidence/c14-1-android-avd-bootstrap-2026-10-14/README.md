# C14.1 Android AVD 부트스트랩 smoke

## 환경

- 대상 AVD/serial: `spinon_api37_1_compat16k` / `emulator-5554`만 사용했다. `adb devices -l` 결과는 [adb-devices.txt](adb-devices.txt)에 있다. 실기기는 연결하거나 선택하지 않았다.
- AVD: `sdk_gphone16k_arm64`, Android 17 / API 37, `arm64-v8a`, `sys.boot_completed=1`.
- 고정 V8 source revision: `7b50b62cb18f28617959e8452e2cd18195b38bcf`.
- NDK: SDK의 `27.1.12297006`을 사용했다. 환경의 `ANDROID_NDK_HOME/ROOT` 불일치 경고는 빌드 스크립트가 기록하고 고정 SDK NDK로 진행했다.

## ADB 실패 원인과 복구

ADB 36.0.2 client는 실행됐지만 처음에는 연결된 device가 없었다. 원인은 ADB daemon이 아니라 `spinon_api37_2.ini`가 가리키는 `/Users/yoonhb/.android/avd/spinon_api37_2.avd` 디렉터리가 없는 불완전한 AVD 정의였다. 해당 프로필 시작 때 QEMU2가 지원하지 않는 `arm` CPU architecture 오류도 확인했다. 유효한 기존 `spinon_api37_1_compat16k` arm64 AVD를 부팅해 `emulator-5554`로 연결했고, `sys.boot_completed=1`과 앱 시작까지 확인했다.

## 실행

```sh
env ANDROID_SDK_ROOT=/Users/yoonhb/Library/Android/sdk \
  SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 \
  mise exec -- bun run build:android
adb -s emulator-5554 install -r platforms/android/app/build/outputs/apk/debug/app-debug.apk
adb -s emulator-5554 shell am start -n dev.spinon.bootstrap/.MainActivity
```

Android ARM64 `spinon-ffi` release target과 WGPU smoke library를 빌드하고 APK를 만들었다. Gradle `:app:assembleDebug`는 `BUILD SUCCESSFUL`로 끝났다. [저장된 재실행 build log](build.log)는 같은 빌드의 UP-TO-DATE 재확인 결과다. compile SDK 37.2에 대해 현재 Android Gradle Plugin 8.13.2가 검증된 최대 버전 36.1보다 낮다는 비차단 경고가 있었다.

앱 프로세스가 AVD foreground에서 실행됐고 기본 bootstrap 로그에 다음 결과가 남았다. 원본은 [bootstrap.log](bootstrap.log)에 있다.

```text
SPINON_BOOTSTRAP_EXECUTION is_main_thread=false
SPINON_BOOTSTRAP_RESULT=nodes=2 last_node=8 tag=text text=이벤트:7 · 문서 revision 25 document_revision=25 render_tree_revision=15 document_nodes=10
```

APK와 ARM64 shared library SHA-256은 [artifact-sha256.txt](artifact-sha256.txt), boot·ABI·PID 자료는 같은 폴더의 개별 텍스트 파일에 보관했다.

## 판정 범위

ADB daemon과 AVD 연결, 현재 PR 소스의 Android ARM64 교차 빌드, 기본 V8 bootstrap 앱의 설치·시작만 확인했다. bootstrap 화면은 C14.1 intrinsic-size snapshot을 생성하거나 호출하지 않으므로 이 결과를 이미지 layout·GPU 표시·C14 runtime 통합 증거로 사용하지 않는다. 실기기와 성능은 검증하지 않았다.
