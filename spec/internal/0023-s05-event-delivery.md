# 0023 · S05 입력 이벤트 전달과 callback 소유권

**인터페이스 버전:** 0.1.0-draft · **상태:** 첫 클릭 경로 내부 계약 제안 · **구현:** 미구현 · **공개 API:** 아님 · **상태 대장:** S05.1

## 목적과 범위

Android·iOS 입력을 GPU 장면의 노드 ID에 연결하고, 대상 Isolate에서 프레임워크 handler를 비동기로 호출할 때 지킬 내부 경계를 정합니다. S04.9의 정적 snapshot hit-test와 달리 제품 입력은 플랫폼 presentation 확인 신호로 선택한 frame의 불변 snapshot을 출처로 삼아야 합니다.

이 계약은 첫 React 수직 구현에서 OwnerId당 하나의 root를 두고, 플랫폼이 인식한 단일 터치 활성화 결과를 click callback으로 보내는 내부 경로만 다룹니다. DOM Event 객체, 캡처·버블링, 기본 동작·동기 취소, pointerdown/up/move/cancel, 키보드·접근성 활성화, 여러 React root·portal, DOM listener API, 웹 호환 지원을 정의하거나 구현 완료로 선언하지 않습니다.

여기서 presentation-confirmed frame은 GPU에 제출한 frame이 아닙니다. 플랫폼이 해당 frame의 presentation을 확인하는 callback을 처리한 frame입니다. 이 신호는 광학 scanout 시각이나 사용자가 실제 픽셀을 보았다는 사실을 증명하지 않습니다. Queue.submit, Queue.present, 성공한 표면 획득만으로 presentation-confirmed 상태를 만들지 않습니다. 이 계약은 플랫폼 확인 신호를 비교 가능한 frame 경계로 쓰자는 제안이며, 어떤 API가 그 근거가 될지는 S04.10에서 플랫폼별로 검증해야 합니다.

## 전달 흐름

1. Rust renderer가 하나의 revision tuple을 가진 불변 render snapshot과 FrameId를 만듭니다. snapshot의 hit-test 데이터와 frame stamp는 같은 원자 결과에서 나와야 합니다.
2. 플랫폼 GPU 호스트가 제출할 때 현재 runtime·surface generation의 제출 기록에 FrameId와 snapshot을 등록합니다. 이 단계에서 hit-test에 쓸 frame을 교체하지 않습니다. 확인 전에 제출 기록을 해제하거나 용량 제한 없이 쌓아 두지 않으며, 제한·만료 정책은 S04.10에서 정합니다.
3. 플랫폼 presentation 확인 결과와 입력 callback은 같은 직렬 UI 호스트 실행 경로에서 순서가 정해집니다. 확인 결과는 같은 runtime·surface generation에서 실제 제출된 FrameId를 지목해야 합니다. 알 수 없는 frame, 다른 generation의 frame, 현재 surface에서 제출되지 않은 frame의 확인은 적용하지 않습니다. GPU callback은 표식만 UI 호스트 경로로 전달하고 JS나 HostDocument를 직접 만지지 않습니다.
4. 확인을 처리하면 그 frame과 같은 revision tuple의 hit-test snapshot을 현재 surface generation의 presentation-confirmed frame으로 원자 교체합니다. 입력은 자기 callback의 선형화 시점에 이 snapshot을 선택합니다.
5. 입력 callback은 현재 surface generation과 확인된 frame이 일치하는지 확인하고, 해당 frame의 viewport·surface transform으로 좌표를 CSS px로 바꿉니다. 좌표와 transform은 유한수여야 하며 viewport는 양수여야 합니다. viewport는 왼쪽·위쪽 경계 포함, 오른쪽·아래쪽 경계 제외의 반열린 사각형으로 판정합니다. 좌표가 letterbox 바깥이면 거부합니다. 선택한 frame snapshot에서 NodeId를 한 번 판정하고 결과와 출처 revision을 immutable envelope에 복사해 Isolate queue로 제출합니다.
6. JS Isolate는 프레임워크 HostCommit 사이에서만 envelope를 처리하고, 세션·문서·owner·노드·현재 handler를 확인한 뒤 callback을 호출하거나 안정된 이유 코드로 버립니다.

