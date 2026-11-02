# C11 Root HTML percentage auto-row Tailnet 동기화 PR diff 검토 · 30개 관점

**검토 대상:** 최신 main 기반 선택 문서 build, 188-route publication manifest, preview preflight/copy/verify scripts, Agent Browser 결과와 README. 기존 Tailnet preview와 소스·사용자 데이터 경계를 확인한다.

| # | PR diff 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | source가 PR candidate 또는 stale main임 | manifest의 source HEAD는 병합된 main `444304968ff930d23d9c73075db83d0ca97cccb8`이다. |
| 2 | source tree hash가 문서와 다름 | tree `943843d78d1b8ff7f17f8cf278c2cbbf6fe52294`를 manifest에 고정했다. |
| 3 | tracked WIP를 publication에 섞음 | source tracked diff hash는 empty diff의 SHA-256이며 build script가 clean tracked state를 요구한다. |
| 4 | repository path traversal route 허용 | route는 absolute, backslash, `.` 및 `..` component를 거부한다. |
| 5 | tracked source symlink를 따라감 | source component마다 symlink를 거부한다. |
| 6 | generated site symlink를 publication에 복사 | generated tree traversal에서 symlink와 특수 file을 거부한다. |
| 7 | 전체 repository를 무제한 공개 | Rspress selected root 및 명시된 tracked prefix만 선택했다. |
| 8 | untracked raw artifact를 publish | `git ls-files`에 있는 파일만 evidence prefix에서 선택하며 manifest는 untracked publish를 false로 고정한다. |
| 9 | 기존 preview route를 대량 삭제 | 동기화 스크립트는 삭제 작업이 없고 report도 `noDeletes: true`다. |
| 10 | 이전 backup을 덮어씀 | 새로운 `pr1047/pass01` backup 경로가 이미 있으면 preflight가 중단한다. |
| 11 | backend가 내려간 상태에서 진행 | `tailscale status --json`의 `BackendState=Running`을 확인했다. |
| 12 | 다른 preview server를 교체 | listener PID 28040의 args가 예상 `tailnet-preview-server.mjs`이고 `127.0.0.1:8768`에 한정됐다. |
| 13 | preview server를 재시작해 live 사용자를 끊음 | copy·verify에서 restart 없이 PID 28040을 유지했다. |
| 14 | 바뀔 파일이 열려 있는 중 overwrite | preflight에서 13 replacement 대상의 open handle은 0개였다. |
| 15 | preflight 이후 대상 파일을 다른 프로세스가 바꿈 | copy 직전 기존 존재 상태와 SHA-256을 다시 비교한다. |
| 16 | copy source가 preflight 뒤 바뀜 | copy 직전에 source SHA-256을 manifest/preflight와 비교한다. |
| 17 | 기존 파일 backup 없이 덮어씀 | 13개 replacement 모두 atomic rename 전 backupRoot로 복사·hash 검증한다. |
| 18 | 파일 일부만 써진 상태로 공개 | 임시 파일을 새 경로에 완성한 뒤 atomic rename한다. |
| 19 | destination mode를 불필요하게 변경 | replacement는 기존 mode를 유지하고 새 파일은 0644로 생성한다. |
| 20 | route·manifest가 일치하지 않음 | copy/verify가 publication 파일과 manifest 경로·개수·size·hash를 비교한다. |
| 21 | local preview가 실제 source와 다름 | local root의 선택 route 188/188 SHA-256 일치를 확인했다. |
| 22 | Tailnet route가 404 또는 stale response를 제공 | HTTPS 188/188 HTTP 200과 예상 body size/SHA-256을 확인했다. |
| 23 | changed route link가 새 card에서 빠짐 | public roadmap의 7개 local link가 각각 publication manifest에 포함됨을 build verifier가 검사한다. |
| 24 | 공식 완료 수나 parent status를 상향 | manifest·page·README 모두 `72/213`, C11 incomplete를 유지한다. |
| 25 | canvas/screenshot 결과를 native pixel parity로 설명 | README에서 desktop/mobile은 Agent Browser capture라고 구분한다. |
| 26 | mobile page-level overflow가 새로 생김 | Agent Browser에서 width 390 viewport와 document/body scrollWidth 390을 확인했다. |
| 27 | 기존 SVG clipping을 숨김 | overview SVG의 오른쪽 잘림을 mobile limit로 명시하고 screenshot에 보존했다. |
| 28 | screenshots가 다른 pass나 route에서 생성 | 둘 다 PR #1047 card가 있는 live Tailnet roadmap의 pass01 screenshot이다. |
| 29 | post-publication browser가 열린 채 남음 | close-all 뒤 Agent Browser daemon/browser와 Chrome process scan을 비웠다. |
| 30 | 전체 docs links 또는 CI를 green이라고 과장 | 선택한 route/hash만 검증하고 full docs link validation과 CI 성공을 주장하지 않는다. |

## 최종 판정

source, 제한 route 집합, atomic backup/copy, live hash 및 Agent Browser 결과가 일치한다. preview server를 재시작하거나 route를 삭제하지 않았고 공식 C11 상태는 승격하지 않았다.
