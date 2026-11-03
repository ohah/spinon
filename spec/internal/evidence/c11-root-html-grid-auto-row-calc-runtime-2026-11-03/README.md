# C11.1 parsed Root HTML `grid-auto-rows: calc(px + %)` 실행 근거

이 근거는 C11.1의 단일 parsed Root HTML Grid에서 두 항 `px + %` 계산식을 implicit row track에 연결한 제한 후보를 기록한다. C11.1/C11 완료, 일반 CSS 지원, WPT 적합성 또는 공개 API를 뜻하지 않으며 공식 완료 수 `72/213`은 바꾸지 않는다.

## 소스와 독립 기대값

- 기준 HEAD: `657a703993892d0b83a403e56b110d719c84d022`; 작업 branch: `feat/c11-root-html-grid-auto-row-calc`. Android/iOS 앱은 dirty working tree의 아래 파일들을 빌드했다. 전체 관련 소스 hash는 [`source-files.sha256`](source-files.sha256)에 있다.
- 두 플랫폼에서 사용한 Root HTML fixture SHA-256: `f5f0bd37a204ea67d653530951f54c1744ed806c4e801e26ccdecf5ae4d49696`.
- 고정 fixture는 content size `50×100px`, explicit rows `[10,20]px`, row-gap `3px`, 8개의 `10×10px` direct items와 `grid-auto-rows: calc(10% + 2px) 25% 15px`를 사용한다.
- 독립 예상 used rows는 `[10,20,12,25,15,12,25,15]px`; item row 시작점은 `[0,13,36,51,79,97,112,140]px`다. 근거와 clamp/overflow/zero/gap/fraction 경계 기대값은 [구현 전 Chrome 156 비교와 독립 산술](../c11-root-html-grid-auto-row-calc-precomparison-2026-11-03/README.md)에 있다.

## Android API 37 AVD

- 기기: `emulator-5556`, API 37, `sdk_gphone64_arm64`; Android 17.
- 빌드 명령은 `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C11_1_ROOT_HTML_PERCENT_COLUMNS_FIXTURE_TEST=1 tools/build-android.sh`와 같은 환경의 `tools/build-android-app.sh`이며 둘 다 성공했다. 기존 opt-in Root HTML host 진입점을 재사용했다.
- 실행은 `adb -s emulator-5556 install -r platforms/android/app/build/outputs/apk/debug/app-debug.apk` 다음 `adb -s emulator-5556 shell am start -W -n dev.spinon.bootstrap/.MainActivity --ez spinon_c111_root_html_percent_columns true`로 수행했다.
- APK SHA-256: `ba4d3cb974c268e046b06cb329b896490bf619c64d69e114b45a9efd43ac5655`.
- runtime은 `status=0`, `layout=ready`, `boxes=11`; draw log는 `presented boxes=11`을 기록했다. item row 시작점은 `[0,13,36,51,79,97,112,140]px`다. Renderer는 ANGLE/Vulkan/SwiftShader software path다. [원본 runtime log](android/avd-runtime.log) · [화면 캡처](android/avd-screen.png) (PNG SHA-256 `e21186d70806a36f96ca59342c1ac78a31bdf3b977d3b95f06a2bee2e4bcdb9d`).

## iOS Simulator

- 기기: iPhone 17 Pro, iOS 26.2 Simulator, UUID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`.
- `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C11_1_ROOT_HTML_PERCENT_COLUMNS_FIXTURE_TEST=1 tools/build-ios-sim.sh`가 `BUILD SUCCEEDED`로 끝났다.
- 앱 실행 파일 SHA-256: `abd1c6928ae17c690c66cb6f1e371a1529f2204a9ff936f86ef9d1935163d2a1`.
- runtime은 `status=0`, `layout=ready`, `boxes=11`; Metal renderer에서 `presented boxes=11`을 기록했다. item row 시작점은 Android와 같은 `[0,13,36,51,79,97,112,140]px`다. [원본 runtime log](ios/iphone17pro-runtime.log) · [화면 캡처](ios/iphone17pro-screen.png) (PNG SHA-256 `6d5419ed3e84ae2cbd01b42bdd7fd32d651e3cb96e599b17f33a1ee045deb70e`).

## 판정 한계

- `cargo clippy --locked -p spinon-layout -p spinon-style-to-layout -p spinon-style -p spinon-runtime --all-targets -- -D warnings`가 통과했다. `cargo fmt --all -- --check`와 `git diff --check`도 통과했다.
- `cargo build --locked -p spinon-ffi --lib` 기본 artifact에서는 `spinon_runtime_gpu_host_new_c11_1_root_html_percent_columns_fixture` 심볼과 새 calc fixture payload가 모두 빠졌다. 같은 crate를 `--features c11-1-root-html-percent-columns-fixture-test`로 빌드했을 때 기존 opt-in 심볼과 `calc(10% + 2px) 25% 15px` payload가 포함됐다. 신규 FFI 심볼은 추가하지 않았다.
- 캡처는 화면의 draw/present 시각 확인이다. Android PixelCopy 또는 iOS Metal drawable readback을 하지 않아 표면 pixel equality를 주장하지 않는다. Android 근거는 SwiftShader AVD에 한정되고, iOS 결과는 Simulator이며 실기기나 compositor 이후 pixel 근거가 아니다.
- 이 simulator smoke는 주 fixture의 성공 장면 하나만 확인한다. clamp, percentage overflow, zero basis, gap 독립성, `fr` 혼합과 실패 atomicity의 경계는 별도 Rust focused tests 근거로 확인한다.
- 빌드는 opt-in 앱 검증 진입점을 사용했다. 신규 public API, 일반 CSS admission 또는 제품 feature flag 추가는 없다. GitHub PR/CI 및 merge는 아직 이 evidence에 포함되지 않는다.
