# C11.4 · fit-content 옆 auto implicit row 앱 런타임 계획

**상위 계획:** [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [공식 상태 대장](../spec/STATUS.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 이 계획의 단일 Android/iOS opt-in app fixture 조각을 구현했다. 일반 CSS admission·Spinon WPT 적합성·전체 C11.4 완료를 선언하지 않는다. 공식 상태 checkbox와 `72/213`은 유지한다.

## 목표

Pinned WPT `css/css-grid/grid-definition/fit-content-track-sizing.html`의 assertion “Leftover space goes to the auto row rather than the fit-content() row beside it”를 기존 C11.4 opt-in CSS→HostDocument/Stylo→typed track input→Taffy→native app 경로로 실행한다. PR #662가 이미 고정한 normative row sizes는 `[10px, 290px]`이다. 새 계획은 같은 동작이 layout-core 입력만이 아니라 실제 앱 fixture adapter를 지나도 보존되는지 확인한다.

## 구현 전 oracle

[Chrome 및 독립 규범 비교](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27/README.md)와 [기대값 JSON](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27/expected.json)을 기준으로 한다.

- Pinned source: WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`; test file SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`.
- Agent Browser 0.31.1 / HeadlessChrome 156에서 upstream testharness 13/13 Pass, target rows `10px 290px`, grid height `300px`를 관찰했다. Live source hash는 pinned file과 일치한다. 이것은 upstream WPT를 Chrome으로 실행한 결과이며 Spinon의 WPT runner 결과가 아니다.
- CSS Grid 2025 CRD §6.6, §11.5, §11.8, §12.2, §12.4–§12.8의 independent derivation: 두 implicit rows의 content minimum contribution은 각각 10px이다. `fit-content(40px)`의 max track sizing function은 max-content이므로 40px argument가 track을 40px로 늘리거나 stretch 대상이 되지 않는다. `align-content: normal`의 양의 남은 공간 280px은 auto max track sizing function에 적용되어 최종 rows는 `[10, 290]px`다.

## 고정 runtime 입력

새로운 allowlisted scene 하나를 추가한다. 새 module은 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_auto_neighbor.rs`에 두고 existing C11.4 fixture route에서만 등록한다.

- Scene ID는 `fit-content-auto-neighbor-wpt`다. Root ID는 `c114-fit-content-auto-neighbor-grid`; ordered child IDs는 `c114-fit-content-auto-neighbor-fit-item`, `c114-fit-content-auto-neighbor-fit-child`, `c114-fit-content-auto-neighbor-auto-item`, `c114-fit-content-auto-neighbor-auto-child`다. 각 item은 해당 child 하나만 직접 가진다.
- 입력은 현재 C11.4 opt-in test scene route가 만드는 HostDocument다. Production app의 root HTML 파일을 `ParsedRootHtml`로 읽는 경로가 이 fixture에 연결되는 것은 아니며, 이 계획은 루트 HTML parser parity나 color-scheme/media-query 동작을 검증하지 않는다.
- Grid root: `display: grid`, `width: 60px`, definite block size 300px, one 60px explicit column, `grid-template-rows: none`, `grid-auto-rows: fit-content(40px) auto`, zero gaps/padding/border, `align-content: normal`, row auto-flow.
- 두 direct grid items를 implicit rows에 순서대로 자동 배치한다. 각 item은 고정 block-size 10px인 nested block child 하나를 가진다. Text, replaced image, min/max constraint, border/padding, span, nested Grid 또는 percentage는 넣지 않는다.
- Root width를 60px로 고정해 전체 frame을 결정적으로 만든다. Upstream test의 inline size는 auto지만 대상은 block-axis sizing이고, 한 60px explicit column과 두 10px child의 수직 입력을 유지한다. Diagnostic paint color는 화면에서 row/item frame을 구별하는 데만 쓰며 sizing style을 추가하지 않는다.
- Computed-style 경계에서 root의 `ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1`과 computed `grid-auto-rows` `[fit-content(40px), auto]`, `align-content: normal`, row auto-flow, 0 row gap, 빈 explicit rows를 확인한다. 별도의 `GridContainerInput` 경계에서는 `placement_mode: GridPlacementMode::TrackSizing`, `GridAutoRows::TrackList([GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40)), GridTrackSize::Auto])`, 빈 explicit rows 및 같은 root identity를 확인한다. 이 검사 또는 동등한 fixture-only JSON 원장이 없으면 후속 Taffy track trace만으로 통과시키지 않는다.
- 계산 입력은 기존 `RuntimeBlockFlexGridTrackSizingV1`에 한정한다. 새 CSS grammar, public API, 기본 CSS admission, 다른 placement profile의 허용 범위는 변경하지 않는다. Unknown scene ID는 source를 반환하지 않아야 하며, mutated row list, 누락/추가 child, extra explicit row, stale revision은 positive matcher에서 거부되고 partial output을 내지 않아야 한다. Focused Rust/FFI tests에서 이 negative controls를 각각 실행해 rejection과 partial output 부재를 확인한다. Unsupported profile/value나 malformed source는 기존 오류로 fail-closed한다.

## 관측값과 판정

Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 각각에서 최종 source를 새로 빌드해 동일 source/input hash에 묶인 1-scene runtime 실행을 보존한다.

1. Root와 ordered item/child node IDs가 예상 tree를 이루고, fixture route가 정확한 allowlisted source를 반환한다. 하나의 grid trace에는 Columns와 Rows 각 pass가 하나씩 있고, Rows pass는 정확히 하나여야 한다.
2. Computed-style 입력과 LayoutInput 투영은 root `display: grid`, 정확한 `grid-auto-rows`, `align-content`, auto-flow, gap, explicit-row 및 TrackSizing profile 값과 root NodeId를 보존한다. 두 경계의 document/style/environment revision이 서로 일치해야 한다.
3. Root border-box는 `[0, 0, 60, 300]` CSS px; Rows trace에는 2개 implicit CSS rows만 있고 used sizes는 정확히 `[10, 290]px`다. Gutter entries는 CSS track count에서 분리한다.
4. 같은 Rows pass에서 `IntrinsicSizingComplete`, `MaximizeTracks`, `ExpandFlexibleTracks`, `StretchAutoTracks`가 각각 한 번 나타나고 이 순서여야 한다. 앞선 stage bases는 `[10,10]px`, 마지막 stage bases는 `[10,290]px`다. 마지막 stage는 `StretchAutoTracks { applied: true }`이며 auto row 하나만 늘어난다.
5. LayoutOutput의 pre-render `LayoutFrame` CSS-px 값은 root `[0,0,60,300]`, 첫 item/child `[0,0,60,10]`, 둘째 item `[0,10,60,290]`, 둘째 child `[0,10,60,10]`이다. 이 frame과 style snapshot, track trace와 draw/present는 같은 source revision에 묶인다. Missing, duplicate, stale, partial 결과, layout error, frame/track 오차, 미제출 scene은 실패다.
6. Track/Frame 값은 surface transform이나 device-pixel capture 전의 Taffy `LayoutOutput` CSS-px 필드에서 읽는다. 따라서 AVD density와 iOS Retina scale을 적용한 pixel 좌표로 바꾸거나 반올림하지 않는다. 화면은 시각 보조 자료일 뿐 sizing oracle이 아니다.
7. Android와 iOS에서 저장한 화면은 layout box와 실제 surface 관측 범위를 구분한다. 화면을 PR 본문에 넣을 수 있으면 최종 commit을 가리키는 immutable raw image URL로 첨부한다. Pixel 표본 equality는 별도 측정하지 않으면 주장하지 않는다.

정수 CSS pixel tracks/frames의 허용 오차는 `0 CSS px`다. 출력 변환에서 float를 사용하면 source/trace의 exact integer를 확인하고 예상과의 실제 차이를 그대로 기록한다. 평균 또는 플랫폼간 다수결로 단일 플랫폼 실패를 통과시키지 않는다.

## 작업 순서와 적대 검토

- 계획 문서/입력/규범 산술의 [20관점 검토](./c11-4-fit-content-auto-neighbor-runtime-plan-review-20.md)는 PASS 20/20이다. 계획 PR diff는 프로젝트 필수 규칙에 따라 별도 [30관점 검토](./c11-4-fit-content-auto-neighbor-runtime-plan-pr-diff-review-30.md)를 거친다. 두 검토는 서로 다른 관점을 사용한다.
- 계획 병합 후 구현은 새 branch에서 시작한다. 실제 source와 focused Rust/FFI tests, Android AVD 및 iOS Simulator 증거를 수집한다. `cargo build --locked -p spinon-ffi --lib`, `cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu`, `cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test` 세 artifact를 각각 만든다. 기본 runtime 및 `c04-runtime-gpu` 제품 artifact에는 C11.4 fixture ABI/scene string/payload가 없고, fixture-test artifact에는 있어야 한다. `llvm-nm`·문자열 검사 또는 동등한 검증으로 exclusion/positive control을 남긴다. 제품 코드 영향이 생기면 영향 범위 테스트와 internal contract 0059를 갱신한다.
- 구현·앱 동작은 계획 검토와 다른 새 관점 20개로 검토하고, 구현 PR 변경 자체도 별도 30개 PR-diff review를 거친다. 결함 수정 뒤 영향을 받은 관점은 재실행한다.
- 최종 source 구현/runtime 검토는 [20개 실패 관점 보고서](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/implementation-review-20.md)에 기록한다. 구현 PR의 diff 검토는 이 계획 검토 및 구현 검토와 별도로 수행한다.
- GitHub check rollup이 비면 CI 통과라고 기록하지 않는다. 병합은 Rebase and merge다. Tailnet preview를 갱신하면 서버 응답 hash와 Agent Browser 화면도 확인하고 Agent Browser/전용 Chrome process를 닫는다. Terminal Browser는 사용하지 않는다.

## 범위 밖

이 계획은 전체 WPT suite·Spinon WPT harness, arbitrary Grid CSS, public author CSS admission, 다른 CSS/Grid profile, production root HTML parsing parity·color-scheme/media-query, runtime fixture 전체 행렬, 실기기·Android hardware GPU·iOS compositor 이후 pixel, C11.4/C11 지원 완료를 다루지 않는다. 공식 C11.4 checkbox와 완료 집계 `72/213`은 변경하지 않는다.
