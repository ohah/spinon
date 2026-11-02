# C11.1 Root HTML Grid fixed `grid-auto-rows` 사전 비교

## 기준과 범위

- 기준 문서는 [CSS Grid Layout Module Level 1, 2025-03-26 Candidate Recommendation Draft §§7.5–7.7, 8.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)다. 명세는 explicit grid 바깥에 배치된 item을 위해 implicit track을 만들고 `grid-auto-rows`가 그 크기를 지정한다고 정의한다. 기본 `grid-auto-flow:row`는 각 row를 차례로 채우며 필요하면 다음 row를 추가한다. 이 W3C 문서는 Candidate Recommendation Draft이므로 작업 중인 명세다.
- 브라우저 기준은 Agent Browser 0.31.1이 실행한 HeadlessChrome 156.0.0.0이다. viewport는 1280×633 CSS px, DPR 1이다. 이 장면은 style·layout geometry 비교이며 Spinon 출력이 아니다.
- HTML 입력은 [root-html-grid-implicit-rows-precomparison.html](../../../../tests/fixtures/css/c11/root-html-grid-implicit-rows-precomparison.html)이다. fixture SHA-256은 `8d08f685f2f714a591ce9705a7cb0de1c73819a1a98ec2f070b5c440cb2fb6cf`이다.
- Agent Browser 관측은 [chrome156-observation.json](./chrome156-observation.json), 보조 화면은 [chrome156-agent-browser.png](./chrome156-agent-browser.png)다. JSON SHA-256은 `ec4c91e5f3459db248fde376493b2980c1b1bd202af71997169b61a890dd5795`, 화면 SHA-256은 `f30b608f78ac4d2ff674340b0b14df4cfdea8c3e43ccb5081365ed191f829ede`다. [독립 expected JSON](./chrome156-expected.json)은 `c2e1445be00644d310e04dad91c72b294eaf66f3418809951edf7c6d40361482`다.

## 독립 기대값

두 explicit columns는 `40px 55px`, column gap은 `5px`라 content width가 `100px`이다. Explicit row는 `20px`; 각 positive implicit row는 `grid-auto-rows:15px`; row gap은 `5px`다. 따라서 implicit row가 없는 장면은 `20px`, 0px implicit row를 하나 만든 장면은 `20 + 5 + 0 = 25px`, 15px implicit row를 하나 만든 장면은 `20 + 5 + 15 = 40px`, 두 15px implicit row를 만든 장면은 `20 + 5 + 15 + 5 + 15 = 60px`다.

기본 row auto-placement는 DOM 순서대로 두 column을 채운 뒤 다음 row를 만든다. 각 item은 자기 column과 row 크기에 맞춘 명시적 고정 크기를 갖는다. 따라서 이 대조는 item stretch나 intrinsic contribution에 의존하지 않고 placement, implicit row track, gap과 frame을 분리한다. 독립 expected JSON은 root-local item 좌표를 기록하고 Chrome observation은 viewport 좌표를 기록한다. 대조할 때 각 item의 `root.x`/`root.y`를 빼며, 예를 들어 `one-row` 세 번째 item의 viewport `(16,69)`는 local `(0,25)`가 된다. 독립 local geometry는 아래와 같으며 이 정규화 뒤 Chrome observation과 CSS px 단위로 일치한다.

| 장면 | Columns / Rows | Root | Item local frames (DOM order) |
|---|---|---|---|
| `one-row` | `[40,55]` / `[20,15]` | `100×40` | `(0,0,40,20)`, `(45,0,55,20)`, `(0,25,40,15)` |
| `zero-row` | `[40,55]` / `[20,0]` | `100×25` | `(0,0,40,20)`, `(45,0,55,20)`, `(0,25,40,0)` |
| `two-rows` | `[40,55]` / `[20,15,15]` | `100×60` | `(0,0,40,20)`, `(45,0,55,20)`, `(0,25,40,15)`, `(45,25,55,15)`, `(0,45,40,15)`, `(45,45,55,15)` |

## 해석 한계

이 비교는 고정 HTML·CSS의 Chrome layout geometry 기준이다. Android/iOS app runtime, native surface pixels, glyph paint, 임의 CSS 문법 또는 WPT 전체 실행을 뜻하지 않는다. 구현은 계획 PR 승인 뒤 기본 Root HTML runtime 경로에 별도로 연결한다.
