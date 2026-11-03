# C11.1 parsed Root HTML Grid의 `align-items`·`justify-items`

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.1 Root HTML Grid 콘텐츠 정렬](./c11-root-html-grid-content-alignment.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 계획 검토 20개 및 구현/runtime 실패 관점 20개 완료; bounded 구현 후보가 현재 branch에 있고 내부 계약·공식 상태 대장에 반영됐다. 구현 PR/diff review/merge는 아직이다. `RuntimeRootHtmlTextPaintV1`의 기존 fixed-track Grid eligibility를 재사용해 Grid container의 `align-items`와 `justify-items`를 그 자식 item의 self-alignment 기본값으로 연결한다. `place-items` shorthand는 Stylo가 산출한 두 longhand computed value를 통해 처리한다. 이 계획만으로 일반 CSS 지원, C11.1/C11 완료, 또는 공식 완료 `72/213` 승격을 주장하지 않는다.

**규범 기준:** [CSS Box Alignment Module Level 3 §4.1 positional alignment, §4.4 overflow alignment, §7.1 `justify-items`, §7.2 `align-items`](https://www.w3.org/TR/css-align-3/#default-alignment) · [CSS Grid Layout Module Level 1 · 2025-03-26 CRD §11.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-align)

**계획 실패 관점 검토:** [서로 다른 경계 20개](./c11-root-html-grid-item-alignment-plan-review-20.md)

**구현 전 비교:** [39-scene Agent Browser 관찰·독립 기대값·화면·hash](../spec/internal/evidence/c11-root-html-grid-item-alignment-precomparison-2026-11-03/README.md)

## 결손과 목표

기본 Root HTML `RuntimeRootHtmlTextPaintV1`에는 고정 track Grid의 `align-content`·`justify-content`를 제한적으로 연결한 경로가 있다. 같은 입력에서 Grid container의 `align-items`·`justify-items`는 computed style projection의 fail-closed 검사를 통과하지 못하고 `GridContainerInput`에 적용되지 않는다. `justify-items:left|right`는 현재 runtime alignment parser에도 없다. CSS Grid item은 grid area 안에서 `align-items`·`justify-items`가 정한 기본 self-alignment를 써야 한다.

이 작업은 기존 `root_html_grid_content_alignment` module이 확인하는 것과 동일한 Grid shape·empty direct `div` item eligibility를 사용한다. 새로운 Grid profile, FFI, public CSS admission 또는 layout algorithm은 만들지 않는다. 허용된 computed alignment value를 layout container에 전달하고 기존 Taffy Grid item alignment를 적용한다.

## 고정 profile

- `RuntimeRootHtmlTextPaintV1`의 parsed Root HTML 경로에서만 적용한다.
- definite finite non-negative fixed-px container width/height와, 비어 있지 않은 finite non-negative fixed-px explicit rows/columns를 요구한다.
- `grid-auto-flow: row`, `grid-auto-rows/columns: auto`, repeat·named line·template area 없음, fixed-px non-negative gap, zero container/item margin·padding·border, visible overflow, static LTR `horizontal-tb`를 요구한다.
- item은 static fixed-size empty HTML `div`, all-auto `grid-*` placement이어야 하며 item 수는 explicit cell capacity 이하여야 한다.
- `align-content`·`justify-content`의 기존 restricted profile은 그대로 유지한다. Grid item의 `align-self`·`justify-self`는 계속 `auto`만 허용한다.
- 적격 Grid 밖의 Root HTML node, dense/implicit/area/repeat Grid 또는 기타 profile에는 비초기 `align-items`·`justify-items`를 새로 열지 않는다. 성공 조건을 만족하지 않는 Grid에서 해당 값을 사용하면 node/property/value 오류로 layout publication 전에 실패해야 한다.

## 값과 변환

`align-items`와 `justify-items`에서 `normal`, `stretch`, `start`, `end`, `flex-start`, `flex-end`, `self-start`, `self-end`, `center`, `safe|unsafe center`, `safe|unsafe end`를 고정한다. CSS Box Alignment §4.1에서 non-flex Grid context의 `flex-start`/`flex-end`는 `start`/`end`처럼 동작한다. 적격 profile의 LTR `horizontal-tb`에서는 `justify-items:left`/`right`를 각각 inline start/end로 변환해 기존 typed positional alignment에 전달한다. `place-items:center`와 `place-items:end start`는 두 computed longhand를 통해 같은 경로를 사용한다.

`baseline`, `first baseline`, `last baseline`, `legacy` 및 parser가 표현하지 못하는 값은 계속 거부한다. `legacy`는 현재 CSS Box Alignment draft에서도 at-risk이며 Grid baseline contribution은 이 slice의 대상이 아니다. CSS-invalid declaration이 cascade에서 initial value로 돌아간 결과와, 유효하지만 이 profile이 받지 않는 값의 runtime rejection을 구분한다. 잘못된 `justify-items`는 오류의 `property` 필드도 `justify-items`여야 한다.

## 사전 비교와 기대값

고정 입력과 원본 관측은 [사전 비교 evidence](../spec/internal/evidence/c11-root-html-grid-item-alignment-precomparison-2026-11-03/README.md)에 둔다. Agent Browser의 HeadlessChrome 156, DPR 1, 1280×633 viewport에서 39개 Grid scene·42개 item을 관찰했다. `align-items`·`justify-items` positional values, safe/unsafe positive·overflow controls, `place-items`, 기존 content distribution과의 결합을 포함한다.

독립 기대값은 40×30px cell과 16×10px fixed item의 axis free space에서 계산했다. Center는 inline +12px/block +10px, end는 +24px/+20px다. 50px/40px 또는 40px/30px cell보다 큰 item에서 safe center/end는 start origin에 두고, unsafe center/end는 각각 -5px/-10px를 허용한다. Combined control은 기존 `space-evenly`/`space-around` track origin에 item center offset을 더한다. 독립 JSON의 전체 frame은 Chrome 관찰과 39/39 scene에서 일치했다. Chrome은 교차 관찰이며 Spinon 실행 또는 WPT 적합성 근거가 아니다.

## 구현·검증 순서

1. **완료:** 계획 review 20개를 코드 변경 전에 고정했다. `parse_runtime_align_items`, `justify-items` 전달, `reject_grid_alignment`, existing Root HTML eligibility를 함께 읽고 변경 지점을 좁혔다.
2. **완료:** `justify-items`를 `align-items`와 구분된 property name으로 parse하는 helper를 추가했다. LTR Grid `left/right`는 start/end에 매핑하고 unsupported 값 오류에 정확한 property/node를 보존한다.
3. **완료:** 기존 Root HTML Grid eligibility module을 `align-items`·`justify-items`와 `place-items` computed output까지 확장했다. Eligible container 외 `justify-items`, child self alignment, unsupported Grid geometry는 거부한다.
4. **완료:** Style projection checks cover positional/self/overflow values, left/right, `place-items`, baseline/`safe start`, wrong shape, item self-alignment rejection 및 property attribution. Root HTML projection 38/38, parser 1/1, full runtime fixture 39 scene·42 item, baseline failure atomicity, 기존 content-alignment runtime regression 통과.
5. **완료:** Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 기본 앱 경로를 같은 fixture로 빌드·실행했다. 두 앱에서 `layout=ready boxes=83` 및 draw/present를 기록했다. Captures는 visual evidence며 PixelCopy/Metal drawable sample은 수집하지 않았다.
6. **완료:** Rust focused tests, `cargo fmt --all -- --check`, scoped all-target Clippy 및 두 simulator build/run 결과를 evidence에 보존했다. 일반/default product artifact에는 fixture-only ABI를 추가하지 않았다.
7. **부분 완료:** [구현/runtime 검토 20개](../spec/internal/evidence/c11-root-html-grid-item-alignment-runtime-2026-11-04/implementation-runtime-review-20.md)를 계획 검토와 분리해 기록했다. PR diff review는 구현 PR 작성 뒤 별도 20개로 검토한다.
8. **부분 완료:** 내부 계약 0059와 공식 `spec/STATUS.md`에 bounded candidate를 반영했다. PR diff review와 PR 본문 screenshot links를 추가한 뒤 merge한다. Merge 후 Tailnet preview hash를 확인하고 Agent Browser로 검증한 다음 browser session/process를 종료한다.

## 완료 조건

- 39 scene·42 item의 fixed expected frames와 overflow alignment 결과가 독립 계산 및 pinned Chrome 156 관찰과 맞는다.
- `align-items`, `justify-items`, `place-items`가 적격 default Root HTML Grid path에서만 적용되고, `left/right`, axis, safety, error property, unsupported profile 경계가 test로 고정된다.
- 기존 `align-content`/`justify-content`, CSS cascade, item auto-size/default behavior, snapshot revision, render publication 및 failure atomicity가 회귀하지 않는다.
- 최종 source Android/iOS simulator builds·runtime, 적용 가능한 surface reads, formatter/Clippy, 구현/runtime review 20개 및 PR diff review 20개가 근거와 함께 통과한다.
- C11.1/C11과 공식 완료 수 `72/213`은 별도 승격 조건을 만족하기 전까지 그대로 둔다.

## 미포함

일반 author CSS/WPT 적합성, arbitrary Grid item sizing/auto-size stretch, `align-self`·`justify-self`, baseline groups, `justify-items:legacy`, writing mode/RTL, implicit tracks, named areas/lines, repeat, dense placement, auto-placement 변경, 실기기/hardware GPU 및 compositor 후 pixel parity는 이 slice로 지원 또는 검증하지 않는다.
