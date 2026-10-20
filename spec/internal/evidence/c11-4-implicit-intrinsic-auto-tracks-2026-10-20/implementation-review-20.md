# C11.4 implicit intrinsic auto-track 구현·runtime 적대 검토 20개

검토 대상은 `RuntimeBlockFlexGridTrackSizingV1`에서 `grid-auto-rows`·`grid-auto-columns` 목록을 보존하고 양의 implicit track에 적용하는 변경이다. 코드 경로, 실패 입력, Chrome oracle, Android AVD·iOS Simulator 실행 결과를 서로 다른 관점으로 대조했다. PR diff 검토는 이 문서를 재사용하지 않고 별도 30개 관점으로 수행한다.

## 결과

| # | 실패 관점 | 공격 입력·검사 | 결과와 근거 |
| --- | --- | --- | --- |
| 1 | profile 누수 | 기존 C11.2.2/C11.2.3 및 TemplateGrid에 intrinsic auto-track 목록을 주입 | projection과 layout 양쪽에서 C11.4 외 목록을 거부한다. [`auto_track_tests.rs`](../../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values/auto_track_tests.rs), [`grid_auto_track_sizing_tests.rs`](../../../../crates/spinon-layout/src/grid_auto_track_sizing_tests.rs) |
| 2 | 축·순서 손실 | 서로 다른 row/column 목록을 cascade snapshot에서 읽고 projection 결과 비교 | 양 축 authored 순서와 typed 항목을 각각 보존한다. [`track_sizing.rs`](../../../../crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/track_sizing.rs), [`auto_track_tests.rs`](../../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values/auto_track_tests.rs) |
| 3 | singleton fixed 회귀 | 기존 profile의 단일 고정 px와 C11.4 singleton 목록 비교 | singleton px는 기존 `LengthPx` 경로를 유지하고, 이전 profile의 허용 문법을 바꾸지 않는다. cascade·profile 회귀 테스트 통과. |
| 4 | unsupported/invalid 값 통과 | 빈 목록, 음수 px, 이전 profile의 intrinsic 목록, converter가 표현하지 않는 문법 | 빈 목록·잘못된 크기·비지원 profile 입력을 property와 node가 보존된 오류로 거부한다. `track_sizing_rejects_empty_invalid_and_oversized_auto_track_lists`, `previous_auto_placement_profile_rejects_intrinsic_auto_track_lists`. |
| 5 | CSS math 축 혼동 | row `calc(10% + 5px)`와 column `calc(10% + 5px)`를 동시에 투영 | row는 `GridTemplateRows`, column은 `GridTemplateColumns` math context를 사용한다. [`auto_track_tests.rs`](../../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values/auto_track_tests.rs). |
| 6 | explicit prefix만큼 잘못 건너뜀 | explicit track 뒤 implicit index 0부터 `[10px,20px]` 적용 | 첫 implicit track은 항상 목록 첫 값으로 시작한다. [`expand_tracks`](../../../../crates/spinon-layout/src/grid_auto_placement/dense_column.rs)의 index는 `count - explicit.len()` 기준이다. |
| 7 | cycle 경계 off-by-one | implicit track 3개에 두 항목 목록 적용 | `[10,20,10]` 순서를 확인한다. `track_sizing_cycles_positive_implicit_auto_track_lists_per_axis` 통과. |
| 8 | 양의 explicit line 범위 오류 | explicit grid 뒤 line 2/3에 item 고정 | track count와 span을 검사한 뒤 auto-track 목록을 확장하며, 축별 10,000 상한 초과를 거부한다. row/column explicit implicit 테스트 통과. |
| 9 | 음수 implicit line 오수용 | explicit track 1개에 row line `-3 / -2` | 역방향 implicit track을 만들지 않고 `InvalidGridPlacement`로 fail-closed 한다. `track_sizing_rejects_negative_implicit_lines_and_ignores_hidden_positive_items` 통과. |
| 10 | positive locked line의 cursor OOB | positive implicit row에 row-locked item을 놓고 column은 auto 배치; column flow는 반대 축으로 반복 | Step 2 cursor를 만들기 전에 definite positive implicit 끝 line까지 축 count를 선계산한다. 발견된 이전 cursor 길이 결함을 수정했고 row/column 회귀가 통과한다. |
| 11 | row/column flow 뒤바뀜 | `grid-auto-flow:row`와 `column`에서 목록 및 item frame 비교 | row flow는 row 목록, column flow는 column 목록을 적용한다. 두 축의 각 runtime scene과 Rust matrix가 일치한다. |
| 12 | explicit/implicit 경계 span | 한 item이 explicit row와 두 implicit row를 함께 span | 확장 결과가 `[12,10,20]`이고 item 높이 42 CSS px임을 확인한다. `track_sizing_keeps_auto_track_pattern_aligned_across_explicit_boundary_spans` 통과. |
| 13 | definite item 겹침·배치 순서 | 겹치는 definite item과 뒤따르는 locked/auto item을 occupancy에 입력 | definite item을 먼저 점유시키고 기존 sparse/dense 순서를 유지한다. 이전 전체 workspace 실행은 199개 통과와 별도 baseline 실패 1개(`auto_repeat_explicit_neighbor_matrix_preserves_tracks_and_frames`, expected 154 / actual 162)였고, 이 변경과 무관함을 `origin/main`에서도 재현했다. |
| 14 | `display:none` 항목이 grid 확장 | hidden child만 positive implicit line에 배치 | hidden child는 placement·row count에서 제외되며 explicit row만 남는다. 새 회귀 테스트에서 normalized 확장 없음과 0×0 frame 확인. |
| 15 | span·반복으로 상한 우회 | 최대 track 수를 넘는 positive line/span 및 목록 10,001개 입력 | `track_end`, `validate_count`, 목록 검증이 각 단계에서 상한을 거부한다. `track_sizing_rejects_empty_invalid_and_oversized_auto_track_lists` 통과. |
| 16 | layout 확장 예약 실패 뒤 부분 결과 게시 | `ExpandedRows` 예약을 실패시킨 뒤 같은 engine으로 재시도 | `GridAllocationFailure`에 node/property를 유지하고 실패 결과를 게시하지 않는다. 같은 입력 재계산 복구도 확인한다. |
| 17 | cascade 예약 실패·축 진단 누락 | auto rows/columns snapshot list reservation을 각각 실패시킴 | `GridAllocationFailure`가 해당 축 property와 node를 보존한다. `track_sizing_auto_track_list_reservation_failures_keep_axis_context` 통과. |
| 18 | auto-repeat 경로에서 목록 무시 | C11.4 auto-repeat container와 auto-track 목록 결합 | auto-repeat placement는 별도 확장 경로라 목록을 전달하지 않는다. 이를 조용히 기본 `auto`로 바꾸지 않도록 `UnsupportedGridAutoRows/Columns`로 거부하며 양 축 regression 통과. 전체 auto-repeat + implicit placement는 범위 밖이다. |
| 19 | stretched text 높이 불일치·border 밖 paint 누락 | implicit min-content item을 root border 밖에 렌더링하고 border 내부·stroke·외부 표본 비교 | text measure는 최종 content height보다 클 때만 실패한다. Android PixelCopy·iOS Simulator Metal surface가 각각 5/5 sample 정확히 일치했고 screenshot에서 overflow를 확인했다. |
| 20 | oracle/runner가 이전 로그나 잘못된 화면으로 통과 | Chrome 20 case와 최종 APK/app 73 scene의 run ID, frame/track trace, draw/present, ADB 수집 범위 점검 | Chrome 20/20, Android API 37 AVD 73/73 (`032916132Z-bdce45bc`), iPhone 17 Pro iOS 26.2 Simulator 73/73 (`033027363Z-dd058ebc`). 첫 ADB 수집 timeout은 stale 전체 logcat 원인으로 분리하고 현재 PID/최근 1,000줄 수집 후 재실행해 통과했다. 실기기는 사용하지 않았다. |

## 실행 근거와 한계

- Rust 집중 검증: `mise exec -- cargo test -p spinon-layout track_sizing_ -- --nocapture` — 22 passed.
- Chrome 관찰과 규범-derived 기대값은 [사전 비교 자료](../c11-4-implicit-intrinsic-auto-tracks-precomparison-2026-10-20/README.md), 앱 장면·표면 근거는 [runtime evidence](README.md)에 둔다.
- Taffy 내부 일반 할당, 전역 OOM 복구, full auto-repeat placement, negative implicit tracks, WPT, 실기기·hardware GPU 및 compositor 이후 surface는 이 검토의 통과 주장이 아니다.
- 이번 검토 후 auto-repeat + auto-track-list 조합의 fail-closed guard와 boundary-span·negative-line·hidden-item regression을 추가했다. 따라서 해당 경로를 포함한 PR diff 검토는 최종 diff에서 다시 수행한다.
