# PR #1326 이후 Tailnet roadmap 동기화 · 2026-11-10

## 동기화 source

- 구현 PR [#1326](https://github.com/ohah/spinon/pull/1326)은 `2026-11-10T05:39:19Z` Rebase and merge됐고 merge SHA는 `b5925aa274718a16c4e8f0bd9ef4bed4423aec6c`다.
- 문서 후보 HEAD는 `a03f1d5b4e6591c587fc5cc253e692717642bf64`, 선택 source tree는 `bc1f96662ea7a12c30b489d145cfdc1b4b51f19e`다. 이 source는 구현 PR의 merge metadata·지원 경계·검증 한계를 기록한다.
- 공식 완료는 `78/214`; C11.4/C11은 미완료, internal numeric contract는 `0.1.0`이다.

## 선택 빌드와 Tailnet 파일

- RSPress `2.0.22`에서 선택 source 121개(31,722,480 bytes)를 260 route(48,060,691 bytes)로 만들었다. 전체 문서 dead-link 검사는 수행하지 않았다.
- Tailscale backend는 `Running`; 기존 preview server PID `90711`은 `127.0.0.1:8768`에서 응답했다. 서버를 재시작하지 않았다.
- Pass03 preflight은 새 route 65개, 교체 64개, 동일 131개를 확인했다. 변경 대상 open handle은 0개였고 route 삭제는 없었다.
- 교체 전 파일 64개는 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1326-c11-4-2026-11-10/pass03`에 백업했다.
- 동기화 뒤 local 260/260 route와 HTTPS 260/260 route에서 HTTP 200·크기·SHA-256이 선택 publication과 일치했다. 교체 전 백업도 64/64 hash 일치를 확인했다.

선택 source·publication·preflight·copy·serve 결과는 `selected-root-pass03.json`, `selected-publication-pass03.json`, `preflight-pass03.json`, `copy-and-backup-pass03.json`, `served-verification-pass03.json`에 보존했다. Pass02 자료는 재시도 이력으로 남겼다.

## Agent Browser 확인

- 공개 [roadmap 페이지](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)를 Agent Browser `0.31.1`로 최종 확인했다.
- Desktop 1440×1000과 mobile 390×844에서 최신 PR #1326, merge SHA `b5925aa274718a16c4e8f0bd9ef4bed4423aec6c`, 공식 `78/214`, C11.4/C11 미완료 및 PR checks 0개를 확인했다. Mobile document width는 viewport와 같은 390px로 가로 overflow가 없다.
- 캡처: [desktop](./tailnet-roadmap-desktop-agent-browser.png), [mobile](./tailnet-roadmap-mobile-agent-browser.png).
- 검증 후 session `spinon-tailnet-roadmap-pr1326-final-20261110`을 닫았다. Chrome PID `8112`와 임시 profile은 이미 사라졌고, 남은 해당 session daemon PID `8111`을 종료했다. 마지막 조회에서 session 프로세스는 없었다.
- 문서 PR [#1327](https://github.com/ohah/spinon/pull/1327) 본문의 commit-pinned raw image 두 장도 Agent Browser에서 로드됐다: desktop `1440×1000`, mobile `390×844`. [PR 페이지 캡처](./pr1327-agent-browser.png)와 image/lifecycle 검증 JSON을 함께 보존했다. PR check-run/status는 0개였다.

## 작업 후 정리

- S03 이름의 simulator profile은 없었다. Simulator 앱은 닫혀 있었고 iOS simulator와 Android AVD는 모두 shutdown, ADB 기기는 연결되지 않았다. iPhone 17 Pro와 Android AVD는 다른 작업도 재사용하는 공유 profile이므로 데이터는 보존했다.
- 선택 빌드 `pass01`, `pass02`, `pass03` build root는 `lsof +D`에서 open handle이 없음을 확인한 뒤 제거했다. `pass02` JSON·screenshot 근거와 hash 검증을 마친 rollback backup은 보존했다.
- 별도 PR #1326 screenshot-link 확인은 Agent Browser로 Android light/dark 1080×2400 및 iOS light/dark 1206×2622의 commit-pinned raw image 네 장을 열었다. PR #1326 check-run/status/review는 각각 0개였으며 이를 CI green으로 해석하지 않았다.
