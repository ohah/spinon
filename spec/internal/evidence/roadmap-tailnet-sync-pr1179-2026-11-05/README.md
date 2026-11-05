# PR #1179 로드맵 Tailnet 동기화 · 2026-11-05

이 기록은 병합된 PR [#1179](https://github.com/ohah/spinon/pull/1179)의 제한 구현과 한계를 공개 로드맵·계획·상태 문서에 맞추고, 사용자가 지정한 Tailnet 미리보기를 갱신한 근거다. GitHub Pages는 배포하지 않았다.

## 문서 상태

- 구현 PR #1179 merge SHA: `7c042f4ac5080609927fb26760112c5a88a56d3c` (`2026-11-05T10:16:16Z`, Rebase and merge).
- 공식 완료 수는 `78/214`이고 C11.1/C11/C11.4 체크는 미완료로 유지한다.
- PR check rollup은 비었고 main에 required status checks가 없어 CI 통과로 표현하지 않는다.
- 수정한 source는 `spec/public/roadmap.html`, `spec/STATUS.md`, `spec/internal/0059-c11-grid-layout.md`, `plan/c11-root-html-grid-auto-repeat.md`다. 구현 범위와 simulator 한계는 [계획](../../../../plan/c11-root-html-grid-auto-repeat.md), [내부 계약 0059](../../0059-c11-grid-layout.md), [runtime evidence](../c11-root-html-grid-auto-repeat-runtime-2026-11-05/README.md)에 남아 있다.

## 선별 문서 빌드와 Tailnet 동기화

RSPress `2.0.22`로 로드맵 카드와 연결 문서·증거에 필요한 route만 생성했다. 최종 선택 publication은 118개 route, 14,334,665 bytes다. 선택된 로드맵 카드의 내부 링크 7개는 publication route에 포함되고 각 SHA-256이 manifest에 기록됐다. 전체 문서 링크·dead-link·dead-image 검증은 수행하지 않았다.

동기화 전 Tailscale backend는 `Running`이었고 기존 preview server PID `28040`이 `127.0.0.1:8768`에서 응답했다. 첫 반영(pass01)은 교체 대상 11개에 열린 파일 handle이 없는 것을 확인하고 기존 파일을 고유 backup directory에 SHA-256 검증해 보관한 뒤 임시 파일과 atomic rename으로 반영했다(생성 89·교체 11·동일 18). plan·STATUS의 병합 상태 문구를 한 차례 더 정리해 최종 pass02를 만들었다. pass02에서는 교체 대상 16개 모두 열린 handle이 없었고, 기존 파일을 별도 backup directory에 보관한 뒤 생성 16·교체 16·동일 86개 route를 반영했다. 파일 삭제나 서버 재시작은 없었다. 최종 118개 route는 local preview와 HTTPS Tailnet에서 HTTP 200 및 publication SHA-256 일치를 확인했다.

pass02 build의 첫 시도는 RSPress config의 selected-root 경로가 한 단계 짧아 실패했다. 그 로그를 보존하고 경로를 고친 뒤 재빌드·publication staging·Tailnet sync를 성공시켰다. 실패·수정·성공 로그는 각각 [`selected-build-pass02.log`](./selected-build-pass02.log), [`selected-build-pass02-retry.log`](./selected-build-pass02-retry.log), [`selected-publication-pass02.json`](./selected-publication-pass02.json), [`served-verification-pass02.json`](./served-verification-pass02.json)에 있다.

최종 선택·빌드·복사 근거는 [`selected-root-pass02.json`](./selected-root-pass02.json), [`selected-publication-pass02.json`](./selected-publication-pass02.json), [`preflight-pass02.json`](./preflight-pass02.json), [`copy-and-backup-pass02.json`](./copy-and-backup-pass02.json), [`served-verification-pass02.json`](./served-verification-pass02.json)에 있다. 최종 재현 파일은 [`prepare-selected-root-pass02.mjs`](./prepare-selected-root-pass02.mjs), [`rspress-pass02.config.ts`](./rspress-pass02.config.ts), [`stage-publication-pass02.mjs`](./stage-publication-pass02.mjs), [`sync-tailnet-pass02.mjs`](./sync-tailnet-pass02.mjs)다. 첫 반영 pass01의 기록도 보존했다.

## Agent Browser 확인

Tailnet [로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)을 Agent Browser에서 확인했다. 데스크톱 1440×1000과 모바일 390×844에서 PR #1179 카드, merge SHA, 공식 `78/214`, C11 미완료 문구를 확인했고 가로 넘침은 없었다. 모바일에서 카드 좌우 여백은 14px, 카드 폭은 362px이었다. 화면은 [desktop 캡처](./tailnet-roadmap-agent-browser-desktop.png), [mobile 캡처](./tailnet-roadmap-agent-browser-mobile.png)에 보존했고 bounds는 [verification JSON](./agent-browser-verification.json)에 기록했다.

Tailnet 화면과 PR #1180 본문 이미지를 확인한 뒤 두 번의 `agent-browser close --all`로 default session을 각각 닫았다. PR 본문 이미지 둘 다 natural size로 로드됐고, 마지막 종료 뒤 daemon과 Chrome 프로세스가 없으며 session 상태가 inactive인 것을 확인했다. tmux 세션은 건드리지 않았다. [lifecycle JSON](./agent-browser-lifecycle.json)에 종료 상태를 기록했다.

스크립트·JSON·로그·캡처의 파일별 SHA-256은 [`hashes.sha256`](./hashes.sha256)에 있다.

## 한계와 PR 검토

Tailnet에는 선별 route만 배포했다. 이 동기화는 전체 문서 링크 검증, CI 성공 또는 GitHub Pages 배포를 뜻하지 않는다. simulator 캡처는 visual smoke이며 Android PixelCopy, iOS Metal drawable readback, 실기기/hardware GPU, iOS compositor 이후 pixels 또는 WPT 근거가 아니다. 공식 완료 수와 C11 상태는 바뀌지 않았다.

이 문서·계획·공개 로드맵·동기화 산출물 변경은 [서로 다른 실패 관점 30개 검토](./pr-diff-review-30.md)에 기록한다.
