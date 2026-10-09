# C05.2 · Runtime CSS `@property` 등록

- **문서 유형:** 구현 계획 · 공식 상태는 [`spec/STATUS.md`](../spec/STATUS.md)에서 관리
- **상위 항목:** [C05 사용자 지정 속성과 재계산](../spec/STATUS.md#css-구현-체크리스트)
- **선행 조건:** C04.11 연결 HTML `<style>` 수집, C05.1 runtime `--*`·`var()` cascade, Stylo `0.22.0`
- **내부 계약:** `0.1.0` 고정 · 출시/호환성 정책 변경 전 숫자 버전 상승 금지
- **구현 전 Chromium 기준:** [C05.2 precomparison](../spec/internal/evidence/c05-runtime-registered-properties-precomparison-2026-10-10.md)
- **구현 검토:** 계획 검토와 다른 관점의 구현 실패 검토를 별도 문서로 기록

## 목표와 완료 경계

C04.11은 연결된 HTML `<style>` 본문을 Stylo author cascade에 전달하지만 `@property` at-rule은 runtime profile에서 거부한다. C05.2는 새 내부 runtime profile에서 Stylo 0.22.0의 stylesheet `@property` 등록을 사용해 등록된 사용자 지정 속성의 문법, `inherits`, `initial-value`, 대체 결과를 기존 제한 Flex layout과 GPU paint 입력에 연결한다.

C05.2는 기존 C04.11/C05.1 profile의 지원 범위를 바꾸지 않는다. `@property`는 C05.2 전용 profile에서만 허용한다. 스타일시트 원문 parser나 등록부를 별도로 구현하지 않고 Stylo가 파싱·검증·등록·계산한다. Spinon은 허용할 일반 layout/paint 속성을 기존 allowlist로 계속 제한한다.

완료는 고정 Chromium reference와 Rust/runtime 결과가 일치하고, Android API 37 emulator와 iOS 26.2 Simulator의 실제 V8 → HostDocument → Stylo → Taffy → WGPU 경로에서 등록 결과를 표시하며, 오류·stylesheet 수명 경계가 검증된 것을 뜻한다. 이는 전체 CSS, 공개 `CSS.registerProperty`, CSSOM 또는 C05 전체 완료를 뜻하지 않는다.

## 입력과 계산 계약

- 등록 입력은 C04.11과 같은 현재 HostDocument snapshot에 연결된 HTML `<style>`의 최상위 `@property` 규칙이다. 등록 source는 DOM stylesheet 순서로 Stylo에 전달한다.
- at-rule의 이름, descriptor 문법, `<length>`·`<color>` 같은 syntax grammar, 초기값의 유효성, typed substitution 및 등록 값 충돌은 Stylo `0.22.0`의 parser·`Stylist`에 위임한다. 자체 CSS parser나 값 검증기를 추가하지 않는다.
- CSS stylesheet 순서와 중복된 등록의 결과는 pinned Chromium `154.0.8037.98`에서 고정 fixture로 관찰한다. plan의 초기 fixture는 `<length>`의 `inherits:false`·`true`, 미지정 초기값, 타입에 맞지 않는 선언, 규칙보다 앞/뒤에서 참조, 중복 이름의 두 등록을 포함한다.
- 같은 이름을 여러 `<style>` source가 등록할 때 등록 승자는 stylesheet DOM 순서와 이동 뒤 순서로 비교한다. 서로 다른 HostRoot가 있더라도 문서의 연결 stylesheet 등록이 각 fragment의 계산에 일관되게 적용되는지 확인한다.
- CSS Properties and Values 문법에서 정의되지 않은 descriptor는 Stylo `0.22.0`이 무시하고 valid registration을 유지하는 표준 동작을 보존한다. Stylo 진단 중 `Unsupported @property descriptor declaration:`만 non-fatal로 취급한다. `syntax`, `inherits`, `initial-value` 오류와 모든 다른 stylesheet diagnostic은 기존처럼 전체 요청 실패다.
- 등록된 속성은 C05.1과 같은 사용자 지정 속성 cascade에서 사용한다. 일반 declaration은 기존 layout allowlist와 GPU paint profile의 `background-color`만 허용한다. `@property` 등록이 일반 CSS 속성 allowlist를 넓히지 않는다.
- C05.2 전용 computed-style profile을 추가한다. 기존 C04.11/C05.1 profile 이름, JSON field, 오류 코드와 판정은 유지하고, 기존 profile에서는 `@property`가 계속 profile 밖 오류여야 한다.
- 신규 CSSOM이나 V8 `CSS.registerProperty()` 호출은 추가하지 않는다. 입력은 연결된 HTML `<style>`뿐이며 외부 `<link>`, CSS `@import` 로딩, `url()` 자원, `@font-face`, CSSOM은 계속 지원하지 않는다.

## 재계산·수명·오류

- 매 요청은 C04.11처럼 현재 immutable HostDocument snapshot에서 stylesheet 원문을 다시 수집하고 새 Stylo registry/Stylist를 구성한다. 등록·계산 사이에 `PropertyRegistration`, Stylo `Stylist`, `ComputedValues`를 별도 캐시에 보관하지 않는다.
- `<style>` 추가·텍스트 변경·이동·분리 뒤 새 `DocumentRevision`은 현재 연결 순서의 등록만 사용한다. 제거된 등록이나 오래된 계산 결과가 새 revision에 남지 않도록 확인한다.
- Stylo가 author stylesheet에 보고한 parse diagnostic은 layout-only와 GPU 양 경로에서 source ID·위치와 함께 전체 계산 실패다. 단, valid registration 안의 표준 unknown `@property` descriptor diagnostic만 무시한다. 잘못된/missing 필수 descriptor를 조용히 성공시키지 않는다.
- C05.2 profile은 최상위 `@property`만 추가 허용한다. `@import`, `@media`, `@supports`, `@layer`, nested style rule 및 다른 at-rule은 새 허용 범위로 확장하지 않는다.
- 지원하지 않는 일반 CSS 선언, 잘못된 root/viewport/revision, stale completion, 비유한 frame은 기존 fail-closed·latest-wins 계약을 유지한다. layout-only profile에서 `background-color`는 계속 거부한다.
- Android/iOS 실행 진입점은 검증 전용 내부 FFI fixture로 제한한다. 새 공개 Rust/C/JS API, ABI schema 변경, 사용자 설정 옵션을 추가하지 않는다.

## Chromium oracle와 판정 기준

기준은 저장소가 이미 고정한 Chrome `154.0.8037.98` (`@b859317bf11f6be47f9b7799ec690a0a42a1fb33`)이다. viewport는 `301×100` CSS px, device scale factor 1, locale `en-US`, timezone `UTC`, light color scheme, coarse pointer, hover none으로 고정한다. 새 fixture·inventory·capture tool·reference JSON은 기존 C04/C05 파일을 덮어쓰지 않는다.

- 등록 사용자 지정 속성의 `getComputedStyle()` 직렬화는 고정 reference와 정확히 같아야 한다.
- 선택한 scene nodes의 `x`, `y`, `width`, `height` 각 절대 오차는 `0.5 CSS px` 이하여야 하며 평균으로 개별 실패를 가리지 않는다.
- `inherits:false`는 조상 선언을 자식이 받지 않고 등록 `initial-value`를 계산해야 한다. `inherits:true`는 상속된 지정 값과 미지정 초기값을 구별해야 한다.
- syntax에 맞지 않는 값, 중복 등록의 승자, 뒤에 선언된 등록, 초기값 및 색상 paint를 각각 oracle로 고정한다.
- valid unknown descriptor가 포함되어도 등록된 선언·계산 결과는 unknown descriptor 없는 경우와 같아야 한다. 반대로 필수 descriptor 누락이나 잘못된 syntax·initial value는 요청 전체를 실패시킨다.
- `.style` inline `!important`와 등록 사용자 지정 속성의 interaction은 기존 cascade 우선순위를 유지해야 하며, 새 profile로 인해 일반 property allowlist 밖 선언을 수용하지 않는다.
- C04.11/C05.1 기존 profile이 `@property`를 거부하고 신규 C05.2 profile만 받아들이는 경계 테스트를 둔다.
- 등록 CSS 수정·이동·분리·재연결 뒤 현재 `DocumentRevision`의 계산 결과만 사용하며 제거된 등록을 재사용하지 않는다.

## 작업 순서

1. 계획을 별도 20개 실패 관점으로 검토하고, 선행 C04.11/C05.1 계약 및 Stylo `0.22.0` 소스 경계를 대조한다.
2. 계획 범위를 고정한 뒤 C05.2 전용 Chromium fixture·inventory·capture tool·precomparison reference를 생성하고, 별도 다중 HostRoot 보조 fixture로 문서 단위 registration scope를 확인한다.
3. Stylo `@property` 파서와 등록 동작을 이용하는 신규 layout/paint profile을 추가한다. 기존 profile에 at-rule을 허용하지 않는다.
4. Runtime layout/GPU 계산, 검증 전용 FFI fixture와 Android/iOS Simulator 화면을 연결한다.
5. computed value, geometry, unknown/invalid descriptor, 여러 stylesheet·HostRoot, inline priority, 기존 profile 회귀, 등록 변경·제거·stale 결과를 Rust fixture로 검증한다.
6. 전체 workspace tests, Clippy, FFI feature check, Android API 37 emulator와 iOS 26.2 Simulator 빌드·실행을 수행한다.
7. 실제 변경 코드·플랫폼 실행을 계획 검토와 다른 20개 실패 관점으로 검토하고, 상태 대장에는 C05.2만 완료 표시한다. C05 상위와 CSS 전체 지원은 미완료로 둔다.

## 미지원

`CSS.registerProperty()`, CSSOM stylesheet mutation/query, external CSS loader, `@import`, nested/conditional at-rules, stylesheet owner 변경 API, dirty-subtree 재계산, persistent style cache, 일반 CSS 선언 확장, 실기기 성능은 이 하위 작업 범위가 아니다.
