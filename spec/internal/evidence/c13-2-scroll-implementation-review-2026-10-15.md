# C13.2 구현 실패 경로 20개 검토

- **대상:** `25f0fe2` 이후 C13.2 runtime, layout projection, FFI, Android/iOS Simulator fixture 변경
- **계획 검토와의 관계:** 기존 계획 20개 표를 재사용하지 않고 구현 코드와 실행 결과를 새로 확인했다.
- **실행 근거:** `cargo test --locked --workspace` 797 passed / 0 failed / 4 ignored, workspace Clippy `-D warnings`, rustfmt, CSS reference 5/5, fixture boundary 5/5, Android API 37 ARM64 AVD와 iPhone 17 Pro iOS 26.2 Simulator의 동일 fixture/lifecycle 실행
- **판정:** 검토 중 `display:none` 하위 트리가 scroll owner로 남는 결함을 재현해 수정하고 회귀 테스트를 추가했다. 수정 뒤 미해결 P0/P1은 없다. 아래 표의 Simulator 미실행 경계는 제품 완료 주장에 포함하지 않는다.

| # | 공격/실패 경로 | 확인 결과와 근거 |
| --- | --- | --- |
| 1 | 문서 generation·style/layout revision이 섞인 snapshot이 scroll state까지 게시된다. | snapshot builder가 기존 `validate_revisions`를 먼저 호출하고 `ScrollStateRevision`을 render key에 별도 싣는다. 불일치는 `SnapshotMismatch`로 후보 snapshot을 반환하지 않는다. |
| 2 | `overflow` 누락이나 미지원 computed 값이 임의 기본값으로 바뀐다. | 양 축 computed 값 누락은 `MissingComputedProperty`를 반환한다. projection test가 `visible/clip/hidden/scroll/auto`를 각각 보존하는지 확인한다. |
| 3 | `display:none` owner 또는 그 숨겨진 자손이 scroll range를 만든다. | 새 regression test는 수정 전 owner 수 2로 실패했다. owner ancestry와 descendant lineage의 `display:none`을 건너뛰게 고친 뒤 부모 auto owner만 남고 range 0인 것을 확인했다. |
| 4 | `visible` border escape가 scroll owner를 만들거나 `clip`이 programmatic offset을 얻는다. | `visible_border_escape_paints_outside_without_creating_scroll_state`, `clip_has_no_programmatic_offset_owner`에서 각각 escape hit와 owner 부재를 확인한다. |
| 5 | border 폭을 client scrollport에 포함하거나 border가 box보다 커 음수 client 크기가 된다. | owner frame에서 양쪽 border를 제외하고 client 크기를 0 이상으로 제한한다. `CssRect`와 `RuntimeScrollContainer::new`가 비유한/역전 geometry를 거부한다. |
| 6 | 한 축의 content overflow가 다른 축 range까지 만든다. | x-only `(86,0)`, y-only `(0,74)`와 hidden 양 축 offset을 `x_only_y_only_and_hidden_offsets_match_the_requested_axis`에서 분리해 검증한다. |
| 7 | absolute child의 containing block이 scroll owner 밖인데 extent에 포함되거나 안쪽 child가 누락된다. | `absolute_containing_block_is_inside`가 positioned owner의 조상 경로를 확인한다. 고정 matrix의 absolute case는 x max 64이고 unrelated out-of-flow case는 owner가 되지 않는다. |
| 8 | 중간 `overflow:clip/hidden/auto`가 descendant range를 잘못 키우거나 ink-only shadow가 range로 들어온다. | extent scan은 각 중간 clip bounds와 scrollport client box로 교차한다. `box-shadow`는 runtime property 지원 전 fail-closed라 Simulator pass로 세지 않는다. |
| 9 | nested owner offset이 합쳐 저장되거나 inner clip이 자체 scroll에 따라 움직인다. | owner offsets는 독립 저장되고 render metadata는 각 clip의 외부 scroll ancestors만 기록한다. `runtime_nested_scroll_offsets_move_inner_clip_with_outer_scroll_only`와 앱 marker/clip assertion이 통과한다. |
| 10 | `html/body` overflow propagation이 일반 element owner 계산으로 섞인다. | Rust geometry tests가 visible html/body auto viewport owner와 html hidden/body auto 독립 body scrollport를 분리한다. 두 Simulator app route에서는 root/body scene을 아직 실행하지 않았다. |
| 11 | fixed descendant가 viewport scroll과 함께 이동하거나 scroll owner range를 늘린다. | `Fixed`는 extent에서 제외되고 scroll ancestor transform도 비운다. root/body Rust test와 앱의 고정 주황 control 위치 확인이 통과한다. |
| 12 | NaN, infinity, 음수 크기나 overflow 산술이 일부만 적용된다. | frame 검사, `CssRect::new`, finite scroll request 검사와 offset 합산 검사가 오류로 끝난다. 해당 값으로 부분 snapshot을 만들지 않는다. |
| 13 | 음수 또는 과대 offset이 범위 밖으로 저장되거나 한 축만 clamp된다. | 유한 입력을 축별 `[0,max]`로 clamp하고 새 immutable snapshot에 두 축을 함께 넣는다. geometry/lifecycle tests에서 shrink 후 0 clamp를 확인한다. |
| 14 | 중복 owner 요청이나 존재하지 않는 owner 요청이 다른 owner 상태까지 바꾼다. | 요청을 후보 vector에만 적용하고 duplicate/unknown owner를 오류로 반환한다. 호출자는 완성된 snapshot만 받는다. |
| 15 | 같은 document revision에서 이전 scroll key의 paint/hit 요청이 통과한다. | render key에 독립 scroll revision이 있고, 이전 revision의 `with_scroll_offsets`는 `StaleScrollRevision`으로 거부된다. FFI 수준 별도 stale 요청을 Simulator에서 주입하지는 않았다. |
| 16 | 동일 offset 재요청이 revision을 올리거나 실제 range 변경이 revision 없이 통과한다. | `reconcile_scroll_containers`는 geometry/owner/offset 차이로 revision을 정하고 no-op 요청은 유지한다. geometry tests에서 no-op과 stale request를 확인한다. |
| 17 | content growth가 offset을 reset하고 shrink·detach·reparent가 이전 offset을 남긴다. | Rust lifecycle test 및 양 플랫폼 fixture의 4단계 성장 보존·축소 clamp·owner 제거·reparent reset이 통과했다. 최종 draw는 scroll revision 5다. |
| 18 | reserve 실패 뒤 일부 owner map 또는 clip만 새 값으로 게시되고 worker 복구가 실패한다. | `scroll_reservation_failures_leave_no_partial_snapshot_and_recover`가 fault injection과 동일 builder 재시도를 확인한다. 이 fault는 Rust 경로에서 확인했고 앱/FFI에서 주입하지 않았다. |
| 19 | scroll revision 소진 또는 presentation sequence 변경 중 오래된 scene이 publish된다. | revision 증가와 presentation invalidation은 checked operation이며 stale sequence publish는 `ERR_STALE`로 거부한다. revision exhaustion의 별도 Simulator 재현은 하지 않았다. |
| 20 | fixture ABI가 default build에 들어가거나 ADB/Surface 준비 전 route가 시작한다. | null/invalid FFI arguments를 거부하고 default archive에서 C13.2 symbols/payload 부재, opt-in positive control 존재를 확인했다. 첫 AVD `-12`는 surface-size callback 전 route 시작이 원인이었고 surface-ready 경로 수정 뒤 API 37 실행이 통과했다. |

## 최종 범위

두 Simulator에서 같은 고정 matrix, visible border escape, paint/hit identity 및 lifecycle 네 단계를 실행했다. 이 결과는 app root/body, ink-only property, pixel readback 또는 app/FFI-level stale·failure injection을 대체하지 않는다. 자세한 이미지·로그·binary hash는 [실행 근거](evidence/c13-2-scroll-container-implementation-2026-10-15/README.md)에 있다.