presentation 확인이 입력보다 먼저 UI 직렬 경로에 도착하면 새 frame을 씁니다. 입력이 먼저 도착하면 이전에 확인된 frame을 씁니다. 확인 결과가 제출 기록과 연결되지 않거나 처리 순서가 정해지지 않으면 순서를 추정하거나 새 문서로 다시 hit-test하지 않고 확인 또는 입력을 거부합니다.

## FrameId와 revision tuple

각 이벤트 envelope는 아래 식별값을 함께 보존합니다. 값의 일부를 최신 값으로 바꾸어 혼합하지 않습니다. `node_id`는 해당 `document_generation`의 내부 `HostNodeHandle`에 해당하는 Rust 노드 ID이며, JS façade ID나 공개 DOM ID가 아닙니다.

| 값 | 범위와 의미 |
| --- | --- |
| runtime_generation | V8 세션 수명. 세션 재생성 때 달라집니다. |
| document_generation | HostDocument 수명. 문서 재생성 때 달라집니다. |
| frame_id | 같은 runtime generation 안에서 1부터 시작하는 frame별 유일한 64비트 순번입니다. 제출 기록과 presentation 확인은 이 ID·surface generation·revision tuple에 정확히 결합됩니다. 넘침 때 재사용하지 않고 새 frame 생성과 입력 경로를 닫습니다. |
| surface_generation | 플랫폼 표면·크기·backing scale 수명입니다. S04 표면 generation과 같은 값을 전달합니다. |
| document_revision | snapshot을 만든 HostDocument의 논리 revision입니다. |
| render_tree_revision | snapshot에 반영된 연결 표시 트리 revision입니다. |
| style_revision | snapshot에 반영된 스타일 입력 revision입니다. |
| environment_revision | viewport·배율 등 snapshot에 반영된 플랫폼 환경 revision입니다. |
| input_sequence | 한 runtime generation에서 UI 호스트가 tap 입력 처리를 시작할 때 1부터 할당하는 64비트 순번입니다. 이후 거부된 입력도 순번을 소비할 수 있어 번호 사이 빈칸은 허용합니다. 넘침 때 재사용하지 않고 이후 입력을 거부합니다. |
| owner_id, node_id | frame snapshot에서 판정한 owner와 정확한 입력 target입니다. S05 첫 React slice에서는 OwnerId당 root 하나만 둡니다. |
| event_kind, coordinates_css_px | 첫 slice에서는 click과 입력 지점만 기록합니다. 좌표는 JS 공개 이벤트 객체가 아닙니다. |

FrameId는 submission sequence나 SurfaceGeneration의 별칭이 아닙니다. presentation 확인은 현재 runtime·surface generation의 아직 유효한 제출 기록과 일치하고, 이미 처리한 frame보다 뒤의 frame일 때만 적용합니다. 오래되거나 다른 표면에서 온 확인은 기존 last-presented 값을 바꾸지 않습니다. surface generation 변경은 이전 표시 snapshot을 즉시 비우며 새 generation의 presentation 확인 전 입력은 거부합니다. 제출 기록의 최대 개수, 확인 제한 시간, 미확인 frame 해제 시점은 S04.10에서 정하고, 이 표의 한도 없는 보존은 금지합니다.

현재 S04.9는 submitted frame sequence와 고정 fixture를 기록합니다. 실제 presentation 확인, 동적 snapshot, 이 revision tuple을 담은 제품 입력을 검증하지 않았습니다. 따라서 현 S04.9 fixture만으로 이 전달 경로를 지원한다고 표시할 수 없습니다.

## 첫 입력·callback 정책

