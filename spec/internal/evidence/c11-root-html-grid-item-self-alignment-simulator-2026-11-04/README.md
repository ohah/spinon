# C11.5 Root HTML Grid item self-alignment 시뮬레이터 근거

공유 19-scene fixture를 기존 `root-html-default-rendering.html` resource로 임시 번들링해 Root HTML runtime/GPU 앱 경로에서 실행했다. 두 플랫폼의 추적 중인 원본 resource는 빌드 후 복구했다. 양쪽 번들 fixture SHA-256은 `30a156fe7c9dfb02c3f4fb7148cb1205b8c9360a1061f349818ace7076c16ad1`이다.

## 결과

- Chrome 156 사전 비교는 positive 17개와 CSS-invalid cascade control 2개, 총 19/19 exact다.
- Android API 37 ARM64 AVD `spinon_api37_0_applied_txn`에서 ANGLE/Vulkan SwiftShader를 사용했다. Runtime은 `layout=ready`, `boxes=79`, `SPINON_C0410_DRAW status=0 presented`를 기록했다. SurfaceView PixelCopy 표본은 7/7 일치했다. APK SHA-256은 `6484469d7962265c9e04ae30a63696df2db72dc5ae44bc628592e1ea1ac38cbb`다.
- iOS iPhone 17 Pro/iOS 26.2 Simulator에서 Metal을 사용했다. Runtime은 `layout=ready`, `boxes=79`, draw/present를 기록했다. 980×735 CSS px viewport와 960×720 drawable에서 compositor 제출 전 Metal drawable 표본 7/7이 일치했다. 앱 executable SHA-256은 `253e189db10ea9ee5e28e77f30f2a32f33b153778f97593cb91d6567c930ee28`다.
- 7개 표본은 기본 정렬, item edge와 경계 바로 바깥, safe/unsafe overflow, viewport 아래쪽 잘림 경계의 안/밖을 검사한다. Android는 `SPINON_ENABLE_C11_5_ROOT_HTML_GRID_ITEM_SELF_ALIGNMENT_FIXTURE_TEST=1`, iOS는 기존 opt-in `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`로 빌드했다.
- 별도 OS 합성 시뮬레이터 screenshot 표본도 각 7/7이다. 전체 19-scene item frame matrix는 focused Rust runtime test에서 확인했다.

## Rust 검증 상태

- 현재 source의 focused C11.1 Grid item alignment tests 7/7, C11.5 runtime tests 3/3, CSS-invalid self-alignment diagnostic 1/1이 통과했다. 변경 crate 네 곳의 `cargo clippy --locked -p spinon-layout -p spinon-style -p spinon-style-to-layout -p spinon-runtime --all-targets -- -D warnings`와 `cargo fmt --all -- --check`도 통과했다.
- broad package tests는 통과 판정이 아니다. `spinon-layout`은 423 passed/1 failed, `spinon-style-to-layout`은 372/1, `spinon-style`은 283/1이다. 각각 C15 Grid text error-kind, C07 aspect-ratio serialization, 다른 Grid profile의 typed track-list assertion이 실패했다. `spinon-runtime`은 261 passed/2 failed/3 ignored이며 두 실패는 PR #1117 dense auto-placement 이후에도 이전 `unsupported_computed_value`를 기대하는 rejection assertion이다. 이 dense 입력은 현재 layout 단계의 unsupported `box-sizing`에서 fail-closed 되며 일부 scene을 공개하지 않는다. 후보 전체 broad tests는 green이 아니다.
- 최신 broad output은 `rust-package-layout-final.log`, `rust-package-style-to-layout-final.log`, `rust-package-style-final.log`, `rust-package-runtime-final.log`에 package별로 보존한다. `rust-package-regressions.log`는 self-alignment assertion 수정 전의 첫 검사 로그이므로 최신 결과로 해석하지 않는다. 그 오래된 C11.1 assertion은 새 지원과 충돌해 `baseline`/`safe start` unsupported-value controls로 바꿨으며, 현재 focused C11.1 regression 7/7이 통과한다.

## Android fixture artifact 경계

