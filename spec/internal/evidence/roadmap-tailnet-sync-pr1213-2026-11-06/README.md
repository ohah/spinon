# PR #1213 병합 후 Tailnet roadmap 동기화 · 2026-11-06

문서 PR [#1213](https://github.com/ohah/spinon/pull/1213)은 2026-11-06T07:07:19Z Rebase and merge로 병합됐다(merge SHA `cf3b29cab58f66a8993845123b0873e1d414a5cd`). ASCII alphanumeric C11.4 제한 slice를 [공식 상태](../../STATUS.md), [내부 계약 0059](../../0059-c11-grid-layout.md), 계획 및 공개 roadmap에 동기화했다. C11.4/C11은 미완료이며 공식 `78/214`를 유지한다.

## 제공본 동기화

공식 source [`spec/public/roadmap.html`](../../../public/roadmap.html)은 595,374 bytes / SHA-256 `0544cb8053d1abf6ba073d64ae53de090d07c4604b20915ae8caf96a3d41ce44`다. Tailnet URL `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`은 merge 전 593,059 bytes / SHA-256 `ed76e6d04639dd10ac660b781237c00dcf3994869f3cab7229261a189b96d8c8`인 이전 제공본을 반환했다.

기존 파일을 덮어쓰기 전에 open file handle이 없음을 확인하고 다음 경로에 byte-for-byte 백업했다: `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1213-2026-11-06/pass01/roadmap.html`. 같은 제공 디렉터리에 임시 복사본을 만들고 source hash를 비교한 뒤 atomic rename했다. Tailnet preview server PID `28040`은 `127.0.0.1:8768`에서 계속 실행 중이었고 재시작하지 않았다. Tailscale `BackendState=Running`, Funnel 및 HTTPS endpoint도 확인했다.

- Source, local HTTP와 Tailnet HTTPS는 모두 595,374 bytes / SHA-256 `0544cb8053d1abf6ba073d64ae53de090d07c4604b20915ae8caf96a3d41ce44`이며 byte-for-byte 같다.
- Local HTTP와 Tailnet HTTPS는 모두 HTTP 200이다.
- 원본 근거: [사전 상태](./preflight.json) · [백업·교체](./copy-and-backup.json) · [제공본 검증](./served-verification.json) · [source hash](./source.sha256).

## Agent Browser 확인

사용자가 지정한 실제 Tailnet URL을 Agent Browser로 열고 병합된 PR #1212 카드를 확인했다.

- Desktop 1440×1000 CSS px: 문서 폭 1440px, 가로 overflow 없음, 카드 x=90px / width=928px. [화면](./tailnet-roadmap-agent-browser-desktop.png) · [DOM/geometry](./tailnet-agent-browser-desktop.json)
- Mobile 390×844 CSS px: 문서 폭 390px, 가로 overflow 없음, 카드 x=14px / width=362px. [화면](./tailnet-roadmap-agent-browser-mobile.png) · [DOM/geometry](./tailnet-agent-browser-mobile.json)
- 제목은 `스피논 구현 로드맵`이며 카드에 PR #1212, merge SHA, `78/214`와 C11.4/C11 미완료가 보인다.
- 사용 후 Agent Browser를 닫았고 `pgrep -x agent-browser` 결과가 비어 프로세스가 남지 않았다.

```text
Docs merge PR: #1213
Docs merge SHA: cf3b29cab58f66a8993845123b0873e1d414a5cd
Source/served SHA-256: 0544cb8053d1abf6ba073d64ae53de090d07c4604b20915ae8caf96a3d41ce44
Tailnet sync: HTTP 200; source/local/remote hashes match
```
