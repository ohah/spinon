# C11.2.3 dense·column 구현 실패 관점 검토 · 2026-10-13

## 검토 범위

[C11.2.3 runtime 구현](../../../plan/c11-2-3-dense-column-placement.md), Rust placement·cascade·projection, 전용 FFI와 모바일 실행 경로, 고정 Chromium 행렬 및 실행 비교기를 서로 다른 실패 관점에서 검토했다. 계획 자체의 검토는 별도 [계획 검토](./c11-2-3-dense-column-placement-plan-review-2026-10-13.md)에 둔다.

## 실패 관점과 확인 결과

| # | 공격 관점 | 확인한 방어·근거 |
| --- | --- | --- |
| 1 | C11.2.3이 기존 sparse row profile을 덮어쓰거나 다른 의미로 재사용 | 별도 `RuntimeBlockFlexGridAutoPlacementDenseV1` profile과 전용 FFI 진입점으로 분리했다. C11.2.2 regression 2개와 runtime Grid 13개가 통과했다. |
| 2 | raw CSS 선언을 다시 읽어 cascade shorthand·computed 값을 놓침 | Stylo의 typed `grid-auto-flow`, `grid-auto-rows`, `grid-auto-columns`를 projection에 전달한다. `runtime_grid_tests` 13개와 `c11_2_3_grid_auto_placement_tests` 2개가 통과했다. |
| 3 | 기본 `row` sparse가 빈 셀을 되채우도록 바뀜 | 기존 cursor를 보존하는 sparse negative control을 두었다. 고정 Chrome 154 비교와 C11.2.2 회귀에서 source item 배치가 유지됐다. |
| 4 | `row dense`가 다음 항목을 시작점부터 다시 검색하지 않음 | dense item마다 검색 cursor를 초기화한다. row dense hole-fill·span scene이 Chrome 기준과 일치했다. |
| 5 | `column` sparse에서 primary/secondary 축 전치가 누락 | 행·열 track 수, span, cursor를 flow 축에 맞춰 계산한다. column sparse·locked item·implicit column scene이 비교 대상에 포함됐다. |
| 6 | `column dense`가 column-major 기준으로 빈칸을 다시 찾지 않음 | column dense 전용 scene에서 축별 occupancy와 dense cursor reset을 확인했다. 모바일 두 행렬 모두 해당 scene의 frame이 기준 좌표와 일치했다. |
| 7 | definite row·column item이 auto item보다 늦게 점유되어 충돌 | 두 축이 definite인 항목을 먼저 occupancy에 등록한다. `mixed_definite_and_auto_items_place_definite_items_first`와 Chrome 혼합 배치 scene이 이를 확인한다. |
| 8 | 고정 row/column item이 반대 축의 첫 빈 범위를 찾지 못함 | flow별 locked-axis cursor와 span 검사를 분리했다. row/column locked item scene과 관련 Rust layout 검사를 통과했다. |
| 9 | definite column이 이전 cursor보다 작아질 때 다음 행으로 이동하지 않음 | 감소를 감지해 cursor row를 전진시킨다. `column_locked_items_wrap_to_the_next_row_when_cursor_column_decreases` 회귀와 고정 Chrome scene이 통과했다. |
| 10 | spanning item의 일부만 점유되어 겹치는 셀에 후속 item 배치 | occupancy가 item 전체의 행·열 직사각형을 검사한다. `span`, overlap, wrap scene의 node rect를 Chrome과 비교했다. |
| 11 | reversed·equal·negative line의 CSS 충돌 정규화가 달라짐 | 축 line 정규화와 충돌 처리를 placement 전에 적용한다. `sparse_placement_normalizes_reversed_equal_and_negative_lines` 및 Chrome 기준 장면이 통과했다. |
| 12 | auto-placement가 필요한 implicit 축을 미리 확장하지 않음 | flow-secondary 축의 auto span을 보고 track 수를 placement 전에 정한다. 양의 implicit row/column fixture와 모바일 frame 비교가 통과했다. |
| 13 | implicit `px` track에서 기존 gap 또는 축별 sizing이 사라짐 | typed auto-track 값을 projection하고 implicit row/column 확장 시 gap을 유지한다. 기존 `implicit_rows_repeat_fixed_track_size_and_keep_row_gap`과 신규 projection 검사가 통과했다. |
| 14 | `auto` 또는 반복 sizing을 고정값처럼 임의 성공 처리 | implicit track 생성이 필요하면 지원하지 않는 `auto`·반복 pattern을 오류로 닫는다. cascade typed-value 보존 및 unsupported scene 거부를 확인했다. |
| 15 | track count·span 산술 overflow 또는 Taffy 제한 초과를 잘라 성공 | checked arithmetic과 10,000-track 상한을 검사하고 초과 scene을 거부한다. `implicit_track_cap_is_checked_before_expanding_rows`와 10,000/10,001 Chrome 비교가 근거다. |
| 16 | negative implicit 영역 또는 explicit line 바깥 위치를 양의 확장으로 오해 | 해당 입력은 전체 계산 오류로 거부한다. expected-reject scene에서 정확한 오류 code·NodeId·property를 확인했다. |
| 17 | `display:none` 자식이 점유 셀을 차지하거나 nested Grid가 부모 상태를 공유 | hidden child는 배치 대상에서 제외하고 nested container는 별도 occupancy를 사용한다. hidden/nested 장면의 NodeId·frame·paint 순서를 실행 비교기가 확인했다. |
| 18 | dense의 시각 위치를 DOM 순서·paint 순서 변경으로 전파 | layout 위치와 child 배열을 분리한다. 성공 scene은 preorder NodeId·paint order를 고정 비교하며 Android/iOS 모두 source order가 유지됐다. |
| 19 | 예상 거부·allocator 실패에서 일부 frame이 남거나 다음 실행이 오염 | 거부 scene은 frame snapshot과 양수 draw가 없고 새 app process·HostDocument로 격리한다. allocator는 여섯 실제 Vec 예약 지점 각각에 실패를 주입해 정확한 `GridAllocationFailure`와 동일 worker 복구를 확인했다. [`reservation_failure` 실행 근거](./c11-2-3-grid-allocation-failure-2026-10-13.md). |
| 20 | FFI 인수·scene/run ID·matrix hash·플랫폼 결과를 잘못 결합하거나 증거를 과장 | FFI null/invalid output/unknown scene을 거부하고, 고정 matrix·inventory·Chrome reference SHA-256 및 허용 scene 목록을 검증한다. Android 실기기와 iOS Simulator를 별도 실행해 기록했으며 표면 pixel·WPT 적합성이라고 부르지 않았다. |

