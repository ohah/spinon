# S04.10 확인 frame registry 구현 실패 관점 검토 · 2026-10-18

## 검토 대상

`crates/spinon-render/src/runtime/presented_frame_registry.rs`와 해당 단위 테스트를 대상으로 계획 검토와 구분되는 새 구현 검토를 수행했다. 검토 뒤 발견사항을 수정하고 영향 관점을 다시 확인했다.

## 서로 다른 실패 관점 20개

| # | 실패 관점 | 판정과 코드 근거 |
| ---: | --- | --- |
| 1 | submit 직후 아직 표시되지 않은 snapshot이 입력 target으로 노출된다. | submit은 pending만 만든다. 확인 전 current는 `None` 또는 이전 확인을 유지한다. test `submit_does_not_publish_unconfirmed_frame_or_hit_target` 통과. |
| 2 | 새 frame이 대기 중이라는 이유로 이전 confirmed snapshot이 먼저 비워진다. | submit은 current를 건드리지 않는다. test `newer_pending_frame_keeps_previous_confirmation_until_it_is_confirmed` 통과. |
| 3 | 낮은 FrameId의 늦은 confirmation이 더 높은 현재 frame으로 회귀시킨다. | 최신 ID 이하 확인을 상태 변경 전에 거부한다. `older_confirmation_after_newer_confirmation_cannot_regress_current_frame` 통과. |
| 4 | 같은 token의 duplicate confirmation이 pending/current 상태를 다시 소비한다. | 두 번째 확인은 거부되고 현재 token은 그대로다. `duplicate_confirmation_is_rejected_without_mutating_current_frame` 통과. |
| 5 | registry에 발급되지 않은 더 높은 숫자 ID를 caller가 구성해 확인한다. | token 필드는 private이며 public 생성자는 registry가 소유한다. pending에 없는 인증 token은 성공하지 않는다. |
| 6 | 숫자 FrameId와 generation이 같은 foreign registry token이 받아들여진다. | token에 `Arc<()>` registry identity를 보유하고 `Arc::ptr_eq`로 검사한다. 같은 숫자 ID token negative test 통과. |
| 7 | registry가 파괴된 뒤 allocator가 같은 주소를 재사용해 살아 있는 token이 다른 registry에 alias한다. | token이 identity `Arc`의 strong reference를 유지하므로 같은 identity 할당은 token 수명 중 살아 있다. |
| 8 | generation 0, 현재 값 또는 낮은 값이 이전 surface를 재활성화한다. | 0은 `new`에서 거부되고 advance는 strictly increasing 조건을 상태 변경 전에 검사한다. |
| 9 | surface 교체 뒤 이전 current/pending token이 새 surface에서 입력에 계속 사용된다. | advance가 current/pending을 제거하고 이전 generation token은 제출·확인·폐기 경로에서 거부된다. |
| 10 | generation 교체가 frame counter를 초기화해 이전 FrameId를 재사용한다. | `next_frame_id`는 advance에서 바꾸지 않는다. `frame_ids_remain_monotonic_across_surface_generations` 통과. |
| 11 | `u64::MAX` 이후 FrameId가 0으로 wrap하거나 재사용된다. | checked increment 실패를 `None`으로 보존하고 이후 제출을 `FrameIdExhausted`로 거부한다. |
| 12 | pending cap을 초과할 때 기존 snapshot을 조용히 밀어내거나 Vec를 확장한다. | capacity 검사와 사전 예약된 bounded Vec로 거부한다. `pending_capacity_rejects_without_overwriting_existing_tokens` 통과. |
| 13 | host가 더는 표시될 수 없는 submission의 명시 폐기를 놓쳐 bounded queue가 영구히 찬다. | higher confirmation이 더 낮은 pending을 정리하고, 명시 discard가 슬롯을 해제한다. 임의 timeout은 사용하지 않는다. |
| 14 | 이미 확인된 최신 frame을 pending이라고 잘못 폐기한다. | 최신 ID는 `FrameAlreadyConfirmed`; 더 낮은 superseded ID는 새 `FrameSuperseded`로 분리했다. 관련 경계 test 통과. |
| 15 | 더 높은 confirmation에서 밀린 낮은 frame을 명시 폐기해 이미 종료된 상태를 성공으로 오인한다. | 낮은 ID는 `FrameSuperseded`로 거부하며 pending count/current를 바꾸지 않는다. 검토 중 발견해 수정 후 재확인했다. |
| 16 | 입력 worker가 frame handle을 받은 뒤 registry advance/update 때문에 snapshot이 바뀐다. | `Arc<RuntimeRenderSnapshot>`을 immutable handle로 복제하며 source identity 유지 test 통과. |
| 17 | confirmed frame에서 별도 hit-test 구현이 clip·edge·paint order와 불일치한다. | `RuntimeConfirmedFrame`은 기존 `RuntimeRenderSnapshot::hit_test_css_point()`를 위임 호출한다. point positive/negative test 통과. |
| 18 | snapshot의 document/render revision이 현재 host revision과 다르지만 registry가 이를 알아서 보장한다고 오해한다. | registry는 현재 document revision owner가 아니다. contract는 render key를 보존하고 S05.1 stale dispatch 정책을 별도 한계로 둔다. |
| 19 | 동시 callback이 `&mut self`를 병렬로 바꾸거나 timestamp로 상태를 임의 정렬한다. | 호출은 단일 serialized owner 계약이다. 내부 lock, async, clock 비교를 추가하지 않았고 실제 platform serialization은 미검증으로 남겼다. |
| 20 | 순수 Rust 통과가 Android/iOS presentation callback, 실제 화면, scanout 또는 입력 event 연결을 증명한다고 과장된다. | `0079`와 status/evidence에서 API·앱 route·simulator UI·기기·compositor 검증을 모두 분리했고 S04.10/S05.2는 미완료로 유지했다. |

## 발견 수정과 재확인

초기 검토에서 `RuntimeConfirmedFrame: PartialEq`가 대형 snapshot의 깊은 비교 API를 노출하는 점과 superseded pending discard를 `FrameAlreadyConfirmed`로 오분류하는 점을 발견했다. `PartialEq`를 제거하고 superseded 오류를 분리했다. 추가로 surface generation 이후 FrameId 단조성을 고정하는 test를 추가했다. 영향 관점 10·14·15·16을 다시 확인했고 renderer 전체 테스트, FFI 회귀, workspace Clippy와 rustfmt를 통과했다.

## 판정

순수 Rust 확인-frame gate 후보의 상태 불변식은 테스트 범위에서 통과했다. 플랫폼 presentation callback과 event clock/order 통합은 붙이지 않았으며 이 구현 리뷰를 S04.10 완료나 표시된 frame 입력 검증으로 확대하지 않는다.
