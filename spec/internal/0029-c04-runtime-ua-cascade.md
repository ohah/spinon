# 0029 · C04.8 Runtime UA cascade 재계산

**상태:** 내부 runtime 계약 초안 · **버전:** `0.1.0-draft` · **제품 지원:** 미완료

## 목적

RuntimeSession의 HostDocument snapshot을 별도 CSS worker가 내장 지원 HTML UA stylesheet로 계산한다. 결과는 document, style, environment revision으로 식별하며 native host가 내부 C ABI로 읽을 수 있다.

## 환경 입력

환경은 양수·유한 `widthCssPx`, `heightCssPx`, `deviceScaleFactor`, `colorScheme`, `primaryPointer`, `primaryHover`, `allPointers`로 구성한다. scheme은 `light|dark`, primary pointer는 `none|coarse|fine`, 전체 집합은 coarse/fine/hover boolean이다. 모순 입력과 비유한/비양수 크기는 거부하며 이전 환경 입력·revision을 보존한다.

RuntimeSession이 환경 revision 소유자다. 첫 유효 환경은 revision 0으로 저장한다. 이후 유효 환경의 실제 변경만 `checked_next()`로 증가하고 같은 값 재전달은 revision과 계산 요청을 바꾸지 않는다. revision 고갈은 fail-closed다. OS 설정 자동 수집은 하지 않는다. 작성자 stylesheet 입력이 없는 동안 style revision은 0이다.

## 문서 root와 cascade 범위

HostRoot 직속 Element 하나마다 독립 fragment cascade scope를 만든다. 선택한 요소는 CSS `:root`/document element가 아니며 root display blockification을 받지 않는다. 따라서 `<span>` root는 `display: inline`을 유지하고 root의 inline `display` 선언도 계산된다. 기존 문서-root Stylo adapter는 그대로 두고, C04.8은 별도의 fragment-root adapter 모드를 사용한다. HostRoot, JavaScript `Document`, `documentElement`, `body` 의미를 HTML 노드나 추가 상속 경계로 만들지 않는다. 모든 root는 같은 immutable HostDocument snapshot과 viewport/environment를 공유하지만 root 사이 selector·상속은 격리된다. 계산 전에 세대·부모·요소 종류를 검증한다. top-level Text가 있거나 root 계산 중 하나라도 실패하면 root 전체 결과를 버리고 오류를 게시한다. root가 없는 snapshot은 유효한 빈 결과다.

현재 stylesheet 목록은 비어 있고 컴파일 시 포함하는 `supported-elements-v0.css`가 UA origin으로 적용된다. `StyloDocumentView`가 기존 HostDocument HTML `style` 속성을 inline author origin으로 파싱하므로 `setAttribute("style", value)`로 설정한 선언도 적용한다. 이 문서 속성 변경은 `DocumentRevision`을 바꾼다. `Element.style`/`CSSStyleDeclaration` CSSOM, author stylesheet 전달·등록, 원격 자원은 포함하지 않는다. 계산 profile은 `SupportedElementsUaV1`이며 선언된 property whitelist만 반환한다.

## 자동 재계산·worker

