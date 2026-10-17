# C19.3 첫 구현 조각 사전 비교 기준

## 범위

이번 조각은 두 가지 불변 조건만 구현한다.

- box의 geometry, clip, hit-test 순서가 같으면 paint color alpha와 opacity 값이 달라져도 hit target은 바뀌지 않는다.
- WCAG 2.2 대비 계산 함수는 alpha 합성이 끝난 불투명 encoded sRGB 전경색·배경색을 입력받아 relative luminance와 contrast ratio를 계산한다.

이 함수는 CSS alpha 합성, 실제 glyph raster, anti-aliasing, OS 접근성 트리, WCAG 적합성 판정을 구현하지 않는다. text paint와의 공통 revision 연결은 C15/C16 작업으로 남긴다.

## 브라우저 hit-test 기준

- Agent Browser `0.31.1`에서 Google Chrome `155.0.8059.85`를 실행했다. Chromium revision은 `0e66b94549a8bd85f03fbdf189b752d6e7b63c32-refs/branch-heads/8059@{#1304}`, OS는 macOS `26.5.1` arm64다.
- `about:blank` viewport는 `1280×633`; 좌표는 모두 CSS px다. `devicePixelRatio`는 이 oracle에서 사용하지 않는다.
- 아래쪽 요소 `#under`는 `(0,0)`부터 `80×60 CSS px`; 위 요소 `#top`은 `(20,20)`부터 `40×30 CSS px`, `z-index:1`이다. `#top`의 opacity `{0, 0.5, 1}`와 배경 alpha `{0, 0.4, 1}`를 모두 조합해 `document.elementFromPoint()`를 조회했다.
- 사전 고정 표본은 겹침 `(30,30) → top`, 아래 요소만 있는 `(10,10) → under`, 위 요소 오른쪽 반열린 경계 `(60,35) → under`다. 아홉 조합 모두 같은 세 결과를 냈다.

| opacity | background alpha | (30,30) | (10,10) | (60,35) |
|---:|---:|---|---|---|
| 0 | 0 | top | under | under |
| 0 | 0.4 | top | under | under |
| 0 | 1 | top | under | under |
| 0.5 | 0 | top | under | under |
| 0.5 | 0.4 | top | under | under |
| 0.5 | 1 | top | under | under |
| 1 | 0 | top | under | under |
| 1 | 0.4 | top | under | under |
| 1 | 1 | top | under | under |

## WCAG 2.2 대비 기준

W3C WCAG 2.2의 sRGB 상대 휘도 및 contrast ratio 정의를 사용한다: [성공 기준 1.4.3 이해하기](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum). 독립 oracle은 같은 Chrome에서 실행한 별도 JavaScript 식이며 Rust 구현과 코드를 공유하지 않는다. 입력 채널은 `[0,1]` 범위 encoded sRGB 실수다.

| 전경 | 배경 | 기대 ratio |
|---|---|---:|
| `#000000` | `#FFFFFF` | `21.0` |
| `#767676` | `#FFFFFF` | `4.542224959605253` |
| `#777777` | `#FFFFFF` | `4.478089453577214` |
| `(0.46531904698148846, same, same)` | `#FFFFFF` | `4.500000000000001` |
| `(0.46531894698148846, same, same)` | `#FFFFFF` | `4.500001630867882` |
| `(0.46531914698148845, same, same)` | `#FFFFFF` | `4.499998369132862` |

공식 계수별 fixture를 추가하는 별도 scalar control은 CSS Color 4가 아닌 WCAG 2.2의 encoded-sRGB 상대 휘도 식으로 계산했다. 이 값은 browser 화면 캡처가 아니라 독립 JavaScript 산술 결과다. 각 primary 1.0 채널을 흰색과 비교해 세 luminance 계수를 서로 구별한다.

| 전경 | 배경 | 기대 ratio |
|---|---|---:|
| `(1, 0, 0)` | `#FFFFFF` | `3.9984767707539985` |
| `(0, 1, 0)` | `#FFFFFF` | `1.3721902770517513` |
| `(0, 0, 1)` | `#FFFFFF` | `8.592471358428805` |

정상 텍스트 기준 `4.5:1`의 정확한 양쪽 값을 확인한다. 색 alpha가 있는 입력은 함수에 직접 전달하지 않고, 호출자가 background/backdrop과 합성한 최종 불투명 sRGB 색을 전달해야 한다. fixture가 이 함수 계산 결과와 WCAG 식을 확인하더라도 제품의 텍스트 대비 적합성 또는 실제 렌더 glyph 대비로 확대 해석하지 않는다.

## 통과 조건과 부정 대조

- renderer hit-test 결과는 위 Chrome matrix의 모든 opacity/alpha 조합과 내부 기하 대조에서 같아야 한다. 반열린 경계와 기존 clip/order 규칙도 유지한다.
- 상대 휘도 선형화, 가중치, lighter/darker 선택, `0.05` 보정 중 하나를 바꾸면 고정 RGB fixture 또는 `4.5:1` 경계 양쪽 비교가 실패해야 한다.
- opacity 0에서 hit을 제외하는 mutation은 `(30,30) → top` 검증에서 실패해야 한다.
- 이 fixture만으로 C19.3 전체, C15/C16 text paint 연결 또는 C19 완료를 표시하지 않는다.
