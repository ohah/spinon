# C06.5 · typed CSS math

- **문서 유형:** 구현 계획 · 완료 상태는 [`spec/STATUS.md`](../spec/STATUS.md)에서 관리
- **대상:** 현재 runtime Flex profile에서 처리하는 `width`, `height`, `flex-basis`, `margin`, `padding`, `row-gap`, `column-gap`
- **내부 숫자 계약 버전:** 출시 전 `0.1.0` 고정
- **비교 기준:** 고정 Chromium 154 reference, Stylo 0.22.0 typed computed values, Taffy 0.14.0 layout
- **선행:** C06.1–C06.4
- **계획 실패 경로 검토:** [계획 검토](../spec/internal/evidence/c06-5-typed-css-math-plan-review-2026-10-10.md)
- **사전 비교 근거:** [Chromium 154 fixture와 Stylo typed AST](../spec/internal/evidence/c06-5-typed-css-math-precomparison-2026-10-10.md)

## 목표와 경계

Stylo가 파싱·cascade·CSS 계산식의 타입 적합성을 담당한다. style-to-layout adapter는 Stylo의 typed CSS 수학값을 소유형식의 layout 표현으로 변환한다. 계산식 문자열을 다시 파싱하거나 CSS 단위를 임의 문자열 연산으로 처리하지 않는다.

첫 구현은 다음 계산 노드를 지원한다.

- 기본 `calc()` 산술: 합, 곱, 부호 반전, 역수. 계산식 안의 CSS `<length>`는 CSS px로, `<percentage>`는 비율로 전달한다.
- `min()`·`max()`의 길이/백분율 인자.
- 숫자형 인자가 있는 세 인수 `clamp(min, value, max)`.
- 위 함수의 중첩 및 `var()` 치환 후 Stylo가 반환한 최종 typed 값.

Stylo Typed OM에 안전하게 reify되지 않는 계산식, 다른 math 함수(`round()`, `mod()`, `rem()`, 삼각 함수 등), 아직 계산되지 않은 viewport/container 단위는 명시적 미지원 오류다. `clamp()`의 `none` 한계도 우선 미지원으로 두고 Chromium·Stylo typed output에서 관찰한 결과를 fixture에 기록한다. 이 범위는 현재 CSS profile의 하위 호환 slice이며 전체 CSS math 적합성 선언이 아니다.

## 계산값 계약

| 내부 값 | 의미 | Taffy 입력 |
|---|---|---|
| `Number(f32)` | 단위 없는 CSS 계산 피연산자 | layout 결과로 직접 사용하지 않음 |
| `LengthPx(f32)` | C06.3·C06.4까지 CSS px로 계산된 길이 | 절대 길이 피연산자 |
| `Percentage(f32)` | `100% = 1.0` | 현재 속성의 CSS percentage basis에 비례 |
| `Sum`, `Product`, `Negate`, `Invert`, `Min`, `Max`, `Clamp` | Stylo Typed OM에서 복사한 허용 연산자 트리 | Taffy layout 계산 중 basis를 받아 평가 |

계산 트리 변환 때 연산자 arity·깊이·전체 노드 수를 제한하고, 결과 CSS dimension이 이 profile의 `<length-percentage>`와 맞는지 확인한다. 유효한 CSS 계산식도 나눗셈으로 NaN/±∞ 중간값을 만들 수 있으므로 그 값을 AST에서 조기에 버리지 않는다. 길이와 백분율은 같은 `<length-percentage>` 계산에서 합칠 수 있지만, 임의 `<number>`를 길이로 오인하지 않는다. CSS parser/Stylo가 이미 무효 선언으로 처리한 값은 그 CSS cascade의 initial/inherited 결과를 따른다. adapter에서 처리할 수 없는 typed 연산은 노드 ID·속성·계산값을 담은 오류로 중단한다.

