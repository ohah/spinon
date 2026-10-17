# S04.10 확인 frame gate 계획 실패 관점 검토 · 2026-10-18

## 검토 대상

[확인 frame gate 계획](../../../plan/s04-10-presented-frame-gate.md)을 Rust 구현 전에 검토했다. 판정 대상은 순수 상태 모델이며 Android/iOS presentation callback 성공 여부가 아니다.

## 서로 다른 실패 관점 20개

| # | 실패 관점 | 계획 판정과 반영 |
| ---: | --- | --- |
| 1 | `Queue::present`가 반환한 submission을 presentation 확인으로 오인한다. | `submit`은 pending만 만든다. 오프스크린 readback·render-ready도 confirmation이 아니라고 명시했다. |
| 2 | 아직 첫 frame이 확인되지 않았는데 기본/제출 snapshot으로 hit-test한다. | confirmation 부재 때 frame handle과 fallback target을 만들지 않는 불변식을 고정했다. |
| 3 | 새 frame이 pending인 동안 기존 confirmed frame을 지워 입력이 끊긴다. | 이전 confirmation은 새 confirmation이 처리될 때까지 유지한다. |
| 4 | 더 높은 FrameId가 처리된 뒤 늦은 낮은 ID callback이 현재 상태를 되돌린다. | 낮은 ID는 stale 오류로 거부하고 현재 confirmed frame을 바꾸지 않는다. |
| 5 | 플랫폼 callback이 batch 또는 worker queue에서 순서가 바뀌어 적용된다. | 단일 owner에 직렬화한다. 실제 presentation 순서를 보장할 수 없는 adapter는 순서를 복구한 뒤 전달해야 하며 이 모델이 timestamp를 추측하지 않는다. |
| 6 | callback delivery가 늦어 실제 input event 시점과 마지막 confirmation이 다르다. | 모델의 `current_confirmed_frame`을 마지막 반영 신호로 한정했다. event timestamp에서 실제 화면 frame 선택은 미해결 platform gate로 남겼다. |
| 7 | presentation timestamp와 Android/iOS input timestamp의 clock domain이 다르다. | 비교하지 않는다. 양 clock의 동일 기준·보정은 별도 사전 계획과 플랫폼 근거가 필요하다고 적었다. |
| 8 | callback이 오지 않는 dropped/aborted frame이 pending capacity를 영구 점유한다. | higher ID confirmation 뒤 오래된 pending을 정리하고, host가 더는 표시될 수 없다고 확정한 token만 명시 폐기하도록 했다. 임의 timeout은 두지 않았다. |
| 9 | caller가 잘못된 token을 폐기해 아직 표시될 수 있는 frame을 잃는다. | 현재 pending token만 폐기할 수 있고 확인된 frame, foreign registry 및 stale generation은 거부하도록 했다. |
| 10 | 폐기 뒤 지연 callback이 오면 해당 snapshot을 다시 현재 상태로 발행한다. | 폐기 token은 pending에서 제거하며 이후 확인은 실패하고 상태를 바꾸지 않는다. |
| 11 | registry token의 frame ID와 generation만 비교해 다른 surface registry의 token을 받아들인다. | token에 불투명 registry identity를 넣고 registry identity까지 검사한다. |
| 12 | surface가 같은 크기로 재생성되어 기존 generation이 재사용된다. | identity는 크기가 아니라 host lifecycle generation에 묶고, generation 증가 시 pending·confirmed를 함께 폐기한다. |
| 13 | surface generation이 0, 같거나 낮은 값으로 입력되어 이전 snapshot을 재활성화한다. | generation은 양수이며 교체 값은 현재보다 커야 한다. 위반은 상태 변경 없이 실패한다. |
| 14 | 문서 generation과 surface generation을 동일 축으로 가정해 정상 snapshot을 잘못 거부한다. | 두 revision 축을 구별한다. registry는 surface generation만 소유하며 snapshot의 render key를 그대로 보존한다. |
| 15 | 입력 worker로 넘긴 frame handle이 이후 `Arc` mutation이나 snapshot 재작성으로 바뀐다. | `RuntimeRenderSnapshot`은 immutable이며 handoff는 그 snapshot과 token을 clone해 소유한다. |
| 16 | 이전 generation에서 캡처한 handle을 generation 교체 뒤 정상 DOM event로 dispatch한다. | 이 slice는 event dispatch를 구현하지 않는다. token은 stale provenance를 보존하며 소비자는 S05.1 stale 정책으로 처리해야 한다고 범위를 제한한다. |
| 17 | 확인된 frame hit-test에 기존 clip·viewport·paint-order 규칙과 다른 복사 구현을 쓴다. | hit-test는 기존 `RuntimeRenderSnapshot::hit_test_css_point()`를 그대로 재사용한다. |
| 18 | 높은 FrameId confirmation 뒤 낮은 ID pending을 유지해 queue가 불필요하게 찬다. | 더 높은 ID가 처리되면 낮은 pending을 정리한다. 이 정책은 per-surface submit이 단일 직렬 owner에서 순서대로 발행된다는 전제와 함께 기록했다. |
| 19 | ID overflow에서 0 또는 이미 쓴 ID로 wrap되어 다른 frame으로 alias한다. | FrameId는 1부터 단조 증가하고 exhaustion에서 명시 오류를 낸다. counter 경계는 고정 test state로 검사한다. |
| 20 | 고정 pending cap을 swapchain 수나 byte 상한이라고 과대해석한다. | hard ceiling은 frame 개수 상한일 뿐이다. snapshot별 byte bound나 플랫폼 swapchain 정책을 뜻하지 않는다고 계획에 적었다. |

## 판정

계획의 P1/P2 결함 3개(늦은 callback과 native input timestamp 의미 혼동, callback 누락 frame의 pending 회수, foreign registry token alias)는 구현 범위·오류 규칙을 명시해 반영했다. 남은 플랫폼 signal, clock mapping, event dispatch, optical scanout gate는 의도적으로 별도 단계다. 구현 전 계획 검토는 20개 모두 반영 완료다.

계획 적용 후 프로젝트 PR 검토 규칙이 30관점인 점과 맞추어 8번 단계의 PR 검토 수를 20에서 30으로 정정했다. 이는 계획 문서 수치 정합성 수정이며 구현 실패 관점 20개와 별도의 PR 검토를 대체하지 않는다.
