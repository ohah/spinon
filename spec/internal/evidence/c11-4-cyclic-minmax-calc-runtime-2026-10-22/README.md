# C11.4 cyclic percentage minmax calc 런타임 근거

**상태:** 고정된 opt-in fixture 네 장면을 Android AVD와 iOS Simulator에서 확인했다. 이 결과는 C11.4 전체 지원 완료나 일반 CSS admission을 뜻하지 않는다.

## 최종 실행

| 대상 | 환경 | scene | 화면 표본 |
| --- | --- | ---: | ---: |
| Android | API 37 ARM64 AVD, emulator-5558, sdk_gphone64_arm64 | 4/4 | PixelCopy 9/9 exact |
| iOS | iPhone 17 Pro, iOS 26.2 Simulator, ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D | 4/4 | 제출 전 Metal drawable 9/9 exact |

두 실행은 같은 tracked source diff SHA-256 **c1961f49c9df02d3c444c1969e73b14424f2227f851b4da9b40abdd9138ad795**를 기록했다. 각 실행의 simulator.json에는 소스 revision, 앱 hash, 기기 정보와 dirty path가 있다.

| scene | Columns | Rows | root | item |
| --- | ---: | ---: | ---: | ---: |
| cyclic-minmax-calc-column | [29] | [20] | 50×20 | 29×20 |
| cyclic-minmax-calc-row | [20] | [22] | 20×40 | 20×22 |
| definite-minmax-calc-column | [24] | [20] | 40×20 | 24×20 |
| definite-minmax-calc-small-limit | [9] | [20] | 10×20 | 9×20 |

모든 scene에서 root와 item의 CSS px frame, 한 track씩의 Columns/Rows trace, 두 box, draw/present를 확인했다. 최종 Android APK SHA-256과 설치된 APK SHA-256은 모두 **ab1b488d8697bd12bdaeb18b07b0c5d9454857a1cf51a7ac914a0c1391856c88**이다. 최종 iOS 앱 실행 파일 SHA-256은 **d4d759808b133290817bdfcf55e60fdfe62c5ae98617abb5ab99895b3f4cca63**이다.

## 화면 바깥 표본

small-limit 장면의 item은 9px, root는 10px이다. root 기준 (5,5)의 item 안과 (15,5)의 root 바깥 표본은 Ahem text 색 #e5484d였고, (25,5)는 텍스트 뒤 배경색이었다. Android PixelCopy와 iOS Metal drawable에서 각각 세 표본이 모두 exact였다. 이는 텍스트 paint overflow만 확인한다. CSS border stroke의 색·두께는 이 fixture에 없으며 검사하지 않았다.

최종 small-limit screenshot:

- Android AVD: android-avd-c114-cyclic-minmax-calc-android-avd-20261022T003653863Z-c6952f2b/definite-minmax-calc-small-limit.png
- iOS Simulator: ios-simulator-c114-cyclic-minmax-calc-ios-simulator-20261022T003845541Z-fdde4073/definite-minmax-calc-small-limit.png

## 빌드와 자동 검사

- Android fixture APK는 android-apk-build-pass3.log의 Gradle :app:assembleDebug 성공 뒤 설치했다. installed-apk-verification.json은 APK hash 일치를 기록한다.
- iOS fixture 앱은 ios-fixture-build-final.log의 tools/build-ios-sim.sh 성공 뒤 설치했다. 이 로그의 앱 실행 파일 hash가 iOS simulator.json과 일치한다.
- 고정 scene matrix는 독립 CSS Grid 2025 CRD 계산 및 Chrome 156 관측과 비교했다. 사전 비교 근거는 ../c11-4-cyclic-minmax-calc-precomparison-2026-10-22/README.md에 있다.
- 통과한 focused 검사는 [style-to-layout 네 scene frame/track](./rust-style-test.log), [runtime typed scene allowlist](./rust-runtime-test.log), [FFI 네 scene allowlist](./rust-ffi-test.log), fixture module boundary, rustfmt, Node syntax, workspace Clippy 및 세 feature 대상 Clippy다. Android/iOS fixture build는 [Android APK](./android-apk-build-pass3.log)와 [최종 iOS build](./ios-fixture-build-final.log)에 기록했다.
- PR #487 최종 diff는 별도 실패 관점 30개로 검토했고 blocking finding이 없다. [PR diff review](./pr-diff-review-30.md).
- PR #487 screenshot 링크는 첫 검사에서 잘못 입력한 commit ref 때문에 0×0으로 깨졌다. 실제 head SHA로 고친 뒤 Agent Browser에서 Android/iOS 이미지를 각각 1080×2400, 1206×2622로 로드했다. 작업 namespace와 네 browser session을 닫았고, 사후 session/process 목록은 비어 있다. [검증 결과와 URL](./agent-browser-pr487-images.json).
- 기본 Rust release static archive target/release/deps/libspinon_ffi-4d40ecba4f9adc56.a는 SHA-256 44af952c38f322c5554bd70e812fcf5f9c3484e6e25dee9708f934255c94d32e이며 새 scene marker가 없다. fixture-test archive target/release/deps/libspinon_ffi-4c30e1cb2f929eca.a는 SHA-256 c1029d27140577abdf77ef6e513b2d4212c376c3878a26b6bec71fd820b5c6d3이며 marker가 있다. artifact 경계 기록은 static archive의 문자열 검사다. 최종 제품 APK/IPA 전체의 fixture 제외나 배포 크기 검증으로 확대 해석하지 않는다.

## 재시도와 수정 이력

실패한 시도도 원인과 수정 근거를 위해 삭제하지 않았다.

1. Android 20261022T003110008Z 시도는 Gradle APK를 다시 만들지 않고 native library만 빌드해 이전 APK가 새 scene ID를 거부했다. Gradle APK를 빌드·설치한 뒤 다시 실행했다.
2. Android 20261022T003300339Z 시도에서는 새 small-limit scene이 기존 PixelCopy sample registry에 없어 invalid-c114-sample-fixture로 거부됐다. 새 scene 전용 sample을 등록해 해결했다.
3. Android 20261022T003335003Z는 네 scene이 통과했지만 후속 hostile review에서 미등록 sample ID가 small-limit 기대값으로 떨어질 수 있는 default 처리를 발견했다. default를 fail-closed로 고치고 APK를 다시 빌드했다.
4. 최종 Android 20261022T003653863Z와 최종 iOS 20261022T003845541Z 실행은 최종 tracked diff SHA를 기록했고 네 scene 모두 통과했다. 앞선 Android/iOS 캡처는 비교·디버깅 이력으로 보존했다.

## 검증 한계

Android는 AVD만 사용했으며 실기기 실행은 하지 않았다. iOS 표본은 Metal drawable 제출 전 값이며 iOS compositor 이후 화면 값은 확인하지 않았다. 하드웨어 GPU, WPT, 일반 CSS source admission, 임의 값 조합, border stroke, 전체 CSS Grid 적합성은 검증하지 않았다.
