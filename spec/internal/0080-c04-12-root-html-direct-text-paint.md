# 0080 · C04.12 파싱 root HTML direct text paint

**계약 버전:** `0.1.0-draft` · 출시 전 숫자 버전 고정
**상태:** 제한된 opt-in fixture slice 구현 · C04.12/C15 미완료 · 공개 API 아님

## 목적과 경계

이 계약은 실제 HTML5 parser가 만든 하나의 root `HostDocument`에서 block element가 소유한 direct Text node를 기존 text layout·glyph raster·runtime WGPU paint 경로에 연결하는 내부 검증 slice를 정의한다. CSS text 입력은 Stylo computed style에서 확인하고, layout·glyph·paint source는 같은 HostDocument generation과 document/render-tree/style/environment revision을 가져야 한다.

경로는 `c04-12-root-html-text-paint-fixture-test` opt-in feature에서만 활성화된다. 기본 `RuntimeSession`, 제품 앱의 일반 HTML text rendering, 공개 DOM/CSS API, 일반 font loading, 전체 C15/C16 지원을 추가하지 않는다. C04.12 viewport·scheme metadata와 Canvas fallback을 재정의하지 않는다.

## 입력과 성공 profile

- viewport: `320×240 CSS px`, fixture device scale factor `1`, page scale `1`.
- 문서: HTML namespace의 root `html`, `body`, `main#stage`; `main`에는 visible `div#visible`, clipped `div#hidden`, hidden `div#not-rendered`가 이 순서로 있다. 요소 ID는 문서 전체에서 고유해야 한다.
- `#visible`과 `#hidden`은 각각 direct Text child `XX` 하나를 가진다. `#not-rendered`는 `display:none`이고 Text `HIDDEN`은 원문에 남지만 layout/paint에 들어가지 않는다.
- visible source 밖의 non-whitespace Text, visible subtree의 ASCII 공백 이외 Unicode whitespace, 중첩·분할 direct text, 다른 namespace/요소, 추가 visible box는 거부한다. HTML ASCII collapsible whitespace-only node는 `white-space:normal`에 따라 별도 줄 상자를 만들지 않는다.
- `html/body/main`은 고정 block frame `320×240`, `320×20`, `320×20` CSS px다. 두 text owner의 content width는 `15px`, height는 `auto`, margin/padding/border는 0, `box-sizing:content-box`다.
- 두 owner의 computed text style은 Ahem `10px`, weight `400`, normal style, `10px` line height, normal whitespace/word break, LTR horizontal writing, text color `#092170`이어야 한다. 첫 owner의 overflow 두 축은 `visible`, 둘째는 `hidden`이어야 한다. 다른 값은 fallback 없이 거부한다.
- Ahem bytes는 고정 fixture asset을 `TextLayoutEngine::new_without_system_fonts()`에 등록한다. 시스템 font discovery나 임의 `@font-face` 수명은 사용하지 않는다.

## 처리·revision 계약

1. `RuntimeSession::new_runtime_gpu_root_html_text_paint_fixture(source)`는 source를 기존 `ParsedRootHtml`로 한 번 파싱하고 같은 parsed document를 actor-owned HostDocument로 전달한다. 별도 parser, 합성 JS DOM, fixture용 HostDocument 복제는 없다.
2. runtime layout worker가 현재 HostDocument snapshot의 tree와 Stylo computed properties를 검증한 뒤 owner의 direct text node를 `NodeId`로 연결한다. 실패하면 scene을 부분 게시하지 않는다.
3. Taffy text layout은 `XX`의 available width `15px`, advance `20×10px`, owner frame `15×10px`을 계산한다. `LayoutTextSnapshot` source range는 동일 Text node의 UTF-16 `[0..1]`, `[1..2]`다.
4. Ahem raster glyph와 `RuntimeRenderText`는 같은 owner, font registry revision, layout source revision, `RuntimeRenderKey`를 유지한다. root HTML generation/document/render tree, style, environment revision 중 하나라도 일치하지 않으면 publish하지 않는다.
5. `DrawText`는 대응 owner `DrawBox` 직후 paint order에 들어간다. `overflow:visible`은 owner 경계를 넘어 glyph ink를 보이고, `overflow:hidden`은 glyph advance를 보존하면서 owner frame으로 clip한다.
6. native 표본은 CSS pixel의 정수 좌표 `(1,1)`, `(16,1)`, `(1,11)`, `(16,11)`, `(100,100)`의 중심을 surface 비율로 투영한다. 정수 DPR 1인 fixture에서도 실제 drawable 크기가 CSS viewport와 다를 수 있으므로 표면 크기 비율을 사용한다. 보고된 surface 경계 밖 표본은 거부한다.

