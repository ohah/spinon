# C14.2 이미지 자연 비율 구현 적대 검토 · 2026-10-14

대상 구현은 [전용 solver](../../../crates/spinon-layout/src/calc_tree/replaced_sizing.rs), [Taffy leaf 연결](../../../crates/spinon-layout/src/calc_tree.rs), [입력 검증](../../../crates/spinon-layout/src/compute.rs), [스타일 preflight](../../../crates/spinon-style-to-layout/src/projection.rs)다. 계획 검토 20개를 재사용하지 않고, 구현 코드·실패 입력·Chrome 155 결과에 대해 별도 20개 경로를 확인했다.

| # | 공격 관점 | 판정과 근거 |
|---:|---|---|
| 1 | 다른 document/style source의 metadata snapshot을 재사용하면 이전 이미지 크기가 새 입력에 붙을 수 있다 | PASS — source stamp가 sizing 전에 비교된다. `measurement_snapshot_must_match_source_and_reference_known_nodes`가 stale source를 거부한다. |
| 2 | 같은 이미지 ID가 두 번 들어오면 map 변환에서 중복 항목이 사라질 수 있다 | PASS — snapshot constructor가 중복 ID를 오류로 반환한다. `duplicate_measurement_node_is_rejected_during_snapshot_creation`으로 확인했다. |
| 3 | subtree 밖 ID, 비이미지 ID, 자식이 있는 node에 metadata를 붙일 수 있다 | PASS — unknown/non-image/non-leaf 경로를 각각 오류로 거부한다. `cache_entries_for_non_images_or_nodes_outside_the_subtree_are_rejected`와 layout leaf 검증으로 확인했다. |
| 4 | zero·negative·NaN·infinity 또는 나눗셈 overflow ratio가 프레임에 전파될 수 있다 | PASS — 입력 ratio·자연 크기와 계산 결과가 유한 양수인지 검사한다. `percentages_ratio_overflow_and_authored_ratio_constraints_fail_closed`가 invalid 값과 overflow를 닫는다. |
| 5 | 두 자연 축과 명시 natural ratio가 서로 다르면 어느 쪽을 사용할지 달라질 수 있다 | PASS — 불일치 ratio를 거부한다. `explicit_natural_ratio_must_match_both_natural_dimensions`가 확인한다. |
| 6 | hidden subtree 또는 고정 크기 이미지에서 decode metadata를 불필요하게 요구하거나 결과에 섞을 수 있다 | PASS — hidden 이미지는 snapshot 없이 성공하고, 두 축 고정 크기는 제공된 natural size에 좌우되지 않는다. `hidden_image_needs_no_measurement_and_fixed_size_ignores_cached_natural_size`와 layout 대응 테스트가 통과했다. |
| 7 | 성공했지만 자연 정보가 비어 있거나 한 축만 있는 metadata가 0×0으로 축소될 수 있다 | PASS — 빈 성공 entry는 기본 객체 크기를, partial entry는 있는 축과 기본 반대축을 사용한다. `successful_empty_and_partial_natural_metadata_use_default_object_axes`로 확인했다. |
| 8 | ratio-only auto/auto에 임의의 contain 규칙을 넣어 문맥 의존 contribution을 가릴 수 있다 | PASS — 이번 profile에서 fail-closed하고 C14.3으로 넘긴다. layout과 HostDocument/Stylo 경계 모두 오류를 확인한다. |
| 9 | 지정 width에서 height transfer가 natural ratio보다 authored 값이나 기본 높이를 잘못 택할 수 있다 | PASS — `width:48px;height:auto`는 `48×32`다. Rust sizing과 Chrome PNG oracle에서 일치한다. |
| 10 | 지정 height에서 width transfer를 빠뜨리거나 반대 방향과 다른 규칙을 쓸 수 있다 | PASS — `width:auto;height:24px`는 `36×24`다. Rust sizing과 Chrome oracle에서 일치한다. |
| 11 | CSS 2.2 natural ratio min/max의 10개 constraint violation 분기 중 경계 조합이 누락될 수 있다 | PASS — table-driven test의 10개 기대값과 Chrome fixture를 대조했다. AVD에서도 table test 1/1이 통과했다. |
| 12 | 동일 축 min이 max보다 클 때 max가 다시 크기를 줄일 수 있다 | PASS — max를 min으로 정규화해 min이 우선한다. `minimum_wins_when_maximum_is_smaller`와 `min-over-max` Chrome 행으로 확인했다. |
| 13 | auto 축의 min/max를 적용한 뒤 natural ratio를 다시 강제해 Chrome의 비율 깨짐 동작과 다를 수 있다 | PASS — `48×20` 및 `30×24` partial-auto 결과가 Chrome과 일치한다. 이 조건은 새 Flex fixture에도 포함했다. |
| 14 | natural ratio를 border-box ratio처럼 계산하거나 padding/border를 두 번 더할 수 있다 | PASS — natural ratio는 content box에서 계산하며 외곽 edge를 한 번 적용한다. content-box/border-box fixture와 unit test가 `48×34`를 확인한다. |
| 15 | authored numeric `aspect-ratio`가 natural ratio를 덮지 못하거나 natural ratio가 authored ratio를 덮을 수 있다 | PASS — authored `1/1`과 width `48`은 `48×48`이며 자연 ratio `3/2`는 결과에 개입하지 않는다. |
| 16 | border-box에서 authored numeric ratio가 padding/border를 포함하지 않은 비율로 해석될 수 있다 | PASS — border-box `48×32`, `aspect-ratio:3/2`를 Chrome과 Rust에서 확인했다. |
| 17 | percent/calc 및 authored aspect-ratio/min-max 조합이 이 제한 solver에 조용히 들어올 수 있다 | PASS — projection/preflight가 지원하지 않는 profile을 식별 오류로 닫으며 negative tests가 각각 거부를 확인한다. |
| 18 | Flex 부모의 natural auto-axis 측정이 최종 크기로 굳거나 synthesized Taffy ratio가 definite width를 줄일 수 있다 | PASS — 최초 회귀는 `48×16`이었고 temporary Taffy ratio hint는 `30×20`을 냈다. hint를 제거하고 ContentSize 측정 callback이 ratio/min/max solver를 사용하게 바꾼 뒤 Chrome의 `48×20`과 일치했다. |
| 19 | definite cross-axis와 `align-items:stretch`가 auto main axis, max constraint 또는 flex-start의 natural size를 섞을 수 있다 | PASS — Chrome oracle과 HostDocument→Stylo→Taffy 비교 8/8이 일치한다. `60×40`, `30×20`, `24×16`, `36×24`를 포함한다. |
| 20 | absolute child의 flow-only tree와 visual tree가 서로 다른 metadata 또는 sizing 경로를 쓸 수 있다 | PASS — 두 tree에 같은 optional metadata를 전달하고 absolute image의 flow/visual frame가 모두 `48×32`인지 확인했다. API 37 AVD와 workspace 결과에도 포함했다. |

20개 별도 실패 관점을 검토했다. Flex 회귀에서 확인한 측정 불일치는 수정 후 Chrome fixture, layout tests, style-to-layout tests 및 AVD에서 재확인했다. 해결되지 않은 구현 P1/P2는 없다. 지원하지 않는 CSS profile과 앱 runtime/decoder integration은 계획에 남은 별도 경계다.

실행 로그와 환경은 [구현 검증 근거](c14-2-image-ratio-implementation-2026-10-14/README.md)에 있다. 이 구현 검토는 stacked PR 적대 검토를 대체하지 않는다.
