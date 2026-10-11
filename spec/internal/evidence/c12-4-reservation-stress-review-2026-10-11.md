# C12.4 painter 예약 실패·깊이 스트레스 검토

**검토 대상:** `spinon-style-to-render`의 C12.4 painter `Vec` 예약 경계와 Android matrix 실행 입구  
**기준:** [내부 계약 0058](../0058-c12-4-stacking-context.md) · [구현 계획](../../../plan/c12-4-stacking-context.md)  
**실행 근거:** [Android·iOS 런타임 기록](c12-4-stacking-runtime-2026-10-11/README.md) · [Matrix 재실행 로그](c12-4-stacking-runtime-2026-10-11/android-api37-emulator-matrix-reserve-rerun-2026-10-11.log)

## 적대적 검토

1. **예약 실패 인덱스가 한 칸 밀리는가?** 성공 경로의 실제 capacity-growth 예약 지점 순서를 기록하고 각 1-based 인덱스를 독립 주입했다. 각 호출이 그 지점에서 중단되고 같은 stage의 `AllocationFailed`를 반환한다.
2. **추가 용량이 0인데 가짜 할당 실패를 만들 수 있는가?** 0개 예약은 실패 인덱스에 포함하지 않고 테스트했다.
3. **남은 capacity가 있는데 할당 실패로 오진하는가?** `additional <= capacity - len`인 경우 주입 횟수가 증가하지 않는 negative control을 추가했다.
4. **용량 증가가 필요한 경우만 주입되는가?** capacity를 채운 뒤 다음 원소를 예약해 실패를 확인했다.
5. **시스템 경로가 실제 `Vec::try_reserve`를 계속 쓰는가?** 운영용 `SystemReserve` 구현은 표준 `try_reserve`의 오류를 기존 `AllocationFailed { stage }`로 변환한다.
6. **테스트 seam이 release 코드에 들어가는가?** 주입 구현은 `cfg(test)`이며 crate 공개 API, FFI, 앱 작성 API에 노출되지 않는다.
7. **동적 dispatch나 전역 mutable fault 상태가 추가되는가?** generic 내부 trait를 호출별로 전달한다. 전역 상태·잠금·공유 실패 카운터는 없다.
8. **예약 stage가 합쳐져 원인 식별을 잃는가?** fixture는 root context, traversal, document nodes/children, stacking contexts, negative/zero/positive buckets, positioned auto, flex item/order, block, paint traversal/nodes, render boxes의 15개 stage를 각각 포함한다.
9. **상태 표본이 특정 bucket을 놓치는가?** fixture에 음수·0·양수 context, positioned auto, static Block, static Flex item, Flex order를 포함하고 관찰 stage 집합을 정확히 비교한다.
10. **예약 실패 후 부분 snapshot이 밖으로 나오는가?** 반환은 `Err`이고 snapshot은 생성되지 않는다. builder는 입력 document/style/layout을 immutable borrow로 읽으며 임시 vector는 소유권 해제 경로에서 drop된다.
11. **각 `Vec::push` 전에 capacity를 확보하는가?** document nodes, contexts, pending, child handles, flex order, context buckets, paint nodes와 render boxes는 각 stage에서 예약한다. `pending.extend`는 확장 길이를 먼저 예약한다.
12. **paint action 초기 예약이 깊이에서 부족한가?** 5만 개의 중첩 context root를 가진 합성 입력에서 모든 node ID의 순서와 개수를 확인했다. 반복형 flatten이 재귀 호출 없이 결과를 냈다.
13. **넓은 형제 fanout에서 pending action queue가 부족한가?** root 아래 5만 개 child context와 root box를 구성해 예약된 문서 노드 수 경계에서 순서와 전체 개수를 확인했다.
14. **깊은 입력에서 결과 순서가 조용히 잘리는가?** 깊은 입력의 모든 paint node가 tree order대로 나오는지 전체 sequence를 비교한다.
15. **넓은 입력에서 tie order가 바뀌는가?** fanout 전체 결과를 context tree order와 비교한다.
16. **size 계산의 overflow가 capacity 검사에 섞이는가?** fault injector는 `capacity - len`의 saturating 차와 `additional`을 비교하며, 실제 크기 요청의 최종 overflow 처리는 시스템 `try_reserve`가 `TryReserveError`로 반환한다.
17. **다른 fallible 예약 callsite가 검토 밖에 있는가?** C12.4 painter 모듈의 명시적 `try_reserve`를 검색하고, 선택 fixture가 각 growth stage를 실제 통과하는지 stage 집합 assertion으로 고정했다.
18. **모든 OOM을 복구한다고 과장하는가?** 아니다. 이 검증은 painter의 명시적 `Vec::try_reserve` 경계다. `BTreeMap`·`BTreeSet`, 문자열 복사 등 Rust의 다른 infallible allocation 실패 복구를 증명하지 않는다. 내부 계약의 이 한계는 유지한다.
19. **깊이 검증을 전체 문서 그래프 검증으로 과장하는가?** 아니다. 5만 단계는 paint-context flattener의 합성 입력이다. HostDocument 생성부터 Stylo cascade와 Taffy layout까지의 깊은 end-to-end graph stress는 남아 있다.
20. **Android 결과를 surface pixel 또는 하드웨어 GPU 결과로 확대하는가?** 아니다. Android 17/API 37.1 AVD에서 새 APK의 C12.4 runtime matrix 26/26 장면을 재실행했다. AVD는 ANGLE/SwiftShader 소프트웨어 경로다. 각 장면별 surface pixel, iOS Metal readback, 하드웨어 GPU 정확성을 검증하지 않는다.

