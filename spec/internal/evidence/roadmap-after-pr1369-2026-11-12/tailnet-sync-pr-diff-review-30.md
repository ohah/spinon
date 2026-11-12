# PR #1369 Tailnet 선택 경로 동기화 diff 검토 30개

실제 변경 대상은 사용자 지정 Tailnet static preview 아래의 선택 문서 route와 Rspress static assets다. Pass 01 검증 뒤 Agent Browser가 발견한 stale NEXT card를 수정했고 pass 02를 최종 상태로 검토했다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 다른 Tailnet host 또는 경로에 배포 | 지정 host와 `/spinon/` base만 사용했다. |
| 2 | preview process를 재시작하거나 종료 | PID 90711 / port 8768을 확인하고 그대로 유지했다. |
| 3 | 다른 디렉터리를 preview root로 오인 | server 설정의 `SpinonPreview` static root를 사용했다. |
| 4 | 경로 접두어 `/spinon/` 누락 | local HTTP 및 Tailnet URL 검증이 동일 접두어를 사용한다. |
| 5 | 전체 사이트를 복사하거나 삭제 포함 동기화 | 선택한 168 route와 static asset만 처리하며 deletion count는 0이다. |
| 6 | 전체 Rspress build 또는 전체 dead-link 검사라고 과장 | Rspress 2.0.22 선택 빌드 12 route만 보고했다. |
| 7 | 변경된 핵심 문서 route 누락 | roadmap, STATUS 양 경로, track-sizing, mixed plan, 계약 0059와 연결 evidence를 포함했다. |
| 8 | plan review·implementation review route 누락 | 선택 빌드 12 route와 HTTP manifest에서 검증했다. |
| 9 | 새 merge metadata page의 생성 누락 | `merge-PR1369.html`이 생성되고 Tailnet에서 열렸다. |
| 10 | source markdown과 built HTML route를 혼동 | markdown 원본과 Rspress output을 각각 목적 경로에 설치했다. |
| 11 | public roadmap.html을 Rspress output에 있다고 가정 | standalone HTML source를 직접 선택 경로에 복사했다. |
| 12 | 새 HTML이 참조하는 static bundle 누락 | selected build의 `static` tree 전체를 복사하고 파일 hash를 비교했다. |
| 13 | 이전 preview file이 symlink 경로를 통해 교체 | source와 destination path components의 symlink를 거부한다. |
| 14 | path traversal 또는 absolute route를 허용 | route path의 절대경로·`.`·`..`·backslash를 거부한다. |
| 15 | 기존 파일의 rollback copy 없이 덮어쓰기 | 변경된 기존 파일마다 unique backup을 만든다. |
| 16 | backup이 원본과 달라 rollback을 무력화 | 각 backup byte count와 SHA-256을 검증했다. |
| 17 | 기존 backup을 조용히 덮어쓰기 | pass 01/02 고유 경로와 exclusive file creation을 사용한다. |
| 18 | partial write 중 서비스 파일 손상 | 임시 파일을 쓴 뒤 atomic rename하고 최종 hash를 확인한다. |
| 19 | unchanged file을 불필요하게 교체 | hash가 같으면 write를 생략한다. |
| 20 | 첫 sync의 dynamic card 오류를 숨김 | pass 01 intermediate 상태를 이름 붙여 보존하고 stale `#1365` 표시를 기록했다. |
| 21 | roadmap latest summary만 고치고 dynamic card 방치 | Agent Browser가 발견한 card source를 `#1369 merged`로 수정했다. |
| 22 | 두 번째 sync가 전체 기존 page를 불필요하게 재기록 | pass 02는 168개 중 roadmap HTML 1개만 교체하고 167개는 unchanged다. |
| 23 | 14개 첫 backup과 추가 1개를 누락 계산 | pass 01 14개, pass 02 1개 backup hash를 모두 확인했다. |
| 24 | 새 파일이 200이어도 다른 origin이 stale | 168/168 local HTTP와 Tailnet HTTPS가 모두 200이다. |
| 25 | source와 local preview bytes가 다름 | 모든 pass 02 record에서 source/preview/local SHA-256이 일치한다. |
| 26 | Tailnet TLS origin의 response body가 다름 | 모든 pass 02 record에서 Tailnet response의 bytes/hash가 일치한다. |
| 27 | C11.4/C11 상위 상태를 완료로 표시 | Agent Browser에서 C11.4/C11 미완료를 확인했다. |
| 28 | 공식 완료 수 또는 next candidate 표시가 과거 값 | 78/214, `#1369 merged`, 다음 candidate 미선정을 확인했다. |
| 29 | screenshot이 다른 URL·viewport·수정 전 상태를 기록 | final screenshots는 Tailnet URL의 1440×1000 및 390×844 viewport에서 수정 후 캡처했다. |
| 30 | 검증 종료 뒤 task browser·ADB·Simulator를 남기거나 preview server 종료 | task Agent Browser session을 닫고 ADB server를 중지했다. Simulator/AVD는 부팅되지 않았고 사용자 preview server는 유지했다. |

최종 외부 상태는 [served verification](./served-verification-pass02.json), 화면 상태는 [Agent Browser verification](./tailnet-agent-browser-verification.json), 프로세스 정리는 [resource cleanup](./resource-cleanup.json)에 있다. 이 검토는 full-site build나 전체 문서 링크 검증을 뜻하지 않는다.
