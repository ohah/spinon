# C04.12 · 기본 root HTML 문서 렌더링

- **문서 유형:** 구현 계획 · 공식 상태는 [`spec/STATUS.md`](../spec/STATUS.md)에서 관리
- **상위:** `plan/c04-root-html-document-environment.md` · `plan/c15-inline-text-layout.md` · [C19 색과 painter](../spec/STATUS.md#css-구현-체크리스트)
- **상태:** 계획 단계 · 구현 전 실패 관점 검토 완료. C04.12/C15/C16/C19의 완료를 주장하지 않는다.
- **사전 기준:** [Chrome 155 HTML·텍스트·색 scheme 관찰](../spec/internal/evidence/c04-12-root-html-default-rendering-precomparison-2026-10-18/README.md)
- **계획 검토:** [독립 실패 관점 20개](../spec/internal/evidence/c04-12-root-html-default-rendering-precomparison-2026-10-18/plan-review-20.md) · PR diff는 별도 [실패 관점 20개](../spec/internal/evidence/c04-12-root-html-default-rendering-precomparison-2026-10-18/pr-delta-review-20.md) 검토

## 목표

일반 생성자 `RuntimeSession::new_with_root_html()`가 파싱해 V8 문서로 사용하는 **같은 HostDocument**의 visible HTML 요소와 text node를 기본 앱 렌더 경로까지 전달한다. 고정 tag/id/text/font에 묶인 fixture 경로가 아니라, 같은 cascade·viewport·OS color scheme·document/style/environment revision을 따라 layout과 glyph paint를 계산한다. 문서의 `meta viewport`, `meta color-scheme`, CSS `color-scheme`, `prefers-color-scheme`, CSS text color와 system font 선택이 한 화면 결과에 연결되어야 한다.

이 계획은 일반 root HTML에 대한 단계별 제품 경로다. 첫 구현은 현재 C04/C15가 지원하는 Block과 inline text subset을 실제 기본 runtime으로 승격하고, 미지원 CSS는 해당 node/property를 식별해 fail-closed한다. 이 결과를 전체 HTML/CSS 지원으로 표시하지 않는다. C04.12 parent와 C15/C16/C19는 후속 범위가 남아 있으므로 미완료로 둔다.

## 현재 기본 경로와 결손

- `RuntimeSession::new_with_root_html()`은 HTML5 document mode parsing 뒤 `RuntimeCssProfile::RootHtmlDocument`를 사용한다.
- `RuntimeGpuHost::new()`는 `RuntimeSession::new_runtime_gpu()`로 시작한다. 기본 GPU host에서 root HTML source를 받는 생성자가 없고, Android/iOS의 현재 root HTML GPU launch는 fixture selector에만 연결되어 있다.
- 이 profile의 `RuntimeCalculationProfile::has_paint()`은 false다. 그러므로 일반 root HTML session 계산은 `RuntimeRenderSnapshot`/WGPU paint를 만들지 않는다.
- 루트 HTML Block paint와 direct-text glyph paint는 각각 opt-in fixture feature에 묶여 있다. direct-text 경로는 고정 HTML tree, Ahem 10px와 `XX`를 요구한다. 기본 앱의 일반 HTML text rendering 근거가 아니다.
- `spinon-text-layout`은 고정 fixture의 재현성을 위해 현재 `FontContext`의 system font discovery를 끈다. 잠금된 Parley 0.12.0 / Fontique 0.12.0의 Android와 Apple system backend를 실제 대상에서 확인하고, fixture용 no-system-font 생성자는 그대로 보존한다.
- C15 `HostDocument`/Taffy 경로는 실제 shaping, inline runs, anonymous block flow, glyph raster snapshot을 이미 제공하지만 기본 root HTML profile의 computed text style 및 scene assembly로 이어지지 않는다.

## 구현 순서

1. 일반 `RuntimeGpuHost`/C ABI에 source를 받는 parsed-root-HTML 생성 경로를 추가하고, actor runtime의 같은 parsed HostDocument와 같은 viewport/scheme/page-scale 입력으로 cascade/layout/render를 실행한다. UTF-8 source의 null/빈 입력·길이 제한·파싱 오류는 명시적으로 거부하고 HostDocument는 호출자 buffer의 lifetime에 의존하지 않아야 한다. Rust constructor만 추가하고 Android/iOS runner가 계속 fixture selector만 쓰는 상태를 완료로 부르지 않는다.
2. 잠금된 Parley/Fontique 소스와 현재 Cargo feature graph에서 Android system fonts XML backend와 Apple CoreText backend의 초기화·fallback 동작을 확인한다. CSS family list 순서와 generic family fallback을 OS collection에 연결하며, 같은 engine token 안에서는 resolved font registry가 불변이어야 한다. macOS host, Android API 37 ARM64 target, iOS Simulator target의 default/feature builds에서 feature가 올바른 대상 구현만 끌어오는지 검증한다. Linux CI와 고정 테스트는 계속 `new_without_system_fonts()`를 사용한다. 제품 글꼴을 위해 Ahem/Noto fixture payload를 재사용하거나 앱에 임의로 복사하지 않는다.
3. Stylo `ComputedStyleSnapshot`에서 실제 element cascade가 계산한 font family, size, weight, style, `line-height: normal`, white-space, word-break, direction, foreground color를 text source에 연결한다. text node의 style은 DOM 부모 상속을 사용한다. `normal` line-height는 선택된 font의 실제 metrics에서 결정하고 고정 배수로 추정하지 않는다. 이 slice의 텍스트 paint color는 정확히 표현 가능한 8-bit sRGB로 제한한다. family list 전체에서 쓸 수 있는 face가 없거나 profile이 지원하지 않는 다른 값은 표시 가능한 fallback으로 바꾸지 않고 구체 오류로 닫는다.
4. `HostDocumentSnapshot`의 source order와 숨김 상태에서 C15 paragraph/run을 만든다. 실제 Parley shaping과 기존 Taffy available-inline-size adapter만 사용한다. 글자 수 기반 width, hard-coded line box, fixture의 별도 host tree 생성은 허용하지 않는다. layout·raster·render snapshot은 document generation/revision, style revision, environment revision, viewport, font registry revision을 일치시킨다.
5. 같은 layout/raster snapshot의 glyph bitmap과 source range를 일반 `RuntimeRenderSnapshot`의 text paint로 전달한다. element/inline run별 color와 weight/style, 부모 overflow clip과 scroll/page-scale 변환을 paint/hit-test의 현재 공통 계약에 적용한다. Android/iOS의 일반 앱 launch path가 root HTML source와 live viewport/scheme을 이 표준 host에 전달해야 한다. OS scheme 변경은 environment/style revision을 갱신해 새 scene을 만들며 이전 scheme의 scene을 게시하지 않는다. 기본 `new_with_root_html` 호출은 fixture 전용 feature를 켜지 않아도 해당 subset을 그려야 한다.
6. 기존 Chrome 155 고정 입력과 새 `root-html-default-rendering.html`에서 light/dark, heading weight, Korean text, inline `strong`, 줄바꿈, 실제 text ink를 비교한다. Android API 37 ARM64 AVD는 앱 `SurfaceView` PixelCopy, iPhone 17 Pro/iOS 26.2 Simulator는 앱 Metal drawable readback을 사용한다. Chrome geometry의 허용 오차는 고정 입력별로 기록하고, glyph antialias 전체 pixel equality는 쓰지 않는다. 두 native 결과는 capture phase, 실제 font family, DPR/page-scale과 함께 보존한다.
7. default와 fixture-test Rust/native artifacts를 각각 비교한다. 제품 경로의 system font backend는 포함될 수 있지만 Ahem/Noto fixture payload, C04/C15 fixture constructors·symbols는 기본 artifact에서 제외되어야 한다. layout/style/renderer/render failure가 생기면 scene 전체를 게시하지 않는 원자성을 확인한다.
8. 구현 전 plan review 20개와 이 계획과 독립된 구현 review 20개를 유지한다. PR에는 사용자의 지시에 따라 서로 다른 diff 실패 관점 20개를 기록한다. 구현 slice의 STATUS, 내부 계약, Chrome·Simulator 원본 근거, 로드맵 미리보기·Tailnet 사본을 같은 변경 주기에서 갱신하고 `Rebase and merge` 한다.

## 기준 fixture와 통과 판정

Chrome oracle fixture는 `tests/fixtures/css/c04/root-html-default-rendering.html`이다. Agent Browser가 보고한 HeadlessChrome `155.0.0.0`, macOS ARM host, CSS viewport `320×240`, DPR 1에서 `main`은 `(0,0,320,240)`, heading은 `(16,16,288,30)`, paragraph는 `(16,58,288,40)`이다. heading text Range는 `124.859375×28px`, paragraph text Range는 `272.40625×38px`, inline `strong` box는 `(155.453125,58,27.6875,18)`다. 일반 폰트의 glyph geometry는 플랫폼과 font backend에 따라 다를 수 있으므로, implementation은 실제 resolved family와 각 플랫폼 기준값을 기록하고 fixture별 기준표를 갱신한다.

light 결과는 body `#263342`/`#ffffff`, main `#f8fafc`, heading `#12345a`; dark 결과는 body `#e4e7eb`/`#121212`, main `#1d2939`, heading `#93c5fd`다. metadata `light dark`, CSS root `color-scheme`, OS media preference가 서로 다른 입력에서도 선택된 scheme과 Canvas background의 우선순위가 기존 C04.12 계약을 보존해야 한다. [관찰 JSON·화면·해시](../spec/internal/evidence/c04-12-root-html-default-rendering-precomparison-2026-10-18/README.md)

통과 기준은 일반 생성자에서 동일 parsed HostDocument의 실제 문자가 화면에 나오고, computed style와 source mapping이 일치하며, light↔dark 전환과 viewport 재계산 뒤 stale layout/font snapshot을 사용하지 않는 것이다. Android와 iOS Simulator에서 화면 표본을 확인한다. 이 simulator 결과는 실기기, hardware GPU, iOS compositor 제출 뒤 픽셀 또는 전체 브라우저 적합성의 증거로 확대하지 않는다.

## 범위 밖

전체 HTML/CSS/DOM API, 외부 stylesheet/font resource loading, `@font-face`, arbitrary downloadable font, CSS `line-height`의 모든 계산식, full Unicode fallback matrix, text selection/caret/accessibility, inline decoration·vertical writing·ruby, 모든 Block/Flex/Grid 조합, 모든 wide-gamut text paint, full WPT/Chromium conformance는 이 first product route만으로 완료되지 않는다. 지원 속성의 범위는 내부 계약과 STATUS에서 명시하고, 미지원 값은 조용한 default로 바꾸지 않는다.
