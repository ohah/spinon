# C11.4 양 축 auto-repeat + template areas 사전 비교

**상태:** 구현 전 Chrome 156 oracle와 규범 독립 기대값이다. Spinon 실행 결과가 아니다.

## 재현 환경

- 고정 기준: W3C CSS Grid Layout Module Level 1, 2025-03-26 Candidate Recommendation Draft, §7.2.3.2, §7.3, §7.5–§7.6, §8.3–§8.4.
- Chrome: Google Chrome `156.0.8078.48`, Agent Browser `0.31.1`, macOS arm64, viewport `1280×633`, DPR 1.
- 입력: [`grid-auto-repeat-template-areas-two-axis-c114.html`](../../../../tests/fixtures/css/c11/grid-auto-repeat-template-areas-two-axis-c114.html).
- 원 관찰: [`chrome-156-observation.json`](./chrome-156-observation.json). 독립 기대: [`expected-values.json`](./expected-values.json).
- 화면: [`chrome-156-two-axis-template-areas.png`](./chrome-156-two-axis-template-areas.png).
- Agent Browser session 종료 후 daemon 상태와 프로세스 목록은 [`agent-browser-close.json`](./agent-browser-close.json) 및 [`agent-browser-doctor-close.log`](./agent-browser-doctor-close.log)에 기록했다. 남은 Agent Browser 프로세스는 0개다.

## 독립 계산

반복 track은 양 축에서 고정 `20px`, gutter는 `4px`다. definite size `100px`이면 4개 track이 `92px`를 차지하고 5개는 `116px`이므로 4회 반복한다. `76px`이면 3개가 `68px`, 4개가 `92px`이므로 3회다. `40px`이면 1개만 들어간다. 각 결과는 CSS Grid §7.2.3.2의 definite content-box 크기와 gutter를 고려한 최대 양의 반복 수에서 도출했다.

§7.3에 따라 명시 area dimension이 expanded template보다 큰 축은 그 dimension까지 explicit grid track을 가진다. template에서 크기를 지정하지 않은 추가 track은 `grid-auto-columns:16px` 또는 `grid-auto-rows:12px`를 사용한다. 따라서 `area-added-columns`와 `area-added-rows`는 각각 `[20,16,16]` / `[20,20,20]` 및 `[20,20,20]` / `[20,12,12]`이고, 양 축 확장 control은 `[20,16,16]` / `[20,12,12]`이다.

§7.2.3.2에서 `auto-fit`은 배치 뒤 빈 반복 track을 0으로 접고 양쪽 gutter도 접는다. Named area 정의만으로 track이 occupied 되지 않는다. 실제 grid item이 area를 가로질러 놓인 경우에는 그 item과 겹치는 반복 track만 유지한다. area aliases는 각 축에서 `hero-start`/`hero-end` named line으로 작동한다.

## Chrome 대조

9개 scene의 `getComputedStyle().gridTemplateColumns/Rows`, grid 크기와 scroll extent, grid 기준 item frame을 Chrome에서 읽었다. 정규화한 값은 [`expected-values.json`](./expected-values.json)과 모두 일치한다. `area-added-both`는 40×40 grid 바깥으로 item이 x=44..52, y=40..48까지 넘친다. 이는 native surface 검증 전 CSS oracle의 overflow 입력이며, border stroke나 플랫폼 paint를 증명하지 않는다.

| Scene | Columns | Rows | Item frame `[x,y,w,h]` |
| --- | --- | --- | --- |
| `area-short` | `[20,20,20,20]` | `[20,20,20]` | `[24,0,8,8]` |
| `area-added-columns` | `[20,16,16]` | `[20,20,20]` | `[44,0,8,8]` |
| `area-added-rows` | `[20,20,20]` | `[20,12,12]` | `[24,40,8,8]` |
| `area-added-both` | `[20,16,16]` | `[20,12,12]` | `[44,40,8,8]` |
| `fill-fit` | `[20,20,20,20]` | `[0,20,0]` | `[24,0,8,8]` |
| `fit-fill` | `[0,20,0,0]` | `[20,20,20]` | `[0,24,8,8]` |
| `fit-fit-empty` | `[0,0,0,0]` | `[0,0,0]` | no item |
| `fit-fit-span` | `[20,20,0,0]` | `[20,20,0]` | `[0,0,44,44]` |
| `area-longhand` | `[20,20,20,20]` | `[20,20,20]` | `[0,24,8,8]` |

## 판정 한계

Chrome 관찰은 독립 규범 계산을 대신하지 않는다. 이 사전 비교는 Chromium 기준의 동작만 보이며, Spinon WPT 적합성이나 Android/iOS runtime 동작을 입증하지 않는다. Native follow-up의 `area-added-both` 표본은 grid origin 기준 `[38,38]`의 `#e5e7eb`, root 밖 item 내부 `[47,43]`의 `#e5484d`, item 밖 `[53,43]`/`[47,49]`의 host-surface clear를 읽는다. 마지막 두 점의 clear color는 Chrome page background에서 가져오지 않고 각 native capture의 원본 surface로 확인한다. auto-repeat 없이 areas를 쓰는 경로, 여러 repeat per axis, intrinsic/fractional repeated sizes, RTL/writing mode, dynamic CSS input은 이 계획의 대상이 아니다.
