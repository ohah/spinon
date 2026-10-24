# C11.4 2개 초과 fixed-repeat 그룹 Chrome 사전 비교 · 2026-10-24

**Phase:** 계획-only. 이번 관찰은 CSS 문법과 used-value를 위한 독립 비교 입력이다. Spinon runtime은 변경하거나 실행하지 않았다.

## 캡처 환경

- Fixture: [`grid-unbounded-fixed-repeat-auto-repeat-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-unbounded-fixed-repeat-auto-repeat-c114-precomparison.html)
- Browser: Agent Browser 0.31.1, HeadlessChrome 156.0.0.0, macOS ARM64
- Viewport: 1280×633 CSS px, DPR 1
- Observation JSON: [`chrome156-observation.json`](./chrome156-observation.json)
- 독립 기대값: [`spec-derived-expectations.json`](./spec-derived-expectations.json)
- Screenshot: [`chrome156-full-page.png`](./chrome156-full-page.png)
- 규범: [CSS Grid 2025 CRD §7.2.3.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-notation)

## 독립 산술과 결과

두 장면은 서로 다른 축에서 네 fixed-repeat 그룹을 사용한다. group breadth는 `1×11 + 2×13 + 1×(17+19) + 3×7 = 94px`이고, 보통 neighbor는 5px다. fixed/ordinary track은 아홉 개다. auto-repeat 최소 breadth 20px과 gap 8px을 포함한 최소 공간은 `99 + 20n + 8(8+n) = 163 + 28n`이다. 다섯 회는 303px로 들어가고 여섯 회는 331px라 320px definite axis에는 5회가 들어간다. 남는 17px을 다섯 `1fr` track에 분배한 수학적 기대값은 각각 23.4px다.

| Scene | Chrome 156 CSSOM / geometry | 독립 판정 |
| --- | --- | --- |
| `four-prefix-fill` | columns: `11,13,13,17,19,7,7,7,23.3906×5,5px` | 네 fixed group과 multitrack pattern 순서가 보존된다. CSSOM 1/64px quantization 차이는 수학값 23.4px 이내다. |
| `four-row-split-fill` | rows: `5,11,13,13,23.3906×5,17,19,7,7,7px`; item `(0,74,10,10)` | auto-repeat 전후의 group 순서가 보존되고, line 5의 item 시작은 `5+11+13+13+4×8=74px`다. |

Chrome 관찰은 구현 결과가 아니며 규범 판단을 대신하지 않는다. CSSOM의 track breadth는 독립 산술에서 1/64 CSS px 이내여야 한다.

## SHA-256

| Artifact | SHA-256 |
| --- | --- |
| `tests/fixtures/css/c11/grid-unbounded-fixed-repeat-auto-repeat-c114-precomparison.html` | `15d16ca5911b5a6dc90e52108d42c8f65791bae92926cd29ef42534751cb22cd` |
| `chrome156-observation.json` | `4bc8822632e4b00ae8b21c6931b93bc325b8a14aa646432ea7d606ee09cf8982` |
| `chrome156-full-page.png` | `45ecd1d50e8d0189ba89cef308d1edc4d54ab09663a8802fc58429f70a7ef27e` |
| `spec-derived-expectations.json` | `f65418dd7e5f432712450f98107cf15ae2a1f20af69f65ed851cb1118a33830b` |
