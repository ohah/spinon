# 기본 문서 전체 빌드 결과

2026-10-23에 `mise exec -- bun run docs:build`를 실행했지만 Rspress 2.0.22가 exit code 1로 끝났다. 전체 `spec/` 입력에서 788개 Markdown 파일의 dead link를 보고했다. 주된 기존 범위는 미리보기 문서 root 밖의 `plan/` 페이지와 Markdown 페이지로 변환되지 않는 raw JSON/log/image 파일 링크다.

새 runtime README에서는 screenshot을 이미지로 embed하고 계획 링크를 Tailnet selected root 기준 상대 경로로 바꿨다. 기본 docs root에는 `plan/`이 포함되지 않으므로 새 README에 남은 해당 링크 하나도 default build에서 dead link로 보고된다. 이 오류는 Tailnet selected root가 `plan/`과 선택 evidence를 넣어 빌드한 결과로 확인한다.

Tailnet selected-root build는 이전 pass59 절차와 동일하게 Rspress dead-link 검사를 끈다. 그러므로 성공 여부는 build exit code만으로 판단하지 않고 roadmap·계획·evidence 화면과 HTTP status·served SHA-256을 별도로 확인한다. 기본 build 전체 로그는 작업 과정의 `/tmp/spinon-c114-docs-build-default.log`에 있다.
