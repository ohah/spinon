# C13.3 style invalidation · 기본/opt-in artifact 경계

## 확인 범위

현재 source로 iOS Simulator `.app`와 Android debug APK를 각각 feature-off 및 opt-in 구성으로 빌드했다. 검사 대상은 이번 style invalidation slice의 FFI symbol, payload, style-update marker다. 다른 C13.3 gesture route의 기존 일반 로그 문자열과는 구분한다.

| 플랫폼·구성 | C13.3 fixture flag | 산출물 SHA-256 | 검사 결과 |
| --- | ---: | --- | --- |
| iOS Simulator `.app` executable, 기본 | 0 | `abb09ba12b15560dc55c963d768d964ca14e4a647e1fb3186238ccfcc4ff9561` | 새 style FFI symbol, payload, marker 없음 |
| iOS Simulator `.app` executable, opt-in | 1 | `549a8802e6e3dc573b56e9690800b78404dd5f47cb91a6bfd14cb3ae4315565d` | FFI symbol·payload·marker 있음 |
| Android debug APK, 기본 | 0 | `0dda9e80ab97cb4b4ea35667afb853edf067068e5adcc82ac22732c209088eab` | APK·`classes2.dex`·ARM64 `.so`에 새 style ABI·payload·완성 marker 없음 |
| Android debug APK, opt-in | 1 | `52c708792a8fafeabf3849fc19d28f07f1f9e70356426315997f458d21410737` | ARM64 `.so`에 FFI symbol·payload 있음; 실제 Simulator run에서 Java가 출력한 marker 확인 |

Android 기본 APK에서 추출한 `classes2.dex` SHA-256은 `b3eee9bb45d3253df16ebdb221a12a7f50505199e85a6d02cdb1186adb5b2037`, ARM64 `.so`는 `b5ae1df268536484b59eef751420e6e90f59071db0cc123fe669a5bb651b7906`이다. opt-in APK의 ARM64 `.so` SHA-256은 `37c636731f7cc447b48e6a7db9014fd4d0d7bec51973b97519e7bf20f4e00be3`이다.

Android 기본 DEX에는 이전 C13.3 gesture adapter가 쓰는 일반 route log가 남아 있을 수 있다. 이번 검사에서 제외를 보장하는 대상은 새 style update payload, FFI symbol, 정확한 `SPINON_C133_STYLE_UPDATE ` literal이다. Java는 marker와 payload 비교용 문자열을 조각으로 조립하고, source-level module boundary 테스트가 완성 literal 유입을 막는다. opt-in 실행의 실제 log에서 marker와 성공 payload를 확인했다.

## 명령과 원본 build

- 기본 iOS: `SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=0 mise exec -- bun run build:ios-sim` — 읽기용 [build log](ios-default-final-build.log), 원본 [gzip 보존본](ios-default-final-build.log.raw.gz).
- 기본 Android: `SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=0 mise exec -- bun run build:android` — [build log](android-default-final-build.log).
- Opt-in positive control: [Android simulator build log](../20261015T111411692Z-d846f2a9/android-build.log), [iOS simulator build log](../20261015T111411692Z-d846f2a9/ios-build.log). iOS executable은 같은 run에서 설치된 Simulator app bundle에서 읽었다.
- Exact symbol·string scan 결과는 [scan-results.txt](scan-results.txt)에 보존했다.

이 검사는 Xcode Debug Simulator `.app` 및 Android debug APK에 대한 이 slice의 feature boundary만 다룬다. 배포 IPA·AAB/APK, release 최적화, app bundle 크기, dead stripping 또는 전체 C13.3 제품 경계 증거로 일반화하지 않는다.
