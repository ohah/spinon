# C11.4 `align-content:stretch` plan 검토 20개

범위: [`c11-root-html-multiple-auto-row-align-content-stretch.md`](./c11-root-html-multiple-auto-row-align-content-stretch.md)와 그 precomparison. 구현 코드는 다루지 않는다. 각 항목은 계획에서 서로 다른 구현·oracle·계약 실패 경로를 점검한다.

| # | 독립 실패 경로 | 결과 |
|---:|---|---|
| 1 | CSS Grid §11.8의 적용 조건을 잘못 인용 | PASS — `normal` 또는 `stretch`, positive definite free space, auto max tracks를 구분 |
| 2 | 새 작업이 앞선 #1231의 `normal` profile을 대체한다고 오해 | PASS — `normal` regression을 보존하고 computed `stretch`만 추가 |
| 3 | browser observation을 독립 기대값의 권위로 사용 | PASS — expected values를 규범 산술로 먼저 고정 |
| 4 | Chrome 버전이나 UA를 다른 실행으로 기록 | PASS — HeadlessChrome 157.0.0.0 고정 |
| 5 | viewport 또는 DPR 차이로 40px geometry가 달라짐 | PASS — 1280×633 CSS px, DPR 1을 기록 |
| 6 | Grid owner가 둘 이상이거나 implicit ancestor가 생김 | PASS — fixture에 표시 Grid 하나와 html/body zero margin만 사용 |
| 7 | definite sizing input이 달라짐 | PASS — 40×40 content box와 명시 track sizes 유지 |
| 8 | implicit row count를 item 수와 혼동 | PASS — 명시 row 하나, 직접 text 두 개가 만든 implicit auto rows 둘 |
| 9 | source order 또는 child grammar가 넓어짐 | PASS — fixed empty, `A`, `B` 순서와 cardinality를 고정 |
| 10 | text min-content contribution을 잘못 계산 | PASS — 두 10px/10px monospace 한 줄의 10px contribution 각 하나 |
| 11 | initial track base 합계를 잘못 계산 | PASS — `[10,10,10]px` |
| 12 | remaining free space 산술 오류 | PASS — definite 40px minus 30px bases gives 10px |
| 13 | stretch share를 모든 세 track에 나눔 | PASS — implicit auto max tracks 두 개에 각 +5px |
| 14 | normal control과 stretch positive를 같다고 미검증 | PASS — same exact rows, frames, lines를 control로 명시 |
| 15 | `start` control이 track stretch를 허용 | PASS — start control은 `[10,10,10]px` 유지 |
| 16 | explicit authored syntax와 computed CSS value를 혼동 | PASS — gate는 computed longhand value만 다루고 shorthand 별도 지원을 주장하지 않음 |
| 17 | `align-items`/item alignment까지 변경 | PASS — Grid와 items의 다른 alignment는 normal로 고정 |
| 18 | unsupported style rejection 뒤 partial scene을 publish | PASS — atomic rejection와 same-session recovery를 계획에 포함 |
| 19 | 새 public API, parser, FFI 또는 product selector 추가 | PASS — 기존 private profile와 opt-in fixture boundary 재사용 |
| 20 | bounded candidate를 C11.4/C11 완료로 게시 | PASS — 체크박스, `78/214`, contract 0.1.0 유지 |

계획 범위와 입력은 구현 전 비교 단계에서 분리됐다. 구현 시작은 계획 PR 병합 뒤에만 한다.