1. V8 actor는 JS task 전후 `DocumentRevision`을 비교한다. 값이 달라지면 task 결과가 JavaScript 오류여도 최신 HostDocument snapshot을 제출한다. detached-only 변경도 계산한다. 이 결과는 전체 document revision을 포함하므로 render-tree revision만 비교해 변경을 생략하지 않는다.
2. 환경 입력이 달라지면 최근 HostDocument snapshot으로 다시 계산한다. 환경 전달은 V8을 호출하지 않는다.
3. 세션별 전용 CSS worker OS thread 하나를 `RuntimeSession::new()` 중 생성해 준비를 마친다. 환경 미설정 중에는 조건 변수에서 대기하며 Stylo 계산을 하지 않는다. 입력은 immutable HostDocument snapshot, root 목록, CssViewport 및 style revision이다. V8 object·callback은 worker 경계를 넘지 않는다. 초기 빈 HostDocument snapshot은 `RuntimeSession::new()` 완료 전 owner에 등록한다. 추가된 startup/idle thread 비용은 실행 근거에 기록한다.
4. worker는 현재 계산 하나와 mutex/condition variable로 보호하는 pending slot 하나를 보유한다. 새 입력은 pending slot을 최신 요청으로 덮어쓴다. 실행 중 계산은 강제 종료하지 않지만 완료 시 요청 key가 최신 key와 같지 않으면 게시하지 않는다. 새 요청을 등록하면 공개 `completed` 결과를 지워 이전 결과가 최신인 것처럼 소비되지 않게 한다.
5. 결과는 각 root의 `ComputedStyleSnapshot`과 동일한 key를 가진다. 한 root 실패 시 partial output은 없다. worker panic은 해당 CSS worker를 사용할 수 없는 `failed` 상태로 바꾸고 이후 환경 변경 요청은 worker 오류로 거부한다. JavaScript 실행 세션은 poisoned 처리하지 않는다.
6. Session 종료는 V8 actor를 먼저 join한 뒤 CSS worker에 close를 전달하고 join한다. 상태 mutex를 풀고 join해야 한다. 이미 실행 중인 Stylo 계산은 취소 불가하며 join 시간 제한을 보장하지 않는다.
7. HostDocument snapshot clone은 현재 전체 노드를 복사하므로 JS task 종료 시 V8 owner thread에서 O(노드 수) 시간이 든다. Stylo 계산을 분리하는 것이 snapshot 생성까지 비동기라는 뜻은 아니다. 고정 소형 DOM과 대량 DOM의 복제 비용을 별도 실행 근거에 기록한다.

문서 변경 제출, 환경 설정, 결과 publish는 하나의 CSS 상태 mutex로 선형화한다. actor와 환경 setter가 겹치면 mutex 획득 순서가 `requested` key의 순서를 결정한다. environment setter는 session의 submission gate로 종료와 순서를 맞춘 뒤 준비된 worker를 깨우며 thread 생성이나 Stylo 계산을 하지 않는다. JSON 조회는 현재 상태를 값/`Arc`로 복제한 뒤 mutex 밖에서 직렬화한다. 조회와 환경 설정은 세션 포인터가 유효한 동안에만 허용하며 `spinon_runtime_session_free`와 병행할 수 없다.

## revision key

```text
generation
documentRevision
renderTreeRevision
styleRevision
environmentRevision
```

revision 값의 오름차순 크기만으로 최신성을 판단하지 않는다. 세션의 generation을 포함한 tuple 전체가 일치해야 한다. `requested`와 `completed` key가 다르면 결과는 오래된 snapshot이다.

요청은 `generation`, `documentRevision`, `renderTreeRevision`, `styleRevision`, `environmentRevision`의 전체 tuple로 식별한다. 새 요청을 등록할 때 이전 `completed`를 비우므로, 공개 상태에 `completed`가 있으면 그 안의 `key`는 현재 `requested`와 항상 같다. `state=failed`는 최신 요청의 계산 실패를 나타내며 `completed`는 null이고 `error`에 오류 문자열을 둔다. `state=empty`는 계산 성공이며 HostRoot 직속 Element가 없음을 뜻한다. Element가 한 개라도 계산되면 `ready`다.

## 내부 C ABI

### 환경 설정

환경 설정 함수는 유효 값 검증, 동일 값 무변경, environment revision 발급과 CSS worker 요청 제출까지 수행하고 V8·Stylo 계산 완료를 기다리지 않는다. C ABI에서 `colorScheme`은 `0=light`, `1=dark`, `primaryPointer`는 `0=none`, `1=coarse`, `2=fine`, boolean은 `0|1`, `allPointerFlags`는 bit `0=coarse`, `1=fine`, `2=hover`다. 알 수 없는 enum, 다른 boolean 값, 미정의 flag bit, 모순 media 환경, NaN·무한대·비양수 viewport/device scale 또는 비유한 곱 결과는 `-1`이다. 오류는 기존 환경·revision을 보존한다. 성공은 `0`, 종료 중이면 `-6`, CSS worker 생성/실패면 `-7`이며 출력 revision은 성공 시에만 쓴다. 동일 입력은 revision과 요청을 바꾸지 않는다. `session`은 유효한 세션이며 같은 호출 중 free와 병행하면 안 된다. 이 setter는 Stylo 계산을 기다리지 않지만 짧은 mutex 경합은 있으므로 UI thread에서 호출 가능한 작업은 상태 갱신에 한정한다.

