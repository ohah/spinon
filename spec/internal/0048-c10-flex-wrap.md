# 0048 · C10.1 Flex 줄바꿈

**문서 ID:** `0048` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** 구현 완료, PR #114 리베이스 병합 · **공개 API:** 아님

`0048`은 문서 ID이며 앱·crate·내부 계약의 숫자 버전이 아니다. 구현·문서 개정으로 버전을 올리지 않는다. 이 문서는 C10.1의 제한된 내부 성공 범위와 실패 경계를 정의한다. 전체 Flexbox 또는 CSS 지원을 선언하지 않는다.

## 입력과 실행 경로

같은 HostDocument 세대·문서 revision·style revision·environment revision의 computed-style snapshot을 입력으로 받는다. Stylo가 cascade와 shorthand 확장을 소유하고, 어댑터는 computed `flex-wrap`을 typed `FlexWrap` 값으로 변환한다. CSS 문자열을 다시 파싱하거나 snapshot에서 빠진 속성을 기본값으로 보충해 성공시키지 않는다. 유효하지 않은 computed 값, stale 입력, 잘못된 frame 또는 Taffy 오류는 node/property 문맥을 보존해 전체 layout 결과를 실패시킨다.

실제 앱에서 사용하는 `RuntimeFlexCustomPropertiesPaintV1`뿐 아니라 다음 여섯 runtime Flex profile은 같은 `nowrap|wrap` typed layout 값을 사용한다.

| profile | cascade 출력 | 줄바꿈 projection |
| --- | --- | --- |
| `RuntimeFlexLayoutV1` | layout computed style | 허용 |
| `RuntimeFlexPaintV1` | layout 및 배경 paint | 허용 |
| `RuntimeFlexCustomPropertiesV1` | layout 및 사용자 지정 속성·`var()` | 허용 |
| `RuntimeFlexCustomPropertiesPaintV1` | 위 항목 및 배경 paint | 허용 |
| `RuntimeFlexRegisteredPropertiesV1` | 등록 사용자 지정 속성 포함 layout | 허용 |
| `RuntimeFlexRegisteredPropertiesPaintV1` | 위 항목 및 배경 paint | 허용 |

각 profile의 full cascade와 incremental cascade는 동일한 computed property 목록을 사용해야 한다. Block paint/formatting profile은 `flex-wrap`을 layout projection 목록에서 제외하며 author 입력으로 받지 않는다. 기존 static compatibility profile도 범위 밖이다.

## 지원 값과 기하

| 입력 | 동작 |
| --- | --- |
| 선언 생략 | CSS 초기값 `nowrap`; 자식에 상속하지 않는다. |
| `flex-wrap: nowrap` | 모든 flex 항목을 한 줄에 둔다. 남은 공간이 부족해도 C10.1에서 grow/shrink 알고리즘을 새로 지원한다고 보지 않는다. |
| `flex-wrap: wrap` | 기존에 지원되는 `row` 또는 `column` 주축과 definite container 크기에 대해 DOM 순서대로 line을 수집하고 Taffy 0.14.0으로 전달한다. |
| `flex-flow` | Stylo가 확장한 `flex-direction`과 `flex-wrap` computed winner를 사용한다. shorthand 이후 longhand override를 cascade 순서대로 보존한다. |
| Block 요소 안의 `flex-wrap` | runtime Flex profile에서는 computed value를 보존하지만 Block geometry는 바꾸지 않는다. |
| Block 전용 profile의 `flex-wrap` | 기존 fail-closed author-property 경계로 거부한다. 조용히 성공한 Block 결과로 만들지 않는다. |

주축 `gap`과 줄 사이 교차축 `gap`은 기존 row/column gap 값을 그대로 사용한다. CSS frame은 CSS px이며 DPR 1과 2에서 같은 geometry를 내야 한다. 고정 Chromium 기준과 비교하는 각 `x`, `y`, `width`, `height` 필드는 최대 절대 오차 `0.5 CSS px` 이내여야 한다.

## C10.1 밖의 입력

