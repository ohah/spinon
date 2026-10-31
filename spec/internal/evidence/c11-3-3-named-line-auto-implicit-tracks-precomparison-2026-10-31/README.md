# C11.3.3 named-line implicit auto-track 사전 비교

이 자료는 template areas가 없는 positive named-line placement에서 생성된 trailing implicit track에 `grid-auto-columns:auto` / `grid-auto-rows:auto`를 사용할 수 있는지 확인하기 위한 기준 입력이다. Spinon 구현 결과는 포함하지 않는다.

- 고정 Chrome 입력: [fixture](../../../../tests/fixtures/css/c11/c1133-named-line-auto-implicit-tracks.html)
- Chrome 관찰값: [observation.json](observation.json)
- 규범 기반 기대값: [expected-values.json](expected-values.json)
- Agent Browser 화면: [chrome-156-agent-browser.png](chrome-156-agent-browser.png)
- 작업 계획: [C11.3.3 계획](../../../../plan/c11-3-3-named-line-auto-implicit-tracks.md)
- 기준 revision: `e34da9554f50653f62096e7cc8c9ffd4afea8eca`

## 규범 기준

[CSS Grid Layout Module Level 1, 2025 Candidate Recommendation Draft §7.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#implicit-grids)는 item이 explicit grid 바깥에 배치되면 implicit track이 생성되고 `grid-auto-*`가 그 크기를 정한다고 설명한다. [§7.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-tracks)는 template track size가 지정되지 않은 implicit track의 sizing을 이 속성에 맡긴다. [§8.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement)는 이름만 지정한 line을 occurrence 1로 처리하고, 충분한 named line이 없으면 implicit grid line을 해당 이름이 있는 것으로 간주해 찾도록 한다.

따라서 긍정적 trailing named-line placement로 생긴 implicit track은 `auto` track sizing의 대상이다. leading/signed 검색 방향, negative extent, 자동 배치까지 이 근거가 허용하는 것은 아니다.

## Chrome 156 관찰

Agent Browser가 연 로컬 fixture에서 UA `HeadlessChrome/156.0.0.0`, viewport 1280×633 CSS px, DPR 1을 사용했다.

| Scene | Computed tracks | Root-relative item frame |
| --- | --- | --- |
| `columns` | columns `40px 40px 0px 30px`; row `20px` | `[80, 0, 30, 10]` |
| `fixed-columns` control | columns `40px 40px 30px 30px`; row `20px` | `[110, 0, 30, 10]` |
| `rows` | column `40px`; rows `20px 0px 25px` | `[0, 20, 20, 25]` |

두 auto scene 모두 content가 있는 track의 크기를 child의 고정 30px/25px 기여와 일치시켰다. 빈 앞 track은 0px이다. Rows item은 20px 높이 root 아래로 5px 넘치고, columns item은 80px root 오른쪽 경계에서 시작한다. 이 장면은 root overflow를 geometry와 screenshot으로 드러낸다.

이 값은 Chrome observation과 CSS Grid 규범상 기대를 연결한 제한 입력이다. Spinon 결과, 플랫폼 surface pixel, WPT 실행 또는 일반 CSS 지원을 입증하지 않는다.
