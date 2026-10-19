# C11.4 image ratio transfer 구현 실패 관점 검토 20개

검토 대상은 direct HTML `img` Grid item의 transferred-size suggestion 변경, Taffy adapter, style projection, Android/iOS fixture다. 검토는 2026-10-19에 구현 코드·27개 Chrome 155 observation·Rust oracle·7-scene Simulator 실행을 서로 대조했다. 각 항목은 별도 입력 경계나 실패 경로를 다룬다.

| # | 실패 관점 | 확인 근거와 판정 |
| --- | --- | --- |
| 1 | 열 track에서 반대 축 높이를 ratio로 곱하지 않고 나누거나 다른 축을 읽는가 | `transferred_size_suggestion`의 Inline 경로는 반대 축 preferred height에 ratio를 곱한다. `column-auto-natural-ratio`와 authored-ratio oracle 및 Chrome 관찰이 `8px → 12px`을 확인한다. **통과.** |
| 2 | 행 track에서 반대 축 너비를 ratio로 나누지 않거나 열 경로를 재사용하는가 | Block 경로는 너비를 ratio로 나눈다. `row-auto-natural-ratio`와 authored-ratio oracle 및 Chrome 관찰이 `12px → 8px`을 확인한다. **통과.** |
| 3 | intrinsic natural ratio와 authored `aspect-ratio`가 서로 다른 입력에서 잘못 합쳐지는가 | Natural `auto` image cases와 authored `3 / 2` cases를 열·행에서 따로 비교했다. 두 입력은 같은 기하 결과를 내고 observation에는 computed ratio가 구분되어 있다. **통과.** |
| 4 | `aspect-ratio`가 있는 non-replaced 요소에도 image transferred suggestion을 적용하는가 | Taffy helper의 `is_replaced` early return, `non_replaced_items_keep_their_min_content_suggestion_instead_of_transfer`, Chrome의 `column-non-replaced-ratio-content-size`를 대조했다. **통과.** |
| 5 | 반대 축 preferred size가 auto/indefinite인데도 ratio transfer를 만들어내는가 | helper가 반대 축 preferred size 해석 실패 시 `None`을 반환한다. ratio만 있는 auto/auto intrinsic input은 adapter에서 명시 거부한다. 해당 분기 자체의 별도 Chrome 음성 case는 없다. **코드 경계 통과, 추가 oracle 미실행.** |
| 6 | 지정된 target-axis size보다 transferred suggestion이 우선하는가 | `column-specified-size-precedes-transfer`에서 definite width 10px, height 8px가 10×8로 유지되고 track도 10px이다. **통과.** |
| 7 | 행 sizing에서도 definite target-axis size가 suggestion보다 우선하는가 | `row-specified-size-precedes-transfer`에서 definite height 6px가 유지되어 12×6 item과 6px row가 된다. **통과.** |
| 8 | 열 transfer 전에 반대 축 `max-height` 제한을 적용하지 않는가 | natural/authored `column-max-height-clamp` 관찰은 max-height 4px에서 6×4를 만든다. layout matrix도 해당 값을 확인한다. **통과.** |
| 9 | 열 transfer 전에 반대 축 `min-height` 제한을 적용하지 않는가 | `column-min-height-clamp`는 min-height 12px에서 18×12가 된다. **통과.** |
| 10 | 행 transfer 전에 반대 축 `max-width` 또는 `min-width` 제한을 적용하지 않는가 | `row-max-width-clamp`는 6×4, `row-min-width-clamp`는 18×12로 Chrome과 Rust가 일치한다. **통과.** |
| 11 | 반대 축에서 min이 max보다 큰 입력에서 max를 잘못 우선하는가 | 새 열 경계 `min-height:12px; max-height:4px`는 18×12이고, 최소값이 비율 변환 전에 적용된다. **통과.** |
| 12 | 같은 min-over-max 경계를 행 반대 축에 대칭 적용하지 않는가 | 새 행 경계 `min-width:18px; max-width:6px`는 18×12 item과 12px row를 만든다. **통과.** |
| 13 | 열 target-axis `max-width`가 전이 결과에 적용되지 않는가 | `column-target-max-width-clamp`는 width 9px, height 8px와 9px track을 만든다. **통과.** |
| 14 | 행 target-axis `max-height`가 전이 결과에 적용되지 않는가 | `row-target-max-height-clamp`는 width 12px, height 6px와 6px row를 만든다. **통과.** |
| 15 | target-axis `min-width`가 적용될 때 지정된 반대 축 높이까지 ratio로 변경하는가 | `column-target-min-width-clamp`는 `min-width:18px`에도 definite height 8px를 유지해 18×8이 된다. Android/iOS verifier는 세 pass `Columns → Rows → Columns`를 모두 검사한다. **통과.** |
| 16 | border-box image에 padding/border를 ratio 결과 위에 중복 더하는가 | `border_box_ratio_transfer_keeps_preferred_size_in_border_box_coordinates`와 Chrome observation은 각 방향 1px padding/border에도 12×8 결과를 확인한다. **통과.** |
| 17 | content-box image에서 padding/border를 빼먹거나 ratio 이전에 잘못 포함하는가 | `content_box_ratio_transfer_adds_padding_and_border_after_conversion`와 Chrome observation은 content transfer 뒤 장식 합계가 더해져 16×12가 되는 것을 확인한다. **통과.** |
| 18 | scrollable overflow의 automatic minimum이 transferred suggestion으로 12px까지 커지는가 | `column-authored-ratio-scrollable-overflow-min-zero`는 8px track, 12×8 item overflow를 유지한다. **통과.** |
| 19 | transferred suggestion이 Grid automatic-minimum의 track/span eligibility 및 fixed-track 제한 바깥에서 사용되는가 | helper 호출은 `content_based` automatic-minimum 분기 내부이고 결과에는 기존 fixed-track limit clamp가 이어진다. 기존 span/flexible-track 분기를 유지하는 코드를 확인했다. 새 ratio oracle에는 multi-track span/fixed-limit 교차 조합이 없다. **코드 검토 통과, 교차 oracle은 남음.** |
| 20 | 제한된 profile 밖의 비-image·nested image까지 허용하거나 simulator trace를 덜 검사하는가 | `direct_html_image_ratio_constraints_are_scoped_to_track_sizing_profile`, `ratio_constraints_remain_rejected_for_non_image_grid_items`, `ratio_constraints_remain_rejected_for_nested_html_images`, layout의 non-track rejection test 및 두 앱 verifier를 확인했다. 두 simulator 모두 7/7 frame/trace를 통과했다. **통과.** |

모든 실행 증거와 범위는 [Chrome oracle README](./README.md), [7-scene Simulator README](../c11-4-image-ratio-transfer-simulator-2026-10-19/README.md), 그리고 해당 폴더의 raw log·screenshot·comparison JSON에 있다. 다섯 항목에서 명시한 미검증 조합과 WPT·nested/anonymous item·dynamic replacement는 추가 작업으로 남는다. 이 검토는 C11.4 또는 C11 완료 판정이 아니다.
