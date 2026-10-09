# 0030 · C04.9 Runtime CSS → Taffy 레이아웃 snapshot

**계약 버전:** 미출시 내부 계약 0.1.0 고정 · **상태:** Android·iOS 시뮬레이터 실행 검증 · **공개 API:** 아님 · **제품 CSS 지원:** 미완료

## 범위

RuntimeSession이 보유한 immutable HostDocument snapshot에서 HTML style 속성을 Stylo로 한 번 계산하고, 계산된 제한 CSS profile을 Taffy로 투영한다. JS task 뒤 기존 CSS worker가 이 작업을 순차 실행하며, caller는 현재 상태와 결과를 JSON으로 복사한다. 같은 revision의 UA cascade와 layout을 묶되 둘의 상태는 분리한다.

이 내부 기능은 CSS 계산 결과와 CSS px 프레임을 제공한다. GPU 장면·화면 표시, 일반 HTML/CSS 구현, CSSOM, author stylesheet 전달, 외부 CSS URL 로딩, 공개 앱 API는 제공하지 않는다. 계약과 구현 계획은 이 기능만으로 숫자 버전을 올리지 않으며 0.1.0을 유지한다.

## 내부 C ABI

    int32_t spinon_runtime_session_copy_layout_json(
        SpinonRuntimeSession *session,
        char *output,
        size_t output_capacity,
        size_t *required_capacity);

- 계산을 기다리지 않고 상태 한 개를 UTF-8 JSON으로 복사한다. 크기 비용은 노드·진단 수에 비례하므로 UI thread에서 호출하지 않는다.
- output은 output_capacity만큼 쓸 수 있고, required_capacity는 쓰기 가능한 size_t여야 한다. 두 출력 memory 범위는 서로 또는 session 저장 공간과 겹치면 안 된다. 세션 해제와 동시에 호출할 수 없다.
- 성공은 0, 잘못된 포인터 또는 capacity는 -1, 버퍼 부족은 -3이다. required_capacity는 마지막 NUL을 포함한다. 부족한 버퍼에는 부분 JSON을 쓰지 않으며 첫 바이트를 쓸 수 있으면 NUL로 둔다. 재시도할 때 상태가 바뀔 수 있으므로 매 호출마다 필요한 크기를 다시 읽는다.
- 이 readback은 계산 완료를 보장하거나 UI 표시 frame을 승인하지 않는다. requested와 completed.key 전체가 같을 때만 현재 layout 결과다.

## JSON 계약

    {
      "schema": "spinon.runtime.layout",
      "state": "ready",
      "requested": {
        "generation": 1,
        "documentRevision": 20,
        "renderTreeRevision": 20,
        "styleRevision": 0,
        "environmentRevision": 1
      },
      "completed": {
        "key": {
          "generation": 1,
          "documentRevision": 20,
          "renderTreeRevision": 20,
          "styleRevision": 0,
          "environmentRevision": 1
        },
        "unit": "css-px",
        "projectionDurationUs": 37,
        "frames": [
          { "nodeId": 1, "x": 0, "y": 0, "width": 300, "height": 140 }
        ]
      },
      "diagnostics": [],
      "error": null
    }

- schema 이름은 고정 문자열이다. JSON 안에 별도 숫자 schema version은 두지 않는다. 내부 계약 숫자 버전은 출시 전 0.1.0으로 유지한다.
- revision key는 generation, document revision, render-tree revision, style revision, environment revision 전체다. 숫자의 크기만으로 최신성을 판정하지 않는다. 새 요청을 등록하면 이전 completed를 지우며, worker는 완료 시 현재 요청 key와 다른 결과를 게시하지 않는다.
- state는 not_configured, pending, ready, empty, failed다. 환경 또는 입력 snapshot이 준비되지 않았으면 not_configured, 요청 처리 중이면 pending, 한 개 이상의 element frame이 있으면 ready, HostRoot가 비어 있으면 성공 상태 empty, 최신 요청이 실패하면 failed다.
- completed는 ready·empty에서만 제공하며, 현재 요청과 같은 key를 갖는다. pending·failed·not_configured에서는 null이다. 실패 결과의 일부 frame을 내보내지 않는다.
- 각 frame은 nodeId, x, y, width, height를 CSS px로 표현한다. 순서는 HostRoot의 DOM preorder이며 생성 ID 순서가 아니다. 모든 값은 유한해야 하고 크기는 음수가 아니어야 한다. display:none 노드 및 그 자손은 (0,0,0,0)으로 보고한다.
- projectionDurationUs는 미지원 author declaration 검사 이후 Taffy 계산과 frame 검증·DOM 순서 생성을 포함한다. cascade, queue 대기, JSON 복사, HostDocument snapshot 생성은 포함하지 않으며 성능 보장값이 아니다.
- diagnostics는 Stylo parser의 syntax recovery 진단 배열이며 각 항목은 sourceId, nodeId, line, column, message를 포함한다. 일반 CSS syntax 오류가 있으면 브라우저식 recovery로 유효 선언을 계산하고 진단을 보존한다.
- error는 실패 시 { "code": "...", "nodeId": null, "property": null } 형태다. CSS 원문이나 계산 값을 담지 않는다. 아래 목록 외의 detail을 새 소비자 계약으로 해석하지 않는다.

