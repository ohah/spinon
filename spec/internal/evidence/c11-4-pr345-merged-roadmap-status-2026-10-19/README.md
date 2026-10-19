# C11.4 PR #345 병합 상태 대장 정정

## 변경 범위

PR #345가 `3d2ec268fd23f977aa4c59d5c42bb319556f92a9`로 `main`에 rebase 병합된 뒤, `spec/STATUS.md`에 남아 있던 “진행 중·검토 중” 문구를 병합된 결과를 설명하도록 바꿨다. C11.4와 C11 상위 항목은 미완료로 유지했다. 구현 코드, 공개 API, 완료 체크는 변경하지 않았다.

## 검증

- GitHub PR 조회에서 #345 상태는 `MERGED`, 병합 커밋은 `3d2ec268fd23f977aa4c59d5c42bb319556f92a9`였다.
- RSPress 2.0.22로 `STATUS.md`, C11.4 계획, 관련 stage-trace 근거 페이지만 선택 렌더링했다. 빌드는 성공했다.
- 전체 문서 미리보기도 RSPress 2.0.22로 렌더링했다. 저장소 전체를 복사하지 않도록 Markdown 파일만 임시 루트에 복사하고 비문서 파일은 symlink했다. 그 임시 사본에서 8개의 `rust,ignore` fence 표기, 기존 상대 이미지 경로 93개, 이미지 파일이 없는 링크 6개를 빌드 입력에 한해 정리하고, RSPress가 route로 오인한 증거용 JS/MJS 파일 9개를 제외했다. 작업 브랜치와 원본 문서는 수정하지 않았다.
- 기본 저장소 루트 전체 빌드는 위 기존 `rust,ignore` 표기·이미지 경로와 증거용 JS/MJS route 때문에 실패했다. 해당 실패와 임시 입력을 통한 전체 렌더 성공은 서로 다른 결과로 기록한다.
- C11.4 작업 범위·근거 링크·남은 미완료 조건을 서로 다른 30개 실패 관점으로 검토했다. 아래 기록에 결함은 없었다.

## 한계

이 정정은 PR #345의 병합 상태를 반영한다. C11.4 intrinsic sizing의 전체 auto-repeat/percentage 조합, 앱 text/image intrinsic input, 전체 contribution 행렬, WPT, 실기기와 독립적인 normative oracle 검증은 완료하지 않는다.

## 검토 기록

[PR 변경 30관점 검토](pr-diff-review-30.html)
