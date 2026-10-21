# C11.4 nested Grid 양 축 intrinsic sizing runtime 근거

이 실행은 고정된 depth-one control과 depth-two nested Grid에서 inline/block 축의 `min-content`·`max-content` intrinsic sizing을 동시에 확인한다. 지원 범위는 네 개의 opt-in fixture scene이다. 전체 CSS Grid 지원이나 C11.4 완료를 뜻하지 않으며 공식 `spec/STATUS.md`의 체크와 완료 수 `72/213`을 변경하지 않는다.

## 입력과 독립 기대값

Chrome 156 Agent Browser 사전 비교와 독립 CSS Grid 계산은 [사전 비교 근거](../c11-4-nested-grid-two-axis-precomparison-2026-10-22/README.md)에 보존했다. Ahem `10px/10px` 글꼴로 고정 text `XX XXXXXXX`를 사용한다. `min-content`는 가장 긴 단어가 70px이므로 70×20px, `max-content`는 공백을 포함해 100px이므로 100×10px이다. 단일 column/row와 gap 0 조건에서 root, 중간 Grid, inner Grid가 같은 두 축 크기를 전달해야 한다.

## Rust·도구 검사

- Runtime focused test: `c114_nested_grid_depth_two_preserves_leaf_intrinsics_at_each_parent` 통과. 각 부모 Grid의 Columns·Rows contribution, span, kind, 값과 전체 frame을 확인한다. 원본은 [runtime-focused-test.log](./runtime-focused-test.log)다.
- FFI focused test: `emits_only_eight_fixed_nested_grid_depth_scenes` 통과. 기존 네 scene과 신규 네 scene을 포함한 allowlist 및 source가 고정됐는지 확인한다. [ffi-focused-test.log](./ffi-focused-test.log)
- 기본 workspace all-target Clippy와 Runtime/FFI fixture feature를 각각 켠 all-target Clippy가 `-D warnings`로 통과했다. [workspace](./workspace-clippy.log) · [Runtime fixture](./runtime-fixture-clippy.log) · [FFI fixture](./ffi-fixture-clippy.log)
- `cargo fmt --all -- --check`, `git diff --check`, `node --check`가 통과했다. [fixture module boundary Node 검사](./fixture-module-boundary-test.log)는 6/6 통과했다.
- 캡처 adapter scene matrix는 중복 없는 192 ID이며 신규 네 ID가 포함된다([scene-list.json](./scene-list.json)).

## 기본·fixture artifact 경계

`spinon-ffi` static archive를 기본 설정과 `c04-runtime-gpu-fixture-test` feature에서 각각 빌드했다. 기본 archive SHA-256은 `e06ec6f47f7695079ac354e84a4c59f0c206dc2d19b073ab1e0791c774b98d92`, fixture archive SHA-256은 `d6fb36b38ad74e837418fcab8caf12033ba26721289513416433f66d6fdf6d3c`다. 기본 archive 전체 byte string scan에는 전용 two-axis projection 함수명과 신규 scene ID가 없고, fixture archive에는 전용 함수명과 네 scene ID가 모두 있다. 이는 기본/opt-in static archive 경계 확인이며 final Android/iOS release archive 검증은 아니다.

- [기본 빌드 로그와 archive 검사](./artifact-default-build.log) · [기본 archive hash](./artifact-default.sha256) · [기본 검사 요약](./artifact-default-boundary.txt)
- [fixture 빌드 로그와 archive 검사](./artifact-fixture-build.log) · [fixture archive hash](./artifact-fixture.sha256) · [fixture 함수명](./artifact-fixture-function-string.txt) · [fixture scene ID](./artifact-fixture-scene-strings.txt)

Apple `nm`은 Rust 1.96 compiler가 만든 일부 LLVM object의 attribute kind를 읽지 못한다는 오류를 출력했다. 양 archive의 전용 함수명과 신규 ID 부재/포함 판정은 archive 전체의 `strings` 출력도 별도로 스캔해 확인했다. 정적 아카이브 scan을 앱 bundle 전체의 release 크기나 로드 동작 근거로 확대하지 않는다.

## Android API 37 ARM64 AVD

`emulator-5558`, API 37 ARM64 AVD(`sdk_gphone64_arm64`)에서 Android 앱을 재빌드하고 네 장면을 각각 실행했다. APK SHA-256과 설치 APK SHA-256은 모두 `004aa8900b2a7cec65c9de9fc2a7a22f74036aa7bb4fb41e0d9c0a28701d9630`이다. `result.json`은 4/4에서 `layout=ready`, 예상 root-relative frame, Columns·Rows used track, trace chunk와 `drawPresented=true`를 기록한다. superseded retry는 0회다.

