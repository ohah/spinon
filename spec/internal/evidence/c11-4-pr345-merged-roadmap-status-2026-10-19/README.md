# C11.4 PR #345 병합 상태 대장 정정

## 변경 범위

PR #345가 `3d2ec268fd23f977aa4c59d5c42bb319556f92a9`로 `main`에 rebase 병합된 뒤, `spec/STATUS.md`에 남아 있던 “진행 중·검토 중” 문구를 병합된 결과를 설명하도록 바꿨다. C11.4와 C11 상위 항목은 미완료로 유지했다. 구현 코드, 공개 API, 완료 체크는 변경하지 않았다.

## 검증

- GitHub PR 조회에서 #345 상태는 `MERGED`, 병합 커밋은 `3d2ec268fd23f977aa4c59d5c42bb319556f92a9`였다.
- RSPress 2.0.22로 `STATUS.md`, C11.4 계획, 관련 stage-trace 근거 페이지만 선택 렌더링했다. 빌드는 성공했다.
- 전체 문서 빌드는 별도 임시 전체 루트에서 시도했으나 기존 `rust,ignore` 코드 fence와 누락 이미지에서 실패했다. 이 문서 변경의 선택 렌더 결과와 혼동하지 않는다.
- C11.4 작업 범위·근거 링크·남은 미완료 조건을 서로 다른 30개 실패 관점으로 검토했다. 아래 기록에 결함은 없었다.

## 한계

이 정정은 PR #345의 병합 상태를 반영한다. C11.4 intrinsic sizing의 전체 auto-repeat/percentage 조합, 앱 text/image intrinsic input, 전체 contribution 행렬, WPT, 실기기와 독립적인 normative oracle 검증은 완료하지 않는다.

## 검토 기록

[PR 변경 30관점 검토](pr-diff-review-30.html)
