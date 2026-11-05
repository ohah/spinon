# PR #1182 병합 후 C11 roadmap·Tailnet 동기화

## 병합된 기능 범위

PR [#1182](https://github.com/ohah/spinon/pull/1182)은 2026-11-05T12:25:28Z에 Rebase and merge됐고 merge SHA는 `8dab763943215e7fc7b5250a8eb22dddba5e811f`다. 제한 구현은 기본 Root HTML direct-text aspect-ratio target 한 item에만 finite direct px `0..=70px` min/max를 연결한다. C11.1/C11은 계속 미완료이고 공식 완료 `78/214`를 유지한다.

이 후속 문서 변경은 [공식 상태 대장](../../../STATUS.md), [내부 계약 0059](../../0059-c11-grid-layout.md), [C11.1 전용 계획](../../../../plan/c11-root-html-grid-text-aspect-ratio-min-max.md)과 [공개 로드맵 원본](../../../public/roadmap.html)을 병합 범위에 맞췄다. 공개 로드맵에는 제한 slice, PR/merge SHA, simulator evidence 링크를 추가했고 완료 체크는 바꾸지 않았다.

## Tailnet 미리보기

Tailnet backend는 `Running`이고 `127.0.0.1:8768` preview server PID `28040`이 응답했다. server는 재시작하거나 종료하지 않았다. 교체 전 대상은 열린 파일 handle이 없었다. [preflight](preflight.json).

- 교체 대상은 `/Users/yoonhb/Library/Application Support/SpinonPreview/roadmap.html` 하나다. 이전 파일은 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-post-merge-c11-rmx-pr1182-2026-11-05/pass01/roadmap.html`에 보존했다.
- 이전 파일은 572,338 bytes, SHA-256 `d31fa3985135c9cb9ffd76d993189fc38af9b752cb4a2f7931d87e7ef79cf985`다. 새 파일은 574,409 bytes, SHA-256 `dcf7a26107dfb29e18d01cef52ae4214766a5325be2a22390cd2a0ef046b2d36`다. 백업 선행 후 atomic rename으로 한 파일만 교체했고 삭제는 없다. [copy 기록](copy-and-backup.json).
- 저장소 source, local HTTP `http://127.0.0.1:8768/spinon/roadmap.html`, Tailnet HTTPS [roadmap](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)은 모두 HTTP 200, 574,409 bytes 및 새 SHA-256으로 일치한다. [served verification](served-verification.json).

## Agent Browser 표시 확인

Tailnet URL을 Agent Browser에서 직접 열었다. PR #1182 card 및 공식 `78/214` 문자열이 desktop/mobile DOM에 보이고, screenshot은 해당 card가 화면에 들어오는 위치에서 저장했다.

- Desktop viewport 1440×1000, document width 1440px, card width 928px. [desktop screenshot](tailnet-desktop.png) · [DOM evidence](agent-browser-desktop.json).
- Mobile viewport 390×844, document/body width 390px, card width 362px, horizontal overflow 없음. [mobile screenshot](tailnet-mobile.png) · [DOM evidence](agent-browser-mobile.json).
- Agent Browser session은 확인 후 닫았다. 이 검사는 공개 roadmap 표시와 반응형 폭만 다루며 앱 동작이나 전체 Grid 지원을 검증하지 않는다.

Evidence hash는 [SHA256SUMS.txt](SHA256SUMS.txt)에 있다.