- [Android 실행 요약](./android-avd-c114-nested-grid-two-axis-depth-two-android-avd-20261021T153032389Z-894472da/result.json) · [기기/APK 정보](./android-avd-c114-nested-grid-two-axis-depth-two-android-avd-20261021T153032389Z-894472da/simulator.json)
- [Android 빌드 재시도 로그](./android-build-retry.log) · [실패한 첫 host allowlist 실행](./android-avd-c114-nested-grid-two-axis-depth-two-android-avd-20261021T152905817Z-6249cdf9/runtime-nested-depth-one-min-content-two-axis.log)
- 화면은 depth-one [min](./android-avd-c114-nested-grid-two-axis-depth-two-android-avd-20261021T153032389Z-894472da/nested-depth-one-min-content-two-axis.png) / [max](./android-avd-c114-nested-grid-two-axis-depth-two-android-avd-20261021T153032389Z-894472da/nested-depth-one-max-content-two-axis.png), depth-two [min](./android-avd-c114-nested-grid-two-axis-depth-two-android-avd-20261021T153032389Z-894472da/nested-depth-two-min-content-two-axis.png) / [max](./android-avd-c114-nested-grid-two-axis-depth-two-android-avd-20261021T153032389Z-894472da/nested-depth-two-max-content-two-axis.png)이다. PNG는 각각 1080×2400이다.

첫 실행은 앱 Java host의 scene allowlist가 신규 ID를 거부해 `SPINON_C114_GRID_ARGUMENT_ERROR`로 끝났다. Android/iOS demo allowlist와 해당 기대값 mapper에 네 ID를 연결한 뒤 앱을 다시 빌드하고 같은 scene을 재실행했다. 실패 결과는 변경하지 않고 별도 capture 폴더에 보존했다.

## iOS Simulator

iPhone 17 Pro, iOS 26.2 Simulator(`ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`)에서 iOS 앱을 빌드하고 네 장면을 각각 실행했다. 실행 app binary SHA-256은 `e6d605c5dc15c936e746368c4fdcdbab142cd9886dbbc01597e23fa728d1e0b1`이다. `result.json`은 Android와 동일한 frame/track 기대값과 4/4 `drawPresented=true`를 기록하며 superseded retry는 0회다.

- [iOS 실행 요약](./ios-simulator-c114-nested-grid-two-axis-depth-two-ios-simulator-20261021T153134714Z-a13a35fb/result.json) · [Simulator/app 정보](./ios-simulator-c114-nested-grid-two-axis-depth-two-ios-simulator-20261021T153134714Z-a13a35fb/simulator.json)
- [iOS 빌드 로그](./ios-build.log)
- 화면은 depth-one [min](./ios-simulator-c114-nested-grid-two-axis-depth-two-ios-simulator-20261021T153134714Z-a13a35fb/nested-depth-one-min-content-two-axis.png) / [max](./ios-simulator-c114-nested-grid-two-axis-depth-two-ios-simulator-20261021T153134714Z-a13a35fb/nested-depth-one-max-content-two-axis.png), depth-two [min](./ios-simulator-c114-nested-grid-two-axis-depth-two-ios-simulator-20261021T153134714Z-a13a35fb/nested-depth-two-min-content-two-axis.png) / [max](./ios-simulator-c114-nested-grid-two-axis-depth-two-ios-simulator-20261021T153134714Z-a13a35fb/nested-depth-two-max-content-two-axis.png)이다. PNG는 각각 1206×2622이다.

## 화면 해석과 남은 범위

화면 캡처에는 앱 제목, 320×240 CSS px surface, 현재 scene의 단순 surface 출력과 `layout=ready` 상태가 보인다. 실제 수치 판정은 결과 JSON과 Android/iOS runtime log의 node frame·used track·trace를 사용한다. PNG는 glyph paint equality나 CSS border 색, Android hardware GPU, iOS compositor 이후 픽셀의 근거가 아니다.

Android 실기기, WPT, 임의 HTML/CSS·depth, spans와 다중 item, writing mode, 전체 CSS Grid, final release archive, iOS compositor 이후 표면은 검증하지 않았다. 이 네 fixed fixture의 성공은 C11.4 전체 지원을 의미하지 않는다. 파일 해시는 [SHA256SUMS.txt](./SHA256SUMS.txt)에 기록한다.
