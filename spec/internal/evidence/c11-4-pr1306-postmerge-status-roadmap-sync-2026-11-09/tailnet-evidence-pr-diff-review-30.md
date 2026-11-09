# Tailnet 로드맵 제공 근거 diff 검토 30개

대상은 정적 공개 로드맵 링크 수정과 선택 Tailnet 파일의 sync/browser evidence다. 각 검토는 별도의 외부 노출·hash·링크·lifecycle 실패 경로를 확인한다.

| # | 실패 관점 | 결과 |
|---|---|---|
| 1 | sync가 요청된 로드맵 파일 외 다른 경로를 덮어쓴다. | PASS — `filesChanged`는 `/spinon/roadmap.html` 하나다. |
| 2 | private preview 대신 공식 GitHub Pages를 배포한다. | PASS — Pages workflow를 실행하지 않았다. |
| 3 | backup이 동기화 이후 파일을 복사한다. | PASS — 기존 target hash 검증 뒤 copy를 만들었다. |
| 4 | backup 내용이 교체 전 제공본과 다르다. | PASS — pass01/pass02 backup hash가 preflight hash와 같다. |
| 5 | Tailscale backend가 내려간 상태를 놓친다. | PASS — 사전 상태는 `Running`이다. |
| 6 | 예상치 못한 포트 프로세스에 쓰기 작업을 적용한다. | PASS — PID 28040 command가 `tailnet-preview-server.mjs`임을 확인했다. |
| 7 | preview target을 다른 프로세스가 잡고 있는 중에 교체한다. | PASS — 교체 전 target open handle은 0개다. |
| 8 | 임시 파일 쓰기가 완료되기 전에 제공 경로를 바꾼다. | PASS — 같은 디렉터리에서 `fsync` 후 atomic rename했다. |
| 9 | 로컬 파일은 바뀌었지만 loopback 제공본은 이전이다. | PASS — loopback 응답이 pass02 source SHA와 일치한다. |
| 10 | Tailnet 응답이 loopback 또는 source와 다르다. | PASS — 세 SHA가 `968ffa...`로 일치한다. |
| 11 | 실수로 전체 preview server를 재시작한다. | PASS — PID 28040을 유지했고 재시작하지 않았다. |
| 12 | 기존 preview를 백업 없이 덮어쓴다. | PASS — pass01과 pass02를 별도 backup path에 보존한다. |
| 13 | PR #1306 merge SHA나 timestamp를 잘못 표시한다. | PASS — 페이지의 PR/merge 메타데이터가 병합 결과와 일치한다. |
| 14 | official completion count가 드리프트한다. | PASS — Browser가 공식 `78/214`를 읽었다. |
| 15 | C11.4 제한 slice를 상위 완료로 보인다. | PASS — C11.4/C11 미완료 문구가 남아 있다. |
| 16 | 이미 병합한 slice를 다음 작업으로 다시 올린다. | PASS — 칸반은 후속 bounded slice 미선정을 표시한다. |
| 17 | Desktop에서 document width가 viewport보다 넓다. | PASS — 1440px viewport에서 document/body 모두 1440px이다. |
| 18 | Mobile에서 document width가 viewport보다 넓다. | PASS — 390px viewport에서 document/body 모두 390px이다. |
| 19 | Mobile 카드가 화면 밖으로 나가거나 좌우 잘린다. | PASS — 카드 bounds x=14..376, viewport 안이다. |
| 20 | 스크린샷이 새 카드 위치로 스크롤되기 전에 저장된다. | PASS — desktop/mobile 각각 card bounds가 viewport 안인 뒤 캡처했다. |
| 21 | 이전 Tailnet `/spinon/plan/...html` 문서 경로가 계속 남는다. | 수정 — 404 확인 뒤 main Markdown GitHub blob 링크로 교체했다. |
| 22 | 계획 GitHub blob URL이 잘못되거나 오래된 문서를 연다. | PASS — Agent Browser에서 main 계획 페이지와 PR #1306 merge SHA를 확인했다. |
| 23 | 내부 계약 링크가 잘못된 파일을 연다. | PASS — 0059 Markdown GitHub blob 페이지가 로드됐다. |
| 24 | runtime evidence README 링크가 없는 경로다. | PASS — main README 페이지가 로드됐다. |
| 25 | 구현/runtime 20개 검토 링크가 누락된다. | PASS — Markdown GitHub blob 페이지가 로드됐다. |
| 26 | PR diff 30개 검토 링크가 누락된다. | PASS — Markdown GitHub blob 페이지가 로드됐다. |
| 27 | Android screenshot이 raw URL에서 404거나 다른 크기다. | PASS — Agent Browser가 완전 로드된 1080×2400 이미지로 확인했다. |
| 28 | iOS screenshot이 raw URL에서 404거나 다른 크기다. | PASS — Agent Browser가 완전 로드된 1206×2622 이미지로 확인했다. |
| 29 | Agent Browser가 확인 뒤 활성 Chrome/session을 남긴다. | PASS — close all 후 세션 0개; CLI 종료, 기존 Chrome/Terminal Browser는 사전 프로세스로 분류해 보존했다. |
| 30 | local Rspress 빌드 실패를 preview 성공 또는 CI green으로 확대한다. | PASS — dead-link 빌드 실패를 별도로 기록하고, static Tailnet page와 링크만 Agent Browser로 검증했다. |

이 증거는 선택된 private Tailnet preview 파일 및 C11.4 PR #1306 카드 범위에 한정한다. 공식 site 배포·전체 문서 link audit·실기기 parity를 의미하지 않는다.
