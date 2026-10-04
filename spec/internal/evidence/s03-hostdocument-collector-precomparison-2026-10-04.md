# S03.3 Rust HostDocument collector 구현 전 비교 기준

## 기준 입력과 현재 동작

비교 입력은 [고정 lifecycle fixture](../../../tests/fixtures/dom/s03/node-lifecycle-v1.rs)의 `TREE_CASES`다. 각 case는 노드 ID·부모·노드별 문자열 UTF-16 수, HostRoot 자식, wrapper root, external root와 예상 생존 ID를 지정한다.

현재 `HostDocument`는 `removeChild()` 뒤 분리 노드를 저장소에서 보존한다. runtime bridge도 외부 ID 매핑과 문자열 계수를 계속 보유한다. 이 동작은 S03.2 세션 수명 기준선이며, collector 입력이 없을 때 계속 유지해야 한다.

## 사전 판정 기준

| 입력·상태 | 예상 결과 |
| --- | --- |
| HostRoot 자식 및 하위 트리 | 전부 유지 |
| 약한 wrapper scan에서 살아 있다고 전달한 분리 노드 | 해당 노드의 부모·자식 연결 성분 유지 |
| external root로 전달한 분리 노드 | 해당 노드의 부모·자식 연결 성분 유지 |
| 어떤 root에도 닿지 않는 분리 연결 성분 | 전체 회수; Rust 노드, façade ID 매핑, 노드별 문자열 계수 제거 |
| 성공한 회수 | DocumentRevision·RenderTreeRevision 유지, node ID와 façade ID high-water mark 유지 |
| 다른 DocumentGeneration 또는 없는 root handle | 전체 거부; 문서·매핑·계수·revision 보존 |
| parent/child·HostRoot 관계가 손상된 그래프 | 일부 회수 없이 전체 거부 |
| runtime 내부 handle/문자열 회계 불일치 | core 변경 전에 전체 거부 |
| 제품 V8 weak registry가 제공되지 않은 경우 | 이 비교 단계로 GC/회수 완료를 주장하지 않음 |

fixture의 `expected_retained`가 실제 Rust 경로의 정답이고, 검증은 생존 ID·회수 ID·문자열 계수·revision·stale lookup을 직접 대조한다. 이 단계에서 측정하지 않는 시간·메모리 성능은 판정에 사용하지 않는다.

## 검증 경계

이번 구현은 Rust `HostDocument`와 runtime bridge의 결정적 API에 root handle 목록을 넣는 검사다. V8이 weak `Global`을 자동 reset하는 동작, 안전 지점 scan, C++ registry publish, Android·iOS GC 실행은 이 기준에서 입력으로 재현하지 않는다. 해당 경계는 기존 [0021 계약](../0021-s03-dom-node-lifecycle.md)의 후속 항목이다.
