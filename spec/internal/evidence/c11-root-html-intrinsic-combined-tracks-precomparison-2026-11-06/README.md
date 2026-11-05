# C11 Root HTML explicit intrinsic tracks + implicit text row 사전 비교

## 재현 조건

- 기준 브라우저: Agent Browser가 실행한 HeadlessChrome `156.0.0.0` (macOS), DPR 1
- 입력: [고정 HTML fixture](../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-combined-tracks-precomparison.html)
- 관측 JSON: [Chrome 156 결과](./chrome156-observation.json)
- 독립 기대값: [expected.json](./expected.json)
- 화면: [전체 페이지 PNG](./chrome156-agent-browser.png)

브라우저는 2026-11-06에 fixture의 `file://` URL을 열어 측정했다. 화면에 표시된 장면 외에도 같은 계산 흐름에서 `getComputedStyle`, `getBoundingClientRect`, text `Range.getClientRects()`를 수집했다. `fontCheck`는 `10px monospace`의 document font readiness를 확인한다.

## 독립 기대와 관측

CSS Grid Layout Level 1 2025 CRD §§11.1, 11.5는 열 track sizing을 먼저 계산하고 그 열 폭을 반영해 행 sizing을 계산하도록 한다. §7.6의 `grid-auto-rows`가 explicit row 이후의 implicit row 크기를 지정한다. 따라서 같은 `A A A A A A` text의 min-content column이 `6.03125px`로 결정되면 text item은 6줄, `60px` block contribution을 만들며 rows는 `[10, 60, 34]px`가 된다. `max-content` 열에서는 텍스트가 한 줄이고 rows는 `[10, 10, 34]px`다. 독립 기대값은 규칙과 별도 intrinsic text probe에서 도출했으며 Chrome 관측은 교차 확인 자료다.

| Scene | Computed columns | Computed rows | Text lines | Grid-local item y positions |
| --- | ---: | ---: | ---: | ---: |
| `min-content` | `6.03125px` | `10px 60px 34px` | 6 | 0, 10, 70 |
| `max-content` | `66.2344px` (used width `66.234375px`) | `10px 10px 34px` | 1 | 0, 10, 20 |
| `auto-track` | `40px` | `10px 20px 34px` | 2 | 0, 10, 30 |
| `minmax-flex` | `40px` | `10px 20px 34px` | 2 | 0, 10, 30 |
| `rows-control` | `40px` | `10px 20px 34px` | 2 | 0, 10, 30 |
| `gap` | `6.03125px` | `10px 60px 34px` | 6 | 0, 12, 74 |

`gap` 장면의 computed `row-gap`은 `5%`이고 item offset은 2px 간격을 관찰했다. Min-content text probe는 `6.03125px`, max-content probe는 `66.234375px`다. 화면 캡처는 Chrome 기준 화면이며 Spinon 구현, WPT 적합성, Android/iOS 픽셀 결과를 뜻하지 않는다.

## Hashes

`fixture-and-screenshot-sha256.txt`에 fixture와 PNG의 SHA-256 및 PNG 해상도를 기록한다.
