# PR #1184 병합 후 C11 상태·Tailnet 로드맵 동기화

## 병합 결과

구현 PR [#1184](https://github.com/ohah/spinon/pull/1184)은 2026-11-05T14:41:41Z에 Rebase and merge됐고 merge SHA는 `5803547796d7039f591ef979175a36617ce86636`다. PR check rollup은 비어 있으며 main required status checks도 없어 CI 통과로 기록하지 않는다. 구현 slice는 기본 parsed Root HTML intrinsic explicit track과 matched template-area 조합으로 한정한다. C11.1/C11.3.3/C11.4/C11 전체 완료와 WPT 적합성은 미승격이고 공식 집계 `78/214`를 유지한다.

병합 기록은 [상태 대장](../../../STATUS.md), [내부 계약 0059](../../0059-c11-grid-layout.md), [전용 계획](../../../../plan/c11-root-html-intrinsic-template-areas.md), [공개 로드맵 원본](../../../public/roadmap.html)에 반영했다.

## Tailnet 로드맵

Tailscale backend는 `Running`이고 기존 preview server PID `28040`이 `127.0.0.1:8768`에서 응답했다. `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`은 동기화 전에 HTTP 200을 반환했다. Server를 재시작하거나 종료하지 않았다.

`roadmap.html` 한 파일만 교체했다. 교체 전 파일은 574,409 bytes, SHA-256 `dcf7a26107dfb29e18d01cef52ae4214766a5325be2a22390cd2a0ef046b2d36`였고, 열린 파일 handle은 없었다. 이 파일을 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-postmerge-c11-template-areas-pr1184-2026-11-05/pass01/roadmap.html`에 먼저 보존했다. 새 파일은 576,785 bytes, SHA-256 `b8db91205f68c5ee61b29976bb87411bd4e04244c5d7c1219a65bd455c70c871`이며 temporary file에서 atomic rename으로 반영했다. 저장소 원본, local HTTP와 Tailnet HTTPS 응답은 모두 이 새 hash로 일치한다. 다른 파일 삭제나 server restart는 없다. 상세한 preflight·backup·serve 결과는 이 폴더의 `preflight.json`, `copy-and-backup.json`, `served-verification.json`에 기록한다.

## Agent Browser 화면 확인

Agent Browser에서 Tailnet URL을 열어 PR #1184와 merge SHA, 공식 `78/214`를 확인했다. Desktop 1440×1000에서 카드 너비는 928px이고 document/body 너비는 1440px이다. Mobile 390×844에서 카드 너비는 362px, 좌우 여백은 14px이며 document/body 너비도 390px로 가로 넘침이 없다. [Desktop screenshot](./tailnet-desktop.png) · [Mobile screenshot](./tailnet-mobile.png). Viewport와 DOM bounds는 `agent-browser-desktop.json`, `agent-browser-mobile.json`에 있다. 확인 뒤 해당 Agent Browser session을 닫고 `close --all`을 실행했다. 후속 process inspection에서 `agent-browser` 프로세스는 없었다. [Lifecycle 기록](./agent-browser-lifecycle.json).

## 검토와 한계

상태 대장·내부 계약·계획·공개 카드·Tailnet serving을 서로 대조한 문서 변경 실패 관점 30개는 [PR diff review](./pr-diff-review-30.md)에 있다. 이 동기화는 공개 Tailnet preview만 갱신한다. GitHub Pages는 배포하지 않았다. Simulator 화면은 visual smoke이며 PixelCopy·Metal drawable readback, 실기기/hardware GPU 또는 WPT 적합성 근거가 아니다.
