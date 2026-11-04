# PR #1157 병합 후 Tailnet 로드맵 동기화

## 원본과 공개 route

PR [#1156](https://github.com/ohah/spinon/pull/1156)의 구현은 `2026-11-04T12:52:44Z` Rebase and merge됐고 merge SHA는 `b667a80ea610827ad666e1fe5d3d039676f69b6c`다. 상태·공개 로드맵 문서를 반영한 PR [#1157](https://github.com/ohah/spinon/pull/1157)도 `2026-11-04T13:03:35Z` Rebase and merge됐으며 source revision은 `f2c1448afe0d8f4322d9aa6b392cfa5544c22deb`다. 공식 완료 수는 `72/213`, C11.1/C11은 미완료다.

변경 route는 `/spinon/roadmap.html` 하나다. `STATUS.html`과 다른 preview 경로는 교체하지 않았다. merged `origin/main` source는 560,630 bytes / SHA-256 `3d3f1b9501a70db5613277ca56e64ffaaf0531d19bbc76383654df0aa173d3f0`다.

## Tailscale server와 안전한 교체

Tailscale backend는 `Running`이고 host는 `macstudio.tailed42f2.ts.net`이다. 기존 preview server PID `28040`은 `/Users/yoonhb/Library/Application Support/SpinonPreview/tailnet-preview-server.mjs`이며 `127.0.0.1:8768`에서 계속 응답했다. 서버를 재시작하지 않았다.

교체 전 preview file은 559,228 bytes / SHA-256 `dc5c76792a21ce4edaa1ab0a4430c762a31a9c6d38bd91323b0812aa0c192157`였고 대상 open handle은 없었다. 이전 파일을 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1157-2026-11-04/roadmap.html`에 백업한 뒤 임시 파일을 같은 directory에서 atomic rename했다. 파일 삭제는 없었다. 최종 preview와 merged source의 560,630 bytes 및 hash가 같다.

local 및 Tailnet route는 HTTP 200을 반환했다. cache-busting URL과 요청의 `Cache-Control: no-cache`를 사용한 최종 응답도 source hash와 같았다. 사용자가 지정한 query 없는 URL `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`을 별도로 재확인해 HTTP 200, 560,630 bytes, 같은 SHA-256, PR #1156 marker 1개와 merge SHA를 확인했다. 기계 판독 보고서는 [preflight](./preflight.json), [copy](./copy.json), [hash 검증](./verify.json), [query 없는 URL 확인](./served-route-noquery.json)에 있다.

## Agent Browser 화면 확인

Agent Browser에서 사용자가 지정한 query 없는 Tailnet URL을 열었다. 제목은 `스피논 구현 로드맵`이고 새 PR #1156 카드가 한 번 표시됐으며 merge SHA, C11 미완료 문구와 `72/213`이 보였다. Desktop `1280×1000`에서 문서 폭은 `1280/1280`, mobile `390×844`에서 `390/390`이라 가로 overflow가 없었다. [검증 JSON](./agent-browser-verification.json) · [desktop capture](./tailnet-roadmap-agent-browser-desktop.png) · [mobile capture](./tailnet-roadmap-agent-browser-mobile.png).

확인 후 Agent Browser 세션을 닫았다. `close --all`은 active session이 없다고 응답했고 후속 process 확인에서 agent-browser 실행 파일이 남아 있지 않았다. Tailnet preview server는 사용자가 접근하도록 계속 실행 중이다.

## 범위와 한계

이번 동기화는 정적 `roadmap.html` 한 route만 다룬다. 전체 RSPress build, 모든 문서 링크, `STATUS.html` 렌더링을 검증하지 않았다. C11 또는 WPT 적합성 완료를 뜻하지 않는다. 파일 hash 목록은 [SHA256SUMS.txt](./SHA256SUMS.txt)에 둔다.