- Default build (`SPINON_ENABLE_C04_RUNTIME_GPU=0`, C04 fixture-test `0`, C11.5 fixture-test `0`) 성공 APK SHA-256은 `56e718d3293e29d9a8422fbe9df5ad9c04962d064d8ad5230598ff8b2d59d18d`다. 생성된 BuildConfig의 C11.5 flag는 `false`이며 APK DEX에서 C11.5 PixelCapture adapter 클래스명과 fixture scene ID를 찾지 못했다. 기본 assets의 `root-html-default-rendering.html`에도 공유 C11.5 fixture 선택자는 없다.
- Opt-in positive build (`SPINON_ENABLE_C04_RUNTIME_GPU=1`, C04 fixture-test `1`, C11.5 fixture-test `1`) 성공 APK SHA-256은 `da28b8f0edd1d2825f4e7e14145dc41d74c5252951f3add1548593fa786e3c94`다. BuildConfig flag는 `true`이며 DEX에서 adapter와 고정 표본 ID를 확인했다.
- 재현 로그는 `android-default-artifact-build-final.log` 및 `android-fixture-artifact-build-final.log`다. 기본 빌드의 DEX 검사에서 adapter class name이 처음 남는 것을 발견해 reflection call을 C11.5 compile-time flag 아래로 옮긴 다음 두 build를 다시 확인했다. 이전 instrumentation APK SHA는 simulator run logs에 보존하고 현재 boundary build hash는 metadata에 별도로 기록한다.
- iOS Simulator 기본 build (`SPINON_ENABLE_C04_RUNTIME_GPU=0`, C04 fixture-test `0`, C11 Metal surface test `0`)의 app executable SHA-256은 `751a82482275b6ef64bb27e595f3c0d62464c6a6924e6ef874de0e5bfd92b7b1`이다. executable에서 C11.5 sample argument 및 fixture scene ID를 찾지 못했다. Opt-in build (세 플래그 `1`)의 executable SHA-256은 `f98ffcad8a3b6648048709a864462b6e82e882c17de669e04fdaa31b558cd4a4`이며 C11.5 argument와 scene ID가 포함된다. Build log는 `ios-default-artifact-build-final.log`, `ios-fixture-artifact-build-final.log`다.
- 기본 iOS binary 검사에서 root HTML route의 conflict filter가 `--spinon-root-*` 인수를 검사하지 않으면서도 C11.5 예외 문자열을 보유한 것을 발견했다. 해당 불필요한 예외를 제거해 기본 executable에서 candidate 전용 인수가 빠지는지와 opt-in executable에서 포함되는지를 다시 빌드해 확인했다.

## 파일

- `android-pixelcopy.log`, `android-surfaceview-pixelcopy.png` — Android PixelCopy 기록과 840×630 SurfaceView 캡처.
- `ios-metal-readback.log`, `ios-iphone17pro-metal-readback.png` — Metal 표본 기록과 별도 OS 합성 screenshot.
- `android-api37-avd.png`, `android-api37-avd.log`, `android-emulator.log`, `ios-iphone17pro-simulator.png`, `ios-simulator.log` — 시뮬레이터 실행·화면 근거.
- `simulator-screen-samples.json`, `verify-screen-samples.py` — screenshot 표본 추출 및 검증기.
- `simulator-build-metadata.json` — build hash, 실행 경로와 근거 종류.

Pillow가 있는 환경에서 `python3 spec/internal/evidence/c11-root-html-grid-item-self-alignment-simulator-2026-11-04/verify-screen-samples.py`를 실행하면 OS 합성 screenshot 표본을 다시 검증할 수 있다.

## 검증 범위

Android PixelCopy와 Metal drawable 표본은 직접 surface readback이며, iOS readback은 compositor 제출 전이다. PNG screenshot은 OS 합성 결과로 별도 근거다. Android는 SwiftShader를 사용했으며 hardware GPU 결과가 아니다. 이번 실행은 실기기 동작, scanout 시각, 전체 CSS Grid 지원, WPT 적합성, 일반 CSS admission 또는 C11/C11.5 전체 완료를 증명하지 않는다. 구현/runtime 검토 20개는 [별도 기록](implementation-runtime-review-20.md)에 있다. 구현 PR diff 검토 30개와 구현 PR 생성은 남아 있다. 공식 완료 수는 `72/213`이다.
