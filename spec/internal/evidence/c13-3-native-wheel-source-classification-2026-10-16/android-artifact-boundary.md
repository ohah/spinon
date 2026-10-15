# Android C13.3 source 분류기 artifact 경계 · 2026-10-16

## 빌드 조건

두 산출물은 같은 task worktree와 `SPINON_ENABLE_C04_RUNTIME_GPU=1`에서 debug 빌드했다. `SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST`만 `0`/`1`로 바꿨다. Gradle build log는 [`android-feature-off-symbol-build.log`](android-feature-off-symbol-build.log), [`android-feature-opt-in-build.log`](android-feature-opt-in-build.log)에 있다. 두 빌드 모두 `BUILD SUCCESSFUL`이다. feature-off native 빌드에는 기존 `renderer.rs`의 `sequence` unused-variable 경고가 있다.

최종 APK hash, `BuildConfig`, DEX marker, harness 부재와 unstripped JNI symbol 검사 출력은 [`android-artifact-inspection.log`](android-artifact-inspection.log)에 보존했다.

| 경계 | C13.3 off | C13.3 opt-in |
| --- | --- | --- |
| APK SHA-256 | `830dbf9fcb15f36df0bb0d30ae8fe1e82a0092fafc656ba7a8590aafef4a73ab` | `3f3cca9c31dafd645bbbb5dbed5cd47f64a1034a5d5605f0003e10117219ebbf` |
| unstripped JNI input `.so` SHA-256 | `6ee4ae15acd395258a9430e0468a9b409827124a6e8bd21d3ff27689a8b884cd` | `12d9585fc8ad32c7d50338795e0aec40aef464e13ce9551d6f1db8ab54e9204d` |
| `BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST` (`apkanalyzer dex code`) | `false` | `true` |
| DEX `event_source_bits`, `pointer_tool`, `device_supported_source_bits` | 없음 | 있음 |
| DEX `WheelSourceClassifier` class | 있음 | 있음 |
| DEX `WheelSourceClassifierJvmTest` harness | 없음 | 없음 |
| unstripped JNI `spinon_runtime_gpu_host_enqueue_c13_3_wheel` (`llvm-nm -g`) | 없음 | 있음 |

`WheelSourceClassifier`는 Android app callback 코드라 debug APK 양쪽에 포함된다. C13.3 event diagnostic string과 native wheel FFI entrypoint는 opt-in positive control에만 나타난다. JVM harness는 `tools/` fixture이며 앱 APK에 포함되지 않는다.

이 대조는 macOS Android Studio JBR, Android SDK/NDK 27.1.12297006, API 37 ARM64 AVD의 debug 산출물 경계다. release APK/AAB, Play packaging, 물리 기기, hardware GPU 또는 iOS artifact를 검증하지 않는다. APK와 `.so` 복사본은 해시 확인 후 임시 경로에서 삭제하며 저장소에는 넣지 않는다.
