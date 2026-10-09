# C05.1 · 런타임 inline 사용자 지정 속성

- **문서 유형:** 구현 계획 · 공식 구현 상태는 [`spec/STATUS.md`](../spec/STATUS.md)에서 관리
- **상위 항목:** [C05 사용자 지정 속성과 재계산](../spec/STATUS.md#css-구현-체크리스트)
- **선행 조건:** C04.10 Runtime CSS→GPU 장면, HostDocument revision 제출, 고정 Stylo `0.22.0`
- **내부 계약 버전:** `0.1.0-draft` 고정 · 계획이나 기능 추가만으로 숫자 버전을 올리지 않음
- **계획 검토 근거:** [실패 경로 검토](../spec/internal/evidence/c05-runtime-custom-properties-plan-review-2026-10-10.md)

## 목표와 완료 경계

현재 런타임은 DOM `style` 속성의 CSS cascade를 매 `DocumentRevision`마다 다시 계산하지만, layout 입력 검증기가 사용자 지정 속성(`--name`)을 허용하지 않는다. C05.1은 기존 Stylo CSS 계산을 사용해 사용자 지정 속성의 기본 상속, `var()` 대체와 fallback, 순환 참조 및 잘못 대체된 값의 계산 결과를 기존 런타임 Flex layout/paint 값에 연결한다. 사용자 지정 속성 변경 뒤 전체 연결 fragment를 다시 계산해 새 문서 revision의 결과만 게시한다.

C05 부모 항목은 이 하위 작업만으로 완료 처리하지 않는다. 런타임에는 author stylesheet 소유자·CSS 자원 로더가 아직 없으므로 `@property` 등록은 범위 밖이다. dirty-subtree 증분 계산도 넣지 않는다. 이 단계는 correctness-first 전체 연결 fragment 재계산을 유지하고 계산 사이에 cascade/style cache를 보존하지 않는다. 따라서 성능 향상, 전체 CSS 지원, CSSOM 또는 공개 사용자 API를 주장하지 않는다.

## 입력 계약

- 입력은 연결된 HostDocument 요소의 기존 `style` 속성이다. 별도 Rust 변수 parser나 자체 CSS token substitution을 만들지 않고, 잠긴 Stylo 버전의 CSS parser·cascade·custom-property substitution을 사용한다.
- `--*` 선언은 기존 inline style 파서가 유효한 사용자 지정 속성으로 보존한 경우만 허용한다. 허용된 기존 Flex layout 속성과 `background-color`에서 `var()`를 사용할 수 있다. 그 외 일반 CSS 속성은 현재와 같이 거부한다.
- 미등록 사용자 지정 속성은 CSS 규칙대로 상속한다. 이름은 대소문자를 구분한다. `var()`의 중첩 fallback·쉼표·빈 값·`!important`·self/mutual cycle을 fixture에 포함한다. cycle 또는 fallback 없는 미정의 값의 대체 결과는 Stylo의 계산 규칙을 따른다.
- CSSOM 변경 함수가 아니라 기존 `setAttribute("style", ...)`가 입력 경로다. 앱에서 선언을 변경하면 HostDocument `DocumentRevision`이 바뀐다. JS task가 성공하거나 예외를 내는지와 무관하게 revision 제출은 기존 runtime 규칙을 따른다.
- 재계산은 각 현재 HostRoot fragment의 연결 요소 subtree 전체를 처리한다. HostRoot 여러 개·직속 텍스트 등 기존 C04.8/C04.9 오류 경계는 유지한다. detached 노드 변경은 현재 결과에 노드를 추가하지 않으며, 나중에 연결되면 새 snapshot에서 새 부모의 상속을 계산한다.
- 이전 request와 다른 revision key의 계산 결과는 기존 latest-wins 규칙대로 게시하지 않는다. 계산 요청 간 Stylo `Stylist`, `ComputedValues`, node handle을 보관하는 사용자 지정 캐시는 추가하지 않는다.

## 비교 모델과 판정 기준

기준은 저장소 CSS fixture와 함께 고정한 Chromium `154.0.8037.98`이다. 같은 viewport·HTML 트리·inline style을 Chromium과 Stylo에 입력하고, Chromium `getComputedStyle()`의 CSS 문자열과 `getBoundingClientRect()` 값을 비교한다. 캡처 결과는 fixture·HTML·캡처 도구·Chromium 실행 파일 해시로 식별하고 기존 reference를 덮어쓰지 않는다.

- 기존 whitelist 대상의 계산 CSS 문자열은 Chromium과 정확히 같아야 한다.
- gap 렌더 fixture의 부모와 두 자식 `x`, `y`, `width`, `height` 최대 절대 오차는 각각 `0.5 CSS px` 이하여야 한다. 평균 오차로 단일 node 실패를 가리지 않는다. 나머지 reference rectangle은 보관하되 이 단계에서 layout 대조를 주장하지 않는다.
- parent custom property 수정 또는 child 이동·detach·재삽입 후 새 `DocumentRevision`의 대상 computed width와 Taffy frame width가 fixture 기대값과 일치해야 한다. 이전 revision의 결과가 최신 완료 결과로 남으면 실패다.
- custom property를 통한 입력 외 일반 미지원 property, 잘못된 root, viewport/revision mismatch, 비유한 frame은 성공으로 바뀌지 않아야 한다.

## 작업 단계

1. C05.1 전용 computed-style profile을 추가하고 기존 C04.9/C04.10 profile과 JSON field·오류 코드를 보존한다.
2. custom property 선언만 inline allowlist에 추가한다. Stylo가 계산한 허용 layout/paint longhand만 Taffy와 renderer에 전달한다.
3. 고정 fixture에 inheritance, case sensitivity, direct/nested fallback, empty value, `!important`, self/mutual cycle, invalid substitution, ancestor value update와 다른 부모로의 이동을 넣는다.
4. runtime worker에 서로 다른 HostDocument snapshot을 순서대로 제출해 계산 결과가 최신 revision을 반영하는지 확인한다. 제거되거나 detached인 node가 이전 fragment 결과에 남지 않는지 확인한다.
5. Chromium reference 캡처와 고정 fixture 검증기를 추가하고, style/runtime/ffi 및 Android·iOS Simulator 경로의 영향받는 검사를 실행한다. 실기기 측정은 이 작업의 완료 조건으로 두지 않는다.
6. C05.1만 완료 표시하고 `@property`, author stylesheet 전달, incremental dirty-subtree, 영속 style cache는 C05 후속으로 남긴다. C05 부모·U02·전체 CSS를 완료 처리하지 않는다.

## 계획 공격 검토에서 반영한 수정

| 관점 | 발견 가능성이 있는 실패 | 최종 계획에 반영한 조치 |
|---:|---|---|
| 1 | 하위 작업 하나를 C05 전체 완료로 과장 | C05.1만 체크하고 부모는 미완료로 유지 |
| 2 | stylesheet가 없는 runtime에서 `@property`를 지원한다고 오해 | stylesheet 소유자 선행 조건을 명시하고 `@property`를 제외 |
| 3 | CSSOM 지원을 약속하는 것으로 오해 | 기존 `setAttribute("style", ...)`만 입력으로 제한 |
| 4 | CSS 변수 문법을 별도 parser로 중복 구현 | Stylo `0.22.0` parser와 cascade에 위임 |
| 5 | 사용자 지정 속성 이름의 case folding | 대소문자 구분 fixture 추가 |
| 6 | 미등록 속성이 상속되지 않음 | parent→child 상속 fixture 추가 |
| 7 | 빈 사용자 지정 속성 값과 미정의 값을 혼동 | 빈 값 fixture로 fallback 미사용 확인 |
| 8 | 중첩 fallback 안 쉼표를 최상위 구분자로 오인 | 중첩 `var()` fallback을 Chromium oracle에 포함 |
| 9 | self-cycle이 정상 값으로 노출 | self-cycle과 fallback 결과를 확인 |
| 10 | multi-node cycle에서 일부 값만 남음 | 상호 cycle 전체의 invalid-at-computed-value 결과를 확인 |
| 11 | 미정의 `var()`가 fallback 없이 선언 시점에 잘못 처리 | 계산 시점의 property 초기값 동작을 확인 |
| 12 | 대체 후 타입이 틀린 값이 기존 used value를 유지 | 유효 CSS 값으로 대체되지 않은 non-inherited longhand의 초기값을 확인 |
| 13 | CSS shorthand 값 속의 `var()`만 검증 누락 | `gap: var(--space)`처럼 기존 shorthand allowlist를 통과하는 경로 포함 |
| 14 | `!important` custom declaration 우선순위 무시 | 중복 custom declaration의 우선순위 fixture 포함 |
| 15 | 사용자 지정 속성 추가로 모든 미지원 CSS 속성이 허용 | 기존 일반 property allowlist는 그대로 제한 |
| 16 | detached node가 결과 tree에 남음 | 현재 연결 fragment만 결과 membership으로 사용 |
| 17 | 다른 parent로 이동한 node가 이전 custom value를 재사용 | 새 snapshot·새 Stylo view에서 재상속 결과 확인 |
| 18 | worker가 낡은 DocumentRevision 결과를 게시 | 기존 전체 revision key와 latest-wins gate 유지·검증 |
| 19 | 매 request 전체 cascade를 증분 계산으로 잘못 보고 | 전체 연결 fragment 재계산을 명시하고 성능 최적화 주장 금지 |
| 20 | 버전·범위 표기가 출시 지원으로 오해 | 계약 `0.1.0-draft` 유지, C05.1 내부 범위 및 C05 후속 항목을 분리 |

이 표의 세부 실행 조건과 보정 결과는 별도 [계획 검토 근거](../spec/internal/evidence/c05-runtime-custom-properties-plan-review-2026-10-10.md)에 기록한다. 구현 뒤에는 이 계획 검토와 다른 관점으로 실제 코드·runtime revision 전이를 다시 공격 검토한다.