## 결론과 남은 경계

명시적 painter `Vec::try_reserve` growth 실패 매핑과 합성 paint-context 깊이·폭 스트레스는 검증했다. 실제 Rust allocator OOM을 재현한 실험은 아니며, global allocator failure recovery를 보장하지 않는다. 전체 HostDocument→Stylo→Taffy 깊은 그래프, Android scene별 GPU surface pixel, iOS Metal surface pixel readback은 별도 게이트로 남긴다. C12.4는 미완료 상태를 유지한다.

## 후속 실행 정정

이 문서의 “전체 HostDocument→Stylo→Taffy 깊은 그래프”와 platform pixel 표면은 당시 남은 항목이었다. 후속으로 Android API 37 AVD와 iOS 26.2 Simulator에서 1,024 depth 성공·1,025 거부·새 revision 복구, Android `PixelCopy` 21 scene·22 sample, iOS Metal reverse Flex 8 sample을 확인했다. worker stack probe는 pthread bounds/mincore 순간 표본이며 stack-only high-water나 allocator OOM 복구가 아니다. 최신 근거는 [깊이 실행](./c12-4-layout-depth-safety-platforms-2026-10-11.md), [Android surface](./c12-4-android-surface-pixels-2026-10-11/), [iOS Metal surface](./c12-4-ios-metal-surface-pixels-2026-10-11/), [worker stack](./c12-4-worker-stack-residency-2026-10-11/README.md)에 있다.

## 후속 pixel·session 측정

이 검토 당시 남아 있던 Android surface pixel은 이후 Android AVD `PixelCopy` 21 scene·22 sample exact match로, iOS Metal surface pixel은 실제 surface의 reverse Flex 8 sample exact match로 별도 확인했다. Android AVD와 iOS Simulator의 동시 session process memory 0·1·2·4·8단계도 측정했고, 별도 iOS runtime matrix는 26/26 scene 결과를 확인했다. 이 후속 실행은 global allocator failure recovery를 추가로 증명하지 않으며, 실기기 메모리/thread별 stack 귀속은 남아 있다. [통합 evidence](./c12-4-stacking-runtime-2026-10-11/README.md)를 참조한다.
