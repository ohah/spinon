# C11.4 PR #391 Tailnet 동기화 실패 관점 검토 20개

**검토 대상:** 선택 문서 빌드, Tailnet 미리보기 파일 교체, HTTP/hash 대조, Agent Browser 확인 및 세션 종료. 각 항목은 별도 실패 조건과 해당 검사 경계를 기록한다.

| # | 실패 조건 | 확인할 불변 조건과 판정 |
| ---: | --- | --- |
| 1 | 임의의 디렉터리를 Tailnet root로 덮어쓴다. | sync helper가 source를 저장소의 pass별 빌드 경로, target을 `~/Library/Application Support/SpinonPreview`로 고정한다. |
| 2 | 백업이 다른 작업 또는 기존 pass 파일과 섞인다. | 전용 `c11-4-pr391-tailnet-sync-2026-10-20/pass16|pass17`만 허용하고 기존 백업 바이트가 다르면 중단한다. |
| 3 | 선택 빌드의 source set이 preflight 뒤 달라진다. | copy 단계가 상대 경로 집합, source byte 수와 SHA-256을 다시 대조한다. |
| 4 | destination이 preflight 뒤 다른 프로세스에서 바뀐다. | target 존재 여부, byte 수와 이전 SHA-256을 다시 대조한 뒤 바뀌었으면 중단한다. |
| 5 | 원본이 symlink를 통해 선택 root 밖으로 빠져나간다. | 선택 복사와 최종 빌드 열거에서 symlink를 거부한다. |
| 6 | target/backup 경로 중간에 symlink가 들어간다. | 디렉터리 경로의 각 segment를 `lstat`하고 symlink 또는 비-directory면 중단한다. |
| 7 | 정적 동기화가 선택되지 않은 기존 페이지를 삭제한다. | helper는 source file만 create/replace하며 delete 작업을 수행하지 않는다. |
| 8 | 서버가 읽는 HTML을 truncate해 부분 파일을 노출한다. | 기존 destination을 직접 쓰지 않고 sibling temp file을 완성한 뒤 atomic rename한다. |
| 9 | 교체 파일의 이전 내용이 백업되지 않는다. | replace 대상은 전용 backup에 exclusive create로 저장하고 이전 byte 수/SHA-256을 검사한다. |
| 10 | 같은 바이트의 파일까지 불필요하게 교체된다. | source와 destination SHA가 같으면 `same`으로 기록하고 쓰기 경로를 건너뛴다. |
| 11 | preflight JSON을 다른 pass에 재사용한다. | run, source HEAD, target root와 pass별 backup root의 identity를 확인한다. |
| 12 | preflight에서 예측한 결과와 실제 복사 결과가 다르다. | 각 destination의 사후 byte 수와 SHA-256을 source와 비교한다. |
| 13 | 서버가 열린 destination을 조사하지 않고 덮어쓴다. | copy 직전 `lsof`로 replace 대상별 PID를 보존한다. 열려 있어도 파일 truncate 없이 rename하므로 서버 프로세스를 종료하지 않는다. |
| 14 | Tailnet 서버가 요청한 파일과 다른 로컬 경로를 읽는다. | 서버 PID의 cwd와 listen socket을 확인하고 기준 preview root를 해당 cwd와 비교한다. |
| 15 | URL encoding 때문에 공백/한글 경로 hash 검사가 다른 파일을 요청한다. | served verifier가 각 경로 segment를 URL encode하고 response byte를 로컬 파일 byte와 직접 대조한다. |
| 16 | 404 또는 서버 오류가 같은 body 길이만으로 통과한다. | 각 응답은 HTTP 200, byte 수, SHA-256 세 조건이 모두 맞아야 통과한다. |
| 17 | 검증 표본만 맞고 나머지 산출 파일은 stale하다. | preflight가 선택된 전체 file set을 열거하고 verifier가 전체 항목을 병렬 요청한다. |
| 18 | 빌드 산출물에 host/worktree 절대 경로가 노출된다. | 선택 root 텍스트를 정리하고 산출물 전체에서 `/Users/...` 및 `/private/tmp/...` 경로를 검색한다. |
| 19 | Tailnet 화면이 확인되지 않았는데 브라우저 성공을 보고한다. | Agent Browser에서 roadmap, STATUS, plan, evidence URL의 실제 렌더링 및 C11.4 상태를 확인하고 캡처한다. |
| 20 | 검증 후 Agent Browser나 공유 Tailnet/tmux 프로세스를 남기거나 종료한다. | 이번 작업에서 시작한 브라우저 session 목록이 비었는지 확인한다. Tailnet server와 기존 tmux 세션은 유지한다. |

## 검토 결과

20개 실패 관점을 모두 실행 근거와 대조했다. pass16은 1,606개 중 1,606개 HTTP 200·byte·SHA-256 일치, 교체 86개 backup 86/86 일치, 열린 교체 파일 0개였다. Agent Browser에서 새 C11.4 evidence image 3개가 GitHub blob 링크로 빠지는 결함을 발견했다. pass17 선택 build에 해당 Android/iOS 캡처를 추가했고, Rspress 2.0.22 산출물 1,612개가 HTTP 200 및 byte/SHA-256 1,612/1,612 일치했다. 교체 96개 backup 96/96 일치, 열린 교체 파일 0개였다. pass17 Agent Browser는 roadmap, STATUS, 계획의 병합/미완료 문구와 runtime evidence 이미지 3개를 확인했다.

최종 리뷰 문구를 포함한 pass18 산출물의 전체 hash·backup·served 결과는 [`preflight-pass18.json`](./preflight-pass18.json), [`open-handles-before-copy-pass18.json`](./open-handles-before-copy-pass18.json), [`copy-and-backup-pass18.json`](./copy-and-backup-pass18.json), [`served-verification-pass18.json`](./served-verification-pass18.json)에 기록한다. 서버와 tmux는 유지하며 파일 삭제는 없다. Tailnet 대상을 truncate하지 않고 sibling temp file에서 atomic rename한다.
