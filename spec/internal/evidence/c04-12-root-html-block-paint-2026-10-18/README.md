# C04.12 파싱 루트 HTML Block paint — 구현 근거

**확인일:** 2026-10-18  
**범위:** opt-in fixture가 기존 HTML5 parser 문서 트리를 제한된 Block cascade·Taffy layout·`RuntimeRenderSnapshot`·WGPU 장면까지 전달하는 내부 검증이다. 일반 HTML 화면 지원이나 C04.12 상위 완료를 뜻하지 않는다.

## 입력과 oracle

- Fixture: [`root-html-block-paint.html`](../../../../tests/fixtures/css/c04/root-html-block-paint.html)
- 입력 SHA-256: `39dabf47aa5e3ff279421d6d399ea0246399b067e5290162805a24dd8826e899`
- 기준은 [Chrome 155 사전 비교](../c04-12-root-html-block-paint-precomparison-2026-10-18/README.md)다. Chromium과 앱에 같은 HTML source, CSS layout viewport `320×240`, light/dark preference를 적용했다.
- 고정 장면은 `html` 배경 `#101827`, `main` 파랑 `#3366ff`, light 자식 초록 `#40a060`, dark 자식 갈색 `#a05c2c`다. 숨겨진 자손의 빨강·초록은 그리지 않는다. HTML text glyph는 범위 밖이다.

| 표본 | CSS 위치 | 기대 light RGBA | 기대 dark RGBA |
|---|---:|---|---|
| hero | `(10, 10)` | `[51,102,255,255]` | `[51,102,255,255]` |
| scheme child | `(10, 40)` | `[64,160,96,255]` | `[160,92,44,255]` |
| child 뒤의 root | `(10, 60)` | `[16,24,39,255]` | `[16,24,39,255]` |
| 투명 Block 아래의 root | `(200, 40)` | `[16,24,39,255]` | `[16,24,39,255]` |
| body 바깥의 root | `(10, 210)` | `[16,24,39,255]` | `[16,24,39,255]` |

## Rust 검증

- `spinon-style`: 180 통과, 실패 0. 새 `root_html_block_paint_rejects_partial_alpha_and_non_srgb_backgrounds` 회귀 테스트는 부분 alpha와 Display-P3 배경을 모두 `UnsupportedComputedBackgroundColor`로 거부한다.
- `spinon-runtime` opt-in C04.12 slice: 2 통과, 실패 0. light/dark frame과 root HTML의 visible text·SVG 거부 및 `display:none` 아래 text 처리를 확인한다.
- `spinon-ffi` readback 좌표 단위 테스트: 2 통과, 실패 0. 해당 실행은 색 경계 보강 전이며 FFI/readback 코드는 그 뒤 바뀌지 않았다.
- Android fixture source set의 현재 Java 코드는 최종 reflection 오류 처리 뒤 `compileDebugJavaWithJavac`가 통과했다. `prepareSpinonBootstrap`은 제외한 Java compile이며 최신 Rust static library 재빌드를 뜻하지 않는다: [android-java-build.log](../c04-12-root-html-block-paint-android-simulator-2026-10-18/android-java-build.log).
- [`cargo-test-style.log`](../c04-12-root-html-block-paint-ios-simulator-2026-10-18/cargo-test-style.log), [`cargo-test-runtime.log`](../c04-12-root-html-block-paint-ios-simulator-2026-10-18/cargo-test-runtime.log), [`cargo-test-ffi.log`](../c04-12-root-html-block-paint-ios-simulator-2026-10-18/cargo-test-ffi.log)
- `cargo fmt --all -- --check`와 `git diff --check`는 최종 Rust 수정 뒤 통과했다.
- 최초 기본 `cargo check`에서 opt-in 계산 함수의 `dead_code` 경고가 나와 함수 자체를 전용 feature로 게이트했다. 이후 최종 기본 archive 빌드는 경고 없이 끝났다. [최초 경고](../c04-12-root-html-block-paint-ios-simulator-2026-10-18/cargo-check-default-ffi-initial-warning.log)와 [최종 빌드](../c04-12-root-html-block-paint-ios-simulator-2026-10-18/cargo-build-default-artifact.log)를 구분한다.
- 기본 `spinon-ffi` archive는 최종 색 경계 수정 뒤 다시 빌드했다. fixture FFI symbol과 fixture title/payload는 없다: [`cargo-build-default-artifact.log`](../c04-12-root-html-block-paint-ios-simulator-2026-10-18/cargo-build-default-artifact.log), [`ffi-default-artifact-check.txt`](../c04-12-root-html-block-paint-ios-simulator-2026-10-18/ffi-default-artifact-check.txt). opt-in positive artifact의 symbol·payload는 [fixture-test build evidence](../c04-12-root-html-block-paint-ios-simulator-2026-10-18/ffi-opt-in-symbols.txt)와 [strings 검색](../c04-12-root-html-block-paint-ios-simulator-2026-10-18/ffi-opt-in-strings.txt)에 기록했다.

