# C04.12 공식 상태 대장 병합 기록 정합성 검토 30개

- **검토일:** 2026-10-18
- **대상:** `spec/STATUS.md`의 C04.12 ID 없는 Block scene 하위 항목과 구현 계획의 PR #313 상태
- **사실 기준:** PR #313 `MERGED`, 병합 시각 `2026-10-18T10:19:23Z`, merge commit `b502a5155dfeeaf6597e64c04fb62190c4256884`; main branch protection에는 required status checks가 없고 PR checks도 보고되지 않음
- **판정:** 아래 30개 관점을 모두 통과했다. 이 수정은 문장과 검토 링크만 바꾸며 상태 대장의 체크 여부·공식 집계에는 변화를 주지 않는다.

| # | 서로 다른 실패 관점 | 판정과 근거 |
|---:|---|---|
| 1 | PR #313을 열린 상태로 오인 | 통과 — GitHub API 응답에서 `MERGED` 확인 |
| 2 | 병합 날짜를 하루 잘못 표기 | 통과 — `mergedAt`은 2026-10-18 |
| 3 | 다른 변경의 merge commit을 기록 | 통과 — PR #313의 SHA `b502a5155dfeeaf6597e64c04fb62190c4256884` |
| 4 | merge 전략을 `Create a merge commit`으로 잘못 기술 | 통과 — GitHub PR은 Rebase and merge 정책으로 병합 |
| 5 | 병합되지 않은 PR을 완료로 설명 | 통과 — 실제 병합 시각과 SHA가 있음 |
| 6 | branch protection의 필수 검사가 있었던 것처럼 기록 | 통과 — `required_status_checks`는 null |
| 7 | CI workflow가 전부 통과했다고 과장 | 통과 — 해당 PR에 보고된 check가 없었다고 한정 |
| 8 | 과거 check 상태를 현재 상태로 일반화 | 통과 — 문장을 “병합 당시”로 시간 한정 |
| 9 | 현재 C04.12 상위가 완료라고 오인 | 통과 — 상위 완료 문구나 checkbox 변경 없음 |
| 10 | ID 없는 scene slice를 상위 ID로 승격 | 통과 — ID 없는 하위 작업이라는 기존 계약 보존 |
| 11 | scene 하위 checkbox를 미완료로 되돌림 | 통과 — 기존 `[x]` 표기 보존 |
| 12 | C04.12 상위 checkbox를 완료로 바꿈 | 통과 — 기존 `[ ]` 표기 보존 |
| 13 | checkbox 개수 변화로 공식 합계를 변경 | 통과 — 전체 checkbox 상태 sequence와 완료 수 동일 |
| 14 | 완료 ID 수를 바꿈 | 통과 — ID와 목록의 구조 변경 없음 |
| 15 | “현재 제품 지원 완료 없음” 상태와 충돌 | 통과 — status heading과 해당 문구 미변경 |
| 16 | 제품 API·C ABI 구현을 주장 | 통과 — 해당 변경에는 API나 ABI 수정 없음 |
| 17 | 계약 0077 숫자 버전을 올림 | 통과 — 계약 버전 변경 없음 |
| 18 | 제한 Block whitelist를 넓힘 | 통과 — scene 성공 범위 문구 미변경 |
| 19 | 일반 HTML text/glyph 지원을 암시 | 통과 — text/paint 미지원 경계 보존 |
| 20 | 전체 Bootstrap `index.html` 렌더링을 주장 | 통과 — 범위 밖 문구 보존 |
| 21 | Android AVD를 실기기로 표시 | 통과 — API 37 ARM64 AVD 문구 보존 |
| 22 | SwiftShader를 hardware GPU로 설명 | 통과 — ANGLE/SwiftShader 한계 보존 |
| 23 | iOS drawable을 compositor 이후 결과로 설명 | 통과 — compositor 전 readback 경계 보존 |
| 24 | 마지막 Rust 변경 뒤 native 재캡처를 주장 | 통과 — 미실행 문구 보존 |
| 25 | PR #313 GitHub URL을 깨뜨림 | 통과 — 기존 URL 유지 |
| 26 | 새 검토 링크의 STATUS 상대 경로를 오기 | 통과 — `spec/STATUS.md` 기준 `internal/evidence/...` 파일 존재 |
| 27 | 계획 문서의 검토 링크를 누락 | 통과 — 계획에도 새 검토 문서 링크 추가 |
| 28 | 공식 대장 원본 대신 Tailnet 사본만 수정 | 통과 — canonical `spec/STATUS.md`를 수정 |
| 29 | 다른 기능 상태나 근거를 함께 바꿈 | 통과 — 변경은 status 문구, 계획 링크, 이 검토 보고서 세 파일로 제한 |
| 30 | 문서 diff whitespace 오류 | 통과 — `git diff --check` 통과 |

## 남은 경계

이 문서는 PR #313의 기능이나 Android/iOS runtime 동작을 다시 시험하지 않는다. 기존 simulator·Rust 근거를 대체하지 않으며 C04.12 상위 완료나 물리 기기 검증을 주장하지 않는다.
