# PR #500 뒤 Tailnet C11.4 로드맵 동기화

- 날짜: 2026-10-22
- 기준 병합: [PR #500](https://github.com/ohah/spinon/pull/500), Rebase and merge, SHA `2f3f45c6aa27f427b36e906fa3a72fd8d864e402`
- 미리보기: [Tailnet 로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html) · [공식 상태](https://macstudio.tailed42f2.ts.net/spinon/STATUS.html) · [C11.4 계획](https://macstudio.tailed42f2.ts.net/spinon/plan/c11-4-cyclic-minmax-calc-overlapping-spans.html) · [실행 근거](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c11-4-cyclic-minmax-calc-overlapping-spans-runtime-2026-10-22/README.html)

## 반영 내용과 공식 상태

PR #500의 새 overlapping span-groups note, C11.4 미완료 상태, 계획, Android/iOS Simulator 실행 근거와 화면을 Tailnet 정적 미리보기에 반영한다. 공식 완료 수는 `77/213`으로 유지한다. GitHub Pages와 `gh-pages`에는 배포하지 않는다.

## 빌드와 복사

선택 사이트는 현재 branch의 commit에서 `git archive`로 만든 별도 source root로 RSPress `2.0.22`가 빌드한다. 사용 중인 checkout을 worktree로 바꾸거나 untracked workspace 파일을 빌드에 섞지 않는다. Tailnet 대상은 `/Users/yoonhb/Library/Application Support/SpinonPreview`, listener는 `127.0.0.1:8768`의 기존 PID 1191이다. 대상 파일은 복사 전에 SHA-256으로 분류하고, 교체 파일을 고유 backup root에 보존하며, 삭제는 하지 않는다.

## 실제 결과

이 절은 선택 build, backup/copy, 로컬 hash, served HTTP hash와 Agent Browser 확인이 끝난 뒤 수치와 화면 링크를 기록한다.
