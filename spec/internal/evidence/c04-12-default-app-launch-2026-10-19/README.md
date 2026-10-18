# C04.12 기본 앱 시작 경로 실행 근거

2026-10-19에 기본 실행 인수 없이 앱을 실행해 실제 `examples/bootstrap/index.html`과 `root-app.js`가 같은 root HTML GPU host에서 표시되는지 확인했다. 기기는 사용하지 않았고 Android API 37 ARM64 AVD, iPhone 17 Pro와 iPad Pro 13-inch M5 / iOS 26.2 Simulator만 사용했다.

## 관찰 결과

| 실행 환경 | Android | iOS |
| --- | --- | --- |
| 시뮬레이터 | API 37 ARM64 AVD, `emulator-5554`, `sdk_gphone16k_arm64` | iPhone 17 Pro와 iPad Pro 13-inch M5, iOS 26.2 |
| drawable / 캡처 크기 | content surface `1080×2209`, 화면 `1080×2400` | iPhone drawable `1206×2334`, 화면 `1206×2622`; iPad 화면 `2064×2752` |
| CSS viewport / scale | `411.42856×841.52374`, `2.625` | iPhone `402×778`, scale `3`; iPad `1032×1319`, scale `2` |
| renderer | OpenGL ES / ANGLE / SwiftShader, `Rgba8Unorm` | Metal simulator GPU, `Bgra8Unorm` |
| launch | `index.html` 367 bytes, `root-app.js` 1650 bytes, `root_app_marker=ready` | 두 Simulator에서 `index.html` 367 bytes, `root-app.js` 1650 bytes, `root_app_marker=ready` |
| scene | `layout=ready`, 3 boxes, 1 text, 1 glyph batch | iPhone과 iPad 모두 `layout=ready`, 3 boxes, 1 text, 1 glyph batch |

Light와 dark 양쪽에서 Android와 iPhone 콘텐츠 surface의 위·아래 좌우 모서리 안쪽과 중앙 픽셀을 확인했다. iPad에서도 같은 다섯 지점을 확인했다. 모두 CSS sRGB `#1e8c46`, 즉 `(30, 140, 70)`이었다. Android 콘텐츠 영역은 캡처 y `128–2336`, iPhone은 y `186–2519`, iPad는 y `64–2702`이며 상태 막대와 하단 시스템 영역은 네이티브 표면으로 남는다.

Android는 인수가 없는 실행과 표준 `android.intent.extra.TEST` extra가 추가된 실행을 각각 확인했다. 둘 다 `SPINON_ROOT_HTML_DEFAULT_APP_ROUTE`로 진입해 `root-app.js` 실행과 `layout=ready`를 기록했다. 플랫폼 debug extra가 없는 일반 intent에 다른 Android extra가 있어도 기본 root HTML 경로를 유지한다.

| 화면 | Android | iOS |
| --- | --- | --- |
| Light | [캡처](android-api37-light.png) | [iPhone](ios-26-2-light.png) · [iPad](ios-ipad-pro-26-2.png) |
| Dark | [캡처](android-api37-dark.png) | [iPhone](ios-26-2-dark.png) |

OS appearance를 dark로 바꿨을 때 Android와 iPhone 런타임 모두 `dark=true`, `environment_revision=1`, `layout=ready`로 갱신됐다. light로 복구한 뒤 새 환경 revision이 기록됐다. 명시적인 body 배경색이 있으므로 두 appearance 모두 content surface 색은 동일하다. Android AVD 표면을 `1080×2400`에서 `720×1600`으로 바꿨다가 원래 크기로 복구했고, 양쪽 변경에서 CSS viewport와 environment revision이 갱신됐다. iPad의 별도 surface 크기도 초기 환경 계산과 pixel capture로 확인했다.

## 발견과 수정

첫 실행에서는 CSS body 배경이 본문 박스 높이까지만 칠해지고 GPU surface의 나머지는 검정으로 남았다. 기존 `RuntimeSession::resolve_root_html_canvas`는 HTML/body 색을 계산했지만 새 기본 GPU route는 그 값을 native window 배경으로만 넘기거나 프레임 clear에 연결하지 않았다. GPU surface가 native window 위에 불투명하게 그려져 있어 window 배경색은 보이지 않았다.

