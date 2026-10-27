# C11.4 `min-height` / `fit-content()` 계획 PR diff · 적대적 검토 20개

이 검토는 계획 내용 검토 및 이후 구현/runtime 검토와 별개로, 이 문서까지 포함한 계획 PR diff의 누락·과장·배포 경계를 확인한다.

1. **계획 PR에 제품 코드가 섞일 수 있음 — PASS.** 변경은 계획, 계획 index, expected JSON, 사전 비교 README 및 review 기록뿐이며 Rust·Java·Swift·FFI source는 없다.
2. **공식 상태 대장이 계획과 함께 완료 처리될 수 있음 — PASS.** `spec/STATUS.md`와 공개 API 목록은 변경하지 않았고 C11.4 checkbox 및 `72/213` 유지 문구만 계획에 둔다.
3. **새 집중 계획이 C11.4 parent index에서 빠질 수 있음 — PASS.** `plan/c11-4-grid-track-sizing.md`의 최신 기록에서 plan·precomparison·20-review를 연결한다.
4. **과거 상위 계획 기록을 덮어쓸 수 있음 — PASS.** 기존 날짜별 기록은 그대로 두고 2026-10-28 section만 앞에 추가했다.
5. **wrong WPT assertion을 계획에 넣을 수 있음 — PASS.** 고정 source 27행 DOM과 87–88행 min-height assertion을 직접 연결한다.
6. **WPT pin과 expected JSON source 값이 다를 수 있음 — PASS.** revision, path, SHA-256과 assertion line이 expected JSON 및 plan의 동일 값과 일치한다.
7. **현재 live hash를 오늘 확인했다고 오기할 수 있음 — PASS.** expected JSON과 README는 10/27 hash match와 10/28 harness rerun을 분리하며 오늘 hash 재검증은 아니라고 기록한다.
8. **Chrome 13/13을 Spinon 지원 증거라고 표현할 수 있음 — PASS.** upstream testharness 결과를 반복해서 구분하고 runtime 결과가 없다고 명시한다.
9. **min-height가 stretch free-space 산식에 쓰이는 규칙을 누락할 수 있음 — PASS.** CSS Grid 2025 CRD §11.8을 인용하고 `300 - 10 = 290px`을 독립 산식에 기록한다.
10. **fit-content row를 stretch 대상 auto track으로 잘못 셀 수 있음 — PASS.** §11.2/§11.8 근거로 eligible index가 빈 배열이라고 적는다.
11. **stretch stage `applied:true`를 크기 증가로 오해할 수 있음 — PASS.** stage 실행 flag와 전후 row base `[10]`을 분리한다.
12. **root min-height와 used row size가 한 값으로 뭉쳐질 수 있음 — PASS.** JSON과 plan 모두 root 300px, row 10px, item/child 10px을 별도 필드로 둔다.
13. **default grid item minimum/overflow가 조용히 바뀔 수 있음 — PASS.** item `min-height:auto`와 `overflow:visible`을 고정한다.
14. **WPT의 auto width를 바꾼 차이가 숨겨질 수 있음 — PASS.** root width 60px 고정 이유와 block-axis-only 비교 한계를 기록했다.
15. **implicit row가 explicit row로 바뀔 수 있음 — PASS.** `grid-template-rows:none`와 단일 `grid-auto-rows:fit-content(40px)`가 입력 계약에 있다.
16. **fixture route가 다른 tree를 받아들일 수 있음 — PASS.** 고정 root/item/child identity, 단일 자식 수, unknown scene 거부와 변형 입력 fail-closed 요구를 둔다.
17. **데이터의 device-pixel 값이 CSS-px oracle로 취급될 수 있음 — PASS.** exact layout frame과 track 값은 surface transform 이전 CSS px라고 고정한다.
18. **fixture-test ABI가 product archive에 노출될 수 있음 — PASS.** 기본/product artifact 제외 및 fixture-test positive control을 모두 계획한다.
19. **docs build 실패가 숨겨지거나 새 문서 오류가 다른 기존 문제와 섞일 수 있음 — PASS.** 최종 build는 exit 1, 976개 다른 dead-link 문서로 실패했고 새 evidence/plan 파일은 최종 error list에 없음을 기록한다.
20. **PR 이미지가 Spinon 앱 실행처럼 오해될 수 있음 — PASS.** Agent Browser로 PR에 연결할 upstream Chrome screenshot raw URL이 1280×633으로 열리는지 확인하고 session/process를 닫았다. PR 본문에는 upstream baseline이라고 분명히 표시한다. 앱 화면은 구현 PR에서 Simulator로 수집한다.

**판정:** diff에 추가 변경이 필요한 누락·과장은 발견하지 못했다. 이 검토는 계획 PR만 다루며 구현 PR의 별도 20개 review를 대신하지 않는다.
