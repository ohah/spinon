# 전체 Rspress 빌드 실패 요약 · 2026-11-05

`bun run docs:build`는 전체 `spec/internal/evidence` 문서의 dead-image 해석에서 실패했다. 예시 오류는 기존의 다른 work item 경로에서 참조한 파일을 찾지 못한 것이다.

- `c11-4-negative-implicit-track-sizing-2026-10-20`: Android/iOS 실행 캡처 일부 누락
- `c11-root-html-column-flow-mixed-runtime-2026-11-04`: `mixed-spans-surface-pixels.png`, `mixed-spans-ios.png` 누락
- `c13-1-overflow-clip-full-viewport-hit-identity-2026-10-14`: `chrome155-full-viewport.png` 누락
- `c11-4-roadmap-card-width-pr518-2026-10-23`: `tailnet-roadmap-agent-browser-pass53.png` 누락

실제 전체 build log는 이 작업 중 임시 증거로 생성됐지만 생성된 formatter whitespace가 다량 포함되어 Git에 추가하지 않았다. 해당 실패를 고친 것으로 간주하지 않는다. 이번 변경에 필요한 route만 고정한 선택 build는 별도 실행했고, 그 성공 범위는 `selected-publication.json`에 기록했다. 이 선택 build는 전체 문서 링크 검증을 대신하지 않는다.
