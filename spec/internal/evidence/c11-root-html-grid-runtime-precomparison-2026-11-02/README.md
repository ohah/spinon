# C11 root HTML Grid runtime 사전 비교

## 목적

현재 root HTML 기본 렌더 경로는 `RuntimeRootHtmlTextPaintV1` computed-style profile을 사용하고, visible tree에서 `display:block`과 `display:inline`만 받는다. 별도 C11.1 Grid 계산 경로 `RuntimeBlockFlexGridV1`과 C11.4 `RuntimeBlockFlexGridTrackSizingV1`은 각각 fixture runtime session profile에서 사용된다. 따라서 author `<style>`이 있는 parsed root HTML 문서가 실제 default runtime entry에서 CSS Grid를 계산하는 동작은 아직 연결되지 않았다. 이 계획은 fixed-ID fixture adapter를 더 늘리지 않고, root HTML author CSS를 기존 C11.1 typed Grid path에 전달하는 runtime 경계를 다룬다. 근거: `crates/spinon-runtime/src/session/ua_cascade/calculation.rs`, `runtime_layout.rs`, `runtime_layout/root_html_default_render.rs`, `session.rs`.

## 기준 입력 및 oracle

- 고정 HTML/CSS 입력: [`root-html-grid-author-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-author-precomparison.html)
- Browser: Agent Browser `0.31.1`, HeadlessChrome `156.0.0.0`, macOS host, viewport `1280×633` CSS px, DPR 1.
- 관측: [`chrome-observation.json`](./chrome-observation.json), 화면 [`chrome-156-root-html-grid.png`](./chrome-156-root-html-grid.png).
- 규범 산술 기대값: [`expected.json`](./expected.json). 두 explicit column은 40+80=120px, rows는 20+50=70px이며, definite numeric placement로 item A는 `(40,0,80,20)`, item B는 `(0,20,40,50)`이다. Chrome 관측은 이 독립 산술의 교차 확인이다.
- Fixture, 독립 기대값, observation 및 screenshot SHA-256은 [`SHA256SUMS.txt`](./SHA256SUMS.txt)에 고정한다.

## 구현 전 확인한 결손

1. Root HTML session의 default render는 `RuntimeRootHtmlDefaultRender` calculation profile로 연결된다.
2. 이 profile은 현재 `RuntimeRootHtmlTextPaintV1` computed-style profile을 선택한다.
3. root HTML default renderer는 block/inline display만 받아 `grid`를 거부한다.
4. 기존 C11.4 track-sizing computed-style profile과 runtime session constructor는 별도의 Grid fixture profile로 노출돼 있다.

따라서 다음 구현은 root HTML stylesheet/cascade/environment identity를 보존하면서 일반 node tree를 Grid layout input에 연결해야 한다. `NodeId`, class, scene id 또는 화면 캡처로 사례를 선택하는 adapter로 대체하지 않는다.

## 미확정 구현 계약

- 구현 전 추가 소스 추적에서 root HTML canvas color scheme, viewport/meta mapping, author media, cascade diagnostic과 `StyleLayoutOutput` revision이 C11 Grid profile과 함께 보존되는 경계를 코드·테스트로 확인한다.
- `display:grid`를 root HTML 입력에서 fail-closed하지 않고 기존 typed Grid projection/layout에 넘긴다. 지원하지 않는 Grid value와 tree shape은 명시된 오류로 거부하며 block fallback으로 렌더하지 않는다.
- 새 root HTML 경로는 opt-in fixture-only profile이 아닌 실제 root HTML runtime entry를 사용하고, 새 HostNode ID와 이 fixture의 DOM 구조 이외의 decoy 입력을 포함해 ID 기반 성공을 배제한다.
- 기존 root HTML block/text/background, viewport, media environment와 Grid fixture profile의 회귀를 따로 확인한다.
- Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 parsed HTML 입력의 root/item frame, used track, layout revision, draw/present를 검사한다. AVD hardware GPU, iOS compositor 이후 pixel 및 전체 browser parity는 이 장면으로 주장하지 않는다.
- `spec/internal/0059-c11-grid-layout.md` 숫자 버전 `0.1.0`을 유지한다. C11/C11.4 공식 checkbox와 `72/213`은 이 integration slice만으로 승격하지 않는다.

## 제외 범위

이 계획 한 건으로 전체 CSS Grid, 전체 C11.4, general arbitrary CSS support, `@media` 신규 지원, intrinsic text/image track sizing의 새 값, auto placement, named lines/areas, 전체 Spinon WPT 또는 100% CSS 적합성을 선언하지 않는다. 기존 root HTML media/color-scheme 경로의 값은 구현 전 precondition으로 검증하고 새 계약으로 넓히지 않는다.
