# C11.4 mixed intrinsic, flexible, and fixed track precomparison

## 목적과 분리된 기준

이 자료는 intrinsic `min-content` base가 flexible track의 §11.7.1 freeze/recompute 결과를 바꾸는 여덟 고정 scene을 구현 전에 고정한다. 각 장면에는 intrinsic text item, 40px fixed track 안의 natural-size PNG image, `minmax(0px, 2fr)` track의 text item이 있다. Columns/Rows, freeze/no-freeze/equality, reversed source-order controls를 포함한다. Chrome 관측과 표준 산술은 서로 다른 기준으로 보존한다.

- 규범 기준: [CSS Grid Layout Module Level 1, 2025 CRD §§11.5.1, 11.7, 11.7.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-flex-tracks). §11.5.1은 `minmax(min-content, 1fr)`의 base를 text item의 min-content contribution에서 구하고, §11.7.1은 definite free space에서 hypothetical flex size가 base보다 작은 flexible track을 freeze한 뒤 분배를 다시 계산한다.
- 독립 계산: [`derive_expected.py`](./derive_expected.py)는 정수/유리수 산술로 [`expected.json`](./expected.json)을 생성한다. Taffy 또는 Chrome의 관측값을 계산 입력으로 사용하지 않는다.
- 교차 관측: [Chrome 156 observation](./chrome156-observation.json)은 Browser DOMRect/CSSOM 결과다. 이 값은 규범적 기대값을 대체하지 않는다.

## 고정 입력과 독립 산술

Columns scene은 300×120px root에 `minmax(min-content, 1fr) 40px minmax(0px, 2fr)` columns와 10px column gap 두 개를 둔다. Rows scene은 이를 120×300px root의 rows로 옮기고, Ahem text item에 `writing-mode: vertical-rl`을 적용해 row-axis intrinsic contribution을 만든다. 양 축에서 Ahem 10px의 `XXXXXXXXXX`는 100px, `XXXX`는 40px min-content contribution을 가진다. 가운데 fixed track에는 24×16px PNG image를 둔다.

300px sizing axis에서 두 gap과 fixed 40px track을 빼면 flexible tracks의 definite space는 240px이다. 1fr:2fr의 첫 hypothetical flex fraction은 80px이다.

- 100px min-content base는 첫 1fr의 80px hypothetical size보다 커서 그 track을 100px에서 freeze한다. 남은 140px는 2fr track에 가므로 최종 tracks는 `[100, 40, 140]px`이다.
- 40px min-content base는 80px hypothetical size보다 작아 freeze하지 않는다. 최종 tracks는 `[80, 40, 160]px`이다.
- 80px min-content base는 hypothetical size와 같다. §11.7.1의 strictly-smaller freeze 조건을 넘지 않으므로 freeze하지 않고 `[80, 40, 160]px`를 만든다.
- reversed source order에서도 fixed placement가 같으면 각 scene의 track과 identity별 item geometry가 유지되어야 한다.

따라서 `track sum + two gaps = 300px`, remaining free space는 여덟 scene 모두 0px이다. 독립 root-relative item frame, axis, text base와 exact rational track 크기는 `expected.json`에 있다.

## Chrome 156 관측

Agent Browser 0.31.1이 macOS ARM64에서 실행한 HeadlessChrome `156.0.0.0`, viewport `1280×633` CSS px, DPR 1을 사용했다. 수집 전 `document.fonts.ready`, Ahem font readiness와 여덟 PNG image decode 완료를 확인했다.

| Scene | Computed tracks | CSSOM/DOMRect root-relative tracks |
| --- | --- | --- |
| `column-freeze` | columns `100px 40px 140px` | `[100, 40, 140]px` |
| `column-no-freeze` | columns `80px 40px 160px` | `[80, 40, 160]px` |
| `column-equal-boundary` | columns `80px 40px 160px` | `[80, 40, 160]px` |
| `column-freeze-reversed` | columns `100px 40px 140px` | `[100, 40, 140]px` |
| `row-freeze` | rows `100px 40px 140px` | `[100, 40, 140]px` |
| `row-no-freeze` | rows `80px 40px 160px` | `[80, 40, 160]px` |
| `row-equal-boundary` | rows `80px 40px 160px` | `[80, 40, 160]px` |
| `row-freeze-reversed` | rows `100px 40px 140px` | `[100, 40, 140]px` |

Chrome의 track array와 모든 root/item global frames는 `chrome156-observation.json`에 기록했다. Browser의 global root 위치는 page layout에 따라 달라질 수 있으므로 구현 expected frames는 root-relative `expected.json`을 사용한다. 전체 비교 화면은 [PNG](./chrome/mixed-intrinsic-flex-fixed.png)다.

## 입력 및 산출물 해시

| Artifact | SHA-256 |
| --- | --- |
| [fixture](../../../../tests/fixtures/css/c11/grid-mixed-intrinsic-flex-fixed-c114-precomparison.html) | `b883498138595aaf37bdb89deecfd64675baec7a28fffa1cabfda369c0f5aebc` |
| Ahem 10px font | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |
| PNG natural-size source | `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2` |
| Chrome observation JSON | `96f3f7ac8029027ac20f0cb79bfb4c3eccf237f6d6c11b4129325b28aff226f9` |
| Independent expected JSON | `7de5cb44203088e2fb0d552d37c027b3dcf6cf2da032d238074e1934e047d9f9` |
| Chrome screenshot | `5872dc66d906248ed3d7eb60d3246568e8b58c2412191a9d9d4f5fd294762872` |

This precomparison establishes browser and normative expected values only. It does not claim Spinon implementation, Android/iOS runtime, overflow paint, WPT pass, general CSS admission, or C11.4 completion.
