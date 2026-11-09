# PR #1318 Tailnet 게시 절차 · 실패 관점 20개

게시 과정과 복사·rollback 경계를 PR diff 검토와 별도로 확인했다. 제품 구현이나 C11.4 완료 검토로 세지 않는다.

| # | 실패 관점 | 확인과 결과 | 판정 |
|---:|---|---|---|
| 1 | Tailscale backend 중단 | preflight에서 backend `Running`을 확인했다. | 통과 |
| 2 | 다른 process가 preview port 점유 | 유일한 listener가 예상된 `tailnet-preview-server.mjs`인지 확인했다. | 통과 |
| 3 | 잘못된 preview root에 복사 | `~/Library/Application Support/SpinonPreview`가 canonical regular directory임을 확인했다. | 통과 |
| 4 | 예상하지 않은 로컬 server 변경 | PID `90711`의 `127.0.0.1:8768` roadmap 응답이 HTTP 200이었다. | 통과 |
| 5 | selected source와 build 결과 불일치 | source HEAD `32b9774f`, tree `69ea5e96` 및 53개 root source file을 manifest에 묶었다. | 통과 |
| 6 | source symlink 통한 경로 이탈 | selected root에 symlink 0개, publication 생성물도 symlink 없이 staging됐다. | 통과 |
| 7 | 빌드 도구 버전 drift | RSPress `2.0.22` 고정 selected build를 기록했다. | 통과 |
| 8 | 선택 빌드를 전체 docs 검증으로 과장 | 135 routes와 전체 dead-link validation false를 함께 기록했다. | 통과 |
| 9 | 생성·교체 대상 수 불일치 | `48 create + 37 replace + 50 same = 135`가 preflight와 copy report에서 일치한다. | 통과 |
| 10 | 열린 target file 교체 | 변경 대상 37개의 preflight 및 copy 직전 열린 file handle이 0개임을 확인했다. | 통과 |
| 11 | 기존 내용 backup 경로 덮어쓰기 | task-specific pass01 backup 경로가 새 경로였고 script가 기존 경로 덮어쓰기를 거부한다. | 통과 |
| 12 | backup 내용 손상 | 변경 전 37개 파일의 SHA-256 backup을 저장하고 37/37을 사후 검증했다. | 통과 |
| 13 | source 또는 target 동시 변경 | copy 전 preflight의 source/target 존재·hash와 각 route를 다시 대조한다. | 통과 |
| 14 | partial file 공개 | 교체 파일을 sibling temp에 기록한 뒤 rename하고 destination hash를 대조한다. | 통과 |
| 15 | 선택하지 않은 Tailnet route 삭제 | `noDeletes=true`; 어떠한 route도 삭제하지 않았다. | 통과 |
| 16 | index 또는 404 route 교체 | 기존 `index.html`과 `404.html`은 publication에서 제외했다. | 통과 |
| 17 | 로컬 preview가 stale | local preview route 135/135의 size/hash가 manifest와 일치한다. | 통과 |
| 18 | HTTPS 제공본 stale 또는 오류 | HTTPS route 135/135가 HTTP 200 및 SHA-256과 일치한다. | 통과 |
| 19 | preview server 또는 전용 browser 변경 | preview server 재시작은 false이며 Agent Browser session은 `active:false`, PID null로 닫혔다. | 통과 |
| 20 | desktop/mobile route·overflow·resource 잔존 | Agent Browser desktop 1440px/mobile 390px에서 PR #1318 상태와 `78/214`를 확인했고 page width overflow가 0이다. 임시 build는 삭제하며 기존 Terminal Browser를 건드리지 않는다. | 통과 |

**결과:** 서로 다른 Tailnet 게시 실패 관점 20개를 확인했다. 게시와 screenshot은 공개 preview 동기화 증거이며 앱 runtime 검증이 아니다.
