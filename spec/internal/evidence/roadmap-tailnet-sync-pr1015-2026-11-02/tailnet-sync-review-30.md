# PR #1016 병합 후 C11.1 Tailnet roadmap 게시 검토 30개

**검토 범위:** PR #1016 병합 후 선택 문서 route 렌더링·게시 스크립트·manifest·백업/HTTP 결과·Agent Browser 데스크톱/모바일 캡처. 이 문서는 기존 C11 구현 코드 검토를 대체하지 않는다.

| # | 실패 관점 | 판정 및 근거 |
|---:|---|---|
| 1 | 게시 source가 병합되지 않은 후보일 수 있음 | **PASS** — source HEAD `3c009d316f270fbfbca55fc8813a050d1921a000`가 `origin/main`과 같다. |
| 2 | PR #1016 merge SHA를 잘못 표기할 수 있음 | **PASS** — README는 PR merge SHA와 시각을 별도 기록하고 PR API 결과와 맞춘다. |
| 3 | PR #1015 구현 PR의 빈 check rollup을 CI green으로 오해할 수 있음 | **PASS** — 구현·문서 PR의 check run이 비어 있었음을 기록하고 통과 주장을 하지 않는다. |
| 4 | C11.1 slice를 C11 전체 완료로 표시할 수 있음 | **PASS** — C11/C11.1 완료 여부를 바꾸지 않고 `72/213`을 유지한다. |
| 5 | 오래된 preview source를 다시 게시할 수 있음 | **PASS** — 새 RSPress root와 선택 route는 병합된 main에서 준비했고 source SHA를 manifest에 묶었다. |
| 6 | 기존 서버를 재시작하거나 바꿀 수 있음 | **PASS** — preflight·copy·verify에서 기존 Tailnet 서버 PID `28040`을 확인했고 유지했다. |
| 7 | Tailnet이 끊겼는데도 게시할 수 있음 | **PASS** — preflight와 copy/verify에서 Tailscale backend `Running`을 확인했다. |
| 8 | 다른 로컬 서버에 게시할 수 있음 | **PASS** — listener 하나와 `tailnet-preview-server.mjs` 프로세스를 확인했다. |
| 9 | 전체 4.1 GB preview를 실수로 덮어쓸 수 있음 | **PASS** — 기존 preview를 복사하거나 clean하지 않고 선택 route 112개만 게시했다. |
| 10 | route traversal이나 절대 경로가 대상에 들어갈 수 있음 | **PASS** — route의 절대 경로·역슬래시·`.`·`..` segment를 거부한다. |
| 11 | source symlink가 게시 root로 유입될 수 있음 | **PASS** — source 파일과 evidence tree의 symlink를 거부했다. |
| 12 | 두 source가 같은 route를 다른 내용으로 덮을 수 있음 | **PASS** — route collision은 SHA-256이 다르면 실패한다. |
| 13 | 변경하지 않은 파일도 복사해 부수 변경을 일으킬 수 있음 | **PASS** — 변경 route 99개만 교체/생성하고 동일 route 13개는 그대로 둔다. |
| 14 | 게시 전 상태가 이전 스냅샷과 다를 수 있음 | **PASS** — 현재 경로 집합 91,454개를 preflight에 기록하고 copy 전에 같은 경로 집합인지 다시 비교했다. |
| 15 | 기존 route 삭제를 새 게시에 숨길 수 있음 | **PASS** — verify에서 preflight 경로 집합 누락 수는 0이다. |
| 16 | 기존 파일이 staging 도중 바뀔 수 있음 | **PASS** — 바뀌는 기존 route 각각을 stage SHA·크기와 preflight 및 copy 직전 대조했다. |
| 17 | 열린 파일을 교체해 서버 응답을 손상할 수 있음 | **PASS** — 교체 전 변경 route 핸들 조회 결과는 비어 있었다. |
| 18 | 백업이 이미 있는 폴더에서 덮어써질 수 있음 | **PASS** — 고유 pass 경로를 새로 만들고 백업 파일은 `COPYFILE_EXCL`로만 작성한다. |
| 19 | 백업 파일이 이전 원본과 다를 수 있음 | **PASS** — 교체 파일 15개의 백업 SHA를 이전 route manifest와 대조했다. |
| 20 | 새 파일 게시가 부분 쓰기로 노출될 수 있음 | **PASS** — 같은 route 디렉터리의 임시 파일에 쓴 뒤 rename으로 교체했다. |
| 21 | 기존 파일 권한이 달라질 수 있음 | **PASS** — 교체 route는 이전 mode bits를 유지한다. |
| 22 | manifest가 source와 실제 route 내용을 다르게 말할 수 있음 | **PASS** — 112개 staged route의 크기와 SHA-256을 검증했다. |
| 23 | 새 roadmap source가 실제 게시 파일과 다를 수 있음 | **PASS** — 공개 roadmap source, 선택 root, RSPress 출력과 HTTP served bytes의 해시를 연결했다. |
| 24 | Tailnet host가 stale 캐시나 다른 서버 응답을 줄 수 있음 | **PASS** — localhost와 Tailnet HTTPS에서 필수 각 route를 새 요청으로 받고 원본 hash와 맞췄다. |
| 25 | 필수 문서/캡처 URL이 404일 수 있음 | **PASS** — 11개 필수 route의 22개 local/remote HTTP 응답이 모두 200이다. |
| 26 | Android/iOS 이미지가 누락되거나 다른 빌드에서 올 수 있음 | **PASS** — 각 simulator PNG가 source manifest 및 두 HTTP 응답과 크기·SHA가 같다. |
| 27 | 모바일에서 새 progress card가 가로 넘침을 만들 수 있음 | **PASS** — Tailnet 화면 390×844에서 `scrollWidth=390`이고 PR #1015 카드가 보인다. |
| 28 | 데스크톱에서 카드가 안 보이거나 가로 넘침을 만들 수 있음 | **PASS** — Tailnet 화면 1280×633에서 카드가 보이고 `scrollWidth=1280`이다. |
| 29 | 화면 캡처가 local source를 보여주고 Tailnet으로 잘못 보고될 수 있음 | **PASS** — desktop/mobile 캡처는 실제 Tailnet HTTPS에서 만들었고 본문 merge SHA와 `72/213`을 읽었다. PR #1017의 두 embedded PNG도 Agent Browser에서 `complete=true`, 각 1280×633·390×844로 표시됐다. |
| 30 | 브라우저 프로세스를 남길 수 있음 | **PASS** — 확인 뒤 `agent-browser close --all`을 실행했고 agent-browser/headless-shell process query는 비어 있었다. |

전체 기존 preview 파일 91,454개의 내용을 다시 해시한 미러 감사는 이 범위에 포함되지 않는다. 이 게시 검증은 사전 경로 집합 보존, 변경 route별 이전/신규 해시, 백업, 선택 route 전체와 필수 HTTP 전달을 확인한다.
