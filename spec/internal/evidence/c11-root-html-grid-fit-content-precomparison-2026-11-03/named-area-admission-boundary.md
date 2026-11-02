# Root HTML fit-content named-area admission 경계

## 현재 코드 경계

- [`project_grid_layout_with_math`](../../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs)는 `RuntimeRootHtmlTextPaintV1`에서 explicit track 검증과 named-area 검증을 각각 호출한다. auto-repeat은 Root HTML profile에서 이미 거부되며 area의 행·열 수는 explicit rows/columns와 일치해야 한다.
- [`validate_root_html_named_area_template`](../../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs)은 현재 header/side/content 2×2 map에 특정 fixed-px dimensions, 양 축 1fr/2fr, 또는 두 축의 2-track px/percentage 패턴을 허용한다.
- Chrome 사전 기준의 `"fixed fit"` 1×2 map, columns `20px fit-content(40px)`, rows `60px`는 위 조건에 속하지 않는다. 따라서 이를 “기존 Root HTML named-area 경로”로 서술하면 실제 code path와 맞지 않는다.

## 제안하는 제한 일반화

이 feature slice는 새 parser, runtime profile, scene/ID allowlist 또는 fixture ABI를 만들지 않는다. `grid-template-areas`의 parsed rectangular map이 explicit track list와 각 축에서 정확히 dimension-match하고, 두 축의 모든 track이 finite non-negative direct px 또는 `fit-content(finite non-negative px)`이며 적어도 하나의 fit-content track을 포함하는 경우를 default Root HTML profile에 허용한다. Area name은 임의의 CSS-valid identifier로 유지한다.

기존 header/side/content fixed/fraction/percentage profiles는 그대로 둔다. Percentage/calc fit-content argument, auto sizing, `fr`, `minmax()`, auto-repeat, named lines, implicit `grid-auto-*` 및 area-created unsized tracks는 이 경로에서 지원하지 않는다. 테스트는 Chrome의 `"fixed fit"` 장면과 이름을 바꾼 대조 map을 typed projection에 사용하고, default route simulator fixture에는 Chrome baseline map을 사용한다.

이 문서는 현재 source를 읽어 찾은 admission gap과 계획 변경의 근거다. Product code는 아직 변경하지 않았으며 Chrome baseline은 Spinon named-area runtime 동작을 증명하지 않는다.
