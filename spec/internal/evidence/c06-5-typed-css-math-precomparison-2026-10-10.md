# C06.5 typed CSS math 사전 비교

**상태:** 사전 비교 완료 · layout evaluator와 Taffy 연결은 미구현 · 내부 계약 버전 `0.1.0` 유지

## 고정 환경

| 항목 | 값 |
|---|---|
| 기준 브라우저 | Google Chrome `154.0.8037.98` · revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33` |
| Chrome 실행 파일 SHA-256 | `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954` |
| 실행 환경 | macOS `25.5.0` · arm64 · Node `v24.20.0` |
| viewport / DPR | `320×800 CSS px` · DPR `1`, `2` |
| locale / timezone | `en-US` / `UTC` |
| Stylo | `0.22.0`, 저장소에서 정확한 버전으로 고정 |
| Taffy | `0.14.0`, 저장소에서 정확한 버전으로 고정 |

Chromium reference ID는 `chromium-darwin-arm64-Chrome-154.0.8037.98-1cac0952928a-af9650f48dd6`이다. inventory SHA-256은 `1cac0952928ae9e0051c57baf850bd19d320bf4f6162cf278e708e03ae11ce05`, HTML fixture SHA-256은 `d8e135a4057d0cfcac34e628b63e3122b384990ba42c23735a967c617cf85ba4`, capture 도구 SHA-256은 `af9650f48dd67af4ce1fb81f427a58bf287ddc541fde87d1f136baacdac6f65f`이다. 전체 capture 환경·도구 정보와 각 노드 관찰값은 `tests/fixtures/css/references/c06-typed-css-math-v1.json`에 저장했다.

## 관찰 결과

26개 요소에서 computed CSSOM 값, CSS Typed OM 타입·직렬화, bounding rect를 캡처했다. `calc()`는 CSSOM 직렬화만 보고 AST라고 가정하지 않는다. Typed OM 값과 used geometry를 각각 보존한다.

| 입력 사례 | Chromium computed / used 관찰 |
|---|---|
| `calc(10px + 25%)`, containing width 300px | Typed OM `CSSMathSum`; used width `85px` |
| 중첩 `calc(10px + calc(20px + 10%))` | `CSSMathSum`; used width `60px` |
| `min(100px, 50%)` / `max(100px, 50%)` | `100px` / `150px` |
| `min(calc(50px + 10%), calc(80px + 5%))` | `80px` |
| `clamp(20px, 50%, 100px)` | `100px` · Typed OM `CSSMathClamp` |
| `clamp(100px, 80px, 50px)` | `100px` · 최소값이 최대값보다 클 때 최소값 우선 |
| `clamp(10px, calc(0px / 0), 20px)` | `0px` |
| `calc(10px / 0)` / `calc(0px / 0)` | 각각 finite `33554428px` / `0px` |
| `calc(20px - 50px)` / `margin-left: calc(5px - 10px)` | size는 `0px`, signed margin은 `-5px`이며 frame x도 `-5` |
| gap `calc(5px + 5%)`, basis `calc(40px + 10%)` | column gap `20px`, 두 번째 Flex child x `50`, basis frame width `70px` |
| 잘못된 `width: calc(1px + 1s)` 앞에 선언한 `width:23px` | invalid 선언을 버리고 `23px` |
| `width: round(10px, 4px)` | `12px`; 첫 구현 범위 밖 math function 사례 |

전 노드의 computed 값과 CSS px rect가 DPR 1과 2에서 같았다. 결과의 중심은 length-only 식이 아니라 percentage basis를 필요로 하는 계산식이다. 0 나눗셈을 문법 오류로 단정하거나 non-finite 계산을 Taffy에 그대로 넘기지 않도록 고정 관찰값을 남겼다.

## Stylo typed 값 연결

실제 고정 의존성은 Stylo `0.22.0`이다. 기존 계획에 적힌 `0.20.0`은 낡은 버전이어서 현재 `Cargo.toml`과 `Cargo.lock`을 확인한 뒤 바로잡았다. `LengthPercentage::to_typed_value()`의 결과 중 현재 slice가 처리할 수 있는 numeric unit과 math node를 별도의 owned `ComputedCssMath`로 복사하도록 `ComputedElementStyle.layout_math_values`를 추가했다. 퍼센트는 Stylo Typed OM 표현에서 `value / 100`으로 바꿔 내부 `100% = 1.0` 계약을 유지한다. AST 복사 중 단위 미지원, 비유한 숫자, 32단계 초과 깊이, 256개 초과 node는 결과에서 빠져 이후 adapter가 명시 실패하도록 한다.

Stylo fixture는 `calc()`·`var()`가 cascade된 값을 재파싱 없이 사용해 `Sum`, `Min`, `Max`, `Clamp` 형태와 spacing·flex-basis 계산식이 snapshot에 보존되는지 검사한다. 이는 Stylo → owned DTO 경계까지의 구현이다. DTO → `LayoutStyle`, layout evaluator, Taffy 저수준 `resolve_calc_value`, Android/iOS 화면은 아직 연결하지 않았으며 C06.5 완료로 표시하지 않는다.

## 검증

- `mise exec -- bun test tools/css-reference/c06-typed-css-math.test.mjs`: 6개 기준 검증 통과
- `mise exec -- cargo test --locked -p spinon-style typed_css_math -- --nocapture`: Stylo owned typed AST test 통과
- 이 기록에는 Taffy 계산·렌더러·시뮬레이터·실기기 검증을 포함하지 않는다.

## 버전 구분

- 모든 Spinon Cargo package 버전은 출시 전 정책에 따라 `0.1.0`으로 유지한다.
- `0037`–`0040`은 내부 계약 문서의 일련번호이지 semver가 아니다.
- Stylo `0.22.0`, Taffy `0.14.0`, Bun `1.4.2`는 각각 외부 도구·의존성의 고정 버전이며 Spinon 출시 버전을 올리는 변경이 아니다.
