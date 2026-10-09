# C04 Runtime CSS→Taffy 구현 전 비교 기준

**상태:** 구현 전 Chromium oracle 확보 · **측정일:** 2026-10-09 · **비교 오차:** 좌표별 최대 `0.5 CSS px`

## 구현 전 비교 모델

- 입력 HTML: [`runtime-style-layout.html`](../../../tests/fixtures/css/c04/runtime-style-layout.html)
- 속성·노드 inventory: [`runtime-style-layout-inventory.json`](../../../tests/fixtures/css/c04/runtime-style-layout-inventory.json)
- 고정 결과: [Chromium JSON](../../../tests/fixtures/css/references/chromium-macos-arm64-macos-26.5.1-25f80-c04-runtime-layout-ebd9482a8d06-6849840baee0-6d02aa4972a1-ccffd5c5fe77/runtime-style-layout.json)
- 기준 ID: `chromium-macos-arm64-macos-26.5.1-25f80-c04-runtime-layout-ebd9482a8d06-6849840baee0-6d02aa4972a1-ccffd5c5fe77`
- reference JSON SHA-256: `90f56f2d68ee3897335edc7e3dd3d66e0f0aafe3c8d9968fb218de8675d6bcb4`
- Fixture SHA-256: `ebd9482a8d0638630d13168ed5c81531cde286ce1602f64061e6ae868de7ff10`
- Inventory SHA-256: `6849840baee033a565880224fbd308e87232a407d95bfa02e0ec90885ab4abd7`
- 수집 도구: [`capture-runtime-style-layout.mjs`](../../../tools/css-reference/capture-runtime-style-layout.mjs) · SHA-256 `6d02aa4972a10b682b20ec4acca48760bffe3d26ae7760565e1e5b89f4988b3a`

## 기준 환경

- Google Chrome `154.0.8037.98`, Chromium revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`
- 실행 파일 SHA-256: `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`
- macOS `26.5.1` (`25F80`), arm64; Node.js `24.20.0`
- viewport `800×600` CSS px, device scale factor `1`, `en-US`, UTC, light scheme, reduced motion `no-preference`, forced colors `none`
- Headless, 외부 네트워크 비활성화, 임시 profile 사용. fixture의 inline style 이외에는 `html, body { margin: 0 }`만 적용한다.

## 관찰 범위와 판정

Chromium `getComputedStyle()`에서 5개 노드의 27개씩 총 135개 CSS 값과 각 노드의 `getBoundingClientRect()` 좌표를 기록했다. computed 값은 앞뒤 공백 제거 후 문자열 정확 일치, `x`·`y`·`width`·`height`는 각각 최대 절대 오차 `0.5 CSS px`로 비교한다. 평균값으로 실패를 상쇄하지 않는다.

| node | `display` | computed `width × height` | Chromium rect `(x, y, width, height)` CSS px |
|---|---|---:|---:|
| `root` | `flex` | `300 × 140` | `(0, 0, 300, 140)` |
| `flex-a` | `block` | `40 × 30` | `(7, 52, 46, 36)` |
| `flex-b` | `flex` | `100 × 80` | `(62, 28, 104, 84)` |
| `block-child` | `block` | `30 × 11` | `(64, 33, 30, 11)` |
| `hidden` | `none` | `20 × 20` | `(0, 0, 0, 0)` |

## 구현 경계

이 기준은 Taffy 구현 전 Chromium 출력만 고정한다. Spinon Rust profile, runtime worker, C ABI, Android/iOS 실행 결과는 아직 비교하지 않았다. 화면 페인트·GPU, 텍스트, inline formatting, Grid, author stylesheet와 CSS 전체 호환을 증명하지 않는다.
