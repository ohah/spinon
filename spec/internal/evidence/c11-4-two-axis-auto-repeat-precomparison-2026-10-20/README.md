# C11.4 양쪽 축 auto-repeat 사전 비교

**상태:** Chrome 155 Agent Browser oracle 관찰만 완료했다. Spinon, Android/iOS Simulator, 실기기 및 WPT 실행 결과가 아니다. C11.4 체크와 공식 완료 수는 바꾸지 않는다.

## 규범 기준과 입력

- 기준: [CSS Grid Layout Module Level 1 §7.2.3.1–7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-notation). auto-repeat은 한 track list 안에서 최대 한 번 허용된다. columns와 rows는 별도 list이므로 각각 하나씩 둘 수 있다. definite 200px axis는 `auto-fill`의 가장 큰 non-overflow 양의 반복 수를, 266px axis는 세 번을 선택한다. `auto-fit`은 반복 수 산정 뒤 placement된 item이 없는 repeated track과 해당 gutter를 collapse한다.
- Oracle: Agent Browser 0.31.1을 통해 띄운 HeadlessChrome `155.0.0.0` UA. Host `arm64`, viewport 1280×633 CSS px, DPR 1.
- Fixture: [grid-auto-repeat-both-axes-c114.html](../../../../tests/fixtures/css/c11/grid-auto-repeat-both-axes-c114.html), SHA-256은 아래 해시 표를 따른다. 모든 case는 `20px repeat(auto-fill|auto-fit, minmax(40px, 1fr) 20px) 10px`, 8px row/column gap을 기본으로 한다.
- Observation JSON은 DOM `getComputedStyle()` resolved track list, `getBoundingClientRect()` root/item frame, UA와 viewport를 저장한다. PNG는 동일 fixture의 전체 화면을 담는다.

## 독립 반복 수 계산

Prefix/suffix 합은 30px이다. 반복 fragment의 두 track 최소 합은 60px이고 전체 track 사이 gutter 수는 `2n + 1`개다. 따라서 minimum used size는 `30 + 60n + 8(2n + 1) = 38 + 76n`이다. 200px에서는 2회가 190px로 들어가고 3회 266px은 넘는다. 266px에서는 3회가 정확히 들어간다. `minmax(40px, 1fr)`의 definite maximum은 없으므로 repeat count 계산에서 definite 40px minimum을 쓴다.

## Chrome 155 관찰

표의 track 값은 prefix, 반복 fragment track 순서, suffix 순서로 나열한다. 원본 문자열과 item rect는 Observation JSON을 기준으로 한다.

| Case | Columns | Rows | Item relative rect |
| --- | --- | --- | --- |
| `columns-fill-control` | `20 45 20 45 20 10` | `20 12` (명시 track) | `(28,28) 12×12` |
| `rows-fill-control` | `20 12` (명시 track) | `20 45 20 45 20 10` | `(28,28) 12×12` |
| `both-fill-square` | `20 45 20 45 20 10` | `20 45 20 45 20 10` | `(28,28) 12×12` |
| `both-fill-asymmetric` | `20 45 20 45 20 10` | `20 40 20 40 20 40 20 10` | `(28,28) 12×12` |
| `both-fit-empty` | `20 0 0 0 0 10` | `20 0 0 0 0 10` | no item |
| `both-fit-single` | `20 154 0 0 0 10` | `20 154 0 0 0 10` | `(28,28) 12×12` |
| `both-fit-span-two` | `20 126 20 0 0 10` | `20 126 20 0 0 10` | `(28,28) 12×12` |
| `both-fit-second-repeat` | `20 0 0 154 0 10` | `20 0 0 154 0 10` | `(28,28) 12×12` |
| `columns-fit-rows-fill-empty` | `20 0 0 0 0 10` | `20 45 20 45 20 10` | no item |
| `columns-fill-rows-fit-empty` | `20 45 20 45 20 10` | `20 0 0 0 0 10` | no item |
| `columns-fit-rows-fill-second-repeat` | `20 0 0 154 0 10` | `20 45 20 45 20 10` | `(28,109) 12×12` |
| `columns-fill-rows-fit-second-repeat` | `20 45 20 45 20 10` | `20 0 0 154 0 10` | `(109,28) 12×12` |

Square `auto-fill` axes independently choose two repetitions; the 266px row axis chooses three while columns remain at two. Mixed `auto-fit`/`auto-fill` cases show that only the fit axis collapses empty tracks, and second-repeat placement preserves the occupied index while the opposite fill axis remains uncollapsed. These are observations for the fixed input, not general WPT coverage.

## 한계와 해석

Chrome computed track list and root/item rects show this browser behavior only. The screenshot does not prove Spinon or simulator rendering. Although the W3C rule permits one auto-repeat in each separate track list, implementation acceptance remains limited to the parser grammar and fixed item/grid cases named in [the focused plan](../../../../plan/c11-4-two-axis-auto-repeat.md). `auto-fit` definite minimum-size axis discrepancy, line names/areas, arbitrary track grammar, WPT, physical devices, hardware GPU and compositor-after pixels remain outside this evidence.

## 해시

```text
ab2a305f2e29ebfe56616125373049330807c8478672d742e079da9ed0adad0f  tests/fixtures/css/c11/grid-auto-repeat-both-axes-c114.html
0f10d241ca27130c67183ef914fc86f1dfec9350eebb538d6f21904ac787b459  spec/internal/evidence/c11-4-two-axis-auto-repeat-precomparison-2026-10-20/chrome155-observation.json
e856a642895f1529fd880d7309a01c18afedebdc2f576c0745fceb028fe752f9  spec/internal/evidence/c11-4-two-axis-auto-repeat-precomparison-2026-10-20/chrome/chrome155-two-axis-agent-browser.png
```
