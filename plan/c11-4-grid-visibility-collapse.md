# C11.4 · Grid item `visibility:collapse` runtime slice 계획

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 제한 fixture 구현과 Android/iOS Simulator 검증을 마쳤다. 일반 CSS admission, 전체 WPT와 상위 C11.4 게이트는 미완료이며 공식 완료 `72/213`을 유지한다.

**사전 비교:** [고정 CSS 및 Agent Browser 증거](../spec/internal/evidence/c11-4-grid-visibility-collapse-precomparison-2026-10-29/README.md) · [계획 실패 관점 20개](./c11-4-grid-visibility-collapse-plan-review-20.md) · [계획 PR 변경 검토 20개](./c11-4-grid-visibility-collapse-pr-diff-review-20.md)

## 목적

현재 C11.4 Grid profile은 author `visibility:collapse`를 거부한다. [CSS Display Level 3 §4](https://www.w3.org/TR/css-display-3/#visibility)는 tables/flex처럼 별도 동작을 정의한 formatting context를 제외하면 `collapse`를 invisible `hidden`과 같이 처리한다. [CSS Grid Level 1 2025 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)는 Grid item에 대한 별도 collapse sizing 절차를 정의하지 않으므로 Grid item은 hidden과 같은 layout effect를 유지해야 한다. Browser가 관측한 computed value는 여전히 `collapse`이고, Flexbox의 strut/re-layout 알고리즘은 이 Grid profile의 계산에 적용하지 않는다.

이 slice는 현재 opt-in `C11.4` Grid fixture path에서 직접 배치한 Grid item의 `visibility:collapse`를 허용하고, same-document `hidden` 대조군과 used track/frame/hit/render 출력을 비교한다. Cascade snapshot에는 computed `collapse`를 보존한다. Grid 전용 layout/render 경계에서 hidden과 같은 sizing 및 invisibility를 적용하며 CSS 값 자체를 `hidden`으로 바꾸지 않는다. 일반 CSS admission, 다른 Grid profile 및 Flexbox behavior를 확장하지 않는다.

## 고정 입력과 기준

입력과 10-case Chrome matrix는 [사전 비교 fixture](../tests/fixtures/css/c11/grid-visibility-collapse-c114-precomparison.html), [Agent Browser output](../spec/internal/evidence/c11-4-grid-visibility-collapse-precomparison-2026-10-29/chrome-agent-browser-observation.json), [독립 expected values](../spec/internal/evidence/c11-4-grid-visibility-collapse-precomparison-2026-10-29/expected.json)에 고정한다. 기준은 Agent Browser 0.31.1의 HeadlessChrome 156, viewport `1280×633`, DPR 1이다.

각 축의 root는 sizing 축 80px, 반대 축 20px이고 tracks는 두 개의 `minmax(auto,max-content)`, gap/edge는 0이다. Span-2 item의 child contribution은 100px이다. Row visible-descendant case에서는 visible 8px child와 flow content 92px를 합해 contribution을 100px로 맞춘다.

| Axis | Cases | Root | Expected tracks | Item | Expected visibility result |
| --- | --- | --- | --- | --- | --- |
| Columns | `collapse-column`, `hidden-column` | `80×20px` | Columns `[50,50]`, Rows `[20]` | `100×20px` | computed collapse/hidden을 유지하고 paint/hit는 hidden |
| Columns | `display-none-column` | `80×20px` | Columns `[0,0]`, Rows `[20]` | `0×0px` | Grid item 기여가 없어짐 |
| Columns | `collapse-visible-descendant-column` | `80×20px` | Columns `[50,50]`, Rows `[20]` | `100×20px` | 8×8px visible child는 paint/hit 가능 |
| Rows | `collapse-row`, `hidden-row` | `20×80px` | Columns `[20]`, Rows `[50,50]` | `20×100px` | computed collapse/hidden을 유지하고 paint/hit는 hidden |
| Rows | `display-none-row` | `20×80px` | Columns `[20]`, Rows `[0,0]` | `0×0px` | Grid item 기여가 없어짐 |
| Rows | `collapse-visible-descendant-row` | `20×80px` | Columns `[20]`, Rows `[50,50]` | `20×100px` | 8×8px visible child는 paint/hit 가능 |
| Columns | visible-column positive control | 80×20px | Columns [50,50], Rows [20] | 100×20px | overflow:visible에서 item red content가 root 오른쪽 1px 밖 (81,10)에 paint/hit 가능 |
| Rows | visible-row positive control | 20×80px | Columns [20], Rows [50,50] | 20×100px | overflow:visible에서 item red content가 root 아래쪽 1px 밖 (10,81)에 paint/hit 가능 |

Chrome observation은 독립 expected를 대체하지 않는다. 2025 CRD의 track sizing calculation과 CSS Display visibility fallback에서 수치를 계산하고, Chrome used track/frame/hit 결과는 browser cross-check로 사용한다. 이 작업은 WPT harness의 Spinon pass를 주장하지 않는다.

표면 기준은 [expected JSON의 14 screenshot samples](../spec/internal/evidence/c11-4-grid-visibility-collapse-precomparison-2026-10-29/expected.json)에 고정한다. collapse/hidden red content 내부 표본은 root background #f0f0f0, visible descendant는 (4,4) 기준 blue #0000ff, collapse/hidden overflow는 root-local (81,10) 또는 (10,81)의 body white #ffffff이어야 한다. 양성 대조군은 같은 경계 위치에서 red #ff0000을 보여야 한다. Simulator에서는 view-to-surface 변환과 canvas baseline을 실행 manifest에 기록하고, hidden/collapse가 overflow content를 그리지 않는 것과 positive control이 그리는 것을 함께 대조한다.

## 구현 계약과 검증

1. C11.4 fixture-test profile에서만 computed `visibility:collapse`를 받는다. Author CSS gate는 root `visibility:visible`, direct target item의 exact computed `collapse` 및 child의 `visible`/inherited `collapse`를 함께 확인한다. Missing, extra item, wrong node/property, changed value와 unsupported profile은 기존처럼 typed error로 거부한다.
2. Computed snapshot에는 CSS `collapse`를 그대로 보존한다. Grid item sizing에서는 `hidden`과 동일하게 DOM item, item automatic minimum 및 min/max-content contribution을 유지한다. `display:none` 대조군과 같이 취급하지 않는다. 다른 Grid profile에는 admission을 넓히지 않는다.
3. Rendering/hit path에서는 `collapse` item을 `hidden`과 동일하게 paint하지 않고 hit target에서도 제외한다. Descendant의 computed `visibility:visible`은 다시 표시·hit-test 가능해야 한다. CSS computed value mutation이나 subtree 전체의 무조건 숨김으로 이 의미를 대체하지 않는다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 양축 10 scene을 각각 실행한다. Scene별 computed value, tracks, item/child frame, contribution source/value, layout/trace revision, draw/present를 엄격히 검증한다. Screenshot 및 surface readback에서는 collapse/hidden item 위 root background, visible descendant, root boundary 안팎의 content와 visible positive control을 고정 root-local 좌표에서 검사한다. `display:none` 대조군은 tracks `[0,0]`으로 track-sizing 차이를 별도로 확인한다.
5. Source-order/revision mismatch, unsupported node tree/style, `collapse`가 다른 node로 이동, `display:none`으로 바뀐 입력, visible-child omission/duplication, row/column route mismatch는 부분 output 없이 거부한다. Feature-off/default artifact에는 fixture route marker/symbol이 없어야 하며 fixture-test positive control에는 있어야 한다.
6. 관련 CSS cascade/layout/runtime/FFI tests, `cargo fmt --all -- --check`, targeted Clippy, fixture-module boundary, Android fixture build, iOS Simulator build/run을 최종 source revision에서 실행한다. Browser precomparison과 Simulator result를 혼동하지 않는다. Agent Browser와 fixture app을 실행 후 종료하고 프로세스 부재를 확인한다.
7. 구현 뒤 새 실패 관점 20개로 implementation/runtime을 검토한다. PR diff는 계획 리뷰와 implementation 리뷰에 재사용하지 않는 20개 관점으로 검토한다. PR 본문에는 Android/iOS Simulator screenshot을 각각 링크하고, CSS Display/Grid 근거 및 미지원 범위를 적는다.
8. [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md), C11.4 parent plan, `spec/STATUS.md` 및 공개 roadmap에 제한 profile과 evidence를 동기화한다. C11.4/C11 체크박스와 공식 `72/213`은 변경하지 않는다.

## 실패·범위 경계

CSS Grid에서는 `visibility:collapse`가 `hidden`과 같은 layout contribution을 유지해야 하지만, paint/hit 결과는 hidden이어야 한다. 이 가운데 하나라도 다르면 route를 통과시키지 말고 실패 layer를 기록한다. CSS Flexbox는 별도 strut sizing/restart 규칙을 정의하므로 이번 변경의 shared mapping이 Flex layout을 바꾸면 실패다. Table row/column collapse, clipping/scrolling, scrollbar, generic HTML/CSS, public CSS admission, WPT 전체, 실기기/hardware GPU, 전체 C11.4 완료는 범위 밖이다.

## 2026-10-29 구현·runtime 결과

고정 10 scene을 기존 opt-in C11.4 fixture route에 연결했다. Computed `collapse`는 보존하고 Grid sizing에서는 `hidden`과 같은 contribution을 유지하며 render/hit 목록에서는 item을 제외한다. `display:none`은 축별 `[0,0]` tracks로 분리하고 visible descendant와 visible overflow controls를 둔다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 10/10 scene 및 14/14 surface sample이 통과했다. Android는 software AVD, iOS Metal readback은 compositor 제출 전이다. 최종 [실행 근거](../spec/internal/evidence/c11-4-grid-visibility-collapse-runtime-2026-10-29/README.md), [구현/runtime 검토 20개](../spec/internal/evidence/c11-4-grid-visibility-collapse-runtime-2026-10-29/implementation-runtime-review-20.md), [static archive 경계](../spec/internal/evidence/c11-4-grid-visibility-collapse-runtime-2026-10-29/artifact-boundary/README.md)를 따른다. C11.4와 전체 C11은 미완료다.
