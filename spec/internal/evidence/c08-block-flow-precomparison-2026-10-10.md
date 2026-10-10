# C08 Block 흐름 구현 전 Chromium 비교 기준

## 고정 환경

- Chromium: Google Chrome 154.0.8037.98, revision @b859317bf11f6be47f9b7799ec690a0a42a1fb33
- 실행 환경: macOS 26.5.1 (25F80), arm64, headless Chromium
- 입력: [HTML fixture](../../../tests/fixtures/css/c08/block-flow.html), [inventory](../../../tests/fixtures/css/c08/block-flow-inventory.json)
- 기준 결과: [고정 JSON](../../../tests/fixtures/css/references/c08-block-flow-v1.json)
- 캡처 도구: [capture-c08-block-flow.mjs](../../../tools/css-reference/capture-c08-block-flow.mjs)
- SHA-256: HTML 73308088081ac74f356aa2f61ca5240b5c2fa86d5ee848966d4fea8719ec4ed7, inventory 181fa27f968b26ded1a5ddc1148777e0bda930dbf29dc1ae7fa8f433cf9d2b2a, capture 92575f317f327c13595b023f6c121ed1ebf1f3154ebb1dfd1b9174272867f9e4, reference 6f4d4db489379d0d225e4640413cb9f59300176e9a74cf721a5fe4410af676b4.

## 사전 관찰값

DPR 1과 2에서 모든 computed value와 CSS px frame이 같았다. 지원 후보의 frame 비교 허용치는 좌표·크기 각 필드별 0.5 CSS px이며 평균으로 오차를 감추지 않는다.

| 노드 | computed display | frame x, y, width, height (CSS px) | 배경색 | 글꼴·전경색 관찰 |
| --- | --- | --- | --- | --- |
| root | block | 0, 0, 280, 86 | rgb(16, 24, 39) | 16px, 검정, 기본 가족 Apple SD Gothic Neo |
| hero | block | 0, 0, 280, 32 | rgb(51, 102, 255) | 20px, rgb(249, 250, 251), 명시 Arial, sans-serif |
| hero-child | block | 0, 0, 280, 10 | rgb(96, 165, 250) | hero의 color·font-size·font-family 상속 |
| group | block | 0, 32, 280, 38 | 투명 | 기본값 16px, 검정, 기본 가족 |
| first | block | 0, 32, 280, 20 | rgb(225, 29, 72) | 기본값 16px, 검정, 기본 가족 |
| second | block | 0, 52, 120, 18 | rgb(249, 115, 22) | 기본값 16px, 검정, 기본 가족 |
| hidden | none | 0, 0, 0, 0 | rgb(255, 0, 0) | 기본값 |
| hidden-child | block | 0, 0, 0, 0 | rgb(0, 255, 0) | 기본값 |
| last | block | 0, 70, 280, 16 | rgb(34, 197, 94) | 기본값 |

root는 작성자 display 선언이 없는 div이며 내장 UA cascade 결과 block이고, 280px 너비의 auto-height container다. auto-width 자식은 280px로 늘어난다. 중첩 block의 자식은 문서 순서로 y=32, 52에 배치되고 뒤의 last는 y=70에서 시작한다. 너비가 명시된 second는 왼쪽에 붙은 120px다. 숨긴 조상과 descendant는 모두 zero frame이며 뒤의 형제 배치에 공간을 남기지 않는다. hero-child는 hero의 foreground color/font size/font family를 상속한다. 이 fixture에는 텍스트 노드, margin, padding, border, inline formatting, percentage, intrinsic content sizing이 없다.

기본 font-family는 이 실행기의 OS 기본 글꼴을 반영하므로 Apple SD Gothic Neo라는 단일 값만 다른 OS의 보편적인 기대값으로 쓰지 않는다. 고정 macOS oracle은 기본 family를 진단값으로 남긴다. CSS font-size와 color 초기값, 명시 family 및 상속값은 exact comparison 대상으로 삼는다. 이 C08 단계에서 글리프는 그리지 않으며, 전경색·글꼴 관찰은 cascade 근거다. GPU scene의 실제 가시 paint 검증은 배경 box만 대상으로 한다.

## 판정 한계

이 자료는 고정 브라우저의 사전 기준일 뿐 Stylo/Taffy 결과가 일치한다는 증거가 아니다. 부분 alpha 배경, 테두리 선, radius, shadow, 텍스트 shaping·glyph, hit-test, native 실기기, GPU backend 간 래스터 동일성은 이 fixture로 판정하지 않는다. 구현 전 입력과 출력은 plan 및 runtime test에서 같은 inventory/reference digest와 연결해야 한다.
