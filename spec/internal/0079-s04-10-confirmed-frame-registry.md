# 0079 · S04.10 확인 frame registry

**계약 버전:** `0.1.0-draft` · **상태:** 순수 Rust 후보 구현 · **공개 앱 API:** 아님 · **플랫폼 callback:** 미연결

## 목적과 범위

이 계약은 제출 snapshot과 플랫폼이 presentation을 확인한 snapshot을 분리하는 `spinon-render` 내부 Rust API를 정의한다. `RuntimeConfirmedFrameRegistry`는 직렬 owner가 발행한 surface generation과 frame ID를 확인 token에 묶고, 확인을 받은 snapshot만 `RuntimeConfirmedFrame`으로 반환한다. 이는 S04.10 완료 판정, Android/iOS callback 연결, compositor 표시 또는 광학 scanout 확인을 뜻하지 않는다.

## 자료형과 소유권

- `RuntimeSurfaceGeneration`은 0이 아닌 surface lifecycle 식별자다. 새 generation은 현재 값보다 커야 한다.
- `RuntimeFrameId`는 registry lifetime 동안 1부터 단조 증가하며, surface generation 교체에서 초기화하지 않는다. `u64::MAX` 다음 발급은 `FrameIdExhausted`로 실패한다.
- `RuntimePresentationToken`은 비공개 registry identity, FrameId와 submission 당시 surface generation을 함께 보유한다. 다른 registry의 token은 숫자 ID가 같아도 사용할 수 없다.
- `RuntimeConfirmedFrameRegistry`는 단일 직렬 owner가 `&mut self`로 호출한다. 내부 동기화·callback 대기·timebase 변환은 하지 않는다.
- pending capacity는 생성 시 1..=8로 제한한다. 상한은 frame 개수이며 snapshot 바이트 크기나 플랫폼 swapchain 크기 상한이 아니다.
- pending 및 confirmed snapshot은 `Arc<RuntimeRenderSnapshot>`으로 공유한다. 스냅샷의 render key와 좌표 의미를 수정·재계산하지 않는다.
- `RuntimeConfirmedFrame::snapshot_handle()`은 입력 worker가 snapshot을 소유하도록 공유 handle을 복제한다. `hit_test_css_point()`는 기존 snapshot hit-test를 재사용한다.

## 상태 전이

| 호출 | 성공 동작 | 거부/무효과 동작 |
| --- | --- | --- |
| `submit(snapshot)` | 현재 generation의 다음 FrameId와 token을 발급하고 snapshot을 bounded pending에 추가한다. | capacity 또는 ID가 소진되면 상태를 바꾸지 않고 실패한다. submit만으로 현재 confirmed frame은 바뀌지 않는다. |
| `confirm_presented(token)` | 현재 registry/generation의 pending token이 확인되면 해당 snapshot을 반환·현재 frame으로 저장한다. token보다 낮은 pending은 폐기하고 더 높은 pending은 보존한다. | 다른 registry, 오래된 generation, unknown/discarded, 중복 또는 최신보다 낮은 확인은 실패하며 현재 frame을 되돌리지 않는다. |
| `discard_unpresented(token)` | 더는 표시될 수 없다고 host가 보증한 현재 pending token 하나를 제거한다. | 이미 확인된 최신 frame, 최신 확인에서 밀려난 낮은 frame, foreign/stale/unknown token은 오류다. 폐기만으로 FrameId를 재사용하지 않는다. |
| `advance_surface_generation(next)` | 이전 generation의 current/pending을 비우고 더 높은 generation으로 전환한다. FrameId counter는 유지한다. | 0은 생성할 수 없으며 같거나 낮은 generation은 상태를 바꾸지 않는다. |
| `current_confirmed_frame()` | 마지막으로 반영한 확인 frame의 불변 handle을 복제한다. | 아직 확인되지 않았거나 generation 교체 후에는 `None`이다. |

확인 token은 플랫폼 adapter가 특정 submission의 frame별 presentation 확인을 받았을 때만 전달한다. `Queue::present` 반환, GPU 작업 완료, draw 완료, offscreen readback, surface 획득 성공 또는 화면이 준비됐다는 앱 상태를 이 callback으로 간주하지 않는다. 입력 event timestamp 시점에 실제 보였던 frame을 선택하려면 platform callback과 event clock의 동일 clock domain, callback ordering 및 serialized handoff를 별도로 검증해야 한다. 이 registry는 마지막 callback 상태만 보유하며 과거 화면을 추정하지 않는다.

## 오류 계약

`RuntimeConfirmedFrameError`는 capacity 설정·할당·포화, FrameId 고갈, foreign registry, stale surface generation, generation 역행, unknown/discarded frame, superseded frame, duplicate/old confirmation과 이미 확인된 최신 frame discard를 구분한다. 모든 거부 경로는 현재 confirmed snapshot과 허용된 pending 상태를 임의로 덮어쓰지 않는다. `discard_unpresented`의 안전성은 호출자가 해당 submission이 더는 표시될 수 없음을 알고 있다는 계약에 의존하며 registry가 플랫폼에서 이를 증명하지 않는다.

## 구현 및 검증 범위

[구현 PR #291](https://github.com/ohah/spinon/pull/291)은 Rebase and merge로 병합됐다(merge SHA `e906aed109a7af2b9f7ccfd0cd64ba4b806f5869`). [구현 근거와 한계](./evidence/s04-10-presented-frame-gate-2026-10-18/README.md) 및 [구현 실패 관점 20개](./evidence/s04-10-presented-frame-gate-implementation-review-20-2026-10-18.md)를 참조한다. 이 계약에 연결된 pure Rust tests는 상태 불변식만 확인한다. Android/iOS frame callback, input callback 순서, 앱 route, simulator UI, real device, hardware GPU, compositor 이후 표시, optical scanout은 검증하지 않았다. S04.10과 S05.2는 미완료다.
