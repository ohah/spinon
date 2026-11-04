# PR #1154 Tailnet 로드맵 동기화 근거 diff 적대 검토 30개

검토 대상은 PR [#1154](https://github.com/ohah/spinon/pull/1154)의 sync script, preflight/copy/verify reports, Agent Browser captures 및 README다. base는 공개 roadmap source PR #1153의 merge SHA `287fd4872f64e62eb00584eea70e5a1645cc270c`, 검토 head는 `74c1db0151957328523d122289610e1a1f17ce51`이다.

| # | 적대 관점 | 확인 결과 |
|---:|---|---|
| 1 | 게시 source가 아직 병합되지 않은 문서 branch일 수 있는가 | script가 `spec/public/roadmap.html`을 `origin/main`의 같은 경로와 byte-for-byte 대조한다. 통과. |
| 2 | 공개 preview에서 의도하지 않은 route가 교체되는가 | route는 `roadmap.html` 하나로 고정한다. 통과. |
| 3 | PR #1153의 merge SHA/source commit이 잘못 기록되는가 | README와 reports의 `287fd4872f64e62eb00584eea70e5a1645cc270c`를 GitHub merge 결과와 대조했다. 통과. |
| 4 | 게시 bytes/hash가 repository source와 다른가 | source 559,228 bytes 및 SHA-256 `dc5c7679…`가 copy와 final verify에서 동일하다. 통과. |
| 5 | Tailscale backend가 offline인데 파일만 바뀐 것으로 기록되는가 | preflight와 final report가 `Running`을 기록한다. 통과. |
| 6 | 잘못된 Tailscale host로 remote 검증하는가 | 검증이 `macstudio.tailed42f2.ts.net` hostname과 HTTPS protocol을 요구한다. 통과. |
| 7 | 8768 listener가 알 수 없는 프로세스인가 | 하나의 PID와 `tailnet-preview-server.mjs` command를 확인한다. 통과. |
| 8 | listener가 의도치 않은 interface에 노출되는가 | `127.0.0.1:8768` bind를 확인한다. 통과. |
| 9 | preview server를 증거 수집 중 종료 또는 재시작하는가 | PID `28040`을 그대로 유지했고 README는 재시작하지 않았다고 기록한다. 통과. |
| 10 | preview root 또는 route가 symlink/path traversal로 바뀌는가 | root가 real directory인지 확인하고 file은 일반 파일로 읽는다. route를 고정해 traversal 입력은 없다. 통과. |
| 11 | 대상 파일이 다른 process의 쓰기 중에 교체되는가 | preflight/copy 직전 open handles를 검사했고 결과는 비어 있다. 통과. |
| 12 | 기존 backup을 덮어써 증거를 잃는가 | backup root와 report 파일이 이미 있으면 중단하고 backup은 `wx`로 생성한다. 통과. |
| 13 | 기존 preview가 backup 없이 교체되는가 | 이전 파일을 먼저 복사하고 old SHA `ab7f3666…`를 확인한다. 통과. |
| 14 | 부분 파일이 보이는 순간이 생기는가 | 같은 preview directory에 임시 파일을 만든 뒤 atomic rename한다. 통과. |
| 15 | 게시 절차가 다른 route나 파일을 삭제하는가 | operation은 `roadmap.html` replace만 수행하고 reports는 `no_deletes: true`다. 통과. |
| 16 | remote fetch failure가 성공으로 저장되는가 | preflight/copy/verify 모두 expected status와 hash를 assert한다. 통과. |
| 17 | source가 preflight 이후 바뀌었는데 copy를 계속하는가 | copy/verify에서 source commit, byte count, SHA를 preflight와 다시 대조한다. 통과. |
| 18 | 이전 preview hash와 크기가 preflight 이후 달라져 다른 내용을 덮는가 | copy 직전에 target existence·bytes·SHA를 다시 대조한다. 통과. |
| 19 | 같은 cache key의 stale response를 새 게시물로 착각하는가 | copy-phase fixed query가 old remote hash를 준 기록을 남겼고 README가 성공으로 세지 않는다. 통과. |
| 20 | Tailnet response가 새 source bytes인지 검증하지 않는가 | unique cache-busting query, no-cache header, length 및 SHA comparison을 수행한다. 통과. |
| 21 | browser cache와 별개인 사용자의 exact URL이 낡았는가 | query 없는 HTTPS route도 no-cache로 요청해 HTTP 200과 같은 source hash를 확인한다. 통과. |
| 22 | 동일 PR card가 누락되거나 중복됐는데 페이지 성공으로 표시되는가 | Agent Browser DOM에서 PR #1152 카드 count 1을 확인했다. 통과. |
| 23 | 페이지에 공식 완료 수 또는 C11 상태가 틀리게 보이는가 | rendered card에 `72/213`과 C11 미완료가 보였다. 통과. |
| 24 | page title 또는 route가 사용자의 대상 페이지와 다른가 | Agent Browser title은 `스피논 구현 로드맵`, URL은 지정된 Tailnet `/spinon/roadmap.html`이다. 통과. |
| 25 | 새 카드가 desktop에서 잘리거나 가로 overflow를 만드는가 | 1280×1000에서 `clientWidth=scrollWidth=1280`을 확인했다. 통과. |
| 26 | 새 카드가 mobile layout을 넘기는가 | 390×844에서 `clientWidth=scrollWidth=390`을 확인했다. 통과. |
| 27 | screenshot이 다른 URL 또는 이전 content를 담는가 | 두 Agent Browser screenshot에서 PR #1152 card와 72/213이 보인다. 통과. |
| 28 | screenshot 파일이 evidence 이후 바뀌는가 | desktop/mobile PNG, reports, script의 SHA-256을 `SHA256SUMS.txt`에 고정했다. 통과. |
| 29 | 요청된 Agent Browser 종료 후에도 browser session/process가 남는가 | `close --all` 뒤 active session 0개, matching browser process 0개를 기록했다. Preview server는 별도 프로세스로 계속 실행한다. 통과. |
| 30 | evidence-only PR이 구현·CI 완료를 과장하거나 WIP를 포함하는가 | README는 limited roadmap publication만 설명한다. PR file list는 이 sync evidence directory에 한정되고 CI pass를 주장하지 않는다. 통과. |

결과: 30개 관점에서 필요한 수정은 발견하지 못했다. Tailnet source/local/remote hash는 최종 검증에서 일치한다. C11 완료와 전체 CSS Grid 적합성은 주장하지 않는다.
