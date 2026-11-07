# C11.4 parsed Root HTML `grid-auto-rows:max-content` 제한 slice

**상태:** Chrome 157 사전 비교와 계획 실패 관점 20개 검토를 마쳤다. C11.4 intrinsic auto-row 경로는 현재 singleton `min-content`만 허용하고 `max-content`는 거부한다. 이 계획은 같은 고정 track Root HTML text profile에 singleton `max-content`를 추가한다. 공개 로드맵의 stale baseline fallback 진행 카드는 이 작업으로 교체했다.

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [Root HTML intrinsic implicit min-content rows](./c11-root-html-intrinsic-auto-rows.md) · [text contribution](./c11-root-html-intrinsic-auto-rows-text.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**사전 비교:** [Chrome 관측과 독립 기대값](../spec/internal/evidence/c11-4-root-html-intrinsic-auto-rows-max-content-precomparison-2026-11-07/README.md) · [max-content 고정 fixture](../tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-max-content-precomparison.html) · [min-content 비교 fixture](../tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-text-precomparison.html)

**계획 실패 관점 검토:** [20개](./c11-4-root-html-intrinsic-auto-rows-max-content-plan-review-20.md). 구현/runtime 20개와 구현 PR diff 20개 검토는 별도로 작성한다.

## 결손과 목표

기본 parsed Root HTML의 `RuntimeRootHtmlTextPaintV1` 경로는 `RootHtmlIntrinsicTrackSizingV1`을 통해 고정 explicit track과 text item이 있는 Grid의 singleton `grid-auto-rows:min-content`를 지원한다. 현재 `root_html_has_intrinsic_implicit_min_content_rows`와 layout admission은 이 값만 통과시키며 singleton `max-content`는 `grid-auto-rows` 오류로 거부한다. CSS Grid 명세에서 `grid-auto-rows`는 `<track-size>+`를 받고 implicit track 크기를 지정하므로, 유효한 max-content 값을 이 한정된 입력 shape에 연결한다.

대상은 기존 text fixture와 같은 explicit 40px column, explicit 10px row, row flow, 0 gap, 고정 40×40px Grid, text-bearing direct `<div>` 한 개, 마지막 고정-height item 한 개다. 새 parser, public API, FFI selector, fixture-only 성공 route 또는 전체 CSS Grid admission은 추가하지 않는다.

## 규범·기준 환경

- CSS Grid Layout Level 2 §7.6은 `grid-auto-rows`의 값을 `<track-size>+`로 정의하고, explicit track이 아닌 행의 크기를 이 속성으로 정한다고 규정한다. §12.5는 min-content와 max-content track의 base size를 각각 해당 item contribution에서 계산한다. 이 slice가 통과시키는 값은 singleton `max-content` 하나뿐이다. [규범 출처](https://www.w3.org/TR/css-grid/#implicit-track-sizing) · [track sizing algorithm](https://www.w3.org/TR/css-grid/#algo-track-sizing)
- 고정 max-content 입력은 [`root-html-grid-intrinsic-auto-rows-max-content-precomparison.html`](../tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-max-content-precomparison.html), SHA-256 `420ed5e05f9055cd5f8c6c9a031394fc7fdc3ff2d5348e18ed4c35b0f37e5502`다. 기존 min-content 비교 fixture의 SHA-256은 `f0759d21c51af46497327686a854b52cd5e004134c35fd562e3e0170c8df49e6`다. Chrome 관측은 Agent Browser HeadlessChrome 157.0.0.0, macOS arm64, viewport 1280×633, DPR 1에서 얻었다.
- viewport-independent fixture geometry는 definite 40px inline track, explicit 10px row, `line-height:10px`의 `A A A A A A` text, 마지막 34px item, zero gap이다. Chrome은 `min-content`, `max-content`, `auto`에서 모두 used rows `[10,20,34]px`, item frames `[0,0,40,10]`, `[0,10,40,20]`, `[0,30,40,34]`, Grid frame `[0,0,40,40]`를 관측했다. Max-content static fixture의 computed `grid-auto-rows`는 `max-content`이며 Range는 text line rect 두 개를 반환했다. 마지막 item의 visible bottom은 64px로 Grid보다 24px 아래다.
- 이 입력에서는 definite inline-size와 고정 text line-height 때문에 min-content와 max-content의 최종 geometry가 같다. 그러므로 geometry equality 하나를 track function 식별 근거로 삼지 않는다. 검증은 CSS computed value `max-content`, style projection의 `GridAutoRows::TrackList([GridTrackSize::MaxContent])`, layout admission의 typed track, exact frame 및 text source/revision을 함께 확인한다.

## 지원·거부 경계

- singleton `[max-content]`만 기존 `RuntimeRootHtmlTextPaintV1` intrinsic auto-row profile에 추가한다. 정확한 고정 explicit tracks, definite positive container axes, row flow, sparse 기본 배치, LTR `horizontal-tb`, visible overflow, zero box edges/gaps와 기존 owner/item/text 제한을 유지한다.
- intrinsic explicit column과 max-content implicit row의 조합, 다른 implicit track 목록, 혼합 track list, auto-repeat, named line/area, dense·column flow, spanning/authored placement, nested Grid, 추가 text node/run, replaced item, min/max/aspect-ratio 변경, 비정상 수치 및 resource ceiling 초과는 계속 fail-closed한다.
- 잘못되거나 이 profile 밖인 computed value는 `grid-auto-rows`와 Grid owner에 귀속된 오류를 내고 layout/draw snapshot 일부를 게시하지 않는다. 같은 세션의 후속 유효 입력 복구를 확인한다.
- 내부 숫자 버전 `0.1.0`, C11.4/C11 완료 checkbox와 공식 `78/214`는 유지한다.

## 구현·검증 관문

1. 계획 실패 관점 검토와 fixture/Chrome/독립 기대값을 구현 전에 완료한다. 계획 검토는 구현 검토에 재사용하지 않는다.
2. Style projection은 computed singleton `max-content`를 typed `GridAutoRows::TrackList([GridTrackSize::MaxContent])`로 보존한다. Layout의 Root HTML intrinsic mode만 그 singleton을 받아들이며, 기존 min-content와 다른 track lists는 그대로 확인한다.
3. Fixed-shape positive test는 고정 max-content fixture를 읽어 computed value, explicit/implicit row identity, rows `[10,20,34]px`, 세 item frame, visible overflow, paint/source order, text lines와 revision identity를 확인한다. Max-content track이 min-content로 변환되지 않는 typed projection assertion을 둔다.
4. Negative controls는 `max-content`와 intrinsic explicit column 조합, `min-content max-content` list, unsupported `fit-content()`/`minmax()` auto-row value, wrong flow/track count, malformed second text item, hidden item, stale revision 및 over-limit input을 확인한다.
5. RuntimeSession의 성공·오류 귀속, partial-output 부재와 same-session recovery를 확인한다. 대상 crate focused tests, scoped Clippy, `cargo fmt --all -- --check`를 실행한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 HTML source hash를 기본 parsed Root HTML route로 실행한다. layout-ready, text/box frame, overflow와 화면 캡처를 기록한다. AVD software GPU와 iOS compositor 전후 경계를 실제 수집 근거에 맞게 쓴다.
7. 계획 검토와 다른 실패 관점 20개로 구현/runtime을 검토하고, 구현 PR diff도 사용자 요청에 따라 별도 20개 관점으로 검토한다. PR은 기존 label을 재사용하고 `Rebase and merge`로 병합한다.
8. 병합 후 내부 계약 0059, 공식 상태, public roadmap과 Tailnet 제공본을 동기화한다. Tailnet source/local/served hash 및 Agent Browser desktop/mobile 확인 뒤 Agent Browser 프로세스를 종료한다.

## 명시적 미지원·해석 경계

이 slice는 일반 CSS Grid conformance, max-content implicit column, intrinsic explicit-column과 max-content-row 결합, arbitrary track list, subgrid, column/dense placement, general DOM admission, WPT 합격, 실기기 GPU, iOS compositor-after pixel equality, C11.4/C11 완료를 뜻하지 않는다. Chrome과 simulator의 geometry가 같더라도 그것만으로 글꼴·glyph pixel 동등성을 주장하지 않는다.