## Android API 37 ARM64 AVD

`sdk_gphone16k_arm64`, API 37, `arm64-v8a`, 화면 `1080×2400 @ 420 dpi`. 앱 CSS viewport는 `320×240`, WGPU surface는 `840×630`, scale `2.625`다. Renderer는 ANGLE을 거친 Vulkan SwiftShader software backend이며 hardware GPU 측정이 아니다.

이 AVD에서는 표면 `COPY_SRC` 생성이 실패했다. 검증은 실제 `SurfaceView`를 Android `PixelCopy`로 캡처하고, Rust가 보고한 CSS pixel-center 투영 좌표와 PixelCopy 표본 좌표를 각각 비교한다. light/dark 모두 좌표와 RGBA가 5/5 일치했다. PixelCopy는 앱 surface 결과이며 물리 패널의 최종 표시를 검증하지 않는다.

| 모드 | 실행 결과 | 화면 | PixelCopy surface PNG | runtime log |
|---|---|---|---|---|
| light | `matched`, 5/5 | [light.png](../c04-12-root-html-block-paint-android-simulator-2026-10-18/light.png) · SHA-256 `10947f09da0d50268ffd5b59158f0e88d2ac31705fc84250b3dab423ae254c70` | [light-surface.png](../c04-12-root-html-block-paint-android-simulator-2026-10-18/light-surface.png) · SHA-256 `133c59061fffdc67d8d2ddeb15d2b09725b57103c162015beb8d4ec31806ad04` | [light-runtime.log](../c04-12-root-html-block-paint-android-simulator-2026-10-18/light-runtime.log) |
| dark | `matched`, 5/5 | [dark.png](../c04-12-root-html-block-paint-android-simulator-2026-10-18/dark.png) · SHA-256 `380d436712512a431f88ce4fb151e9cd652e254b9dc9d39a559ef28b20efd672` | [dark-surface.png](../c04-12-root-html-block-paint-android-simulator-2026-10-18/dark-surface.png) · SHA-256 `7268926720c87c6dd7c9a5574ecc0a74df0d65fb773abe7ccedf2e3482a90c25` | [dark-runtime.log](../c04-12-root-html-block-paint-android-simulator-2026-10-18/dark-runtime.log) |

Android 표면 표본 좌표는 `(27,27)`, `(27,106)`, `(27,158)`, `(526,106)`, `(27,552)`다. 표본 개수 불일치, 오래된 캡처, reflection adapter 부재는 화면에 실패 상태를 표시한다. 캡처 뒤 앱 프로세스를 종료했고 AVD는 light mode로 켜 둔 상태다.

## iOS 26.2 Simulator

iPhone 17 Pro Simulator (`iPhone18,1`), WGPU Metal `Bgra8Unorm`, 표면 `960×720`, CSS viewport `320×240`, scale `3`. light/dark의 Metal drawable 표본 5개는 위 표와 일치했다. readback은 compositor 제출 전이므로 최종 compositor 또는 실기기 결과를 증명하지 않는다.

| 모드 | 화면 | runtime log |
|---|---|---|
| light | [light.png](../c04-12-root-html-block-paint-ios-simulator-2026-10-18/light.png) · SHA-256 `427e47f68ec42e3a67635286b4205645fe2428e81453fa34dc9095bcf12ffa94` | [light-runtime.log](../c04-12-root-html-block-paint-ios-simulator-2026-10-18/light-runtime.log) |
| dark | [dark.png](../c04-12-root-html-block-paint-ios-simulator-2026-10-18/dark.png) · SHA-256 `3cd17e7c70ac1f252bcdc4294c22d51759899180e254ad9f3480e53b33f06220` | [dark-runtime.log](../c04-12-root-html-block-paint-ios-simulator-2026-10-18/dark-runtime.log) |

캡처 뒤 Simulator 앱을 종료하고 appearance를 light로 복원했다. 두 플랫폼 모두 연결된 실기기를 사용하지 않았다.

## 변경 뒤 검증 경계

위 Android/iOS opt-in 앱 빌드와 네 Simulator 캡처는 마지막 Rust 색상 경계 보강 전에 생성했다. 수정은 새 profile을 기존 root HTML 색 공간 변환기에 연결하고 부분 alpha를 거부한다. 고정 fixture는 불투명 8-bit sRGB만 사용하므로 캡처의 성공 경로 입력은 그대로다. 수정 뒤 `spinon-style` 180개 전체 테스트와 `spinon-runtime` 집중 테스트 2개가 통과했다. 수정 후 Android/iOS native 재빌드와 재캡처는 하지 않았다.

구현 검토는 [서로 다른 실패 관점 20개](../c04-12-root-html-block-paint-implementation-review-20-2026-10-18.md)다. PR 변경분은 병합 전 별도 30개 관점으로 검토한다. C04.12 전체, 일반 HTML layout/paint, text rendering, 실기기, hardware GPU, compositor 이후 픽셀은 범위 밖이다.
