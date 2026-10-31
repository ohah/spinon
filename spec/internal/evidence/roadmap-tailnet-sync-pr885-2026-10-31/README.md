# PR #885 병합 후 Tailnet 로드맵 동기화 · 2026-10-31

## 기준과 build

최종 기준은 PR #886 병합 뒤 `main` `01dc8487cf56652c1bf733fb393ce35d153005a1` (tree `42a3fcfbdd610495156cdcaf0789b3e3959ec2eb`)이다. PR #885 구현 병합 SHA는 `9ca5c04ac6aaeb45d378b33f1bfcfe616d04cfd3`다. RSPress `2.0.22`로 추린 문서 root 734 files / 27,376,934 bytes를 build해 861 routes / 39,767,695 bytes publication을 만들었다. 전체 문서 링크 검사는 수행하지 않았다.

## Tailnet 반영과 bytes 검증

- 기존 Tailnet preview server PID `28040` (`tailnet-preview-server.mjs`)와 loopback listener `127.0.0.1:8768`을 유지했다.
- `/Users/yoonhb/Library/Application Support/SpinonPreview`에서 861 selected routes를 비교했다. 결과는 create 265, replace 53, same 543, delete 0이다.
- 기존 파일 53개, 7,740,245 bytes를 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr885-2026-10-31/pass01`에 보관했다. Target 자체 검사에서 861/861 source hash, backup 53/53 old hash가 일치했다.
- HTTPS 확인은 861/861 HTTP 200, 861/861 served SHA-256 match, 실패 0이다. 이는 선택된 파일 집합의 bytes 검증이며 전체 문서 링크 검사는 아니다.

근거: [선택 root](./selected-root.json) · [RSPress build log](./rspress-build-pass01.log.gz) · [선택 publication](./selected-publication-pass01.log) · [사전 검사](./preflight-pass01.json) · [copy와 backup](./copy-and-backup-pass01.json) · [target 검증](./target-verification-pass01.json) · [HTTPS 검증 요약](./verify-tailnet-pass01.log) · [served hash 전체 결과](./served-verification-pass01.json) · [파일 hash 목록](./capture-files.sha256) · [Tailnet publication diff 검토](./tailnet-sync-review-30.md).

처음 RSPress 명령에서 Bun option 위치가 잘못돼 usage만 출력했다. 올바른 `bun run --cwd packages/docs docs:build -- --config ...` 명령으로 다시 실행해 build를 통과했다. 실패 출력과 성공 log는 원본 trailing-space formatting을 보존한 gzip archive에 담았다: [실패](./rspress-build-command-failed.log.gz) · [성공](./rspress-build-pass01.log.gz).

## Agent Browser 확인

사용자가 지정한 방법대로 Agent Browser에서 [실제 Tailnet 로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)을 열었다. 렌더된 DOM에서 PR #885 merge SHA `9ca5c04ac6aaeb45d378b33f1bfcfe616d04cfd3`, 공식 완료 `72/213`, C11.4/C11 미완료 문구를 확인했다. [Agent Browser 화면](./tailnet-roadmap-agent-browser.png)과 [검증 결과](./agent-browser-verification-pass01.json)를 저장했다. 확인 뒤 Agent Browser session을 닫았고 `agent-browser` executable process가 없는 것을 확인했다. Terminal Browser는 사용하지 않았다.

GitHub Pages는 배포하지 않았다. PR #885의 제한 slice는 일반 authored CSS, 전체 WPT, glyph pixel 검증 또는 C11.4/C11 완료가 아니다.
