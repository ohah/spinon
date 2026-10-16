# 전체 문서 빌드 경계

이 기록은 C18.4 범위 밖의 저장소 전체 문서 빌드 결과를 C18.4 scoped build와 구분한다.

- 저장소 기본 `bun run docs:build`는 exit 1로 끝났다. Rspress 2.0.22가 기존 466개 문서 파일의 dead link와 12개 unresolved image import를 보고했다. 원본은 [`full-docs-build.log.raw.gz`](./full-docs-build.log.raw.gz)다.
- 임시 설정에서 dead-link/image 검사를 끈 전체 문서 build도 exit 1이었다. MDX의 unresolved asset import 176개가 남았다. 로그는 [`full-docs-build-checks-disabled.log.raw.gz`](./full-docs-build-checks-disabled.log.raw.gz)에 있다.
- `preferRelative`를 임시로 적용해도 C11.3.1의 실제 누락 이미지 12개 때문에 전체 build는 실패했다. 저장소 Rspress 설정은 바꾸지 않았다. 로그는 [`full-docs-build-prefer-relative.log.raw.gz`](./full-docs-build-prefer-relative.log.raw.gz)에 있다.
- C18.4와 링크된 계약·실행 자료만 넣은 Rspress 2.0.22 scoped build는 통과했다. 이 결과는 저장소 전체 문서 build 성공을 뜻하지 않는다.
