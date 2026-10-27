# PR #675 Tailnet roadmap 동기화 실패 관점 검토 30회

**대상:** 선택 route 동기화 스크립트, preflight/결과 원장, Tailnet Agent Browser 관찰·화면 및 이 README
**판정:** 서로 다른 파일 경계·복구·서빙·검증 실패 조건 30개를 확인했다.

| # | 실패 관점 | 확인한 조건과 판정 |
| ---: | --- | --- |
| 1 | merge 전 미리보기 원본을 live에 복사 | 원본 HEAD는 병합된 PR #676 main SHA다. 통과 |
| 2 | worktree가 merged main roadmap과 다름 | source bytes를 `git show origin/main`과 byte-exact 비교한다. 통과 |
| 3 | 다른 roadmap content가 복사됨 | source SHA-256을 `4a721943…`로 고정한다. 통과 |
| 4 | 선택 경로가 `STATUS.html`까지 확대됨 | route는 `roadmap.html` 하나다. 통과 |
| 5 | preview root가 기대한 폴더가 아님 | `SpinonPreview` 실경로와 non-symlink 디렉터리를 확인한다. 통과 |
| 6 | 기존 target이 symlink나 디렉터리임 | regular file 및 non-symlink 조건을 확인한다. 통과 |
| 7 | 이전 target이 사전 검사 후 달라짐 | 이전 SHA와 크기를 고정해 변경 시 중단한다. 통과 |
| 8 | 백업 파일이 기존 경로를 덮어씀 | backup root가 없음을 검사하고 `COPYFILE_EXCL`로 생성한다. 통과 |
| 9 | 백업이 원본 바이트를 보존하지 못함 | backup bytes와 SHA를 이전 target과 대조한다. 통과 |
| 10 | 교체가 백업보다 먼저 실행됨 | byte-exact backup 후에만 새 파일을 설치한다. 통과 |
| 11 | 부분 작성된 HTML이 서비스됨 | 같은 디렉터리 임시 파일을 fsync 후 atomic rename한다. 통과 |
| 12 | atomic 교체가 파일 mode를 바꿈 | 기존 target mode를 임시 파일에 적용한다. 통과 |
| 13 | 동시 서버 request 중 truncate로 빈 문서 반환 | 기존 inode를 truncate하지 않고 pathname을 atomic rename한다. 통과 |
| 14 | 기존 open descriptor를 잘못된 프로세스가 점유 | `lsof` handle은 preview server PID의 read descriptor뿐이었다. 통과 |
| 15 | idle handle을 활성 request로 오해 | 서버 TCP 목록에는 listener 외 active connection이 없었다. 통과 |
| 16 | 요청마다 경로를 다시 읽지 않아 서버 재시작 필요 | 서버 source의 `stat`과 `createReadStream` 호출이 GET마다 경로를 연다. 통과 |
| 17 | preview server를 재시작하거나 종료 | PID 28040이 계속 8768 listener를 유지했다. 통과 |
| 18 | 오래된 Tailnet route를 교체 전 확인하지 않음 | 교체 전 Tailnet GET은 HTTP 200 및 이전 hash였다. 통과 |
| 19 | 교체 전 local route가 target과 불일치 | local GET은 HTTP 200 및 이전 target hash였다. 통과 |
| 20 | Tailscale이 offline인데 파일만 복사 | sync 전에 backend `Running`을 확인한다. 통과 |
| 21 | 새 file이 잘렸거나 누락됨 | target bytes가 source 336,731과 같다. 통과 |
| 22 | 로컬 route가 새 pathname을 읽지 못함 | 교체 후 local route가 HTTP 200과 source hash를 반환한다. 통과 |
| 23 | Tailnet proxy가 오래된 route를 계속 반환 | Tailnet GET이 HTTP 200과 새 source hash를 반환한다. 통과 |
| 24 | 원격 응답 hash만으로 UI 가시성을 주장 | Agent Browser에서 제목과 PR #675 카드 텍스트를 별도 확인한다. 통과 |
| 25 | merge SHA 링크가 로드맵에서 잘못 표시 | Agent Browser body text에서 `96a392d3…`를 찾았다. 통과 |
| 26 | 공식 진행 수가 roadmap에서 어긋남 | Agent Browser body에 `72/213`이 보였다. 통과 |
| 27 | C11.4 상태가 완료로 오해됨 | 카드와 관찰 JSON 모두 미완료 상태를 표시한다. 통과 |
| 28 | Terminal Browser를 사용하거나 증거 경로를 혼동 | Agent Browser 캡처만 사용했다. 통과 |
| 29 | 종료 시 별도 Agent Browser 세션까지 닫음 | 이 작업의 세션을 닫고 HWP 세션은 건드리지 않았다. 통과 |
| 30 | 화면 캡처를 HTTP/hash 증거 대신 사용 | screenshot은 UI 보조 근거이며 HTTP status/hash는 별도 원장에 기록한다. 통과 |
