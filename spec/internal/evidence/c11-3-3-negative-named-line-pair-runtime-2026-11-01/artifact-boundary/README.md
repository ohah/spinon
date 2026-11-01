# Android fixture artifact 경계

확인 대상은 Android ARM64 debug 앱 build가 연결하는 `build/spinon/android/jniLibs/arm64-v8a/libspinon_bootstrap.so`다. Release 배포물이나 APK 전체의 symbol 제거를 주장하지 않는다.

## 기본 빌드

`SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=0 bash tools/build-android-app.sh`가 성공했다. 원본 출력은 [android-default-build.log](./android-default-build.log)다.

```text
SPINON_ANDROID_BUILD_FLAGS c04_runtime_gpu=1 c04_runtime_gpu_fixture_test=0 ...
BUILD SUCCESSFUL
```

NDK `llvm-nm -g`에서 `spinon_runtime_gpu_host_new_c11_3_3_grid_template_areas_fixture`는 찾지 못했고, `strings`에서도 `negative-named-line-pair-matrix` payload가 없었다. 검사 대상 ELF SHA-256은 `972a4129df5303c3ef4d849e0782a950288cf03c4c17f5b3c2ef7ea64031b03a`다.

## fixture-test positive control

`SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bash tools/build-android-app.sh`도 성공했다. 원본 출력은 [android-fixture-build.log](./android-fixture-build.log)다.

```text
SPINON_ANDROID_BUILD_FLAGS c04_runtime_gpu=1 c04_runtime_gpu_fixture_test=1 ...
BUILD SUCCESSFUL
```

동일 `llvm-nm -g` 검사에서 위 C11.3.3 fixture export가 나타났고, `strings`에서 새 scene source payload를 확인했다. 검사 대상 ELF SHA-256은 `e54770d958457140c4d3a3695708825effeab5bf85ba1c3551037b515f47e8a3`다.

이 경계는 기본 Android linked ELF와 opt-in fixture-test ELF의 부정·긍정 대조다. Android packaged APK stripping, iOS archive/app bundle, release artifact, 일반 CSS route에는 확대 해석하지 않는다.