실제 사용값은 percentage basis를 받은 뒤 계산한다. width/height와 flex-basis는 기존 C06.1 percentage 기준 판정을 그대로 사용한다. spacing은 C06.2의 margin/padding/gap별 definite-axis 기준을 그대로 따른다. 계산식 안에 percentage가 있으면 단순 percentage와 같은 basis 검증을 받는다. indefinite basis를 0으로 대체하지 않는다.

CSS Values 4는 Working Draft다. 고정 Chromium이 관찰하는 결과를 이 내부 slice의 비교 기준으로 쓰며, 문서가 정의한 ±∞/NaN 계산을 “divide by zero invalid”로 단순화하지 않는다. 이 slice는 typed AST 중간값을 `f64`로 평가하고, 최종 used value에서만 Chromium 154가 관찰한 `33,554,428 CSS px` 범위와 NaN/±∞ censor를 적용한다. width/height/flex-basis/padding/gap은 nonnegative, margin은 signed 결과를 보존한다. 이 값은 전체 브라우저 적합성 선언이 아니다. 정의된 censor로도 유한 frame을 만들 수 없거나 typed contract가 맞지 않으면 계산 입력 전체를 실패 처리하고 부분 layout/frame을 공개하지 않는다.

## Taffy 연결 규칙

현재 `spinon-layout`은 Taffy의 고수준 `TaffyTree`를 사용한다. Taffy 0.14.0에서 `calc` feature만 켜도 이 고수준 tree는 `resolve_calc_value`의 기본 구현(0.0)을 사용한다. 따라서 고수준 tree에 opaque calc 포인터를 넣는 구현은 금지한다.

계획된 연결은 Taffy가 제공하는 저수준 tree trait와 Flex/Block 계산 함수를 쓰는 작은 내부 tree adapter다.

1. Taffy `calc` feature를 명시적으로 활성화한다.
2. 저수준 adapter가 child traversal·layout cache·Flex/Block dispatch·결과 저장을 맡고, `LayoutPartialTree::resolve_calc_value`에서 내부 owned expression을 계산한다.
3. Taffy의 `*const ()` calc 값은 `#[repr(align(8))]` owner 안에 보관한 내부 expression만 가리킨다. Taffy는 포인터 하위 3 bit를 tag로 쓰므로 8-byte 정렬을 보장한다. Stylo의 포인터·비공개 calc node 포인터를 Taffy에 넘기지 않는다. Arc/owner는 단일 layout 계산이 끝날 때까지 유지하고, adapter가 pointer→expression map으로 해석한다. Taffy가 전달한 opaque pointer를 직접 역참조하지 않는다.
4. Taffy callback은 오류를 반환할 수 없으므로, 수치 오류를 adapter side channel에 기록하고 안전한 placeholder로 계산을 마친 뒤 최종 결과를 폐기한다. 오류가 발생한 계산에서 부분 frame은 반환하지 않는다.
5. CSS math가 없는 기존 입력은 같은 Taffy 0.14.0 Flex/Block 알고리즘 및 기존 반올림·percentage basis 결과를 유지한다.

저수준 adapter가 Taffy 고수준 tree의 동작 중 하나라도 빠뜨리면 calc 기능을 붙이지 말고 차이를 고친다. layout cache key, display:none subtree, block/Flex dispatch, panic 처리, frame 수집을 기존 결과와 비교한다.

## 비교 fixture와 구현 순서

Chromium fixture는 `320×800 CSS px`, DPR 1과 2, 고정 locale/timezone을 쓴다. 각 node에 input declaration, cascade 결과, CSSOM 관찰값, bounding rect, 기대 분류(`supported`, `css-invalid`, `adapter-unsupported`, `oracle-only`)를 별도 기록한다. CSSOM serialization은 관찰용이며 final geometry와 typed AST를 대체하지 않는다. 지원 rect의 축별 최대 absolute error는 `0.5 CSS px`다.

계획 순서는 다음과 같다.

