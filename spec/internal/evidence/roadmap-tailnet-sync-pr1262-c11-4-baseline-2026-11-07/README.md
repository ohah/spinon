# PR #1262 병합 후 Tailnet 로드맵 확인 · 2026-11-07

PR [#1262](https://github.com/ohah/spinon/pull/1262)은 2026-11-07T09:46:46Z Rebase and merge됐다. merge SHA는 `e275882ecceb1a2e2545eb827cdcde3673d8efd2`다. 이 bounded `align-content:baseline` fallback slice는 병합됐지만 C11.4/C11 전체는 미완료이고 공식 완료 수 `78/214`를 유지한다.

## Tailnet 제공본

`/spinon/roadmap.html`은 `spec/public/roadmap.html`을 직접 제공한다. Tailscale backend `Running`, preview server PID `28040`, root `/Users/yoonhb/Library/Application Support/SpinonPreview`를 확인했다. PR #1260까지 포함한 이전 제공본은 622,905 bytes / SHA-256 `980c92d9360b39a5ccc319aede900108c68ceb1fcc519c4936fae528b78d2494`였다. 병합된 source는 623,127 bytes / SHA-256 `32650007aa38c6d3b41e4d47abf141f801a8d531df4bf641613b58b258f5c252`다.

- Tailnet preview의 `roadmap.html` 한 파일만 바꿨다. 이전 파일을 [백업](./copy-and-backup.json)으로 보존했고 SHA-256이 preflight의 이전 제공본과 일치한다.
- 같은 preview 디렉터리의 임시 파일을 만든 다음 atomic rename으로 교체했다. 파일 삭제나 preview server 재시작은 없었다.
- Source, `http://127.0.0.1:8768/spinon/roadmap.html`, Tailnet HTTPS 모두 623,127 bytes / SHA-256 `32650007aa38c6d3b41e4d47abf141f801a8d531df4bf641613b58b258f5c252`로 일치하며 두 제공 URL은 HTTP 200이다.
- [preflight](./preflight.json) · [교체·백업](./copy-and-backup.json) · [served verification](./served-verification.json) · [재현 스크립트](./sync-tailnet-roadmap.mjs)

## Agent Browser 화면 확인

요청한 Tailnet URL을 Agent Browser로 열고 desktop 1440×1000 CSS px와 mobile 390×844 CSS px에서 확인했다. 두 크기에서 칸반 `진행 중 1/2`, `C11.4 · ACTIVE`, 완료 78개를 볼 수 있고, 페이지 document/body 폭은 각각 viewport와 같아 가로 overflow가 없다. Mobile 칸반 내부는 현재 진행 카드를 맞춰 보여주도록 `scrollLeft=272`이며 내부 board 폭 1120px은 문서 폭 overflow와 구분된다. 카드 제목은 칸반 타일의 15자 제한으로 `Root HTML alig…`로 줄지만 C11.4 active 배지는 표시된다.

- [Desktop DOM·geometry](./tailnet-agent-browser-desktop.json) · [Desktop 화면](./tailnet-roadmap-agent-browser-desktop.png)
- [Mobile DOM·geometry](./tailnet-agent-browser-mobile.json) · [Mobile 화면](./tailnet-roadmap-agent-browser-mobile.png)
- Screenshot SHA-256: desktop `a89bbb7d9d67d3b554dc0eeae2b5d0e55cc0f31d2a1ff12ceb73d5032737c2b3`; mobile `d83547c9d610caf90e27fb8e675c2bfa886e7ed0383570456037e67a12d726a3`.
- Agent Browser를 닫은 뒤 일치 프로세스가 남지 않았다([lifecycle](./agent-browser-lifecycle.json)).

이 확인은 standalone Tailnet roadmap 경로만 갱신한다. 전체 RSPress 문서 site나 GitHub Pages 배포를 뜻하지 않는다. C11.4/C11의 공식 완료 판정, WPT 적합성, physical device/hardware GPU 동작을 추가로 주장하지 않는다.

모든 동기화·화면 evidence 파일의 hash 목록은 [SHA256SUMS.txt](./SHA256SUMS.txt)에 있다. 독립 관점 30개의 후속 문서 검토는 [post-merge doc diff review](./post-merge-doc-diff-review-30.md)에서 확인할 수 있다.
