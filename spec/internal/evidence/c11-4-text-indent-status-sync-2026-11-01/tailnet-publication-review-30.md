# C11.4 상태 route Tailnet 게시 적대 검토 30개

**대상:** selected-root 준비, RSPress 출력 비교, Tailnet route preflight/copy/verify 및 Agent Browser 화면 확인.

| # | 독립 실패 관점 | 판정 |
|---:|---|---|
| 1 | selected-root가 현재 main의 `STATUS.md`가 아닌 오래된 복사본을 사용 | PASS — working-tree SHA와 root overlay SHA를 고정했다. |
| 2 | working-tree에서 무관한 tracked 변경도 선택 root에 게시 | PASS — base가 origin/main이고 tracked 변경은 `spec/STATUS.md`뿐인지 검사했다. |
| 3 | 이전 selected-root에 예상치 못한 파일·symlink가 들어 있음 | PASS — regular file count 434와 symlink 부재를 검사했다. |
| 4 | Rspress 버전이 저장소 pin 2.0.22에서 이탈 | PASS — 실제 package-local CLI가 2.0.22임을 확인했다. |
| 5 | 전체 문서 build를 성공한 것처럼 과장 | PASS — selected-root build 성공만 기록하고 dead-link/image 검사 설정 한계를 명시했다. |
| 6 | `STATUS.md` 두 alias 중 한 경로 누락 | PASS — `STATUS.md`와 `spec/STATUS.md`를 같은 SHA로 overlay했다. |
| 7 | generated site route와 raw Markdown route를 혼동 | PASS — `.html` output과 raw `.md` aliases를 각각 delta에 넣었다. |
| 8 | 이전 publication route를 source list에서 누락 | PASS — 845-route previous manifest를 기준으로 delta를 계산했다. |
| 9 | 파일 삭제로 기존 Tailnet 자료를 제거 | PASS — sync는 create/replace만 수행하고 delete는 없다. |
| 10 | 새 hashed JavaScript chunk가 HTML보다 늦게 반영 | PASS — 72개 새 static route를 같은 copy 단계에서 게시했다. |
| 11 | 새 route가 이미 존재하는데 사용자 파일로 덮어씀 | PASS — previous manifest에 없는 route는 target이 없거나 같은 SHA일 때만 허용한다. |
| 12 | 교체 파일 target이 known previous version이 아닌데 덮어씀 | PASS — 각 target hash가 직전 publication SHA와 정확히 일치해야 한다. |
| 13 | `..`, 절대 경로 또는 separator 변형으로 previewRoot 이탈 | PASS — route를 component별로 검사하고 resolve된 source가 repo 안에 있어야 한다. |
| 14 | 기존 preview root 또는 경로 중간에 symlink가 있어 다른 파일을 변경 | PASS — root와 모든 target parent component의 symlink를 거부한다. |
| 15 | 열린 파일을 무조건 교체 | PASS — PID 28040 FD 428r의 route를 확인하고 보존했다. |
| 16 | 열린 README 본문 자체가 새 status 수정으로 바뀜 | PASS — old/new HTML에서 JS hash 참조만 정규화한 뒤 나머지 body가 동일한지 검사했다. |
| 17 | 열린 route를 건너뛰고 차이를 문서화하지 않음 | PASS — `preservedOpen` 예외와 old/new SHA를 기록하고 기존 파일을 삭제하지 않았다. |
| 18 | open route의 이전 JavaScript chunk를 삭제해 페이지를 깨뜨림 | PASS — 이전 publication 파일은 삭제하지 않았고 old chunk를 보존했다. |
| 19 | server의 Tailnet 상태를 확인하지 않고 쓰기 시작 | PASS — backend `Running`, self online, local port 및 process command를 확인했다. |
| 20 | 다른 프로세스나 listener가 8768을 소유 | PASS — listener 하나와 `tailnet-preview-server.mjs` PID를 확인했다. |
| 21 | backup 경로에 기존 자료가 있는데 재사용·덮어쓰기 | PASS — 고유한 새 backup root가 없어야 copy 단계가 진행한다. |
| 22 | 백업 없이 기존 파일을 교체 | PASS — 71개 기존 target을 교체 전에 별도 root에 복사하고 SHA를 비교했다. |
| 23 | copy 도중 target이 반쪽짜리 파일로 읽힘 | PASS — 같은 디렉터리에 unique temporary file을 쓴 뒤 rename한다. |
| 24 | target 파일 권한 변경 | PASS — 기존 mode를 보존하고 신규 파일에만 0644를 사용한다. |
| 25 | preflight 이후 source가 달라짐 | PASS — 각 source bytes/size/SHA를 copy 직전 다시 검사한다. |
| 26 | preflight 이후 target drift를 덮어씀 | PASS — create/replace/same 상태와 target SHA를 copy 직전에 재검사한다. |
| 27 | HTTP 200만으로 stale bytes를 올바르다고 판단 | PASS — 160개 route마다 HTTPS response bytes와 SHA-256을 manifest에 대조했다. |
| 28 | local target은 검증하지 않고 remote cache만 검사 | PASS — 같은 160개 route의 local path·size·SHA도 독립 검사했다. |
| 29 | 72/213 또는 PR #959 merge가 실제 페이지에 반영되지 않음 | PASS — Agent Browser가 STATUS 페이지 SHA/count와 roadmap의 #959/#962/count를 확인했다. |
| 30 | 브라우저 세션/프로세스를 작업 뒤 남김 | PASS — Agent Browser 세션을 닫고 전용 process 부재를 확인했다. |

## 판정

검토 중 active open file 하나는 atomic replacement라도 기존 파일을 덮지 않도록 보존 대상으로 분리했다. 새 status route와 다른 159개 update route는 검증을 마쳤다. Server PID 28040은 재시작하지 않았다.
