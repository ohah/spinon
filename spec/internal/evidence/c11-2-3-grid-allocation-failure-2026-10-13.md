# C11.2.3 Grid allocator 실패 주입 · 2026-10-13

## 범위

C11.2.3 dense·column auto-placement에서 실제 `Vec::try_reserve`와 `Vec::try_reserve_exact` 호출을 식별하고, 각 예약 지점에 한 번의 allocator 실패를 주입했다. 오류가 정확한 `NodeId`·property를 포함하는지, 호출이 panic이나 부분 성공으로 끝나지 않는지, 같은 worker에서 다음 정상 layout이 복구되는지 확인했다.

## 실행

```sh
cargo test --locked -p spinon-layout reservation_failure -- --nocapture
```

결과: 2 tests passed. 다섯 공용·sparse 경로와 C11.2.3의 추가 `ExpandedColumns` 경로를 포함한 여섯 예약 지점에서 실패 관측과 오류 반환을 확인했다.

| 예약 지점 | 오류 NodeId | property | allocator 실패 관측 | 결과 |
| --- | ---: | --- | --- | --- |
| `VisibleItemList` | 1 | `grid-placement-occupancy` | true | 오류 반환·동일 worker 복구 |
| `RowLockedCursor` | 1 | `grid-placement-occupancy` | true | 오류 반환·동일 worker 복구 |
| `ExpandedRows` | 1 | `grid-template-rows` | true | 오류 반환·동일 worker 복구 |
| `ExpandedColumns` | 1 | `grid-template-columns` | true | 오류 반환·동일 worker 복구 |
| `OccupancyRows` | 2 | `grid-placement-occupancy` | true | 오류 반환·동일 worker 복구 |
| `OccupancyIntervals` | 2 | `grid-placement-occupancy` | true | 오류 반환·동일 worker 복구 |

`ExpandedColumns` 전용 회귀 테스트는 column flow에서 implicit column을 실제 확장하는 입력을 사용한다. `GridAllocationFailure { node: 1, property: "grid-template-columns" }`를 확인한 다음 동일 worker·동일 engine에서 같은 입력의 정상 계산을 완료하고 item frame이 존재하는지 검사한다.

## 한계

이는 `spinon-layout`의 명시적 `Vec` 예약 지점에 대한 오류 주입 결과다. 전역 allocator OOM, `BTreeMap`·`BTreeSet` 등 다른 할당 경로, Android/iOS 시스템 allocator 실패 및 프로세스 전체의 OOM 복구를 보장하지 않는다.
