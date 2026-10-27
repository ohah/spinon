# Tailnet 로드맵 동기화 PR diff 검토 · 30개 관점

이 PR의 게시 스크립트, hash·백업 원장, Agent Browser 관측, 화면, 범위 설명을 서로 다른 실패 경로로 점검했다. 게시 대상 확대, 기존 파일 유실, 거짓 완료 상태, 잘못된 화면 증거나 프로세스 잔존 주장을 발견하지 못했다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | Tailnet에 게시한 source tree가 PR #680 병합 tree와 다르다. | preflight에서 두 tree가 `3396de1a...`로 일치함을 확인했다. |
| 2 | 빌드 source가 병합 전후에 달라졌는데 같은 것으로 취급한다. | PR #680 main tree와 선택 build source tree를 대조했다. |
| 3 | Tailscale backend가 꺼졌는데 이전 응답을 서버 정상으로 설명한다. | 사전 원장에 backend Running과 온라인 상태가 기록됐다. |
| 4 | Mac Studio가 Tailnet에서 offline이다. | preflight가 `Self.Online=true`와 DNS name을 확인했다. |
| 5 | Tailnet DNS certificate가 다른 host를 가리킨다. | `macstudio.tailed42f2.ts.net` cert domain을 검사했다. |
| 6 | preview listener가 없거나 다른 interface에만 열린다. | PID 28040의 `127.0.0.1:8768` LISTEN 결과를 기록했다. |
| 7 | 원격 HTTPS 경로가 게시 전부터 실패한다. | 같은 Tailnet roadmap의 사전 HEAD 요청이 HTTP 200이었다. |
| 8 | preview root 또는 교체 경로에 symbolic link가 있다. | root와 각 component를 `lstat`해 symlink를 거부한다. |
| 9 | 백업 경로가 이미 존재해 이전 backup을 덮어쓴다. | 새 날짜·pass 경로가 비어 있음을 확인하고 재사용을 거부한다. |
| 10 | route에 `..`, 절대 경로나 구분자 주입이 있다. | build manifest와 sync script가 route component를 검증한다. |
| 11 | 교체 중인 파일을 서버 또는 다른 프로세스가 열고 있다. | 교체 전 lsof 검사에서 7개 대상이 모두 닫혀 있었다. |
| 12 | 사전 검사 후 target이 달라졌는데 변경을 덮어쓴다. | sync 직전 target의 존재, 길이와 SHA를 preflight와 다시 비교한다. |
| 13 | 새 route를 create로 분류했으나 copy 전에 파일이 생긴다. | create 경로는 즉시 다시 비어 있는지 검사하고 exclusive link를 쓴다. |
| 14 | target write 중 부분 파일이 외부에 보인다. | 임시 파일을 fsync한 뒤 같은 directory에서 rename/link한다. |
| 15 | 교체 전 원본 backup이 변형되거나 빠진다. | 교체 7건을 byte-exact hash와 size로 검사한다. |
| 16 | backup에 기존 파일 mode가 보존되지 않는다. | 각 기존 target mode를 backup 및 교체 파일에 전달한다. |
| 17 | 동기화 스크립트가 더 이상 필요하지 않은 route를 삭제한다. | 어떠한 route도 삭제하지 않았고 원장에 `no_deletes=true`다. |
| 18 | 기존 파일을 불필요하게 교체한다. | 내용이 같은 5개 route는 unchanged로 두었다. |
| 19 | 선택 route 수 또는 byte 합계가 report 간에 어긋난다. | build·preflight·copy·verify 원장 모두 84개 route와 일치한다. |
| 20 | local copy가 build 산출물과 다르다. | local 84/84의 size와 SHA-256이 build manifest와 일치한다. |
| 21 | 대체본 백업이 실제 이전 target과 다르다. | backup 7/7 hash와 길이가 preflight의 이전 값과 일치한다. |
| 22 | HTTP 오류 페이지가 정상 body처럼 보인다. | remote 각 route에 대해 HTTP 200, body 길이와 SHA를 모두 요구했다. |
| 23 | CDN 또는 HTTP cache가 옛 화면을 돌려준다. | no-cache 헤더와 고유 query를 사용해 84/84 served body hash를 비교했다. |
| 24 | 화면 링크가 PNG가 아니거나 손상됐다. | selected file에서 PNG signature를 검사했다. |
| 25 | 파일 hash만 맞지만 Agent Browser에서 이미지가 표시되지 않는다. | Agent Browser가 Tailnet Android/iOS URL 각각에서 `complete=true`와 intrinsic dimensions를 확인했다. |
| 26 | roadmap HTML에 새 기록이 없거나 오래된 기록을 표시한다. | Agent Browser에서 PR #679와 merge SHA가 든 direct-text 문단을 관측했다. |
| 27 | 공개 집계나 C11 checkbox가 예기치 않게 바뀐다. | Tailnet 화면의 72/213과 unchecked CSS Grid/C11 상태를 확인했다. |
| 28 | 좁은 선택 build를 전체 docs build 또는 전체 링크 통과로 과장한다. | 18 generated page/84 route 범위와 379개 넓은 스캔 누락을 README에 함께 적었다. |
| 29 | Agent Browser close 이후 이 PR의 Chrome process가 남아 있다. | session 종료 후 정확한 Agent Browser PID 및 Chrome group을 끝내고 빈 session list를 기록했다. |
| 30 | 작업용 브라우저 종료가 일반 사용자 Chrome이나 다른 session을 중단한다. | 종료는 이 evidence의 Agent Browser PID/process group에 한정하고 다른 browser는 건드리지 않았다. |

근거 파일은 같은 evidence directory의 build/preflight/copy/verify 원장, 압축된 넓은 링크 스캔, pass02 direct-link 검증, Agent Browser JSON 및 lifecycle 기록이다. 검토는 선택 route 게시를 대상으로 하며 전체 문서 링크 그래프나 C11 구현 완료를 뜻하지 않는다.
