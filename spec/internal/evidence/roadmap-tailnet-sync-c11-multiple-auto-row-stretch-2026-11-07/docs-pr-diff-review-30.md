# C11.4 두 implicit auto 행 후속 문서 PR diff 검토 30개

범위: `spec/STATUS.md`, 내부 계약 0059, C11·C11.4 계획 문서, public `spec/public/roadmap.html`, 그리고 Tailnet 제공 evidence. 각 항목은 이번 변경에서 독립된 상태·수치·범위 오표기 경로를 확인한다.

| # | 독립 실패 경로 | 결과 |
|---:|---|---|
| 1 | 계획 PR #1230을 구현 PR로 표기 | PASS — 기능 구현은 #1231로 표기 |
| 2 | PR #1231을 아직 열리지 않은 후보로 남김 | PASS — Rebase and merge 완료로 표기 |
| 3 | merge timestamp를 로컬 시간과 혼동 | PASS — GitHub UTC timestamp 사용 |
| 4 | merge SHA를 이전 candidate SHA로 대체 | PASS — main merge SHA `5c79d19619617e7424aaeabbe884579ac383d130` 사용 |
| 5 | official C11.4 체크를 완료로 바꿈 | PASS — 체크박스 미완료 유지 |
| 6 | parent C11 체크를 완료로 바꿈 | PASS — 체크박스 미완료 유지 |
| 7 | 공식 완료 수를 78/214에서 증가 | PASS — STATUS와 public roadmap 모두 78/214 유지 |
| 8 | 내부 계약 버전을 0.1.0에서 변경 | PASS — 숫자 버전 고정 |
| 9 | 제한 Root HTML profile을 일반 author CSS 지원으로 확대 해석 | PASS — 고정된 `RuntimeRootHtmlTextPaintV1` 경계 명시 |
| 10 | grid owner 또는 item 수를 잘못 기록 | PASS — Grid 하나, direct item 세 개 |
| 11 | explicit row와 implicit rows를 혼동 | PASS — explicit row 1, implicit auto rows 2 |
| 12 | 각 text minimum contribution을 잘못 기록 | PASS — implicit row마다 10px |
| 13 | 초기 track base 값을 잘못 계산 | PASS — `[10,10,10]px` |
| 14 | definite free space를 잘못 계산 | PASS — 40px에서 base 합 30px을 빼 10px |
| 15 | 균등 분배량을 잘못 계산 | PASS — implicit auto rows 각 +5px |
| 16 | used row order/value를 뒤집음 | PASS — `[10,15,15]px` |
| 17 | item frame geometry를 과장/오표기 | PASS — 세 frame을 각각 명시 |
| 18 | A/B text source order를 바꿈 | PASS — source order와 각 item identity trace를 명시 |
| 19 | acceptance profile 밖 alignment를 허용 | PASS — `align-content:normal`만 positive acceptance |
| 20 | 기존 single-auto-row 동작을 삭제한 것처럼 표기 | PASS — 회귀 profile 보존을 명시 |
| 21 | rejection이 atomic하지 않다는 계약 누락 | PASS — unsupported shape publication 전 거부 기록 |
| 22 | 새 public API·parser·FFI가 추가됐다고 잘못 주장 | PASS — 기존 내부 경계 재사용, API/FFI 추가 없음 |
| 23 | focused test 수를 전체 crate 성공으로 확대 | PASS — style/runtime focused test 집합만 기록 |
| 24 | Clippy 범위를 workspace 전체로 확대 | PASS — changed-crate all-target Clippy로 한정 |
| 25 | release fixture marker 결과를 뒤집음 | PASS — default release artifact에서 marker 제외 |
| 26 | Android AVD 결과를 실기기/hardware GPU 증거로 부름 | PASS — API 37 ARM64 AVD와 visual smoke라고 한정 |
| 27 | iOS Simulator screenshot을 compositor pixel parity로 부름 | PASS — iOS 26.2 Simulator와 pixel 비교 한계 명시 |
| 28 | WPT 또는 GitHub CI 통과를 주장 | PASS — WPT 미확인, PR check rollup 부재 표기 |
| 29 | roadmap 카드 순서가 merge 시간과 다르거나 source와 Tailnet 제공본 내용이 다름 | PASS — DOM 순서는 `#1224 → #1228 → #1231`; source/target/local/Tailnet bytes·hash가 일치 |
| 30 | Agent Browser 확인 뒤 session/process 종료 누락 | PASS — named sessions 종료, `close --all`, active session 0개 및 Agent Browser process 0개 확인 |

이 변경은 #1231의 bounded slice를 공식 완료 항목으로 승격하지 않는다. Tailnet copy hash, HTTP 응답, 화면 캡처와 browser lifecycle 값은 같은 evidence 디렉터리의 `README.md`와 JSON 기록에서 확인한다.
