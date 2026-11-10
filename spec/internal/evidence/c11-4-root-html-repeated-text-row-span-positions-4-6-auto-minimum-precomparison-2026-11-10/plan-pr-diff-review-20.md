# C11.4 positions 4–6 automatic minimum · 계획 PR diff 검토 20개

2026-11-10에 계획 PR의 실제 변경분(계획, fixture, Chrome 수집기, 관측 NDJSON, 검증기, 상태·명세 동기화, 그림)을 계획 실패 관점 검토와 별도로 읽었다.

| # | 별도 PR diff 관점 | 검토 |
| ---: | --- | --- |
| 1 | 변경 파일이 기능 계획과 oracle 근거 안에 머무는지 | PASS — 제품 Rust/native 구현은 없고 계획·상태·fixture·비교 evidence만 수정한다. |
| 2 | `spec/STATUS.md` 공식 완료 표시가 변하는지 | PASS — C11.4/C11 checkbox는 그대로 미완료이며 완료 집계 `78/214`를 그대로 적는다. |
| 3 | 내부 계약 숫자 버전이 초안 변경만으로 올라가는지 | PASS — `0059`의 `0.1.0`을 유지한다. |
| 4 | 이전 단계가 현재 후보처럼 남는지 | PASS — #1306, #1311, #1322, #1326의 완료 상태를 parent plan/status/internal contract에 반영하고 새 후보를 최상단에 둔다. |
| 5 | 후보 링크가 구현 완료를 암시하는지 | PASS — 계획, 내부 계약, conformance entry 모두 plan/oracle로 한정하고 제품 미구현을 명시한다. |
| 6 | 범위 밖 auto-placement 또는 두-fit profile을 혼합하는지 | PASS — single-fit row-span positions 4–6만 후보로 선택했다. |
| 7 | Chrome 수집 fixture가 프로젝트 기본 앱 입력으로 오인되는지 | PASS — 별도 `tests/fixtures/css/c11` HTML이며 앱 runtime 결과로 표시하지 않는다. |
| 8 | 브라우저 관측치와 독립 검증 결과가 같은 파일을 가리키는지 | PASS — README가 NDJSON, capture summary, Python verifier와 JSON output을 각각 연결한다. |
| 9 | fixture·수집기 SHA가 실행 결과와 맞는지 | PASS — independent JSON에는 fixture/capture hash가 있고 `SHA256SUMS`가 해당 파일들을 다시 고정한다. |
| 10 | scene 조합 수와 authored dimension 곱이 어긋나는지 | PASS — 3×30×6×2×4×2가 8,640이며 baseline/candidate 각각 4,320이다. |
| 11 | capture index가 빠지거나 중복되는지 | PASS — 독립 검증기는 0–8,639 index의 연속성, scene key별 uniqueness와 4,320 paired keys를 검사한다. |
| 12 | typed cap, authored position 또는 tail order가 이동하는지 | PASS — computed `grid-auto-rows`를 기대 목록과 대조하고 `0px`/`0%`를 보존한다. |
| 13 | screenshot이 비교 pair 외 scene을 보여주는지 | PASS — sample selector는 고정 key에서 `0px`/`auto` pair를 찾아 양쪽 label과 grid를 출력한다. |
| 14 | screenshot panel label이 서로 겹치는지 | PASS — 각 panel 폭을 150px, gap을 40px로 지정해 label과 시각적 Grid 사이를 구분한다. |
| 15 | matrix eligibility count가 capture와 verifier에서 다르게 계산되는지 | PASS — CSS line → zero-based track 변환과 explicit row 제외를 수정해 양쪽 모두 12,240 occurrence다. |
| 16 | text fixture의 line shape가 변해 sizing evidence가 흔들리는지 | PASS — direct ASCII A–E payload와 expected line count 6/5/4/4/3를 독립 검증한다. |
| 17 | session lifecycle evidence가 browser 미종료 상태를 감추는지 | PASS — capture `finally`에서 닫고 전용 session name이 이후 session list에 없다. 기존 unrelated session은 보존한다. |
| 18 | README가 mobile/native result 또는 WPT pass로 넓혀 말하는지 | PASS — browser-only oracle이며 Android/iOS와 구현 gate는 후속이라고 한정한다. |
| 19 | PR 변경이 다른 미추적 프로젝트 evidence를 stage하도록 유도하는지 | PASS — manifest와 범위 링크는 이 후보 전용 파일만 명시하며 stage 목록도 explicit path만 사용한다. |
| 20 | 문서의 relative URL이 현재 branch에서 해결 가능한지 | PASS — plan·status·0059·0001·README 링크 대상과 fixture 경로를 검증했다. |

최종 계획 diff 판정: blockers 없음. 최초 독립 검증 초안에서 eligible track 시작 인덱스가 한 칸 어긋난 점을 찾아 수정했고, 최종 독립 결과는 browser 수집기와 일치한다. 이 review는 계획 PR 변경분 전용이며 구현/runtime 및 구현 PR 검토로 재사용하지 않는다.
