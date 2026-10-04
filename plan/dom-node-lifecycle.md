# S03.3 DOM 노드와 JS wrapper 수명 관리 계획

**상태:** 계획 중 · 미구현

**상위 작업:** S03, J10, R03

**현재 계약:** [S03.2 제한 DOM façade `0.1.0`](../spec/internal/0020-s03-dom-facade.md)

## 목적

S03.2는 Rust `HostDocument`를 기준으로 DOM façade를 검증했지만, 노드와 JavaScript wrapper를 세션이 끝날 때까지 유지한다. 이 수명 모델은 단순하고 동작이 예측 가능하지만, 노드를 반복 생성·제거하는 긴 세션에서는 제거된 노드가 계속 한도를 차지한다.

S03.3은 노드 트리의 소유권, JS wrapper identity, 분리 서브트리의 생존 조건, V8 GC와 Rust 회수의 경계, 한도 동작을 고정한 뒤 안전한 회수 경로를 검증한다. 이 계획과 실험은 공개 DOM 지원을 뜻하지 않는다.

## 현재 구현 동작

- `removeChild()`는 부모 연결만 끊는다. 제거된 노드와 자손은 `HostDocument`에 남고 재삽입할 수 있다.
- JS wrapper는 강한 `Map`에 보관되어 같은 handle은 같은 JS 객체로 돌아온다.
- 분리 노드와 wrapper 모두 V8 세션 종료 전에는 회수하지 않는다.
- 제거된 노드도 세션당 16,384개 노드 한도에 포함된다. 연결 중인 노드 수만 세는 정책은 아니다.
- 양수 노드 ID는 세션 안에서 재사용하지 않는다. stale handle이나 재사용 ID에 대한 회수 후 계약은 아직 없다.

위 동작은 현재 `0.1.0` 계약이다. 다음 모델을 구현한 것처럼 읽어서는 안 된다.

## 검토할 수명 모델

| 모델 | 동작 | 장점 | 주요 비용·위험 |
| --- | --- | --- | --- |
| 세션 수명 유지 | 현재처럼 노드와 wrapper를 세션 종료까지 보존 | 구현이 단순하고 wrapper identity가 명확함 | 반복 생성·제거가 노드·문자열 한도를 소모함 |
| 명시적 dispose | 앱 코드가 분리 노드의 해제를 직접 요청 | 회수 시점과 오류를 명시적으로 다룰 수 있음 | 표준 DOM과 다른 API이며, JS 참조가 남은 노드의 해제는 안전하게 보장하기 어려움 |
| V8 GC 연동 회수 | wrapper를 약한 참조로 캐시하고 문서 트리·살아 있는 JS 참조를 기준으로 분리 노드를 회수 | JS 참조 수명에 가까운 동작과 세션 중 자원 회수를 목표로 할 수 있음 | GC callback 수명·스레드·재진입, 자손 참조가 있는 분리 트리, 테스트 결정성, shutdown 순서를 정해야 함 |

우선 검토 후보는 V8 GC 연동 회수다. 아직 확정된 제품 정책이 아니므로 구현 전에 아래 결정을 계약에 반영한다. 명시적 dispose를 추가하거나 현재 세션 수명 모델을 유지하는 결정도 비교 근거와 함께 기록한다.

## 권고안 · 결정 전

- HostRoot에 연결된 노드는 문서 트리가 살린다. JavaScript에서 wrapper가 계속 참조되는 노드는 그 wrapper가 살린다.
- JS가 참조하는 노드가 분리 서브트리 안에 있으면, `parentNode`·`firstChild`·`nextSibling` 관찰 결과를 보존하도록 해당 분리 서브트리 전체를 유지한다. 어떤 wrapper도 참조하지 않는 분리 서브트리는 회수 후보가 된다.
- wrapper cache는 약한 참조로 바꾼다. wrapper가 JS에서 살아 있는 동안 같은 handle 조회는 같은 객체를 반환한다. wrapper가 수거된 뒤 연결된 native 노드를 다시 조회하면 새 wrapper를 만들 수 있다.
- 첫 회수 구현에서는 노드 ID를 세션 안에서 재사용하지 않는다. 나중에 재사용이 필요해지면 generation을 포함한 handle 계약을 먼저 추가한다.
- 한도는 누적 생성 횟수보다 살아 있는 노드·문자열의 resident 사용량을 기준으로 하는 안을 우선한다. 한도 직전 GC·회수 통지 drain 한 번을 요청하고도 초과하면 `QuotaExceededError`를 반환하는 방안을 비교한다. 강제 GC 지연과 실패의 결정성을 측정한 뒤 최종 선택한다.
- GC 중 callback은 HostDocument를 직접 변경하지 않고 회수 token만 남긴다. GC가 끝난 뒤 isolate 소유 실행 경로에서 token을 적용하는 방안을 우선한다. 세션 종료는 callback 등록을 먼저 닫고 pending token을 drain한 다음 Rust 문서를 파기한다.

