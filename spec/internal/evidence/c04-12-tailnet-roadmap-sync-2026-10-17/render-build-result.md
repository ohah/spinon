# Tailnet 선택 문서 Rspress build

- Rspress: `@rspress/core 2.0.22`.
- 임시 문서 root: STATUS, C04.12 계획, 내부 계약 0077, viewport 실행 근거, Tailnet 동기화 근거.
- 결과: HTML 5개 페이지 렌더 완료, node/web build 성공. Android·iOS 앱을 실행하지 않은 문서 build다.
- 임시 config에서 `markdown.link.checkDeadLinks=false`를 설정했다. 선택 root 밖의 기존 문서 링크는 검사하지 않았다.
- 선택 root의 사이드바와 검색 index는 위 다섯 페이지에 한정된다. 기존 문서 파일을 삭제하거나 전체 문서 검색 index를 교체하지 않았다.
- 저장소 전체 docs build는 성공하지 않았다. 별도 전체 build는 기존 evidence의 누락 PNG import 때문에 실패했고 [전체 로그](./full-docs-build-failure.log)를 보존한다.
- 빌드와 게시 대상 digest는 [manifest](./manifest.json), [선택 build 로그](./selected-build.log)에서 확인한다.
