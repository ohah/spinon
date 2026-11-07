# C11.4 Root HTML fit-content/min-content/auto implicit-row list 사전 비교 · 2026-11-08

고정 HTML fixture SHA-256은 `2daedf8cb1a755258e51b634c8907168ac27cef3106a1571b9c71cdbdf4292f0`이다. Agent Browser의 HeadlessChrome 157.0.0.0, macOS arm64, viewport 1280×633, DPR 1에서 세 장면을 관측했다. 화면은 [Agent Browser 캡처](./chrome157-agent-browser.png)이며 SHA-256은 `3fef3a98a803cfbd1602e4249a9d2f1f302242e8f2febc7f54e367b71b0a2aeb`이다.

## 비교 장면

고정 입력은 definite 40×100px Grid content box, explicit 40px column·10px row, row auto-flow, zero gaps/edges, visible overflow, `align-content:stretch`다. 각 장면에는 하나의 explicit row와 네 개의 auto-placed implicit-row item이 있다. Item은 fixed-height nested block을 가지며 min-height는 0이다. 자세한 Chrome frames는 [관측 JSON](./chrome-observation.json), 별도 track-sizing 알고리즘 계산은 [기대값 JSON](./expected-values.json)에 있다.

| 장면 | `grid-auto-rows` | 독립 기대 used rows (explicit 포함) |
| --- | --- | --- |
| 첫 순서 | `fit-content(20%) min-content auto` | `[10, 20, 34, 30, 6]px` |
| 순서 대조 | `fit-content(20%) auto min-content` | `[10, 20, 54, 10, 6]px` |
| overflow | `fit-content(120%) min-content auto` | `[10, 6, 34, 10, 40]px` |

첫 두 장면은 같은 item contributions를 유지하고 track list의 `auto`와 `min-content` 순서만 바꾼다. `auto` 최대 크기 트랙만 positive definite free space를 받아 각각 30px와 54px가 되고 min-content row는 34px와 10px로 유지된다. 세 번째 장면은 네 번째 implicit row가 fit-content member로 순환한다. 50px payload가 40px track 밖으로 그려져 100px Grid 아래 10px까지 보인다.

## 규범 모델

- [CSS Grid Layout Module Level 2 §7.6](https://www.w3.org/TR/css-grid-2/#auto-tracks): `grid-auto-rows`의 track-size list는 첫 implicit row부터 순환한다.
- [§12.3 Track Sizing Algorithm](https://www.w3.org/TR/css-grid-2/#algo-track-sizing): min-content/auto track contributions와 fit-content growth limit 및 사용 크기를 계산한다.
- [§12.8 Stretch auto Tracks](https://www.w3.org/TR/css-grid-2/#algo-stretch): `normal|stretch` content distribution은 남은 positive definite free space를 auto max-sizing-function track에 균등 배분한다.

독립 기대값을 Chrome frames에서 복사하지 않았다. 고정 fixture의 세 scene에서 grid 3개, item frame 15개, nested payload frame 12개를 비교했고 최대 차이는 0 CSS px다. 이 결과는 Chrome reference이며 Spinon runtime 결과, WPT 통과 또는 일반 CSS 지원을 뜻하지 않는다.

Finite px cap coverage uses a separate fixture and independent calculation: [`fit-content(20px) min-content auto`](./px-cap/README.md). At the same definite 100px block-size, Chrome 157 reports the same 20px cap and used row frames as the 20% scene.
