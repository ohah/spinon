# C11.4 parsed Root HTML `grid-auto-rows:auto` track stretch

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [기존 parsed Root HTML intrinsic rows](./c11-root-html-intrinsic-auto-rows-multiple-text.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**규범 기준:** [CSS Grid Layout Module Level 1 · 2025-03-26 CRD §7.6, §11.4–§11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/). 특히 §11.8은 `align-content: normal|stretch`인 축에서 `auto` max track sizing function을 가진 행들에 남은 양의 definite free space를 균등 분배한다.

**사전 비교:** [HeadlessChrome 157 observation, 독립 기대값, 단일-owner fixture와 화면](../spec/internal/evidence/c11-root-html-grid-auto-row-stretch-precomparison-2026-11-07/README.md).

**계획 실패 관점 검토:** [20개](./c11-root-html-grid-auto-row-stretch-plan-review-20.md). **계획 PR 변경 검토:** 실제 변경분에서 독립 실패 경로 30개를 검토한다. 구현 뒤에는 새 구현/runtime 실패 관점 20개와 실제 구현 PR diff 30개를 별도로 기록한다.

## 결손과 목표

현재 기본 parsed Root HTML C11.4 text 경로는 `grid-auto-rows:min-content`만 text-bearing implicit row sizing으로 허용한다. `grid-auto-rows:auto`는 이 경로에서 거부되어, 기본 `align-content:normal`이 만드는 Grid auto-track stretch를 고정 Root HTML 입력으로 확인할 수 없다.

현재 direct-text source, text measurement, item ownership, source order와 revision 검사를 재사용해 `grid-auto-rows:auto`가 있는 단일 implicit row에서 §11.8 동작을 제한적으로 연결한다. auto row의 20 CSS px minimum contribution은 text의 2줄 min-content height에서 오며, definite 40px Grid 높이에서 explicit row 10px을 뺀 남은 10px을 받아 30px이 된다. runtime trace는 auto track의 item contribution을 `Minimum` 20px로 기록하며, text-layout trace가 두 줄·20px 높이를 따로 입증한다. `align-content:normal`인 Grid의 auto-sized text item도 해당 30px row를 채운다. 고정 fixture는 기본 Root HTML runtime route를 사용하며 새 runtime profile·FFI selector·CSS parser·public API를 추가하지 않는다.

## 고정 입력과 독립 기대값

고정 Chrome 비교는 [precomparison HTML](../tests/fixtures/css/c11/root-html-grid-auto-row-stretch-precomparison.html)에서 `A A A A A A`를 가진 direct text item과 동일 구조의 대조 장면을 실행한다. viewport는 CSS 1280×633, DPR 1, LTR, `horizontal-tb`; Grid content box는 40×40 CSS px, explicit column은 40px, explicit row는 10px, gap은 0이다. 첫 direct child는 높이 10px인 빈 고정 대조 item이며 두 번째 direct child는 text item이다.

CSS Grid §11.4–§11.8에 따른 독립 기대값은 다음과 같다.

| 장면 | 계산 | used rows | text item frame (Grid 좌표) | 줄 수 |
|---|---|---|---|---:|
| `auto-default` | min-content base 20px + §11.8 잔여 10px | `[10,30]px` | `[0,10,40,30]` | 2 |
| `auto-content-start` control | `align-content:start`라 stretch 없음 | `[10,20]px` | `[0,10,40,20]` | 2 |
| `min-content-control` | intrinsic min-content row는 stretch 대상 아님 | `[10,20]px` | `[0,10,40,20]` | 2 |
| `max-content-control` | 이 definite-width text의 max-content block contribution도 20px | `[10,20]px` | `[0,10,40,20]` | 2 |
| `fixed-control` | authored fixed row는 10px 유지 | `[10,10]px` | `[0,10,40,10]` | 2; text overflow 10px |

독립값은 explicit row 10px, 두 줄 text contribution 20px, definite inner block-size 40px의 산술로 정한다. HeadlessChrome 157 observation은 같은 단일-owner fixture에서 CSSOM으로 control을 적용해 계산을 교차 확인하며 규범 기대값을 대체하지 않는다. 앱 검증은 같은 fixture hash의 기본 `auto`/`normal` 상태를 사용한다. computed `grid-auto-rows`, `align-content`, `align-items`, used track list, direct item frame, Text range line rectangles를 분리해 기록한다.

## 구현 범위와 거부 경계

- 기본 parsed Root HTML visible Grid owner는 정확히 하나여야 한다. LTR `horizontal-tb`, row sparse flow, static direct `<div>` items, definite positive 40px width/height, 단일 40px explicit column·10px explicit row, zero gap, no padding/border, `overflow:visible`을 사용한다.
- computed `grid-auto-rows:auto`, computed `align-content:normal`, `align-items:normal`을 요구한다. 첫 direct child는 10px fixed-height empty `<div>`, 두 번째는 하나의 direct text child를 가진 `<div>`이고 auto-placement로 정확히 첫 implicit row에 놓인다. 추가 visible item은 거부한다.
- direct text profile은 기존 Root HTML min-content row의 fixed monospace 10px/10px, `white-space:normal`, `word-break:normal`, `overflow-wrap:normal`, ASCII alphanumeric/hyphenated tokens 및 ASCII collapsible whitespace source gate를 그대로 사용한다. 이 작업은 text grammar나 line-break algorithm을 넓히지 않는다.
- minimum contribution과 auto row의 base size는 기존 text-layout 및 Grid track-sizing engine에서 계산한다. 고정된 20px 기여를 코드에 넣지 않는다. implicit auto row를 명시 template row로 펼치지 않아 trace가 explicit/implicit track 경계를 보존해야 한다. `StretchAutoTracks` trace가 auto row의 20→30px 변화를 기록하고, 최종 item line count·frame·NodeId·source order·document/style/layout revision을 대조한다.
- `align-content:start`는 독립 브라우저 control이지만 제품 input으로는 제한 profile 밖에 둔다. `align-content:stretch` 또는 다른 distribution, 추가 implicit row, multiple text item, multiple Grid, gap, nested/inline text, template area, named/numeric placement, auto-repeat, intrinsic columns, non-40px container, unsupported styles는 이 slice에서 성공 처리하지 않는다.
- 실패 입력은 partial `LayoutOutput`, stale frame, text draw 또는 track trace를 새 revision으로 게시하지 않는다. 기존 default/product feature 경계와 opt-in fixture-test 경계는 그대로 유지한다.

## 구현과 검증 순서

1. 구현 전 Chrome observation, 독립 expected JSON, browser version, fixture/source hashes 및 CSS Text/Grid computed-value·line-count 경계를 저장한다. 계획 자체의 20개 실패 관점 검토를 별도 파일로 남긴다.
2. 이 계획과 precomparison 자료만 담은 계획 PR을 올리고, 실제 diff를 30개의 독립 실패 경로로 검토한 뒤 `Rebase and merge`한다.
3. 정확한 Grid owner와 computed `auto` implicit-row route를 기존 Root HTML gate에 추가한다. computed alignment defaults와 단일 implicit row/cardinality 조건을 검증하고 기존 min-content·fixed rows 경로를 보존한다.
4. Style-to-layout와 RuntimeSession에서 default `RuntimeRootHtmlTextPaintV1` 경로로 positive scene과 min-content/fixed controls를 확인한다. 10,000-track boundary, extra item, wrong alignment, wrong owner/revision, unsupported source/style, malformed track 및 실패 후 같은 session의 정상 요청 복구를 검사한다.
5. Chrome expected values와 같은 source fixture hash로 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 layout-ready, root/item frames, 2 text lines, `StretchAutoTracks` trace, draw/present를 확인한다. 적용 가능하면 화면 밖/border 경계 sample도 별도 기록하되 screenshot을 pixel equality로 해석하지 않는다. Simulator 결과를 실기기·hardware GPU·iOS compositor 이후 표시 근거로 확대하지 않는다.
6. focused Rust tests, changed-crate Clippy, `cargo fmt --all -- --check`, default/product와 opt-in fixture artifact 경계를 기록한다. 기존 baseline failure는 변경으로 생긴 실패와 분리한다.
7. 구현/runtime 변경 자체를 새 20개 관점으로 검토하고, 구현 PR diff를 별도 30개 관점으로 검토한다. 결함을 고치면 영향받은 기대값·runtime·화면 근거를 다시 실행한다.
8. 병합 후 내부 계약 0059, 상위 계획, `spec/STATUS.md`, 공식 roadmap 및 사용자가 지정한 Tailnet 제공본을 제한 동작·merge SHA·검증 한계와 맞춘다. Agent Browser desktop/mobile을 사용한 preview 확인 뒤 해당 browser 세션을 닫는다. 이 slice로 C11.4/C11 체크나 공식 완료 수를 변경하지 않는다.

## 제한 구현 결과와 병합 상태

- 계획 PR [#1227](https://github.com/ohah/spinon/pull/1227)은 Rebase and merge됐고, 구현 PR [#1228](https://github.com/ohah/spinon/pull/1228)은 2026-11-06T16:37:39Z Rebase and merge됐다(merge SHA `d7a26e93e09ba30fdfaea88ad3e8907df89f9afc`). 구현은 같은 checkout의 `feat/c11-root-html-auto-row-stretch` 브랜치에서 진행했다.
- Strict Root HTML 제한 경로에만 `RootHtmlAutoRowStretchV1` 내부 placement mode를 부여했다. 기존 `RootHtmlIntrinsicTrackSizingV1` 경로는 auto implicit row를 계속 거부한다. 허용된 scene은 40×40px Grid, 40px column, 10px explicit row, 기본 alignment, empty fixed item 하나와 직접 text item 하나뿐이다.
- Layout은 단일 implicit `auto` row를 template에 펼치지 않고 Taffy가 만들도록 둬 explicit/implicit trace 경계를 보존한다. RuntimeSession opt-in trace는 minimum contribution 20px, `10px 30px` used rows, explicit/implicit count 1/1, `StretchAutoTracks` 20→30px를 확인한다.
- `spinon-layout` focused 2/2, `spinon-style-to-layout` acceptance gate 1/1, RuntimeSession rejection/recovery 2/2와 fixture-test trace 1/1을 통과했다. 변경 세 crate all-target Clippy와 `cargo fmt --all -- --check`도 통과했다.
- Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 HTML fixture SHA로 기본 Root HTML 앱이 `layout=ready`, 5 boxes로 부팅했다. build/install/runtime log와 screenshot은 [runtime evidence](../spec/internal/evidence/c11-root-html-grid-auto-row-stretch-runtime-2026-11-07/README.md)에 둔다. Screenshot은 visual smoke이며 surface pixel readback이 아니다.
- 기본 release `spinon-runtime` artifact에서 fixture marker가 빠지고 test executable의 positive control에는 포함되는 경계를 확인했다. 새 구현/runtime 실패 관점 20개와 구현 diff 실패 관점 30개 검토를 각각 [20개 검토](../spec/internal/evidence/c11-root-html-grid-auto-row-stretch-runtime-2026-11-07/implementation-runtime-review-20.md), [30개 검토](../spec/internal/evidence/c11-root-html-grid-auto-row-stretch-runtime-2026-11-07/pr-diff-review-30.md)에 완료했다. GitHub workflow/check/status 결과와 main required status checks가 없어 CI 통과로 기록하지 않는다. 병합 후 상태 문서·공개 roadmap·Tailnet roadmap 동기화는 [문서와 미리보기 evidence](../spec/internal/evidence/roadmap-tailnet-sync-c11-auto-row-stretch-2026-11-07/README.md)에 연결한다. C11.4/C11은 계속 미완료이고 공식 완료 `78/214` 및 내부 버전 `0.1.0`을 유지한다.

## 명시적 미포함

이 제한은 general CSS author admission, 임의 text/font/language, inline/nested/replaced content, multiple auto rows, other content-distribution values, generic auto-placement, C11.4 track-sizing algorithm 전체, WPT conformance, physical Android/iOS device, hardware GPU, iOS compositor-after pixel parity 또는 C11/C11.4 완료를 선언하지 않는다. 내부 계약 숫자 버전 `0.1.0`을 유지한다.

## 다음 C11.4 track-sizing 후보

단일 implicit auto row 다음 단계인 two implicit `auto` rows의 제한 균등 분배는 [계획](./c11-root-html-multiple-auto-row-stretch.md) 및 구현 PR [#1231](https://github.com/ohah/spinon/pull/1231)에서 완료돼 main에 병합됐다 (merge SHA `5c79d19619617e7424aaeabbe884579ac383d130`). 이 장면의 used rows는 `[10,15,15]px`이며 제한된 세 item/source profile만 허용한다. [실행·테스트·Simulator 근거](../spec/internal/evidence/c11-root-html-multiple-auto-row-stretch-runtime-2026-11-07/README.md)를 참고한다. 이는 일반 auto-placement나 전체 track-sizing 알고리즘 구현이 아니므로 C11.4/C11 체크와 공식 완료 `78/214`는 그대로다.
