# C11.4 행 축 overlapping-span 단계 사전 비교

이 자료는 `Rows` 축의 span별 intrinsic sizing stage를 테스트로 고정하기 위한 사전 비교다. CSS Grid의 규범 계산과 Chrome 156의 최종 used rows 관찰을 서로 분리한다. Chrome은 내부 stage snapshot을 제공하지 않으므로 stage 기대값은 CSS 규범의 §11.5.1/§11.6 산술에서 직접 도출한다.

## 규범 입력

네 장면은 각각 `220×220px` Grid, `grid-template-columns: 220px`, `grid-template-rows: repeat(4, minmax(0px, max-content))`, 0px row/column gap, `align-content:start`를 쓴다. Items는 빈 HTML `div`로 만들고 220px 너비, 고정 높이, 명시 `grid-row`만 둔다. 이는 기존 Rust `HostDocument` builder의 empty `div` 구조와 맞춘다.

- A: 높이 80px, rows 1–2 (`grid-row: 1 / 3`).
- B: 높이 120px, rows 2–3 (`grid-row: 2 / 4`). A와 B는 row 2를 겹쳐 span-2 동시 batch에서 겹치는 track의 planned increase를 검증한다.
- C: 높이 200px, rows 1–3 (`grid-row: 1 / 4`). A/B의 span-2 batch 다음 span-3 batch가 앞 세 row의 현재 growth limit을 사용하는지 확인한다.
- Controls는 A/B source order forward/reversed, A/B/C forward/reversed 네 가지다. 배치는 유지하고 DOM 순서만 바꾼다.

이 profile은 네 `minmax(0px, max-content)` row와 definite 220px block size만 다룬다. CSS track algorithm 전체, 다른 sizing function, gap, automatic minimum, flex, intrinsic text/image, implicit rows 또는 임의 placement를 일반화하지 않는다.

## 독립 규범 산술

CSS Grid Level 1, 2025 CRD §11.5는 intrinsic sizing에서 span이 작은 item group부터 순서대로 처리한다. §11.5.1은 각 span group 안에서 item별 planned increase를 구해 track마다 최댓값을 적용하고, §11.6은 양의 남은 공간을 growth limit에 닿을 때까지 base size에 균등 배분한다([§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content), [§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#extra-space), [§11.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grow-tracks)).

- Initialize: base `[0,0,0,0]`, growth limits `[∞,∞,∞,∞]`.
- A/B span-2 group: A contributes `80/2 = 40px` to rows 1–2; B contributes `120/2 = 60px` to rows 2–3. The overlap uses `max(40,60)`, so the group ends at growth limits `[40,60,60,∞]` while base sizes remain zero.
- A/B/C span-3 group: row 4 is outside C's span; rows 1–3 currently total 160px. C's 200px contribution leaves 40px, distributed as `40/3` to each affected row. Growth limits become `[160/3,220/3,220/3,∞]`.
- IntrinsicSizingComplete closes the untouched fourth row's unbounded growth limit at its zero base size. MaximizeTracks reaches `[160/3,220/3,220/3,0]` and leaves 20px of the definite block size unused. The span-2-only control reaches `[40,60,60,0]` and leaves 60px.

[`derive_expected.py`](./derive_expected.py) uses hardcoded constants for this fixed input, exact `Fraction` arithmetic, and span groups to check that forward/reversed source-order controls have identical normative snapshots. [`expected.json`](./expected.json) is its output. The script does not parse the HTML fixture or use Spinon's trace as input, so changing the fixture does not automatically update its modeled inputs.

## Chrome 156 final geometry observation

The hashed [HTML fixture](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-multispan-rows.html) was opened in Agent Browser. Raw DOM/style measurements and browser metadata are in [`chrome156-observation.json`](./chrome156-observation.json); the rendered page is [`chrome156-row-span-precomparison.png`](./chrome156-row-span-precomparison.png).

Chrome 156 serializes `gridTemplateRows` as `40px 60px 60px 0px` for both span-2 controls and `53.3281px 73.3281px 73.3438px 0px` for both span-2/3 controls. CSSOM prints four decimal places; these values are within 1/64 CSS px of the normative ideal `160/3, 220/3, 220/3, 0px`. The `span2-before-span3` B item DOMRect begins at local y=`53.328125px` and retains its fixed `120px` height. Reversed DOM order preserves row sizes and each item frame.

Observation environment: `HeadlessChrome/156.0.0.0`, macOS agent-browser host, viewport `1280×633`, DPR 1. The screenshot is Chrome reference output, not Spinon, Android, iOS, or WPT evidence. Browser session `spinon-c114-row-span-20261027` was closed after capture; no browser process for that session remained.

## Hashes

SHA-256 values for the fixture, observation, screenshot, exact-arithmetic script, and generated JSON are recorded in [`SHA256SUMS`](./SHA256SUMS). The pinned normative source is CSS Grid Level 1, 2025 CRD, published 2025-03-26.
