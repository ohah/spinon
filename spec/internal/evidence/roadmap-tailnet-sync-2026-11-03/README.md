# Tailscale 공개 로드맵 C11.1 동기화 · 2026-11-03

`spec/public/roadmap.html`의 C11.1 parsed Root HTML dense auto-placement 기록을 최신 병합 PR #1117로 갱신한다. C11 checkbox와 공식 `72/213`은 유지한다. 새 HTML이 main에 병합된 뒤에만 Tailnet preview의 `/spinon/roadmap.html`을 교체한다.

## 게시 절차

1. 저장소 main에서 `node spec/internal/evidence/roadmap-tailnet-sync-2026-11-03/sync-roadmap.mjs preflight`를 실행해 대상의 현재 byte 수와 SHA-256을 기록한다.
2. `node spec/internal/evidence/roadmap-tailnet-sync-2026-11-03/sync-roadmap.mjs copy preflight.json`로 기존 파일을 별도 backup에 보존한 뒤 atomic rename으로 교체한다.
3. `SPINON_PREVIEW_ORIGIN=https://macstudio.tailed42f2.ts.net node spec/internal/evidence/roadmap-tailnet-sync-2026-11-03/sync-roadmap.mjs verify preflight.json`를 실행한다. 로컬 preview와 HTTPS 응답의 크기 및 SHA-256이 main의 원본과 같고 HTTP 200이어야 통과한다.

이 스크립트는 `roadmap.html` 한 파일만 다루며 삭제를 수행하지 않는다. 대상이 preflight 뒤 바뀌었거나 backup 경로가 이미 존재하면 중단한다. 브라우저 화면 확인은 Agent Browser로 별도 수행한다. 실행 전후 보고서는 같은 evidence directory에 보존한다.

## 현재 상태

- PR #1117과 후속 문서 PR #1118이 merge됐다. official C11 state는 미완료이며 `72/213`이다.
- Tailnet preview는 현재 이전 `roadmap.html`을 서빙한다. 이 PR 병합 후 위 절차를 실행하고 `preflight.json`, `copy.json`, `verify.json`을 추가한다.
