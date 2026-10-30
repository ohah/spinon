# C11.4 PR #861 Tailnet roadmap sync · 2026-10-30

PR [#862](https://github.com/ohah/spinon/pull/862)는 2026-10-30T14:49:59Z Rebase and merge로 병합됐다. 이 선택 publication은 병합된 `origin/main` `9450dca8754f7bf63ae271c86e40d5f815c35d2c` (tree `4960286292a086bc4526b366558508849f0158fe`)에서 빌드해 [Spinon Tailnet 미리보기](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)에 동기화했다. GitHub Pages 배포는 하지 않았다.

## 선택 publication과 빌드

- [RSPress 2.0.22 빌드 로그, byte-exact gzip](./tailnet-build-pass01.log.gz)는 성공했다. 전체 문서 사이트가 아닌 선택 root의 162개 route, 총 12,990,739 bytes다.
- 선택 root에는 source 49개와 staged route 99개, 3,943,376 bytes가 들어갔다. `spec/STATUS.md`, `spec/public/roadmap.html`, C11.4 내부 계약·계획, PR #861 사전 비교·runtime 근거·화면과 PR #862의 로드맵 diff 검토를 포함한다.
- 선택 build는 dead-link와 dead-image 검사를 끈 부분 publication이다. 저장소 전체 문서 빌드, 모든 문서 링크와 오래된 로드맵 카드 링크를 검증했다고 보지 않는다.
- 로드맵 카드와 렌더링된 STATUS에서 PR #861, merge SHA `9b7c3a357d94db0e47f4397b73d3376a35de4439`, 공식 전체 집계 `72/213`, C11.4 미완료를 확인했다.

## Tailnet 복사와 HTTP 검증

- 기존 preview 서버 PID `28040` (`tailnet-preview-server.mjs`, `127.0.0.1:8768`)를 확인했고 중지하거나 재시작하지 않았다.
- 사전 비교 대상은 162개다: 신규 129개, 교체 15개, 동일 18개. 파일 삭제는 없었다.
- 교체한 기존 파일 15개, 5,776,631 bytes를 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr861-2026-10-30/pass01`에 byte-exact 백업했다.
- 복사 뒤 로컬 preview 파일 162/162와 backup 15/15의 byte 수·SHA-256이 일치했다. HTTPS는 162/162 route에서 HTTP 200과 source byte 수·SHA-256 일치를 반환했다.
- 다른 process가 대상 파일을 열고 있는 경우는 없었다. 복사는 파일별 임시 파일과 atomic rename을 사용했고 삭제는 하지 않았다.

## Agent Browser 확인

Agent Browser에서 로드맵을 열어 첫 카드의 PR #861, merge SHA, `72/213`, C11.4/C11 미완료 문구를 읽었다. STATUS 화면에서도 공식 집계 `72/213`과 C11/C11.4 미완료 checkbox를 확인했다. 카드의 Android nowrap 화면은 1080×2400, iOS nowrap 화면은 1206×2622로 열렸다.

- [로드맵 Agent Browser 화면](./tailnet-roadmap-agent-browser-pass01.png) · [구조화된 browser receipt](./tailnet-agent-browser-pass01.json)
- Agent Browser 창을 닫고 프로세스가 종료된 것을 확인했다. terminal-browser는 사용하지 않았다.

## 재현 자료

- [선택 root 만들기](./stage-root-pass01.mjs) · [root manifest](./selected-root-pass01.json) · [실행 로그](./stage-root-pass01.log)
- [RSPress config](./rspress-pass01.config.ts) · [선택 publication](./select-publication-pass01.mjs) · [publication manifest](./selected-publication-pass01.json) · [선택 로그](./select-publication-pass01.log)
- [Tailnet preflight/copy/local verification](./sync-tailnet-pass01.mjs) · [preflight](./preflight-pass01.json) · [copy와 backup receipt](./copy-and-backup-pass01.json) · [local verification](./local-verification-pass01.json)
- [HTTPS route 검증기](./verify-tailnet-pass01.mjs) · [요약 로그](./served-verification-summary-pass01.log) · [162 route 상세 결과](./served-verification-pass01.json)
- [PR diff 적대 검토 30개](./roadmap-pr-diff-review-30.md)
- [Tailnet server 확인](./tailnet-server-pass01.json) · [첫 stage guard 실패와 복구](./stage-root-config-collision-pass00.log)
- [Tailnet 동기화 PR diff 적대 검토 30개](./tailnet-sync-pr-diff-review-30.md)


저장소 전체 C11 적합성이나 C11.4 완료를 주장하지 않는다. C11.4와 C11은 미완료이며 공식 완료 집계는 `72/213` 그대로다.
