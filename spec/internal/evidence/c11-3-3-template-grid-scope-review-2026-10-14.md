# C11.3.3 TemplateGrid mode 범위 후속 검토 · 2026-10-14

이 검토는 template-area item의 auto sizing을 허용하면서 공유 `RuntimeBlockFlexGridTemplateV1` 경로의 C11.3.1/.2 repeat·named-line scene으로 동작이 번지지 않는지 확인한다. 확인 대상은 style→layout profile 선택, layout item 제약, 최신 Android/iOS 실행이다. `PASS · 코드 확인`은 별도의 runtime 입력을 실행했다는 뜻이 아니다.

| # | 독립 실패 관점 | 결과와 근거 |
| ---: | --- | --- |
| 1 | `template_areas`가 설정되어 있는데도 area item에 `TemplateGrid` mode가 전달되지 않는다 | **PASS · Rust test.** `template_area_profile_selects_template_grid_sizing_mode`가 area metadata가 있는 projection을 검사한다. |
| 2 | 일반 template profile에 area metadata와 alias가 없는데 auto item sizing mode로 바뀐다 | **PASS · Rust test.** `template_profile_projects_n_plus_one_line_names_and_dense_grid_contract`는 `DenseAutoPlacement`를 확인한다. |
| 3 | 양 축의 같은 이름 `-start`/`-end` pair가 implicit area mode를 활성화하지 않는다 | **PASS · Rust test.** `paired_area_edge_aliases_on_both_axes_select_template_grid_sizing_mode`가 pair를 검사한다. |
| 4 | alias pair가 한 축에만 있는데 area mode가 선택된다 | **PASS · negative test.** 같은 projection test가 column-only pair를 dense mode로 유지한다. |
| 5 | row에만 pair가 있을 때 false positive가 난다 | **PASS · 코드 확인.** 후보 이름은 양 축 각각의 start와 end 존재를 요구한다. row-only 전용 unit 입력은 없다. |
| 6 | column pair에서 `-start`만 있어 implicit area로 오인한다 | **PASS · 코드 확인.** `has_start && has_end`가 거짓이면 pair가 아니다. 독립 input test는 없다. |
| 7 | column pair에서 `-end`만 있어 implicit area로 오인한다 | **PASS · 코드 확인.** 동일한 완전 pair 조건으로 차단된다. 독립 input test는 없다. |
| 8 | row pair에서 start 또는 end가 빠져 있는데 area mode가 선택된다 | **PASS · 코드 확인.** row 축에도 같은 완전 pair 조건이 적용된다. 독립 input test는 없다. |
| 9 | column의 `foo`와 row의 `bar`가 이름이 다른데 하나의 area로 합쳐진다 | **PASS · 코드 확인.** 양 축 모두 같은 captured base name을 검사한다. 이름 불일치 unit 입력은 없다. |
| 10 | `foo-started` 같은 유사 접미사가 `foo-start`로 잘못 인식된다 | **PASS · 코드 확인.** suffix 제거는 정확한 `-start`/`-end`에서만 수행한다. 별도 suffix negative input은 없다. |
| 11 | `header`, `content` 같은 보통 named line만 있어 auto sizing이 켜진다 | **PASS · projection test.** area alias가 없는 named-line template은 dense mode로 남는다. |
| 12 | area metadata가 없는 repeat-only 입력이 C11.3.1 template mode에서 새 item sizing을 얻는다 | **PASS · projection boundary.** template profile에는 guard가 있고 다른 profile branch는 변경하지 않았다. runtime matrix는 C11.3.3 대상이라 C11.3.1 전체 matrix를 다시 실행한 것은 아니다. |
| 13 | C11.3.2 named-line placement mode와 auto-track projection이 바뀐다 | **PASS · regression.** 기존 projection test와 전체 Rust workspace test가 통과했다. 최신 C11.3.3 AVD matrix는 26 scenes다. |
| 14 | `RuntimeBlockFlexGridTemplateV1` 이외의 profile이 template area mode로 오염된다 | **PASS · 코드 확인.** mode 선택 match arm이 해당 profile로 제한된다. 다른 모든 profile의 전체 모바일 matrix를 이번 단계에서 재실행하지 않았다. |
| 15 | dense auto-placement item의 intrinsic `auto` 크기가 area item 수정으로 허용된다 | **PASS · Rust test.** `auto_placed_intrinsic_item_size_is_rejected_and_next_layout_recovers`와 C11.3.3 runtime matrix가 통과했다. |
| 16 | dense auto-placement에서 `grid-auto-rows/columns` 계산이 사라진다 | **PASS · projection/runtime regression.** auto track values는 기존 projection 경로를 유지하고 전체 Rust workspace test 및 26-scene matrix가 통과했다. |
| 17 | definite area item의 `width:auto` 또는 `height:auto`가 계속 거부된다 | **PASS · Rust test.** `template_area_mode_stretches_definite_auto_sized_items_without_widening_dense_mode`가 definite item의 stretch를 검사한다. |
| 18 | auto-sized item 허용이 auto-placed area 밖 item으로 번진다 | **PASS · Rust test.** 같은 layout test가 dense mode의 기존 fixed-size 제한도 확인한다. |
| 19 | 0 min-size 예외가 positive min 또는 max constraint까지 허용한다 | **PASS · Rust negative controls.** definite `TemplateGrid`의 0 `min-width`·`min-height`는 성공한다. 추가한 회귀 loop가 positive `min-width`·`min-height`·`max-width`·`max-height` 각각에서 property가 특정된 `UnsupportedGridItemConstraint`를 요구한다. dense mode의 0 `min-width` 거부도 같은 test가 확인한다. |
| 20 | Android scene, geometry 또는 surface oracle이 source 변경 뒤 달라진다 | **PASS · 최신 source 실행.** API 37 AVD 26/26 scene, visible frame 61개 오차 0 CSS px, PixelCopy 2/2 exact; iOS Simulator success/rejection 각 1 scene, success 4 frame, Metal 2/2 exact다. WPT·실기기·hardware GPU는 제외다. |

## 실행 근거와 한계

- projection 160/160, layout 117/117, fixture FFI 2/2, CSS reference 412/412, 전체 Rust workspace 694 passed·4 ignored, workspace Clippy와 rustfmt가 통과했다.
- Android 실행: [API 37 AVD matrix](./c11-3-3-grid-template-areas-runtime-2026-10-14/android-avd-c1133-matrix-20261014T023829118Z-d027504f/README.md). Run `c1133-matrix-20261014T023829118Z-d027504f`, 26 scene·65 node, 19 success·5 cascade fallback·2 expected rejection.
- iOS 실행: [iPhone 17 Pro / iOS 26.2 Simulator](./c11-3-3-grid-template-areas-runtime-2026-10-14/ios-simulator-c1133-ios-simulator-20261014T023918758Z-45918e12/README.md). Metal drawable은 OS compositor 제출 전 표면이다.
- 범위 guard와 관련 layout 검사는 [grid projection](../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs), [layout rules](../../../crates/spinon-layout/src/grid.rs), [layout tests](../../../crates/spinon-layout/src/grid_tests.rs)에 있다.

이 검토는 C11.3.3 전체 CSS Grid 지원이나 Spinon WPT 적합성을 판정하지 않는다. Android 실기기·hardware GPU, iOS 실기기와 compositor 이후 표면은 검증하지 않았다. 별도 PR 30-view review와 병합 관문은 남아 있다.