## 검토 중 수정한 문제

- 행렬 validator가 고정 viewport의 `deviceScaleFactors: [1, 2]` 필드를 누락한 가정을 갖고 있었다. 실제 matrix schema와 일치하도록 검사를 고쳤다.
- hidden item 비교기의 기대 ID가 fixture와 달랐다. `c1123-hidden-item-excluded-hidden`으로 맞췄다.
- Clippy가 auto-placement projection의 동일한 두 분기를 지적해 조건을 합쳤다.
- Android/iOS 앱 진입점의 C11.2.3 scene·run ID allowlist 검증을 추가하고 두 플랫폼 행렬을 최신 소스로 다시 실행했다.
- 재검토 중 C11.2.3에서 새로 추가한 `ExpandedColumns` 예약 지점이 기존 allocator failure 주입 목록에서 빠진 것을 발견했다. column-flow 실제 확장 입력의 실패 테스트를 추가하고 여섯 지점을 다시 실행했다. 전체 `Vec` 예약 목록은 `grid_auto_placement*` 내 `reserve` 호출과 일치한다.

## 최신 실행 근거

- Rust: `cargo test --locked -p spinon-layout reservation_failure -- --nocapture`에서 실제 예약 지점 여섯 곳의 실패 반환·동일 worker 복구를 확인했다. `cargo fmt --all -- --check`, `git diff --check`, 관련 Clippy `-D warnings`, C11.2.3 layout/style-to-layout, C11.2.2 regression, runtime Grid, FFI 및 CSS matrix/precomparison JavaScript 검사를 확인한다. 전체 workspace test suite 통과를 주장하지 않는다.
- Android 16/API 36 · SM-S731N 실기기: 25 scene·107 node, 성공 19·기대 거부 6, geometry 최대 오차 0 CSS px. 장면마다 새 앱 process를 썼다. [실행 요약과 원본 로그](./c11-2-3-dense-column-placement-android-physical-runtime-matrix-2026-10-13/c1123-a-mv5yld3e-4ded3cc6/README.md)
- iPhone 17 Pro · iOS 26.2 Simulator: 같은 25 scene·107 node, 성공 19·기대 거부 6, geometry 최대 오차 0 CSS px. 장면마다 새 앱 process를 썼다. [실행 요약과 원본 로그](./c11-2-3-dense-column-placement-ios-simulator-runtime-matrix-2026-10-13/c1123-i-mv5ymb0x-10eed1fb/README.md)
- Chrome 154는 고정 oracle이다. Chrome의 WPT 실행 결과를 Spinon WPT 적합성으로 승격하지 않는다.

## 아직 보장하지 않는 범위

Android PixelCopy와 iOS Metal drawable 표면 readback은 별도 표면 실행에서 각 19 scene·99/99 RGBA exact로 확인했다. iOS 결과는 compositor 제출 전이며 screenshot만으로 주장하지 않는다. 전체 CSS Grid, 전체 WPT, iOS 실기기, 전역 allocator OOM 복구와 성능 우위는 이 검토의 결론에 포함하지 않는다. C11.2.3은 제한 profile의 geometry/runtime 검증 완료이며 CSS Grid 전체 지원 완료가 아니다.
