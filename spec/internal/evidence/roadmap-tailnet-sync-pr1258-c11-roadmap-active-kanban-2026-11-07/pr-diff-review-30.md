# Tailnet roadmap 동기화 PR 변경 검토 30개

이 PR은 병합된 roadmap의 Tailnet preview 동기화 및 확인 evidence를 추가한다. 제공본 경계, 이전 파일 보존, browser viewport와 현재 상태 표기를 별도 실패 관점으로 검토했다.

| # | 실패 경로·불변 조건 | 확인 기준 | 결과 |
|---:|---|---|---|
| 1 | 동기화 source가 병합 전 branch일 수 있음 | source SHA와 main merge revision을 evidence에 기록 | 통과 |
| 2 | Tailnet backend가 내려가 있을 수 있음 | preflight `BackendState=Running` | 통과 |
| 3 | preview listener가 다른 포트나 주소일 수 있음 | PID `28040`, `127.0.0.1:8768` 및 server command 확인 | 통과 |
| 4 | 기존 파일에 열린 handle이 있어 atomic replacement가 불안정할 수 있음 | preflight `lsof` 결과 empty | 통과 |
| 5 | backup 경로가 기존 자료를 덮어쓸 수 있음 | 고유 경로 사전 부재 확인과 `COPYFILE_EXCL` | 통과 |
| 6 | 원본 preview를 보존하지 않고 바꿀 수 있음 | 이전 620,756-byte 제공본을 별도 backup | 통과 |
| 7 | backup이 원본과 달라질 수 있음 | backup SHA가 이전 제공본 SHA와 같음 | 통과 |
| 8 | HTTP preview가 sync 전 stale일 수 있음 | 이전 local HTTP bytes/hash가 target과 일치 | 통과 |
| 9 | HTTPS preview가 sync 전 stale일 수 있음 | 이전 Tailnet HTTPS bytes/hash가 target과 일치 | 통과 |
| 10 | source에서 핵심 병합 정보가 빠질 수 있음 | PR #1256, merge SHA, `flex-end 계획` marker 확인 | 통과 |
| 11 | 모바일 viewport 변경이 빠진 artifact일 수 있음 | `kanbanColumnStep`과 모바일 안내 marker 확인 | 통과 |
| 12 | `78/214`가 sync 중 변할 수 있음 | source marker 및 Agent Browser status note 일치 | 통과 |
| 13 | C11.4/C11이 완료로 과장될 수 있음 | 미완료 marker와 status note 유지 | 통과 |
| 14 | temp file이 다른 filesystem에 생성되어 atomic rename이 아닐 수 있음 | target과 같은 디렉터리에 temp 생성 | 통과 |
| 15 | write 완료 전에 바뀐 파일이 노출될 수 있음 | rename 전에 temp descriptor `fsync` | 통과 |
| 16 | source와 target bytes가 달라질 수 있음 | source·target SHA-256 exact match | 통과 |
| 17 | local HTTP가 새 bytes를 제공하지 않을 수 있음 | sync 후 HTTP 200, 621,472 bytes, matching SHA | 통과 |
| 18 | Tailnet HTTPS가 새 bytes를 제공하지 않을 수 있음 | sync 후 HTTPS 200, 621,472 bytes, matching SHA | 통과 |
| 19 | 응답이 200이어도 HTML 내용이 stale일 수 있음 | response SHA를 source SHA와 대조 | 통과 |
| 20 | post-sync mismatch가 나도 기존 파일을 잃을 수 있음 | mismatch branch가 backup을 atomic restore하도록 스크립트에 포함 | 통과 |
| 21 | preview server 재시작이 필요했다고 오해할 수 있음 | sync record `serverRestarted=false`, PID 유지 | 통과 |
| 22 | Tailnet page와 GitHub Pages 배포를 혼동할 수 있음 | Pages/gh-pages unchanged를 명시 | 통과 |
| 23 | browser가 localhost나 cached body를 볼 수 있음 | Agent Browser URL은 Tailnet HTTPS, DOM에 새 source markers 확인 | 통과 |
| 24 | desktop viewport에서 card가 fold 아래에 남을 수 있음 | 1440×1000 card y=455.02px, viewport 안 | 통과 |
| 25 | mobile viewport에서 card가 off-screen column에 남을 수 있음 | 390×844, scrollLeft 272px, active card y=438.53px | 통과 |
| 26 | mobile 내부 scroll이 전체 문서를 가로로 넘길 수 있음 | document width 390px로 viewport와 일치 | 통과 |
| 27 | desktop board가 폭 때문에 잘릴 수 있음 | frame 1260px, content 1260px | 통과 |
| 28 | screenshot과 DOM observation이 서로 다른 capture이거나 PR 이미지 URL이 깨질 수 있음 | viewport·bounds·screenshot path/hash 기록 및 PR #1259 본문 이미지 natural dimensions 확인 | 통과 |
| 29 | Agent Browser 또는 Chrome process가 남을 수 있음 | close 후 process scan empty 기록 | 통과 |
| 30 | 이전 preview backup 또는 Tailnet server를 cleanup할 수 있음 | backup 보존, server PID 28040 유지 | 통과 |

검증은 제공본 hash와 desktop/mobile UI 관측에 한정된다. C11 implementation, simulator, WPT, physical device, GitHub Pages 배포는 이 동기화 PR의 범위가 아니다.
