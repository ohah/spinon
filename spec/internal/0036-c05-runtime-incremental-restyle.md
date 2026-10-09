# 0036 · C05.4 runtime 하위 트리 cascade 재계산

- **내부 계약 버전:** `0.1.0` 고정 · 출시·호환성 정책을 사용자가 정하기 전 숫자를 올리지 않는다.
- **상위 상태:** [C05 사용자 지정 속성과 재계산](../STATUS.md#css-구현-체크리스트)
- **작업 계획:** [C05.4 구현 계획](../../plan/c05-runtime-incremental-restyle.md)
- **구현 전 비교:** [Chromium 기준](evidence/c05-runtime-incremental-restyle-precomparison-2026-10-10.md)
- **계획 검토:** [계획 실패 관점 검토](evidence/c05-runtime-incremental-restyle-plan-review-2026-10-10.md)
- **구현 검토와 실행 근거:** [구현 실패 관점 검토](evidence/c05-runtime-incremental-restyle-implementation-review-2026-10-10.md)

## 목적과 공개 범위

C05.4는 안전 조건을 모두 증명할 수 있는 runtime inline `style` 변경에서 Stylo cascade 출력의 dirty subtree만 다시 계산한다. 계산 worker는 직전 성공 cascade 출력 한 건을 보유한다. 전체 Taffy layout, frame 변환, renderer scene 구성은 기존처럼 현재 연결 트리 전체에서 다시 수행한다.

이 문서는 Rust runtime worker와 내부 C ABI 진단 경계의 계약이다. `Element`, 공개 JS API, CSSOM 또는 앱 작성자용 성능 보장을 추가하지 않는다. fixture 실행용 `spinon_runtime_gpu_host_eval_incremental_restyle_fixture` 진입점은 검증 앱 전용이다. 기존 `spinon.runtime.ua-cascade.v1` 완료 JSON의 cascade 재계산·재사용·context 수는 내부 진단값이다.

## 입력과 무효화 분류

`HostDocumentSnapshot` 전후 쌍을 worker에서 분류한다. 분류기는 한 generation에서 revision이 증가한 단일 HostRoot 장면만 후보로 받으며 다음 연결 입력이 동일한지 확인한다.

- 노드 ID·종류·owner, element namespace·local name·state, text data
- parent와 자식 ID·순서
- HTML 요소의 namespace 없는 `style` 이외 모든 attribute name·value

HTML의 namespace 없는 `style` 속성만 달라지면 그 element를 변경 root로 표시한다. HTML 속성 이름 비교는 ASCII 대소문자를 무시한다. 중복된 대소문자 변형 이름, 다른 namespace의 속성 변경, 모호한 요소, snapshot에서 빠진 handle은 전체 cascade로 되돌린다. 겹치는 변경 root는 현재 snapshot에서 조상 관계를 확인해 가장 바깥 root만 남긴다.

다음 조건을 **모두** 만족해야 직전 스타일 출력을 부분 재사용할 수 있다.

1. 같은 generation, 단일 root ID, 계산 profile, StyleRevision, EnvironmentRevision, viewport와 media 입력이다.
2. 이전 스타일 출력의 node ID 집합과 현재 연결 element 집합이 같다.
3. 변경 입력이 위에서 허용한 inline `style` 속성뿐이며 이전 cache revision과 변경 분류의 source revision이 정확히 이어진다.
4. 연결 author stylesheet가 없다. 내장 UA stylesheet는 고정 profile이어야 하며 selector는 요소 namespace·이름 이외의 속성·상태·관계 입력에 의존하지 않아야 한다.
5. 이전 결과가 worker에서 성공적으로 publish된 결과다.

입력이나 selector 의존성이 이 조건을 벗어나면 `RuntimeStyleInvalidation::Full`로 승격하고 전체 cascade를 수행한다. 증분 API가 `None`을 반환하는 것은 오류가 아니라 전체 계산으로 되돌릴 신호다. cascade 자체가 실패하면 worker 오류 경계를 유지하고 style cache를 비운다.

### 분류 결과

| 결과 | 의미 | 계산 |
| --- | --- | --- |
| `Full` | 기준 snapshot 부재, 구조·상태·비-style 입력 변화, revision 단절, stylesheet·환경 변화, 모호성 | 전체 Stylo cascade |
| `Unchanged` | revision은 달라졌으나 연결 스타일 입력은 동일 | 기존 직렬화 스타일 출력 재사용, 현재 진단 재수집 |
| `Subtrees` | 허용된 inline `style` 변경 root 하나 이상 | ancestor context 계산 + 변경 subtree cascade + 나머지 출력 재사용 |

## cascade 계산 규칙

- 변경 root의 모든 연결 element 자손에 현재 inline declaration을 적용한다. 부모의 사용자 지정 속성이나 inheritance가 바뀌면 그 subtree 전체를 다시 cascade한다.
- dirty root부터 document root까지의 element ancestor는 새 `ComputedValues`를 계산해 하위 계산 context로 전달한다. 해당 ancestor의 직렬화 출력은 스타일 입력이 변하지 않았으므로 이전 출력을 재사용한다.
- 변경 subtree 밖 element는 이전 `ComputedElementStyle`의 직렬화 결과를 NodeId로 찾아 복사한다. 이전 `ComputedValues`, Stylo DOM adapter, V8 객체, HostDocument snapshot이나 handle을 cache에 장기 보관하지 않는다.
- 진단은 과거 snapshot에서 복사하지 않고 현재 view에서 다시 수집한다.
- 스타일 출력은 임시 cascade 전체가 성공한 뒤, 최신 request key 검사까지 통과한 경우에만 worker cache로 교체한다. stale 결과는 published snapshot과 cache를 갱신하지 않는다.
- pending request는 연속된 revision 경계에서만 가장 이른 기준 snapshot과 최신 snapshot을 합친다. worker cache base와 source revision이 다르면 full fallback을 사용한다.
- 환경·profile·generation 변화, root 수 변경, DOM 구조·text·state·`class`·`id`·기타 attribute 변경, 연결 stylesheet 존재 또는 UA selector invariant 실패는 부분 재사용을 금지한다.

## 진단 필드 의미

기존 완료 JSON에 다음 내부 필드가 있다.

| 필드 | 단위·계산 | 해석 |
| --- | --- | --- |
| `cascadeRecomputedStyleElements` | 요소 수 | 현재 request에서 새 Stylo cascade 결과를 직렬화한 요소 수 |
| `cascadeReusedStyleElements` | 요소 수 | 이전 직렬화 출력 항목을 복사한 요소 수. context 요소도 이 값에 포함된다. |
| `cascadeContextStyleElements` | 요소 수 | computed inheritance context는 다시 계산했지만 직렬화 출력은 재사용한 조상 수. `reused`의 부분집합이다. |
| `computationDurationUs` | cascade 누적 시간 | Stylo cascade 단계 시간이다. 전체 request latency가 아니다. |
| `projectionDurationUs` | layout/scene projection 누적 시간 | Taffy layout, frame mapping, renderer scene snapshot 생성 시간이다. GPU 제출·화면 표시 시간은 포함하지 않는다. |

전체 재계산에서는 연결 element 수만큼 `recomputed`가 증가한다. 부분 계산에서는 dirty subtree의 실제 재계산 요소 수만큼 증가한다. 카운터는 layout 시간, GPU frame time, 앱 전체 성능을 뜻하지 않는다.

## 런타임 재요청·fallback 표

| 입력 변화 | 처리 | cache 갱신 |
| --- | --- | --- |
| inline `style` 변경이 없는 연결 tree revision | style output 재사용, 전체 layout·scene | 최신 request로 publish된 성공 결과만 보관 |
| 단일 subtree의 허용 inline `style` 변경 | ancestor context + dirty subtree cascade, 전체 layout·scene | 전체 cascade 결과 snapshot 성공 뒤 교체 |
| 여러 sibling의 허용 style 변경 | 겹치지 않는 각 dirty subtree만 cascade | 최신 결과가 성공적으로 publish될 때 교체 |
| style 제거/빈 값·`var()` 값 변경 | 위 입력 규칙에 따라 subtree cascade; Stylo computed 값 사용 | 성공 결과만 교체 |
| stylesheet, 구조, 비-style 속성, state, text 변경 | 전체 cascade fallback | 오류면 cache 제거 |
| revision gap·cache 기준 불일치·worker panic | 전체 cascade fallback 또는 worker 실패 | 실패/오래된 계산은 cache에 남기지 않음 |
| viewport/media/profile/generation 변경 | 전체 cascade | 새 환경에서 계산 성공 후 교체 |

Taffy와 renderer scene은 `Unchanged`와 `Subtrees`에서도 전체 트리를 다시 순회한다. layout·paint dirty region, GPU 부분 제출, CSS selector invalidation map은 이 계약에 포함되지 않는다.

분류기는 이전·현재 snapshot을 비교하며 재사용 계획과 새 직렬화 출력도 전체 연결 요소 수에 비례해 확인·복사한다. 따라서 이 구현이 줄이는 것은 주로 Stylo cascade 호출 수다. 이 단계만으로 subtree 크기에 비례하는 전체 처리 시간이나 프레임 성능을 약속하지 않는다.

## 검증 조건

- pinned Chromium fixture의 computed CSS 문자열을 정확 비교하고 각 node의 rectangle 좌표·크기별 최대 절대 오차를 `0.5 CSS px` 이내로 제한한다.
- Rust full-cascade oracle과 증분 결과의 NodeId별 style·진단·geometry가 같아야 한다. sibling branch의 값과 위치가 보존되어야 한다.
- fallback mutation에서는 결과뿐 아니라 `recomputed == 전체 연결 element 수`인지 확인한다.
- release benchmark는 16·256·2048 connected elements, 각 30회 mutation, 독립 프로세스 3회다. cascade·layout/scene projection·snapshot 등록부터 최종 완료까지 wall time의 p50/p95를 별도 기록한다.
- Android API 37 ARM64 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 동일 V8 fixture의 초기 full 계산, inline style mutation, partial cascade counters와 WGPU 화면을 확인한다. 실기기는 측정하지 않는다.

### 고정 구현 경계

- Author stylesheet가 하나라도 연결된 경우 selector dependency를 추적하지 않으므로 full fallback이다.
- UA stylesheet는 `spinon-ua-supported-elements-v0` 고정 profile이다. selector 입력이 현재 단순 요소 이름/namespace보다 넓어지면 invariant test와 계약을 먼저 갱신해야 하며, 검증을 통과하지 못하면 full fallback한다.
- 선택 요소 외 computed property snapshot, `getComputedStyle`, CSSOM, media 변경, selector matcher 캐시는 범위 밖이다.
- Android API 37 emulator의 OpenGL ES/ANGLE SwiftShader 및 iOS Simulator의 Metal은 기능 시연 환경이다. release benchmark는 Mac Studio host의 Rust worker만 측정하며 모바일 성능으로 일반화하지 않는다.