예시:

```rust
let (session, _) = RuntimeSession::new_runtime_gpu_root_html_text_paint_fixture(html)?;
session.set_root_html_environment(320.0, 240.0, 1.0, media)?;
let result = session.wait_for_layout_snapshot(timeout);
assert_eq!(result.state, RuntimeLayoutState::Ready);
let scene = result.completed.unwrap().render_snapshot.unwrap();
assert_eq!(scene.texts().len(), 2);
```

## 결과와 실패

성공 scene은 text owner 2개, owner마다 glyph 2개, glyph와 box 사이의 source-order paint command를 가진다. fixed Ahem sample colors는 visible text `[9,33,112,255]`, clipped/outside background `[247,249,252,255]`다. Chrome fixture geometry 허용 오차는 `0.05 CSS px`, 고정 native RGBA 표본은 exact 비교다.

문서 구조·ID, direct text count/source, viewport, computed font/color/overflow, text source mapping, glyph raster, revision 또는 surface sample 검증이 실패하면 구체 failure code와 node/property/stage를 보고하고 전체 scene을 내보내지 않는다. 예: `invalid_root_html_text_paint_environment`, `root_html_text_direct_child_count`, `root_html_text_source_mismatch`, `unsupported_root_html_text_style`, `unsupported_root_html_text_color`, `root_html_text_revision_mismatch`, `root_html_text_utf16_source_mismatch`, `root_html_text_glyph_mapping_mismatch`, `root_html_text_scene_attach_failed`.

Android PixelCopy 또는 iOS Metal drawable readback 실패는 해당 실행을 실패로 남긴다. Android는 API 37 AVD `SurfaceView` 캡처이고 ANGLE/SwiftShader backend다. iOS는 iPhone 17 Pro/iOS 26.2 Simulator의 Metal drawable이며 compositor 제출 전 표본이다. 둘 다 실기기·hardware GPU·최종 compositor 픽셀을 증명하지 않는다.

## ABI와 수명

- Rust/C ABI와 native launch route는 전용 Cargo feature, Android build env/JNI/header define, iOS Xcode build setting/C define으로만 컴파일한다. 기본 archive에는 fixture 전용 심볼·Ahem payload가 없어야 한다.
- `spinon_runtime_gpu_host_new_root_html_text_paint_fixture`는 host를 만들고 `spinon_runtime_gpu_host_eval_root_html_text_paint_fixture`는 같은 host의 현재 scene을 평가한다. C 호출자는 output buffer의 capacity를 제공하고, host는 다른 호출·해제와 경합하지 않게 해야 한다. evaluation은 기다림이 가능한 background executor에서 호출한다.
- Android PixelCopy callback은 surface generation, 크기, renderer identity와 run ID를 확인하며, stale surface callback은 현재 실행 성공으로 세지 않는다. iOS readback은 Metal drawable generation과 표면 크기를 확인한다.

## 제외와 증거

중첩 inline, 여러 paragraph, arbitrary text/font/color CSS, general line breaking, selection·text hit geometry, scroll, dynamic font fallback, 일반 앱 HTML text rendering, C04.12 전체, C15/C16, WPT, 실기기, hardware GPU는 포함하지 않는다.

- [구현 전 Chrome 기준](./evidence/c04-12-root-html-text-paint-precomparison-2026-10-18/README.md)
- [구현 및 Android/iOS Simulator 실행·화면·hash](./evidence/c04-12-root-html-text-paint-2026-10-18/README.md)
- [구현 실패 관점 검토](./evidence/c04-12-root-html-text-paint-2026-10-18/implementation-review-20.md)
- [PR 변경 검토](./evidence/c04-12-root-html-text-paint-2026-10-18/pr-delta-review-30.md)