위 항목은 다음 설계를 시작하기 위한 권고안이다. `0020`의 현재 시제품 계약을 바꾸지 않으며, 구현 계약을 고정하기 전에 각 항목을 검증·확정한다.

## 구현 전에 확정할 결정

1. **살아 있는 노드의 정의:** 권고안의 detached subtree 전체 보존이 최소 Node 관계 API와 맞는지 확인하고, HostRoot·wrapper root에서 정확히 도달성 추적할 범위를 정한다.
2. **wrapper identity:** wrapper가 JS에서 살아 있는 동안 같은 HostDocument handle 조회가 같은 객체를 반환하는지, wrapper가 수거된 뒤 재생성할 수 있는지 확정한다.
3. **GC와 Rust 변경 경계:** GC callback이 V8·Rust 객체를 직접 변경하지 않도록 할지, 회수 통지를 어느 isolate 소유 실행 경로에서 적용할지, 세션 종료와 경쟁할 때 폐기 순서를 정한다.
4. **handle 유효성:** 권고한 세션 내 ID 비재사용과 stale native handle의 실패 결과를 확정한다. ID를 재사용한다면 generation 검증을 포함한다.
5. **quota 의미:** 노드 수와 보존 문자열 수를 live resident 자원량으로 세는 안, 한도 직전 회수 시도, 실패 오류를 확정한다. GC 시점에 따른 성공·실패 차이와 지연 비용을 측정한다.
6. **결정적 검증:** 테스트 전용 GC 요청·회수 queue drain 제어를 둘지 정한다. 제품 공개 API로 노출하지 않는다.

## 검증 기준

계약과 구현은 아래 경로를 Android·iOS의 실제 V8 실행과 Rust/Bun 자동화에서 검증한다.

- 연결된 노드의 wrapper만 사라져도 HostDocument 노드와 트리가 유지되고, 다음 조회에서 유효한 wrapper를 만들 수 있다.
- 살아 있는 JS 참조가 있는 분리 노드는 조회·수정·재삽입 가능하다.
- 분리된 자손 wrapper가 살아 있는 경우 필요한 부모·형제 관계가 회수 때문에 깨지지 않는다.
- JS 참조가 없는 분리 서브트리는 합의한 안전 지점에서 회수되고, 재사용되지 않는 ID의 stale 접근은 실패 폐쇄한다.
- 같은 wrapper가 살아 있는 동안 handle 조회가 identity를 유지한다. 수거 이후 identity 보장은 계약에 적힌 범위를 따른다.
- 반복 create/remove/collect에서 노드·문자열·wrapper 자원 사용량이 합의한 한도 안에 머문다. 수거 전에 한도에 도달하는 경우 오류와 회수 재시도 규칙이 결정적이다.
- 세션 종료 중 pending GC 통지, callback 중복, 다른 세션 handle, Rust panic·할당 실패가 다른 노드나 세션을 손상하지 않는다.

완료하려면 버전 있는 내부 인터페이스 계약, 실패 경로 fixture, Rust·Bun 검증, Android·iOS 시뮬레이터의 실제 V8 실행 근거를 같은 변경 흐름에 연결한다. 구현 전에는 이 문서의 미결정 항목을 해결하고, S03.3과 J10을 완료로 표시하지 않는다.

## 범위 밖

`textContent` setter, `NodeList`·`HTMLCollection`, selector API, 전체 DOM/Web IDL 적합성, 여러 Owner의 동시 수정, 이벤트 listener 수명, CSS invalidation, layout·GPU 연결은 이 작업에 포함하지 않는다. 각각의 동작과 테스트는 해당 작업 계약에서 따로 정한다.
