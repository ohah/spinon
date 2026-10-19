# PR #368 최종 diff 적대적 검토 30개

**검토 대상:** `origin/main...b95201a5`의 C11.4 양축 auto-repeat 코드, Android/iOS fixture·validator, 실행 근거와 명세 갱신. 계획 20개 및 구현 후 코드/runtime 20개 검토를 재사용하지 않았다.

| # | 실패 경로·불변식 | diff와 독립 근거 | 판정 |
| --- | --- | --- | --- |
| 1 | columns와 rows cascade metadata가 한 축 값을 덮어쓴다. | cascade가 axis별 결과를 그대로 보존하며 `track_sizing_profile_preserves_independent_auto_repeats_on_both_axes`에서 auto-fit/auto-fill과 range를 따로 확인한다. | 통과 |
| 2 | 지원 범위가 아닌 CSS profile에도 양축 허용이 새어 나간다. | `RuntimeBlockFlexGridTrackSizingV1` 조건을 유지하고 그 외 profile은 기존 fail-closed 경로를 사용한다. | 통과 |
| 3 | `grid-template-areas`와 auto-repeat이 새로 허용된다. | cascade와 projection 모두 `template_areas.is_some()`을 거부한다. 기존 unsupported-profile fixture가 회귀 경계를 유지한다. | 통과 |
| 4 | auto-repeat이 없는 single-axis 입력의 typed projection이 변한다. | 별도 `track_sizing_profile_keeps_single_axis_auto_repeat_projection`가 반대 축 `None`, explicit row와 placement mode를 검사한다. | 통과 |
| 5 | typed projection이 columns와 rows의 서로 다른 mode/range를 혼합한다. | `track_sizing_profile_projects_independent_auto_repeat_modes_on_both_axes`가 두 output metadata를 각각 비교한다. | 통과 |
| 6 | layout validator가 두 축의 유효한 auto-repeat을 다시 배타 처리한다. | `auto_repeat_accepts_independent_repeats_on_both_axes`에서 Columns와 Rows stage trace가 모두 생성된다. | 통과 |
| 7 | columns range 오류를 유효한 rows range가 가린다. | `auto_repeat_rejects_an_invalid_first_axis_independently`는 `grid-template-columns` 오류 property와 node ID를 검사한다. | 통과 |
| 8 | rows range 오류를 유효한 columns range가 가린다. | `auto_repeat_rejects_an_invalid_second_axis_independently`는 `grid-template-rows` 오류 property와 node ID를 검사한다. | 통과 |
| 9 | `start == end`의 빈 repeat range가 빈 track slice로 통과한다. | `validate_auto_repeat_axis`의 `start >= end` guard가 slice 전에 실행된다. 이 동등 경계에는 별도 runtime scene이 없다. | 통과 |
| 10 | `end > tracks.len()`가 slice panic을 만든다. | 같은 guard의 `end > tracks.len()` 조건이 slice 전 처리된다. 두 축 개별 오류 regression은 `end=4`, length 3으로 초과 경로를 실행한다. | 통과 |
| 11 | repeated axis line names가 silent drop 또는 잘못된 line mapping을 만든다. | `auto_repeat_rejects_named_lines_through_the_layout_api`가 columns와 rows 각각의 named lines를 거부한다. | 통과 |
| 12 | 기존 placement mode가 adapter 외부에서 주입된 auto-repeat을 실행한다. | `validate_grid_layout`은 auto-repeat profile에서 `GridPlacementMode::TrackSizing`을 먼저 요구한다. | 통과 |
| 13 | percent/negative/non-finite explicit neighbor를 fixed track으로 취급한다. | 기존 `auto_repeat_neighbor_track_is_fixed_px` 검사에서 유한·0 이상 `LengthPx`만 허용한다. 미지원 이웃 grammar fixture는 계속 거부된다. | 통과 |
| 14 | repeated fragment에 definite fixed minimum이 없는 track을 허용한다. | `auto_repeat_track_has_fixed_minimum` 경계가 axis별 validator에서 유지되며 기존 unsupported profile test가 적용된다. | 통과 |
| 15 | list 하나에 두 번째 repeat이나 integer-repeat 혼합을 실수로 허용한다. | cascade의 `auto_repeat_rejects_profiles_and_shapes_outside_the_supported_subset`에 복수 repeat 및 혼합 shape 거부가 남아 있다. | 통과 |
| 16 | 두 축의 repeat count가 공유되어 비대칭 크기에서 동일해진다. | `both-fill-asymmetric`의 Rust expectation은 columns 2회·rows 3회이고 양 Simulator 결과 JSON에서 track vector를 별도 대조했다. | 통과 |
| 17 | 한 축만 사용하는 기존 controls가 matrix에서 빠진다. | `columns-fill-control`과 `rows-fill-control`이 각 방향의 2회 반복과 12×12 item frame을 검사한다. | 통과 |
| 18 | 양축 `auto-fill`이 빈 repeated track을 collapse한다. | `both-fill-square`/`both-fill-asymmetric` 기대값은 두 축의 모든 repeat track을 유지한다. | 통과 |
| 19 | 양축 `auto-fit` empty 상태에서 한 축 track/gutter가 남는다. | `both-fit-empty`는 양축 모두 `[20, 0, 0, 0, 0, 10]`과 collapsed flags를 validator에 비교한다. | 통과 |
| 20 | 단일 점유 item이 반대 축 또는 다른 반복 track까지 활성화한다. | `both-fit-single`은 양축의 한 154px track과 나머지 collapsed track을 각각 검사한다. | 통과 |
| 21 | item이 두 repeated track을 span하지만 둘째 track을 점유 처리하지 않는다. | `both-fit-span-two`는 양축 `[20, 126, 20, 0, 0, 10]`을 비교한다. | 통과 |
| 22 | 두 번째 repeat에 배치된 item이 첫 repeat으로 이동한다. | `both-fit-second-repeat`와 mixed-mode 두 scene이 axis별 track vector 및 `(28,28)`, `(28,109)`, `(109,28)` frame을 검사한다. | 통과 |
| 23 | columns fit / rows fill 혼합에서 fit만 collapse한다. | `columns-fit-rows-fill-empty` 및 second-repeat fixture가 collapsed columns와 유지된 rows를 각각 비교한다. | 통과 |
| 24 | columns fill / rows fit 혼합에서 반대 축 collapse 처리가 반대로 된다. | `columns-fill-rows-fit-empty`와 second-repeat fixture가 유지된 columns와 collapsed rows를 검사한다. | 통과 |
| 25 | collapsed gutter를 두 번 합산하거나 모두 없애 explicit suffix line이 이동한다. | Rust suffix regression과 Chrome 155 Agent Browser probe가 suffix item `(190,190)`을 확인한다. [CSS Grid §7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat) 및 [§10.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#gutters)의 collapsed-gutter 규칙과 대조했다. | 통과 |
| 26 | scene ID 중복·누락이 validator를 통과한다. | capture script는 33개 장면과 ID 중복을 검사하고, diff 대조에서 기존 21개 ID가 유지되며 12개 신규 ID가 추가됐음을 확인했다. 각 launch는 summary·trace·draw/present 및 지정 pixel 표본이 빠지면 실패한다. | 통과 |
| 27 | 신규 scene 추가가 이전 21개 neighbor/minimum-size fixture를 대체한다. | 최신 Android/iOS `result.json`은 기존 21개와 신규 12개 합계 33개를 모두 포함한다. | 통과 |
| 28 | Android allowlist, root/item frame, box count 또는 overflow 표본 중 하나가 누락된다. | Android scene map과 validator를 대조했고 최신 AVD run 33개가 모두 `drawPresented: true`; overflow sample은 4/4 exact다. | 통과 |
| 29 | iOS allowlist/frame 또는 Metal pixel sample이 Android와 다르게 빠진다. | Swift scene map·frame/pixel mapping을 확인했고 최신 iOS run 33개가 모두 `drawPresented: true`; overflow sample은 4/4 exact다. | 통과 |
| 30 | fixture harness가 production artifact·C11.4 완료 주장으로 새어 나오거나 PR screenshot 링크가 깨진다. | static rlib positive/negative sentinel 경계와 unchecked C11.4 status를 확인했다. Agent Browser에서 두 raw PR image 모두 `complete=true`와 양의 intrinsic size로 로드됐고 종료 뒤 process가 없었다. WPT·실기기·hardware GPU·iOS compositor 이후 픽셀은 미검증으로 남았다. | 통과 |

## 결론

이 diff에서 병합을 막는 결함은 찾지 못했다. Review는 고정 profile·scene과 기록된 Simulator 경로에 한정한다. 현재 GitHub required checks와 merge state는 PR 화면/CLI에서 별도 확인한다. Tailnet roadmap은 구현 PR 병합 뒤 별도 변경으로 동기화한다.
