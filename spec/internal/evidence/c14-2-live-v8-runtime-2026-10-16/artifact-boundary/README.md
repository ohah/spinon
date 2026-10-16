# C14.2 fixture artifact 경계

고정 Rust fixture·FFI entrypoint·PNG payload는 전용 Cargo feature `c14-2-app-fixture-test` 및 Android/iOS build flag에서만 포함한다. 기본/opt-in 비교는 Debug Android ARM64 APK와 native library, iOS Simulator static library와 app executable에 한정했다.

## 양성 대조

[Opt-in scan](opt-in-payload-scan.json)은 모든 검사 산출물에서 아래 항목을 찾았다.

- `spinon_c14_2_app_image_ratio_fixture` FFI symbol
- `c14-2-live-v8-image-ratio-sizing` report label
- `aspect-ratio-override` case ID
- 고정 PNG 전체 97 bytes

빌드 flag는 [Android](../android-avd-api37-arm64-android17-run-01-20261016/build-final-after-clippy-fix.log)와 [iOS](../ios-simulator-iphone17pro-ios26.2-run-02-20261016T0306Z/rebuild-final-after-clippy-fix.log) log의 `c14_2_app_fixture=1`이다.

## 기본 부재와 negative control

- [Android default scan](android-default-payload-scan.json)과 [iOS default scan](ios-default-payload-scan.json)에서 symbol, label, case ID, PNG bytes가 static archive·native library·app executable에 없었다.
- 기본 빌드 flag는 각각 [Android](android-default-build.log)와 [iOS](ios-default-build.log) log에서 `c14_2_app_fixture=0`이다.
- 기본 Android APK와 iOS Simulator app에 C14.2 route argument를 전달했을 때 두 앱 모두 `SPINON_C142_DISABLED fixture-test feature is off`를 기록했다. [Android disabled route log](android-default-disabled-logcat.txt) · [iOS disabled route log](ios-default-disabled-log.txt).
- 기본 실행 뒤 캡처는 [Android](android-default-disabled-screen.png)와 [iOS](ios-default-disabled-screen.png)다.

scanner는 선택한 artifact의 raw bytes에서 동일한 FFI symbol 문자열·고유 report label·case ID·고정 PNG byte sequence를 찾는다. 결과를 release 전체, 다른 ABI/device, 제품 image loader 또는 설치된 빌드 전반으로 확대 해석하지 않는다.