`wrap-reverse`, `row-reverse`, `column-reverse`, 비기본 `order`, `align-content`, `align-self`, baseline 정렬, 자동 최소 크기, 내재 크기, text/replaced item 측정, indefinite main-size의 column wrap, writing mode 및 RTL의 wrap 상호작용은 이 단계에서 지원하지 않는다. 계산 결과를 기본값으로 바꾸지 않고 node/property를 식별해 실패시킨다. `wrap-reverse`처럼 Stylo가 computed value로 남기는 값은 Taffy projection 경계에서 거부한다.

CSS 선언이 Block에 적용되거나 중첩된 flex container가 부모의 `wrap`을 상속하는 경우도 따로 검사한다. Flex 프로필 전체에서 CSS 비상속 동작을 보존하고, `display:none` subtree는 줄 수집과 geometry에서 제외한다.

## 고정 기준과 완료 조건

비교 기준은 Chromium `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, viewport `320×240 CSS px`, DPR 1·2다. fixture는 12개 case·44개 node로 기본 `nowrap`, exact-fit, 1px 초과, 3개 line과 gap, column 주축·교차축 gap, 빈 컨테이너, 단일 항목, `display:none`, shorthand/longhand override, Block no-op, 중첩 비상속, 실제 Android/iOS 앱의 7개 node 실행 fixture를 포함한다. runtime root는 viewport를 나타내는 세로 Flex 컨테이너로 두어 Block 자식 margin collapse를 C10.1 비교에 섞지 않는다. Block margin collapse는 C09 계약이 별도로 소유한다. reference는 capture HTML·inventory·실행 파일 digest와 일치해야 하며 자동 덮어쓰지 않는다.

C10.1을 완료 표시하려면 다음 증거가 모두 필요하다.

- Chromium reference와 모든 Rust cascade→typed projection→Taffy frame 비교가 통과한다.
- 여섯 runtime Flex profile의 inline 입력과 실제 앱 사용자 지정 속성 paint profile의 author stylesheet 입력을 검증한다. full/incremental cascade 결과가 같고 Block/static profile의 실패 경계를 지킨다.
- Android API 37 emulator와 iOS 26.2 Simulator에서 기본 V8 앱의 `RuntimeFlexCustomPropertiesPaintV1` 경로가 같은 fixture node frame을 보고하고 WGPU 표면에 표시한다. 현재 구현 브랜치에서 Android 7/7·iOS 7/7 frame이 Chromium reference와 일치했고 각 표면에서 7개 box 제출을 확인했다.
- 각 플랫폼 실행 로그·이미지와 구현 실패 경로 검토를 이 계약의 근거로 연결한다. Simulator 결과는 실기기·hardware GPU 성능 검증으로 확대 해석하지 않는다.
- C10.1 완료 뒤에도 C10 상위와 CSS 전체 범위는 미완료로 남긴다.

## 근거

- [C10 Flexbox 계획](../../plan/c10-flexbox.md)
- [구현 전 계획 실패 경로 검토](./evidence/c10-flexbox-plan-review-2026-10-10.md)
- [Chromium fixture](../../tests/fixtures/css/c10/flex-wrap.html) · [inventory](../../tests/fixtures/css/c10/flex-wrap-inventory.json) · [reference](../../tests/fixtures/css/references/c10-flex-wrap-v1.json) · [capture 도구](../../tools/css-reference/capture-c10-flex-wrap.mjs) · [reference 테스트](../../tools/css-reference/c10-flex-wrap.test.mjs)
- [구현 실패 경로 20개 검토·Android/iOS 실행 근거](./evidence/c10-1-flex-wrap-implementation-review-2026-10-10.md) · [Android log](./evidence/c10-1-flex-wrap/android-api37-emulator-logcat.txt) · [iOS log](./evidence/c10-1-flex-wrap/ios-26.2-simulator-log.txt)
- [Android API 37 emulator 화면](./evidence/c10-1-flex-wrap/android-api37-emulator.png) · [iPhone 17 Pro / iOS 26.2 Simulator 화면](./evidence/c10-1-flex-wrap/ios-26.2-iphone-17-pro-simulator.png)
