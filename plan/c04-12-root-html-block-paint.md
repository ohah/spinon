# C04.12 · 파싱한 루트 HTML의 제한 Block scene

- **문서 유형:** 구현 계획 · 공식 상태는 [`spec/STATUS.md`](../spec/STATUS.md)에서 관리
- **상위 항목:** [C04.12 앱 루트 HTML과 문서 환경 metadata](c04-root-html-document-environment.md), [C08 첫 화면 Block·기본 페인트](c08-first-screen-block-paint.md)
- **현재 상태:** 제한 구현, Android API 37 AVD·iOS 26.2 Simulator 검증, 구현 검토와 별도 PR diff 검토를 마쳤다. PR diff 검토에서 발견한 Unicode whitespace 경계를 수정했고 core 1개·opt-in runtime 2개 집중 테스트가 통과했다. GitHub branch protection과 PR에는 required status check가 없다. PR 병합이 남아 있다. `RuntimeSession::new_with_root_html`이 만드는 같은 HTML5 root document tree를 제한 Block cascade/layout/scene 경로에 연결한다. 일반 HTML 표시, 글리프, 전체 Bootstrap `index.html` 렌더링은 포함하지 않는다.
- **구현 전 Chromium 기준:** [Agent Browser Chrome 155 비교와 캡처](../spec/internal/evidence/c04-12-root-html-block-paint-precomparison-2026-10-18/README.md)
- **계획 적대 검토:** [20개 서로 다른 실패 관점](../spec/internal/evidence/c04-12-root-html-block-paint-plan-review-20-2026-10-18.md)
- **구현 후 검토와 실행 근거:** [구현 실패 관점 20개](../spec/internal/evidence/c04-12-root-html-block-paint-implementation-review-20-2026-10-18.md) · [Android/iOS 및 Rust 검증](../spec/internal/evidence/c04-12-root-html-block-paint-2026-10-18/README.md)
- **PR 변경 검토:** [30개 서로 다른 실패 관점과 판정](../spec/internal/evidence/c04-12-root-html-block-paint-pr-diff-review-30-2026-10-18.md)

## 목표와 계약 경계

현재 C04.12는 source HTML을 `html5ever`로 파싱해 HostDocument와 V8 `document`로 초기화하고, viewport·scheme metadata를 cascade 입력과 content-root Canvas background에 연결한다. Root HTML 계산에는 paint가 비활성화되어 있고, C08.1 Block paint는 별도 C08 profile에서만 켜진다. 따라서 실제 `<html>/<head>/<body>` tree가 layout이나 scene에 들어가지는 않는다.

구현은 별도 internal `RootHtmlBlockPaint` profile을 추가해 고정된 root HTML 입력을 Stylo cascade → Taffy Block → `RuntimeRenderSnapshot`까지 잇는다. profile은 같은 HostDocument snapshot, CSS viewport, media environment와 style revision을 소비한다. HTML을 다시 파싱하거나 JS DOM에서 synthetic render tree를 만들지 않는다. C04.12의 metadata·canvas 경로와 현재 JavaScript-only `RuntimeSession::new()`는 유지한다. 이 fixture 단계에서는 기본 앱의 모든 HTML 내용을 화면에 표시한다고 주장하지 않으며, 지원되지 않는 콘텐츠가 있는 실제 기본 HTML은 기존 C04.12 fallback만 사용한다.

공식 상태에 하위 계획을 추가하되 구현 전까지 미완료로 둔다. 제품 API, C ABI, 계약 숫자 버전은 추가하지 않는다. Android/iOS 검증 adapter와 FFI symbol은 `c04-12-root-html-block-paint-fixture-test` 전용 feature/build flag에서만 활성화한다. 제품 runtime API가 아닌 opt-in native fixture로 실제 parser input의 WGPU scene을 검증한다.

## 비교 입력과 성공 기준

- 고정 입력은 [사전 비교 fixture](../tests/fixtures/css/c04/root-html-block-paint.html)다. HTML document mode, `meta viewport`, `meta color-scheme`, `<head><style>`, `<main>`, `<div>`를 포함한다. body 하위 visible node는 빈 block이며 텍스트·이미지·폼 control은 없다.
- 기준은 Headless Chrome `155.0.8059.85`다. Agent Browser에서 CSS `320×240`, DPR 1, light/dark media preference를 고정해 computed style·각 node rectangle·캡처 픽셀을 수집한다. 입력, 환경, expected geometry, RGBA 표본과 hash는 [기준 README](../spec/internal/evidence/c04-12-root-html-block-paint-precomparison-2026-10-18/README.md)에 고정했다.
- 지원 성공 경로는 동일한 parsed document revision을 Stylo, Taffy, renderer가 공유하고 `html` root 아래 `head` 비표시와 `body`/`main`/`div`의 block-flow 및 배경을 계산하는 것이다. `display:none` 조상 아래 descendant는 자신의 computed display와 무관하게 layout·paint에서 빠진다. `<style>`의 scheme media 결과는 현재 root HTML environment revision에 맞아야 한다.
- computed display/background와 각 node의 x/y/width/height는 Chrome 값을 따로 비교한다. geometry 허용치는 최대 절대 오차 0.5 CSS px이다. CSS px frame과 device-pixel surface는 DPR로만 연결하며 page-scale은 이 slice에서 적용하지 않는다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 같은 HTML source를 실제 V8·Stylo·Taffy·WGPU 경로로 제출한다. light/dark 각각 layout key, source node IDs, scene order, 표면 표본 및 앱 화면을 보존한다. Android AVD의 WGPU surface는 `COPY_SRC`를 광고하지 않으므로 실제 `SurfaceView` 결과를 PixelCopy로 읽고 Rust가 계산한 surface 좌표와 대조한다. iOS는 Metal drawable을 직접 읽으며 compositor 제출 전 결과로 기록한다. 화면만으로 계산값을 판정하지 않는다.

