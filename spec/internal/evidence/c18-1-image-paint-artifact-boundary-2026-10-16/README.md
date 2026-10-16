# C18.1 기본·옵트인 artifact 경계 — 2026-10-16

## 판정

기본 Cargo, Android APK, iOS 앱 실행 파일에서는 C18.1 전용 FFI 심볼·route·fixture label·고정 PNG payload를 찾지 못했다. 같은 소스를 C18.1 opt-in으로 빌드한 positive control에서는 Cargo fixture 심볼·고정 URL·PNG bytes, Android JNI adapter와 route, iOS route·로그와 PNG bytes가 포함됐다.

각 artifact는 생성 직후 파일 전체를 byte 검색했다. Cargo는 Rust static archive와 rlib을 따로 검사했고, Android APK는 archive 안의 DEX·native library·asset을 검사했다. iOS는 `.app` 안의 모든 파일을 재귀 검색했다. PNG 검사는 고정 `c14-1-png-basic.png` 97 bytes 전체의 포함 여부를 확인했다. archive·app별 전체 hash, marker scan 결과와 Android embedded `.so` hash는 [scan-results](./scan-results.txt)에 기록했다. 입력 PNG SHA-256은 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`다.

## 산출물

| 대상 | 기본 산출물 SHA-256 | 기본 검색 결과 | opt-in positive control SHA-256 | opt-in 검색 결과 |
| --- | --- | --- | --- | --- |
| Android-target Cargo `libspinon_ffi.a` | `3b8f35c632c84b5fce8d9135d629fda2b74f7f4d465efd0be400122ab41bd63b` | C18.1 symbol·URL·report marker 없음, PNG bytes 없음 | `e557922a0bf548872086b9d6bdab0ca446124bc31d2e5a2a8788072206d64e5e` | C18.1 new/eval symbol, fixture URL·report JSON marker와 PNG bytes 있음 |
| Android-target Cargo `libspinon_ffi.rlib` | `c13fec50a64c7eee928298ffd52bb33d4ffa27b37b60637256c6263464d86d09` | C18.1 symbol·URL·report marker 없음, PNG bytes 없음 | `15e908806fa5d3c34cc83e677382265fd6518aed2b6ca87411741b21d3f2b762` | C18.1 new/eval symbol, fixture URL·report JSON marker와 PNG bytes 있음 |
| Android `app-debug.apk` | `ead7d94c3cabb1582cd8391f150233229876644a9363693008f6755e002304bd` | route, labels, sample IDs, adapter classes, PNG bytes 없음 | `5ccbbf60c918a33c985b4a6ca7f68682c453d60bf5bdf590e72ec6f3d112e3cf` | 전용 route·capture/native adapter·sample labels·PNG bytes 있음 |
| iOS `SpinonBootstrap.app/SpinonBootstrap` | `da32ced85eb80b3186cfa301b32a01caee2abc81e78a2935268a27908ce232c6` | route, selector, C18.1 label, PNG bytes 없음 | `a40b9d549fd9ec31c388311dc69e39b6e3b8fba5b22856c0092731b778377479` | route·C18.1 logs·fixture URL·PNG bytes 있음 |

Cargo는 `cargo build --locked -p spinon-ffi --lib --target aarch64-linux-android --release`로 기본 archive를, 여기에 `--features c18-1-image-paint-fixture-test`를 추가해 opt-in positive control을 만들었다. 두 artifact의 전용 FFI symbol, fixture URL, `c18_1_image_report_json` 및 고정 PNG 전체 97 bytes를 직접 byte-scan했다. Android 기본 APK는 C18.1 flag 없이 빌드했고 opt-in APK는 `SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST=1` 및 `-PspinonC04RuntimeGpu=1`을 사용했다. APK SHA는 archive 전체이고 내부 native library SHA는 기본 `a3d536da057fc9c6021d645d0a154688b0ad9bb395600e4294a9c949a7bf3d33`, opt-in `a99e17063d472ca1b423719e71f9f99809a3be32b6995a65f78434e7d3b11e7f`이다. opt-in `libspinon_bootstrap.so`에는 다음 두 동적 JNI symbol이 있다.

```text
Java_dev_spinon_bootstrap_ImagePaintFixtureNativeAdapter_createHost
Java_dev_spinon_bootstrap_ImagePaintFixtureNativeAdapter_evalFixture
```

iOS 기본 앱은 fixture flag 없이, opt-in 앱은 `SPINON_ENABLE_C04_RUNTIME_GPU=1`과 `SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST=1`로 빌드했다. app executable scan에서 기본 route·selector·label·PNG bytes가 없고 opt-in route, `SPINON_C181`, fixture URL 및 정확한 97-byte PNG가 있음을 확인했다.

Android 실행·화면·PixelCopy 근거는 [API 37 ARM64 AVD](../c18-1-image-paint-android-api37-2026-10-16/README.md), iOS 실행·화면·Metal drawable 근거는 [iPhone 17 Pro Simulator](../c18-1-image-paint-ios-simulator-2026-10-16/README.md)에 기록한다. 이 scan은 지정한 Android-target Cargo release library와 개발 app build의 경계 근거이며 production release archive 크기나 다른 제품 feature 조합을 주장하지 않는다.
