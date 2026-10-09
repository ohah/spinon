# 0035 · C05.3 연결 장면 불변 runtime 결과 재사용

- **내부 계약:** `0.1.0` 고정 · 첫 공식 릴리스 또는 호환성 정책 결정 전 숫자 변경 금지
- **상위 상태:** [C05 사용자 지정 속성과 재계산](../STATUS.md#css-구현-체크리스트)
- **작업 계획:** [C05.3 구현 계획](../../plan/c05-runtime-result-cache.md)
- **구현 전 비교:** [detached-only Chromium 기준](evidence/c05-runtime-result-cache-precomparison-2026-10-10.md)
- **계획 검토:** [실패 관점 검토](evidence/c05-runtime-result-cache-plan-review-2026-10-10.md)
- **구현 검토·실행 근거:** [적대적 구현 검토·release 측정·시뮬레이터 결과](evidence/c05-runtime-result-cache-implementation-review-2026-10-10.md)

## 범위와 목적

C05.3은 CSS cascade/layout worker 한 개 안에서 직전 성공 장면 하나를 유지한다. 같은 문서 세대·연결 RenderTreeRevision·StyleRevision·EnvironmentRevision과 viewport/media 입력에 대해 `DocumentRevision`만 바뀐 요청은 이전 계산 결과를 재사용한다. 이 단계는 detached-only 변경의 중복 계산을 제거한다. 연결 요소의 부분 계산이나 selector dependency 추적은 구현하지 않는다.

내부 계약·Rust crate의 버전은 `0.1.0`으로 유지한다. C05.3 구현은 출시가 아니며 semver 증가 사유가 아니다. `cacheHit`은 개발·검증 상태를 관찰하는 내부 진단값이고 공개 JavaScript API가 아니다.

## 캐시 소유와 입력 key

- 캐시는 계산 profile이 고정된 CSS worker의 지역 변수로 소유하며 profile/coordinator 간 공유하지 않는다. 수용하는 entry는 최근 성공 결과 하나다. worker 종료 때 entry는 worker와 함께 drop한다.
- semantic key에는 `DocumentGeneration`, `RenderTreeRevision`, `StyleRevision`, `EnvironmentRevision`, viewport width·height·device scale의 IEEE-754 `to_bits()` 값, color scheme·primary pointer·primary hover·all pointer capabilities를 넣는다.
- `DocumentRevision`은 semantic key에서 생략한다. 동일 `DocumentGeneration`이어도 연결 내용이 바뀌면 core가 발급하는 `RenderTreeRevision`이 달라져 miss가 된다.
- 새 request key가 기존 entry와 다르면 계산 전에 이전 entry를 지운다. cascade 오류, panic, layout 오류 결과는 저장하지 않는다.
- 계산 결과가 만들어지는 사이 최신 key가 바뀌면 stale 계산은 발행하거나 캐시에 저장하지 않는다. 기존 latest-wins full-key 검사를 유지한다.

## hit 시 재발행과 공유 저장

cache hit은 이전 계산 결과의 의미상 출력이 동일한 경우에만 허용한다. 새 request의 `DocumentRevision`에 맞게 아래 표식을 전부 다시 묶는다.

- `RuntimeUaCascadeCompleted.key`
- 모든 `ComputedStyleSnapshot.document_revision`
- `RuntimeLayoutCompleted.key`
- `RuntimeRenderKey.document_revision`

generation·render-tree·style·environment revision은 semantic key와 같아야 하며 그대로 유지한다. 스타일 요소 목록, layout frame, render box 목록은 `Arc<[T]>` 불변 backing storage를 공유한다. rekey는 계산 출력의 CSS 값·frame·paint를 변경하지 않는다.

`RuntimeRenderSnapshot::with_document_revision`은 기존 snapshot의 generation·render-tree·style·environment key, viewport와 box 배열을 유지하고 document revision만 새 값으로 교체한다. 사용자가 임의의 revision tuple을 만들어 장면을 검증 없이 승인하는 함수는 제공하지 않는다.

## 진단 JSON

기존 `spinon.runtime.ua-cascade.v1` 및 `spinon.runtime.layout` JSON 객체에 완료 결과의 `cacheHit: boolean`을 추가한다.

- 실제 계산: `cacheHit`은 `false`; 실제 실행하지 않은 cascade/projection 단계의 시간은 0이다.
- cache hit: 양쪽 완료 결과에서 `cacheHit`은 `true`; cascade 및 projection duration은 0이다.
- 실패·pending·not-configured snapshot은 완료 객체가 없으며 cache hit 표시도 없다.

duration은 해당 계산 단계의 누적 수행 시간이지 요청 등록부터 완료까지의 wall-clock latency가 아니다. `cacheHit`과 0 duration만으로 사용자 체감 속도나 특정 비율의 향상을 주장하지 않는다. JSON schema의 기존 이름은 유지하며 내부 진단 필드 하나를 추가한다.

## 무효화 조건

| 변경 | 결과 |
| --- | --- |
| 같은 snapshot 재등록 | hit |
| detached node의 attribute·style·text 변경, 연결 projection 불변 | hit 및 document revision 재발행 |
| connected node 값·구조·순서, `<style>` 내용·순서 변경 | miss 후 전체 계산 |
| viewport·device scale·media 입력 변경 | miss 후 전체 계산 |
| DocumentGeneration 변경 | miss 후 전체 계산 |
| cascade/layout 실패, worker panic, stale completion | 저장 금지 |

`DocumentRevision`만으로 hit을 결정하지 않는다. 모든 무효화 입력은 worker-local key를 비교해야 한다.

## 검증 경계

Rust tests는 cache key, `DocumentRevision` 재발행, immutable payload 공유, 연결 attach/detach, viewport/media/generation 변경, 오류 제외와 latest-wins 경합을 검증한다. 고정 Chromium fixture는 detached node의 `isConnected=false`, 빈 rectangle, 연결 노드 computed style·rectangle 불변을 비교한다. Android API 37 emulator와 iOS 26.2 Simulator의 실제 V8 경로에서는 detached-only `false → true`, attach `→ false`, detach `→ false`, 분리 상태 수정 `→ true` 전이를 확인한다.

Release 반복 측정은 16·256·2048 connected node에서 각 30쌍을 세 독립 프로세스로 실행해 miss와 detached-only hit의 worker 완료 wall-clock p50/p95 및 Arc 공유 여부를 기록한다. `/usr/bin/time -l` 최대 RSS는 test binary 기준선을 포함한다고 명시한다. 이 결과는 해당 Mac host의 측정이며 Android/iOS GPU 또는 실기기 성능으로 일반화하지 않는다. C05 부모, dirty subtree, 전체 CSS 지원, 실기기 성능 검증은 이 계약의 범위가 아니다.