- 첫 입력 종류는 플랫폼에서 단일 손가락 tap으로 판정한 click 하나입니다. 플랫폼 recognizer의 이동 한계·취소·두 손가락 배제 기준은 Android·iOS host가 각각 기록하며 웹과 같다고 간주하지 않습니다.
- hit-test는 입력 시점의 presentation-confirmed snapshot에서 한 번만 합니다. 좌표는 현재 문서나 최신 frame에서 다시 해석하지 않습니다. 제출됐지만 확인되지 않은 frame이나 아직 제출 기록에 없는 frame은 hit-test에 쓸 수 없습니다.
- 입력 target NodeId는 전달 동안 바뀌지 않습니다. 최신 문서에서 해당 노드가 같은 document generation과 React owner root에 연결되어 있으면 handler를 실행합니다. document/style/environment revision이 더 최신이라는 이유만으로 입력을 버리지는 않습니다. 입력 뒤 노드가 같은 root 안에서 이동·숨김되거나 새 frame이 만들어져도 다른 노드로 retarget하지 않습니다.
- 전달 시점에 노드가 분리·폐기됐거나 문서 세대·owner root가 달라졌으면 callback을 호출하지 않습니다. 가장 가까운 조상으로 바꾸지 않습니다. 같은 HostCommit 안에서 일시 분리 후 다시 삽입됐다면 commit 종료 뒤의 최종 연결 상태를 사용합니다.
- callback은 대상 Isolate의 기존 UserBlocking JS 작업 등급으로 제출합니다. Isolate에서 이미 실행 중인 JavaScript는 선점하지 않으며 UI·GPU 경로는 callback 완료를 기다리지 않습니다. 등급 간 선택은 [0006 스케줄러 계약](0006-js-task-scheduler.md), 같은 등급의 FIFO만 이 문서의 입력 순번을 따릅니다.
- click은 개별 입력으로 보존하며 병합하지 않습니다. runtime queue 제출은 UI 호스트에서 용량이나 JS 실행을 기다리는 blocking 호출이 아니어야 합니다. queue가 포화됐거나 즉시 접수할 수 없으면 UI 호스트는 대기·재시도하지 않고 해당 순번을 queue_full로 진단합니다. 현재 실험 queue의 64개 제한은 제품 capacity 계약이 아닙니다.
- callback을 찾지 못하면 오래된 closure를 호출하지 않고 handler_missing으로 처리합니다. enqueue 뒤 callback이 교체되면 Isolate 처리 시점의 최신 등록을 사용합니다. callback 제거 뒤에는 과거 등록을 다시 사용하지 않습니다.
- callback 등록부와 V8 strong Global은 Isolate owner가 소유합니다. Rust 트리에는 V8 handle을 저장하지 않습니다. 등록 교체·해제·root unmount·세션 종료는 Isolate owner에서만 적용합니다. 프레임워크 commit 동안 입력 task는 re-enter하지 않으며, commit이 끝나기 전에 트리와 callback 등록부의 변경을 함께 공개해야 합니다. HostCommit 도중 호스트 명령이 실패해 두 상태가 일치하지 않으면 root를 입력 불가 상태로 닫고, 부분 상태에서 callback을 호출하지 않습니다. 복구·재생성 방법은 React adapter 계약에 남습니다.
- callback 예외는 현재 runtime의 예외 경계로 한 번 보고하고 재시도하지 않습니다. 후속 queue task가 계속되는지와 진단 보존은 S05 실행 fixture에서 확인해야 합니다.
- 세션이 Closing에 들어가는 순간 신규 입력 접수를 닫고 이미 대기 중인 입력 task는 `session_closing`으로 종료합니다. 이미 실행을 시작한 callback은 R06에서 정한 실행 중 JS 취소·종료 규칙을 따르며, shutdown이 UI thread에서 callback 완료를 동기 대기하지 않습니다. callback Global 해제는 [0021 수명 계약](0021-s03-dom-node-lifecycle.md)의 owner-thread 순서를 따릅니다.

onClick의 JavaScript 인수·event object 형태와 TypeScript 선언은 이 내부 envelope로 정하지 않습니다. React adapter의 versioned public API contract가 작성되기 전까지는 onClick을 공개 지원으로 표시하지 않습니다.

## 명시적 차이와 미결 항목

분리된 target을 버리고 상위 노드로 retarget하지 않는 정책은 첫 slice의 fail-closed 정책입니다. Pointer Events Level 3의 click target 알고리즘은 pointerdown·pointerup의 DOM target을 이용해 dispatch 시점의 가장 가까운 공통 조상을 구합니다. mapped mouse event 규칙도 원 target이 문서 트리에서 빠졌을 때 제거 시점의 가장 가까운 연결 조상을 권고합니다. 이 첫 slice는 두 알고리즘의 입력·event path를 구현하지 않으므로 의도적 차이가 있으며 웹 click 적합성을 주장하지 않습니다. 전체 DOM dispatch, listener 전파·제거 시점, 포커스·disabled button 동작은 J12 및 S07 계약에 남습니다.

다음은 이 draft가 플랫폼 구현 완료로 간주하지 않는 사항입니다.

