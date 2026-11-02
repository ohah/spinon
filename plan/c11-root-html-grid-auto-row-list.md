# C11.1 parsed Root HTML `grid-auto-rows` fixed-px list 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML Grid runtime](./c11-root-html-grid-runtime.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 계획 PR [#1041](https://github.com/ohah/spinon/pull/1041)이 병합됐고 구현 PR [#1042](https://github.com/ohah/spinon/pull/1042)이 열려 있다. 기본 parsed Root HTML runtime 구현은 테스트·구현/runtime 실패 관점 20개와 실제 PR diff 실패 관점 30개 검토를 마쳤다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기본 route의 layout-ready 및 11-box draw/present를 확인했고, Agent Browser가 PR 본문의 두 화면을 정상 로드하는 것도 확인했다. GitHub check rollup은 비었고 main required checks도 설정되어 있지 않다. Simulator 화면은 PixelCopy·Metal readback이 아니다. 이 slice를 병합해도 C11.1/C11 상위 체크와 공식 완료 `72/213`은 그대로 둔다.

## 목적과 경계

현재 기본 Root HTML 경로는 `grid-auto-rows:auto` 또는 단일 finite non-negative px 값을 허용한다. `ComputedCssGridAutoRows::TrackList`는 Root HTML profile에서 거부한다. layout core도 `RootHtmlImplicitRowsV1`에서 단일 `LengthPx`만 받아 implicit rows를 같은 크기로 확장한다. 반면 이미 병합된 C11.3.3/C11.4 경로에는 fixed track list와 순환 동작이 있으므로, 이 slice는 새 Grid sizing algorithm이나 새 CSS parser 문법을 만들지 않고 그 패턴을 기본 parsed Root HTML route에 한정해 연결한다.

받아들일 범위:

- 기본 `RuntimeRootHtmlTextPaintV1` parsed Root HTML 경로만 확장한다. opt-in C11.4 fixture profile이나 scene-id 기반 제품 경로를 추가하지 않는다.
- `grid-auto-flow: row`, `grid-auto-columns: auto`, 기존 Root HTML placement·item 제한을 유지한다.
- `grid-auto-rows`의 기존 단일 fixed-px 입력을 그대로 유지하고, 추가로 비어 있지 않은 직접 fixed-px track list를 허용한다. 각 값은 finite이고 0 이상이어야 한다. 목록은 explicit rows 뒤 첫 implicit row부터 순서대로 순환한다.
- 기존 fixed implicit-row profile에서 허용한 explicit direct px/percentage/fr tracks와의 조합은 회귀 없이 유지한다. `grid-template-areas`와 implicit row sizing의 조합은 계속 거부한다.
- `display:none` 직접 자식은 기존처럼 placement occupancy와 implicit-row count에서 제외한다. DOM order가 placement 순서를 결정하며 item ID/class는 계산 결과를 선택하지 않는다.
- resource cap, checked arithmetic, allocation failure의 오류, stale revision 거부와 failure atomicity를 유지한다.

계속 거부할 입력은 percentage, `fr`, `auto`, intrinsic track, `minmax()`, `fit-content()`, auto-repeat, 계산식, 빈 track list, 음수·비유한 길이, `grid-auto-columns` 변경, implicit columns, `grid-auto-flow:column|dense`, named-area와 implicit-row sizing의 결합, authored placement, nested Grid, intrinsic text/image item 및 기존 Root HTML unsupported style이다. Syntax-invalid CSS가 cascade에서 무시되는 경우와 syntax-valid computed value가 Root HTML profile에서 unsupported로 거부되는 경우를 나누어 확인한다.

## 사전 기준과 독립 기대값

[Chrome 156 Agent Browser 사전 비교](../spec/internal/evidence/c11-root-html-grid-auto-row-list-precomparison-2026-11-02/README.md)는 같은 repository fixture, raw computed values, item frame과 screenshot을 보존한다. fixture의 Grid root는 `100×100px`, explicit columns는 `[40px,55px]`, explicit row는 `[20px]`, row/column gap은 `5px`, 기본 `align-content:normal`·`justify-content:normal`, `grid-auto-rows:15px 25px`다. 여덟 빈 direct block item의 크기를 DOM row에 맞춘다. Root HTML 경로가 거부하는 non-default alignment declaration은 넣지 않는다.

