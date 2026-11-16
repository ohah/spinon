# C15.3 Flex intrinsic test · pass07 source hash closure recheck · 2026-11-16

이 자료는 기존 PR 준비 적대 검토 30관점의 항목 22·23에서 남긴 hash-closure 조건만 다시 확인한다. 새 PR diff 검토나 추가 구현/runtime 검토 횟수로 세지 않는다.

C11.4 sibling pass07은 43개 선택 test 중 36 passed·7 failed로 끝났다. 입력 1,920/1,920의 SHA-256 mismatch는 0이지만 matrix 전체 green은 아니다. C15 후보의 두 Rust source는 pass07 manifest 사전·사후와 C15 focused pass02 사전·사후, 현재 checkout hash가 모두 같다. 정확한 값과 검증 스크립트는 [closure JSON](./source-hash-closure-pass01.json)과 [재실행 스크립트](./verify-source-hash-closure-pass01.mjs)에 있다.

| 항목 | 결과 |
| --- | --- |
| PR 준비 검토 30관점 #22 | 해소: test source SHA를 실행 근거에 연결했다. |
| PR 준비 검토 30관점 #23 | 해소: pass07 입력 경계 사전·사후와 현재 hash가 일치한다. |
| pass07 full matrix | 36/43; 실패 7개. hash closure는 성공 결과를 뜻하지 않는다. |
| C15.3 scope | 고정 Noto Sans KR Rust LayoutInput/Taffy Flex max-content test 하나. |
| 남음 | Cargo.lock의 C11.4 sha2, mise.toml toolchain bump를 배제해 선택 staging; 실제 PR diff 30관점 검토와 CI/merge 확인. |

이 후보는 별도 파일 경계라 C11.4와 기술적으로 독립적으로 선택 staging할 수 있다. 이 기록은 commit, PR 생성, CI, merge를 뜻하지 않는다. C15.3/C15는 미완료이고 공식 집계 78/214를 유지한다.
