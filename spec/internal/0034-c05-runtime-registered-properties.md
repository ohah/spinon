# 0034 · C05.2 Runtime CSS `@property` 등록

- **내부 계약:** `0.1.0` 고정 · 첫 공식 릴리스 또는 호환성 정책 결정 전 숫자 변경 금지
- **상위 상태:** [C05 사용자 지정 속성과 재계산](../STATUS.md#css-구현-체크리스트)
- **작업 계획:** [C05.2 구현 계획](../../plan/c05-runtime-registered-properties.md)
- **구현 전 비교 모델:** [Chromium precomparison](evidence/c05-runtime-registered-properties-precomparison-2026-10-10.md)
- **기준 reference:** [Chromium C05.2](../../tests/fixtures/css/references/c05-runtime-registered-properties-v1.json)
- **구현 검토·실행 근거:** [구현 검토와 Android·iOS Simulator](evidence/c05-runtime-registered-properties-implementation-review-2026-10-10.md)

## 범위

이 계약은 현재 HostDocument snapshot에 연결된 HTML `<style>` 요소에서 최상위 `@property` 규칙을 수집해 Stylo `0.22.0` stylesheet cascade에 등록하는 C05.2 내부 동작을 정의한다. registration의 `syntax`, `inherits`, `initial-value`, 타입 검증, 대체 및 동일 이름 충돌 결과는 Stylo에 위임한다. C05.2 고정 입력은 `<length>`, `<color>`, `<number>`와 unknown descriptor, stylesheet 순서 변경을 검증한다.

등록 값은 기존 제한 Flex layout profile과 GPU paint profile이 계산하는 속성에만 영향을 준다. layout profile의 일반 CSS 선언은 기존 allowlist를 유지하며, paint profile에는 기존 `background-color`만 추가 허용한다. C05.2는 사용자 지정 속성 자체를 JS API 또는 computed-style 결과 객체로 노출하지 않는다.

## 입력 및 수명

- 연결된 HTML `<style>`의 텍스트를 DOM 순서대로 처리한다. `<link>`, CSS `@import`, 외부 자원 및 CSSOM 변경은 입력에 포함하지 않는다.
- 연결 stylesheet 등록은 문서 전체 입력으로 수집해 각 HostRoot cascade fragment에 전달한다. 한 root의 `<style>`에 있는 등록은 다른 root의 fragment 계산에서도 보인다. 단, 현재 runtime layout은 여러 HostRoot 장면을 지원하지 않고 `multiple_host_roots`로 실패한다.
- 각 계산 요청은 현재 HostDocument snapshot의 stylesheet를 새 Stylo registry와 Stylist에 전달한다. 등록부나 계산 결과를 요청 사이 캐시에 보존하지 않는다.
- stylesheet 수정·이동·분리·재연결 뒤 새 `DocumentRevision`은 새 snapshot의 연결 순서만 사용한다. 이전 revision의 등록은 다음 계산에 재사용되지 않는다.
- Stylo 진단 `Unsupported @property descriptor declaration:`은 해당 optional/unknown descriptor만 무시한다. `syntax`, `inherits`, `initial-value` 오류 및 다른 stylesheet parse 진단은 전체 요청을 실패시킨다.

## profile과 호환 경계

- 신규 `RuntimeFlexRegisteredPropertiesV1`과 `RuntimeFlexRegisteredPropertiesPaintV1` profile에서만 stylesheet `@property` rule을 허용한다.
- C04.11/C05.1 profile은 계속 `@property`를 거부한다. 신규 profile은 `@media`, `@supports`, `@layer`, `@import`, 중첩 rule 또는 다른 at-rule을 함께 허용하지 않는다.
- `CSS.registerProperty()`, `getComputedStyle()`의 custom-property 조회, `CSSStyleSheet`, stylesheet rule 조회·수정 API를 구현하지 않는다.
- 구현 fixture용 FFI 함수는 플랫폼 검증 harness 전용이다. 사용자 C/JS API·패키지 API·ABI schema를 추가하지 않는다.
- Android API 37 emulator 및 iOS 26.2 Simulator의 V8 → HostDocument → Stylo → Taffy → WGPU 경로를 검증 대상에 포함한다. 실기기·전체 Chromium CSS 호환성은 이 계약의 완료 근거가 아니다.

## 검증 전용 C ABI

플랫폼 앱은 C05.2 검증 모드에서 전용 CSS profile host를 만들고 고정 fixture를 재실행한다. 일반 C04.10/C04.11/C05.1 host에 등록 profile을 적용하지 않는다.

```c
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_registered_properties_fixture(
    char *output,
    size_t output_capacity);

int32_t spinon_runtime_gpu_host_eval_registered_properties_fixture(
    SpinonRuntimeGpuHost *host,
    uint64_t layout_timeout_millis,
    char *output,
    size_t output_capacity);
```

- 생성 함수는 V8·CSS worker 준비를 기다린 뒤 host를 반환한다. 평가 함수도 JavaScript와 CSS 결과 대기 때문에 호출자를 막는다. 두 함수 모두 기다림이 허용된 platform runtime background executor에서 호출하며 UI/main thread에서는 호출하지 않는다.
- 평가 함수의 `host`는 `spinon_runtime_gpu_host_new_registered_properties_fixture`가 돌려준 살아 있는 host여야 한다. 일반 `spinon_runtime_gpu_host_new` host는 등록 `@property`를 지원하지 않는다. 같은 host의 다른 호출을 직렬화하고 해제와 경합시키지 않는다.
- `layout_timeout_millis`는 평가 함수에서 0보다 커야 하며 JavaScript 실행 제한이 아니라 평가 뒤 Stylo·Taffy 완료 대기 제한이다. JavaScript 실행 자체는 이 함수에서 취소하지 않는다.
- 두 함수의 `output`은 `output_capacity` 바이트만큼 쓸 수 있어야 한다. 성공·오류 보고는 UTF-8 NUL 종료 문자열이다. 생성 함수는 실패 시 NULL을 반환하고 오류 보고를 쓴다. 평가 함수는 버퍼 부족 시 `-3`을 반환하고 가능한 경우 첫 바이트를 NUL로 만든다.
- 평가 함수의 `0`은 fixture 평가·계산·장면 발행 성공이다. 잘못된 인자 `-1`, 대기 제한 시간 초과 `-10`, layout/render 장면 오류 `-11`, stale scene 거부 `-12`를 반환할 수 있다. 내부 V8 평가 상태 코드는 C04.10 [`0031 상태 코드 계약`](0031-c04-runtime-css-to-gpu.md#상태-코드와-보고)을 따른다.
- 두 함수 모두 저장소 내 고정 검증 모드용이다. 평가 함수는 저장소 fixture만 실행하고 사용자 임의 코드를 받지 않는다. 이들은 사용자 C/JS API나 ABI schema의 일부가 아니다.

## 판정 입력

기준 브라우저는 Chrome `154.0.8037.98` revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`이다. Chromium fixture의 computed value 문자열은 정확 일치, 각 노드 좌표·크기는 축별 최대 절대 오차 `0.5 CSS px`를 적용한다. flex 아이템의 `width:auto`는 `ComputedStyleSnapshot`의 계산 값과 Chromium used value를 혼동하지 않고, 고정 reference의 실제 frame geometry로 비교한다.

C05.2 구현 완료는 [상태 대장](../STATUS.md)에 연결한 Rust 테스트, 실제 V8 플랫폼 fixture, Chromium reference 및 implementation review가 모두 기록된 경우에만 표시한다. 이는 C05 부모 또는 CSS 전체 범위의 완료를 뜻하지 않는다.
