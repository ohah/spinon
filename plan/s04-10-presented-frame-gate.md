# S04.10 · 확인된 frame만 입력 대상으로 내보내는 Rust gate

**상태:** 플랫폼 callback에 앞서는 순수 Rust 후보 slice. 이 계획과 구현은 S04.10 또는 S05.2 완료 근거가 아니다.

## 목적과 범위

`RuntimeRenderSnapshot`은 GPU에 제출하기 전에 만들어진 불변 장면이다. 제출만 된 장면을 화면에 표시된 것으로 간주하지 않고, 플랫폼이 특정 submission의 실제 presentation을 확인한 뒤에만 confirmed-frame handle로 발행하는 작은 Rust 상태 모델을 추가한다. 확인 신호가 아직 없으면 새 입력 대상도 없다.

이 slice는 `spinon-render` 안에서 다음 자료형과 전이를 구현한다.

- registry identity, `FrameId`, surface generation을 함께 묶는 불투명 presentation token.
- 하나의 surface generation에서 제출했지만 아직 확인되지 않은 snapshot의 bounded registry.
- 플랫폼 adapter가 확인 통지를 준 frame만 보관하는 `RuntimeConfirmedFrame`.
- 입력 callback이 worker에 넘길 수 있도록 확인된 snapshot·token을 함께 복사해 두는 API.
- stale generation, 다른 registry의 token, 미등록·중복·낮은 frame ID 확인, capacity 초과, 명시적으로 폐기된 submission과 ID exhaustion의 오류.

플랫폼 callback, wgpu HAL, timestamp·clock-domain 비교, Android/iOS 앱 wiring은 구현하지 않는다. `Queue::present`, GPU 작업 완료, AVD/Simulator 화면 ready 및 오프스크린 readback을 presentation confirmation으로 바꾸지 않는다. S04.10 실제 callback과 S05 input routing은 별도 단계다.

## 비교 모델, fixture, 통과 기준

독립 브라우저 oracle이 필요한 화면 동작 변경이 아니다. 판정 oracle은 상태 불변식과 고정 전이 fixture다.

- **고정 입력:** 서로 다른 NodeId와 frame key를 가진 2개 이상의 `RuntimeRenderSnapshot`, generation `1→2`, 임의 순서의 submit/confirm/input 이벤트, `max_pending=2` 및 counter exhaustion test state.
- **관찰값:** token의 FrameId·surface generation, confirm 결과, 현재 confirmed token과 render key, 입력 시 복사한 snapshot identity와 `hit_test_css_point()` NodeId, pending 개수.
- **핵심 불변식:**
  1. submit은 pending만 만든다. confirm 전에는 `current_confirmed_frame()`이 비어 있거나 이전 confirmation을 유지한다.
  2. 유효한 최신 confirmation만 현재 frame을 전진시킨다. 낮은 ID·중복·알 수 없는 callback은 현재 frame을 바꾸지 않는다.
  3. generation이 바뀌면 confirmed 및 pending 상태를 모두 지우며, 이전 token은 이후 generation의 상태를 바꿀 수 없다.
  4. 다른 registry에서 받은 token은 FrameId와 generation이 같아도 거부한다.
  5. pending frame 개수는 생성자 상한을 넘지 않는다. 용량 초과 submission은 기존 frame을 덮어쓰지 않고 실패한다. higher FrameId confirmation 또는 caller가 더는 표시되지 않는다고 확정한 명시 폐기에서만 pending을 해제한다.
  6. 확인된 frame handle을 복사해 입력 queue에 보관하면 이후 새 confirmation이나 generation 교체가 그 복사본의 snapshot을 바꾸지 않는다.
  7. frame counter는 wrap 또는 재사용 없이 exhaustion 오류로 종료한다.
- **허용 기준:** 모든 positive/negative 전이가 고정 fixture 예상값과 정확히 같고, Rust focused tests와 `cargo fmt --all -- --check`가 통과한다.
- **한계:** 모델은 직렬 owner가 적용한 confirmation 중 가장 높은 FrameId를 보존한다. `current_confirmed_frame()`은 마지막으로 반영한 platform confirmation이며 native input의 과거 timestamp 시점에 실제 화면에 있던 frame을 판별한다는 뜻이 아니다. native input timestamp와 presentation timestamp의 같은 clock domain/order 보장은 별도 플랫폼 gate다. 이 모델만으로 S04.10 입력 연결을 통과 처리하지 않는다.

