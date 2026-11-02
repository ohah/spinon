# C11 Root HTML fixed-px implicit row list 계획 실패 관점 검토 20개

대상: [계획](./c11-root-html-grid-auto-row-list.md), Chrome 156 사전 비교 및 repository fixture. 이 검토는 구현 성공을 주장하지 않는다.

1. **C11 상위 완료로 과장될 위험 — PASS.** 계획은 bounded Root HTML profile만 추가하고 C11.1/C11 checkbox와 공식 `72/213`을 유지하도록 했다.
2. **이미 존재하는 C11.3.3 track-list 구현 중복 — PASS.** 계획은 해당 구현을 중복 작성하지 않고 기본 `RuntimeRootHtmlTextPaintV1` route의 explicit adapter gap만 닫는다. `implicit_rows` profile detection도 scalar뿐 아니라 유효한 px-only list를 인식하도록 적었다.
3. **목록 순서의 기준(track 0 vs 첫 implicit row)이 모호함 — PASS.** cycle index는 explicit rows 바로 다음 첫 implicit row를 0으로 정의했다.
4. **일반 `grid-auto-rows` grammar 전체로 범위가 확장될 위험 — PASS.** fixed direct px 값만 허용하고 percentage/fr/intrinsic/calc/auto-repeat은 거부한다.
5. **음수·NaN·Infinity가 Taffy에 도달할 위험 — PASS.** 각 항목에 finite/non-negative 검사와 거부 조건을 요구한다.
6. **빈 TrackList modulo-by-zero — PASS.** computed-style와 layout core 양쪽에서 non-empty 조건을 명시했다.
7. **단일 scalar가 바뀔 위험 — PASS.** `LengthPx` variant와 기존 동일-track 확장을 보존한다.
8. **single-item TrackList 표현이 scalar와 상충할 위험 — PASS.** 구현은 list variant가 전달될 때 non-empty px-only list로만 허용하며 scalar 처리와 독립시킨다.
9. **0px track이 거부되거나 일반 auto track으로 오인될 위험 — PASS.** 0을 유효 경계값으로 두고 별도 regression을 계획했다.
10. **explicit row를 pattern cycle에 포함할 위험 — PASS.** 예제의 explicit `[20px]`과 implicit `[15,25,15]`를 분리해 검산했다.
11. **row-gap이 track base로 합산될 위험 — PASS.** 3개의 5px gutter를 row-start 계산에서 별도로 더하고 trace에서도 분리하도록 했다.
12. **align-content가 남은 공간을 row pattern에 분배할 위험 — PASS.** definite 100px root의 default `normal`을 사용하고 tracks는 모두 fixed라 남은 10px가 auto row pattern에 배분되지 않음을 명시했다.
13. **item size를 track size로 혼동할 위험 — PASS.** 각 DOM row의 item 높이를 expected used row에 맞춰 고정하고 row track 자체도 관측한다.
14. **implicit track 확장 자원 한도 초과 — PASS.** track quota, checked arithmetic, reserve failure와 publication atomicity를 계획에 포함했다.
15. **hidden item이 row 개수에 포함될 위험 — PASS.** `display:none` occupancy regression을 명시했다.
16. **CSS `order`가 DOM 순서를 뒤바꿀 위험 — PASS.** ID/class를 입력 선택 기준으로 쓰지 않고 DOM source order를 고정·검증한다.
17. **template areas와 implicit rows의 미지원 경계가 흐려질 위험 — PASS.** 해당 조합은 계속 거부한다고 명시했다.
18. **기존 explicit fr/percentage + scalar implicit row 회귀 — PASS.** PR #1027 이후 허용 조합을 regression으로 유지하도록 요구했다.
19. **invalid computed value와 CSS syntax-invalid cascade의 실패 계층 혼동 — PASS.** 둘을 다른 거부 경계로 비교한다.
20. **simulator screenshot/draw를 surface pixel 또는 전체 CSS 적합성으로 과장할 위험 — PASS.** draw/present, PixelCopy/Metal, physical GPU와 WPT 범위를 분리해 기록하도록 했다.

**판정:** 계획 후보로 진행 가능. 구현 전 oracle/fixture, fixed-px admission, failure atomicity와 legacy combinations의 경계가 정해졌다. 구현 검토 및 PR diff 검토는 별도다.
