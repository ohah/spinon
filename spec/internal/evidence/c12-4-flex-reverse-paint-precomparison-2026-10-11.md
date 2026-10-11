# C12.4 · reverse Flex paint 사전 비교

**기준:** Google Chrome `154.0.8037.98`, revision `b859317bf11f6be47f9b7799ec690a0a42a1fb33`, 실행 파일 SHA-256 `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`  
**목적:** Android API 37 AVD의 `flex-row-reverse-paint` pixel 차이를 수정하기 전 고정된 Chromium 관측과 명세를 분리한다.  
**범위:** 서로 겹치는 두 불투명 Flex item, 단일 줄, 정수 `order` 2개. 레이아웃 크기와 위치는 paint 순서 oracle로 사용하지 않는다.

## 재현 입력

Chrome headless DPR 1, `320×240` CSS viewport, sRGB. root는 `left:10px;top:10px;width:150px;height:100px`이다. Flex item은 source-first 파랑 `#2866f6`, source-second 빨강 `#e5484d`이며 각각 row에서는 `110×60px`, column에서는 `100×60px`이다. 같은 방향의 항목을 겹치게 하려고 row / row-reverse는 각각 `margin-right:-40px` / `margin-left:-40px`, column / column-reverse는 `margin-bottom:-40px` / `margin-top:-40px`를 쓴다. `order` 두 값은 `(0,0)`과 `(1,0)`이다. 각 표본점은 두 item의 교집합 중앙이며 Chrome screenshot pixel, `elementFromPoint()`를 함께 읽었다.

| `flex-direction` | source-first / source-second `order` | 교집합 위 픽셀 | Chrome top hit-test |
| --- | --- | --- | --- |
| `row` | `0 / 0` | 빨강 `[229,72,77,255]` | source-second |
| `row` | `1 / 0` | 파랑 `[40,102,246,255]` | source-first |
| `row-reverse` | `0 / 0` | 파랑 `[40,102,246,255]` | source-first |
| `row-reverse` | `1 / 0` | 빨강 `[229,72,77,255]` | source-second |
| `column` | `0 / 0` | 빨강 `[229,72,77,255]` | source-second |
| `column` | `1 / 0` | 파랑 `[40,102,246,255]` | source-first |
| `column-reverse` | `0 / 0` | 파랑 `[40,102,246,255]` | source-first |
| `column-reverse` | `1 / 0` | 빨강 `[229,72,77,255]` | source-second |

이 microfixture에서는 reverse 방향일 때 이미 계산한 stable `(order, source order)` 순서를 paint 목록에서 뒤집으면 관측과 일치한다. 이 규칙은 고정 Chrome 버전에 대한 경험적 결과다. 검증 입력은 WPT 전체가 아니며 이 표만으로 모든 Writing Mode, wrap, positioned child, stacking context 조합을 주장하지 않는다.

## 독립 비교와 명세 경계

- 고정 C12.4 fixture의 `flex-row-reverse-paint`도 Chrome screenshot에서 파랑 `[40,102,246,255]`, Android AVD `PixelCopy`에서 빨강 `[229,72,77,255]`였다. AVD frames는 Chrome과 같고 paint-list 순서만 달랐다. 전체 원본은 [Android surface pixel run](c12-4-android-surface-pixels-2026-10-11/run-c124-20261011T095959787Z-3559b685/comparison.json)과 같은 디렉터리의 `flex-row-reverse-paint.png` / `.log`다.
- W3C [CSS Flexbox Level 1 §4.3, §5.1](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#flex-direction-property)는 Flex item paint에 order-modified document order를 사용하고, reverse 값은 box/paint order를 바꾸지 않는다고 명시한다. 고정 Chrome screenshot과 그 명세 설명 사이에 차이가 있으므로 Chrome 호환 회귀와 표준 적합성 주장을 구분한다.
- 구현 목표는 저장소의 기존 기준인 pinned Chrome 관측을 재현하는 제한 동작이다. 변경 결과를 표준상 일반 규칙이라고 표현하지 않는다. 새 Chrome 버전에서 관측이 바뀌면 기준 버전·예상값·호환 정책을 먼저 갱신한다.

## 사전 판정

1. 변경은 render-tree paint 순서만 바꾼다. Stylo computed value, Taffy frame, HostDocument child order, NodeId, DOM 유사 조회, 접근성/source order는 바꾸지 않는다.
2. 8개 row/column 방향·order 조합의 paint order를 서로 독립적으로 고정한다. 같은 색상 픽셀만 통과하는 비교는 쓰지 않고, NodeId rank와 WGPU pixel을 함께 확인한다.
3. out-of-flow child, nested stacking context, display-none subtree 및 non-reverse Flex의 기존 순서를 보존한다.
4. Android API 37 AVD에서 전체 지원 21 scene / 22 sample의 `PixelCopy` 비교를 다시 수행한다. scene 전부가 통과하기 전 C12.4 pixel gate는 닫지 않는다. 실기기·iOS surface pixel 결과로 일반화하지 않는다.
