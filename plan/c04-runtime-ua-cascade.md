# C04.8 · HostDocument 변경 뒤 runtime UA cascade 재계산

- **문서 유형:** 구현 계획 · 현재 구현 상태는 [상태 대장](../spec/STATUS.md#css-구현-체크리스트)에서 관리
- **기준일:** 2026-10-09
- **상위 항목:** [C04 stylesheet·selector·cascade](../spec/STATUS.md#css-구현-체크리스트)
- **선행 구현:** C03 Stylo DOM adapter, C04.5 UA cascade, C04.7 media environment 입력, R03 HostDocument snapshot

## 목표

V8 세션의 연결 HostDocument 변경을 Stylo UA cascade까지 자동 전달한다. V8 실행기와 별도 CSS worker가 불변 문서 snapshot을 계산하고, 완료 결과를 revision tuple과 함께 네이티브 host가 읽을 수 있게 한다. 계산 중인 JavaScript 실행기를 기다리게 하거나 Stylo 계산을 UI thread에서 실행하지 않는다.

## 입력·소유권 계약

- `RuntimeSession`은 viewport 크기·device scale·색상 scheme·pointer capability를 받는다. 입력은 CSS px로 해석한다. OS 설정을 자동 수집하거나 임의 기본값을 선택하지 않는다.
- `RuntimeSession`의 CSS 환경 소유자가 첫 유효 환경을 `EnvironmentRevision::INITIAL`인 0으로 저장한다. 이후 실제 입력 변경만 `checked_next()`로 revision을 올린다. 같은 입력 재전달은 revision을 올리지 않는다. overflow는 새 입력을 거부하고 이전 입력을 보존한다.
- 현재 session에는 author stylesheet 목록이 없으므로 `StyleRevision::INITIAL`과 내장 `supported-elements-v0.css`를 사용한다. 다만 기존 `HostDocument` HTML `style` 속성은 `StyloDocumentView`가 inline author origin으로 파싱하므로 그대로 cascade에 반영한다. `setAttribute("style", ...)` 변경은 `DocumentRevision`으로 다시 계산한다. `Element.style`/`CSSStyleDeclaration` CSSOM과 framework별 별도 inline-style API는 이 단계에서 만들지 않는다.
- HostRoot 직속 Element 각각을 독립 fragment의 자식으로 계산한다. 선택한 요소는 CSS `:root`/document element가 아니며 Stylo의 root display blockification을 받지 않는다. HostRoot·`document`·`documentElement`·`body`를 HTML 노드처럼 만들거나 상속 경계로 끼우지 않는다. 기존 Stylo 문서-root adapter와 구별되는 fragment-root adapter 모드를 사용한다. root가 없으면 성공한 빈 결과를 만든다. 직속 Text 또는 잘못된 generation/root가 있으면 전체 요청을 실패 처리한다.
- 각 root 계산 결과는 같은 generation·document revision·render-tree revision·style revision·environment revision으로 묶는다. root 하나라도 실패하면 일부 root 결과를 공개하지 않는다.
- actor가 준비되기 전에 초기 빈 `HostDocumentSnapshot`을 CSS 상태 owner에 등록한다. `RuntimeSession::new()`가 반환된 직후 host가 환경을 설정해도 snapshot이 아직 등록되지 않은 경쟁 상태를 만들지 않는다.

## 재계산·스케줄 계약

- JS task가 성공하거나 예외를 반환하는 것과 무관하게 task 전후 `DocumentRevision`이 바뀌면 재계산 요청을 만든다. detached-only mutation도 포함한다. 결과 key가 `DocumentRevision`을 포함하므로 연결 표시 트리 revision만으로 생략하면 published snapshot이 현재 문서 revision을 반영하지 못한다. 이 단계에서는 detached-only 변경 최적화를 하지 않는다.
- viewport/media 환경이 바뀌면 가장 최근 불변 HostDocument snapshot으로 재계산한다. 환경을 V8 scheduler에 동기 왕복시키지 않는다.
- CSS worker OS thread는 `RuntimeSession::new()` 중 세션별로 하나 생성하고 준비를 기다린다. 환경 미설정 동안 worker는 대기만 하며 Stylo 계산은 하지 않는다. 이 선택은 UI 환경 setter에서 OS thread 생성 지연을 피하지만 세션 시작 비용과 유휴 thread stack/스케줄러 비용을 추가하므로 둘 다 측정한다. worker에는 Rust `HostDocumentSnapshot`과 CSS 환경 값만 보낸다. V8 callback, Isolate, JS 값은 넘기지 않는다.
- snapshot 복제는 현재 `HostDocument`가 소유한 전체 노드를 복사하므로 task 종료 시 V8 owner thread에서 O(노드 수) 비용이 든다. Stylo 계산은 분리하지만 snapshot 복사까지 non-blocking이라고 주장하지 않는다. 고정 크기 문서와 대량 문서에서 복제 비용을 별도로 측정해 근거에 기록한다.
- worker의 대기 입력은 mutex/condition variable로 관리하는 최신 요청 slot 하나다. 계산 중 새 요청은 대기 slot을 최신 요청으로 교체한다. 실행 중인 계산은 강제 취소하지 않고, 완료 시 최신 요청 key와 다르면 publish하지 않는다. 계산 실패는 해당 최신 key에만 게시한다. worker panic은 그 세션의 CSS 상태를 `failed`로 고정하고 이후 환경 변경 요청은 worker 오류로 거부한다. JavaScript 실행기와 세션 전체를 poisoned 처리하지 않는다.
- 환경 설정·문서 snapshot 제출·결과 publish는 하나의 CSS 상태 mutex로 선형화한다. actor task와 환경 설정이 겹치면 mutex 획득 순서가 `requested`의 순서를 정한다. 환경 setter는 이미 준비된 worker를 깨우기만 하며 thread 생성이나 Stylo 계산을 하지 않는다. JSON 복사 API는 상태를 짧게 복제한 뒤 lock 밖에서 직렬화한다. worker close/join은 상태 mutex를 풀고 실행하며, 이미 시작한 Stylo 계산을 취소할 수 없어 종료가 지연될 수 있다.

## 읽기 API와 비교 기준

- 내부 Rust/C ABI는 환경 설정, 최신 cascade 상태 JSON 복사, 필요한 바이트 수 조회를 제공한다. JSON schema는 `spinon.runtime.ua-cascade.v1`이다.
- 응답은 `requested`와 `completed` 전체 revision key를 각각 담는다. 새 요청은 이전 `completed`를 비우고 `pending`으로 전환한다. worker는 최신 key 결과만 게시하므로 `completed`가 있으면 `requested`와 정확히 같다. top-level `state` 값은 `not_configured`, `pending`, `ready`, `empty`, `failed`다. 실패 JSON에는 오류 문자열만 두며 부분 root/style 결과는 두지 않는다.
- copy API는 계산 완료를 기다리지 않는다. caller가 준 출력 버퍼가 짧으면 유효 JSON을 쓰지 않고 NUL guard와 required byte 수만 반환한다. retry 사이에 상태가 바뀔 수 있어 매 호출이 돌려주는 required byte 수를 다시 확인한다. JSON 생성량에 비례하므로 UI thread 호출은 금지한다. 호출 중 session free도 금지한다.
- 환경 설정 ABI는 `session`, viewport float 3개, `colorScheme`, `primaryPointer`, 0/1 `primaryHover`, pointer flag bitset, environment revision 출력 포인터를 받는다. C ABI enum은 `colorScheme: 0=light, 1=dark`, `primaryPointer: 0=none, 1=coarse, 2=fine`, `allPointerFlags: 0=none, bit0=coarse, bit1=fine, bit2=hover`다. 다른 boolean/enum/flag 조합, NaN·무한대·비양수 크기, 곱한 device viewport의 비유한·비양수 값은 거부하고 이전 환경·revision을 보존한다. 성공 `0`, 잘못된 인자·환경 `-1`, 버려진/고장 난 CSS worker `-7`, 종료 중 `-6`이다. 첫 환경 revision은 0이다. 세션 종료와 환경 setter는 같은 submission gate를 사용한다. setter는 CSS 계산을 기다리지 않지만 mutex 경합은 가능하므로 UI thread에서 허용되는 범위는 짧은 상태 갱신뿐이다.
- snapshot JSON ABI는 `session`, output pointer/capacity, `requiredCapacity` 출력 포인터를 받는다. 성공 `0`, 인자 오류 `-1`, output 부족 `-3`이다. 작은 버퍼에는 NUL guard만 쓰며 부분 JSON은 쓰지 않는다. required capacity는 UTF-8 JSON과 종료 NUL을 포함한다. 호출 시점 snapshot을 만든 다음 상태가 바뀌어도 반환 JSON은 그 한 snapshot으로 일관된다. JSON 생성·전체 복사는 node/property 수에 비례하므로 이 함수는 UI thread에서 호출하지 않는다. 환경 setter·조회는 유효 세션 수명 동안만 가능하고 session free와 병행하지 않는다.
- 비교 모델은 기존 고정 Chromium UA fixture와 `compute_supported_elements_ua_cascade`다. 지원 HTML 요소 9개·19개 UA computed value reference를 재사용하고 덮어쓰지 않는다. runtime 경로에서 생성한 같은 DOM의 각 root와 속성을 정확 비교한다. 실제 API는 UA profile이 선언한 property whitelist의 computed value만 반환하며 full CSS computed style을 주장하지 않는다. 기존 inline `style` 속성은 override fixture로 별도 확인한다.
- Android emulator와 iOS Simulator에서 실제 V8이 JS DOM façade로 생성·부착한 노드를 계산해 `div=block`, `span=inline`, `button=inline-block`, `p`의 기본 margin 등 fixture 값을 확인한다. 실기기·OS media 설정 자동수집은 요구하지 않는다.
- Rust 테스트는 초기 `not_configured`→빈 문서 `empty`, 단일·복수 fragment root, `span`의 Chromium `inline` 값과 root inline-style override, CSS `:root` 미매칭, 기존 문서-root adapter 동작 보존, top-level Text 거부, detached 변경과 재삽입, JS throw 뒤 commit, inline syntax diagnostic 보존, 같은 환경/잘못된 입력/revision overflow, 빠른 환경·문서 burst의 latest-wins, worker 실패, 짧은 JSON buffer와 재시도 상태 변경을 다룬다. 비교 fixture는 Chromium reference에 포함된 값을 수정하지 않고 검증한다.

## 범위 밖

- author stylesheet 전달·CSSOM (`Element.style`/`CSSStyleDeclaration`)·CSS 자원 로딩. 현재 DOM의 `style` 속성은 기존 `setAttribute` 경로로 반영한다.
- Taffy layout, `StaticRenderSnapshot`, GPU 제출 및 화면 표시
- 스타일 차이만으로 `StyleRevision`을 올리는 stylesheet owner
- OS color scheme/pointer/viewport 자동 관찰과 회전·멀티윈도우 동작
- selector invalidation, dirty subtree, 계산 취소, worker pool 공유 정책·성능 최적화
- 공개 앱 API 지원 주장

## 구현 단계

1. `spinon-style`에 runtime에서 쓸 수 있는 viewport/media 입력 validation을 공개하고, 기존 `StyloDocumentView`와 `compute_supported_elements_ua_cascade`를 조합해 immutable snapshot의 각 direct Element root를 원자적으로 계산하는 함수를 추가한다.
2. `spinon-runtime`에 명시적 환경·최신 문서 snapshot owner, `RuntimeSession::new()` 안의 CSS worker 준비, actor가 ready를 알리기 전 초기 snapshot handshake, capacity-one latest-wins slot을 추가한다.
3. JS task 전후 `DocumentRevision`이 달라진 경우와 환경 입력 변경 시 자동 재계산 요청을 제출한다. detached mutation도 key 일관성을 위해 계산한다. 요청 key 불일치 결과는 버린다.
4. 네이티브 host용 내부 C ABI와 JSON schema를 추가한다. 환경 설정은 계산을 기다리지 않고, snapshot 복사는 짧은 mutex 구간 밖에서 JSON을 만들며 출력 capacity를 확인한다.
5. 실제 V8 JS fixture를 Android emulator 및 iOS Simulator에서 실행해 결과 revision과 UA 값을 확인한다. 4개 연결 root에 detached node 256개를 더한 두 입력에서 session startup/idle worker 비용, task 종료 snapshot 복제·제출 시간과 cascade 계산 시간을 기록한다. 이 fixture는 플랫폼별 단일 진단 표본이며 cascade 첫 계산과 후속 계산의 순서 효과를 포함하므로 성능 비교로 해석하지 않는다. CSS→Taffy·GPU 연결은 다음 별도 작업으로 둔다.
6. 상태 대장, 내부 계약, 테스트 근거, Tailnet CSS 계획 미리보기를 갱신한다. 숫자 계약 버전은 미출시 정책에 따라 `0.1.0`으로 유지한다.

## 계획 적대 검토 · 20개 독립 실패 관점

| # | 실패 관점 | 반영한 방지 기준 |
|---:|---|---|
| 1 | desktop 기본값이 모바일 앱 환경에 자동 적용됨 | 환경 미설정 상태는 계산하지 않고 host 입력을 요구한다. |
| 2 | viewport 값과 scheme/pointer 값이 서로 다른 시점의 snapshot으로 섞임 | 하나의 `RuntimeCssEnvironment` 입력과 하나의 environment revision으로 묶는다. |
| 3 | 같은 환경을 다시 설정할 때 revision이 불필요하게 증가함 | 필드가 동일하면 revision·재계산 요청이 모두 유지된다. |
| 4 | NaN·무한대·0 이하 viewport 값이 Stylo Device로 전달됨 | 환경 설정 단계에서 전체 입력을 검증하고 기존 값을 보존한다. |
| 5 | primary pointer가 전체 capability에 없는 모순 입력이 저장됨 | C04.7의 pointer 일관성 검증을 재사용한다. |
| 6 | environment revision이 `u64::MAX`에서 wraparound함 | `checked_next()` 실패 시 입력 변경을 거부한다. |
| 7 | 여러 HostRoot 직속 Element 중 첫 번째만 계산되고 나머지가 누락됨 | 모든 직속 Element를 순서대로 별도 root 계산에 포함한다. |
| 8 | HostRoot 직속 `<span>`이 Stylo `documentElement`로 간주되어 blockification되는가 | fragment-root adapter를 사용해 앱 root를 CSS `:root`/document element에서 제외하고 Chromium의 `inline`을 보존한다. |
| 9 | root 하나의 Stylo 오류 뒤 앞 root의 부분 스타일이 노출됨 | root 집합 전체를 한 계산 결과로 만들고 실패 시 전부 버린다. |
| 10 | top-level Text가 조용히 무시되어 노드 ID 집합이 달라짐 | top-level Text는 전체 요청 오류로 기록한다. |
| 11 | detached-only 변경이 전체 document revision key와 불일치한 채 생략되거나 attach를 놓침 | 모든 `DocumentRevision` 변경에서 계산해 full tuple을 맞춘다. detached 변경 최적화는 하지 않고 detached→attach 전이를 테스트한다. |
| 12 | JS task가 예외를 던지면 앞선 DOM commit의 cascade가 누락됨 | JS 결과 코드와 무관하게 전후 `DocumentRevision`을 비교한다. |
| 13 | 계산 중 viewport 변경 뒤 이전 환경 결과가 최신으로 게시됨 | publish 직전 전체 key를 비교하고 stale 결과를 버린다. |
| 14 | burst 변경이 무제한 큐 메모리와 지연을 만듦 | pending 요청은 1개로 제한하고 최신 입력으로 교체한다. |
| 15 | Stylo worker에서 V8 callback/Isolate를 접근하거나 환경 setter가 UI thread에서 thread 생성 때문에 지연됨 | worker는 session 초기화 중 준비하고 setter는 깨우기만 한다. 유휴 worker 비용을 명시·측정하며 입력은 immutable Rust snapshot·환경 값만 허용한다. |
| 16 | 이전 completed result를 최신 결과처럼 소비하거나 worker panic 뒤 pending에 멈춤 | 새 요청 때 completed를 비우고 계산 직전 key를 재검사한다. worker panic은 `failed`와 worker-unavailable 오류를 원자 게시한다. |
| 17 | 작은 FFI 출력 버퍼가 부분 JSON을 유효 결과처럼 반환함 | required capacity를 계산하고 부족하면 NUL 종료 빈 출력만 둔다. |
| 18 | JSON 직렬화가 publication lock을 오래 점유하거나 task 종료 snapshot clone이 JS owner thread를 오래 점유함 | JSON은 state/Arc clone 후 lock 밖에서 만들고 전체 snapshot clone의 O(노드 수) 비용은 소형·대량 문서에서 측정한다. |
| 19 | Session free 뒤 worker가 계속 쓰거나 join 중 lock 교착·무기한 대기가 발생함 | actor 종료 뒤 상태 lock을 풀고 CSS worker를 close/join한다. Stylo 계산은 취소되지 않아 join 지연은 제한 없는 미검증 사항으로 남긴다. |
| 20 | UA-only 문구가 이미 지원하는 inline `style` 속성 반영을 지우거나 CSSOM·전체 CSS·Taffy·GPU 지원으로 오해됨 | C04.5 Stylo view와 같은 inline attribute cascade·diagnostics를 보존하되 author stylesheet·CSSOM API·Taffy/GPU/실기기와 구분한다. |

이 표는 초기 계획 검토 전용이다. 구현 비교에서 발견한 root blockification 계약 결함은 [계획 보정 검토](../spec/internal/evidence/c04-runtime-ua-root-semantics-plan-review-2026-10-09.md)에서 별도로 검토했다. 구현 후 코드·V8 실행을 대상으로 한 검토는 다시 새로운 실패 관점으로 기록한다.
