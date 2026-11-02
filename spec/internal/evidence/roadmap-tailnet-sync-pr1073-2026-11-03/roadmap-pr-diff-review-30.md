# PR diff 적대 검증 30개 — #1073 병합 후 Tailnet roadmap 동기화 근거

대상은 선택 RSPress 발행·Tailnet 복사/검증 스크립트, manifest와 실행 결과, Agent Browser 화면 증거 및 이 README다. 각 항목은 별도 실패 경로 또는 안전 불변 조건이다.

| # | 독립 실패 경로 / 불변 조건 | 확인 결과 |
|---:|---|---|
| 1 | source branch가 PR #1073 미병합 revision을 가리킴 | source HEAD `227011db0df2cf403de779cde4e70e2a06551bb5`가 PR #1073 merge SHA와 일치한다. |
| 2 | preview card가 다른 구현 PR을 가리킴 | HTML marker에서 implementation PR #1072 및 merge SHA `6508a1a91d04c7e83c00b10f230519d52e8ed86b`를 확인했다. |
| 3 | 전체 Grid 기능이 완료로 승격 | publication, README와 Browser 관찰 모두 C11.1/C11 미완료다. |
| 4 | 공식 완료 수가 변경 또는 누락 | Tailnet roadmap와 status 페이지에 `72/213`이 표시된다. |
| 5 | C11 checkbox가 의도치 않게 체크 | Agent Browser에서 disabled checkbox의 `checked=false`를 확인했다. |
| 6 | 선택 root가 Git 미추적 snapshot을 수집 | source 선택은 `git ls-files` 결과만 사용한다. |
| 7 | source 경로에 symlink를 따라가 임의 파일 포함 | 선택 스크립트는 source 각 경로의 symlink를 거부하고 전 파일 존재 여부를 검사한다. |
| 8 | 상위 경로 밖 route를 publication에 쓰기 | route 구성 요소에서 절대 경로, `.`·`..` 및 역슬래시를 거부한다. |
| 9 | RSPress 버전이 프로젝트 pin에서 이탈 | 선택 build는 프로젝트 docs 설치의 RSPress `2.0.22`를 사용했다. |
| 10 | 전체 사이트 build 실패를 성공으로 기록 | 전체 build exit 1과 선택 build 성공을 별개로 기록하고 full-site validation을 주장하지 않는다. |
| 11 | 선택 build가 기존 전체 dead-link 오류를 이번 변경의 green으로 혼합 | 선택 config가 dead-link 검사 비활성인 제한 build임을 README에 적었다. |
| 12 | 선택 source head/tree와 manifest가 달라짐 | 선택 root와 publication 모두 HEAD `227011d…`, tree `38325dc…`를 기록한다. |
| 13 | 선택 파일 수·byte 수가 원본과 어긋남 | root 81 files, 4,421,933 bytes 및 publication 120 routes, 12,858,881 bytes를 manifest로 고정했다. |
| 14 | roadmap가 PR #1072 card 대신 다른 카드로 검증 | 준비·stage·verify 스크립트가 `PR #1072 Rebase and merge` 문단을 고정 선택한다. |
| 15 | PR card 링크 중 internal route가 누락 | stage가 8개 local href 모두 publication route로 존재하는지 확인했다. |
| 16 | 공식 status route가 빠짐 | `STATUS.html` route를 필수 목록과 post-copy HTTP 검증에 포함했다. |
| 17 | C11 plan 또는 percentage-gap plan route가 없음 | 두 계획 route가 선택 publication manifest의 필수 경로다. |
| 18 | 내부 계약 0059가 이전 버전으로 제공 | 내부 계약의 생성 HTML route가 manifest에 있고 local/Tailnet hash가 일치한다. |
| 19 | 구현/계획/PR diff/roadmap review 링크 중 하나가 누락 | 네 evidence HTML route를 필수 publication 목록과 10-route HTTP 검증에서 확인했다. |
| 20 | Android/iOS screenshot 상대 경로 또는 asset bytes가 틀림 | 두 PNG route의 byte 수와 SHA-256을 local·Tailnet 양쪽에서 비교했다. |
| 21 | Tailnet 자체가 내려갔는데 덮어쓰기 진행 | preflight에서 Tailscale `BackendState=Running`과 local HTTP 200을 확인한 뒤 copy했다. |
| 22 | 다른 프로세스가 port 8768을 점유 | 유일한 listener PID `28040`과 `tailnet-preview-server.mjs` 명령을 검증했다. |
| 23 | preview server를 중단하거나 재시작 | 복사 스크립트는 기존 PID를 유지했고 갱신 후에도 local HTTP 200을 기록했다. |
| 24 | source publication이 변경된 뒤 copy | copy 직전 preflight 대상 상태와 source size/hash를 다시 비교한다. |
| 25 | 기존 파일을 truncate해 open reader를 깨뜨림 | 교체 전 reader 두 개를 확인했고 backup 후 same-directory temp와 atomic rename을 사용했다. |
| 26 | rollback용 이전 bytes가 없거나 손상 | 교체 전 13개 route를 고유 backup 위치에 복사하고 기존 hash를 대조했다. |
| 27 | 변경 밖의 기존 route를 삭제 | 코드에 delete 경로가 없고 결과 보고서도 `noDeletes=true`다. |
| 28 | 로컬과 원격이 서로 다른 내용인데 HTTP 200만 보고 | 10개 주요 route 모두 local/Tailnet status 200, byte 수, SHA-256을 publication과 비교했다. |
| 29 | 모바일 레이아웃 overflow를 텍스트 존재만으로 통과 | Agent Browser 390px viewport에서 document/body 390px, card x=14px·폭 362px를 측정하고 화면을 직접 확인했다. |
| 30 | Agent Browser 또는 서버 프로세스 정리 여부를 확인하지 않음 | Browser session/process 종료 후 결과를 별도 확인하고 Tailnet preview server PID는 유지한다. |

## 결과

서로 다른 30개 경로를 확인했다. 이 작업은 선택 문서 route와 responsive 표시·hash consistency만 검증한다. 전체 사이트 dead-link 검사, GitHub Pages 배포, 앱 simulator 및 surface-pixel equality 결과를 주장하지 않는다.
