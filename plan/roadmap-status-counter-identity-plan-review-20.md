# 공식 로드맵 상태 identity 계획 실패 관점 검토 · 20개

**대상:** [집계 identity 복구 계획](./roadmap-status-counter-identity.md)과 [사전 비교 근거](../spec/internal/evidence/roadmap-status-counter-identity-2026-11-03/README.md). 제품 코드 변경 전 계획을 검토했다.

| # | 실패 관점 | 계획에서 확인한 기준 |
|---:|---|---|
| 1 | 비공식 상태 원본 사용 | `spec/STATUS.md`만 source of truth로 고정했다. |
| 2 | 단순 완료 수만 맞추고 행 상태를 잘못 연결 | unique ID는 해당 status row를 base key에 연결하고 중복 ID는 정규화 제목으로 구분한다. |
| 3 | 중복 ID 덮어쓰기 | 반복 `C04.12`의 각 제목·상태를 따로 대조한다. |
| 4 | 숫자 suffix 지원을 잊음 | `C06.6a`·`C06.6b`를 고정 positive input으로 둔다. |
| 5 | 미래 접미 ID의 과도한 parser 허용 | 초기 ID 구조와 선택적 alphabetic suffix만 지원한다. |
| 6 | base row와 하위 ID를 같은 행으로 취급 | base identity와 stable detail identity를 분리한다. |
| 7 | base title drift가 조용히 미완료 처리 | unique ID의 요약 제목 drift는 허용하되 source row가 없으면 sync 실패로 처리한다. |
| 8 | 정규화로 서로 다른 제목이 합쳐짐 | normalized ID/title collision을 거부하도록 요구한다. |
| 9 | 동일한 ID·title 두 행의 충돌 | duplicate identity를 성공 처리하지 않는다. |
| 10 | unnumbered child가 반복 표시 | 기존 parent별 child key/count를 유지한다. |
| 11 | unnumbered child를 모두 공식 ID로 오인 | 기존 `미구현 하위 항목` pattern에 한정한다. |
| 12 | denominator 하드코딩 | 표시 task list의 실제 길이에서 계산한다. |
| 13 | 완료 수를 ID distinct 수로 계산 | 각 task row status를 합산한다. |
| 14 | 검색 중 표시 수와 완료 수 혼동 | visible count와 total completed를 별도 유지한다. |
| 15 | 미완료 전용 필터에서 stale key 사용 | 필터는 task identity status를 읽는다. |
| 16 | 칸반 active/waiting mapping 회귀 | work-order code lookup과 row identity를 구분한다. |
| 17 | 반복 HTML `id` 속성 | label/input pair마다 고유 DOM id를 검증한다. |
| 18 | API 실패가 0 완료로 표시 | fetch 실패는 기존 미판정 오류 화면을 유지한다. |
| 19 | 사용자 저장 상태 혼입 | disable/read-only 동작과 no-storage invariants를 유지한다. |
| 20 | desktop/mobile clipping 누락 | Agent Browser viewport별 가로 overflow와 화면 잘림을 확인한다. |

**판정:** 사전 비교는 표시값·원본 ID/title/check 상태·예상 task row 수를 고정했다. 중복 identity와 missing base 대응을 fail-closed로 요구하므로 단지 분자 숫자만 맞추는 구현을 허용하지 않는다. 계획 단계의 누락은 확인하지 못했다.
