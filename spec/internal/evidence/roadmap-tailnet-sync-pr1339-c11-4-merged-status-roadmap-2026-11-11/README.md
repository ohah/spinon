# PR #1339 병합 후 Tailnet 로드맵 동기화

문서 PR [#1339](https://github.com/ohah/spinon/pull/1339)은 `2026-11-10T16:36:08Z` Rebase and merge됐고 merge SHA는 `8ad25ed9bfd7a2a1295e16d606648e282f51d9b8`이다. 공개 Tailnet 미리보기는 병합 뒤 이 main tree에서 선택 문서 사이트를 만들고 기존 preview server에 동기화했다.

## Source와 build

- 병합된 `origin/main` HEAD는 `8ad25ed9bfd7a2a1295e16d606648e282f51d9b8`, tree는 `2491db9590f922e936b867b6c3a6d19b31513fcd`다.
- 현재 checkout은 PR head `96088f8ec6f7d63c694ae0347e418770ec4691ba`였지만 tree가 main과 동일했다. 선택 root 스크립트는 두 tree의 일치를 확인하고 모든 입력 파일의 checkout hash를 `origin/main` blob과 대조했다. 선택 대상 디렉터리는 main의 tracked tree에서 열거했다.
- RSPress 2.0.22 build는 성공했고 output 93 files 가운데 HTML file은 38개다. Selected root는 symlink 없이 186 files / 90,288,295 bytes다. 공식 집계는 `78/214`, C11.4/C11은 미완료다.
- [빌드 log](./selected-build-pass02.log) · [selected-root manifest](./selected-root-pass02.json) · [publication manifest](./selected-publication-pass02.json)

## Tailnet 동기화와 hash

- Tailscale backend는 `Running`이었다. 기존 preview server PID `90711`가 `127.0.0.1:8768`에서 듣고 있었고 공개 `/spinon/roadmap.html`은 동기화 전 HTTP 200이었다. Server를 재시작하지 않았다.
- Publication은 275 routes / 104,337,918 bytes다. 기존 경로 187개가 같았고 41개를 교체했으며 47개를 추가했다. 교체 전 열린 파일 handle은 0개였고 기존 파일 41개를 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1339-c11-4-merged-status-roadmap-2026-11-11/pass02`에 백업했다. 파일 삭제는 없었다.
- 동기화 후 local preview와 HTTPS preview 모두 275/275 route의 HTTP 200 및 SHA-256 일치를 확인했다. 이전 파일 백업도 41/41 hash가 일치한다. Preview server는 계속 PID `90711`을 사용한다.
- [preflight](./preflight-pass02.json) · [copy/backup report](./copy-and-backup-pass02.json) · [local/HTTPS verification](./served-verification-pass02.json)

## Agent Browser 화면 확인

Agent Browser 0.31.1의 새 세션에서 공개 Tailnet URL을 열어 확인하고 캡처했다. GitHub API가 읽은 공식 상태는 두 viewport 모두 `78/214`였다. `세 fit row-span`은 `진행 중`의 `1/2` column에 한 번만 나타났고, Kanban heading은 하나다. 모바일의 viewport/body 폭은 390px이며 Kanban은 362px 컨테이너 내부에서 1,120px 가로 scroll을 갖는다.

- [데스크톱 1440×1000](./tailnet-roadmap-agent-browser-desktop.png) · SHA-256 `e437e1a3667089bb472d0c87f199a47e1cdeecfc344b0e8a130d008daba40a9b`
- [모바일 390×844](./tailnet-roadmap-agent-browser-mobile.png) · SHA-256 `92cb83ccc7debe42e2ce10640fa9e0b0d51de5c6176f725c0544be55220020f9`
- [Agent Browser 측정 결과](./agent-browser-verification.json)

Agent Browser 세션을 닫았고 종료 뒤 세션 목록은 비어 있었다. 종료 시점에 Agent Browser process와 booted iOS simulator는 없었다. Tailnet preview server는 사용 중이므로 유지한다.

이 결과는 Tailnet 정적 문서 route와 두 viewport의 화면 확인이다. 전체 문서 링크 적합성, CI, 제품 runtime 또는 C11.4 전체 완료를 뜻하지 않는다. GitHub Pages 배포는 하지 않았다.