## 입력과 계산 범위

입력은 C04.8과 동일한 HTML fragment-root Stylo 문서 의미, 내장 UA stylesheet, HostDocument의 기존 HTML style 속성, 명시된 viewport/media 환경이다. root를 CSS document element로 바꾸지 않는다. 같은 HostDocument revision을 UA profile과 layout profile에 한 번 cascade해 결과를 공유한다. C04.8의 기존 UA JSON은 원래 7개 property, key, schema와 필드 구성을 보존한다.

허용되는 계산 입력은 display, box-sizing, width/height, flex-direction, flex grow/shrink/basis, direction, align-items, justify-content, gap, margin과 padding의 physical/logical shorthand·longhand다. flex shorthand는 Stylo가 longhand로 확장한 결과가 허용 목록에 있을 때 통과한다. 예를 들어 flex: 0 1 auto는 허용되고, flex: 1이 계산한 percentage flex-basis는 지원되지 않아 실패한다. 유효하지만 목록 밖인 declaration은 무시하지 않고 layout만 실패시킨다.

Taffy 입력은 display flex/block/none, px 또는 auto 크기, 제한 Flex 속성, 유한한 0 이상 px padding과 C04 margin profile이 허용하는 px margin으로 한정한다. 단일 HostRoot 직속 element 하나와 하위 element만 지원한다. root의 nonzero margin, auto margin, 음수 padding, percentage/font-relative computed value, 여러 root, text node, inline formatting, replaced element, list marker는 지원하지 않는다. viewport는 root element의 containing block이다. 새 slice가 지원하지 않는 값은 기본값으로 대체하지 않는다.

## 오류 코드

| code | 의미 |
|---|---|
| cascade_failed | DOM root 또는 Stylo cascade 입력을 계산하지 못함 |
| worker_unavailable, worker_failed | CSS worker를 사용할 수 없거나 panic으로 종료됨 |
| multiple_host_roots | HostRoot 직속 element가 둘 이상임 |
| unsupported_inline_property | 유효한 inline declaration이 runtime layout allowlist 밖임 |
| unsupported_computed_value | 계산된 property value가 Taffy projection 범위 밖임 |
| unsupported_root_margin | layout root의 nonzero margin |
| unsupported_text_node | element-only slice에서 text node를 만남 |
| invalid_root | layout root가 입력 계약에 맞지 않음 |
| snapshot_mismatch | 스타일과 layout 입력 snapshot의 revision/identity 불일치 |
| missing_frame, invalid_frame, frame_set_mismatch | Taffy 결과 frame이 DOM 요소 집합·수치 계약과 맞지 않음 |
| layout_failed | 위 코드로 좁힐 수 없는 Taffy 입력·계산 실패 |

layout 실패는 UA cascade의 완료 결과를 덮어쓰지 않는다. 다만 cascade 자체가 실패하면 cascade_failed를 layout 상태에도 기록한다. 진단 오류에서 임의 CSS value를 복사하지 않는다.

## 비교·검증 기준

비교 oracle은 [구현 전 고정 Chromium 기준](evidence/css-c04-runtime-style-layout-precomparison-2026-10-09.md)이다. 고정 HTML/reference/hash는 수정하지 않는다. 각 computed property는 문자열 정확 비교, 각 노드 geometry 좌표는 최대 절대 오차 0.5 CSS px로 비교하며 평균으로 좌표 실패를 상쇄하지 않는다.

Android API 37.1 ARM64 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서는 실제 V8 DOM 변경 → CSS worker → layout JSON 복사를 검증한다. 시뮬레이터 통과는 실기기, GPU draw, 화면 표시 callback, 일반 CSS 지원 또는 성능을 증명하지 않는다. [계획 적대 검토](evidence/c04-runtime-style-layout-plan-review-2026-10-09.md), [구현 후 검토 기록](evidence/c04-runtime-css-taffy-implementation-review-2026-10-09.md), [시뮬레이터 실행 기록](evidence/c04-runtime-css-taffy-simulators-2026-10-09.md)에 각각 계획·코드·플랫폼 evidence를 기록한다.
