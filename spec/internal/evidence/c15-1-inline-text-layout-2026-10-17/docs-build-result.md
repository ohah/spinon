# C15.1 문서 빌드 결과

- 명령: `mise exec -- bun run docs:build`
- Rspress는 잠금 파일에 고정된 `2.0.22`를 사용했다. 문서 의존성은 `mise exec -- bun install --frozen-lockfile`로 설치했으며 `bun.lock` 변경은 없다.
- 전체 문서 빌드는 exit 1이며 471개 문서에서 dead-link 오류를 보고했다. 변경한 `spec/0001-conformance.md`, `spec/STATUS.md`, `spec/internal/README.md`에도 C15와 무관한 기존 plan/evidence 링크 오류가 남아 있다.
- C15 이름을 가진 source 문서 또는 href 오류는 0개다. 새 C15 plan 링크는 GitHub source URL이며 계약·관찰·실행·검토 자료 간 상대 문서 링크는 해당 빌드의 오류 목록에 없다.
- 전체 출력은 2.1 MiB라 PR에 넣지 않았다. 원본 빌드 출력 SHA-256은 `8923264e08de486f03011950d231a0e727308e499d9dffcbffa0507aa26d29de`다.
