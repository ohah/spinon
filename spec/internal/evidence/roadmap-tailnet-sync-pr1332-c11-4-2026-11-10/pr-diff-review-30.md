# Tailnet roadmap PR diff 실패 관점 검토 30개

범위는 `roadmap-tailnet-sync-pr1332-c11-4-2026-11-10`의 선택 입력 생성, pinned Rspress build, Tailnet 파일 반영, rollback, Agent Browser 확인과 정리다. 공식 CSS 지원 상태나 구현 범위를 확장하지 않는다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 후속 동기화가 잘못된 source commit/tree에 묶임 | source HEAD `41e391117a76d3a6db8329adf89bddf64f281367`, tree `f626ac9f68d045b4d9bf833c0208bbb0c06bbbe4`를 manifest에 기록했다. |
| 2 | 문서 PR이 실제 병합 전에 제공됨 | PR #1332의 GitHub state는 `MERGED`, merge SHA `41e391117a76d3a6db8329adf89bddf64f281367`다. |
| 3 | 로드맵이 다른 구현 commit을 최근 병합으로 표시함 | #1331과 merge SHA `4b4f48ea67ab4383954cc89cc21e1a71bdb29732`를 원본·Tailnet 화면에서 확인했다. |
| 4 | 완료 수가 작업 진행에 따라 부풀려짐 | 선택 `STATUS.md`, roadmap 및 Agent Browser 화면은 모두 `78/214`다. |
| 5 | 제한된 slice가 C11/C11.4 완료로 잘못 표시됨 | 공식 체크는 미완료이고 로드맵에 C11.4/C11 미완료 문구가 남아 있다. |
| 6 | 내부 인터페이스 숫자 계약이 불필요하게 상승함 | 내부 계약 `0.1.0`을 선택 문서와 로드맵에서 확인했다. |
| 7 | conformance 기록이 누락돼 공개 범위를 오해함 | `spec/0001-conformance.md`를 `0001-conformance.html` 경로로 선택 입력에 넣었다. |
| 8 | 상태 대장 또는 Grid 내부 계약 페이지가 누락됨 | `STATUS.md`와 내부 계약 0059의 HTML route alias를 각각 생성했다. |
| 9 | 현재 계획 및 계획 검토 페이지가 누락됨 | 현재 C11.4 position 4–6 계획·20관점 리뷰 파일이 선택 root에 있다. |
| 10 | 런타임/사전 비교 페이지의 상대 자료가 빠짐 | 두 evidence 디렉터리를 재귀 수집해 화면·로그·fixture·검증기를 함께 넣었다. |
| 11 | source symlink가 publication 밖 파일을 가져옴 | source 파일·디렉터리는 symlink를 거부하고 selected-root manifest에 symlink 0을 기록한다. |
| 12 | route에 절대 경로나 `..`가 들어가 preview root 밖을 덮음 | source와 publication 스크립트의 `safeRoute`가 절대·backslash·빈·`.`·`..` segment를 거부한다. |
| 13 | 서로 다른 파일이 같은 route를 덮어씀 | 중복 route는 SHA-256이 같을 때만 허용한다. |
| 14 | Rspress 산출물의 symlink나 특수 파일이 게시됨 | publication walk는 symlink와 regular file/directory 이외 항목을 거부한다. |
| 15 | 프로젝트가 고정하지 않은 Rspress 버전으로 build됨 | 실제 build는 저장소 설치본 `@rspress/core` 2.0.22 CLI를 사용했다. |
| 16 | `bunx`가 내려받은 1.47.2가 제품 문서를 생성함 | 1.47.2는 도움말 조회에서만 출력됐고 실제 build에는 사용되지 않았다. |
| 17 | 상대 이미지가 package import로 해석돼 evidence page가 실패함 | 선택 build 전용 Rspack `preferRelative` 적용 후 네 후보 화면이 포함된 페이지 build가 통과했다. |
| 18 | 실패한 build 로그가 성공 근거로 오인됨 | 초기 상대 PNG 오류 로그와 최종 2.0.22 성공 로그를 별도 파일로 보존했다. |
| 19 | 전체 문서 링크 검증을 통과했다고 과장함 | 선택 route 빌드만 했고 전체 dead-link 검사는 수행하지 않았다고 기록했다. |
| 20 | #1331 kanban 요약과 상세 진행 문단을 혼동함 | 검증 marker가 상세 progress 문단을 선택하고, 내부 링크 두 개를 각각 route manifest에 대조한다. |
| 21 | 만들어진 route 수가 manifest와 다름 | generated route와 selected source 합계 220개, 총 59,120,197 bytes로 일치한다. |
| 22 | preview 시작 페이지나 404 경로를 선택 build가 덮음 | `index.html`, `404.html`은 publication에서 제외하고 기존 파일을 보존했다. |
| 23 | 선택 동기화가 기존 preview 파일을 삭제함 | 동기화는 생성·교체만 하며 preflight/copy/verify 기록 모두 `noDeletes: true`다. |
| 24 | 꺼진 Tailnet 또는 다른 preview 주소에 게시함 | Tailscale backend `Running`과 사용자가 지정한 HTTPS route를 확인했다. |
| 25 | 다른 프로세스를 잘못된 서버로 교체함 | 8768 listener는 PID 90711이며 지정 `tailnet-preview-server.mjs`임을 확인했다. |
| 26 | copy 전 기존 roadmap을 잘못된 source로 간주함 | preflight는 local HTTP 200과 이전 roadmap hash `bb10c50a…`를 기록했다. |
| 27 | 열려 있는 route를 바꿔 사용자 응답이 중간 상태가 됨 | 교체 route의 preflight open handle은 0이며 copy 직전에도 기존 상태와 open handle을 재검사한다. |
| 28 | backup이 기존 자료를 덮거나 복구 hash가 다름 | task 전용 backup 경로가 비어 있음을 확인했고 교체된 27개 backup hash가 모두 검증됐다. |
| 29 | 경로 분류·write가 유실 또는 비원자적으로 됨 | create/replace/same은 `147/27/46`이며, 교체는 같은 디렉터리 임시 파일에서 rename한다. |
| 30 | 최종 경로·화면·리소스 확인 뒤 세션이나 임시 산출물이 남음 | local·HTTPS hash 220/220, desktop/mobile 폭 overflow 0, 이미지 4/4 로드, browser errors 0; Agent Browser session은 닫았다. 임시 build root는 open handle 0 확인 후 제거하고 rollback backup은 보존했다. |

상세 요청·응답 hash는 [preflight](./preflight.json), [copy/backup](./copy-and-backup.json), [최종 제공 확인](./served-verification.json), [Agent Browser 확인](./agent-browser-verification.json)에 있다.
