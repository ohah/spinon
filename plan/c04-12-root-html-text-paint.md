# C04.12 · 파싱 루트 HTML direct text glyph paint

- **문서 유형:** 구현 계획 · 공식 상태는 [`spec/STATUS.md`](../spec/STATUS.md)에서 관리
- **상위:** [C04.12 앱 루트 HTML과 문서 환경 metadata](c04-root-html-document-environment.md) · [C15 인라인 텍스트 shaping과 줄 상자](c15-inline-text-layout.md) · [C08 첫 화면 Block·기본 페인트](c08-first-screen-block-paint.md)
- **상태:** Chrome 155 사전 기준, 계획 실패 관점 20개, opt-in runtime/FFI/native fixture 구현과 Android/iOS Simulator pixel readback을 마쳤다. 내부 계약 `0080`과 [구현 실패 관점 검토](../spec/internal/evidence/c04-12-root-html-text-paint-2026-10-18/implementation-review-20.md), [PR 변경 검토](../spec/internal/evidence/c04-12-root-html-text-paint-2026-10-18/pr-delta-review-30.md)를 갱신했다. 구현 PR #317과 re-export 보완 PR #318이 2026-10-18 Rebase and merge로 병합됐다. 이 문서는 제한된 opt-in slice를 기록하며 C04.12/C15 전체 구현이 아니다.
- **기준:** [Chrome 155 관찰·geometry·pixel 표본](../spec/internal/evidence/c04-12-root-html-text-paint-precomparison-2026-10-18/README.md)
- **내부 계약:** [0080](../spec/internal/0080-c04-12-root-html-direct-text-paint.md), 숫자 버전 `0.1.0`.

## 목표와 현재 경계

`RuntimeSession::new_with_root_html()`이 HTML5 document mode로 읽은 **같은** root document와 actor-owned `HostDocumentSnapshot`에서 HTML `div`가 소유한 direct text node 하나를 찾아, 기존 C15 text layout·glyph raster·runtime paint 경로에 전달한다. 화면의 text source, `NodeId`, UTF-16 source range, layout/paint revision은 이 파싱 문서와 일치해야 한다. 새 parser, JS로 합성한 DOM, 별도로 만든 fixture `HostDocument`를 성공 경로에 사용하지 않는다.

이번 단계는 `c04-12-root-html-text-paint-fixture-test` opt-in profile에서만 동작하는 내부 검증 slice다. 기본 앱의 일반 HTML text rendering, 공개 DOM/CSS API, 전체 C15 완료를 뜻하지 않는다. C04.12의 `meta viewport`, `meta color-scheme`, CSS viewport·OS scheme metadata와 Canvas fallback은 기존 경로를 그대로 사용한다. 이번 text oracle은 color-scheme 의미를 재검증하지 않는다.

## 고정 입력과 성공 subset

Chrome oracle fixture는 [root-html-text-paint.html](../tests/fixtures/css/c04/root-html-text-paint.html), runtime input은 [root-html-text-paint-runtime.html](../tests/fixtures/css/c04/root-html-text-paint-runtime.html)이다. 두 입력은 같은 DOM·CSS profile이며 runtime copy에서만 Chrome이 Ahem을 읽기 위한 `@font-face` resource loader를 제거한다. Android/iOS opt-in harness가 기존 고정 Ahem asset을 직접 등록한다. 이 차이를 CSS `@font-face` 지원으로 해석하지 않는다. 고정 Chrome oracle와 두 fixture/Ahem hash는 [사전 비교 기록](../spec/internal/evidence/c04-12-root-html-text-paint-precomparison-2026-10-18/README.md)에 둔다.

성공 입력은 viewport `320×240 CSS px`, DPR `1`, HTML namespace의 단일 `html/body/main` Block 뼈대다. `main`에는 visible `div` 두 개가 있고 각 `div`에는 non-empty direct Text child `XX`가 정확히 하나 있다. HTML source에 보존된 block 사이의 collapsible whitespace-only Text node는 `white-space:normal` 규칙에 따라 anonymous paragraph나 line box를 만들지 않아야 한다. 빈 Text node도 glyph나 line box를 만들지 않는다. 그 외 visible non-whitespace text는 허용하지 않는다. 첫 `div`의 computed overflow 두 축은 `visible`, 둘째는 둘 다 `hidden`이어야 한다. 두 요소의 content width는 `15px`, height는 `auto`, margin/padding/border는 `0`, `box-sizing`은 `content-box`, 글꼴은 등록된 Ahem `10px`, 내부 line-height는 `10px`, text color는 정확한 8-bit sRGB `#092170`이다. 분리되지 않는 단어 `XX`의 text advance는 `20×10px`, owner block은 `15×10px`다. `display:none`인 세 번째 `div`의 `HIDDEN` text는 source document에는 남되 layout·paint에서 빠져야 한다.

