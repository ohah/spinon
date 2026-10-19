# C11.4 이미지 intrinsic track contribution 사전 판정

**상태:** Chrome 155의 기존 고정 관찰값을 구현 전에 추렸다. Spinon 구현·runtime·시뮬레이터 실행 결과가 아니다.

## 기준 입력

- 기준은 Agent Browser가 실행한 Google Chrome `155.0.8059.85`, macOS `26.5.1` ARM, locale `ko-KR`, DPR 1이다. 원본 전체 관찰 JSON은 [`c11-4-grid-track-sizing-precomparison`의 Chrome 결과](./c11-4-grid-track-sizing-precomparison-2026-10-19/chrome/chrome155-observation.json), SHA-256 `c7453b69936c734d110671d9fcb2525c733f0ac42be1c91b0c783084d7f498ce`다.
- 입력 HTML [`grid-track-sizing-c114-precomparison.html`](../../../tests/fixtures/css/c11/grid-track-sizing-c114-precomparison.html)의 SHA-256은 `f2bfd4fc295988c9550026627c8d83f9b2daa04c52633e4a4c10d2dc2e9bc8a6`이다. 기준 이미지 [`c14-1-png-basic.png`](../../../tests/fixtures/resources/c14-1-png-basic.png)은 137 bytes, SHA-256 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`, natural size `24×16 CSS px`다.
- 수치 판정은 Chrome의 grid track, root/item frame, natural image size를 사용한다. 각 CSS px 값의 허용 오차는 `0.05`다. 비교 대상은 기존 26-case 중 image 전용 8개와 image spanning control 1개다.

## 구현 전 고정 기대값

| Case | Columns | Rows | Grid | Item | 별도 불변 조건 |
| --- | --- | --- | --- | --- | --- |
| `auto-image-definite` | `24px` | `16px` | `100×40` | `24×16` | natural `24×16` |
| `minmax-auto-image` | `24px` | `16px` | `100×40` | `24×16` | `minmax(0, auto)` |
| `minmax-min-max-image` | `24px` | `16px` | `100×40` | `24×16` | `minmax(min-content, max-content)` |
| `fit-content-below-minimum` | `24px` | `16px` | `100×40` | `24×16` | 12px limit이 24px automatic minimum을 낮추지 않음 |
| `fit-content-above-intrinsic` | `24px` | `16px` | `100×40` | `24×16` | 40px limit에서 max-content 24px 유지 |
| `fit-content-percent` | `24px` | `16px` | `100×40` | `24×16` | definite inline basis에서 50% limit |
| `fixed-small-track-overflow` | `12px` | `16px` | `12×40` | `24×16` | item은 track 밖으로 12px 넘음 |
| `spanning-min-content-items` | `10px 10px` | `16px` | `100×30` | `24×16` | 두 column 사이 gap `4px`, item span 2 |
| `min-content-row-image` | `100px` | `16px` | `100×80` | `24×16` | natural block contribution으로 row 16px |

## 판정 기준과 경계

1. PNG는 고정 bytes에서 디코드하고, document source와 intrinsic revision을 함께 보존한 `LayoutIntrinsicSizeSnapshot`으로 계산한다. 누락·잘못된 source·stale revision을 natural `0×0`으로 대체하지 않는다.
2. 각 scene에서 computed track, root/item frame, raw replaced contribution `24×16`, Columns/Rows sizing trace의 item·span·contribution source를 확인한다. span scene에서는 두 min-content track의 `10px 10px`, `4px` gap, `24px` item frame을 각각 비교한다.
3. Android 검증은 API 37 ARM64 AVD, iOS 검증은 iPhone 17 Pro / iOS 26.2 Simulator에서 수행한다. 앱 로그의 frame·trace와 Chrome CSS px 기준을 대조한다. screenshot은 box geometry/색 sample만 확인하며 PNG glyph/pixel rendering, OS compositor, 실기기를 입증하지 않는다.
4. 고정 case allowlist 밖의 tag, root/item ID, source, descendant와 track 조합은 fixture에서 거부한다. 기본/product artifact에는 C11.4 fixture route와 추가 payload가 없어야 한다.
5. 이 조각은 direct-leaf HTML `img`와 고정 PNG 하나의 C11.4 제한 fixture다. 일반 image resource loading, mixed text/image item, nested/anonymous item, 전체 CSS Grid 적합성, WPT 완료로 확대하지 않는다.