### snapshot 복사

JSON schema 이름은 `spinon.runtime.ua-cascade.v1`이다.

```json
{
  "schema": "spinon.runtime.ua-cascade.v1",
  "state": "not_configured | pending | ready | empty | failed",
  "requested": {
    "generation": 1,
    "documentRevision": 2,
    "renderTreeRevision": 2,
    "styleRevision": 0,
    "environmentRevision": 1
  },
  "completed": {
    "key": {
      "generation": 1,
      "documentRevision": 2,
      "renderTreeRevision": 2,
      "styleRevision": 0,
      "environmentRevision": 1
    },
    "computationDurationUs": 1,
    "roots": [
      { "rootNodeId": 1, "elements": [
        { "nodeId": 1, "properties": { "display": "block" } }
      ], "diagnostics": [] }
    ]
  },
  "error": null
}
```

위 객체는 필드 모양 설명이다. 각 profile에서 `properties`는 `SupportedElementsUaV1`이 선언한 property whitelist만 포함한다. root별 `diagnostics`는 C04.5와 같이 `sourceId`, nullable `nodeId`, 0-based `line`, 1-based UTF-16 `column`, `message`를 보존한다. 잘못된 inline declaration은 Stylo CSS error-recovery로 무시되고 진단을 남기며 그 자체로 cascade 전체를 실패시키지 않는다. 계산 전에는 `requested` key가 있고 `completed`는 null이다. 새 요청을 게시할 때 이전 `completed`를 비운다. worker는 계산 완료 직전에 요청 key를 재확인하고 현재 요청과 다르면 결과를 버린다. 따라서 `completed`가 있으면 항상 `requested`와 같은 key를 가진다. fatal document/cascade 계산 오류는 top-level `state=failed`, `completed=null`, 오류 문자열로 표현하며 부분 root 결과를 공개하지 않는다. `not_configured`는 두 key가 모두 null이고 `pending`은 `requested`만 존재한다. caller는 top-level state가 `ready` 또는 `empty`일 때만 computed-style 값을 사용한다. `empty`는 유효한 빈 cascade 성공이며 `failed`와 구별한다.

JSON timing 필드 `completed.computationDurationUs`는 worker가 cascade 계산을 시작한 때부터 계산 함수가 반환할 때까지의 시간이며 queue 대기, JSON 직렬화, actor snapshot 복제 시간은 포함하지 않는다. 새 runtime 측정값은 `RuntimeSession::new()` 보고의 `css_worker_ready_us`와 `session_startup_us`, JS task 보고의 `ua_snapshot_clone_us`와 `ua_snapshot_submit_us`다. 앞의 worker 시간은 thread 생성 요청부터 ready handshake까지, session 시간은 `RuntimeSession::new()` 진입부터 반환 직전까지, clone 시간은 전체 immutable HostDocument snapshot 생성, submit 시간은 CSS 상태 owner 등록까지를 포함한다. 양수 duration은 정수 microsecond로 올림해 1µs 미만을 0으로 오인하지 않게 한다. 소형·대형 fixture의 각 측정은 플랫폼별 단일 진단 표본이며 cascade 측정은 첫 요청과 후속 요청의 순서 효과를 포함하므로 서로의 성능 우열로 해석하지 않는다. 시간값은 해당 에뮬레이터/시뮬레이터 실행의 관찰값이며 제품 성능 보장이 아니다.

