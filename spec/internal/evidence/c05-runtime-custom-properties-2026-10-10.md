# C05.1 런타임 사용자 지정 속성 구현 검토와 실행 근거

**구현 계약:** [0032 `0.1.0` 고정](../0032-c05-runtime-custom-properties.md) · **계획:** [C05.1](../../../plan/c05-runtime-custom-properties.md) · **계획 검토:** [실패 관점 기록](c05-runtime-custom-properties-plan-review-2026-10-10.md)

## 구현 검토에서 고친 내용

- C05.1 computed-style profile이 renderer 허용 목록에 없어 runtime cascade가 성공해도 장면 구성에서 실패할 수 있었다. renderer에 새 paint profile을 명시적으로 허용하고 typed Stylo paint만 읽도록 연결했다.
- 사용자 지정 속성 허용이 일반 속성 허용으로 번지지 않도록 `color:var(--x)`와 layout-only `background-color:var(--x)` 거부를 테스트했다. C05 paint profile도 `color`는 계속 거부한다.
- 초기 scene 테스트는 `gap` 위치를 `31px` 상수만으로 확인했다. pinned Chrome reference의 gap 부모·두 자식 rectangle과 상대 좌표를 직접 비교하도록 바꾸고 각 축 `0.5 CSS px` 한계를 적용했다.
- runtime transition 테스트가 computed width만 읽고 Taffy frame을 확인하지 않았다. 부모 값 수정, 부모 이동, detach, 재삽입 뒤 computed width와 layout frame width를 함께 확인하도록 보완했다.
- 내부 C ABI fixture 함수가 추가됐는데 초안 명세는 C ABI 함수가 전혀 추가되지 않는다고 잘못 썼다. 테스트 전용 함수의 전제·입출력·caller queue·host 경합 금지를 기록했다. 제품 API와 JSON schema/error code는 추가하지 않는다.
- 기존 문서가 Chrome reference에 저장된 모든 rectangle을 Taffy가 비교하는 것으로 읽힐 수 있었다. 실제 자동 geometry oracle은 scene gap 부모·두 자식이며, 나머지 rectangle은 현재 유한성만 확인한다고 명확히 했다.

## 구현 변경에 대한 20개 독립 실패 관점

| # | 공격 관점 | 확인 내용과 결과 |
|---:|---|---|
| 1 | 출시 전 버전이 오르는가 | Cargo·내부 계약 숫자는 계속 `0.1.0`; C05.1만 체크하고 C05 부모는 미완료다. |
| 2 | 입력 범위가 CSSOM으로 확장되는가 | 기존 `style` 속성만 받는다. 새 CSSOM·author stylesheet 입력은 없다. |
| 3 | 별도 var parser가 Stylo와 달라지는가 | parser와 substitution은 고정 Stylo `0.22.0`에 맡긴다. 자체 토큰 치환 코드는 추가되지 않았다. |
| 4 | 사용자 지정 속성 이름을 소문자로 합치는가 | `--Measure`와 `--measure` 구분 사례가 Chrome 값 `23px`와 정확히 일치한다. |
| 5 | 등록되지 않은 사용자 지정 속성이 상속되지 않는가 | 부모 `--measure:41px`를 자식 `width`가 받아 Chrome과 일치한다. |
| 6 | `inherit`·`unset`·`initial`·`revert`가 같은 값이 되는가 | 네 keyword를 개별 고정 fixture로 두고 Chrome 값 `41, 41, 23, 41px`와 비교한다. |
| 7 | 중첩 fallback 안 쉼표를 잘못 분할하는가 | `var(--missing,var(--space,29px))`가 `13px`가 되어 Chrome과 일치한다. |
| 8 | 빈 사용자 지정 속성을 미정의로 취급하는가 | 빈 값은 fallback을 쓰지 않고 일반 `margin-left` 초기값 `0px`를 낸다. |
| 9 | self/mutual cycle이 일부 정상 값으로 남는가 | self·상호 cycle 값을 모두 fallback으로 계산하고 Chrome과 일치한다. |
| 10 | 잘못 대체된 일반 속성이 이전/임의 값을 유지하는가 | `margin-left:var(--bad)` 결과가 `0px`로 Chrome과 일치한다. |
| 11 | `!important`가 custom property 순서를 보존하는가 | `19px!important`가 뒤 선언 `43px`보다 우선해 width가 `19px`다. |
| 12 | `--*` 허용이 임의 일반 속성 허용으로 번지는가 | literal `color`와 `color:var(--supported)` 모두 `unsupported_inline_property`로 남는다. |
| 13 | layout profile이 paint를 몰래 받거나 paint가 모든 속성을 받는가 | layout은 `background-color`를, C05 paint도 `color`를 fail-closed로 거부한다. |
| 14 | 기존 Flex shorthand 확장값이 새 profile에서 누락되는가 | `gap:var(--space)`가 row/column gap 각 `11px`로 cascade되어 Taffy에 전달된다. |
| 15 | 배경색을 CSS 문자열 재파싱으로 그리는가 | renderer는 Stylo의 typed opaque sRGB paint만 쓰고 세 색의 정확한 채널을 확인한다. |
| 16 | viewport와 scene geometry가 달라지는가 | pinned Chrome `gap-parent`와 두 자식의 상대 좌표·크기를 `0.5 CSS px` 이내로 직접 대조한다. |
| 17 | 문서 update 결과가 layout에 반영되지 않는가 | 상속 width와 Taffy frame width가 새 revision에서 `41→73px`로 함께 바뀐다. |
| 18 | 노드 이동·detach 뒤 옛 부모 값이나 옛 frame이 남는가 | 이동 후 `91px`, detach 후 style/frame membership 없음, 재삽입 후 `73px`다. |
| 19 | 내부 FFI host 수명·동시 호출 조건이 빠지는가 | 기존 host를 재사용하고 Android executor/iOS `runtimeQueue`에서 순차 호출한다. C ABI 안전 계약에 해제·경합 금지와 출력 버퍼 조건을 둔다. |
| 20 | Android/iOS가 다른 JS 입력·표시 경로를 쓰는가 | 두 플랫폼이 같은 고정 JS fixture·C ABI·Rust CSS/layout/paint 경로를 사용하고 실제 V8 평가와 3-box WGPU 표시를 로그·캡처에서 확인한다. |