## API 경계와 오류 정책

- Registry 메서드는 `&mut self`를 사용한다. 내부 lock이나 callback 대기는 두지 않으며, 호출자는 UI/runtime host의 단일 직렬 owner를 보장한다.
- Token은 registry가 발급하고 registry identity를 소유한다. 필드 생성은 비공개로 두어 caller가 ID·generation을 임의 조합하지 못하게 한다.
- `max_pending`은 1..=8의 hard ceiling으로 제한한다. pending은 개수로 제한하며 각 `Arc<RuntimeRenderSnapshot>`의 크기를 byte 단위로 제한한다고 주장하지 않는다. 해당 hard ceiling에서 제출이 거부되면 host는 실제 presentation 확인 또는 안전한 lifecycle 교체 전까지 다음 snapshot을 제출하지 않는다.
- 각 surface의 frame 제출은 단일 owner에서 직렬화한다. FrameId는 1부터 registry lifetime 동안 단조 증가하고 surface generation 변경에서 재시작하지 않는다. FrameId를 더 발급할 수 없으면 실패한다.
- generation은 양수이며 교체 시 기존 값보다 커야 한다. resize 여부와 무관하게 host가 새 surface lifecycle을 발행할 때 전달한다.
- 낮은 FrameId callback을 받은 경우 현재 confirmed snapshot을 유지하고 stale 오류를 반환한다. 높은 FrameId confirmation 뒤에 이전 pending은 정리한다.
- adapter가 callback 도착 순서를 바꿀 수 있으면 실제 presentation order를 보장하는 방식으로 직렬화한 뒤 이 registry에 전달해야 한다. 이 slice는 timebase를 비교해 callback을 재정렬하지 않는다.
- caller는 정확한 token이 현재 pending이고 더 이상 presentation될 수 없음을 아는 경우에만 이를 명시 폐기할 수 있다. 확인된 frame이나 다른 registry/generation의 token 폐기는 허용하지 않는다. 폐기 뒤 늦은 confirm은 상태를 바꾸지 않고 실패한다.
- 확인되지 않은 input에 fallback node, submitted snapshot 또는 이전 document snapshot을 만들지 않는다.

## 실행 단계

1. 이 계획을 구현 전에 서로 다른 실패 관점 20개로 검토하고 P1/P2 결함을 반영한다.
2. `spinon-render`에 bounded `RuntimeConfirmedFrameRegistry`와 typed token/result/error를 추가한다.
3. unconfirmed, confirmed, duplicate/unknown, out-of-order, cross-registry, explicit discard, capacity, generation replace 및 overflow 경계를 서로 다른 test로 고정한다.
4. 입력 handoff clone이 기존 confirmed snapshot을 고정하는지 확인한다. renderer hit-test 구현은 재사용하고 여기서 좌표 규칙을 복제하지 않는다.
5. 새 구현 diff에 별도 실패 관점 20개를 적용하고 수정된 결함의 영향 관점을 다시 실행한다.
6. 기존 C13 scroll/input queue, default FFI artifact와 관련 crate 회귀를 확인한다. 이 Rust model만으로 Android/iOS runtime 또는 실제 presentation callback을 통과했다고 보고하지 않는다.
7. 내부 S04.10 문서·상태 대장에 후보 slice와 빠진 callback gate를 기록하고 S04.10/S05.2 체크를 미완료로 유지한다.
8. PR은 프로젝트 규칙에 따라 구현 검토와 다른 실패 관점 30개로 검토하고 `Rebase and merge`만 사용한다. 순수 Rust 변경에는 동작 화면을 첨부하지 않으며, simulator UI 확인이 없었다고 명시한다.

## 이 slice에서 완료로 주장하지 않는 항목

Android `VK_GOOGLE_display_timing` 또는 iOS `MTLDrawable` callback 연결, 실제 `presentID`/drawable timestamp, timestamp와 input event의 clock mapping, callback·input 동시성의 실제 플랫폼 실행, surface 교체 중 native callback, optical scanout, S04.10/S05.2/C13.3 전체, DOM event dispatch, 실기기 검증은 미완료로 남긴다.
