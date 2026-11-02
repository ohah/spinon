# PR #1075 Tailnet 상태 대장 동기화 변경 검토 30개

**검토 대상:** PR #1075 병합 뒤 선택 발행, Tailnet `/STATUS.html` 갱신, Agent Browser 확인, process 종료 근거.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | publication이 PR #1075 이전 main source를 가리킴 | **통과.** source HEAD는 merge SHA `42d591f3f2509a0b5c9795815eb45ea206c0197d`다. |
| 2 | source tree identity가 다른 상태로 기록됨 | **통과.** source tree `3065acaf805369958426c43680ebc1555021d46a`를 manifest에 고정했다. |
| 3 | 선택 root가 Git 추적 밖 파일을 포함 | **통과.** tracked source 목록으로 root를 만들었다. |
| 4 | selected root의 symlink가 복사 범위에 섞임 | **통과.** source·생성 site symlink를 거부한다. |
| 5 | manifest 밖 route가 publication에 추가됨 | **통과.** publication route와 manifest route 집합을 대조했다. |
| 6 | 경로 traversal 또는 absolute route가 preview에 기록 | **통과.** 각 route의 절대 경로·`.`·`..`·역슬래시를 거부한다. |
| 7 | 전체 문서 build와 selected build를 같은 근거로 보고 | **통과.** 선택된 RSPress build만 성공으로 기록한다. |
| 8 | 저장소 전체 dead-link 검사가 실행된 것으로 오해 | **통과.** 선택 build는 full-site link validation이 아니라고 쓴다. |
| 9 | RSPress dependency가 승인되지 않은 버전으로 바뀜 | **통과.** 고정 `2.0.22`를 사용했다. |
| 10 | publication이 예상 밖 route 개수 또는 크기 | **통과.** 81개 selected source, 120개 route, 12,858,919 bytes다. |
| 11 | Tailscale backend가 중지된 상태에서 remote 갱신 시도 | **통과.** 사전 점검에서 backend `Running`을 확인했다. |
| 12 | 예상치 못한 listener를 기존 preview server로 오인 | **통과.** PID `28040`, `tailnet-preview-server.mjs`, `127.0.0.1:8768`을 확인했다. |
| 13 | 갱신 전 local origin이 이미 실패 | **통과.** local roadmap는 HTTP 200이었다. |
| 14 | 갱신 전 Tailnet origin이 이미 실패 | **통과.** Tailnet roadmap는 HTTP 200이었다. |
| 15 | 상태 대장 stale 문구가 실제 live route에 있었는지 미확인 | **통과.** 갱신 전 Agent Browser에서 `implementation candidate` 문구를 읽었다. |
| 16 | preflight action count 합이 manifest와 다름 | **통과.** 17 create + 16 replace + 87 same = 120 routes다. |
| 17 | 파일을 열고 있는 reader가 있는 route를 무시 | **통과.** 기존 reader PID `28040`을 `STATUS.html`에서 확인했다. |
| 18 | 기존 파일을 truncate/write해 reader가 빈 문서를 볼 수 있음 | **통과.** 동일 디렉터리 temp와 atomic rename을 사용했다. |
| 19 | 교체 전 bytes 백업이 없거나 target이 불명확 | **통과.** PR #1075 고유 backup 경로에 보존했다. |
| 20 | 기존 route를 정리 명목으로 삭제 | **통과.** `noDeletes=true`이고 삭제 route가 없다. |
| 21 | server restart로 활성 미리보기를 끊음 | **통과.** PID `28040`은 계속 실행 중이다. |
| 22 | Tailnet `STATUS.html`이 publication과 다른 bytes 반환 | **통과.** local·Tailnet 양쪽에서 expected SHA-256과 일치한다. |
| 23 | public roadmap까지 불필요하게 바뀌거나 stale 상태로 기록 | **통과.** 기존 `/roadmap.html` SHA-256은 전후 동일하다. |
| 24 | 검증 route 중 한 origin만 성공 | **통과.** 주요 10 route 각각 local·Tailnet HTTP 200을 확인했다. |
| 25 | `STATUS.html` count가 바뀌어 보임 | **통과.** 화면 본문에서 공식 `72/213`을 확인했다. |
| 26 | 제한 C11 slice를 C11 완료로 승격 | **통과.** 본문과 manifest 모두 `c11Complete=false`다. |
| 27 | stale `#1070` 후보 label이 갱신 후 남음 | **통과.** Agent Browser가 새 limited-runtime label을 읽었다. |
| 28 | Agent Browser desktop 화면이 수평 overflow를 가림 | **통과.** `documentWidth=1280`, viewport width도 1280px이다. |
| 29 | Agent Browser session 또는 process를 남김 | **통과.** `close --all` 후 session 목록은 빈 배열이고 Agent Browser process 검색 결과도 없다. |
| 30 | 검증 뒤 preview server나 GitHub Pages 상태를 잘못 바꿈 | **통과.** preview server는 보존했고 GitHub Pages는 실행하지 않았다. |

**결론:** `/STATUS.html`을 PR #1075의 main 상태와 맞추고 local/Tailnet byte identity와 브라우저 표시를 확인했다. public roadmap, C11 완료 checkbox 및 공식 완료 수는 변경하지 않았다.
