# C05.2 `@property` 구현 계획 검토

- **대상:** [`plan/c05-runtime-registered-properties.md`](../../../plan/c05-runtime-registered-properties.md)
- **구분:** 기능 코드를 대상으로 한 구현 검토와 별개인 계획 검토
- **결론:** unknown descriptor의 표준 무시 동작을 기존 fail-closed 규칙과 구별하고, source order·HostRoot·profile 경계를 완료 조건에 추가했다.

| # | 실패 관점 | 계획에 반영한 경계·수정 |
|---:|---|---|
| 1 | C05.2 하나가 C05 전체 또는 CSS 지원 완료로 표시되는가 | C05.2 하위 항목만 완료 대상으로 두고 C05 부모와 전체 CSS는 미완료로 남긴다. |
| 2 | 새 at-rule 허용이 C04.11/C05.1 기존 profile로 새어 가는가 | 전용 computed-style profile을 추가하고 기존 profile에서 유효한 `@property`가 계속 거부되는 회귀 조건을 둔다. |
| 3 | 사용자 코드에서 전역 `CSS.registerProperty()`가 가능한 것으로 오해되는가 | 입력을 현재 snapshot의 연결된 HTML `<style>`로만 한정하고 DOM/CSSOM API를 추가하지 않는다. |
| 4 | CSS Properties and Values 문법을 자체 parser로 잘못 재현하는가 | Stylo `0.22.0` parser와 `Stylist` 등록 구현을 사용한다. Spinon은 일반 layout/paint allowlist만 관리한다. |
| 5 | 필수 `syntax`가 없거나 invalid grammar가 조용히 무시되는가 | `syntax` parse/필수 descriptor diagnostic은 계산 전체 실패로 처리한다. |
| 6 | `inherits`가 누락되거나 boolean이 아닌 경우 기본값으로 성공하는가 | 필수 descriptor 오류를 진단으로 보존하고 두 runtime 계산이 fail-closed 하도록 했다. |
| 7 | `initial-value`가 등록 syntax와 맞지 않거나 computationally dependent한가 | Stylo 검증을 사용하고 invalid initial value diagnostic은 전체 요청 실패 조건으로 정했다. |
| 8 | `inherits:false`가 조상 값을 자식에 전파하는가 | Chrome reference에서 조상 지정값과 자식 등록 초기값을 각각 비교한다. |
| 9 | `inherits:true`가 상속값과 초기값을 혼동하는가 | 조상에서 상속된 값과 값이 없는 별도 노드의 initial-value를 함께 고정한다. |
| 10 | 등록된 property에 syntax 불일치 값을 지정했을 때 미등록 token이 used value로 새는가 | typed declaration의 computed custom-property와 최종 width를 함께 대조해 등록 initial-value 전이를 확인한다. |
| 11 | style rule보다 뒤에 선언된 등록이 앞선 사용에 적용되지 않는가 | 같은 stylesheet에서 사용 규칙 뒤에 `@property`를 선언하는 fixture를 포함한다. |
| 12 | 같은 이름 등록 두 개의 승자 순서가 node ID나 등록 코드 순서로 뒤바뀌는가 | Chrome 기준은 두 번째 등록이 이기는 사례를 포함하고, 여러 `<style>` source와 DOM 이동 순서도 확인한다. |
| 13 | 서로 다른 stylesheet source 사이에서 등록 충돌 순서가 달라지는가 | source order를 현재 connected DOM order로 고정하고 두 source 및 이동 뒤를 Chromium과 비교한다. |
| 14 | 잘못된 등록과 표준상 무시해야 하는 unknown descriptor를 같은 오류로 취급하는가 | Stylo가 보고하는 `Unsupported @property descriptor declaration:`만 무시하고 valid registration을 유지한다. 그 밖의 diagnostics는 실패시킨다. |
| 15 | at-rule allowlist가 `@media`, `@import`, `@layer` 등으로 확대되는가 | 최상위 `@property`만 허용하며 다른 at-rule·nested rule은 기존 profile 경계로 거부한다. |
| 16 | registration이 다른 HostRoot fragment에 적용되지 않거나 root 간 selector 의미까지 과장되는가 | stylesheet registry를 각 HostRoot cascade에 전달하되 HostRoot 간 selector 관계는 보장하지 않고 계산 결과를 분리 검증한다. |
| 17 | layout profile이 `background-color`나 임의 property를 통해 paint 입력을 받는가 | layout·paint profile을 분리하고 기존 일반 속성 allowlist를 변경하지 않는다. |
| 18 | inline style, `!important`, author stylesheet의 기존 cascade 우선순위가 바뀌는가 | 기존 C04.11/C05.1 cascade fixture를 유지하고 등록 custom property와 inline 중요 선언의 상호작용을 추가한다. |
| 19 | style 변경·분리 뒤 이전 registration이나 오래된 worker 결과가 재사용되는가 | 요청마다 snapshot에서 새 registry를 만들고 편집·이동·detach·재연결과 latest-wins 결과를 검사한다. |
| 20 | 미출시 계약 버전이나 내부 FFI fixture가 새 공개 API로 오해되는가 | 내부 contract `0.1.0` 고정, 검증 전용 FFI, 공개 API 없음, C05 부모 미완료를 plan/contract에 명시한다. |

## 계획 보정

계획 초안은 기존 author stylesheet diagnostic fail-closed 동작을 그대로 적용해 unknown `@property` descriptor도 요청 실패로 만들 수 있었다. Stylo `0.22.0` 소스의 descriptor parser는 알 수 없는 descriptor를 무시하고 registration은 유지하는 경로를 제공한다. 그 경로를 통해 표준 CSS 동작을 보존하도록 계획을 보정했다. `syntax`, `inherits`, `initial-value` 오류와 그 외 diagnostics는 여전히 전체 요청 실패다.
