# C11.1 parsed Root HTML Grid named-area 제한 통합 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [기본 Root HTML Grid 경로](./c11-root-html-grid-runtime.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 계획 및 Chrome 사전 비교 후보. 구현/runtime 미시작. 이 계획은 C11.1, C11.3.3, C11.4 또는 C11의 완료 표시를 바꾸지 않는다.

**구현 전 비교:** [고정 source와 독립 기대값](../spec/internal/evidence/c11-root-html-grid-template-areas-precomparison-2026-11-02/README.md) · JSON 자료: `chrome-observation.json`, `expected.json`.

## 결손과 목표

병합된 기본 Root HTML Grid 경로는 이름 없는 고정 px explicit tracks, 기본 row auto-placement, fixed non-negative px gap과 빈 direct block items를 계산한다. Typed cascade snapshot에는 `grid-template-areas`와 item placement가 있지만 Root HTML projection은 area template를 거부하고 item placement를 일반 Root HTML Grid 경로에 연결하지 않는다. 이 계획은 **기본 parsed Root HTML runtime entry**에서 한정된 rectangular named areas와 같은 이름의 `grid-area`를 연결한다.

이 범위는 일반 CSS Grid 지원이 아니다. C11.3.3 fixture profile을 기본 Root HTML 경로에서 우회 호출하지 않고, 현재 parsed document·computed style·layout revision을 사용하는 기존 pipeline 안에서만 실행한다.

## 표준과 독립 판정 기준

