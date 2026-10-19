# C11.4 explicit-neighbor auto-repeat 구현 실패 관점 검토 20개

**대상:** `feat/c11-4-auto-repeat-explicit-neighbors-20261020`의 cascade, typed snapshot/projection, layout adapter, 앱 fixture 및 simulator capture 변경. 2026-10-20 구현 뒤 변경 코드를 서로 다른 실패 경로 20개로 다시 살폈다.

| # | 적대 관점 | 결과와 근거 |
| ---: | --- | --- |
| 1 | track list에 auto-repeat이 없거나 두 개면 잘못된 반복 위치를 만들지 않는가? | **통과.** cascade가 auto-repeat 수를 세고 0개·복수·integer repeat을 거부한다. [`computed_auto_repeat_tracks`](../../../../crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs:359). |
| 2 | CSSOM line-name 벡터가 누락되거나 길이가 달라도 prefix index를 신뢰하는가? | **통과.** outer line metadata가 `values + 1`인지 확인하고 모든 line name을 제한한다. 같은 파일의 `has_valid_outer_line_names`. |
| 3 | repeat 내부 named line 또는 빈 repeated fragment가 Taffy까지 새는가? | **통과.** `track_sizes` 비어 있음과 fragment line metadata 불일치를 사전 거부한다. 같은 파일의 repeat 검증 분기. |
| 4 | explicit neighbor에 음수, 비유한 값, `fr`·percent·calc가 섞이는가? | **통과.** cascade와 layout validation 모두 유한·0 이상 `LengthPx`만 통과한다. [`is_auto_repeat_neighbor_track`](../../../../crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs:450), [`auto_repeat_neighbor_track_is_fixed_px`](../../../../crates/spinon-layout/src/grid.rs:711). |
| 5 | repeated track의 지원 조건이 neighbor 조건과 혼동되어 임의 track이 허용되는가? | **통과.** repeat fragment는 기존 `track_size_is_valid` 및 fixed-minimum 검사를 별도로 거친다. cascade와 `validate_grid_layout`. |
| 6 | prefix + fragment + suffix 합산에서 `usize` overflow 또는 authored track 상한 초과가 발생하는가? | **통과.** `checked_add`와 `MAX_COMPUTED_GRID_TRACKS` 필터 뒤 fallible reservation을 한다. [`computed_auto_repeat_tracks`](../../../../crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs:405). |
| 7 | repeat가 prefix/suffix 가운데 있을 때 시작·끝 index가 한 칸 밀리는가? | **통과.** `start=prefix_count`, `end=prefix_count + fragment_len`을 exclusive range로 유지한다. cascade snapshot 및 projection 테스트 [`track_sizing_profile_projects_auto_repeat_pattern_and_mode`](../../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values/tests.rs:339). |
| 8 | cascade snapshot을 지나는 동안 mode나 range가 유실·재계산되는가? | **통과.** typed `ComputedCssGridAutoRepeat`의 mode/start/end를 projection에서 그대로 `GridAutoRepeat`에 복사한다. [`project_auto_repeat`](../../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs:249). |
| 9 | 한 grid의 양 축 auto-repeat 또는 non-track-sizing profile에서 처리되는가? | **통과.** `validate_grid_layout`이 track-sizing profile 외의 repeat와 두 축 동시 repeat을 거부한다. [`grid.rs`](../../../../crates/spinon-layout/src/grid.rs:368). |
| 10 | 손상된 range에서 slice panic이 나는가? | **통과.** layout validation과 Taffy projection 각각 `start < end <= len`을 확인한 뒤 slice한다. [`grid.rs`](../../../../crates/spinon-layout/src/grid.rs:417), [`taffy_style/grid.rs`](../../../../crates/spinon-layout/src/taffy_style/grid.rs:79). |
| 11 | layout 계층에서 직접 만든 입력으로 cascade의 fixed-neighbor 제한을 우회하는가? | **통과.** prefix/suffix 모두 layout validation에서 다시 유한·0 이상 fixed px인지 검사한다. [`grid.rs`](../../../../crates/spinon-layout/src/grid.rs:428). |
| 12 | Taffy component 변환 중 authored 순서가 바뀌거나 repeat fragment가 쪼개지는가? | **통과.** prefix Singles → 하나의 Repeat(fragment 전체) → suffix Singles 순서로 만든다. [`to_taffy_grid_tracks_with_auto_repeat`](../../../../crates/spinon-layout/src/taffy_style/grid.rs:71). |
| 13 | component vector나 fragment vector allocation 실패가 panic/부분 layout으로 이어지는가? | **통과.** 두 buffer에 `try_reserve_exact`를 사용하고 `GridAllocationFailure`를 반환한다. 같은 Taffy adapter 함수. |
| 14 | 반복 축 placement에서 line 0, signed overflow, span 0 또는 span overflow가 통과하는가? | **통과.** 0 line을 제외하고 `i16` line 및 양수 `u16` span 범위를 확인한다. [`validate_auto_repeat_placement`](../../../../crates/spinon-layout/src/grid_auto_placement/dense_column.rs:467). |
| 15 | 반복하지 않는 반대 축의 line/span 검증을 건너뛰는가? | **통과.** 반대 축은 기존 `resolve_axis`를 호출해 explicit line bounds, positive span 및 implicit-track 필요 오류를 유지한다. [`normalize_auto_repeat_grid_items`](../../../../crates/spinon-layout/src/grid_auto_placement/dense_column.rs:384), [`axis.rs`](../../../../crates/spinon-layout/src/grid_auto_placement/axis.rs:27). |
| 16 | 자동 배치 item을 definite placement로 잘못 표시해 track size/auto-fit occupancy를 왜곡하는가? | **통과.** 반대 축이 indefinite이거나 repeat 축에 line이 없으면 auto-placed 집합에 넣고 원 placement를 Taffy에 전달한다. 같은 normalizer. |
| 17 | `display:none` subtree 자식이 occupancy나 box count를 바꾸는가? | **통과.** visible child iteration은 기존 hidden map으로 필터링한다. [`normalize_dense_grid_container`](../../../../crates/spinon-layout/src/grid_auto_placement/dense_column.rs:35)와 auto-repeat helper. |
| 18 | columns/rows 방향 분기, multi-track fragment, span 점유, auto-fit collapse의 축이 뒤바뀌는가? | **통과.** 13개 고정 oracle matrix가 column·row, one/two-track fragment, occupied/span/empty collapse를 포함하며 양 simulator의 전체 used track과 frame이 일치한다. [실행 근거](./README.md). |
| 19 | 앱 검증용 scene 입력이 임의 문자열 실행이나 허용 목록 우회를 허용하는가? | **통과.** fixture scene/style/id는 compile-time 상수이고 런타임 route는 13개 고정 scene ID로 제한한다. [`c11_4.rs`](../../../../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs:340), [Android route](../../../../platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:126), [iOS route](../../../../platforms/ios/Sources/C0410RuntimeGpuDemo.swift:16). |
| 20 | 검증 script가 다른 기기, trace 일부, 잘못된 frame 또는 draw 전에 끝난 실행을 pass로 세는가? | **통과.** Android는 qemu/API 37을, iOS는 iPhone 17 Pro/iOS 26.2를 고정 검사한다. 13개 모두 full node frame, 모든 최종 used track/collapse, trace chunk 전체, draw/present를 확인한다. [capture script](../../../../tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs:1), [Android/iOS result](./README.md). |

## 결과

차단 결함은 찾지 못했다. 적용 범위는 한 축의 단일 auto-repeat, fixed-minimum repeated fragment와 유한·0 이상 fixed px neighbor다. 전체 placement/span/gutter 조합, 10,000-track 경계, WPT, 실기기·hardware GPU 및 border-stroke surface pixel 비교는 이 검토의 통과로 확대하지 않는다.
