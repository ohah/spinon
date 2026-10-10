# 0037 · C06.1 백분율 크기 전달

- **내부 계약 숫자 버전:** `0.1.0` 고정 · 출시 또는 호환성 정책을 사용자가 정하기 전 숫자를 올리지 않는다.
- **문서 식별자:** `0037`은 내부 명세 일련번호이며 제품·계약 버전이 아니다.
- **상위 상태:** [CSS 구현 체크리스트](../STATUS.md#css-구현-체크리스트)
- **작업 계획:** [C06 값·단위 변환](../../plan/c06-value-unit-conversion.md)
- **비교 fixture:** [Chromium 관찰값](../../tests/fixtures/css/references/c06-percentage-dimensions-v1.json)
- **계획 검토:** [C06 실패 경로](evidence/c06-value-unit-conversion-plan-review-2026-10-10.md)

## 목적과 공개 범위

이 내부 계약은 `width`, `height`, `flex-basis`의 cascade 결과가 백분율일 때 그 비율을 픽셀로 미리 바꾸지 않고 Taffy layout 입력까지 보존한다. `spinon-style`은 Stylo의 typed computed value에서 독립 DTO를 만들고 `spinon-style-to-layout`은 이를 `LayoutDimension::Percent`로 전달한다. 레이아웃 결과는 CSS px다.

이는 내부 Rust/FFI 런타임 계약과 검증 fixture다. JavaScript 공개 API, `getComputedStyle`, 전체 CSS 지원 보장이나 출시 호환성 선언을 추가하지 않는다. C06.1 범위는 이 계약으로 관리하고, C06.2는 별도 [계약 0038](0038-c06-spacing-percentages.md), C06.3은 [계약 0039](0039-c06-absolute-lengths.md)로 관리한다. C06 상위와 C06.4–C06.6은 미완료다.

## 값 표현

| Stylo typed 값 | 내부 DTO | Taffy 입력 | 의미 |
|---|---|---|---|
| `auto` | `ComputedCssDimension::Auto` | `Dimension::auto()` | 자동 크기 |
| finite nonnegative CSS length | `LengthPx(css_px)` | `Dimension::length(css_px)` | CSS px 길이 |
| finite nonnegative percentage ratio | `Percentage(fraction)` | `Dimension::percent(fraction)` | `50%`는 `0.5`, `125%`는 `1.25` |
| math expression 또는 이 계약의 keyword 밖 값 | `Unsupported` | 입력하지 않음 | 속성·노드 문맥을 유지한 명시적 오류 |

Stylo의 percentage는 이미 0–1 비율로 표현되므로 다시 100으로 나누지 않는다. 비율을 `[0, 1]`로 clamp하지 않는다. `0%`는 `0px`와 같은 값으로 합치지 않고 DTO 경계까지 percentage로 유지한다.

`NaN`, 무한대, 음수, 지원하지 않는 computed variant는 기본값으로 대체하지 않는다. CSS authored 값의 invalid declaration 처리와 어댑터에 직접 주어진 잘못된 내부 값은 구분한다. Stylo가 invalid declaration을 cascade 결과의 `auto` 등으로 바꿨다면 그 typed 결과를 쓴다. 어댑터에서 잘못된 비율을 직접 받으면 구체적인 layout 오류로 거부한다.

## 퍼센트 기준과 root 정책

- 자식 `width`와 `height`는 definite containing block의 content-box 축을 기준으로 Taffy가 계산한다.
- flex row의 `flex-basis`는 definite main width, column은 definite main height를 기준으로 한다. Flex 최종 크기는 basis, grow, shrink와 다른 size 제약을 반영한 Taffy 결과다.
- `HostDocument`의 CSS runtime 생성 경로는 viewport를 root containing block으로 제공한다. 이 계약은 그 경로의 root percentage를 허용한다.
- core `Tree`의 `RootSizingPolicy::MatchViewport`는 유지한다. 모든 `LayoutInput` 경로에서 root의 고정 viewport invariant를 삭제하지 않는다.
- `LayoutStyle`이 border edge를 보존하지 않으므로 nonzero border를 포함한 box-sizing 판정은 지원 경계가 아니다. padding만 포함하는 기준과 border 모델은 별도 fixture·계약으로 다룬다.
- definite가 아닌 height containing block의 percentage height는 Chromium 결과를 관찰하고 비교하지만, Taffy 의미가 일치하지 않으면 C06.1의 지원으로 주장하지 않는다.

## 문자열 직렬화 경계

CSSOM의 computed style 문자열은 Taffy 입력이 아니다. 고정 Chromium 관찰에서 root `height: 100%`는 `getComputedStyle()`에 used value인 `800px`로 노출되는 반면 Stylo typed computed value는 `100%`를 보존했다. 따라서 크기 속성은 typed percentage kind와 실제 frame으로 비교한다. 두 엔진에서 의미와 계산 단계가 같은 나머지 fixture property만 CSSOM 문자열을 정확 비교한다. Stylo computed string을 Chrome CSSOM used value와 동일해야 한다고 강제하지 않는다.

## 포함·제외 속성

| 입력 | C06.1 처리 |
|---|---|
| `width`, `height`, `flex-basis`의 `%`, finite nonnegative px, `auto` | typed 추출 후 Taffy에 전달 |
| 음수 size declaration | Stylo의 cascade 결과 사용; 어댑터 직접 음수 값은 거부 |
| `>100%`, `0%`, fractional percentage | clamp·정수 반올림 없이 유지 |
| `calc()`, intrinsic sizing keyword, unsupported computed variant | 명시적 미지원 오류 |
| percentage `margin`, `padding`, `gap` | 미지원, 성공으로 변환하지 않음 |
| viewport/container 단위, `em`/`rem`, 절대 길이 단위 | C06의 후속 하위 항목 |
| nonzero `border`, absolute positioning | 이 profile/layout DTO 비교 범위 밖 |

## 업데이트와 수명

percentage DTO는 해당 `ComputedElementStyle`의 `NodeId` 및 style snapshot revision과 함께 이동한다. incremental cascade 재사용 시 문자열 출력과 typed dimension은 같은 노드·같은 계산 revision에서 복사한다. 기존 cache entry가 typed 필드를 누락하거나 revision을 증명하지 못하면 부분 재사용하지 않고 전체 cascade를 수행한다. DOM/V8 객체나 Stylo 포인터를 layout crate로 보관하지 않는다.

## 검증 기준

- Chromium 154.0.8037.98 고정 기준의 C06.1 fixture에 포함된 각 지원 노드에 대해 `x`, `y`, `width`, `height` 절대 오차를 각각 `0.5 CSS px` 이하로 비교한다.
- `0%`, `125%`, 분수 비율, auto, invalid negative declaration, content-box padding, definite/indefinite height, root viewport, row/column basis는 서로 독립된 assertion을 가진다.
- `calc()`와 percentage margin/padding/gap은 fail-closed 동작이어야 한다.
- Android API 37 ARM64 emulator 및 iPhone 17 Pro / iOS 26.2 Simulator는 같은 3-node JavaScript fixture를 V8→Stylo→Taffy→WGPU 경로로 실행한다. `layout=ready`, root CSS viewport frame, 3 boxes, `presented` 로그와 캡처를 확인한다. 이 모바일 smoke fixture는 22-node Chromium/Rust 수치 fixture와 다르며 child frame별 수치 비교는 제공하지 않는다. simulator는 기능 확인 환경이고 실기기 성능·호환 보장을 제공하지 않는다.
- C06.1 완료 조건은 별도 fixture·Rust oracle·Android/iOS Simulator smoke로 확인했다. C06.1 체크는 상위 C06, C06.2 또는 C06.4–C06.6의 완료를 뜻하지 않는다. C06.3은 계약 0039로 분리해 검증한다.
