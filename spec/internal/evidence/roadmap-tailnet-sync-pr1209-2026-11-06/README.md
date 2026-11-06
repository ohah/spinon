# PR #1209 병합 후 Tailnet roadmap 동기화 · 2026-11-06

PR [#1209](https://github.com/ohah/spinon/pull/1209)은 2026-11-06T05:47:19Z Rebase and merge로 병합됐다(merge SHA `710b359a7e57ce59110f384a1b7b458f0cafc95f`). C11.4 ASCII word 제한 구현 PR #1208의 상태·계획·내부 계약·공개 roadmap을 동기화했다. C11.4/C11은 미완료이며 공식 완료 수 `78/214`를 유지한다.

## 제공본 동기화

공식 source [`spec/public/roadmap.html`](../../../public/roadmap.html)의 merge 후 SHA-256은 `ed76e6d04639dd10ac660b781237c00dcf3994869f3cab7229261a189b96d8c8`(593059 bytes)다. 사용자가 확인한 URL `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`의 HTTP body를 조사하니 미리보기 루트 `/Users/yoonhb/Library/Application Support/SpinonPreview/roadmap.html`과 byte-for-byte 일치했다. 하위 `spec/public/roadmap.html` 파일은 그 URL의 body가 아니므로 잘못된 경로를 갱신하지 않았다.

기존 루트 파일은 590947 bytes / SHA-256 `02c61ff2923610bd7b7c9359af0fc2ec6c47651e063d86ffe734a2e2d0207acc`였다. 열린 파일 handle이 없는 것을 확인하고 백업 경로: `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1209-2026-11-06/pass01/roadmap.html`에 보존한 뒤, 같은 디렉터리 임시 파일에서 atomic replace했다. 서버 PID 28040은 `127.0.0.1:8768`에서 계속 실행 중이었고 재시작하지 않았다. Tailscale은 `Running`이며 macstudio Funnel이 켜져 있었다.

- Source, local HTTP, Tailnet HTTPS 모두 593,059 bytes / SHA-256 `ed76e6d04639dd10ac660b781237c00dcf3994869f3cab7229261a189b96d8c8`이며 response body가 source와 byte-for-byte 같다.
- Local HTTP와 Tailnet HTTPS 모두 HTTP 200을 반환했다.
- 상세 원본: [preflight](./preflight.json) · [copy and backup](./copy-and-backup.json) · [served verification](./served-verification.json) · [source hash](./source.sha256).

## Agent Browser 확인

Agent Browser에서 실제 Tailnet URL의 병합 카드, PR·계획·evidence·Android/iOS screenshot 링크, 문서 폭과 가로 overflow를 확인했다.

- Desktop 1440×1000 CSS px: document width 1440px, 가로 overflow 없음. PR #1208 카드 x=90px, width=928px로 화면에 보였다. [화면](./tailnet-roadmap-agent-browser-desktop.png) · [DOM/geometry](./tailnet-agent-browser-desktop.json)
- Mobile 390×844 CSS px: document width 390px, 가로 overflow 없음. PR #1208 카드 x=14px, width=362px로 화면에 보였다. [화면](./tailnet-roadmap-agent-browser-mobile.png) · [DOM/geometry](./tailnet-agent-browser-mobile.json)
- 페이지 제목은 `스피논 구현 로드맵`이며 카드에 PR #1208과 공식 `78/214`가 표시된다. C11.4/C11 상위 완료로 표시되지 않는다.
- 사용 후 Agent Browser 세션을 닫았고 `pgrep -x agent-browser` 결과가 비어 있어 해당 프로세스가 남지 않았다.

```text
PR #1209 merge/source main SHA: 710b359a7e57ce59110f384a1b7b458f0cafc95f
Source tree: 32178b6754100cf438e607390ca228387e5722f5
Source/served SHA-256: ed76e6d04639dd10ac660b781237c00dcf3994869f3cab7229261a189b96d8c8
Tailnet sync: HTTP 200; source/local/remote hashes match
```
