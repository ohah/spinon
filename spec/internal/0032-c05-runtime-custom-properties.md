# 0032 · C05.1 Runtime inline 사용자 지정 속성

**계약 버전:** 미출시 내부 계약 `0.1.0` 고정 · **상태:** C05.1 구현·시뮬레이터 검증 완료 · **공개 API:** 아님 · **C05 전체 지원:** 미완료

## 범위

C04.8~C04.10 runtime 경로에서 기존 HTML `style` 속성의 CSS 사용자 지정 속성과 `var()` 대체를 Stylo cascade에 연결한다. 계산 대상은 현재 허용한 Runtime Flex layout 속성 및 C04.10 `background-color`다. 같은 HostDocument·viewport·style·environment revision으로 계산된 결과만 layout과 GPU 장면에 사용한다.

이 문서는 내부 구현 계약이다. 앱 작성자 API, `CSSStyleDeclaration`/CSSOM, author stylesheet 등록, `@property`, 외부 CSS 자원 로딩, 일반 CSS 속성 지원, 성능 보장을 제공하지 않는다. 미출시 내부 계약 숫자는 `0.1.0`으로 유지한다. `v1` 문자열은 고정 fixture/reference 자료 형식 이름이며 제품·계약 출시 버전이 아니다.

## 입력·CSS 계산

- 입력은 각 HostRoot 아래 연결된 HTML 요소의 기존 `style` 속성이다. V8 DOM 호출 뒤 HostDocument의 새 `DocumentRevision`을 제출하는 C04.8 경로를 따른다.
- `--*` 사용자 지정 속성 선언을 허용한다. 이름은 대소문자를 구분하며 등록되지 않은 속성의 기본 상속, `inherit`·`unset`·`initial`·`revert`, `!important`, 중첩 fallback, 빈 값, self/mutual cycle, 미정의 참조를 Stylo `0.22.0`의 parser·cascade가 계산한다. 별도 자체 token substitution은 구현하지 않는다.
- `var()`를 사용할 수 있는 일반 속성은 기존 runtime layout allowlist와 paint profile의 `background-color`로 제한한다. `display`, 크기, Flex, gap, 허용 margin/padding 계열 등 목록 밖 일반 CSS 선언은 기존처럼 `unsupported_inline_property` 실패를 낸다. paint profile이 아닌 layout profile에서는 `background-color` 역시 허용하지 않는다.
- cycle 또는 fallback 없이 invalid-at-computed-value가 된 값은 CSS cascade의 해당 속성 초기값 처리 결과를 따른다. 빈 custom value는 미정의 custom property와 같은 것으로 처리하지 않는다.
- 선언 파서의 recovery diagnostic, HostRoot·text·다중 root 검증, computed value의 Taffy 허용 범위는 C04.8~C04.10 계약을 유지한다. 파싱 가능한 사용자 지정 속성의 임의 값은 민감 정보 보호를 위해 JSON 진단에 복사하지 않는다.

## 재계산과 수명

- 현재 runtime은 새 DocumentRevision 또는 환경 revision을 받을 때 연결된 HostRoot fragment 전체를 다시 cascade하고 layout한다. dirty-subtree 증분 재계산은 구현하지 않는다.
- Stylist·ComputedValues·NodeId를 request 사이에 두는 별도 사용자 지정 style cache를 추가하지 않는다. 현재 request에 연결되지 않은 detached 노드는 계산 결과와 layout frame에 들어가지 않는다.
- 이동한 노드는 이동 후 snapshot의 새 부모에서 상속을 다시 계산한다. 제거 뒤 재삽입은 이후 revision에서 새 연결 상태를 계산한다.
- CSS worker의 기존 latest-wins admission과 full revision key 검사를 따른다. 이전 revision 계산이 나중에 끝나도 현재 완료 결과를 덮어쓰지 않는다. 순서 의미가 있는 JS 평가·DOM 변경은 병합하지 않는다.
- 기존 C04.8 UA cascade JSON, C04.9 layout JSON, C04.10 render key JSON과 ABI 함수의 signature는 유지한다. C05.1 전용 내부 fixture C ABI 호출 하나는 아래 별도 계약으로 추가한다. 제품 API·JSON schema·오류 코드는 추가하지 않는다.

## 내부 시뮬레이터 fixture 호출

