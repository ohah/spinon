# C04.12 문서 Canvas 배경 · Chrome 사전 비교

**상태:** 구현 전 oracle 관찰 · 앱/Spinon 결과 아님

**계획:** [`plan/c04-12-document-canvas-background.md`](../../../../plan/c04-12-document-canvas-background.md)

## 고정 환경

- 실행 방식: Agent Browser `0.31.1`, 독립 session `spinon-c04-12-background-c5b41ef599cc`; Terminal Browser는 사용하지 않았다.
- 기준: Headless Chromium `155.0.8059.85` (`navigator.userAgentData.fullVersionList`), macOS `26.5.1`.
- CSS viewport `390×844`, device scale factor `3`, screenshot `1170×2532`.
- fixture: [`canvas-background.html`](canvas-background.html), SHA-256 `47bf1b50febe3272fb2216e0bba73cf1de0c94a84829111fa54cd5bf7042a126`.
- 문서에는 `html`과 `body`만 있고 body 자식은 없다. 1차 초안에서 body에 `min-height:100vh`를 주자 body box 배경이 viewport를 덮어 root canvas 비교와 구분되지 않았다. 그 선언을 제거해 CSS canvas 배경 선택만 측정했다. 이는 body box 배경 paint가 별개라는 점도 확인했으며, 이번 구현 범위에 그 paint를 넣지 않는다.
- 각 screenshot의 물리 pixel `(4,4)`와 `(300,700)`은 같았다. 표의 값은 둘 다에서 읽은 RGBA8이다. screenshot 파일 hash는 [`SHA256SUMS`](SHA256SUMS)에 있다.

## 관찰 결과

| Case | OS preference | html computed background | body computed background | Canvas screenshot sample |
|---|---|---|---|---|
| 기본 transparent | light | `rgba(0, 0, 0, 0)` | `rgba(0, 0, 0, 0)` | `(255,255,255,255)` · [화면](chrome-light-default.png) |
| 기본 transparent | dark | `rgba(0, 0, 0, 0)` | `rgba(0, 0, 0, 0)` | `(18,18,18,255)` · [화면](chrome-dark-fallback.png) |
| html inline `rgb(18,52,86)`, body inline `rgb(171,205,239)` | light | `rgb(18, 52, 86)` | `rgb(171, 205, 239)` | `(18,52,86,255)` · [화면](chrome-light-both-opaque-empty-body.png) |
| html stylesheet `rgb(18,52,86)` | light | `rgb(18, 52, 86)` | `rgba(0, 0, 0, 0)` | `(18,52,86,255)` · [화면](chrome-light-html-stylesheet.png) |
| html transparent, body stylesheet `rgb(30,140,70)` | light | `rgba(0, 0, 0, 0)` | `rgb(30, 140, 70)` | `(30,140,70,255)` · [화면](chrome-light-body-stylesheet.png) |
| html stylesheet `rgb(18,52,86)`, body stylesheet `rgb(171,205,239)` | light | `rgb(18, 52, 86)` | `rgb(171, 205, 239)` | `(18,52,86,255)` · [화면](chrome-light-both-stylesheets-empty-body.png) |
| html transparent, body `var(--canvas-test-color)` | light | `rgba(0, 0, 0, 0)` | `rgb(18, 52, 86)` | `(18,52,86,255)` · [화면](chrome-light-body-var.png) |
| html transparent, body `hsl(120 100% 25%)` | light | `rgba(0, 0, 0, 0)` | `rgb(0, 128, 0)` | `(0,128,0,255)` · [화면](chrome-light-body-hsl.png) |
| html transparent, body `hwb(240 20% 10%)` | light | `rgba(0, 0, 0, 0)` | `rgb(51, 51, 230)` | `(51,51,229,255)` · [화면](chrome-light-body-hwb.png) |
| html transparent, body `color(srgb 0.2 0.2 0.9)` | light | `rgba(0, 0, 0, 0)` | `color(srgb 0.2 0.2 0.9)` | `(51,51,229,255)` · [화면](chrome-light-body-fractional-srgb.png) |
| html transparent, body color는 prefers media에서 선택 | light | `rgba(0, 0, 0, 0)` | `rgb(210, 220, 230)` | `(210,220,230,255)` · [화면](chrome-light-body-media.png) |
| html transparent, body color는 prefers media에서 선택 | dark | `rgba(0, 0, 0, 0)` | `rgb(12, 34, 56)` | `(12,34,56,255)` · [화면](chrome-dark-body-media.png) |
| html transparent, body `rgba(200,100,50,.5)` | light | `rgba(0, 0, 0, 0)` | `rgba(200, 100, 50, 0.5)` | `(227,177,152,255)` · [화면](chrome-light-body-alpha.png) |

