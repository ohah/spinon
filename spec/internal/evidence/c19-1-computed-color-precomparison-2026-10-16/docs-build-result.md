# C19.1 문서 빌드 결과

- 사용 명령: `bun install --frozen-lockfile`, `bun run docs:build`
- Rspress는 저장소 pin인 `2.0.22`를 사용했다. 첫 실행은 worktree에 의존성이 없어 `rspress: command not found`였고, frozen lockfile install 후 빌드를 재실행했다. lockfile 변경은 없다.
- 최종 전체 문서 빌드는 exit 1이며 저장소 문서 468페이지에서 dead-link 오류가 보고됐다. 오류가 남은 기존 `spec/STATUS.md`와 `spec/internal/README.md`를 포함해 다른 계획·산출물 경로가 함께 나온다.
- 새 C19.1 contract, Chrome 사전 비교, 호환성 분류, 구현 20항목 리뷰, PR diff 30항목 리뷰에는 dead-link 오류가 없다. C19 계획 링크는 기존 문서 route로 해석되지 않아 GitHub main 링크로 바꾸었다.
- 첫 빌드 때 추가한 로컬 C19 artifact 링크도 GitHub main 링크 또는 inline path로 바꿔 새 dead link를 제거했다. 저장소 전체의 기존 링크를 이번 C19 변경에 끌어와 수정하지 않았다.

반복 전체 출력은 각 1 MB를 넘는 로그 파일이므로 PR에는 저장하지 않는다. 최종 판정은 위 결과와 Rspress 2.0.22 실행, C19 문서별 dead-link 검사를 근거로 기록했다.
