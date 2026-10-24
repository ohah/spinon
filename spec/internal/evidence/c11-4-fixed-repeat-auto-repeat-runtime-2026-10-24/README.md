# C11.4 fixed-repeat + auto-repeat 제한 runtime slice

구현 PR [#591](https://github.com/ohah/spinon/pull/591)은 2026-10-24 08:48:30 UTC에 Rebase and merge됐고 merge SHA는 `79a2f3e2ec412d8a039125cf8930b3c633c04b63`다. 병합 당시 PR check rollup은 비어 있었고 main required status checks는 설정되지 않았다. C11.4 전체는 미완료이며 공식 집계 `72/213`을 유지한다. 병합 결과를 반영한 문서 변경은 [로드맵 동기화 실패 관점 검토](roadmap-pr-diff-review-30.md)에 기록한다.

## 범위

한 track list에 auto-repeat 하나와 양의 정수 fixed-repeat 하나를 함께 둔다. fixed-repeat은 prefix 또는 suffix이며 1–2개의 computed finite nonnegative <code>LengthPx</code> track으로 제한한다. computed AST가 <code>LengthPx</code>로 정규화한 constant absolute <code>calc()</code>와 absolute units는 같은 범위다. CSS percentage, unresolved <code>calc()</code>, intrinsic/flexible track, <code>minmax()</code>, line names, 복수 repeat group, 양축 혼합, template areas는 이 slice에서 거부한다.

일반 CSS admission이나 전체 C11.4 구현은 아니다. 내부 계약 0059의 숫자 버전은 0.1.0으로 유지한다. 공식 C11.4 checkbox와 완료 수 72/213도 유지한다.

## 비교와 실행

- 사전 비교: [Chrome 156 결과와 입력](../c11-4-auto-repeat-fixed-repeat-precomparison-2026-10-24/README.md)
- 집중 계획: [fixed-repeat + auto-repeat 계획](../../../../plan/c11-4-auto-repeat-fixed-repeat.md)
- Android: API 37 ARM64 AVD, <code>emulator-5558</code>. 고정 9 scene의 root/item frame, axis track와 auto-fit collapse, trace, draw/present가 9/9 통과했다. PixelCopy의 root/item 표본은 11/11 RGBA exact다. AVD는 ANGLE/SwiftShader software path이며 hardware GPU 결과가 아니다.
- iOS: iPhone 17 Pro, iOS 26.2 Simulator. 같은 9 scene의 frame, tracks/collapse, trace, draw/present가 9/9 통과했다. Metal drawable 표본은 11/11 RGBA exact이며 OS compositor 제출 전이다. Xcode build는 성공했고 기존 duplicate RawWindowMetalLayer symbol/debug-map warnings가 남아 있어 원본 build log에 보존했다.
- 스크린샷: [Android occupied auto-fit](android-avd-c114-fixed-repeat-android-avd-20261024T083023372Z-aaa752b7/fixed-repeat-fit-prefix-occupied.png) · [iOS occupied auto-fit](ios-simulator-c114-fixed-repeat-ios-simulator-20261024T083117370Z-b1bedf35/fixed-repeat-fit-prefix-occupied.png)
- 각 실행의 장치 정보, scene별 trace와 frame, 원본 log 및 screenshot은 해당 run directory에 있다. 앱 실행 시점 source revision과 dirty tracked paths도 simulator.json에 기록했다. 마지막 두 캡처는 새 Rust/JSON fixture까지 stage한 뒤 다시 실행해 전체 feature source가 diff hash와 dirty path 목록에 들어간다.

## 구현과 검사

- Rust adapter는 authored order대로 integer repeat을 확장하고 auto-repeat의 start/end index를 보정한다. checked arithmetic, 10,000 descriptor 상한, fallible reservation 및 node/property 오류 경로를 유지한다.
- CSS cascade tests는 columns/rows, prefix/suffix, constant absolute calc, absolute unit, rejection cases, metadata, 상한과 allocation failure recovery를 확인한다.
- Android/iOS fixture route와 pixel sample validation은 opt-in fixture-test build에서만 작동한다.
- [검사 로그](.)에는 spinon-style 216/216, C11.4 FFI 56/56, fixture boundary 10/10, iOS build flags 13/13, rustfmt 및 capture script syntax 결과가 있다.
- 별도의 [구현/runtime 실패 관점 검토 20개](implementation-runtime-review-20.md)와 [최종 PR diff 실패 관점 검토 30개](pr-diff-review-30.md)를 마쳤다.
- [기본/opt-in artifact 비교](artifact-boundary.md)와 machine-readable [marker scan](artifact-boundary.json)은 APK의 모든 zip entry와 iOS app bundle files의 9개 새 scene ID를 검색한다. 기본 산출물에는 9개 marker가 모두 없고 positive controls에는 모두 있다. 이는 Debug APK 및 iOS Simulator app에 대한 marker 검증이며 Release APK/IPA 전체 symbol, 크기 또는 하드웨어 동작 검증으로 확대하지 않는다.
- 문서 출력: 선택된 RSPress 입력으로 로드맵, 상태 대장, 집중 계획, 계약 및 이 slice 근거 페이지를 다시 빌드하고 Tailnet 미리보기에 선택 반영했다. 전체 문서 build는 이 변경과 무관한 기존 내부 evidence의 dead link에서 중단되어 선택 build와 구분한다. [선택 build 및 preview 범위](selected-docs-build-scope.md).

## 실패한 초기 시도

첫 세 Android AVD 캡처는 각각 신규 scene ID whitelist 누락, host의 기존 기본 frame/item 예상값 사용, root 초록색 표본의 RGB mapping 누락으로 실패했다. 해당 시도 로그는 이 directory 아래에 보존했다. matrix validation, scene geometry 기대값, 색상 mapping을 고친 뒤 기록된 최종 AVD 실행 9/9와 PixelCopy 11/11이 통과했다.

첫 iOS compile에서는 Swift collection literal 안의 conditional compilation 실패가 있었다. opt-in scene ID set을 선언 단위 conditional compilation으로 분리하고, 수정된 소스의 두 번째 Simulator build와 9-scene capture를 통과했다.

## 남은 범위

여러 fixed-repeat, fixed-repeat line names, fixed-size minmax, percentage/unresolved calc, intrinsic/flex breadth, 양축 혼합, template areas, 일반 CSS admission, WPT, 물리 Android 기기와 hardware GPU, iOS 실기기 및 compositor 이후 pixels, 전체 C11.4는 검증하지 않았다.
