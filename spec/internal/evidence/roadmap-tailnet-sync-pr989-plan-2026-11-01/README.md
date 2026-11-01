# C11.4 Unicode 계획 로드맵 동기화 근거

## 변경 범위

기준 `main`은 C11.4 Unicode `text-transform:capitalize` 계획 PR #989의 merge SHA `2cfc2f09aa8a03b50aaef9db9d1895ef44f10d34`다. 공개 로드맵에 병합된 계획의 날짜, 범위, 기준 차이와 미완료 상태를 추가한다. `spec/STATUS.md`, 구현 코드, 공식 완료 수는 변경하지 않는다. 이 계획의 구현과 Android/iOS Simulator 실행은 아직 시작하지 않았다. C11.4와 C11은 미완료이며 공식 완료 수는 `72/213`이다.

## Agent Browser 확인

저장소의 `spec/public/roadmap.html`을 Agent Browser에서 열고 넓은 화면과 390×844 모바일 viewport로 확인했다. 모바일 viewport에서 `innerWidth`, `document.documentElement.scrollWidth`, `document.body.scrollWidth`는 각각 `390`, `390`, `390`이었다. 화면 텍스트에 PR #989와 `72/213`이 모두 나타났다. 캡처는 앱 실행 화면이 아니라 공개 로드맵 문서다.

- 넓은 화면 캡처: [roadmap-plan-unicode-agent-browser.png](./roadmap-plan-unicode-agent-browser.png)
- 모바일 캡처: [roadmap-plan-unicode-mobile-agent-browser.png](./roadmap-plan-unicode-mobile-agent-browser.png)
- 넓은 화면 SHA-256: `bf0f15ed757dcdf24ce204a2d4a78347bceeecb96816e783176d331dc73ca92d`
- 모바일 SHA-256: `c9f9d5aa9e84684ff13a9df4a6ebc00918b9299472635a7cf8ae7d28fa4af032`

확인 뒤 해당 Agent Browser 세션을 닫았고 활성 세션이 남지 않았다.

## 문서 빌드 한계

`mise exec -- bun run docs:build`는 저장소의 다른 문서에 있는 기존 dead-link 진단 때문에 종료 코드 1로 끝났다. 전체 빌드 성공으로 기록하지 않는다. 해당 실행 로그에서 `c11-4-anonymous-grid-text-transform-unicode` 경로는 발견되지 않았다. 이 동기화 PR은 그 전체 빌드를 통과했다고 주장하지 않는다.

## 남은 범위

이 변경은 로드맵 설명과 검토 증거만 동기화한다. Tailnet 미리보기 갱신은 PR 병합 뒤 별도 수행하고, Unicode titlecase 구현 및 Android/iOS Simulator 검증도 계획된 순서에 따라 후속 진행한다. 공개 CSS 지원, 전체 C11.4/C11 완료, 공식 완료 수 변경은 주장하지 않는다.

## 검토

[로드맵 PR diff 실패 관점 검토 30개](./roadmap-pr-diff-review-30.md)
