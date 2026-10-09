# C05.4 · 안전한 범위의 runtime 하위 트리 재계산

- **문서 유형:** 구현 계획 · 공식 상태는 [`spec/STATUS.md`](../spec/STATUS.md)에서 관리
- **상위 항목:** [C05 사용자 지정 속성과 재계산](../spec/STATUS.md#css-구현-체크리스트)
- **선행 조건:** C04.11 author stylesheet, C05.1 사용자 지정 속성, C05.2 `@property`, C05.3 연결 장면 결과 재사용
- **내부 계약 숫자 버전:** `0.1.0` 고정 · 출시·호환성 정책을 사용자가 정하기 전 숫자를 올리지 않음
- **계획 공격 검토:** 별도 근거 문서의 20개 실패 관점

## 목표와 완료 경계

C05.3은 연결된 트리가 바뀌면 CSS cascade와 Taffy layout을 전체 계산한다. 이 작업은 현재 inline `style` 속성만 바뀐 안전한 경우에 한해 cascade의 출력만 하위 트리 단위로 다시 계산한다. 레이아웃과 GPU 장면은 기존처럼 전체 연결 트리로 다시 만든다.

이 단계의 snapshot diff, 재사용 계획 작성, 출력 배열 재조립은 여전히 연결 트리 전체를 방문하고 복사한다. 줄이는 일은 Stylo cascade 호출 수이며 전체 작업량이 O(dirty subtree)로 바뀌는 것은 아니다. 이는 후속 benchmark에서 cascade 시간과 layout/scene projection·전체 worker 요청 시간을 각각 비교한다.

증분 cascade는 아래 조건을 모두 만족할 때만 허용한다.

1. 이전 완료 계산과 새 요청이 같은 문서 세대, CSS profile, viewport·media·환경 revision, 단일 HostRoot를 사용한다.
2. 이전·현재 연결 노드의 ID, 종류, 소유자, 요소 이름, 부모·자식 순서, text data, 상태와 `style` 이외 속성이 같다.
3. 연결 트리에 존재하는 바뀐 입력은 요소의 namespace 없는 `style` 속성뿐이다.
4. 이전·현재 snapshot에 연결 author stylesheet가 없다. 런타임에는 고정 내장 UA stylesheet가 항상 있으므로 그 profile ID가 동일해야 하고, selector가 변하지 않은 namespace·요소 이름만 참조해야 한다. UA stylesheet에 속성·상태·관계 selector가 추가되거나 profile 검증이 실패하면 전체 cascade로 되돌린다.
5. dirty root가 이전 스타일 계산의 `DocumentRevision`과 연속된 변경 요약에서 왔고, worker의 스타일 출력 기준 revision과 일치한다.

조건을 증명할 수 없으면 전체 cascade를 계산한다. 캐시 결과나 부분 계산이 성공처럼 보이도록 오류를 숨기지 않는다.

## 계산 경계

- DOM commit 뒤 actor가 이미 가진 이전·현재 `HostDocumentSnapshot` 참조를 `WorkRequest`에 임시로 싣고 CSS worker가 연결 입력을 비교한다. 이전 snapshot은 dirty root 분류가 끝나면 버린다. worker cache에는 snapshot, V8 객체, DOM handle을 보관하지 않는다.
- 연결 구조가 같고 namespace 없는 HTML `style` 속성만 바뀌면 바뀐 요소와 그 자손을 dirty subtree로 표시한다. 이름은 HTML 규칙대로 ASCII 대소문자 무시로 판정하되 중복·모호한 이름이나 namespace 속성 변화는 전체 계산으로 되돌린다. 여러 dirty root가 겹치면 가장 바깥 root만 남긴다.
- dirty subtree의 부모 computed value는 그 요소까지의 ancestor path를 새 view에서 다시 계산해 얻는다. 이 경로의 cascade 출력은 변하지 않은 것으로 검증된 이전 snapshot 값을 재사용할 수 있지만, 상속 context 계산 호출은 별도로 센다.
- dirty subtree의 각 요소는 새 inline style과 현재 부모 context로 Stylo cascade를 다시 수행한다. dirty subtree 밖의 요소는 이전 `ComputedStyleSnapshot` 항목을 NodeId로 찾아 재사용한다.
- 단일 root가 아니거나 구조·text·non-style 속성·상태가 변경되거나, stylesheet source가 추가·수정·이동·제거되거나, profile·viewport·media·환경·generation이 달라지거나 revision gap이 있으면 전체 cascade fallback이다. `class`·`id` 등 CSS selector 입력 변경도 항상 전체 계산한다.
- Taffy layout, paint projection, renderer scene은 항상 현재 snapshot 전체를 입력으로 다시 계산한다. 이 작업을 부분 레이아웃·부분 페인트 구현으로 보고하지 않는다.

## 캐시와 요청 생명주기

- worker당 직전의 성공한 computed-style 출력 한 건만 보관한다. node별 `ComputedValues`, `StyloDocumentView`, `HostDocumentSnapshot`, JS/V8/native handle은 장기 보관하지 않는다.
- 보관하는 출력은 worker·문서 generation·root·profile·revision tuple·viewport/media 입력에 묶는다. generation이나 계산 owner가 달라지면 재사용하지 않는다.
- 연속된 pending request는 가장 이른 비교 기준 snapshot과 가장 최신 snapshot으로 합쳐 worker에서 최종 diff를 한 번 계산한다. revision 경계가 맞지 않거나 기존 request가 full invalidation이면 merged request도 full invalidation으로 표시한다. worker가 처리 중인 request의 이전 snapshot은 분류 직후 해제해 cache lifetime과 구분한다.
- 계산 중 도착한 최신 snapshot과 이전 request가 경합하면 기존 latest-wins publish 검사를 유지한다. 오래된 계산은 published result와 partial cache를 갱신하지 않는다. 스타일 cache 기준 revision과 invalidation source revision이 다르면 부분 계산을 거부하고 전체 계산한다.
- cascade 실패·panic·worker 종료 시 임시 partial 결과가 기존 cache를 덮지 않게 하고 cache를 무효화한다. 성공한 cascade 뒤 layout 실패가 난 경우에는 cascade 출력만 내부 기준으로 보관할 수 있으나 layout은 실패 상태로 유지하고 다음 요청에서 모든 key·revision 조건을 다시 검증한다.
- diagnostics는 이전 snapshot에서 복사하지 않고 현재 view에서 다시 수집해 변경 선언의 오류가 stale 상태로 남지 않게 한다.
- style 출력 cache는 직전 성공 결과 한 건을 보유한다. 계산 중 이전 출력과 새 출력이 동시에 살아 있을 수 있으나 `ComputedValues`나 document node graph는 장기 보관하지 않는다.
- 계산 출력의 `cascadeRecomputedStyleElements`, `cascadeReusedStyleElements`, `cascadeContextStyleElements`는 내부 진단값이다. 세 값의 의미를 명세하고 화면 프레임 시간이나 end-to-end 성능으로 오인하지 않는다.

## 구현 전 비교 기준과 오차

- author stylesheet 없는 고정 HTML fixture에서 중첩된 inline 사용자 지정 속성을 부모에 설정하고, 자손 한 branch의 부모 값을 변경한다. sibling branch와 연결 tree 구조는 유지한다.
- Chromium 기준은 변경 전·후 각 요소의 지원 computed properties와 Taffy 대상 CSS rectangle이다. 변경한 branch와 모든 자손은 새 값으로, sibling branch는 이전과 정확히 같은 값으로 남아야 한다.
- Rust full-cascade oracle과 incremental 결과를 NodeId별로 비교한다. CSS 문자열은 Chromium fixture의 기대값과 정확히 비교하고 rectangle의 각 좌표·크기 최대 절대 오차는 `0.5 CSS px` 이하로 제한한다. 평균 오차로 개별 요소의 실패를 가리지 않는다.
- author stylesheet 제한 조건을 벗어난 stylesheet·구조 변경은 결과 비교뿐 아니라 전체 fallback이 실제로 실행되는지도 계측한다. 내장 UA stylesheet는 고정 profile이며, type/namespace selector만 사용한다는 invariant test를 둔다. 다른 selector dependency가 추가되면 부분 cascade를 거부하도록 profile 검토를 요구한다.
- release benchmark는 16·256·2048 연결 요소에서 한 branch inline style 변경을 반복한다. full cascade와 incremental cascade의 p50/p95 및 실제 재계산 요소 수를 3개 독립 프로세스에서 비교한다. `projectionDurationUs`는 Taffy layout·frame mapping·renderer scene snapshot을 합친 단일 구간이다. 전체 요청 wall time도 기록하며 cascade-only 수치로 화면 성능을 단정하지 않는다. GPU 제출·표시 시간은 이 Rust worker 측정에 포함되지 않는다.

## 검증 행렬

- Rust: dirty subtree, dirty root 중첩 제거, 여러 sibling 변경, inherited custom property, style 제거, 계산 실패 후 복구, revision gap, empty/multiple roots, cache generation/profile/viewport 분리, worker pending coalescing과 최신 결과 게시.
- Chromium: 변경된 상위 사용자 지정 속성의 하위 전달, unaffected sibling 보존, style 속성 삭제, 잘못된 값과 fallback, 연결·분리·이동, 전체 fallback mutation.
- Android API 37 Emulator와 iOS 26.2 Simulator: 같은 V8 fixture로 branch style 변경을 실행하고 revision, 재계산·재사용·context 수, computed value, frame 및 최종 GPU 화면 전이를 각각 확인한다. 실기기는 별도 연결 시까지 미검증으로 남긴다.
- 전체 workspace test, Clippy, FFI all-features, fmt, JS fixture 검증, Android·iOS Simulator 빌드를 수행한다.

## 명시적 미포함

- author stylesheet가 존재할 때의 selector dependency / Stylo invalidation-map 기반 부분 restyle
- 내장 UA stylesheet의 속성·상태·관계 selector dependency 변경
- DOM 구조 변경, `class`·`id`·기타 속성·element state·text mutation의 부분 restyle
- 부분 Taffy layout, 부분 paint, GPU dirty region, 프레임 deadline 보장
- CSSOM, external CSS URL loader, general DOM selector invalidation, 공개 제품 API
- C05 부모 전체, 전체 CSS 구현, 실기기 성능 완료 판정

## 작업 단계

1. 고정 fixture와 Chromium pre-comparison 결과를 구현 전에 저장한다.
2. snapshot diff와 pending request 병합 규칙을 구현해 dirty root가 revision gap이나 coalescing에서 잘못 귀속되지 않게 한다.
3. 이전 computed-style 항목을 NodeId로 재사용하며 ancestor context와 dirty subtree만 Stylo cascade하는 경로를 추가한다. 재계산·재사용·context 수를 실제 호출 지점에서 계수한다.
4. 실패·panic·generation/profile/environment 변화와 full fallback을 포함한 runtime 테스트를 추가한다.
5. full-cascade oracle와 release benchmark를 실행하고, Chromium과 Android·iOS Simulator의 계산값·geometry·화면을 대조한다.
6. 구현을 계획 검토와 다른 20개 실패 관점에서 검토한다. C05.4만 상태 대장에서 완료하고 C05 부모 및 명시적 미포함 항목은 미완료로 유지한다.
