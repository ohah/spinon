# C11.4 auto-repeat absolute calc 최소값 · 제한 runtime 근거

계획 [C11.4 auto-repeat 고정 `calc()` 최소 track](../../../../plan/c11-4-auto-repeat-calc-minimum.md)의 Stylo computed-value 정규화 가설을 검증하고, 여섯 고정 장면을 Android/iOS 앱 시뮬레이터에 연결했다. 실제 결과는 처음 예상한 preflight 결손을 확인하지 않았다. Stylo가 `calc(20px + 20px)`를 cascade 단계에서 `LengthPx(40)`으로 정규화하고 현재 fixed-size gate가 이 값을 이미 허용한다. 제품 계산 경로를 추가하지 않았다.

## 판정과 장면

CSS Grid 2025 CRD §7.2.1·§7.2.3.2와 Chrome 156 관찰을 고정한 [사전 비교](../c11-4-auto-repeat-calc-minimum-precomparison-2026-10-23/README.md)를 사용했다. definite axis는 184px, gap은 0px, 최소값은 `calc(20px + 20px)`, 최대값은 `1fr`이다. fixed control은 `minmax(40px,1fr)`이다.

| 장면 | 예상 및 양 플랫폼 결과 |
| --- | --- |
| `columns-calc-fill` | root `184×48`; Columns `[46,46,46,46]`; 네 item `46×48` |
| `columns-fixed-fill-control` | calc fill control과 같은 root·track·item frame |
| `columns-calc-fit` | root `184×48`; Columns `[184,0,0,0]`; 빈 세 track 접힘, item `184×48` |
| `rows-calc-fill` | root `48×184`; Rows `[46,46,46,46]`; 네 item `48×46` |
| `rows-fixed-fill-control` | calc fill control과 같은 root·track·item frame |
| `rows-calc-fit` | root `48×184`; Rows `[184,0,0,0]`; 빈 세 track 접힘, item `48×184` |

Android API 37 ARM64 AVD `emulator-5558`과 iPhone 17 Pro/iOS 26.2 Simulator `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`에서 각 6/6 장면을 통과했다. 두 결과 모두 scene ID, root/item node frame, axis track, collapse 상태, C11.4 trace, `drawPresented=true`, `supersededRetries=0`을 기록했다. Android와 iOS의 모든 observed node frame 및 track 값은 고정 기대값과 일치한다.

| 플랫폼 | 실행 결과 | 실행 로그·결과 |
| --- | --- | --- |
| Android AVD | 6/6 scene, draw/present 6/6 | [결과 JSON](./android-avd-c114-absolute-calc-minimum-android-avd-20261023T075907042Z-3781412e/result.json) · [시뮬레이터 정보](./android-avd-c114-absolute-calc-minimum-android-avd-20261023T075907042Z-3781412e/simulator.json) · [캡처 stdout](./builds/android-capture-pass03.log) |
| iPhone 17 Pro Simulator | 6/6 scene, draw/present 6/6 | [결과 JSON](./ios-simulator-c114-absolute-calc-minimum-ios-simulator-20261023T080040579Z-bb22adde/result.json) · [시뮬레이터 정보](./ios-simulator-c114-absolute-calc-minimum-ios-simulator-20261023T080040579Z-bb22adde/simulator.json) · [캡처 stdout](./builds/ios-capture-pass01.log) |

화면은 장면 표시와 앱의 `layout=ready` 상태를 확인하기 위한 자료다. 이번 slice는 PixelCopy 또는 Metal drawable pixel equality를 검사하지 않았다. root 바깥/테두리 stroke, Android 실기기/hardware GPU, iOS compositor 이후 pixels, 일반 HTML/CSS 경로를 통과했다고 주장하지 않는다. [Android 전체 화면](./android-avd-c114-absolute-calc-minimum-android-avd-20261023T075907042Z-3781412e/columns-calc-fill.png) · [iOS 전체 화면](./ios-simulator-c114-absolute-calc-minimum-ios-simulator-20261023T080040579Z-bb22adde/columns-calc-fill.png) · [화면 SHA-256 12개](./screenshot-manifest.sha256).

## Stylo·fixture·빌드 검증

- Stylo fixture는 columns/rows × auto-fill/auto-fit 네 조합 모두에서 calc와 fixed control이 같은 `MinMax { min: LengthPx(40), max: Fraction(1) }`이 되는지 검사한다. Percentage control은 root node와 `grid-template-columns` 문맥이 있는 `UnsupportedComputedGridValue`로 거부된다.
- FFI scene validator는 6개의 고유 ID, fixture node/frame shape와 runtime source 경로를 검사한다. CSS-reference fixture boundary test는 19/19를 통과했다.
- Android app은 `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`로 빌드했다. APK SHA-256은 `f008e7475f574e36b85495f401e08cedbe723bc379244212bba1de9ef3b60f32`이다.
- iOS Simulator app은 같은 두 feature flag로 빌드했다. 실행 binary SHA-256은 `038b5185bece87fd087e8b79338e001bd8470b3d7c4a2a0a57d7eb137c9b6ab9`이다.
- 두 simulator 결과는 checkout commit `7c93a8e518e0820f1535b434fd62e61da03a0b9a`, tracked diff hash `a1b695da99af181c2dbcc4bee8c538317b9e0a773857856a4aec8c45a040f7a1`을 기록한다. 새 untracked 입력은 [source file manifest](./source-files-manifest.sha256)에 별도 고정한다.

