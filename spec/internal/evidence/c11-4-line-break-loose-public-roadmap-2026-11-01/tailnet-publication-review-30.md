# C11.4 Tailnet publication diff 적대적 검토 30개

## 검토 범위

선택 root 구성, RSPress build, publication manifest, Tailnet preflight/copy/verify helper, Agent Browser evidence를 서로 다른 실패 관점에서 검토했다. Preview root 밖의 파일은 수정하지 않고, 선택 route 삭제 없이 변경 전 bytes를 backup했다.

| # | 실패 관점 | 확인과 판정 |
| ---: | --- | --- |
| 1 | preview 서버가 Tailscale 종료 상태에서 갱신되는가 | preflight에서 backend `Running`을 요구한다. |
| 2 | 임의의 8768 listener를 Tailnet server로 오인하는가 | PID command line에 `tailnet-preview-server.mjs`를 확인한다. |
| 3 | 다른 preview PID를 중복으로 무시하는가 | TCP 8768 listener가 정확히 하나인지 검사한다. |
| 4 | 서버를 재시작해 사용자 세션을 끊는가 | workflow는 파일 sync만 하고 기존 PID 28040을 유지했다. |
| 5 | 다른 root를 실제 preview root로 착각하는가 | 설정 root realpath가 고정 preview root와 일치하는지 확인한다. |
| 6 | 오래된 PR #956 manifest를 새 source로 잘못 취급하는가 | 이전 529-route manifest는 baseline으로만 해시 검증하고 새 845-route manifest를 따로 만든다. |
| 7 | 새로운 public roadmap 후보가 선택 root에서 빠지는가 | working-tree roadmap hash를 manifest에 결속하고 root alias를 세 route로 구성한다. |
| 8 | source와 staged root 사이에서 파일 bytes가 바뀌는가 | copy 시 source/staged 길이와 SHA-256을 대조한다. |
| 9 | publication에 symlink를 따라 외부 파일을 넣는가 | root·이전 publication·신규 publication을 재귀 검사하며 symlink를 거부한다. |
| 10 | `../` 또는 절대경로가 preview root 밖으로 나가는가 | route마다 절대경로, 역슬래시, dot segment를 거부한다. |
| 11 | 기존 publication hash를 확인 없이 신뢰하는가 | 529개 기존 route 모두 이전 manifest와 길이/hash를 대조한다. |
| 12 | 요구된 card route가 publication에서 누락되는가 | #959/#962 카드 내부 local href를 실제 selected route와 대조한다. |
| 13 | 계획·내부 계약·runtime review 중 하나가 빠지는가 | 14개 핵심 route를 required route 목록으로 고정한다. |
| 14 | 신규 route가 preflight 이후 조용히 달라지는가 | copy 직전에 모든 source SHA를 preflight manifest와 재비교한다. |
| 15 | 기존 target이 preflight 뒤 바뀌었는데 덮어쓰는가 | target 존재 상태와 hash가 달라지면 copy를 중단한다. |
| 16 | 쓰기 중인 target을 덮어써 유실시키는가 | 열려 있는 target handle을 확인하고, 달라지는 open target이 있으면 거부한다. |
| 17 | 삭제 작업이 site cleanup을 통해 섞이는가 | sync helper에는 target deletion이 없고 manifest는 `noDeletes`다. |
| 18 | 열린 파일의 이전 내용을 잃는가 | preflight의 열려 있는 변경 대상은 0개였고 기존 55개를 backup했다. |
| 19 | backup이 기존 사용자 파일을 덮어쓰는가 | 고유 backup root가 이미 있으면 중단하고 각 backup은 `COPYFILE_EXCL`로 만든다. |
| 20 | backup 내용이 원본과 다른가 | backup 후 SHA-256을 target의 사전 hash와 비교한다. |
| 21 | preview에서 부분 파일이 보이는가 | 고유 temp 파일을 완전히 쓴 뒤 `rename`으로 교체한다. |
| 22 | 교체 파일의 기존 접근 권한이 사라지는가 | 기존 mode bits를 temp 파일에 적용한 뒤 교체한다. |
| 23 | 복사 후 local target이 손상되는가 | 각 local file hash와 source hash를 확인하고 전체 845개를 다시 검증한다. |
| 24 | HTTP 상태만 200이고 원격 bytes가 다른가 | HTTPS 응답 길이와 SHA-256을 route마다 대조한다. |
| 25 | percent-encoded route가 원격 URL에서 달라지는가 | route path segment 단위로 encode하여 fetch한다. |
| 26 | 공식 완료 수가 미리보기에서 늘어나는가 | public cards와 `STATUS` 모두 `72/213` 및 미완료 상태를 유지한다. |
| 27 | #959 계획 후보가 구현 완료처럼 보이지 않는가 | 카드가 제한 opt-in fixture, 20/20 flags matrix 및 남은 범위를 적는다. |
| 28 | #962 Simulator screenshot이 glyph paint로 과장되는가 | card는 route/status 화면 한계와 non-goal을 명시한다. |
| 29 | 화면이 모바일 viewport 밖으로 넓어지는가 | Agent Browser에서 document width 390px, card width 362px을 확인했다. |
| 30 | 사용 후 Agent Browser 프로세스가 남는가 | session을 닫고 browser/Chrome 실행 프로세스가 남지 않았음을 확인했다. |

## 결론

845개 selected route의 preflight, copy, backup, local/HTTPS hash 검증과 live browser 화면이 manifest에 연결되어 있다. 전체 docs dead-link 검사는 별도로 통과한 것으로 주장하지 않는다.