`spinon_runtime_gpu_host_eval_custom_properties_fixture(host, layout_timeout_millis, output, output_capacity)`는 기존 C04.10 GPU host의 살아 있는 V8 세션에서 정적 C05.1 JS fixture를 한 번 평가한다. 새 HostDocument·노드를 만들지 않는다. 선행 C04.10 fixture가 전역 lexical binding `root`, `opaque`, `transparent`를 만들었다는 전제 아래 기존 요소의 `style` 속성을 `setAttribute()`로 변경한다. 호출은 기존 host evaluate 상태 코드와 NUL 종료 보고 문자열을 쓴다. 출력 포인터·용량·host 유효성 및 동시 host 호출·해제 금지는 기존 runtime GPU FFI 계약을 따른다.

Android 데모는 runtime executor, iOS 데모는 `runtimeQueue`에서 호출한다. UI 버튼과 명시적 launch probe만 이 fixture를 시작한다. 이는 앱 작성자에게 제공하는 C ABI·DOM API가 아니라 C04.10 surface로 이어지는 플랫폼 검증 진입점이다. 기본 launch에는 C05.1 fixture를 자동 적용하지 않는다.

## 오류와 fail-closed 경계

| 상황 | 동작 |
|---|---|
| 허용된 사용자 지정 속성 및 `var()`가 유효한 계산값을 냄 | 해당 allowlist style 값으로 layout/paint 계산 |
| 순환, 미정의 참조, 빈 값 또는 substituted value가 속성 문법에 맞지 않음 | Stylo CSS cascade 결과를 사용하며 유효하지 않은 일반 속성을 임의 보정하지 않음 |
| 지원하지 않는 일반 CSS 속성 | layout/paint profile 검증에서 전체 layout request 실패 |
| layout profile에서 `background-color` 선언 | 기존 layout-only profile과 같이 거부 |
| 잘못된 viewport·root·HostDocument 관계 또는 revision mismatch | 기존 C04.8~C04.10 오류 경로로 요청 실패, 일부 결과를 게시하지 않음 |
| worker 실패 또는 오래된 계산 완료 | 현재 요청 상태에 실패를 보존하거나 오래된 완료를 버리고 기존 복구 규칙을 따름 |

새 오류 코드, 제품 JSON field, 앱 공개 함수는 추가하지 않는다. 내부 fixture용 C ABI 함수는 위 계약에 한해 추가한다. 문자열 진단의 기존 오류 `code` 계약은 C04.9 명세를 따른다.

## 고정 비교 기준과 검증

비교 기준은 fixture HTML과 capture 도구의 hash로 고정한 Google Chrome `154.0.8037.98`이다. 테스트는 computed style 문자열을 정확 비교하고, Chromium reference는 각 노드의 CSS px rectangle을 보관한다. 고정 사례는 상속, case-sensitive 이름, 중첩 fallback, 네 CSS-wide keyword, self/mutual cycle, 잘못된 값, 빈 값, `!important`, `gap:var(...)`, 조상 값 변경 및 다른 부모로 이동을 포함한다.

Rust cascade 비교는 고정 Chromium reference의 width·margin-left·row-gap·column-gap 결과를 검사한다. runtime worker 테스트는 상속 값 변경·이동·detach·재삽입 뒤 대상 computed width와 Taffy frame width가 `41→73→91→없음→73 CSS px`가 되는지 확인한다. runtime render test는 변수로 계산한 gap 부모와 두 자식의 상대 위치·크기를 Chromium geometry와 대조하고, 허용 오차는 각 축 최대 `0.5 CSS px`다. 평균으로 실패를 숨기지 않는다. reference에는 다른 노드의 rectangle도 저장하지만 현재 Rust 자동 geometry 비교 범위는 이 세 scene 노드로 제한한다.

이 범위는 Rust/runtime 고정 fixture 검증과 Android API 37 emulator·iPhone 17 Pro / iOS 26.2 Simulator의 실제 V8 → HostDocument → CSS worker → WGPU 화면 실행을 포함한다. [Android 화면](evidence/c05-runtime-custom-properties-android-emulator.png)과 [iOS 화면](evidence/c05-runtime-custom-properties-ios-simulator.png)은 emulator/simulator 증거이며 실기기·하드웨어 GPU 성능 검증은 범위 밖이다.

## 후속으로 남기는 C05 항목

- stylesheet source/owner가 생긴 뒤의 `@property` registration, syntax descriptor, inherits 및 initial value
- 실제 author stylesheet를 통한 사용자 지정 속성 cascade와 CSS resource/bundler 결합
- parent dependency 추적과 dirty-subtree 단위 증분 무효화·재계산
- 반복 계산 성능·메모리 기준 및 persistent style cache 도입 근거
- DOM CSSOM 변경 API와 computed style 조회 API

따라서 C05.1 완료는 상위 C05, C29 CSSOM, 일반 CSS 호환 또는 100% CSS 지원 완료를 뜻하지 않는다.
