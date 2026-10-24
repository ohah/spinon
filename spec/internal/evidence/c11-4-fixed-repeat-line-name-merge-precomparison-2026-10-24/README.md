# C11.4 fixed-repeat named-line merge Chrome 사전 비교 · 2026-10-24

**Phase:** 계획-only. CSS Grid 문법과 used-value를 Chrome으로 교차 관찰하고 규범에서 독립 기대값을 계산했다. Spinon runtime은 변경하거나 실행하지 않았다.

## 캡처 환경

- Fixture: [`grid-multi-fixed-repeat-line-name-merge-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-multi-fixed-repeat-line-name-merge-c114-precomparison.html)
- Browser: Agent Browser 0.31.1, HeadlessChrome 156.0.0.0, macOS ARM64
- Viewport: 1280×633 CSS px, DPR 1
- Observation JSON: [`chrome156-observation.json`](./chrome156-observation.json)
- 독립 기대값: [`spec-derived-expectations.json`](./spec-derived-expectations.json)
- 판정 스크립트: [`validate-precomparison.py`](./validate-precomparison.py)
- 계획 PR 변경분 30관점 검토: [`pr-diff-review-30.md`](./pr-diff-review-30.md)
- Screenshot: [`chrome156-full-page.png`](./chrome156-full-page.png)
- 규범: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-syntax), [§7.2.2 named lines](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#common-uses-named-lines), [§8.3 line-based placement](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement)

## 독립 sizing과 name merge 기대값

세 fixed-repeat 그룹의 track breadth는 `2×(12+8) + 2×(6+6) + 2×10 = 84px`이다. auto-repeat은 최소 `24px`, gap은 `4px`이고 전체 non-auto track은 10개다. 300px definite axis에서 최소 필요 공간은 `84 + 24n + 4(9+n) = 120 + 28n`이다. 여섯 반복은 288px, 일곱 반복은 316px이므로 기대 반복 수는 6이다. 각 축의 기대 explicit tracks는 `[12, 8, 12, 8, 6, 6, 6, 6, 24, 24, 24, 24, 24, 24, 10, 10]`이다.

반복 사이의 같은 경계에 닿는 이름 목록을 CSSOM에서 확인했다. 특히 fixed-repeat 마지막 `[mid-end]`, list 경계 `[seam]`, auto-repeat 시작 `[seam]`은 `[mid-end seam seam]`으로 합쳐진다. 두 `seam` 토큰은 같은 물리 line에 있지만 named occurrence는 그 line을 한 번만 센다. 그래서 `seam 5 / seam 6`은 8번과 9번 explicit grid lines 사이의 첫 auto-repeat track이다. auto-fit 장면은 `repeat-end 3 / repeat-end 4`가 네 번째 반복 track을 점유하며, 그 밖의 반복 track과 gutters는 collapse한다.

| Scene | Chrome 관찰과 독립 판정 |
| --- | --- |
| `col-auto-fill` | 16 tracks, auto-fill 6회. 양·음수 occurrence, span, 두 fixed-repeat group 경계, auto-repeat 경계, suffix 이름을 확인했다. |
| `col-auto-fit` | 6회 산정 후 auto-repeat used breadth는 `0, 0, 0, 24, 0, 0px`; named item은 visible fourth repeated track에 남는다. |
| `col-ordinary-join` | ordinary 8px track의 양쪽 line names를 fixed-repeat·auto-repeat 경계에 병합했다. 120px axis에는 `[10,10,8,12,12,12,12,12]` tracks가 들어가고 named placements는 `(28,0,8,40)`, `(56,0,12,40)`이다. |
| `row-auto-fill` | columns와 같은 16-track sequence·6회 산정. named line의 세로 위치는 auto-fill 1회분 track geometry와 일치한다. |

독립 rectangle 기대값과 Chrome 관찰은 `validate-precomparison.py`로 대조한다. CSSOM track/name 관찰은 oracle cross-check이고 규범·독립 산술을 대체하지 않는다. 결과는 Spinon 구현 또는 공개 CSS 지원의 증거가 아니다.

## SHA-256

| Artifact | SHA-256 |
| --- | --- |
| `tests/fixtures/css/c11/grid-multi-fixed-repeat-line-name-merge-c114-precomparison.html` | `30665d6c52ece50fbcfe564c0484b69d53ecc697904b2365078b9d54059b2409` |
| `chrome156-observation.json` | `6d6b980ad3092c9d00cce4b8b149e5680913984f2890bc2701b066e16146534d` |
| `chrome156-full-page.png` | `600c5e877bb7db9f48db4159b91fbf5eca5d3d4d469588a6d03d9fd743481000` |
| `spec-derived-expectations.json` | `916f36abb0aa37d9fd10e93b835c7ef524027a6a4be1f744969218dcdc55a25d` |
| `validate-precomparison.py` | `7f641b68a1d92641ddda5f1fc94b06ef695c889251656cad5fe0af97c10ff2e9` |
