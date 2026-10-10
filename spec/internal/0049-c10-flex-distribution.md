# 0049 · C10.2 Flex 크기 배분

**문서 ID:** `0049` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** 구현 브랜치 검증 완료·미병합 · **공개 API:** 아님

`0049`는 C10.2의 제한된 내부 레이아웃 계약이다. 앱 작성자에게 공개하는 CSS 지원 선언이나 전체 Flexbox 완료 표시가 아니다. 실제 구현 상태는 [상태 대장](../STATUS.md)을 따른다.

## 입력과 실행 경로

같은 HostDocument 세대·문서 revision·style revision·environment revision의 computed-style snapshot을 입력으로 사용한다. Stylo가 CSS cascade와 `flex` shorthand 확장을 소유한다. 레이아웃 어댑터는 computed `flex-basis`, `flex-grow`, `flex-shrink`, 주축·교차축 크기와 explicit min/max를 `LayoutStyle`로 투영한 뒤 Taffy 0.14.0에 전달한다. CSS 문자열을 어댑터에서 다시 해석하거나 누락된 값을 임의 기본값으로 보완하지 않는다.

대상은 `RuntimeFlexLayoutV1`, `RuntimeFlexPaintV1`, `RuntimeFlexCustomPropertiesV1`, `RuntimeFlexCustomPropertiesPaintV1`, `RuntimeFlexRegisteredPropertiesV1`, `RuntimeFlexRegisteredPropertiesPaintV1`이다. 실제 앱의 기본 paint profile과 author stylesheet를 포함한다. inline full cascade와 incremental cascade는 같은 typed input을 내야 한다. 어댑터에 직접 전달된 음수·NaN·무한대 factor는 node와 property를 식별하는 오류로 거부한다.

## 지원 입력

| 입력 | 내부 동작 계약 |
| --- | --- |
| definite `row`·`column` main size | 해당 line의 독립 free space를 계산한다. |
| `flex-basis`의 px·definite percentage·`auto` | Stylo의 computed winner를 사용한다. `auto`는 명시 주축 크기를 기준으로만 시험한다. |
| positive free space | `flex-grow` 비율로 분배한다. factor 합이 1보다 작으면 남는 공간을 일부 남기는 CSS 규칙을 유지한다. |
| negative free space | `flex-shrink × flex base size`인 scaled factor로 분배한다. |
| explicit min/max | clamp 뒤 위반이 있는 item을 freeze하고 나머지 item에 재분배한다. min이 max보다 큰 경우는 min 우선 결과를 고정한다. |
| main-axis gap·fixed non-auto margin | line의 free space를 계산하기 전에 차감한다. wrap line은 서로 독립적으로 배분한다. |
| CSS invalid declaration | CSS cascade 단계에서 무효 선언을 무시하고 앞선 유효 선언을 보존하는 Chromium 동작을 따른다. |

지원 기하의 최대 절대 오차는 node별 `x`, `y`, `width`, `height` 각각 `0.5 CSS px`다. Chromium `154.0.8037.98` revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`을 수치 oracle로 사용하며 DPR 1·2 결과가 각각 통과하고 서로 같아야 한다. Taffy는 계산 구현 후보이지 정답 oracle이 아니다.

## 범위 밖과 실패 경계

`flex-basis:content`, auto main size에서의 intrinsic basis, indefinite percentage basis, 자동 최소 크기, text/replaced intrinsic measurement, reverse axis, `order`, 비기본 정렬, writing mode·RTL, float·position, fragmentation은 범위 밖이다. 이 단계의 성공이나 Taffy 매핑만으로 이 항목들을 지원한다고 간주하지 않는다.

CSS parser/cascade 이후 어댑터에 직접 주입된 유효하지 않은 factor, 표현할 수 없는 값, stale revision, layout 실패는 성공 frame으로 바꾸지 않는다. 새 revision 계산이 실패하면 이전 frame을 새 revision 결과라고 공개하지 않는다. 플랫폼 report가 잘리거나 frame marker가 없으면 fixture 비교는 실패로 처리한다.

## 재현 자료와 완료 조건

- 독립 입력은 [Chromium HTML fixture](../../tests/fixtures/css/c10/flex-distribution.html) 및 [inventory](../../tests/fixtures/css/c10/flex-distribution-inventory.json)다.
- 실제 V8 앱 입력은 [runtime fixture](../../tests/fixtures/css/c10/runtime-flex-distribution.js)다.
- 고정 기준은 [reference JSON](../../tests/fixtures/css/references/c10-flex-distribution-v1.json), 생성기는 [capture 도구](../../tools/css-reference/capture-c10-flex-distribution.mjs), 회귀 검사는 [reference 테스트](../../tools/css-reference/c10-flex-distribution.test.mjs)다.
- 구현 비교는 모든 관찰 node의 frame을 검증하며 평균으로 개별 오차를 숨기지 않는다. 잘못된 직접 factor 입력, inline full/incremental 스타일 변경, author stylesheet의 사용자 지정 속성, 등록 shorthand를 별도로 확인한다.
- 앱 실행에서는 Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator의 V8 runtime report를 Chromium case의 같은 DOM frame과 비교하고 WGPU 제출·화면을 함께 기록한다. Simulator 통과는 실기기·hardware GPU 성능 증거가 아니다.
- 실제 앱 fixture는 root·flex container·세 item의 다섯 frame을 두 플랫폼에서 각각 Chromium reference와 대조했고, 각 WGPU surface에 다섯 box가 제출됐다. Android는 ANGLE/SwiftShader software backend다. 증거는 [Android 로그](./evidence/c10-2-flex-distribution/android-api37-emulator-logcat.txt)·[화면](./evidence/c10-2-flex-distribution/android-api37-emulator.png), [iOS 로그](./evidence/c10-2-flex-distribution/ios-26.2-simulator-log.txt)·[화면](./evidence/c10-2-flex-distribution/ios-26.2-iphone-17-pro-simulator.png)에 있다. 전체 27개 case·92개 node 비교는 별도 고정 reference 기반 Rust 시험이다.
- 상위 C10은 C10.3/C10.4 등 남은 범위가 있으므로 계속 미완료다.

## 참고

- [C10 Flexbox 계획](../../plan/c10-flexbox.md) · [C10.2 전용 계획](../../plan/c10-2-flex-distribution.md)
- [C10.2 계획 검토 기록](./evidence/c10-2-flex-distribution-plan-review-2026-10-10.md)
- [C10.2 구현 검토와 Simulator 근거](./evidence/c10-2-flex-distribution-implementation-review-2026-10-10.md)
