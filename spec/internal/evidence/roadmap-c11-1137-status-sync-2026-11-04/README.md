# C11 #1137 병합 상태 동기화 근거

## 발견한 불일치

`origin/main`의 `spec/STATUS.md`에는 C11.1 숫자형 Grid 배치가 “제한 구현 후보(미병합)”라고 적힌 오래된 줄과, 같은 작업의 PR #1137 병합 기록이 함께 있었다. `plan/c11-css-grid.md`의 해당 절도 PR 병합과 Tailnet 공개 로드맵 동기화가 남았다고 표시했다. GitHub에서 PR #1137, #1138, #1139는 모두 이미 병합 상태였다.

## 동기화 내용

- 중복된 미병합 후보 줄을 제거하고 `spec/STATUS.md`의 병합 기록 하나를 기준으로 남겼다.
- 상위 C11 계획의 절 제목과 상태를 제한 구현 병합 결과로 고쳤다.
- #1137 merge SHA와 후속 문서 PR 번호, 이전 Tailnet 공개 roadmap 검증 결과를 정확히 적었다. Tailnet 서버가 파일 핸들을 유지하던 `STATUS.html` 및 내부 계약 0059 미리보기는 덮어쓰거나 재시작하지 않았다는 한계를 남겼다.
- C11/C11.1 체크박스와 공식 집계 `72/213`은 변경하지 않았다. 공개 roadmap HTML은 이미 #1137 병합 상태를 표시하므로 수정하지 않았다.

## 확인

- 기준 원격: `origin/main` `5f8c5be299ac726b00cfcbd29aaef7debaf767be`
- GitHub: #1137 merged `6af14f69512c0c73d890efc728751e0ca342f9c0`; #1138 merged `42946bbac20f0a73dfc4c3d861321f239692b3c6`; #1139 merged `5f8c5be299ac726b00cfcbd29aaef7debaf767be`.
- `git diff --check`: 통과. 문서 변경만 있어 제품 빌드·시뮬레이터는 다시 실행하지 않았다.
- 변경분의 서로 다른 실패 관점 30개 검토: [pr-diff-review-30.md](./pr-diff-review-30.md).
