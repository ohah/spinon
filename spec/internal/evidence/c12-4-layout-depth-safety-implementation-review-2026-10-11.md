# C12.4 runtime 깊이 안전성 구현 실패 관점 검토

**대상:** 반복형 깊이 사전 검사, Taffy 입력 검사, CSS worker stack, 오류/복구 상태, Android·iOS 진단 경로.  
**실행 조건:** Android API 37 AVD와 iPhone 17 Pro / iOS 26.2 Simulator. Android 실기기는 사용하지 않았다.  
**계획 검토와 구분:** [구현 전 계획 검토](c12-4-layout-depth-safety-plan-review-2026-10-11.md)는 이번 코드·앱 검토에 재사용하지 않았다.

| # | 실패 관점 | 확인과 판정 |
| ---: | --- | --- |
| 1 | 루트 깊이 정의가 호출자마다 0 또는 1로 갈리는가 | Taffy preflight와 runtime preflight 모두 HostRoot 바로 아래 첫 node를 depth 1로 시작한다. Rust·양 플랫폼 앱에서 이 정의를 사용했다. |
| 2 | 허용 최댓값 바로 아래에서 off-by-one 거부가 나는가 | 1,024 HostDocument depth의 cascade style, layout frame, paint box를 모두 확인했다. 통과. |
| 3 | 첫 초과 node 대신 부모나 마지막 정상 node가 오류로 보고되는가 | 1,025 요청은 `tree_depth_exceeded`, depth 1025, maximum 1024, node 1034를 반환했다. Rust 테스트도 overflow handle의 ID를 비교한다. |
| 4 | 같은 node 수의 넓은 트리를 잘못 거부하는가 | 직접 `LayoutInput` 1,025-node shallow graph 대조가 통과한다. 거부 기준은 총 node 수가 아니다. |
| 5 | 사전 검사 자체가 깊은 트리에서 재귀 stack을 소모하는가 | runtime preflight는 pending vector를 쓰는 반복 순회다. 실제 1,024-depth 앱 경로가 양쪽 시뮬레이터에서 통과했다. |
| 6 | Stylo·author stylesheet 단계가 깊이 검사를 앞질러 stack overflow를 일으키는가 | `compute_request_with_profile`은 depth preflight를 stylesheet 수집과 cascade 앞에서 호출한다. 실제 1,025 입력은 cascade/layout 전에 명시적으로 실패했다. |
| 7 | runtime 밖의 직접 Taffy 진입으로 제한을 우회하는가 | `TaffyLayoutEngine::compute`도 tree validation에서 동일한 1,024 cap을 적용한다. 1,025 deep direct input은 Taffy 실행 전에 typed error를 반환한다. |
| 8 | root 여러 개를 하나의 연속 깊이처럼 합산하는가 | runtime preflight는 각 root child를 depth 1로 넣는다. 현재 runtime painter는 여러 host root를 별도로 거부하므로 multi-root 성공 동작으로 확대하지 않는다. |
| 9 | snapshot에서 자식을 읽지 못할 때 초과 검사가 조용히 통과하는가 | node 누락과 child-list 누락을 서로 다른 오류로 반환하도록 fail-closed 처리했다. 공개 생성 경로로 해당 malformed snapshot을 주입하는 테스트는 아직 없다. |
| 10 | 순환/중복 edge 때문에 preflight가 무한 반복하는가 | visited set이 같은 NodeId 재방문을 차단하며, 이후 layout topology validator가 유효하지 않은 graph를 거부한다. HostDocument의 public commit도 잘못된 cycle을 원자적으로 거부한다. |
| 11 | detached graph가 화면 layout 깊이에 섞이는가 | preflight는 HostRoot 직속 subtree만 순회한다. 복구 시 제거한 깊은 subtree는 root traversal에서 제외된다. 메모리 reclamation/GC는 별도 항목이다. |
| 12 | 초과 오류가 기존 FFI JSON schema를 깨뜨리는가 | layout 오류는 기존 `code`, `nodeId`, `property`만 유지하고 `tree_depth_exceeded`, 초과 node, `null` property를 전달한다. 새 필드를 추가하지 않았다. |
| 13 | 초과 revision에서 성공한 이전 frame/cache가 새 결과로 남는가 | 실패 상태는 completed layout/cascade를 비우고 runtime worker가 성공 cache를 해제한다. Rust 회귀 테스트는 실패 cascade에 completed 값이 없고 실패 layout에도 completed frame이 없음을 확인한다. |
| 14 | 복구 테스트가 최초 snapshot 재등록으로 새 revision을 가장하는가 | 검토에서 기존 테스트가 최초 shallow snapshot을 다시 등록하는 허점을 찾아 수정했다. 이제 동일 HostDocument에 깊은 subtree를 붙인 뒤 제거해 실제 새 revision을 만든다. |
| 15 | 실패 뒤 worker를 재생성해야만 얕은 문서가 되는가 | 같은 coordinator/runtime worker에 새 revision을 등록한다. 2-node cascade를 다시 계산한 수가 2임을 단위 테스트에서 확인했다. |
| 16 | 1,024 통과를 frame 누락이나 최심부 손실로 오인하는가 | Rust runtime test는 전체 frame/paint box 수와 최심부 NodeId·좌표를 대조한다. Android/iOS runtime report는 1,033 boxes 및 cascade style count를 확인한다. |
| 17 | 8 MiB 설정이 앱 runtime에서 실제 사용되지 않는 설정인가 | CSS cascade worker `thread::Builder::stack_size(8 MiB)` 설정을 코드에서 확인하고, 그 worker에서 실행되는 V8 앱 경로가 Android AVD와 iOS Simulator에서 성공했다. 이는 실기기 최소 stack 보장이 아니다. |
| 18 | Android 물리 기기 결과로 잘못 분류되는가 | `adb devices`에서 `emulator-5554`만 online임을 확인했다. Android 17/API 37 arm64 AVD 결과만 기록했고 physical device claim은 하지 않았다. |
| 19 | 실패 뒤 scene은 계산됐지만 화면 갱신이 누락되는가 | Android 복구 완료 뒤 `requestDraw()`가 호출되는지 확인하고 캡처에서 색상 scene 및 성공 상태를 봤다. iOS 캡처에는 색상 scene이 표시되고 앱 log에 복구 완료가 있다. 이 캡처는 pixel oracle이 아니다. |
| 20 | FFI 보고서 buffer 한도/할당 실패가 무시되는가 | JNI와 Objective-C++는 256 KiB output buffer를 heap의 nothrow allocation으로 요청하고 할당 실패를 상태 오류로 처리한다. 현재 최대-depth report는 약 49.7 KiB였다. 의도적 할당 실패 주입은 하지 않았다. |

