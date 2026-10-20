# C11.4 negative implicit track sizing implementation review

2026-10-20 구현 후보를 계획 검토표와 분리한 20개 실패 관점으로 검토했다. 검토 기준은 base `70877fff99306033ae38b2d7e347d34d7b76c429`와 현재 implementation worktree의 Rust normalizer, CSS projection, fixture route, simulator evidence다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | Numeric line 0과 positive/negative CSS line 식 | `axis.rs:115-155`에서 0을 거부하고 positive boundary `line - 1`, negative boundary `explicit_count + 1 + line`을 checked `i64`로 계산한다. `line_zero_and_more_than_ten_thousand_total_tracks_fail_closed`와 33-case Chrome validator가 대응한다. |
| 2 | Negative line 기준이 expanded implicit extent에 따라 흔들리는지 | line resolution은 normalizer의 signed extent 생성 전 explicit track count만 받는다(`axis.rs:19-33,115-155`). 추가 leading tracks가 -1 line의 기준을 이동시키지 않는다. |
| 3 | Reversed/equal definite line 및 span 조합 | `resolve_axis`는 reversed boundary를 정렬하고 같은 boundary의 end를 auto로 바꾼다. line+span은 checked span 길이로 변환하고 두 span이면 한쪽 span만 유지한다(`axis.rs:27-112`). 고정 Chrome case와 projection fixture가 reversed/equal 및 span 조합을 포함한다. |
| 4 | Extreme line/span arithmetic와 normalized Taffy line 범위 | 경계 덧셈·뺄셈, span 변환, origin shift 뒤 `i32` line 변환을 checked 처리한다(`axis.rs:37-110`, `lines.rs:18-34`). `i32::MIN/MAX` regression은 track cap 오류로 닫힌다. |
| 5 | Empty explicit `none`의 음수 line anchor와 다른 profile 오염 | C11.4 cascade에서만 `none`을 빈 track vector로 보존하고(`incremental/grid.rs:372-395`), TrackSizing 검증만 빈 explicit track list를 허용한다(`grid.rs:384-395,652-665`). `non_track_sizing_profiles_still_reject_an_empty_explicit_track_list`와 `track_sizing_profile_anchors_negative_lines_when_template_is_none`가 경계를 검사한다. |
| 6 | Leading + explicit + trailing extent cap과 subtraction overflow | `Extent::count`는 `end - start`를 checked 계산한 후 10,000 제한을 적용한다(`extent.rs:29-44`). 9,999/10,000 허용과 10,001 거부를 extent 단위로 검사한다(`extent.rs:153-184`). |
| 7 | Auto placement로 생긴 trailing extent와 두 축 합계 제한 | definite placement는 auto scan 전 extent에 포함되고, auto placement 후보는 `track_end`에서 축별 cap을 검사한다(`track_sizing.rs:82-112,347-408`, `extent.rs:83-105`). 최종 expanded vector에서도 다시 합계를 검사한다(`tracks.rs:20-29`). |
| 8 | Leading-side list cycle의 방향과 0px track | leading tracks는 explicit 경계에 가까운 순서로 reverse cycle을 만들고 trailing tracks는 authored order를 유지한다(`tracks.rs:51-79`). 1-track list, 양방향 pattern과 `0px` case가 Rust/Chrome matrix에 있다. |
| 9 | Leading tracks를 붙일 때 positive implicit index의 기존 phase | explicit tracks를 사이에 그대로 복사하고 trailing index를 0부터 시작한다(`tracks.rs:63-79`). mixed negative/explicit/positive pattern test와 33-case oracle이 index seam을 검사한다. |
| 10 | Definite item이 auto placement보다 늦게 점유되는지 | 모든 두 축 definite item을 먼저 occupancy에 넣고 `placed`에 등록한다(`track_sizing.rs:125-160`). simulator의 definite/auto overlap 및 paired hole scenes에서 row/column positions와 frame을 대조했다. |
| 11 | Leading origin에서 row-locked/column-locked cursor index | locked cursor vector 크기는 signed extent count로 예약하고, 고정 좌표는 `coordinate - origin`으로 index한다(`track_sizing.rs:162-183`, `extent.rs:108-119`). row와 column 방향 leading placement scene이 각 simulator에서 통과했다. |
| 12 | Sparse cursor 진행과 dense cursor reset이 leading origin을 보존하는지 | auto scan은 start-most signed origin에서 시작하며 dense는 같은 origin으로 되돌린다(`track_sizing.rs:263-349`). paired sparse/dense hole regression에서 다음 item의 위치 차이를 확인한다. |
| 13 | Row/column flow 전환에서 축 역할이 바뀌어도 올바른 extent인지 | `row_flow`로 primary/secondary, locked axis, track size를 한 번 정하고 두 flow 분기를 같은 signed coordinate 규칙으로 처리한다(`track_sizing.rs:39-44,162-185,194-262`). row/column과 dense/sparse 각 방향 scene이 106-scene matrix에 포함된다. |
| 14 | Hidden child가 extent 또는 occupancy를 늘리는지 | hidden map에서 보이는 child만 목록을 예약하고 placement로 변환한다(`track_sizing.rs:47-84`). `hidden-negative-does-not-expand`에서 root width와 box count를 Chrome 및 양 simulator에 대조했다. |
| 15 | Occupancy interval의 half-open overlap과 경계 인접 배치 | overlap 검색은 `end <= start`인 이전 interval을 건너뛰고 시작점 `< query end`만 겹침으로 본다(`occupancy.rs:28-48`). 인접 interval은 겹치지 않으면서 저장 시 병합되어 중복 탐색을 피한다(`occupancy.rs:86-117`). |
| 16 | 최종 origin shift가 placement line과 expanded track에서 같은 기준인지 | row/column shift를 한 번 계산해 각 item boundary를 `coordinate + shift + 1`로 변환하고, 같은 extent에서 leading/trailing track 수를 산출한다(`track_sizing.rs:400-445`, `lines.rs:18-34`, `extent.rs:53-81`). 화면의 leading/explicit/trailing frame 순서가 두 축에서 확인됐다. |
| 17 | Spinon-owned vector reservation failure 뒤 부분 결과와 worker 복구 | visible item, locked cursor, occupancy rows/intervals, expanded row/column 여섯 failpoint를 구체 오류로 전파한다. 같은 worker의 다음 계산 성공까지 `track_sizing_reservation_failures_return_errors_and_recover_on_same_worker`가 검사한다(`track_sizing_tests.rs:392-505`). |
| 18 | TrackSizing 도입이 기존 profile 및 auto-repeat route를 우회 변경하는지 | TrackSizing + auto-repeat은 기존 normalizer로 먼저 라우팅하고 typed `grid-auto-*` list 결합은 오류로 닫는다(`dense_column.rs:32-60`). 그 외 profile은 기존 normalizer를 사용하며 negative-line/auto-repeat 상호작용은 이번 범위에서 지원/거부를 단정하지 않도록 계획 문구를 수정했다. 통합 회귀에서 확인한 empty auto-fit prefix-boundary gutter는 별도 Chrome 155 probe와 Rust test로 보존했다(`explicit_grid.rs:333-403`, `grid_track_sizing_tests.rs:537-598`). |
| 19 | Cascade와 style-to-layout에서 `none`, track list, negative line metadata가 끊기는지 | C11.4 track-sizing cascade와 projection fixture가 columns·rows, reverse list, `none` explicit grid를 통해 LayoutOutput을 확인한다(`incremental/grid.rs:372-395`, `grid_values.rs:261-270`, `c11_4_track_sizing_areas_tests.rs:377-447`). named line·area·unsupported value는 기존 fail-closed 경계다. |
| 20 | App fixture 경계, 실제 surface assertion, 검증 범위의 과장 | 신규 고정 payload는 `c04-runtime-gpu-fixture-test` 경계에 있고 기본/opt-in artifact marker control을 실행했다. Android AVD와 iOS Simulator 각각 106/106 draw/present 및 overflow 표본이 통과했다. root edge sample은 배경색이며 border stroke 자체를 검증하지 않고, AVD는 software renderer, iOS 표면은 compositor 제출 전이다. 계획과 evidence 문구를 이 한계에 맞췄다. |

## 수정·잔여 항목

- 초기 계획 문구가 auto-repeat 경로의 negative line을 명시 거부한다고 과장했으나 실제 route는 기존 auto-repeat normalizer로 분리된다. 계획을 수정하고 이 교차 동작은 검증 대기 범위로 남겼다.
- 계획의 `border stroke` pixel 조건은 현재 renderer가 stroke를 그리지 않아 실행할 수 없다. 검사한 root interior/edge background와 overflow item/tail을 정확히 기재하고 stroke 검증을 후속 범위로 옮겼다.
- 10,000 track 합계 제한은 checked extent 단위 시험으로 검증한다. Taffy trace의 100,000 track-state 예산으로 전체 9,999-track layout이 먼저 fail-closed할 수 있어 전체 layout 경계 통과로 표현하지 않도록 기록했다.
- 이 검토에서 blocking layout defect는 발견하지 않았다. CSS Reference 전체 명령은 별도로 563개 중 559개 통과, 4개 source-contract test 실패이며 현재 main 기준선에도 같은 검사 불일치가 있다. 이를 C11.4 통과로 합산하지 않았다.
