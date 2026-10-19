# C11.4 PR #342 Tailnet 로드맵 미리보기 동기화

**날짜:** 2026-10-19
**문서 원본 커밋:** `0d35a10b1e720993a247dcc84ad3c46179e2940d` (PR #342 리베이스 병합)
**공식 상태:** 완료 72/212 유지 · C11.4 미완료 유지

## 범위

PR #342에서 갱신한 [상태 대장](../../../STATUS.md), [C11.4 계획](../../../../plan/c11-4-grid-track-sizing.md), [내부 계약 0059](../../0059-c11-grid-layout.md), [contribution input PR diff 검토](../c11-4-grid-track-sizing-precomparison-2026-10-19/grid-item-intrinsic-contribution-pr-diff-review-30.md)를 Tailnet 문서 미리보기에 반영했다. 기존 C11.4 문서 경로와 연결된 evidence/static 자산도 함께 준비했다.

이 자료는 선택된 문서의 렌더·파일 동기화·브라우저 표시 근거다. 앱 화면, Android/iOS simulator·실기기 또는 Grid track sizing runtime을 검증하지 않았다.

## 렌더

Rspress `2.0.22`로 Markdown 경로 16개를 선택 렌더했다. 이전 PR #341의 15개 경로에 PR #342의 새 검토 문서를 더했다. 정적 로드맵과 기존 연결 자료를 포함해 배포 대상 80개를 만들었다. 성공 로그는 [render-build.log](render-build.log)에 있다.

전체 사이트 `mise exec -- bun run docs:build`도 실행했으나 변경 범위 밖의 기존 Markdown dead link에서 종료 코드 1로 실패했다. 압축한 전체 로그는 [full-site-build-failure.log.gz](full-site-build-failure.log.gz)에 보존했다. 선택 렌더는 한정된 경로만 빌드하며 `markdown.link.checkDeadLinks=false`를 사용했다. 따라서 이 결과를 전체 사이트 빌드나 링크 검사 통과로 해석하지 않는다.

## 동기화와 해시

- 배포 대상 80개: 기존 58개, 교체 18개, 신규 22개, 동일 40개
- 삭제 0개, 복사 뒤 원본과 다른 파일 0개
- 교체 전 기존 파일 18개를 미리보기 경로 바깥에 백업했고 SHA-256 불일치 0개
- HTTPS에서 80/80개가 HTTP 200을 반환했고, 응답 본문 80/80개의 SHA-256과 길이가 렌더 원본과 일치
- 서버가 read-only 파일 디스크립터로 열어 둔 `404.html`은 그대로 남겼다

원자료: [SHA256SUMS.json](SHA256SUMS.json), [preflight.json](preflight.json), [backup-verification.json](backup-verification.json), [copy-verification.json](copy-verification.json), [served-verification.json](served-verification.json). 변경된 네 문서의 상대 링크 1,045개 중 새 누락은 없고, [7개 기존 누락](targeted-link-audit.json)은 PR #342의 기준 커밋에도 있었다.

## Agent Browser 화면

Agent Browser로 아래 경로를 확인했다. 로드맵 화면에는 공식 72/212 집계와 아직 미완료인 C11.4 행이 보인다.

- [로드맵 — C11.4 행과 공식 집계](tailnet-roadmap-agent-browser.png)
- [로드맵 상단](tailnet-roadmap-overview-agent-browser.png)
- [상태 대장 — C11.4와 검토 링크](tailnet-status-agent-browser.png)
- [C11.4 계획](tailnet-plan-agent-browser.png)
- [PR 변경 검토 30개](tailnet-pr-review-agent-browser.png)
- [브라우저 관찰값](browser-verification.json)

Terminal Browser는 사용하지 않았다. Agent Browser의 창을 닫은 뒤 세션 목록과 process를 다시 확인했고, Agent Browser·Chrome·Chromium 프로세스가 남지 않았다. [종료 확인](browser-session-shutdown.json).

## 남은 범위

이 동기화는 문서 미리보기 갱신이다. C11.4 CSS contribution 선택, percentage/calc basis, aspect-ratio 전달, automatic minimum, Grid track stages, auto-repeat 및 Android/iOS runtime은 구현·검증 완료로 바뀌지 않았다.
