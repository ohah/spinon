# C13.3 snapshot delta-router 구현 실패 검토 20개

이 검토는 이번 변경의 `RuntimeRenderSnapshot` 정책 입력형 delta router만 다룬다. gesture 상태, input queue, computed CSS style, Android/iOS adapter는 아직 이 코드에 연결되지 않아 검토 범위 밖이며 완료 판정을 대신하지 않는다.

| # | 독립 실패 관점 | 확인 결과 | 근거 |
| --- | --- | --- | --- |
| 1 | 이전 scroll revision을 가진 sample이 현재 offset을 덮어쓸 수 있는가 | 거부한다. delta 계산 전에 expected revision을 비교한다. | `stale_or_non_finite_samples_fail_without_changing_the_source_snapshot` |
| 2 | NaN 또는 무한 delta가 한 축만 반영될 수 있는가 | 두 축을 모두 finite 검사하고 snapshot 생성 전에 거부한다. | 같은 stale/non-finite 테스트 |
| 3 | 비유한 viewport point가 다른 target을 선택할 수 있는가 | `InvalidScrollPoint`로 거부한다. | 같은 stale/non-finite 테스트 |
| 4 | viewport 밖의 point가 target을 만들 수 있는가 | 기존 hit-test 경계에서 miss가 나고 revision은 유지된다. | `viewport_hit_miss_and_noop_do_not_advance_scroll_revision` |
| 5 | hit clip 밖의 자식이 scroll chain을 훔칠 수 있는가 | hit-test가 clip을 반영해 `None`을 반환한다. | `hit_test_clip_miss_does_not_select_a_scroll_target` |
| 6 | 이 snapshot에 없는 NodeId target을 직접 넘길 수 있는가 | snapshot에 없는 target은 `InvalidScrollTarget`이다. 새 document/tree 세대와의 비교는 아직 session integration 책임이다. | stale/non-finite 테스트의 unknown-target control |
| 7 | target 자체가 scroll owner일 때 자기 owner가 빠지는가 | target owner를 ancestor 앞에 먼저 넣어 nearest inclusive 동작을 유지한다. | `target_scroll_owner_is_inclusive_and_its_policy_is_preserved_by_offsets` |
| 8 | inner/outer owner 순서가 역전되는가 | snapshot metadata의 nearest-to-outer 순서를 그대로 처리하며 change 기록도 같은 순서다. | `routes_inner_remainder_to_outer_and_publishes_one_revision` |
| 9 | 중복 ancestor 또는 owner가 delta를 두 번 소비하는가 | owner를 한 번만 수락하고 반복 항목은 `InvalidScrollState`로 종료한다. | `route_scroll_delta_for_target`의 `seen` 검사 |
| 10 | metadata가 없는 owner로 일부 offset이 먼저 적용되는가 | owner lookup 실패가 candidate snapshot 생성 전 오류가 된다. | `route_scroll_delta_for_target`의 owner lookup과 `with_scroll_offsets` 호출 경계 |
| 11 | X만 user-scrollable이 아닌 owner가 Y까지 막는가 | 축별 플래그로 계산해 Y는 inner→outer로 진행하고 X는 outer로 전달한다. | `user_scrollability_is_independent_per_axis` |
| 12 | scroll range 0인 축에서 remainder가 사라지는가 | inner가 소비하지 않은 축 delta가 auto parent로 전달된다. | `axis_without_user_scroll_range_forwards_delta_to_parent` |
| 13 | `overflow:hidden` 대응 owner가 직접 delta를 소비하는가 | user-scrollable 플래그가 꺼진 owner는 offset을 유지한다. | `hidden_boundary_does_not_consume_but_auto_forwards_remainder` |
| 14 | hidden owner의 `auto` remainder가 ancestor로 안 가는가 | outer가 전량 소비하고 inner는 그대로 남는다. | 같은 hidden/auto 테스트 |
| 15 | hidden owner의 contain 정책이 소비 없이 chain을 계속하는가 | 해당 축 remainder를 유지하고 ancestor는 움직이지 않는다. | `hidden_boundary_can_stop_chaining_without_consuming` |
| 16 | X contain이 Y까지 차단하는가 | X만 남고 Y remainder는 outer로 전달된다. | `overscroll_contain_stops_only_its_axis` |
| 17 | 현재 local affordance가 없는 상태에서 `none`이 chaining을 허용하는가 | `contain`처럼 축별 ancestor chain을 차단한다. 두 값의 시각 효과 차이는 이 API에 없다. | `overscroll_none_and_contain_both_stop_only_the_configured_axis` |
| 18 | 음수 delta가 min offset 아래에서 wrap/overflow되는가 | min 0에 clamp하고 소비한 값을 제외한 remainder를 보존한다. | `fractional_and_reverse_deltas_keep_css_px_precision_and_clamp_at_zero` |
| 19 | fractional CSS px delta가 중간 정수화로 손실되는가 | 1.25 px를 offset에 보존한다. | 같은 fractional/reverse 테스트 |
| 20 | inner와 outer가 함께 움직일 때 revision이 여러 번 증가하거나 boundary no-op이 revision을 올리는가 | 변경은 한 revision으로 publish하고 양쪽 max boundary의 no-op은 revision과 offset을 유지한다. | `routes_inner_remainder_to_outer_and_publishes_one_revision`, `saturated_inner_and_outer_boundaries_keep_revision_and_report_remainder` |

## 발견 및 처리

- 첫 target-inclusive 테스트는 scroll된 자식 box가 fixture point에 남아 있어서 inner owner를 target으로 선택했다. target owner 경계를 직접 호출하는 검증으로 고쳐 child-hit과 inclusive-owner 조건을 분리했다.
- 첫 Clippy 실행은 결과 accessor의 `self: &Self` 서명을 지적했다. 관용 `&self`로 바꾸고 workspace Clippy를 다시 통과시켰다.
- 축별 contain 경로는 x가 막혀도 y ancestor 계산을 계속해야 한다. 독립 stopped flag와 diagonal control로 이를 검증했다.

실제 native gesture lifecycle, touch-action, overscroll computed-style 갱신, queue saturation/allocation recovery, event correlation, Android/iOS 화면 실행은 아직 검증하지 않았다.
