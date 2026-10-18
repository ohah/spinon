# C11.2.3 Android JNI feature gate · 2026-10-19

## 범위

`origin/main` `0168e89e2ebff3543e4d2153bf21e9557bb7544a`에서 Android 기본 APK 조합(`C04 runtime GPU=1`, `C12.4 worker-stack diagnostics=1`, `C11.2.3 fixture=0`)이 JNI translation unit의 C11.2.3 fixture FFI symbol을 무조건 참조해 컴파일되지 않았다. [원본 빌드 오류](android-build-origin-main-failure-repro.log)

두 JNI method 이름은 유지하고, C11.2.3 Rust FFI 호출만 `SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST`와 동일한 전처리 gate로 제한했다. Feature-off create는 0을 반환하며, eval은 명시적인 disabled status를 반환한다. 헤더·Cargo feature·build-script·Gradle flag는 변경하지 않았다.

## 판정 기준과 결과

ARM64 native library와 APK 재현 명령은 다음과 같다. Gradle은 APK를 포장하고 `tools/build-android.sh`는 JNI `.so`를 생성한다. 두 설정의 결과를 혼동하지 않도록 APK도 각각 별도로 다시 포장했다.

```sh
mise exec -- env SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 \
  SPINON_ENABLE_C04_RUNTIME_GPU=1 \
  SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST=1 \
  SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=0 ./tools/build-android.sh

mise exec -- env ANDROID_HOME=/Users/yoonhb/Library/Android/sdk \
  ANDROID_SDK_ROOT=/Users/yoonhb/Library/Android/sdk \
  SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 \
  SPINON_ENABLE_C04_RUNTIME_GPU=1 \
  SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST=1 \
  SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=0 \
  ./platforms/android/gradlew -p platforms/android :app:assembleDebug
```

Positive control은 두 command에서 `SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=1`로 바꿔 실행했다.

| 판정 | 결과 | 근거 |
|---|---|---|
| 기본/off Android ARM64 `.so`와 APK가 빌드된다 | 통과 | [native build](android-build-c1123-off-rerun.log) · [APK build](android-apk-build-c1123-off.log) |
| off artifact에 두 JNI method가 남고 C11.2.3 fixture FFI symbol은 없다 | 통과 | [APK ELF symbols](apk-elf-symbols-c1123-off.txt) · [APK SHA-256](apk-sha256-c1123-off.txt) |
| opt-in positive control은 빌드되고 두 C11.2.3 FFI symbol을 포함한다 | 통과 | [native build](android-build-c1123-on-rerun.log) · [APK build](android-apk-build-c1123-on.log) · [APK ELF symbols](apk-elf-symbols-c1123-on.txt) · [APK SHA-256](apk-sha256-c1123-on.txt) |
| C04 GPU off + C11.2.3 fixture on 조합을 거부한다 | 통과 | [잘못된 조합 로그](invalid-feature-combination.log) |
| JNI gate 정합성 검사가 통과하고 정적 검사가 깨끗하다 | Node 1/1 통과, shell syntax·diff whitespace 오류 없음 | [최종 검사 로그](final-static-checks.log) · `bun run test:css-c11-2-3-android-jni-gate` |
| Android API 37 AVD 기본 route가 켜지고 draw한다 | 통과, no-regression smoke | [실행 로그](avd-default-route-off.log) · [화면](android-api37-default-route.png) |
| API 37 AVD에서 feature-off C11.2.3 요청이 명시적으로 비활성 처리된다 | 통과 | [실행 로그](avd-feature-off-entry.log) · [요청 화면](android-api37-feature-off-entry.png). `nativeCreate`가 0을 반환해 Java가 eval을 생략하고 일반 host-create 실패 UI를 표시한다. 직접 eval 호출은 별도 ABI harness가 아니므로 수행하지 않았다. |

처음 AVD 요청 시 native `.so`만 갱신하고 Gradle APK를 다시 포장하지 않아, 이전 opt-in APK가 설치되어 있었다. 그 실행은 feature-off 증거로 제외했다. 이후 off 설정으로 Gradle APK를 다시 만들고 APK 내부 ELF symbol과 APK SHA-256을 확인한 뒤 재설치해 off 경로를 검증했다. APK build logs를 판정 근거로 사용한다.

AVD는 `emulator-5554`, API 37, ARM64 `sdk_gphone16k_arm64`, Android 17 userdebug, ANGLE/Vulkan/SwiftShader였다. 물리 기기와 iOS는 사용하지 않았다. 화면 캡처는 Android 기본 root HTML route의 no-regression smoke이며 C11.2.3 geometry/pixel 적합성 근거가 아니다. C11.2.3 제한 profile의 공식 완료 상태나 공개 CSS Grid 지원 범위는 바뀌지 않았다.

## 검토

- [구현 실패 관점 20개](../c11-2-3-android-jni-gate-implementation-review-2026-10-19.md)
- [PR 변경 적대 검토 30개](../c11-2-3-android-jni-gate-pr-review-2026-10-19.md)
- [계획 실패 관점 20개](../c11-2-3-android-jni-feature-gate-plan-review-2026-10-19.md)
- [계획](../../../../plan/c11-2-3-android-jni-feature-gate.md)
