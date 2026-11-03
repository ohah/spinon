# C11.1 parsed Root HTML numeric Grid placement 제한 계획

**상위 계획:** [C11 CSS Grid](./c11-css-grid.md) · [기존 C11.2.1 numeric placement profile](./c11-2-1-explicit-grid-placement.md) · [공식 상태](../spec/STATUS.md#c11-css-grid) · [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 제한 구현 PR [#1137](https://github.com/ohah/spinon/pull/1137)이 2026-11-03T22:48:45Z Rebase and merge됐다(merge SHA `6af14f69512c0c73d890efc728751e0ca342f9c0`). 코드·focused Rust tests·scoped Clippy/rustfmt·Chrome 비교·Android/iOS Simulator 실행 근거와 구현/runtime 및 PR diff 검토는 [runtime evidence](../spec/internal/evidence/c11-root-html-grid-explicit-placement-runtime-2026-11-04/README.md)에 기록했다. 이 bounded slice는 C11/C11.1 체크나 공식 완료 수 `72/213`을 바꾸지 않는다. [구현/runtime 검토 20개](../spec/internal/evidence/c11-root-html-grid-explicit-placement-runtime-2026-11-04/implementation-runtime-review-20.md) · [PR diff 검토 30개](../spec/internal/evidence/c11-root-html-grid-explicit-placement-runtime-2026-11-04/pr-diff-review-30.md)

## 확인한 결손

기본 parsed Root HTML cascade는 Stylo가 계산한 `grid-row`·`grid-column`·`grid-area` shorthand의 typed longhand placement를 이미 `grid_item_placement`로 보존한다. 현재 `root_html_named_area::validate_root_html_named_area_candidate`는 no-area row-flow Grid의 non-auto placement를 거부하고, Style→Layout projection도 named-area·column-flow·dense Root HTML route에만 placement를 투영한다. 따라서 numeric placement를 가진 기본 no-area Grid는 현재 fail-closed되며, C11.2.1에서 이미 구현한 numeric subset이 기본 parsed Root HTML 앱 경로로 연결되지 않았다.

C11.2.1은 numeric line/span 변환, reversed endpoint normalization, shorthand의 Stylo longhand 결과와 explicit track 경계 검사를 `RuntimeBlockFlexGridPlacementV1` 및 `GridPlacementMode::ExplicitOnly`로 이미 구현했다. 이 계획은 그 엔진을 Root HTML에 재사용한다. 새 shorthand parser나 두 번째 numeric placement algorithm은 만들지 않는다.

## 목표와 admission

`RuntimeRootHtmlTextPaintV1`의 각 no-area, 기본 `grid-auto-flow:row` Grid에서 모든 layout-participating item이 양 축 모두 definite numeric line/span placement를 갖고 현재 explicit rows/columns 안에 놓일 때 기존 `project_grid_item_placement`와 `ExplicitOnly` validator를 사용한다.

| 입력 | 예상 처리 |
| --- | --- |
| visible item 전부 computed placement가 `auto` | 기존 기본 Root HTML auto-placement 경로를 그대로 유지 |
| 한 Grid의 모든 visible item이 양 축에서 numeric definite placement | 기존 C11.2.1 변환을 적용하고 explicit track 범위 안에서 성공 |
| 일부 item만 definite, 부분 축만 definite, named line, implicit track 범위 요구 | Grid/layout scene 전체를 실패 처리하고 부분 결과를 publish하지 않음 |
| `display:none` child의 placement | visible Grid item이 아니므로 placement 검증에서 제외 |
| 직접 anonymous text/replaced item이 auto placement를 요구 | auto와 explicit을 혼합하지 않고 해당 scene을 실패 처리 |
| named-area, dense, column-flow Grid | 해당 기존 route의 소유권과 검증을 유지; 이 계획의 no-area explicit route에서 처리하지 않음 |
| explicit placement와 기존 C11.5 item/content alignment 조건이 함께 있는 Grid | placement mode만 explicit-only로 바꾸고 이미 계산된 alignment values를 유지; 해당 교차 scene을 확인 |

Route admission은 Grid owner별로 결정한다. 한 문서의 explicit Grid가 all-auto sibling Grid를 `ExplicitOnly`로 바꾸거나, 한 Grid의 declaration이 다른 Grid item의 placement를 가져오면 안 된다. visible grid item의 전체 집합은 HostDocument layout tree에서 확인한다. CSSOM element 목록만으로 anonymous text item을 생략하지 않는다.

기존 Root HTML track projection과 그 허용 목록을 유지한다. 새로운 track grammar, implicit track 생성, auto-repeat, template area/line-name 해석, arbitrary CSS admission은 포함하지 않는다. numeric placement는 모든 축에서 현재 authored explicit track 범위 안에 있어야 한다. CSS shorthand 결과는 Stylo computed longhand에서 가져오며 자체 문자열 parser를 추가하지 않는다.

## 사전 비교 자료

[Chrome 사전 비교 fixture](../tests/fixtures/css/c11/root-html-grid-explicit-placement-precomparison.html)는 Agent Browser `0.31.1`의 HeadlessChrome 156에서 열한 장면을 수집했다. 관측 JSON과 규범에서 별도 계산한 기대값을 분리하고, 기대값 비교 verifier에서 11/11 scene, 23 item frame, computed longhand, hidden/text control, overlap hit-test 및 C11.5 self-alignment 교차 장면을 확인했다. 화면과 입력 hash는 [사전 비교 README](../spec/internal/evidence/c11-root-html-grid-explicit-placement-precomparison-2026-11-04/README.md)에 있다.

이 기준은 CSS Grid Level 2의 [numeric line/span placement](https://www.w3.org/TR/css-grid-2/#placement-shorthands), [placement conflict handling](https://www.w3.org/TR/css-grid-2/#grid-placement-conflict-handling), [auto-placement](https://www.w3.org/TR/css-grid-2/#auto-placement-algo)와 기존 C11.2.1 제한을 사용한다. 브라우저 관측은 규범상의 전체 CSS 지원이나 Spinon 결과로 승격하지 않는다.

## 구현 및 검증 순서

1. Cascade snapshot의 owner, typed Grid style, `grid_item_placement`, visible layout children을 대조한다. `root_html_named_area::validate_root_html_named_area_candidate`의 현재 no-area 거부는 선택된 explicit owner에서만 기존 C11.2.1 validator로 넘기도록 바꾼다. named-area, dense, column-flow의 별도 검증은 우회하지 않는다.
2. no-area 기본 row-flow Grid에 authored non-auto placement가 있는지 computed typed value로 판정한다. 해당 owner의 visible grid item 전체를 기존 `project_grid_item_placement`로 투영하고, 성공 후보에만 기존 `GridPlacementMode::ExplicitOnly`를 지정한다. C11.5 alignment가 먼저 지정한 Root HTML mode를 덮을 때도 typed content/item alignment 값은 유지한다. Element ID, DOM index, class, author property 문자열에 따라 frame이나 mode를 선택하지 않는다.
3. 혼합 auto/explicit, partial-axis auto, named value, overflowed integer, implicit boundary 요구, invalid span, nested/hidden/replaced/text 경계를 Rust projection/layout tests에서 확인한다. invalid scene에는 partial grid item/frame, 새 render revision 또는 stale snapshot을 반환하지 않는다.
4. Chrome 156 사전 비교의 양수·음수 line, reversed endpoints, `span`, shorthand cascade winner, overlap, hidden child와 explicit+`align-self`/`justify-self` 교차를 Root HTML projection/runtime fixture에 연결한다. 기존 all-auto Grid, C11.1 track/gap/areas, C11.5 alignment/auto-size scene을 회귀 대조한다.
5. 기본 `RuntimeSession::new_runtime_gpu_with_root_html()` 앱 경로에서 같은 HTML source를 실행한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 scene·source revision·node frame·draw/present를 기록하고 Chrome 독립 기대값과 대조한다. 가능한 surface readback은 별도 PixelCopy/Metal 표본으로 보고하며 draw/present나 캡처 화면을 pixel parity로 표현하지 않는다.
6. `cargo fmt --all -- --check`, 변경 crate의 focused tests 및 scoped Clippy를 실행한다. 전체 suite 결과는 기존 실패/ignored와 분리하며, 실행하지 않은 게이트는 통과로 보고하지 않는다. 새 fixture ABI를 만들지 않으며 기본/product build에 구현 전용 payload가 들어가지 않는지 diff 경계를 확인한다.
7. 계획 내용은 이 계획 파일의 독립 실패 관점 20개로 검토한다. 기능 구현 뒤 코드/runtime/evidence를 새로운 20개 관점으로 검토하고, 실제 PR diff 전체는 프로젝트 규칙에 따라 별도의 30개 적대 관점으로 검토한다. 결함을 고치면 해당 관점을 다시 실행한다.
8. plan PR과 구현 PR은 별도 PR로 만들고 Rebase and merge한다. 각 PR 본문에 화면을 첨부할 수 있으면 이 fixture/runtime 화면을 embed하고, PR check rollup 및 merge state를 직접 확인한다.
9. 구현이 확인된 뒤 내부 계약 0059, `spec/STATUS.md`, 상위 C11 plan과 공개 roadmap source를 실제 제한 범위로 갱신한다. `C11`, `C11.3`, `C11.3.3`, `C11.4` 상태와 공식 `72/213`은 그대로 둔다. Tailnet preview는 merge된 source hash와 serve root를 확인한 뒤 Agent Browser로 확인하고 해당 browser session/process를 종료한다.

## 완료 조건과 제외 범위

- 기본 Root HTML 경로에서 all-auto scene은 이전 geometry와 route를 유지한다.
- 모든 visible item이 양 축에서 explicit-grid 내부의 definite numeric placement를 가지면 CSSOM computed lines와 final node frames가 독립 기대값에 `0.5 CSS px` 이내로 일치한다.
- line/span normalization, reversed endpoints, shorthand cascade, overlap/source paint order와 hidden-item 제외가 검증된다.
- explicit placement가 기존 C11.5 self-alignment values와 함께 계산될 때 alignment offset/size가 기존 기대값을 유지한다.
- mixed/partial auto, anonymous auto item, named lines, implicit track 요구와 line/arithmetic overflow는 진단과 함께 전체 scene을 실패시킨다. invalid scene은 partial 또는 stale `completed` snapshot을 노출하지 않고, 같은 RuntimeSession에서 유효한 media environment로 돌아오면 정상 layout을 다시 publish한다.
- all-auto, named-area, sparse/dense/column-flow, C11.4 text, C11.5 item alignment 경로의 기존 검증 결과를 회귀시키지 않는다.
- Android API 37 ARM64 AVD와 iOS Simulator의 동일 input 실행 근거, build/source hashes, 화면과 renderer/readback 경계를 보존한다.

이 slice는 C11.2.1 전체 profile 추가, CSS Grid 일반 지원, implicit tracks, named lines/areas placement, mixed auto placement, auto-repeat, track sizing 확장, absolute/replaced/content item, WPT 전체 적합성, 실제 기기·hardware GPU, iOS compositor 이후 pixel parity 또는 C11 완료를 뜻하지 않는다. 내부 contract 숫자 버전은 `0.1.0`을 유지한다.

## 계획 실패 관점 검토

[계획 자체의 서로 다른 실패 관점 20개](./c11-root-html-grid-explicit-placement-plan-review-20.md)를 구현 PR과 별도로 기록한다. plan PR diff는 실제 변경 파일·hash·상태·화면 link를 대상으로 별도 20개 관점으로 검토한다.
