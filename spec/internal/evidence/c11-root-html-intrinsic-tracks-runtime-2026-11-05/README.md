# C11.1 Root HTML intrinsic Grid tracks 실행 근거

## 판정 범위

이 근거는 `RuntimeRootHtmlTextPaintV1`의 기본 parsed Root HTML 실행 경로에서 고정 explicit intrinsic track fixture가 layout과 draw/present까지 도달하는지 기록한다. 앱의 기존 C04.12 Root HTML DefaultRender 진입점을 사용했으며 일반 bootstrap HTML이나 새 C11 전용 FFI scene selector를 추가하지 않았다. Android/iOS 앱에는 [runtime fixture](../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-tracks-runtime.html)를 일시 적용하고 빌드 뒤 기존 resource를 복원했다. Chrome 사전 비교는 19 scenes이고, 이 Root HTML runtime fixture는 양의 definite container axis를 가진 17 scenes다. zero-axis percentage/`fr` 두 장면은 Chrome oracle only이며 앱 runtime은 zero-width/height container를 진단으로 거부한다.

- runtime fixture source: 5,448 bytes, SHA-256 `1cb65aa1c2169fa09b39287ba3998b4c5783b869a6aa634ab01ac131df2c7018`
- Android APK와 iOS 앱 번들의 `root-html-default-rendering.html` hash가 runtime fixture hash와 일치한다.
- 복원된 Android/iOS tracked asset SHA-256은 각각 `38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85`로 기존 원본과 일치한다.
- 두 simulator 모두 `layout=ready`, `boxes=54`, `runtime_text_count=17`, `glyph_batch_draw_count=6`, `presented boxes=54`를 보고했다.
- runtime 문서는 17개 고정 scene, 320×240 CSS px viewport와 320×680 CSS px root frame을 사용한다. 전체 scene/frame 판정은 Rust runtime matrix에서 수행했다.
- 기본 runtime font backend의 `min-content` item은 30.004883 CSS px였고 Chrome 156은 30.1094 CSS px였다. 약 0.105px 차이는 text advance에 한정되어 통합 runtime matrix에는 0.5px tolerance를 쓴다. Text measurement와 독립된 layout trace matrix는 0.02px 이내로 별도 확인한다.

## Android API 37 ARM64 AVD

- AVD: `spinon_api37_0_applied_txn`, serial `emulator-5554`, API 37 ARM64
- renderer: ANGLE / SwiftShader software GPU (`OpenGL ES Translator`, Vulkan SwiftShader)
- viewport: 320×240 CSS px; root frame: 320×680 CSS px; device scale: 2.625; surface: 840×630
- 최종 캡처: [simulator-screen.png](android/simulator-screen.png), 1080×2400, SHA-256 `9293d464ccbf7db7c9fe8baa2ba882596f90f686bbb3da753aacf8fb743dee0d`
- 최종 실행 로그: [logcat-final.txt](android/logcat-final.txt), [launch-final.log](android/launch-final.log), [build-final.log](android/build-final.log), [install-final.log](android/install-final.log)
- APK와 포함 fixture hash: [apk.sha256](android/apk.sha256), [apk-root-html-asset.sha256](android/apk-root-html-asset.sha256)

`logcat-final.txt`는 source hash, Root HTML layout 결과, renderer 및 draw/present 표식의 실제 logcat 행이다. 앱은 캡처 후 force-stop했다. PixelCopy는 수집하지 않았으므로 이 화면을 Android surface pixel readback으로 해석하지 않는다.

## iOS Simulator

