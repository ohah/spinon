# C11.1 · parsed Root HTML intrinsic Grid track sizing

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML Grid runtime](./c11-root-html-grid-runtime.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 기준:** [Chrome 156 Agent Browser 사전 비교](../spec/internal/evidence/c11-root-html-intrinsic-tracks-precomparison-2026-11-05/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-intrinsic-tracks-precomparison.html) · [독립 기대값](../spec/internal/evidence/c11-root-html-intrinsic-tracks-precomparison-2026-11-05/expected.json) · [Chrome 관찰·화면](../spec/internal/evidence/c11-root-html-intrinsic-tracks-precomparison-2026-11-05/chrome156-observation.json)

**계획 실패 관점 검토:** [20개](./c11-root-html-intrinsic-grid-tracks-plan-review-20.md) · **계획 PR diff 적대 검토:** 구현 전 기록한다. **구현/runtime 검토:** 구현 전에 별도 20개를 기록한다. **구현 PR diff 적대 검토:** 계획 검토와 중복 없이 30개를 기록한다.

## 목표와 결손

기본 parsed Root HTML runtime은 explicit Grid track을 이미 cascade부터 typed projection·Taffy·scene까지 연결하지만, `grid-template-columns`와 `grid-template-rows`의 intrinsic track size를 projection validation에서 거부한다. `GridTrackSize` 변환과 Taffy track-sizing algorithm은 `Auto`, `MinContent`, `MaxContent`, `MinMax`를 표현할 수 있다. C11.4 opt-in runtime도 item의 text·image min/max-content contribution을 계산하지만, 그 opt-in 경로는 일반 Root HTML 실행이 아니다.

이번 변경은 기본 `RuntimeRootHtmlTextPaintV1` 경로에서 기존 HostDocument, computed style, style/layout revision 및 현재 text measurement 경계를 유지하면서 intrinsic explicit track sizing을 실제 Root HTML 문서에 연결한다. CSS parser, 새 runtime profile, FFI scene ID, public API 또는 별도 test-only success path는 추가하지 않는다.

## 지원 범위

- `grid-template-columns`와 `grid-template-rows`의 direct `auto`, `min-content`, `max-content` track.
- 명시 track list 내 `minmax()` breadth 조합:
  - minimum: finite non-negative `px`, percentage, `auto`, `min-content`, `max-content`.
  - maximum: finite non-negative `px`, percentage, `auto`, `min-content`, `max-content`, `fr`.
  - `<flex>` minimum은 CSS 문법상 무효이므로 computed-style parser fallback 또는 typed projection rejection을 고정한다.
- 기존 fixed integer `repeat()` expansion 안의 위 track 값.
- 기본 no-area, `grid-auto-flow: row`, 기존 root/block 및 child placement 조건에서 direct text/box item의 unspanned intrinsic contribution과 적어도 하나의 multi-track span 대조. Columns/rows 모두 실행한다.
- `%` breadth와 `fr` maximum은 기존 definite-axis, finite value, aspect-ratio·min/max 및 resolved-range gate를 그대로 지킨다. Intrinsic value를 만났다는 이유로 축의 뒤쪽 fixed/percentage 값 검사를 건너뛰지 않는다.
- `justify-content:start`와 `align-content:start`를 사용해 auto-track stretch와 content alignment의 효과를 분리한다. 별도의 auto-track stretch scene은 기본값 동작도 고정한다.

## 이번 범위 밖

`grid-template-areas`와 intrinsic track의 조합, line names, `auto-fill`/`auto-fit`, intrinsic `grid-auto-rows`/`grid-auto-columns`, column-flow·dense placement, orthogonal writing mode, arbitrary inline/author CSS admission, public CSS support, full WPT conformance, C11.1/C11 완료는 포함하지 않는다. 이 조합은 현재의 root HTML route에서 기존처럼 fail-closed한다. `fit-content()`의 px/% cap, 기존 direct `px`/`%`/`fr`, numeric placement, area 및 gap profile은 별도 회귀로 고정하고 이 변경에서 넓히지 않는다.

지원하지 않는 computed intrinsic combination은 scene publish 전에 property·node가 식별되는 진단으로 실패해야 한다. CSS-invalid declaration은 Stylo cascade가 선택한 computed value/fallback을 따르며, 무효 declaration을 구현 거부로 잘못 보고하지 않는다. `auto`, 빈 track 또는 부분 frame으로 대체하는 처리는 허용하지 않는다.

## 규범과 판정 기준

CSS Grid Level 1, 2025-03-26 CRD §7.2.1과 §11.3–§11.8을 기준으로 한다. `min-content` track은 item의 min-content contribution을, `max-content` track은 max-content contribution을 사용한다. `minmax()`는 minimum과 maximum track sizing function을 각각 적용하며 max sizing function이 최소보다 작으면 최소로 바닥 처리한다. Flexible maximum은 definite available space의 gutter·fixed contribution을 반영한 후 여유 공간을 분배한다. `auto` maximum은 §11.8 auto-track stretch와 content alignment의 영향도 받으므로 이를 별도 scene으로 분리한다.

Agent Browser Chrome 156 관찰값과 독립 expected JSON은 축별 `0.02 CSS px` 안에서 비교한다. Browser는 참고 구현 관찰값이며 normative 기대값은 명세와 고정 입력으로 계산한다. monospace 글꼴 advance 차이가 있는 Android/iOS simulator geometry는 각 플랫폼의 실제 text advance를 반영해 별도로 보고하며 expected 대비 `0.5 CSS px`를 허용한다. Rust projection/layout regression은 고정 fixture에서 `0.02 CSS px` 안에 들어야 한다. DPR 간 CSS px 기대값은 변하지 않는다.

구현 중 Chrome oracle, Android/iOS 결과 또는 Taffy 동작이 독립 기대와 다르면 단순 tolerance 확장이 아니라 명세·입력·엔진의 차이를 재현한 뒤 계획을 갱신한다.

## 실패 경계와 회귀 집합

1. 고정한 14개 Chrome scene을 Root HTML default app route에서 같은 source revision으로 실행한다. Columns와 rows, `auto`, direct intrinsic keywords, `minmax()` intrinsic breadth, fixed/intrinsic floor, `fr` expansion 및 integer repeat의 resolved track과 item frame을 비교한다.
2. 추가 Chrome scene에는 item의 2-track span, empty/hidden item control, min-/max-content가 다른 wrapping text, content alignment가 auto-track size에 주는 효과와 container 밖 max-content overflow를 포함한다. 각 scene에는 독립 expected를 둔다.
3. Rust projection 검사는 typed allowed combinations를 축별로 표로 검증한다. 음수·NaN·무한대 breadth, `<flex>` minimum, unsupported area/auto-repeat/implicit-track 조합은 문법상 invalid, cascade fallback, 또는 explicit fail-closed 중 정확한 계층에서 판정한다.
4. Percentage/minmax와 `fr`/minmax의 definite·zero·indefinite basis, aspect-ratio, min/max override, finite f32 경계와 track-list aggregate overflow를 positive/rejection matrix로 확인한다. Intrinsic breadth 뒤의 large fixed/percentage track도 preflight를 우회하지 않아야 한다.
5. 여러 Grid scene을 한 Root HTML 문서에 두어 한 grid의 error/hidden child가 다른 grid의 frame·scene을 부분 게시하지 않는지 확인한다. root, computed-style, layout, text measurement revisions가 같아야 한다.
6. 기존 Root HTML grid tests: gap, direct px/%/fr, `minmax(px|%, fr|px|%)`, fixed repeat, implicit fixed rows, named areas, auto minimum, column/dense flow, placements, content/item/self alignment, direct-text aspect-ratio의 focused 회귀를 실행한다.
7. `cargo fmt --all -- --check`, 변경 크레이트 focused tests와 scoped Clippy, Rust runtime tests를 실행한다. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 기본 app entry로 fixture를 실행하고 frame·layout state·draw/present를 보존한다. 제품 화면 evidence로 사용한 경로가 읽기 가능한 경우 Android PixelCopy와 iOS Metal drawable 표본을 별도 수집한다. Simulator readback을 실기기·hardware GPU·iOS compositor 이후 결과로 확대 해석하지 않는다.
8. 실제 Default/product artifact에 fixture 전용 ABI, 고정 scene selector 또는 fixture payload가 추가되지 않았는지 확인한다. PR CI가 없거나 check rollup이 비어 있으면 CI 통과로 보고하지 않는다.
9. Agent Browser로 Chrome precomparison 및 PR diff 화면을 열어 확인한다. 사용한 Agent Browser session/process는 검증 뒤 닫고, 다른 Chrome process는 건드리지 않는다.

## 수락·병합 순서

1. 고정 precomparison, 이 계획과 계획 실패 관점 20개를 마친 다음 계획 PR을 올린다. 계획 PR diff도 별도 30개 적대 관점으로 검토하고 Rebase and merge한다.
2. 병합한 계획을 기준으로 같은 checkout의 기본 Root HTML validator, typed intrinsic track projection, revision-bound layout matrix와 Android/iOS Simulator 실행을 구현한다.
3. 실제 변경 코드와 runtime을 대상으로 새로운 실패 관점 20개를 기록한다. 이어서 구현 PR diff 30개를 별도로 검토하고 발견을 수정한 뒤 PR을 올린다.
4. 구현 PR은 기존 `enhancement`, `area: layout` 라벨과 [PR 양식](../.github/pull_request_template.md)을 사용하고, 변경/미변경 상태 ID, 기준, 화면·실행 근거 및 제한을 쓴다. 필요하면 Android/iOS Simulator 화면을 PR 본문에 첨부한다. Rebase and merge 후 merge SHA와 check 상태를 재확인한다.
5. 병합 근거를 내부 계약 0059, `spec/STATUS.md`, C11 전용 계획 및 공개 roadmap에 반영한다. C11.1/C11/C11.4의 상위 체크는 유지한다. 사용자가 지정한 Tailnet roadmap preview는 새 SHA, 완료 집계, desktop/mobile scene을 Agent Browser로 대조한 뒤 현재 서버에 갱신하고 관련 프로세스를 정리한다.
