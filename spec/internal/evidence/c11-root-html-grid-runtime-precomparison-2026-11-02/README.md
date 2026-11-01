# C11 root HTML Grid runtime 사전 비교

이 기록은 Agent Browser 사전 비교와 후속 C11.1 runtime 후보 검증을 함께 보존한다. 제한 입력 계약은 [내부 계약 0059](../../0059-c11-grid-layout.md)에 추가했으며 숫자 버전 `0.1.0`은 유지한다. 화면 캡처·앱 실행은 제한된 fixture 입력만 증명하며 C11 또는 CSS Grid 전체의 완료 근거가 아니다.

## 조사 목적과 당시 결손

아래 결손 설명은 후보 구현 전 상태를 기록한다. 현재 작업 트리에서 확인한 구현 결과와 남은 일은 뒤의 `구현/runtime 후보 실행` 및 `범위와 상태` 절을 따른다.

현재 root HTML 기본 렌더 경로는 `RuntimeRootHtmlTextPaintV1` computed-style profile을 사용하고, visible tree에서 `display:block`과 `display:inline`만 받는다. 별도 C11.1 Grid 계산 경로 `RuntimeBlockFlexGridV1`과 C11.4 `RuntimeBlockFlexGridTrackSizingV1`은 각각 fixture runtime session profile에서 사용된다. 따라서 author `<style>`이 있는 parsed root HTML 문서가 실제 default runtime entry에서 CSS Grid를 계산하는 동작은 아직 연결되지 않았다. 이 계획은 fixed-ID fixture adapter를 더 늘리지 않고, root HTML author CSS를 기존 C11.1 typed Grid path에 전달하는 runtime 경계를 다룬다. 근거: `crates/spinon-runtime/src/session/ua_cascade/calculation.rs`, `runtime_layout.rs`, `runtime_layout/root_html_default_render.rs`, `session.rs`.

## 기준 입력 및 oracle

