# C11.4 Unicode titlecase runtime candidate

## 판정

최종 소스 commit `b3dadbc6ae28712b656a5ff24157cb4939d16252`에는 같은 scene 재시도에서만 root/node ID를 재사용하도록 scene ID guard를 추가했다. 기존 12개 ASCII text-transform 장면과 신규 Unicode 11개가 Android 및 iOS Simulator에서 각각 23/23 통과했다. 최종 simulator metadata의 source HEAD는 이 commit이며 설치 artifact SHA-256도 빌드 artifact와 일치한다. 이 제한 fixture candidate는 C11.4 또는 C11 완료가 아니며 공식 완료 수 72/213도 바꾸지 않는다.

계약, 입력 allowlist, Chrome·표준 차이와 기대값 근거는 [계획](../../../../plan/c11-4-anonymous-grid-text-transform-unicode-titlecase.md), [내부 계약 0059](../../../internal/0059-c11-grid-layout.md), [Chrome 156 사전 비교](../c11-4-anonymous-grid-text-transform-unicode-precomparison-2026-11-01/README.md)에 있다.

## Simulator 실행

- iOS: iPhone 17 Pro, iOS 26.2 Simulator, UDID 9DCCE836-7659-4914-B6C1-13F54BFFA9DB. [최종 23-scene matrix](c114-ios-23-scenes-final-pass06/matrix.tsv), [simulator metadata](c114-ios-23-scenes-final-pass06/simulator.json), [대표 화면](c114-ios-23-scenes-final-pass06/representative-screen.png).
- Android: Android API 37 ARM64 AVD, emulator-5556, QEMU, model sdk_gphone64_arm64. [최종 23-scene matrix](c114-android-23-scenes-final-pass05/matrix.tsv), [simulator metadata](c114-android-23-scenes-final-pass05/simulator.json), [대표 화면](c114-android-23-scenes-final-pass05/representative-screen.png).
- 각 simulator.json은 소스 HEAD와 설치 실행 파일 SHA-256을 기록한다. 빌드 산출물 해시와 설치된 산출물 해시가 각 대상에서 일치한다.
- 실기기는 사용하지 않았다. 캡처 도구는 실행 후 Android 앱을 force-stop하고 iOS 앱을 terminate한다.
- 대표 화면은 앱 셸, runtime surface 및 layout 상태를 보여준다. 검정 surface 때문에 Unicode glyph raster나 개별 글리프 모양의 증거로 사용하지 않는다. 실제 장면별 text, source mapping, intrinsic contribution, line count, track 및 frame 판정은 로그와 matrix로 확인했다.

## 실행한 검사

- cargo fmt --all -- --check: 통과.
- Node CSS reference 및 fixture boundary suite: 33/33 통과.
- spinon-text-layout 기본 기능: 96/96 통과. Unicode fixture feature 포함: 102/102 통과.
- spinon-runtime C11.4 fixture feature: unit 395 pass, 3 ignored; integration 3/3; lifecycle 16/16.
- spinon-ffi C04 runtime GPU fixture feature: 188 pass, 2 ignored.
- FFI text-transform fixture source test after scene-scoped retry guard: 1/1 passed.
- Clippy: spinon-text-layout all-targets + Unicode fixture feature, spinon-runtime lib + C11.4 feature, spinon-ffi all-targets + C04 fixture feature가 -D warnings로 통과.
- Android AVD 및 iOS Simulator fixture app build: 성공.
- 기본 spinon-ffi rlib/staticlib에는 고유 Unicode scene marker가 없고 C04 fixture feature rlib/staticlib에는 marker가 있다. 기본 spinon-text-layout dependency tree에는 icu_casemap이 없으며 Unicode fixture feature를 켰을 때만 추가된다.
- 전체 workspace all-targets Clippy는 미수정 crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests.rs의 unused Fixture.child 및 unused fixture_with_child_style 경고로 실패했다. spinon-runtime all-targets Clippy도 미수정 anonymous_text_hyphens.rs의 기존 too_many_arguments 경고로 실패했으며, 해당 패키지 lib Clippy는 통과했다. 이 오류들은 이번 변경 파일에 없다.

## 보존한 실패 cohort

초기 iOS 실행의 [failed matrix](c114-ios-23-scenes-final-pass01/matrix.tsv), [대표 실패 로그](c114-ios-23-scenes-final-pass01/text-transform-none-min-content.log)와 [simulator metadata](c114-ios-23-scenes-final-pass01/simulator.json)는 Unicode 신규 11개를 통과했지만 기존 ASCII 12개에서 앱 내부 text intrinsic validator가 false를 반환했음을 보인다. 구현 당시 새 Unicode expectation route가 기존 ASCII text-transform scene에도 적용되어 app summary가 생기지 않았다. 첫 실행의 simulator metadata는 이전 candidate source HEAD da57108e9baec63ed0c314c5dc140572547e8213을 기록한다. matrix의 빈 error 필드만 none으로 표기했고 실행 log 원문은 유지했다.

validator 분기를 Unicode scene으로 한정하고 양쪽 플랫폼 구조 검사를 추가했다. 최종 source 84d70e8e에서 iOS [final pass 03](c114-ios-23-scenes-final-pass03/matrix.tsv)와 Android [final pass 02](c114-android-23-scenes-final-pass02/matrix.tsv)의 기존 12개와 신규 11개가 모두 통과했다. 실패 artifact는 삭제하거나 최종 통과 결과로 대체하지 않았다.

## 범위와 제한

실행은 고정된 font/style/source allowlist와 fixture-only feature만 다룬다. Android 결과는 QEMU AVD 결과이며 hardware GPU 또는 실기기 결과가 아니다. iOS 결과는 Simulator 결과이며 실기기 Metal 결과가 아니다. 스크린샷은 glyph paint 검증이 아니고 전체 WPT 적합성도 검증하지 않는다. Chrome 156은 ßeta를 보존하는 반면 CSS Text full titlecase mapping과 Unicode 17 기대값은 Sseta다. 이 차이는 계획과 precomparison 근거에서 별도로 유지한다. 일반 author CSS, arbitrary locale/font/input, inline formatting, 전체 C11.4/C11은 미완료다.
