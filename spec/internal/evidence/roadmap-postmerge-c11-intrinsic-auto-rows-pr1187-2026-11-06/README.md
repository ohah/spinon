# PR #1187 병합 후 C11.4 상태·Tailnet 로드맵 동기화

## 병합 상태

구현 PR [#1187](https://github.com/ohah/spinon/pull/1187)은 2026-11-05T17:12:55Z에 Rebase and merge됐고 merge SHA는 `0c32617f76e18694bd692ff84a8035325d2ad020`다. PR check rollup은 비었으며 `main`의 required status checks는 설정되지 않았다. 이 제한 구현만으로 C11.4/C11을 완료 처리하지 않았다. [상태 대장](../../../STATUS.md)의 checkbox와 공식 완료 `78/214`는 유지한다.

## 공개 로드맵 변경

`spec/public/roadmap.html`의 PR #1184 항목 다음에 PR #1187 기록 하나를 추가했다. bounded `grid-auto-rows:min-content` slice, simulator 사실과 한계, broad candidate 133 passed/9 failed 대 HEAD baseline 131 passed/같은 9 failed, 빈 CI check rollup, 미완료 상태와 공식 `78/214`를 함께 적었다. 새 기록의 내부 문서 링크는 GitHub main으로 고정했고 simulator 화면은 merge SHA `0c32617...`의 raw asset URL을 가리킨다. `spec/STATUS.md`와 구현 checkbox는 수정하지 않았다.

## Tailnet 미리보기 동기화

동기화 전 Agent Browser로 `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`을 열 수 있었다. 안전 preflight에서 Tailscale backend `Running`, 기존 preview server PID `28040` (`127.0.0.1:8768`, `tailnet-preview-server.mjs`), local HTTP 200, 교체 파일의 열린 handle 없음이 확인됐다. 바꾼 경로는 `roadmap.html` 하나다. 서버를 재시작하거나 파일을 삭제하지 않았다.

첫 동기화(pass01)에서 이전 파일 576,785 bytes / SHA-256 `b8db91205f68c5ee61b29976bb87411bd4e04244c5d7c1219a65bd455c70c871`을 `~/Library/Application Support/SpinonPreviewBackups/roadmap-postmerge-c11-intrinsic-auto-rows-pr1187-2026-11-06/pass01/roadmap.html`에 보존했다. pass01의 source·preview는 579,313 bytes / SHA-256 `05e26a91ed141dcdd2cdd38316b11c518bcc827256a8028b487ee088b55231bf`로 일치했다. 공개 문구 수정 뒤 pass02로 다시 동기화했고, pass02 직전 파일은 별도 backup `pass02/roadmap.html`에 보존했다. 파일 끝의 추가 공백 줄을 제거한 뒤 pass03으로 최종 동기화해, 이전 pass02 파일은 `~/Library/Application Support/SpinonPreviewBackups/roadmap-postmerge-c11-intrinsic-auto-rows-pr1187-2026-11-06/pass03/roadmap.html`에 보존했다. 최종 source·preview target·local HTTP·Tailnet HTTPS는 모두 579,312 bytes / SHA-256 `41e2fed488313dea51676247b941ab93eb13244b1a2156468eadaefcafaac24b`다. 세 번 모두 임시 파일 생성 후 atomic rename으로 반영했다. pass01 보고서는 [preflight](./preflight.json), [backup/copy](./copy-and-backup.json), [served verification](./served-verification.json)이고, pass02·pass03 결과도 각각 [pass02 preflight](./preflight-pass02.json) · [copy](./copy-and-backup-pass02.json) · [verify](./served-verification-pass02.json), [pass03 preflight](./preflight-pass03.json) · [copy](./copy-and-backup-pass03.json) · [verify](./served-verification-pass03.json)에 있다. 재현 script는 [sync-tailnet-roadmap.mjs](./sync-tailnet-roadmap.mjs)다.

기존 Agent Browser tab은 기본 URL에서 hard refresh 후에도 이전 HTML을 계속 보여줬다. 그 HTML의 정확한 hash는 판정하지 않았다. 기존 stale-view는 [별도 캡처](./tailnet-desktop-stale-cache-pass01.png)로 보존했다. 이전 Browser session을 종료하고 process가 없는 것을 확인한 뒤 새 Agent Browser에서 query 없이 canonical URL을 열었다. 새 PR #1187 항목이 정상 표시됐다. final canonical page에서 Android/iOS screenshot raw links도 실제 로드되어 각각 1080×2400, 1206×2622를 보고했다([image verification](./agent-browser-image-links.json)). 최종 [Agent Browser desktop 캡처](./tailnet-desktop.png)와 [mobile 캡처](./tailnet-mobile.png), 각 DOM·viewport 측정은 [desktop verification](./agent-browser-desktop.json)과 [mobile verification](./agent-browser-mobile.json)에 있다.

Desktop viewport 1440×1000에서 카드 bounds는 x=90px, 너비 928px이다. Mobile viewport 390×844에서 카드 bounds는 x=14px, 너비 362px이며 document width와 viewport width가 모두 390px로 가로 overflow가 없다. 두 viewport 모두 canonical URL에서 PR #1187과 merge SHA, `78/214`, C11.4/C11 미완료 문구, 계획·spec·evidence·screenshots 링크를 표시했다. GitHub Pages는 배포하지 않았다.

## 변경분 검토

병합 후 공개 roadmap과 Tailnet 동기화 artifact에 대한 30개 서로 다른 실패 관점은 [roadmap PR diff review](./pr-diff-review-30.md)에 남겼다. 최종 Agent Browser 종료와 process 확인은 [lifecycle evidence](./agent-browser-lifecycle.json)에 기록한다. source와 핵심 증거 파일 hash는 [SHA256SUMS](./SHA256SUMS.txt)에 있다.