CSS Grid의 implicit track sizing list 순환 규칙과 고정 입력에서 직접 계산한 값은 다음과 같다. Explicit row 뒤에 만들어지는 세 implicit rows는 `[15,25,15]px`이다. Row starts는 `0`, `20+5=25`, `25+15+5=45`, `45+25+5=75`px이며 columns는 `[40,55]px`, column start는 `0`, `40+5=45`px다. 따라서 item frames는 네 DOM row의 각 item에 대해 `(0,0,40,20)`, `(45,0,55,20)`; `(0,25,40,15)`, `(45,25,55,15)`; `(0,45,40,25)`, `(45,45,55,25)`; `(0,75,40,15)`, `(45,75,55,15)`이다. Used row tracks와 세 row gaps의 합은 `90px`이다. `normal`은 고정 track을 늘리지 않으므로 나머지 `10px`은 root 하단에 남는다.

Chrome observation도 `grid-template-rows: 20px 15px 25px 15px`, 같은 8개 item frame과 root frame `[0,0,100,100]`을 반환했다. 이 비교는 해당 브라우저 관측의 기록이며 Chrome 출력만으로 규범을 정의하지 않는다. fixture SHA-256, 관찰 JSON과 screenshot SHA-256은 위 evidence README에 기록했다.

## 구현 순서와 수락 기준

1. 먼저 이 계획과 oracle fixture를 계획 PR로 병합한다. 계획 diff와 계획 실패 관점은 각각 별도 20회 검토 기록으로 남긴다.
2. 구현 PR에서 Root HTML computed-style validator/projection은 non-empty px-only `TrackList`를 허용하되 list 원소 각각의 finite/non-negative 조건을 검증한다. `implicit_rows` profile detection과 explicit-track/area interaction gate도 유효한 fixed-px `TrackList`를 single `LengthPx`와 같이 분류해야 한다. Root HTML profile에 한해서만 `project_auto_rows`가 그 목록을 layout input으로 전달하게 한다. 기존 scalar `LengthPx`의 표현과 계산은 바꾸지 않는다.
3. `RootHtmlImplicitRowsV1`의 admission은 단일 px 또는 non-empty finite non-negative px-only list만 받는다. Root profile이 아닌 다른 placement modes에 허용 범위를 넓히지 않는다. implicit row 확장은 explicit row 다음에서 list의 첫 원소부터 `implicit_index % list.len()` 순으로 적용한다.
4. Row list 유효성, track-count cap과 allocation failure를 배치 결과/frame publication보다 먼저 검사한다. 오류 입력은 root/item partial frame, row trace 또는 surface draw를 남기지 않아야 한다. `display:none`과 revision/identity 처리도 명시적인 regression으로 둔다.
5. Rust projection/layout matrix는 기존 scalar case와 위 Chrome oracle을 포함해 implicit row 1·2·3개에 대한 순환, 0px 경계, row-gap geometry, source-order control, invalid/unsupported list rejection, quota/allocation failure, no-partial-output를 확인한다. 기대값은 고정 fixture 식과 기록된 Chrome observation을 함께 사용한다.
6. 같은 parsed HTML input을 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS Simulator의 기본 Root HTML runtime route에서 실행한다. Build/app/source/input hashes, root/item frames, expanded tracks, layout revision, `layout=ready`, draw/present와 screenshots를 남긴다. PixelCopy/Metal drawable을 수행하면 해당 표면 결과를 draw/present와 구분하며 software AVD, simulator surface, physical device, hardware GPU, compositor-after evidence를 서로 바꾸어 주장하지 않는다.
7. `cargo fmt`, focused Rust/runtime checks, affected crate Clippy와 Android/iOS simulator builds를 실행한다. PR check rollup이 비었거나 required checks가 없으면 CI 통과로 기록하지 않는다. 구현/runtime 실패 관점 20회와 PR 변경 diff 30회는 별도 artifact로 보존한다.
8. 구현 PR 병합 후 `plan/c11-root-html-grid-runtime.md`, `spec/STATUS.md`, internal contract 0059와 공개 roadmap을 확인하고 이 bounded behavior만 반영한다. C11.1/C11/C11.4 완료 checkbox, WPT 적합성 및 공식 `72/213`은 이 slice만으로 바꾸지 않는다.

## 실패 판정과 미포함 주장

- List order를 reverse하거나 첫 implicit row를 list의 두 번째 항목에 연결하면 실패다.
- Scalar regression, zero track, row gap, item source order, CSS Grid overflow-visible frame, hidden-child occupancy 또는 root revision이 바뀌면 실패다.
- `TrackList`를 scene ID로 선택하거나 invalid list에서 partial layout을 publication하면 실패다.
- 지원하지 않는 computed list의 어떤 항목도 기본 root route에 새어 들어오면 실패다.
- 성공은 fixed-px implicit-row list의 이 bounded Root HTML route만 증명한다. 일반 CSS Grid admission, C11.3/C11.4 전체, 전체 WPT, 임의 track value, 실기기·hardware GPU 전체 행렬, glyph pixels, iOS compositor-after pixels 또는 C11 상위 완료를 뜻하지 않는다.
