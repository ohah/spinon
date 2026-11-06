# C11.4 parsed Root HTML implicit auto-row alignment과 고정 row-gap

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [선행 auto-row alignment slice](./c11-root-html-auto-row-content-alignment.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 제한 구현 후보 PR [#1240](https://github.com/ohah/spinon/pull/1240) 리뷰 가능. 선행 제한 구현 PR [#1237](https://github.com/ohah/spinon/pull/1237)의 fixed Root HTML shape에 computed `row-gap:0..=7.5px`를 추가하고 gutter 포함 free-space 동작을 검증했다. 실제 PR diff 30관점 검토는 완료했다. GitHub는 checks/workflow를 보고하지 않았고 `main`에는 필수 status check나 승인 수가 설정되어 있지 않으며, PR은 mergeable 상태다. 이 plan slice는 C11.4/C11 완료나 공식 완료 수 증가를 뜻하지 않는다.

**규범 기준:** [CSS Grid Layout Module Level 1 · 2025-03-26 CRD §10.1 gutters, §§11.1·11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/) · [CSS Box Alignment Module Level 3 §§4.3–4.4, §5.1.4](https://www.w3.org/TR/2025/WD-css-align-3-20250311/)

**계획 실패 관점 검토:** [20개](./c11-root-html-auto-row-content-alignment-row-gap-plan-review-20.md) · **실제 계획 PR diff 검토:** PR diff를 고정한 뒤 별도 30개 관점으로 수행한다.

**구현 전 비교:** [Chrome 157·독립 기대값·40 scene 관측](../spec/internal/evidence/c11-root-html-auto-row-content-alignment-gap-precomparison-2026-11-07/README.md)

## 결손과 목표

기존 `RootHtmlAutoRowStretchV1` parsed Root HTML route는 이 정렬 shape에서 `row-gap:0`만 허용한다. CSS Grid gutters는 인접 track 사이에 고정-size empty track처럼 들어가며, 이후 `align-content`가 추가한 distribution space도 gutter 간격에 더해진다. 따라서 row gap을 free-space 계산에서 빠뜨리면 auto-row stretch 크기와 각 item의 track 시작점이 달라진다.

같은 fixed 40×40px root, 40px column, 10px explicit row, `grid-auto-rows:auto`, fixed empty item 1개와 direct text `A`/`B` 두 개에서 computed `row-gap`의 finite px `0..=7.5` 범위를 허용한다. `column-gap`은 계속 0이다. 기존 computed `align-content` allowlist 10개, text contribution, direct-child shape, `align-items:normal`, default Root HTML runtime route, failure atomicity는 유지한다. 새 Grid profile, FFI/scene selector, public API 또는 test-only success path는 추가하지 않는다.

상한 7.5px는 이 고정 장면에서 두 10px implicit row와 10px explicit row에 두 gutter를 더한 extent가 45px 이하, 즉 negative free space가 최대 5px가 되도록 둔다. 이는 general CSS gap limit가 아니며 이 profile 밖으로 전파하지 않는다.

## 사전 비교와 독립 기대값

고정 입력, Chrome observation 및 verifier는 [precomparison evidence](../spec/internal/evidence/c11-root-html-auto-row-content-alignment-gap-precomparison-2026-11-07/README.md)에 둔다. Agent Browser 0.31.1의 HeadlessChrome 157.0.0.0, viewport 1280×633 CSS px, DPR 1에서 row-gap `0`, `2.5`, `5`, `7.5px` 각각에 `align-content` 10개를 적용한 40 scenes를 측정했다.

세 row base는 `[10,10,10]px`이고 row gutters 두 개가 차지하는 크기는 `2g`다. Alignment 전 남는 공간은 `40 - 30 - 2g`; 네 비교점의 값은 `[10,5,0,-5]px`다. `normal|stretch`는 positive space만 두 implicit auto rows에 균등 배분한다. Positional values는 free-space offset을 적용한다. Distributed values는 positive space를 gutters에 더하고, negative space에서 규범상 fallback을 사용한다. 기준 geometry는 [expected-values.json](../spec/internal/evidence/c11-root-html-auto-row-content-alignment-gap-precomparison-2026-11-07/expected-values.json)과 [chrome-observation.json](../spec/internal/evidence/c11-root-html-auto-row-content-alignment-gap-precomparison-2026-11-07/chrome-observation.json)에 고정했다.

Negative free-space 대조에서 `align-content:end`의 첫 row/item은 y=-5px, `center`는 y=-2.5px에 시작한다. `overflow:visible`이므로 이는 의도된 top-edge overflow다. `space-between`은 start fallback을, `space-around`/`space-evenly`는 safe-center fallback을 사용해 세 row가 `[0,17.5,35]px`에 놓인다. 기존 text item은 `align-items:normal`에 따라 auto-row track 크기로 stretch한다. Chrome 1/64 CSS px quantization을 고려한 좌표 허용차는 0.03125 CSS px다.

## 지원 경계와 거부 조건

- Root HTML Grid owner 1개, definite 40×40px content box, 40px single explicit column, 10px single explicit row를 요구한다.
- `grid-auto-flow:row`, `grid-auto-rows:auto`, `align-items:normal`, `justify-content:normal`, `justify-items:normal`, `column-gap:0`, LTR `horizontal-tb`, static position, zero margin/padding/border, `overflow:visible`을 유지한다.
- 보이는 direct item은 source 순서의 empty fixed 40×10px `div`, 10px/10px monospace direct text `A`, `B`다. Text node identity, owner 및 style/layout revision이 같은 snapshot이어야 한다.
- `row-gap`은 typed finite non-negative fixed px만 받고 7.5px를 포함한다. Percentage, unresolved CSS math, 7.5px 초과, non-zero `column-gap`, 다른 root size·tracks·items는 해당 property/node를 포함해 layout publication 전에 거부한다. CSS-invalid declaration이 computed initial value로 돌아온 경우와 유효하지만 이 runtime profile이 받지 않는 값을 구분한다.
- `align-content` computed allowlist는 `normal`, `stretch`, `start`, `flex-start`, `end`, `flex-end`, `center`, `space-between`, `space-around`, `space-evenly`다. `baseline`, `safe|unsafe`, `self-start/end`, math 및 다른 값은 계속 거부한다.
- `place-content`는 computed longhand 둘을 각각 검증한다. 이 slice는 `justify-content` 또는 `justify-items`에 새 비초기 값을 열지 않는다.
- 다른 Root HTML intrinsic/mixed/Grid fixture profile, 일반 author CSS admission, WPT 적합성 및 C11.4/C11 전체 지원을 주장하지 않는다.

## 구현 및 검증 순서

1. precomparison fixture/hash/독립 기대값을 유지하고 40 scene verifier를 실행해 소스 관측과 수식이 동기화됐는지 확인한다.
2. Style projection의 `root_html_auto_row_stretch_element_candidate`에서 typed row-gap `LengthPx(0..=7.5)`만 허용한다. 기존 row-gap 오류의 property/node 귀속을 보존하고 column-gap 및 다른 profile gates를 약화하지 않는다.
3. `spinon-layout`의 `RootHtmlAutoRowStretchV1` contract gate에서 같은 finite px 상한을 검사한다. track sizing·content distribution은 기존 Taffy 경로를 사용하고 gap을 별도 보정해 두 번 더하지 않는다.
4. Layout·projection·RuntimeSession tests에서 네 gap 값 × alignment 10개, `[10,10,10]`/stretch track sizes, exact item frames, distributed negative fallback, `end`·`center`의 y<0 overflow 및 `place-content`/unsupported profile을 검사한다. Gap 7.5의 top overflow와 root boundary exact geometry를 고정한다.
5. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 fixture/source hash를 실행한다. 최소 zero-gap baseline, `2.5px + space-between`, `7.5px + end`, `7.5px + center`를 양 플랫폼에서 확인한다. `end`/`center`의 root 밖 item을 screenshot 및 가능한 surface readback으로 검사하고, Rust frame equality, Android PixelCopy, iOS Metal drawable 결과를 서로 다른 증거로 보고한다.
6. Focused Rust tests, 영향을 받는 crate Clippy, `cargo fmt --all -- --check`, simulator build/runtime, default-vs-fixture artifact boundary와 `git diff --check`를 수행한다. Base/candidate broad suite가 다르면 동일 실패와 새 실패를 분리해 기록하고 전체 suite를 green이라고 부르지 않는다.
7. 구현/runtime은 plan review와 독립적인 failure review 20개, 실제 구현 PR diff는 별도의 30개 관점으로 검토한다. PR body에 Android/iOS 화면을 첨부할 수 있으면 포함한다.
8. 병합 후 내부 contract `0059` 숫자 버전 `0.1.0`을 유지하고, plan·`spec/STATUS.md`·public roadmap 및 지정 Tailnet preview를 제한 동작·merge SHA·검증 한계와 동기화한다. C11.4/C11 상위 checkbox와 공식 `78/214`는 자동 승격하지 않는다.

계획 병합과 이후 구현은 현재 checkout 하나에서 순차 진행한다. 별도 worktree나 병렬 implementation PR을 만들지 않는다. Agent Browser로 웹/fixture를 확인한 뒤 세션을 종료하고 browser process가 남지 않았는지 검사한다.

## 계획 검토에서 확인한 핵심 위험

- Gap은 track 사이에만 있으므로 세 row에 gap을 세 번 빼면 40px 경계가 틀어진다.
- `normal|stretch`만 auto tracks를 늘린다. Gap을 뺀 free space 전체를 item 높이에 주거나 explicit row까지 stretch하면 안 된다.
- `row-gap:5px`는 free space 0인 경계, `7.5px`는 negative 5px 경계다. 음수 free space를 zero로 clamp하면 `end`/`center` overflow가 사라진다.
- Distributed fallback은 positional `end`/`center`의 기본 unsafe 동작과 다르다. Chrome oracle 및 spec fallback을 별도로 검사한다.
- Precomparison screenshot은 browser reference다. Simulator screenshot/readback은 native runtime 증거이며 서로 대신하지 않는다.
- 전체 C11 Grid semantics와 일반 CSS support, hardware GPU 또는 WPT coverage로 승격하지 않는다.

## 명시적 미포함

이 slice는 percentage/calc/auto gap, 7.5px 초과 gap, non-zero column gap, collapsed gutter, auto-repeat/areas/named lines/spanning, alternate root geometry, safe/unsafe modifier, arbitrary text, multiple Grid owners, nested item, RTL/other writing mode, scroll/clip, WPT, 실기기, hardware GPU, full compositor parity 및 C11.4/C11 완료를 포함하지 않는다.