주요 검증 로그:

- [Rust formatter](./builds/fmt-check.log), [새 cascade tests 2/2](./builds/style-check.log), [auto-repeat regression 8/8](./builds/auto-repeat-regressions.log), [FFI scene validator 1/1](./builds/ffi-check.log), [area projection regression 2/2](./builds/layout-area-auto-repeat-regressions.log)
- [workspace all-target Clippy `-D warnings`](./builds/clippy-workspace.log), [fixture boundary 19/19](./builds/boundary-pass02.log)
- [Android app build](./builds/android-build-pass02.log), [iOS Simulator app build](./builds/ios-build-pass01.log)

체크인한 stdout 로그는 도구 출력의 줄 끝 공백과 마지막 빈 줄만 정리했으며, 나머지 출력 내용은 보존했다.

## FFI static archive 경계

`cargo build --locked --release --no-default-features -p spinon-ffi --lib --features c04-runtime-gpu` product archive는 75,106,976 bytes, SHA-256 `29dd2bd115c0a417e0b98dffd45051e2d295dee90baebe21302b9f33e3dd4e68`이다. [raw-byte marker scan](./ffi-product-markers.json)은 새 module marker와 6 scene ID가 모두 absent임을 확인했다.

`--features c04-runtime-gpu-fixture-test` positive control archive는 78,138,384 bytes, SHA-256 `c22a374345c499c60e9a0d4f80560b0b1add9883a5d72f266ed6041664499220`이다. [raw-byte marker scan](./ffi-fixture-markers.json)은 7개 marker 모두 present임을 확인했다. `llvm-nm` 출력에는 내부 Rust 함수 `spinon_ffi::runtime_gpu::fixtures::c11_4_auto_repeat_calc_minimum::runtime_source`가 나타났으며, [symbol output](./ffi-fixture-symbols.txt)에 남겼다. 전체 archive symbol 명령은 출력 뒤 exit code 1을 반환했으므로 symbol 출력은 참고 기록으로만 둔다. 새 C ABI symbol은 추가하지 않았다. 이 정적 archive 비교는 최종 APK/IPA 전체의 fixture 제외를 증명하지 않는다.

## 재시도 기록과 제한

첫 Android 시도는 이전 APK를 사용해 `SPINON_C114_GRID_ARGUMENT_ERROR rejected=true`로 scene route 진입 전에 거부됐다. 앱을 최신 source로 다시 빌드한 뒤의 두 번째 시도는 신규 scene ID가 기존 pixel matrix allowlist에 없어 `scene-index-mismatch`로 끝났다. 이번 고정 slice에서는 pixel sampling 기능을 주장하지 않으므로 새 scene의 `pixelSamples` 입력을 빼고 재실행했다. 최종 APK에서 Android 여섯 장면이 통과했고, 이어 iOS에서도 여섯 장면이 통과했다. [실패한 첫 시도](../c11-4-auto-repeat-calc-minimum-runtime-2026-10-23-2026-10-23/android-avd-c114-absolute-calc-minimum-android-avd-20261023T075155809Z-cbee45aa/runtime-columns-calc-fill.log) · [실패한 두 번째 시도](./android-avd-c114-absolute-calc-minimum-android-avd-20261023T075424891Z-a17e962e/runtime-columns-calc-fill.log).

캐시는 이번 작업 시작 전에 이미 비워져 있었다. 재생성된 `target`은 전체 검증 뒤 7.1GB였다. 측정된 주요 단계는 workspace Clippy 14.36초, auto-repeat 회귀 tests 17.77초, area projection 회귀 tests 17.71초, Android Gradle assemble 4초다. iOS Xcode build는 성공 로그를 남겼지만 수행 시간을 별도 측정하지 않았다. 이 slice는 CSS Grid 전체 지원, WPT, 제품 CSS admission, 공개 API, 다른 calc 연산자/단위, percentage calc 지원 또는 전체 C11.4 완료를 뜻하지 않는다. 내부 계약 numeric version `0.1.0`과 공식 C11.4 미완료 상태를 유지한다. 실행 기록의 `74/207`은 당시 집계 스냅샷이며 현재 공식 집계는 `72/213`이다.

계획 수정은 [별도 실패 관점 20개](../../../../plan/c11-4-auto-repeat-calc-minimum-revision-review-20.md)로 검토했다. 구현/runtime 관점은 [새 실패 관점 20개](./implementation-review-20.md), implementation PR diff는 [별도 30관점](./pr-diff-review-30.md)에 기록한다.
