# S03.3 DOM 노드 수명 계약 reference fixture 검증

이 기록은 [내부 계약 0021](../0021-s03-dom-node-lifecycle.md)의 결정성 Rust reference model과 문서 경계를 확인한다. 실제 `HostDocument` collector, V8 GC, 모바일 실행 결과로 해석하지 않는다.

## 비교 기준과 검증 범위

[고정 tree fixture](../../../../tests/fixtures/dom/s03/node-lifecycle-v1.rs)와 [quota fixture](../../../../tests/fixtures/dom/s03/node-lifecycle-limits-v1.rs)를 같은 모델 테스트에서 사용한다. 모델은 HostRoot·wrapper·external root가 살아 있는 연결 성분, parent/child 양방향 관계, facade ID와 JS 객체 identity 분리, 외부 lease, 한도, 생성·회수 전 commit 실패를 고정 입력으로 검사한다. 약한 handle이 비었는지는 테스트에서 V8 동작을 호출하지 않고 명시적으로 주입한다.

검토 중 확인한 stale empty wrapper 항목, parent/child 역참조 불일치, 세션·DocumentGeneration이 다른 root, 회수 뒤 façade ID 재사용 가능성을 fail-closed 및 고정 ID mapping 규칙으로 보완했다. 잘못된 root나 그래프, 저장소 상한 초과, 준비·commit 실패가 노드·wrapper registry·root·revision 상태 일부만 바꾸지 않는지 비교한다.

## 로컬 검증 결과

| 확인 | 결과 |
| --- | --- |
| `cargo fmt --all -- --check` | 통과 |
| `cargo test --locked -p spinon-runtime --test s03_node_lifecycle_contract` | 16개 통과, 실패 0개 |
| `cargo test --locked --workspace` | 148개 통과, 실패 0개; doc-test도 통과 |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과 |
| `git diff --check` | 통과 |

## 제품 검증에 남은 항목

- `HostDocument`의 실제 sweep, node/handle/string 회수 계수, DocumentGeneration 검증과 atomic commit은 아직 구현하지 않았다.
- C++ V8 adapter의 실제 weak `Global` 자동 reset, owner safe point scan, JS façade wrapper identity/ID 연결을 아직 실행하지 않았다.
- Android·iOS V8, real GC, 종료 중 비동기 완료, 16,384개 전체 scan 비용은 아직 측정하지 않았다.
- DOM EventTarget listener의 cross-heap closure cycle은 J12 범위다.

따라서 이 결과는 계약 oracle의 자기 일관성과 실패 사례만 확인한다. S03.3, J10, 공개 DOM 지원을 완료로 표시하지 않는다.
