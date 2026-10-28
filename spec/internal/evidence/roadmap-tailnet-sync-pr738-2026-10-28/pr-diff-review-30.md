# PR #738 Tailnet roadmap 동기화 evidence diff 검토 30개

이 PR은 이미 main에 반영된 roadmap source의 선택 route sync 증거와 재현 guard를 기록한다. 앱 구현·문서 source PR의 검토와 별개로, 서버 상태·교체 전후 bytes·backup·browser provenance에서 생길 수 있는 서로 다른 실패를 대조했다.

| # | 별도 실패 가설 | 확인과 결과 |
| ---: | --- | --- |
| 1 | 오래된 source commit을 Tailnet에 복사한다 | script는 local `HEAD`와 `origin/main`이 `23304539ac9d6fab0e79e2bb8cc1154e77cc8469`인지 확인한다. |
| 2 | worktree roadmap이 merge된 main file과 다르다 | script는 source bytes와 `git show origin/main:spec/public/roadmap.html`을 exact 비교한다. |
| 3 | 새 source SHA가 잘못 전사된다 | main file, local route와 Tailnet route는 모두 `0adb06aff0f5c2733a02b20d18099ccad2c6093e060b164a4f8ccad612483ea6`다. |
| 4 | 현재 대상이 예상치 못한 다른 사용자 변경을 포함한다 | 대상은 직전 main source commit `bf4ab21e...`의 정확한 361424-byte 결과와 match했다. |
| 5 | Tailnet backend가 꺼져 있는데 copy가 진행된다 | 사전 `tailscale status --json`은 backend `Running`, host `Online=true`를 기록했다. |
| 6 | 다른 machine의 preview route를 본다 | Tailnet host는 `macstudio.tailed42f2.ts.net`, local route는 `127.0.0.1:8768`로 고정됐다. |
| 7 | 서버를 임의 재시작하거나 PID를 바꾼다 | 기존 preview server PID 28040을 기록하고 재시작 없이 유지했다. |
| 8 | 예상과 다른 local server가 포트 8768을 점유한다 | script는 해당 port의 단일 listener PID를 확인한다. |
| 9 | copy 중 활성 HTTP 요청의 파일 handle이 잘못 처리된다 | 교체 전에 8768의 client TCP 연결과 roadmap 대상 open handle이 모두 없었다. |
| 10 | 두 번째 preview listener를 닫아 다른 작업에 영향을 준다 | port 4174의 별도 PID 33319는 관찰만 했고 종료·변경하지 않았다. |
| 11 | target directory가 symlink로 backup 경계를 벗어난다 | preview와 backup parent는 realpath가 일치하는 non-symlink directory였다. |
| 12 | target 자체가 symlink 또는 비정규 파일이다 | 대상은 regular file이며 symlink가 아님을 확인했다. |
| 13 | 기존 tailnet content를 복구할 방법 없이 덮어쓴다 | overwrite 전 고유한 task backup 경로에 361424 bytes를 copy하고 SHA가 이전 target과 같은지 확인했다. |
| 14 | 기존 backup을 덮어쓰거나 전용 backup을 재사용한다 | script는 task backup root가 사전 부재해야 진행하며 모든 산출물을 `wx`로 생성한다. |
| 15 | 파일을 직접 truncate해 concurrent read에 부분 HTML을 노출한다 | 새 바이트를 같은 디렉터리 temporary file에 쓴 뒤 `roadmap.html`로 atomic rename했다. |
| 16 | atomic replacement가 기존 access mode를 바꾼다 | temp file은 기존 target의 mode `0644`를 유지한다. |
| 17 | 교체 뒤 파일 내용이 source와 다르다 | 설치한 bytes를 main source bytes와 exact 비교했다. |
| 18 | local route가 404 또는 stale response를 반환한다 | 교체 뒤 local route는 HTTP 200, main과 같은 363975 bytes와 SHA를 반환했다. |
| 19 | Tailnet reverse proxy가 다른/stale 내용을 반환한다 | 교체 뒤 Tailnet route도 HTTP 200, local·main과 같은 bytes/hash를 반환했다. |
| 20 | 변경 전부터 Tailnet이 stale했음을 놓친다 | preflight에서 local과 Tailnet 모두 직전 main source hash를 반환했다. |
| 21 | 선택 이외 route나 파일도 함께 교체된다 | script 결과는 `replacedRoutes: [roadmap.html]`, `otherRoutesChanged: false`, `filesDeleted: false`다. |
| 22 | HTTP check를 전체 docs build 또는 GitHub Pages 성공으로 설명한다 | README는 Tailnet static route만 scope로 기록하고 전체 build/Pages 배포를 포함하지 않는다고 명시한다. |
| 23 | Tailnet 화면에서 PR #738 대신 이전 hidden-item 카드만 확인한다 | Agent Browser DOM에서 PR #738 card, Rebase and merge와 merge SHA를 확인했다. |
| 24 | 공식 완료 count가 잘못 증가해 보인다 | live Tailnet DOM에서 C11.4 미완료와 `72/213`을 확인했다. |
| 25 | Browser가 예상한 local file만 보고 live Tailnet을 열지 않는다 | Agent Browser page URL은 실제 `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`이다. |
| 26 | 스크린샷은 target card가 화면 밖에 있는데 관찰로 제출된다 | 첫 smooth-scroll 캡처를 버리고 card rect가 viewport 안임을 확인한 뒤 캡처를 다시 생성했다. |
| 27 | 캡처가 다른 크기 또는 corrupt image다 | 재생성된 PNG는 1280×633이고 view-image로 PR #738 card가 실제 표시됨을 확인했다. |
| 28 | Agent Browser session/browser가 계속 남는다 | close 뒤 session list는 비었고 `agent-browser` process lookup 결과도 없었다. |
| 29 | Terminal Browser로 사용자의 터미널 pane을 점유한다 | 이 확인은 Agent Browser만 사용했고 terminal-browser를 실행하지 않았다. |
| 30 | 다음 실행이 기존 preflight/result를 조용히 overwrite한다 | 재현 script는 이미 생성된 report가 있으면 즉시 거부해 run evidence를 보존한다. |

**판정:** 교체 대상·source revision·전후 HTTP payload·백업·Agent Browser 관찰이 서로 일치한다. server restart, route 밖 변경, 삭제, GitHub Pages 배포는 하지 않았다.