## 판정과 경계

- opaque `html` 배경은 빈 body 문서의 canvas 색을 결정했다.
- 투명 `html`에서 opaque `body` 배경은 canvas 전체로 전파됐다.
- inline style, author stylesheet, `var()` 및 `prefers-color-scheme`에서 계산된 sRGB body 배경은 모두 CSSOM computed value에 반영됐고 테스트한 RGBA8 pixel과 일치했다.
- HSL 표본은 CSSOM과 pixel이 일치했지만 다른 HSL values 전체를 고정하지 않았으므로 구현 범위에서 제외한다. HWB 및 fractional sRGB 표본은 CSSOM의 파란 채널 `230`과 final screenshot pixel `229`가 달랐다. 이 값들은 구현에서 성공 처리하지 않는다.
- 두 배경이 투명하면 computed `color-scheme: light dark` 아래 light `(255,255,255)` / dark `(18,18,18)`가 선택됐다.
- body의 `prefers-color-scheme` CSS가 선택한 색도 투명 root 위의 canvas로 전파됐다. OS 선호는 기존 media cascade 입력과 그대로 분리된다.
- 부분 alpha body 색은 Chrome에서 `(227,177,152)`로 합성됐다. 이번 slice는 alpha compositing을 구현하지 않으며 해당 값의 구현 지원을 주장하지 않는다.
- root opaque + body opaque 케이스는 빈 body로 canvas만 확인했다. body content/box가 root 위에서 따로 paint되는 HTML 화면 전체 결과는 이번 비교나 구현의 판정 대상이 아니다.
- 이 절 앞부분은 Chrome oracle 기록이며, 아래 시뮬레이터 검증은 별도 앱 실행 근거다.

## 앱 시뮬레이터 확인

기본 앱의 [`index.html`](../../../../examples/bootstrap/index.html)은 `body`에 `#1e8c46`를 지정한다. source 파일 SHA-256은 `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`이며, 두 플랫폼 앱 번들에 복사된 파일과도 일치하는 것을 빌드 후 확인했다.

| 환경 | 빌드·실행 관찰 | 화면 표본 |
|---|---|---|
| Android API 37 ARM64 AVD `emulator-5554` | 빌드 성공. 기본 `dev.spinon.bootstrap` 실행에서 root HTML boot `status=pass`, 20 document nodes, Canvas `#1e8c46`; configuration change 뒤에도 같은 색과 최신 document revision을 반환했다. | [`1080×2400` 화면](android-api37-root-body-background.png), `(300,700)`, `(540,1200)`, `(540,800)`이 각각 `(30,140,70,255)`. 원본 로그: [빌드](android-build.log), [logcat](android-logcat.txt). |
| iPhone 17 Pro / iOS 26.2 Simulator `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D` | 빌드 성공. 기본 앱 실행에서 viewport `402×874` CSS px, DPR 3, root HTML boot `status=pass`, 20 document nodes, Canvas `#1e8c46`를 관찰했다. | [`1206×2622` 화면](ios-iphone17pro-root-body-background.png), `(300,700)`, `(603,1311)`, `(603,874)`이 각각 `(30,140,70,255)`. 원본 로그: [빌드](ios-build.log), [simulator](ios-simulator.log). |

화면 픽셀은 PNG의 RGBA8 표본이다. 두 앱은 물리 기기가 아니라 명시한 시뮬레이터에서 실행했다. 이 결과는 앱 content root 배경색 적용만 확인한다. HTML body box의 별도 paint, element/text rendering, system bar theme, 최종 compositor 색상 동등성, 실기기는 검증하지 않았다.

## Rust 검증

- `cargo test -p spinon-style -p spinon-runtime --locked root_html`: 통과.
- 전체 관련 크레이트 테스트: runtime 182 passed / 3 ignored, style 179 passed; runtime integration target은 3 passed 및 16 passed. 원본은 [cargo-test.log](cargo-test.log).
- `cargo clippy -p spinon-style -p spinon-runtime --locked --all-targets -- -D warnings` 통과. 최종 원본은 [cargo-clippy.log](cargo-clippy.log).
- iOS build와 cargo test의 끝 공백이 제거된 읽기용 로그를 제공한다. 원래 명령 stdout/stderr의 byte-for-byte 사본은 [iOS build 원본](ios-build.raw.log.gz)과 [cargo test 원본](cargo-test.raw.log.gz)에 보존했다.