CSS Grid Layout Module Level 1 2025 CRD의 [Grid Areas §3.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-areas), [Named Areas §7.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-template-areas), [Named Area Placement §8.1.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#common-uses-named-areas), [grid-area shorthand §8.4](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#placement-shorthands) 및 [placement와 source order §8.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#order-property)를 기준으로 한다. 명세는 Grid area를 두 축의 line으로 정의하고 `grid-area`로 named area에 배치하는 방법을 보여 준다. 시각 배치는 source order와 달라질 수 있으므로 fixture는 DOM order를 역전해 semantic item frame을 확인한다.

고정 독립 기대값과 Agent Browser HeadlessChrome 156 관측은 [precomparison evidence](../spec/internal/evidence/c11-root-html-grid-template-areas-precomparison-2026-11-02/README.md)를 따른다.

| 대상 | 기대값 |
|---|---|
| root | `(0,0,120,70)` CSS px; columns `[40,80]`; rows `[20,50]`; areas `"header header" "side content"` |
| header | `(0,0,120,20)` CSS px; 1×2 cells |
| side | `(0,20,40,50)` CSS px; 1×1 cell |
| content | `(40,20,80,50)` CSS px; 1×1 cell |
| reordered DOM | semantic item frame 동일; DOM/source paint order는 그대로 보존 |

기대 frame은 고정 track과 row/column cell을 직접 더해 계산한다. Chrome은 독립 CSSOM/DOMRect 교차 관측이고 normative calculation을 대신하지 않는다.

## 지원 후보의 제한 입력

- `RuntimeSession::new_runtime_gpu_with_root_html()`의 기본 `RuntimeRootHtmlTextPaintV1` route만 확장한다.
- root는 하나의 visible HTML Grid container이며 definite `120×70px`, 두 finite non-negative px columns `[40,80]`, 두 rows `[20,50]`, gap 0, 기본 `grid-auto-flow:row`, static position, visible overflow, zero margin/padding/border를 사용한다.
- `grid-template-areas`는 정확히 두 행·두 열의 `header header` / `side content` rectangular pattern이며, 각 고유 area는 정확히 한 semantic item에 연결한다.
- 정확히 세 개의 visible, in-flow, direct, empty HTML block `<div>`가 있고 각각 computed `grid-area`가 `header`, `side`, `content` 중 하나다. 각 area는 정확히 한 item이 소유하며 item 수·이름 집합이 template와 일치한다.
- Item source order는 고정 ID, class, DOM child index에 성공 여부를 의존하지 않는다. 동일 CSS 의미의 다른 element ID·selector 이름 및 reordered child source가 같은 semantic frames를 내야 한다.
- `grid-template-areas`와 `grid-area` computed typed values를 같은 Root HTML style snapshot에서 가져온다. generated area line names는 기존 typed placement/area projection을 통해 해석하며 CSS 문자열을 다시 파싱하거나 item frame을 사후 보정하지 않는다.
- Track sizing은 existing fixed-px Root HTML subset에 한정한다. gap은 기존 #1002가 허용한 non-negative finite px 또는 0만 유지한다. 이번 고정 oracle은 gap 0이다.

## 제외 입력과 실패 처리

Unknown, missing, duplicate, extra, malformed/non-rectangular areas/items, area/template dimension mismatch, 추가 implicit tracks, auto-repeat, named authored track lines, numeric/span placement, explicit per-line placement, non-default auto-flow, nested Grid, non-empty direct text/image item, non-px/negative/non-finite tracks or gaps, non-zero edges, unsupported display/position/overflow/direction/writing mode는 성공 scene을 publish하기 전에 node/property/value 진단으로 거부한다. CSS syntax-invalid declarations의 cascade drop 동작과 syntactically valid하지만 profile 밖의 computed value 거부를 서로 구분한다.

Failure는 partial layout/render scene, stale revision 결과, 이전 scene의 cached area assignment를 반환하면 안 된다. Area owner는 NodeId가 아니라 같은 snapshot의 computed `grid-area` value로 결정한다. NodeId는 frame provenance 및 diagnostic 식별에만 사용한다.

## 계획된 검증

1. Chrome baseline의 source, expected geometry, computed placement, screenshot과 hashes를 재검증한다.
2. cascade→typed snapshot→layout projection 경계를 확인하고 Root HTML profile만 area/placement를 허용한다. C11.3 fixture profile과 그 positive/negative matrices는 변경하지 않는다.
3. Rust Root HTML integration cases에서 exact root/item frames, computed area ownership, same document/style/layout revision, source-order reverse, renamed IDs/selectors를 확인한다.
4. Negative matrix는 missing/unknown/duplicate/extra area item, malformed area dimensions, non-rectangular names, wrong grid dimension, implicit track, numeric/span placement, non-default flow, nested/hidden/replaced/text items, wrong track/gap/edge/root style, stale revision와 allocation/layout failure를 검사한다. CSS-invalid declaration controls는 computed fallback과 valid-but-unsupported inputs를 분리한다.
5. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 parsed HTML source와 source revision을 기본 Root HTML entry로 실행한다. Frame/root overflow·`layout=ready`·draw/present를 검사한다. PixelCopy/Metal sample을 실행할 경우 별도 surface evidence로 분리하고 iOS compositor 제출 시점을 명시한다. 실기기나 hardware GPU 성능을 주장하지 않는다.
6. Root HTML block/text/background, viewport/color-scheme, fixed track, auto-placement, fixed-px gap, visible-overflow regression과 C11.3.3 template-area fixture profile을 별도 대조한다. 새 일반 route가 fixture-only ABI/payload를 추가하지 않았는지 기본/product 및 opt-in archive controls를 검사한다.
7. `cargo fmt --all -- --check`, focused Rust suites, 관련 crate lint 및 Android/iOS simulator builds를 실행한다. 실행하지 않은 gate는 통과로 기록하지 않는다. PR check rollup이 비어 있으면 CI 성공으로 말하지 않는다.
8. 구현/runtime 코드와 platform evidence를 새 실패 관점 20개로 검토하고, 구현 PR 전체 diff를 별도의 30개 관점으로 검토한다. 이 계획 PR은 계획 검토 20개와 별도로 diff 30개 검토를 거친다.
9. 승인된 구현 뒤 내부 계약 0059, `spec/STATUS.md`, C11 plan과 public roadmap을 제한 범위로 동기화한다. 공식 C11/C11.3.3/C11.4 checkbox와 `72/213`은 그대로 둔다.

## 완료로 주장하지 않을 사항

이 slice는 CSS Grid 일반 `grid-template-areas`/`grid-area` grammar, implicit tracks, arbitrary area shape, arbitrary track/gap sizing, nested Grid, anonymous text or image items, responsive media-query layouts, WPT conformance, browser parity beyond this fixture, all C11.1/C11.3.3/C11.4 or C11 completion, physical Android device, hardware GPU, or post-compositor iOS pixels를 완료로 표시하지 않는다.

## 계획 실패 관점 검토

[계획의 독립 실패 관점 20개](./c11-root-html-grid-template-areas-plan-review-20.md)는 구현 전 완료했다. 계획 PR diff의 독립 30관점 검토는 [증거 기록](../spec/internal/evidence/c11-root-html-grid-template-areas-precomparison-2026-11-02/plan-pr-diff-review-30.md)에 남긴다.