- 기기: iPhone 17 Pro, iOS 26.2 Simulator, UUID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`
- renderer: Metal, Apple iOS simulator GPU
- viewport: 320×240 CSS px; root frame: 320×680 CSS px; device scale: 3; surface: 960×720
- 최종 캡처: [simulator-screen.png](ios/simulator-screen.png), 1206×2622, SHA-256 `4567e10ae2587788b7a62f6aee1b431092c6e9737143f1b43e887bb2bbba9685`
- 최종 실행 로그: [unified-log-final.txt](ios/unified-log-final.txt), [launch-final.log](ios/launch-final.log), [build-final.log](ios/build-final.log), [install-final.log](ios/install-final.log), [simulator-screen.sha256](ios/simulator-screen.sha256), [app-executable.sha256](ios/app-executable.sha256)
- 앱 asset hash: [app-root-html-asset.sha256](ios/app-root-html-asset.sha256)

앱은 캡처 후 terminate했다. Metal drawable 표본은 읽지 않았으므로 screenshot은 compositor 이후 색상 동등성 근거가 아니다.

## Rust 및 경계 검사

마지막 코드 수정 뒤 실행한 focused gate:

| 명령 | 결과 |
| --- | --- |
| `cargo test --locked -p spinon-runtime --features c11-1-grid-intrinsic-tracks-fixture-test parsed_root_html_intrinsic --lib` | 4 passed |
| `cargo test --locked -p spinon-layout --lib root_html_intrinsic_tracks` | 2 passed; direct `auto`, `min-content`, `max-content`, `minmax()` track trace와 mixed auto/one-axis placement rejection을 확인하고 geometry를 0.02 CSS px 안에서 비교 |
| `cargo test --locked -p spinon-style-to-layout --lib root_html_profile` | 22 passed |
| `cargo test --locked -p spinon-style-to-layout --lib parsed_root_html_aspect_ratio_fails_closed_outside_the_fixed_owner_slice` | 2 passed |
| `cargo clippy --locked -p spinon-layout -p spinon-style-to-layout -p spinon-style -p spinon-runtime --all-targets --features spinon-runtime/c11-1-grid-intrinsic-tracks-fixture-test -- -D warnings` | 통과 |
| `cargo check --locked -p spinon-ffi --features c04-runtime-gpu` | 통과 |
| `mise exec -- bun test tools/css-reference/fixture-module-boundary.test.mjs` | 33 passed |
| `cargo fmt --all -- --check` | 통과 |

전체 변경 크레이트 단위 테스트에는 아래의 알려진 실패가 남아 있어 전체 suite green으로 판정하지 않는다.

- `spinon-style-to-layout --lib`: 394/404 통과. C07 aspect-ratio 직렬화 기대값 1건, 현재 fixed-track auto-size/stretch 계약에 걸리는 C11.5 alignment/gap 회귀 8건, 혼합 partial implicit placement 기대값 1건.
- `spinon-layout --lib`: 443/444 통과. 이번 변경의 두 고정 입력 intrinsic track/placement 검사는 통과했고, 기존 C15 `host_document_collects_direct_text_and_inherited_spans_in_source_order` 1건이 실패했다.

위 실패들은 이번 intrinsic runtime focused matrix의 통과에 포함하지 않았다. 정적 feature-boundary 검사와 제품 `c04-runtime-gpu` check는 기본 제품 feature에 이 slice 전용 fixture ABI를 추가하지 않은 상태로 통과했다.

## 화면 및 프로세스 한계

두 캡처는 전체 simulator screenshot이다. Android PixelCopy와 iOS Metal drawable readback은 수행하지 않았다. simulator 결과를 실기기, hardware GPU 또는 iOS compositor 이후 색상 일치로 확대 해석하지 않는다. Chrome 156 precomparison은 Agent Browser로 캡처했고 해당 session을 닫았다. 마지막 프로세스 확인에서 `agent-browser`는 남아 있지 않았다.

Android 최종 빌드는 V8 도구 경로를 상대경로로 전달한 첫 시도에서 실패했다. clang이 `-fuse-ld=build/c11-v8-rebuild/.../ld.lld`를 linker name으로 처리했다. 절대 `SPINON_V8_DIR`로 재실행해 성공했으며 해당 환경 오류는 [별도 기록](android/build-relative-v8-path-blocker.md)에 남겼다. 실패 시에도 app asset 복원 trap이 실행됐고 양 플랫폼 원본 hash는 유지됐다.