검토 중 확인한 별도 범위 제한: Chrome reference에는 더 많은 노드의 rectangle이 저장되지만 현재 Taffy 자동 geometry 대조는 gap scene 세 노드뿐이다. 이는 전체 HTML layout parity, 전체 CSS, `@property`, stylesheet cascade, 증분 계산, 실기기·하드웨어 GPU 성능 근거가 아니다.

## 실행 근거

### 기준 비교

- Oracle: Google Chrome `154.0.8037.98`, macOS arm64, viewport `390×844 CSS px`, DPR 1, light screen.
- Reference: [`c05-runtime-custom-properties-v1.json`](../../../tests/fixtures/css/references/c05-runtime-custom-properties-v1.json). HTML, input JSON, app JavaScript, capture script, Chromium helper와 실행 파일 hash가 저장되고 검증기에서 일치 여부를 검사한다.
- computed style 문자열 비교는 정확 비교다. gap scene 부모·두 자식 geometry는 상대 좌표와 크기마다 최대 `0.5 CSS px` 차이를 허용한다.

### Rust·JavaScript

- `mise exec -- cargo test --locked --workspace`
- `mise exec -- bun run test`
- `mise exec -- cargo fmt --all -- --check`
- `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings`
- `mise exec -- node --test tools/css-reference/c05-runtime-custom-properties.test.mjs`

### 플랫폼 시뮬레이터

- Android: API 37, arm64 emulator `emulator-5554`; fixture 실행 보고 `status=0`, `document_revision=15`, `render_tree_revision=7`, `layout=ready`, `boxes=3`. [화면 캡처](c05-runtime-custom-properties-android-emulator.png)
- iOS: iPhone 17 Pro, iOS 26.2 Simulator; fixture 실행 보고 `status=0`, `document_revision=15`, `render_tree_revision=7`, `layout=ready`, `boxes=3`. [화면 캡처](c05-runtime-custom-properties-ios-simulator.png)
- 빌드 명령은 각각 `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_V8_DIR=<V8 checkout> mise exec -- bash tools/build-android-app.sh`, `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_V8_DIR=<V8 checkout> mise exec -- bash tools/build-ios-sim.sh`다. 두 앱은 실제 V8로 동일 C05.1 JavaScript fixture를 실행했다.
- 물리 Android 기기·iOS 기기, GPU 성능, 메모리 사용량, 화면 간 픽셀 차이는 측정하지 않았다.

## 남은 C05 작업

`@property`, author stylesheet와 CSS resource 연결, CSSOM API, 의존 property 단위 dirty-subtree 재계산, persistent cache와 성능·메모리 근거는 구현하지 않았다. C05.1은 내부 runtime slice이며 사용자용 CSS 지원 항목을 완료하지 않는다.
