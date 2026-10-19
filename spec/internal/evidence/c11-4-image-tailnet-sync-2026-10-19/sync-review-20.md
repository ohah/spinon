# C11.4 Tailnet 미리보기 동기화 검토 20개

실제 정적 preview 대상, 열린 read handle, pass-3/4 파일 복사, HTTP 제공 결과를 기준으로 동기화 작업 자체의 실패 경로를 확인했다.

| # | 실패 관점 | 확인 및 판정 |
| --- | --- | --- |
| 1 | `/spinon/` 요청이 preview 루트의 다른 경로로 해석 | 기존 server handler를 읽어 `/spinon` prefix 제거 규칙을 확인했다. 로드맵·상태·계획 페이지를 해당 base path로 실제 GET했다. |
| 2 | 복사 중 Tailnet 미리보기 server가 내려가거나 다른 경로로 재기동 | 기존 Node server와 Tailscale Serve 설정을 read-only로 확인했다. 동기화는 server/process 설정을 바꾸지 않았다. |
| 3 | 1GB preview 전체를 새 빌드로 교체해 기존 작업 페이지가 사라짐 | 22,014개 기존 파일 가운데 선택 출력만 overlay했다. root 전체 삭제·교체는 없었다. |
| 4 | preflight가 target 상태를 바꾸거나 backup을 미리 덮음 | preflight는 source/target size와 SHA-256만 읽고 기록한다. backup 생성은 copy 단계에서 replacement마다 처음 한 번만 한다. |
| 5 | 선택 빌드 밖 산출물이 실수로 대량 복사 | pass-4는 151개 파일·6,403,775 bytes만 선택했다. 전체 문서 사이트는 빌드하지 않았다. |
| 6 | 호스트 경로나 임의 심볼릭 링크가 preview 경계 밖 파일로 이어짐 | source·target traversal에서 symlink를 거부하고, source root는 worktree 내부인지 검사한다. host path는 정적 text output에서 redaction 후 재검색했다. |
| 7 | 기존 대상의 현재 바이트가 preflight 뒤 바뀌었는데 오래된 backup으로 덮임 | copy는 매 파일의 target 존재 여부·size·SHA를 preflight 값과 다시 비교하고, 달라지면 해당 copy run을 중단한다. |
| 8 | preflight 이후 빌드 산출물이 바뀌어 다른 내용을 동기화 | copy는 파일 집합과 각 source size/SHA를 preflight와 비교한다. 변경된 파일 집합·바이트는 복사 전에 거부한다. |
| 9 | 새 파일을 만들 때 과거 파일을 불필요하게 backup하거나 덮어씀 | `create`, `replace`, `same`을 분리한다. backup은 `replace`에만 만들고 `same` 파일은 쓰지 않는다. |
| 10 | replacement backup이 기존 다른 작업 증거를 덮어씀 | pass-3/4 별도 backup 경로를 사용하고, 기존 backup 경로가 있으면 prior SHA와 일치하지 않는 경우 실패한다. |
| 11 | backup 직후 실제 대상 교체 전에 backup 바이트가 손상 | backup의 byte count/SHA를 target의 preflight 값과 대조한다. pass-4의 17/17 backup이 검증됐다. |
| 12 | 파일을 직접 truncate한 뒤 복사 도중 요청이 부분 바이트를 읽음 | 같은 target 디렉터리에 temp 파일을 쓴 뒤 `rename`으로 원자 교체한다. 부분 내용이 보이는 구간이 없다. |
| 13 | server가 열린 old file descriptor를 잡은 채 중간 내용을 읽음 | pass-3에서 열린 STATUS·contract·JS는 atomic rename으로 old inode를 유지했다. pass-4 교체 대상 41개에는 열린 handle이 없었다. |
| 14 | 실패 뒤 복구하면서 원본 target에 backup을 역복사해 새 상태를 더 손상 | 자동 rollback은 하지 않는다. copy는 완료 후 hash를 검증하고 이전 파일은 별도 backup에 그대로 보존한다. |
| 15 | 일부 파일 누락이나 크기 mismatch를 성공으로 보고 | pass-4 151개 모두 target bytes와 source bytes/SHA가 일치하고 `all_verified=true`다. |
| 16 | 로컬 target만 확인하고 Funnel/Serve가 이전 내용을 계속 제공 | `/spinon/`의 151개 경로를 HTTPS GET해 응답 status·byte count·SHA를 로컬 target과 비교했다. 151/151 exact다. |
| 17 | URL path의 공백·특수문자 인코딩이 바뀌어 잘못된 파일을 검증 | HTTP 검증기는 경로 각 segment를 별도 percent-encode하고 응답 hash를 동일 경로 파일과 대조한다. |
| 18 | RSPress가 public icon이나 static evidence asset 없이 빌드되어 일부 페이지가 누락 | 선택 root에 `icon.svg`, `logo.svg`, roadmap SVG, PNG/JSON/log asset을 명시해 넣고 최종 선택 빌드가 성공했다. |
| 19 | config/script가 RSPress 문서 route로 해석되거나 worker bundle에 host 경로가 남음 | code asset은 staging root의 `public/`로 보내고, build 결과에서 발견한 flexsearch fallback의 절대 host path를 sanitize했다. 남은 host path 파일은 0개다. |
| 20 | 확인 후 Agent Browser daemon이나 기존 preview/Tailscale server까지 종료 | 새 Agent Browser session과 그 process만 닫았다. 기존 Agent Browser PID 44813, preview Node server와 Tailscale Serve 설정은 그대로 두었다. |

## 판정

초기 선택 root에서 누락된 public icon과 config route 문제를 보완하고, 정적 worker bundle의 host path도 제거했다. pass-4 preview 제공은 151/151 hash 일치이며 이전 파일 17개는 backup됐다. pass-5에서는 PR diff 검토 문서와 manifest를 더해 재빌드·재동기화한다.
