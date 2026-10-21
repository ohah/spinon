# C11.4 동시 cyclic percentage 다중 track 사전 비교

**날짜:** 2026-10-22

**상태:** 계획 단계. Chrome 156 보조 oracle과 CSS Grid 2025 CRD 기반 독립 계산만 기록한다. Spinon runtime 또는 Simulator 실행 결과가 아니다.

## 입력과 재현 환경

- 고정 입력: `tests/fixtures/css/c11/grid-cyclic-percent-two-axis-multitrack-c114-precomparison.html`.
- 원본 Chrome 관찰: `chrome156-observation.json`.
- Agent Browser `0.31.1`, macOS arm64, `HeadlessChrome/156.0.0.0`, viewport `1280×633` CSS px, DPR 1.
- `document.fonts.status`는 `loaded`, `document.fonts.check('10px Ahem')`은 `true`였다.
- [Chrome 화면](./chrome156-two-axis-multitrack.png)은 두 축 cyclic scene, 축별 controls, definite control의 green third-cell item이 Grid root 우측·하단으로 넘치는 모습을 보인다.
- 관찰 당시 checkout source revision: `7db827461f2b2722034b9a84d9a7710129bcb356`.

## 기대값과 관찰값

기대값은 브라우저 결과를 복사하지 않고 Ahem advance와 Grid intrinsic/final sizing 두 단계를 계산했다. `aa`의 intrinsic width는 20px, line-height는 10px이다. intrinsic pass에서 cyclic percentage track은 `auto`처럼 기여하므로 `max-content` 축의 root basis는 40×20px이다. final pass의 inline percentage tracks는 `[20,10]px`, block percentage tracks는 `[10,5]px`이 된다. 마지막 fixed track은 각각 20px과 10px이다.

| Scene | 독립 기대 Columns / Rows | Chrome 156 CSSOM | Root / text item | overflow item, root 기준 |
| --- | --- | --- | --- | --- |
| `cyclic-percent-multitrack-both` | `[20,10,20]` / `[10,5,10]` | `20px 10px 20px` / `10px 5px 10px` | `40×20` / `20×10px` | `(30,15) 20×10px` |
| `cyclic-percent-multitrack-columns` | `[20,10,20]` / `[10,5,5]` | `20px 10px 20px` / `10px 5px 5px` | `40×20` / `20×10px` | `(30,15) 20×10px` |
| `cyclic-percent-multitrack-rows` | `[20,10,10]` / `[10,5,10]` | `20px 10px 10px` / `10px 5px 10px` | `40×20` / `20×10px` | `(30,15) 20×10px` |
| `definite-percent-multitrack-both` | `[20,10,20]` / `[10,5,10]` | `20px 10px 20px` / `10px 5px 10px` | `40×20` / `20×10px` | `(30,15) 20×10px` |

For the both-cyclic scene, used tracks total 50×25px while the root remains 40×20px. The third-cell item extends 10px right and 5px below the root. Its background is `rgb(32, 191, 107)`; a later Simulator matrix will sample a point inside that overflow and outside the root.

## 한계와 보존된 hash

- 한 버전의 Chrome 관찰이며 Chrome 155 baseline, 다른 UA 비교 또는 CSS Grid 전반의 적합성으로 확대하지 않는다.
- 이 precomparison은 Spinon layout, app route, Android/iOS, surface pixel, WPT 또는 fixture artifact boundary를 검증하지 않는다.
- `chrome156-observation.json`: `0b0ab6454c5377e228a07d633f4c9140417aff826a3b660ea42bf8b746bf81ba`
- `chrome156-two-axis-multitrack.png`: `304a1d028df65f18e6dce6333ad976401ac22a542856b3a2d5a2c81bc81eeda7`
- Input HTML: `223ad70341030d41ec705706914a80bcf52ea6b96bbd5bc797131eea51bbcff3`
- Agent Browser `close` returned success; the post-close diagnostic reported `browserLaunched:false` and `pageCount:0`. A process check after the CLI exited found no Agent Browser or HeadlessChrome process. No physical device was connected.
- Separate plan review: [plan PR diff, 20 failure perspectives](./plan-pr-diff-review-20.md).