복사 API는 Stylo 계산을 기다리지 않는다. `requiredCapacity`는 UTF-8 JSON과 마지막 NUL을 포함한 byte 수다. 출력 포인터·required pointer가 null이거나 capacity가 0이면 인자 오류 `-1`이다. 한 호출에서 현재 상태를 고정해 JSON을 만든다. 필요한 크기가 capacity보다 크면 output[0]을 NUL로 만들 수 있을 때만 그렇게 하고 `-3`과 required capacity를 반환한다. 부분 JSON은 쓰지 않는다. 버퍼 부족 뒤 다음 호출에서 상태가 바뀔 수 있고 caller는 매번 새 required capacity를 확인한다. 아직 환경 또는 결과가 없을 때도 schema가 있는 상태 JSON을 반환한다. JSON 생성·복사 비용은 node/property 수에 비례하므로 UI thread 호출 금지다.

## 오류·수명

- 환경 오류는 기존 환경·revision을 유지한다.
- CSS syntax diagnostics는 C04.5 동작처럼 root snapshot에 보존하고 Stylo의 recovery 결과를 반환한다. root mapping/view 생성 등 fatal Stylo 오류는 전체 requested result를 실패 처리하며 부분 root/style은 공개하지 않는다.
- pending 입력 교체는 취소가 아니다. 교체 전 계산은 완료될 수 있지만 현재 요청 key와 다른 경우 게시하지 않는다.
- 조회 결과는 값 복사이며 내부 Stylo 객체나 V8 wrapper를 FFI 밖으로 노출하지 않는다.
- Session free는 실행 중인 CSS 계산을 취소하지 못한다. worker join이 끝나기 전에는 관련 Rust session 자원을 해제하지 않는다.

## 지원 경계

이 계약은 embedded UA computed-style snapshot의 자동 runtime 계산·조회까지만 다룬다. OS 환경 감지, author stylesheet 등록·변경, `Element.style`/`CSSStyleDeclaration` CSSOM, 스타일 무효화 최적화, Taffy, RenderSnapshot, GPU, Android/iOS 화면 렌더링, 공개 제품 API를 약속하지 않는다. 기존 HostDocument HTML `style` 속성의 inline-origin cascade만 포함한다.

## Chromium 비교 모델

고정 Chromium C01 UA reference의 동일 HTML 요소·property computed string과 비교한다. runtime이 만드는 각 direct-root scope는 input fixture의 동일 subtree와 대응한다. 기준 reference를 수정·덮어쓰지 않는다. Android emulator 및 iOS Simulator에서는 실제 V8 세션에서 같은 DOM을 생성하고 JSON 결과의 key·property를 확인한다. 이 실행은 실기기 또는 GPU 표시 검증이 아니다.

## 구현 검증 범위

- Rust 단위 테스트: `not_configured`, 최초 빈 snapshot, `empty`, 단일·다중 fragment root, `span`의 Chromium `inline` 값, root inline-style override와 `:root` 미매칭, 기존 문서-root adapter 회귀, top-level Text 및 root 오류의 원자 실패, detached mutation과 재삽입, task throw 뒤 commit, inline `style` invalid syntax diagnostics, 환경 동일값·오류·overflow, worker 최신 pending slot, stale 결과 폐기, panic 뒤 worker 불능 상태, 짧은 출력 버퍼와 재시도 사이 변경.
- 고정 Chromium fixture: `div`, `span`, `button`, `p` 등의 기존 C04.5 property 문자열을 수정하지 않고 같은 HostDocument node와 property를 정확 비교.
- Android emulator 및 iOS Simulator: 실제 V8 JS façade가 생성·부착한 요소, 명시 CSS viewport/media 환경 설정, C ABI JSON의 schema·revision·computed value를 각각 확인.
- session 초기화/유휴 worker 비용과 snapshot 복제 비용: `css_worker_ready_us`, `session_startup_us`, `ua_snapshot_clone_us`, `ua_snapshot_submit_us`, `computationDurationUs`를 Android emulator와 iOS Simulator에서 각각 기록한다. 각 플랫폼에서 4개 연결 root와 추가 256개 detached node 문서의 단일 표본을 확인했다. 유휴 thread 수는 세션별 1개다. 별도 memory profiler를 사용한 thread stack/RSS 계측과 더 큰 문서 크기의 복수 반복 표본은 아직 수행하지 않았다. 첫 cascade와 후속 cascade 시간은 warm/cold 순서가 같지 않아 비교하지 않는다. 이 측정은 native GPU frame latency를 증명하지 않는다.