## 지원 범위와 실패 처리

- HTML5 parser 결과의 `<html>` root를 계산 root로 사용한다. `<head>`, metadata, title/style text가 visible node나 box로 나타나지 않아야 한다. stylesheet source는 이미 연결된 author stylesheet collector를 따른다.
- parser는 기존 `html5ever` document-mode 경로와 source/node/depth/attribute budget, `NoQuirks` 검증을 그대로 사용한다. HTML을 별도 parser나 덜 제한된 변환 경로로 우회하지 않는다.
- 이 slice의 visible layout node는 HTML namespace의 Block `html`, `body`, `main`, `div`로 제한한다. `block`/`none`, fixture에 고정한 px width/height, zero margin, transparent 또는 완전 불투명 sRGB `background-color`만 성공 경로다. root `color-scheme`/metadata는 기존 C04.12 Canvas 결정을 바꾸지 않으며, `prefers-color-scheme`은 동일 environment input의 stylesheet rule 선택에만 사용한다.
- visible text node, inline/table/flex/grid display, unsupported visible element 또는 non-HTML namespace, inline formatting, content-based intrinsic sizing, nonzero margin collapse, partial alpha/unsupported color space, malformed stylesheet diagnostic, duplicate/unknown rendering root, stale style/environment revision은 부분 장면으로 대체하지 않고 실패한다. hidden subtree의 text는 실제 `display:none` 경계가 증명될 때만 paint 입력에서 제외된다.
- 실패한 scene은 직전 revision의 scene을 현재 revision처럼 노출하지 않는다. root HTML Canvas background resolver는 scene layout 실패로 바뀌지 않으며 기존 결과·오류를 그대로 반환한다.

## 구현 순서

1. fixture와 Chromium 결과를 고정하고 코드 변경 전에 이 계획을 서로 다른 실패 관점 20개로 검토한다.
2. root HTML Block profile의 property whitelist를 C08.1의 지원 subset과 대조한다. `color-scheme`/media environment와 Block paint의 computed style을 같은 revision에서 보존하고 unsupported property가 새 profile에 우회로로 들어오지 않는지 확인한다.
3. root document의 단일 `<html>` root, `<head>` 숨김, `<body>`와 허용 자식, visible/hidden text 및 namespace 경계를 검증한다. HostDocument generation/revision, CSS viewport와 media environment가 profile 전반에 보존되어야 한다.
4. layout/scene snapshot을 연결하고 오류·stale revision·empty/hidden tree를 기존 상태와 구분한다. RootHtml Canvas API와 기본 JavaScript-only session의 회귀를 확인한다.
5. pinned Chrome expected style/frame/pixel에 대한 고정 reference test와 HTML parser→runtime→scene 실패 경로 테스트를 추가한다. 기존 C08 inline/stylesheet cascade 회귀도 유지한다. 기본/opt-in FFI artifact 경계도 검증한다.
6. Android API 37 AVD와 iOS 26.2 Simulator에서 실제 parsed HTML fixture를 WGPU까지 구동하고 light/dark 캡처, 플랫폼별 표면 readback, source hash, 로그를 기록한다. Android PixelCopy와 iOS Metal drawable readback의 경계를 구분하고 simulator를 실기기/hardware-GPU proof로 표현하지 않는다.
7. 구현 뒤 새 실패 관점 20개로 코드·runtime·native 경로를 별도 검토한다. 결함 수정 후 영향받은 oracle·테스트·simulator 확인을 반복한다. 그 검토 표는 이 계획의 20개 표와 섞지 않는다.
8. 내부 계약 0077, 공식 STATUS, evidence와 PR을 같은 변경에서 갱신하고 Tailnet preview를 SHA로 대조한다. PR은 한국어 본문·라벨·가능한 Android/iOS 화면을 포함하며 병합 전에 별도의 PR diff 적대 검토 30개를 수행한다.

## 완료 판정

- Chrome reference의 supported computed values, frame geometry, hidden subtree, light/dark media pixels가 고정 기준에 맞는다.
- Android/iOS Simulator의 실제 parsed HTML source가 같은 CSS-space geometry와 scene order를 만든다. Android `SurfaceView` PixelCopy와 iOS Metal drawable readback이 각각의 platform scale에서 기대 RGBA를 만족하고, Android 표면 좌표는 Rust projection 결과와 일치한다.
- 오류나 stale result는 부분 scene으로 나타나지 않고, Canvas fallback과 JavaScript-only session 회귀가 없다.
- 성공·실패 범위, 내부 계약, precomparison, simulator source/evidence, 구현 실패 관점 검토가 연결된다. 이 하위 slice가 구현돼도 C04.12 전체, C08 전체, text rendering, default Bootstrap page의 완전한 화면 표시를 완료로 표시하지 않는다.
