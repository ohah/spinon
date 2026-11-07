# PR #1250 이후 Tailnet roadmap 동기화 PR diff 실패 관점 검토 30개

검토 대상은 [동기화 보고서](./README.md), preflight/copy/served JSON, fail-closed sync와 read-only verifier, Agent Browser desktop/mobile DOM 및 이미지, lifecycle record, `spec/STATUS.md`의 새 근거 링크다.

| # | 실패 관점 | 결과 |
|---:|---|---|
| 1 | Tailnet backend가 종료됐는데 기존 상태로 가정하는가? | 통과 — sync 실행 시 `tailscale status --json`에서 `Running`을 검사했다. |
| 2 | 8768 포트의 다른 서버에 roadmap 파일을 덮어쓰는가? | 통과 — listener PID `28040`과 `tailnet-preview-server.mjs` command를 확인한다. |
| 3 | preview server가 작업 중 재시작되거나 죽는가? | 통과 — PID `28040` 유지, `serverRestarted:false`다. |
| 4 | target이 일반 파일이 아니거나 open handle이 있는데 교체하는가? | 통과 — sync 전에 regular-file 및 open-handle preflight를 수행했다. |
| 5 | 백업 경로가 기존 자료를 덮어쓰는가? | 통과 — unique pass01 경로와 `COPYFILE_EXCL`을 사용한다. |
| 6 | 이전 target이 백업과 byte-identical하지 않은가? | 통과 — 두 파일 모두 615,087 bytes, 동일 SHA-256이다. |
| 7 | 현재 source hash가 merge 뒤 public roadmap이 아닌가? | 통과 — merged `spec/public/roadmap.html`에서 615,956 bytes와 SHA를 읽었다. |
| 8 | copy 도중 partial file이 제공되는가? | 통과 — same-directory temporary file, `fsync`, atomic rename을 사용한다. |
| 9 | rename 뒤 target hash가 달라졌는가? | 통과 — source와 target SHA-256이 `0ded9f...006a0a`로 같다. |
| 10 | local HTTP route가 404 또는 stale response를 주는가? | 통과 — local response 200, source와 같은 bytes/hash다. |
| 11 | Tailnet HTTPS route가 404 또는 stale response를 주는가? | 통과 — HTTPS response 200, source와 같은 bytes/hash다. |
| 12 | cache 때문에 해시 확인이 잘못되는가? | 통과 — verifier는 `cache:no-store`로 fetch한다. |
| 13 | sync 뒤 PR #1249/merge SHA marker가 빠졌는가? | 통과 — required-marker verifier가 둘 다 확인한다. |
| 14 | 전체 `78/214`가 바뀌거나 C11 완료로 오표시되는가? | 통과 — source marker와 화면에서 counter 및 미완료 상태를 확인했다. |
| 15 | 다음 safe/unsafe end 계획 링크가 누락되거나 다른 경로를 가리키는가? | 통과 — desktop/mobile DOM에서 `plan/c11-root-html-auto-row-safe-end.md` 링크를 확인했다. |
| 16 | desktop viewport가 실제로 1440×1000 CSS px인가? | 통과 — DOM record에 viewport와 DPR을 보존했다. |
| 17 | mobile viewport가 실제로 390×844 CSS px인가? | 통과 — DOM record에 viewport와 DPR을 보존했다. |
| 18 | desktop에서 최신 카드가 숨겨지거나 화면 밖인가? | 통과 — visible bounds x=90, y=243.421875, right=1018이다. |
| 19 | mobile에서 최신 카드가 숨겨지거나 화면 밖인가? | 통과 — visible bounds x=14, y=239.734375, right=376이다. |
| 20 | mobile에서 가로 스크롤/overflow가 생기는가? | 통과 — scrollWidth와 clientWidth 모두 390이고 overflow=false다. |
| 21 | desktop에서 가로 스크롤/overflow가 생기는가? | 통과 — scrollWidth와 clientWidth 모두 1440이고 overflow=false다. |
| 22 | desktop screenshot이 다른 viewport나 이전 문서를 찍는가? | 통과 — screenshot은 1440×1000이고 현재 Tailnet URL DOM record와 연결했다. |
| 23 | mobile screenshot이 다른 viewport나 이전 문서를 찍는가? | 통과 — screenshot은 390×844이고 현재 Tailnet URL DOM record와 연결했다. |
| 24 | DOM record의 screenshot SHA가 실제 파일과 어긋나는가? | 통과 — PNG bytes hash를 record에 저장했다. |
| 25 | PR #1250 본문의 raw screenshot 중 링크가 깨졌는가? | 통과 — Chrome, Android, iOS 세 image의 `complete`와 natural size를 Agent Browser에서 확인했다. |
| 26 | PR 이미지 크기를 simulator/device 증거 구분 없이 과장하는가? | 통과 — AVD/iOS Simulator 화면이라는 label을 보존했다. |
| 27 | preview의 기존 서버 파일이 실행 중인데 backup 없이 교체하는가? | 통과 — old target을 교체 전에 unique backup에 복사하고 hash를 맞췄다. |
| 28 | agent-browser를 닫은 뒤 프로세스가 남는가? | 통과 — close 뒤 `agent-browser`, `terminal-browser`, `HeadlessChrome` process scan은 빈 배열이다. |
| 29 | 브라우저 종료 과정에서 Tailnet preview server까지 닫히는가? | 통과 — process scan과 별도 listener 검사에서 PID `28040`이 계속 실행 중이다. |
| 30 | GitHub Pages/`gh-pages`에 의도치 않은 배포가 발생하는가? | 통과 — 보고서·검증 JSON 모두 Pages 배포를 하지 않았다고 기록한다. |

이 PR은 제공본 동기화와 브라우저 검증 evidence만 추가한다. 제품 구현이나 C11 완료 상태를 변경하지 않는다.
