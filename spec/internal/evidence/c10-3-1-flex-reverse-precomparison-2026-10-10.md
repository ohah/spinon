# C10.3.1 · Flex 역방향 사전 비교 기준

## 범위와 판정 기준

이 자료는 `row-reverse`, `column-reverse`, `wrap-reverse`, `flex-flow` 조합을 구현하기 전에 고정한 Chromium 기준이다. 지원 환경은 `writing-mode: horizontal-tb`, `direction: ltr`, viewport `320×240 CSS px`, locale `en-US`, timezone `UTC`다. DPR 1과 2를 각각 관찰하며, CSS px 좌표·크기는 DPR에 따라 바뀌면 안 된다. 각 node의 `x`, `y`, `width`, `height` 최대 절대 오차는 `0.5 CSS px`다. 평균으로 단일 초과를 가리지 않는다.

**oracle:** Google Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, macOS `25.5.0` arm64. 실행 파일 SHA-256은 `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`다. reference ID는 `chromium-darwin-arm64-Chrome-154.0.8037.98-c0e49361e122-da53d5b7e9bf`다.

고정 결과는 [reference JSON](../../../tests/fixtures/css/references/c10-3-1-flex-reverse-v1.json), 입력은 [HTML fixture](../../../tests/fixtures/css/c10/flex-reverse.html)와 [case inventory](../../../tests/fixtures/css/c10/flex-reverse-inventory.json), 재현 명령은 `node tools/css-reference/capture-c10-3-1-flex-reverse.mjs`다. inventory SHA-256은 `c0e49361e122a2dc6699e8cc29428d9092650000caa4adf987fe0ce58c7fa17e`, HTML SHA-256은 `05e1d733bde689a3d5e5ee6ee2e0bc60e69af0513ee201d487b64ab1e1c6ac53`, runtime fixture SHA-256은 `c6abe815f2d2ec4aeec16b42e011eb8752390fc9ecf8f5adc6f552bbd00fe92f`다. capture tool SHA-256은 `da53d5b7e9bfaac87b5998bd0826aa1d68538677a45c8bde4abcd685c80f32de`, 의존 helper SHA-256은 `b010ada41ed75dc2d67ee25abd4266b17930f1de95fc683eebc5cdab6ad2cdc1`다. 기준 파일은 exclusive create로 한 번만 생성되며 테스트 실행이 이를 갱신하지 않는다.

## 고정한 입력과 핵심 관찰값

17개 case, 64개 node를 DPR 1·2에서 수집했다. 전체 computed property와 geometry는 reference JSON을 기준으로 비교한다. 아래 좌표는 viewport 기준이며 자식은 source preorder로 열거한다.

| case | 핵심 기준 결과 |
| --- | --- |
| `row-reverse-start` | 세 20×10 item의 x는 source order대로 `80, 60, 40`; 자식 목록은 바뀌지 않는다. |
| `row-reverse-end` | `justify-content:flex-end`에서 두 item x는 `20, 0`이다. |
| `column-reverse-start` | 두 40×20 item의 y는 `60, 40`이다. |
| `row-wrap-reverse-three-lines` | 세 line의 y는 `24, 12, 0`; 각 line 안의 source item 순서는 유지한다. |
| `column-wrap-reverse-two-lines` | column line의 x는 첫 source line `13`, 다음 line `0`; 주축 item 위치는 각각 `0, 22`다. |
| `row-reverse-wrap-reverse` | 첫 source line item의 frame은 `(55,24)`와 `(10,24)`; line y는 `24,12,0`이다. |
| `column-reverse-wrap-reverse` | 첫 line item의 y는 `22,0`, 다음 source line은 x `0`, y `22`다. |
| `row-reverse-exact-fit` | `40 + 5 gap + 40 = 85` 경계에서 두 item은 같은 line의 x `45,0`이다. |
| `row-reverse-one-pixel-over` | container `84px`에서는 다음 item이 다음 line으로 가고 둘 다 reverse start의 x `44`에서 시작한다. |
| `empty-row-reverse`, `single-column-reverse` | 빈 container는 크기를 보존하고, 단일 item은 column reverse의 끝 y `20`에 놓인다. |
| `flex-flow-direction-first`, `flex-flow-wrap-first` | 두 토큰 순서 모두 computed `row-reverse` / `wrap`이며 동일한 x `45,5` frame을 만든다. |
| `flex-flow-longhand-override` | shorthand 다음 `flex-direction:row`가 승리해 computed `row` / `wrap`, x `0,40`이 된다. |
| `nested-directions-independent` | outer row reverse는 inner box를 x `60`, sibling을 x `40`에 놓고 inner column reverse는 자식 y `20,10`을 유지한다. |
| `display-none-child` | 숨긴 item frame은 0이고 나머지 source item은 reverse 순서의 x `80,60`에 놓인다. |
| `runtime-row-wrap-reverse` | root 320×240, inner container `(8,8,170,68)`; 5개 item은 source 순서대로 `(108,56)`, `(32,56)`, `(108,32)`, `(32,32)`, `(108,8)`이다. |

교차축 방향은 `row`와 `column`에서 서로 다르므로 두 방향을 별도로 포함했다. row의 `wrap-reverse`는 line만 위에서 아래의 반대 방향으로 쌓고, column의 `wrap-reverse`는 column line만 가로 교차축에서 반대로 쌓는다. `row-reverse`/`column-reverse`는 item의 계산 위치를 바꾸지만 원본 child vector나 node ID를 재정렬하지 않는다. 위의 계산은 입력 크기·gap에서 도출한 독립적인 불변 조건이며, Taffy 결과를 기대값으로 사용하지 않는다.

## WPT 교차표

상류 Web Platform Tests는 commit `d5a765f1089ce6d3f72300281481edf3dddff7f3`에 고정했다.

| WPT 경로 | 이 fixture에서 대응하는 범위 |
| --- | --- |
| `css/css-flexbox/flex-direction-row-reverse-001-visual.html` | row 주축 reverse |
| `css/css-flexbox/flex-direction-column-reverse-001-visual.html` | column 주축 reverse |
| `css/css-flexbox/flexbox-flex-wrap-wrap-reverse.htm` | LTR horizontal-tb의 wrap-reverse line stacking |
| `css/css-flexbox/flex-flow-006.html` | row-reverse와 wrap-reverse 조합 |
| `css/css-flexbox/flex-flow-012.html` | column-reverse와 wrap-reverse 조합 |

WPT 파일은 원래 시각 reftest라 이 제한 runtime profile에 그대로 실행했다고 간주하지 않는다. local inventory는 단위 크기·정확/초과 줄바꿈·두 shorthand 토큰 순서·후속 longhand 우선·중첩 컨테이너를 관찰하고, 위 WPT는 같은 CSS 규칙의 상류 대조 경로로 연결한다. vertical writing mode·RTL은 C17 경계 밖이라 포함하지 않았다.

## 범위 경계

이번 사전 기준은 Chromium 결과만 기록한다. Taffy, Stylo typed adapter, V8, WGPU, Android, iOS의 결과는 아직 측정하지 않았다. source traversal 및 기본 paint 순서가 역전되지 않는지 구현 후 별도 검사한다. hit-test는 C10.3.1 범위에 포함하지 않는다.
