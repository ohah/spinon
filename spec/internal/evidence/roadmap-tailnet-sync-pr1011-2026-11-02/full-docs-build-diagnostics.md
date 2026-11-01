# 전체 문서 빌드 진단

- 실행 명령: `SPINON_DOC_OUT_DIR=build/spinon/roadmap-tailnet-sync-pr1011-20261102/pass01/site mise exec -- bun run docs:build`
- 사용 버전: Rspress 2.0.22
- 결과: exit code 1. 전체 `spec/` 트리를 빌드하는 동안 `Dead link found` 오류가 여러 문서에서 발생했다. 예로 `spec/0001-conformance.md`, `spec/STATUS.md`, 그리고 10월 C11 runtime evidence의 Markdown에서 보관된 `.log`, `.json`, `.png` 경로를 dead-link로 보고했다.
- 판정: 전체 사이트 빌드나 전체 링크 검사를 통과했다고 기록하지 않는다. 원래 출력은 로컬 evidence 작업 폴더의 `rspress-candidate-build.log`에 보존했다.
- 범위가 제한된 후속 확인: C11 상태·계획·검토 문서 101개 source route를 선택 root로 만들어 Rspress 2.0.22 빌드를 통과시켰다. 이 빌드는 `checkDeadLinks`와 `checkDeadImages`를 끄므로 전체 링크 적합성 판정이 아니다. 16개 생성 route와 공개 roadmap 카드의 6개 local link는 별도 verifier로 확인했다.

이 진단은 기존 dead-link 오류를 C11 구현 실패로 분류하지 않으며, unrelated documentation 링크를 이 작업에서 완료 처리하지 않는다.
