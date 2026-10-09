# C04.4 · CSS Cascade Layers

- **문서 유형:** 구현 계획과 비교 기준 · 구현 상태는 [상태 대장](../spec/STATUS.md)에서 확인
- **기준일:** 2026-10-09
- **상위 항목:** [구현 상태 대장 C04](../spec/STATUS.md#css-구현-체크리스트)
- **선행 내부 경로:** C04.3 `FlexAlignmentV1` Stylo cascade → Taffy adapter

## 목적과 범위

CSS `@layer`를 author stylesheet의 cascade에 적용해 같은 선언 집합의 계산값과 Flex geometry가 Chromium 동작을 따르게 한다. 기존 C04.3 profile과 고정 reference는 바꾸지 않는다. 새 `FlexAlignmentCascadeLayersV1` profile을 추가하고, 지원 CSS 선언 집합 안에서 named/anonymous layer block, layer order statement, nested layer 및 여러 author stylesheet 사이의 layer 순서를 처리한다.

허용된 선언은 현재 Flex alignment profile의 property allowlist를 따른다. 레이어 안에서도 미지원 property·custom property·중첩 style rule·미지원 at-rule은 전체 계산을 실패시킨다. `@import`, 조건부 `@layer` 등록, CSS nesting, DOM inline style, 스타일 무효화, 일반 앱 stylesheet 로딩, 공개 API, 제품 runtime·GPU·Android/iOS 표시는 범위 밖이다.

## 비교 모델과 통과 기준

- 규범 기준: [CSS Cascading and Inheritance Level 5 §6.4](https://www.w3.org/TR/css-cascade-5/#cascade-layers), 특히 cascade sorting 및 layer ordering.
- 실행 oracle: Chrome `154.0.8037.98`, Chromium revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`; C04.3과 같은 immutable Chromium capture helper와 고정 environment를 쓴다.
- 새 입력·HTML·reference 경로를 별도로 만든다. C04.3 reference JSON을 덮어쓰거나 이전 결과에 새 case를 덧붙이지 않는다.
- named, anonymous, nested layer와 order statement, 다중 author stylesheet를 fixture로 구분한다. normal/important·layered/unlayered·specificity·source order 조합별 computed `align-items` 또는 `justify-content` 문자열을 Chromium과 정확 비교한다.
- 각 case의 부모와 자식 frame을 모두 비교한다. 좌표·크기 각각의 최대 절대 오차가 `0.5 CSS px` 이하여야 한다. 평균값으로 실패를 가리지 않는다.
- 허용 property 외 선언, 알 수 없는 at-rule, parse diagnostic, 중첩 style selector, 기존 profile 오용은 부분 결과 없이 fail-closed여야 한다.

| fixture case | author CSS 구조 | 기대 winner |
|---|---|---|
| `normal-layer-order` | 선언 순서 `base`, `theme`; 두 layer에서 `justify-content` 충돌 | 나중 `theme` |
| `explicit-layer-order` | `@layer theme, base;` 뒤 block은 반대 순서 | 나중 순서로 명시한 `base` |
| `layer-reopen-order` | `base` block을 두 번 열고 사이에 `theme` 선언 | 기존 `base` 순서 유지, `theme`가 이김 |
| `cross-stylesheet-order` | 첫 stylesheet와 두 번째 stylesheet에서 같은 layer 이름 재사용 | layer registry의 첫 선언 순서와 두 번째 sheet의 source order 적용 |
| `same-layer-cross-stylesheet-order` | 두 author stylesheet가 같은 named layer에 같은 property를 선언 | 하나의 named layer로 합치고 두 번째 sheet 선언이 승리 |
| `unlayered-normal` | explicit layer와 unlayered normal 선언 충돌 | unlayered |
| `important-layer-reverse` | ordered layers에서 같은 property에 `!important` 충돌 | 먼저 정해진 layer |
| `important-unlayered` | explicit-layer와 unlayered `!important` 충돌 | explicit layer |
| `importance-before-layer` | higher normal layer와 lower `!important` 선언 충돌 | `!important` 선언 |
| `specificity-within-layer` | 한 layer 안 class selector와 ID selector 충돌 | ID selector |
| `layer-before-specificity` | 낮은 specificity의 후순위 layer와 높은 specificity의 선순위 layer 충돌 | 후순위 layer |
| `nested-parent-implicit-layer` | `framework.theme` 자식 선언과 `framework` 직접 선언 충돌 | normal cascade에서 parent의 implicit sublayer |
| `nested-sibling-order` | `framework.components, framework.theme`로 순서를 미리 선언한 뒤 block은 역순으로 등장 | 명시 순서에서 나중인 `framework.theme` |
| `anonymous-layer-identity` | 두 anonymous block이 같은 property를 선언 | 두 번째 anonymous layer |
| `multi-name-layer-statement` | `@layer reset, app, override;` | normal 기준 `override` |
| `same-layer-source-order` | 같은 layer·같은 selector의 두 선언 | 뒤 선언 |
| `unsupported-nested-rule` | 허용하지 않는 CSS nested selector가 layer 안에 있음 | 계산 실패 |

각 성공 case는 정확 computed string과 4개 frame을 기록한다. reject case는 Chromium geometry와 비교하지 않고 오류 종류와 partial output 부재를 확인한다.

## 구현 순서와 완료 조건

1. plan 검토 20개 관점의 실패 경로를 별도로 검토한다.
2. 고정 Chromium capture와 immutable reference를 만든다. 기존 파일이 있으면 절대 덮어쓰지 않는다.
3. Stylo `0.22.0`의 `CssRule::LayerBlock`·`LayerStatement`를 재귀 순회하되, layer 아래에서도 declaration allowlist를 강제한다.
4. 새 `FlexAlignmentCascadeLayersV1` computed-style profile과 style-to-layout entrypoint를 추가한다. C04.3/C04.2 결과·whitelist는 보존한다.
5. 정해진 layer cascade 조합, 지원 밖 구문·선언, multi-sheet order와 revision 경계를 Chromium·단위 fixture로 검증한다.
6. 버전 있는 내부 계약, 실행 근거, C04 하위 상태와 CSS 계획 요약을 같은 변경에 갱신한다. C04 전체·제품 CSS·runtime·GPU 지원은 완료 처리하지 않는다.

## 계획 적대 검토 · 20개 독립 실패 관점

| # | 실패 관점 | 계획에 반영한 방지 기준 |
|---:|---|---|
| 1 | author layer 처리가 UA/author origin 우선순위를 바꿈 | layer는 현재 author origin 안에서만 검증하고 기존 UA 규칙과 cascade 비교를 유지한다. |
| 2 | normal layer 우선순위가 선언 순서와 반대로 계산됨 | 먼저 선언된 두 named layer의 normal 충돌을 각 순서로 비교한다. |
| 3 | `!important`에서 layer 순서를 뒤집지 않음 | 동일 두 layer에 important 충돌을 두고 첫 layer 우선 결과를 확인한다. |
| 4 | unlayered normal 규칙을 명시 layer보다 낮게 처리 | layered normal과 unlayered normal을 같은 property에서 충돌시킨다. |
| 5 | unlayered important 규칙을 layered important보다 높게 처리 | important 선언의 explicit/unlayered 순서를 별도 case로 고정한다. |
| 6 | `@layer first, second`의 사전 선언 순서를 무시 | statement만으로 정한 순서를 뒤의 block 선언과 대조한다. |
| 7 | 기존 layer를 재선언할 때 처음 등록된 순서를 바꿈 | layer 재개방 전후의 winner와 computed string을 대조한다. |
| 8 | stylesheet별 layer tree를 분리해 전역 author order가 깨짐 | 같은 이름을 서로 다른 author stylesheet에서 선언한다. |
| 9 | nested layer를 parent layer와 같은 우선순위로 평탄화 | parent 직접 선언과 nested child 선언 충돌을 따로 둔다. |
| 10 | 익명 layer를 이름이 같은 단일 layer로 합침 | 인접한 anonymous block 두 개의 order를 독립 관찰한다. |
| 11 | `@layer` 문장 안 다중 이름의 등록 순서가 소실 | 다중 이름 statement를 단일 이름 statement의 역순과 대조한다. |
| 12 | layer 우선순위가 selector specificity보다 뒤에서 적용됨 | 낮은 specificity의 높은 priority layer와 높은 specificity의 낮은 priority layer를 비교한다. |
| 13 | 같은 layer 안 specificity가 무시됨 | 동일 layer 안에서 class/ID selector 충돌을 비교한다. |
| 14 | 같은 layer·specificity에서 stylesheet source order가 무시됨 | 두 stylesheet의 같은 layer·selector 충돌을 입력 순서대로 비교한다. |
| 15 | normal과 important를 함께 둘 때 importance 단계가 layer보다 뒤섞임 | higher-priority layer의 normal과 lower-priority layer의 important를 대조한다. |
| 16 | `@supports`·`@media`·`@import` 등 미지원 at-rule을 layer 안에서 허용하거나 외부 자원을 요청 | layer AST 재귀 검사와 import-disabled parser가 전체 계산을 실패시킨다. |
| 17 | layer 안 미지원 property가 allowlist 검사를 우회 | 일반 rule과 동일한 property allowlist를 모든 layer depth에 적용한다. |
| 18 | CSS nesting의 style rule 자식을 일반 layer 규칙으로 오인 | 중첩 style rule을 별도 fail-closed 입력으로 둔다. |
| 19 | 잘못된 layer 구문 진단 뒤 일부 element snapshot이 반환됨 | parse diagnostic이 있으면 계산 결과 전체를 거부한다. |
| 20 | 새 profile 추가로 C04.3 또는 C04.2 동작이 바뀌거나 제품 완료로 과장 | 이전 profile/reference 회귀를 실행하고 상태·계약에 fixture 한계를 명시한다. |

이 표는 계획 단계 검토다. 구현 뒤에는 다른 실패 관점 20개를 별도로 기록한다.