이 profile은 runtime HTML의 위 고정 CSS 값을 Stylo computed style에서 확인해 typed layout input으로 전달하고, opt-in font registry가 보유한 동일 Ahem bytes를 선택한다. 값이 다르면 fallback·system font 조회·기본값 대체 없이 node/property/stage를 붙여 거부한다. `font-family`는 fixture registry의 Ahem만, `font-size`·line-height·weight·style·white-space·word-break·overflow-wrap·hyphens·letter/word spacing·text indent/transform/alignment·direction·writing mode·text color·overflow axes·box sizing·border는 fixture 값 또는 해당 CSS initial value만 허용한다. typed text-style input에 표현되지 않은 text shaping·fragment origin·line breaking 관련 computed property가 non-initial 값이면 거부한다. 일반 font matching, CSS `@font-face` 수명, async load/fallback, 임의 line-height와 전체 text CSS cascade는 포함하지 않는다.

## Chrome 기준과 판정값

- Agent Browser가 실행한 Chrome `155.0.8059.85`, macOS `26.5.1` ARM64, `320×240 CSS px`, DPR `1`에서 Ahem load 완료 뒤 DOM geometry와 화면을 관찰했다. 브라우저의 user-agent high-entropy version, viewport, computed style과 screenshot sample은 JSON에 보존했다. Chrome 실행 파일 SHA-256과 CDP revision은 수집하지 못했으므로 이 oracle은 해당 환경의 geometry/pixel 비교 자료이며 재현 가능한 browser binary pin으로 과장하지 않는다.
- `#visible` border box는 `(0,0) 15×10px`; text Range는 `(0,0) 20×10px`다. `#hidden` border box는 `(0,10) 15×10px`; text Range는 `(0,10) 20×10px`다. `overflow:hidden`은 text advance를 줄이지 않고 실제 paint만 owner clip으로 자른다.
- screenshot 표본 `(1,1)`, `(16,1)`, `(1,11)`은 RGBA `[9,33,112,255]`; `(16,11)`과 `(100,100)`은 `[247,249,252,255]`다. visible overflow의 두 번째 glyph와 hidden overflow의 바깥 배경을 각각 negative/positive boundary로 확인한다.
- `#not-rendered` computed `display:none`의 border box는 `0×0`, Text Range는 비어 있다. 이 노드의 원문 `HIDDEN`은 source document에 남지만 layout/paint 결과에서는 사라져야 한다.

Geometry는 CSS px에서 `0.05px` 이내로 비교한다. 정수 DPR 1의 고정 색 표본은 RGBA exact로 판정한다. 전체 글리프 screenshot의 antialias 유사도나 OS 색관리 parity는 oracle로 삼지 않는다.

## Runtime·revision·실패 계약

- `LayoutInput`/`LayoutOutput`의 `LayoutInputRevision`은 HostDocument generation/document/render-tree, style, environment revision을 결속하고, `LayoutTextSnapshot.revision`이 이를 그대로 가져야 한다. 내부 `TextLayoutSnapshot`은 font-registry revision, owner `NodeId`, available width, 실제 HTML text source와 UTF-16 range를 보존한다. glyph raster는 같은 layout/font revision과 glyph/source를 검증한다. `RuntimeRenderText`의 owner와 `RuntimeRenderKey`를 scene에 넣기 전에 bridge가 이 provenance를 현재 session revision과 대조하며, raw `TextLayoutSnapshot`이나 `RuntimeRenderText` 단독이 모든 revision을 가진다고 가정하지 않는다.
- `overflow:visible`에서는 glyph ink가 15px owner 바깥 5px까지 보인다. `overflow:hidden`에서는 같은 glyph geometry를 유지하면서 owner의 effective paint clip과 교차한다. clip은 `display:none` 상태나 page viewport clip과 혼동하지 않는다.
- visible inline span, 중첩/분할 non-empty text, 두 개 이상의 visible paragraph, mixed block/inline flow, `white-space:pre|nowrap`, 비정상 `word-break`/`overflow-wrap`, non-zero letter-spacing/text-indent, unsupported namespace/element, unsupported text style/font/color, text-own scroll, stale layout/font/style revision은 정확한 diagnostic으로 전체 scene을 거부한다. collapsible whitespace-only/empty Text는 위 성공 입력처럼 glyph나 별도 line box를 만들지 않아야 한다. 부분 scene, 이전 revision, 잘못된 색 또는 다른 폰트로 나타나지 않는다.
- `head`/`style`/`title`의 비표시 text와 `display:none` subtree가 visible glyph나 layout height를 만들지 않는 negative control을 둔다. parser source/node/depth/attribute budget과 `NoQuirks` 검사는 기존 경로를 유지한다.
- 실제 Android/iOS native fixture route도 전용 Cargo feature, Android build flag/JNI/header define, iOS build setting/C header define에서만 켠다. 기본 archive에서는 고유 FFI symbol·Ahem payload가 없어야 하고 opt-in positive control에는 있어야 한다.

