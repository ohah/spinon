# 0081 · C04.12 기본 root HTML runtime 렌더링

**계약 버전:** `0.1.0-draft` · 출시 전 숫자 버전 고정<br>
**상태:** 일반 root HTML GPU runtime의 제한 Block/inline-text slice 구현 중 · C04.12/C15/C16/C19 미완료 · 공개 API 아님

## 목적과 경계

`RuntimeSession::new_runtime_gpu_with_root_html(source)`와 `spinon_runtime_gpu_host_new_with_root_html()`은 앱이 전달한 HTML bytes를 기존 HTML5 parser로 한 번 파싱하고, 그 document를 V8 session의 `HostDocument`, Stylo cascade, Taffy layout, Parley shaping/raster와 WGPU `RuntimeRenderSnapshot`에 연결한다. 별도의 fixture DOM이나 tag/id/text를 선택하는 paint route를 만들지 않는다. 앱 예제의 C04.12 launch argument는 같은 일반 host 생성자의 데모 진입점이다.

이 계약은 첫 제품 경로의 지원 subset과 실패 원자성을 고정한다. 일반 browser compatibility, 전체 C04.12 문서 환경, C15/C16 텍스트, C19 색/painter 지원 완료를 뜻하지 않는다.

## 입력과 환경

- C ABI 입력은 NUL 종료가 필요 없는 UTF-8 bytes이며 최대 길이는 1,048,576 bytes다. malformed UTF-8, 빈/과대 입력, HTML parse/session 생성 실패는 host 생성 실패로 반환한다. source는 호출 중 읽히며 parsed document가 입력 buffer 수명에 의존하지 않는다.
- root element는 HTML namespace의 `html`이어야 하며 HostDocument의 generation/document/render-tree revision이 cascade, layout, raster, scene key와 일치해야 한다.
- viewport와 device scale factor는 runtime environment API로 전달한다. Android/iOS 데모는 `320×240 CSS px`를 사용하고, OS light/dark scheme 변경 때 환경 revision과 계산 scene을 갱신한다. 캡처한 화면은 meta viewport, meta color-scheme, CSS `color-scheme` 및 `prefers-color-scheme` cascade를 같은 parsed source에서 확인한다.
- 화면 text는 Parley/Fontique Android system-font 또는 Apple CoreText backend를 사용한다. 테스트 fixture의 Ahem/Noto payload를 기본 artifact로 복사하지 않는다. 실제 선택 family와 glyph metrics는 대상 OS/font database에 따라 달라진다.

## 지원 subset

- visible namespace는 HTML만 허용한다. visible `display:block` element는 기존 Block/paint profile을 사용한다. `display:none` subtree는 layout/paint에서 제외한다. visible inline element allowlist는 `span`, `strong`, `b`, `em`, `i`다.
- block-owned direct text와 nested inline run은 실제 HostDocument source order, UTF-16 source ranges, computed style, Taffy available width, Parley shaping과 glyph raster snapshot을 따른다.
- computed `font-family`, positive `font-size`, numeric `font-weight`, `font-style: normal|italic|oblique`, `line-height: normal|<positive px>`를 사용한다. `normal` line height는 선택된 font metrics를 사용한다.
- paragraph flow subset은 `white-space` normal/nowrap/pre-line에 해당하는 computed collapse/wrap 조합, `word-break: normal|break-all`, `direction: ltr|rtl`, `line-break: auto|normal|strict`, `text-align:start`, `overflow-wrap:normal`, `text-transform:none`, `writing-mode:horizontal-tb`, zero/normal letter-spacing·word-spacing·text-indent다. `line-break:auto`는 현재 Parley adapter가 제공하는 고정 strict 정책으로 매핑한다.
- text color는 gamut mapping 후 encoded 8-bit sRGB에 정확히 놓여야 한다. display, text style/color, layout frame, font lookup, glyph raster, source mapping 또는 revision 검증이 실패하면 부분 render scene을 게시하지 않는다.

## 실패 경계와 비지원

visible foreign namespace·form/control/media/canvas/SVG 요소, inline allowlist 밖의 visible inline tag, unsupported display/property/color, 비유한 frame, source-range/glyph 불일치는 구체 node/property/stage 오류로 거부한다. 넓은 text 지원을 위해 값을 조용히 바꾸지 않는다.

이 slice는 JavaScript/script 실행, CSSOM, 외부 stylesheet·font fetch, `@font-face`, 임의 font 다운로드, 모든 Unicode fallback, 모든 line breaking/line-height 표현식, text selection/caret/accessibility, inline decoration·ruby·vertical writing, full Block/Flex/Grid 조합, wide-gamut paint, pinch/scroll input mapping, full WPT/Chromium conformance를 제공하지 않는다. Android API 37 AVD/SwiftShader와 iOS 26.2 Simulator Metal drawable 결과는 실기기·hardware GPU·iOS compositor 제출 이후 픽셀 증거가 아니다.

## 실행 근거

[Android/iOS Simulator light/dark 화면, 실행 log, build 환경과 hash](./evidence/c04-12-root-html-default-rendering-2026-10-19/README.md) · [제품 기본/fixture-test artifact 경계](./evidence/c04-12-root-html-default-rendering-2026-10-19/artifact-boundary.txt) · [Chrome 155 사전 기준](./evidence/c04-12-root-html-default-rendering-precomparison-2026-10-18/README.md) · [구현 계획](../../plan/c04-12-root-html-default-rendering.md)

## 기본 앱 launch와 Canvas clear

- 인수 없는 Android/iOS 기본 launch는 `examples/bootstrap/index.html`과 `root-app.js`를 같은 root HTML GPU host에서 사용한다. script의 `data-runtime=ready`, 최초 environment/layout/render 성공 뒤 scene을 게시한다. HTML bytes나 script marker가 없는 부분 화면은 성공 장면으로 보고하지 않는다.
- 기본 host는 `set_root_html_environment()`의 반환 environment snapshot을 `resolve_root_html_canvas_for_environment()`에 넘겨 같은 environment revision의 computed `html`/`body` background 또는 scheme fallback을 구한다. 이 경로는 environment revision을 다시 증가시키지 않는다. root canvas 선택 실패는 새 scene을 게시하지 않고 environment operation 오류로 끝난다.
- `RuntimeGpuHost::draw()`는 선택한 불투명 sRGB RGBA8을 `RuntimeRenderer::draw_if_with_srgb_clear_color()`에 전달한다. `Unorm` surface에는 encoded sRGB channel을, sRGB format에는 대응 linear channel을 전달한다. no-scene·normal paint·opacity compositor 경로가 같은 clear color를 공유한다. 다른 GPU host는 기존 opaque black clear를 유지한다.
- CSS body background가 root canvas로 전파되어 app content GPU surface 전체에 보인다. 이것은 body element box가 전체 viewport로 커진다는 뜻이 아니며 status/navigation UI palette나 safe-area CSS를 바꾸지 않는다. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator의 light/dark 캡처는 content-edge sRGB 표본이 Chrome 155 기준과 같음을 확인했다. compositor 이후 pixel은 검사하지 않았다. [기본 app launch 캡처·로그·archive 대조](./evidence/c04-12-default-app-launch-2026-10-19/README.md) · [구현 계획](../../plan/c04-12-default-app-launch.md)
