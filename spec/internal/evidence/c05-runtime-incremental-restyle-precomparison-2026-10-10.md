# C05.4 구현 전 Chromium 비교 기준

## 고정 조건

- Chromium: Google Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`.
- Viewport: `320×180` CSS px, device scale `1`, locale `en-US`, timezone `UTC`.
- 입력: [`runtime-incremental-restyle.html`](../../../tests/fixtures/css/c05/runtime-incremental-restyle.html) 및 runtime에서 동일하게 실행하는 [`runtime-incremental-restyle.js`](../../../tests/fixtures/css/c05/runtime-incremental-restyle.js). 외부·내장 author stylesheet는 없다.
- 반복 명령: `mise exec -- node tools/css-reference/verify-c05-runtime-incremental-restyle-precomparison.mjs`.
- 기준 결과 파일은 fixture·capture tool·Chromium helper hash와 함께 [`c05-runtime-incremental-restyle-v1.json`](../../../tests/fixtures/css/references/c05-runtime-incremental-restyle-v1.json)에 고정한다.

## 변경 전과 후

첫 실행은 `c054-left`에 `--tile-size:32px`, `c054-right`에 `--tile-size:41px`를 둔다. 두 번째 실행은 왼쪽 branch의 inline `style`만 바꾸어 `--tile-size:46px`로 만든다. 연결 tree와 모든 다른 속성은 유지한다.

| node | 전 computed width | 후 computed width | 전 rect (x, y, w, h) | 후 rect (x, y, w, h) |
|---|---:|---:|---|---|
| `c054-app` | 300px | 300px | (0, 0, 300, 100) | (0, 0, 300, 100) |
| `c054-left` | 140px | 140px | (0, 0, 140, 80) | (0, 0, 140, 80) |
| `c054-left-a` | 32px | 46px | (0, 0, 32, 10) | (0, 0, 46, 10) |
| `c054-left-b` | 32px | 46px | (0, 14, 32, 10) | (0, 14, 46, 10) |
| `c054-right` | 140px | 140px | (150, 0, 140, 80) | (150, 0, 140, 80) |
| `c054-right-a` | 41px | 41px | (150, 0, 41, 10) | (150, 0, 41, 10) |
| `c054-right-b` | 41px | 41px | (150, 14, 41, 10) | (150, 14, 41, 10) |

전·후 computed width/height는 Chromium 문자열과 정확히 같아야 한다. runtime Taffy rectangle은 각 좌표·크기별 최대 절대 오차 `0.5 CSS px` 이내여야 한다. 오른쪽 branch는 값뿐 아니라 위치도 유지되어야 한다.

## 판정 경계

이 결과는 사용자 지정 속성의 inherited cascade와 CSS 입력을 고정한 비교 모델이다. Chromium 결과만으로 Spinon cache가 구현됐다고 판정하지 않는다. 이후 Rust full-cascade oracle, 부분 cascade 계산 수, Taffy 전체 재계산, Android API 37 Emulator와 iOS 26.2 Simulator 화면을 각각 검증한다. 일반 selector invalidation, author stylesheet가 있는 부분 재계산, 부분 layout·paint 또는 실기기 성능을 증명하지 않는다.
