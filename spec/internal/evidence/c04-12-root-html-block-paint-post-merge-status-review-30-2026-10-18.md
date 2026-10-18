# C04.12 root HTML Block paint 병합 후 문서 정합성 검토 30개

- **검토일:** 2026-10-18
- **대상:** PR #313 병합 뒤 구현 계획과 Tailnet roadmap 원본의 상태 문구
- **사실 기준:** GitHub PR #313 `MERGED`, `mergedAt=2026-10-18T10:19:23Z`, merge commit `b502a5155dfeeaf6597e64c04fb62190c4256884`; PR diff에는 required status check가 없었음
- **검토 결과:** 아래 30개 관점 모두 통과. 공식 상태 ID의 체크나 완료 수는 바꾸지 않음.

| # | 서로 다른 실패 관점 | 판정과 근거 |
|---:|---|---|
| 1 | 닫히지 않은 PR을 병합 완료로 표시 | 통과 — GitHub API에서 PR #313 `MERGED` 확인 |
| 2 | 병합 날짜를 잘못 기록 | 통과 — `mergedAt`은 2026-10-18 |
| 3 | 다른 PR의 merge commit을 연결 | 통과 — 기록 SHA는 PR #313의 `b502a5155dfeeaf6597e64c04fb62190c4256884` |
| 4 | 리베이스 머지 정책과 다른 방식으로 설명 | 통과 — 프로젝트 정책의 `Rebase and merge`와 일치 |
| 5 | 존재하지 않는 PR 번호를 링크 | 통과 — 기존 GitHub URL은 `/pull/313` |
| 6 | roadmap 링크 문구가 대기 중으로 읽힘 | 통과 — 링크 표기를 `병합 PR #313`으로 고침 |
| 7 | 구현 계획이 여전히 병합 대기라고 표시 | 통과 — 대기 문구를 제거하고 실제 병합 사실을 기록 |
| 8 | required checks가 있었던 것처럼 CI 통과를 주장 | 통과 — required status check가 없었다는 경계를 유지 |
| 9 | CI 전체 통과를 암시 | 통과 — CI 성공 주장을 추가하지 않음 |
| 10 | 공식 상위 C04.12를 완료로 오인 | 통과 — roadmap에서 상위 미완료 문구를 보존 |
| 11 | 세부 slice 완료를 상위 완료로 합산 | 통과 — 상위 상태와 공식 합계는 바꾸지 않음 |
| 12 | `spec/STATUS.md` 체크를 암묵적으로 바꿈 | 통과 — 해당 파일 변경 없음 |
| 13 | 상태 대장의 완료 수를 변경 | 통과 — count source 파일 변경 없음 |
| 14 | 계획 상태 문구가 범위를 넓힘 | 통과 — 제한된 opt-in scene 경계 유지 |
| 15 | 일반 HTML layout을 지원한다고 표현 | 통과 — 일반 HTML layout 미검증을 명시 |
| 16 | 일반 HTML text/glyph paint를 지원한다고 표현 | 통과 — text/paint 미검증 경계 유지 |
| 17 | 전체 Bootstrap `index.html` 표시를 주장 | 통과 — 계획에서 계속 범위 밖으로 분리 |
| 18 | Android AVD를 실기기로 설명 | 통과 — API 37 AVD 표기를 보존 |
| 19 | SwiftShader 결과를 hardware GPU 결과로 격상 | 통과 — ANGLE/SwiftShader 제한 유지 |
| 20 | iOS Simulator drawable 표본을 compositor 이후로 설명 | 통과 — compositor 전 readback 경계 유지 |
| 21 | 마지막 Rust 수정 뒤 native 재캡처를 했다고 주장 | 통과 — 미실행 사실을 roadmap/evidence에서 보존 |
| 22 | Unicode whitespace 보강 검증을 누락 | 통과 — core 1개와 runtime 2개 집중 테스트 문구 유지 |
| 23 | 후속 테스트 로그 링크를 잃음 | 통과 — 구현 근거 README의 로그 링크 변경 없음 |
| 24 | 내부 계약을 새 제품 API로 승격 | 통과 — 공개 API나 C ABI 변경을 주장하지 않음 |
| 25 | 내부 계약 버전을 변경 | 통과 — `0077` 계약 및 숫자 버전 파일 변경 없음 |
| 26 | roadmap에서 계획 페이지 경로를 훼손 | 통과 — `/spinon/plan/c04-12-root-html-block-paint.html` 유지 |
| 27 | 내부 계약·검토 페이지 링크를 제거 | 통과 — 기존 대상 링크 유지 |
| 28 | 병합 후 검토 링크 경로가 잘못됨 | 통과 — plan의 상대 경로를 대조 |
| 29 | 관련 없는 상태·구현 파일을 함께 변경 | 통과 — 계획, roadmap, 이 검토 문서만 변경 |
| 30 | 문서 diff에 공백 오류가 남음 | 통과 — `git diff --check` 통과 |

## 남은 경계

이 문서는 PR #313의 기능 동작을 재검증하지 않는다. 원래 PR의 구현·런타임·플랫폼 근거와 적대적 diff 검토는 기존 링크를 따른다. 여기서는 병합 후 문서의 상태 사실과 완료 범위만 확인했다.
