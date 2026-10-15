# C13.3 Android native wheel fixture 산출물 경계 · 2026-10-16

## 빌드 입력

기준 revision은 `120c933a08037e7b7ea11fc4e89ca9d9c97d6f89`이다. 빌드에 사용한 adapter 파일 SHA-256은 `C0410RuntimeGpuDemo.java` `1c7652d81aec463b55ca5d00449f663ee42ea462108977d5bbf65e42531273ca`, `MainActivity.java` `178a775663b2dfad66676258493a674ee65016fa760efbdcf8902b9ba9a14564`, `spinon_jni.cc` `eb13740183ddf251ead00f8b0ecbfd666f3e80656dd5141ce10b947fb7457f2c`다. 두 APK는 같은 worktree에서 `SPINON_ENABLE_C04_RUNTIME_GPU=1`로 빌드했고 `SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST` 값만 바꿨다. 두 Android Gradle 빌드는 `BUILD SUCCESSFUL`이었다.

| 산출물 | fixture flag | APK SHA-256 | `classes2.dex` SHA-256 | ARM64 `libspinon_bootstrap.so` SHA-256 |
| --- | ---: | --- | --- | --- |
| 기본 debug APK | `0` | `712fcd89cab6f7e66987acf65f5104a41ff2ec9ad03e08e311eb44922d313284` | `e50984a18271ec3cd183f91d851f3d9d2263568788fe3840f8ebc5f17d9694fa` | `29665f92a00b17d2af11a3e9aa6d9ee44185e555700d8ac75d6b633f15910a74` |
| opt-in debug APK | `1` | `4e968d96c0300b1b6a0c966e48d9f5e38dbe5b532def29f8dc130aa2bade3886` | `285b475b0cc87093275eb9262fea724e9803fc380878c6315a3733448cb0aa72` | `359e8fb72335726c8efcba656a330975c11adca1321d5b04676bf5be6d886cdd` |

생성된 Android `BuildConfig`는 기본 빌드에서 `false`, opt-in 빌드에서 `true`였다. `llvm-nm --defined-only` 결과 기본 ARM64 라이브러리에는 `spinon_runtime_gpu_host_enqueue_c13_3_wheel` 심볼이 없고 opt-in 라이브러리에는 있다. `strings`에서도 `SPINON_C133_WHEEL_ENQUEUE` native log marker는 기본 산출물에 없고 opt-in에 있다.

JNI wrapper 자체는 두 debug 라이브러리에 모두 있다. compile-time feature-off 분기는 wheel 인자를 무시하고 `status=-1 C13.3 fixture-test feature is off`를 반환한다. Debug DEX는 minify되지 않아 guard 안의 Java route/log 문자열이 양쪽 APK에 남는다. 실행 가능 여부는 `BuildConfig`와 native FFI 심볼 조건으로 기록했다. Surface bounds source marker는 여러 조각으로 조립하고 `fixture-module-boundary.test.mjs`에서 literal 포함 여부를 확인한다.

빌드 명령:

```sh
SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 \
SPINON_ENABLE_C04_RUNTIME_GPU=1 \
SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=0 \
mise exec -- bun run build:android

SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 \
SPINON_ENABLE_C04_RUNTIME_GPU=1 \
SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=1 \
mise exec -- bun run build:android
```

이 검사는 compile-time native FFI 경계를 확인한다. Android API 37 AVD opt-in 동작과 남은 플랫폼 관문은 [실행 요약](README.md)에 기록했다. C13.3 전체 완료를 뜻하지 않는다.
