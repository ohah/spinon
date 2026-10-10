# 0041 · C06.5 Typed CSS math

- **상태:** 현재 작업 브랜치 구현·검증 완료 · 미병합 · 공개 API 아님
- **내부 계약 숫자 버전:** 출시 전 `0.1.0` 고정
- **문서 식별자:** `0041`은 내부 문서 번호이며 제품·계약 숫자 버전이 아니다.
- **구현 계획:** [C06.5 Typed CSS math](../../plan/c06-5-typed-css-math.md)
- **상위 계약:** C06 값·단위 변환
- **비교 기준:** Chrome 154.0.8037.98, Stylo 0.22.0, Taffy 0.14.0

## 계약 목표

현재 runtime Flex/Block profile에서 Stylo가 계산한 typed CSS math를 소유형 layout AST로 옮기고, Taffy resolver의 percentage basis를 받아 used value로 계산한다. CSS 문자열을 다시 파싱하지 않으며 Stylo 내부 calc 객체나 Taffy opaque pointer를 외부 소유자에게 넘기지 않는다.

이 문서는 내부 구현 계약이다. 전체 CSS 지원, 웹 호환성, 공개 API, 출시 호환성을 선언하지 않는다.

## 지원 입력

현재 계산 대상 속성:

- `width`, `height`, `flex-basis`
- 각 방향 `margin` 및 `padding`
- `row-gap`, `column-gap`

현재 owned AST 연산자와 단위:

- CSS px, percentage (`100% = 1.0`), unitless number
- 합, 곱, 부호 반전, 역수, `min()`, `max()`, 세 인수 `clamp()`
- 중첩 계산식 및 Stylo cascade가 반환한 `var()` 치환 결과

AST는 Stylo typed output에서 복사한다. 소스 declaration 보완 경로는 cascade winner에 한해서 사용하며, computed typed output이 제공한 값이 있으면 이를 우선한다. 이 adapter가 직접 `var()`를 치환하거나 CSS 텍스트를 해석하지 않는다.

`round()`, `mod()`, `rem()`, `abs()`, `sign()`, 삼각 함수, 지수·로그 계열, viewport/container/dynamic viewport 및 font metric 단위는 이 문서가 지원한다고 약속하지 않는다. Stylo가 별도 함수를 이미 primitive typed value로 계산해 돌려준 경우 그 값은 일반 typed value처럼 취급될 수 있지만, 해당 함수 전체의 지원 근거로 간주하지 않는다.

## 계산·layout 계약

1. AST 추출에서 연산자 인자 수(최대 64), 트리 깊이(최대 32), 전체 노드 수(최대 256)를 확인한다. 잘못된 구조와 지원하지 않는 dimension은 `InvalidCssMath`로 실패한다.
2. `<length-percentage>`끼리 합산할 수 있다. product에는 최대 한 개의 `<length-percentage>`만 허용하고, 그 외 피연산자는 unitless number여야 한다. 역수는 number에만 허용한다.
3. 계산 중에는 `f64`를 사용한다. 유효 CSS math가 만든 NaN/±∞를 중간 AST에서 조기 거부하지 않는다. 최종 used value 단계에서만 고정 Chrome이 관찰한 `±33,554,428 CSS px` 범위와 NaN/±∞ censor를 적용한다.
4. `width`·`height`·`flex-basis`·`padding`·`gap`은 최종 nonnegative 값을 사용한다. `margin`은 음수와 부호를 보존한다. `clamp(min > max)`는 고정 Chrome 기준의 min 우선 결과를 사용한다.
5. Percentage는 resolver가 해당 property에 전달하는 basis로 계산한다. width/height/flex-basis 기준은 C06.1, margin/padding/gap의 definite-axis 기준은 C06.2를 따른다. 비율 계산식도 단순 percentage와 같은 definite-basis 검증을 받으며 indefinite basis를 0으로 대체하지 않는다.
6. 각 계산식은 입력 `NodeId`, property, 고유 `LayoutCalcId`에 묶인다. 중복 ID, owner/property mismatch, 알 수 없는 Taffy calc handle은 실패 처리한다. 알 수 없는 handle을 callback에서 만났을 때만 안전한 임시 `0`을 반환하고, side-channel 오류로 전체 layout 결과를 폐기한다. 부분 frame은 공개하지 않는다.
7. Taffy calc handle은 8-byte 정렬된 내부 owner의 주소만 가리킨다. owner는 해당 layout 계산이 끝날 때까지 살아 있고, 전달된 opaque pointer는 직접 역참조하지 않는다. callback 오류가 발생해도 계산을 안전하게 마친 뒤 최종 오류를 반환한다.

## 비교 기준과 허용치

`tests/fixtures/css/references/c06-typed-css-math-v1.json`의 두 DPR observation을 Chrome 154 기준으로 사용한다. 현재 Rust geometry 검증은 25개 지원 node의 각 `x`, `y`, `width`, `height`를 비교하며 축별 허용 오차는 `0.5 CSS px`다. oracle-only/지원 밖 함수인 `round-function`은 지원 통계에 포함하지 않는다.

Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 동일 V8 HostDocument runtime fixture를 실행한다. 확인 항목은 `status=0`, `layout=ready`, `boxes=11`, renderer present 및 화면 캡처다. Android emulator의 관찰 renderer는 ANGLE/SwiftShader software GL이며 이 결과는 hardware GPU나 성능 근거가 아니다. 실기기 검증은 하지 않았다.

## 실패 및 검증 근거

- typed node 차원·basis·소유자·연산자 구조 오류: `InvalidCssMath` 또는 구체 binding 오류로 실패한다.
- unknown handle: placeholder 계산을 최종 결과로 쓰지 않고 전체 layout을 실패시킨다.
- Chromium 비교 fixture: [C06.5 기준 JSON](../../tests/fixtures/css/references/c06-typed-css-math-v1.json)
- 계획 실패 경로 검토: [계획 검토](evidence/c06-5-typed-css-math-plan-review-2026-10-10.md)
- 구현 실패 경로 검토: [구현 검토](evidence/c06-typed-css-math-implementation-review-2026-10-10.md)
- 플랫폼 증거: [Android 화면·로그](evidence/c06-typed-css-math/android-api37-emulator.png) · [iOS 화면·로그](evidence/c06-typed-css-math/ios-26.2-simulator.png)

문서 식별자 `0041`은 제품 버전이 아니다. 계약 숫자와 모든 Spinon crate 버전은 출시 전 `0.1.0`으로 유지한다.
