# PR #1316 Tailnet 게시 절차 · 실패 관점 20개

게시 절차와 파일 복사 경계를 PR diff 검토와 별도로 확인했다. 제품 구현이나 C11.4 완료 검토로 세지 않는다.

| # | 실패 관점 | 확인과 결과 | 판정 |
|---:|---|---|---|
| 1 | Tailscale backend 중단 | preflight에서 `BackendState=Running`과 online Mac Studio를 확인했다. | 통과 |
| 2 | 다른 프로세스가 preview port를 점유 | 유일한 `127.0.0.1:8768` listener의 command가 `tailnet-preview-server.mjs`인지 확인했다. | 통과 |
| 3 | 잘못된 preview root를 수정 | sync script가 `~/Library/Application Support/SpinonPreview`의 실제 경로와 일반 디렉터리 여부를 검사한다. | 통과 |
| 4 | source commit/tree와 게시 파일 불일치 | selected-root와 publication manifest가 `91179a34089c1fa2f3f964a83c4c4b28d424d9f1` 및 tree hash에 묶였다. | 통과 |
| 5 | symlink를 통해 예상 밖 파일을 게시 | selected-root source와 생성 site의 symlink를 거부하고 source file 47개를 기록했다. | 통과 |
| 6 | 다음 계획 route 누락 | public roadmap의 `/spinon/plan/...auto-minimum.html`와 publication의 실제 route를 사전 assertion으로 연결했다. | 통과 |
| 7 | 계획 검토·Chrome 증거 링크 미포함 | 계획 본문, plan-review-20, PR-review-30 및 Chrome evidence README·연결된 screenshot을 selected root에서 확인했다. | 통과 |
| 8 | 현재 `PR #1314` history 손실 | 기존 implementation card의 merge SHA와 세 local links를 확인했고 수정 route를 모두 manifest에 넣었다. | 통과 |
| 9 | 공식 완료 수나 미완료 체크 변형 | `78/214`, C11/C11.4 unchecked, 계약 `0.1.0`을 빌드 전 assertion으로 유지했다. | 통과 |
| 10 | 새 경로 선택 과정에서 전체 문서 build로 과장 | 선택 build는 Rspress 2.0.22의 정해진 root만 빌드하고 full docs link validation은 false로 기록한다. | 통과 |
| 11 | 기존 시작·404 페이지 덮어쓰기 | `index.html`과 `404.html`은 publication에서 명시적으로 제외했다. | 통과 |
| 12 | 교체 전 열린 파일 바꾸기 | preflight가 25개 교체 대상 각각의 `lsof` handle을 검사해 전체 0개를 확인했다. | 통과 |
| 13 | 기존 preview 파일의 rollback 자료 손실 | backup 경로가 고유함을 확인하고 교체 대상 25개를 덮어쓰기 전 복사했다. | 통과 |
| 14 | preflight 후 source 또는 target 동시 변경 | copy 직전에 target의 존재·기존 hash와 publication source hash가 preflight와 같은지 다시 확인한다. | 통과 |
| 15 | 부분 파일이 노출되는 복사 | 임시 sibling file에 쓴 다음 rename하고 destination hash를 즉시 대조했다. | 통과 |
| 16 | 미선택 경로 삭제 | 실행 결과 `noDeletes=true`; 47개 생성·25개 교체·45개 동일, 삭제 0개다. | 통과 |
| 17 | local 파일의 사후 손상 | preview root의 117개 route가 publication hash와 모두 일치한다. | 통과 |
| 18 | Tailnet HTTPS에서 stale 또는 실패 응답 | HTTPS 117 route 모두 HTTP 200과 publication SHA-256 일치를 확인했다. | 통과 |
| 19 | 서버 재시작 또는 세션 잔존 | sync는 preview server PID `90711`을 유지했다. Agent Browser 세션은 닫혀 `active:false`, PID null이다. | 통과 |
| 20 | 모바일 clipping·잘못된 다음 링크 | Agent Browser desktop 1440px, mobile 390px에서 route·`78/214`를 확인하고 두 화면 모두 document/body 가로 overflow 0을 확인했다. | 통과 |

**결과:** 서로 다른 게시 절차 실패 관점 20개를 확인했다. route 복사와 화면 캡처는 Tailnet preview 확인이며 앱 런타임 동작이나 제품 완료 근거가 아니다.
