# C11.4 direct-text preferred aspect-ratio axis transfer 사전 비교

이 기록은 `RuntimeBlockFlexGridTrackSizingV1` direct-text fixture가 아직 거부하는 item의 definite width/height와 preferred aspect-ratio 결합을 검토하기 위한 고정 browser baseline이다. 구현 결과나 Spinon WPT pass가 아니다.

## 입력과 환경

- Fixture: [`grid-direct-text-aspect-ratio-axis-transfer-precomparison.html`](../../../../tests/fixtures/css/c11/grid-direct-text-aspect-ratio-axis-transfer-precomparison.html)
- SHA-256: `d06fd4d534e3ca9c2032718ad30e1c30befc77bab6ae2998b4eb3749ae77cb8b` · 3,499 bytes
- Browser: Agent Browser 0.31.1, HeadlessChrome 156.0.0.0, viewport 1280×633, DPR 1
- `document.fonts.check('10px Ahem')`: true
- [Chrome 관찰 JSON](./chrome-observation-agent-browser.json) · SHA-256 `6f736b092d3e69700b30e790a37821a7e0156bb98ff9e6e3a00838cf935f14ac`
- [Chrome 화면](./chrome-156-agent-browser.png) · SHA-256 `f64131fb8d30fcbc458eee5b8880f23d97967a32557248c036241b6c48c4a8e2` · 1280×633 px
- [독립 계산과 expected values](./expected-values.json) · SHA-256 `d65674d8a1ddcc86862639612e98d539c80aff350ce447817af5cbe7740b766b`
- Observation JSON SHA-256: `6f736b092d3e69700b30e790a37821a7e0156bb98ff9e6e3a00838cf935f14ac`
- Screenshot SHA-256: `f64131fb8d30fcbc458eee5b8880f23d97967a32557248c036241b6c48c4a8e2`

## 관찰 결과

모든 root는 content width 100px, 두 `max-content` columns, 한 `auto` row, 0px gap과 `normal` self-alignment를 쓴다. Ahem `A`는 10×10px이고 `AAAA`의 text range는 40×10px이다. `AAAA` 하나만 `aspect-ratio:2 / 1`을 가진다.

| Scene | `AAAA` CSS | Root-relative columns / rows | `AAAA` border box | Text range |
|---|---|---|---|---|
| `width-ratio` | `width:60px;height:auto;aspect-ratio:2 / 1` | `[10,60]` / `[30]` | `(10,0) 60×30` | `(10,0) 40×10` |
| `width-control` | `width:60px;height:auto;aspect-ratio:auto` | `[10,60]` / `[10]` | `(10,0) 60×10` | `(10,0) 40×10` |
| `height-ratio` | `width:auto;height:30px;aspect-ratio:2 / 1` | `[10,60]` / `[30]` | `(10,0) 60×30` | `(10,0) 40×10` |
| `height-control` | `width:auto;height:30px;aspect-ratio:auto` | `[10,40]` / `[30]` | `(10,0) 40×30` | `(10,0) 40×10` |

독립 계산은 ratio `width / height = 2 / 1`을 사용한다. definite `width:60px`은 auto height 30px으로, definite `height:30px`은 auto width 60px으로 전달된다. 각 no-ratio control은 전달을 제거해 height-driven column이 text max-content 40px로 남고 width-driven auto row가 line-height 10px로 남는지 분리한다. CSS Grid의 `normal` sizing과 Ahem text line box의 차이 때문에 `AAAA` text range는 40×10px으로 유지되면서 item box는 60×30px일 수 있다.

## 기준 명세와 한계

- [CSS Grid Layout Module Level 1, 2025 CRD §6.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-item-sizing)은 preferred aspect-ratio가 있는 Grid item의 automatic size 계산이 alignment에 따라 달라짐을 정의한다.
- [CSS Box Sizing Module Level 4, 2026-09-04 WD §4.1–4.3](https://www.w3.org/TR/2026/WD-css-sizing-4-20260904/#aspect-ratio)은 ratio-dependent axis와 definite input에서의 automatic-size 계산을 설명한다. 이 문서는 Working Draft이며 이 fixture는 해당 초안으로 전체 지원을 주장하지 않는다.
- 이 입력은 fixed width/height와 numeric ratio 2/1, Ahem ASCII direct text, 두 `max-content` columns 및 네 scene으로 제한된다. Min/max constraints, box edges, other ratios, other fonts, replaced elements, spans, root HTML 일반 admission, WPT 실행, 실기기/GPU와 C11.4/C11 완료는 검증하지 않는다.
