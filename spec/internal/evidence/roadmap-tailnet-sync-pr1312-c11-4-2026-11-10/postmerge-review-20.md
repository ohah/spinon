# PR #1312 Tailnet 병합 후 동기화 · 실패 관점 20개

검토 대상은 PR #1312 병합 상태, 선택 문서 빌드, 53-route 제공본, desktop/mobile Agent Browser 캡처와 프로세스 정리 기록이다.

| # | 실패 관점 | 확인 방법 | 결과 |
|---:|---|---|---|
| 1 | 열린 PR을 병합 완료로 기록 | GitHub PR 재조회에서 `closed`, `merged=true` 및 merge SHA 확인 | 통과 |
| 2 | 계획 merge를 CI green으로 과장 | GitHub check rollup이 비어 있음을 기록 | 통과 |
| 3 | 오래된 main을 게시 원본으로 사용 | source commit `6439049c16cc107682292dd7c4d6c64dc4c1b413` 확인 | 통과 |
| 4 | 공식 완료 수를 계획으로 승격 | roadmap과 status가 `78/214`를 유지하는지 대조 | 통과 |
| 5 | 구현 전 계획을 구현 중/완료로 표시 | roadmap과 새 plan에 `구현 전` 표시 확인 | 통과 |
| 6 | row-span PR #1311을 auto-placement 지원으로 혼합 | roadmap에 #1311을 병합 이력, auto-placement를 다음 계획으로 분리 | 통과 |
| 7 | Tailnet server가 중단된 상태에서 게시 | `tailscale status --json`의 `Running`, online 확인 | 통과 |
| 8 | 기존 server를 불필요하게 재시작 | 게시 전후 PID 28040 유지 확인 | 통과 |
| 9 | 기존 파일을 backup 없이 덮어씀 | 11개 교체본의 백업 SHA-256 확인 | 통과 |
| 10 | 관련 없는 파일이나 route를 삭제 | publication result의 `noDeletes=true` 및 삭제 작업 없음 확인 | 통과 |
| 11 | 링크가 main 아래 경로를 가리키지 않음 | 새 plan route가 공개 roadmap HTML에 존재하는지 확인 | 통과 |
| 12 | 새 plan route가 게시 전에 노출 | 새 문서 route를 먼저 게시하고 `roadmap.html`을 마지막에 교체 | 통과 |
| 13 | 제공본이 게시 입력과 달라짐 | Tailnet 53/53 route의 byte length와 SHA 대조 | 통과 |
| 14 | local preview가 Tailnet과 다르게 남음 | localhost preview 53/53 route의 HTTP 200과 SHA 대조 | 통과 |
| 15 | PR #1311 merge 기록이나 공식 수가 빠짐 | desktop/mobile DOM에서 #1311과 `78/214` 확인 | 통과 |
| 16 | mobile page 자체에 가로 overflow 발생 | `390px` viewport에서 document width `390px` 확인 | 통과 |
| 17 | 표본 이미지가 링크만 있고 로드되지 않음 | Tailnet README의 image `complete` 및 natural size `1280×900` 확인 | 통과 |
| 18 | selected build를 전체 docs green으로 잘못 보고 | selected build와 disabled dead-link checks를 분리 기록, full build green 주장 제외 | 통과 |
| 19 | Agent Browser session이 남음 | 세 개의 고유 session을 닫고 post-run 프로세스 목록 확인 | 통과 |
| 20 | 다른 사용처의 simulator/browser 자원을 삭제 | 이번 run은 simulator를 시작하지 않았고 공유 profile과 사전 실행 프로세스는 보존 | 통과 |

**결과:** 20개 실패 관점을 확인했다. 이번 게시에서는 파일 삭제나 server 재시작이 없었고, Chrome/Agent Browser 화면은 제품 runtime 증거로 분류하지 않는다.
