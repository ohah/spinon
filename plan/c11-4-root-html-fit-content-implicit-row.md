# C11.4 parsed Root HTML `grid-auto-rows:fit-content(px)` 제한 slice

**상태:** Chrome 157 사전 관측, 독립 기대값, 계획 실패 관점 20개, 구현/runtime 실패 관점 20개, PR diff 실패 관점 30개 검토를 마쳤다. CSS projection, 별도 implicit-row layout mode, track placement와 기본 RuntimeSession 경로를 구현했다. Layout oracle 2/2, Root HTML RuntimeSession 2/2 및 explicit `fit-content` regression 2/2 focused tests, workspace Clippy·rustfmt·diff check가 통과했다. PR diff 검토 중 발견한 기본 layout trace flag를 제거한 최종 source로 다시 빌드해 Android API 37 ARM64 AVD와 새 iPhone 17 Pro/iOS 26.2 Simulator의 기본 route에서 `page_scale=1`, `layout=ready`, `boxes=7`을 확인했다. 캡처는 visual smoke다. 이제 feature PR 본문과 이미지 링크를 준비한다. 이전 iOS Simulator의 app-data container identity 차이는 해결된 것으로 간주하지 않는다. Android는 기존 AVD에 `install -r`했고 app-data contents는 확인하지 않았다. [실행·source·data-container 근거](../spec/internal/evidence/c11-4-root-html-fit-content-auto-rows-runtime-2026-11-07/README.md) · [PR diff 실패 관점 검토 30개](../spec/internal/evidence/c11-4-root-html-fit-content-auto-rows-runtime-2026-11-07/pr-diff-review-30.md).

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**선행 slice:** parsed Root HTML singleton [`grid-auto-rows:max-content`](./c11-4-root-html-intrinsic-auto-rows-max-content.md)는 PR [#1264](https://github.com/ohah/spinon/pull/1264)로 병합됐다. C11.4/C11은 미완료이고 공식 `78/214`는 유지한다.

**사전 비교:** [Chrome 관측·독립 기대값](../spec/internal/evidence/c11-4-root-html-fit-content-implicit-row-precomparison-2026-11-07/README.md) · [고정 fixture](../tests/fixtures/css/c11/root-html-grid-auto-rows-fit-content-cap-precomparison.html)

**계획 실패 관점 검토:** [20개](./c11-4-root-html-fit-content-implicit-row-plan-review-20.md).

## 결손과 목표

기본 parsed Root HTML `RuntimeRootHtmlTextPaintV1`은 fixed-px explicit track을 가진 제한 Grid에서 singleton `min-content`와 `max-content` implicit row를 각각 허용하지만 `grid-auto-rows:fit-content(<length>)`는 거부한다. 별도의 `RootHtmlFitContentV1` 경로는 explicit track에 한정되어 있고, fit-content implicit row의 CSS cascade → projection → layout → default `RuntimeSession` 연결은 없다.

이 slice는 finite non-negative px cap 하나를 담은 singleton `fit-content(px)` implicit row를 같은 default Root HTML profile에 제한 연결한다. CSS Grid의 fit-content track size는 `max(minimum, min(limit, max-content))`로 계산된다. Chrome oracle은 cap 24px에서 34px content가 24px row로 제한되고, cap 40px에서는 34px을 유지하며, item `min-height:28px`은 24px cap보다 우선해 row를 28px로 만드는 경계를 확인했다. [CSS Grid Level 2 §7.2.1, §7.6, §12.2, §12.5](https://www.w3.org/TR/css-grid/#track-sizing).

## 고정 입력과 독립 기대값

고정 fixture의 Grid는 content box `40×100px`, explicit columns `[40px]`, explicit rows `[10px]`, row flow, gap 0, `align-content:start`, visible overflow다. 순서대로 explicit-row item 10px, implicit-row item 하나, 뒤따르는 6px probe item을 둔다. Implicit item에는 `min-height:0`과 nested block 34px를 주며 fit-content track의 max-content contribution은 34px다. 세 장면은 각각 `fit-content(24px)`, `fit-content(40px)`, `fit-content(24px)` + item `min-height:28px`다.

| 장면 | Used rows | implicit item | nested content | 다음 probe |
|---|---|---|---|---|
| cap24 | `[10,24,6]px` | `[0,10,40,24]` | `[0,10,40,34]` | `[0,34,40,6]` |
| room40 | `[10,34,6]px` | `[0,10,40,34]` | `[0,10,40,34]` | `[0,44,40,6]` |
| min-floor24 | `[10,28,6]px` | `[0,10,40,28]` | `[0,10,40,34]` | `[0,38,40,6]` |

모든 장면의 Grid frame은 `[0,0,40,100]`이다. Chrome의 computed `grid-auto-rows`, used `grid-template-rows`와 모든 자식 frame은 [관측 JSON](../spec/internal/evidence/c11-4-root-html-fit-content-implicit-row-precomparison-2026-11-07/chrome-observation.json)에 있다. 독립 산술과 판정값은 [expected values](../spec/internal/evidence/c11-4-root-html-fit-content-implicit-row-precomparison-2026-11-07/expected-values.json)에 고정했다. Oracle은 Agent Browser HeadlessChrome 157.0.0.0, macOS arm64, viewport 1280×633, DPR 1이다.

## 지원·거부 경계

- 기본 parsed Root HTML `RuntimeRootHtmlTextPaintV1`에서만 computed `[FitContent(LengthPx(cap))]` singleton auto-row track을 허용한다. `cap`은 finite non-negative CSS px다. Stylo가 authored length 또는 constant length-only `calc()`를 동일한 `LengthPx`로 계산하면 같은 값으로 수용한다. Percentage 및 `Calc` typed AST로 남는 limit은 거부한다.
- Grid owner 하나, definite positive px width/height, 단일 positive fixed-px explicit row/column, sparse row flow, `grid-auto-columns:auto`, 0 gap, no area/repeat/name, visible overflow와 기존 item-capacity·track-limit을 유지한다.
- 이 slice는 percentage cap, percentage가 포함되거나 computed `Calc` AST로 남는 cap, negative/non-finite cap, 여러 auto-row track, fit-content와 min/max/길이 혼합, intrinsic explicit tracks, implicit columns, dense/column flow, areas, auto-repeat, nested Grid 및 일반 CSS admission을 열지 않는다. Authored syntax가 아니라 computed typed value를 기준으로 판정한다.
- 잘못되거나 지원 밖인 valid computed value는 `grid-auto-rows`/Grid owner에 귀속된 오류로 fail-closed한다. 실패 시 partial layout/draw snapshot은 발행하지 않으며 같은 `RuntimeSession`에서 유효 입력 회복을 확인한다.
- 내부 계약 버전 `0.1.0`, C11.4/C11 완료 체크와 공식 `78/214`를 유지한다.

## 구현 진행 상황 · 2026-11-07

- Computed singleton `fit-content(px)` 값을 새 `RootHtmlFitContentAutoRowsV1` mode로 분리해 기존 explicit `RootHtmlFitContentV1` 경로를 건드리지 않고 implicit row placement에 연결했다.
- Layout oracle 2/2는 세 장면의 final row sizes와 item/nested-content/probe frames를 검사한다. Default `RuntimeSession` positive path는 같은 세 장면의 computed track identity와 render frames를 검사하고, constant absolute `calc()`와 inch unit이 literal px와 같은 `LengthPx`로 정규화되는 장면을 추가한다. Percentage cap은 atomic no-partial publication 후 같은 session에서 복구되며 percentage가 남는 `Calc` AST도 거부한다.
- Android/iOS Simulator pass 1은 fixture의 viewport meta 부재로 980 CSS px layout viewport와 page scale 약 0.41을 기록했다. Pass 2/3 artifacts를 보존했고, product trace flag 제거를 포함한 최종 source pass 4에서 Android API 37 ARM64 AVD와 새 iPhone 17 Pro/iOS 26.2 Simulator의 기본 route를 다시 실행해 `page_scale=1`, `layout=ready`, `boxes=7`을 확인했다. Pass 4 screenshot·log·source/artifact hashes를 보존했으며 화면은 visual smoke다. Surface pixel readback/equality는 확인하지 않는다. 기존 iOS Simulator의 baseline data-container 경로 문제는 unresolved로 남기고 Android app-data contents도 검사하지 않았다. [실행·source·data-container 근거](../spec/internal/evidence/c11-4-root-html-fit-content-auto-rows-runtime-2026-11-07/README.md).
- C11.4/C11과 공식 `78/214`는 미완료 상태다. 계획·구현/runtime·PR diff review 각 20/20/30개, focused tests, Clippy·rustfmt·diff check, 최종 Android/iOS Simulator smoke를 완료했다. 다음은 이 slice의 feature PR 작성·검토·병합이다.

## 구현·검증 관문

1. 이 계획의 실패 관점 20개 검토와 Chrome/독립 기대값을 구현 전에 고정한다. 구현 검토는 새로운 관점으로 작성한다.
2. Computed `fit-content(px)` type과 px limit을 projection에서 `GridTrackSize::FitContent(LengthPx)`로 보존한다. 다른 style profile과 explicit fit-content 경로는 바꾸지 않는다.
3. Layout admission, implicit-track placement, item capacity는 singleton fit-content cap만 수용한다. Taffy final row trace와 item/probe frame은 위 세 장면 기대값과 일치해야 한다.
4. 기본 RuntimeSession positive path는 computed cascade, typed projection, used tracks, frames, nested content overflow와 source/document/render-tree/style revision identity를 확인한다. Constant length-only `calc()` 및 px로 계산되는 단위는 literal px와 동일한 computed value임을 확인한다. Mixed list, percentage 및 percentage-bearing computed calc limit, nonzero gap, intrinsic column, wrong flow/placement negative controls를 둔다.
5. 실패 요청의 property/node 오류, atomic no-partial output, same-session valid recovery를 확인한다. Focused layout/projection/runtime suites, workspace Clippy 및 rustfmt를 실행한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 최종 source hash로 기본 Root HTML route를 빌드·설치·실행한다. screenshots는 visual smoke와 실제 제한을 구분한다.
7. 구현/runtime 20개와 PR diff 30개 실패 관점 검토를 분리해 기록한다. 완료. PR 본문에 Android/iOS screen을 commit-pinned URL로 넣고 Agent Browser에서 두 이미지의 natural dimensions를 확인한다.
8. PR은 `Rebase and merge`한다. 병합 뒤 STATUS·0059·로드맵·Tailnet 제공본을 동기화하고 제공 hash 및 Agent Browser desktop/mobile 표시를 확인한 뒤 Agent Browser process를 종료한다.

## 명시적 미지원

이 slice는 Root HTML의 모든 `fit-content()` track, percentage 및 computed `Calc` limit, multi-track lists, intrinsic explicit columns와 fit-content implicit row 결합, arbitrary DOM/grid items, WPT 적합성, 실기기/hardware GPU, surface readback, 일반 CSS Grid admission 또는 C11.4/C11 완료를 선언하지 않는다. Stylo computed style이 length-only constant `calc()` 또는 CSS length 단위를 finite non-negative `LengthPx`로 정규화하면 그 계산 결과를 수용한다.
