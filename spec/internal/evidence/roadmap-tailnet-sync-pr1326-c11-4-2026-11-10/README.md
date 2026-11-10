# PR #1326 이후 Tailnet roadmap 동기화 · 2026-11-10

## 동기화 기준

- 구현 PR [#1326](https://github.com/ohah/spinon/pull/1326)은 `2026-11-10T05:39:19Z` Rebase and merge됐고 merge SHA는 `b5925aa274718a16c4e8f0bd9ef4bed4423aec6c`다.
- 병합 후 상태·문서 동기화 PR [#1327](https://github.com/ohah/spinon/pull/1327)은 `2026-11-10T06:18:10Z` Rebase and merge됐고 merge SHA는 `e0c904ce35f302847dd90567f68c3f45dc015272`다. GitHub check-run/status/review는 각각 0개여서 CI green으로 기록하지 않았다.
- 공식 완료는 `78/214`; C11.4/C11은 미완료, internal numeric contract는 `0.1.0`이다.

## Pass03 이전 배포 기록

Pass03는 PR #1327이 병합되기 전 문서 후보 HEAD `a03f1d5b4e6591c587fc5cc253e692717642bf64`, tree `bc1f96662ea7a12c30b489d145cfdc1b4b51f19e`를 사용했다. 당시 121 source files(31,722,480 bytes)에서 260 routes(48,060,691 bytes)를 만들고 Tailnet을 업데이트했다. 이후 PR #1327에 포함된 구현 근거 README 버전 정정과 SHA256SUMS 갱신으로 선택 source가 바뀌어, 아래 pass04를 최종 배포로 다시 실행했다. Pass03 결과는 이력으로 보존하며 최종 검증으로 취급하지 않는다.

## PR #1327 병합 후 최종 Pass04

- Source HEAD `e0c904ce35f302847dd90567f68c3f45dc015272`, tree `0f8dc428f093ddacfd3e84d7c37eee39d4ec2a58`에서 선택된 121 files(31,722,466 bytes)를 RSPress `2.0.22`로 260 routes(48,060,483 bytes)로 빌드했다. 전체 문서 dead-link 검사는 수행하지 않았다.
- Tailscale backend는 `Running`; 기존 preview server PID `90711`은 `127.0.0.1:8768`에서 응답했다. 서버 재시작은 없었다.
- Preflight에서 새 route 65개, 교체 66개, 동일 129개를 확인했다. 변경 route open handle은 0개였고 삭제는 없었다. 교체 전 파일 66개는 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1326-c11-4-2026-11-10/pass04`에 백업했다.
- 동기화 후 local 260/260 route와 HTTPS 260/260 route에서 HTTP 200·크기·SHA-256이 선택 publication과 일치했다. 교체 전 backup도 66/66 hash 일치를 확인했다.

Pass04 source·publication·preflight·copy·serve 기록은 `selected-root-pass04.json`, `selected-publication-pass04.json`, `preflight-pass04.json`, `copy-and-backup-pass04.json`, `served-verification-pass04.json`에 있다.

## Agent Browser 확인과 정리

- 공개 [roadmap 페이지](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)를 Agent Browser `0.31.1`로 pass04 확인했다. Desktop 1440×1000과 mobile 390×844에서 PR #1326, merge SHA, 공식 `78/214`, C11.4/C11 미완료 및 PR checks 0개가 표시됐다. Mobile document width는 390px로 viewport와 같아 가로 overflow가 없다.
- 화면: [desktop](./tailnet-roadmap-pass04-desktop-agent-browser.png), [mobile](./tailnet-roadmap-pass04-mobile-agent-browser.png). 화면 판정은 브라우저 viewport evidence이며 simulator surface readback이 아니다.
- PR #1327 본문에 넣은 commit-pinned desktop/mobile raw images도 Agent Browser에서 각각 `1440×1000`, `390×844`로 로드됐다. 링크 확인 시 PR은 open/mergeable, checks 0개였고 이후 위 merge SHA로 병합됐다. [PR 화면 캡처](./pr1327-agent-browser.png) 및 상태·이미지 확인 JSON을 보존했다.
- 검증 후 session `spinon-tailnet-roadmap-pass04-20261110`을 닫았다. Chrome PID `58110`은 close 시 종료됐고 남은 agent-browser daemon PID `58109`을 종료했다. 전용 temporary profile은 없어졌으며 해당 session process는 남지 않았다.
- S03 이름의 simulator profile은 없었다. Simulator 앱은 닫혀 있고 iOS simulator 및 Android AVD는 shutdown, ADB 기기는 연결되지 않았다. 다른 작업도 재사용하는 iPhone 17 Pro와 Android AVD profile의 데이터는 보존했다.
- 선택 빌드 pass01–pass03과 최종 pass04의 task build root는 `lsof +D`에서 open handle이 없음을 확인한 후 제거했다. PR screenshot과 route/hash manifests, 검증된 외부 rollback backups는 보존했다.
