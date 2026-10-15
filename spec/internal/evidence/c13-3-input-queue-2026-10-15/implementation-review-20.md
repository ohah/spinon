# C13.3 bounded input queue 구현 실패 관점 20개

## 검토 범위

- 대상은 `RuntimeScrollGestureInputQueue`, queue 입력 type, 해당 focused tests다.
- 기준은 사전 판정의 7개 invariant와 계약 0063의 Begin reservation·ordering·coalescing·saturation 규칙이다.
- 테스트 결과: queue focused 24/24, `spinon-render` 전체 72/72. native callback lock 경합이나 Android/iOS Simulator는 이 코드 slice 밖이다.

| # | 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 0 또는 과도한 capacity가 panic/OOM 경로를 만든다 | 0 regular/terminal을 거부하고 `usize::MAX` reserve 실패를 오류로 돌린다. 포화 fixture는 작은 고정 capacity만 쓴다. 통과. |
| 2 | Begin이 regular slot만 차지하고 terminal slot 예약 없이 active stream을 연다 | regular capacity와 terminal slot을 모두 확보한 뒤 reservation과 Begin을 추가한다. 각각의 capacity 실패에서 partial Begin이 남지 않는다. 통과. |
| 3 | terminal pool이 다 찬 상태의 새 Begin이 이전 reservation을 덮어쓴다 | 새 Begin을 `TerminalSlotsFull`로 거부하고 기존 identity/reservation을 유지한다. 통과. |
| 4 | gesture ID 0이나 동일 identity Begin이 여러 terminal slot을 점유한다 | 0 ID와 active duplicate identity를 거부하고 대기 queue/reservation 수를 확인한다. 통과. |
| 5 | session이 Begin 결과를 확정하기 전에 Move/terminal을 실행한다 | Begin dequeue 뒤 `complete_begin` 전 `pop_next`는 멈춘다. Tracking/Suppressed/Rejected를 구분하는 disposition으로 gate한다. 통과. |
| 6 | Begin 거부 뒤 그 gesture의 이미 대기한 PanBegan/Move/terminal이 늦게 실행된다 | `Rejected` disposition이 해당 regular entry와 예약 terminal slot을 함께 반환한다. 통과. |
| 7 | 두 번째 pointer가 suppressed contact로 남았는데 Begin 실패 처리와 함께 terminal slot도 반환된다 | `Suppressed` disposition은 reservation을 유지하고 해당 pointer End까지 slot을 보유한다. 통과. |
| 8 | PanBegan 중복 또는 PanBegan 전 Move가 session을 잘못 전이한다 | gesture당 PanBegan 한 번만 queue하고 그 전 Move를 거부한다. 통과. |
| 9 | queue regular event와 terminal slot의 sequence가 역행하거나 동률로 처리된다 | 모든 accepted stream의 증가 순서를 검사하고 regular/terminal head를 sequence로 비교한다. 통과. |
| 10 | regular queue가 꽉 차면 End가 Move보다 앞서거나 사라진다 | End는 예약 slot을 쓰고 앞선 regular Move 뒤에 dequeue된다. regular capacity가 찬 fixture에서 통과. |
| 11 | 동시에 대기한 여러 terminal이 slot 배열 순서로 잘못 배달된다 | 가장 작은 pending terminal sequence를 골라 distinct gesture terminal 순서를 확인한다. 통과. |
| 12 | Move 포화가 부분 Move를 넣거나 active gesture를 영구히 남긴다 | Move를 regular queue에 넣지 않고 reserved slot에 `InputQueueSaturated` Cancel을 기록한다. 기존 accepted event 뒤에 terminal이 오며 slot은 한 번 반환된다. 통과. |
| 13 | 같은 cumulative Move를 병합해 최종 translation/sequence를 잃는다 | 같은 identity·key·policy epoch의 tail만 마지막 cumulative sample로 바꾸고 coalesced count를 유지한다. 통과. |
| 14 | style/snapshot/policy 변경을 가로질러 Move를 합친다 | Move의 full `RuntimeRenderKey`와 policy epoch, identity가 다르면 별도 event가 된다. 통과. |
| 15 | 다른 gesture의 reserved End/Cancel을 넘어 tail Move를 합쳐 terminal 뒤로 재정렬한다 | 초기 적대 검토에서 regular tail만 비교하던 결함을 발견했다. `last_accepted_sequence`를 추가해 reserved terminal 등 다른 accepted event를 coalescing barrier로 만들고 4→5→6 sequence 회귀 검사를 추가했다. 수정 후 통과. |
| 16 | capacity에서 reject된 wheel sample을 accepted queue barrier로 잘못 세어 Move를 불필요하게 cancel한다 | last accepted sequence만 coalescing adjacency에 쓰므로 rejected wheel은 accepted Move를 갈라놓지 않는다. 통과. |
| 17 | wheel이 point/target/revision/policy/source/phase/unit/scale/modifier가 다른 sample과 합쳐진다 | complete coalesce key를 비교한다. point, target, scroll revision 및 raw metadata 변형 control에서 count가 증가하지 않는다. 통과. |
| 18 | wheel queue 포화에서 축 하나만 반영되거나 gesture도 cancel된다 | wheel 두 축 sample 전체를 reject하고 saturated diagnostic counter만 올린다. 예약 gesture 수와 queued sample을 유지한다. 통과. |
| 19 | 비유한 point/delta/raw 값, scale 0, 또는 key와 다른 basis revision이 queue를 오염시킨다 | invalid sample을 거부하고 queue 길이를 유지한다. 유효한 다음 sequence는 수락된다. 통과. |
| 20 | wheel 합산이 `f32` 범위를 넘어 기존 sample을 변형하거나 일부만 보존한다 | CSS/raw 합산은 `f64`로 range check한다. 여유 slot이면 두 원본 sample을 유지하고, 가득 차면 새 sample 전체를 거부한다. 두 경로 통과. |

## 남은 검증 경계

queue의 regular entries와 terminal reservations는 생성 시 크기를 예약하고 event payload는 고정 크기 값이다. 이 코드는 `Mutex`나 worker를 포함하지 않는다. 실제 Android/iOS callback이 짧은 synchronization 구간에서만 queue를 쓰고 dequeue 뒤 session/render를 lock 밖에서 호출하는지는 native adapter 단계에서 따로 측정해야 한다. 이번 검토는 그 host 동작이나 화면 결과를 증명하지 않는다.
