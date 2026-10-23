# C11.4 positive percentage auto-repeat minimum runtime

이 자료는 `RuntimeBlockFlexGridTrackSizingV1` fixture에서 finite positive percentage minimum이 definite fixed content axis에 있을 때 auto-repeat을 구성하는 범위를 검증한다. 일반 CSS admission, 전체 C11.4 완료 또는 WPT 적합성 자료가 아니다.

## 최종 simulator 결과

- Android API 37 ARM64 AVD `emulator-5558`: [최종 16-scene 캡처](./android-avd-c114-positive-percent-minimum-commit-android-avd-20261023T104649350Z-da0f371b/)와 [실행 로그](./builds/android-capture-commit.log). 모든 장면의 frame, track 크기, collapse 상태가 기대값과 일치하고 `drawPresented=true`다.
- iOS 26.2 Simulator `iPhone 17` (`D6E8EEE5-62C2-4BED-960A-F49F06158B06`): [최종 16-scene 캡처](./ios-simulator-c114-positive-percent-minimum-commit-ios-simulator-20261023T104719112Z-2d91e097/)와 [실행 로그](./builds/ios-capture-commit.log). 모든 장면의 frame, track 크기, collapse 상태가 기대값과 일치하고 `drawPresented=true`다.
- 각 simulator 기록은 구현 커밋 `d2b4826e4a76de5367e0671984d5e7b2c8c62ae0`, tree `b5b0116a390584b59a8b0dae494a11df0e3c6d48`를 가리키며 tracked diff는 비어 있다. 앱별 SHA-256은 각 폴더의 `simulator.json`에 있다.
- iPhone 17은 다른 iOS simulator의 실행 중인 Spinon 작업과 분리해 사용했고, 실행 후 원래 `Shutdown`으로 돌렸다. 기존 iPhone 17 Pro, iPhone 17 Pro Max, iPad Pro 13-inch와 Android AVD는 종료하지 않았다.

## 검사한 16개 장면

12개 비교 장면은 columns/rows × auto-fill/auto-fit에서 `calc(0px + 10%)`, 직접 `10%`, 고정 `12px`를 대조한다. 10% minimum은 120px content axis에서 12px다. auto-fill은 12px track 10개, auto-fit은 item이 차지한 120px track 하나와 collapse된 빈 track 9개가 된다. 모든 percentage 결과가 고정 control과 같은 frame을 냈다.

네 overflow 장면은 양 축에서 `calc(0px + 150%)`와 고정 `180px`를 대조한다. 150% minimum은 180px track 한 개를 만들고, item은 124px outer border box를 58px 넘는다. [Android 화면](./android-avd-c114-positive-percent-minimum-commit-android-avd-20261023T104649350Z-da0f371b/percentage-overflow-columns-calc.png)과 [iOS 화면](./ios-simulator-c114-positive-percent-minimum-commit-ios-simulator-20261023T104719112Z-2d91e097/percentage-overflow-columns-calc.png)은 2px border 바깥으로 이어지는 item을 보여준다. 캡처는 화면/제출 확인이며 Chrome과의 pixel equality 또는 compositor 이후 표본 일치를 주장하지 않는다.

모든 fixture는 2px border, padding 0, gap 0을 사용한다. content-box 기준은 120px이고 border box는 124px다. padding/gap이 있는 조합, indefinite/cyclic basis, 다른 maximum 형식은 계획된 이번 subset 밖이다.

## 빌드와 자동 검사

- [Android fixture 앱 빌드](./builds/android-build-final.log)
- [iOS fixture 앱 빌드](./builds/ios-build-final.log)
- [스타일 테스트: 12 passed](./builds/style-tests.log)
- [레이아웃 테스트: 4 passed](./builds/layout-tests-pass02.log)
- [FFI catalog/routing 테스트: 1 passed](./builds/ffi-tests.log)
- [fixture module 경계 테스트: 19 passed](./builds/fixture-module-boundary-test.log)
- [cargo fmt check](./builds/fmt-check.log)
- [전체 workspace Clippy: `-D warnings` 통과](./builds/clippy-workspace-pass02.log)
- `cargo build --locked --release --no-default-features -p spinon-ffi --lib --features c04-runtime-gpu`로 만든 [기본 제품 archive](./builds/product-archive-marker-scan.log)에는 새 scene ID가 없고, `--features c04-runtime-gpu-fixture-test`로 만든 [fixture-test archive](./builds/fixture-archive-marker-scan.log)에는 새 scene ID가 있다. APK/IPA 전체에서 fixture 제외 여부는 검사하지 않았다.

처음 Android 실행에서는 플랫폼 데모 harness에 새 장면의 기대 frame이 연결되지 않아 harness assertion이 실패했다. percentage grid 결과 자체는 기대 JSON과 일치했고, 양 플랫폼 기대 frame 연결을 추가한 뒤 최종 소스에서 Android/iOS 각 16/16을 재실행했다. 이 수정 전 로그는 [`builds/android-capture.log`](./builds/android-capture.log)에 보존했다.

## 상태

구현 PR과 공식 상태 문서 갱신/병합 전 evidence다. `spec/STATUS.md`의 C11.4 체크와 공식 완료 수 `72/213`을 아직 바꾸지 않았다. [구현·runtime 적대 검토 20개](./implementation-runtime-review-20.md)와 [PR diff 적대 검토 20개](./pr-diff-review-20.md)는 계획 검토와 분리했다.
