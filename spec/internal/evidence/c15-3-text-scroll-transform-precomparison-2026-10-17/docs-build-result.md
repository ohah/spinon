# 문서 빌드 결과

- `mise exec -- bun install --frozen-lockfile`: 성공, lockfile에서 209개 패키지 설치.
- `mise exec -- bun run docs:build`: Rspress `2.0.22`, exit `1`. 전체 사이트에서 기존 dead-link 진단 490건이 발생했다. 진단 원본은 작업 실행 로그 `/private/tmp/spinon-c15-scroll-plan-docs-build-verified.log`에 있다.
- 전체 빌드는 통과로 세지 않는다. 새 scroll-transform README와 plan review의 로컬 Markdown 링크는 오류 목록에 없고, 새 raw JSON/fixture/font/manifest 링크는 Rspress dead-link 검사 대상이 되지 않도록 GitHub 경로를 사용했다. `spec/0001-conformance.md`의 다른 기존 링크 오류는 여전히 남아 있다.
- `git diff --check`: 성공.

이 기록은 계획·사전 비교 PR의 문서 검사 결과다. Rust 구현, WPT, Android/iOS 앱이나 GPU 검증은 실행하지 않았다.
