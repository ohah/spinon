# 선택 문서 Tailnet 렌더 결과

- 기준 commit: `8a0e79a1ba03e1e0e554fb38d4451b197a4252c3` (PR #334 Rebase and merge)
- RSPress: 프로젝트 고정 버전 `2.0.22`
- 명령: `RSPRESS_PERSISTENT_CACHE=false rspress build --config rspress.tailnet-sync.config.ts`
- 출력: 새 경로 `build/spinon/c11-4-selected-site-final-20261019`
- 범위: 상태 대장, conformance, Grid 내부 계약, C11 Grid 계획, C11.4 계획, C11.4 oracle·검토 근거와 정적 roadmap
- 결과: 선택한 9개 Markdown route와 정적 roadmap을 렌더했다. 출력 파일 34개이며 build exit code는 0이다.
- 프로젝트 전체 문서 빌드는 별도로 실행했으나 이번 변경과 무관한 기존 C06·R05·S04 dead link와 빠진 이미지에서 실패했다. Tailnet에는 선택한 문서만 렌더했다. 제외된 자료의 링크·이미지 전체 적합성이나 전체 문서 build 통과를 뜻하지 않는다.
- 선택 root에서 제외한 문서의 기존 dead link/image가 새 route 빌드를 막지 않도록 RSPress `markdown.link.checkDeadLinks`와 `markdown.image.checkDeadImages`를 끄고, 새로 제공한 route·핵심 링크는 Agent Browser로 직접 열어 확인했다.
- `render-build.log`는 선택 렌더 로그 원본이다. Pages 배포는 하지 않았다.