- Android와 iOS에서 실제 presentation 결과를 받을 API, wgpu 경계 밖의 연동 방법, 신호가 없는 backend의 동작
- S05.2 첫 입력은 해당 frame의 viewport·surface transform을 사용하지만 일반 CSS transform·clip·stacking-context hit-test는 제공하지 않음
- 두 플랫폼에서 presentation 결과와 입력 callback의 직렬화 지점 및 실제 순서 로그
- 제품 render snapshot이 모든 revision과 interactive NodeId mapping을 제공하는 방식
- React renderer의 HostCommit 시작·종료 callback과 여러 root/portal의 OwnerId 정책
- HostCommit 실패 뒤 adapter가 root를 재생성하거나 앱에 오류를 노출하는 구체 복구 동작
- queue 용량·역압력 사용자 진단, runtime generation 할당, 안정된 ABI 자료형
- callback 뒤 Promise reaction·microtask checkpoint와 다음 task의 상대 순서; 이 경로에서 표준 이벤트 루프 적합성을 주장하지 않음
- React onClick event argument, TypeScript 타입, 오류 화면·개발 진단 노출
- keyboard·VoiceOver·TalkBack activation, disabled·inert·pointer-events, 기본 동작과 동기 preventDefault

## 내부 진단 결과

아래 reason code는 이 내부 계약의 제안이며 공개 JavaScript API나 안정 ABI가 아닙니다. 원본 입력의 user data를 포함하지 않고 runtime/document generation, FrameId, input sequence와 reason을 진단에 남깁니다. 진단의 용량·보존 기간·개발 도구 노출은 S05.2에서 정하며 이 표가 무제한 저장을 뜻하지 않습니다.

| reason code | 거부·진단 조건 |
| --- | --- |
| no_presented_frame | 현재 surface generation에서 표시 확인된 frame이 없음 |
| presentation_unknown_frame | 확인 callback이 현재 runtime·surface generation의 제출 기록과 연결되지 않음 |
| stale_runtime_generation | 입력·frame이 현재 runtime session과 다름 |
| stale_document_generation | 입력·frame이 현재 HostDocument 수명과 다름 |
| stale_surface_generation | 입력 또는 presentation 확인이 이전 surface generation에서 옴 |
| frame_out_of_order | 이미 확인한 frame보다 같거나 오래된 presentation 결과 |
| frame_id_exhausted | runtime 안에서 다음 고유 FrameId를 만들 수 없어 frame·입력 경로를 닫음 |
| input_sequence_exhausted | runtime 안에서 다음 고유 입력 순번을 만들 수 없어 신규 입력을 거부함 |
| invalid_frame_transform | 해당 frame의 CSS viewport·surface transform이 없음 또는 잘못됨 |
| outside_viewport | point가 letterbox 또는 유효 viewport 밖에 있음 |
| hit_miss | 유효한 frame에는 입력 가능한 target이 없음 |
| target_detached | dispatch 전에 target이 root에서 분리·폐기됨 |
| owner_mismatch | target 또는 callback이 현재 OwnerId에 속하지 않음 |
| handler_missing | dispatch 시점에 현재 click callback이 없음 |
| queue_full | Isolate queue가 입력 task를 명시적으로 거부함 |
| session_closing | runtime 종료가 입력보다 먼저 선형화됨 |
| callback_exception | JS callback이 예외를 던짐; 재시도하지 않음 |

## S05.2 실행 전 고정 검증 행렬

이 표는 향후 S05 실행의 통과 조건이지 현재 통과 결과가 아닙니다.

