# Tailnet 미리보기 동기화 실패 관점 20개

PR #503 병합 결과를 private Tailnet 문서 미리보기로 반영한 pass28-final 작업을 동기화 대상·백업·동시 접근·served bytes·공식 상태 경계로 나눠 검토했다. 목적은 문서 표시 갱신이며 static preview 밖의 시스템에 쓰지 않는다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 미리보기 source가 작업 중인 변경분 또는 다른 branch commit을 섞음 | 선택 root metadata와 build summary가 source commit `530691ee889b03f40e79e216bab042efd5427e09` 및 그 tree SHA를 기록한다. |
| 2 | 이전 selected root의 내용이 어떤 merge 상태를 대표하는지 모호함 | pass27 metadata의 source SHA `b4fa472...`를 검사하고 그 root의 복제본에서만 작업했다. 이전 root는 덮어쓰지 않았다. |
| 3 | 미추적 workspace evidence가 문서 사이트에 섞임 | pass28 prep은 Git commit object의 계획·STATUS·roadmap·runtime evidence만 복사했고 `ignoredUntrackedWorkspaceEvidence=true`를 남겼다. |
| 4 | archive 또는 selected root에 symlink가 들어가 대상 밖 파일을 읽음 | 새 selected root와 RSPress site의 전체 walk에서 symlink가 0개다. |
| 5 | RSPress 업그레이드가 문서 preview 동작을 바꿈 | 저장소 pin `@rspress/core` 2.0.22로 build했고 package/lock 버전은 수정하지 않았다. |
| 6 | 경로 alias 누락으로 STATUS, plan 또는 evidence 링크가 다른 route를 가리킴 | root alias와 내부 evidence 네 경로를 기존 pass27 layout과 동일하게 유지하고 필수 route 10개를 build summary에서 확인했다. |
| 7 | 최적화된 이미지 경로를 원본 파일명으로 확인해 실제 누락을 놓침 | Rspress 생성 HTML의 두 `img src`와 각 hashed `static/image` asset을 모두 검사했다. |
| 8 | C11.4 체크 또는 완료 수가 simulator slice만으로 승격됨 | 빌드된 roadmap, STATUS 양 alias, plan 모두 PR #503 merge SHA와 `77/213`을 표시한다. C11.4는 unchecked다. |
| 9 | target root가 지정된 private preview 이외의 경로임 | 동기화 script는 `/Users/yoonhb/Library/Application Support/SpinonPreview`만 target으로 고정한다. |
| 10 | preflight 이후 target 파일이 바뀌었는데 덮어씀 | 각 replace 직전에 현재 target의 존재 여부·byte 길이·SHA-256을 preflight 기록과 다시 비교하고 불일치 시 중단한다. |
| 11 | 교체 전에 기존 target bytes가 백업되지 않음 | 280개 replace 파일의 이전 bytes를 전용 pass28-final backup root에 먼저 쓰고 읽어 SHA-256을 검증했다. |
| 12 | 재시도에서 다른 backup을 덮어씀 | 이미 있는 backup은 expected 이전 byte 수와 hash가 일치할 때만 재사용하고 mismatch는 거부한다. |
| 13 | 열린 target 파일을 non-preview 사용자가 참조하는 중 rename | 매 교체 직전 `lsof`를 실행한다. 열려 있으면 정확한 preview server PID·command 및 단일 idle listen socket이 아니면 중단한다. |
| 14 | 확인한 PID가 다른 프로세스 또는 다른 listener임 | PID 1191 command는 `tailnet-preview-server.mjs`, listener는 `127.0.0.1:8768` 한 개였다. 그 process를 종료하거나 재시작하지 않았다. |
| 15 | 복사 중 부분 파일이 preview에 노출됨 | 같은 target directory에 exclusive temporary file을 쓰고 `fsync`한 뒤 atomic rename을 사용한다. |
| 16 | preview tree에서 더 이상 없어야 할 stale 파일을 임의 삭제함 | preflight와 copy report 모두 `no_deletes=true`, 삭제 수 0이다. |
| 17 | 동일 파일도 불필요하게 다시 씀 | preflight는 3,847개 동일 파일을 `same`으로 표시했고 copy 단계에서 이 파일들을 수정하지 않았다. |
| 18 | 로컬 target이 source manifest와 다름 | local verification은 4,534/4,534 target과 280/280 backup을 통과시켰다. |
| 19 | HTTP server가 다른 bytes를 제공하거나 stale inode를 읽음 | `127.0.0.1:8768/spinon/` 응답 4,534/4,534가 source bytes/hash와 일치했다. |
| 20 | 서버 API 성공만으로 실제 사용자 화면을 판정함 | Agent Browser로 Tailnet domain의 roadmap·STATUS·plan·runtime evidence를 열어 merge SHA, `77/213`, page route, 이미지 dimensions/load state를 확인하고 화면 캡처를 보관했다. |

## 판정

이번 pass에서 삭제는 없고 교체 파일 280개는 백업했다. 로컬 target, 이전 파일 백업, HTTP served bytes가 각각 hash 기준으로 일치한다. GitHub Pages 배포, `gh-pages` 수정, 서버 종료, 실기기 연결은 하지 않았다.
