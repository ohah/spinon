# PR #1314 병합 후 상태·공개 로드맵·Tailnet 동기화

이 기록은 구현 PR #1314의 병합 정보와 후속 상태 문서·내부 계약·계획·공개 로드맵을 맞춘 뒤, 사용자가 지정한 Tailnet 미리보기 서버에 선택 문서 경로를 동기화한 결과다.

## 병합된 구현과 공식 상태

- 구현 PR [#1314](https://github.com/ohah/spinon/pull/1314)은 `2026-11-09T17:54:36Z` Rebase and merge됐고 merge SHA는 `352e6c0b41a9fd4442eeaa5205f18dc1c94accc4`다.
- PR 후속 문서 커밋은 `5c7a9a5ba8cdf8b1a9cf44e855f235b986d597b7`, tree는 `543f13f22bcf38357a43441113fffe1749fdd237`다. 이 변경은 `spec/STATUS.md`, 내부 계약 0059, 계획, 구현 evidence와 공개 로드맵에 PR 병합 사실을 반영한다.
- 병합된 구현의 GitHub combined status check와 review 제출은 없었다. 이를 CI 성공으로 계산하지 않는다. 공식 완료는 `78/214`, C11.4/C11은 미완료, 내부 숫자 계약은 `0.1.0`으로 유지한다.
- 수정된 실행 evidence README에서 원본 플랫폼 로그의 줄 끝 공백을 숨기지 않도록 `git diff --check` 설명을 고쳤다. 소스·문서 검사는 통과했으며, raw Xcode/emulator 로그를 포함한 전체 diff 검사는 도구 출력의 공백 줄 끝을 표시한다.

## Tailnet 서버와 제공본

게시 전 Tailscale backend는 Running이었고 `macstudio`가 온라인이었다. `127.0.0.1:8768`의 기존 preview server는 살아 있었으며 공개 로드맵을 HTTP 200으로 제공했다. 따라서 서버가 꺼진 상태는 아니었다. Agent Browser에서 본 본문은 직전 계획 단계라 PR #1314 병합 정보가 빠져 있었다.

첫 안전 preflight는 교체 대상 `spec/internal/0059-c11-grid-layout.md`를 preview server PID 28040이 열고 있어 중단했다. 파일은 변경하지 않았다. 기존 preview server의 정확한 스크립트와 열린 파일 소유자를 확인한 뒤 해당 서버에만 SIGTERM을 보냈다. 관리 프로세스가 같은 스크립트를 PID 90711로 다시 띄웠고 `127.0.0.1:8768`에서 계속 응답했다. 재시작 후 preflight는 교체할 파일에 열린 handle이 없음을 확인했다.

선택 RSPress publication은 source commit `5c7a9a5ba8cdf8b1a9cf44e855f235b986d597b7`, 93 routes, 총 17,657,088 bytes다. 기존 `index.html`과 `404.html`은 건드리지 않았고 route 삭제는 없었다.

- Preflight: 50개 생성, 20개 교체, 23개 동일 hash; 교체 경로 열린 handle 0개. 기존 파일 20개는 backup했다.
- Copy: 50개 생성, 20개 교체, 23개 동일.
- Verify: 로컬 route/hash 93/93, HTTPS HTTP 200 및 SHA-256 93/93, backup 20/20 검증.
- Backup은 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1314-c11-4-2026-11-10/pass01`에 남겨 두었다.

상세 route와 hash는 [선택 root](./selected-root.json), [publication manifest](./selected-publication.json), [preflight](./preflight.json), [copy/backup](./copy-and-backup.json), [served verification](./served-verification.json)에 있다. RSPress 선택 빌드 출력은 [`selected-build.log`](./selected-build.log)다. 기본 전체 문서 build 또는 전체 문서 링크 검증을 통과했다고 주장하지 않는다.

## Agent Browser 확인

Agent Browser `0.31.1` / HeadlessChrome `157.0.0.0`으로 다음 URL을 확인했다.

- Desktop `1440×1000`: PR #1314, UTC 병합 시각과 merge SHA, 2,880/2,880, `78/214`를 확인했다. 문서 폭은 1440px이다.
- Mobile `390×844`: 같은 병합 카드와 공식 `78/214`를 확인했다. document/body 폭 모두 390px로 가로 잘림이 없다.
- 실행 화면: [Desktop 보드](./tailnet-roadmap-agent-browser-desktop.png) · [Desktop PR #1314 카드](./tailnet-roadmap-agent-browser-desktop-progress.png) · [Mobile PR #1314 카드](./tailnet-roadmap-agent-browser-mobile-progress.png).

확인에 사용한 `spinon-tailnet-post1314`, `spinon-tailnet-card-check`, `spinon-tailnet-mobile-card` Agent Browser 세션은 모두 닫았다. 후속 `agent-browser session list`도 `No active sessions`를 반환했다. 프로세스 목록에는 이 작업 세션이 아닌 기존 Terminal Browser가 실행한 Agent Browser sidecar가 남아 있어, Terminal Browser를 사용하거나 종료하지 않도록 그대로 보존했다. 이번 정리 차례에는 새 브라우저 세션을 띄우지 않았다.

## Simulator·작업 임시 자료

이전 구현 실행에서 만든 iPhone 17 Pro 임시 profile과 Android 임시 user-data는 이미 지웠고 공유 AVD 상태·기존 캡처는 보존했다. 이번 확인에서는 iOS `S03` profile이 device inventory에 없었고 부팅된 iOS simulator나 ADB 기기도 없었다. 선택 빌드·publication 확인 뒤 열려 있는 파일이 없는 것을 확인하고 `build/spinon/roadmap-tailnet-sync-pr1314-c11-4-2026-11-10`의 34 MB 생성물을 제거했다. CoreSimulator 시스템 서비스는 부팅 장치 없이도 OS가 관리하므로 종료하지 않았다. 공개 Tailnet preview server는 사용자 페이지 제공에 필요하므로 계속 실행 상태로 둔다.
