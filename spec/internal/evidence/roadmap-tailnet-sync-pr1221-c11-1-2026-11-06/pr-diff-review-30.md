# PR #1221 병합 후 Tailnet roadmap evidence diff 검토 30개

대상은 Tailnet sync script, preflight/copy/verify reports, Agent Browser DOM/screenshot/lifecycle 자료, README 및 이 review 파일이다. 각 행은 서로 다른 제공·복구·증거 경계를 확인했다.

| # | 실패 관점 | 결과 |
|---:|---|---|
| 1 | 동기화 source가 병합 전 branch 파일인가 | 통과 — source HEAD `e18223233244ec73bf0766093288a3b3e132e4aa`다. |
| 2 | 문서 PR merge SHA가 source main과 다른가 | 통과 — GitHub merge SHA와 local `HEAD`가 같다. |
| 3 | PR #1220 merge SHA marker가 빠지거나 잘못됐는가 | 통과 — `2e022c9589eacb49ec2d04c5ea0dd1cb3ca575af`를 체크했다. |
| 4 | official count가 `78/214`가 아닌가 | 통과 — source marker와 rendered card 모두 `78/214`다. |
| 5 | C11.1/C11을 완료로 표시하는가 | 통과 — 둘 다 미완료다. |
| 6 | Tailnet host 자체가 다른 장치를 가리키는가 | 통과 — 요청한 `macstudio.tailed42f2.ts.net`을 썼다. |
| 7 | Tailscale backend가 내려가 있는데 copy를 진행하는가 | 통과 — preflight에서 `Running`을 요구했다. |
| 8 | 8768 listener가 여러 개라 server owner가 모호한가 | 통과 — 하나의 listener PID `28040`을 확인했다. |
| 9 | 예상하지 못한 Node server를 재사용하는가 | 통과 — command line이 `tailnet-preview-server.mjs`인지 확인했다. |
| 10 | server가 다른 directory를 제공하는가 | 통과 — cwd가 `SpinonPreview` root인지 확인했다. |
| 11 | target preview root가 symlink인가 | 통과 — 일반 directory이고 canonical path가 root와 같았다. |
| 12 | 대상 파일 외에 다른 preview 경로를 덮어쓰는가 | 통과 — `roadmap.html` 한 파일만 교체 대상으로 고정했다. |
| 13 | 기존 preview 파일이 사라졌거나 다른 hash인가 | 통과 — preflight의 596,819-byte 기존 hash를 기록했다. |
| 14 | 다른 process가 대상 파일을 열고 있는가 | 통과 — target open handle PID가 비었다. |
| 15 | 이전 제공본 백업이 byte-identical하지 않은가 | 통과 — backup hash와 pre-copy target hash가 같다. |
| 16 | 기존 백업을 덮어쓰는가 | 통과 — 새 pass path가 이미 있으면 중단하도록 script가 검사한다. |
| 17 | 복사 중 부분 파일이 server에 보이는가 | 통과 — 같은 directory temp file 후 atomic rename이다. |
| 18 | preview 파일 mode가 변경되는가 | 통과 — 기존 mode `0644`를 새 파일에 보존했다. |
| 19 | replace 직전에 source나 target이 바뀌었는가 | 통과 — copy 단계가 preflight hash를 다시 대조했다. |
| 20 | local HTTP route가 실패하는가 | 통과 — 200, source와 같은 598,625 bytes/hash다. |
| 21 | Tailnet HTTPS route가 실패하는가 | 통과 — 200, source와 같은 598,625 bytes/hash다. |
| 22 | served response가 오래된 cache body인가 | 통과 — verifier는 no-store fetch의 본문 hash를 source와 비교했다. |
| 23 | server 재시작이나 다른 운영 process 종료가 있었는가 | 통과 — server restart는 없었고 다른 process를 종료하지 않았다. |
| 24 | 파일 삭제가 sync에 포함됐는가 | 통과 — script가 deletes를 하지 않음을 기록했다. |
| 25 | RSPress/Pages 배포가 사실과 다르게 주장되는가 | 통과 — standalone raw HTML route 한 개만 갱신했다고 제한했다. |
| 26 | desktop Agent Browser가 새 card를 실제로 렌더링했는가 | 통과 — PR #1220과 merge SHA가 보였다. |
| 27 | desktop layout이 가로로 잘리는가 | 통과 — viewport/document/body 1440px, card x=90 width=928px다. |
| 28 | mobile 화면에 horizontal overflow가 있는가 | 통과 — viewport/document/body 390px, card x=14 width=362px다. |
| 29 | Agent Browser 프로세스가 남았거나 screenshot hash가 틀린가 | 통과 — session list는 비었고 `agent-browser` daemon은 없으며 screenshot 크기/hash를 기록했다. |
| 30 | preview가 C11 implementation/WPT 완료 증거로 확대되는가 | 통과 — evidence는 roadmap 제공 검증뿐이며 implementation 범위는 #1220 한정이다. |

확인 과정에서 제공 대상, 원본, backup 및 served hashes가 일치했다. 상태 집계·C11 완료 승격·CI·Pages 배포를 주장하지 않는다.
