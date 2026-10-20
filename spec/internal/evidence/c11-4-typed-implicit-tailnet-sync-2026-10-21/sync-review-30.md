# Tailnet 로드맵 동기화 실패 관점 검토

| # | 독립 실패 관점과 확인 | 결과 |
| --- | --- | --- |
| 1 | 병합되지 않은 소스에서 미리보기를 만들 수 있는가 — 기록한 기준 SHA와 현재 `origin/main`을 대조한다. | 입력 기준은 PR #399 병합 SHA `4d69a05e316685497ea3950bb325c81a79608819`다. |
| 2 | C11.4 실기기 부분 결과를 전체 완료로 과장하는가 — Tailnet 로드맵 note와 공식 체크를 브라우저에서 함께 확인한다. | 로드맵은 14 scene·PixelCopy 5/5만 보고하고 공식 체크는 미완료다. |
| 3 | 공식 완료 수가 입력 변환 중 달라지는가 — 두 STATUS 경로의 현재 소스 hash와 Agent Browser 체크를 확인한다. | 두 상태 경로는 같은 현재 문서 내용이며 C11.4 체크는 꺼져 있다. |
| 4 | 중복 계획 route 중 하나가 오래된가 — `/plan/`과 `/spec/plan/` 입력을 같은 원본에서 덮는다. | 두 route 모두 `plan/c11-4-grid-track-sizing.md`의 현재 bytes를 쓴다. |
| 5 | 내부 색인이 새 evidence 페이지를 놓치는가 — 복제된 `internal/README.md`와 브라우저 링크를 확인한다. | 색인에 typed implicit 실기기 근거 링크가 있고 목적지에서 14/14와 5/5가 보인다. |
| 6 | 로드맵 링크만 바뀌고 렌더링 스크립트가 이전 버전인가 — 같은 RSPress 출력의 HTML과 chunk를 함께 복사한다. | 선택 사이트 전체를 빌드해 HTML과 JS를 한 snapshot으로 동기화했다. |
| 7 | 문서 사이트의 하위 base path가 asset URL과 어긋나는가 — `/spinon/` route의 Agent Browser 페이지와 모든 served URL을 확인한다. | 1,732개 URL이 `/spinon/` 아래에서 HTTP 200·hash 일치로 제공된다. |
| 8 | 새 evidence Markdown이 링크만 있고 페이지가 생성되지 않는가 — `README.html` 출력과 실제 Tailnet route를 연다. | 실기기 evidence route가 완전히 렌더링됐다. |
| 9 | Android screenshot asset을 선택 root에서 누락하는가 — 페이지 image URL과 natural dimensions를 본다. | 두 기기 캡처가 모두 1080×2340으로 로드됐다. |
| 10 | RSPress나 dependency version이 문서 작업 중 바뀌는가 — 선택 config와 package files를 확인한다. | config만 별도로 만들었고 프로젝트 RSPress 버전은 변경하지 않았다. |
| 11 | dead-link 검사 비활성화를 저장소 전체 성공으로 보고하는가 — config와 빌드 로그의 범위를 기록한다. | 검사 비활성화는 pass25 선택 빌드 config에만 있고 저장소 전체 빌드는 이번에 실행하지 않았다. |
| 12 | 기존 성공 input에 PR #399 근거가 빠져 있는가 — 새 evidence 44개 원본 파일과 두 선택 경로를 비교한다. | 현재 실기기 근거 파일 44개가 두 evidence route 아래 들어 있다. |
| 13 | 현재 source와 새 output 사이에 누락 파일이 생기는가 — 재귀 파일 집합·바이트·SHA manifest를 만든다. | output 전체 1,732개 파일을 manifest에 기록했다. |
| 14 | 오래된 RSPress async chunk가 새 페이지를 가리키는가 — 전체 build output과 Tailnet served hash를 비교한다. | 새 bundle과 정적 파일을 포함해 1,732/1,732가 일치한다. |
| 15 | 선택 root에 symlink가 들어가 대상 밖 파일을 복사하는가 — root와 새 evidence tree에서 symlink를 거부한다. | 선택 입력과 output 순회에서 symlink는 발견되지 않았다. |
| 16 | 선택 root를 반복 사용해 지난 pass의 새 문서를 덮어쓰는가 — pass25 root와 site가 없음을 확인한 뒤 전용 경로를 만든다. | 새 pass25 경로만 사용했다. |
| 17 | 빌드 파일이 사전 비교 이후 바뀌어 혼합본을 복사하는가 — copy·verify 단계에서 source bytes와 SHA를 다시 확인한다. | 모든 source가 preflight SHA와 일치했다. |
| 18 | 기존 preview 파일이 사전 비교 후 다른 내용으로 바뀌는가 — 교체 직전 대상의 크기와 SHA를 재검증한다. | 변경된 대상이 있으면 동기화 스크립트가 중단하도록 되어 있다. |
| 19 | 동일한 기존 파일을 불필요하게 교체하는가 — preflight status별 처리 수를 확인한다. | 동일 1,537개는 그대로 두고 create/replace만 썼다. |
| 20 | 새 route에서 참조하지 않는 옛 파일을 지워 링크를 끊는가 — summary와 preflight의 delete 정책을 확인한다. | 삭제 0건이다. |
| 21 | 교체 전 원본이 다른 동기화 백업과 섞이는가 — 날짜·PR 범위를 포함한 새 pass25 backup root를 사용한다. | 전용 backup root에 93개를 보존했다. |
| 22 | 백업 write가 잘렸거나 bytes가 바뀌는가 — 각 backup의 크기·SHA를 copy 후와 최종 확인에서 계산한다. | 백업 93/93이 기존 target SHA와 일치한다. |
| 23 | 재개 때 잘못된 기존 backup을 원본으로 간주하는가 — 존재하는 backup도 이전 크기·SHA가 정확해야 재사용한다. | 동기화 스크립트가 일치하지 않는 backup이면 중단한다. |
| 24 | 대상 파일 교체 중 서버가 부분 파일을 읽는가 — 같은 디렉터리 임시 파일에 쓴 뒤 fsync·rename한다. | 대상은 원자 rename으로 반영되고 매 파일을 다시 읽어 검증했다. |
| 25 | 파일을 열어 둔 다른 프로세스의 inode를 무시하는가 — 교체 직전 `lsof` PID를 제한 서버와 대조한다. | 열린 이전 파일 6개는 모두 PID 1191 하나만 보유했다. |
| 26 | 열린 파일을 가진 PID가 예상 Tailnet 서버가 아닌가 — `ps` 명령과 포트 수신 상태를 확인한다. | PID 1191 명령은 `tailnet-preview-server.mjs`이고 `127.0.0.1:8768`만 listen했다. |
| 27 | 업데이트 중 Tailnet preview 서버를 종료하거나 재시작하는가 — 반영 뒤 PID와 listener를 다시 본다. | 서버 PID 1191과 listener가 유지됐다. |
| 28 | 부분 동기화나 backup 오류가 성공처럼 기록되는가 — copy 결과, 전체 local verify와 backup verify의 세 결과를 비교한다. | 1,732 target과 93 backup 모두 확인됐다. |
| 29 | reverse proxy나 캐시가 다른 문서를 제공하는가 — 빌드 파일 1,732개를 Tailnet HTTPS에서 각각 내려받아 SHA를 비교한다. | HTTP 200·바이트·SHA가 모두 일치하고 mismatch는 0이다. |
| 30 | CLI 해시 결과와 사람에게 보이는 사이트가 다른가 — Agent Browser로 네 화면을 열고 본문·체크·이미지 완료 상태를 관찰한다. | 로드맵, 상태, 계획, evidence가 모두 렌더링됐고 C11.4 체크와 image dimensions가 예상과 맞다. |