기존 Canvas 색 선택을 이미 적용된 HTML 환경 snapshot으로 재사용할 수 있게 분리하고, 일반 GPU renderer의 sRGB clear color로 전달했다. 기본 clear 동작은 기존처럼 검정이며 root HTML 기본 앱 host에서만 계산된 Canvas 색을 사용한다. Simulator 캡처의 모든 content 가장자리에서 Chrome 155 사전 관찰값과 같은 RGB를 확인했다.

## 빌드와 fixture 경계

- Android와 iOS Simulator 모두 `c04_runtime_gpu=1`, `c04_runtime_gpu_fixture_test=0`으로 빌드했다. 나머지 C04.12/C11/C13/C14/C15/C18/C19 fixture flags도 빌드 로그에서 모두 `0`이다.
- Android APK SHA-256: `3904ff9e2b668d430c41847c600c69f74e60bb751f135209767fc7dcb1f68181`
- iOS Simulator executable SHA-256: `0aab85c709c4c7b06418f882bd832ab94977ba8629f2c0189df2d01529becb82`
- [Android 빌드 로그](android-build.log) · [iOS Simulator 빌드 로그](ios-build.log)
- [root Canvas focused Rust 테스트 9개](rust-root-canvas-test.log) · [FFI product feature check](cargo-ffi-check.log) · [iOS build-flag 테스트 13개](bun-ios-build-flags-test.log)
- [Rust format 확인](rustfmt-check.log) · [수정한 19개 JS 도구 syntax 확인](node-script-syntax-check.log) · [shell syntax 확인](shell-syntax-check.log) · [Xcode project lint](pbxproj-lint.log) · [diff whitespace 확인](diff-check.log)
- [Android 인수 없는 실행 로그](android-runtime-rerun.log) · [Android 일반 extra 실행 로그](android-runtime-generic-extra.log) · [Android fixture 비활성 경로 로그](android-runtime-fixture-disabled.log)
- [Android 화면 크기 변경 로그](android-runtime-resize.log)
- [iPhone 런타임 로그](ios-runtime-rerun.log) · [iPhone dark 전환 로그](ios-runtime-dark-rerun.log) · [iOS fixture 비활성 경로 로그](ios-runtime-fixture-disabled.log) · [iPad 런타임 로그](ios-ipad-pro-runtime.log)
- [시뮬레이터 캡처 픽셀 표본 로그](screenshot-pixels.log)
- macOS arm64 FFI staticlib을 현재 소스로 release 빌드해 product/positive-control feature를 대조했다. 두 archive 모두 generic root HTML host symbol이 있고, text fixture 심볼과 `tests/fixtures/resources/c15/Ahem.ttf`의 정확한 bytes는 product archive에는 없고 fixture positive-control에만 있다. [경계 검사 로그](artifact-boundary-check.log)
  - product: `--features c04-runtime-gpu`, 74,710,352 bytes, SHA-256 `558a4059385c5d4d0182eb0549afa3e9bbfa4abf6d279bfd9f843f31afaacc9a`
  - fixture positive control: `--features c04-12-root-html-text-paint-fixture-test`, 75,471,312 bytes, SHA-256 `955369fc6539b9187b55a30d0a593b75416b773b46d99b066b69a260199eca94`
  - [product archive build log](artifact-product-build.log) · [fixture archive build log](artifact-fixture-build.log)
- 현재 macOS `nm`은 Rust 1.96 LLVM 22.1.2 object 속성을 읽지 못해 archive 전체 byte-search로 심볼 문자열과 fixture payload를 비교했다. 이 archive 검사는 모바일 최종 링크 산출물 검사가 아니다.
- 이 byte-search 대조는 macOS arm64 staticlib 경계다. Android/iOS app build flags는 fixture disabled를 확인했지만 final mobile linker artifact에서 모든 fixture family가 absent인지 증명하지 않는다.
- 빌드는 pinned V8 checkout `/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8`를 읽기 전용 입력으로 사용했다. 빌드 산출물은 이 작업 checkout 안에 생성했다.
- 전체 화면 캡처는 simulator screenshot이다. Android PixelCopy, iOS compositor 이후 픽셀, 물리 기기 및 hardware GPU는 검증하지 않았다.

새 구현 실패 관점 20개 검토는 [검토 기록](implementation-review-20.md)에 남겼다. PR diff 30개 검토와 PR 병합은 아직 남아 있다. 이 근거는 C04.12/C15/C16/C19 전체 지원 완료를 뜻하지 않는다.
