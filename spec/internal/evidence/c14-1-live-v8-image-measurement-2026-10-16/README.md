# C14.1 live V8 문서 → PNG 자연 크기 측정 실행 근거

## 범위

이 실행은 C14.1 opt-in 앱 fixture가 `RuntimeSession::eval`로 만든 V8 document를 actor-owned immutable `HostDocumentSnapshot`으로 읽고, 고정 PNG 요청·decode·intrinsic map·Stylo/Taffy frame 계산까지 전달하는지 확인한다. 제품 URL fetch, 제품 cache, async loader/replacement scheduling, GPU image paint, 화면의 PNG pixel, border 밖 overflow는 범위에 없다. C14.1·C14·U05는 미완료다.

## 고정 입력과 통과 조건

- Chrome oracle: HeadlessChrome 155 / 기존 고정 reference.
- PNG: `tests/fixtures/resources/c14-1-png-basic.png`, 137 bytes, SHA-256 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`.
- V8 script: `tests/fixtures/css/c14/c14-1-live-v8-document.js`. `div#root` 아래 연결된 `img#image`를 만들고 `.invalid` 고정 fixture URL을 설정한다.
- Oracle size/frame: natural `24×16 CSS px`, image frame `(0,0) 24×16 CSS px`, viewport `120×80 CSS px`, 최대 차이 `0.5 CSS px`.
- 실행 중 source stamp: document generation 1, initial document revision 8, render-tree revision 2. Stale-source control 뒤 document revision 9.

두 플랫폼 report의 주요 필드는 모두 같았다.

| 필드 | Android API 37 AVD | iPhone 17 Pro / iOS 26.2 Simulator |
|---|---:|---:|
| `status` / `document_origin` | `ok` / `live-v8` | `ok` / `live-v8` |
| `request_source_matches` | `true` | `true` |
| `intrinsic_source_matches` / revision | `true` / `true` | `true` / `true` |
| natural size | `24×16 CSS px` | `24×16 CSS px` |
| frame | `(0,0) 24×16 CSS px` | `(0,0) 24×16 CSS px` |
| stale-src document revision | `8→9` | `8→9` |
| old completion admission | rejected as `SourceChanged` | rejected as `SourceChanged` |

원본 log 파일: `android-api37-avd.log`, `ios-iphone17pro.log`.

## 화면

앱은 계산된 report text를 표시한다. PNG를 화면에 그리지는 않는다.

| 실행 | 저장된 화면 캡처 |
|---|---|
| Android API 37 ARM64 AVD, opt-in | `screenshots/android-api37-avd.png` |
| iPhone 17 Pro / iOS 26.2 Simulator, opt-in | `screenshots/ios-iphone17pro.png` |
| Android 기본 artifact negative control | `screenshots/android-api37-default-disabled.png` |
| iOS 기본 artifact negative control | `screenshots/ios-iphone17pro-default-disabled.png` |

기본 route는 양쪽에서 `SPINON_C141_DISABLED fixture-test feature is off`를 기록했다. 원본은 `android-default-disabled.log`와 `ios-default-disabled.log`다.

## 플랫폼과 재현 명령

- Pinned V8: `7b50b62cb18f28617959e8452e2cd18195b38bcf`.
- Android: `emulator-5554`, `ro.kernel.qemu=1`, API 37, `arm64-v8a`, `sdk_gphone16k_arm64`.
- iOS: iPhone 17 Pro Simulator, iOS 26.2, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`.
- 물리 기기는 조회·연결하지 않았다.

공유된 정확한 V8 checkout은 `SPINON_V8_DIR`로 읽기 지정했고, 소스나 산출물을 재빌드하지 않았다.

처음 isolated worktree에서 `SPINON_V8_DIR` 없이 Android build를 시작했을 때, fixture flag 검증 후 고정 V8 source preflight에서 멈췄다(`V8 소스가 고정 커밋과 다릅니다`). 이 단계는 compile 전이며 `checkout.sh`를 실행하지 않았다. 기존 공유 checkout과 양쪽 archive가 pinned revision과 일치함을 확인한 뒤 해당 경로를 env로 지정해 Android/iOS build를 성공시켰다.

```sh
SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 \
  mise exec -- env SPINON_ENABLE_C14_1_APP_FIXTURE_TEST=1 bun run build:android
adb -s emulator-5554 install -r platforms/android/app/build/outputs/apk/debug/app-debug.apk
adb -s emulator-5554 shell am start -n dev.spinon.bootstrap/.MainActivity --ez spinon_c141_intrinsic_size true

SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 \
SPINON_ENABLE_C14_1_APP_FIXTURE_TEST=1 mise exec -- bun run build:ios-sim
xcrun simctl install ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app
xcrun simctl launch --terminate-running-process ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D dev.spinon.bootstrap --spinon-c141-intrinsic-size
```

실행 시각은 2026-10-16 09:12–09:18 KST다. Android build는 성공했고 AGP가 compile SDK 37.2 지원 경고와 고정 NDK 선택 안내를 출력했다. iOS build도 성공했고 기존 V8 archive의 중복 debug-map timestamp 경고가 있었다. 이 경고들은 fixture 실행 실패가 아니었다.

## 기본/opt-in artifact boundary

| Artifact | Opt-in positive control | 기본 build negative control |
|---|---|---|
| Android native library | exported `spinon_c14_1_app_intrinsic_size_fixture` 및 `RuntimeSession::snapshot_document_for_c14_1_fixture`; `c14-1-live-v8-png-basic`, `__spinonC141Image`, fixed fixture URL 문자열 확인 | Rust FFI archive에서 두 symbol 없음; `.so`에서 고유 fixture id, JS global, fixed URL 문자열 없음 |
| iOS linked app | linked executable에서 C ABI와 actor snapshot symbol 및 고유 fixture payload 확인 | linked executable에서 두 symbol 및 고유 fixture id, JS global, fixed URL 문자열 없음 |
| 앱 route | report `status=ok` | 양쪽 default app에서 C14.1 route가 `disabled`로 fail-closed |

Android negative control은 default Rust FFI archive와 최종 `.so`에서, iOS는 최종 linked simulator executable에서 검사했다. Apple `nm`은 Rust 1.96이 만든 iOS static archive의 새 LLVM attribute를 읽지 못해 archive 검사는 제외하고, 실제 Simulator에서 실행한 최종 linked app binary를 검사했다.

### Artifact SHA-256

| Build | SHA-256 |
|---|---|
| Android opt-in APK | `d5941655069cde85f0f33f6c1e7ef701689eadf80aee6d373e71cd15e3d383bf` |
| Android opt-in `libspinon_bootstrap.so` | `4e60166abdda581a1118f3d32e9231e78cfcd3b667e23bc1cb8abcc97f3c0958` |
| iOS opt-in linked app executable | `0da39b2906ab3cc8bb2680a8326a602dc645e82e02b76fd788b609b3ee7e0701` |
| Android default APK | `6affd9510403b839f18283bb94969528bce6468de4f12ab545ae069fc41a26de` |
| Android default `libspinon_bootstrap.so` | `e20cf8b3743d7400e7a8968ce641667c10e85722455072f84dc6843a07f95b74` |
| iOS default linked app executable | `5bb77807f4a1e217e4995006cb4d94b5755474731c9e5b00581bf90f2cb4d8e8` |

### Screenshot SHA-256

| File | SHA-256 |
|---|---|
| `screenshots/android-api37-avd.png` | `7601a434e873965007c04aa2080178938ad617136737a098664b7ef59f07ef2c` |
| `screenshots/ios-iphone17pro.png` | `f1b9a8a272fb6005195aac02c71ecb785f41012821f9d3ae188cea31d0f4ac52` |
| `screenshots/android-api37-default-disabled.png` | `424c686964309e47fb9a1324e4446a4b5f8248dbae19b4aed3faf881781462d0` |
| `screenshots/ios-iphone17pro-default-disabled.png` | `1ee1860e5f1db7563f2b2acbcc18fe5f3894fae27d7c798a8ef5ac00e4c50f7b` |

## Rust verification

- `mise exec -- cargo fmt --all -- --check`: pass.
- `git diff --check`: pass.
- `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings`: pass.
- `mise exec -- cargo clippy --locked -p spinon-ffi --features c14-1-app-fixture-test --all-targets -- -D warnings`: pass.
- `mise exec -- cargo test --locked --workspace --quiet`: 866 passed, 4 ignored, 0 failed (`cargo-test-workspace.log`에 summary 보존).
- `mise exec -- cargo test --locked -p spinon-runtime --features c14-1-live-v8-fixture-test c14_fixture_snapshot_is_actor_owned_and_closed_sessions_fail`: 1/1 passed.
- `mise exec -- cargo test --locked -p spinon-ffi --features c14-1-app-fixture-test c14_1_app::tests`: 2/2 passed.
- `mise exec -- bun run docs:build`: blocked by 436 Rspress dead-link groups across the docs source. After removing links to non-page log/fixture assets, a targeted scan found no dead-link or image-resolution error pointing to the new C14.1 live-V8 pages.

초기 host FFI end-to-end 시험은 test harness에 Android/iOS V8 provider가 없어 isolate 초기화에서 실패했다. 이를 제품 실패나 통과로 숨기지 않았다. V8 문서 생성·layout·stale control은 실제 Android/iOS opt-in app route에서 확인하고, host actor와 PNG reference 경계는 각 focused unit test에서 확인했다.

## 남은 경계

이 adapter는 고정 bytes를 직접 디코드하고 report를 반환한다. 제품 네트워크 loader·캐시·재시도·async replacement admission·실제 `img` paint·Metal/GPU surface pixel·border 밖 overflow·실기기는 검증하지 않는다. 두 screenshot은 image pixel이나 overflow clip의 증거가 아니다. C14.1과 상위 C14는 체크하지 않는다.
