# PR #1316 병합 후 상태·공개 roadmap·Tailnet 동기화

계획 PR [#1316](https://github.com/ohah/spinon/pull/1316)의 Rebase and merge 뒤 공식 상태 기록과 공개 roadmap에 merge SHA를 반영하고, 사용자가 지정한 Tailnet preview의 다음 계획 링크를 갱신했다. C11.4 구현은 시작하지 않았고 공식 완료 수는 `78/214` 그대로다.

## 병합된 계획과 source

- PR #1316은 `2026-11-09T19:35:10Z` Rebase and merge됐다. merge SHA는 `83c1c57e9c448202040f8efe4e12ff8f3d3b8d8c`다.
- Tailnet 게시용 source commit은 `91179a34089c1fa2f3f964a83c4c4b28d424d9f1`, tree는 `684837d3d678de664920e47b080509e64d9fd406`다.
- `spec/STATUS.md`에 PR merge SHA와 계획 검토 결과를 기록했다. 공개 roadmap의 다음 계획 카드에는 C11.4 `min-height:auto` 계획과 PR #1316 병합 링크가 표시된다.
- 게시된 계획 URL: `https://macstudio.tailed42f2.ts.net/spinon/plan/c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum.html`.
- 공식 C11/C11.4 체크는 미완료, 완료 수는 `78/214`, 내부 계약은 `0.1.0`이다.

## 서버와 선택 게시

- 게시 전 Tailscale backend는 `Running`이었고 Mac Studio는 온라인이었다. 기존 preview server PID `90711`은 `127.0.0.1:8768`에서 실행 중이었으며 sync 동안 재시작하지 않았다.
- 선택 RSPress 2.0.22 빌드에서 source file 47개를 사용해 route 117개, 총 `18,289,237` bytes를 만들었다. 이 선택 build는 전체 문서 link validation이 아니다.
- Preflight에서 교체 대상 25개 모두 열린 file handle이 0개임을 확인했다. 47개 route 생성, 25개 route 교체, 45개 route 동일 hash였다. 기존 `index.html`·`404.html`은 제외하고 어떤 route도 삭제하지 않았다.
- Copy 뒤 local route/hash `117/117`, HTTPS HTTP 200 및 SHA-256 `117/117`, 교체 전 backup `25/25`를 검증했다. `previewServerRestarted`는 false다.
- rollback backup은 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1316-c11-4-2026-11-10/pass01`에 보존했다.

선택 route·hash는 [root manifest](./selected-root.json), [publication manifest](./selected-publication.json), [preflight](./preflight.json), [copy/backup](./copy-and-backup.json), [served verification](./served-verification.json)에 있다. 선택 빌드 결과는 [summary](./selected-build-summary.json)에 기록했다. 전체 Rspress `docs:build`은 이 게시 검증과 별도다. PR #1316에서 기록한 전체 build는 1,518개 dead-link source로 실패했으며 green으로 주장하지 않는다.

## Agent Browser 확인

Agent Browser `0.31.1`로 Tailnet roadmap과 새 계획 route를 열었다.

- Desktop `1440×1000`: 다음 계획 링크와 PR #1316 Rebase and merge를 표시했다. 공식 상태 문구는 완료 `78/214`다. document/body 폭은 모두 1440px였다.
- Mobile `390×844`: 같은 계획·병합 상태를 표시했다. document/body 폭은 모두 390px이고 칸반의 가로 스크롤은 페이지 폭을 늘리지 않는다.
- 새 계획 route의 title/H1은 C11.4 `min-height:auto` 계획으로 로드됐다.
- 화면: [Desktop](./tailnet-roadmap-agent-browser-desktop-pr1316-next-plan.png) · [Mobile](./tailnet-roadmap-agent-browser-mobile-pr1316-next-plan.png).
- 이 확인에서 연 `spinon-tailnet-pr1316` Agent Browser 세션은 종료했고 `session info`에서 `active:false`, `pid:null`을 확인했다. Terminal Browser와 simulator는 사용하지 않았고 simulator를 시작하지 않았다.

검증 결과는 [Agent Browser 관측](./agent-browser-verification.json), 종료 상태는 [lifecycle](./agent-browser-lifecycle.json)에 있다. 전체 SHA-256 목록은 [SHA256SUMS](./SHA256SUMS)다.

## 임시 산출물 정리

게시 완료 뒤 작업용 `build/spinon/roadmap-tailnet-sync-pr1316-c11-4-2026-11-10` 35 MB를 삭제했다. Tailnet이 직접 제공하는 preview root, server, rollback backup 및 확인에 필요한 screenshot·hash evidence는 보존했다. 시뮬레이터나 task-owned Agent Browser 프로세스는 남아 있지 않다.