| 사례 | 기대 결과 |
| --- | --- |
| 확인된 frame이 없는 surface에서 입력 | callback 없음, no_presented_frame |
| NaN·무한 좌표, 0 크기 viewport, 잘못된 backing scale | callback 없음, invalid_frame_transform |
| 제출 기록이 없는 frame의 presentation callback | last-presented 상태 불변, presentation_unknown_frame |
| 같은 FrameId에 중복되거나 역순으로 온 확인 callback | last-presented 상태 불변, frame_out_of_order |
| submit 뒤 presentation 확인 전 입력 | 직전 확인 frame으로만 판정하거나 이전 frame이 없으면 거부; 제출 frame은 사용하지 않음 |
| 새 surface generation 이후 이전 generation 확인·입력 | 이전 확인은 상태를 바꾸지 않고 입력 거부 |
| 제출 기록 만료 뒤 늦은 presentation callback | last-presented 상태 불변, presentation_unknown_frame; 만료·용량 정책은 무한 보존으로 우회하지 않음 |
| 입력 callback과 presentation 확인이 연속 도착 | UI host 실행 순서와 선택한 FrameId가 로그에서 일치 |
| frame 확인 뒤 target이 같은 owner root 안에서 이동·숨김 | 입력 시 판정한 NodeId가 현재 연결되어 있으면 해당 NodeId의 현재 handler만 실행 |
| envelope 생성 뒤 표면 resize, JS dispatch 전 | 이미 선택한 target·revision을 유지하되 session/document가 바뀌거나 target이 분리되면 거부 |
| callback 처리 전 target 제거 또는 다른 root로 이동 | callback 없음; 조상·아래 노드로 retarget하지 않음 |
| 입력 뒤 새 handler 등록, callback 처리 전 등록 완료 | 처리 시점의 현재 handler를 실행; 오래된 handler는 호출하지 않음 |
| 같은 HostCommit 안에서 target detach 후 재삽입 | commit 종료 뒤 같은 NodeId가 연결되어 있으면 현재 handler 실행 |
| 한 commit에서 분리한 뒤 별도 commit에서 같은 NodeId를 다시 연결 | dispatch 시점에 동일 document generation·owner root에 연결돼 있으면 현재 handler 실행; NodeId 재사용은 금지 |
| 같은 NodeId에서 callback 교체·제거·다시 등록 | dispatch 전에 교체됐으면 최신 handler 1회, 제거 상태면 callback 0회, 다시 등록되면 현재 등록 handler 1회 |
| 연속 click과 우선순위 혼합 대기 작업 | click끼리 순번 FIFO, runtime의 등급 선택과 OS 입력 처리는 독립 |
| queue 포화, callback throw, HostCommit 실패, 세션 종료 | UI 비대기, 입력의 명시적 거부·진단, callback 예외 1회 보고 뒤 다음 유효 task 진행, commit 실패 root는 복구 전까지 입력 거부 |
| 대기 중 세션 종료와 이미 실행 중인 callback | 대기 입력은 session_closing, 실행 중 JS는 R06 취소·종료 규칙을 따르고 UI host는 완료를 기다리지 않음 |
| surface resize·문서 재생성·frame/입력 번호 소진 | 예전 surface 확인은 무시하고 이전 generation 입력은 거부; 재생성 후 ID 재사용 없음, 번호 소진은 fail-closed |

실행 비교는 동일한 한 개 button과 두 개 입력 좌표를 가진 고정 작은 화면으로 시작합니다. Chromium elementFromPoint()와 표준 event source는 DOM 목표 차이를 확인하는 기준일 뿐, 이 제한된 GPU route의 전체 적합성 oracle이 아닙니다. Android·iOS 시뮬레이터의 실제 입력, JS callback 횟수·순서·오류, 사용 frame/revision 로그가 모두 필요합니다. 실기기와 광학 표시시각·성능 검증은 이 단계에 포함하지 않습니다.

## 참고 기준

- [WHATWG DOM · event dispatch](https://dom.spec.whatwg.org/#concept-event-dispatch) — 이벤트 경로와 listener 목록의 규범 기준.
- [W3C UI Events · native mouse click](https://www.w3.org/TR/uievents/event-algo.html#handle-native-mouse-click) — 입력 target hit-test 뒤 click을 만들고 dispatch하는 알고리즘.
- [W3C Pointer Events Level 3 · click event dispatch](https://www.w3.org/TR/pointerevents3/#the-click-auxclick-and-contextmenu-events) — pointerdown·pointerup target의 가장 가까운 공통 조상으로 click target을 정합니다. [mapped mouse event target](https://www.w3.org/TR/pointerevents3/#compatibility-mapping-with-mouse-events)은 원 target이 ownerDocument tree를 떠난 경우 nearest connected ancestor를 권고합니다. Spinon 첫 slice는 이 두 알고리즘을 구현하지 않습니다.
- [0006 JavaScript 작업 스케줄러](0006-js-task-scheduler.md) — UI 입력 callback의 기존 UserBlocking 실행 등급.
- [0019 S04 GPU fixture 계약](0019-s04-css-layout-gpu-slice.md) — 현재 submitted-frame fixture의 구현 범위와 한계.
- [0021 DOM node·wrapper 수명](0021-s03-dom-node-lifecycle.md) — V8 callback Global과 세션 종료 owner 규칙.