1. **완료:** Chromium fixture·고정 capture 도구에서 `calc()`의 length-only/mixed percentage, 연산 우선순위·중첩, `min/max/clamp`, negative clamp, non-finite, `var()` 치환, invalid dimension, unsupported math, DPR 결과를 캡처했다.
2. **완료:** Stylo 0.22.0의 typed AST를 확인하고 Stylo 소유 메모리를 넘기지 않는 owned DTO 추출을 구현했다. 유효 CSS 연산 중간값의 IEEE NaN/±∞는 보존하고 트리 크기와 연산자 형태를 제한한다.
3. **완료:** 별도 layout AST와 Rust evaluator를 구현했다. 계산 dimension·percentage basis·속성별 signedness를 검증하고 최종 단계에서만 Chromium 154의 used-value censor를 적용한다.
4. **완료:** Taffy 0.14.0 저수준 tree adapter를 구현했다. Flex/Block/cache/hidden/leaf 계산을 연결하고 기존 수치 fixture 및 layout 오류의 원자적 폐기를 검증했다.
5. **완료:** `calc` resolver를 연결하고 고정 Chrome fixture의 25개 지원 node geometry를 비교했다. 최대 축별 absolute error는 `0.5 CSS px` 이하였다. unsupported `round()`는 math 지원으로 세지 않았다.
6. **완료:** Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 같은 V8 HostDocument fixture를 실행했다. `layout=ready`, 11개 layout box, WGPU `presented`, 화면 캡처와 로그를 저장했다. Android emulator는 ANGLE/SwiftShader software renderer다. 실기기·성능 결과로 확대 해석하지 않는다.
7. **완료:** 전체 Rust workspace 테스트와 CSS reference verifier, Clippy, rustfmt, FFI 검사, Android/iOS simulator build/run을 마쳤다. 실제 명령과 결과는 구현 검토 기록에 둔다.
8. **완료:** [구현 실패 경로 검토](../spec/internal/evidence/c06-typed-css-math-implementation-review-2026-10-10.md)에 별도 공격 사례 20개와 코드·fixture 근거를 기록했다. 깊이·노드 수·연산자 arity 한계를 독립적으로 두드리는 회귀 테스트를 추가한 뒤 전체 Rust workspace와 CSS reference 검증을 다시 수행했다.

## 완료 조건

- Stylo typed values에서 허용된 math AST만 추출되고, 문자열 재파싱·Stylo/Taffy 내부 포인터 직접 공유가 없다.
- Taffy calc callback은 올바른 속성 basis에서 계산한다. undefined basis, invalid typed node 및 callback 실패는 부분 layout commit 없이 보고된다. 유효한 CSS math의 NaN/±∞ 중간값은 final used value까지 보존한 뒤 Chromium 기준으로 censor된다.
- CSS width/height/flex-basis와 margin/padding/gap에서 property range·signedness를 구별한다. `clamp(min > max)`의 min 우선 동작을 고정 Chromium과 대조한다.
- CSS-invalid와 adapter-unsupported 값을 서로 다른 fixture 결과로 다루며 unsupported 기능이 0 또는 기본값처럼 성공 처리되지 않는다.
- 기존 C06.1–C06.4 Rust/Chromium frame 결과가 바뀌지 않고 Android/iOS simulator의 실제 runtime/WGPU 경로가 성공한다.
- 상위 C06 및 C06.6은 미완료로 유지한다. 내부 숫자 계약 버전과 모든 Spinon crate 버전은 `0.1.0`이다. 새 명세 일련번호가 추가되더라도 제품/계약 semver가 아니다.

## 남는 미지원 범위

- `clamp()`의 `none` bound (Stylo typed output이 이를 안전하게 전달하지 못하면 명시적 오류)
- `round()`, `mod()`, `rem()`, `abs()`, `sign()`, trig, exponent/logarithm, `hypot()`, `progress()` 등 추가 CSS math 함수
- viewport/container/dynamic viewport units와 metric font units
- intrinsic sizing 함수·anchor functions·table/grid/multicol/position layout profile
- text layout, CSSOM, 외부 stylesheet fetch, 전체 CSS 100% 적합성 및 실기기 성능