- 고정 HTML/CSS 입력: [`root-html-grid-author-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-author-precomparison.html)
- Browser: Agent Browser `0.31.1`, HeadlessChrome `156.0.0.0`, macOS host, viewport `1280×633` CSS px, DPR 1.
- 관측: [`chrome-observation.json`](./chrome-observation.json), 화면 [`chrome-156-root-html-grid.png`](./chrome-156-root-html-grid.png).
- 규범 산술 기대값: [`expected.json`](./expected.json). 두 explicit column은 40+80=120px, row는 70px이다. 기존 C11.1 입력은 Grid item의 fixed width·height를 요구하므로 각 item 크기를 지정 track과 같게 고정한다. Gap 선언은 생략해 initial 0px를 사용한다. Default row auto-placement에서 첫 item은 `(0,0,40,70)`, 다음 item은 `(40,0,80,70)`이다. Chrome 관측은 이 독립 산술의 교차 확인이다.
- Grid cell 경계를 넘는 item의 별도 입력은 [`root-html-grid-visible-overflow.html`](../../../../tests/fixtures/css/c11/root-html-grid-visible-overflow.html)이다. 독립 기대값은 [`visible-overflow-expected.json`](./visible-overflow-expected.json), Chrome frame·scroll overflow 관측은 [`chrome-visible-overflow-observation.json`](./chrome-visible-overflow-observation.json), 화면은 [`chrome-visible-overflow.png`](./chrome-visible-overflow.png)에 둔다. root는 120×70px로 유지되고 item은 track과 root의 경계를 넘는다.
- Fixture, 독립 기대값, observation 및 screenshot SHA-256은 [`SHA256SUMS.txt`](./SHA256SUMS.txt)에 고정한다.

## 구현 전 확인한 결손

1. Root HTML session의 default render는 `RuntimeRootHtmlDefaultRender` calculation profile로 연결된다.
2. 이 profile은 현재 `RuntimeRootHtmlTextPaintV1` computed-style profile을 선택한다.
3. root HTML default renderer는 block/inline display만 받아 `grid`를 거부한다.
4. 기존 C11.4 track-sizing computed-style profile과 runtime session constructor는 별도의 Grid fixture profile로 노출돼 있다.

따라서 다음 구현은 root HTML stylesheet/cascade/environment identity를 보존하면서 일반 node tree를 C11.1 Grid layout input에 연결해야 한다. `NodeId`, class, scene id 또는 화면 캡처로 사례를 선택하는 adapter로 대체하지 않는다. 이 C11.1 scene은 authored `grid-row`/`grid-column`을 사용하지 않는다. 구현 경로를 추적하는 동안 초기 계획의 explicit line placement가 C11.2.1 범위를 포함할 수 있는 것을 발견해 제거한 뒤, fixture와 Chrome 사전 비교를 다시 수집했다.

## 구현/runtime 후보 실행

실제 `RuntimeSession::new_runtime_gpu_with_root_html` entry를 연결해 Root HTML 기본 `RuntimeRootHtmlTextPaintV1` profile에서 author `display:grid`, fixed pixel template tracks, 기본 row auto-placement, item paint와 Grid container paint를 같은 HostDocument/style/layout/render revision으로 전달했다. `NodeId`, id, class, root element tag에 따른 분기는 추가하지 않았다. 별도 alternate HTML은 `<main>`→`<section>`, class selector→id/attribute selector, 숨김 prefix와 HostNode ID가 달라진 조건에서도 같은 CSS 기하를 냈다. source-order 역전은 고정 item size를 유지하면서 item frame만 column 1/2로 바꿨다.

`c11_1_root_html_default_runtime_projects_author_grid_tracks_and_auto_places_items`와 `c11_1_root_html_default_runtime_rejects_out_of_scope_author_grid_inputs`는 현재 소스에서 2/2 통과했다([test source](../../../../crates/spinon-runtime/src/session/tests/root_html_grid.rs), [실행 로그](./root-html-runtime-current-source.log)). 전체 Root HTML regression `cargo test --locked -p spinon-runtime root_html_`도 17/17 통과했다. Rust positive는 viewport `1280×633`, root background `(0,0,120,70)`, 빨강 `(0,0,40,70)`, 초록 `(40,0,80,70)`과 source-order 역전을 검사한다. CSS `display:inline` Grid item은 Stylo의 blockification 뒤 같은 frame을 내는 positive control이다. `display:flex` item은 `unsupported_root_html_default_render_style`, nested Grid는 `cascade_failed`로 닫힌다. CSS Grid §6에 따라 `white-space:pre`를 포함해 whitespace-only text sequence는 item count에서 빠지고, non-whitespace 직접 텍스트는 computed white-space와 무관하게 거부한다. 별도 visible-overflow fixture에서 root frame은 `(0,0,120,70)`, 빨강 `(0,0,41,71)`, 초록 `(40,0,81,70)`이며 Chrome 156 `getBoundingClientRect()` 결과와 같다. item이 오른쪽 `x=121`, 아래쪽 `y=71`까지 그려지는 기본 visible overflow를 자르거나 root 크기에 흡수하지 않는다. negative controls는 nonzero gap, Grid box padding·margin·border, non-static `position`, non-visible `overflow`, explicit placement, non-default `grid-auto-flow`, `repeat()`, `1fr`, percentage track, named line, explicit track를 넘는 추가 item, item `min-width`, RTL direction, vertical writing mode, 직접 Grid text를 실패시키고 부분 scene을 거부한다. out-of-scope author declaration 중 일부는 root profile cascade gate의 `cascade_failed`, box edge·track projection은 `unsupported_computed_value`, Grid 직접 텍스트는 `unsupported_root_html_grid_text_item`으로 닫힌다.

기존 root HTML viewport·color-scheme·block/text/background 테스트 17/17이 통과했다. `spinon-style-to-layout` 전체 실행 로그는 [style-to-layout-full-current-source.log](./style-to-layout-full-current-source.log)에 보존했다. `spinon-style-to-layout` 전체는 276/277이며 C07.3 `ratio-calc-number.aspect-ratio` 직렬화의 기존 차이(`1.7777778 / 1` 대 `1.77778 / 1`) 하나가 남았다. 이 실패는 Grid/root HTML 변경과 무관하고 [상태 대장](../../../STATUS.md)에 이미 기록된 미해결 비교다. 변경 패키지 Clippy와 `cargo fmt --all -- --check`는 통과했다.

### Android API 37 ARM64 AVD — 최신 작업 트리 재실행

- 실행 대상: `emulator-5556`, API 37, ARM64; renderer log는 ANGLE/Vulkan 위의 SwiftShader software path를 표시한다.
- 앱 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU=1`; fixture-test feature는 0. 후보 APK의 `assets/index.html`과 `root-app.js` SHA-256은 각각 fixture와 probe에 일치한다. 두 APK는 [`platform-runtime-observation.json`](./platform-runtime-observation.json) 및 [`platform-visible-overflow-observation.json`](./platform-visible-overflow-observation.json)에 경로와 hash를 고정했다. source manifest는 앱 제품 빌드에 포함되는 변경 Rust 소스와 fixture/probe hash를 묶는다. `#[cfg(test)]` 전용 테스트 소스는 앱 빌드 입력에서 제외하고, 저장소 파일 hash 목록에 포함한다. `baseHead`는 `caaf10f187553b4c4a351d9b9c1e96eb53387e08`이다.
- 실행: 기본 `MainActivity` route, launch override 없음. V8 probe는 parsed `<main>`과 class `item-a`, `item-b`를 확인했다. Runtime log: `layout=ready`, `boxes=5`, CSS viewport `411.42856×841.52374`, DPR `2.625`, root height `70px`; draw/present는 `boxes=5`다.
- latest-source 캡처: [Android API 37 AVD](./android-api37-avd-grid-current-source.png), 실행 로그: [runtime](./android-api37-avd-runtime-current-source.log). 보존 APK: `build/spinon/artifacts/android-c11-root-html-grid-base-current-source.apk`.
- visible-overflow variant는 같은 기본 route에서 root `411.42856×70px`, `layout=ready`, `boxes=5` 및 draw/present 5 boxes를 기록했다. [overflow 화면](./android-api37-avd-visible-overflow-current-source.png), [overflow runtime log](./android-api37-avd-visible-overflow-current-source.log), APK: `build/spinon/artifacts/android-c11-root-html-grid-visible-overflow-current-source.apk`.

### iPhone 17 Pro / iOS 26.2 Simulator — 최신 작업 트리 재실행

- 실행 대상: `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`; viewport `402×778` CSS px, DPR `3`, drawable `1206×2334`.
- 앱 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU=1`; fixture-test feature는 0. Xcode Simulator build가 성공했다. 기본 app route에서 검증 fixture와 probe만 `.app` resource에 복사해 설치했으며, iOS executable은 [`build/spinon/artifacts/ios-c11-root-html-grid-current-source`](../../../../build/spinon/artifacts/ios-c11-root-html-grid-current-source)에 보존했다. 테스트 뒤 선택한 simulator 앱은 종료하고 기본 bootstrap assets를 복구·재설치했다.
- 실행: 기본 앱 route, launch argument 없음. V8 probe와 runtime은 `layout=ready`, `boxes=5`, root height `70px`을 보고했고 draw/present가 `boxes=5`를 제출했다.
- latest-source 캡처: [iOS 26.2 Simulator](./ios-26_2-simulator-grid-current-source.png), 실행 로그: [runtime](./ios-26_2-simulator-runtime-current-source.log).
- visible-overflow variant도 기본 route에서 CSS viewport `402×778`, DPR `3`, root `402×70px`, `layout=ready`, `boxes=5` 및 draw/present 5 boxes를 기록했다. [overflow 화면](./ios-26_2-simulator-visible-overflow-current-source.png), [overflow runtime log](./ios-26_2-simulator-visible-overflow-runtime-current-source.log).

두 앱에서 보이는 빨강·초록 면은 scene이 화면에 제출된 시각 대조다. Android PixelCopy나 iOS Metal drawable readback을 수행하지 않았고 iOS compositor 이후 픽셀도 확인하지 않았다. 개별 item 좌표의 독립 수치 검증은 Rust runtime test의 frame assertion이며, 앱 screenshot은 NodeId/frame protocol이 아니다. Android 실기기·hardware GPU, iOS 실기기, CSS overflow clipping/scroll, WPT 및 일반 CSS/Grid 지원은 미검증이다.

### 범위와 상태

- 구현/runtime adversarial review: [20개 확인](./implementation-runtime-review-20.md). PR 변경분 별도 검토: [30개 실패 관점](./implementation-pr-diff-review-30.md).
- Platform app/source/hash manifest: [platform-runtime-observation.json](./platform-runtime-observation.json). Hash 목록은 저장소 루트에서 `shasum -a 256 -c spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/SHA256SUMS.txt`로 확인한다.
- Visible-overflow app/source/hash manifest: [platform-visible-overflow-observation.json](./platform-visible-overflow-observation.json), candidate Rust/fixture hash list: [candidate-source.sha256](./candidate-source.sha256).
- Product/fixture FFI marker 검사와 기존 iOS demo 문자열 범위: [artifact-boundary.txt](./artifact-boundary.txt).
- 내부 계약 0059는 `0.1.0` 유지. C11.1/C11.4 체크와 공식 구현 완료 `72/213`은 바꾸지 않는다. 이 제한 통합만으로 C11 전체를 완료 처리하지 않는다.

## 남은 작업과 지원 한계

- PR diff에 대해 별도의 30개 실패 관점 검토를 기록하고 implementation PR을 만든 뒤 GitHub checks와 병합 상태를 확인한다. 현재 구현/runtime review 20개는 별도로 기록했다.
- C11 parent, C11.3 및 C11.4는 계속 미완료다. `spec/STATUS.md`는 이 후보만으로 변경하지 않으며 공식 완료 `72/213`을 유지한다.
- 검증은 fixed-pixel explicit columns, 기본 row auto-placement, 두 item과 visible overflow fixture에 한정한다. intrinsic text/image sizing, implicit tracks, authored placement, CSS overflow clipping/scroll, WPT 전체 적합성은 범위 밖이다.
- Android AVD는 ANGLE/Vulkan/SwiftShader software path다. Android hardware GPU·실기기, iOS 실기기·compositor 이후 pixel은 확인하지 않았다. 화면 캡처는 PixelCopy나 Metal drawable readback이 아니다.
- `spec/internal/0059-c11-grid-layout.md` 숫자 버전은 `0.1.0`으로 유지한다. 기존 viewport/color-scheme/media behavior를 새로 확장한 것이 아니다.

## 제외 범위

이 계획 한 건으로 전체 CSS Grid, 전체 C11.4, general arbitrary CSS support, `@media` 신규 지원, intrinsic text/image track sizing의 새 값, auto placement, named lines/areas, 전체 Spinon WPT 또는 100% CSS 적합성을 선언하지 않는다. 기존 root HTML media/color-scheme 경로의 값은 구현 전 precondition으로 검증하고 새 계약으로 넓히지 않는다.
