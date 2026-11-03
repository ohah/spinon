# C11.1 calc roadmap Tailnet sync PR 실패 관점 검토 30개

현재 PR의 선택 publication manifest, preflight/copy/served verification, RSPress config, sync helper, README와 Agent Browser 캡처를 서로 다른 실패 경로로 검토했다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | publication source가 merge 전 branch 상태인가 | manifest source HEAD/tree가 PR #1098 merge 이후 origin/main과 일치한다. |
| 2 | tracked 미커밋 변경이 publication에 섞였는가 | tracked diff는 빈 tree hash이며 source selection은 git index tracked files다. |
| 3 | 선택 목록에 symlink가 포함됐는가 | selected root와 publication walker가 symlink를 거부하고 manifest에도 symlink 없음으로 기록한다. |
| 4 | route에 traversal이나 절대 경로가 있는가 | stage/sync에서 절대 경로·역슬래시·`.`·`..`를 거부한다. |
| 5 | destination root가 alias/symlink인가 | Tailnet sync preflight에서 preview root의 lstat/realpath를 검사한다. |
| 6 | 다른 서비스가 8768을 점유하는가 | 유일 listener PID 28040의 commandline과 `127.0.0.1:8768` bind를 확인했다. |
| 7 | Tailscale backend가 내려가 있는가 | preflight의 backend 상태는 `Running`이다. |
| 8 | HTTPS가 다른 local preview port로 연결되는가 | `tailscale serve status`가 443 → 8768을 표시한다. |
| 9 | 동기화 전에 기존 page 응답이 끊겼는가 | local과 HTTPS roadmap가 모두 HTTP 200이며 baseline hash가 일치했다. |
| 10 | RSPress 전체 build의 unrelated 오류를 전체 성공으로 취급했는가 | README가 실패 원인과 선택 build 범위를 별도로 명시한다. |
| 11 | dead-link 검사가 켜진 것으로 주장하는가 | 선택 config는 링크 검사를 끄고 전체 링크 검증이 아니라고 기록한다. |
| 12 | publication route 수와 manifest가 다른가 | stage helper가 file walk와 manifest route set을 같게 검사한다. |
| 13 | route alias가 서버가 열어 둔 다른 파일을 바꾸는가 | 최종 목록은 열린 무관 `spec/public/roadmap.html` alias를 제외한다. |
| 14 | 바꾸려는 파일이 열려 있는 동안 교체되는가 | preflight 대상의 open file handle은 0개다. |
| 15 | preview 외 다른 경로를 덮어쓰는가 | destination을 사용자가 지정한 SpinonPreview root로 고정한다. |
| 16 | 기존 백업을 덮어쓰는가 | unique `pass04` backup 경로를 사전 확인하고 기존 경로를 거부한다. |
| 17 | backup 데이터가 새 파일로 오염됐는가 | 교체 전 9 backup bytes의 SHA-256을 이전 target hash와 9/9 대조했다. |
| 18 | 복사 중 partial file이 공개되는가 | 임시 고유 파일을 기록한 뒤 atomic rename한다. |
| 19 | permission mode가 뜻하지 않게 바뀌는가 | replace는 기존 mode bits를 유지하고 create는 0644를 사용한다. |
| 20 | 선택하지 않은 preview 파일을 삭제하는가 | `noDeletes=true`; route deletion 단계가 없다. |
| 21 | 동일 파일까지 교체해 불필요한 churn을 만드는가 | 22 route는 동일 hash라 건너뛴다. |
| 22 | 새 route의 생성/교체/동일 수가 맞지 않는가 | preflight와 copy manifest 수가 76/9/22로 일치한다. |
| 23 | roadmap가 이전 PR #1093 상태로 남아 있는가 | served card에 #1097 merge 시각과 merge SHA가 확인된다. |
| 24 | page가 C11.1/C11 완료로 표시되는가 | roadmap와 STATUS served output은 C11 미완료, `72/213`이다. |
| 25 | calc 계획 링크가 build selection에서 빠졌는가 | 새 roadmap 내부 5개 route가 stage manifest에 포함되고 served된다. |
| 26 | simulator README와 review route가 빠졌는가 | runtime README와 review URL의 local/HTTPS 200/hash가 일치한다. |
| 27 | Android/iOS screenshot link가 wrong path를 가리키는가 | 두 PNG route가 HTTP 200이며 Agent Browser에서 각각 1080×2400/1206×2622로 열렸다. |
| 28 | mobile에서 C11 card가 viewport 밖으로 나가는가 | 390px viewport에서 document width 390px, card x=14px·width=362px다. |
| 29 | Agent Browser daemon/session을 남기는가 | session list는 비었고 agent-browser/Chromium process scan도 0이었다. |
| 30 | Tailnet visual/runtime 근거를 pixel parity 또는 CI로 확대하는가 | README는 visual smoke, no pixel readback, empty CI rollup을 각각 제한한다. |

## 결과

최종 pass04 publication에서 남은 sync 결함은 발견하지 못했다. 이전 넓은 selection에서 열려 있던 무관 route는 교체하지 않고 대상에서 제외했다. 최종 107-route selection의 local/HTTPS status와 SHA-256 검증은 통과했다.