## 발견 후 수정

1. runtime 복구 단위 테스트가 최초 snapshot을 재등록하던 문제를 발견했다. 같은 `HostDocument`에 depth 1,025 subtree를 연결·제거하고 증가한 revision을 등록하도록 변경했으며 재계산된 style 수 2를 확인한다.
2. malformed snapshot에서 `snapshot.children(handle)?`의 `None`을 빈 자식 목록으로 오해해 사전 검사를 우회할 수 있었다. 누락 node/자식 목록을 명시 오류로 처리하도록 바꿨다.
3. 앱 진단은 초기에는 없는 `document.body` API와 별도의 top-level root를 가정해 실제 HostDocument fixture와 맞지 않았다. 기존 `c124-root`에 붙이도록 수정했다.
4. 첫 AVD 진단 후 scene 재그리기 요청이 없어 캡처가 검게 남았다. 복구 성공 때 `requestDraw()`를 호출하도록 수정하고 색상 scene 캡처를 다시 확인했다.

## 남은 제한

- allocation failure injection, malformed snapshot injection, Android 실기기, 다중 session 메모리 비용, Android scene pixel 비교와 iOS Metal surface pixel readback은 검증하지 않았다.
- Android AVD의 ANGLE/SwiftShader는 GPU 성능·하드웨어 정확성 검증이 아니다.
- 현재 결론은 두 시뮬레이터에서 8 MiB worker로 depth 1,024 성공·1,025 명시 거부·새 revision 복구가 된다는 범위다. C12.4 완료 판정은 별도 미완료다.