## 구현 전·후 실행 기준

1. fixture, Chrome observation JSON, screenshot, Ahem SHA-256을 고정하고 이 계획을 구현 코드와 분리해 서로 다른 실패 관점 20개로 검토한다. 계획 PR을 Rebase and merge하기 전에는 Rust/native 제품 코드를 변경하지 않는다.
2. 구현 전에 existing `RootHtmlBlockPaint`와 C15.3 `HostDocument text layout→runtime glyph paint`의 ownership, style/profile gate, source mapping, renderer clip 및 FFI feature 경계를 코드와 잠금 dependency API에서 다시 확인한다. 중복 parser/layout/raster 경로는 추가하지 않는다.
3. root HTML fixture profile, checked CSS text values, actual HostDocument direct-text extraction, fixed-font layout, same-snapshot glyph paint를 연결한다. main/product default profile은 그대로 fail-closed한다. empty/hidden/visible, visible/hidden overflow, stale revision, wrong font/property, fixture source mutation을 구별하는 Rust/FFI failure tests를 둔다.
4. Chrome fixture의 geometry와 exact RGBA sample에 연결된 runtime reference checks를 실행한다. 기존 C04.12 Block paint 및 C15.1/C15.3 layout·raster·clip 회귀도 확인한다. `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, fixture module/feature boundary 검사와 default/opt-in artifact scan을 기록한다.
5. Android API 37 ARM64 AVD에서는 실제 앱 `SurfaceView` PixelCopy를, iPhone 17 Pro/iOS 26.2 Simulator에서는 실제 Metal drawable readback을 사용해 같은 CSS-space geometry와 네 고정 sample 위치를 확인한다. Native sample pixel은 보고된 surface size·DPR/page-scale projection에서 계산하며 CSS 좌표를 physical pixel 좌표로 그대로 복사하지 않는다. Android AVD는 ANGLE/SwiftShader 여부를 기록하고 hardware GPU나 실기기 결과로 부르지 않는다. iOS Metal drawable은 compositor 제출 전으로 표기한다. 이 slice에서는 실기기를 요구하지 않는다.
6. 구현 뒤 이 계획 검토와 겹치지 않는 실패 관점 20개로 코드·runtime·native fixture를 검토하고 발견한 결함을 수정한 뒤 영향받은 oracle/test/simulator 경로를 재확인한다. 별도 PR diff 검토는 사용자의 요청에 맞춰 서로 다른 실패 관점 20개로 한다.
7. 내부 계약 `0080`, 공식 상태, 실행 근거·화면·hash, 한국어 PR 본문·영역/유형 label을 연결한다. Tailnet roadmap을 동기화하고 Agent Browser session과 로컬 서버를 종료한다. 병합은 `Rebase and merge`만 사용한다.

## 완료 판정과 제외

성공 판정에는 parsed HTML에서 나온 동일 Text node의 20×10px layout advance와 15×10px owner frame, visible/hidden clip 차이, display-none omission, revision-safe text layout→glyph paint, Chrome geometry/pixel sample, Android PixelCopy 및 iOS Metal drawable 표본, 기본/opt-in artifact 분리, 20개 구현 실패 관점 근거가 모두 필요하다. 실패 scene은 부분 draw를 내보내지 않아야 한다.

이 slice는 C04.12 전체, C15.3/C15, C16, 기본 앱 전체 HTML, arbitrary CSS font/color, 일반 line wrapping, nested inline, text selection/hit geometry, scroll, 실기기/hardware GPU, iOS compositor-after pixels, WPT/전체 Chromium 적합성을 완료하지 않는다. 공식 상태의 하위 구현 slice만 갱신하고 C04.12/C15 상위 항목은 미완료로 둔다.
