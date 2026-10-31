# C11.3.3 양 끝 implicit named-line 사전 비교

**날짜:** 2026-11-01

**기준:** Chrome `156.0.8078.80` Headless, Agent Browser `0.31.1`, macOS, viewport `720×400` CSS px, DPR 1

**Spinon 기준 source:** `76968e0a6dbbd364fa2794954df4ecab87074cc6` (Chrome 비교만 수행했으며 Spinon 구현은 실행하지 않음)

**고정 입력:** [`c1133-both-implicit-named-lines-precomparison.html`](../../../../tests/fixtures/css/c11/c1133-both-implicit-named-lines-precomparison.html)

**관찰 JSON:** [`chrome-156-observations.json`](./chrome-156-observations.json) · **화면:** [`chrome-156.png`](./chrome-156.png)

## 규범 기준

[CSS Grid Layout Module Level 1, 2025-03-26 CRD §8.3–8.3.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement)은 area 이름과 일치하지 않는 bare `<custom-ident>`를 occurrence `1`이 붙은 named line으로 처리하고, 요청한 이름의 선이 부족하면 implicit grid의 line을 그 이름이 있는 것으로 가정한다. 두 line placement가 있으면 start가 end보다 endward인 경우 둘을 swap하고, 같은 line이면 end placement를 제거한다. §7.5–7.6은 explicit grid 밖에 생성되는 implicit track과 `grid-auto-*` sizing을 정의한다.

## Chrome 관찰값

`itemFrame`은 각 grid root의 좌상단 기준 `[x, y, width, height]` CSS px다. track 문자열은 Chrome CSSOM `getComputedStyle(...).gridTemplateColumns/Rows` 결과다.

| Scene | Authored placement | Chrome tracks | Item frame |
| --- | --- | --- | --- |
| `columns-distinct` | `alpha / omega` | Columns `40px 40px 0px 30px`; Rows `20px` | `[80, 0, 30, 10]` |
| `columns-occurrences` | `2 alpha / 3 omega` | Columns `40px 40px 0px 0px 30px`; Rows `20px` | `[80, 0, 30, 10]` |
| `columns-same-name` | `alpha / alpha` | Columns `40px 40px 0px 30px`; Rows `20px` | `[80, 0, 30, 10]` |
| `columns-forward-adjacent` | `1 alpha / 2 omega` | Columns `40px 40px 0px 30px`; Rows `20px` | `[80, 0, 30, 10]` |
| `columns-reverse-adjacent` | `2 alpha / 1 omega` | Columns `40px 40px 0px 30px`; Rows `20px` | `[80, 0, 30, 10]` |
| `columns-multi-track` | `1 alpha / 3 omega` | Columns `40px 40px 0px 15px 15px`; Rows `20px` | `[80, 0, 30, 10]` |
| `rows-distinct` | `alpha / omega` | Columns `40px`; Rows `20px 20px 0px 25px` | `[0, 40, 20, 25]` |
| `rows-adjacent` | `1 alpha / 2 omega` | Columns `40px`; Rows `20px 20px 0px 25px` | `[0, 40, 20, 25]` |
| `fixed-control` | `alpha / omega`, `grid-auto-columns:30px` | Columns `40px 40px 30px 30px`; Rows `20px` | `[110, 0, 30, 10]` |

The multi-track case confirms that the fixed 30px item contribution distributes as 15px to each of its two auto tracks. The reversed pair matches §8.3.1 line swapping. The equal-line cases match end-line removal followed by the default one-track span. Rows and Columns produce corresponding results. The fixed-track control intentionally starts at x=110px, unlike the auto-track cases at x=80px.

This is a Chrome reference only. It does not establish Spinon behavior, platform surface-pixel parity, WPT conformance, or general CSS Grid support. The screenshot records layout placement only.

## SHA-256

| File | SHA-256 |
| --- | --- |
| Fixture | `506ccf7cd47dfcba2c7097c78e8b4919fe6d2f3cb18add6cea5069b0cef2cc3e` |
| Observation JSON | `85e72190c10b4f540d9c322a1d90ef9962ee1717b0f3dd6db4e994e12df71520` |
| Screenshot | `5c6fddcb06a2eb20069abd135bd0d59d33d0351a7f911986a6988ba35cd784e1` |
