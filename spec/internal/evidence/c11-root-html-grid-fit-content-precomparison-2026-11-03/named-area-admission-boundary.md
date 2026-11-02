# Root HTML fit-content named-area admission 경계

## 현재 projection 경계

- [`project_grid_layout_with_math`](../../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs)는 `RuntimeRootHtmlTextPaintV1`에서 explicit track 검증과 named-area 검증을 각각 호출한다. auto-repeat과 named line은 Root HTML profile에서 거부되고, area의 행·열 수는 explicit rows/columns와 일치해야 한다.
- [`validate_root_html_named_area_template`](../../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs)은 현재 `header/side/content` 2×2 map에 columns `[40px, 80px]`와 rows `[20px, 50px]`, 양 축 `[1fr, 2fr]`, 또는 양 축의 직접 px/percentage 2-track profile을 허용한다.
- Chrome 사전 기준의 `"fixed fit"` 1×2 map, columns `20px fit-content(40px)`, rows `60px`는 그 profile에 포함되지 않는다.

## 현재 default-route semantic 경계

- [`validate_root_html_named_area_candidate`](../../../../crates/spinon-style-to-layout/src/projection/style_values/root_html_named_area.rs)은 template area가 있으면 visible Grid container 하나와 정확히 `120px × 70px` 크기를 요구한다. min/max는 기본값이어야 하며 gap은 기존 Root HTML gap validator를 통과해야 한다.
- 직계 자식은 whitespace-only text 또는 비어 있는 visible in-flow HTML `div`만 가능하다. 의미 있는 텍스트, 숨김 자식, nested child, 비-div, 다른 Grid item owner는 거부한다.
- 현재 semantic gate는 `header`, `side`, `content` 세 area 이름에 대해 각각 하나씩 정확히 세 item을 요구한다. 따라서 임의 area name, 다른 area 수, `30px × 20px` scene은 projection을 통과해도 default Root HTML route에서 실패한다.
- [`computed_grid_template_areas`](../../../../crates/spinon-style/src/stylo_dom/cascade/incremental/grid_areas.rs)는 Stylo가 계산한 rectangular map을 typed row/column dimensions와 area rectangle로 옮긴다. 이 slice는 CSS parser나 serialized stylesheet를 새로 해석하지 않는다.

## 제안하는 제한 일반화

새 named-area fit-content path는 아래 조건을 모두 만족할 때만 추가한다.

1. parsed area map의 두 dimensions가 explicit row/column track 수와 정확히 같다.
2. 두 축의 모든 track은 finite non-negative direct px 또는 `fit-content(finite non-negative px)`이고, explicit `fit-content(px)`가 적어도 하나 있다.
3. grid container width와 height는 finite positive px이고 min/max dimensions는 기본값이다. gap은 기존 gap validator를 그대로 쓴다.
4. 각 named area에는 비어 있는 visible in-flow HTML `div` 직계 item 하나만 연결한다. 각 item의 네 computed placement edge는 같은 parsed area name을 가리키고, 각 area name을 정확히 한 번 사용한다. whitespace-only direct text 외의 추가 child, 중복·누락 placement, 다른 item owner는 거부한다.
5. area name, DOM id/class, node ID, child index와 fixture/scene 이름은 성공 분기로 사용하지 않는다.

기존 fixed/fraction/percentage named-area profiles와 120×70 `header/side/content` semantic path는 바꾸지 않는다. `fit-content(%)`, `fit-content(calc())`, direct percentage in the new path, auto sizing, `fr`, `minmax()`, auto-repeat, named lines, implicit `grid-auto-*`, area-created unsized tracks는 계속 거부한다.

이 문서는 projection과 default-route source에서 찾은 admission gap 및 계획 근거다. 해당 구현 PR은 제품 코드와 runtime 결과를 포함하지 않는다. Chrome baseline은 Spinon named-area runtime 동작을 증명하지 않는다.
