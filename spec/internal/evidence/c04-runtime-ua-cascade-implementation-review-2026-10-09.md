# C04.8 · Runtime UA cascade 구현 후 실패 관점 검토

**검토 기준:** 2026-10-09 작업 트리, 기준 커밋 `57aa140` 이후 C04.8 변경과 Android·iOS Simulator 실행 결과
**결과:** 발견한 보고서/UI 불일치와 계약 설명 불일치를 수정했다. 남은 항목은 아래 마지막 절의 명시적 제품 범위 밖 검증이다.

| # | 공격 관점 | 확인 근거 | 결과 |
|---:|---|---|---|
| 1 | 앱 mount root를 HTML `documentElement`처럼 취급해 document-root selector·상태를 잘못 부여하는가 | `new_html_fragment_child_shared`가 별도 fragment-root adapter를 만들고 기존 document-root 생성자를 보존한다. | 통과 |
| 2 | fragment child가 CSS `:root` 선택자와 일반 element selector에 동시에 잘못 매칭되는가 | Stylo adapter 테스트에서 `main:root`는 불일치하고 일반 `main#app-root.shell`은 일치한다. | 통과 |
| 3 | 최상위 `<span>`에 문서 루트 `display` 보정이 적용되는가 | root `div`/`span` 계산 테스트와 실제 V8 probe에서 `span` UA 값 검증이 통과한다. | 통과 |
| 4 | HostRoot 직속 요소가 여럿일 때 하나만 계산되거나 순서가 바뀌는가 | root 목록을 snapshot 순서로 독립 계산하고 다중 root 테스트·플랫폼 probe에서 4 roots를 확인한다. | 통과 |
| 5 | top-level Text 또는 root 오류에서 앞서 계산된 일부 스타일이 공개되는가 | 직속 Text가 전체 요청을 실패시키는 단위 테스트가 있고, 실패 결과는 root별 부분 출력을 반환하지 않는다. | 통과 |
| 6 | 다른 generation의 handle 또는 선택 subtree 밖 node를 view가 노출하는가 | invalid root·foreign generation·selected subtree 경계 테스트에서 거부 또는 숨김을 확인한다. | 통과 |
| 7 | 기존 `style` attribute가 UA 값에 덮이거나 잘못된 선언이 전체 계산을 실패시키는가 | inline origin precedence와 parser diagnostics 테스트, Android·iOS 실제 V8 probe의 inline style·diagnostics 검증이 통과한다. | 통과 |
| 8 | JavaScript가 DOM 변경 뒤 예외를 던지면 actor가 변경 snapshot 제출을 생략하는가 | actor는 반환 status와 무관하게 전후 `DocumentRevision`을 비교한다. 두 플랫폼 실제 V8 fixture의 `expected_js_exception=PASS`가 확인됐다. | 통과 |
| 9 | detached-only DOM 생성이 `DocumentRevision`을 바꾸지 않거나 root output을 오염시키는가 | 두 플랫폼에서 detached node 256개를 추가한 뒤 revision 9→265, 연결 root 결과 동일성을 확인했다. | 통과 |
| 10 | 명시적 viewport/media 환경 전에 desktop 기본값으로 cascade를 실행하는가 | 환경 미설정 시 `not_configured`, 계산 요청 없음과 초기 빈 snapshot handshake를 단위 테스트·코드에서 확인한다. | 통과 |
| 11 | 동일한 환경 재전달이 revision을 올리거나 현재 completed 결과를 지우는가 | 같은 입력이 revision·계산 상태를 유지하는 단위 테스트가 통과한다. | 통과 |
| 12 | NaN·비양수·모순 media 입력 실패가 마지막 유효 환경이나 결과를 덮는가 | 환경 검증 전에 상태를 변경하지 않으며 invalid environment 보존 단위 테스트가 통과한다. | 통과 |
| 13 | revision이 최대값에서 wrap되어 과거 키와 충돌하는가 | Runtime setter는 `checked_next()` 실패를 `RevisionExhausted`로 변환한다. core revision 경계 테스트에서 최대값 뒤 증가 실패를 확인한다. | 통과 |
| 14 | 오래된 계산이 새로운 environment/document 요청 뒤 현재 결과로 publish되는가 | key에 generation·document·render-tree·style·environment 축이 들어간다. 요청 교체 뒤 stale completion을 폐기하는 단위 테스트가 통과한다. | 통과 |
| 15 | 빠른 변경이 요청 무제한 적재 또는 이전 completed의 최신 결과 오인을 유발하는가 | pending은 단일 `Option<WorkRequest>`이고 교체 시 completed를 지운다. 최신 key 확인 단위 테스트가 통과한다. 지속 부하에서의 공정성·지연 분포는 측정하지 않았다. | 통과 · 부하 분포 미측정 |
| 16 | Stylo panic이 V8 actor나 세션 전체를 죽이거나 환경 setter를 계속 성공시키는가 | worker 경계에서 panic을 잡아 CSS 상태만 failed로 만들고 후속 환경 변경은 worker 오류를 반환한다. panic 주입 테스트가 통과한다. | 통과 |
| 17 | `RuntimeSession::new()` 직후 환경 설정이 초기 문서 snapshot보다 먼저 와 요청이 유실되는가 | actor가 초기 빈 snapshot을 등록한 뒤 Isolate 준비 결과를 보낸다. Android·iOS probe는 session 생성 직후 fixture를 실행해 완료했다. 강제 스케줄 지연 race 주입은 별도로 하지 않았다. | 코드 순서·실행 통과 |
| 18 | 종료 중 mutex를 잡은 채 worker join하거나 Stylo 실행을 거짓으로 취소하는가 | actor 종료 뒤 CSS worker를 닫고 상태 lock 밖에서 join한다. normal session free는 양 플랫폼 probe가 통과했다. 진행 중 Stylo 계산의 강제 취소·종료 시간 상한은 지원하지 않는다고 계약에 남겼다. | 제한 명시 |
| 19 | 실패 JSON에 stale/partial `completed`가 남거나 명세가 구현과 다르게 `roots: []`를 약속하는가 | 구현은 `failed`에서 `completed=null`과 top-level `error`를 반환한다. 계약의 잘못된 실패 설명을 고치고 이 형태의 FFI 단위 테스트를 추가했다. | 통과 · 문서 수정 |
| 20 | FFI 입력 오류·작은 버퍼·NUL 종료 실패가 잘못된 JSON을 유효한 결과처럼 보이게 하는가 | enum/boolean/flag와 null 입력, output 부족·정확한 capacity·종료 NUL 테스트가 있다. 실제 probe는 작은 버퍼로 required capacity를 확인한 뒤 전체 JSON을 다시 복사한다. | 통과 |
| 21 | Android/iOS ABI 또는 화면 보고가 달라 성공 결과가 실패처럼 보이는가 | 두 target 앱 빌드와 실제 실행이 통과했다. iOS screenshot에서 래퍼의 `status=0` 중복을 발견해 C 보고서를 그대로 반환하도록 고친 뒤 재빌드·재실행·새 screenshot에서 한 번만 보이는 것을 확인했다. Android 화면은 기존 단일 status 출력을 유지한다. | 통과 · iOS 표시 수정 |

## 측정값 해석에서 수정한 점

처음 cascade와 detached mutation 뒤 cascade는 같은 warm 상태가 아니어서 계산 시간의 대소를 성능 차이라고 읽을 수 없다. 계약·구현 계획·실행 근거에 플랫폼별 단일 표본이며 cascade 시간은 서로 비교하지 않는다고 적었다. snapshot clone·submit 역시 두 시뮬레이터에서 한 번씩 관측한 진단값이다. 이는 반복 벤치마크가 아니다.

## 남은 경계

이 변경은 UA computed-style snapshot을 내부 JSON으로 제공하는 단계다. author stylesheet registry, `Element.style` CSSOM, 실제 OS media 수집, Taffy layout, GPU render snapshot, 화면 표시·frame latency는 구현하거나 검증하지 않았다. Stylo in-flight 강제 취소와 bounded shutdown도 지원하지 않는다. 이 경계는 C04.8 완료로 승격하지 않는다.
