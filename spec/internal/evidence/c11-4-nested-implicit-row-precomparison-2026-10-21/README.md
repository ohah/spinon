# Nested Grid positive implicit row 사전 비교

## 범위

기존 [nested Grid intrinsic 계획](../../../../plan/c11-4-nested-grid-intrinsic.md)은 nested implicit track 확장을 명시적으로 제외한다. 이 fixture는 그 미포함 교차 범위를 분리해, nested Grid의 첫 explicit row 뒤에 positive implicit rows 두 개가 생기고 그 block-axis contribution이 outer Grid에 전달되는 두 장면을 고정한다. Spinon runtime 및 Simulator 검증은 아직 하지 않았다.

## 독립 계산

기준 문서는 [CSS Grid Layout Module Level 1, 2025 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/) §7.5–7.6, §11.3–11.5.1이다. 명시된 nested row는 하나라서 `grid-row: 3 / 4` 배치는 두 개의 positive implicit rows를 만든다. §7.6의 auto-track list 반복에 따라 두 번째 row에는 `min-content`, 세 번째 row에는 `max-content`가 적용된다. 고정-height 8px block과 natural-size 24×16px image의 track 기대값은 `[8,8,16]px`이며, 4px 내부 gap 두 개를 더한 nested contribution은 40px이다.

Outer Grid도 `min-content min-content` 두 행과 4px gap을 가진다. nested item이 두 행을 span하면 §11.5.1에 따라 gap을 제외한 36px을 두 행에 나누어 `[18,18]px`, root height 40px이 된다. 첫 행 하나만 점유하면 `[40,0]px`, root height 44px이다. 이 계산은 아래 browser observation과 별도 파일에 기록했다.

## Chrome 156 관찰

Agent Browser 0.31.1이 연 `HeadlessChrome/156.0.0.0`에서 fixture를 열고 이미지 decode 이후 DOM frame과 used `gridTemplateRows`를 평가했다. viewport는 1280×633 CSS px, DPR 1이다. 두 PNG 모두 `complete=true`, natural size 24×16px였다. Chrome 관찰은 두 scene에서 내부 rows `[8,8,16]`, nested height 40px, outer rows `[18,18]`/`[40,0]`, root heights 40/44px를 반환해 독립 계산과 일치했다.

| scene | Chrome outer rows | Chrome root / nested height | inner rows |
| --- | --- | --- | --- |
| `spanTwo` | `[18,18]px` | `40px / 40px` | `[8,8,16]px` |
| `oneRow` | `[40,0]px` | `44px / 40px` | `[8,8,16]px` |

- Oracle 입력: [HTML fixture](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-nested-implicit-rows-precomparison.html)
- 독립 계산: [spec-derived-expectations.json](./spec-derived-expectations.json)
- Browser 관찰: [chrome156-observation.json](./chrome156-observation.json)
- Browser screenshot: [chrome156-nested-implicit-rows.png](./chrome156-nested-implicit-rows.png)

## 재현 hash와 한계

- Fixture SHA-256: `c6ac7bc85820e023871c5534f9dcfb1c51c7f9e18a12826e9933262c9a2fed4f`
- PNG SHA-256: `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`
- Screenshot SHA-256: `9cf5b0d1dcae9742aba1ffb2e983cbcb9bcd2a105725ea96e616aca4d54f820f`

이 결과는 Chrome 한 버전의 CSSOM·geometry observation이다. 앱 렌더링, Rust layout, WPT, 다른 browser, border/overflow surface 또는 일반 CSS Grid 지원을 입증하지 않는다. fixture에는 border 밖으로 넘치는 항목이 없다. Agent Browser session을 종료했고 종료 후 `agent-browser`/`HeadlessChrome` 프로세스는 남아 있지 않았다.
