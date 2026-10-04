# S03.3 Rust HostDocument 회수 경로 검증

## 범위

고정 [node-lifecycle-v1 tree fixture](../../../tests/fixtures/dom/s03/node-lifecycle-v1.rs)를 실제 Rust `HostDocument`와 `HostDocumentBridge`에 적용했다. HostRoot 및 명시 root가 보존하는 노드, 회수 대상, 노드별 UTF-16 계수, 성공 sweep의 document/render revision 불변을 fixture 기대값과 대조한다.

이 검증은 Rust 내부 회수 메서드의 결정적 동작을 확인한다. 현재 V8 façade는 강한 wrapper `Map`을 사용하며 제품 실행 경로에서 회수 메서드를 부르지 않는다. V8 weak `Global` 자동 reset, safe-point scan, Android·iOS 실제 GC, 성능 측정과 메모리 프로파일링은 검증하지 않았다.

## 코드 경로

- Core는 전체 parent/child graph, HostRoot, owner 경계, root generation과 중복을 sweep 전에 검사한다. HashSet·HashMap·worklist·삭제 계획의 사전 확보가 실패하면 문서를 바꾸지 않는다.
- 계획과 commit 사이 document generation 또는 DOM/render revision이 달라지면 stale 계획을 거부한다. production caller는 owner safe point에서 root snapshot을 고정한 채 두 단계를 연속 실행해야 한다.
- Runtime bridge는 양방향 facade ID/handle 인덱스와 전체 UTF-16 계수를 먼저 검사하고, core commit 뒤 회수 대상 mapping과 이름·문자열·속성 계수를 제거한다.
- JS façade ID는 회수·실패 뒤에도 재사용하지 않는다. 발급 high-water mark와 예약 집합은 live mapping과 별개이며 예약은 최대 256개다. 생성 batch 안에서도 새 ID 발급 순서를 역행할 수 없다.

## 검증 결과

| 명령 | 결과 |
| --- | --- |
| `cargo fmt --all -- --check` | 통과 |
| `mise exec -- bun run test` | 통과: Bun façade 2개, CSS reference 3개, Rust workspace 전체 |
| `cargo test --locked --workspace` | 통과: core 37, FFI 4, layout 18, render 3, runtime 단위 42 및 lifecycle reference 16, style 18, style-layout spike 8, style-to-layout 8, style-to-render 9개 테스트 |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과 |
| `git diff --check` | 통과 |
| Clang C++20 `-fsyntax-only` raw-string include 확인 | 통과. `spinon_dom_facade.inc`의 C++ 문자열 문법만 검사 |

## 실패 경로 검토에서 보완한 부분

- 서로 다른 Owner를 연결한 간선, Text를 부모로 둔 손상 그래프, 순환·중복·누락 child/root 연결을 sweep 전에 거부한다.
- runtime 양방향 인덱스 또는 문자열 총계가 손상되면 core node를 지우기 전에 전체 회수를 거부한다.
- query callback을 통한 ID 예약 상한과 `QuotaExceededError` 접두어, 회수·실패 후 ID 비재사용, i32 공간 소진을 확인한다.
- 한 batch에서 더 큰 ID 뒤에 더 작은 미예약 ID를 제출해 high-water 순서를 역행할 수 없음을 확인한다.
- HostRoot·명시 root 생존, orphan component 회수, stale/foreign/duplicate root, stale 계획, quota 재사용, revision 불변을 확인한다.

초기 Clippy 검사에서 불리언 조건 두 곳을 지적해 수정했다. 해당 수정과 추가 실패 경로 테스트 뒤 전체 workspace Clippy 및 테스트를 다시 실행해 통과했다.

## 아직 필요한 검증

- 고정 V8 소스 checkout과 모바일용 archive가 이 작업 clone에 없어 Android/iOS 네이티브 앱 빌드 및 시뮬레이터 실행은 하지 않았다.
- C++에서 weak `Global` registry를 읽어 Rust collector를 owner safe point에 호출하는 연결이 없다. 현재 앱 동작은 세션 수명 노드/wrapper 보존이다.
- 실제 V8 wrapper identity·GC 재현, 반복 회수 memory profile, 16,384개 root scan 비용, 외부 root lease와 J12 listener cycle은 미검증이다.
